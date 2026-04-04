use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConstantCombinationLens: Try arithmetic combinations of data values against known constants.
/// Measures combination_matches, best_combination, match_precision, n6_combinations.
pub struct ConstantCombinationLens;

const TOL: f64 = 0.02;

struct Target { value: f64, is_n6: bool }

const TARGETS: &[Target] = &[
    Target { value: std::f64::consts::PI, is_n6: false },
    Target { value: std::f64::consts::E, is_n6: false },
    Target { value: 1.618033988749895, is_n6: false }, // golden ratio
    Target { value: std::f64::consts::SQRT_2, is_n6: false },
    Target { value: std::f64::consts::LN_2, is_n6: false },
    Target { value: 0.36787944117144233, is_n6: false }, // 1/e
    Target { value: 0.28768207245178085, is_n6: true },  // ln(4/3)
    Target { value: 12.0, is_n6: true },  // sigma
    Target { value: 4.0, is_n6: true },   // tau
    Target { value: 24.0, is_n6: true },  // J2
    Target { value: 6.0, is_n6: true },   // n
];

fn near(v: f64, target: f64) -> f64 {
    if target.abs() < 1e-15 { return f64::MAX; }
    ((v - target) / target).abs()
}

impl Lens for ConstantCombinationLens {
    fn name(&self) -> &str { "ConstantCombinationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        // Collect representative values: row means
        let mut vals: Vec<f64> = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let row_mean = data[i * d..(i * d + d).min(data.len())].iter().sum::<f64>() / d as f64;
            if row_mean.is_finite() && row_mean.abs() > 1e-15 { vals.push(row_mean); }
        }
        let cap = vals.len().min(50);
        vals.truncate(cap);

        let mut matches = 0usize;
        let mut n6_matches = 0usize;
        let mut best_err = f64::MAX;

        for i in 0..vals.len() {
            for j in (i + 1)..vals.len() {
                let combos = [vals[i] + vals[j], vals[i] * vals[j],
                              vals[i] / vals[j], vals[j] / vals[i],
                              (vals[i] - vals[j]).abs()];
                for &c in &combos {
                    if !c.is_finite() { continue; }
                    for t in TARGETS {
                        let err = near(c, t.value);
                        if err < TOL {
                            matches += 1;
                            if t.is_n6 { n6_matches += 1; }
                        }
                        if err < best_err { best_err = err; }
                    }
                }
            }
        }

        let mut result = HashMap::new();
        result.insert("combination_matches".into(), vec![matches as f64]);
        result.insert("best_combination".into(), vec![if best_err < f64::MAX { best_err } else { 1.0 }]);
        result.insert("match_precision".into(), vec![1.0 - best_err.min(1.0)]);
        result.insert("n6_combinations".into(), vec![n6_matches as f64]);
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
        let result = ConstantCombinationLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
