use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 균류학 렌즈 — 균류 생물학 n=6 패턴 (균사 분기 n=6, 포자 6면체, 키틴 CN=6)
/// n=6 연결: 균사 분기=n=6, 균류 계통=6계, 키틴 N-아세틸 CN=6, 포자 크기~6마이크론
pub struct MycologyLens;

impl Lens for MycologyLens {
    fn name(&self) -> &'static str { "MycologyLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let myco_consts: &[(f64, &str)] = &[
            (6.0,  "hyphal_branch_n"), (6.0, "kingdom_class"),
            (12.0, "sigma_spore_size"), (4.0, "tau_growth_phase"),
            (2.0,  "phi_dikaryotic"), (3.0, "n_phi_mycelium"),
            (24.0, "J2_chitin_chain"),
        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        for &m in &means {
            for &(c, _) in myco_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.08 { n6_matches += 1; break; }
            }
        }

        // 균사 분기 패턴: 이진/삼진 분기 반복 구조
        let branching_score = {
            let col: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            let diffs: Vec<f64> = col.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
            if !diffs.is_empty() {
                let mean_d = diffs.iter().sum::<f64>() / diffs.len() as f64;
                let var_d = diffs.iter().map(|x| (x - mean_d).powi(2)).sum::<f64>() / diffs.len() as f64;
                let cv = if mean_d > 1e-12 { var_d.sqrt() / mean_d } else { 1.0 };
                // Low CV = regular branching intervals
                (-cv * 3.0).exp()
            } else { 0.0 }
        };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.5 + branching_score * 0.5;

        let mut r = HashMap::new();
        r.insert("n6_myco_matches".to_string(), vec![n6_matches as f64]);
        r.insert("hyphal_branching_score".to_string(), vec![branching_score]);
        r.insert("mycology_score".to_string(), vec![score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_mycology_basic() {
        let n = 8; let d = 3;
        let data: Vec<f64> = (0..n*d).map(|i| [6.0f64, 12.0, 4.0][i%d]).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = MycologyLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mycology_score"));
        assert!(result["n6_myco_matches"][0] >= 2.0);
    }
}
