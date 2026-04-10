use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 오토바이 동역학 렌즈 — 2·3·6기통 밸런스, tau=4 행정, 기어 n=6 수렴
/// n=6 연결: 병렬4기통=tau, 직렬6기통=n, 6단변속, 발화120도=sigma/tau*40
pub struct MotorcycleDynamicsLens;

impl Lens for MotorcycleDynamicsLens {
    fn name(&self) -> &'static str { "MotorcycleDynamicsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let moto_consts: &[(f64, &str)] = &[
            (4.0, "four_stroke_tau"), (6.0, "six_gear"), (2.0, "twin_phi"),
            (3.0, "triple_n_phi"), (120.0, "firing_deg"), (12.0, "sigma_volt"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in moto_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 4행정 사이클 패턴 (tau=4)
        let four_stroke_score = {
            let signal: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let period_4_corr = if n >= 8 {
                let mean = signal.iter().sum::<f64>() / n as f64;
                let lag4: f64 = (0..n-4).map(|i| (signal[i]-mean)*(signal[i+4]-mean)).sum::<f64>();
                let var: f64 = signal.iter().map(|x| (x-mean).powi(2)).sum::<f64>();
                if var > 1e-12 { (lag4 / var).abs() } else { 0.0 }
            } else { 0.0 };
            period_4_corr.min(1.0)
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5
            + four_stroke_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_moto_matches".to_string(), vec![n6_matches as f64]);
        r.insert("four_stroke_score".to_string(), vec![four_stroke_score]);
        r.insert("motorcycle_dynamics_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_motorcycle_basic() {
        let n = 8; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| {
            let vals = [4.0f64, 6.0, 120.0];
            vals[i % d]
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MotorcycleDynamicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("motorcycle_dynamics_score"));
        assert!(result["n6_moto_matches"][0] >= 2.0);
    }
}
