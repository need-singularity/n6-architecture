use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 고분자 사슬 n=6 렌즈 (BT-121) — n=6 연결: repeat_unit_n:branch_phi:crosslink_tau:monomer_sigma:crystallinity_J2:Tg_6
pub struct PolymerChainLens;

impl Lens for PolymerChainLens {
    fn name(&self) -> &'static str { "PolymerChainLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let dom_consts: &[f64] = &[6.0, 2.0, 4.0, 12.0, 24.0, 3.0];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &c in dom_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.07 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }

        let attractor_288 = means.iter().map(|&m| {
            if m.abs() > 1e-12 { (-((m - 288.0) / 288.0).powi(2) * 40.0).exp() }
            else { 0.0 }
        }).fold(0.0f64, f64::max);

        let sorted_m: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 1.0).collect();
            v.sort_by(|a,b| a.partial_cmp(b).unwrap()); v
        };
        let phi_doubling = if sorted_m.len() >= 2 {
            let cnt = sorted_m.windows(2).filter(|w|
                w[0]>1e-12 && ((w[1]/w[0]) - 2.0).abs() < 0.15).count();
            cnt as f64 / (sorted_m.len()-1).max(1) as f64
        } else { 0.0 };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.45
            + attractor_288 * 0.25 + phi_doubling * 0.30;

        let mut r = HashMap::new();
        r.insert("n6_matches".to_string(), vec![n6_matches as f64]);
        r.insert("attractor_288".to_string(), vec![attractor_288]);
        r.insert("phi_doubling".to_string(), vec![phi_doubling]);
        r.insert("polymer_chain_score".to_string(), vec![score]);
        r.insert("matched_vals".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_polymer_chain_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 6.0 } else { 12.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = PolymerChainLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("polymer_chain_score"));
    }
}
