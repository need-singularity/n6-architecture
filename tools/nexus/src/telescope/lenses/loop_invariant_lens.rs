use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// LoopInvariantLens: Detect loop-invariant computations that can be hoisted.
///
/// Analyzes data for repeating sub-patterns that remain constant across iterations,
/// indicating computations that a compiler could hoist out of a loop body.
///
/// Metrics:
///   1. invariant_fraction: fraction of dimensions with near-zero variance across rows
///   2. hoistable_dims: count of dimensions that are effectively constant
///   3. row_similarity: mean cosine similarity between consecutive rows (high = redundant work)
///   4. period_strength: autocorrelation at detected period (strong = predictable loop)
///   5. redundancy_ratio: fraction of data computable from a smaller invariant set
pub struct LoopInvariantLens;

impl Lens for LoopInvariantLens {
    fn name(&self) -> &str { "LoopInvariantLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Per-dimension mean and variance across rows
        let mut dim_mean = vec![0.0f64; d];
        let mut dim_var = vec![0.0f64; d];

        for i in 0..max_n {
            for j in 0..d {
                dim_mean[j] += data[i * d + j];
            }
        }
        for j in 0..d {
            dim_mean[j] /= max_n as f64;
        }
        for i in 0..max_n {
            for j in 0..d {
                let diff = data[i * d + j] - dim_mean[j];
                dim_var[j] += diff * diff;
            }
        }
        for j in 0..d {
            dim_var[j] /= max_n as f64;
        }

        // Invariant detection: dimensions with variance < 1% of max variance
        let max_var = dim_var.iter().cloned().fold(0.0f64, f64::max);
        let threshold = max_var * 0.01 + 1e-15;
        let hoistable_dims = dim_var.iter().filter(|&&v| v < threshold).count();
        let invariant_fraction = hoistable_dims as f64 / d as f64;

        // Row similarity: cosine similarity between consecutive rows
        let mut sim_sum = 0.0f64;
        let mut sim_count = 0usize;
        for i in 1..max_n {
            let mut dot = 0.0f64;
            let mut norm_a = 0.0f64;
            let mut norm_b = 0.0f64;
            for j in 0..d {
                let a = data[(i - 1) * d + j];
                let b = data[i * d + j];
                dot += a * b;
                norm_a += a * a;
                norm_b += b * b;
            }
            let denom = (norm_a * norm_b).sqrt();
            if denom > 1e-15 {
                sim_sum += dot / denom;
                sim_count += 1;
            }
        }
        let row_similarity = if sim_count > 0 { sim_sum / sim_count as f64 } else { 0.0 };

        // Period detection via autocorrelation on first dimension
        let col0: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let col_mean = col0.iter().sum::<f64>() / max_n as f64;
        let col_var: f64 = col0.iter().map(|&x| (x - col_mean).powi(2)).sum::<f64>() / max_n as f64;

        let mut best_period = 1usize;
        let mut best_autocorr = 0.0f64;
        let max_lag = (max_n / 2).min(64);
        if col_var > 1e-15 {
            for lag in 1..=max_lag {
                let mut ac = 0.0f64;
                let count = max_n - lag;
                for i in 0..count {
                    ac += (col0[i] - col_mean) * (col0[i + lag] - col_mean);
                }
                ac /= count as f64 * col_var;
                if ac > best_autocorr {
                    best_autocorr = ac;
                    best_period = lag;
                }
            }
        }
        let _ = best_period; // used implicitly via best_autocorr
        let period_strength = best_autocorr.max(0.0);

        // Redundancy: ratio of hoistable data volume
        let redundancy_ratio = invariant_fraction * row_similarity.max(0.0);

        let mut result = HashMap::new();
        result.insert("invariant_fraction".to_string(), vec![invariant_fraction]);
        result.insert("hoistable_dims".to_string(), vec![hoistable_dims as f64]);
        result.insert("row_similarity".to_string(), vec![row_similarity]);
        result.insert("period_strength".to_string(), vec![period_strength]);
        result.insert("redundancy_ratio".to_string(), vec![redundancy_ratio]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_column_detected() {
        // 3 dimensions: col0 varies, col1 constant, col2 varies
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(i as f64);       // varying
            data.push(42.0);           // invariant
            data.push(i as f64 * 0.5); // varying
        }
        let shared = SharedData::compute(&data, 20, 3);
        let result = LoopInvariantLens.scan(&data, 20, 3, &shared);
        assert!(result["hoistable_dims"][0] >= 1.0);
        assert!(result["invariant_fraction"][0] > 0.0);
    }

    #[test]
    fn test_identical_rows_high_similarity() {
        // All rows identical
        let data: Vec<f64> = (0..20).flat_map(|_| vec![1.0, 2.0, 3.0]).collect();
        let shared = SharedData::compute(&data, 20, 3);
        let result = LoopInvariantLens.scan(&data, 20, 3, &shared);
        assert!(result["row_similarity"][0] > 0.99);
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 4];
        let shared = SharedData::compute(&data, 4, 1);
        let result = LoopInvariantLens.scan(&data, 4, 1, &shared);
        assert!(result.is_empty());
    }
}
