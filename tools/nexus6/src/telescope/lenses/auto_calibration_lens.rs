use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy, mean_var, min_max};

/// Auto-calibrates analysis parameters for optimal results.
pub struct AutoCalibrationLens;

impl Lens for AutoCalibrationLens {
    fn name(&self) -> &str { "AutoCalibrationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Optimal k for KNN: test k=2..sqrt(n), pick max entropy of density
        let k_max = ((max_n as f64).sqrt() as usize).max(3).min(20);
        let mut best_k = 2_usize;
        let mut best_k_entropy = 0.0_f64;
        for k in 2..=k_max {
            // Approximate: use existing KNN but measure density spread
            let densities: Vec<f64> = (0..max_n).map(|i| {
                let knn = shared.knn(i);
                let k_actual = knn.len().min(k);
                if k_actual == 0 { return 0.0; }
                let sum_dist: f64 = knn[..k_actual].iter()
                    .map(|&j| shared.dist(i, j as usize))
                    .sum();
                1.0 / (sum_dist / k_actual as f64 + 1e-12)
            }).collect();
            let ent = shannon_entropy(&densities, 16);
            if ent > best_k_entropy {
                best_k_entropy = ent;
                best_k = k;
            }
        }

        // Optimal bins: Sturges' rule vs sqrt rule, pick higher entropy
        let sturges = (1.0 + (max_n as f64).log2()).ceil() as usize;
        let sqrt_bins = (max_n as f64).sqrt().ceil() as usize;
        let flat_data: Vec<f64> = data[..max_n * d].to_vec();
        let ent_sturges = shannon_entropy(&flat_data, sturges.max(2));
        let ent_sqrt = shannon_entropy(&flat_data, sqrt_bins.max(2));
        let optimal_bins = if ent_sqrt > ent_sturges { sqrt_bins } else { sturges };

        // Optimal threshold: median distance
        let mut dists = Vec::with_capacity(max_n.min(50) * (max_n.min(50) - 1) / 2);
        let sample = max_n.min(50);
        for i in 0..sample {
            for j in (i + 1)..sample {
                dists.push(shared.dist(i, j));
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let optimal_threshold = dists.get(dists.len() / 2).copied().unwrap_or(1.0);

        // Signal-to-noise ratio
        let (means, vars) = mean_var(data, max_n, d);
        let signal = vars.iter().sum::<f64>();
        // Noise estimate: mean of squared differences between adjacent points
        let noise = if max_n > 1 {
            let mut n_sum = 0.0;
            for i in 0..(max_n - 1) {
                for j in 0..d {
                    let diff = data[(i + 1) * d + j] - data[i * d + j];
                    n_sum += diff * diff;
                }
            }
            n_sum / ((max_n - 1) * d) as f64
        } else { 1.0 };
        let snr = if noise > 1e-12 { signal / noise } else { signal * 1e6 };

        // Calibration confidence: based on sample size relative to dimensionality
        let confidence = 1.0 - (d as f64 / max_n as f64).min(1.0);

        // Dynamic range
        let (lo, hi) = min_max(&flat_data);
        let dynamic_range = if lo.abs() > 1e-12 { (hi / lo).abs().log10() } else { hi.abs().log10().max(0.0) };

        result.insert("optimal_k".into(), vec![best_k as f64]);
        result.insert("optimal_bins".into(), vec![optimal_bins as f64]);
        result.insert("optimal_threshold".into(), vec![optimal_threshold]);
        result.insert("signal_to_noise".into(), vec![snr]);
        result.insert("calibration_confidence".into(), vec![confidence]);
        result.insert("dynamic_range".into(), vec![dynamic_range]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = AutoCalibrationLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("optimal_k"));
        assert!(r.contains_key("signal_to_noise"));
        assert!(*r["optimal_k"].first().unwrap() >= 2.0);
    }
}
