use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 광업 지질학 렌즈 — 결정 배위수 CN=6, 광맥 패턴, 암석 분류 n=6 (BT-139)
/// n=6 연결: 광물 CN=n=6(팔면체), 암석 경도 Mohs 1-10=sigma-phi+tau, 광산 갱도 tau=4
pub struct MiningGeologyLens;

impl Lens for MiningGeologyLens {
    fn name(&self) -> &'static str { "MiningGeologyLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let geo_consts: &[(f64, &str)] = &[
            (6.0, "cn_octahedral"), (10.0, "mohs_max_sigma_phi"), (4.0, "tau_adit"),
            (3.0, "n_phi_ore_class"), (12.0, "sigma_mineral"), (7.0, "mohs_quartz"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in geo_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 층위 구조 탐지 (지층 = 계단 패턴)
        let strata_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let mut steps = 0usize;
            for w in col.windows(3) {
                let d1 = (w[1] - w[0]).abs();
                let d2 = (w[2] - w[1]).abs();
                if d1 < 0.1 * (w[0].abs() + 1.0) && d2 > 0.2 * (w[0].abs() + 1.0) { steps += 1; }
            }
            (steps as f64 / (n.saturating_sub(2)).max(1) as f64).min(1.0)
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5 + strata_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_geo_matches".to_string(), vec![n6_matches as f64]);
        r.insert("strata_score".to_string(), vec![strata_score]);
        r.insert("mining_geology_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mining_geology_basic() {
        let n = 6; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 6.0 } else { 10.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MiningGeologyLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mining_geology_score"));
    }
}
