use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// StabilityLens: Lyapunov exponent approximation for dynamical stability.
///
/// Algorithm:
///   1. Treat row order as trajectory in d-dimensional space
///   2. For nearby point pairs, track divergence over time steps
///   3. Lyapunov exponent = mean log(divergence ratio)
///   4. Positive = chaotic, negative = stable, zero = periodic
pub struct StabilityLens;

impl Lens for StabilityLens {
    fn name(&self) -> &str {
        "StabilityLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 8 {
            return HashMap::new();
        }

        // Find pairs of initially close points (not time-adjacent)
        let min_time_sep = 3;
        let max_pairs = 50;

        let mut lyap_estimates: Vec<f64> = Vec::new();

        for i in 0..n.saturating_sub(min_time_sep + 2) {
            if lyap_estimates.len() >= max_pairs {
                break;
            }
            for j in (i + min_time_sep)..n.saturating_sub(2) {
                if lyap_estimates.len() >= max_pairs {
                    break;
                }

                let d0 = shared.dist(i, j);
                if d0 < 1e-15 {
                    continue;
                }

                // Track divergence at next time step
                let i_next = i + 1;
                let j_next = j + 1;
                if i_next >= n || j_next >= n {
                    continue;
                }

                let d1 = euclidean_dist_rows(data, d, i_next, j_next);
                if d1 > 1e-15 {
                    lyap_estimates.push((d1 / d0).ln());
                }
            }
        }

        if lyap_estimates.is_empty() {
            return HashMap::new();
        }

        let lyapunov = lyap_estimates.iter().sum::<f64>() / lyap_estimates.len() as f64;

        // Stability classification: max Lyapunov
        let max_lyap = lyap_estimates
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        let mut result = HashMap::new();
        result.insert("lyapunov_exponent".to_string(), vec![lyapunov]);
        result.insert("max_lyapunov".to_string(), vec![max_lyap]);
        result
    }
}

fn euclidean_dist_rows(data: &[f64], d: usize, i: usize, j: usize) -> f64 {
    let row_i = &data[i * d..(i + 1) * d];
    let row_j = &data[j * d..(j + 1) * d];
    row_i
        .iter()
        .zip(row_j.iter())
        .map(|(a, b)| (a - b) * (a - b))
        .sum::<f64>()
        .sqrt()
}
