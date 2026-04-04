use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MutationLens: Sudden mutation/change detection.
///
/// Scans sequential data for abrupt changes, measuring mutation count,
/// magnitude, rate, beneficial fraction, and reversion rate.
pub struct MutationLens;

impl Lens for MutationLens {
    fn name(&self) -> &str { "MutationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Compute sequential distances between consecutive points
        let mut dists: Vec<f64> = Vec::with_capacity(max_n - 1);
        for i in 0..(max_n - 1) {
            dists.push(shared.dist(i, i + 1));
        }

        if dists.is_empty() { return HashMap::new(); }

        let mean_dist = dists.iter().sum::<f64>() / dists.len() as f64;
        let std_dist = (dists.iter().map(|&d| (d - mean_dist).powi(2)).sum::<f64>()
            / dists.len() as f64).sqrt();
        let threshold = mean_dist + 1.5 * std_dist;

        // Detect mutations: abrupt changes exceeding threshold
        let mut mutation_indices: Vec<usize> = Vec::new();
        let mut mutation_magnitudes: Vec<f64> = Vec::new();
        for (i, &dist) in dists.iter().enumerate() {
            if dist > threshold {
                mutation_indices.push(i);
                mutation_magnitudes.push(dist);
            }
        }

        let mutation_count = mutation_indices.len() as f64;
        let mutation_magnitude = if !mutation_magnitudes.is_empty() {
            mutation_magnitudes.iter().sum::<f64>() / mutation_magnitudes.len() as f64
        } else { 0.0 };
        let mutation_rate = mutation_count / (max_n - 1) as f64;

        // Beneficial: mutation leads to higher local density (closer neighbors)
        let mut beneficial = 0u32;
        for &idx in &mutation_indices {
            if idx + 1 < max_n {
                let density_before = shared.knn_density(idx);
                let density_after = shared.knn_density(idx + 1);
                if density_after > density_before {
                    beneficial += 1;
                }
            }
        }
        let beneficial_fraction = if mutation_count > 0.0 {
            beneficial as f64 / mutation_count
        } else { 0.0 };

        // Reversion: mutation followed by return to similar distance range
        let mut reversions = 0u32;
        for &idx in &mutation_indices {
            if idx + 1 < dists.len() {
                let next_dist = dists[idx + 1];
                if next_dist < mean_dist + 0.5 * std_dist {
                    reversions += 1;
                }
            }
        }
        let reversion_rate = if mutation_count > 0.0 {
            reversions as f64 / mutation_count
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("mutation_count".to_string(), vec![mutation_count]);
        result.insert("mutation_magnitude".to_string(), vec![mutation_magnitude]);
        result.insert("mutation_rate".to_string(), vec![mutation_rate]);
        result.insert("beneficial_fraction".to_string(), vec![beneficial_fraction]);
        result.insert("reversion_rate".to_string(), vec![reversion_rate]);
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
        let result = MutationLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("mutation_count"));
        assert!(result.contains_key("mutation_rate"));
    }

    #[test]
    fn test_with_spike() {
        let mut data: Vec<f64> = (0..20).map(|i| (i as f64 * 0.05)).collect();
        // Inject a spike at point 5
        data[10] = 100.0;
        data[11] = 100.0;
        let shared = SharedData::compute(&data, 10, 2);
        let result = MutationLens.scan(&data, 10, 2, &shared);
        assert!(result["mutation_count"][0] >= 1.0);
    }
}
