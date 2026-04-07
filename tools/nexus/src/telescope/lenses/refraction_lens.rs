use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RefractionLens: Bending at interfaces.
/// Measures refraction index, Snell's law fit, critical angle, dispersion.
pub struct RefractionLens;

impl Lens for RefractionLens {
    fn name(&self) -> &str { "RefractionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();

        // Split signal into two halves (two "media")
        let mid = max_n / 2;
        let media1 = &signal[..mid];
        let media2 = &signal[mid..max_n];

        // "Speed" in each medium: average absolute first derivative
        let speed = |s: &[f64]| -> f64 {
            if s.len() < 2 { return 1.0; }
            let sum: f64 = s.windows(2).map(|w| (w[1] - w[0]).abs()).sum();
            sum / (s.len() - 1) as f64
        };
        let v1 = speed(media1).max(1e-12);
        let v2 = speed(media2).max(1e-12);

        // Refraction index: ratio of speeds
        let refraction_index = v1 / v2;

        // Snell's law score: check if angle change at boundary matches speed ratio
        // Angle ~ atan of derivative
        let deriv_at_boundary_1 = if mid >= 2 { (signal[mid - 1] - signal[mid - 2]).abs() } else { 0.0 };
        let deriv_at_boundary_2 = if mid + 1 < max_n { (signal[mid + 1] - signal[mid]).abs() } else { 0.0 };
        let theta1 = deriv_at_boundary_1.atan();
        let theta2 = deriv_at_boundary_2.atan();
        let sin_ratio = if theta2.sin().abs() > 1e-12 {
            theta1.sin() / theta2.sin()
        } else {
            refraction_index
        };
        let snell_score = 1.0 / (1.0 + (sin_ratio - refraction_index).abs());

        // Critical angle: angle of total internal reflection
        let critical_angle = if refraction_index > 1.0 {
            (1.0 / refraction_index).asin()
        } else if refraction_index > 1e-12 {
            (refraction_index).asin()
        } else {
            std::f64::consts::FRAC_PI_2
        };

        // Dispersion: wavelength-dependent bending
        // Use FFT-free approach: measure speed ratio for high-freq vs low-freq components
        let smooth_speed = |s: &[f64]| -> f64 {
            if s.len() < 4 { return 1.0; }
            // Moving average of size 3
            let smoothed: Vec<f64> = s.windows(3).map(|w| (w[0] + w[1] + w[2]) / 3.0).collect();
            speed(&smoothed)
        };
        let sv1 = smooth_speed(media1).max(1e-12);
        let sv2 = smooth_speed(media2).max(1e-12);
        let smooth_ratio = sv1 / sv2;
        let dispersion = (refraction_index - smooth_ratio).abs();

        let mut result = HashMap::new();
        result.insert("refraction_index".to_string(), vec![refraction_index]);
        result.insert("snell_score".to_string(), vec![snell_score]);
        result.insert("critical_angle".to_string(), vec![critical_angle]);
        result.insert("dispersion".to_string(), vec![dispersion]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Two different frequency regions
        let mut data = Vec::new();
        for i in 0..15 { data.push((i as f64 * 0.2).sin()); }
        for i in 0..15 { data.push((i as f64 * 0.8).sin()); }
        let shared = SharedData::compute(&data, 30, 1);
        let result = RefractionLens.scan(&data, 30, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["refraction_index"][0] > 0.0);
    }
}
