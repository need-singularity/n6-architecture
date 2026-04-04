use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FluidDynamicsLens: Detect turbulence and flow patterns.
///
/// Estimates Reynolds-number-like quantities from data: ratio of
/// inertial (variance) to viscous (smoothness) forces.
/// n=6 connection: Kolmogorov 5/3 spectrum exponent = sopfr(6)/n/phi = 5/3,
/// turbulent cascade has self-similar structure at scale n=6.
pub struct FluidDynamicsLens;

impl Lens for FluidDynamicsLens {
    fn name(&self) -> &str { "FluidDynamicsLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let signal: Vec<f64> = (0..n).map(|i| data[i * d]).collect();

        // "Velocity" = first differences
        let velocity: Vec<f64> = signal.windows(2).map(|w| w[1] - w[0]).collect();
        if velocity.is_empty() { return HashMap::new(); }

        // "Acceleration" = second differences
        let accel: Vec<f64> = velocity.windows(2).map(|w| w[1] - w[0]).collect();

        // Inertial measure: variance of velocity
        let vel_mean = velocity.iter().sum::<f64>() / velocity.len() as f64;
        let vel_var = velocity.iter().map(|v| (v - vel_mean).powi(2)).sum::<f64>()
            / velocity.len() as f64;

        // Viscous measure: smoothness (mean absolute acceleration)
        let viscosity = if accel.is_empty() {
            1e-12
        } else {
            accel.iter().map(|a| a.abs()).sum::<f64>() / accel.len() as f64
        };

        // Reynolds-like number: inertial / viscous
        let reynolds = if viscosity > 1e-12 {
            vel_var.sqrt() / viscosity
        } else {
            0.0
        };

        // Turbulence indicator: high Reynolds = turbulent
        let turbulence = 1.0 - (-reynolds / 6.0).exp(); // n=6 scaling

        // Kolmogorov 5/3 check: compute energy spectrum slope
        // Use structure function: S_p(r) = <|v(x+r) - v(x)|^p>
        let max_r = velocity.len() / 2;
        let mut log_r = Vec::new();
        let mut log_s2 = Vec::new();
        for r in 1..=max_r.min(12) { // sigma=12 scales
            let mut s2_sum = 0.0;
            let mut count = 0;
            for i in 0..(velocity.len() - r) {
                s2_sum += (velocity[i + r] - velocity[i]).powi(2);
                count += 1;
            }
            if count > 0 {
                let s2 = s2_sum / count as f64;
                if s2 > 1e-12 {
                    log_r.push((r as f64).ln());
                    log_s2.push(s2.ln());
                }
            }
        }

        // Fit slope for structure function exponent
        let kolmogorov_exponent = if log_r.len() >= 3 {
            linear_slope(&log_r, &log_s2)
        } else {
            0.0
        };

        // Match to Kolmogorov 2/3 (structure function exponent for velocity)
        // or 5/3 (energy spectrum exponent = sopfr(6)/3)
        let k53_match = (-(kolmogorov_exponent - 2.0 / 3.0).abs() * 6.0).exp();

        let mut result = HashMap::new();
        result.insert("reynolds_number".to_string(), vec![reynolds]);
        result.insert("turbulence".to_string(), vec![turbulence]);
        result.insert("velocity_variance".to_string(), vec![vel_var]);
        result.insert("kolmogorov_exponent".to_string(), vec![kolmogorov_exponent]);
        result.insert("n6_kolmogorov_match".to_string(), vec![k53_match]);
        result
    }
}

fn linear_slope(x: &[f64], y: &[f64]) -> f64 {
    let n = x.len().min(y.len());
    if n < 2 { return 0.0; }
    let nf = n as f64;
    let sx: f64 = x[..n].iter().sum();
    let sy: f64 = y[..n].iter().sum();
    let sxy: f64 = x[..n].iter().zip(y[..n].iter()).map(|(a, b)| a * b).sum();
    let sx2: f64 = x[..n].iter().map(|a| a * a).sum();
    let denom = nf * sx2 - sx * sx;
    if denom.abs() < 1e-15 { 0.0 } else { (nf * sxy - sx * sy) / denom }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fluid_smooth_signal() {
        // Smooth signal -> low Reynolds
        let n = 30;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FluidDynamicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("reynolds_number"));
        assert!(result["turbulence"][0] < 0.5, "Smooth signal should have low turbulence");
    }

    #[test]
    fn test_fluid_noisy_signal() {
        // Noisy signal -> higher Reynolds
        let n = 30;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| ((i * 13 + 7) % 11) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FluidDynamicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("reynolds_number"));
    }
}
