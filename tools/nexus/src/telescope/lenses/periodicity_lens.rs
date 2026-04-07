use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PeriodicityLens: Detect repeating cycles and periodic structures in data.
///
/// Algorithm:
///   1. Autocorrelation: For each lag k, compute normalized autocorrelation
///      across all dimensions to find repeating patterns.
///   2. Peak detection: Find local maxima in the autocorrelation function
///      to identify dominant periods.
///   3. Periodicity strength: Ratio of dominant peak to baseline,
///      measuring how strongly periodic the signal is.
///   4. Distance-based recurrence: Count near-recurrences in the
///      pairwise distance matrix at multiples of detected period.
pub struct UperiodicityLens;

impl Lens for UperiodicityLens {
    fn name(&self) -> &str {
        "PeriodicityLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // --- 1. Autocorrelation across all dimensions ---
        // Compute per-dimension means
        let mut means = vec![0.0_f64; d];
        for i in 0..n {
            for dim in 0..d {
                means[dim] += data[i * d + dim];
            }
        }
        for dim in 0..d {
            means[dim] /= n as f64;
        }

        // Compute per-dimension variances
        let mut variances = vec![0.0_f64; d];
        for i in 0..n {
            for dim in 0..d {
                let diff = data[i * d + dim] - means[dim];
                variances[dim] += diff * diff;
            }
        }
        for dim in 0..d {
            variances[dim] /= n as f64;
        }

        // Total variance (sum across dims) for normalization
        let total_var: f64 = variances.iter().sum();

        let max_lag = n / 2;
        let mut autocorr = vec![0.0_f64; max_lag];

        if total_var > 1e-15 {
            for lag in 1..max_lag {
                let mut sum = 0.0;
                let count = n - lag;
                for i in 0..count {
                    for dim in 0..d {
                        let a = data[i * d + dim] - means[dim];
                        let b = data[(i + lag) * d + dim] - means[dim];
                        sum += a * b;
                    }
                }
                autocorr[lag] = sum / (count as f64 * total_var);
            }
        }

        // --- 2. Peak detection in autocorrelation ---
        // Find local maxima (lag >= 2 to avoid trivial near-zero lags)
        let mut peaks: Vec<(usize, f64)> = Vec::new();
        for lag in 2..max_lag.saturating_sub(1) {
            if autocorr[lag] > autocorr[lag - 1] && autocorr[lag] > autocorr[lag + 1] {
                peaks.push((lag, autocorr[lag]));
            }
        }

        // Sort peaks by strength descending
        peaks.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Dominant period = lag of strongest peak
        let dominant_period = peaks.first().map(|&(lag, _)| lag).unwrap_or(0);
        let dominant_strength = peaks.first().map(|&(_, s)| s).unwrap_or(0.0);

        // Collect top-k peak lags (up to 5)
        let top_k = peaks.len().min(5);
        let peak_lags: Vec<f64> = peaks[..top_k].iter().map(|&(lag, _)| lag as f64).collect();
        let peak_strengths: Vec<f64> = peaks[..top_k].iter().map(|&(_, s)| s).collect();

        // --- 3. Periodicity strength ---
        // Mean absolute autocorrelation as baseline
        let baseline = if max_lag > 1 {
            autocorr[1..].iter().map(|v| v.abs()).sum::<f64>() / (max_lag - 1) as f64
        } else {
            0.0
        };
        let periodicity_score = if baseline > 1e-15 {
            (dominant_strength / baseline).min(100.0)
        } else if dominant_strength > 1e-15 {
            100.0
        } else {
            0.0
        };

        // --- 4. Distance-based recurrence rate ---
        // If a period was detected, measure how well points at multiples
        // of the period recur (small distances relative to median distance)
        let recurrence_rate = if dominant_period >= 2 && n > dominant_period {
            // Compute median pairwise distance for threshold
            let pair_count = n * (n - 1) / 2;
            let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
            for i in 0..n {
                for j in (i + 1)..n {
                    all_dists.push(shared.dist(i, j));
                }
            }
            all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median_dist = all_dists[pair_count / 2];
            let threshold = median_dist * 0.5;

            // Count recurrences at multiples of dominant_period
            let mut recur_count = 0_usize;
            let mut total_checks = 0_usize;
            for i in 0..n {
                let mut k = 1;
                while i + k * dominant_period < n {
                    let j = i + k * dominant_period;
                    if shared.dist(i, j) < threshold {
                        recur_count += 1;
                    }
                    total_checks += 1;
                    k += 1;
                }
            }

            if total_checks > 0 {
                recur_count as f64 / total_checks as f64
            } else {
                0.0
            }
        } else {
            0.0
        };

        // --- Build result ---
        let mut result = HashMap::new();
        result.insert("dominant_period".to_string(), vec![dominant_period as f64]);
        result.insert("dominant_strength".to_string(), vec![dominant_strength]);
        result.insert("periodicity_score".to_string(), vec![periodicity_score]);
        result.insert("recurrence_rate".to_string(), vec![recurrence_rate]);
        result.insert("num_peaks".to_string(), vec![peaks.len() as f64]);

        if !peak_lags.is_empty() {
            result.insert("peak_lags".to_string(), peak_lags);
            result.insert("peak_strengths".to_string(), peak_strengths);
        }

        // Full autocorrelation curve (for downstream analysis)
        if max_lag > 1 {
            result.insert("autocorrelation".to_string(), autocorr[1..].to_vec());
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_periodic_signal_detected() {
        // Create a clearly periodic 1D signal: repeating pattern of period 4
        // Pattern: [0, 1, 0, -1] repeated 6 times = 24 points
        let mut data = Vec::new();
        for _ in 0..6 {
            data.push(0.0);
            data.push(1.0);
            data.push(0.0);
            data.push(-1.0);
        }
        let n = data.len();
        let d = 1;
        let shared = SharedData::compute(&data, n, d);

        let lens = UperiodicityLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "Result must be non-empty");
        assert!(result.contains_key("dominant_period"));
        assert!(result.contains_key("periodicity_score"));
        assert!(result.contains_key("autocorrelation"));

        let period = result["dominant_period"][0] as usize;
        assert_eq!(period, 4, "Should detect period 4");

        let score = result["periodicity_score"][0];
        assert!(score > 1.0, "Periodicity score should be well above baseline for periodic data");
    }

    #[test]
    fn test_non_periodic_returns_result() {
        // Non-periodic monotonic data: should still return metrics, but weak periodicity
        let data: Vec<f64> = (0..20).map(|i| i as f64 * 0.3).collect();
        let n = data.len();
        let d = 1;
        let shared = SharedData::compute(&data, n, d);

        let lens = UperiodicityLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "Result must be non-empty even for non-periodic data");
        assert!(result.contains_key("dominant_period"));
        assert!(result.contains_key("periodicity_score"));
        assert!(result.contains_key("num_peaks"));
    }

    #[test]
    fn test_multidim_periodic() {
        // 2D data with period 3: [(cos, sin)] pattern
        let mut data = Vec::new();
        for i in 0..18 {
            let angle = (i % 3) as f64 * std::f64::consts::TAU / 3.0;
            data.push(angle.cos());
            data.push(angle.sin());
        }
        let n = 18;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);

        let lens = UperiodicityLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty());
        let period = result["dominant_period"][0] as usize;
        assert_eq!(period, 3, "Should detect period 3 in 2D circular pattern");
    }
}
