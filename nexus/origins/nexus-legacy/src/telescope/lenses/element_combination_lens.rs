use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// ElementCombinationLens: Chemical combination pattern detection.
///
/// Measures how features combine: affinity between pairs, energy of
/// combination, stoichiometric (integer ratio) tendencies, and
/// catalyst-like features that enable others without changing.
pub struct ElementCombinationLens;

impl Lens for ElementCombinationLens {
    fn name(&self) -> &str { "ElementCombinationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        if d < 2 { return HashMap::new(); }

        let cols = column_vectors(data, max_n, d);

        // combination_affinity: mean absolute correlation between feature pairs
        let mut affinity_sum = 0.0_f64;
        let mut pair_count = 0u32;
        for di in 0..d {
            for dj in (di + 1)..d {
                let mi = shared.mi(di, dj);
                affinity_sum += mi;
                pair_count += 1;
            }
        }
        let combination_affinity = if pair_count > 0 {
            affinity_sum / pair_count as f64
        } else { 0.0 };

        // reaction_energy: mean change in variance when features are combined (sum)
        let mut energy_sum = 0.0_f64;
        let mut energy_count = 0u32;
        for di in 0..d.min(8) {
            for dj in (di + 1)..d.min(8) {
                let var_i: f64 = cols[di].iter().map(|&v| v * v).sum::<f64>() / max_n as f64;
                let var_j: f64 = cols[dj].iter().map(|&v| v * v).sum::<f64>() / max_n as f64;
                let combined: Vec<f64> = (0..max_n).map(|k| cols[di][k] + cols[dj][k]).collect();
                let var_c: f64 = combined.iter().map(|&v| v * v).sum::<f64>() / max_n as f64;
                let delta = var_c - (var_i + var_j);
                energy_sum += delta;
                energy_count += 1;
            }
        }
        let reaction_energy = if energy_count > 0 {
            energy_sum / energy_count as f64
        } else { 0.0 };

        // stoichiometry_score: how close feature ratios are to small integers
        let mut stoich_sum = 0.0_f64;
        let mut stoich_count = 0u32;
        for di in 0..d.min(8) {
            for dj in (di + 1)..d.min(8) {
                for k in 0..max_n {
                    let denom = cols[dj][k].abs();
                    if denom > 1e-8 {
                        let ratio = cols[di][k] / cols[dj][k];
                        let rounded = ratio.round();
                        if rounded.abs() <= 6.0 && rounded.abs() >= 0.5 {
                            stoich_sum += 1.0 - (ratio - rounded).abs().min(1.0);
                            stoich_count += 1;
                        }
                    }
                }
            }
        }
        let stoichiometry_score = if stoich_count > 0 {
            stoich_sum / stoich_count as f64
        } else { 0.0 };

        // catalyst_fraction: features with high MI to others but low self-variance
        let col_vars: Vec<f64> = cols.iter().map(|col| {
            let m = col.iter().sum::<f64>() / col.len() as f64;
            col.iter().map(|&v| (v - m).powi(2)).sum::<f64>() / col.len() as f64
        }).collect();
        let max_var = col_vars.iter().cloned().fold(0.0_f64, f64::max).max(1e-12);

        let mut catalyst_count = 0u32;
        for di in 0..d {
            let norm_var = col_vars[di] / max_var;
            let mean_mi: f64 = (0..d).filter(|&dj| dj != di)
                .map(|dj| shared.mi(di, dj))
                .sum::<f64>() / (d - 1).max(1) as f64;
            // Low self-variance but high coupling to others
            if norm_var < 0.3 && mean_mi > combination_affinity {
                catalyst_count += 1;
            }
        }
        let catalyst_fraction = catalyst_count as f64 / d as f64;

        let mut result = HashMap::new();
        result.insert("combination_affinity".to_string(), vec![combination_affinity]);
        result.insert("reaction_energy".to_string(), vec![reaction_energy]);
        result.insert("stoichiometry_score".to_string(), vec![stoichiometry_score]);
        result.insert("catalyst_fraction".to_string(), vec![catalyst_fraction]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..60).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 3);
        let result = ElementCombinationLens.scan(&data, 20, 3, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("combination_affinity"));
        assert!(result.contains_key("stoichiometry_score"));
    }

    #[test]
    fn test_single_dim_guard() {
        let data: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 10, 1);
        let result = ElementCombinationLens.scan(&data, 10, 1, &shared);
        assert!(result.is_empty());
    }
}
