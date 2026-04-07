use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// MolecularCombinationLens: How features combine like atoms forming molecules.
/// Measures bond_count, bond_strength, molecular_weight, valence_saturation.
pub struct MolecularCombinationLens;

const BOND_THRESHOLD: f64 = 0.7;

impl Lens for MolecularCombinationLens {
    fn name(&self) -> &str { "MolecularCombinationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let cols = column_vectors(data, max_n, d);
        let (means, vars) = mean_var(data, max_n, d);
        let max_d = d.min(24);

        // Compute pairwise correlations to find "bonds"
        let mut bond_count = 0usize;
        let mut bond_strength_sum = 0.0_f64;
        let mut bonds_per_feature = vec![0usize; max_d];

        for i in 0..max_d {
            let si = vars[i].sqrt();
            if si < 1e-12 { continue; }
            for j in (i + 1)..max_d {
                let sj = vars[j].sqrt();
                if sj < 1e-12 { continue; }
                let mut cov = 0.0;
                for k in 0..max_n {
                    cov += (cols[i][k] - means[i]) * (cols[j][k] - means[j]);
                }
                cov /= max_n as f64;
                let r = (cov / (si * sj)).abs();
                if r > BOND_THRESHOLD {
                    bond_count += 1;
                    bond_strength_sum += r;
                    bonds_per_feature[i] += 1;
                    bonds_per_feature[j] += 1;
                }
            }
        }

        let bond_strength = if bond_count > 0 { bond_strength_sum / bond_count as f64 } else { 0.0 };

        // Molecular weight: sum of feature magnitudes for bonded features
        let molecular_weight: f64 = (0..max_d)
            .filter(|&i| bonds_per_feature[i] > 0)
            .map(|i| cols[i].iter().map(|v| v.abs()).sum::<f64>() / max_n as f64)
            .sum();

        // Valence saturation: fraction of features that have at least one bond
        let bonded_features = bonds_per_feature.iter().filter(|&&b| b > 0).count();
        let valence_saturation = if max_d > 0 { bonded_features as f64 / max_d as f64 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("bond_count".into(), vec![bond_count as f64]);
        result.insert("bond_strength".into(), vec![bond_strength]);
        result.insert("molecular_weight".into(), vec![molecular_weight]);
        result.insert("valence_saturation".into(), vec![valence_saturation]);
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
        let result = MolecularCombinationLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
