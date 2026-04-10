use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 항공우주 모빌리티 렌즈 — 모빌리티×컴퓨팅 강결합 도메인 (BT-196, BT-241, BT-276)
/// n=6 연결: SE(3)=n=6 자유도, 3중 여분=n/phi, 조종면 tau=4, 항법 sigma=12, GNSS J2=24
pub struct AerospaceMobilityLens;

impl Lens for AerospaceMobilityLens {
    fn name(&self) -> &'static str { "AerospaceMobilityLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }
        let n6: f64 = 6.0;
        let aero_constants: &[(f64, &str)] = &[
            (6.0, "SE3_dof"), (12.0, "nav_sigma"), (24.0, "gnss_J2"),
            (4.0, "ctrl_tau"), (3.0, "triple_redund"), (48.0, "sigma_tau"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let se3_score = {
            if d >= 6 {
                let var_per: Vec<f64> = (0..6).map(|j| {
                    let m = means[j];
                    (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64
                }).collect();
                let mv = var_per.iter().sum::<f64>() / 6.0;
                let vv = var_per.iter().map(|v| (v - mv).powi(2)).sum::<f64>() / 6.0;
                let cv = if mv > 1e-12 { vv.sqrt() / mv } else { 1.0 };
                (-cv * n6).exp()
            } else { (d as f64 / n6).min(1.0) }
        };

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in aero_constants {
                if c > 1e-12 && ((m - c) / c).abs() < 0.06 { n6_matches += 1; break; }
            }
        }

        let gps_score = (-((d as f64 - n6) / n6).abs().min(2.0)).exp();
        let score = se3_score * 0.4 + (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.35 + gps_score * 0.25;

        let mut r = HashMap::new();
        r.insert("se3_dof_score".to_string(), vec![se3_score]);
        r.insert("n6_aero_matches".to_string(), vec![n6_matches as f64]);
        r.insert("gps_orbit_score".to_string(), vec![gps_score]);
        r.insert("aerospace_mobility_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_aerospace_mobility_basic() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 * 0.3).sin()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = AerospaceMobilityLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("aerospace_mobility_score"));
        assert!(result.contains_key("se3_dof_score"));
    }
}
