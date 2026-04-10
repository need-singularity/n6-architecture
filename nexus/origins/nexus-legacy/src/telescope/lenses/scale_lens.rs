use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ScaleLens (스케일/돋보기): power-law fitting + Hurst exponent.
///
/// Algorithm:
///   1. Distance distribution: histogram of pairwise distances
///   2. Log-log regression to detect power-law tail
///   3. Hurst exponent via rescaled range (R/S) on first dimension
///   4. Reports power-law exponent and Hurst exponent
pub struct ScaleLens;

impl Lens for ScaleLens {
    fn name(&self) -> &str {
        "ScaleLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 {
            return HashMap::new();
        }

        // Collect pairwise distances
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(shared.dist(i, j));
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Log-log CDF: P(D > d) vs d
        let total = dists.len() as f64;
        let num_bins = 20;
        let d_min = dists.first().copied().unwrap_or(0.0).max(1e-10);
        let d_max = dists.last().copied().unwrap_or(1.0);

        if d_max <= d_min {
            let mut result = HashMap::new();
            result.insert("power_law_exponent".to_string(), vec![0.0]);
            result.insert("hurst_exponent".to_string(), vec![0.5]);
            return result;
        }

        // Log-spaced bins
        let log_min = d_min.ln();
        let log_max = d_max.ln();
        let step = (log_max - log_min) / num_bins as f64;

        let mut log_d: Vec<f64> = Vec::new();
        let mut log_cdf: Vec<f64> = Vec::new();

        for b in 0..num_bins {
            let threshold = (log_min + (b as f64 + 0.5) * step).exp();
            let count = dists.iter().filter(|&&x| x > threshold).count();
            if count > 0 {
                log_d.push(threshold.ln());
                log_cdf.push((count as f64 / total).ln());
            }
        }

        // Linear regression on log-log: slope = -(power-law exponent)
        let power_law_exponent = if log_d.len() >= 3 {
            let (slope, _) = linear_regression(&log_d, &log_cdf);
            -slope
        } else {
            0.0
        };

        // Hurst exponent via R/S analysis on first dimension
        let signal: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
        let hurst = hurst_exponent(&signal);

        let mut result = HashMap::new();
        result.insert("power_law_exponent".to_string(), vec![power_law_exponent]);
        result.insert("hurst_exponent".to_string(), vec![hurst]);
        result
    }
}

fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len() as f64;
    let sum_x: f64 = x.iter().sum();
    let sum_y: f64 = y.iter().sum();
    let sum_xy: f64 = x.iter().zip(y.iter()).map(|(a, b)| a * b).sum();
    let sum_xx: f64 = x.iter().map(|a| a * a).sum();

    let denom = n * sum_xx - sum_x * sum_x;
    if denom.abs() < 1e-15 {
        return (0.0, sum_y / n);
    }

    let slope = (n * sum_xy - sum_x * sum_y) / denom;
    let intercept = (sum_y - slope * sum_x) / n;
    (slope, intercept)
}

fn hurst_exponent(signal: &[f64]) -> f64 {
    let n = signal.len();
    if n < 6 {
        return 0.5;
    }

    // R/S analysis at multiple scales
    let mut log_n: Vec<f64> = Vec::new();
    let mut log_rs: Vec<f64> = Vec::new();

    let mut size = 4;
    while size <= n / 2 {
        let chunks = n / size;
        if chunks == 0 {
            break;
        }

        let mut rs_values: Vec<f64> = Vec::new();
        for c in 0..chunks {
            let chunk: Vec<f64> = signal[c * size..(c + 1) * size].to_vec();
            let mean = chunk.iter().sum::<f64>() / size as f64;
            let deviations: Vec<f64> = chunk.iter().map(|x| x - mean).collect();

            // Cumulative deviation
            let mut cumsum = vec![0.0; size];
            cumsum[0] = deviations[0];
            for i in 1..size {
                cumsum[i] = cumsum[i - 1] + deviations[i];
            }

            let r = cumsum.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                - cumsum.iter().cloned().fold(f64::INFINITY, f64::min);
            let s = (deviations.iter().map(|x| x * x).sum::<f64>() / size as f64).sqrt();

            if s > 1e-15 {
                rs_values.push(r / s);
            }
        }

        if !rs_values.is_empty() {
            let mean_rs = rs_values.iter().sum::<f64>() / rs_values.len() as f64;
            if mean_rs > 0.0 {
                log_n.push((size as f64).ln());
                log_rs.push(mean_rs.ln());
            }
        }

        size *= 2;
    }

    if log_n.len() >= 2 {
        let (slope, _) = linear_regression(&log_n, &log_rs);
        slope.max(0.0).min(1.0)
    } else {
        0.5
    }
}
