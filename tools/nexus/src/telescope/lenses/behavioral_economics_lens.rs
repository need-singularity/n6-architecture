use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 행동경제학 n=6 렌즈 — BT-143, BT-189, BT-212
/// n=6 연결: 6 인지 편향 클러스터, 손실회피 계수 phi(6)=2, 선택 과부하 임계 n=6
pub struct BehavioralEconomicsLens;

impl Lens for BehavioralEconomicsLens {
    fn name(&self) -> &'static str { "BehavioralEconomicsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let bias_consts: &[f64] = &[6.0, 2.0, 4.0, 3.0, 12.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in bias_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // 손실회피 비대칭: 음수 절댓값이 양수보다 phi=2배일 때 최대
        let end = (n * d).min(data.len());
        let pos_sum: f64 = data[..end].iter().filter(|&&x| x > 0.0).sum();
        let neg_sum: f64 = data[..end].iter().filter(|&&x| x < 0.0).map(|&x| x.abs()).sum();
        let loss_aversion = if pos_sum > 1e-12 && neg_sum > 1e-12 {
            let ratio = neg_sum / pos_sum;
            (-(ratio - 2.0).abs() * 3.0).exp()
        } else { 0.0 };
        // 선택 과부하 임계 6: 옵션 수 6에서 결정 효율 최대
        let choice_6_proximity = means.iter().map(|&m| {
            (-((m - 6.0) / 6.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.40
            + loss_aversion * 0.35 + choice_6_proximity * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_bias_matches".to_string(), vec![n6_matches as f64]);
        r.insert("loss_aversion_score".to_string(), vec![loss_aversion]);
        r.insert("choice_6_proximity".to_string(), vec![choice_6_proximity]);
        r.insert("behavioral_economics_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_behavioral_economics_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 6.0 } else { 2.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = BehavioralEconomicsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("behavioral_economics_score"));
        let score = result["behavioral_economics_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
