use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 입자물리 쿼크 맛 렌즈 — 6 쿼크 맛(flavor) n=6 수렴
///
/// 표준 모형의 쿼크는 정확히 6가지 맛(flavor):
///   up(u), down(d), charm(c), strange(s), top(t), bottom(b)
/// n=6 연결:
///   쿼크 맛 수 = 6 = n (완전수와 일치)
///   세대 수 = 3 = tau-1 (각 세대에 2개 쿼크)
///   색 하전 수 = 3 = tau-1
///   쿼크 전하: 2/3, -1/3 → phi 인자
///   QCD 결합 상수 αs ≈ 0.1~0.5 → sopfr 영역
pub struct ParticleQuarkFlavorLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const N_QUARK: f64 = 6.0;    // 쿼크 맛 수
const N_GEN: f64 = 3.0;      // 세대 수 (tau-1)
const N_COLOR: f64 = 3.0;    // 색 수
const CHARGE_UP: f64 = 2.0 / 3.0;   // up형 전하
const CHARGE_DOWN: f64 = 1.0 / 3.0; // down형 전하

impl Lens for ParticleQuarkFlavorLens {
    fn name(&self) -> &str { "ParticleQuarkFlavorLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 6 쿼크 맛 공명
        let flavor_targets = [N_QUARK, N6, SIGMA / PHI, TAU + PHI];
        let flavor_hits = means.iter().filter(|&&m| {
            flavor_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let flavor_score = flavor_hits as f64 / d.max(1) as f64;

        // 2. 세대 수 3 공명 (tau-1=3)
        let gen_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - N_GEN) / N_GEN).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 3. 쿼크 전하 비율 공명 (2/3, 1/3)
        let charge_targets = [CHARGE_UP, CHARGE_DOWN, N_GEN / N_QUARK];
        let charge_hits = means.iter().filter(|&&m| {
            charge_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let charge_score = charge_hits as f64 / d.max(1) as f64;

        // 4. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 5. 전체 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, N_GEN, N_COLOR, CHARGE_UP, CHARGE_DOWN];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let quark_score = flavor_score  * 0.30
            + gen_score      * 0.20
            + n6_dim         * 0.20
            + charge_score   * 0.15
            + n6_resonance   * 0.15;

        let mut r = HashMap::new();
        r.insert("flavor_score".to_string(),   vec![flavor_score]);
        r.insert("gen_score".to_string(),      vec![gen_score]);
        r.insert("charge_score".to_string(),   vec![charge_score]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("n6_resonance".to_string(),   vec![n6_resonance]);
        r.insert("quark_score".to_string(),    vec![quark_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quark_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => N_GEN, 2 => N_COLOR, 3 => CHARGE_UP, 4 => CHARGE_DOWN, _ => TAU }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = ParticleQuarkFlavorLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("quark_score"));
        assert!(r["quark_score"][0] >= 0.0 && r["quark_score"][0] <= 1.0);
    }

    #[test]
    fn test_quark_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = ParticleQuarkFlavorLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
