use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// CompressionLens: Data compressibility analysis.
/// Measures compression_ratio, kolmogorov_estimate, redundancy, entropy_rate, lossless_potential.
pub struct CompressionLens;

impl Lens for CompressionLens {
    fn name(&self) -> &str { "CompressionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let total = max_n * d;
        let slice = &data[..total.min(data.len())];

        // Entropy rate (bits per value)
        let n_bins = 32;
        let entropy = shannon_entropy(slice, n_bins);
        let max_entropy = (n_bins as f64).ln(); // max possible
        let entropy_rate = if max_entropy > 0.0 { entropy / max_entropy } else { 0.0 };

        // Compression ratio: 1 - normalized_entropy (1 = perfectly compressible, 0 = random)
        let compression_ratio = 1.0 - entropy_rate;

        // Kolmogorov estimate: approximate via run-length encoding uniqueness
        // Count distinct quantized values
        let (min_v, max_v) = slice.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
        let range = (max_v - min_v).max(1e-15);
        let quantized: Vec<u32> = slice.iter().map(|&v| ((v - min_v) / range * (n_bins as f64 - 1.0)) as u32).collect();

        // Run-length: count number of runs
        let mut runs = 1usize;
        for i in 1..quantized.len() {
            if quantized[i] != quantized[i - 1] { runs += 1; }
        }
        let kolmogorov_est = runs as f64 / quantized.len().max(1) as f64;

        // Redundancy: repeated consecutive pairs
        let mut pair_counts: HashMap<(u32, u32), usize> = HashMap::new();
        for i in 0..quantized.len().saturating_sub(1) {
            *pair_counts.entry((quantized[i], quantized[i + 1])).or_insert(0) += 1;
        }
        let total_pairs = quantized.len().saturating_sub(1).max(1);
        let unique_pairs = pair_counts.len();
        let max_possible = (n_bins * n_bins) as usize;
        let redundancy = 1.0 - unique_pairs as f64 / max_possible.min(total_pairs) as f64;

        // Lossless potential: fraction recoverable (higher entropy_rate = lower potential)
        let lossless_potential = compression_ratio.max(0.0);

        let mut result = HashMap::new();
        result.insert("compression_ratio".into(), vec![compression_ratio]);
        result.insert("kolmogorov_estimate".into(), vec![kolmogorov_est]);
        result.insert("redundancy".into(), vec![redundancy.max(0.0)]);
        result.insert("entropy_rate".into(), vec![entropy_rate]);
        result.insert("lossless_potential".into(), vec![lossless_potential]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = CompressionLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
