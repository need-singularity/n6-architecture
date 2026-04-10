use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// GoldenRatioLens: Golden ratio phi=1.618... detection.
/// Detects phi ratios, Fibonacci patterns, spiral fits.
/// n=6: phi(6)=2 (Euler totient), but golden ratio phi=(1+sqrt(5))/2 appears in nature.
pub struct GoldenRatioLens;

const PHI: f64 = 1.618033988749895;
const PHI_TOL: f64 = 0.05;

impl Lens for GoldenRatioLens {
    fn name(&self) -> &str { "GoldenRatioLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();

        // Phi matches: count consecutive ratios near golden ratio
        let mut phi_matches = 0usize;
        let mut ratio_count = 0usize;
        for i in 1..max_n {
            let prev = signal[i - 1].abs();
            let curr = signal[i].abs();
            if prev > 1e-12 {
                let ratio = curr / prev;
                ratio_count += 1;
                if (ratio - PHI).abs() < PHI_TOL || (ratio - 1.0 / PHI).abs() < PHI_TOL {
                    phi_matches += 1;
                }
            }
        }

        // Fibonacci score: how Fibonacci-like the sequence is
        // Compare signal[i] with signal[i-1] + signal[i-2]
        let mut fib_errors = Vec::new();
        for i in 2..max_n {
            let expected = signal[i - 1] + signal[i - 2];
            let scale = signal[i].abs().max(expected.abs()).max(1e-12);
            let err = (signal[i] - expected).abs() / scale;
            fib_errors.push(err);
        }
        let fibonacci_score = if !fib_errors.is_empty() {
            let mean_err = fib_errors.iter().sum::<f64>() / fib_errors.len() as f64;
            (1.0 / (1.0 + mean_err)).min(1.0)
        } else {
            0.0
        };

        // Golden spiral fit: check if log of distances grows linearly (log spiral)
        let abs_vals: Vec<f64> = signal.iter().map(|x| x.abs().max(1e-15)).collect();
        let log_vals: Vec<f64> = abs_vals.iter().map(|x| x.ln()).collect();
        // Linear regression on log values
        let n_f = max_n as f64;
        let x_mean = (n_f - 1.0) / 2.0;
        let y_mean = log_vals.iter().sum::<f64>() / n_f;
        let mut ss_xy = 0.0f64;
        let mut ss_xx = 0.0f64;
        let mut ss_yy = 0.0f64;
        for i in 0..max_n {
            let dx = i as f64 - x_mean;
            let dy = log_vals[i] - y_mean;
            ss_xy += dx * dy;
            ss_xx += dx * dx;
            ss_yy += dy * dy;
        }
        let golden_spiral_fit = if ss_xx > 1e-12 && ss_yy > 1e-12 {
            let r = ss_xy / (ss_xx * ss_yy).sqrt();
            r.abs().min(1.0)
        } else {
            0.0
        };

        // Phi fraction: fraction of consecutive ratios near phi
        let phi_fraction = if ratio_count > 0 { phi_matches as f64 / ratio_count as f64 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("phi_matches".to_string(), vec![phi_matches as f64]);
        result.insert("fibonacci_score".to_string(), vec![fibonacci_score]);
        result.insert("golden_spiral_fit".to_string(), vec![golden_spiral_fit]);
        result.insert("phi_fraction".to_string(), vec![phi_fraction]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fibonacci_sequence() {
        // Actual Fibonacci sequence should score high
        let mut fib = vec![1.0, 1.0];
        for _ in 2..20 { let next = fib[fib.len() - 1] + fib[fib.len() - 2]; fib.push(next); }
        let shared = SharedData::compute(&fib, 20, 1);
        let result = GoldenRatioLens.scan(&fib, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["fibonacci_score"][0] > 0.5);
    }
}
