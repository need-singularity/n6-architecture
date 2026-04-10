use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 원예학 렌즈 — 식물 성장 n=6 패턴 (피보나치 잎, 꽃잎 n=6, 과실 분절)
/// n=6 연결: 꽃잎=n=6(백합), 잎 배열 6회전, 과실 분절=n=6, 성장계절=n=6단계
pub struct HorticultureLens;

impl Lens for HorticultureLens {
    fn name(&self) -> &'static str { "HorticultureLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let hort_consts: &[(f64, &str)] = &[
            (6.0, "petal_n"), (3.0, "n_phi_sepal"), (12.0, "sigma_month"),
            (4.0, "tau_season"), (5.0, "sopfr_petal_alt"), (24.0, "J2_hour"),
            (2.0, "phi_cotyledon"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in hort_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 성장 곡선 탐지 (로지스틱 S-커브 패턴)
        let growth_curve_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let diffs: Vec<f64> = col.windows(2).map(|w| w[1] - w[0]).collect();
            if diffs.len() >= 3 {
                let peak_idx = diffs.iter().enumerate()
                    .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap()).map(|(i,_)| i).unwrap_or(0);
                if peak_idx > 0 && peak_idx < diffs.len() - 1 { 0.7 } else { 0.3 }
            } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5 + growth_curve_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_hort_matches".to_string(), vec![n6_matches as f64]);
        r.insert("growth_curve_score".to_string(), vec![growth_curve_score]);
        r.insert("horticulture_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_horticulture_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 6.0 } else { 4.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = HorticultureLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("horticulture_score"));
    }
}
