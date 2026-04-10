use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 호버크래프트 쿠션 압력 렌즈 — 공기부양 압력 n=6 수렴
///
/// 호버크래프트 쿠션(air cushion) 압력 설계의 n=6 연결:
///   쿠션 압력 P = W/(A·φ) — φ=phi=2 (유효 면적 인자)
///   스커트 세그먼트 수 = 6~12 → [n, sigma]
///   압력비 최적값 = phi = 2 (내/외부 압력비)
///   6자유도 운동: surge, sway, heave, roll, pitch, yaw = n
///   레이놀즈 수 기준: Re > 4·10^6 → tau 인자
pub struct HovercraftCushionPressureLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const PRESSURE_RATIO: f64 = 2.0;  // 최적 압력비 = phi
const SKIRT_MIN: f64 = 6.0;       // 최소 스커트 = n
const SKIRT_MAX: f64 = 12.0;      // 최대 스커트 = sigma

impl Lens for HovercraftCushionPressureLens {
    fn name(&self) -> &str { "HovercraftCushionPressureLens" }
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

        // 1. 압력비 phi=2 공명
        let pressure_score = means.iter().filter(|&&m| {
            [PRESSURE_RATIO, PHI, N6 / TAU + PHI / PHI].iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.08
            })
        }).count() as f64 / d.max(1) as f64;

        // 2. 스커트 세그먼트 수 공명 [6, 12]
        let skirt_score = means.iter().filter(|&&m| {
            [SKIRT_MIN, SKIRT_MAX, N6, SIGMA].iter().any(|&t| {
                t > 1e-12 && ((m - t) / t).abs() < 0.07
            })
        }).count() as f64 / d.max(1) as f64;

        // 3. 6 DoF 운동 공명 (d=6이면 최대)
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 4. 압력 안정성 (낮은 변동 = 안정적 부양)
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let cushion_stability = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        // 5. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, PRESSURE_RATIO];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let cushion_score = pressure_score    * 0.25
            + skirt_score       * 0.20
            + n6_dim            * 0.20
            + cushion_stability * 0.20
            + n6_resonance      * 0.15;

        let mut r = HashMap::new();
        r.insert("pressure_score".to_string(),     vec![pressure_score]);
        r.insert("skirt_score".to_string(),        vec![skirt_score]);
        r.insert("n6_dim".to_string(),             vec![n6_dim]);
        r.insert("cushion_stability".to_string(),  vec![cushion_stability]);
        r.insert("n6_resonance".to_string(),       vec![n6_resonance]);
        r.insert("cushion_score".to_string(),      vec![cushion_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cushion_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => PHI, 2 => SIGMA, 3 => TAU, 4 => SOPFR, _ => 2.0 }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = HovercraftCushionPressureLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("cushion_score"));
        assert!(r["cushion_score"][0] >= 0.0 && r["cushion_score"][0] <= 1.0);
    }

    #[test]
    fn test_cushion_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = HovercraftCushionPressureLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
