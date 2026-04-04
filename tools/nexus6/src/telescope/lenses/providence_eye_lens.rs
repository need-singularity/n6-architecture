use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ProvidenceEyeLens: Providential/purposeful pattern detection.
///
/// Measures purpose (directed vs random motion), optimization trajectory,
/// intervention points (high-leverage), and serendipity (unexpected
/// beneficial connections).
pub struct ProvidenceEyeLens;

impl Lens for ProvidenceEyeLens {
    fn name(&self) -> &str { "ProvidenceEyeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // purpose_score: directedness of sequential motion
        // Compare cumulative displacement vs total path length
        let mut path_length = 0.0_f64;
        for i in 0..(max_n - 1) {
            path_length += shared.dist(i, i + 1);
        }
        let displacement = if max_n >= 2 { shared.dist(0, max_n - 1) } else { 0.0 };
        let purpose_score = if path_length > 1e-12 {
            (displacement / path_length).min(1.0)
        } else { 0.0 };

        // optimization_trajectory: is local density increasing over time?
        let half = max_n / 2;
        let density_first: f64 = (0..half)
            .map(|i| shared.knn_density(i))
            .sum::<f64>() / half.max(1) as f64;
        let density_second: f64 = (half..max_n)
            .map(|i| shared.knn_density(i))
            .sum::<f64>() / (max_n - half).max(1) as f64;
        let optimization_trajectory = if density_first > 1e-12 {
            ((density_second - density_first) / density_first).clamp(-1.0, 1.0)
        } else { 0.0 };

        // intervention_points: points whose removal would maximally change
        // the mean distance to neighbors (high betweenness proxy)
        let mut leverage_scores: Vec<f64> = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let knn = shared.knn(i);
            // Sum distances between this point's neighbors
            let mut neighbor_dist_sum = 0.0_f64;
            let mut nd_count = 0u32;
            let neighbors: Vec<usize> = knn.iter().take(4.min(knn.len())).map(|&j| j as usize).collect();
            for a in 0..neighbors.len() {
                for b in (a + 1)..neighbors.len() {
                    neighbor_dist_sum += shared.dist(neighbors[a], neighbors[b]);
                    nd_count += 1;
                }
            }
            let mean_nd = if nd_count > 0 { neighbor_dist_sum / nd_count as f64 } else { 0.0 };
            // High leverage = neighbors are far apart (this point bridges them)
            leverage_scores.push(mean_nd);
        }
        let mean_leverage = leverage_scores.iter().sum::<f64>() / max_n as f64;
        let std_leverage = (leverage_scores.iter()
            .map(|&l| (l - mean_leverage).powi(2)).sum::<f64>() / max_n as f64).sqrt();
        let threshold = mean_leverage + std_leverage;
        let intervention_points = leverage_scores.iter()
            .filter(|&&l| l > threshold).count() as f64;

        // serendipity_score: unexpected high MI between distant dimensions
        let mut serendipity = 0.0_f64;
        let mut ser_count = 0u32;
        if d >= 3 {
            for di in 0..d {
                for dj in (di + 2)..d {
                    let mi = shared.mi(di, dj);
                    if mi > 0.2 {
                        serendipity += mi;
                        ser_count += 1;
                    }
                }
            }
        }
        let serendipity_score = if ser_count > 0 {
            serendipity / ser_count as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("purpose_score".to_string(), vec![purpose_score]);
        result.insert("optimization_trajectory".to_string(), vec![optimization_trajectory]);
        result.insert("intervention_points".to_string(), vec![intervention_points]);
        result.insert("serendipity_score".to_string(), vec![serendipity_score]);
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
        let result = ProvidenceEyeLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("purpose_score"));
        assert!(result.contains_key("intervention_points"));
    }

    #[test]
    fn test_directed_motion() {
        // Straight line: high purpose
        let data: Vec<f64> = (0..20).map(|i| i as f64 * 0.5).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let result = ProvidenceEyeLens.scan(&data, 10, 2, &shared);
        let ps = result["purpose_score"][0];
        assert!(ps > 0.5, "directed motion should have high purpose: {ps}");
    }
}
