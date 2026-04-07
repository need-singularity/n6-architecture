use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SpecializationLens: Detect monomorphization/specialization opportunities.
///
/// Analyzes data to find clusters of rows that follow distinct type-like patterns,
/// suggesting that generic code paths could be specialized for each cluster's
/// characteristics (analogous to template specialization / monomorphization).
///
/// Metrics:
///   1. cluster_count: number of distinct behavioral clusters detected
///   2. specialization_gain: variance reduction from cluster-specific handling
///   3. type_purity: fraction of rows that fit cleanly into one cluster
///   4. polymorphism_cost: overhead of generic (non-specialized) processing
///   5. dominant_cluster_ratio: size of largest cluster / total (skew indicator)
pub struct SpecializationLens;

impl Lens for SpecializationLens {
    fn name(&self) -> &str { "SpecializationLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Simple k-means-like clustering with k=min(6, sqrt(n))
        let k = (max_n as f64).sqrt().ceil().min(6.0) as usize;
        let k = k.max(2);

        // Initialize centroids from evenly spaced data points
        let mut centroids: Vec<Vec<f64>> = Vec::with_capacity(k);
        for ci in 0..k {
            let idx = (ci * max_n / k).min(max_n - 1);
            centroids.push(data[idx * d..(idx * d + d)].to_vec());
        }

        // Run 10 iterations of assignment + update
        let mut assignments = vec![0usize; max_n];
        for _iter in 0..10 {
            // Assign each point to nearest centroid
            for i in 0..max_n {
                let mut best_c = 0;
                let mut best_dist = f64::MAX;
                for (c, centroid) in centroids.iter().enumerate() {
                    let mut dist = 0.0;
                    for j in 0..d {
                        let diff = data[i * d + j] - centroid[j];
                        dist += diff * diff;
                    }
                    if dist < best_dist {
                        best_dist = dist;
                        best_c = c;
                    }
                }
                assignments[i] = best_c;
            }
            // Update centroids
            let mut new_centroids = vec![vec![0.0f64; d]; k];
            let mut counts = vec![0usize; k];
            for i in 0..max_n {
                let c = assignments[i];
                counts[c] += 1;
                for j in 0..d {
                    new_centroids[c][j] += data[i * d + j];
                }
            }
            for c in 0..k {
                if counts[c] > 0 {
                    for j in 0..d {
                        new_centroids[c][j] /= counts[c] as f64;
                    }
                    centroids[c] = new_centroids[c].clone();
                }
            }
        }

        // Count non-empty clusters
        let mut cluster_sizes = vec![0usize; k];
        for &a in &assignments[..max_n] {
            cluster_sizes[a] += 1;
        }
        let cluster_count = cluster_sizes.iter().filter(|&&s| s > 0).count();

        // Global variance
        let mut global_mean = vec![0.0f64; d];
        for i in 0..max_n {
            for j in 0..d {
                global_mean[j] += data[i * d + j];
            }
        }
        for j in 0..d { global_mean[j] /= max_n as f64; }
        let global_var: f64 = (0..max_n).map(|i| {
            (0..d).map(|j| (data[i * d + j] - global_mean[j]).powi(2)).sum::<f64>()
        }).sum::<f64>() / max_n as f64;

        // Within-cluster variance
        let within_var: f64 = (0..max_n).map(|i| {
            let c = assignments[i];
            (0..d).map(|j| (data[i * d + j] - centroids[c][j]).powi(2)).sum::<f64>()
        }).sum::<f64>() / max_n as f64;

        // Specialization gain: variance reduction
        let specialization_gain = if global_var > 1e-15 {
            1.0 - within_var / global_var
        } else {
            0.0
        };

        // Type purity: fraction of points within 1 std of their centroid
        let within_std = within_var.sqrt();
        let pure_count = (0..max_n).filter(|&i| {
            let c = assignments[i];
            let dist: f64 = (0..d).map(|j| (data[i * d + j] - centroids[c][j]).powi(2)).sum();
            dist.sqrt() <= within_std * 1.5 + 1e-15
        }).count();
        let type_purity = pure_count as f64 / max_n as f64;

        // Polymorphism cost: normalized within-cluster variance (higher = more expensive generic path)
        let polymorphism_cost = if global_var > 1e-15 { within_var / global_var } else { 0.0 };

        // Dominant cluster ratio
        let max_cluster = *cluster_sizes.iter().max().unwrap_or(&0);
        let dominant_cluster_ratio = max_cluster as f64 / max_n as f64;

        let mut result = HashMap::new();
        result.insert("cluster_count".to_string(), vec![cluster_count as f64]);
        result.insert("specialization_gain".to_string(), vec![specialization_gain]);
        result.insert("type_purity".to_string(), vec![type_purity]);
        result.insert("polymorphism_cost".to_string(), vec![polymorphism_cost]);
        result.insert("dominant_cluster_ratio".to_string(), vec![dominant_cluster_ratio]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_distinct_clusters() {
        // Two well-separated clusters
        let mut data = Vec::new();
        for _ in 0..10 { data.push(0.0); data.push(0.0); }
        for _ in 0..10 { data.push(100.0); data.push(100.0); }
        let shared = SharedData::compute(&data, 20, 2);
        let result = SpecializationLens.scan(&data, 20, 2, &shared);
        assert!(result["specialization_gain"][0] > 0.5);
        assert!(result["cluster_count"][0] >= 2.0);
    }

    #[test]
    fn test_uniform_data_low_gain() {
        // All same = no specialization benefit
        let data = vec![5.0; 60];
        let shared = SharedData::compute(&data, 20, 3);
        let result = SpecializationLens.scan(&data, 20, 3, &shared);
        assert!(result["specialization_gain"][0] < 0.1);
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let result = SpecializationLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty());
    }
}
