use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConsciousnessLens: IIT Phi approximation via clique detection + cluster independence.
///
/// Algorithm:
///   1. Build adjacency from distance threshold (median distance)
///   2. Find maximal cliques (factions) via greedy expansion
///   3. Measure integration = mean intra-clique connectivity
///   4. Measure differentiation = inter-clique distance spread
///   5. Phi ~ integration * differentiation
pub struct ConsciousnessLens;

impl Lens for ConsciousnessLens {
    fn name(&self) -> &str {
        "ConsciousnessLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // Compute median distance as threshold
        let pair_count = n * (n - 1) / 2;
        let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_dist = all_dists[pair_count / 2];

        // Build adjacency: connected if distance < median
        let mut adj: Vec<Vec<bool>> = vec![vec![false; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if shared.dist(i, j) < median_dist {
                    adj[i][j] = true;
                    adj[j][i] = true;
                }
            }
        }

        // Greedy clique detection: assign each point to a clique
        let mut assigned = vec![false; n];
        let mut cliques: Vec<Vec<usize>> = Vec::new();

        for seed in 0..n {
            if assigned[seed] {
                continue;
            }
            let mut clique = vec![seed];
            assigned[seed] = true;

            for candidate in (seed + 1)..n {
                if assigned[candidate] {
                    continue;
                }
                // Check if candidate is connected to all clique members
                let fits = clique.iter().all(|&m| adj[m][candidate]);
                if fits {
                    clique.push(candidate);
                    assigned[candidate] = true;
                }
            }
            cliques.push(clique);
        }

        // Integration: mean intra-clique density (edges / possible edges)
        let mut integration = 0.0;
        let mut clique_count = 0;
        for clique in &cliques {
            if clique.len() >= 2 {
                let possible = clique.len() * (clique.len() - 1) / 2;
                let mut edges = 0;
                for i in 0..clique.len() {
                    for j in (i + 1)..clique.len() {
                        if adj[clique[i]][clique[j]] {
                            edges += 1;
                        }
                    }
                }
                integration += edges as f64 / possible as f64;
                clique_count += 1;
            }
        }
        if clique_count > 0 {
            integration /= clique_count as f64;
        }

        // Differentiation: variance of inter-clique mean distances
        let num_cliques = cliques.len();
        let mut inter_dists: Vec<f64> = Vec::new();
        for a in 0..num_cliques {
            for b in (a + 1)..num_cliques {
                let mut sum = 0.0;
                let mut count = 0;
                for &ia in &cliques[a] {
                    for &ib in &cliques[b] {
                        sum += shared.dist(ia, ib);
                        count += 1;
                    }
                }
                if count > 0 {
                    inter_dists.push(sum / count as f64);
                }
            }
        }

        let differentiation = if inter_dists.len() >= 2 {
            let mean = inter_dists.iter().sum::<f64>() / inter_dists.len() as f64;
            let var = inter_dists.iter().map(|x| (x - mean) * (x - mean)).sum::<f64>()
                / inter_dists.len() as f64;
            var.sqrt()
        } else {
            0.0
        };

        let phi = integration * differentiation;

        let mut result = HashMap::new();
        result.insert("phi_approx".to_string(), vec![phi]);
        result.insert(
            "integration_differentiation".to_string(),
            vec![integration, differentiation],
        );
        result.insert("num_cliques".to_string(), vec![num_cliques as f64]);
        result
    }
}
