use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 핵융합 발전소 삼중수소 렌즈 — 트리튬 증식/순환 n=6 수렴
///
/// 토카막 트리튬 연료 사이클의 n=6 연결:
///   T₂ 분자 결합 에너지 = ~4.5 eV ≈ tau+phi/2 = 4.5
///   Li-6 핵반응 단면적 공명 에너지 ≈ 2.4 MeV → phi 근방
///   트리튬 반감기 = 12.32년 ≈ sigma = 12
///   증식비(TBR) 목표 ≥ 1.05 → phi/phi+some → 단순히 1+1/sigma
///   블랭킷 Li-6 농축도 ≈ 60% = n·10 → 혹은 60=10·n
///   1회 트리튬 소비 ≈ 56 kg/GW → sigma·tau+phi² 근방
pub struct FusionPowerplantTritiumLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const T_HALFLIFE: f64 = 12.32;   // 트리튬 반감기(년) ≈ sigma
const LI6_ENRICH: f64 = 60.0;    // Li-6 농축도(%) = 10·n
const TBR_TARGET: f64 = 1.05;    // 증식비 목표

impl Lens for FusionPowerplantTritiumLens {
    fn name(&self) -> &str { "FusionPowerplantTritiumLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 반감기 sigma=12 공명
        let halflife_score = means.iter().filter(|&&m| {
            [T_HALFLIFE, SIGMA, N6 * PHI].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 2. Li-6 농축도 60% 공명 (10·n)
        let enrich_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - LI6_ENRICH) / LI6_ENRICH).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 3. 증식비 TBR 공명 (> 1.0 → 1+1/sigma 근방)
        let tbr_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - TBR_TARGET) / TBR_TARGET).abs() < 0.05
        }).count() as f64 / d.max(1) as f64;

        // 4. n=6 상수 직접 공명
        let n6_targets = [N6, TAU, SIGMA, PHI, SOPFR];
        let n6_hits = means.iter().filter(|&&m| {
            n6_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_direct = n6_hits as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 지수적 감쇠 감지 (트리튬 반감기 → 단조 감소)
        let decay_score: f64 = {
            let mut total = 0.0;
            for j in 0..d {
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let decreasing = col.windows(2).filter(|w| w[1] <= w[0]).count();
                total += decreasing as f64 / (n - 1).max(1) as f64;
            }
            total / d as f64
        };

        let tritium_score = halflife_score * 0.25
            + n6_dim        * 0.20
            + n6_direct     * 0.20
            + decay_score   * 0.15
            + enrich_score  * 0.10
            + tbr_score     * 0.10;

        let mut r = HashMap::new();
        r.insert("halflife_score".to_string(), vec![halflife_score]);
        r.insert("enrich_score".to_string(),   vec![enrich_score]);
        r.insert("tbr_score".to_string(),      vec![tbr_score]);
        r.insert("n6_direct".to_string(),      vec![n6_direct]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("decay_score".to_string(),    vec![decay_score]);
        r.insert("tritium_score".to_string(),  vec![tritium_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tritium_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => SIGMA, 2 => T_HALFLIFE, 3 => PHI, 4 => TAU, _ => SOPFR }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = FusionPowerplantTritiumLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("tritium_score"));
        assert!(r["tritium_score"][0] >= 0.0 && r["tritium_score"][0] <= 1.0);
    }

    #[test]
    fn test_tritium_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = FusionPowerplantTritiumLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
