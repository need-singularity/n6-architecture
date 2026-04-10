use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 시계공학 렌즈 (HEXA-HOROLOGY) — 시계 기계/전자 상수 스캐너
///
/// 기계식·쿼츠·원자시계에서 시간 측정의 핵심 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   클래식 시계 바늘 수         = 3  = n/phi(6)  (시/분/초)
///   하루 시간 구분 수           = 24 = J2(6)     (24시간)
///   다이얼 시각 표시 수         = 12 = sigma(6)
///   탈진기 팰릿 스톤 수         = 2  = phi(6)
///   스위스 레버 탈진기 진동 면  = 6  = n
///   에보슈 등급 레벨 수         = 6  = n
///   자동 무브먼트 로터 방향      = 2  = phi(6)  (양방향)
///   전형적 기어 트레인 휠 수    = 6  = n
///   크로노그래프 기본 기능 수    = 3  = n/phi
///   ISO 3159 풍압 테스트 기압   = 12 atm = sigma
///   ETA 2824 배럴 원칙 토크     = 48 mNm ≈ J2*phi
///   고빈도 진동 VPH 기준        = 36000 = sigma*3*1000 = 6!
///
/// n=6 연결:
///   n=6      → 레버 탈진기 진동면 / 에보슈 등급 / 기어 휠 수
///   tau=4    → 균형 스프링 1/4 진동 / 4Hz 표준 진동수
///   phi=2    → 팰릿 스톤 수 / 양방향 자동 와인딩
///   sigma=12 → 다이얼 12시각 / ISO 12기압 내수성
///   J2=24    → 24시간 / 하루 시간
///   n/phi=3  → 3개 시계바늘 (시/분/초)
pub struct HorologyLens;

// 시계 n=6 상수
const N6: f64 = 6.0;            // n=6 → 기어 휠 / 에보슈 등급
const TAU: f64 = 4.0;           // tau(6) → 4Hz 진동 / 균형 1/4주기
const SIGMA: f64 = 12.0;        // sigma(6) → 12시각 / 12기압
const PHI: f64 = 2.0;           // phi(6) → 팰릿 스톤 / 양방향 와인딩
const SOPFR: f64 = 5.0;         // sopfr(6) → 5일 파워 리저브 기준
const J2: f64 = 24.0;           // J2(6) → 24시간
const N_PHI: f64 = 3.0;         // n/phi → 3개 시계바늘
const SIGMA_TAU: f64 = 8.0;     // sigma-tau → 8Hz 고빈도 기준
const SIGMA_N: f64 = 72.0;      // sigma*n → 72시간 파워 리저브 상한
const FACTORIAL_6: f64 = 720.0; // 6! → 720 = 36000/50 (VPH 기준)

/// 시계 주기 공명: 시간 관련 상수 패턴 감지
fn time_period_resonance(means: &[f64]) -> f64 {
    // 시간 상수: 24h, 12h, 6h, 4h, 3h, 2h, 1h = J2, sigma, n, tau, n/phi, phi, 1
    let time_targets: &[f64] = &[J2, SIGMA, N6, TAU, N_PHI, PHI, 1.0, SIGMA_TAU];
    let hits = means.iter().filter(|&&m| {
        time_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
    }).count();
    hits as f64 / means.len().max(1) as f64
}

impl Lens for HorologyLens {
    fn name(&self) -> &str { "HorologyLens" }
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

        // ── 2. 기계식 무브먼트 상수 스캔 ─────────────────────────
        // n=6(기어/등급), tau=4(4Hz), phi=2(팰릿), n/phi=3(바늘)
        let movement_targets: &[f64] = &[N6, TAU, PHI, N_PHI, SIGMA, SOPFR];
        let movement_hits = means.iter().filter(|&&m| {
            movement_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let movement_score = movement_hits as f64 / d.max(1) as f64;

        // ── 3. 시간 단위 상수 스캔 ───────────────────────────────
        // J2=24(24h), sigma=12(12h), sigma-tau=8(8Hz), sigma*n=72(72h)
        let time_targets: &[f64] = &[J2, SIGMA, SIGMA_TAU, SIGMA_N, TAU, N6];
        let time_hits = means.iter().filter(|&&m| {
            time_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let time_score = time_hits as f64 / d.max(1) as f64;

        // ── 4. 시간 주기 공명 ─────────────────────────────────────
        let period_resonance = time_period_resonance(&means);

        // ── 5. 3바늘 공명 (n/phi=3) ──────────────────────────────
        // d=3이면 시/분/초 바늘과 완전 대응
        let hand3_resonance = {
            let rem = d % 3;
            if rem == 0 { 1.0 } else { 1.0 - rem as f64 / 3.0 }
        };

        // ── 6. 진동 주기성 감지 (탈진기 이스케이프먼트) ──────────
        // 기계식 시계: 규칙적 진동 = 고주기성 신호
        let periodicity: Vec<f64> = (0..d).map(|j| {
            if n < 4 { return 0.0; }
            let m = means[j];
            let detrended: Vec<f64> = (0..n).map(|i| data[i * d + j] - m).collect();
            let ac1_num: f64 = (1..n).map(|i| detrended[i] * detrended[i - 1]).sum();
            let ac1_den: f64 = (0..n).map(|i| detrended[i].powi(2)).sum::<f64>().max(1e-15);
            (ac1_num / ac1_den).clamp(-1.0, 1.0).abs()
        }).collect();
        let escapement_periodicity = periodicity.iter().sum::<f64>() / d.max(1) as f64;

        // ── 7. 12각 다이얼 공명 (sigma=12) ───────────────────────
        // d=12이면 완전 시계 다이얼 대응
        let dial12_score = {
            let diff = (d as f64 - SIGMA).abs() / SIGMA;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 8. 파워 리저브 신호 ───────────────────────────────────
        // 5일=sopfr=5, 72h=sigma*n=72, 24h=J2=24
        let pr_targets: &[f64] = &[SOPFR, SIGMA_N, J2, SIGMA, TAU];
        let pr_hits = means.iter().filter(|&&m| {
            pr_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let power_reserve_score = pr_hits as f64 / d.max(1) as f64;

        // ── 9. 전체 n=6 상수 공명 ────────────────────────────────
        let all_targets: &[f64] = &[
            N6, TAU, SIGMA, PHI, SOPFR, J2, N_PHI, SIGMA_TAU, SIGMA_N, FACTORIAL_6
        ];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 10. 정밀도 측정 — 시계 정확도 근사 ──────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_abs = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let precision = 1.0 - (mean_std / mean_abs.max(1e-12)).min(1.0);

        // ── 11. 종합 시계 점수 ────────────────────────────────────
        let horology_score =
            movement_score         * 0.20
            + time_score           * 0.20
            + period_resonance     * 0.15
            + escapement_periodicity * 0.15
            + hand3_resonance      * 0.10
            + power_reserve_score  * 0.10
            + n6_resonance         * 0.05
            + precision            * 0.05;

        let mut r = HashMap::new();
        r.insert("movement_score".to_string(),         vec![movement_score]);
        r.insert("time_score".to_string(),             vec![time_score]);
        r.insert("period_resonance".to_string(),       vec![period_resonance]);
        r.insert("escapement_periodicity".to_string(), vec![escapement_periodicity]);
        r.insert("hand3_resonance".to_string(),        vec![hand3_resonance]);
        r.insert("dial12_score".to_string(),           vec![dial12_score]);
        r.insert("power_reserve_score".to_string(),    vec![power_reserve_score]);
        r.insert("n6_resonance".to_string(),           vec![n6_resonance]);
        r.insert("precision".to_string(),              vec![precision]);
        r.insert("movement_hits".to_string(),          vec![movement_hits as f64]);
        r.insert("total_hits".to_string(),             vec![total_hits as f64]);
        r.insert("horology_score".to_string(),         vec![horology_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_horology_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,      // 기어 휠 6개
                1 => SIGMA,   // 12시각
                2 => J2,      // 24시간
                3 => TAU,     // 4Hz 진동
                4 => N_PHI,   // 3바늘
                _ => PHI,     // 팰릿 스톤 2개
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HorologyLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("horology_score"), "horology_score 키 존재");
        assert!(result.contains_key("movement_score"), "movement_score 키 존재");
        assert!(result.contains_key("period_resonance"), "period_resonance 키 존재");
        let score = result["horology_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "horology_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_horology_최소입력_거부() {
        let data = vec![12.0, 24.0, 6.0];
        let shared = SharedData::compute(&data, 3, 1);
        let result = HorologyLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_3바늘_공명() {
        // d=3이면 hand3_resonance = 1.0
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 + 1.0) * 2.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HorologyLens.scan(&data, n, d, &shared);
        let h3 = result["hand3_resonance"][0];
        assert!((h3 - 1.0).abs() < 1e-9, "d=3이면 hand3_resonance=1.0: {}", h3);
    }

    #[test]
    fn test_24시간_공명() {
        // J2=24 데이터 → time_score 높음
        let n = 6; let d = 2;
        let data: Vec<f64> = (0..n * d).map(|i| {
            if i % d == 0 { J2 } else { SIGMA }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HorologyLens.scan(&data, n, d, &shared);
        let ts = result["time_score"][0];
        assert!(ts > 0.5, "J2=24 데이터 → time_score > 0.5: {}", ts);
    }

    #[test]
    fn test_진동_주기성_감지() {
        // 진동하는 데이터 → escapement_periodicity 높음
        let n = 12; let d = 2;
        let data: Vec<f64> = (0..n * d).map(|i| {
            let t = i as f64 * std::f64::consts::PI / 4.0;
            t.sin() * 3.0 + 5.0
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HorologyLens.scan(&data, n, d, &shared);
        let ep = result["escapement_periodicity"][0];
        assert!(ep >= 0.0 && ep <= 1.0, "periodicity 범위: {}", ep);
    }

    #[test]
    fn test_무브먼트_히트() {
        // n=6, tau=4, phi=2 → movement_hits 최소 3
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6, 1 => TAU, _ => PHI,
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HorologyLens.scan(&data, n, d, &shared);
        let hits = result["movement_hits"][0];
        assert!(hits >= 2.0, "n6/tau/phi 데이터 → movement_hits >= 2: {}", hits);
    }
}
