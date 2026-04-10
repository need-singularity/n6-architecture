use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 오디오/스피커 렌즈 (BT-1136~1141) — 음향 재생 상수 스캐너
///
/// BT-1136~1141 오디오·스피커 6 연속 돌파에서 파생.
/// 드라이버 구조·크로스오버·위상·임피던스·재생 대역의 핵심 파라미터가
/// n=6 산술함수로 수렴함을 검증한다.
///
/// 스캔 대상 상수 (12종):
///   표준 스피커 드라이버 종류   = 4   = tau  (우퍼/미드/트위터/슈퍼트위터)
///   포인트소스 방사 차수        = 6   = n
///   크로스오버 필터 차수        = 4~6 → [tau, n]
///   임피던스 공칭값 (Ω)        = 4,8,12 → [tau, tau*phi, sigma]
///   감도 기준 레퍼런스 (dB)    = 1   = mu   (1W/1m 기준)
///   최소 재생 주파수 (Hz) 배율 = 12  = sigma (베이스 한계 기준)
///   크로스오버 주파수 개수      = 2   = phi  (2-웨이 기준)
///   THD 측정 고조파 차수        = 6   = n    (1~6차 고조파)
///   위상 응답 분할 구간 수      = 4   = tau
///   스테레오 채널 수            = 2   = phi
///   서라운드 채널 배치 기준     = 6   = n    (5.1에서 .1 포함 6채널)
///   바스 반사 포트 공명 배율    = 12  = sigma (포트 주파수 비율)
///
/// n=6 연결:
///   n=6    → 포인트소스 방사 차수 / THD 고조파 / 5.1 서라운드 6채널
///   tau=4  → 드라이버 종류 / 크로스오버 필터 하한 / 위상 구간 수
///   sigma=12 → 임피던스 상한 / 재생 주파수 배율 / 포트 공명 배율
///   phi=2  → 크로스오버 주파수 개수 / 스테레오 채널 수
///   mu=1   → 감도 기준 1W/1m
pub struct AudioSpeakerLens;

// BT-1136~1141 핵심 상수
const N6: f64 = 6.0;        // n=6 (완전수)
const TAU: f64 = 4.0;       // tau(6) = 약수 개수
const SIGMA: f64 = 12.0;    // sigma(6) = 약수합
const PHI: f64 = 2.0;       // phi(6) = 오일러 토티언트
const SOPFR: f64 = 5.0;     // sopfr(6) = 최대 소인수합
const MU: f64 = 1.0;        // mu(6) = 뫼비우스 함수 절댓값
const IMP8: f64 = 8.0;      // 8Ω 임피던스 = sigma - tau

/// 크로스오버 차수 → n=6 근접도
fn crossover_n6_proximity(order: f64) -> f64 {
    // 표준 크로스오버 차수: 1차(6dB), 2차(12dB), 4차(24dB), 6차(36dB)
    let known: &[f64] = &[MU, PHI, TAU, N6, IMP8, SIGMA];
    known.iter().map(|&k| {
        1.0 / (1.0 + (order - k).abs())
    }).fold(0.0_f64, f64::max)
}

impl Lens for AudioSpeakerLens {
    fn name(&self) -> &str { "AudioSpeakerLens" }
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

        // ── 2. 드라이버 구조 상수 스캔 (BT-1136) ─────────────────
        // 드라이버 종류=tau, 포인트소스 방사=n, 스테레오=phi
        let driver_targets: &[f64] = &[TAU, N6, PHI, SOPFR, MU];
        let driver_hits = means.iter().filter(|&&m| {
            driver_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let driver_score = driver_hits as f64 / d.max(1) as f64;

        // ── 3. 임피던스·감도 상수 스캔 (BT-1137) ─────────────────
        // 4Ω=tau, 8Ω=sigma-tau, 12Ω=sigma, 기준=mu
        let impedance_targets: &[f64] = &[TAU, IMP8, SIGMA, MU, PHI];
        let impedance_hits = means.iter().filter(|&&m| {
            impedance_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let impedance_score = impedance_hits as f64 / d.max(1) as f64;

        // ── 4. 크로스오버 필터 근접도 (BT-1138) ──────────────────
        let crossover_proximity: Vec<f64> = means.iter()
            .map(|&m| crossover_n6_proximity(m))
            .collect();
        let crossover_score = crossover_proximity.iter().sum::<f64>() / d.max(1) as f64;

        // ── 5. THD 고조파 분포 스캔 (BT-1139) ────────────────────
        // 1~6차 고조파 = n=6, 2차/3차 지배 → phi/3.0
        let thd_targets: &[f64] = &[N6, PHI, 3.0_f64, TAU, SOPFR, MU];
        let thd_hits = means.iter().filter(|&&m| {
            thd_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let thd_score = thd_hits as f64 / d.max(1) as f64;

        // ── 6. 서라운드 채널 배치 스캔 (BT-1140~1141) ────────────
        // 5.1 = 6채널=n, 7.1 = sigma-tau, 스테레오=phi
        let surround_targets: &[f64] = &[N6, IMP8, PHI, SIGMA, TAU];
        let surround_hits = means.iter().filter(|&&m| {
            surround_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count();
        let surround_score = surround_hits as f64 / d.max(1) as f64;

        // ── 7. n=6 채널 차원 완전성 (5.1 = n 대응) ───────────────
        let n6_channel_score = {
            let diff = (d as f64 - N6).abs() / N6;
            (1.0 - diff * 2.0).max(0.0)
        };

        // ── 8. 음파 주기 패턴 감지 (위상 응답 분할) ─────────────
        // 위상 응답 tau=4 구간 → 라그-1 자기상관으로 주기성 추정
        let phase_periodicity: Vec<f64> = (0..d).map(|j| {
            if n < 4 { return 0.0; }
            let m = means[j];
            let detrended: Vec<f64> = (0..n).map(|i| data[i * d + j] - m).collect();
            let ac1_num: f64 = (1..n).map(|i| detrended[i] * detrended[i - 1]).sum();
            let ac1_den: f64 = (0..n).map(|i| detrended[i].powi(2)).sum::<f64>().max(1e-15);
            (ac1_num / ac1_den).clamp(-1.0, 1.0)
        }).collect();
        let mean_periodicity = phase_periodicity.iter().sum::<f64>() / d.max(1) as f64;
        // tau=4 구간 기준: 0.25 주기성 기대
        let wave_rhythm = (1.0 - (mean_periodicity - 1.0 / TAU).abs() * 4.0).max(0.0);

        // ── 9. 신호 균일성 — 주파수 응답 평탄도 ──────────────────
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_mean = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let flatness = 1.0 - (mean_std / mean_mean.max(1e-12)).min(1.0);

        // ── 10. 전체 n=6 상수 공명 스캔 ──────────────────────────
        let all_targets: &[f64] = &[N6, TAU, SIGMA, PHI, SOPFR, MU, IMP8, 3.0_f64];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        // ── 11. 종합 오디오/스피커 점수 ──────────────────────────
        let audio_score =
            driver_score      * 0.20
            + impedance_score   * 0.15
            + crossover_score   * 0.15
            + thd_score         * 0.15
            + surround_score    * 0.10
            + n6_channel_score  * 0.10
            + wave_rhythm       * 0.10
            + n6_resonance      * 0.05;

        let mut r = HashMap::new();
        r.insert("driver_score".to_string(),      vec![driver_score]);
        r.insert("impedance_score".to_string(),   vec![impedance_score]);
        r.insert("crossover_score".to_string(),   vec![crossover_score]);
        r.insert("thd_score".to_string(),         vec![thd_score]);
        r.insert("surround_score".to_string(),    vec![surround_score]);
        r.insert("n6_channel_score".to_string(),  vec![n6_channel_score]);
        r.insert("wave_rhythm".to_string(),       vec![wave_rhythm]);
        r.insert("flatness".to_string(),          vec![flatness]);
        r.insert("n6_resonance".to_string(),      vec![n6_resonance]);
        r.insert("driver_hits".to_string(),       vec![driver_hits as f64]);
        r.insert("total_hits".to_string(),        vec![total_hits as f64]);
        r.insert("audio_score".to_string(),       vec![audio_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_기본_출력() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,    // 포인트소스 방사 차수 = 6
                1 => TAU,   // 드라이버 종류 = 4
                2 => SIGMA, // 임피던스 상한 = 12
                3 => PHI,   // 스테레오 채널 = 2
                4 => IMP8,  // 8Ω 임피던스
                _ => MU,    // 1W/1m 기준
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = AudioSpeakerLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("audio_score"), "audio_score 키 존재");
        assert!(result.contains_key("driver_score"), "driver_score 키 존재");
        assert!(result.contains_key("n6_resonance"), "n6_resonance 키 존재");
        let score = result["audio_score"][0];
        assert!(score >= 0.0 && score <= 1.0, "audio_score 범위 [0,1]: {}", score);
    }

    #[test]
    fn test_audio_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = AudioSpeakerLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty(), "n<6 이면 빈 결과");
    }

    #[test]
    fn test_n6_채널_점수() {
        // d=6이면 n6_channel_score = 1.0
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i as f64 + 1.0).ln()).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = AudioSpeakerLens.scan(&data, n, d, &shared);
        let ch = result["n6_channel_score"][0];
        assert!((ch - 1.0).abs() < 1e-9, "d=6이면 n6_channel_score=1.0: {}", ch);
    }

    #[test]
    fn test_crossover_근접도_함수() {
        // 정확히 일치하는 차수는 근접도 = 1.0
        let prox_n6 = crossover_n6_proximity(N6);
        let prox_n6_near = crossover_n6_proximity(N6 + 0.5);
        assert!(prox_n6 > prox_n6_near, "6차 크로스오버가 6.5차보다 n=6 근접도 높음");
        assert!((prox_n6 - 1.0).abs() < 1e-9, "6차 정확 근접도=1.0: {}", prox_n6);
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
        let result = AudioSpeakerLens.scan(&data, n, d, &shared);
        let hits = result["total_hits"][0];
        assert!(hits >= 3.0, "n=6 상수 3개 히트 확인: {}", hits);
    }

    #[test]
    fn test_audio_전체_키_존재() {
        let n = 12; let d = 4;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 % 12.0 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = AudioSpeakerLens.scan(&data, n, d, &shared);
        let expected_keys = [
            "driver_score", "impedance_score", "crossover_score",
            "thd_score", "surround_score", "n6_channel_score",
            "audio_score", "n6_resonance",
        ];
        for key in &expected_keys {
            assert!(result.contains_key(*key), "키 존재 확인: {}", key);
        }
    }
}
