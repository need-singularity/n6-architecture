use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 면역유전학 n=6 렌즈 — BT-155, BT-194, BT-220
/// n=6 연결: IgG/IgM/IgA/IgD/IgE/IgG2 = 6 면역글로불린 클래스, MHC-II 6서브유닛, HLA 6좌위
pub struct ImmunogeneticsLens;

impl Lens for ImmunogeneticsLens {
    fn name(&self) -> &'static str { "ImmunogeneticsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let ig_consts: &[f64] = &[6.0, 12.0, 4.0, 3.0, 2.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in ig_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // HLA 다형성 지수: max/min 비율이 n=6에 가까울 때 최대
        let pos_means: Vec<f64> = means.iter().cloned().filter(|&x| x > 1e-12).collect();
        let hla_diversity = if pos_means.len() >= 2 {
            let min_m = pos_means.iter().cloned().fold(f64::MAX, f64::min);
            let max_m = pos_means.iter().cloned().fold(f64::MIN, f64::max);
            if min_m > 1e-12 {
                let ratio = max_m / min_m;
                (-(ratio - 6.0).abs() / 6.0 * 10.0).exp()
            } else { 0.0 }
        } else { 0.0 };
        // 6 면역글로불린 클래스 근접도
        let ig6_proximity = means.iter().map(|&m| {
            (-((m - 6.0) / 6.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.45
            + hla_diversity * 0.30 + ig6_proximity * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_ig_matches".to_string(), vec![n6_matches as f64]);
        r.insert("hla_diversity".to_string(), vec![hla_diversity]);
        r.insert("ig6_proximity".to_string(), vec![ig6_proximity]);
        r.insert("immunogenetics_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_immunogenetics_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 6.0 } else { 12.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ImmunogeneticsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("immunogenetics_score"));
        let score = result["immunogenetics_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
