use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// Recursive infinite discovery — each pass reveals new patterns to explore.
pub struct InfiniteDiscoveryLens;

impl Lens for InfiniteDiscoveryLens {
    fn name(&self) -> &str { "InfiniteDiscoveryLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Recursive discovery: iteratively partition and find new patterns
        let max_depth = 6_usize; // n=6 recursion limit
        let mut discoveries_per_level = Vec::with_capacity(max_depth);
        let mut active_set: Vec<usize> = (0..max_n).collect();
        let mut total_discoveries = 0usize;
        let mut saturation_depth = max_depth;

        for depth in 0..max_depth {
            if active_set.len() < 3 {
                saturation_depth = depth;
                break;
            }

            // At each depth: compute density spread within active set
            let densities: Vec<f64> = active_set.iter()
                .map(|&i| shared.knn_density(i))
                .collect();
            let ent = shannon_entropy(&densities, densities.len().max(2).min(24));

            // "Discovery" = points whose density deviates significantly from mean
            let mean_d = densities.iter().sum::<f64>() / densities.len() as f64;
            let std_d = (densities.iter().map(|&x| (x - mean_d) * (x - mean_d)).sum::<f64>()
                / densities.len() as f64).sqrt().max(1e-12);

            let mut outliers = Vec::new();
            let mut normals = Vec::new();
            for (idx, &den) in densities.iter().enumerate() {
                if (den - mean_d).abs() > std_d * 1.5 {
                    outliers.push(active_set[idx]);
                } else {
                    normals.push(active_set[idx]);
                }
            }

            let found = outliers.len();
            discoveries_per_level.push(found as f64);
            total_discoveries += found;

            if found == 0 {
                saturation_depth = depth;
                break;
            }

            // Next level: explore the outliers further
            active_set = if outliers.len() >= 3 { outliers } else { normals };
        }

        // Pad to max_depth
        while discoveries_per_level.len() < max_depth {
            discoveries_per_level.push(0.0);
        }

        // Fractal discovery dimension: self-similarity of discovery counts
        let fractal_dim = if discoveries_per_level.len() >= 2 {
            let non_zero: Vec<f64> = discoveries_per_level.iter()
                .copied().filter(|&x| x > 0.0).collect();
            if non_zero.len() >= 2 {
                let first = non_zero[0].max(1.0);
                let last = non_zero.last().copied().unwrap_or(1.0).max(1.0);
                (first / last).ln() / (non_zero.len() as f64).ln()
            } else { 0.0 }
        } else { 0.0 };

        // Mean discoveries per level (for non-zero levels)
        let active_levels = discoveries_per_level.iter().filter(|&&x| x > 0.0).count().max(1);
        let mean_per_level = total_discoveries as f64 / active_levels as f64;

        result.insert("discovery_depth".into(), vec![saturation_depth as f64]);
        result.insert("discoveries_per_level".into(), discoveries_per_level);
        result.insert("saturation_point".into(), vec![saturation_depth as f64]);
        result.insert("fractal_discovery_dimension".into(), vec![fractal_dim]);
        result.insert("total_discoveries".into(), vec![total_discoveries as f64]);
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
        let r = InfiniteDiscoveryLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("discovery_depth"));
        assert!(r.contains_key("total_discoveries"));
    }
}
