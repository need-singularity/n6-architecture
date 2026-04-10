use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 칩×컴퓨팅 강결합 렌즈 — σ²=144 SM, J2=24 HBM, τ=4 레이어 (BT-28, BT-55, BT-79)
pub struct ChipComputeCouplingLens;

impl Lens for ChipComputeCouplingLens {
    fn name(&self) -> &'static str { "ChipComputeCouplingLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let chip_consts: &[(f64, &str)] = &[
            (144.0, "sigma_sq_SM"), (288.0, "sigma_J2_HBM"), (12.0, "sigma"),
            (24.0, "J2_HBM"), (4.0, "tau_layers"), (8.0, "sigma_tau_kv"),
            (48.0, "sigma_tau_freq"), (192.0, "sigma_16"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        let mut matched_vals = Vec::new();
        for &m in &means {
            for &(c, _) in chip_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.05 {
                    n6_matches += 1; matched_vals.push(c); break;
                }
            }
        }

        // σ²=144 어트랙터 검출 (BT-79)
        let sigma_sq_proximity = means.iter().map(|&m| {
            (-((m - 144.0) / 144.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);

        // σ·J₂=288 어트랙터
        let sigma_j2_proximity = means.iter().map(|&m| {
            (-((m - 288.0) / 288.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.4
            + sigma_sq_proximity * 0.3
            + sigma_j2_proximity * 0.3;

        let mut r = HashMap::new();
        r.insert("n6_chip_matches".to_string(), vec![n6_matches as f64]);
        r.insert("sigma_sq_proximity".to_string(), vec![sigma_sq_proximity]);
        r.insert("sigma_j2_proximity".to_string(), vec![sigma_j2_proximity]);
        r.insert("chip_compute_score".to_string(), vec![score]);
        r.insert("matched_chip_consts".to_string(), matched_vals);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_chip_compute_144() {
        let n = 8; let d = 3;
        let data: Vec<f64> = (0..n * d).map(|i| {
            let vals = [144.0f64, 288.0, 12.0];
            vals[i % d] + (i as f64 * 0.01)
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ChipComputeCouplingLens.scan(&data, n, d, &shared);
        assert!(result["n6_chip_matches"][0] >= 2.0);
        assert!(result["sigma_sq_proximity"][0] > 0.5);
    }
}
