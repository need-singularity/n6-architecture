use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, shannon_entropy, column_vectors};

/// KeywordLens: Key feature extraction.
///
/// Finds the most important dimensions/patterns by measuring per-dimension
/// importance (variance + MI contribution), redundancy between dims,
/// and compression ratio.
pub struct KeywordLens;

impl Lens for KeywordLens {
    fn name(&self) -> &str { "KeywordLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        if d < 1 { return HashMap::new(); }

        let (_means, vars) = mean_var(data, max_n, d);
        let total_var: f64 = vars.iter().sum::<f64>().max(1e-12);

        // importance_scores: combine variance share + mean MI with other dims
        let mut importance: Vec<f64> = Vec::with_capacity(d);
        for di in 0..d {
            let var_share = vars[di] / total_var;
            let mi_sum: f64 = (0..d)
                .filter(|&dj| dj != di)
                .map(|dj| shared.mi(di, dj))
                .sum::<f64>();
            let mi_contrib = if d > 1 { mi_sum / (d - 1) as f64 } else { 0.0 };
            // Importance = variance share + MI contribution (both informative)
            importance.push(var_share + mi_contrib);
        }

        // Normalize importance to sum to 1
        let imp_sum: f64 = importance.iter().sum::<f64>().max(1e-12);
        for imp in &mut importance {
            *imp /= imp_sum;
        }

        // key_dimensions: indices of top-k most important dims (sorted desc)
        let mut indexed: Vec<(usize, f64)> = importance.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let top_k = d.min(6);
        let key_dimensions: Vec<f64> = indexed[..top_k].iter().map(|&(i, _)| i as f64).collect();

        // redundancy: mean MI between all dimension pairs (high MI = redundant)
        let mut mi_sum = 0.0_f64;
        let mut mi_count = 0u32;
        for di in 0..d {
            for dj in (di + 1)..d {
                mi_sum += shared.mi(di, dj);
                mi_count += 1;
            }
        }
        let redundancy = if mi_count > 0 { mi_sum / mi_count as f64 } else { 0.0 };

        // compression_ratio: entropy captured by top-k dims vs all dims
        let cols = column_vectors(data, max_n, d);
        let total_entropy: f64 = cols.iter()
            .map(|col| shannon_entropy(col, 16))
            .sum::<f64>()
            .max(1e-12);

        let topk_entropy: f64 = indexed[..top_k].iter()
            .map(|&(i, _)| shannon_entropy(&cols[i], 16))
            .sum();
        let compression_ratio = topk_entropy / total_entropy;

        let mut result = HashMap::new();
        result.insert("key_dimensions".to_string(), key_dimensions);
        result.insert("importance_scores".to_string(), importance);
        result.insert("redundancy".to_string(), vec![redundancy]);
        result.insert("compression_ratio".to_string(), vec![compression_ratio]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..60).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 3);
        let result = KeywordLens.scan(&data, 20, 3, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("key_dimensions"));
        assert!(result.contains_key("importance_scores"));
        assert!(result.contains_key("compression_ratio"));
    }

    #[test]
    fn test_importance_sums_to_one() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1)).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = KeywordLens.scan(&data, 20, 2, &shared);
        let imp_sum: f64 = result["importance_scores"].iter().sum();
        assert!((imp_sum - 1.0).abs() < 1e-6, "importance should sum to 1.0: {imp_sum}");
    }
}
