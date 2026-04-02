use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// NetworkLens: graph metrics — degree distribution + clustering coefficient.
///
/// Algorithm:
///   1. Build k-NN graph (adjacency from k nearest neighbors)
///   2. Degree distribution statistics
///   3. Clustering coefficient: fraction of closed triangles per node
///   4. Reports mean degree, clustering coefficient, and degree variance
pub struct NetworkLens;

impl Lens for NetworkLens {
    fn name(&self) -> &str {
        "NetworkLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        let k = ((n as f64).sqrt().ceil() as usize).max(2).min(n - 1);

        // Build k-NN adjacency (symmetric)
        let mut adj = vec![vec![false; n]; n];
        for i in 0..n {
            let mut neighbors: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, shared.dist(i, j)))
                .collect();
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            for &(j, _) in neighbors.iter().take(k) {
                adj[i][j] = true;
                adj[j][i] = true; // symmetric
            }
        }

        // Degree of each node
        let degrees: Vec<usize> = (0..n)
            .map(|i| (0..n).filter(|&j| adj[i][j]).count())
            .collect();

        let mean_degree = degrees.iter().sum::<usize>() as f64 / n as f64;
        let degree_var = degrees
            .iter()
            .map(|&deg| {
                let d = deg as f64 - mean_degree;
                d * d
            })
            .sum::<f64>()
            / n as f64;

        // Clustering coefficient
        let mut total_cc = 0.0;
        let mut cc_count = 0;
        for i in 0..n {
            let neighbors_i: Vec<usize> = (0..n).filter(|&j| adj[i][j]).collect();
            let ki = neighbors_i.len();
            if ki < 2 {
                continue;
            }

            // Count triangles: edges among neighbors
            let mut triangles = 0;
            for a in 0..ki {
                for b in (a + 1)..ki {
                    if adj[neighbors_i[a]][neighbors_i[b]] {
                        triangles += 1;
                    }
                }
            }

            let possible = ki * (ki - 1) / 2;
            total_cc += triangles as f64 / possible as f64;
            cc_count += 1;
        }

        let clustering_coeff = if cc_count > 0 {
            total_cc / cc_count as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("mean_degree".to_string(), vec![mean_degree]);
        result.insert("clustering_coefficient".to_string(), vec![clustering_coeff]);
        result.insert("degree_variance".to_string(), vec![degree_var]);
        result
    }
}
