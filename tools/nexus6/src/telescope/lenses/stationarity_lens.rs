use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// StationarityLens: Detect non-stationarity in sequential data.
///
/// Splits data into windows and compares statistics between windows
/// to detect regime changes and drift patterns.
pub struct StationarityLens;

impl Lens for StationarityLens {
    fn name(&self) -> &str { "StationarityLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let n_windows = 4; // tau=4 windows
        let window_size = n / n_windows;
        if window_size < 2 { return HashMap::new(); }

        let mut drift_scores = Vec::with_capacity(d);
        let mut variance_ratios = Vec::with_capacity(d);

        for col in columns.iter().take(d.min(12)) {
            // Compute mean per window
            let mut window_means = Vec::with_capacity(n_windows);
            let mut window_vars = Vec::with_capacity(n_windows);
            for w in 0..n_windows {
                let start = w * window_size;
                let end = if w == n_windows - 1 { n } else { (w + 1) * window_size };
                let slice = &col[start..end];
                let mean = slice.iter().sum::<f64>() / slice.len() as f64;
                let var = slice.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / slice.len() as f64;
                window_means.push(mean);
                window_vars.push(var);
            }

            // Drift = variance of window means / overall variance
            let overall_mean = col.iter().sum::<f64>() / n as f64;
            let overall_var = col.iter().map(|v| (v - overall_mean).powi(2)).sum::<f64>() / n as f64;

            let mean_drift = if window_means.len() >= 2 {
                let wm_mean = window_means.iter().sum::<f64>() / window_means.len() as f64;
                window_means.iter().map(|m| (m - wm_mean).powi(2)).sum::<f64>() / window_means.len() as f64
            } else { 0.0 };

            let drift = if overall_var > 1e-12 { mean_drift / overall_var } else { 0.0 };
            drift_scores.push(drift);

            // Variance ratio: max window var / min window var
            let max_var = window_vars.iter().cloned().fold(0.0_f64, f64::max);
            let min_var = window_vars.iter().cloned().fold(f64::INFINITY, f64::min);
            let vr = if min_var > 1e-12 { max_var / min_var } else { max_var };
            variance_ratios.push(vr);
        }

        let mean_drift = drift_scores.iter().sum::<f64>() / drift_scores.len().max(1) as f64;
        let mean_var_ratio = variance_ratios.iter().sum::<f64>() / variance_ratios.len().max(1) as f64;
        let is_stationary = mean_drift < 0.1 && mean_var_ratio < 3.0;

        let mut result = HashMap::new();
        result.insert("drift_scores".to_string(), drift_scores);
        result.insert("variance_ratios".to_string(), variance_ratios);
        result.insert("mean_drift".to_string(), vec![mean_drift]);
        result.insert("mean_variance_ratio".to_string(), vec![mean_var_ratio]);
        result.insert("is_stationary".to_string(), vec![if is_stationary { 1.0 } else { 0.0 }]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stationarity_constant() {
        let data: Vec<f64> = vec![5.0; 24]; // n=24=J2 constant values
        let n = 24;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = StationarityLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("is_stationary"));
        assert_eq!(result["is_stationary"][0], 1.0, "Constant data should be stationary");
    }

    #[test]
    fn test_stationarity_drift() {
        // Data with clear mean drift
        let mut data = Vec::new();
        for i in 0..24 {
            data.push(i as f64 * 10.0); // Strong upward trend
        }
        let n = 24;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = StationarityLens.scan(&data, n, d, &shared);
        assert!(result["mean_drift"][0] > 0.01, "Trending data should show drift");
    }
}
