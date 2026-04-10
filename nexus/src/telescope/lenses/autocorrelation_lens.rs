use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// AutocorrelationLens: Temporal autocorrelation analysis.
///
/// Computes autocorrelation at multiple lags, detecting periodicity
/// and memory effects. Checks for n=6 period signatures.
pub struct AutocorrelationLens;

impl Lens for AutocorrelationLens {
    fn name(&self) -> &str { "AutocorrelationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let max_lag = (n / 2).min(24); // Cap at J2=24

        let mut acf_first = Vec::with_capacity(max_lag);
        let mut best_period = 0.0;
        let mut best_acf_peak = 0.0;

        // Compute ACF for first feature
        if let Some(col) = columns.first() {
            let mean = col.iter().sum::<f64>() / n as f64;
            let var = col.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
            if var < 1e-12 { return HashMap::new(); }

            for lag in 1..=max_lag {
                let mut cov = 0.0;
                let count = n - lag;
                for i in 0..count {
                    cov += (col[i] - mean) * (col[i + lag] - mean);
                }
                cov /= count as f64;
                let acf = cov / var;
                acf_first.push(acf);

                // Detect peak (positive autocorrelation after initial decay)
                if lag >= 2 && acf > best_acf_peak && acf > 0.0 {
                    best_acf_peak = acf;
                    best_period = lag as f64;
                }
            }
        }

        // Check if detected period matches n=6 constants
        let n6_periods = [2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 24.0];
        let mut period_match = 0.0;
        for &p in &n6_periods {
            if (best_period - p).abs() < 0.5 {
                period_match = 1.0;
                break;
            }
        }

        // Decorrelation length: first lag where ACF drops below 1/e
        let decorrelation_lag = acf_first.iter()
            .position(|&a| a < 1.0 / std::f64::consts::E)
            .map(|i| (i + 1) as f64)
            .unwrap_or(max_lag as f64);

        let mut result = HashMap::new();
        result.insert("acf".to_string(), acf_first);
        result.insert("best_period".to_string(), vec![best_period]);
        result.insert("best_acf_peak".to_string(), vec![best_acf_peak]);
        result.insert("n6_period_match".to_string(), vec![period_match]);
        result.insert("decorrelation_lag".to_string(), vec![decorrelation_lag]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acf_periodic() {
        // Periodic signal with period 6
        let data: Vec<f64> = (0..48)
            .map(|i| (2.0 * std::f64::consts::PI * i as f64 / 6.0).sin())
            .collect();
        let n = 48;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = AutocorrelationLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("best_period"));
        let period = result["best_period"][0];
        assert!((period - 6.0).abs() < 1.5, "Should detect period ~6, got {}", period);
        assert_eq!(result["n6_period_match"][0], 1.0, "Period 6 should match n=6");
    }

    #[test]
    fn test_acf_random() {
        // White noise should have low autocorrelation
        let mut rng = 12345u32;
        let data: Vec<f64> = (0..48).map(|_| {
            rng ^= rng << 13; rng ^= rng >> 17; rng ^= rng << 5;
            rng as f64 / u32::MAX as f64
        }).collect();
        let n = 48;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = AutocorrelationLens.scan(&data, n, d, &shared);
        assert!(result["best_acf_peak"][0] < 0.5, "White noise should have low ACF peak");
    }
}
