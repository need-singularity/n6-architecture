use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// WaveLens: DFT frequency spectrum + peak detection.
///
/// Algorithm:
///   1. Treat first dimension as a 1D signal (row order)
///   2. Compute DFT magnitudes via naive O(n^2) DFT
///   3. Find spectral peaks (local maxima above mean)
///   4. Reports dominant frequency and spectral entropy
pub struct WaveLens;

impl Lens for WaveLens {
    fn name(&self) -> &str {
        "WaveLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // Use first dimension as signal
        let signal: Vec<f64> = (0..n).map(|i| data[i * d]).collect();

        // Remove mean
        let mean = signal.iter().sum::<f64>() / n as f64;
        let centered: Vec<f64> = signal.iter().map(|&x| x - mean).collect();

        // Naive DFT: compute magnitudes for frequencies 1..n/2
        let half = n / 2;
        let mut magnitudes: Vec<f64> = Vec::with_capacity(half);
        let two_pi = 2.0 * std::f64::consts::PI;

        for k in 1..=half {
            let mut re = 0.0;
            let mut im = 0.0;
            for (t, &x) in centered.iter().enumerate() {
                let angle = two_pi * k as f64 * t as f64 / n as f64;
                re += x * angle.cos();
                im -= x * angle.sin();
            }
            magnitudes.push((re * re + im * im).sqrt() / n as f64);
        }

        // Find dominant frequency
        let max_mag = magnitudes
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let dominant_freq = magnitudes
            .iter()
            .position(|&m| (m - max_mag).abs() < 1e-15)
            .map(|i| (i + 1) as f64)
            .unwrap_or(0.0);

        // Spectral entropy: normalize magnitudes to probabilities
        let total_power: f64 = magnitudes.iter().map(|m| m * m).sum();
        let spectral_entropy = if total_power > 1e-15 {
            magnitudes
                .iter()
                .filter(|&&m| m * m > 1e-30)
                .map(|&m| {
                    let p = (m * m) / total_power;
                    -p * p.ln()
                })
                .sum::<f64>()
        } else {
            0.0
        };

        // Peak count: local maxima above mean magnitude
        let mean_mag = magnitudes.iter().sum::<f64>() / magnitudes.len() as f64;
        let mut peak_count = 0;
        for i in 0..magnitudes.len() {
            let left = if i > 0 { magnitudes[i - 1] } else { 0.0 };
            let right = if i + 1 < magnitudes.len() {
                magnitudes[i + 1]
            } else {
                0.0
            };
            if magnitudes[i] > left && magnitudes[i] > right && magnitudes[i] > mean_mag {
                peak_count += 1;
            }
        }

        let mut result = HashMap::new();
        result.insert("dominant_frequency".to_string(), vec![dominant_freq]);
        result.insert("spectral_entropy".to_string(), vec![spectral_entropy]);
        result.insert("peak_count".to_string(), vec![peak_count as f64]);
        result
    }
}
