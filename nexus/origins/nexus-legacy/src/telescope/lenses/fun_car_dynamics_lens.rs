use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 자동차 동역학 렌즈 — Inline-6/변속기/전압래더 n=6 수렴 (BT-287, BT-288, BT-289)
/// n=6 연결: 인라인6 완전밸런스, 기어6단, 전압 6→12→24→48V
pub struct FunCarDynamicsLens;

impl Lens for FunCarDynamicsLens {
    fn name(&self) -> &'static str { "FunCarDynamicsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let car_consts: &[(f64, &str)] = &[
            (6.0, "inline6_cylinders"), (12.0, "voltage_12V"), (24.0, "voltage_24V"),
            (48.0, "voltage_48V_mild_hybrid"), (4.0, "torque_phase_tau"),
            (3.0, "triple_lambda"), (120.0, "firing_interval_deg"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in car_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 { n6_matches += 1; break; }
            }
        }

        // 전압 배증 패턴 탐지: 6→12→24→48 (phi=2 배증)
        let sorted_means: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 1.0).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap());
            v
        };
        let doubling_score = if sorted_means.len() >= 2 {
            let ratios: Vec<f64> = sorted_means.windows(2)
                .filter(|w| w[0] > 1e-12)
                .map(|w| w[1] / w[0]).collect();
            let near2 = ratios.iter().filter(|&&r| (r - 2.0).abs() < 0.15).count();
            near2 as f64 / ratios.len().max(1) as f64
        } else { 0.0 };

        // 120도 점화 간격 (Inline-6 완전밸런스)
        let firing_score = means.iter().map(|&m| {
            (-((m - 120.0) / 120.0).powi(2) * 30.0).exp()
        }).fold(0.0f64, f64::max);

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.4
            + doubling_score * 0.35 + firing_score * 0.25;

        let mut r = HashMap::new();
        r.insert("n6_car_matches".to_string(), vec![n6_matches as f64]);
        r.insert("voltage_doubling_score".to_string(), vec![doubling_score]);
        r.insert("firing_interval_score".to_string(), vec![firing_score]);
        r.insert("fun_car_dynamics_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_fun_car_voltage_ladder() {
        let n = 8; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| {
            let vals = [6.0f64, 12.0, 24.0, 48.0];
            vals[i % d]
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FunCarDynamicsLens.scan(&data, n, d, &shared);
        assert!(result["voltage_doubling_score"][0] > 0.5);
    }
}
