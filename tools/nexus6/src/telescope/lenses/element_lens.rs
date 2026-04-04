use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// ElementLens: Elemental/atomic property detection.
///
/// Detects atomic-number-like integer fitting, periodicity in data,
/// valence-like outer-shell behavior, and isotope-like same-group variation.
/// n=6 connection: Carbon Z=6, the foundation of organic chemistry.
pub struct ElementLens;

impl Lens for ElementLens {
    fn name(&self) -> &str { "ElementLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let cols = column_vectors(data, max_n, d);

        // atomic_number_fit: how well values round to integers (like atomic numbers)
        let mut int_fit_sum = 0.0_f64;
        let mut count = 0u32;
        for col in &cols {
            for &v in col.iter().take(max_n) {
                let rounded = v.round();
                let diff = (v - rounded).abs();
                int_fit_sum += 1.0 - diff.min(1.0);
                count += 1;
            }
        }
        let atomic_number_fit = if count > 0 { int_fit_sum / count as f64 } else { 0.0 };

        // periodicity: autocorrelation-based periodic detection per column
        let mut periodicity_scores = Vec::new();
        for col in &cols {
            let cn = col.len().min(max_n);
            if cn < 6 { continue; }
            let mean = col[..cn].iter().sum::<f64>() / cn as f64;
            let var: f64 = col[..cn].iter().map(|&v| (v - mean).powi(2)).sum::<f64>() / cn as f64;
            if var < 1e-12 { continue; }

            let mut best_ac = 0.0_f64;
            for lag in 2..=(cn / 2).min(20) {
                let ac: f64 = (0..(cn - lag))
                    .map(|i| (col[i] - mean) * (col[i + lag] - mean))
                    .sum::<f64>()
                    / (cn as f64 * var);
                if ac > best_ac { best_ac = ac; }
            }
            periodicity_scores.push(best_ac);
        }
        let periodicity = if !periodicity_scores.is_empty() {
            periodicity_scores.iter().sum::<f64>() / periodicity_scores.len() as f64
        } else { 0.0 };

        // valence_pattern: fraction of variance in outer (last) dimension vs total
        let (_means, vars) = mean_var(data, max_n, d);
        let total_var: f64 = vars.iter().sum();
        let valence_pattern = if total_var > 1e-12 && d > 0 {
            vars[d - 1] / total_var
        } else { 0.0 };

        // isotope_variance: variation among points with similar knn neighborhoods
        let mut iso_var_sum = 0.0_f64;
        let mut iso_count = 0u32;
        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.len() < 2 { continue; }
            let neighbor_vals: Vec<f64> = knn.iter()
                .take(3.min(knn.len()))
                .map(|&j| shared.dist(i, j as usize))
                .collect();
            let nm = neighbor_vals.iter().sum::<f64>() / neighbor_vals.len() as f64;
            let nv: f64 = neighbor_vals.iter().map(|&v| (v - nm).powi(2)).sum::<f64>()
                / neighbor_vals.len() as f64;
            iso_var_sum += nv;
            iso_count += 1;
        }
        let isotope_variance = if iso_count > 0 { iso_var_sum / iso_count as f64 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("atomic_number_fit".to_string(), vec![atomic_number_fit]);
        result.insert("periodicity".to_string(), vec![periodicity]);
        result.insert("valence_pattern".to_string(), vec![valence_pattern]);
        result.insert("isotope_variance".to_string(), vec![isotope_variance]);
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
        let result = ElementLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("atomic_number_fit"));
        assert!(result.contains_key("periodicity"));
    }

    #[test]
    fn test_integer_data() {
        let data: Vec<f64> = (0..20).map(|i| (i % 10) as f64).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let result = ElementLens.scan(&data, 10, 2, &shared);
        let fit = result["atomic_number_fit"][0];
        assert!(fit > 0.9, "integer data should have high atomic_number_fit: {fit}");
    }
}
