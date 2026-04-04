use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// LoRALens: Low-Rank Adaptation structure detection.
///
/// Detects low-rank structure in data — analogous to LoRA/QLoRA where
/// a high-dimensional weight matrix W is approximated by W + BA where
/// B∈R^{d×r}, A∈R^{r×d} with rank r << d.
///
/// Metrics:
///   1. effective_rank: estimated intrinsic dimensionality (numeric rank)
///   2. rank_ratio: effective_rank / actual_d (low = highly compressible)
///   3. top_r_explained: variance explained by top r=σ-τ=8 components
///   4. lora_efficiency: how well rank-r approximation captures the data
///   5. quantization_error: discretization error at 4-bit (QLoRA analogy)
///   6. adapter_fraction: fraction of dimensions that are "active" (LoRA adapters)
///
/// n=6: LoRA rank r=σ-τ=8 is the universal AI constant (BT-58).
///       QLoRA 4-bit = τ bits. LoRA α/r = φ=2 typical scaling.
///       Full-rank = d, adapter rank = r << d, savings = 1 - r/d.
pub struct LoRALens;

impl Lens for LoRALens {
    fn name(&self) -> &str { "LoRALens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d < 2 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let (_means, vars) = mean_var(data, n, d);

        // 1. Estimate effective rank via variance decomposition
        // Sort variances descending (proxy for eigenvalues of covariance)
        let mut sorted_vars = vars.clone();
        sorted_vars.sort_by(|a, b| b.partial_cmp(a).unwrap());

        let total_var: f64 = sorted_vars.iter().sum();
        if total_var < 1e-15 {
            let mut result = HashMap::new();
            result.insert("effective_rank".to_string(), vec![0.0]);
            result.insert("rank_ratio".to_string(), vec![0.0]);
            result.insert("top_r_explained".to_string(), vec![0.0]);
            result.insert("lora_efficiency".to_string(), vec![0.0]);
            result.insert("quantization_error".to_string(), vec![0.0]);
            result.insert("adapter_fraction".to_string(), vec![0.0]);
            return result;
        }

        // Effective rank (Shannon entropy of normalized eigenvalues)
        let normalized: Vec<f64> = sorted_vars.iter()
            .map(|&v| v / total_var)
            .filter(|&p| p > 1e-15)
            .collect();
        let entropy: f64 = -normalized.iter()
            .map(|&p| p * p.ln())
            .sum::<f64>();
        let effective_rank = entropy.exp();
        let rank_ratio = effective_rank / d as f64;

        // 2. Top r=8 (σ-τ) explained variance (LoRA rank)
        let r = 8.min(d); // σ-τ = 8
        let top_r_var: f64 = sorted_vars[..r].iter().sum();
        let top_r_explained = top_r_var / total_var;

        // 3. LoRA efficiency: how much can we compress?
        // Find minimal rank that explains 95% variance
        let mut cumulative = 0.0;
        let mut min_rank_95 = d;
        for (i, &v) in sorted_vars.iter().enumerate() {
            cumulative += v;
            if cumulative / total_var >= 0.95 {
                min_rank_95 = i + 1;
                break;
            }
        }
        let lora_efficiency = 1.0 - (min_rank_95 as f64 / d as f64);

        // 4. Quantization error: 4-bit (QLoRA) quantization simulation
        // Quantize each column to 16 levels (4-bit = 2^4)
        let n_levels = 16u32; // τ bits → 2^τ = 16 levels
        let mut quant_error_sum = 0.0f64;
        for col in &columns {
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for &v in col { if v < lo { lo = v; } if v > hi { hi = v; } }
            let range = (hi - lo).max(1e-12);
            let scale = (n_levels - 1) as f64 / range;

            let col_var = col.iter()
                .map(|v| {
                    let q = ((v - lo) * scale).round() / scale + lo;
                    let err = v - q;
                    err * err
                })
                .sum::<f64>() / n as f64;
            quant_error_sum += col_var;
        }
        let quantization_error = quant_error_sum / d as f64;
        // Normalize by total variance
        let normalized_quant_error = quantization_error / (total_var / d as f64 + 1e-15);

        // 5. Adapter fraction: dims with significant variance / total dims
        let var_threshold = total_var / d as f64 * 0.1; // 10% of mean variance
        let active_dims = vars.iter().filter(|&&v| v > var_threshold).count();
        let adapter_fraction = active_dims as f64 / d as f64;

        let mut result = HashMap::new();
        result.insert("effective_rank".to_string(), vec![effective_rank]);
        result.insert("rank_ratio".to_string(), vec![rank_ratio]);
        result.insert("top_r_explained".to_string(), vec![top_r_explained]);
        result.insert("lora_efficiency".to_string(), vec![lora_efficiency]);
        result.insert("quantization_error".to_string(), vec![normalized_quant_error]);
        result.insert("adapter_fraction".to_string(), vec![adapter_fraction]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lora_lens_low_rank() {
        // Data with rank-1 structure: all variation in first dimension
        let n = 50;
        let d = 10;
        let mut data = vec![0.0; n * d];
        for i in 0..n {
            data[i * d] = i as f64; // only dim 0 varies
            for j in 1..d {
                data[i * d + j] = 0.5; // constant
            }
        }
        let shared = SharedData::compute(&data, n, d);
        let result = LoRALens.scan(&data, n, d, &shared);
        assert!(result.contains_key("effective_rank"));
        assert!(result["rank_ratio"][0] < 0.5, "Low-rank data should have low rank ratio");
        assert!(result["lora_efficiency"][0] > 0.5, "Should be highly compressible");
    }

    #[test]
    fn test_lora_lens_full_rank() {
        // Data with independent variation in all dimensions
        let n = 50;
        let d = 5;
        let mut data = vec![0.0; n * d];
        for i in 0..n {
            for j in 0..d {
                data[i * d + j] = ((i * (j + 1)) as f64 * 0.37).sin() * (j + 1) as f64;
            }
        }
        let shared = SharedData::compute(&data, n, d);
        let result = LoRALens.scan(&data, n, d, &shared);
        assert!(result["rank_ratio"][0] > 0.3, "Full-rank data should have higher rank ratio");
    }
}
