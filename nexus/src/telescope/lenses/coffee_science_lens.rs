use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 커피과학 렌즈 — 추출 화학 n=6 패턴 (카페인 C8H10N4O2, 추출비율 1:6~1:18)
/// n=6 연결: 추출비율=1:n=6~1:sigma=12, 로스팅 단계=n=6, 클로로겐산 이성질체=n=6
pub struct CoffeeScienceLens;

impl Lens for CoffeeScienceLens {
    fn name(&self) -> &'static str { "CoffeeScienceLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let coffee_consts: &[(f64, &str)] = &[
            (6.0, "brew_ratio_n"), (12.0, "brew_ratio_sigma"), (18.0, "brew_ratio_3n"),
            (4.0, "roast_stages_tau"), (6.0, "chlorogenic_isomers"), (96.0, "sigma_sq_min"),
            (3.0, "extraction_phases_n_phi"), (200.0, "brew_temp_c_approx"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in coffee_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 추출 커브: 초반 급증 후 안정 (over-extraction 전)
        let extraction_curve_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            if n >= 4 {
                let first_half_mean = col[..n/2].iter().sum::<f64>() / (n/2) as f64;
                let second_half_mean = col[n/2..].iter().sum::<f64>() / (n - n/2) as f64;
                if first_half_mean > second_half_mean && first_half_mean > 1e-12 {
                    (1.0 - second_half_mean / first_half_mean).min(1.0).max(0.0)
                } else { 0.0 }
            } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5 + extraction_curve_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_coffee_matches".to_string(), vec![n6_matches as f64]);
        r.insert("extraction_curve_score".to_string(), vec![extraction_curve_score]);
        r.insert("coffee_science_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_coffee_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 6.0 } else { 12.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = CoffeeScienceLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("coffee_science_score"));
    }
}
