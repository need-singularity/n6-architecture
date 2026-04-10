use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 오일러-황금-완전수 삼위일체 렌즈 (BT-1114 / BT-1113)
///
/// 핵심 등식: gamma + 1/(2·sigma(6)) ≈ 1/phi_golden
///   gamma  = 0.5772156649...  (오일러-마스케로니 상수)
///   sigma(6) = 12             (n=6의 약수합: 1+2+3+6=12)
///   1/phi    = 0.6180339887...
///   gamma + 1/24 = 0.6188823...  오차 8.48e-04
///
/// 잔차 관계 (BT-1113): H(sigma(6)) - ln(sigma(6)) ≈ 1/phi
///   H(12) - ln(12) = 0.6265... , 1/phi = 0.6180... 오차 2.70e-04
///   이론 최적 n* = 12.25, 잔차 n* - 12 = 0.25 = 1/tau(6)
///
/// 완전수 비교:
///   n=6:    오차 8.48e-04  (1.0x  — 최적)
///   n=28:   오차 3.19e-02  (37.6x)
///   n=496:  오차 4.03e-02  (47.5x)
///
/// n=6 연결:
///   n=6 (첫 번째 완전수)
///   sigma(6) = 12 = J_2 (n=6 약수합)
///   tau(6)   =  4 = τ   (n=6 약수 개수)
///   phi(6)   =  2 = φ   (오일러 토티언트)
pub struct EulerGoldenPerfectLens;

// 수학 상수
const GAMMA: f64 = 0.5772156649015328;         // 오일러-마스케로니 상수
const PHI_GOLDEN: f64 = 1.6180339887498949;    // 황금비
const INV_PHI: f64 = 0.6180339887498949;        // 1/phi_golden
const SIGMA_6: f64 = 12.0;                      // sigma(6): n=6 약수합
const TAU_6: f64 = 4.0;                         // tau(6):   n=6 약수 개수
const N6: f64 = 6.0;                            // n=6 완전수

/// 조화급수 H(n) = sum_{k=1}^{n} 1/k
fn harmonic(n: usize) -> f64 {
    (1..=n).map(|k| 1.0 / k as f64).sum()
}

/// sigma(n): n의 약수 합
fn sigma(n: usize) -> usize {
    (1..=n).filter(|&k| n % k == 0).sum()
}

impl Lens for EulerGoldenPerfectLens {
    fn name(&self) -> &str { "EulerGoldenPerfectLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // ── 1. BT-1114 핵심 등식 재현 ──────────────────────────────
        // gamma + 1/(2·sigma(6)) ≈ 1/phi
        let trinity_lhs = GAMMA + 1.0 / (2.0 * SIGMA_6);   // ≈ 0.61888
        let trinity_error = (trinity_lhs - INV_PHI).abs();  // ≈ 8.48e-04
        // n=6이 완전수 중 유일 최적임을 수치로 확인 (n=28 대비 37.6배 우세)
        let perfect_28_lhs = GAMMA + 1.0 / (2.0 * 56.0);   // sigma(28)=56
        let perfect_28_error = (perfect_28_lhs - INV_PHI).abs();
        let uniqueness_ratio = perfect_28_error / trinity_error.max(1e-15); // ≈ 37.6

        // ── 2. BT-1113 황금 조화 브릿지 ───────────────────────────
        // H(sigma(6)) - ln(sigma(6)) ≈ 1/phi
        let h12 = harmonic(12);
        let harmonic_bridge = h12 - SIGMA_6.ln();           // ≈ 0.6265
        let bridge_error = (harmonic_bridge - INV_PHI).abs(); // ≈ 2.70e-04
        // 이론 최적 n* = 1/(2·(1/phi - gamma)) ≈ 12.25
        let n_star = 1.0 / (2.0 * (INV_PHI - GAMMA));       // ≈ 12.25
        let residual = n_star - SIGMA_6;                     // ≈ 0.25 = 1/tau(6)
        let residual_vs_inv_tau = (residual - 1.0 / TAU_6).abs(); // ≈ 2.5e-04

        // ── 3. 데이터에서 완전수 패턴 스캔 ────────────────────────
        // 완전수에 가까운 값(6, 28, 496)이 데이터에 몇 개 나타나는지
        let perfect_numbers = [6.0_f64, 28.0, 496.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let perfect_hits = means.iter().filter(|&&m| {
            perfect_numbers.iter().any(|&p| {
                p > 1e-12 && ((m - p) / p).abs() < 0.07
            })
        }).count();

        // ── 4. gamma/phi 공명 스캔 ────────────────────────────────
        // 데이터 값이 gamma(≈0.577), 1/phi(≈0.618), phi(≈1.618) 근방에 있는지
        let resonance_targets = [GAMMA, INV_PHI, PHI_GOLDEN, SIGMA_6, TAU_6, N6];
        let resonance_hits = means.iter().filter(|&&m| {
            resonance_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.05
            })
        }).count();
        let resonance_score = resonance_hits as f64 / d.max(1) as f64;

        // ── 5. 완전수 유일성 점수 ─────────────────────────────────
        // 완전수 n에 대한 gamma+1/(2·sigma(n)) 오차 비교
        // n=6 최적이면 uniqueness_ratio > 10
        let uniqueness_score = (uniqueness_ratio / 40.0).min(1.0); // 37.6 → ≈ 0.94

        // ── 6. 삼위일체 통합 점수 ────────────────────────────────
        // 낮은 오차, 높은 유일성, 잔차=1/tau 구조를 통합
        let trinity_score =
            (1.0 - trinity_error * 100.0).max(0.0) * 0.30   // BT-1114 등식 정밀도
            + (1.0 - bridge_error * 1000.0).max(0.0) * 0.25 // BT-1113 조화 브릿지
            + uniqueness_score * 0.25                         // 완전수 유일 최적
            + (1.0 - residual_vs_inv_tau * 1000.0).max(0.0) * 0.10 // 잔차=1/tau 구조
            + resonance_score * 0.10;                         // 데이터 공명

        // ── 7. sigma-tau-phi(n=6 관계망) 내부 검증 ───────────────
        // sigma(6) / tau(6) = 12 / 4 = 3 = n/phi(6) = 6/2
        let st_ratio = SIGMA_6 / TAU_6;                      // = 3.0 (정확)
        let n_div_euler_phi = N6 / 2.0;                      // phi(6)=2, = 3.0 (정확)
        let internal_consistency = (st_ratio - n_div_euler_phi).abs(); // = 0.0 → 완전

        // ── 8. 완전수 조건 내부 스캔 ─────────────────────────────
        // 데이터 범위에서 sigma(m)=2m 조건 만족하는 m 탐색 (1..=100)
        let perfect_in_range: Vec<f64> = (1usize..=100)
            .filter(|&m| sigma(m) == 2 * m)
            .map(|m| m as f64)
            .collect();

        let mut r = HashMap::new();
        r.insert("trinity_lhs".to_string(),         vec![trinity_lhs]);
        r.insert("trinity_error".to_string(),        vec![trinity_error]);
        r.insert("uniqueness_ratio".to_string(),     vec![uniqueness_ratio]);
        r.insert("uniqueness_score".to_string(),     vec![uniqueness_score]);
        r.insert("harmonic_bridge_h12".to_string(),  vec![harmonic_bridge]);
        r.insert("bridge_error".to_string(),          vec![bridge_error]);
        r.insert("n_star".to_string(),               vec![n_star]);
        r.insert("residual_vs_inv_tau".to_string(),  vec![residual_vs_inv_tau]);
        r.insert("resonance_score".to_string(),      vec![resonance_score]);
        r.insert("perfect_hits".to_string(),         vec![perfect_hits as f64]);
        r.insert("sigma_tau_consistency".to_string(), vec![internal_consistency]);
        r.insert("trinity_score".to_string(),        vec![trinity_score]);
        r.insert("perfect_numbers_1_100".to_string(), perfect_in_range);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt1114_trinity_핵심등식() {
        // gamma + 1/(2·sigma(6)) ≈ 1/phi — 오차 < 1e-3 확인
        let lhs = GAMMA + 1.0 / (2.0 * SIGMA_6);
        let err = (lhs - INV_PHI).abs();
        assert!(err < 1e-3, "BT-1114 삼위일체 오차 {:.2e} < 1e-3 실패", err);
    }

    #[test]
    fn test_bt1113_조화_브릿지() {
        // H(12) - ln(12) ≈ 1/phi — 오차 < 1e-3 확인
        let h12 = harmonic(12);
        let bridge = h12 - SIGMA_6.ln();
        let err = (bridge - INV_PHI).abs();
        assert!(err < 1e-3, "BT-1113 조화 브릿지 오차 {:.2e} < 1e-3 실패", err);
    }

    #[test]
    fn test_n6_유일성_완전수비교() {
        // n=6 오차가 n=28 오차보다 10배 이상 작아야 함
        let err6  = (GAMMA + 1.0 / (2.0 * 12.0) - INV_PHI).abs();  // sigma(6)=12
        let err28 = (GAMMA + 1.0 / (2.0 * 56.0) - INV_PHI).abs();  // sigma(28)=56
        let ratio = err28 / err6.max(1e-15);
        assert!(ratio > 10.0, "n=6 유일성 비율 {:.1} > 10 실패", ratio);
    }

    #[test]
    fn test_sigma_함수() {
        // sigma(6) = 1+2+3+6 = 12
        assert_eq!(sigma(6), 12);
        // sigma(28) = 1+2+4+7+14+28 = 56
        assert_eq!(sigma(28), 56);
        // sigma(1) = 1, sigma(2) = 3, sigma(3) = 4
        assert_eq!(sigma(1), 1);
        assert_eq!(sigma(2), 3);
    }

    #[test]
    fn test_harmonic_함수() {
        // H(1)=1, H(2)=1.5, H(4)≈2.083
        assert!((harmonic(1) - 1.0).abs() < 1e-12);
        assert!((harmonic(2) - 1.5).abs() < 1e-12);
        assert!((harmonic(4) - (1.0 + 0.5 + 1.0/3.0 + 0.25)).abs() < 1e-12);
    }

    #[test]
    fn test_잔차_1_over_tau() {
        // n* - sigma(6) ≈ 1/tau(6) = 0.25 — 오차 < 1e-3
        let n_star = 1.0 / (2.0 * (INV_PHI - GAMMA));
        let residual = n_star - SIGMA_6;
        let err = (residual - 1.0 / TAU_6).abs();
        assert!(err < 1e-3, "잔차-1/tau 오차 {:.2e} < 1e-3 실패", err);
    }

    #[test]
    fn test_스캔_기본() {
        // 정상 데이터에서 렌즈 출력 확인
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => 6.0,    // n=6
                1 => 12.0,   // sigma(6)
                2 => 0.618,  // 1/phi
                3 => 0.577,  // gamma
                4 => 28.0,   // 2위 완전수
                _ => 4.0,    // tau(6)
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = EulerGoldenPerfectLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("trinity_score"), "trinity_score 키 존재");
        assert!(result.contains_key("uniqueness_ratio"), "uniqueness_ratio 키 존재");
        assert!(result.contains_key("harmonic_bridge_h12"), "harmonic_bridge_h12 키 존재");
        let score = result["trinity_score"][0];
        assert!(score > 0.0, "trinity_score > 0: {}", score);
    }

    #[test]
    fn test_스캔_최소입력_거부() {
        // n < 6이면 빈 결과
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = EulerGoldenPerfectLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_완전수_1_100() {
        // 1..=100 범위 완전수: 6, 28
        let n = 6; let d = 1;
        let data: Vec<f64> = (0..n).map(|i| (i + 1) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = EulerGoldenPerfectLens.scan(&data, n, d, &shared);
        let pn = &result["perfect_numbers_1_100"];
        // 6과 28이 포함되어야 함
        assert!(pn.contains(&6.0), "완전수 목록에 6 포함: {:?}", pn);
        assert!(pn.contains(&28.0), "완전수 목록에 28 포함: {:?}", pn);
        // 496은 100 초과이므로 제외
        assert!(!pn.contains(&496.0), "496은 100 초과 — 제외 확인");
    }

    #[test]
    fn test_sigma_tau_내부일관성() {
        // sigma(6)/tau(6) = 12/4 = 3 = n=6/phi(6)=2 — 완전 일관성
        let st = SIGMA_6 / TAU_6;
        let nf = N6 / 2.0;
        assert!((st - nf).abs() < 1e-12, "sigma/tau = n/phi(6) 내부 일관성");
    }
}
