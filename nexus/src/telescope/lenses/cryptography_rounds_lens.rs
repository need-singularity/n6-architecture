use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 암호화 라운드 n=6 렌즈 — BT-114, BT-216
/// n=6 연결: AES=2^(sigma-sopfr)=128, SHA=256=2^(sigma-tau)*8, RSA=2048
pub struct CryptographyRoundsLens;

impl Lens for CryptographyRoundsLens {
    fn name(&self) -> &'static str { "CryptographyRoundsLens" }
    fn category(&self) -> &'static str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }
        let domain_consts: &[(f64, &str)] = &[
    (128.0, "aes_bits"),
    (256.0, "sha_bits"),
    (12.0, "sigma"),
    (24.0, "J2"),
    (4.0, "tau_rounds"),
    (10.0, "aes_rounds"),        ];
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();

        let mut n6_matches = 0usize;
        let mut matched = Vec::new();
        for &m in &means {
            for &(c, _) in domain_consts {
                if c > 1e-12 && ((m - c) / c).abs() < 0.06 {
                    n6_matches += 1; matched.push(c); break;
                }
            }
        }

        // σ·J₂=288 어트랙터 근접도
        let attractor_288 = means.iter().map(|&m| {
            if m.abs() > 1e-12 { (-((m - 288.0) / 288.0).powi(2) * 40.0).exp() }
            else { 0.0 }
        }).fold(0.0f64, f64::max);

        // 주요 n=6 비율 패턴 (phi=2 배증)
        let sorted_m: Vec<f64> = {
            let mut v: Vec<f64> = means.iter().cloned().filter(|&x| x > 1.0).collect();
            v.sort_by(|a,b| a.partial_cmp(b).unwrap()); v
        };
        let doubling_score = if sorted_m.len() >= 2 {
            let cnt = sorted_m.windows(2).filter(|w| w[0]>1e-12 && ((w[1]/w[0]) - 2.0).abs() < 0.15).count();
            cnt as f64 / (sorted_m.len()-1).max(1) as f64
        } else { 0.0 };

        let score = (n6_matches as f64 / d.max(1) as f64).min(1.0) * 0.45
            + attractor_288 * 0.30 + doubling_score * 0.25;

        let mut r = HashMap::new();
        r.insert("n6_domain_matches".to_string(), vec![n6_matches as f64]);
        r.insert("attractor_288_score".to_string(), vec![attractor_288]);
        r.insert("doubling_score".to_string(), vec![doubling_score]);
        r.insert("cryptography_rounds_score".to_string(), vec![score]);
        r.insert("matched_consts".to_string(), matched);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_cryptography_rounds_basic() {
        let n = 8; let d = 2;
        let data: Vec<f64> = (0..n*d).map(|i| if i%d==0 { 12.0 } else { 6.0 }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = CryptographyRoundsLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("cryptography_rounds_score"));
    }
}
