use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DensityLens: Local and global density estimation.
///
/// Uses k-NN density estimation to detect density variations,
/// voids, and concentration patterns matching n=6 structure.
pub struct DensityLens;

impl Lens for DensityLens {
    fn name(&self) -> &str { "DensityLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 { return HashMap::new(); }

        let _k = shared.knn_k.min(n - 1).max(1);

        // Per-point density via k-NN
        let mut densities = Vec::with_capacity(n);
        for i in 0..n {
            densities.push(shared.knn_density(i));
        }

        // Global statistics
        let mean_density = densities.iter().sum::<f64>() / n as f64;
        let max_density = densities.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
        let min_density = densities.iter().cloned().fold(f64::INFINITY, f64::min);

        // Density variance (heterogeneity)
        let density_var = densities.iter()
            .map(|&d| (d - mean_density).powi(2))
            .sum::<f64>() / n as f64;
        let density_cv = if mean_density > 1e-12 { density_var.sqrt() / mean_density } else { 0.0 };

        // Density ratio: max/min (concentration factor)
        let density_ratio = if min_density > 1e-12 { max_density / min_density } else { max_density };

        // Fraction of points in dense regions (above mean)
        let dense_fraction = densities.iter().filter(|&&d| d > mean_density).count() as f64 / n as f64;

        let mut result = HashMap::new();
        result.insert("densities".to_string(), densities);
        result.insert("mean_density".to_string(), vec![mean_density]);
        result.insert("density_cv".to_string(), vec![density_cv]);
        result.insert("density_ratio".to_string(), vec![density_ratio]);
        result.insert("dense_fraction".to_string(), vec![dense_fraction]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_density_lens_uniform() {
        // Grid points: roughly uniform density
        let mut data = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                data.push(x as f64);
                data.push(y as f64);
            }
        }
        let n = 16;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = DensityLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_density"));
        assert!(result["density_cv"][0] < 2.0, "Uniform grid should have low density CV");
    }

    #[test]
    fn test_density_lens_clustered() {
        // Two tight clusters + outlier
        let mut data = vec![
            0.0, 0.0, 0.1, 0.0, 0.0, 0.1, 0.1, 0.1, // Tight cluster
            100.0, 100.0, // Far outlier
        ];
        let n = 5;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = DensityLens.scan(&data, n, d, &shared);
        assert!(result["density_ratio"][0] > 1.0, "Should detect density variation");
    }
}
