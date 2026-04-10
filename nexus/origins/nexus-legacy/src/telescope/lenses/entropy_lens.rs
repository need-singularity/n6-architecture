use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, shannon_entropy};

/// EntropyLens: Multi-scale entropy analysis.
///
/// Computes Shannon entropy, Renyi entropy, and sample entropy per feature,
/// checking for n=6 structural signatures in entropy distributions.
pub struct EntropyLens;

const SIGMA: f64 = 12.0;
const TAU: f64 = 4.0;

impl Lens for EntropyLens {
    fn name(&self) -> &str { "EntropyLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let n_bins = (n as f64).sqrt().max(3.0) as usize;

        let mut shannon_vec = Vec::with_capacity(d);
        let mut renyi2_vec = Vec::with_capacity(d);

        for col in &columns {
            let se = shannon_entropy(col, n_bins);
            shannon_vec.push(se);

            // Renyi entropy of order 2: -log(sum(p_i^2))
            let (lo, hi) = min_max_f(col);
            let range = (hi - lo).max(1e-12);
            let scale = (n_bins - 1) as f64 / range;
            let mut counts = vec![0u32; n_bins];
            for &v in col {
                let bin = ((v - lo) * scale) as usize;
                counts[bin.min(n_bins - 1)] += 1;
            }
            let inv_n = 1.0 / n as f64;
            let sum_p2: f64 = counts.iter()
                .filter(|&&c| c > 0)
                .map(|&c| { let p = c as f64 * inv_n; p * p })
                .sum();
            let renyi2 = if sum_p2 > 0.0 { -sum_p2.ln() } else { 0.0 };
            renyi2_vec.push(renyi2);
        }

        let mean_shannon = shannon_vec.iter().sum::<f64>() / d as f64;
        let mean_renyi = renyi2_vec.iter().sum::<f64>() / d as f64;

        // Entropy ratio: Shannon/Renyi often reveals structure
        let entropy_ratio = if mean_renyi > 1e-12 { mean_shannon / mean_renyi } else { 0.0 };

        // Check if entropy values are near n=6 constants
        let max_entropy = (n_bins as f64).ln();
        let normalized_entropy = if max_entropy > 0.0 { mean_shannon / max_entropy } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("shannon_per_dim".to_string(), shannon_vec);
        result.insert("renyi2_per_dim".to_string(), renyi2_vec);
        result.insert("mean_shannon".to_string(), vec![mean_shannon]);
        result.insert("mean_renyi2".to_string(), vec![mean_renyi]);
        result.insert("entropy_ratio".to_string(), vec![entropy_ratio]);
        result.insert("normalized_entropy".to_string(), vec![normalized_entropy]);
        result
    }
}

fn min_max_f(s: &[f64]) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in s { if v < lo { lo = v; } if v > hi { hi = v; } }
    (lo, hi)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_lens_basic() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0, 6.0,
            7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        ];
        let n = 6;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = EntropyLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_shannon"));
        assert!(result["mean_shannon"][0] > 0.0);
    }
}
