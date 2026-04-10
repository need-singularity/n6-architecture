use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 음성학 n=6 렌즈 — BT-112, BT-137
/// n=6 연결: 6 조음 위치(입술/치/구개/연구개/인두/성문), sigma(6)=12 모음 포먼트 쌍, tau(6)=4 성조
pub struct PhoneticsLens;

impl Lens for PhoneticsLens {
    fn name(&self) -> &'static str { "PhoneticsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let phoneme_consts: &[f64] = &[6.0, 12.0, 4.0, 2.0, 3.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in phoneme_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // 포먼트 F1/F2 비율 패턴: 장2도(9/8) ~ 완전5도(3/2) 범위
        let sorted_m: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 0.5).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v
        };
        let formant_ratio_score = if sorted_m.len() >= 2 {
            let cnt = sorted_m.windows(2).filter(|w| {
                let r = w[1] / w[0].max(1e-12);
                (r - 2.0).abs() < 0.2 || (r - 3.0).abs() < 0.2
            }).count();
            cnt as f64 / (sorted_m.len() - 1).max(1) as f64
        } else { 0.0 };
        // 6 조음 위치 근접도
        let articulatory6 = means.iter().map(|&m| {
            (-((m - 6.0) / 6.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.40
            + formant_ratio_score * 0.35 + articulatory6 * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_phoneme_matches".to_string(), vec![n6_matches as f64]);
        r.insert("formant_ratio_score".to_string(), vec![formant_ratio_score]);
        r.insert("articulatory6".to_string(), vec![articulatory6]);
        r.insert("phonetics_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_phonetics_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 12.0 } else { 6.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = PhoneticsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("phonetics_score"));
        let score = result["phonetics_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
