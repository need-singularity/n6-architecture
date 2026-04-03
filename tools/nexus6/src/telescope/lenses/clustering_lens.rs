use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ClusteringLens: Detect natural cluster structure via gap statistics.
///
/// Algorithm:
///   1. Hierarchical single-linkage clustering via sorted pairwise distances
///   2. Detect optimal cluster count from distance gaps
///   3. Check if cluster count matches n=6 constants (n, tau, phi, sigma)
///   4. Compute silhouette-like quality metric
pub struct ClusteringLens;

const N6_CLUSTER_COUNTS: &[(f64, &str)] = &[
    (2.0, "phi=2"), (3.0, "n/phi=3"), (4.0, "tau=4"),
    (5.0, "sopfr=5"), (6.0, "n=6"), (8.0, "sigma-tau=8"),
    (12.0, "sigma=12"), (24.0, "J2=24"),
];

impl Lens for ClusteringLens {
    fn name(&self) -> &str { "ClusteringLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        // Collect sorted NN distances for gap analysis
        let mut nn_dists: Vec<f64> = (0..n).map(|i| {
            let mut min_d = f64::MAX;
            for j in 0..n {
                if j != i {
                    let dd = shared.dist(i, j);
                    if dd < min_d { min_d = dd; }
                }
            }
            min_d
        }).collect();
        nn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Find largest gap in sorted NN distances -> natural cluster boundary
        let mut max_gap = 0.0;
        let mut gap_idx = 0;
        for i in 1..nn_dists.len() {
            let gap = nn_dists[i] - nn_dists[i - 1];
            if gap > max_gap {
                max_gap = gap;
                gap_idx = i;
            }
        }

        // Estimate cluster count from gap position
        // Points below gap are intra-cluster, above are inter-cluster
        let threshold = if gap_idx > 0 {
            (nn_dists[gap_idx - 1] + nn_dists[gap_idx.min(nn_dists.len() - 1)]) / 2.0
        } else {
            nn_dists[n / 2]
        };

        // Simple connected-components clustering with threshold
        let mut labels = vec![0usize; n];
        let mut next_label = 0;
        let mut assigned = vec![false; n];

        for seed in 0..n {
            if assigned[seed] { continue; }
            let label = next_label;
            next_label += 1;
            let mut stack = vec![seed];
            while let Some(cur) = stack.pop() {
                if assigned[cur] { continue; }
                assigned[cur] = true;
                labels[cur] = label;
                for j in 0..n {
                    if !assigned[j] && shared.dist(cur, j) <= threshold {
                        stack.push(j);
                    }
                }
            }
        }

        let cluster_count = next_label;

        // Match cluster count to n=6 constants
        let mut best = ("none", f64::MAX);
        for &(c, name) in N6_CLUSTER_COUNTS {
            let dist = (cluster_count as f64 - c).abs();
            if dist < best.1 { best = (name, dist); }
        }
        let n6_cluster_match = (-best.1 * 0.5).exp();

        // Silhouette-like score
        let mut sil_sum = 0.0;
        for i in 0..n {
            let li = labels[i];
            let mut intra_sum = 0.0;
            let mut intra_count = 0;
            let mut inter_min = f64::MAX;

            for j in 0..n {
                if i == j { continue; }
                let dd = shared.dist(i, j);
                if labels[j] == li {
                    intra_sum += dd;
                    intra_count += 1;
                } else if dd < inter_min {
                    inter_min = dd;
                }
            }

            let a = if intra_count > 0 { intra_sum / intra_count as f64 } else { 0.0 };
            let b = if inter_min < f64::MAX { inter_min } else { a };
            let denom = a.max(b).max(1e-12);
            sil_sum += (b - a) / denom;
        }
        let silhouette = sil_sum / n as f64;

        let mut result = HashMap::new();
        result.insert("cluster_count".to_string(), vec![cluster_count as f64]);
        result.insert("n6_cluster_match".to_string(), vec![n6_cluster_match]);
        result.insert("silhouette".to_string(), vec![silhouette]);
        result.insert("max_gap".to_string(), vec![max_gap]);
        result.insert("cluster_labels".to_string(), labels.iter().map(|&l| l as f64).collect());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clustering_two_clusters() {
        let mut data = Vec::new();
        for i in 0..6 { data.push(i as f64); data.push(0.0); }
        for i in 0..6 { data.push(100.0 + i as f64); data.push(0.0); }
        let n = 12;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = ClusteringLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("cluster_count"));
        let cc = result["cluster_count"][0];
        assert!(cc >= 2.0, "Should detect at least 2 clusters, got {}", cc);
    }
}
