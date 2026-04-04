use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SingularityLens: Singularity/infinity point detection.
/// Detects density spikes, curvature divergence, resolution limits.
pub struct SingularityLens;

impl Lens for SingularityLens {
    fn name(&self) -> &str { "SingularityLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Compute local densities via kNN
        let densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();

        let mean_density = densities.iter().sum::<f64>() / max_n as f64;
        let median_density = {
            let mut sorted = densities.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());
            sorted[max_n / 2]
        };

        // Singularity count: points with density > 5x median
        let spike_threshold = median_density * 5.0;
        let singularity_count = densities.iter().filter(|&&d| d > spike_threshold).count();

        // Singularity strength: peak / median
        let peak_density = densities.iter().cloned().fold(0.0f64, f64::max);
        let singularity_strength = if median_density > 1e-12 {
            peak_density / median_density
        } else {
            0.0
        };

        // Curvature divergence: second derivative analog via distance differences
        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let mut max_curvature = 0.0f64;
        for i in 1..(max_n - 1) {
            let curvature = (signal[i + 1] - 2.0 * signal[i] + signal[i - 1]).abs();
            if curvature > max_curvature { max_curvature = curvature; }
        }
        let mean_abs = signal.iter().map(|x| x.abs()).sum::<f64>() / max_n as f64;
        let curvature_divergence = if mean_abs > 1e-12 {
            max_curvature / mean_abs
        } else {
            max_curvature
        };

        // Resolution limit: smallest distinct distance between consecutive sorted points
        let mut sorted_sig = signal.clone();
        sorted_sig.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let mut min_gap = f64::MAX;
        for i in 1..sorted_sig.len() {
            let gap = sorted_sig[i] - sorted_sig[i - 1];
            if gap > 1e-15 && gap < min_gap { min_gap = gap; }
        }
        let resolution_limit = if min_gap < f64::MAX { min_gap } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("singularity_count".to_string(), vec![singularity_count as f64]);
        result.insert("singularity_strength".to_string(), vec![singularity_strength]);
        result.insert("curvature_divergence".to_string(), vec![curvature_divergence]);
        result.insert("resolution_limit".to_string(), vec![resolution_limit]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Data with a spike at index 10
        let mut data: Vec<f64> = (0..30).map(|i| (i as f64 * 0.1).sin()).collect();
        data[10] = 100.0; // singularity
        let shared = SharedData::compute(&data, 30, 1);
        let result = SingularityLens.scan(&data, 30, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["singularity_strength"][0] > 1.0);
    }
}
