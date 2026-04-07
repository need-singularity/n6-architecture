use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// HypothesisGenLens: Scans data for n=6 signature patterns that suggest new hypotheses.
///
/// Algorithm:
///   1. Extract statistical features: mean, std, ratios between dimensions
///   2. Match features against n=6 constant family (σ=12, φ=2, τ=4, J₂=24, sopfr=5)
///   3. Score each match by proximity (EXACT < 1%, CLOSE < 5%, WEAK < 10%)
///   4. Count hypothesis candidates = features matching n=6 constants
///   5. Compute hypothesis_density = candidates / total_features
///
/// Outputs:
///   - hypothesis_candidates: number of n=6-matching features found
///   - hypothesis_density: fraction of features that match n=6
///   - exact_matches: count of EXACT matches (< 1% deviation)
///   - close_matches: count of CLOSE matches (1-5% deviation)
///   - best_match_constant: which n=6 constant matched best
///   - best_match_deviation: deviation of best match (lower = better)
pub struct HypothesisGenLens;

// n=6 constant family for hypothesis matching
const N6_CONSTANTS: &[(f64, &str)] = &[
    (1.0, "mu"),           // μ(6)=1
    (2.0, "phi"),          // φ(6)=2
    (3.0, "n/phi"),        // n/φ=3
    (4.0, "tau"),          // τ(6)=4
    (5.0, "sopfr"),        // sopfr(6)=5
    (6.0, "n"),            // n=6
    (8.0, "sigma-tau"),    // σ-τ=8
    (10.0, "sigma-phi"),   // σ-φ=10
    (11.0, "sigma-mu"),    // σ-μ=11
    (12.0, "sigma"),       // σ(6)=12
    (24.0, "J2"),          // J₂(6)=24
    (0.288, "ln43"),       // ln(4/3)≈0.288
    (0.693, "ln2"),        // ln(2)≈0.693
    (1.333, "4/3"),        // 4/3≈1.333
    (0.1, "1/sigma-phi"),  // 1/(σ-φ)=0.1
    (0.5, "1/phi"),        // 1/φ=0.5
    (0.333, "1/n/phi"),    // 1/3
    (0.167, "1/n"),        // 1/6
    (144.0, "sigma2"),     // σ²=144
    (120.0, "sigma*sigma-phi"), // σ(σ-φ)=120
];

impl Lens for HypothesisGenLens {
    fn name(&self) -> &str {
        "HypothesisGenLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d < 1 {
            return HashMap::new();
        }

        let mut exact_matches = 0u32;
        let mut close_matches = 0u32;
        let mut weak_matches = 0u32;
        let mut best_deviation = f64::MAX;
        let mut best_constant_idx = 0usize;
        let mut all_features: Vec<f64> = Vec::new();

        // Feature 1: dimension means
        for dim in 0..d.min(24) {
            let col: Vec<f64> = (0..n).map(|r| data[r * d + dim]).collect();
            let mean = col.iter().sum::<f64>() / n as f64;
            all_features.push(mean);

            // Feature 2: dimension std
            let variance = col.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
            let std = variance.sqrt();
            if std > 1e-12 {
                all_features.push(std);
            }
        }

        // Feature 3: inter-dimension ratios (top pairs by variance)
        for di in 0..d.min(12) {
            for dj in (di + 1)..d.min(12) {
                let mean_i: f64 = (0..n).map(|r| data[r * d + di]).sum::<f64>() / n as f64;
                let mean_j: f64 = (0..n).map(|r| data[r * d + dj]).sum::<f64>() / n as f64;
                if mean_j.abs() > 1e-12 {
                    all_features.push((mean_i / mean_j).abs());
                }
                if mean_i.abs() > 1e-12 {
                    all_features.push((mean_j / mean_i).abs());
                }
            }
        }

        // Feature 4: global distance statistics from SharedData
        let pair_count = n * (n - 1) / 2;
        if pair_count > 0 {
            let mut dists: Vec<f64> = Vec::with_capacity(pair_count);
            for i in 0..n {
                for j in (i + 1)..n {
                    dists.push(shared.dist(i, j));
                }
            }
            let mean_dist = dists.iter().sum::<f64>() / dists.len() as f64;
            all_features.push(mean_dist);
            let max_dist = dists.iter().cloned().fold(0.0_f64, f64::max);
            let min_dist = dists.iter().cloned().fold(f64::MAX, f64::min);
            if min_dist > 1e-12 {
                all_features.push(max_dist / min_dist);
            }
        }

        // Feature 5: number of data points and dimensions as features
        all_features.push(n as f64);
        all_features.push(d as f64);
        if d > 0 {
            all_features.push(n as f64 / d as f64);
        }

        // Match each feature against n=6 constants
        for &feat in &all_features {
            if !feat.is_finite() || feat.abs() < 1e-15 {
                continue;
            }
            for (idx, &(constant, _name)) in N6_CONSTANTS.iter().enumerate() {
                if constant.abs() < 1e-15 {
                    continue;
                }
                let deviation = ((feat - constant) / constant).abs();
                if deviation < 0.01 {
                    exact_matches += 1;
                } else if deviation < 0.05 {
                    close_matches += 1;
                } else if deviation < 0.10 {
                    weak_matches += 1;
                }
                if deviation < best_deviation {
                    best_deviation = deviation;
                    best_constant_idx = idx;
                }
            }
        }

        let total_candidates = exact_matches + close_matches + weak_matches;
        let total_features = all_features.len().max(1) as f64;
        let hypothesis_density = total_candidates as f64 / total_features;

        let mut result = HashMap::new();
        result.insert("hypothesis_candidates".into(), vec![total_candidates as f64]);
        result.insert("hypothesis_density".into(), vec![hypothesis_density]);
        result.insert("exact_matches".into(), vec![exact_matches as f64]);
        result.insert("close_matches".into(), vec![close_matches as f64]);
        result.insert("weak_matches".into(), vec![weak_matches as f64]);
        result.insert("best_match_constant".into(), vec![best_constant_idx as f64]);
        result.insert("best_match_deviation".into(), vec![best_deviation]);
        result.insert("total_features_scanned".into(), vec![total_features]);
        result
    }
}
