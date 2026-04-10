use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// ConstantFormulaLens: Finds formulas whose coefficients are known constants.
/// Measures formula_count, constant_coefficient_count, n6_formula_score, universality.
pub struct ConstantFormulaLens;

const N6_CONSTS: &[f64] = &[6.0, 12.0, 4.0, 24.0, 2.0, 5.0, 10.0, 8.0, 144.0, 288.0,
                              0.28768207245178085, 1.3333333333333333];
const MATH_CONSTS: &[f64] = &[std::f64::consts::PI, std::f64::consts::E,
                                1.618033988749895, std::f64::consts::SQRT_2, std::f64::consts::LN_2];
const COEFF_TOL: f64 = 0.03;

fn matches_known(v: f64) -> (bool, bool) {
    // Returns (matches_any, matches_n6)
    for &c in N6_CONSTS {
        if c.abs() > 1e-12 && ((v - c) / c).abs() < COEFF_TOL { return (true, true); }
    }
    for &c in MATH_CONSTS {
        if ((v - c) / c).abs() < COEFF_TOL { return (true, false); }
    }
    (false, false)
}

impl Lens for ConstantFormulaLens {
    fn name(&self) -> &str { "ConstantFormulaLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }
        let max_n = n.min(200);
        let cols = column_vectors(data, max_n, d);
        let (means, vars) = mean_var(data, max_n, d);
        let max_d = d.min(12);

        let mut formula_count = 0usize;
        let mut const_coeff_count = 0usize;
        let mut n6_coeff_count = 0usize;
        let mut features_covered = vec![false; max_d];

        for i in 0..max_d {
            if vars[i] < 1e-12 { continue; }
            for j in (i + 1)..max_d {
                if vars[j] < 1e-12 { continue; }
                // Linear fit: xj = slope * xi + intercept
                let mut num = 0.0;
                let mut den = 0.0;
                for k in 0..max_n {
                    num += (cols[i][k] - means[i]) * (cols[j][k] - means[j]);
                    den += (cols[i][k] - means[i]).powi(2);
                }
                let slope = if den.abs() > 1e-15 { num / den } else { continue };
                let intercept = means[j] - slope * means[i];

                let mut ss_res = 0.0;
                let mut ss_tot = 0.0;
                for k in 0..max_n {
                    ss_res += (cols[j][k] - slope * cols[i][k] - intercept).powi(2);
                    ss_tot += (cols[j][k] - means[j]).powi(2);
                }
                let r2 = if ss_tot > 1e-15 { 1.0 - ss_res / ss_tot } else { 0.0 };
                if r2 < 0.8 { continue; }

                formula_count += 1;
                features_covered[i] = true;
                features_covered[j] = true;

                let (s_any, s_n6) = matches_known(slope.abs());
                let (i_any, i_n6) = matches_known(intercept.abs());
                if s_any { const_coeff_count += 1; }
                if i_any { const_coeff_count += 1; }
                if s_n6 { n6_coeff_count += 1; }
                if i_n6 { n6_coeff_count += 1; }
            }
        }

        let universality = features_covered.iter().filter(|&&c| c).count() as f64 / max_d.max(1) as f64;
        let n6_score = if formula_count > 0 { n6_coeff_count as f64 / (formula_count * 2) as f64 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("formula_count".into(), vec![formula_count as f64]);
        result.insert("constant_coefficient_count".into(), vec![const_coeff_count as f64]);
        result.insert("n6_formula_score".into(), vec![n6_score]);
        result.insert("universality".into(), vec![universality]);
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
        let result = ConstantFormulaLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
