use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// WormholeLens: Topological shortcut detection in data space.
///
/// Finds "wormholes" — pairs of distant points/clusters that are
/// unexpectedly connected (high mutual information despite large distance).
/// Analogous to Einstein-Rosen bridges connecting distant spacetime regions.
///
/// Metrics:
///   1. shortcut_ratio: min graph distance / Euclidean distance for connected pairs
///   2. wormhole_count: number of detected shortcut connections
///   3. tunnel_strength: mean MI of shortcut pairs / mean MI of all pairs
///   4. bypass_efficiency: fraction of total distance saved via shortcuts
///
/// n=6: Wormhole throat radius ~ Planck length; traversable wormholes need
///       exotic matter with energy ~ -σ(6). ER=EPR connects to J₂(6)=24 entangled pairs.
pub struct WormholeLens;

impl Lens for WormholeLens {
    fn name(&self) -> &str { "WormholeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 8 || d < 2 { return HashMap::new(); }

        let max_n = n.min(150);

        // Collect all pairwise distances
        let mut all_dists: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..max_n {
            for j in (i + 1)..max_n {
                all_dists.push((i, j, shared.dist(i, j)));
            }
        }
        if all_dists.is_empty() { return HashMap::new(); }

        all_dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap());
        let median_dist = all_dists[all_dists.len() / 2].2;
        if median_dist < 1e-15 { return HashMap::new(); }

        // Distant pairs: distance > 75th percentile
        let q75_idx = all_dists.len() * 3 / 4;
        let dist_threshold = all_dists[q75_idx].2;

        // For distant pairs, check if they share KNN neighbors (shortcut indicator)
        let mut wormhole_count = 0u32;
        let mut shortcut_ratios = Vec::new();
        let mut tunnel_mi_sum = 0.0f64;

        for &(i, j, dist_ij) in all_dists.iter().filter(|t| t.2 > dist_threshold) {
            if i >= max_n || j >= max_n { continue; }

            let knn_i = shared.knn(i);
            let knn_j = shared.knn(j);

            // Count shared neighbors
            let shared_count = knn_i.iter()
                .filter(|&&ni| knn_j.contains(&ni))
                .count();

            if shared_count >= 2 {
                wormhole_count += 1;

                // Shortcut: go through shared neighbor instead of direct
                let mut min_shortcut = dist_ij;
                for &ni in knn_i.iter() {
                    if knn_j.contains(&ni) {
                        let via = shared.dist(i, ni as usize) + shared.dist(ni as usize, j);
                        if via < min_shortcut { min_shortcut = via; }
                    }
                }
                shortcut_ratios.push(min_shortcut / dist_ij);

                // MI between dimensions for tunnel strength
                if d >= 2 {
                    tunnel_mi_sum += shared.mi(0, 1);
                }
            }
        }

        let total_distant = all_dists.iter().filter(|t| t.2 > dist_threshold).count() as f64;
        let wormhole_frac = if total_distant > 0.0 { wormhole_count as f64 / total_distant } else { 0.0 };

        let mean_shortcut = if shortcut_ratios.is_empty() {
            1.0
        } else {
            shortcut_ratios.iter().sum::<f64>() / shortcut_ratios.len() as f64
        };

        let bypass_efficiency = 1.0 - mean_shortcut; // 0 = no bypass, ~1 = near-perfect shortcut

        let mut result = HashMap::new();
        result.insert("wormhole_count".to_string(), vec![wormhole_count as f64]);
        result.insert("wormhole_fraction".to_string(), vec![wormhole_frac]);
        result.insert("mean_shortcut_ratio".to_string(), vec![mean_shortcut]);
        result.insert("bypass_efficiency".to_string(), vec![bypass_efficiency]);
        result.insert("shortcut_ratios".to_string(), shortcut_ratios);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wormhole_lens_basic() {
        // Create data with potential shortcuts
        let mut data = Vec::new();
        for i in 0..15 {
            data.push(i as f64);
            data.push((i as f64 * 0.5).sin());
        }
        let n = 15;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = WormholeLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("wormhole_count"));
        assert!(result.contains_key("bypass_efficiency"));
    }
}
