use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BoundaryLens: density gradient boundary detection.
///
/// Algorithm:
///   1. Compute k-NN density for each point
///   2. Compute density gradient magnitude between neighbors
///   3. Points with high gradient = boundary regions
///   4. Reports boundary point count and mean gradient magnitude
pub struct BoundaryLens;

impl Lens for BoundaryLens {
    fn name(&self) -> &str {
        "BoundaryLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        let k = ((n as f64).sqrt().ceil() as usize).max(2).min(n - 1);

        // Compute k-NN density
        let densities: Vec<f64> = (0..n)
            .map(|i| {
                let mut dists: Vec<f64> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| shared.dist(i, j))
                    .collect();
                dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let knn_dist = dists[k.min(dists.len()) - 1];
                if knn_dist > 0.0 {
                    1.0 / knn_dist
                } else {
                    f64::MAX
                }
            })
            .collect();

        // Density gradient for each point: max |density_i - density_j| / dist(i,j) among k-NN
        let mut gradients: Vec<f64> = Vec::with_capacity(n);
        for i in 0..n {
            let mut neighbors: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, shared.dist(i, j)))
                .collect();
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            let max_grad = neighbors
                .iter()
                .take(k)
                .map(|&(j, d)| {
                    if d > 1e-15 {
                        (densities[i] - densities[j]).abs() / d
                    } else {
                        0.0
                    }
                })
                .fold(0.0_f64, f64::max);

            gradients.push(max_grad);
        }

        // Boundary threshold: points with gradient > mean + 1 std
        let mean_grad = gradients.iter().sum::<f64>() / n as f64;
        let std_grad = (gradients
            .iter()
            .map(|g| (g - mean_grad).powi(2))
            .sum::<f64>()
            / n as f64)
            .sqrt();

        let threshold = mean_grad + std_grad;
        let boundary_count = gradients.iter().filter(|&&g| g > threshold).count();

        let mut result = HashMap::new();
        result.insert("boundary_point_count".to_string(), vec![boundary_count as f64]);
        result.insert("mean_density_gradient".to_string(), vec![mean_grad]);
        result
    }
}
