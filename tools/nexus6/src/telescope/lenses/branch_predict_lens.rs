use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BranchPredictLens: Static branch bias inference for compiler optimization.
///
/// Analyzes data array patterns to detect conditional branching characteristics:
///   1. branch_entropy: Shannon entropy of binary condition patterns (high = unpredictable)
///   2. bias_ratio: fraction of values above median (how biased the "branch" is)
///   3. run_length_mean: average consecutive same-direction streak (longer = more predictable)
///   4. toggle_rate: fraction of adjacent sign changes (high = branch-unfriendly)
///   5. predictability_score: 1 - normalized entropy (1.0 = perfectly predictable)
///
/// Compiler use: Guides branch prediction hints (__builtin_expect), if-conversion,
/// and conditional move generation.
pub struct BranchPredictLens;

impl Lens for BranchPredictLens {
    fn name(&self) -> &str { "BranchPredictLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let total = n * d;

        // Compute median for binary classification
        let mut sorted = data[..total].to_vec();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = sorted[total / 2];

        // Binary encoding: 1 if above median, 0 if below
        let binary: Vec<u8> = data[..total].iter().map(|&v| if v > median { 1 } else { 0 }).collect();

        // Bias ratio: fraction of 1s
        let ones = binary.iter().filter(|&&b| b == 1).count();
        let bias_ratio = ones as f64 / total as f64;

        // Branch entropy: H(p) = -p*log2(p) - (1-p)*log2(1-p)
        let p = bias_ratio;
        let branch_entropy = if p > 1e-12 && p < 1.0 - 1e-12 {
            -p * p.log2() - (1.0 - p) * (1.0 - p).log2()
        } else {
            0.0
        };

        // Toggle rate: fraction of adjacent changes
        let mut toggles = 0usize;
        for i in 1..total {
            if binary[i] != binary[i - 1] {
                toggles += 1;
            }
        }
        let toggle_rate = if total > 1 { toggles as f64 / (total - 1) as f64 } else { 0.0 };

        // Run-length statistics
        let mut run_lengths = Vec::new();
        let mut current_run = 1usize;
        for i in 1..total {
            if binary[i] == binary[i - 1] {
                current_run += 1;
            } else {
                run_lengths.push(current_run as f64);
                current_run = 1;
            }
        }
        run_lengths.push(current_run as f64);
        let run_length_mean = run_lengths.iter().sum::<f64>() / run_lengths.len() as f64;

        // Predictability: 1 - entropy (1.0 = fully predictable)
        let predictability_score = 1.0 - branch_entropy;

        let mut result = HashMap::new();
        result.insert("branch_entropy".to_string(), vec![branch_entropy]);
        result.insert("bias_ratio".to_string(), vec![bias_ratio]);
        result.insert("run_length_mean".to_string(), vec![run_length_mean]);
        result.insert("toggle_rate".to_string(), vec![toggle_rate]);
        result.insert("predictability_score".to_string(), vec![predictability_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_biased_branch() {
        // Mostly positive values = highly biased, predictable
        let data: Vec<f64> = (0..20).map(|i| if i < 18 { 10.0 } else { -1.0 }).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = BranchPredictLens.scan(&data, 20, 1, &shared);
        assert!(result.contains_key("bias_ratio"));
        assert!(result["predictability_score"][0] > 0.5);
    }

    #[test]
    fn test_alternating_unpredictable() {
        // Alternating pattern: each value unique so median splits cleanly
        let data: Vec<f64> = (0..20).map(|i| if i % 2 == 0 { 10.0 + i as f64 } else { -(10.0 + i as f64) }).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = BranchPredictLens.scan(&data, 20, 1, &shared);
        assert!(result["toggle_rate"][0] > 0.8);
    }

    #[test]
    fn test_small_data_returns_empty() {
        let data = vec![1.0, 2.0];
        let shared = SharedData::compute(&data, 2, 1);
        let result = BranchPredictLens.scan(&data, 2, 1, &shared);
        assert!(result.is_empty());
    }
}
