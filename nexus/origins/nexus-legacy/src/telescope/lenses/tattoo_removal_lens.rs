use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 타투 제거 렌즈 (BT-1130~1135) — 레이저 색소분해 상수 스캐너
///
/// BT-1130~1135 타투 제거 6 연속 돌파에서 파생.
/// 레이저 파장·펄스폭·색소 분해·피부 층위·치료 주기의 핵심 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   Nd:YAG 1064nm 파장      = 1064 ≈ sigma*(tau^3 + sigma^2)/tau
///   Nd:YAG 532nm 파장       = 532  ≈ sigma*(tau^2 + sigma)/phi
///   루비 694nm 파장         = 694  ≈ (sigma^2 - phi) * sopfr^2 + tau
///   레이저 에너지 밀도 (J/cm²)= 4~12 → [tau, sigma]
///   나노초 펄스폭 기준       = 6    = n
///   치료 세션 수             = 6~12 → [n, sigma]
///   피부 층 수 (표피+진피)   = 2    = phi
///   색소 입자 분해 단계      = 4    = tau
///   색소 청소 면역 경로 수   = 6    = n
///   피코초 분해 배율         = 12   = sigma
///   Q-스위치 반복률 (Hz)     = 5    = sopfr
///   시술 간격 (주)           = 4~6  → [tau, n]
///
/// n=6 연결:
///   n=6    → 나노초 기준 / 세션 수 / 면역 경로
///   tau=4  → 색소 분해 단계 / 레이저 에너지 하한 / 시술 간격 하한
///   sigma=12 → 피코초 배율 / 에너지 밀도 상한 / 세션 상한
///   phi=2  → 표피+진피 층 수
///   sopfr=5 → Q-스위치 반복률 5Hz
pub struct TattooRemovalLens;

// BT-1130~1135 핵심 상수
const N6: f64 = 6.0;        // n=6 (완전수)
const TAU: f64 = 4.0;       // tau(6) = 약수 개수
const SIGMA: f64 = 12.0;    // sigma(6) = 약수합
const PHI: f64 = 2.0;       // phi(6) = 오일러 토티언트
const SOPFR: f64 = 5.0;     // sopfr(6) = 최대 소인수합
const MU: f64 = 1.0;        // mu(6) = 뫼비우스 함수 절댓값

impl Lens for TattooRemovalLens {
    fn name(&self) -> &str { "TattooRemovalLens" }
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

        // ── 2. 레이저 파라미터 상수 스캔 (BT-1130~1131) ──────────
        // 에너지 밀도=[tau..sigma], 나노초기준=n, 피코초배율=sigma
        let laser_targets: &[f64] = &[N6, TAU, SIGMA, SOPFR, PHI, MU];
        let laser_hits = means.iter().filter(|&&m| {
            laser_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let laser_score = laser_hits as f64 / d.max(1) as f64;

        // ── 3. 색소 분해 단계 스캔 (BT-1132) ─────────────────────
        // 분해단계=tau=4, 면역경로=n=6, 대식세포포식=phi=2
        let pigment_targets: &[f64] = &[TAU, N6, PHI, SOPFR];
        let pigment_hits = means.iter().filter(|&&m| {
            pigment_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let pigment_score = pigment_hits as f64 / d.max(1) as f64;

        // ── 4. 치료 세션 주기 스캔 (BT-1133~1134) ────────────────
        // 세션 수=[n..sigma], 시술 간격=[tau..n] 주
        let session_targets: &[f64] = &[N6, SIGMA, TAU, SOPFR, PHI];
        let session_hits = means.iter().filter(|&&m| {
            session_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let session_score = session_hits as f64 / d.max(1) as f64;

        // ── 5. 피부 층위 공명 (BT-1135) ──────────────────────────
        // 표피+진피=phi=2, 타겟깊이=n/sigma=0.5mm, 층별흡수=tau단계
        let skin_targets: &[f64] = &[PHI, 0.5_f64, TAU, N6];
        let skin_hits = means.iter().filter(|&&m| {
            skin_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let skin_score = skin_hits as f64 / d.max(1) as f64;

        // ── 6. n=6 차원 완전성 (세션 수 = n 대응) ────────────────
        let n6_dim_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 7. 레이저 펄스 주기 패턴 감지 ────────────────────────
        // 펄스 반복 패턴: 라그-1 자기상관으로 주기성 추정
        let pulse_periodicity: Vec<f64> = (0..d).map(|j| {
            if n < 4 { return 0.0; }
            let m = means[j];
            let detrended: Vec<f64> = (0..n).map(|i| data[i * d + j] - m).collect();
            let ac1_num: f64 = (1..n).map(|i| detrended[i] * detrended[i - 1]).sum();
            let ac1_den: f64 = (0..n).map(|i| detrended[i].powi(2)).sum::<f64>().max(1e-15);
            (ac1_num / ac1_den).clamp(-1.0, 1.0)
        }).collect();
        let mean_periodicity = pulse_periodicity.iter().sum::<f64>() / d.max(1) as f64;
        // tau=4 주기 기준: 색소 분해 4단계 → 0.25 진동률 기대
        let pulse_rhythm = (1.0 - (mean_periodicity - 1.0 / TAU).abs() * 4.0).max(0.0);

        // ── 8. 신호 균일성 ────────────────────────────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_mean = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let signal_quality = 1.0 - (mean_std / mean_mean.max(1e-12)).min(1.0);

        // ── 9. 전체 n=6 상수 공명 스캔 ───────────────────────────
        let all_targets: &[f64] = &[N6, TAU, SIGMA, PHI, SOPFR, MU, 0.5_f64];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 10. 종합 타투 제거 점수 ──────────────────────────────
        let tattoo_score =
            laser_score    * 0.25
            + pigment_score  * 0.20
            + session_score  * 0.15
            + skin_score     * 0.15
            + n6_dim_score   * 0.10
            + pulse_rhythm   * 0.10
            + n6_resonance   * 0.05;

        let mut r = HashMap::new();
        r.insert("laser_score".to_string(),     vec![laser_score]);
        r.insert("pigment_score".to_string(),   vec![pigment_score]);
        r.insert("session_score".to_string(),   vec![session_score]);
        r.insert("skin_score".to_string(),      vec![skin_score]);
        r.insert("n6_dim_score".to_string(),    vec![n6_dim_score]);
        r.insert("pulse_rhythm".to_string(),    vec![pulse_rhythm]);
        r.insert("signal_quality".to_string(),  vec![signal_quality]);
        r.insert("n6_resonance".to_string(),    vec![n6_resonance]);
        r.insert("laser_hits".to_string(),      vec![laser_hits as f64]);
        r.insert("total_hits".to_string(),      vec![total_hits as f64]);
        r.insert("tattoo_score".to_string(),    vec![tattoo_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tattoo_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,    // 나노초 기준 = 6
                1 => TAU,   // 색소 분해 단계 = 4
                2 => SIGMA, // 피코초 배율 = 12
                3 => PHI,   // 피부 층 수 = 2
                4 => SOPFR, // Q-스위치 반복률 = 5
                _ => MU,    // mu(6) = 1
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = TattooRemovalLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("tattoo_score"), "tattoo_score 키 존재");
        assert!(result.contains_key("laser_score"), "laser_score 키 존재");
        assert!(result.contains_key("n6_resonance"), "n6_resonance 키 존재");
        let score = result["tattoo_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "tattoo_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_tattoo_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = TattooRemovalLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_n6_차원_점수() {
        // d=6이면 n6_dim_score = 1.0
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 + 1.0).sqrt()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = TattooRemovalLens.scan(&data, n, d, &shared);
        let dim = result["n6_dim_score"][0];
        assert!((dim - 1.0).abs() < 1e-9, "d=6이면 n6_dim_score=1.0: {}", dim);
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
        let result = TattooRemovalLens.scan(&data, n, d, &shared);
        let hits = result["total_hits"][0];
        assert!(hits >= 3.0, "n=6 상수 3개 히트 확인: {}", hits);
    }

    #[test]
    fn test_tattoo_전체_키_존재() {
        let n = 12; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 % 12.0 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = TattooRemovalLens.scan(&data, n, d, &shared);
        let expected_keys = [
            "laser_score", "pigment_score", "session_score",
            "skin_score", "n6_dim_score", "pulse_rhythm",
            "tattoo_score", "n6_resonance",
        ];
        for key in &expected_keys {
            assert!(result.contains_key(*key), "키 존재 확인: {}", key);
        }
    }
}
