use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// InfoLens: Shannon entropy + LZ compression ratio approximation.
///
/// Algorithm:
///   1. Shannon entropy via histogram binning of all values
///   2. LZ complexity: count distinct substrings in quantized sequence
///   3. Compression ratio = distinct_patterns / total_length
pub struct InfoLens;

impl Lens for InfoLens {
    fn name(&self) -> &str {
        "InfoLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let total = n * d;

        // Shannon entropy over all data values
        let num_bins = ((total as f64).sqrt().ceil() as usize).max(2);
        let min_val = data.iter().take(total).cloned().fold(f64::INFINITY, f64::min);
        let max_val = data.iter().take(total).cloned().fold(f64::NEG_INFINITY, f64::max);
        let range = max_val - min_val;

        let shannon_entropy = if range < 1e-15 {
            0.0
        } else {
            let mut bins = vec![0usize; num_bins];
            for &v in data.iter().take(total) {
                let idx = ((v - min_val) / range * (num_bins - 1) as f64).round() as usize;
                bins[idx.min(num_bins - 1)] += 1;
            }
            bins.iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / total as f64;
                    -p * p.ln()
                })
                .sum::<f64>()
        };

        // LZ complexity approximation:
        // Quantize data to symbols, then count distinct substrings (LZ76-like)
        let num_symbols = num_bins.min(256);
        let symbols: Vec<usize> = if range < 1e-15 {
            vec![0; total]
        } else {
            data.iter()
                .take(total)
                .map(|&v| {
                    let idx =
                        ((v - min_val) / range * (num_symbols - 1) as f64).round() as usize;
                    idx.min(num_symbols - 1)
                })
                .collect()
        };

        // LZ76 complexity: scan through, count new substrings
        let mut complexity = 0usize;
        let mut i = 0;
        while i < symbols.len() {
            let mut len = 1;
            // Find the longest substring starting at i that appeared before
            'outer: while i + len <= symbols.len() {
                let substr = &symbols[i..i + len];
                // Search in symbols[0..i]
                for start in 0..i {
                    if start + len <= i && symbols[start..start + len] == *substr {
                        len += 1;
                        continue 'outer;
                    }
                }
                break;
            }
            complexity += 1;
            i += len;
        }

        let compression_ratio = if total > 0 {
            complexity as f64 / total as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("shannon_entropy".to_string(), vec![shannon_entropy]);
        result.insert("lz_compression_ratio".to_string(), vec![compression_ratio]);
        result
    }
}
