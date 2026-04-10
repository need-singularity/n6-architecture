use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 기후 대기 n=6 렌즈 — BT-119, BT-218
/// n=6 연결: 대류권=sigma=12km, 권역=n=6, 온실가스=n=6종, CO2 380ppm
pub struct ClimateAtmosphereLens;

impl Lens for ClimateAtmosphereLens {
    fn name(&self) -> &'static str { "ClimateAtmosphereLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let domain_consts: &[(f64, &str)] = &[
    (6.0, "ghg_species"),
    (12.0, "tropo_km"),
    (24.0, "strato_km"),
    (48.0, "sigma_tau_pressure"),
    (6.0, "n_layers"),
    (4.0, "tau_season"),        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &(c, _) in domain_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.06 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }

        // σ·J₂=288 어트랙터 근접도
        let attractor_288 = means.iter().map(|&m| {
            if m.abs() > 1e-12 { (-((m - 288.0) / 288.0).powi(2) * 40.0).exp() }
            else { 0.0 }
        }).fold(0.0f64, f64::max);

        // 주요 n=6 비율 패턴 (phi=2 배증)
        let sorted_m: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 1.0).collect();
            v.sort_by(|a,b| a.partial_cmp(b).unwrap()); v
        };
        let doubling_score = if sorted_m.len() >= 2 {
            let cnt = sorted_m.windows(2).filter(|w| w[0]>1e-12 && ((w[1]/w[0]) - 2.0).abs() < 0.15).count();
            cnt as f64 / (sorted_m.len()-1).max(1) as f64
        } else { 0.0 };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.45
            + attractor_288 * 0.30 + doubling_score * 0.25;

        let mut r = HashMap::new();
        r.insert("n6_domain_matches".to_string(), vec![n6_matches as f64]);
        r.insert("attractor_288_score".to_string(), vec![attractor_288]);
        r.insert("doubling_score".to_string(), vec![doubling_score]);
        r.insert("climate_atmosphere_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_climate_atmosphere_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 12.0 } else { 6.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ClimateAtmosphereLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("climate_atmosphere_score"));
    }
}
