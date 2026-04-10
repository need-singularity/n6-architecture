use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TachyonLens: Faster-than-expected propagation.
/// Measures superluminal fraction, tachyon energy, causality violation, group velocity ratio.
pub struct TachyonLens;

impl Lens for TachyonLens {
    fn name(&self) -> &str { "TachyonLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Compute "velocities" = distances between consecutive points
        let velocities: Vec<f64> = (1..max_n).map(|i| shared.dist(i, i - 1)).collect();
        if velocities.is_empty() { return HashMap::new(); }

        let mean_vel = velocities.iter().sum::<f64>() / velocities.len() as f64;
        let std_vel = (velocities.iter().map(|&v| (v - mean_vel).powi(2)).sum::<f64>()
            / velocities.len() as f64).sqrt().max(1e-12);

        // "Speed limit" = mean + 2*std (expected maximum)
        let speed_limit = mean_vel + 2.0 * std_vel;

        // Superluminal fraction: fraction exceeding speed limit
        let superluminal = velocities.iter().filter(|&&v| v > speed_limit).count();
        let superluminal_fraction = superluminal as f64 / velocities.len() as f64;

        // Tachyon energy: imaginary mass analog
        // For tachyonic particles, E^2 = p^2 - m^2 (negative m^2)
        // Proxy: sum of (v^2 - limit^2) for superluminal points
        let limit_sq = speed_limit * speed_limit;
        let tachyon_energy: f64 = velocities.iter()
            .filter(|&&v| v > speed_limit)
            .map(|&v| v * v - limit_sq)
            .sum::<f64>();
        let tachyon_energy = tachyon_energy / max_n as f64;

        // Causality violation: effect before cause
        // Check if signal[i+1] predicts signal[i] better than signal[i] predicts signal[i+1]
        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let mut forward_err = 0.0f64;
        let mut backward_err = 0.0f64;
        for i in 1..(max_n - 1) {
            forward_err += (signal[i + 1] - signal[i]).powi(2);
            backward_err += (signal[i - 1] - signal[i]).powi(2);
        }
        let causality_violation = if forward_err + backward_err > 1e-12 {
            ((backward_err - forward_err) / (forward_err + backward_err)).abs()
        } else {
            0.0
        };

        // Group velocity ratio: ratio of envelope speed to phase speed
        // Phase speed ~ mean velocity, group speed ~ derivative of velocity
        let vel_diffs: Vec<f64> = velocities.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
        let group_speed = if !vel_diffs.is_empty() {
            vel_diffs.iter().sum::<f64>() / vel_diffs.len() as f64
        } else { 0.0 };
        let group_velocity_ratio = if mean_vel > 1e-12 {
            group_speed / mean_vel
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("superluminal_fraction".to_string(), vec![superluminal_fraction]);
        result.insert("tachyon_energy".to_string(), vec![tachyon_energy]);
        result.insert("causality_violation".to_string(), vec![causality_violation]);
        result.insert("group_velocity_ratio".to_string(), vec![group_velocity_ratio]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Mostly smooth with one big jump
        let mut data: Vec<f64> = (0..20).map(|i| i as f64 * 0.1).collect();
        data[10] = 50.0; // superluminal jump
        let shared = SharedData::compute(&data, 20, 1);
        let result = TachyonLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["superluminal_fraction"][0] > 0.0);
    }
}
