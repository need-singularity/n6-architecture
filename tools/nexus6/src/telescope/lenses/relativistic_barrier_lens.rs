use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// RelativisticBarrierLens: Asymptotic divergence and speed-limit detection.
///
/// Detects relativistic-barrier-like structures: regions where quantities
/// approach asymptotic limits (mass → ∞, gradients → ∞), analogous to the
/// light-speed barrier where Lorentz factor γ = 1/√(1-v²/c²) → ∞.
///
/// Metrics:
///   1. divergence_points: fraction of points near asymptotic boundary
///   2. lorentz_factor: max detected γ-like ratio (finite-to-infinite scaling)
///   3. barrier_sharpness: how abruptly values approach the limit
///   4. subluminal_fraction: fraction of data safely below the barrier
///   5. gradient_explosion: max gradient magnitude relative to median
///
/// n=6: v/c → 1 barrier. Lorentz group SO(3,1) has dim=n=6.
///       Rest mass energy E=mc² connects via σ(6)=12 in pair production.
///       Relativistic kinetic energy expansion has τ=4 terms at leading order.
pub struct RelativisticBarrierLens;

impl Lens for RelativisticBarrierLens {
    fn name(&self) -> &str { "RelativisticBarrierLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);

        let mut max_lorentz = 0.0f64;
        let mut total_divergence_pts = 0u32;
        let mut max_gradient_ratio = 0.0f64;
        let mut barrier_sharpness_sum = 0.0f64;
        let mut barrier_count = 0u32;

        for col in &columns {
            // Find min/max to define the "speed of light" boundary
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for &v in col.iter() {
                if v < lo { lo = v; }
                if v > hi { hi = v; }
            }
            let range = hi - lo;
            if range < 1e-12 { continue; }

            // "Speed of light" = max value in this dimension
            let c_limit = hi;
            let barrier_zone = range * 0.05; // top 5% = near-barrier

            // Compute gradients (first differences)
            let mut gradients = Vec::with_capacity(n - 1);
            for i in 1..n {
                gradients.push((col[i] - col[i - 1]).abs());
            }
            if gradients.is_empty() { continue; }

            gradients.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            let median_grad = gradients[gradients.len() / 2];
            let max_grad = gradients[gradients.len() - 1];

            if median_grad > 1e-15 {
                let ratio = max_grad / median_grad;
                if ratio > max_gradient_ratio { max_gradient_ratio = ratio; }
            }

            // Lorentz-like factor: γ = 1/√(1 - (v/c)²)
            for (i, &v) in col.iter().enumerate() {
                let v_ratio = (v - lo) / range; // normalized to [0, 1]
                let v_sq = v_ratio * v_ratio;
                if v_sq < 1.0 {
                    let gamma = 1.0 / (1.0 - v_sq).sqrt();
                    if gamma > max_lorentz { max_lorentz = gamma; }
                }

                // Near barrier?
                if (c_limit - v).abs() < barrier_zone {
                    total_divergence_pts += 1;
                }

                // Barrier sharpness: how fast gradient increases near the limit
                if i > 0 && (c_limit - v).abs() < barrier_zone {
                    let grad = (col[i] - col[i - 1]).abs();
                    barrier_sharpness_sum += grad / range;
                    barrier_count += 1;
                }
            }
        }

        let total_pts = (n * d) as f64;
        let divergence_fraction = total_divergence_pts as f64 / total_pts;
        let subluminal_fraction = 1.0 - divergence_fraction;
        let barrier_sharpness = if barrier_count > 0 {
            barrier_sharpness_sum / barrier_count as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("divergence_fraction".to_string(), vec![divergence_fraction]);
        result.insert("max_lorentz_factor".to_string(), vec![max_lorentz]);
        result.insert("barrier_sharpness".to_string(), vec![barrier_sharpness]);
        result.insert("subluminal_fraction".to_string(), vec![subluminal_fraction]);
        result.insert("gradient_explosion".to_string(), vec![max_gradient_ratio]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_relativistic_barrier_basic() {
        // Data approaching asymptote: 1/(1-x) behavior
        let n = 50;
        let data: Vec<f64> = (0..n)
            .map(|i| {
                let x = i as f64 / n as f64 * 0.99; // approach 1.0
                1.0 / (1.0 - x) // diverges as x → 1
            })
            .collect();
        let shared = SharedData::compute(&data, n, 1);
        let result = RelativisticBarrierLens.scan(&data, n, 1, &shared);
        assert!(result.contains_key("max_lorentz_factor"));
        assert!(result["max_lorentz_factor"][0] > 1.0);
        assert!(result["gradient_explosion"][0] > 1.0);
    }

    #[test]
    fn test_relativistic_barrier_uniform() {
        // Uniform data: no barrier
        let n = 30;
        let data: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, n, 1);
        let result = RelativisticBarrierLens.scan(&data, n, 1, &shared);
        assert!(result.contains_key("subluminal_fraction"));
        // Mostly subluminal for linear data
        assert!(result["subluminal_fraction"][0] > 0.8);
    }
}
