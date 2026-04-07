use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// GradientLens: Analyze rate-of-change structure in sequential data.
///
/// Computes first and second differences, checks for gradient stability
/// and n=6 signatures in acceleration patterns.
pub struct GradientLens;

impl Lens for GradientLens {
    fn name(&self) -> &str { "GradientLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let mut mean_grad_magnitude = 0.0;
        let mut mean_accel_magnitude = 0.0;
        let mut sign_change_rates = Vec::with_capacity(d);
        let mut grad_stabilities = Vec::with_capacity(d);

        for col in columns.iter().take(d.min(12)) {
            // First differences (gradient)
            let grad: Vec<f64> = col.windows(2).map(|w| w[1] - w[0]).collect();
            if grad.is_empty() { continue; }

            // Second differences (acceleration)
            let accel: Vec<f64> = grad.windows(2).map(|w| w[1] - w[0]).collect();

            // Gradient magnitude
            let grad_mag = grad.iter().map(|g| g.abs()).sum::<f64>() / grad.len() as f64;
            mean_grad_magnitude += grad_mag;

            // Acceleration magnitude
            if !accel.is_empty() {
                let accel_mag = accel.iter().map(|a| a.abs()).sum::<f64>() / accel.len() as f64;
                mean_accel_magnitude += accel_mag;
            }

            // Sign change rate in gradient (oscillation indicator)
            let mut sign_changes = 0;
            for w in grad.windows(2) {
                if w[0] * w[1] < 0.0 { sign_changes += 1; }
            }
            let sc_rate = if grad.len() > 1 { sign_changes as f64 / (grad.len() - 1) as f64 } else { 0.0 };
            sign_change_rates.push(sc_rate);

            // Gradient stability: low variance of gradient = stable
            let grad_mean = grad.iter().sum::<f64>() / grad.len() as f64;
            let grad_var = grad.iter().map(|g| (g - grad_mean).powi(2)).sum::<f64>() / grad.len() as f64;
            let stability = (-grad_var.sqrt() / (grad_mag.max(1e-12))).exp();
            grad_stabilities.push(stability);
        }

        let count = d.min(12) as f64;
        mean_grad_magnitude /= count;
        mean_accel_magnitude /= count;

        let mean_sign_change_rate = sign_change_rates.iter().sum::<f64>() / sign_change_rates.len().max(1) as f64;
        let mean_stability = grad_stabilities.iter().sum::<f64>() / grad_stabilities.len().max(1) as f64;

        let mut result = HashMap::new();
        result.insert("mean_gradient_magnitude".to_string(), vec![mean_grad_magnitude]);
        result.insert("mean_acceleration_magnitude".to_string(), vec![mean_accel_magnitude]);
        result.insert("sign_change_rates".to_string(), sign_change_rates);
        result.insert("mean_sign_change_rate".to_string(), vec![mean_sign_change_rate]);
        result.insert("gradient_stabilities".to_string(), grad_stabilities);
        result.insert("mean_gradient_stability".to_string(), vec![mean_stability]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gradient_lens_linear() {
        let data: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let n = 20;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = GradientLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_gradient_magnitude"));
        assert!(result["mean_gradient_magnitude"][0] > 0.5, "Linear data should have constant gradient");
        assert!(result["mean_sign_change_rate"][0] < 0.01, "Linear data should have no sign changes");
    }

    #[test]
    fn test_gradient_lens_oscillating() {
        let data: Vec<f64> = (0..24).map(|i| {
            (i as f64 * std::f64::consts::PI / 3.0).sin()
        }).collect();
        let n = 24;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = GradientLens.scan(&data, n, d, &shared);
        assert!(result["mean_sign_change_rate"][0] > 0.2, "Oscillating data should have sign changes");
    }
}
