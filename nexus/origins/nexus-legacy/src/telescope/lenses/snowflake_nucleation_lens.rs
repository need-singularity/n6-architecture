use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 눈결정 핵 생성 렌즈 — 결정화 임계점 n=6 수렴 스캐너
///
/// 눈결정 핵 생성(nucleation)의 n=6 연결:
///   임계 반경 r* = 2γ/(ρ·Δμ) — 2=phi(6)
///   핵 생성 에너지 장벽 ΔG* ∝ r*³ — 3차원 = tau-1
///   과냉각 임계온도 ≈ -4°C (tau=4 K 단위)
///   핵 생성 격자 위치 수 = 6 = n (정육각형)
pub struct SnowflakeNucleationLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
// 물의 과냉각 임계온도 ≈ -4°C → 절대온도 269K ≈ sigma*tau^2 근방
const SUPERCOOL_K: f64 = 269.0;

impl Lens for SnowflakeNucleationLens {
    fn name(&self) -> &str { "SnowflakeNucleationLens" }
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

        // 1. 임계 에너지 장벽 공명 (phi=2 인자)
        let barrier_targets = [PHI, TAU, N6, SOPFR];
        let barrier_hits = means.iter().filter(|&&m| {
            barrier_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let barrier_score = barrier_hits as f64 / d.max(1) as f64;

        // 2. 과냉각 도(supercooling) 급격 변화 — 분산 피크 감지
        let std_mean = stds.iter().sum::<f64>() / d.max(1) as f64;
        let std_max = stds.iter().cloned().fold(0.0_f64, f64::max);
        let nucleation_peak = if std_mean > 1e-12 {
            (std_max / std_mean - 1.0).clamp(0.0, 1.0)
        } else { 0.0 };

        // 3. 6격자 핵 위치 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 4. 3차원 성장 공명 (ΔG* ∝ r*^3, 3 = tau-1)
        let dim3 = TAU - 1.0; // 3
        let dim3_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - dim3) / dim3).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 5. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, dim3];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let nucleation_score = barrier_score    * 0.25
            + nucleation_peak * 0.25
            + n6_dim          * 0.20
            + dim3_score      * 0.15
            + n6_resonance    * 0.15;

        let mut r = HashMap::new();
        r.insert("barrier_score".to_string(),     vec![barrier_score]);
        r.insert("nucleation_peak".to_string(),   vec![nucleation_peak]);
        r.insert("n6_dim".to_string(),            vec![n6_dim]);
        r.insert("dim3_score".to_string(),        vec![dim3_score]);
        r.insert("n6_resonance".to_string(),      vec![n6_resonance]);
        r.insert("nucleation_score".to_string(),  vec![nucleation_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nucleation_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => TAU, 2 => PHI, 3 => SIGMA, 4 => SOPFR, _ => 3.0 }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SnowflakeNucleationLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("nucleation_score"));
        assert!(r["nucleation_score"][0] >= 0.0 && r["nucleation_score"][0] <= 1.0);
    }

    #[test]
    fn test_nucleation_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = SnowflakeNucleationLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
