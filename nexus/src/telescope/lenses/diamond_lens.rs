use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DiamondLens: Diamond lattice/crystal structure detection.
/// Measures tetrahedral coordination, hardness, brilliance, lattice regularity.
/// n=6: Carbon Z=6, diamond = sp3 hybridization.
pub struct DiamondLens;

impl Lens for DiamondLens {
    fn name(&self) -> &str { "DiamondLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Tetrahedral score: in sp3, each atom has 4 nearest neighbors at equal angles
        // Check if kNN distances to first 4 neighbors are similar
        let k = shared.knn_k.min(4);
        let mut tetrahedral_scores = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.len() < k { continue; }
            let dists: Vec<f64> = (0..k).map(|ki| {
                let j = knn[ki] as usize;
                if j < max_n && j != i { shared.dist(i, j) } else { 0.0 }
            }).collect();
            let mean_d = dists.iter().sum::<f64>() / k as f64;
            if mean_d > 1e-12 {
                let var = dists.iter().map(|&dd| ((dd - mean_d) / mean_d).powi(2)).sum::<f64>() / k as f64;
                tetrahedral_scores.push(1.0 / (1.0 + var));
            }
        }
        let tetrahedral_score = if !tetrahedral_scores.is_empty() {
            tetrahedral_scores.iter().sum::<f64>() / tetrahedral_scores.len() as f64
        } else {
            0.0
        };

        // Hardness index: resistance to deformation = low variance in nearest-neighbor distances
        let mut all_nn_dists = Vec::new();
        for i in 0..max_n {
            let knn = shared.knn(i);
            if !knn.is_empty() {
                let j = knn[0] as usize;
                if j < max_n && j != i { all_nn_dists.push(shared.dist(i, j)); }
            }
        }
        let hardness_index = if all_nn_dists.len() >= 2 {
            let mean = all_nn_dists.iter().sum::<f64>() / all_nn_dists.len() as f64;
            let var = all_nn_dists.iter().map(|&dd| (dd - mean).powi(2)).sum::<f64>() / all_nn_dists.len() as f64;
            let cv = if mean > 1e-12 { var.sqrt() / mean } else { 1.0 };
            1.0 / (1.0 + cv)
        } else {
            0.0
        };

        // Brilliance: internal reflection score = how much energy bounces within the structure
        // Proxy: average ratio of second-neighbor distance to first-neighbor distance
        let mut brilliance_vals = Vec::new();
        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.len() >= 2 {
                let d1 = { let j = knn[0] as usize; if j < max_n && j != i { shared.dist(i, j) } else { continue; } };
                let d2 = { let j = knn[1] as usize; if j < max_n && j != i { shared.dist(i, j) } else { continue; } };
                if d1 > 1e-12 { brilliance_vals.push(d2 / d1); }
            }
        }
        let brilliance = if !brilliance_vals.is_empty() {
            let mean_ratio = brilliance_vals.iter().sum::<f64>() / brilliance_vals.len() as f64;
            1.0 / (1.0 + (mean_ratio - 1.0).abs()) // closer to 1 = more regular = more brilliant
        } else {
            0.0
        };

        // Lattice regularity: how regular the spacing is
        // Coefficient of variation of all nearest-neighbor distances
        let lattice_regularity = hardness_index; // same metric applies

        let mut result = HashMap::new();
        result.insert("tetrahedral_score".to_string(), vec![tetrahedral_score]);
        result.insert("hardness_index".to_string(), vec![hardness_index]);
        result.insert("brilliance".to_string(), vec![brilliance]);
        result.insert("lattice_regularity".to_string(), vec![lattice_regularity]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        // Regular grid should show high lattice regularity
        let data: Vec<f64> = (0..20).map(|i| i as f64 * 1.0).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = DiamondLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["hardness_index"][0] > 0.0);
    }
}
