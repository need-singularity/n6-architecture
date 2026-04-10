use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 향수과학 렌즈 — 향 분자 n=6 구조 (벤젠 Z=6, 테르펜 C6, 노트 3계층)
/// n=6 연결: 탑/미들/베이스=n/phi=3노트, 벤젠 C6=n, 테르펜 C5=sopfr 단위, 무스크 18원환
pub struct PerfumeryLens;

impl Lens for PerfumeryLens {
    fn name(&self) -> &'static str { "PerfumeryLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let perfume_consts: &[(f64, &str)] = &[
            (6.0, "benzene_C6_n"), (3.0, "note_tiers_n_phi"), (5.0, "terpene_C5_sopfr"),
            (18.0, "musk_ring_3n"), (12.0, "sigma_aldehyde"), (2.0, "phi_chiral"),
            (10.0, "sigma_phi_boiling_decade"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in perfume_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 3계층 향 지속성 패턴 (탑=빠름, 미들=중간, 베이스=느림)
        let three_tier_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let third = n / 3;
            if third > 0 {
                let t1 = col[..third].iter().sum::<f64>() / third as f64;
                let t2 = col[third..2*third].iter().sum::<f64>() / third as f64;
                let t3 = col[2*third..].iter().sum::<f64>() / (n - 2*third).max(1) as f64;
                if t1 >= t2 && t2 >= t3 && t3 > 0.0 {
                    let ratio = t3 / t1;
                    1.0 - ratio.min(1.0) // 큰 감쇠 = 좋은 3계층 패턴
                } else { 0.0 }
            } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5 + three_tier_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_perfume_matches".to_string(), vec![n6_matches as f64]);
        r.insert("three_tier_score".to_string(), vec![three_tier_score]);
        r.insert("perfumery_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_perfumery_basic() {
        let n = 9; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| {
            let p = i / d;
            if i%d==0 { if p < 3 { 6.0 } else if p < 6 { 3.0 } else { 1.0 } } else { 6.0 }
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = PerfumeryLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("perfumery_score"));
    }
}
