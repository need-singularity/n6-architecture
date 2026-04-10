use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PiLens: Detect π-related patterns — circularity, angular uniformity, ratio analysis.
///
/// n=6 connection: π appears in σ(6)·φ(6)=6·τ(6), area of hexagon = 3√3/2·r² ≈ πr².
/// All primes >3 are 6k±1; π itself encodes the structure of n=6 arithmetic.
pub struct PiLens;

impl Lens for PiLens {
    fn name(&self) -> &str {
        "PiLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 {
            return HashMap::new();
        }

        let max_n = n.min(200);
        let pi = std::f64::consts::PI;
        let mut result = HashMap::new();

        // 1. Circularity: how close the point cloud is to lying on a circle/sphere.
        //    Compute mean distance from centroid, then check variance of distances.
        let (means, _) = crate::telescope::shared_data::mean_var(data, n, d);
        let mut radii = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let mut r2 = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - means[j];
                r2 += diff * diff;
            }
            radii.push(r2.sqrt());
        }
        let mean_r = radii.iter().sum::<f64>() / radii.len() as f64;
        let var_r = if mean_r > 1e-12 {
            radii.iter().map(|&r| (r - mean_r).powi(2)).sum::<f64>() / radii.len() as f64
        } else {
            0.0
        };
        let circularity = if mean_r > 1e-12 {
            1.0 - (var_r.sqrt() / mean_r).min(1.0)
        } else {
            0.0
        };
        result.insert("circularity".to_string(), vec![circularity]);

        // 2. Pi ratio: fraction of consecutive value ratios close to π (within 5%).
        let vals: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let mut pi_hits = 0usize;
        let mut ratio_count = 0usize;
        for i in 0..vals.len() - 1 {
            if vals[i].abs() > 1e-12 {
                let ratio = (vals[i + 1] / vals[i]).abs();
                if (ratio - pi).abs() / pi < 0.05 || (1.0 / ratio - pi).abs() / pi < 0.05 {
                    pi_hits += 1;
                }
                ratio_count += 1;
            }
        }
        let pi_ratio = if ratio_count > 0 {
            pi_hits as f64 / ratio_count as f64
        } else {
            0.0
        };
        result.insert("pi_ratio".to_string(), vec![pi_ratio]);

        // 3. Angular uniformity: project to 2D (first 2 dims), compute angles from centroid,
        //    then measure uniformity via circular variance.
        if d >= 2 {
            let mut angles = Vec::with_capacity(max_n);
            for i in 0..max_n {
                let dx = data[i * d] - means[0];
                let dy = data[i * d + 1] - means[1];
                angles.push(dy.atan2(dx));
            }
            // Circular variance: 1 - R, where R = |mean of unit vectors|
            let sum_cos: f64 = angles.iter().map(|&a| a.cos()).sum();
            let sum_sin: f64 = angles.iter().map(|&a| a.sin()).sum();
            let r_bar = ((sum_cos / max_n as f64).powi(2) + (sum_sin / max_n as f64).powi(2)).sqrt();
            let angular_uniformity = 1.0 - r_bar; // 1.0 = perfectly uniform
            result.insert("angular_uniformity".to_string(), vec![angular_uniformity]);
        }

        // 4. Buffon estimate: use consecutive pairs as "needle" crossings.
        //    For 1D data, count how many times sign changes occur relative to grid lines
        //    spaced at interval = range/6 (n=6 connection). Ratio estimates 2/π.
        let (lo, hi) = crate::telescope::shared_data::min_max(&vals);
        let range = hi - lo;
        if range > 1e-12 && vals.len() >= 2 {
            let spacing = range / 6.0; // n=6 grid
            let mut crossings = 0usize;
            for i in 0..vals.len() - 1 {
                let grid_a = ((vals[i] - lo) / spacing).floor();
                let grid_b = ((vals[i + 1] - lo) / spacing).floor();
                if grid_a != grid_b {
                    crossings += 1;
                }
            }
            let crossing_rate = crossings as f64 / (vals.len() - 1) as f64;
            // For uniform random needles, crossing_rate ≈ 2/π ≈ 0.6366
            let buffon_estimate = if crossing_rate > 1e-12 {
                2.0 / crossing_rate
            } else {
                0.0
            };
            result.insert("buffon_estimate".to_string(), vec![buffon_estimate]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_pi_lens_basic() {
        // Points on a circle: high circularity expected
        let n = 12;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            data.push(angle.cos());
            data.push(angle.sin());
        }
        let shared = SharedData::compute(&data, n, d);
        let result = PiLens.scan(&data, n, d, &shared);
        assert!(!result.is_empty());
        let circ = result.get("circularity").unwrap()[0];
        assert!(circ > 0.9, "Points on circle should have high circularity, got {}", circ);
    }

    #[test]
    fn test_pi_lens_angular_uniformity() {
        // Evenly spaced points on circle should have high angular uniformity
        let n = 24;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            data.push(5.0 * angle.cos());
            data.push(5.0 * angle.sin());
        }
        let shared = SharedData::compute(&data, n, d);
        let result = PiLens.scan(&data, n, d, &shared);
        let au = result.get("angular_uniformity").unwrap()[0];
        assert!(au > 0.8, "Uniform circle should have high angular uniformity, got {}", au);
    }

    #[test]
    fn test_pi_lens_small_n() {
        let data = vec![1.0, 2.0, 3.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = PiLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty(), "Should return empty for n < 6");
    }
}
