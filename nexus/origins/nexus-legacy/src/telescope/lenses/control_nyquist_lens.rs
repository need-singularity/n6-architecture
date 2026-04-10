use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 제어 나이퀴스트 렌즈 — 안정도 판별 n=6 수렴 스캐너
///
/// 나이퀴스트 안정도 판별(Nyquist criterion): 개루프 전달함수가
/// (-1, 0) 포위 여부로 폐루프 안정성 판별.
/// n=6 연결:
///   안정 여유(gain margin) 기준 = 6 dB = n
///   위상 여유(phase margin) 기준 = 60° = 360°/n
///   개루프 이득 교차 주파수 비율 = tau/phi = 4/2 = 2
///   나이퀴스트 표본화 정리: fs = 2·fc → phi=2 인자
pub struct ControlNyquistLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const GAIN_MARGIN_DB: f64 = 6.0;   // 6 dB 이득 여유
const PHASE_MARGIN_DEG: f64 = 60.0; // 60° 위상 여유

impl Lens for ControlNyquistLens {
    fn name(&self) -> &str { "ControlNyquistLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();

        // 1. 이득 여유 6 dB 공명
        let gm_targets = [GAIN_MARGIN_DB, N6, SIGMA, TAU];
        let gm_hits = means.iter().filter(|&&m| {
            gm_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let gain_margin_score = gm_hits as f64 / d.max(1) as f64;

        // 2. 위상 여유 60° 공명
        let pm_targets = [PHASE_MARGIN_DEG, N6 * TAU, SIGMA * SOPFR];
        let pm_hits = means.iter().filter(|&&m| {
            pm_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let phase_margin_score = pm_hits as f64 / d.max(1) as f64;

        // 3. 표본화 정리 — 2배 인자 (phi=2) 공명
        let nyquist_factor_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - PHI) / PHI).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 4. 안정도 지표 — 과도 응답 감쇠비 공명 (ζ ≈ 0.707 → phi/sqrt(2·phi))
        let damping_ratio = PHI.sqrt() / 2.0; // ≈ 0.707
        let damping_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - damping_ratio) / damping_ratio).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 신호 안정성 (낮은 변동계수 = 안정적 제어)
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let stability = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        let nyquist_score = gain_margin_score   * 0.25
            + phase_margin_score  * 0.25
            + n6_dim              * 0.20
            + stability           * 0.15
            + nyquist_factor_score * 0.10
            + damping_score       * 0.05;

        let mut r = HashMap::new();
        r.insert("gain_margin_score".to_string(),    vec![gain_margin_score]);
        r.insert("phase_margin_score".to_string(),   vec![phase_margin_score]);
        r.insert("nyquist_factor_score".to_string(), vec![nyquist_factor_score]);
        r.insert("damping_score".to_string(),        vec![damping_score]);
        r.insert("n6_dim".to_string(),               vec![n6_dim]);
        r.insert("stability".to_string(),            vec![stability]);
        r.insert("nyquist_score".to_string(),        vec![nyquist_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_nyquist_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => GAIN_MARGIN_DB, 2 => PHASE_MARGIN_DEG, 3 => PHI, 4 => TAU, _ => SIGMA }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = ControlNyquistLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("nyquist_score"));
        assert!(r["nyquist_score"][0] >= 0.0 && r["nyquist_score"][0] <= 1.0);
    }

    #[test]
    fn test_nyquist_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = ControlNyquistLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
