use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 주사위 확률 렌즈 — n=6 면 주사위의 확률 분포 수렴 스캐너
///
/// 표준 주사위는 정확히 6면(n=6).
/// tau(6)=4, sigma(6)=12, phi(6)=2 가 주사위 수학에 나타남:
///   1/6  확률 = 1/n
///   합=7 (= tau+phi+1 = 4+2+1) 이 마주보는 면 쌍
///   최대합=12=sigma, 최소합=2=phi
///   균일분포 엔트로피 = ln(6) = ln(n)
pub struct DiceProbabilityLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;

impl Lens for DiceProbabilityLens {
    fn name(&self) -> &str { "DiceProbabilityLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        // 1. 1/6 확률 공명 (각 면의 등장 확률)
        let p6 = 1.0 / N6;
        let prob_hits = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - p6) / p6).abs() < 0.08
        }).count();
        let prob_score = prob_hits as f64 / d.max(1) as f64;

        // 2. 합=7 공명 (마주보는 면 합)
        let face_sum = N6 + 1.0; // 7
        let sum7_hits = means.iter().filter(|&&m| {
            m > 1e-12 && ((m - face_sum) / face_sum).abs() < 0.07
        }).count();
        let sum7_score = sum7_hits as f64 / d.max(1) as f64;

        // 3. 균일분포 점수 — 표준편차가 작을수록 균일
        let stds: Vec<f64> = (0..d).map(|j| {
            let m = means[j];
            let var = (0..n).map(|i| (data[i * d + j] - m).powi(2)).sum::<f64>() / n as f64;
            var.sqrt()
        }).collect();
        let mean_std = stds.iter().sum::<f64>() / d.max(1) as f64;
        let uniformity = (1.0 - mean_std / (N6.max(1.0))).clamp(0.0, 1.0);

        // 4. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        // 5. 전체 n=6 상수 히트
        let targets = [N6, TAU, SIGMA, PHI, SOPFR, p6, face_sum];
        let total_hits = means.iter().filter(|&&m| {
            targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.07)
        }).count();
        let n6_resonance = total_hits as f64 / d.max(1) as f64;

        let dice_score = prob_score * 0.30
            + sum7_score * 0.20
            + uniformity * 0.20
            + n6_dim    * 0.15
            + n6_resonance * 0.15;

        let mut r = HashMap::new();
        r.insert("prob_score".to_string(),    vec![prob_score]);
        r.insert("sum7_score".to_string(),    vec![sum7_score]);
        r.insert("uniformity".to_string(),    vec![uniformity]);
        r.insert("n6_dim".to_string(),        vec![n6_dim]);
        r.insert("n6_resonance".to_string(),  vec![n6_resonance]);
        r.insert("total_hits".to_string(),    vec![total_hits as f64]);
        r.insert("dice_score".to_string(),    vec![dice_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_probability_기본() {
        let n = 12; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| {
            match i % d { 0 => 1.0/N6, 1 => N6, 2 => TAU, 3 => SIGMA, 4 => PHI, _ => SOPFR }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = DiceProbabilityLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("dice_score"));
        assert!(r["dice_score"][0] >= 0.0 && r["dice_score"][0] <= 1.0);
    }

    #[test]
    fn test_dice_n6_dim() {
        let n = 8; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| i as f64 * 0.1 + 1.0).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = DiceProbabilityLens.scan(&data, n, d, &shared);
        let dim = r["n6_dim"][0];
        assert!((dim - 1.0).abs() < 1e-9, "d=6 → n6_dim=1.0: {}", dim);
    }

    #[test]
    fn test_dice_최소입력_거부() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let r = DiceProbabilityLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
