use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DestinyLens: Deterministic trajectory prediction.
///
/// Measures predictability from kNN consistency, attractor basin count,
/// convergence rate, free will (1 - predictability), and proximity
/// to bifurcation points.
pub struct DestinyLens;

impl Lens for DestinyLens {
    fn name(&self) -> &str { "DestinyLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // predictability: for sequential points, how often does the next point
        // fall within the kNN neighborhood of the current point?
        let mut predict_hits = 0u32;
        let mut predict_total = 0u32;
        for i in 0..(max_n - 1) {
            let knn = shared.knn(i);
            let next = (i + 1) as u32;
            if knn.contains(&next) {
                predict_hits += 1;
            }
            predict_total += 1;
        }
        let predictability = if predict_total > 0 {
            predict_hits as f64 / predict_total as f64
        } else { 0.0 };

        // attractor_count: number of distinct dense regions (local density peaks)
        let mut densities: Vec<(usize, f64)> = (0..max_n)
            .map(|i| (i, shared.knn_density(i)))
            .collect();
        densities.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // A point is an attractor if it has higher density than all its neighbors
        let mut attractors: Vec<usize> = Vec::new();
        for i in 0..max_n {
            let my_density = shared.knn_density(i);
            let knn = shared.knn(i);
            let is_peak = knn.iter().all(|&j| shared.knn_density(j as usize) <= my_density);
            if is_peak {
                // Check it's not too close to an existing attractor
                let far_enough = attractors.iter().all(|&a| shared.dist(i, a) > 1e-6);
                if far_enough {
                    attractors.push(i);
                }
            }
        }
        let attractor_count = attractors.len().max(1) as f64;

        // convergence_rate: how fast do sequential points approach nearest attractor?
        let mut convergence_deltas: Vec<f64> = Vec::new();
        for i in 0..(max_n - 1) {
            if attractors.is_empty() { break; }
            let d_curr = attractors.iter()
                .map(|&a| if a != i { shared.dist(i, a) } else { 0.0 })
                .fold(f64::MAX, f64::min);
            let d_next = attractors.iter()
                .map(|&a| if a != (i + 1) { shared.dist(i + 1, a) } else { 0.0 })
                .fold(f64::MAX, f64::min);
            if d_curr > 1e-12 {
                convergence_deltas.push((d_curr - d_next) / d_curr);
            }
        }
        let convergence_rate = if !convergence_deltas.is_empty() {
            convergence_deltas.iter().sum::<f64>() / convergence_deltas.len() as f64
        } else { 0.0 };

        let free_will_score = 1.0 - predictability;

        // bifurcation_proximity: how close are we to points where kNN neighborhoods
        // split (neighbors have very different neighborhoods)
        let mut bifurc_scores: Vec<f64> = Vec::new();
        let sample_step = (max_n / 30).max(1);
        for i in (0..max_n).step_by(sample_step) {
            let knn = shared.knn(i);
            if knn.len() < 2 { continue; }
            // Compare neighborhoods of first two neighbors
            let n0 = shared.knn(knn[0] as usize);
            let n1 = shared.knn(knn[1.min(knn.len() - 1)] as usize);
            let overlap = n0.iter().filter(|&&x| n1.contains(&x)).count();
            let max_k = n0.len().max(1);
            let divergence = 1.0 - (overlap as f64 / max_k as f64);
            bifurc_scores.push(divergence);
        }
        let bifurcation_proximity = if !bifurc_scores.is_empty() {
            bifurc_scores.iter().sum::<f64>() / bifurc_scores.len() as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("predictability".to_string(), vec![predictability]);
        result.insert("attractor_count".to_string(), vec![attractor_count]);
        result.insert("convergence_rate".to_string(), vec![convergence_rate]);
        result.insert("free_will_score".to_string(), vec![free_will_score]);
        result.insert("bifurcation_proximity".to_string(), vec![bifurcation_proximity]);
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
        let result = DestinyLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("predictability"));
        assert!(result.contains_key("attractor_count"));
        assert!(result.contains_key("free_will_score"));
    }

    #[test]
    fn test_free_will_complement() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1)).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = DestinyLens.scan(&data, 20, 2, &shared);
        let p = result["predictability"][0];
        let fw = result["free_will_score"][0];
        assert!((p + fw - 1.0).abs() < 1e-10, "predictability + free_will should = 1.0");
    }
}
