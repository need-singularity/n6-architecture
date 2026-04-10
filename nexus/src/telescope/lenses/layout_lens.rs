use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// LayoutLens: Struct field reordering for cache line alignment analysis.
///
/// Analyzes per-dimension access patterns to detect sub-optimal field ordering.
/// If certain dimensions are always accessed together but placed far apart,
/// a reordering could improve cache performance.
///
/// Metrics:
///   1. co_access_score: mean pairwise correlation between adjacent dimensions
///   2. padding_waste: estimated wasted cache line fraction from current ordering
///   3. optimal_reorder_gain: potential improvement from dimension reordering
///   4. hot_field_count: number of dimensions with high access variance (frequently read)
///   5. alignment_score: how well current layout fits 64-byte cache lines
pub struct LayoutLens;

impl Lens for LayoutLens {
    fn name(&self) -> &str { "LayoutLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }
        let max_n = n.min(500);

        // Per-dimension statistics
        let mut dim_mean = vec![0.0f64; d];
        let mut dim_var = vec![0.0f64; d];
        for i in 0..max_n {
            for j in 0..d {
                dim_mean[j] += data[i * d + j];
            }
        }
        for j in 0..d { dim_mean[j] /= max_n as f64; }
        for i in 0..max_n {
            for j in 0..d {
                let diff = data[i * d + j] - dim_mean[j];
                dim_var[j] += diff * diff;
            }
        }
        for j in 0..d { dim_var[j] /= max_n as f64; }

        // Hot fields: dimensions with above-mean variance (frequently varying = hot)
        let mean_var = dim_var.iter().sum::<f64>() / d as f64;
        let hot_field_count = dim_var.iter().filter(|&&v| v > mean_var).count();

        // Co-access score: correlation between adjacent dimensions
        let mut co_access_sum = 0.0f64;
        let mut co_access_count = 0usize;
        for j in 0..d - 1 {
            let std_a = dim_var[j].sqrt();
            let std_b = dim_var[j + 1].sqrt();
            if std_a < 1e-15 || std_b < 1e-15 { continue; }
            let mut cov = 0.0f64;
            for i in 0..max_n {
                cov += (data[i * d + j] - dim_mean[j]) * (data[i * d + j + 1] - dim_mean[j + 1]);
            }
            cov /= max_n as f64;
            co_access_sum += (cov / (std_a * std_b)).abs();
            co_access_count += 1;
        }
        let co_access_score = if co_access_count > 0 {
            co_access_sum / co_access_count as f64
        } else {
            0.0
        };

        // Compute full correlation matrix for reorder analysis
        // Find optimal ordering: group highly correlated dimensions together
        let mut corr_matrix = vec![vec![0.0f64; d]; d];
        for a in 0..d {
            for b in a + 1..d {
                let std_a = dim_var[a].sqrt();
                let std_b = dim_var[b].sqrt();
                if std_a < 1e-15 || std_b < 1e-15 { continue; }
                let mut cov = 0.0f64;
                for i in 0..max_n {
                    cov += (data[i * d + a] - dim_mean[a]) * (data[i * d + b] - dim_mean[b]);
                }
                cov /= max_n as f64;
                let corr = (cov / (std_a * std_b)).abs();
                corr_matrix[a][b] = corr;
                corr_matrix[b][a] = corr;
            }
        }

        // Current layout adjacency correlation sum
        let mut current_adj_corr = 0.0f64;
        for j in 0..d - 1 {
            current_adj_corr += corr_matrix[j][j + 1];
        }

        // Greedy optimal: start from highest-variance dim, always pick most-correlated next
        let mut used = vec![false; d];
        let mut order = Vec::with_capacity(d);
        // Start with highest variance dimension
        let start = dim_var.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i).unwrap_or(0);
        order.push(start);
        used[start] = true;

        for _ in 1..d {
            let last = *order.last().unwrap();
            let mut best_j = 0;
            let mut best_corr = -1.0f64;
            for j in 0..d {
                if !used[j] && corr_matrix[last][j] > best_corr {
                    best_corr = corr_matrix[last][j];
                    best_j = j;
                }
            }
            order.push(best_j);
            used[best_j] = true;
        }

        let mut optimal_adj_corr = 0.0f64;
        for i in 0..order.len() - 1 {
            optimal_adj_corr += corr_matrix[order[i]][order[i + 1]];
        }

        let optimal_reorder_gain = if current_adj_corr > 1e-15 {
            (optimal_adj_corr - current_adj_corr) / current_adj_corr
        } else if optimal_adj_corr > 1e-15 {
            1.0
        } else {
            0.0
        };
        let optimal_reorder_gain = optimal_reorder_gain.max(0.0);

        // Padding waste: assuming 64-byte (8 f64) cache lines
        let cache_line_elems = 8usize;
        let lines_needed = (d + cache_line_elems - 1) / cache_line_elems;
        let total_slots = lines_needed * cache_line_elems;
        let padding_waste = (total_slots - d) as f64 / total_slots as f64;

        // Alignment score: how well d fits cache line boundaries
        let alignment_score = if d % cache_line_elems == 0 {
            1.0
        } else {
            1.0 - padding_waste
        };

        let mut result = HashMap::new();
        result.insert("co_access_score".to_string(), vec![co_access_score]);
        result.insert("padding_waste".to_string(), vec![padding_waste]);
        result.insert("optimal_reorder_gain".to_string(), vec![optimal_reorder_gain]);
        result.insert("hot_field_count".to_string(), vec![hot_field_count as f64]);
        result.insert("alignment_score".to_string(), vec![alignment_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_correlated_adjacent_dims() {
        // Dimensions 0,1 correlated; dim 2 independent
        let mut data = Vec::new();
        for i in 0..20 {
            let x = i as f64;
            data.push(x);
            data.push(x * 1.1 + 0.5); // correlated with dim 0
            data.push(100.0 - x * 3.0); // anti-correlated
        }
        let shared = SharedData::compute(&data, 20, 3);
        let result = LayoutLens.scan(&data, 20, 3, &shared);
        assert!(result.contains_key("co_access_score"));
        assert!(result["hot_field_count"][0] >= 1.0);
    }

    #[test]
    fn test_perfect_alignment() {
        // d=8 = exactly one cache line
        let data: Vec<f64> = (0..80).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 10, 8);
        let result = LayoutLens.scan(&data, 10, 8, &shared);
        assert!((result["alignment_score"][0] - 1.0).abs() < 1e-10);
        assert!((result["padding_waste"][0]).abs() < 1e-10);
    }

    #[test]
    fn test_single_dim_returns_empty() {
        let data: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 10, 1);
        let result = LayoutLens.scan(&data, 10, 1, &shared);
        assert!(result.is_empty());
    }
}
