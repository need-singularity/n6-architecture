use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// GraphLens: Graph-theoretic analysis of data point connectivity.
///
/// Builds a proximity graph and computes graph metrics:
/// degree distribution, clustering coefficient, connected components.
/// Checks for n=6 structure in graph topology.
pub struct GraphLens;

const N6_DEGREES: &[(f64, &str)] = &[
    (2.0, "phi=2"), (4.0, "tau=4"), (5.0, "sopfr=5"),
    (6.0, "n=6"), (8.0, "sigma-tau=8"), (12.0, "sigma=12"),
];

impl Lens for GraphLens {
    fn name(&self) -> &str { "GraphLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        let _ = data;
        if n < 4 { return HashMap::new(); }

        // Build adjacency via median distance threshold
        let mut all_dists = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let threshold = all_dists[all_dists.len() / 2]; // Median distance

        // Compute degree per node
        let mut degrees = vec![0usize; n];
        let mut adj = vec![vec![false; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if shared.dist(i, j) <= threshold {
                    adj[i][j] = true;
                    adj[j][i] = true;
                    degrees[i] += 1;
                    degrees[j] += 1;
                }
            }
        }

        let mean_degree = degrees.iter().sum::<usize>() as f64 / n as f64;

        // Match mean degree to n=6 constants
        let mut best = ("none", f64::MAX);
        for &(deg, name) in N6_DEGREES {
            let dist = (mean_degree - deg).abs();
            if dist < best.1 { best = (name, dist); }
        }
        let degree_match = (-best.1 * 0.5).exp();

        // Clustering coefficient
        let mut cc_sum = 0.0;
        let mut cc_count = 0;
        for i in 0..n {
            if degrees[i] < 2 { continue; }
            let neighbors: Vec<usize> = (0..n).filter(|&j| adj[i][j]).collect();
            let mut triangles = 0;
            for a in 0..neighbors.len() {
                for b in (a + 1)..neighbors.len() {
                    if adj[neighbors[a]][neighbors[b]] {
                        triangles += 1;
                    }
                }
            }
            let possible = neighbors.len() * (neighbors.len() - 1) / 2;
            if possible > 0 {
                cc_sum += triangles as f64 / possible as f64;
                cc_count += 1;
            }
        }
        let clustering_coeff = if cc_count > 0 { cc_sum / cc_count as f64 } else { 0.0 };

        // Connected components via BFS
        let mut visited = vec![false; n];
        let mut components = 0;
        for start in 0..n {
            if visited[start] { continue; }
            components += 1;
            let mut stack = vec![start];
            while let Some(cur) = stack.pop() {
                if visited[cur] { continue; }
                visited[cur] = true;
                for j in 0..n {
                    if !visited[j] && adj[cur][j] {
                        stack.push(j);
                    }
                }
            }
        }

        let mut result = HashMap::new();
        result.insert("mean_degree".to_string(), vec![mean_degree]);
        result.insert("degree_match".to_string(), vec![degree_match]);
        result.insert("clustering_coefficient".to_string(), vec![clustering_coeff]);
        result.insert("connected_components".to_string(), vec![components as f64]);
        result.insert("degree_distribution".to_string(), degrees.iter().map(|&d| d as f64).collect());
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_graph_lens_grid() {
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
        let result = GraphLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_degree"));
        assert!(result.contains_key("clustering_coefficient"));
        assert!(result["connected_components"][0] >= 1.0);
    }
}
