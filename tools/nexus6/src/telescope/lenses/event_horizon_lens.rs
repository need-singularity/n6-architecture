use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// EventHorizonLens: Point of no return detection.
/// Measures horizon radius, escape fraction, information loss, hawking temperature.
pub struct EventHorizonLens;

impl Lens for EventHorizonLens {
    fn name(&self) -> &str { "EventHorizonLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Find centroid
        let mut centroid = vec![0.0f64; d];
        for i in 0..max_n {
            for j in 0..d { centroid[j] += data[i * d + j]; }
        }
        for j in 0..d { centroid[j] /= max_n as f64; }

        // Compute distances from centroid
        let mut radii: Vec<f64> = (0..max_n).map(|i| {
            let mut r2 = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - centroid[j];
                r2 += diff * diff;
            }
            r2.sqrt()
        }).collect();
        radii.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let median_r = radii[max_n / 2];

        // Horizon radius: find radius where density drops sharply
        // Use cumulative fraction; horizon = radius containing 90% of points
        let horizon_idx = (max_n as f64 * 0.9) as usize;
        let horizon_radius = if horizon_idx < radii.len() { radii[horizon_idx] } else { *radii.last().unwrap() };

        // Escape fraction: points beyond horizon
        let beyond = radii.iter().filter(|&&r| r > horizon_radius).count();
        let escape_fraction = beyond as f64 / max_n as f64;

        // Information loss: entropy difference inside vs outside horizon
        let inner: Vec<f64> = (0..max_n)
            .filter(|&i| {
                let mut r2 = 0.0;
                for j in 0..d { let diff = data[i * d + j] - centroid[j]; r2 += diff * diff; }
                r2.sqrt() <= horizon_radius
            })
            .map(|i| data[i * d])
            .collect();
        let outer: Vec<f64> = (0..max_n)
            .filter(|&i| {
                let mut r2 = 0.0;
                for j in 0..d { let diff = data[i * d + j] - centroid[j]; r2 += diff * diff; }
                r2.sqrt() > horizon_radius
            })
            .map(|i| data[i * d])
            .collect();

        let entropy_inner = shannon_entropy(&inner, 10);
        let entropy_outer = if outer.len() >= 2 { shannon_entropy(&outer, 10) } else { 0.0 };
        let information_loss = (entropy_outer - entropy_inner).abs();

        // Hawking temperature: boundary radiation analog = density gradient at horizon
        let shell_width = median_r * 0.2;
        let near_horizon: Vec<f64> = radii.iter()
            .filter(|&&r| (r - horizon_radius).abs() < shell_width)
            .cloned().collect();
        let hawking_temperature = if near_horizon.len() >= 2 {
            let shell_density = near_horizon.len() as f64 / (2.0 * shell_width).max(1e-12);
            1.0 / (shell_density.max(1e-12))
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("horizon_radius".to_string(), vec![horizon_radius]);
        result.insert("escape_fraction".to_string(), vec![escape_fraction]);
        result.insert("information_loss".to_string(), vec![information_loss]);
        result.insert("hawking_temperature".to_string(), vec![hawking_temperature]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..20).map(|i| (i as f64 - 10.0) * 0.5).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = EventHorizonLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("horizon_radius"));
        assert!(result["escape_fraction"][0] >= 0.0);
    }
}
