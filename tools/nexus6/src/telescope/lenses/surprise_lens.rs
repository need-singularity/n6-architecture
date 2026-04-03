use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SurpriseLens: Quantify deviation from expectation — anomaly significance.
///
/// Algorithm:
///   1. Compute per-point local density via k-NN distance (mean dist to k neighbors)
///   2. Compute Local Outlier Factor (LOF): ratio of a point's density to its neighbors' density
///   3. Derive z-score of each point's mean k-NN distance from the global distribution
///   4. Compute per-dimension surprise via deviation from Gaussian expectation (excess kurtosis)
///
/// Outputs:
///   - "surprise_scores": per-point surprise (LOF-based), length = n
///   - "z_anomaly": per-point z-score of mean neighbor distance, length = n
///   - "dimension_kurtosis": per-dimension excess kurtosis, length = d
///   - "summary": [mean_surprise, max_surprise, fraction_anomalous (|z| > 2)]
pub struct UsurpriseLens;

impl Lens for UsurpriseLens {
    fn name(&self) -> &str {
        "UsurpriseLens"
    }

    fn category(&self) -> &str {
        "anomaly"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let k = shared.knn_k.min(n - 1).max(1);

        // Step 1: per-point mean k-NN distance (local reachability distance)
        let mut mean_knn_dist = vec![0.0f64; n];
        for i in 0..n {
            let neighbors = shared.knn(i);
            let count = neighbors.len().min(k);
            if count == 0 {
                continue;
            }
            let mut sum = 0.0;
            for idx in 0..count {
                let j = neighbors[idx] as usize;
                sum += shared.dist(i, j);
            }
            mean_knn_dist[i] = sum / count as f64;
        }

        // Step 2: Local Outlier Factor (LOF)
        // LOF(i) = mean over neighbors j of [ mean_knn_dist(i) / mean_knn_dist(j) ]
        // High LOF => point is in a sparser region than its neighbors => surprising
        let mut surprise_scores = vec![0.0f64; n];
        for i in 0..n {
            let neighbors = shared.knn(i);
            let count = neighbors.len().min(k);
            if count == 0 {
                surprise_scores[i] = 1.0;
                continue;
            }
            let my_dist = mean_knn_dist[i];
            let mut ratio_sum = 0.0;
            for idx in 0..count {
                let j = neighbors[idx] as usize;
                let neighbor_dist = mean_knn_dist[j];
                if neighbor_dist > 1e-15 {
                    ratio_sum += my_dist / neighbor_dist;
                } else {
                    ratio_sum += if my_dist > 1e-15 { 10.0 } else { 1.0 };
                }
            }
            surprise_scores[i] = ratio_sum / count as f64;
        }

        // Step 3: z-score of mean k-NN distance from global distribution
        let global_mean = mean_knn_dist.iter().sum::<f64>() / n as f64;
        let global_var = mean_knn_dist
            .iter()
            .map(|&x| (x - global_mean) * (x - global_mean))
            .sum::<f64>()
            / n as f64;
        let global_std = global_var.sqrt().max(1e-15);

        let z_anomaly: Vec<f64> = mean_knn_dist
            .iter()
            .map(|&x| (x - global_mean) / global_std)
            .collect();

        // Step 4: per-dimension excess kurtosis (deviation from Gaussian)
        // kurtosis = E[(X-mu)^4] / sigma^4 - 3
        let mut dimension_kurtosis = vec![0.0f64; d];
        for dim in 0..d {
            let mut sum = 0.0;
            let mut sum2 = 0.0;
            for i in 0..n {
                let v = data[i * d + dim];
                sum += v;
                sum2 += v * v;
            }
            let mean = sum / n as f64;
            let var = (sum2 / n as f64) - mean * mean;
            let std = var.sqrt();
            if std < 1e-15 {
                dimension_kurtosis[dim] = 0.0;
                continue;
            }
            let mut m4 = 0.0;
            for i in 0..n {
                let v = (data[i * d + dim] - mean) / std;
                m4 += v * v * v * v;
            }
            m4 /= n as f64;
            dimension_kurtosis[dim] = m4 - 3.0; // excess kurtosis
        }

        // Summary statistics
        let mean_surprise = surprise_scores.iter().sum::<f64>() / n as f64;
        let max_surprise = surprise_scores
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let anomalous_count = z_anomaly.iter().filter(|&&z| z.abs() > 2.0).count();
        let fraction_anomalous = anomalous_count as f64 / n as f64;

        let mut result = HashMap::new();
        result.insert("surprise_scores".to_string(), surprise_scores);
        result.insert("z_anomaly".to_string(), z_anomaly);
        result.insert("dimension_kurtosis".to_string(), dimension_kurtosis);
        result.insert(
            "summary".to_string(),
            vec![mean_surprise, max_surprise, fraction_anomalous],
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_surprise_scan_basic() {
        // 4 clustered points + 1 outlier
        let data = vec![
            0.0, 0.0, // point 0: cluster
            1.0, 0.0, // point 1: cluster
            0.0, 1.0, // point 2: cluster
            1.0, 1.0, // point 3: cluster
            20.0, 20.0, // point 4: outlier
        ];
        let n = 5;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UsurpriseLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        assert!(result.contains_key("surprise_scores"));
        assert!(result.contains_key("z_anomaly"));
        assert!(result.contains_key("dimension_kurtosis"));
        assert!(result.contains_key("summary"));

        let scores = &result["surprise_scores"];
        assert_eq!(scores.len(), n);

        // The outlier (point 4) should have a higher surprise than cluster points
        let outlier_surprise = scores[4];
        let cluster_mean = (scores[0] + scores[1] + scores[2] + scores[3]) / 4.0;
        assert!(
            outlier_surprise > cluster_mean,
            "outlier surprise ({}) should exceed cluster mean ({})",
            outlier_surprise,
            cluster_mean
        );

        let z = &result["z_anomaly"];
        assert!(z[4] > 1.0, "outlier z-score should be high, got {}", z[4]);
    }

    #[test]
    fn test_surprise_uniform_cluster() {
        // All points close together — low surprise expected
        let data = vec![
            1.0, 1.0, 1.1, 1.0, 1.0, 1.1, 1.1, 1.1,
        ];
        let n = 4;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UsurpriseLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");

        let summary = &result["summary"];
        assert_eq!(summary.len(), 3);
        let mean_surprise = summary[0];
        // In a uniform cluster, all LOF scores should be close to 1.0
        assert!(
            (mean_surprise - 1.0).abs() < 0.5,
            "uniform cluster mean surprise should be ~1.0, got {}",
            mean_surprise
        );

        let kurtosis = &result["dimension_kurtosis"];
        assert_eq!(kurtosis.len(), d);
    }

    #[test]
    fn test_surprise_too_few_points() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let n = 2;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UsurpriseLens;
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.is_empty(), "n<3 should return empty");
    }
}
