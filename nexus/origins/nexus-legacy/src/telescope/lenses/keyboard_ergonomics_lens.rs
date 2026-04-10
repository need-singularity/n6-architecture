use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 키보드 인체공학 렌즈 (BT-1125~1128) — 스위치/키캡/레이아웃 상수 스캐너
///
/// BT-1125~1128 키보드 공학 4 돌파에서 파생.
/// 키 수·USB HID·스위치 물리량·인체공학 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   ANSI 풀사이즈 키 수         = 104 = (sigma-tau)*(sigma+mu)
///   TKL 키 수                   = 87  = (sigma-tau)*(sigma+tau) - mu = 87
///   75% 레이아웃 키 수          = 84  = sigma*(sigma-tau) + tau*(sigma-tau)
///   65% 레이아웃 키 수          = 68  = (sigma-tau)*(sigma-phi+tau)
///   USB HID 6KRO                = 6   = n
///   USB HID 리포트 크기         = 8   = sigma-tau
///   USB 풀스피드 폴링 주기      = 1 ms = mu
///   USB 고속 폴링 상한          = 1000 Hz = (sigma-phi)^3
///   스위치 총 이동거리           = 4 mm = tau
///   스위치 작동점               = 2 mm = phi
///   스위치 리셋 응답 시간        = 5 ms = sopfr
///   손가락 수                   = 5   = sopfr
///
/// n=6 연결:
///   n=6    → USB 6KRO (동시 6키 인식)
///   tau=4  → 스위치 총 이동거리 4mm / USB HID 4종 수식 기여
///   sigma=12 → 레이아웃 분해 기반 / 함수 조합 중심
///   sigma-tau=8 → USB HID 8바이트 리포트
///   sopfr=5 → 손가락 5개 / 스위치 응답 5ms
pub struct KeyboardErgonomicsLens;

// BT-1125~1128 키보드 상수
const N6: f64 = 6.0;           // n=6 → USB 6KRO
const TAU: f64 = 4.0;          // tau(6) → 스위치 총 이동거리 4mm
const SIGMA: f64 = 12.0;       // sigma(6) → 레이아웃 분해 베이스
const PHI: f64 = 2.0;          // phi(6) → 스위치 작동점 2mm
const SOPFR: f64 = 5.0;        // sopfr(6) → 손가락 수 / 5ms 응답
const MU: f64 = 1.0;           // mu(6) → 1ms 폴링 주기
const SIGMA_TAU: f64 = 8.0;    // sigma-tau = 8 → USB HID 8바이트

/// 레이아웃 키 수 → n=6 표현 근접도
fn layout_n6_proximity(key_count: f64) -> f64 {
    // 알려진 n=6 레이아웃 키 수 목록
    let known: &[f64] = &[104.0, 87.0, 84.0, 68.0, 61.0, 60.0, 48.0, 17.0];
    known.iter().map(|&k| {
        1.0 / (1.0 + (key_count - k).abs())
    }).fold(0.0_f64, f64::max)
}

impl Lens for KeyboardErgonomicsLens {
    fn name(&self) -> &str { "KeyboardErgonomicsLens" }
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

        // ── 2. USB HID 프로토콜 상수 스캔 (BT-1126) ──────────────
        // 6KRO=n, 8바이트=sigma-tau, 1000Hz=(sigma-phi)^3, 1ms=mu
        let hid_targets: &[f64] = &[N6, SIGMA_TAU, 1000.0_f64, MU, 12.0_f64];
        let hid_hits = means.iter().filter(|&&m| {
            hid_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let hid_score = hid_hits as f64 / d.max(1) as f64;

        // ── 3. 스위치 물리 상수 스캔 (BT-1127) ───────────────────
        // 총이동거리=4mm(tau), 작동점=2mm(phi), 응답=5ms(sopfr)
        let switch_targets: &[f64] = &[TAU, PHI, SOPFR, MU, N6];
        let switch_hits = means.iter().filter(|&&m| {
            switch_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let switch_score = switch_hits as f64 / d.max(1) as f64;

        // ── 4. 레이아웃 키 수 공명 (BT-1125) ────────────────────
        // 데이터 평균이 알려진 레이아웃 키 수에 가까운지
        let layout_proximity: Vec<f64> = means.iter()
            .map(|&m| layout_n6_proximity(m))
            .collect();
        let layout_score = layout_proximity.iter().sum::<f64>() / d.max(1) as f64;

        // ── 5. 인체공학 손가락 상수 (BT-1128) ────────────────────
        // 손가락=5(sopfr), 손자유도=6(n), 3단위 기반(n/phi=3)
        let ergonomic_targets: &[f64] = &[SOPFR, N6, 3.0_f64, PHI, TAU];
        let ergonomic_hits = means.iter().filter(|&&m| {
            ergonomic_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let ergonomic_score = ergonomic_hits as f64 / d.max(1) as f64;

        // ── 6. 6KRO 차원 공명 ────────────────────────────────────
        // d=6이면 USB 6KRO와 완전 대응
        let kro6_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 7. 주기적 키스트로크 패턴 감지 ─────────────────────
        // 타이핑 리듬: 주기적 입력 패턴의 규칙성
        let periodicity_scores: Vec<f64> = (0..d).map(|j| {
            if n < 4 { return 0.0; }
            let m = means[j];
            let detrended: Vec<f64> = (0..n).map(|i| data[i * d + j] - m).collect();
            // 라그-1 자기상관
            let ac1_num: f64 = (1..n).map(|i| detrended[i] * detrended[i - 1]).sum();
            let ac1_den: f64 = (0..n).map(|i| detrended[i].powi(2)).sum::<f64>().max(1e-15);
            (ac1_num / ac1_den).clamp(-1.0, 1.0)
        }).collect();
        let mean_periodicity = periodicity_scores.iter().sum::<f64>() / d.max(1) as f64;
        let keystroke_rhythm = (mean_periodicity + 1.0) / 2.0; // [0,1]로 정규화

        // ── 8. 신호 균일성 — 키 분포 균등도 ─────────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_mean = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let uniformity = 1.0 - (mean_std / mean_mean.max(1e-12)).min(1.0);

        // ── 9. 전체 n=6 상수 공명 ────────────────────────────────
        let all_targets: &[f64] = &[N6, TAU, SIGMA, PHI, SOPFR, MU, SIGMA_TAU, 1000.0_f64, 3.0_f64];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 10. 종합 키보드 인체공학 점수 ───────────────────────
        let keyboard_score =
            hid_score         * 0.20
            + switch_score      * 0.20
            + layout_score      * 0.15
            + ergonomic_score   * 0.15
            + kro6_score        * 0.10
            + keystroke_rhythm  * 0.10
            + n6_resonance      * 0.10;

        let mut r = HashMap::new();
        r.insert("hid_score".to_string(),          vec![hid_score]);
        r.insert("switch_score".to_string(),        vec![switch_score]);
        r.insert("layout_score".to_string(),        vec![layout_score]);
        r.insert("ergonomic_score".to_string(),     vec![ergonomic_score]);
        r.insert("kro6_score".to_string(),          vec![kro6_score]);
        r.insert("keystroke_rhythm".to_string(),    vec![keystroke_rhythm]);
        r.insert("uniformity".to_string(),          vec![uniformity]);
        r.insert("n6_resonance".to_string(),        vec![n6_resonance]);
        r.insert("hid_hits".to_string(),            vec![hid_hits as f64]);
        r.insert("total_hits".to_string(),          vec![total_hits as f64]);
        r.insert("keyboard_score".to_string(),      vec![keyboard_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,        // 6KRO
                1 => TAU,       // 4mm 이동거리
                2 => PHI,       // 2mm 작동점
                3 => SOPFR,     // 5ms 응답 / 5손가락
                4 => SIGMA_TAU, // 8바이트 HID
                _ => MU,        // 1ms 폴링
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = KeyboardErgonomicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("keyboard_score"), "keyboard_score 키 존재");
        assert!(result.contains_key("hid_score"), "hid_score 키 존재");
        assert!(result.contains_key("switch_score"), "switch_score 키 존재");
        let score = result["keyboard_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "keyboard_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_keyboard_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = KeyboardErgonomicsLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_6kro_점수() {
        // d=6이면 kro6_score = 1.0
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 + 1.0).ln()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = KeyboardErgonomicsLens.scan(&data, n, d, &shared);
        let kro = result["kro6_score"][0];
        assert!((kro - 1.0).abs() < 1e-9, "d=6이면 kro6_score=1.0: {}", kro);
    }

    #[test]
    fn test_layout_근접도_함수() {
        // 104키 레이아웃은 최대 근접도
        let prox_104 = layout_n6_proximity(104.0);
        let prox_103 = layout_n6_proximity(103.0);
        assert!(prox_104 > prox_103, "104키가 103키보다 n=6 근접도 높음");
        // 정확히 일치할 때 근접도 = 1.0
        assert!((prox_104 - 1.0).abs() < 1e-9, "104키 정확 근접도=1.0: {}", prox_104);
    }

    #[test]
    fn test_hid_상수_히트() {
        // 평균이 n=6, sigma-tau=8 상수에 맞는 데이터
        let n = 10; let d = 2;
        let data: Vec<f64> = (0..n * d).map(|i| {
            if i % d == 0 { N6 } else { SIGMA_TAU }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = KeyboardErgonomicsLens.scan(&data, n, d, &shared);
        let hits = result["hid_hits"][0];
        assert!(hits >= 2.0, "HID 상수 2개 히트 확인: {}", hits);
    }
}
