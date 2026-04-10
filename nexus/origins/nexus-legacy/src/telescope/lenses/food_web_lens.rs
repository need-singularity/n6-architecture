use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 먹이그물 n=6 렌즈 — BT-178, BT-201, BT-233
/// n=6 연결: 영양 단계 평균 6, 포식-피식 비율 sigma(6)/tau(6)=3, 생물량 피라미드 6층
pub struct FoodWebLens;

impl Lens for FoodWebLens {
    fn name(&self) -> &'static str { "FoodWebLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let trophic_consts: &[f64] = &[6.0, 3.0, 4.0, 12.0, 2.0, 1.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in trophic_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // 에너지 전달 효율: 연속 계층간 비율이 3~12 배수(10~30% 전달)
        let sorted_m: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 1e-12).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v
        };
        let energy_transfer_score = if sorted_m.len() >= 2 {
            let cnt = sorted_m.windows(2).filter(|w| {
                let ratio = w[1] / w[0].max(1e-12);
                ratio >= 3.0 && ratio <= 12.0
            }).count();
            cnt as f64 / (sorted_m.len() - 1).max(1) as f64
        } else { 0.0 };
        // 영양 단계 6층 근접도
        let trophic6 = means.iter().map(|&m| {
            (-((m - 6.0) / 6.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.40
            + energy_transfer_score * 0.35 + trophic6 * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_trophic_matches".to_string(), vec![n6_matches as f64]);
        r.insert("energy_transfer_score".to_string(), vec![energy_transfer_score]);
        r.insert("trophic6".to_string(), vec![trophic6]);
        r.insert("food_web_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_food_web_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 4.0 } else { 6.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = FoodWebLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("food_web_score"));
        let score = result["food_web_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
