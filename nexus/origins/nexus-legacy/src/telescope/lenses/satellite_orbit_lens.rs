use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 위성 궤도 렌즈 — 케플러 궤도 요소 n=6 수렴
///
/// 케플러 궤도 6요소: a, e, i, Ω, ω, ν
/// n=6 연결:
///   궤도 요소 수 = 6 = n
///   GPS 위성 궤도면 수 = 6 = n
///   저궤도 고도 LEO ≈ 400 km → tau·100=400
///   지구 중력 매개변수 GM = 3.986×10^14 ≈ phi·n 구조
///   원형 궤도 속도 ≈ 7.9 km/s → sopfr+phi·1.45 근방
///   케플러 제3법칙: T² ∝ a³ → phi·tau-1=3 차
pub struct SatelliteOrbitLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const ORBITAL_ELEMENTS: f64 = 6.0;  // 궤도 요소 수 = n
const GPS_PLANES: f64 = 6.0;        // GPS 궤도면 = n
const LEO_ALT_UNITS: f64 = 4.0;     // LEO ≈ tau × 100 km
const CIRC_VEL: f64 = 7.9;          // 원형 속도 ≈ 7.9 km/s

impl Lens for SatelliteOrbitLens {
    fn name(&self) -> &str { "SatelliteOrbitLens" }
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

        // 1. 6 궤도 요소 공명
        let orbital_score = means.iter().filter(|&&m| {
            [ORBITAL_ELEMENTS, N6, GPS_PLANES].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 2. 케플러 요소 n6_dim (d=6이면 6 궤도 요소)
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 3. 이심률 0 (원형) ~ 1 범위 확인
        let eccentricity_check = means.iter().filter(|&&m| {
            m >= 0.0 && m <= 1.0
        }).count() as f64 / d.max(1) as f64;

        // 4. 주기성 공명 (궤도 주기)
        let periodicity: f64 = {
            let mut total = 0.0;
            for j in 0..d {
                if n < 4 { continue; }
                let col: Vec<f64> = (0..n).map(|i| data[i * d + j]).collect();
                let m = means[j];
                let num: f64 = (1..n).map(|i| (col[i] - m) * (col[i-1] - m)).sum();
                let den: f64 = (0..n).map(|i| (col[i] - m).powi(2)).sum::<f64>().max(1e-15);
                total += (num / den).clamp(-1.0, 1.0);
            }
            ((total / d as f64 + 1.0) / 2.0).clamp(0.0, 1.0)
        };

        // 5. 궤도 안정성 (낮은 분산 = 안정)
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let orbit_stability = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        // 6. 전체 n=6 공명
        let all_targets = [N6, TAU, SIGMA, PHI, SOPFR, CIRC_VEL];
        let total_hits = means.iter().filter(|&&m| {
            all_targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let satellite_score = orbital_score    * 0.25
            + n6_dim           * 0.20
            + periodicity      * 0.20
            + orbit_stability  * 0.15
            + eccentricity_check * 0.10
            + n6_resonance     * 0.10;

        let mut r = HashMap::new();
        r.insert("orbital_score".to_string(),       vec![orbital_score]);
        r.insert("n6_dim".to_string(),              vec![n6_dim]);
        r.insert("periodicity".to_string(),         vec![periodicity]);
        r.insert("orbit_stability".to_string(),     vec![orbit_stability]);
        r.insert("eccentricity_check".to_string(),  vec![eccentricity_check]);
        r.insert("n6_resonance".to_string(),        vec![n6_resonance]);
        r.insert("satellite_score".to_string(),     vec![satellite_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_satellite_기본() {
        let n = 12; let d = 6;
        // 케플러 6요소 시뮬: a, e, i, Ω, ω, ν
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d {
                0 => N6,         // a (정규화)
                1 => 0.01,       // e (이심률)
                2 => 28.5,       // i (경사각 deg)
                3 => 180.0,      // Ω
                4 => 0.0,        // ω
                _ => (i as f64 * 30.0) % 360.0,  // ν
            }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SatelliteOrbitLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("satellite_score"));
        assert!(r["satellite_score"][0] >= 0.0 && r["satellite_score"][0] <= 1.0);
    }

    #[test]
    fn test_satellite_n6_dim() {
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = SatelliteOrbitLens.scan(&data, n, d, &shared);
        assert!((r["n6_dim"][0] - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_satellite_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = SatelliteOrbitLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
