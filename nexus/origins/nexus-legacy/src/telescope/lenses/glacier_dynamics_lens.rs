use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 빙하 동역학 렌즈 — 빙하 유동/균형 n=6 수렴 스캐너
///
/// 빙하 동역학(glacier dynamics)의 n=6 연결:
///   빙하 유동 메커니즘 수 = 3~6 → [tau-1, n]
///   빙점(0°C) = 273.15 K → tau×sigma+...
///   Glen's 유동 법칙 지수 n = 3 = tau-1
///   빙하 균형선 고도 적설/융해 비율 = 1 → phi/phi
///   빙핵 산소 동위원소 δ¹⁸O 해석 주기 = 12만년 = sigma 만
///   주요 빙하기 주기: 4만년 = tau 만년
pub struct GlacierDynamicsLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const GLEN_N: f64 = 3.0;          // Glen 법칙 지수 = tau-1
const ICE_CYCLE_40: f64 = 40.0;   // 4만년 주기 단위 = tau 만
const ICE_CYCLE_100: f64 = 100.0; // 10만년 주기 단위

impl Lens for GlacierDynamicsLens {
    fn name(&self) -> &str { "GlacierDynamicsLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // 1. Glen 법칙 지수 n=3 공명 (tau-1=3)
        let glen_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - GLEN_N) / GLEN_N).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 2. 빙하기 주기 공명 (tau·10=40, sigma·10=120 근사)
        let cycle_score = means.iter().filter(|&&m| {
            [ICE_CYCLE_40, ICE_CYCLE_100, TAU, SIGMA].iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count() as f64 / d.max(1) as f64;

        // 3. 유동 속도 단조성 (빙하는 서서히 흐름)
        let slow_flow: f64 = {
            let mut total = 0.0;
            for j in 0..d {
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let m = means[j];
                if m.abs() < 1e-12 { continue; }
                let cv = stds[j] / m.abs();
                total += (1.0 - cv.min(1.0));
            }
            total / d as f64
        };

        // 4. 빙하 균형 공명 (적설=융해 → 비율≈1, phi)
        let balance_score = means.iter().filter(|&&m| {
            [1.0, PHI, PHI / PHI].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count() as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, GLEN_N];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let glacier_score = glen_score    * 0.25
            + n6_dim        * 0.20
            + slow_flow     * 0.20
            + cycle_score   * 0.15
            + balance_score * 0.10
            + n6_resonance  * 0.10;

        let mut r = HashMap::new();
        r.insert("glen_score".to_string(),     vec![glen_score]);
        r.insert("cycle_score".to_string(),    vec![cycle_score]);
        r.insert("slow_flow".to_string(),      vec![slow_flow]);
        r.insert("balance_score".to_string(),  vec![balance_score]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("n6_resonance".to_string(),   vec![n6_resonance]);
        r.insert("glacier_score".to_string(),  vec![glacier_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glacier_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => GLEN_N, 2 => TAU, 3 => PHI, 4 => SIGMA, _ => SOPFR }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = GlacierDynamicsLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("glacier_score"));
        assert!(r["glacier_score"][0] >= 0.0 && r["glacier_score"][0] <= 1.0);
    }

    #[test]
    fn test_glacier_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = GlacierDynamicsLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
