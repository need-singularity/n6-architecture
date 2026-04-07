use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 생태적 지위 n=6 렌즈 — BT-178, BT-201
/// n=6 연결: 생태 계층 6단계(생산자→최상위포식자), sigma(6)=12 군집 구조, tau(6)=4 계절 주기
pub struct EcologicalNicheLens;

impl Lens for EcologicalNicheLens {
    fn name(&self) -> &'static str { "EcologicalNicheLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let eco_consts: &[f64] = &[6.0, 12.0, 4.0, 3.0, 2.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in eco_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // Simpson 다양성 지수 (1 - sum(p^2))
        let total: f64 = means.iter().map(|&m| m.abs()).sum::<f64>();
        let simpson = if total > 1e-12 {
            let sum_sq: f64 = means.iter().map(|&m| (m.abs() / total).powi(2)).sum();
            1.0 - sum_sq
        } else { 0.0 };
        // 먹이사슬 계층 6단계 근접도
        let trophic6_proximity = means.iter().map(|&m| {
            (-((m - 6.0) / 6.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.40
            + simpson * 0.35 + trophic6_proximity * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_eco_matches".to_string(), vec![n6_matches as f64]);
        r.insert("simpson_diversity".to_string(), vec![simpson]);
        r.insert("trophic6_proximity".to_string(), vec![trophic6_proximity]);
        r.insert("ecological_niche_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_ecological_niche_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 6.0 } else { 12.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = EcologicalNicheLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("ecological_niche_score"));
        let score = result["ecological_niche_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
