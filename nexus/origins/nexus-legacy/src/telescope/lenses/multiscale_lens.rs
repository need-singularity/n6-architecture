use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MultiscaleLens: wavelet-like multi-resolution analysis.
///
/// Algorithm:
///   1. Compute distance statistics at multiple scales (k=2,4,8,16,...)
///   2. At each scale k: mean k-NN distance = characteristic length
///   3. Scale invariance = how linearly length grows with k (log-log)
///   4. Reports scale-invariance score and characteristic lengths
pub struct MultiscaleLens;

impl Lens for MultiscaleLens {
    fn name(&self) -> &str {
        "MultiscaleLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 8 {
            return HashMap::new();
        }

        // Compute mean k-NN distance at multiple scales
        let mut scales: Vec<usize> = Vec::new();
        let mut k = 2;
        while k < n {
            scales.push(k);
            k *= 2;
        }

        if scales.is_empty() {
            return HashMap::new();
        }

        let mut log_k: Vec<f64> = Vec::new();
        let mut log_len: Vec<f64> = Vec::new();
        let mut char_lengths: Vec<f64> = Vec::new();

        for &kval in &scales {
            let mean_knn: f64 = (0..n)
                .map(|i| {
                    let mut dists: Vec<f64> = (0..n)
                        .filter(|&j| j != i)
                        .map(|j| shared.dist(i, j))
                        .collect();
                    dists.sort_by(|a, b| {
                        a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal)
                    });
                    dists[kval.min(dists.len()) - 1]
                })
                .sum::<f64>()
                / n as f64;

            char_lengths.push(mean_knn);

            if mean_knn > 1e-15 {
                log_k.push((kval as f64).ln());
                log_len.push(mean_knn.ln());
            }
        }

        // Scale-invariance: R^2 of log-log linear fit
        let scale_invariance = if log_k.len() >= 2 {
            let (slope, intercept) = linear_regression(&log_k, &log_len);
            // R^2 = 1 - SS_res / SS_tot
            let mean_y = log_len.iter().sum::<f64>() / log_len.len() as f64;
            let ss_tot: f64 = log_len.iter().map(|y| (y - mean_y).powi(2)).sum();
            let ss_res: f64 = log_k
                .iter()
                .zip(log_len.iter())
                .map(|(x, y)| {
                    let pred = slope * x + intercept;
                    (y - pred).powi(2)
                })
                .sum();
            if ss_tot > 1e-15 {
                (1.0 - ss_res / ss_tot).max(0.0)
            } else {
                1.0
            }
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert(
            "scale_invariance_r2".to_string(),
            vec![scale_invariance],
        );
        result.insert("characteristic_lengths".to_string(), char_lengths);
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
