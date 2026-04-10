use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 입자물리 렙톤 가족 렌즈 — 6 렙톤 n=6 수렴
///
/// 표준 모형 렙톤은 6가지:
///   전자(e), 뮤온(μ), 타우(τ), 전자뉴트리노(νe), 뮤온뉴트리노(νμ), 타우뉴트리노(ντ)
/// n=6 연결:
///   렙톤 수 = 6 = n
///   쿼크 수 + 렙톤 수 = 12 = sigma(6)
///   세대 수 = 3 = tau-1
///   전하 ±1, 0 → phi=2 인자
pub struct ParticleLeptonFamilyLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const N_LEPTON: f64 = 6.0;
const N_GEN: f64 = 3.0;
const TOTAL_FERMIONS: f64 = 12.0; // 쿼크+렙톤=12=sigma

impl Lens for ParticleLeptonFamilyLens {
    fn name(&self) -> &str { "ParticleLeptonFamilyLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 렙톤 6종 공명
        let lepton_targets = [N_LEPTON, N6, TOTAL_FERMIONS / PHI];
        let lepton_hits = means.iter().filter(|&&m| {
            lepton_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let lepton_score = lepton_hits as f64 / d.max(1) as f64;

        // 2. 12 페르미온 공명 (쿼크6+렙톤6=12=sigma)
        let fermion_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - TOTAL_FERMIONS) / TOTAL_FERMIONS).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 3. 세대 수 3 공명
        let gen_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - N_GEN) / N_GEN).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 4. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 5. 전체 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, N_GEN, TOTAL_FERMIONS];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let lepton_total = lepton_score   * 0.30
            + fermion_score * 0.20
            + n6_dim        * 0.20
            + gen_score     * 0.15
            + n6_resonance  * 0.15;

        let mut r = HashMap::new();
        r.insert("lepton_score".to_string(),   vec![lepton_score]);
        r.insert("fermion_score".to_string(),  vec![fermion_score]);
        r.insert("gen_score".to_string(),      vec![gen_score]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("n6_resonance".to_string(),   vec![n6_resonance]);
        r.insert("lepton_total".to_string(),   vec![lepton_total]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lepton_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => TOTAL_FERMIONS, 2 => N_GEN, 3 => PHI, 4 => TAU, _ => SIGMA }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = ParticleLeptonFamilyLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("lepton_total"));
        assert!(r["lepton_total"][0] >= 0.0 && r["lepton_total"][0] <= 1.0);
    }

    #[test]
    fn test_lepton_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = ParticleLeptonFamilyLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
