use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 조성 화성 n=6 렌즈 — BT-108, BT-135
/// n=6 연결: 6음 헥사토닉 스케일, 3화음 tau=4 전위, 장/단 2극, 조성 순환 12*2=24
pub struct TonalHarmonyLens;

impl Lens for TonalHarmonyLens {
    fn name(&self) -> &'static str { "TonalHarmonyLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let tonal_consts: &[f64] = &[6.0, 3.0, 4.0, 2.0, 12.0, 24.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in tonal_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }
        // 5도권 순환 근접도: 12 key * 2 mode = 24
        let cycle24_proximity = means.iter().map(|&m| {
            (-((m - 24.0) / 24.0).powi(2) * 20.0).exp()
        }).fold(0.0f64, f64::max);
        // 3화음 완전성: 장3도(5/4=1.25) 또는 단3도(6/5=1.2) 비율 패턴
        let sorted_m: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 0.5).collect();
            v.sort_by(|a, b| a.partial_cmp(b).unwrap()); v
        };
        let triad_score = if sorted_m.len() >= 2 {
            let cnt = sorted_m.windows(2).filter(|w|
                w[0] > 1e-12 && ((w[1] / w[0]) - 1.5).abs() < 0.12).count();
            cnt as f64 / (sorted_m.len() - 1).max(1) as f64
        } else { 0.0 };
        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.45
            + cycle24_proximity * 0.30 + triad_score * 0.25;
        let mut r = HashMap::new();
        r.insert("n6_tonal_matches".to_string(), vec![n6_matches as f64]);
        r.insert("cycle24_proximity".to_string(), vec![cycle24_proximity]);
        r.insert("triad_score".to_string(), vec![triad_score]);
        r.insert("tonal_harmony_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_tonal_harmony_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i % d == 0 { 6.0 } else { 3.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = TonalHarmonyLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("tonal_harmony_score"));
        let score = result["tonal_harmony_score"][0];
        assert!(score >= 0.0 && score <= 1.0);
    }
}
