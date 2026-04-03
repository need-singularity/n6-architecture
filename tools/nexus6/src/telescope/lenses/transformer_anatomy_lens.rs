use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, column_vectors, shannon_entropy};

/// n=6 transformer constants (BT-33, BT-54, BT-56, BT-58)
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const N: f64 = 6.0;
const J2: f64 = 24.0;
const SOPFR: f64 = 5.0;
const SIGMA_MINUS_TAU: f64 = 8.0; // universal AI constant

/// Transformer architecture constants from BT-33, BT-56
const TRANSFORMER_CONSTANTS: &[(f64, &str)] = &[
    (12.0, "sigma (heads, BERT layers)"),
    (8.0, "sigma-tau (KV heads, LoRA rank base)"),
    (24.0, "J2 (GPT-3 layers)"),
    (6.0, "n (attention heads base)"),
    (128.0, "2^(sigma-sopfr) d_head"),
    (256.0, "2^(sigma-tau) width"),
    (512.0, "2^(sigma-tau+1)"),
    (768.0, "sigma*2^n (BERT d_model)"),
    (1024.0, "2^(sigma-phi)"),
    (2048.0, "2^(sigma-mu)"),
    (4096.0, "2^sigma (GPT-3 d_model)"),
    (2.667, "SwiGLU_ratio 8/3"),
    (0.1, "1/(sigma-phi) weight_decay"),
    (0.288, "Mertens_dropout ln(4/3)"),
    (32.0, "2^sopfr (layers)"),
    (16.0, "2^tau (FP16)"),
    (96.0, "sigma*(sigma-tau) (GPT-3 layers)"),
    (48.0, "sigma*tau"),
    (64.0, "2^n (d_k typical)"),
];

const REL_TOL: f64 = 0.05;

/// TransformerAnatomyLens: Detect transformer architecture patterns in data.
///
/// Algorithm:
///   1. Match feature means against transformer-specific n=6 constants
///   2. Detect head count patterns (multiples of σ-τ=8 or σ=12)
///   3. Check dimension alignment (powers of 2 in {64, 128, 256, 512, ...})
///   4. Detect SwiGLU ratio 8/3 in feature ratios
///   5. Measure layer regularity (uniform spacing = transformer blocks)
///   6. Reports head_pattern, dimension_alignment, swiglu_ratio, transformer_score
pub struct TransformerAnatomyLens;

impl Lens for TransformerAnatomyLens {
    fn name(&self) -> &str {
        "TransformerAnatomyLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let (means, vars) = mean_var(data, n, d);
        let columns = column_vectors(data, n, d);

        // 1. Transformer constant matching
        let mut n6_matches = 0;
        let mut matched = Vec::new();
        for &m in &means {
            for &(c, _) in TRANSFORMER_CONSTANTS {
                if c.abs() < 1e-12 { continue; }
                let tol = if c.abs() > 1.0 { REL_TOL } else { 0.15 };
                if ((m - c) / c).abs() < tol {
                    n6_matches += 1;
                    matched.push(c);
                    break;
                }
            }
        }

        // 2. Head count pattern: check if values are multiples of 8 or 12
        let mut head_pattern_score = 0.0;
        let mut head_count = 0;
        for &m in &means {
            if m > 0.5 {
                let rounded = m.round();
                // Check if divisible by σ-τ=8
                let mod8 = rounded % SIGMA_MINUS_TAU;
                let mod12 = rounded % SIGMA;
                let mod6 = rounded % N;
                if mod8.abs() < 0.5 || mod12.abs() < 0.5 || mod6.abs() < 0.5 {
                    head_count += 1;
                }
            }
        }
        if d > 0 {
            head_pattern_score = head_count as f64 / d as f64;
        }

        // 3. Dimension alignment: check for powers of 2 in range [64, 16384]
        let mut dim_aligned = 0;
        for &m in &means {
            if m > 32.0 && m < 20000.0 {
                let log2 = m.log2();
                if (log2 - log2.round()).abs() < 0.1 {
                    dim_aligned += 1;
                }
            }
        }
        let dimension_alignment = dim_aligned as f64 / d.max(1) as f64;

        // 4. SwiGLU ratio 8/3 ≈ 2.667 detection in feature ratios
        let swiglu_target = 8.0 / 3.0;
        let mut swiglu_matches = 0;
        let mut total_ratios = 0;
        for i in 0..d {
            for j in 0..d {
                if i == j { continue; }
                if means[j].abs() > 1e-12 {
                    let ratio = means[i] / means[j];
                    total_ratios += 1;
                    if (ratio - swiglu_target).abs() < 0.2 {
                        swiglu_matches += 1;
                    }
                }
            }
        }
        let swiglu_ratio_score = if total_ratios > 0 {
            swiglu_matches as f64 / total_ratios as f64
        } else {
            0.0
        };

        // 5. Layer regularity: check if rows have uniform structure
        //    (transformer blocks are identical, so variance should be low per feature)
        let mean_cv = if d > 0 {
            let mut cv_sum = 0.0;
            let mut count = 0;
            for j in 0..d {
                if means[j].abs() > 1e-12 {
                    cv_sum += vars[j].sqrt() / means[j].abs();
                    count += 1;
                }
            }
            if count > 0 { cv_sum / count as f64 } else { 1.0 }
        } else {
            1.0
        };
        let layer_regularity = (-mean_cv * 2.0).exp();

        // 6. Attention sparsity pattern: check for 1/2 + 1/3 + 1/6 = 1 distribution
        //    Look at data distribution across features
        let mut egyptian_score = 0.0;
        if d >= 3 {
            // Normalize means to sum to 1 and check Egyptian fraction pattern
            let total: f64 = means.iter().map(|m| m.abs()).sum();
            if total > 1e-12 {
                let normed: Vec<f64> = means.iter().map(|m| m.abs() / total).collect();
                let mut sorted_normed = normed.clone();
                sorted_normed.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
                // Check if top-3 ratios match 1/2, 1/3, 1/6
                if sorted_normed.len() >= 3 {
                    let diff = (sorted_normed[0] - 0.5).abs()
                        + (sorted_normed[1] - 1.0 / 3.0).abs()
                        + (sorted_normed[2] - 1.0 / 6.0).abs();
                    egyptian_score = (-diff * 5.0).exp();
                }
            }
        }

        // Combined transformer score
        let transformer_score = (n6_matches as f64 / d.max(1) as f64) * 0.25
            + head_pattern_score * 0.2
            + dimension_alignment * 0.2
            + layer_regularity * 0.15
            + swiglu_ratio_score * 0.1
            + egyptian_score * 0.1;

        let mut result = HashMap::new();
        result.insert("n6_transformer_matches".to_string(), vec![n6_matches as f64]);
        result.insert("matched_constants".to_string(), matched);
        result.insert("head_pattern_score".to_string(), vec![head_pattern_score]);
        result.insert("dimension_alignment".to_string(), vec![dimension_alignment]);
        result.insert("swiglu_ratio_score".to_string(), vec![swiglu_ratio_score]);
        result.insert("layer_regularity".to_string(), vec![layer_regularity]);
        result.insert("egyptian_fraction_score".to_string(), vec![egyptian_score]);
        result.insert("transformer_score".to_string(), vec![transformer_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_transformer_detects_architecture_constants() {
        // d_model=4096, n_heads=12, d_head=128, layers=96
        let n = 10;
        let d = 4;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let noise = (i as f64 * 0.01).sin() * 0.1;
            data.push(4096.0 + noise);
            data.push(12.0 + noise * 0.01);
            data.push(128.0 + noise);
            data.push(96.0 + noise * 0.1);
        }
        let shared = make_shared(&data, n, d);
        let result = TransformerAnatomyLens.scan(&data, n, d, &shared);

        let matches = result["n6_transformer_matches"][0];
        assert!(matches >= 3.0, "Should match transformer constants, got {matches}");
    }

    #[test]
    fn test_transformer_head_pattern() {
        // Values that are multiples of 8 (σ-τ)
        let n = 10;
        let d = 3;
        let mut data = Vec::with_capacity(n * d);
        for _i in 0..n {
            data.push(8.0);
            data.push(16.0);
            data.push(24.0);
        }
        let shared = make_shared(&data, n, d);
        let result = TransformerAnatomyLens.scan(&data, n, d, &shared);

        let hp = result["head_pattern_score"][0];
        assert!(hp > 0.5, "Multiples of 8 should give head pattern, got {hp}");
    }

    #[test]
    fn test_transformer_layer_regularity() {
        // Uniform rows = high layer regularity
        let n = 20;
        let d = 3;
        let mut data = Vec::with_capacity(n * d);
        for _i in 0..n {
            data.push(768.0);
            data.push(12.0);
            data.push(64.0);
        }
        let shared = make_shared(&data, n, d);
        let result = TransformerAnatomyLens.scan(&data, n, d, &shared);

        let lr = result["layer_regularity"][0];
        assert!(lr > 0.9, "Uniform data should give high regularity, got {lr}");
    }
}
