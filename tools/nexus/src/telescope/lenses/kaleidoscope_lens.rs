use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// KaleidoscopeLens: Symmetric pattern repetition.
/// Detects n-fold symmetry, repeated motifs, reflection, rotation.
/// Especially checks for 6-fold symmetry (n=6).
pub struct KaleidoscopeLens;

impl Lens for KaleidoscopeLens {
    fn name(&self) -> &str { "KaleidoscopeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();

        // Test rotational symmetry for orders 2..12
        // For each order k, shift signal by n/k and measure correlation
        let mut best_order = 0usize;
        let mut best_score = -1.0f64;
        let norm2: f64 = signal.iter().map(|x| x * x).sum::<f64>();

        for k in 2..=12 {
            let shift = max_n / k;
            if shift == 0 { continue; }
            let mut corr = 0.0f64;
            let mut count = 0;
            for i in 0..(max_n - shift) {
                corr += signal[i] * signal[i + shift];
                count += 1;
            }
            let score = if norm2 > 1e-12 && count > 0 {
                corr / (count as f64) / (norm2 / max_n as f64).max(1e-12)
            } else {
                0.0
            };
            if score > best_score {
                best_score = score;
                best_order = k;
            }
        }
        let symmetry_order = best_order as f64;

        // Pattern count: count repeated motifs via autocorrelation peaks
        let mean = signal.iter().sum::<f64>() / max_n as f64;
        let centered: Vec<f64> = signal.iter().map(|&x| x - mean).collect();
        let var: f64 = centered.iter().map(|x| x * x).sum::<f64>();
        let mut pattern_count = 0;
        if var > 1e-12 {
            for lag in 1..max_n / 2 {
                let mut ac = 0.0f64;
                for i in 0..(max_n - lag) {
                    ac += centered[i] * centered[i + lag];
                }
                ac /= var;
                if ac > 0.5 { pattern_count += 1; }
            }
        }

        // Reflection score: bilateral symmetry (compare first half with reversed second half)
        let half = max_n / 2;
        let mut refl_corr = 0.0f64;
        let mut refl_norm = 0.0f64;
        for i in 0..half {
            let mirror = max_n - 1 - i;
            if mirror < max_n {
                refl_corr += signal[i] * signal[mirror];
                refl_norm += signal[i] * signal[i] + signal[mirror] * signal[mirror];
            }
        }
        let reflection_score = if refl_norm > 1e-12 {
            (2.0 * refl_corr / refl_norm).clamp(-1.0, 1.0)
        } else {
            0.0
        };

        // Rotation score: best rotational correlation (already computed)
        let rotation_score = best_score.clamp(-1.0, 1.0);

        let mut result = HashMap::new();
        result.insert("symmetry_order".to_string(), vec![symmetry_order]);
        result.insert("pattern_count".to_string(), vec![pattern_count as f64]);
        result.insert("reflection_score".to_string(), vec![reflection_score]);
        result.insert("rotation_score".to_string(), vec![rotation_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Periodic signal with 6-fold repetition
        let data: Vec<f64> = (0..60).map(|i| ((i as f64) * std::f64::consts::TAU / 10.0).sin()).collect();
        let shared = SharedData::compute(&data, 60, 1);
        let result = KaleidoscopeLens.scan(&data, 60, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["symmetry_order"][0] >= 2.0);
    }
}
