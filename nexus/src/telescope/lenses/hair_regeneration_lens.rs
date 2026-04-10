use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 모발재생 렌즈 (BT-1115~1120) — 모낭 재생 상수 스캐너
///
/// BT-1115~1120 탈모치료제 6 연속 돌파에서 파생.
/// 모낭 해부학·신호전달·치료·전달·평가의 핵심 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   모낭 동심 구조 층 수  = 6 = n
///   모발 성장 주기 단계   = 4 = tau(6)
///   핵심 신호 경로 수     = 6 = n  (Wnt/SHH/BMP/Notch/GF/DHT)
///   Wnt 리간드 종류       = 6 = n
///   성장인자 종류         = 6 = n
///   Notch 수용체 종류     = 4 = tau
///   치료 모달리티 수      = 6 = n
///   나노입자 표적 크기    = 12 nm = sigma
///   전달 경로 수          = 6 = n
///   트리코스코피 지표 수  = 6 = n
///   치료 효과 판정 최소   = 12 주 = sigma
///   헥사펩타이드 잔기 수  = 6 = n
///
/// n=6 연결:
///   n=6   → 모낭층/Wnt리간드/성장인자/치료모달리티/전달경로/트리코스코피
///   tau=4  → 성장주기/Notch수용체
///   sigma=12 → 나노입자크기(nm)/판정주기(주)
pub struct HairRegenerationLens;

// BT-1115~1120 핵심 상수
const N6: f64 = 6.0;          // n=6 (완전수)
const TAU: f64 = 4.0;         // tau(6) = 약수 개수
const SIGMA: f64 = 12.0;      // sigma(6) = 약수합
const PHI: f64 = 2.0;         // phi(6) = 오일러 토티언트
const SOPFR: f64 = 5.0;       // sopfr(6) = 최대 소인수합
const J2: f64 = 24.0;         // J_2(6) = 요르단 토티언트

impl Lens for HairRegenerationLens {
    fn name(&self) -> &str { "HairRegenerationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // ── 1. 각 차원 평균 계산 ──────────────────────────────────
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // ── 2. 모낭 해부학 상수 스캔 (BT-1115) ───────────────────
        // 핵심 상수: 6(층)/4(주기)/12(nm, 주)/2(IRS/ORS)/5(BMP아과)
        let anatomy_targets: &[f64] = &[N6, TAU, SIGMA, PHI, SOPFR, J2];
        let anatomy_hits = means.iter().filter(|&&m| {
            anatomy_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let anatomy_score = anatomy_hits as f64 / d.max(1) as f64;

        // ── 3. 신호전달 경로 공명 (BT-1116) ──────────────────────
        // Wnt리간드=6, Notch=4, BMP아과=5, 전체경로=6
        let signaling_targets: &[f64] = &[N6, TAU, SOPFR, PHI];
        let signaling_hits = means.iter().filter(|&&m| {
            signaling_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let signaling_score = signaling_hits as f64 / d.max(1) as f64;

        // ── 4. 치료 모달리티 점수 (BT-1117~1118) ─────────────────
        // 치료 모달리티=6, FDA승인=2, PRP시술=4~6회, RNA유형=4
        let treatment_targets: &[f64] = &[N6, PHI, TAU, SOPFR];
        let treatment_hits = means.iter().filter(|&&m| {
            treatment_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let treatment_score = treatment_hits as f64 / d.max(1) as f64;

        // ── 5. 나노전달 상수 스캔 (BT-1119) ─────────────────────
        // 나노입자=12nm(sigma), 전달경로=6, 침투깊이≈1.5=n/tau, 리포솜층=2
        let delivery_targets: &[f64] = &[SIGMA, N6, PHI, 1.5_f64];
        let delivery_hits = means.iter().filter(|&&m| {
            delivery_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let delivery_score = delivery_hits as f64 / d.max(1) as f64;

        // ── 6. 평가 지표 공명 (BT-1120) ──────────────────────────
        // 트리코스코피=6지표, 판정=12주, 해밀턴-노우드=7(sigma-sopfr)
        let outcome_targets: &[f64] = &[N6, SIGMA, 7.0_f64, TAU];
        let outcome_hits = means.iter().filter(|&&m| {
            outcome_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let outcome_score = outcome_hits as f64 / d.max(1) as f64;

        // ── 7. 모발 주기 진동 패턴 감지 ───────────────────────────
        // 성장기(2-6년)/퇴행기/휴지기/이탈기 = tau=4 단계
        // 연속 샘플 간 부호 변화로 주기성 추정
        let cycle_oscillations: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let sign_changes = (1..n).filter(|&i| {
                let prev = data[(i - 1) * d + j] - m;
                let curr = data[i * d + j] - m;
                prev * curr < 0.0
            }).count();
            sign_changes as f64 / (n - 1).max(1) as f64
        }).collect();

        let mean_oscillation = cycle_oscillations.iter().sum::<f64>() / d.max(1) as f64;
        // tau=4 주기 기준: 4단계 → 한 주기당 ~0.25 진동률 기대
        let tau_cycle_score = (1.0 - (mean_oscillation - 1.0 / TAU).abs() * 4.0).max(0.0);

        // ── 8. 6층 구조 완전성 점수 ───────────────────────────────
        // 데이터 차원이 6에 근접할수록 모낭 6층 구조와 대응
        let layer6_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 9. n=6 전체 공명 스캔 ────────────────────────────────
        // anatomy + signaling + treatment + delivery + outcome 통합
        let all_targets: &[f64] = &[N6, TAU, SIGMA, PHI, SOPFR, J2, 1.5_f64, 7.0_f64];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 10. 변동계수 — 신호 대 노이즈 ───────────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_mean = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let cv = mean_std / mean_mean.max(1e-12);
        let signal_quality = (1.0 - cv.min(2.0) / 2.0).max(0.0);

        // ── 11. 종합 모발재생 점수 ────────────────────────────────
        let hair_score =
            anatomy_score    * 0.20
            + signaling_score  * 0.20
            + treatment_score  * 0.15
            + delivery_score   * 0.15
            + outcome_score    * 0.10
            + tau_cycle_score  * 0.10
            + layer6_score     * 0.05
            + n6_resonance     * 0.05;

        let mut r = HashMap::new();
        r.insert("anatomy_score".to_string(),     vec![anatomy_score]);
        r.insert("signaling_score".to_string(),   vec![signaling_score]);
        r.insert("treatment_score".to_string(),   vec![treatment_score]);
        r.insert("delivery_score".to_string(),    vec![delivery_score]);
        r.insert("outcome_score".to_string(),     vec![outcome_score]);
        r.insert("tau_cycle_score".to_string(),   vec![tau_cycle_score]);
        r.insert("layer6_score".to_string(),      vec![layer6_score]);
        r.insert("n6_resonance".to_string(),      vec![n6_resonance]);
        r.insert("mean_oscillation".to_string(),  vec![mean_oscillation]);
        r.insert("signal_quality".to_string(),    vec![signal_quality]);
        r.insert("hair_score".to_string(),        vec![hair_score]);
        r.insert("anatomy_hits".to_string(),      vec![anatomy_hits as f64]);
        r.insert("total_hits".to_string(),        vec![total_hits as f64]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hair_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,    // 모낭층 수 = 6
                1 => TAU,   // 성장 주기 단계 = 4
                2 => SIGMA, // 나노입자 크기 = 12nm, 판정 = 12주
                3 => PHI,   // FDA 승인 = 2
                4 => SOPFR, // BMP 아과 = 5
                _ => J2,    // J_2(6) = 24
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HairRegenerationLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("hair_score"), "hair_score 키 존재");
        assert!(result.contains_key("anatomy_score"), "anatomy_score 키 존재");
        assert!(result.contains_key("n6_resonance"), "n6_resonance 키 존재");
        let score = result["hair_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "hair_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_hair_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = HairRegenerationLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_6층_구조_점수() {
        // d=6이면 layer6_score = 1.0
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64).sin()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HairRegenerationLens.scan(&data, n, d, &shared);
        let layer6 = result["layer6_score"][0];
        assert!((layer6 - 1.0).abs() < 1e-9, "d=6이면 layer6_score=1.0: {}", layer6);
    }

    #[test]
    fn test_n6_공명_히트() {
        // 평균이 n=6 상수에 정확히 맞는 데이터 → total_hits > 0
        let n = 10; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,
                1 => TAU,
                _ => SIGMA,
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HairRegenerationLens.scan(&data, n, d, &shared);
        let hits = result["total_hits"][0];
        assert!(hits >= 3.0, "n=6 상수 3개 히트 확인: {}", hits);
    }

    #[test]
    fn test_모발_전체_키_존재() {
        let n = 12; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 % 12.0 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HairRegenerationLens.scan(&data, n, d, &shared);
        let expected_keys = [
            "anatomy_score", "signaling_score", "treatment_score",
            "delivery_score", "outcome_score", "tau_cycle_score",
            "hair_score", "n6_resonance",
        ];
        for key in &expected_keys {
            assert!(result.contains_key(*key), "키 존재 확인: {}", key);
        }
    }
}
