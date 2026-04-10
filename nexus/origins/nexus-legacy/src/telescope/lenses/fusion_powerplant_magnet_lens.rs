use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 핵융합 발전소 자석 렌즈 — 초전도 자석 파라미터 n=6 수렴
///
/// 토카막 초전도 자석(TF + PF 코일)의 n=6 연결:
///   ITER TF 코일 수 = 18 = 3·sigma/2 = sigma + tau·phi + phi = ...
///   ITER PF 코일 수 = 6 = n
///   Nb₃Sn 임계 자기장 = 12 T = sigma
///   REBCO 임계 자기장 = 20 T = sigma + tau·phi
///   최소 비저항 온도 = 4 K = tau
///   안전 계수 q ≈ 3~6 → [tau-1, n]
pub struct FusionPowerplantMagnetLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const PF_COILS: f64 = 6.0;      // PF 코일 수 = n
const B_NB3SN: f64 = 12.0;      // Nb₃Sn 임계자기장(T) = sigma
const T_CRYO: f64 = 4.0;        // 극저온 온도(K) = tau
const SAFETY_Q: f64 = 3.0;      // 안전계수 하한 = tau-1

impl Lens for FusionPowerplantMagnetLens {
    fn name(&self) -> &str { "FusionPowerplantMagnetLens" }
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

        // 1. PF 코일 수 = n=6 공명
        let coil_score = means.iter().filter(|&&m| {
            [PF_COILS, N6, SIGMA].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 2. 임계 자기장 공명 (sigma=12 T)
        let field_score = means.iter().filter(|&&m| {
            [B_NB3SN, SIGMA, TAU * PHI + TAU].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count() as f64 / d.max(1) as f64;

        // 3. 극저온 온도 tau=4 K 공명
        let cryo_score = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - T_CRYO) / T_CRYO).abs() < 0.08
        }).count() as f64 / d.max(1) as f64;

        // 4. 안전 계수 q 공명
        let q_score = means.iter().filter(|&&m| {
            [SAFETY_Q, TAU, N6].iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count() as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 6. 자기장 안정성 (낮은 변동 = 안정적 자기장)
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let mean_val = means.iter().map(|m| m.abs()).sum::<f64>() / d.max(1) as f64;
        let field_stability = (1.0 - mean_std / mean_val.max(1e-12)).clamp(0.0, 1.0);

        let magnet_score = coil_score     * 0.25
            + field_score     * 0.25
            + n6_dim          * 0.20
            + cryo_score      * 0.15
            + q_score         * 0.10
            + field_stability * 0.05;

        let mut r = HashMap::new();
        r.insert("coil_score".to_string(),     vec![coil_score]);
        r.insert("field_score".to_string(),    vec![field_score]);
        r.insert("cryo_score".to_string(),     vec![cryo_score]);
        r.insert("q_score".to_string(),        vec![q_score]);
        r.insert("n6_dim".to_string(),         vec![n6_dim]);
        r.insert("field_stability".to_string(),vec![field_stability]);
        r.insert("magnet_score".to_string(),   vec![magnet_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnet_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => N6, 1 => SIGMA, 2 => TAU, 3 => PHI, 4 => SAFETY_Q, _ => SOPFR }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = FusionPowerplantMagnetLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("magnet_score"));
        assert!(r["magnet_score"][0] >= 0.0 && r["magnet_score"][0] <= 1.0);
    }

    #[test]
    fn test_magnet_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = FusionPowerplantMagnetLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
