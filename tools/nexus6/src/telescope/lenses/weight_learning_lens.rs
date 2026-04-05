use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, shannon_entropy};

/// Learns importance weights for features and n=6 constants.
pub struct WeightLearningLens;

/// N=6 fundamental constants for matching.
const N6_CONSTANTS: [(f64, &str); 7] = [
    (12.0, "sigma"),    // sigma(6) = 12
    (4.0, "tau"),       // tau(6) = 4
    (2.0, "phi"),       // phi(6) = 2
    (24.0, "J2"),       // J_2(6) = 24
    (6.0, "n"),         // n = 6
    (5.0, "sopfr"),     // sopfr(6) = 5
    (1.0, "mu"),        // |mu(6)| = 1
];

impl Lens for WeightLearningLens {
    fn name(&self) -> &str { "WeightLearningLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Feature weights: variance-based importance ranking
        let (means, vars) = mean_var(data, max_n, d);
        let total_var: f64 = vars.iter().sum::<f64>().max(1e-12);
        let feature_weights: Vec<f64> = vars.iter().map(|&v| v / total_var).collect();

        // Constant matching: check ratios in data against each n=6 constant
        // Collect meaningful ratios from data statistics
        let mut ratio_pool = Vec::with_capacity(d * 4);
        for j in 0..d {
            if means[j].abs() > 1e-12 {
                ratio_pool.push(means[j].abs());
            }
            if vars[j] > 1e-12 {
                ratio_pool.push(vars[j].sqrt());
            }
        }
        // Add inter-feature ratios
        for j1 in 0..d.min(6) {
            for j2 in (j1 + 1)..d.min(6) {
                if means[j2].abs() > 1e-12 {
                    ratio_pool.push((means[j1] / means[j2]).abs());
                }
            }
        }
        // Add distance-based statistics
        let densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
        let mean_density = densities.iter().sum::<f64>() / max_n as f64;
        if mean_density > 1e-12 { ratio_pool.push(mean_density); }
        ratio_pool.push(max_n as f64);
        ratio_pool.push(d as f64);

        // Score each constant: how well it matches ratios in the data
        let mut constant_weights = vec![0.0_f64; N6_CONSTANTS.len()];
        for &(cval, _) in &N6_CONSTANTS {
            let idx = N6_CONSTANTS.iter().position(|&(v, _)| v == cval).unwrap();
            let mut match_score = 0.0;
            let mut count = 0;
            for &r in &ratio_pool {
                if r < 1e-12 { continue; }
                // Check if ratio or its reciprocal matches the constant
                let ratio = r / cval;
                let log_ratio = ratio.ln().abs();
                // Perfect match when log_ratio ~ 0 or ~ ln(integer)
                let proximity = (-log_ratio * log_ratio).exp(); // Gaussian kernel
                match_score += proximity;
                count += 1;

                // Also check integer multiples/fractions
                let rounded = ratio.round();
                if rounded > 0.0 {
                    let frac_err = (ratio - rounded).abs() / rounded;
                    if frac_err < 0.1 {
                        match_score += 1.0 - frac_err;
                    }
                }
            }
            constant_weights[idx] = if count > 0 { match_score / count as f64 } else { 0.0 };
        }

        // Normalize constant weights
        let cw_sum: f64 = constant_weights.iter().sum::<f64>().max(1e-12);
        for w in &mut constant_weights { *w /= cw_sum; }

        // Top constant index
        let top_idx = constant_weights.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i).unwrap_or(0);

        // Estimated optimal learning rate: based on data scale and curvature
        let scale = total_var.sqrt();
        let learning_rate = 1.0 / (scale * max_n as f64).max(1.0);

        // Weight entropy: how uniform the feature weights are
        let weight_entropy = shannon_entropy(&feature_weights, d.max(2).min(32));

        result.insert("feature_weights".into(), feature_weights);
        result.insert("constant_weights".into(), constant_weights);
        result.insert("top_constant".into(), vec![top_idx as f64]);
        result.insert("learning_rate".into(), vec![learning_rate]);
        result.insert("weight_entropy".into(), vec![weight_entropy]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = WeightLearningLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("feature_weights"));
        assert!(r.contains_key("constant_weights"));
        assert_eq!(r["feature_weights"].len(), 2); // d=2
        assert_eq!(r["constant_weights"].len(), 7); // 7 n=6 constants
    }
}
