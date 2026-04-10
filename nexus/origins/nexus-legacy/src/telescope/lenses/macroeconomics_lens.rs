use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 거시경제 n=6 렌즈 — BT-143, BT-189
/// n=6 연결: 경기순환 6국면(회복→정점→하강→침체→저점→반전), tau(6)=4 분기, sigma(6)=12 월
pub struct MacroeconomicsLens;

impl Lens for MacroeconomicsLens {
    fn name(&self) -> &'static str { "MacroeconomicsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let macro_consts: &[f64] = &[6.0, 4.0, 12.0, 2.0, 3.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in macro_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // 경기 변동성: 3구간 이동 분산 근사
        let cycle_volatility = if n >= 6 {
            let third = n / 3;
            let s1: f64 = data[..third * d].iter().map(|&x| x.abs()).sum::<f64>()
                / (third * d).max(1) as f64;
            let s2: f64 = data[third*d..2*third*d].iter().map(|&x| x.abs()).sum::<f64>()
                / (third * d).max(1) as f64;
            let end = (n * d).min(data.len());
            let s3: f64 = data[2*third*d..end].iter().map(|&x| x.abs()).sum::<f64>()
                / ((n - 2 * third) * d).max(1) as f64;
            let avg = (s1 + s2 + s3) / 3.0;
            if avg > 1e-12 {
                ((s1 - avg).powi(2) + (s2 - avg).powi(2) + (s3 - avg).powi(2)).sqrt() / avg
            } else { 0.0 }
        } else { 0.0 };
        // sigma*phi=288 통화 승수 어트랙터
        let attractor_288 = means.iter().map(|&m| {
            if m.abs() > 1e-12 { (-((m - 288.0) / 288.0).powi(2) * 40.0).exp() } else { 0.0 }
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.45
            + (1.0 - cycle_volatility.min(1.0)) * 0.30 + attractor_288 * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_macro_matches".to_string(), vec![n6_matches as f64]);
        r.insert("cycle_volatility".to_string(), vec![cycle_volatility]);
        r.insert("attractor_288".to_string(), vec![attractor_288]);
        r.insert("macroeconomics_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_macroeconomics_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 6.0 } else { 24.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MacroeconomicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("macroeconomics_score"));
        let score = result["macroeconomics_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
