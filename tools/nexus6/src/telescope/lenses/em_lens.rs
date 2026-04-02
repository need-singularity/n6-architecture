use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// EmLens: electromagnetic analogy — gradient magnitude + divergence.
///
/// Algorithm:
///   1. Compute local gradient for each point via finite differences to nearest neighbor
///   2. Gradient magnitude = "field strength"
///   3. Divergence approximation = net outward gradient flow from k-neighbors
///   4. Reports mean gradient, divergence distribution
pub struct EmLens;

impl Lens for EmLens {
    fn name(&self) -> &str {
        "EmLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        let k = ((n as f64).sqrt().ceil() as usize).max(1).min(n - 1);

        // For each point, compute gradient vector toward k nearest neighbors
        let mut grad_magnitudes: Vec<f64> = Vec::with_capacity(n);
        let mut divergences: Vec<f64> = Vec::with_capacity(n);

        // Use data magnitude at each point as a scalar field
        let field: Vec<f64> = (0..n)
            .map(|i| {
                let row = &data[i * d..(i + 1) * d];
                row.iter().map(|x| x * x).sum::<f64>().sqrt()
            })
            .collect();

        for i in 0..n {
            // Find k nearest neighbors
            let mut neighbors: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, shared.dist(i, j)))
                .collect();
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            neighbors.truncate(k);

            // Gradient: weighted average of (field_j - field_i) * direction_ij
            let mut grad = vec![0.0; d];
            for &(j, dist) in &neighbors {
                if dist < 1e-15 {
                    continue;
                }
                let df = field[j] - field[i];
                for dim in 0..d {
                    let dir = (data[j * d + dim] - data[i * d + dim]) / dist;
                    grad[dim] += df * dir / dist;
                }
            }

            let mag = grad.iter().map(|g| g * g).sum::<f64>().sqrt();
            grad_magnitudes.push(mag);

            // Divergence: sum of (field_j - field_i) / dist for each neighbor
            let div: f64 = neighbors
                .iter()
                .map(|&(j, dist)| {
                    if dist > 1e-15 {
                        (field[j] - field[i]) / dist
                    } else {
                        0.0
                    }
                })
                .sum();
            divergences.push(div);
        }

        let mean_grad = grad_magnitudes.iter().sum::<f64>() / n as f64;
        let mean_div = divergences.iter().sum::<f64>() / n as f64;

        let mut result = HashMap::new();
        result.insert("mean_gradient".to_string(), vec![mean_grad]);
        result.insert("mean_divergence".to_string(), vec![mean_div]);
        result
    }
}
