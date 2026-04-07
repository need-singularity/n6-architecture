use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, shannon_entropy};

/// n=6 constants used for proximity detection
const N: f64 = 6.0;
const SIGMA: f64 = 12.0;    // σ(6) = sum of divisors
const PHI: f64 = 2.0;       // φ(6) = Euler totient
const TAU: f64 = 4.0;       // τ(6) = number of divisors
const J2: f64 = 24.0;       // Jordan J₂(6)
const SOPFR: f64 = 5.0;     // sum of prime factors with repetition (2+3)
const SIGMA_SQ: f64 = 144.0; // σ² = 144 (SM count, AD102)

/// Key computing architecture constants from BT-28, BT-55, BT-58, BT-69
const ARCH_CONSTANTS: &[(f64, &str)] = &[
    (6.0, "n"),
    (12.0, "sigma"),
    (24.0, "J2"),
    (144.0, "sigma_sq"),
    (132.0, "sigma*(sigma-mu)"),  // H100 SMs
    (2.0, "phi"),
    (4.0, "tau"),
    (8.0, "sigma-tau"),           // universal AI constant (BT-58)
    (48.0, "sigma*tau"),          // gate pitch nm
    (128.0, "2^(sigma-sopfr)"),   // d_head
    (256.0, "2^(sigma-tau)"),     // width
    (288.0, "sigma*J2"),          // HBM capacity
    (10.0, "sigma-phi"),          // layers, HBM interface
    (16.0, "2^tau"),              // FP16 bits
    (32.0, "2^sopfr"),            // FP32 bits
    (64.0, "2^n"),                // FP64 bits
    (96.0, "sigma*(sigma-tau)"),  // Tesla 96S, layers
];

/// Tolerance for matching a value to an n=6 constant (relative)
const REL_TOL: f64 = 0.05;

/// ChipArchitectureLens: Detect computing architecture patterns in data.
///
/// Algorithm:
///   1. Compute feature-wise statistics (mean, variance, ratios)
///   2. Check each feature mean/median against n=6 architecture constants
///   3. Detect ratio patterns between features (powers of 2, σ/τ ratios)
///   4. Measure pipeline-stage regularity (layered structure)
///   5. Reports n6_match_count, architecture_score, detected_constants
pub struct ChipArchitectureLens;

fn matches_constant(val: f64, target: f64) -> bool {
    if target.abs() < 1e-12 {
        return val.abs() < 1e-6;
    }
    ((val - target) / target).abs() < REL_TOL
}

fn count_n6_matches(values: &[f64]) -> (usize, Vec<f64>) {
    let mut count = 0;
    let mut matched = Vec::new();
    for &v in values {
        for &(c, _) in ARCH_CONSTANTS {
            if matches_constant(v, c) {
                count += 1;
                matched.push(c);
                break;
            }
        }
    }
    (count, matched)
}

impl Lens for ChipArchitectureLens {
    fn name(&self) -> &str {
        "ChipArchitectureLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let (means, vars) = mean_var(data, n, d);

        // 1. Check feature means against architecture constants
        let (mean_matches, matched_constants) = count_n6_matches(&means);

        // 2. Check feature-wise ratios between adjacent dimensions
        let mut ratio_matches = 0;
        let mut ratios = Vec::new();
        for i in 0..d.saturating_sub(1) {
            if means[i].abs() > 1e-12 {
                let ratio = means[i + 1] / means[i];
                ratios.push(ratio);
                // Check if ratio matches n=6 pattern: 2, 3, 4, 6, 8, 12
                for &target in &[PHI, 3.0, TAU, N, SIGMA - TAU, SIGMA] {
                    if matches_constant(ratio, target) || matches_constant(ratio, 1.0 / target) {
                        ratio_matches += 1;
                        break;
                    }
                }
            }
        }

        // 3. Power-of-2 ladder detection (common in memory hierarchies)
        //    Check if values form geometric sequences with base 2
        let mut sorted_means = means.clone();
        sorted_means.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut power2_count = 0;
        for &v in &sorted_means {
            if v > 0.0 {
                let log2v = v.log2();
                if (log2v - log2v.round()).abs() < 0.1 {
                    power2_count += 1;
                }
            }
        }

        // 4. Pipeline regularity: check if data has layered structure
        //    via entropy of pairwise distance distribution
        let pair_count = n.min(50) * (n.min(50) - 1) / 2;
        let nn = n.min(50);
        let mut pair_dists = Vec::with_capacity(pair_count);
        for i in 0..nn {
            for j in (i + 1)..nn {
                pair_dists.push(shared.dist(i, j));
            }
        }
        let dist_entropy = shannon_entropy(&pair_dists, ((pair_count as f64).sqrt() as usize).max(4));

        // 5. SM count proxy: check if n*d or feature products match σ²=144
        let nd_product = (n * d) as f64;
        let sm_match = matches_constant(nd_product, SIGMA_SQ)
            || matches_constant(nd_product, J2)
            || matches_constant(nd_product, SIGMA);

        // 6. Architecture score: weighted combination
        let total_features = d.max(1) as f64;
        let mean_match_ratio = mean_matches as f64 / total_features;
        let ratio_match_ratio = if d > 1 {
            ratio_matches as f64 / (d - 1) as f64
        } else {
            0.0
        };
        let power2_ratio = power2_count as f64 / total_features;

        let architecture_score = mean_match_ratio * 0.4
            + ratio_match_ratio * 0.3
            + power2_ratio * 0.2
            + if sm_match { 0.1 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("n6_match_count".to_string(), vec![mean_matches as f64]);
        result.insert("architecture_score".to_string(), vec![architecture_score]);
        result.insert("ratio_match_count".to_string(), vec![ratio_matches as f64]);
        result.insert("power2_count".to_string(), vec![power2_count as f64]);
        result.insert("distance_entropy".to_string(), vec![dist_entropy]);
        result.insert("matched_constants".to_string(), matched_constants);
        result.insert("feature_ratios".to_string(), ratios);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_chip_architecture_detects_n6_constants() {
        // Features with means near n=6 constants: 12, 24, 144
        let n = 20;
        let d = 3;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let noise = (i as f64 * 0.01).sin() * 0.1;
            data.push(12.0 + noise);   // σ
            data.push(24.0 + noise);   // J₂
            data.push(144.0 + noise);  // σ²
        }
        let shared = make_shared(&data, n, d);
        let result = ChipArchitectureLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("n6_match_count"));
        let matches = result["n6_match_count"][0];
        assert!(matches >= 2.0, "Expected at least 2 n6 matches, got {matches}");
    }

    #[test]
    fn test_chip_architecture_power2_ladder() {
        // Memory hierarchy: 16, 32, 64, 128, 256
        let n = 10;
        let d = 5;
        let mut data = Vec::with_capacity(n * d);
        for _i in 0..n {
            data.push(16.0);
            data.push(32.0);
            data.push(64.0);
            data.push(128.0);
            data.push(256.0);
        }
        let shared = make_shared(&data, n, d);
        let result = ChipArchitectureLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("power2_count"));
        let p2 = result["power2_count"][0];
        assert!(p2 >= 4.0, "Expected at least 4 power-of-2 features, got {p2}");
    }

    #[test]
    fn test_chip_architecture_ratio_detection() {
        // Features with ratio of 2 (phi) between them
        let n = 15;
        let d = 3;
        let mut data = Vec::with_capacity(n * d);
        for _i in 0..n {
            data.push(6.0);
            data.push(12.0);  // ratio = 2 = φ
            data.push(24.0);  // ratio = 2 = φ
        }
        let shared = make_shared(&data, n, d);
        let result = ChipArchitectureLens.scan(&data, n, d, &shared);

        let ratio_matches = result["ratio_match_count"][0];
        assert!(ratio_matches >= 1.0, "Expected ratio matches, got {ratio_matches}");
    }
}
