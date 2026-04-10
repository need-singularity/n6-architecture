use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TopologyLens: simple persistent homology — Betti_0 via single-linkage filtration.
///
/// Algorithm:
///   1. Sort all pairwise distances (filtration values)
///   2. Union-Find to track connected components
///   3. Record birth/death of components = persistence diagram
///   4. Betti_0(epsilon) = number of components at each threshold
pub struct TopologyLens;

impl Lens for TopologyLens {
    fn name(&self) -> &str {
        "TopologyLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // Collect all pairwise edges sorted by distance
        let mut edges: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j, shared.dist(i, j)));
            }
        }
        edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        // Union-Find
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank: Vec<usize> = vec![0; n];
        let mut _components = n;

        // Persistence: record (birth, death) for each merged component
        // All components born at distance 0
        let mut persistence_pairs: Vec<(f64, f64)> = Vec::new();

        for &(i, j, dist) in &edges {
            let ri = find(&mut parent, i);
            let rj = find(&mut parent, j);
            if ri != rj {
                // Merge: the younger component dies
                union(&mut parent, &mut rank, ri, rj);
                _components -= 1;
                persistence_pairs.push((0.0, dist));
            }
        }

        // The last surviving component has infinite persistence (never dies)
        // We represent it as (0, max_dist)
        let max_dist = edges.last().map(|e| e.2).unwrap_or(0.0);

        // Persistence = death - birth for each pair
        let lifetimes: Vec<f64> = persistence_pairs.iter().map(|(b, d)| d - b).collect();

        // Key topological features:
        // 1. Number of significant components (lifetime > median lifetime)
        let median_life = if lifetimes.is_empty() {
            0.0
        } else {
            let mut sorted = lifetimes.clone();
            sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            sorted[sorted.len() / 2]
        };

        let significant_components = lifetimes.iter().filter(|&&l| l > median_life).count();

        // 2. Total persistence (sum of lifetimes)
        let total_persistence: f64 = lifetimes.iter().sum();

        let mut result = HashMap::new();
        result.insert(
            "betti0_significant".to_string(),
            vec![significant_components as f64],
        );
        result.insert("total_persistence".to_string(), vec![total_persistence]);
        result.insert("max_gap".to_string(), vec![max_dist]);
        result
    }
}

fn find(parent: &mut [usize], mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]]; // path compression
        x = parent[x];
    }
    x
}

fn union(parent: &mut [usize], rank: &mut [usize], x: usize, y: usize) {
    if rank[x] < rank[y] {
        parent[x] = y;
    } else if rank[x] > rank[y] {
        parent[y] = x;
    } else {
        parent[y] = x;
        rank[x] += 1;
    }
}
