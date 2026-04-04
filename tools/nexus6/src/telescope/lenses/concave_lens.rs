use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConcaveLens: Divergent/spreading patterns.
/// Measures divergence rate, virtual focus, spread angle, magnification.
pub struct ConcaveLens;

impl Lens for ConcaveLens {
    fn name(&self) -> &str { "ConcaveLens" }
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

        // Divergence rate: compare average distance in first third vs last third
        let third = max_n / 3;
        let first_avg = if third > 0 { radii[..third].iter().sum::<f64>() / third as f64 } else { 0.0 };
        let last_avg = if third > 0 {
            radii[(max_n - third)..].iter().sum::<f64>() / third as f64
        } else { 0.0 };
        let divergence_rate = if first_avg > 1e-12 {
            (last_avg - first_avg) / first_avg
        } else {
            last_avg
        };

        // Virtual focus: extrapolate inward to find where lines converge
        // Use linear fit of radius vs index, extrapolate to r=0
        let n_f = max_n as f64;
        let x_mean = (n_f - 1.0) / 2.0;
        let y_mean = radii.iter().sum::<f64>() / n_f;
        let mut ss_xy = 0.0f64;
        let mut ss_xx = 0.0f64;
        for i in 0..max_n {
            let dx = i as f64 - x_mean;
            let dy = radii[i] - y_mean;
            ss_xy += dx * dy;
            ss_xx += dx * dx;
        }
        let slope = if ss_xx > 1e-12 { ss_xy / ss_xx } else { 0.0 };
        let intercept = y_mean - slope * x_mean;
        let virtual_focus = if slope.abs() > 1e-12 { -intercept / slope } else { f64::MAX };
        let virtual_focus = virtual_focus.clamp(-1000.0, 1000.0);

        // Spread angle: angular spread of points
        let max_r = radii.iter().cloned().fold(0.0f64, f64::max);
        let min_r = radii.iter().cloned().fold(f64::MAX, f64::min);
        let spread_angle = if max_r > 1e-12 {
            ((max_r - min_r) / max_r * std::f64::consts::FRAC_PI_2).min(std::f64::consts::PI)
        } else {
            0.0
        };

        // Magnification: negative for concave (divergent)
        let magnification = if divergence_rate > 0.0 { -divergence_rate.abs() } else { divergence_rate.abs() };

        let mut result = HashMap::new();
        result.insert("divergence_rate".to_string(), vec![divergence_rate]);
        result.insert("virtual_focus".to_string(), vec![virtual_focus]);
        result.insert("spread_angle".to_string(), vec![spread_angle]);
        result.insert("magnification".to_string(), vec![magnification]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Expanding data
        let data: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = ConcaveLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("divergence_rate"));
    }
}
