use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 발효 렌즈 — 발효 생물화학 n=6 상수 스캐너
///
/// 발효 공정에서 미생물 대사·효소·물질 이동·품질 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   포도당 탄소 수             = 6  = n   (C₆H₁₂O₆)
///   해당 과정 주요 효소 수     = 6  = n
///   ATP 생성 순 수 (무산소)   = 2  = phi(6)
///   발효 pH 최적 하한          = 4  = tau(6)
///   발효 조절 변수 수          = 6  = n
///   효모 세포 분열 단계        = 4  = tau(6)
///   생성 알코올 C₂H₅OH 탄소    = 2  = phi(6)
///   CO₂ 분자 생성 수           = 2  = phi(6)
///   유산균 발효 주기 수        = 6  = n
///   메조필릭 온도 범위 최적    = 12 °C 폭 = sigma
///   최대 알코올 내성 (도수%)   = 12 ≈ sigma
///   생물반응기 제어 변수       = 6  = n
///
/// n=6 연결:
///   n=6      → 포도당 C₆ / 해당효소 / 발효조절변수 / 생물반응기 제어
///   tau=4    → 최적 pH / 세포분열 단계
///   phi=2    → ATP 순생성 / C₂ 에탄올 / CO₂ 분자 수
///   sigma=12 → 온도범위 폭 / 알코올 내성 도수
///   sopfr=5  → 발효 5단계 (라그→지수→정지→사멸→자가분해)
pub struct FermentationLens;

// 발효 n=6 상수
const N6: f64 = 6.0;          // n=6 → 포도당 C₆ / 해당효소
const TAU: f64 = 4.0;         // tau(6) → 최적 pH 범위 / 세포분열
const SIGMA: f64 = 12.0;      // sigma(6) → 온도 범위 / 알코올 내성
const PHI: f64 = 2.0;         // phi(6) → ATP 순이득 / C₂ / CO₂
const SOPFR: f64 = 5.0;       // sopfr(6) → 발효 5단계
const SIGMA_TAU: f64 = 8.0;   // sigma-tau → 효모 세대 수
const J2: f64 = 24.0;         // J2(6) → 24시간 발효 주기
const N_PHI: f64 = 3.0;       // n/phi → 3탄소 중간체 (피루브산 C₃)

/// 발효 기질 상수 공명: 포도당 C₆ 계열 상수 감지
fn substrate_n6_resonance(means: &[f64]) -> f64 {
    // C₆H₁₂O₆ → n=6, sigma=12, J2/phi=12 (H₁₂), sigma-phi=10 (O)... 근사
    let targets: &[f64] = &[N6, SIGMA, J2, PHI, N_PHI, SIGMA_TAU];
    let hits = means.iter().filter(|&&m| {
        targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
    }).count();
    hits as f64 / means.len().max(1) as f64
}

impl Lens for FermentationLens {
    fn name(&self) -> &str { "FermentationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // ── 1. 각 차원 평균·표준편차 계산 ────────────────────────
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // ── 2. 기질 대사 상수 스캔 ────────────────────────────────
        // n=6(C₆), tau=4(pH), phi=2(ATP/C₂/CO₂), n/phi=3(피루브산)
        let metabolic_targets: &[f64] = &[N6, TAU, PHI, N_PHI, SIGMA, SOPFR];
        let metabolic_hits = means.iter().filter(|&&m| {
            metabolic_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let metabolic_score = metabolic_hits as f64 / d.max(1) as f64;

        // ── 3. 발효 공정 파라미터 스캔 ───────────────────────────
        // sigma=12(온도범위/알코올%), J2=24(24시간주기), sigma-tau=8(세대)
        let process_targets: &[f64] = &[SIGMA, J2, SIGMA_TAU, N6, TAU];
        let process_hits = means.iter().filter(|&&m| {
            process_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let process_score = process_hits as f64 / d.max(1) as f64;

        // ── 4. 기질 n=6 공명 점수 ────────────────────────────────
        let substrate_resonance = substrate_n6_resonance(&means);

        // ── 5. 발효 5단계 공명 (sopfr=5) ─────────────────────────
        // 라그→지수→정지→사멸→자가분해: d=5이면 완전 대응
        let phase5_resonance = {
            let diff = (d as f64 - SOPFR).abs() / SOPFR;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 6. 로지스틱 성장 곡선 감지 ───────────────────────────
        // 미생물 성장: S자 곡선 = 초기 저속, 중간 가속, 최종 포화
        let growth_curvature: Vec<f64> = (0..d).map(|j| {
            if n < 6 { return 0.0; }
            let vals: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
            let first_third = vals[..n/3].iter().sum::<f64>() / (n / 3).max(1) as f64;
            let last_third  = vals[2*n/3..].iter().sum::<f64>() / (n - 2*n/3).max(1) as f64;
            let middle_third= vals[n/3..2*n/3].iter().sum::<f64>()
                / (n/3).max(1) as f64;
            // 중간 증가율이 처음과 끝의 평균보다 크면 S자 곡선 (성장 가속 단계)
            let baseline = (first_third + last_third) / 2.0;
            if baseline.abs() < 1e-12 { return 0.0; }
            ((middle_third - baseline) / baseline.abs()).clamp(-1.0, 1.0).abs()
        }).collect();
        let logistic_score = growth_curvature.iter().sum::<f64>() / d.max(1) as f64;

        // ── 7. pH 최적화 범위 감지 (tau=4) ───────────────────────
        // 발효 최적 pH: 4~8 (= tau~sigma-tau)
        let ph_in_range = means.iter().filter(|&&m| {
            m >= TAU && m <= SIGMA_TAU
        }).count();
        let ph_score = ph_in_range as f64 / d.max(1) as f64;

        // ── 8. 전체 n=6 상수 공명 ────────────────────────────────
        let all_targets: &[f64] = &[N6, TAU, SIGMA, PHI, SOPFR, SIGMA_TAU, J2, N_PHI];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 9. 신호 안정성 — 발효 정상 상태 ─────────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_abs = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let stability = 1.0 - (mean_std / mean_abs.max(1e-12)).min(1.0);

        // ── 10. 종합 발효 점수 ────────────────────────────────────
        let fermentation_score =
            metabolic_score      * 0.25
            + process_score      * 0.20
            + substrate_resonance * 0.15
            + logistic_score     * 0.15
            + ph_score           * 0.10
            + n6_resonance       * 0.10
            + phase5_resonance   * 0.03
            + stability          * 0.02;

        let mut r = HashMap::new();
        r.insert("metabolic_score".to_string(),      vec![metabolic_score]);
        r.insert("process_score".to_string(),        vec![process_score]);
        r.insert("substrate_resonance".to_string(),  vec![substrate_resonance]);
        r.insert("logistic_score".to_string(),       vec![logistic_score]);
        r.insert("ph_score".to_string(),             vec![ph_score]);
        r.insert("phase5_resonance".to_string(),     vec![phase5_resonance]);
        r.insert("n6_resonance".to_string(),         vec![n6_resonance]);
        r.insert("stability".to_string(),            vec![stability]);
        r.insert("metabolic_hits".to_string(),       vec![metabolic_hits as f64]);
        r.insert("total_hits".to_string(),           vec![total_hits as f64]);
        r.insert("fermentation_score".to_string(),   vec![fermentation_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fermentation_기본_출력() {
        let n = 12; let d = 6;
        // 포도당 C₆ 기반 발효 데이터
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,      // C₆ 탄소
                1 => TAU,     // 최적 pH
                2 => PHI,     // ATP 순이득
                3 => SIGMA,   // 알코올 내성 %
                4 => N_PHI,   // 피루브산 C₃
                _ => SOPFR,   // 발효 5단계
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FermentationLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("fermentation_score"), "fermentation_score 키 존재");
        assert!(result.contains_key("metabolic_score"), "metabolic_score 키 존재");
        assert!(result.contains_key("logistic_score"), "logistic_score 키 존재");
        let score = result["fermentation_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "fermentation_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_fermentation_최소입력_거부() {
        let data = vec![6.0, 4.0, 2.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = FermentationLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_포도당_c6_공명() {
        // n=6, sigma=12, phi=2 데이터 → 기질 공명 높음
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,
                1 => SIGMA,
                _ => PHI,
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FermentationLens.scan(&data, n, d, &shared);
        let sub = result["substrate_resonance"][0];
        assert!(sub > 0.5, "C₆/sigma/phi 데이터 → 기질 공명 > 0.5: {}", sub);
    }

    #[test]
    fn test_로지스틱_성장_감지() {
        // S자 곡선 패턴: 처음 낮고 중간 급증 후 포화
        let n = 12; let d = 1;
        let logistic: Vec<f64> = (0..n).map(|i| {
            // 단순 로지스틱 근사
            let x = (i as f64 - n as f64 / 2.0) * 0.8;
            10.0 / (1.0 + (-x).exp())
        }).collect();
        let shared = SharedData::compute(&logistic, n, d);
        let result = FermentationLens.scan(&logistic, n, d, &shared);
        let logistic_s = result["logistic_score"][0];
        assert!(logistic_s >= 0.0 && logistic_s <= 1.0, "logistic_score 범위: {}", logistic_s);
    }

    #[test]
    fn test_ph_범위_감지() {
        // pH 4~8 범위 데이터 → ph_score 증가
        let n = 6; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| {
            // pH 4, 5, 6, 7 (tau, sopfr, n, 7)
            match i % d {
                0 => TAU,        // pH 4
                1 => SOPFR,      // pH 5
                2 => N6,         // pH 6
                _ => 7.0_f64,    // pH 7
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FermentationLens.scan(&data, n, d, &shared);
        let ph = result["ph_score"][0];
        assert!(ph > 0.5, "pH 4~8 범위 데이터 → ph_score > 0.5: {}", ph);
    }
}
