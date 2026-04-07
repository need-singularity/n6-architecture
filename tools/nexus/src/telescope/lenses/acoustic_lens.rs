use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// AcousticLens: Detect sound-wave-like patterns in data.
///
/// Analyzes data for standing wave signatures, harmonic series,
/// and resonance patterns. n=6 connection: musical octave = phi=2,
/// chromatic scale = sigma=12 semitones, overtone series follows
/// divisor structure of 6.
pub struct AcousticLens;

impl Lens for AcousticLens {
    fn name(&self) -> &str { "AcousticLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // Analyze first dimension as a signal
        let signal: Vec<f64> = (0..n).map(|i| data[i * d]).collect();

        // Compute autocorrelation at lag=1..n/2 to find periodicity
        let mean = signal.iter().sum::<f64>() / n as f64;
        let var: f64 = signal.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;

        let max_lag = n / 2;
        let mut acf = Vec::with_capacity(max_lag);
        for lag in 1..=max_lag {
            let mut c = 0.0;
            for i in 0..(n - lag) {
                c += (signal[i] - mean) * (signal[i + lag] - mean);
            }
            c /= (n - lag) as f64;
            let r = if var > 1e-12 { c / var } else { 0.0 };
            acf.push(r);
        }

        // Find dominant period (first positive peak in ACF)
        let mut dominant_period = 0.0;
        let mut peak_strength = 0.0;
        for i in 1..acf.len().saturating_sub(1) {
            if acf[i] > acf[i - 1] && acf[i] > acf[i + 1] && acf[i] > peak_strength {
                dominant_period = (i + 1) as f64;
                peak_strength = acf[i];
                break;
            }
        }

        // Harmonic ratio: check if period aligns with n=6 harmonics
        // Perfect harmonics of 6: 1, 2, 3, 6 (divisors of 6)
        let harmonic_score = if dominant_period > 0.0 {
            let ratio_to_6 = dominant_period / 6.0;
            let nearest_harmonic = ratio_to_6.round();
            if nearest_harmonic > 0.0 {
                let deviation = (ratio_to_6 - nearest_harmonic).abs() / nearest_harmonic;
                (-deviation * 6.0).exp() // n=6 scaling
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Spectral flatness approximation (Wiener entropy)
        // Flat spectrum = noise, peaked = tonal
        let abs_acf: Vec<f64> = acf.iter().map(|x| x.abs().max(1e-12)).collect();
        let geo_mean = (abs_acf.iter().map(|x| x.ln()).sum::<f64>() / abs_acf.len() as f64).exp();
        let arith_mean = abs_acf.iter().sum::<f64>() / abs_acf.len() as f64;
        let spectral_flatness = if arith_mean > 1e-12 { geo_mean / arith_mean } else { 1.0 };
        let tonality = 1.0 - spectral_flatness; // 1 = pure tone, 0 = noise

        let mut result = HashMap::new();
        result.insert("dominant_period".to_string(), vec![dominant_period]);
        result.insert("peak_acf_strength".to_string(), vec![peak_strength]);
        result.insert("n6_harmonic_score".to_string(), vec![harmonic_score]);
        result.insert("tonality".to_string(), vec![tonality]);
        result.insert("spectral_flatness".to_string(), vec![spectral_flatness]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_acoustic_lens_periodic() {
        // Generate a sine wave with period=6
        let n = 60;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| {
            (2.0 * std::f64::consts::PI * i as f64 / 6.0).sin()
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = AcousticLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("dominant_period"));
        assert!(result.contains_key("tonality"), "Should produce tonality metric");
    }

    #[test]
    fn test_acoustic_lens_noise() {
        // Pseudorandom data (no periodicity)
        let n = 30;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| ((i * 7 + 13) % 17) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = AcousticLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("spectral_flatness"));
    }
}
