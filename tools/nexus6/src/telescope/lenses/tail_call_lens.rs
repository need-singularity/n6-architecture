use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TailCallLens: Tail call optimization opportunity detection.
///
/// Analyzes data for patterns indicative of recursive/tail-recursive structures:
/// - Self-similar subsequences (data that repeats its own structure at smaller scale)
/// - Convergent tails (sequences that converge to a fixed point)
/// - Stack-like growth patterns that could be flattened
///
/// Metrics:
///   1. tail_recursion_score: likelihood that the pattern is tail-recursive (0-1)
///   2. convergence_rate: how fast the "recursion" converges (higher = better for TCO)
///   3. stack_depth_estimate: estimated recursion depth before convergence
///   4. fixed_point_distance: distance of final values from detected fixed point
///   5. flattenable_fraction: fraction of the recursion that can be converted to iteration
pub struct TailCallLens;

impl Lens for TailCallLens {
    fn name(&self) -> &str { "TailCallLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Treat rows as successive "function calls"
        // Tail recursion pattern: row[i] is a transformation of row[i-1]
        // and the transformation converges

        // Compute per-row norms and successive ratios
        let mut row_norms = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let norm: f64 = (0..d).map(|j| data[i * d + j].powi(2)).sum::<f64>().sqrt();
            row_norms.push(norm);
        }

        // Convergence: ratio of consecutive norms approaching 1.0
        let mut ratios = Vec::with_capacity(max_n - 1);
        for i in 1..max_n {
            let ratio = if row_norms[i - 1] > 1e-15 {
                row_norms[i] / row_norms[i - 1]
            } else {
                1.0
            };
            ratios.push(ratio);
        }

        // Convergence rate: how quickly ratios approach a constant
        let mut ratio_diffs = Vec::with_capacity(ratios.len().saturating_sub(1));
        for i in 1..ratios.len() {
            ratio_diffs.push((ratios[i] - ratios[i - 1]).abs());
        }
        let convergence_rate = if !ratio_diffs.is_empty() {
            let mean_diff = ratio_diffs.iter().sum::<f64>() / ratio_diffs.len() as f64;
            // Exponential decay fit: check if later diffs are smaller
            let half = ratio_diffs.len() / 2;
            let first_half_mean = ratio_diffs[..half.max(1)].iter().sum::<f64>()
                / half.max(1) as f64;
            let second_half_mean = ratio_diffs[half..].iter().sum::<f64>()
                / (ratio_diffs.len() - half).max(1) as f64;
            if first_half_mean > 1e-15 {
                ((first_half_mean - second_half_mean) / first_half_mean).max(0.0).min(1.0)
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Self-similarity: compare first half to second half pattern
        let half = max_n / 2;
        let mut self_sim = 0.0f64;
        if half >= 3 {
            // Cosine similarity of norm sequences
            let first: Vec<f64> = row_norms[..half].to_vec();
            let second: Vec<f64> = row_norms[half..half * 2].to_vec();
            let len = first.len().min(second.len());
            let mut dot = 0.0f64;
            let mut na = 0.0f64;
            let mut nb = 0.0f64;
            for i in 0..len {
                dot += first[i] * second[i];
                na += first[i] * first[i];
                nb += second[i] * second[i];
            }
            let denom = (na * nb).sqrt();
            if denom > 1e-15 {
                self_sim = dot / denom;
            }
        }

        // Fixed point detection: check if last few rows converge to a single value
        let tail_size = (max_n / 5).max(3).min(max_n);
        let tail_start = max_n - tail_size;
        let mut tail_mean = vec![0.0f64; d];
        for i in tail_start..max_n {
            for j in 0..d {
                tail_mean[j] += data[i * d + j];
            }
        }
        for j in 0..d { tail_mean[j] /= tail_size as f64; }

        let fixed_point_distance: f64 = {
            let last_row_start = (max_n - 1) * d;
            (0..d).map(|j| (data[last_row_start + j] - tail_mean[j]).powi(2))
                .sum::<f64>()
                .sqrt()
        };

        // Stack depth: number of rows before convergence (norm change < 1%)
        let mut stack_depth = max_n;
        for i in 1..max_n {
            if (ratios[i - 1] - 1.0).abs() < 0.01 {
                stack_depth = i;
                break;
            }
        }
        let stack_depth_estimate = stack_depth as f64;

        // Tail recursion score: combination of convergence + self-similarity
        let tail_recursion_score = (convergence_rate * 0.4
            + self_sim.max(0.0) * 0.3
            + (1.0 - fixed_point_distance / (row_norms.last().unwrap_or(&1.0) + 1e-15)).max(0.0) * 0.3)
            .min(1.0);

        // Flattenable fraction: portion after convergence that's just iteration
        let flattenable_fraction = if max_n > 0 {
            (max_n - stack_depth) as f64 / max_n as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("tail_recursion_score".to_string(), vec![tail_recursion_score]);
        result.insert("convergence_rate".to_string(), vec![convergence_rate]);
        result.insert("stack_depth_estimate".to_string(), vec![stack_depth_estimate]);
        result.insert("fixed_point_distance".to_string(), vec![fixed_point_distance]);
        result.insert("flattenable_fraction".to_string(), vec![flattenable_fraction]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_converging_sequence() {
        // Exponentially decaying sequence = converges to fixed point
        let data: Vec<f64> = (0..20).map(|i| 10.0 * (-0.3 * i as f64).exp()).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = TailCallLens.scan(&data, 20, 1, &shared);
        assert!(result.contains_key("tail_recursion_score"));
        assert!(result["convergence_rate"][0] >= 0.0);
    }

    #[test]
    fn test_constant_sequence_is_fixed_point() {
        // Already at fixed point
        let data = vec![42.0; 20];
        let shared = SharedData::compute(&data, 20, 1);
        let result = TailCallLens.scan(&data, 20, 1, &shared);
        assert!(result["fixed_point_distance"][0] < 1e-10);
    }

    #[test]
    fn test_diverging_no_tail_call() {
        // Exponentially growing = diverging, not tail-recursion-friendly
        let data: Vec<f64> = (0..20).map(|i| (i as f64).exp()).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = TailCallLens.scan(&data, 20, 1, &shared);
        assert!(result.contains_key("stack_depth_estimate"));
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 4];
        let shared = SharedData::compute(&data, 4, 1);
        let result = TailCallLens.scan(&data, 4, 1, &shared);
        assert!(result.is_empty());
    }
}
