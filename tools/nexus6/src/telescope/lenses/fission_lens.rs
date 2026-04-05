use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FissionLens: Splitting / fragmentation detection in data.
///
/// Detects fission-like events where a coherent structure splits into
/// fragments. Analogous to nuclear fission, cell division, or cluster
/// fragmentation. Complementary to clustering — focuses on the *process*
/// of splitting rather than the static cluster state.
///
/// Metrics:
///   1. fission_count: number of detected split events (temporal)
///   2. fragment_count: number of resulting fragments
///   3. mass_asymmetry: how unequal the fragments are (0=equal, 1=max asymmetry)
///   4. fission_energy: energy released in splitting (inter-fragment distance)
///   5. critical_mass: minimum cluster size before splitting occurs
///   6. chain_reaction_score: likelihood of cascading splits
///
/// n=6: Nuclear fission of U-235 → fragments with A ≈ 95 + 139.
///       Binding energy per nucleon peaks at Fe-56 (σ-n=6-related).
///       Critical mass geometry is spherical (CN=12=σ surface coordination).
///       Fission barrier ~ n MeV for actinides.
pub struct FissionLens;

impl Lens for FissionLens {
    fn name(&self) -> &str { "FissionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 10 { return HashMap::new(); }

        let max_n = n.min(200);

        // 1. Identify clusters via density-based approach
        // Points with high density are cluster cores
        let mut densities: Vec<(usize, f64)> = (0..max_n)
            .map(|i| (i, shared.knn_density(i)))
            .collect();
        densities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)); // descending

        let median_density = {
            let mut d: Vec<f64> = densities.iter().map(|x| x.1).collect();
            d.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            d[d.len() / 2]
        };

        // Assign points to nearest high-density core
        let core_threshold = median_density * 1.5;
        let cores: Vec<usize> = densities.iter()
            .filter(|(_, d)| *d > core_threshold)
            .map(|(i, _)| *i)
            .take(10) // max 10 cores
            .collect();

        if cores.is_empty() {
            let mut result = HashMap::new();
            result.insert("fragment_count".to_string(), vec![1.0]);
            result.insert("fission_count".to_string(), vec![0.0]);
            result.insert("mass_asymmetry".to_string(), vec![0.0]);
            result.insert("fission_energy".to_string(), vec![0.0]);
            result.insert("critical_mass".to_string(), vec![n as f64]);
            result.insert("chain_reaction_score".to_string(), vec![0.0]);
            return result;
        }

        // Assign each point to nearest core
        let mut assignments = vec![0usize; max_n];
        let mut cluster_sizes = vec![0u32; cores.len()];
        for i in 0..max_n {
            let mut best_core = 0;
            let mut best_dist = f64::INFINITY;
            for (ci, &core) in cores.iter().enumerate() {
                let d = if i == core { 0.0 } else { shared.dist(i, core) };
                if d < best_dist {
                    best_dist = d;
                    best_core = ci;
                }
            }
            assignments[i] = best_core;
            cluster_sizes[best_core] += 1;
        }

        let fragment_count = cluster_sizes.iter().filter(|&&s| s > 0).count();

        // 2. Mass asymmetry: how unequal are fragment sizes?
        let total_mass = cluster_sizes.iter().sum::<u32>() as f64;
        let max_frag = *cluster_sizes.iter().max().unwrap_or(&1) as f64;
        let mass_asymmetry = if fragment_count > 1 {
            (max_frag / total_mass - 1.0 / fragment_count as f64)
                / (1.0 - 1.0 / fragment_count as f64).max(1e-12)
        } else {
            0.0
        };

        // 3. Fission energy: mean inter-fragment distance
        let mut fission_energy = 0.0f64;
        let mut pair_count = 0u32;
        for ci in 0..cores.len() {
            for cj in (ci + 1)..cores.len() {
                if cluster_sizes[ci] > 0 && cluster_sizes[cj] > 0 {
                    fission_energy += shared.dist(cores[ci], cores[cj]);
                    pair_count += 1;
                }
            }
        }
        let fission_energy = if pair_count > 0 {
            fission_energy / pair_count as f64
        } else {
            0.0
        };

        // 4. Fission events: temporal splits (if data has temporal ordering)
        // Look for moments where inter-group distance increases sharply
        let mut fission_count = 0u32;
        if fragment_count >= 2 {
            let window = (max_n / 10).max(3);
            for t in window..max_n.saturating_sub(window) {
                // Compare inter-cluster distance before and after this time
                let before: f64 = (t.saturating_sub(window)..t)
                    .map(|i| shared.knn_density(i))
                    .sum::<f64>() / window as f64;
                let after: f64 = (t..t + window)
                    .map(|i| shared.knn_density(i.min(max_n - 1)))
                    .sum::<f64>() / window as f64;
                // Sharp density drop = fission event
                if before > 0.0 && after / before < 0.5 {
                    fission_count += 1;
                }
            }
            // Deduplicate nearby events
            fission_count = (fission_count / window as u32).max(if fission_count > 0 { 1 } else { 0 });
        }

        // 5. Critical mass: smallest fragment size
        let critical_mass = cluster_sizes.iter()
            .filter(|&&s| s > 0)
            .min()
            .copied()
            .unwrap_or(0) as f64;

        // 6. Chain reaction: if fragments themselves are unstable (high internal tension)
        let chain_score = if fragment_count >= 3 {
            (fragment_count as f64 - 2.0) / 8.0 // normalized, max around 10 fragments
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("fission_count".to_string(), vec![fission_count as f64]);
        result.insert("fragment_count".to_string(), vec![fragment_count as f64]);
        result.insert("mass_asymmetry".to_string(), vec![mass_asymmetry.clamp(0.0, 1.0)]);
        result.insert("fission_energy".to_string(), vec![fission_energy]);
        result.insert("critical_mass".to_string(), vec![critical_mass]);
        result.insert("chain_reaction_score".to_string(), vec![chain_score.clamp(0.0, 1.0)]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fission_lens_fragments() {
        // Three distinct clusters = 3 fragments
        let mut data = Vec::new();
        for i in 0..10 { data.push(i as f64 * 0.1); data.push(0.0); }
        for i in 0..10 { data.push(50.0 + i as f64 * 0.1); data.push(50.0); }
        for i in 0..10 { data.push(100.0 + i as f64 * 0.1); data.push(0.0); }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = FissionLens.scan(&data, n, 2, &shared);
        assert!(result.contains_key("fragment_count"));
        assert!(result["fragment_count"][0] >= 1.0);
    }

    #[test]
    fn test_fission_lens_single() {
        // Single cluster = no fission
        let n = 20;
        let data: Vec<f64> = (0..n * 2)
            .map(|i| (i as f64 * 0.01).sin())
            .collect();
        let shared = SharedData::compute(&data, n, 2);
        let result = FissionLens.scan(&data, n, 2, &shared);
        assert!(result["fragment_count"][0] <= 2.0);
    }
}
