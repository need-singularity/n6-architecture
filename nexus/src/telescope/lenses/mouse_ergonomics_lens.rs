use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 마우스 인체공학 렌즈 (HEXA-MOUSE) — DPI/센서/그립/버튼 상수 스캐너
///
/// HEXA-MOUSE 25/25 EXACT 검증 결과에서 파생.
/// 마우스의 DPI·폴링레이트·버튼 수·센서·무게·그립이
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   기본 버튼 수         = 2    = phi(6)
///   표준 버튼 수         = 5    = sopfr(6)
///   추적 축 수           = 3    = n/phi(6)
///   그립 유형 수         = 3    = n/phi(6)
///   PS/2 커넥터 핀 수    = 6    = n
///   6DoF 공간 입력 수    = 6    = n
///   USB HID 리포트 바이트= 4    = tau(6)
///   PTFE 피트 수         = 4    = tau(6)
///   MMO 사이드 버튼 수   = 12   = sigma(6)
///   스크롤 노치/회전     = 12   = sigma(6)
///   울트라 폴링 레이트   = 8 kHz= sigma-tau
///   울트라라이트 무게    = 60 g = sigma*sopfr
///
/// n=6 연결:
///   phi=2   → 기본 2버튼 / 듀얼 무선 / LOD 2mm
///   sopfr=5 → 표준 5버튼 / 5손가락
///   n/phi=3 → 3축 추적 / 3그립 / 스위치 3단자
///   n=6     → PS/2 6핀 / 6DoF
///   tau=4   → HID 4바이트 / PTFE 4피트 / DPI 4프리셋
///   sigma=12 → MMO 12버튼 / 스크롤 12노치
///   sigma-tau=8 → 8kHz 울트라 폴링
///   sigma*sopfr=60 → 60g 울트라라이트 / 60M 수명
pub struct MouseErgonomicsLens;

// HEXA-MOUSE n=6 상수
const N6: f64 = 6.0;              // n=6 → PS/2 6핀 / 6DoF
const TAU: f64 = 4.0;             // tau(6) → HID 4바이트 / PTFE 4피트
const SIGMA: f64 = 12.0;          // sigma(6) → MMO 12버튼 / 12노치
const PHI: f64 = 2.0;             // phi(6) → 기본 2버튼 / LOD 2mm
const SOPFR: f64 = 5.0;           // sopfr(6) → 표준 5버튼 / 5손가락
const SIGMA_TAU: f64 = 8.0;       // sigma-tau → 8kHz 울트라 폴링
const SIGMA_SOPFR: f64 = 60.0;    // sigma*sopfr → 60g / 60M 수명
const J2: f64 = 24.0;             // J_2(6) → 24스텝 인코더 / USB-C 24핀
const SOPFR_N: f64 = 30.0;        // sopfr*n → 30×30 센서 어레이

impl Lens for MouseErgonomicsLens {
    fn name(&self) -> &str { "MouseErgonomicsLens" }
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

        // ── 2. 버튼/입력 상수 스캔 ────────────────────────────────
        // phi=2(기본버튼), sopfr=5(표준버튼), sigma=12(MMO), n=6(6DoF)
        let button_targets: &[f64] = &[PHI, SOPFR, SIGMA, N6, 3.0_f64];
        let button_hits = means.iter().filter(|&&m| {
            button_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let button_score = button_hits as f64 / d.max(1) as f64;

        // ── 3. 센서/DPI 상수 스캔 ────────────────────────────────
        // sopfr*n=30(30×30 센서), tau=4(DPI 프리셋), sigma_tau=8(8kHz 폴링)
        let sensor_targets: &[f64] = &[SOPFR_N, TAU, SIGMA_TAU, J2, 1000.0_f64];
        let sensor_hits = means.iter().filter(|&&m| {
            sensor_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let sensor_score = sensor_hits as f64 / d.max(1) as f64;

        // ── 4. 기구부 상수 스캔 ──────────────────────────────────
        // sigma*sopfr=60(g/M), tau=4(피트), phi=2(LOD), J2=24(인코더)
        let physical_targets: &[f64] = &[SIGMA_SOPFR, TAU, PHI, J2, SIGMA];
        let physical_hits = means.iter().filter(|&&m| {
            physical_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let physical_score = physical_hits as f64 / d.max(1) as f64;

        // ── 5. 그립 유형 공명 ─────────────────────────────────────
        // 3그립(팜/클로/핑거팁) = n/phi = 3
        // 데이터 차원이 3의 배수이면 공명
        let grip3_resonance = {
            let rem = d % 3;
            if rem == 0 { 1.0 } else { 1.0 - rem as f64 / 3.0 }
        };

        // ── 6. 6DoF 입력 공명 ────────────────────────────────────
        // SpaceMouse 6DoF = n=6. d=6이면 최고 점수
        let dof6_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 7. 폴링 계층 점수 ────────────────────────────────────
        // 표준: 125/250/500/1000 Hz = tau=4 단계 (USB 2.0 §5.7.4)
        // 평균이 1000 근방이면 울트라 폴링 데이터로 판정
        let polling_targets: &[f64] = &[125.0_f64, 250.0_f64, 500.0_f64, 1000.0_f64, SIGMA_TAU];
        let polling_hits = means.iter().filter(|&&m| {
            polling_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let polling_score = polling_hits as f64 / d.max(1) as f64;

        // ── 8. 이진 스위치 패턴 감지 ─────────────────────────────
        // 마우스 클릭 = phi=2 이진 상태 (0/1). 데이터가 이진 분포이면 고점수
        let binary_scores: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let positive_ratio = (0..n).filter(|&i| data[i * d + j] > m).count() as f64 / n as f64;
            // phi=2 이진: 0.5 근방이면 이진 분포에 가장 가까움
            1.0 - (positive_ratio - 0.5).abs() * 2.0
        }).collect();
        let binary_score = binary_scores.iter().sum::<f64>() / d.max(1) as f64;

        // ── 9. 전체 n=6 상수 공명 ────────────────────────────────
        let all_targets: &[f64] = &[
            N6, TAU, SIGMA, PHI, SOPFR, SIGMA_TAU, SIGMA_SOPFR, J2, SOPFR_N, 3.0_f64
        ];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 10. 신호 변동성 — 마우스 움직임 다이나믹스 ──────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_abs = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let dynamics = (mean_std / mean_abs.max(1e-12)).min(2.0) / 2.0; // 클수록 동적

        // ── 11. 종합 마우스 인체공학 점수 ────────────────────────
        let mouse_score =
            button_score    * 0.20
            + sensor_score    * 0.20
            + physical_score  * 0.15
            + dof6_score      * 0.10
            + grip3_resonance * 0.10
            + polling_score   * 0.10
            + binary_score    * 0.05
            + n6_resonance    * 0.10;

        let mut r = HashMap::new();
        r.insert("button_score".to_string(),      vec![button_score]);
        r.insert("sensor_score".to_string(),      vec![sensor_score]);
        r.insert("physical_score".to_string(),    vec![physical_score]);
        r.insert("dof6_score".to_string(),        vec![dof6_score]);
        r.insert("grip3_resonance".to_string(),   vec![grip3_resonance]);
        r.insert("polling_score".to_string(),     vec![polling_score]);
        r.insert("binary_score".to_string(),      vec![binary_score]);
        r.insert("n6_resonance".to_string(),      vec![n6_resonance]);
        r.insert("dynamics".to_string(),          vec![dynamics]);
        r.insert("button_hits".to_string(),       vec![button_hits as f64]);
        r.insert("total_hits".to_string(),        vec![total_hits as f64]);
        r.insert("mouse_score".to_string(),       vec![mouse_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mouse_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => PHI,         // 기본 2버튼
                1 => SOPFR,       // 표준 5버튼
                2 => TAU,         // PTFE 4피트 / HID 4바이트
                3 => SIGMA,       // MMO 12버튼
                4 => N6,          // PS/2 6핀
                _ => SIGMA_TAU,   // 8kHz 폴링
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MouseErgonomicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mouse_score"), "mouse_score 키 존재");
        assert!(result.contains_key("button_score"), "button_score 키 존재");
        assert!(result.contains_key("dof6_score"), "dof6_score 키 존재");
        let score = result["mouse_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "mouse_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_mouse_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = MouseErgonomicsLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_6dof_점수() {
        // d=6이면 dof6_score = 1.0
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 * 0.7).cos() * 5.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MouseErgonomicsLens.scan(&data, n, d, &shared);
        let dof = result["dof6_score"][0];
        assert!((dof - 1.0).abs() < 1e-9, "d=6이면 dof6_score=1.0: {}", dof);
    }

    #[test]
    fn test_그립3_공명() {
        // d=3 → grip3_resonance=1.0, d=6 → 1.0, d=9 → 1.0
        for &d in &[3usize, 6, 9] {
            let n = 8;
            let data: Vec<f64> = (0..n * d).map(|i| i as f64).collect();
            let shared = SharedData::compute(&data, n, d);
            let result = MouseErgonomicsLens.scan(&data, n, d, &shared);
            let grip = result["grip3_resonance"][0];
            assert!((grip - 1.0).abs() < 1e-9, "d={} → grip3_resonance=1.0: {}", d, grip);
        }
    }

    #[test]
    fn test_버튼_상수_히트() {
        // 평균이 phi=2, sopfr=5 값인 데이터 → button_hits >= 2
        let n = 10; let d = 2;
        let data: Vec<f64> = (0..n * d).map(|i| {
            if i % d == 0 { PHI } else { SOPFR }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MouseErgonomicsLens.scan(&data, n, d, &shared);
        let hits = result["button_hits"][0];
        assert!(hits >= 2.0, "버튼 상수 2개 히트 확인: {}", hits);
    }

    #[test]
    fn test_전체_키_존재() {
        let n = 12; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 % 6.0 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MouseErgonomicsLens.scan(&data, n, d, &shared);
        let expected_keys = [
            "button_score", "sensor_score", "physical_score",
            "dof6_score", "grip3_resonance", "n6_resonance", "mouse_score",
        ];
        for key in &expected_keys {
            assert!(result.contains_key(*key), "키 존재 확인: {}", key);
        }
    }
}
