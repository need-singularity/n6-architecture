use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// MoleculeLens: Molecular structure detection — graph-like bonding patterns.
/// Measures bond_graph_density, ring_count, branching_factor, molecular_symmetry, functional_group_count.
pub struct MoleculeLens;

const BOND_THRESH: f64 = 0.6;

impl Lens for MoleculeLens {
    fn name(&self) -> &str { "MoleculeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }
        let max_n = n.min(200);
        let cols = column_vectors(data, max_n, d);
        let (means, vars) = mean_var(data, max_n, d);
        let max_d = d.min(20);

        // Build adjacency matrix from correlations
        let mut adj = vec![vec![false; max_d]; max_d];
        let mut degree = vec![0usize; max_d];
        let mut edge_count = 0usize;

        for i in 0..max_d {
            let si = vars[i].sqrt();
            if si < 1e-12 { continue; }
            for j in (i + 1)..max_d {
                let sj = vars[j].sqrt();
                if sj < 1e-12 { continue; }
                let mut cov = 0.0;
                for k in 0..max_n { cov += (cols[i][k] - means[i]) * (cols[j][k] - means[j]); }
                cov /= max_n as f64;
                if (cov / (si * sj)).abs() > BOND_THRESH {
                    adj[i][j] = true; adj[j][i] = true;
                    degree[i] += 1; degree[j] += 1;
                    edge_count += 1;
                }
            }
        }

        let possible = max_d * (max_d - 1) / 2;
        let bond_graph_density = if possible > 0 { edge_count as f64 / possible as f64 } else { 0.0 };

        // Ring count via simple cycle detection: edges - vertices + components
        // Euler formula: cycles = E - V + C (for forest C = V - E, so rings = E - V + C)
        let mut visited = vec![false; max_d];
        let mut components = 0usize;
        for start in 0..max_d {
            if visited[start] || degree[start] == 0 { continue; }
            components += 1;
            let mut stack = vec![start];
            while let Some(node) = stack.pop() {
                if visited[node] { continue; }
                visited[node] = true;
                for nb in 0..max_d { if adj[node][nb] && !visited[nb] { stack.push(nb); } }
            }
        }
        let active_v = degree.iter().filter(|&&d| d > 0).count();
        let ring_count = if active_v > 0 { (edge_count + components).saturating_sub(active_v) } else { 0 };

        // Branching factor: mean degree of nodes with degree > 0
        let branching = if active_v > 0 { degree.iter().sum::<usize>() as f64 / active_v as f64 } else { 0.0 };

        // Molecular symmetry: how symmetric the degree sequence is
        let mut degs: Vec<usize> = degree[..max_d].iter().copied().filter(|&d| d > 0).collect();
        degs.sort();
        let sym = if degs.len() > 1 {
            let rev: Vec<usize> = degs.iter().rev().copied().collect();
            let diffs: f64 = degs.iter().zip(rev.iter()).map(|(&a, &b)| (a as f64 - b as f64).abs()).sum();
            1.0 - diffs / (degs.len() as f64 * (*degs.last().unwrap_or(&1)).max(1) as f64)
        } else { 1.0 };

        // Functional group count: distinct degree values
        let mut unique_degs: Vec<usize> = degs.clone();
        unique_degs.dedup();
        let functional_groups = unique_degs.len();

        let mut result = HashMap::new();
        result.insert("bond_graph_density".into(), vec![bond_graph_density]);
        result.insert("ring_count".into(), vec![ring_count as f64]);
        result.insert("branching_factor".into(), vec![branching]);
        result.insert("molecular_symmetry".into(), vec![sym]);
        result.insert("functional_group_count".into(), vec![functional_groups as f64]);
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
        let result = MoleculeLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
