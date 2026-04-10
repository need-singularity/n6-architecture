use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// 주사위 엔트로피 렌즈 — 6면 주사위 정보엔트로피 수렴
///
/// 균일 6면 주사위의 섀넌 엔트로피 = ln(6) ≈ 1.7918 nats
/// 비트 단위: log2(6) ≈ 2.585 bits
/// n=6 상수와 엔트로피 관계:
///   H_max = ln(n) = ln(6)
///   정보 이득 = H_max - H_observed
pub struct DiceEntropyLens;

const N6: f64 = 6.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const LN6: f64 = 1.791_759_469_228_327; // ln(6)
const LOG2_6: f64 = 2.584_962_500_721_156; // log2(6)

impl Lens for DiceEntropyLens {
    fn name(&self) -> &str { "DiceEntropyLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        // 1. 섀넌 엔트로피 추정 (히스토그램 기반, 6 빈)
        let (lo, hi) = data.iter().fold((f64::INFINITY, f64::NEG_INFINITY), |(mn, mx), &v| {
            (mn.min(v), mx.max(v))
        });
        let range = (hi - lo).max(1e-12);
        let bins = N6 as usize; // 6 bins
        let mut hist = vec![0usize; bins];
        for &v in data {
            let idx = ((v - lo) / range * bins as f64).floor() as usize;
            hist[idx.min(bins - 1)] += 1;
        }
        let total = data.len() as f64;
        let shannon_nats: f64 = hist.iter()
            .filter(|&&c| c > 0)
            .map(|&c| { let p = c as f64 / total; -p * p.ln() })
            .sum();

        // 2. ln(6) 공명
        let ln6_score = (1.0 - (shannon_nats - LN6).abs() / LN6).max(0.0);

        // 3. log2(6) 공명
        let shannon_bits = shannon_nats / std::f64::consts::LN_2;
        let log2_6_score = (1.0 - (shannon_bits - LOG2_6).abs() / LOG2_6).max(0.0);

        // 4. 차원 평균의 n=6 상수 공명
        let means: Vec<f64> = (0..d).map(|j| {
            (0..n).map(|i| data[i * d + j]).sum::<f64>() / n as f64
        }).collect();
        let targets = [N6, TAU, SIGMA, PHI, LN6, LOG2_6];
        let hits = means.iter().filter(|&&m| {
            targets.iter().any(|&t| t > 1e-12 && ((m - t) / t).abs() < 0.08)
        }).count();
        let n6_resonance = hits as f64 / d.max(1) as f64;

        // 5. n=6 차원 공명
        let n6_dim = (1.0 - (d as f64 - N6).abs() / N6 * 2.0).max(0.0);

        let entropy_score = ln6_score    * 0.35
            + log2_6_score  * 0.25
            + n6_resonance  * 0.20
            + n6_dim        * 0.20;

        let mut r = HashMap::new();
        r.insert("shannon_nats".to_string(),  vec![shannon_nats]);
        r.insert("shannon_bits".to_string(),  vec![shannon_bits]);
        r.insert("ln6_score".to_string(),     vec![ln6_score]);
        r.insert("log2_6_score".to_string(),  vec![log2_6_score]);
        r.insert("n6_resonance".to_string(),  vec![n6_resonance]);
        r.insert("n6_dim".to_string(),        vec![n6_dim]);
        r.insert("entropy_score".to_string(), vec![entropy_score]);
        r
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dice_entropy_기본() {
        let n = 12; let d = 6;
        // 1~6 균일 분포 시뮬
        let data: Vec<f64> = (0..n * d).map(|i| (i % 6 + 1) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = DiceEntropyLens.scan(&data, n, d, &shared);
        assert!(r.contains_key("entropy_score"));
        assert!(r["entropy_score"][0] >= 0.0 && r["entropy_score"][0] <= 1.0);
        let h = r["shannon_nats"][0];
        assert!(h > 0.0, "엔트로피 > 0: {}", h);
    }

    #[test]
    fn test_dice_entropy_ln6_공명() {
        // 균일 6분포 → ln6_score 높아야
        let n = 60; let d = 6;
        let data: Vec<f64> = (0..n * d).map(|i| (i % 6 + 1) as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let r = DiceEntropyLens.scan(&data, n, d, &shared);
        let h = r["shannon_nats"][0];
        assert!((h - LN6).abs() < 0.15, "균일분포 엔트로피 ≈ ln6: {}", h);
    }

    #[test]
    fn test_dice_entropy_최소입력_거부() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let r = DiceEntropyLens.scan(&data, 5, 1, &shared);
        assert!(r.is_empty());
    }
}
