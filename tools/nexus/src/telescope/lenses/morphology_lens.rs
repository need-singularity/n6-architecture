use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 형태소 분포 n=6 렌즈 — BT-112, BT-215
/// n=6 연결: 핵심 품사 6종, tau(6)=4 어형변화 클래스, sigma(6)=12 음운 패턴
pub struct MorphologyLens;

impl Lens for MorphologyLens {
    fn name(&self) -> &'static str { "MorphologyLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let morph_consts: &[f64] = &[6.0, 4.0, 12.0, 2.0, 3.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in morph_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        let mean_all: f64 = means.iter().sum::<f64>() / means.len().max(1) as f64;
        let var: f64 = means.iter().map(|&m| (m - mean_all).powi(2)).sum::<f64>()
            / means.len().max(1) as f64;
        let morpheme_diversity = if mean_all.abs() > 1e-12 { var.sqrt() / mean_all.abs() } else { 0.0 };
        let tau4_proximity = means.iter().map(|&m| {
            (-((m - 4.0) / 4.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let sigma12_proximity = means.iter().map(|&m| {
            (-((m - 12.0) / 12.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.40
            + tau4_proximity * 0.30 + sigma12_proximity * 0.20
            + (1.0 - morpheme_diversity.min(1.0)) * 0.10;
        let mut r = HashMap::new();
        r.insert("n6_morph_matches".to_string(), vec![n6_matches as f64]);
        r.insert("morpheme_diversity".to_string(), vec![morpheme_diversity]);
        r.insert("tau4_proximity".to_string(), vec![tau4_proximity]);
        r.insert("sigma12_proximity".to_string(), vec![sigma12_proximity]);
        r.insert("morphology_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_morphology_basic() {
        let n = 8; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| match i % d {
            0 => 6.0, 1 => 4.0, _ => 12.0,
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MorphologyLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("morphology_score"));
        let score = result["morphology_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
