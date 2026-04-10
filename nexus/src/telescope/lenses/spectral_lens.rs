use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// SpectralLens: Frequency domain analysis via discrete Fourier transform.
///
/// Detects periodic structure and dominant frequencies in features,
/// checking for n=6 harmonic signatures (sigma=12, J2=24 harmonic ratios).
pub struct SpectralLens;

impl Lens for SpectralLens {
    fn name(&self) -> &str { "SpectralLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let mut dominant_freqs = Vec::new();
        let mut spectral_entropy_vals = Vec::new();

        for col in columns.iter().take(d.min(12)) {
            // Compute power spectrum via DFT
            let power = dft_power(col);
            if power.is_empty() { continue; }

            // Find dominant frequency
            let mut max_pow = 0.0;
            let mut max_idx = 0;
            for (i, &p) in power.iter().enumerate() {
                if p > max_pow { max_pow = p; max_idx = i; }
            }
            let dominant_freq = max_idx as f64 / n as f64;
            dominant_freqs.push(dominant_freq);

            // Spectral entropy: normalized power spectrum entropy
            let total_pow: f64 = power.iter().sum();
            if total_pow > 1e-12 {
                let mut se = 0.0;
                for &p in &power {
                    if p > 0.0 {
                        let norm_p = p / total_pow;
                        se -= norm_p * norm_p.ln();
                    }
                }
                let max_se = (power.len() as f64).ln();
                let norm_se = if max_se > 0.0 { se / max_se } else { 0.0 };
                spectral_entropy_vals.push(norm_se);
            }
        }

        if dominant_freqs.is_empty() { return HashMap::new(); }

        let mean_spectral_entropy = spectral_entropy_vals.iter().sum::<f64>()
            / spectral_entropy_vals.len().max(1) as f64;

        // Check for harmonic ratios (frequency ratios that are n=6 fractions)
        let mut harmonic_ratio_matches = 0;
        let n6_ratios = [0.5, 1.0 / 3.0, 1.0 / 6.0, 2.0 / 3.0, 0.25];
        if dominant_freqs.len() >= 2 {
            for i in 0..dominant_freqs.len() {
                for j in (i + 1)..dominant_freqs.len() {
                    if dominant_freqs[j].abs() > 1e-12 {
                        let ratio = dominant_freqs[i] / dominant_freqs[j];
                        for &r in &n6_ratios {
                            if (ratio - r).abs() < 0.05 || (1.0 / ratio - r).abs() < 0.05 {
                                harmonic_ratio_matches += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }

        let mut result = HashMap::new();
        result.insert("dominant_frequencies".to_string(), dominant_freqs);
        result.insert("spectral_entropy".to_string(), spectral_entropy_vals);
        result.insert("mean_spectral_entropy".to_string(), vec![mean_spectral_entropy]);
        result.insert("harmonic_ratio_matches".to_string(), vec![harmonic_ratio_matches as f64]);
        result
    }
}

/// Compute power spectrum via naive DFT (first n/2 frequencies).
fn dft_power(signal: &[f64]) -> Vec<f64> {
    let n = signal.len();
    if n < 4 { return Vec::new(); }
    let half = n / 2;
    let mut power = Vec::with_capacity(half);

    for k in 1..=half { // Skip DC component
        let mut re = 0.0;
        let mut im = 0.0;
        for (t, &x) in signal.iter().enumerate() {
            let angle = 2.0 * std::f64::consts::PI * k as f64 * t as f64 / n as f64;
            re += x * angle.cos();
            im -= x * angle.sin();
        }
        power.push((re * re + im * im) / n as f64);
    }
    power
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_lens_sine() {
        // Pure sine wave with 6 cycles
        let n = 48;
        let d = 1;
        let data: Vec<f64> = (0..n)
            .map(|i| (2.0 * std::f64::consts::PI * 6.0 * i as f64 / n as f64).sin())
            .collect();
        let shared = SharedData::compute(&data, n, d);
        let result = SpectralLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("dominant_frequencies"));
        let freq = result["dominant_frequencies"][0];
        assert!((freq - 6.0 / 48.0).abs() < 0.05, "Should detect freq=6/48, got {}", freq);
    }
}
