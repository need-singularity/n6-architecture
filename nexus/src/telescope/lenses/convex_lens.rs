use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConvexLens: Convergent/focusing patterns.
/// Measures convergence rate, focal length, depth of field, magnification.
pub struct ConvexLens;

impl Lens for ConvexLens {
    fn name(&self) -> &str { "ConvexLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Compute centroid
        let mut centroid = vec![0.0f64; d];
        for i in 0..max_n {
            for j in 0..d { centroid[j] += data[i * d + j]; }
        }
        for j in 0..d { centroid[j] /= max_n as f64; }

        // Radii from centroid
        let radii: Vec<f64> = (0..max_n).map(|i| {
            let mut r2 = 0.0;
            for j in 0..d { let diff = data[i * d + j] - centroid[j]; r2 += diff * diff; }
            r2.sqrt()
        }).collect();

        // Convergence rate: how fast points approach a focus
        let third = max_n / 3;
        let first_avg = if third > 0 { radii[..third].iter().sum::<f64>() / third as f64 } else { 0.0 };
        let last_avg = if third > 0 {
            radii[(max_n - third)..].iter().sum::<f64>() / third as f64
        } else { 0.0 };
        let convergence_rate = if first_avg > 1e-12 {
            (first_avg - last_avg) / first_avg
        } else {
            0.0
        };

        // Focal length: find index where density is highest (tightest cluster)
        let densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
        let (focal_idx, _) = densities.iter().enumerate()
            .fold((0, 0.0f64), |(bi, bv), (i, &v)| if v > bv { (i, v) } else { (bi, bv) });
        let focal_length = focal_idx as f64;

        // Depth of field: range of indices where density stays > 50% of peak
        let peak_density = densities[focal_idx];
        let dof_threshold = peak_density * 0.5;
        let in_focus: Vec<usize> = (0..max_n).filter(|&i| densities[i] > dof_threshold).collect();
        let depth_of_field = if in_focus.len() >= 2 {
            (in_focus.last().unwrap() - in_focus[0]) as f64
        } else {
            1.0
        };

        // Magnification: positive for convex (convergent)
        let magnification = convergence_rate.abs();

        let mut result = HashMap::new();
        result.insert("convergence_rate".to_string(), vec![convergence_rate]);
        result.insert("focal_length".to_string(), vec![focal_length]);
        result.insert("depth_of_field".to_string(), vec![depth_of_field]);
        result.insert("magnification".to_string(), vec![magnification]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Converging data: values approach center
        let data: Vec<f64> = (0..20).map(|i| {
            let x = (i as f64 - 10.0) / 10.0;
            x * x // parabola, converges at center
        }).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = ConvexLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("convergence_rate"));
        assert!(result["depth_of_field"][0] >= 1.0);
    }
}
