use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SphericalLens: Detect spherical/hyperspherical structure in high-dimensional data.
///
/// n=6 connection: kissing number in 3D = σ(6)=12, sphere packing in 24D
/// (Leech lattice, J₂=24). The hexagonal close-packing is nature's optimal sphere arrangement.
pub struct SphericalLens;

impl Lens for SphericalLens {
    fn name(&self) -> &str {
        "SphericalLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 {
            return HashMap::new();
        }

        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Compute centroid and distances from centroid
        let (means, _) = crate::telescope::shared_data::mean_var(data, n, d);
        let mut radii = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let mut r2 = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - means[j];
                r2 += diff * diff;
            }
            radii.push(r2.sqrt());
        }
        let mean_r = radii.iter().sum::<f64>() / radii.len() as f64;

        // 1. Sphericity: 1 - CV(radii). Perfect sphere has CV=0 → sphericity=1.
        let var_r = radii.iter().map(|&r| (r - mean_r).powi(2)).sum::<f64>() / radii.len() as f64;
        let sphericity = if mean_r > 1e-12 {
            (1.0 - (var_r.sqrt() / mean_r)).max(0.0)
        } else {
            0.0
        };
        result.insert("sphericity".to_string(), vec![sphericity]);

        // 2. Shell fraction: fraction of points on the "surface" (outer 1/6 of radius range).
        //    For uniform ball, shell fraction in outer 1/6 by radius is small.
        //    For hollow sphere, most points are on the shell.
        let max_r = radii.iter().cloned().fold(0.0_f64, f64::max);
        if max_r > 1e-12 {
            let shell_threshold = max_r * (1.0 - 1.0 / 6.0); // outer 1/6 by radius
            let on_shell = radii.iter().filter(|&&r| r >= shell_threshold).count();
            let shell_fraction = on_shell as f64 / radii.len() as f64;
            result.insert("shell_fraction".to_string(), vec![shell_fraction]);
        }

        // 3. Radial uniformity: KS-like test — compare CDF of radii to uniform CDF.
        //    For d-dimensional sphere, radii follow r^(d-1) distribution, but
        //    we measure general uniformity as sorted rank correlation.
        let mut sorted_radii = radii.clone();
        sorted_radii.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let max_dev = sorted_radii.iter().enumerate().map(|(i, &r)| {
            let expected = if max_r > 1e-12 { r / max_r } else { 0.0 };
            let actual = (i + 1) as f64 / sorted_radii.len() as f64;
            (expected - actual).abs()
        }).fold(0.0_f64, f64::max);
        let radial_uniformity = 1.0 - max_dev.min(1.0);
        result.insert("radial_uniformity".to_string(), vec![radial_uniformity]);

        // 4. Kissing estimate: average number of near-neighbors per point.
        //    For 3D sphere packing, kissing number = 12 = σ(6).
        if n > 1 && shared.knn_k >= 1 {
            // Count neighbors within 1.1 * min_neighbor_distance (contact threshold)
            let mut total_contacts = 0usize;
            let mut point_count = 0usize;
            for i in 0..max_n {
                let knn = shared.knn(i);
                if knn.is_empty() {
                    continue;
                }
                let min_dist = shared.dist(i, knn[0] as usize);
                if min_dist < 1e-15 {
                    continue;
                }
                let threshold = min_dist * 1.2;
                let contacts = knn.iter().filter(|&&j| {
                    shared.dist(i, j as usize) <= threshold
                }).count();
                total_contacts += contacts;
                point_count += 1;
            }
            let kissing_estimate = if point_count > 0 {
                total_contacts as f64 / point_count as f64
            } else {
                0.0
            };
            result.insert("kissing_estimate".to_string(), vec![kissing_estimate]);
        }

        // 5. Dimension estimate: use ratio of volume to surface to estimate
        //    effective dimension. For d-dim sphere, V ∝ r^d, S ∝ r^(d-1).
        //    We use log(N_inner)/log(r_ratio) as a proxy.
        if max_r > 1e-12 && radii.len() >= 6 {
            let r_half = max_r * 0.5;
            let inner_count = radii.iter().filter(|&&r| r <= r_half).count();
            let total_count = radii.len();
            if inner_count > 0 && inner_count < total_count {
                let frac = inner_count as f64 / total_count as f64;
                // For uniform d-dim ball: frac = 0.5^d
                let dimension_estimate = if frac > 1e-15 {
                    -(frac.ln()) / (2.0_f64.ln())
                } else {
                    d as f64 // fallback
                };
                result.insert("dimension_estimate".to_string(), vec![dimension_estimate]);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_spherical_lens_circle() {
        // Points on a circle — high sphericity
        let n = 20;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            data.push(3.0 * angle.cos());
            data.push(3.0 * angle.sin());
        }
        let shared = SharedData::compute(&data, n, d);
        let result = SphericalLens.scan(&data, n, d, &shared);
        assert!(!result.is_empty());

        let sph = result.get("sphericity").unwrap()[0];
        assert!(sph > 0.9, "Circle points should have high sphericity, got {}", sph);

        let shell = result.get("shell_fraction").unwrap()[0];
        assert!(shell > 0.8, "Circle points should mostly be on shell, got {}", shell);
    }

    #[test]
    fn test_spherical_lens_line() {
        // Points along a line — low sphericity
        let n = 20;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            data.push(i as f64);
            data.push(0.0);
        }
        let shared = SharedData::compute(&data, n, d);
        let result = SphericalLens.scan(&data, n, d, &shared);
        assert!(!result.is_empty());

        let sph = result.get("sphericity").unwrap()[0];
        assert!(sph < 0.7, "Line points should have low sphericity, got {}", sph);
    }

    #[test]
    fn test_spherical_lens_small_n() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = SphericalLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty());
    }
}
