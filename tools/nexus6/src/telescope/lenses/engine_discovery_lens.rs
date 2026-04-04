use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy, mean_var};

/// Discovers optimal algorithmic engine for the data.
pub struct EngineDiscoveryLens;

impl Lens for EngineDiscoveryLens {
    fn name(&self) -> &str { "EngineDiscoveryLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let mut result = HashMap::new();

        let (means, vars) = mean_var(data, max_n, d);
        let total_var: f64 = vars.iter().sum();

        // Distance distribution analysis
        let mut dists = Vec::with_capacity(max_n * (max_n - 1) / 2);
        for i in 0..max_n {
            for j in (i + 1)..max_n {
                dists.push(shared.dist(i, j));
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let dist_spread = if let (Some(&lo), Some(&hi)) = (dists.first(), dists.last()) {
            if hi > 1e-12 { (hi - lo) / hi } else { 0.0 }
        } else { 0.0 };
        let dist_entropy = shannon_entropy(&dists, 20);

        // Smoothness: average gradient magnitude (data as sequence)
        let grad_mag = if data.len() > d {
            let steps = (data.len() / d).saturating_sub(1);
            if steps == 0 { 0.0 } else {
                let mut sum = 0.0;
                for i in 0..steps {
                    for j in 0..d {
                        let diff = data[(i + 1) * d + j] - data[i * d + j];
                        sum += diff * diff;
                    }
                }
                (sum / steps as f64).sqrt()
            }
        } else { 0.0 };

        // Density clustering signal
        let densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
        let density_var = {
            let m = densities.iter().sum::<f64>() / max_n as f64;
            densities.iter().map(|&x| (x - m) * (x - m)).sum::<f64>() / max_n as f64
        };

        // Score engines: brute_force(0), gradient(1), evolutionary(2), stochastic(3), combinatorial(4)
        let smoothness = 1.0 / (1.0 + grad_mag);
        let scores = [
            1.0 / (1.0 + max_n as f64 / 50.0),   // brute_force: small n
            smoothness,                             // gradient: smooth landscape
            dist_spread,                            // evolutionary: spread solutions
            dist_entropy / (dist_entropy + 1.0),    // stochastic: high entropy
            density_var / (density_var + 0.1),      // combinatorial: clustered
        ];
        let best_idx = scores.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i).unwrap_or(0);

        // Convergence estimate: based on smoothness and dimensionality
        let convergence = (d as f64 * 10.0) / smoothness.max(0.01);

        // Parallelizability: high when data is separable (low MI between dims)
        let mi_sum = if d >= 2 {
            let pairs = d.min(6);
            let mut s = 0.0;
            for di in 0..pairs.min(d) {
                for dj in (di + 1)..pairs.min(d) {
                    s += shared.mi(di, dj);
                }
            }
            s / (pairs * (pairs - 1) / 2).max(1) as f64
        } else { 0.0 };
        let parallelizability = 1.0 / (1.0 + mi_sum);

        // Search space size estimate (log scale)
        let search_space = (max_n as f64).ln() * (d as f64).ln().max(1.0);

        // Efficiency: insights per op estimate
        let efficiency = if search_space > 1e-12 {
            dist_entropy / search_space
        } else { 0.0 };

        result.insert("optimal_engine_type".into(), vec![best_idx as f64]);
        result.insert("convergence_estimate".into(), vec![convergence]);
        result.insert("parallelizability".into(), vec![parallelizability]);
        result.insert("search_space_size".into(), vec![search_space]);
        result.insert("engine_efficiency".into(), vec![efficiency]);
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
        let r = EngineDiscoveryLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("optimal_engine_type"));
        assert!(r.contains_key("parallelizability"));
    }
}
