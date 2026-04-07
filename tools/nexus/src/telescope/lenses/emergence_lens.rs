use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// EmergenceLens: Identify macro-patterns arising from micro-level interactions.
///
/// Algorithm:
///   1. Compute local density per point via k-NN (inverse distance to k-th neighbor)
///   2. Build density gradient flow: each point flows to its nearest higher-density neighbor
///   3. Identify attractors (density peaks that flow to themselves)
///   4. Count emergent clusters and measure their coherence
///   5. Emergence score = inter-cluster variance / total variance (how much structure
///      arises beyond uniform distribution)
///   6. Emergence entropy = Shannon entropy of cluster size distribution (organizational complexity)
pub struct UemergenceLens;

impl Lens for UemergenceLens {
    fn name(&self) -> &str {
        "UemergenceLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // --- Step 1: Local density via k-NN ---
        let densities: Vec<f64> = (0..n).map(|i| shared.knn_density(i)).collect();

        // --- Step 2: Density gradient flow ---
        // Each point flows to its nearest neighbor with strictly higher density.
        // If no such neighbor exists, the point is an attractor (flows to itself).
        let mut flow_to: Vec<usize> = vec![0; n];
        for i in 0..n {
            let mut best_j = i; // default: self (attractor)
            let mut best_dist = f64::MAX;
            for j in 0..n {
                if j == i {
                    continue;
                }
                if densities[j] > densities[i] {
                    let dij = shared.dist(i, j);
                    if dij < best_dist {
                        best_dist = dij;
                        best_j = j;
                    }
                }
            }
            flow_to[i] = best_j;
        }

        // --- Step 3: Follow flow chains to find attractor for each point ---
        let mut attractor: Vec<usize> = vec![0; n];
        for i in 0..n {
            let mut cur = i;
            let mut steps = 0;
            while flow_to[cur] != cur && steps < n {
                cur = flow_to[cur];
                steps += 1;
            }
            attractor[i] = cur;
        }

        // --- Step 4: Build clusters from attractors ---
        // Map attractor index -> cluster id
        let mut attractor_to_cluster: HashMap<usize, usize> = HashMap::new();
        let mut cluster_id: Vec<usize> = vec![0; n];
        let mut next_cluster = 0usize;
        for i in 0..n {
            let a = attractor[i];
            let cid = attractor_to_cluster.entry(a).or_insert_with(|| {
                let c = next_cluster;
                next_cluster += 1;
                c
            });
            cluster_id[i] = *cid;
        }
        let num_clusters = next_cluster;

        // Cluster sizes
        let mut cluster_sizes: Vec<usize> = vec![0; num_clusters];
        for i in 0..n {
            cluster_sizes[cluster_id[i]] += 1;
        }

        // --- Step 5: Emergence score (inter-cluster / total variance of distances) ---
        // Compute global mean distance
        let pair_count = n * (n - 1) / 2;
        let mut global_sum = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                global_sum += shared.dist(i, j);
            }
        }
        let global_mean = global_sum / pair_count as f64;

        // Total variance of pairwise distances
        let mut total_var = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                let diff = shared.dist(i, j) - global_mean;
                total_var += diff * diff;
            }
        }
        total_var /= pair_count as f64;

        // Inter-cluster mean distances (between cluster centroids approximated by mean pairwise)
        let mut inter_var = 0.0;
        let mut inter_count = 0;
        let mut cluster_mean_dists: Vec<f64> = Vec::new();
        for ca in 0..num_clusters {
            for cb in (ca + 1)..num_clusters {
                let mut sum = 0.0;
                let mut cnt = 0;
                for i in 0..n {
                    if cluster_id[i] != ca {
                        continue;
                    }
                    for j in 0..n {
                        if cluster_id[j] != cb {
                            continue;
                        }
                        if i != j {
                            sum += shared.dist(i, j);
                            cnt += 1;
                        }
                    }
                }
                if cnt > 0 {
                    cluster_mean_dists.push(sum / cnt as f64);
                }
            }
        }

        if cluster_mean_dists.len() >= 2 {
            let cmean =
                cluster_mean_dists.iter().sum::<f64>() / cluster_mean_dists.len() as f64;
            for &v in &cluster_mean_dists {
                let diff = v - cmean;
                inter_var += diff * diff;
                inter_count += 1;
            }
            if inter_count > 0 {
                inter_var /= inter_count as f64;
            }
        }

        // Emergence score: how much macro-structure exists relative to total spread
        let emergence_score = if total_var > 1e-15 {
            (inter_var / total_var).sqrt().min(1.0)
        } else {
            0.0
        };

        // --- Step 6: Emergence entropy (Shannon entropy of cluster size distribution) ---
        let n_f = n as f64;
        let mut emergence_entropy = 0.0;
        for &sz in &cluster_sizes {
            if sz > 0 {
                let p = sz as f64 / n_f;
                emergence_entropy -= p * p.ln();
            }
        }

        // --- Step 7: Density contrast (max density / min density) ---
        let d_min = densities
            .iter()
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
            .fold(f64::MAX, f64::min);
        let d_max = densities
            .iter()
            .copied()
            .filter(|v| v.is_finite())
            .fold(0.0_f64, f64::max);
        let density_contrast = if d_min > 1e-15 && d_max.is_finite() {
            (d_max / d_min).ln().max(0.0)
        } else {
            0.0
        };

        // --- Step 8: Mean cluster coherence (mean intra-cluster distance / global mean) ---
        let mut coherence = 0.0;
        let mut coherence_count = 0;
        for c in 0..num_clusters {
            let members: Vec<usize> = (0..n).filter(|&i| cluster_id[i] == c).collect();
            if members.len() < 2 {
                continue;
            }
            let mut intra_sum = 0.0;
            let mut intra_cnt = 0;
            for mi in 0..members.len() {
                for mj in (mi + 1)..members.len() {
                    intra_sum += shared.dist(members[mi], members[mj]);
                    intra_cnt += 1;
                }
            }
            if intra_cnt > 0 {
                coherence += (intra_sum / intra_cnt as f64) / global_mean.max(1e-15);
                coherence_count += 1;
            }
        }
        let mean_coherence = if coherence_count > 0 {
            coherence / coherence_count as f64
        } else {
            1.0
        };

        let mut result = HashMap::new();
        result.insert("emergence_score".to_string(), vec![emergence_score]);
        result.insert("num_clusters".to_string(), vec![num_clusters as f64]);
        result.insert("emergence_entropy".to_string(), vec![emergence_entropy]);
        result.insert("density_contrast".to_string(), vec![density_contrast]);
        result.insert("mean_coherence".to_string(), vec![mean_coherence]);
        result.insert(
            "cluster_sizes".to_string(),
            cluster_sizes.iter().map(|&s| s as f64).collect(),
        );
        result.insert("densities".to_string(), densities);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    /// Two well-separated clusters should produce non-empty results with >1 cluster.
    #[test]
    fn test_emergence_two_clusters() {
        // Cluster A near origin, Cluster B far away
        let data = vec![
            0.0, 0.0, // A
            0.1, 0.1, // A
            0.2, 0.0, // A
            10.0, 10.0, // B
            10.1, 10.1, // B
            10.2, 10.0, // B
        ];
        let n = 6;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UemergenceLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must not be empty");
        assert!(result.contains_key("emergence_score"));
        assert!(result.contains_key("num_clusters"));
        assert!(result.contains_key("emergence_entropy"));

        let score = result["emergence_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "score={score} out of [0,1]");

        let nc = result["num_clusters"][0];
        assert!(nc >= 2.0, "expected >=2 clusters for separated data, got {nc}");
    }

    /// Uniform random-ish data should still produce valid output (non-empty, finite).
    #[test]
    fn test_emergence_spread_data() {
        // Points spread in a grid-like pattern
        let data = vec![
            0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, 1.0, 1.0, 1.0, 2.0, 1.0, 0.0, 2.0, 1.0, 2.0,
            2.0, 2.0,
        ];
        let n = 9;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UemergenceLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must not be empty");

        // All values should be finite
        for (key, vals) in &result {
            for &v in vals {
                assert!(v.is_finite(), "non-finite value in key={key}: {v}");
            }
        }

        let densities = &result["densities"];
        assert_eq!(densities.len(), n, "should have one density per point");
    }

    /// Edge case: n < 3 returns empty.
    #[test]
    fn test_emergence_too_few_points() {
        let data = vec![0.0, 1.0, 2.0, 3.0];
        let n = 2;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UemergenceLens;
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.is_empty());
    }
}
