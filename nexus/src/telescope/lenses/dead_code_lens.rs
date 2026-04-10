use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DeadCodeLens: Dead code, dead branch, and dead allocation detection.
///
/// Finds data regions that contribute nothing to the output: zero-impact
/// dimensions, unreachable data regions, and allocations whose values
/// are never consumed. LLVM's DCE operates on SSA IR; this lens works
/// on data patterns to find semantic dead code that survives compilation.
///
/// Metrics:
///   1. dead_dimension_fraction: dims with zero variance or zero MI
///   2. unreachable_fraction: data points disconnected from main cluster
///   3. dead_allocation_score: allocated but never-read-like patterns
///   4. zero_impact_score: fraction of data removable without info loss
///   5. branch_liveness: fraction of data on "live" computation paths
///   6. effective_dimensionality: true rank vs nominal dimensionality
pub struct DeadCodeLens;

impl Lens for DeadCodeLens {
    fn name(&self) -> &str { "DeadCodeLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // --- Dead dimensions: zero variance or zero MI with all other dims ---
        let mut dead_dims = 0usize;
        let mut dim_variances = vec![0.0f64; d];
        let mut dim_means = vec![0.0f64; d];

        for dim in 0..d {
            let mut sum = 0.0f64;
            for i in 0..max_n {
                sum += data[i * d + dim];
            }
            dim_means[dim] = sum / max_n as f64;
        }
        for dim in 0..d {
            let mut var_sum = 0.0f64;
            for i in 0..max_n {
                let diff = data[i * d + dim] - dim_means[dim];
                var_sum += diff * diff;
            }
            dim_variances[dim] = var_sum / max_n as f64;
        }

        for dim in 0..d {
            if dim_variances[dim] < 1e-12 {
                dead_dims += 1;
                continue;
            }
            // Also check MI: if a dim has zero MI with all other dims, it's dead
            let d_check = d.min(16);
            if dim < d_check {
                let total_mi: f64 = (0..d_check)
                    .filter(|&dj| dj != dim)
                    .map(|dj| shared.mi(dim, dj))
                    .sum();
                if total_mi < 1e-6 && dim_variances[dim] < 1e-6 {
                    dead_dims += 1;
                }
            }
        }
        let dead_dimension_fraction = dead_dims as f64 / d as f64;

        // --- Unreachable fraction: points not connected to the main cluster ---
        // Union-find on KNN graph
        let mut parent: Vec<usize> = (0..max_n).collect();
        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]);
            }
            parent[i]
        }
        fn union(parent: &mut Vec<usize>, a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb { parent[ra] = rb; }
        }

        for i in 0..max_n {
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let j = j as usize;
                if j < max_n {
                    union(&mut parent, i, j);
                }
            }
        }

        // Find largest component
        let mut component_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..max_n {
            let root = find(&mut parent, i);
            *component_sizes.entry(root).or_insert(0) += 1;
        }
        let main_cluster_size = component_sizes.values().copied().max().unwrap_or(max_n);
        let unreachable_fraction = 1.0 - (main_cluster_size as f64 / max_n as f64);

        // --- Dead allocation: rows that are near-zero (allocated but unused) ---
        let mut near_zero_rows = 0usize;
        for i in 0..max_n {
            let row_norm: f64 = (0..d)
                .map(|dim| data[i * d + dim] * data[i * d + dim])
                .sum::<f64>().sqrt();
            if row_norm < 1e-8 {
                near_zero_rows += 1;
            }
        }
        let dead_allocation_score = near_zero_rows as f64 / max_n as f64;

        // --- Zero impact score: fraction removable without information loss ---
        // Combine dead dimensions + dead allocations + unreachable
        let zero_impact = (dead_dimension_fraction * 0.4
            + dead_allocation_score * 0.3
            + unreachable_fraction * 0.3)
            .min(1.0);

        // --- Branch liveness: fraction of data on "live" paths ---
        // Points with high KNN density are on live paths; isolated points are dead branches
        let mut densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
        densities.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        // Median density as threshold
        let median_density = if densities.is_empty() { 0.0 }
            else { densities[densities.len() / 2] };
        let live_count = densities.iter().filter(|&&d| d >= median_density * 0.5).count();
        let branch_liveness = live_count as f64 / max_n as f64;

        // --- Effective dimensionality via variance explained ---
        // Sort dimensions by variance; count how many are needed for 95% of total variance
        let total_var: f64 = dim_variances.iter().sum();
        let mut sorted_vars: Vec<f64> = dim_variances.clone();
        sorted_vars.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        let effective_d = if total_var < 1e-12 { 1.0 } else {
            let mut cumulative = 0.0f64;
            let mut count = 0usize;
            for &v in &sorted_vars {
                cumulative += v;
                count += 1;
                if cumulative / total_var >= 0.95 { break; }
            }
            count as f64
        };

        let mut result = HashMap::new();
        result.insert("dead_dimension_fraction".to_string(), vec![dead_dimension_fraction]);
        result.insert("unreachable_fraction".to_string(), vec![unreachable_fraction]);
        result.insert("dead_allocation_score".to_string(), vec![dead_allocation_score]);
        result.insert("zero_impact_score".to_string(), vec![zero_impact]);
        result.insert("branch_liveness".to_string(), vec![branch_liveness]);
        result.insert("effective_dimensionality".to_string(), vec![effective_d]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_dead_code_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = DeadCodeLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("dead_dimension_fraction"));
        assert!(r.contains_key("effective_dimensionality"));
    }

    #[test]
    fn test_dead_code_constant_dim() {
        // One dim varies, one is constant = 50% dead
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(i as f64); // varies
            data.push(5.0);      // constant (dead)
        }
        let shared = SharedData::compute(&data, 20, 2);
        let r = DeadCodeLens.scan(&data, 20, 2, &shared);
        assert!(r["dead_dimension_fraction"][0] >= 0.5);
    }

    #[test]
    fn test_dead_code_zero_rows() {
        // Half zero rows = dead allocations
        let mut data = Vec::new();
        for i in 0..10 { data.push(i as f64 + 1.0); data.push(i as f64 + 1.0); }
        for _ in 0..10 { data.push(0.0); data.push(0.0); } // dead
        let shared = SharedData::compute(&data, 20, 2);
        let r = DeadCodeLens.scan(&data, 20, 2, &shared);
        assert!(r["dead_allocation_score"][0] >= 0.4);
    }
}
