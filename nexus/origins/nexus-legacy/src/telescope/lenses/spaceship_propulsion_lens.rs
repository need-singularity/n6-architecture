use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 우주선 추진 렌즈 — 로켓 단수 phi~n/phi, SE(3) 자세제어 n=6, BT-275/BT-273
/// n=6 연결: 3단로켓=n/phi, 승무원 1→2→3=div(6), ISS 궤도 90분=sigma*tau+18
pub struct SpaceshipPropulsionLens;

impl Lens for SpaceshipPropulsionLens {
    fn name(&self) -> &'static str { "SpaceshipPropulsionLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let space_consts: &[(f64, &str)] = &[
            (3.0, "rocket_stages_n_phi"), (6.0, "se3_attitude"), (2.0, "phi_twin"),
            (24.0, "J2_gnss"), (12.0, "sigma_nav"), (90.0, "iss_period_min"),
            (1.0, "mu_single"), (9.8, "g0_m_s2"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in space_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // Tsiolkovsky 비추력 검출: 지수 감쇠 패턴
        let exponential_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            if col.len() >= 4 {
                let ratios: Vec<f64> = col.windows(2)
                    .filter(|w| w[0].abs() > 1e-12)
                    .map(|w| (w[1] / w[0]).abs())
                    .collect();
                if !ratios.is_empty() {
                    let mean_r = ratios.iter().sum::<f64>() / ratios.len() as f64;
                    let var_r = ratios.iter().map(|r| (r - mean_r).powi(2)).sum::<f64>() / ratios.len() as f64;
                    let cv = if mean_r > 1e-12 { var_r.sqrt() / mean_r } else { 1.0 };
                    if mean_r < 1.0 { (-cv * 3.0).exp() } else { 0.0 }
                } else { 0.0 }
            } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5
            + exponential_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_space_matches".to_string(), vec![n6_matches as f64]);
        r.insert("tsiolkovsky_exp_score".to_string(), vec![exponential_score]);
        r.insert("spaceship_propulsion_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_spaceship_basic() {
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| {
            let vals = [3.0f64, 6.0, 24.0];
            vals[i % d]
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = SpaceshipPropulsionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("spaceship_propulsion_score"));
    }
}
