use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// FormulaCombinationLens: Find simple formulas relating features.
/// Measures linear_relations, ratio_relations, power_relations, formula_complexity, r_squared.
pub struct FormulaCombinationLens;

const REL_TOL: f64 = 0.05;

impl Lens for FormulaCombinationLens {
    fn name(&self) -> &str { "FormulaCombinationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }
        let max_n = n.min(200);
        let cols = column_vectors(data, max_n, d);
        let (means, vars) = mean_var(data, max_n, d);
        let max_d = d.min(12);

        let mut linear_count = 0usize;
        let mut ratio_count = 0usize;
        let mut power_count = 0usize;
        let mut best_r2 = 0.0_f64;

        for i in 0..max_d {
            if vars[i] < 1e-12 { continue; }
            for j in (i + 1)..max_d {
                if vars[j] < 1e-12 { continue; }
                // Linear: a*xi + b ≈ xj via least-squares slope
                let slope = {
                    let mut num = 0.0;
                    let mut den = 0.0;
                    for k in 0..max_n {
                        let di = cols[i][k] - means[i];
                        let dj = cols[j][k] - means[j];
                        num += di * dj;
                        den += di * di;
                    }
                    if den.abs() > 1e-15 { num / den } else { 0.0 }
                };
                let intercept = means[j] - slope * means[i];
                let mut ss_res = 0.0;
                let mut ss_tot = 0.0;
                for k in 0..max_n {
                    let pred = slope * cols[i][k] + intercept;
                    ss_res += (cols[j][k] - pred).powi(2);
                    ss_tot += (cols[j][k] - means[j]).powi(2);
                }
                let r2 = if ss_tot > 1e-15 { 1.0 - ss_res / ss_tot } else { 0.0 };
                if r2 > 0.9 { linear_count += 1; }
                if r2 > best_r2 { best_r2 = r2; }

                // Ratio: xi/xj ≈ constant
                let ratios: Vec<f64> = (0..max_n)
                    .filter(|&k| cols[j][k].abs() > 1e-12)
                    .map(|k| cols[i][k] / cols[j][k])
                    .collect();
                if ratios.len() > 3 {
                    let rmean = ratios.iter().sum::<f64>() / ratios.len() as f64;
                    let rvar = ratios.iter().map(|r| (r - rmean).powi(2)).sum::<f64>() / ratios.len() as f64;
                    let cv = if rmean.abs() > 1e-12 { rvar.sqrt() / rmean.abs() } else { 1.0 };
                    if cv < REL_TOL { ratio_count += 1; }
                }

                // Power: xi^k ≈ xj for k in {0.5, 2, 3}
                for &k in &[0.5_f64, 2.0, 3.0] {
                    let mut pow_err = 0.0;
                    let mut pow_tot = 0.0;
                    for row in 0..max_n {
                        let xp = cols[i][row].abs().powf(k) * cols[i][row].signum();
                        pow_err += (xp - cols[j][row]).powi(2);
                        pow_tot += (cols[j][row] - means[j]).powi(2);
                    }
                    if pow_tot > 1e-15 && (1.0 - pow_err / pow_tot) > 0.85 { power_count += 1; }
                }
            }
        }

        // Formula complexity: 1=linear, 2=ratio, 3=power, 0=none
        let complexity = if linear_count > 0 { 1.0 } else if ratio_count > 0 { 2.0 }
                         else if power_count > 0 { 3.0 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("linear_relations".into(), vec![linear_count as f64]);
        result.insert("ratio_relations".into(), vec![ratio_count as f64]);
        result.insert("power_relations".into(), vec![power_count as f64]);
        result.insert("formula_complexity".into(), vec![complexity]);
        result.insert("r_squared".into(), vec![best_r2]);
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
        let result = FormulaCombinationLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
