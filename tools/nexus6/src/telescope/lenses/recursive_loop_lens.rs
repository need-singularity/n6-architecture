use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RecursiveLoopLens: Self-referential recursive analysis.
///
/// Applies analysis recursively — each pass uses results from the previous
/// pass as input, detecting self-reinforcing patterns, fixed points,
/// limit cycles, and convergence/divergence of recursive processes.
///
/// n=6: Recursion depth often stabilizes at n=6 or divisors {1,2,3,6}.
///       OUROBOROS evolution uses recursive self-improvement cycles.
pub struct RecursiveLoopLens;

impl Lens for RecursiveLoopLens {
    fn name(&self) -> &str { "RecursiveLoopLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Recursive density analysis: compute density, then density-of-density, etc.
        let max_depth = 6; // n=6 recursion depth
        let mut prev: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
        let mut fixed_point_depth = max_depth;
        let mut convergence_history = Vec::with_capacity(max_depth);
        let mut cycle_detected = false;
        let mut history: Vec<Vec<f64>> = vec![prev.clone()];

        for depth in 1..max_depth {
            // Compute "density of density" — rank-based transformation
            let mut sorted = prev.clone();
            sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
            let median = sorted[sorted.len() / 2];
            let next: Vec<f64> = prev.iter()
                .map(|&v| if median > 1e-15 { v / median } else { 1.0 })
                .collect();

            // Check convergence: did values stabilize?
            let delta: f64 = prev.iter().zip(next.iter())
                .map(|(a, b)| (a - b).abs())
                .sum::<f64>() / max_n as f64;
            convergence_history.push(delta);

            if delta < 1e-6 {
                fixed_point_depth = depth;
                break;
            }

            // Check for cycle: compare with all previous states
            for (hi, h) in history.iter().enumerate() {
                let cycle_delta: f64 = h.iter().zip(next.iter())
                    .map(|(a, b)| (a - b).abs())
                    .sum::<f64>() / max_n as f64;
                if cycle_delta < 1e-4 {
                    cycle_detected = true;
                    fixed_point_depth = depth - hi;
                    break;
                }
            }
            if cycle_detected { break; }

            history.push(next.clone());
            prev = next;
        }

        // Self-similarity across recursion depths
        let self_similarity = if convergence_history.len() >= 2 {
            let first = convergence_history[0];
            let last = convergence_history[convergence_history.len() - 1];
            if first > 1e-15 { last / first } else { 0.0 }
        } else { 1.0 };

        let convergence_rate = if !convergence_history.is_empty() {
            convergence_history.iter().sum::<f64>() / convergence_history.len() as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("fixed_point_depth".to_string(), vec![fixed_point_depth as f64]);
        result.insert("cycle_detected".to_string(), vec![if cycle_detected { 1.0 } else { 0.0 }]);
        result.insert("convergence_rate".to_string(), vec![convergence_rate]);
        result.insert("self_similarity".to_string(), vec![self_similarity]);
        result.insert("convergence_history".to_string(), convergence_history);
        result.insert("recursion_depth".to_string(), vec![history.len() as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_recursive_loop() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = RecursiveLoopLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("fixed_point_depth"));
        assert!(result.contains_key("convergence_rate"));
    }
}
