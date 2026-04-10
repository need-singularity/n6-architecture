use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConstantCollectorLens: Detect known mathematical/physical constants in data.
/// Checks ratios and values against pi, e, phi, sqrt(2), sqrt(3), ln2, 1/137,
/// and n=6 constants (sigma=12, tau=4, J2=24, etc).
pub struct ConstantCollectorLens;

struct KnownConstant {
    name: &'static str,
    value: f64,
    is_n6: bool,
}

const CONSTANTS: &[KnownConstant] = &[
    KnownConstant { name: "pi", value: std::f64::consts::PI, is_n6: false },
    KnownConstant { name: "e", value: std::f64::consts::E, is_n6: false },
    KnownConstant { name: "phi_golden", value: 1.618033988749895, is_n6: false },
    KnownConstant { name: "sqrt2", value: std::f64::consts::SQRT_2, is_n6: false },
    KnownConstant { name: "sqrt3", value: 1.7320508075688772, is_n6: false },
    KnownConstant { name: "ln2", value: std::f64::consts::LN_2, is_n6: false },
    KnownConstant { name: "fine_structure_inv", value: 137.036, is_n6: false },
    KnownConstant { name: "fine_structure", value: 0.007297353, is_n6: false },
    // n=6 constants
    KnownConstant { name: "n", value: 6.0, is_n6: true },
    KnownConstant { name: "sigma", value: 12.0, is_n6: true },
    KnownConstant { name: "tau", value: 4.0, is_n6: true },
    KnownConstant { name: "phi_euler", value: 2.0, is_n6: true },
    KnownConstant { name: "J2", value: 24.0, is_n6: true },
    KnownConstant { name: "sopfr", value: 5.0, is_n6: true },
    KnownConstant { name: "sigma_minus_phi", value: 10.0, is_n6: true },
    KnownConstant { name: "sigma_minus_tau", value: 8.0, is_n6: true },
    KnownConstant { name: "sigma_sq", value: 144.0, is_n6: true },
    KnownConstant { name: "sigma_times_J2", value: 288.0, is_n6: true },
    KnownConstant { name: "ln_4_3", value: 0.28768207245178085, is_n6: true },
    KnownConstant { name: "tau_sq_over_sigma", value: 1.3333333333333333, is_n6: true }, // 4/3
];

const MATCH_TOL: f64 = 0.02; // 2% tolerance

impl Lens for ConstantCollectorLens {
    fn name(&self) -> &str { "ConstantCollectorLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let mut matches = 0usize;
        let mut n6_matches = 0usize;
        let mut best_precision = 1.0f64;
        let mut best_const_idx = 0usize;
        let mut total_checked = 0usize;

        // Check individual values
        for i in 0..max_n {
            let val = data[i * d].abs();
            if val < 1e-15 { continue; }
            for (ci, c) in CONSTANTS.iter().enumerate() {
                if c.value < 1e-15 { continue; }
                let rel_err = (val - c.value).abs() / c.value;
                if rel_err < MATCH_TOL {
                    matches += 1;
                    if c.is_n6 { n6_matches += 1; }
                    if rel_err < best_precision {
                        best_precision = rel_err;
                        best_const_idx = ci;
                    }
                }
            }
            total_checked += 1;
        }

        // Check consecutive ratios
        for i in 1..max_n {
            let prev = data[(i - 1) * d].abs();
            let curr = data[i * d].abs();
            if prev < 1e-15 { continue; }
            let ratio = curr / prev;
            for (ci, c) in CONSTANTS.iter().enumerate() {
                let rel_err = (ratio - c.value).abs() / c.value.max(1e-12);
                if rel_err < MATCH_TOL {
                    matches += 1;
                    if c.is_n6 { n6_matches += 1; }
                    if rel_err < best_precision {
                        best_precision = rel_err;
                        best_const_idx = ci;
                    }
                }
            }
            total_checked += 1;
        }

        let constant_matches = matches as f64;
        let match_precision = if matches > 0 { 1.0 - best_precision } else { 0.0 };
        let n6_fraction = if matches > 0 { n6_matches as f64 / matches as f64 } else { 0.0 };
        let best_match_name = best_const_idx as f64; // index into CONSTANTS

        let mut result = HashMap::new();
        result.insert("constant_matches".to_string(), vec![constant_matches]);
        result.insert("best_match_idx".to_string(), vec![best_match_name]);
        result.insert("match_precision".to_string(), vec![match_precision]);
        result.insert("n6_fraction".to_string(), vec![n6_fraction]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_with_constants() {
        // Data containing known constants
        let data = vec![3.14159, 2.71828, 1.618, 12.0, 6.0, 24.0, 0.5, 1.0, 4.0, 8.0];
        let shared = SharedData::compute(&data, 10, 1);
        let result = ConstantCollectorLens.scan(&data, 10, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["constant_matches"][0] > 0.0);
    }
}
