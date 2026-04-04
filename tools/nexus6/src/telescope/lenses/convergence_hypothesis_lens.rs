use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConvergenceHypothesisLens: Measures multi-metric convergence toward n=6 values.
///
/// Algorithm:
///   1. Treat rows as time-ordered observations
///   2. For each dimension, measure trend toward n=6 constants over time
///   3. Compute convergence_velocity = rate of approach to nearest n=6 value
///   4. Multi-metric consensus: how many dimensions converge simultaneously
///   5. Convergence phase detection: pre-convergence / converging / converged / diverging
///
/// Outputs:
///   - convergence_score: 0-1, overall convergence toward n=6
///   - converging_dims: number of dimensions trending toward n=6
///   - convergence_velocity: rate of convergence (higher = faster approach)
///   - consensus_ratio: fraction of dims converging together
///   - phase: 0=diverging, 1=pre-convergence, 2=converging, 3=converged
///   - nearest_attractor: which n=6 constant is the dominant attractor
pub struct ConvergenceHypothesisLens;

// n=6 attractor values
const ATTRACTORS: &[f64] = &[
    1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 11.0, 12.0, 24.0, 0.288, 0.693, 1.333, 0.1,
    0.5, 0.333, 0.167, 144.0, 120.0,
];

impl Lens for ConvergenceHypothesisLens {
    fn name(&self) -> &str {
        "ConvergenceHypothesisLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims_check = d.min(24); // J₂=24 max
        let mut converging_count = 0u32;
        let mut total_velocity = 0.0_f64;
        let mut attractor_votes: Vec<u32> = vec![0; ATTRACTORS.len()];

        // Split data into first half and second half (temporal analysis)
        let mid = n / 2;
        let first_half = mid;
        let second_half = n - mid;

        for dim in 0..dims_check {
            // First half mean
            let mean_1: f64 =
                (0..first_half).map(|r| data[r * d + dim]).sum::<f64>() / first_half as f64;
            // Second half mean
            let mean_2: f64 =
                (mid..n).map(|r| data[r * d + dim]).sum::<f64>() / second_half as f64;

            // Find nearest attractor for each half
            let (_attr_1_idx, dist_1) = nearest_attractor(mean_1);
            let (attr_2_idx, dist_2) = nearest_attractor(mean_2);

            // Converging if second half is closer to attractor than first half
            if dist_2 < dist_1 {
                converging_count += 1;
                let velocity = (dist_1 - dist_2) / dist_1.max(1e-12);
                total_velocity += velocity;
                attractor_votes[attr_2_idx] += 1;
            }
        }

        let consensus_ratio = converging_count as f64 / dims_check.max(1) as f64;
        let avg_velocity = if converging_count > 0 {
            total_velocity / converging_count as f64
        } else {
            0.0
        };

        // Overall convergence score = consensus * velocity
        let convergence_score = (consensus_ratio * avg_velocity).min(1.0);

        // Phase detection
        let phase = if consensus_ratio > 0.8 && avg_velocity > 0.5 {
            3.0 // converged
        } else if consensus_ratio > 0.5 {
            2.0 // converging
        } else if consensus_ratio > 0.2 {
            1.0 // pre-convergence
        } else {
            0.0 // diverging
        };

        // Dominant attractor
        let dominant_attractor = attractor_votes
            .iter()
            .enumerate()
            .max_by_key(|&(_, &v)| v)
            .map(|(i, _)| i)
            .unwrap_or(0);

        // Monotonicity check: is the trend consistent (not oscillating)?
        let mut monotonic_dims = 0u32;
        let window = n / 4;
        if window >= 2 {
            for dim in 0..dims_check {
                let mut window_means: Vec<f64> = Vec::new();
                for w in 0..4 {
                    let start = w * window;
                    let end = ((w + 1) * window).min(n);
                    let wm: f64 =
                        (start..end).map(|r| data[r * d + dim]).sum::<f64>() / (end - start) as f64;
                    window_means.push(wm);
                }
                // Check if distances to nearest attractor monotonically decrease
                let attr_dists: Vec<f64> =
                    window_means.iter().map(|&m| nearest_attractor(m).1).collect();
                let is_monotonic = attr_dists.windows(2).all(|w| w[1] <= w[0] + 1e-6);
                if is_monotonic {
                    monotonic_dims += 1;
                }
            }
        }

        let mut result = HashMap::new();
        result.insert("convergence_score".into(), vec![convergence_score]);
        result.insert("converging_dims".into(), vec![converging_count as f64]);
        result.insert("convergence_velocity".into(), vec![avg_velocity]);
        result.insert("consensus_ratio".into(), vec![consensus_ratio]);
        result.insert("phase".into(), vec![phase]);
        result.insert(
            "nearest_attractor".into(),
            vec![ATTRACTORS[dominant_attractor]],
        );
        result.insert("monotonic_dims".into(), vec![monotonic_dims as f64]);
        result.insert("dims_checked".into(), vec![dims_check as f64]);
        result
    }
}

/// Find the nearest n=6 attractor and return (index, distance).
fn nearest_attractor(value: f64) -> (usize, f64) {
    let abs_val = value.abs();
    ATTRACTORS
        .iter()
        .enumerate()
        .map(|(i, &a)| {
            let dist = if a.abs() > 1e-12 {
                ((abs_val - a) / a).abs()
            } else {
                abs_val
            };
            (i, dist)
        })
        .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .unwrap_or((0, f64::MAX))
}
