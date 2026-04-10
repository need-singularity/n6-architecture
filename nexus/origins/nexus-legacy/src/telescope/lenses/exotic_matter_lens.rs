use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// ExoticMatterLens: Negative energy density / anomalous state detection.
///
/// Detects exotic-matter-like signatures in data: regions of negative
/// curvature, anti-correlated dimensions, and energy-violation anomalies.
/// Analogous to exotic matter with negative energy density required
/// for Alcubierre drives and traversable wormholes.
///
/// Metrics:
///   1. negative_curvature_fraction: fraction of local regions with negative curvature
///   2. anti_correlation_strength: mean magnitude of negative dimension correlations
///   3. energy_violation_score: how far data deviates from "normal" energy conditions
///   4. exotic_fraction: overall fraction of data exhibiting exotic properties
///
/// n=6: Exotic matter violates weak energy condition (WEC). Casimir effect gives
///       negative energy at scale ~ 1/σ(6). String theory needs σ-μ=11 dimensions,
///       with n=6 compactified on Calabi-Yau manifold.
pub struct ExoticMatterLens;

impl Lens for ExoticMatterLens {
    fn name(&self) -> &str { "ExoticMatterLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(200);

        // 1. Negative local curvature detection
        // Curvature sign: if center point is farther from mean of neighbors
        // than expected, local curvature is negative (saddle/hyperbolic)
        let mut neg_curv_count = 0u32;
        let mut total_curv = 0u32;
        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.len() < 2 { continue; }
            total_curv += 1;

            // Mean distance from i to neighbors
            let mean_d: f64 = knn.iter()
                .map(|&j| shared.dist(i, j as usize))
                .sum::<f64>() / knn.len() as f64;

            // Mean inter-neighbor distance
            let mut inter_d_sum = 0.0f64;
            let mut inter_count = 0u32;
            for ki in 0..knn.len().min(6) {
                for kj in (ki + 1)..knn.len().min(6) {
                    let a = knn[ki] as usize;
                    let b = knn[kj] as usize;
                    if a != b {
                        inter_d_sum += shared.dist(a, b);
                        inter_count += 1;
                    }
                }
            }
            let mean_inter = if inter_count > 0 { inter_d_sum / inter_count as f64 } else { mean_d };

            // Negative curvature: neighbors spread out more than center-to-neighbor distance
            if mean_inter > mean_d * 1.2 {
                neg_curv_count += 1;
            }
        }
        let neg_curv_frac = if total_curv > 0 {
            neg_curv_count as f64 / total_curv as f64
        } else {
            0.0
        };

        // 2. Anti-correlation detection between dimensions
        let mut anti_corr_sum = 0.0f64;
        let mut anti_corr_count = 0u32;
        if d >= 2 {
            let columns = column_vectors(data, n, d);
            let (means, _) = mean_var(data, n, d);
            for di in 0..d {
                for dj in (di + 1)..d {
                    // Pearson correlation
                    let mut num = 0.0;
                    let mut den_a = 0.0;
                    let mut den_b = 0.0;
                    for k in 0..n {
                        let a = columns[di][k] - means[di];
                        let b = columns[dj][k] - means[dj];
                        num += a * b;
                        den_a += a * a;
                        den_b += b * b;
                    }
                    let denom = (den_a * den_b).sqrt();
                    let corr = if denom > 1e-12 { num / denom } else { 0.0 };
                    if corr < -0.1 {
                        anti_corr_sum += corr.abs();
                        anti_corr_count += 1;
                    }
                }
            }
        }
        let total_pairs = if d >= 2 { d * (d - 1) / 2 } else { 1 };
        let anti_corr_strength = anti_corr_sum / total_pairs as f64;

        // 3. Energy violation: points with anomalously low local density
        // surrounded by high-density regions (negative energy pocket)
        let mut violation_count = 0u32;
        for i in 0..max_n {
            let density_i = shared.knn_density(i);
            let knn = shared.knn(i);
            if knn.is_empty() { continue; }
            let mean_neighbor_density: f64 = knn.iter()
                .map(|&j| shared.knn_density(j as usize))
                .sum::<f64>() / knn.len() as f64;
            // "Exotic": center much less dense than surroundings
            if density_i < mean_neighbor_density * 0.3 {
                violation_count += 1;
            }
        }
        let violation_score = violation_count as f64 / max_n as f64;

        // 4. Combined exotic fraction
        let exotic_frac = (neg_curv_frac + anti_corr_strength + violation_score) / 3.0;

        let mut result = HashMap::new();
        result.insert("negative_curvature_fraction".to_string(), vec![neg_curv_frac]);
        result.insert("anti_correlation_strength".to_string(), vec![anti_corr_strength]);
        result.insert("energy_violation_score".to_string(), vec![violation_score]);
        result.insert("exotic_fraction".to_string(), vec![exotic_frac]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_exotic_matter_lens_basic() {
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(i as f64);
            data.push(-(i as f64) * 0.5); // anti-correlated
        }
        let n = 20;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = ExoticMatterLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("negative_curvature_fraction"));
        assert!(result.contains_key("anti_correlation_strength"));
        // Anti-correlated data should show non-zero anti-correlation
        assert!(result["anti_correlation_strength"][0] > 0.0);
    }
}
