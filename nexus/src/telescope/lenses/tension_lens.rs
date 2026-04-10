use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TensionLens: Internal stress / tension field detection.
///
/// Detects tension in data manifold — regions under stress where
/// opposing forces pull in different directions. Analogous to
/// surface tension, string tension, or stress-energy tensor.
///
/// Metrics:
///   1. tension_field: per-point tension magnitude (pull from neighbors)
///   2. mean_tension: average tension across the manifold
///   3. tension_anisotropy: directional imbalance of forces
///   4. rupture_risk: fraction of points near breaking threshold
///   5. surface_tension: boundary tension at cluster interfaces
///
/// n=6: String theory tension T = 1/(2πα'), with α' in Planck units.
///       Surface tension σ(6)=12 in CN=6 crystal structures.
///       Stress-energy tensor T_μν has (σ-μ)(σ-μ+1)/2 = 78 components in 11D.
pub struct TensionLens;

impl Lens for TensionLens {
    fn name(&self) -> &str { "TensionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }

        let max_n = n.min(200);
        let mut tension_per_point = Vec::with_capacity(max_n);
        let mut anisotropy_per_point = Vec::with_capacity(max_n);

        for i in 0..max_n {
            let knn = shared.knn(i);
            if knn.is_empty() { continue; }

            // Force vector: sum of unit vectors from i to each neighbor,
            // weighted by distance (farther = more tension)
            let mut force = vec![0.0f64; d];
            for &j in knn.iter() {
                let j = j as usize;
                if j >= n { continue; }
                let dist = shared.dist(i, j);
                if dist < 1e-15 { continue; }
                for dim in 0..d {
                    let diff = data[j * d + dim] - data[i * d + dim];
                    force[dim] += diff; // unnormalized: magnitude encodes tension
                }
            }

            // Tension = magnitude of net force (high if pulled in one direction)
            let tension: f64 = force.iter().map(|f| f * f).sum::<f64>().sqrt()
                / knn.len() as f64;
            tension_per_point.push(tension);

            // Anisotropy: max component / total magnitude
            let max_comp = force.iter().map(|f| f.abs()).fold(0.0f64, f64::max);
            let total = force.iter().map(|f| f.abs()).sum::<f64>();
            let aniso = if total > 1e-15 { max_comp / total } else { 1.0 / d as f64 };
            anisotropy_per_point.push(aniso);
        }

        if tension_per_point.is_empty() { return HashMap::new(); }

        let nf = tension_per_point.len() as f64;
        let mean_tension = tension_per_point.iter().sum::<f64>() / nf;
        let mean_anisotropy = anisotropy_per_point.iter().sum::<f64>() / nf;

        // Rupture risk: points with tension > 2x mean
        let rupture_threshold = mean_tension * 2.0;
        let rupture_risk = tension_per_point.iter()
            .filter(|&&t| t > rupture_threshold)
            .count() as f64 / nf;

        // Surface tension: tension at boundary points (high density gradient)
        let mut surface_tension = 0.0f64;
        let mut boundary_count = 0u32;
        for i in 0..max_n.min(tension_per_point.len()) {
            let density_i = shared.knn_density(i);
            let knn = shared.knn(i);
            let mean_neighbor_density: f64 = knn.iter()
                .map(|&j| shared.knn_density(j as usize))
                .sum::<f64>() / knn.len().max(1) as f64;
            // Boundary: density gradient is high
            let gradient = (density_i - mean_neighbor_density).abs();
            if gradient > density_i * 0.5 {
                surface_tension += tension_per_point[i];
                boundary_count += 1;
            }
        }
        let surface_tension = if boundary_count > 0 {
            surface_tension / boundary_count as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("tension_field".to_string(), tension_per_point);
        result.insert("mean_tension".to_string(), vec![mean_tension]);
        result.insert("tension_anisotropy".to_string(), vec![mean_anisotropy]);
        result.insert("rupture_risk".to_string(), vec![rupture_risk]);
        result.insert("surface_tension".to_string(), vec![surface_tension]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tension_lens_two_clusters() {
        // Two clusters = tension at interface
        let mut data = Vec::new();
        for i in 0..15 { data.push(i as f64 * 0.1); data.push(0.0); }
        for i in 0..15 { data.push(10.0 + i as f64 * 0.1); data.push(0.0); }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = TensionLens.scan(&data, n, 2, &shared);
        assert!(result.contains_key("mean_tension"));
        assert!(result["mean_tension"][0] > 0.0);
    }
}
