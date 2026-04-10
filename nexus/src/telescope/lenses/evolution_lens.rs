use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// EvolutionLens: fitness landscape analysis via density + dominance.
///
/// Algorithm:
///   1. Fitness = local density (k-NN inverse distance)
///   2. Dominance: point A dominates B if fitness(A) > fitness(B) and dist(A,B) < threshold
///   3. Non-dominated points = Pareto front (evolutionary peaks)
///   4. Reports fitness variance and Pareto front size
pub struct EvolutionLens;

impl Lens for EvolutionLens {
    fn name(&self) -> &str {
        "EvolutionLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        let k = ((n as f64).sqrt().ceil() as usize).max(1).min(n - 1);

        // Fitness = k-NN density
        let fitness: Vec<f64> = (0..n)
            .map(|i| {
                let mut dists: Vec<f64> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| shared.dist(i, j))
                    .collect();
                dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let knn_dist = dists[k.min(dists.len()) - 1];
                if knn_dist > 0.0 {
                    1.0 / knn_dist
                } else {
                    f64::MAX
                }
            })
            .collect();

        // Median distance as dominance radius
        let mut all_dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let radius = all_dists[all_dists.len() / 2];

        // Find non-dominated (Pareto) points
        let mut dominated = vec![false; n];
        for i in 0..n {
            if dominated[i] {
                continue;
            }
            for j in 0..n {
                if i == j || dominated[j] {
                    continue;
                }
                if shared.dist(i, j) < radius && fitness[i] > fitness[j] {
                    dominated[j] = true;
                }
            }
        }

        let pareto_size = dominated.iter().filter(|&&d| !d).count();

        // Fitness variance
        let mean_fit = fitness.iter().sum::<f64>() / n as f64;
        let var_fit =
            fitness.iter().map(|&f| (f - mean_fit).powi(2)).sum::<f64>() / n as f64;

        let mut result = HashMap::new();
        result.insert("pareto_front_size".to_string(), vec![pareto_size as f64]);
        result.insert("fitness_variance".to_string(), vec![var_fit]);
        result
    }
}
