use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 여성청결제 렌즈 (BT-1158) — 여성 친밀부위 세정 상수 스캐너
///
/// BT-1158 여성청결제 돌파에서 파생.
/// 마이크로바이옴/pH·생화학/월경·계면활성제·해부학의 핵심 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   질내 pH 최적            = 4     = tau
///   질내 pH 상한            = 5     = sopfr
///   외음부 pH               = 6     = n
///   락토바실러스 주요 종 수  = 4     = tau
///   마이크로바이옴 균총 다양성= 6    = n
///   월경 주기 (일)          = 28    = sigma*phi + tau
///   월경 pH 변동 주기 (일)   = 14    = sigma + phi
///   에스트로겐 대사 경로 수  = 5     = sopfr
///   계면활성제 임계미셀농도  = 12    = sigma
///   계면활성제 안전 분류 수  = 4     = tau
///   해부학 조직층 수        = 2     = phi
///   해부학 구역 수          = 6     = n
///
/// n=6 연결:
///   n=6    → 외음부 pH / 균총 다양성 / 구역 수
///   tau=4  → 질내 pH / 락토바실러스 종 / 계면활성제 분류
///   sigma=12 → 임계미셀농도
///   phi=2  → 조직층 수
///   sopfr=5 → 질내 pH 상한 / 에스트로겐 대사 경로
///   sigma*phi+tau=28 → 월경 주기
///   sigma+phi=14 → 월경 pH 변동 주기
pub struct WomensIntimateCleanserLens;

// BT-1158 핵심 상수
const N6: f64 = 6.0;        // n=6 (완전수)
const TAU: f64 = 4.0;       // tau(6) = 약수 개수
const SIGMA: f64 = 12.0;    // sigma(6) = 약수합
const PHI: f64 = 2.0;       // phi(6) = 오일러 토티언트
const SOPFR: f64 = 5.0;     // sopfr(6) = 최대 소인수합
const MU: f64 = 1.0;        // mu(6) = 뫼비우스 함수 절댓값

impl Lens for WomensIntimateCleanserLens {
    fn name(&self) -> &str { "WomensIntimateCleanserLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // ── 1. 각 차원 평균·분산 계산 ────────────────────────────
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // ── 2. 마이크로바이옴/pH 스캔 (BT-1158) ────────────────
        // 질내pH=tau=4, 외음부pH=n=6, 질내상한=sopfr=5, 균총다양성=n=6
        let microbiome_ph_targets: &[f64] = &[TAU, N6, SOPFR, PHI, MU];
        let microbiome_ph_hits = means.iter().filter(|&&m| {
            microbiome_ph_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let microbiome_ph_score = microbiome_ph_hits as f64 / d.max(1) as f64;

        // ── 3. 생화학/월경 주기 스캔 ────────────────────────────
        // 월경주기=sigma*phi+tau=28, pH변동주기=sigma+phi=14, 대사경로=sopfr=5
        let menstrual_targets: &[f64] = &[
            SIGMA * PHI + TAU,  // 28 (월경 주기)
            SIGMA + PHI,        // 14 (월경 pH 변동 주기)
            SOPFR,              // 5  (에스트로겐 대사 경로)
            N6,                 // 6
        ];
        let menstrual_hits = means.iter().filter(|&&m| {
            menstrual_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let menstrual_score = menstrual_hits as f64 / d.max(1) as f64;

        // ── 4. 계면활성제 화학 스캔 ─────────────────────────────
        // 임계미셀농도=sigma=12, 안전분류=tau=4, sigma-tau=8
        let surfactant_targets: &[f64] = &[SIGMA, TAU, SIGMA - TAU, SOPFR];
        let surfactant_hits = means.iter().filter(|&&m| {
            surfactant_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let surfactant_score = surfactant_hits as f64 / d.max(1) as f64;

        // ── 5. 해부학 스캔 ──────────────────────────────────────
        // 조직층=phi=2, 구역수=n=6, 표면적비=tau=4
        let anatomy_targets: &[f64] = &[PHI, N6, TAU, SOPFR];
        let anatomy_hits = means.iter().filter(|&&m| {
            anatomy_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let anatomy_score = anatomy_hits as f64 / d.max(1) as f64;

        // ── 6. n=6 차원 완전성 ──────────────────────────────────
        let n6_dim_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 7. 월경 주기 패턴 감지 ──────────────────────────────
        // 월경 주기 28일: 라그-1 자기상관으로 주기성 추정
        let cycle_periodicity: Vec<f64> = (0..d).map(|j| {
            if n < 4 { return 0.0; }
            let m = means[j];
            let detrended: Vec<f64> = (0..n).map(|i| data[i * d + j] - m).collect();
            let ac1_num: f64 = (1..n).map(|i| detrended[i] * detrended[i - 1]).sum();
            let ac1_den: f64 = (0..n).map(|i| detrended[i].powi(2)).sum::<f64>().max(1e-15);
            (ac1_num / ac1_den).clamp(-1.0, 1.0)
        }).collect();
        let mean_periodicity = cycle_periodicity.iter().sum::<f64>() / d.max(1) as f64;
        // tau=4 주기 기준: pH 완충 4단계 → 0.25 진동률 기대
        let cycle_rhythm = (1.0 - (mean_periodicity - 1.0 / TAU).abs() * 4.0).max(0.0);

        // ── 8. 신호 균일성 ──────────────────────────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_mean = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let signal_quality = 1.0 - (mean_std / mean_mean.max(1e-12)).min(1.0);

        // ── 9. 전체 n=6 상수 공명 스캔 ──────────────────────────
        let all_targets: &[f64] = &[
            N6, TAU, SIGMA, PHI, SOPFR, MU,
            SIGMA * PHI + TAU,  // 28
            SIGMA + PHI,        // 14
        ];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 10. 종합 여성청결제 점수 ────────────────────────────
        let cleanser_score =
            microbiome_ph_score * 0.25
            + menstrual_score     * 0.20
            + surfactant_score    * 0.15
            + anatomy_score       * 0.15
            + n6_dim_score        * 0.10
            + cycle_rhythm        * 0.10
            + n6_resonance        * 0.05;

        let mut r = HashMap::new();
        r.insert("microbiome_ph_score".to_string(), vec![microbiome_ph_score]);
        r.insert("menstrual_score".to_string(),     vec![menstrual_score]);
        r.insert("surfactant_score".to_string(),    vec![surfactant_score]);
        r.insert("anatomy_score".to_string(),       vec![anatomy_score]);
        r.insert("n6_dim_score".to_string(),        vec![n6_dim_score]);
        r.insert("cycle_rhythm".to_string(),        vec![cycle_rhythm]);
        r.insert("signal_quality".to_string(),      vec![signal_quality]);
        r.insert("n6_resonance".to_string(),        vec![n6_resonance]);
        r.insert("microbiome_ph_hits".to_string(),  vec![microbiome_ph_hits as f64]);
        r.insert("total_hits".to_string(),          vec![total_hits as f64]);
        r.insert("cleanser_score".to_string(),      vec![cleanser_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_여성청결제_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,    // 외음부 pH = 6
                1 => TAU,   // 질내 pH = 4
                2 => SIGMA, // 임계미셀농도 = 12
                3 => PHI,   // 조직층 수 = 2
                4 => SOPFR, // 질내 pH 상한 = 5
                _ => MU,    // mu(6) = 1
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = WomensIntimateCleanserLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("cleanser_score"), "cleanser_score 키 존재");
        assert!(result.contains_key("microbiome_ph_score"), "microbiome_ph_score 키 존재");
        assert!(result.contains_key("n6_resonance"), "n6_resonance 키 존재");
        let score = result["cleanser_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "cleanser_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_여성청결제_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = WomensIntimateCleanserLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_여성청결제_n6_차원_점수() {
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 + 1.0).sqrt()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = WomensIntimateCleanserLens.scan(&data, n, d, &shared);
        let dim = result["n6_dim_score"][0];
        assert!((dim - 1.0).abs() < 1e-9, "d=6이면 n6_dim_score=1.0: {}", dim);
    }

    #[test]
    fn test_여성청결제_월경주기_히트() {
        // 평균이 28(월경주기), 14(pH변동), 5(대사경로)에 맞는 데이터
        let n = 10; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => 28.0,   // sigma*phi+tau = 월경 주기
                1 => 14.0,   // sigma+phi = pH 변동 주기
                _ => SOPFR,  // 에스트로겐 대사 경로
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = WomensIntimateCleanserLens.scan(&data, n, d, &shared);
        let hits = result["total_hits"][0];
        assert!(hits >= 3.0, "월경 주기 상수 3개 히트 확인: {}", hits);
    }

    #[test]
    fn test_여성청결제_전체_키_존재() {
        let n = 12; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 % 12.0 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = WomensIntimateCleanserLens.scan(&data, n, d, &shared);
        let expected_keys = [
            "microbiome_ph_score", "menstrual_score", "surfactant_score",
            "anatomy_score", "n6_dim_score", "cycle_rhythm",
            "cleanser_score", "n6_resonance",
        ];
        for key in &expected_keys {
            assert!(result.contains_key(*key), "키 존재 확인: {}", key);
        }
    }
}
