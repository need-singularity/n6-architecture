use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// GodsEyeLens: Omniscient global view — sees everything at once.
///
/// Measures global coherence, hidden order invisible at local scale,
/// omniscience score (global vs local info capture), and destiny
/// convergence (whether all paths lead to the same attractor).
pub struct GodsEyeLens;

impl Lens for GodsEyeLens {
    fn name(&self) -> &str { "GodsEyeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // global_coherence: ratio of mean pairwise distance to max distance
        // Low ratio = points are tightly clustered = high coherence
        let mut dist_sum = 0.0_f64;
        let mut max_dist = 0.0_f64;
        let mut pair_count = 0u64;
        for i in 0..max_n {
            for j in (i + 1)..max_n {
                let d = shared.dist(i, j);
                dist_sum += d;
                if d > max_dist { max_dist = d; }
                pair_count += 1;
            }
        }
        let mean_dist = if pair_count > 0 { dist_sum / pair_count as f64 } else { 0.0 };
        let global_coherence = if max_dist > 1e-12 {
            1.0 - (mean_dist / max_dist)
        } else { 1.0 };

        // hidden_order: compare global entropy vs mean local entropy
        // If global entropy is lower than expected from local, hidden order exists
        let all_vals: Vec<f64> = data.iter().copied().take(max_n * d).collect();
        let global_entropy = shannon_entropy(&all_vals, 20);

        let mut local_entropy_sum = 0.0_f64;
        let mut local_count = 0u32;
        for i in 0..max_n {
            let knn = shared.knn(i);
            let local_vals: Vec<f64> = knn.iter()
                .take(5.min(knn.len()))
                .flat_map(|&j| {
                    let start = (j as usize) * d;
                    let end = (start + d).min(data.len());
                    data[start..end].to_vec()
                })
                .collect();
            if local_vals.len() >= 4 {
                local_entropy_sum += shannon_entropy(&local_vals, 10);
                local_count += 1;
            }
        }
        let mean_local_entropy = if local_count > 0 {
            local_entropy_sum / local_count as f64
        } else { 0.0 };

        let hidden_order = if mean_local_entropy > 1e-12 {
            (mean_local_entropy - global_entropy).max(0.0) / mean_local_entropy
        } else { 0.0 };

        // omniscience_score: how much more info global view captures vs local
        let omniscience_score = if mean_local_entropy > 1e-12 {
            (global_entropy / mean_local_entropy).min(2.0)
        } else { 0.0 };

        // destiny_convergence: do knn neighborhoods overlap significantly?
        // If many points share similar neighbors, they converge to same "attractor"
        let mut overlap_sum = 0.0_f64;
        let mut overlap_count = 0u32;
        let sample_step = (max_n / 30).max(1);
        for i in (0..max_n).step_by(sample_step) {
            let knn_i = shared.knn(i);
            for j in ((i + 1)..max_n).step_by(sample_step) {
                let knn_j = shared.knn(j);
                let shared_neighbors = knn_i.iter()
                    .filter(|&&ni| knn_j.contains(&ni))
                    .count();
                let k = knn_i.len().max(1);
                overlap_sum += shared_neighbors as f64 / k as f64;
                overlap_count += 1;
            }
        }
        let destiny_convergence = if overlap_count > 0 {
            overlap_sum / overlap_count as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("global_coherence".to_string(), vec![global_coherence]);
        result.insert("hidden_order".to_string(), vec![hidden_order]);
        result.insert("omniscience_score".to_string(), vec![omniscience_score]);
        result.insert("destiny_convergence".to_string(), vec![destiny_convergence]);
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
        let result = GodsEyeLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("global_coherence"));
        assert!(result.contains_key("hidden_order"));
        assert!(result.contains_key("destiny_convergence"));
    }

    #[test]
    fn test_coherence_range() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.01)).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = GodsEyeLens.scan(&data, 20, 2, &shared);
        let coh = result["global_coherence"][0];
        assert!(coh >= 0.0 && coh <= 1.0, "coherence should be in [0,1]: {coh}");
    }
}
