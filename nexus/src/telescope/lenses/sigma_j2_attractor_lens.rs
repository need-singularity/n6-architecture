use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// σ·J₂=288 어트랙터 렌즈 — 범도메인 288 수렴 탐지 (BT-55, BT-69, BT-75)
/// 288 = sigma(6) * J2(6) = 12 * 24 = HBM 용량 GB, GPU SM 배열
pub struct SigmaJ2AttractorLens;

impl Lens for SigmaJ2AttractorLens {
    fn name(&self) -> &'static str { "SigmaJ2AttractorLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let attractor: f64 = 288.0;
        let sub_attractors: &[f64] = &[144.0, 288.0, 576.0, 48.0, 24.0, 12.0, 6.0];

        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let primary_score = means.iter().map(|&m| {
            if m.abs() > 1e-12 { (-((m - attractor) / attractor).powi(2) * 50.0).exp() }
            else { 0.0 }
        }).fold(0.0f64, f64::max);

        let mut sub_hits = 0usize;
        for &m in &means {
            for &s in sub_attractors {
                if s > 1e-12 && ((m - s) / s).abs() < 0.04 { sub_hits += 1; break; }
            }
        }

        // 288 분할 패턴: 288 = 144*2 = 96*3 = 48*6 = 24*12
        let divisor_pattern_score = {
            let divisors_288: &[f64] = &[1.0,2.0,3.0,4.0,6.0,8.0,12.0,16.0,18.0,24.0,
                                          32.0,36.0,48.0,72.0,96.0,144.0,288.0];
            let mut hits = 0usize;
            for &m in &means {
                if divisors_288.iter().any(|&dv| dv > 1e-12 && ((m-dv)/dv).abs() < 0.04) {
                    hits += 1;
                }
            }
            hits as f64 / means.len().max(1) as f64
        };

        let score = primary_score * 0.4 + (sub_hits as f64 / d.max(1) as f64).min(1.0) * 0.35
            + divisor_pattern_score * 0.25;

        let mut r = HashMap::new();
        r.insert("sigma_j2_288_score".to_string(), vec![primary_score]);
        r.insert("sub_attractor_hits".to_string(), vec![sub_hits as f64]);
        r.insert("divisor_pattern_score".to_string(), vec![divisor_pattern_score]);
        r.insert("sigma_j2_attractor_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_sigma_j2_288() {
        let n = 6; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 288.0 } else { 144.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = SigmaJ2AttractorLens.scan(&data, n, d, &shared);
        assert!(result["sigma_j2_288_score"][0] > 0.8);
    }
    #[test]
    fn test_sigma_j2_sub() {
        let n = 8; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| [48.0f64, 24.0, 12.0][i%d]).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = SigmaJ2AttractorLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("sigma_j2_attractor_score"));
    }
}
