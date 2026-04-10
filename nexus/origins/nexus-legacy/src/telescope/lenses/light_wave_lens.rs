use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// LightWaveLens: Electromagnetic wave patterns.
/// Measures wavelength, frequency, amplitude, polarization, interference.
pub struct LightWaveLens;

impl Lens for LightWaveLens {
    fn name(&self) -> &str { "LightWaveLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let mean = signal.iter().sum::<f64>() / max_n as f64;
        let centered: Vec<f64> = signal.iter().map(|&x| x - mean).collect();

        // DFT to find dominant frequency
        let half = max_n / 2;
        let two_pi = 2.0 * std::f64::consts::PI;
        let mut max_mag = 0.0f64;
        let mut dominant_k = 1usize;
        let mut magnitudes = Vec::with_capacity(half);

        for k in 1..=half {
            let mut re = 0.0f64;
            let mut im = 0.0f64;
            for (t, &x) in centered.iter().enumerate() {
                let angle = two_pi * k as f64 * t as f64 / max_n as f64;
                re += x * angle.cos();
                im -= x * angle.sin();
            }
            let mag = (re * re + im * im).sqrt() / max_n as f64;
            magnitudes.push(mag);
            if mag > max_mag {
                max_mag = mag;
                dominant_k = k;
            }
        }

        // Wavelength (in samples) and frequency
        let wavelength = if dominant_k > 0 { max_n as f64 / dominant_k as f64 } else { max_n as f64 };
        let frequency = dominant_k as f64 / max_n as f64;

        // Amplitude: peak magnitude
        let amplitude = max_mag * 2.0; // factor of 2 for real signal

        // Polarization: if d >= 2, compare energy in dim 0 vs dim 1
        let polarization = if d >= 2 {
            let e0: f64 = (0..max_n).map(|i| data[i * d].powi(2)).sum::<f64>();
            let e1: f64 = (0..max_n).map(|i| data[i * d + 1].powi(2)).sum::<f64>();
            let total = e0 + e1;
            if total > 1e-12 { (e0 - e1).abs() / total } else { 0.0 }
        } else {
            1.0 // 1D = fully linearly polarized
        };

        // Interference pattern: constructive/destructive score
        // Count zero crossings relative to expected for dominant frequency
        let mut zero_crossings = 0usize;
        for i in 1..max_n {
            if centered[i] * centered[i - 1] < 0.0 {
                zero_crossings += 1;
            }
        }
        let expected_crossings = 2.0 * dominant_k as f64;
        let interference_pattern = if expected_crossings > 0.0 {
            1.0 - (zero_crossings as f64 - expected_crossings).abs() / expected_crossings.max(1.0)
        } else {
            0.0
        };
        let interference_pattern = interference_pattern.clamp(0.0, 1.0);

        let mut result = HashMap::new();
        result.insert("wavelength".to_string(), vec![wavelength]);
        result.insert("frequency".to_string(), vec![frequency]);
        result.insert("amplitude".to_string(), vec![amplitude]);
        result.insert("polarization".to_string(), vec![polarization]);
        result.insert("interference_pattern".to_string(), vec![interference_pattern]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sine_wave() {
        // Pure sine wave should give clear frequency
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * std::f64::consts::TAU / 10.0).sin()).collect();
        let shared = SharedData::compute(&data, 40, 1);
        let result = LightWaveLens.scan(&data, 40, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["amplitude"][0] > 0.0);
        assert!(result["wavelength"][0] > 1.0);
    }
}
