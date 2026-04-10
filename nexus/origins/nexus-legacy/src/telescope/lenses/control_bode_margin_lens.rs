use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 제어 보드 여유 렌즈 — 보드 선도 안정 여유 n=6 수렴
///
/// 보드 선도(Bode plot)에서 이득 여유와 위상 여유 분석.
/// n=6 연결:
///   이득 여유 6 dB (n=6)
///   위상 여유 60° = 360°/n
///   -20 dB/decade 슬로프 = -tau·sopfr = -20
///   이득 교차주파수 ωgc: 오차 1/n 이내
///   n차 시스템: 안정 극 n개
pub struct ControlBodeMarginLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const DB6: f64 = 6.0;     // 6 dB 이득 여유
const DEG60: f64 = 60.0;  // 60° 위상 여유
const SLOPE_20: f64 = 20.0; // -20 dB/decade

impl Lens for ControlBodeMarginLens {
    fn name(&self) -> &str { "ControlBodeMarginLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 보드 이득 여유 6 dB 공명
        let gm_score = means.iter().filter(|&&m| {
            [DB6, N6, SIGMA].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 2. 위상 여유 60° 공명
        let pm_score = means.iter().filter(|&&m| {
            [DEG60, N6 * TAU + SIGMA, SLOPE_20 * TAU].iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count() as f64 / d.max(1) as f64;

        // 3. -20 dB/decade 슬로프 공명
        let slope_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - SLOPE_20) / SLOPE_20).abs() < 0.07
        }).count() as f64 / d.max(1) as f64;

        // 4. 단조 감쇠 검사 — 보드 이득은 단조 감소해야 안정
        let monotone_score: f64 = {
            let mut mono = 0.0;
            for j in 0..d {
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let decreasing = col.windows(2).filter(|w| w[1] <= w[0]).count();
                mono += decreasing as f64 / (n - 1) as f64;
            }
            mono / d as f64
        };

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 전체 n=6 상수 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, DB6, DEG60];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let bode_score = gm_score      * 0.25
            + pm_score       * 0.25
            + n6_dim         * 0.20
            + monotone_score * 0.15
            + n6_resonance   * 0.10
            + slope_score    * 0.05;

        let mut r = HashMap::new();
        r.insert("gm_score".to_string(),       vec![gm_score]);
        r.insert("pm_score".to_string(),       vec![pm_score]);
        r.insert("slope_score".to_string(),    vec![slope_score]);
        r.insert("monotone_score".to_string(), vec![monotone_score]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("n6_resonance".to_string(),   vec![n6_resonance]);
        r.insert("bode_score".to_string(),     vec![bode_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bode_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => DB6, 2 => DEG60, 3 => TAU, 4 => PHI, _ => SIGMA }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = ControlBodeMarginLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("bode_score"));
        assert!(r["bode_score"][0] >= 0.0 && r["bode_score"][0] <= 1.0);
    }

    #[test]
    fn test_bode_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = ControlBodeMarginLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
