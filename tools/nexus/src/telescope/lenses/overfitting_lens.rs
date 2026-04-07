use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// OverfittingLens: Overfitting / generalization gap detection.
///
/// Detects overfitting-like signatures in data: excessive local complexity
/// relative to global structure, memorization patterns, and poor
/// generalization indicators.
///
/// Metrics:
///   1. complexity_ratio: local complexity / global complexity (>1 = overfit)
///   2. memorization_score: how much data is perfectly "memorized" (zero residual)
///   3. leave_one_out_variance: LOO stability (high = overfitting)
///   4. noise_sensitivity: how much small perturbations change local structure
///   5. bias_variance_ratio: estimated bias/variance decomposition
///   6. regularization_need: estimated regularization strength needed
///
/// n=6: Optimal dropout p=ln(4/3)≈0.288 (BT-46, Mertens). Weight decay
///       λ=1/(σ-φ)=0.1 (BT-64). LoRA rank r=σ-τ=8 prevents overfit.
///       Chinchilla ratio tokens/params=J₂-τ=20 (BT-26).
pub struct OverfittingLens;

impl Lens for OverfittingLens {
    fn name(&self) -> &str { "OverfittingLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 8 || d < 1 { return HashMap::new(); }

        let max_n = n.min(200);
        let columns = column_vectors(data, n, d);
        let (_means, vars) = mean_var(data, n, d);

        // 1. Complexity ratio: local vs global structure complexity
        // Local: mean variance within KNN neighborhoods
        // Global: overall variance
        let global_var: f64 = vars.iter().sum::<f64>() / d as f64;
        if global_var < 1e-15 {
            let mut result = HashMap::new();
            result.insert("complexity_ratio".to_string(), vec![0.0]);
            result.insert("memorization_score".to_string(), vec![0.0]);
            result.insert("leave_one_out_variance".to_string(), vec![0.0]);
            result.insert("noise_sensitivity".to_string(), vec![0.0]);
            result.insert("bias_variance_ratio".to_string(), vec![0.0]);
            result.insert("regularization_need".to_string(), vec![0.0]);
            return result;
        }

        let mut local_var_sum = 0.0f64;
        let mut local_count = 0u32;
        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.len() < 2 { continue; }
            // Variance of distances to neighbors
            let dists: Vec<f64> = knn.iter()
                .map(|&j| shared.dist(i, j as usize))
                .collect();
            let mean_d = dists.iter().sum::<f64>() / dists.len() as f64;
            let var_d = dists.iter().map(|d| (d - mean_d) * (d - mean_d)).sum::<f64>()
                / dists.len() as f64;
            local_var_sum += var_d;
            local_count += 1;
        }
        let local_var = if local_count > 0 { local_var_sum / local_count as f64 } else { 0.0 };
        let complexity_ratio = local_var / global_var;

        // 2. Memorization: fraction of points with near-zero distance to nearest neighbor
        let mut memorized = 0u32;
        let median_density = {
            let mut densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
            densities.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            densities[densities.len() / 2]
        };
        let mem_threshold = if median_density > 0.0 { 1.0 / median_density * 0.01 } else { 1e-10 };
        for i in 0..max_n {
            let knn = shared.knn(i);
            if !knn.is_empty() {
                let nearest_dist = shared.dist(i, knn[0] as usize);
                if nearest_dist < mem_threshold { memorized += 1; }
            }
        }
        let memorization_score = memorized as f64 / max_n as f64;

        // 3. Leave-one-out variance: how much does removing a point change local structure?
        let mut loo_var_sum = 0.0f64;
        let sample_n = max_n.min(50);
        for i in 0..sample_n {
            let knn = shared.knn(i);
            if knn.len() < 3 { continue; }
            // Compare: mean dist with all neighbors vs without first neighbor
            let full_mean: f64 = knn.iter()
                .map(|&j| shared.dist(i, j as usize))
                .sum::<f64>() / knn.len() as f64;
            let partial_mean: f64 = knn[1..].iter()
                .map(|&j| shared.dist(i, j as usize))
                .sum::<f64>() / (knn.len() - 1) as f64;
            let diff = (full_mean - partial_mean).abs();
            loo_var_sum += diff * diff;
        }
        let loo_variance = loo_var_sum / sample_n as f64;

        // 4. Noise sensitivity: compare gradients across dimensions
        let mut sensitivity_sum = 0.0f64;
        for col in &columns {
            if col.len() < 3 { continue; }
            let mut grads: Vec<f64> = Vec::new();
            for i in 1..col.len().min(max_n) {
                grads.push((col[i] - col[i - 1]).abs());
            }
            if grads.is_empty() { continue; }
            let mean_grad = grads.iter().sum::<f64>() / grads.len() as f64;
            let grad_var = grads.iter()
                .map(|g| (g - mean_grad) * (g - mean_grad))
                .sum::<f64>() / grads.len() as f64;
            sensitivity_sum += grad_var;
        }
        let noise_sensitivity = sensitivity_sum / d as f64 / global_var;

        // 5. Bias-variance decomposition estimate
        // High local var + low global var = high variance (overfit)
        // Low local var + high global var = high bias (underfit)
        let bias_estimate = global_var - local_var;
        let variance_estimate = local_var;
        let bv_ratio = if variance_estimate > 1e-15 {
            bias_estimate.abs() / variance_estimate
        } else {
            0.0
        };

        // 6. Regularization need: composite score
        // High complexity ratio + high noise sensitivity + high memorization → needs regularization
        let reg_need = (complexity_ratio.min(2.0) / 2.0 * 0.3
            + noise_sensitivity.min(5.0) / 5.0 * 0.3
            + memorization_score * 0.2
            + (1.0 / (1.0 + bv_ratio)) * 0.2)
            .clamp(0.0, 1.0);

        let mut result = HashMap::new();
        result.insert("complexity_ratio".to_string(), vec![complexity_ratio]);
        result.insert("memorization_score".to_string(), vec![memorization_score]);
        result.insert("leave_one_out_variance".to_string(), vec![loo_variance]);
        result.insert("noise_sensitivity".to_string(), vec![noise_sensitivity]);
        result.insert("bias_variance_ratio".to_string(), vec![bv_ratio]);
        result.insert("regularization_need".to_string(), vec![reg_need]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_overfitting_lens_basic() {
        let mut data = Vec::new();
        for i in 0..30 {
            data.push(i as f64 * 0.1);
            data.push((i as f64 * 0.3).sin());
        }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = OverfittingLens.scan(&data, n, 2, &shared);
        assert!(result.contains_key("complexity_ratio"));
        assert!(result.contains_key("regularization_need"));
        assert!(result["regularization_need"][0] >= 0.0);
        assert!(result["regularization_need"][0] <= 1.0);
    }

    #[test]
    fn test_overfitting_lens_noisy() {
        // Noisy data should show higher overfitting indicators
        let n = 40;
        let data: Vec<f64> = (0..n * 2)
            .map(|i| ((i as f64 * 7.3).sin() * 100.0 + (i as f64 * 0.01).cos()))
            .collect();
        let shared = SharedData::compute(&data, n, 2);
        let result = OverfittingLens.scan(&data, n, 2, &shared);
        assert!(result.contains_key("noise_sensitivity"));
    }
}
