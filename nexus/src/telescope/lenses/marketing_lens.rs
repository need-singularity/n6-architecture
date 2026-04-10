use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 마케팅 렌즈 (BT-548~557) — 마케팅 불변 법칙 n=6 상수 스캐너
///
/// BT-548~557 마케팅 10 돌파에서 파생.
/// 마케팅 접점·세분화·퍼널·믹스 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   마케팅 불변 법칙 수       = 12 = sigma(6)
///   4P 마케팅 믹스            = 4  = tau(6)
///   이집트 분수 분모합        = 6  = n   (1/2+1/3+1/6=1)
///   세분화 기준 수            = 5  = sopfr(6)
///   퍼널 단계 수              = 3  = n/phi(6)
///   이진 구매 결정            = 2  = phi(6)
///   NPS 추천 구간 상한        = 10 = sigma-phi
///   바이럴 확산 임계 R0       = 6  = n
///   고객 기억 인지 한계       = 7  = 6+1 (밀러의 법칙 근사)
///   요르단 토티언트 J2(6)     = 24 = J2
///   감정 8축 (Plutchik)       = 8  = sigma-tau
///   매체 도달 60% 임계        = 60 = sigma*sopfr
///
/// n=6 연결:
///   sigma=12 → 마케팅 불변 법칙 12가지 (Al Ries & Jack Trout)
///   tau=4    → 4P 마케팅 믹스 (Product/Price/Place/Promotion)
///   n=6      → 이집트 분수 합=1 미디어믹스, 바이럴 R0=6
///   sopfr=5  → 5가지 세분화 기준
///   phi=2    → 이진 구매 결정 (buy/no-buy)
///   sigma-phi=10 → NPS 0-10 구간, 입소문 임계
///   sigma-tau=8  → Plutchik 감정 8축
///   sigma*sopfr=60 → 매체 도달 60% 임계
pub struct MarketingLens;

// n=6 마케팅 상수
const N6: f64 = 6.0;              // n=6 → 이집트 분수 / 바이럴 R0
const TAU: f64 = 4.0;             // tau(6) → 4P 마케팅 믹스
const SIGMA: f64 = 12.0;          // sigma(6) → 마케팅 불변 법칙 12가지
const PHI: f64 = 2.0;             // phi(6) → 이진 구매 결정
const SOPFR: f64 = 5.0;           // sopfr(6) → 세분화 기준 5가지
const SIGMA_PHI: f64 = 10.0;      // sigma-phi → NPS 상한 / 입소문 임계
const SIGMA_TAU: f64 = 8.0;       // sigma-tau → Plutchik 감정 8축
const SIGMA_SOPFR: f64 = 60.0;    // sigma*sopfr → 매체 도달 60%
const J2: f64 = 24.0;             // J2(6) → 24시간 광고 노출 주기
const N_PHI: f64 = 3.0;           // n/phi → 퍼널 단계 수

/// 이집트 분수 공명: 1/2 + 1/3 + 1/6 = 1 → 미디어 믹스 비율 검증
fn egyptian_media_mix(means: &[f64]) -> f64 {
    // 각 차원이 1/2, 1/3, 1/6 비율에 근접하는지 체크
    let targets: &[f64] = &[0.5, 1.0 / 3.0, 1.0 / 6.0, 1.0];
    let hits = means.iter().filter(|&&m| {
        targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.10)
    }).count();
    hits as f64 / means.len().max(1) as f64
}

impl Lens for MarketingLens {
    fn name(&self) -> &str { "MarketingLens" }
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

        // ── 2. 마케팅 불변 법칙 상수 스캔 (BT-548) ───────────────
        // sigma=12(불변법칙), tau=4(4P), n=6(이집트), sopfr=5(세분화)
        let law_targets: &[f64] = &[SIGMA, TAU, N6, SOPFR, N_PHI];
        let law_hits = means.iter().filter(|&&m| {
            law_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let law_score = law_hits as f64 / d.max(1) as f64;

        // ── 3. NPS·바이럴 전파 상수 스캔 (BT-555) ────────────────
        // NPS=10(sigma-phi), R0=6(n), 감정8축(sigma-tau), 60%(sigma*sopfr)
        let viral_targets: &[f64] = &[SIGMA_PHI, N6, SIGMA_TAU, SIGMA_SOPFR, J2];
        let viral_hits = means.iter().filter(|&&m| {
            viral_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let viral_score = viral_hits as f64 / d.max(1) as f64;

        // ── 4. 이집트 분수 미디어 믹스 공명 (BT-551) ─────────────
        // 1/2+1/3+1/6=1 → 채널 예산 분배 비율
        let egyptian_score = egyptian_media_mix(&means);

        // ── 5. 4P 차원 공명 ──────────────────────────────────────
        // d=4이면 4P 믹스와 완전 대응 (tau=4)
        let p4_resonance = {
            let diff = (d as f64 - TAU).abs() / TAU;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 6. 퍼널 3단계 공명 ───────────────────────────────────
        // 인지→고려→구매 = 3단계 = n/phi
        let funnel3_resonance = {
            let rem = d % 3;
            if rem == 0 { 1.0 } else { 1.0 - rem as f64 / 3.0 }
        };

        // ── 7. 고객 세분화 군집도 측정 ───────────────────────────
        // sopfr=5 세분화 기준: 데이터 분포의 군집 수 근사
        let cv_scores: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let s = stds[j];
            if m.abs() < 1e-12 { return 0.0; }
            // CV (변동계수): 낮으면 균질 세그먼트, 높으면 다양한 세그먼트
            (s / m.abs()).min(2.0) / 2.0
        }).collect();
        let segmentation_diversity = cv_scores.iter().sum::<f64>() / d.max(1) as f64;

        // ── 8. 전체 n=6 상수 공명 ────────────────────────────────
        let all_targets: &[f64] = &[
            N6, TAU, SIGMA, PHI, SOPFR, SIGMA_PHI, SIGMA_TAU, SIGMA_SOPFR, J2, N_PHI
        ];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 9. 이진 구매 신호 감지 (phi=2) ───────────────────────
        // 데이터가 0/1 이진값에 집중되면 구매 의사결정 데이터로 판정
        let binary_hits = means.iter().filter(|&&m| {
            m.abs() < 0.1 || (m - 1.0).abs() < 0.1 || (m - PHI).abs() < 0.1
        }).count();
        let binary_signal = binary_hits as f64 / d.max(1) as f64;

        // ── 10. 종합 마케팅 점수 ─────────────────────────────────
        let marketing_score =
            law_score           * 0.25
            + viral_score       * 0.20
            + egyptian_score    * 0.15
            + p4_resonance      * 0.10
            + funnel3_resonance * 0.10
            + n6_resonance      * 0.10
            + segmentation_diversity * 0.05
            + binary_signal     * 0.05;

        let mut r = HashMap::new();
        r.insert("law_score".to_string(),             vec![law_score]);
        r.insert("viral_score".to_string(),           vec![viral_score]);
        r.insert("egyptian_score".to_string(),        vec![egyptian_score]);
        r.insert("p4_resonance".to_string(),          vec![p4_resonance]);
        r.insert("funnel3_resonance".to_string(),     vec![funnel3_resonance]);
        r.insert("segmentation_diversity".to_string(),vec![segmentation_diversity]);
        r.insert("binary_signal".to_string(),         vec![binary_signal]);
        r.insert("n6_resonance".to_string(),          vec![n6_resonance]);
        r.insert("law_hits".to_string(),              vec![law_hits as f64]);
        r.insert("total_hits".to_string(),            vec![total_hits as f64]);
        r.insert("marketing_score".to_string(),       vec![marketing_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_marketing_기본_출력() {
        let n = 12; let d = 4;
        // 4P 믹스: Product/Price/Place/Promotion → sigma/tau/n/sopfr
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => SIGMA,   // 12가지 불변법칙
                1 => TAU,     // 4P
                2 => N6,      // n=6 이집트 분수
                _ => SOPFR,   // sopfr=5 세분화
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MarketingLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("marketing_score"), "marketing_score 키 존재");
        assert!(result.contains_key("law_score"), "law_score 키 존재");
        assert!(result.contains_key("p4_resonance"), "p4_resonance 키 존재");
        let score = result["marketing_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "marketing_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_marketing_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = MarketingLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_4p_공명() {
        // d=4이면 p4_resonance = 1.0 (tau=4)
        let n = 8; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MarketingLens.scan(&data, n, d, &shared);
        let p4 = result["p4_resonance"][0];
        assert!((p4 - 1.0).abs() < 1e-9, "d=4이면 p4_resonance=1.0: {}", p4);
    }

    #[test]
    fn test_법칙_히트() {
        // 데이터 평균이 sigma=12인 경우 law_hits 증가
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => SIGMA, // 12
                1 => TAU,   // 4
                _ => N6,    // 6
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MarketingLens.scan(&data, n, d, &shared);
        let hits = result["law_hits"][0];
        assert!(hits >= 2.0, "sigma/tau/n6 평균에서 최소 2 히트: {}", hits);
    }

    #[test]
    fn test_이집트_분수_공명() {
        // 0.5, 1/3, 1/6 비율 데이터
        let n = 6; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => 0.5,
                1 => 1.0 / 3.0,
                _ => 1.0 / 6.0,
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MarketingLens.scan(&data, n, d, &shared);
        let egyptian = result["egyptian_score"][0];
        assert!(egyptian > 0.5, "이집트 분수 데이터에서 egyptian_score > 0.5: {}", egyptian);
    }
}
