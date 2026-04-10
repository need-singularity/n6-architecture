use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 도자기과학 렌즈 — 세라믹 결정 CN=6, 소결 단계, 유약 화학 n=6 패턴
/// n=6 연결: Al2O3 CN=6(팔면체), 소결 단계=n=6, 유약 성분 SiO2=Si Z+O Z=14+8=22≈J2-phi
pub struct CeramicsLens;

impl Lens for CeramicsLens {
    fn name(&self) -> &'static str { "CeramicsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let ceramic_consts: &[(f64, &str)] = &[
            (6.0, "cn_alumina"), (4.0, "cn_silica_tau"), (12.0, "sigma_phases"),
            (3.0, "n_phi_glaze_layer"), (1400.0, "sintering_temp_c"), (6.0, "firing_stages"),
            (24.0, "J2_crystal_planes"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in ceramic_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 소결 온도 임계점 탐지 (급격한 밀도 증가 패턴)
        let sintering_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let max_jump = col.windows(2).map(|w| (w[1] - w[0]).abs())
                .fold(0.0f64, f64::max);
            let range = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                - col.iter().cloned().fold(f64::INFINITY, f64::min);
            if range > 1e-12 { (max_jump / range).min(1.0) } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5 + sintering_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_ceramic_matches".to_string(), vec![n6_matches as f64]);
        r.insert("sintering_score".to_string(), vec![sintering_score]);
        r.insert("ceramics_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ceramics_basic() {
        let n = 6; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 6.0 } else { 4.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = CeramicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("ceramics_score"));
    }
}
