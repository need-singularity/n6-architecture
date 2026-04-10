use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SimdOpportunityLens: Find vectorizable loops and operations.
///
/// Detects SIMD-friendly patterns: uniform operations across dimensions,
/// aligned data, and absence of cross-lane dependencies. LLVM's
/// auto-vectorizer misses many cases involving non-contiguous access,
/// conditional operations, or mixed-width data.
///
/// Metrics:
///   1. uniform_op_score: how uniform operations are across dimensions
///   2. alignment_score: data alignment friendliness (powers of 2/4/8)
///   3. lane_independence: fraction of dims with independent computation
///   4. vectorizable_fraction: estimated fraction of work that can be SIMD'd
///   5. optimal_vector_width: recommended SIMD width (128/256/512 bits)
///   6. predication_need: fraction requiring masked/predicated operations
pub struct SimdOpportunityLens;

impl Lens for SimdOpportunityLens {
    fn name(&self) -> &str { "SimdOpportunityLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // --- Uniform operation score: do all dimensions behave similarly? ---
        // Compare per-dim statistics (mean, var, range)
        let mut dim_means = vec![0.0f64; d];
        let mut dim_vars = vec![0.0f64; d];
        for i in 0..max_n {
            for dim in 0..d {
                dim_means[dim] += data[i * d + dim];
            }
        }
        for dim in 0..d { dim_means[dim] /= max_n as f64; }
        for i in 0..max_n {
            for dim in 0..d {
                let diff = data[i * d + dim] - dim_means[dim];
                dim_vars[dim] += diff * diff;
            }
        }
        for dim in 0..d { dim_vars[dim] /= max_n as f64; }

        // Coefficient of variation of the variances across dims
        let mean_var = dim_vars.iter().sum::<f64>() / d as f64;
        let var_of_vars = dim_vars.iter()
            .map(|&v| (v - mean_var) * (v - mean_var))
            .sum::<f64>() / d as f64;
        let uniform_op_score = if mean_var > 1e-12 {
            1.0 - (var_of_vars.sqrt() / mean_var).min(1.0)
        } else { 1.0 }; // constant data is perfectly uniform

        // --- Alignment score: is d a power of 2, or multiple of SIMD width? ---
        let alignment_score = if d % 8 == 0 { 1.0 }
            else if d % 4 == 0 { 0.8 }
            else if d % 2 == 0 { 0.5 }
            else { 0.2 };

        // --- Lane independence: measure MI between dimensions ---
        // Low MI between dims = independent lanes = SIMD friendly
        let d_check = d.min(16);
        let mut total_mi = 0.0f64;
        let mut mi_pairs = 0usize;
        for di in 0..d_check {
            for dj in (di + 1)..d_check {
                total_mi += shared.mi(di, dj);
                mi_pairs += 1;
            }
        }
        let avg_mi = if mi_pairs > 0 { total_mi / mi_pairs as f64 } else { 0.0 };
        // Low MI = high independence
        let lane_independence = (-avg_mi * 3.0).exp().min(1.0); // exponential decay

        // --- Vectorizable fraction: combine signals ---
        // Check for data-dependent branches (high local variance = branching)
        let mut needs_predication = 0usize;
        for i in 0..max_n {
            let row = &data[i * d..(i * d + d).min(data.len())];
            // Check if any element is an "outlier" within the row (would need masking)
            let row_mean = row.iter().sum::<f64>() / d as f64;
            let row_std = (row.iter().map(|&v| (v - row_mean) * (v - row_mean)).sum::<f64>()
                / d as f64).sqrt();
            if row_std > 1e-12 {
                let outliers = row.iter()
                    .filter(|&&v| (v - row_mean).abs() > 2.5 * row_std)
                    .count();
                if outliers > 0 { needs_predication += 1; }
            }
        }
        let predication_need = needs_predication as f64 / max_n as f64;

        let vectorizable_fraction = (uniform_op_score * 0.3
            + alignment_score * 0.2
            + lane_independence * 0.3
            + (1.0 - predication_need) * 0.2)
            .min(1.0);

        // --- Optimal vector width ---
        // Based on d and alignment
        let optimal_width = if d >= 8 && d % 8 == 0 { 512.0 }
            else if d >= 4 && d % 4 == 0 { 256.0 }
            else if d >= 2 { 128.0 }
            else { 64.0 };

        let mut result = HashMap::new();
        result.insert("uniform_op_score".to_string(), vec![uniform_op_score]);
        result.insert("alignment_score".to_string(), vec![alignment_score]);
        result.insert("lane_independence".to_string(), vec![lane_independence]);
        result.insert("vectorizable_fraction".to_string(), vec![vectorizable_fraction]);
        result.insert("optimal_vector_width".to_string(), vec![optimal_width]);
        result.insert("predication_need".to_string(), vec![predication_need]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_simd_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = SimdOpportunityLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("vectorizable_fraction"));
        assert!(r["vectorizable_fraction"][0] >= 0.0);
    }

    #[test]
    fn test_simd_aligned() {
        // d=8 is perfectly SIMD-aligned
        let data: Vec<f64> = (0..80).map(|i| (i as f64).sin()).collect();
        let shared = SharedData::compute(&data, 10, 8);
        let r = SimdOpportunityLens.scan(&data, 10, 8, &shared);
        assert_eq!(r["alignment_score"][0], 1.0);
        assert_eq!(r["optimal_vector_width"][0], 512.0);
    }
}
