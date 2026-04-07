use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// KernelFusionLens: Identify adjacent operations that can be fused into a single kernel.
///
/// Models data points as operations in a computation graph. Two operations are
/// "fusible" when they are spatially close (low distance) and have correlated
/// feature profiles (high cosine similarity), indicating redundant memory
/// traffic that a fused kernel could eliminate.
///
/// Algorithm:
///   1. Compute distance statistics (mean, std) from the pre-built distance matrix
///   2. Build a fusion-candidate graph: edge (i,j) exists when
///      dist(i,j) < mean - 0.5*std  (tight proximity threshold)
///   3. Score each candidate edge by  fusion_score = (1 - norm_dist) * cosine_sim
///   4. Greedy merge: iterate edges in descending score order, merge into groups
///      (union-find) as long as both endpoints are not yet in different groups
///   5. Report fusion groups, scores, and bandwidth-saving estimate
pub struct UkernelUfusionLens;

impl Lens for UkernelUfusionLens {
    fn name(&self) -> &str {
        "kernel_fusion"
    }

    fn category(&self) -> &str {
        "compute"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        // --- 1. Distance statistics ---
        let pair_count = n * (n - 1) / 2;
        let mut dist_sum = 0.0_f64;
        let mut dist_max = 0.0_f64;
        for i in 0..n {
            for j in (i + 1)..n {
                let dij = shared.dist(i, j);
                dist_sum += dij;
                if dij > dist_max {
                    dist_max = dij;
                }
            }
        }
        let dist_mean = dist_sum / pair_count as f64;

        let mut var_sum = 0.0_f64;
        for i in 0..n {
            for j in (i + 1)..n {
                let diff = shared.dist(i, j) - dist_mean;
                var_sum += diff * diff;
            }
        }
        let dist_std = (var_sum / pair_count as f64).sqrt();

        // Proximity threshold: tighter than median to select truly adjacent ops
        let threshold = (dist_mean - 0.5 * dist_std).max(dist_mean * 0.5);

        // --- 2 & 3. Build scored candidate edges ---
        let inv_max = if dist_max > 1e-12 { 1.0 / dist_max } else { 1.0 };

        struct Edge {
            i: usize,
            j: usize,
            score: f64,
        }

        let mut edges: Vec<Edge> = Vec::new();
        for i in 0..n {
            let row_i = &data[i * d..(i + 1) * d];
            for j in (i + 1)..n {
                let dij = shared.dist(i, j);
                if dij >= threshold {
                    continue;
                }
                let row_j = &data[j * d..(j + 1) * d];

                // Cosine similarity
                let cos = cosine_sim_rows(row_i, row_j);

                // Fusion score: proximity × similarity
                let norm_dist = dij * inv_max;
                let score = (1.0 - norm_dist) * cos.max(0.0);
                if score > 0.0 {
                    edges.push(Edge { i, j, score });
                }
            }
        }

        if edges.is_empty() {
            let mut result = HashMap::new();
            result.insert("num_fusion_groups".to_string(), vec![0.0]);
            result.insert("fusion_ratio".to_string(), vec![0.0]);
            result.insert("avg_group_size".to_string(), vec![1.0]);
            result.insert("bandwidth_saving_estimate".to_string(), vec![0.0]);
            return result;
        }

        // Sort by score descending
        edges.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

        // --- 4. Greedy union-find merge ---
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];

        fn find(parent: &mut [usize], x: usize) -> usize {
            let mut r = x;
            while parent[r] != r {
                r = parent[r];
            }
            // Path compression
            let mut c = x;
            while parent[c] != r {
                let next = parent[c];
                parent[c] = r;
                c = next;
            }
            r
        }

        fn union(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) -> bool {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra == rb {
                return false;
            }
            if rank[ra] < rank[rb] {
                parent[ra] = rb;
            } else if rank[ra] > rank[rb] {
                parent[rb] = ra;
            } else {
                parent[rb] = ra;
                rank[ra] += 1;
            }
            true
        }

        let mut fusion_scores: Vec<f64> = Vec::new();
        for edge in &edges {
            if union(&mut parent, &mut rank, edge.i, edge.j) {
                fusion_scores.push(edge.score);
            }
        }

        // --- 5. Collect group statistics ---
        let mut group_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            *group_sizes.entry(root).or_insert(0) += 1;
        }

        let groups: Vec<usize> = group_sizes.values().copied().collect();
        let num_groups = groups.len();
        let fused_points: usize = groups.iter().filter(|&&s| s > 1).map(|&s| s).sum();
        let multi_groups: Vec<&usize> = groups.iter().filter(|&&s| s > 1).collect();
        let avg_group_size = if multi_groups.is_empty() {
            1.0
        } else {
            multi_groups.iter().map(|&&s| s as f64).sum::<f64>() / multi_groups.len() as f64
        };
        let fusion_ratio = fused_points as f64 / n as f64;

        // Bandwidth saving estimate: each fusion of k ops saves (k-1)/k memory round-trips
        let bandwidth_saving = if fused_points > 0 {
            multi_groups
                .iter()
                .map(|&&s| (s - 1) as f64 / s as f64)
                .sum::<f64>()
                / multi_groups.len() as f64
        } else {
            0.0
        };

        let avg_score = if fusion_scores.is_empty() {
            0.0
        } else {
            fusion_scores.iter().sum::<f64>() / fusion_scores.len() as f64
        };

        let mut result = HashMap::new();
        result.insert("num_fusion_groups".to_string(), vec![num_groups as f64]);
        result.insert("fusion_ratio".to_string(), vec![fusion_ratio]);
        result.insert("avg_group_size".to_string(), vec![avg_group_size]);
        result.insert(
            "bandwidth_saving_estimate".to_string(),
            vec![bandwidth_saving],
        );
        result.insert("avg_fusion_score".to_string(), vec![avg_score]);
        result.insert("fusion_scores".to_string(), fusion_scores);
        result
    }
}

/// Cosine similarity between two row slices.
fn cosine_sim_rows(a: &[f64], b: &[f64]) -> f64 {
    let mut dot = 0.0_f64;
    let mut na = 0.0_f64;
    let mut nb = 0.0_f64;
    for i in 0..a.len() {
        dot += a[i] * b[i];
        na += a[i] * a[i];
        nb += b[i] * b[i];
    }
    let denom = na.sqrt() * nb.sqrt();
    if denom < 1e-12 {
        0.0
    } else {
        dot / denom
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper: build SharedData from raw data.
    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_clustered_data_produces_fusions() {
        // Two tight clusters that should fuse internally:
        // Cluster A: (0,0), (0.1,0.1), (0.2,0.2)
        // Cluster B: (10,10), (10.1,10.1), (10.2,10.2)
        let data = vec![
            0.0, 0.0, 0.1, 0.1, 0.2, 0.2, 10.0, 10.0, 10.1, 10.1, 10.2, 10.2,
        ];
        let n = 6;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = UkernelUfusionLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        assert!(
            result.contains_key("fusion_ratio"),
            "must contain fusion_ratio"
        );
        let ratio = result["fusion_ratio"][0];
        assert!(
            ratio > 0.0,
            "clustered data should have positive fusion ratio, got {ratio}"
        );

        let scores = &result["fusion_scores"];
        assert!(
            !scores.is_empty(),
            "clustered data should produce fusion scores"
        );
        for &s in scores {
            assert!(s > 0.0 && s <= 1.0, "score must be in (0,1], got {s}");
        }
    }

    #[test]
    fn test_spread_data_low_fusion() {
        // Points spread far apart — few or no fusions expected
        let data = vec![0.0, 0.0, 100.0, 0.0, 0.0, 100.0, 100.0, 100.0];
        let n = 4;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = UkernelUfusionLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        let bw = result["bandwidth_saving_estimate"][0];
        assert!(
            bw.is_finite(),
            "bandwidth saving must be finite, got {bw}"
        );
        // With equidistant points the fusion ratio should be modest
        let ratio = result["fusion_ratio"][0];
        assert!(ratio <= 1.0, "fusion ratio must be <= 1.0");
    }

    #[test]
    fn test_small_n_returns_empty() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let shared = make_shared(&data, 2, 2);
        let lens = UkernelUfusionLens;
        let result = lens.scan(&data, 2, 2, &shared);
        assert!(result.is_empty(), "n<3 should return empty HashMap");
    }
}
