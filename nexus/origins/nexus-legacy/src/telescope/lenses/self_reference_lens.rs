use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SelfReferenceLens: Detects self-referential cyclic structures in data (Ouroboros pattern).
///
/// Algorithm:
///   1. Compute similarity between each point and its temporal recurrence
///   2. Detect fixed-point attractors: points that map to themselves
///   3. Measure cycle lengths and their n=6 alignment
///   4. Self-similarity across scales (fractal self-reference)
///
/// n=6: μ(6)=1 (identity/self-reference), n=6 perfect self-mapping
pub struct SelfReferenceLens;

impl Lens for SelfReferenceLens {
    fn name(&self) -> &str {
        "SelfReferenceLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. Recurrence detection: for each point, find nearest future point
        let mut recurrence_dists: Vec<f64> = Vec::new();
        let mut cycle_lengths: Vec<usize> = Vec::new();
        let look_ahead = n / 2;

        for i in 0..n.saturating_sub(6) {
            let mut best_dist = f64::MAX;
            let mut best_j = i;
            // Search forward at least 2 steps
            for j in (i + 2)..n.min(i + look_ahead) {
                let dist = shared.dist(i, j);
                if dist < best_dist {
                    best_dist = dist;
                    best_j = j;
                }
            }
            if best_j > i {
                recurrence_dists.push(best_dist);
                cycle_lengths.push(best_j - i);
            }
        }

        let mean_recurrence = if recurrence_dists.is_empty() {
            f64::MAX
        } else {
            recurrence_dists.iter().sum::<f64>() / recurrence_dists.len() as f64
        };

        // 2. Fixed-point detection: points near the global centroid
        let mut centroid = vec![0.0_f64; dims];
        for r in 0..n {
            for di in 0..dims {
                centroid[di] += data[r * d + di];
            }
        }
        for c in &mut centroid {
            *c /= n as f64;
        }
        let mut fixed_points = 0u32;
        let nn_median = {
            let mut dists: Vec<f64> = (0..n.min(100))
                .map(|i| {
                    let mut min_d = f64::MAX;
                    for j in 0..n {
                        if i != j {
                            let d = shared.dist(i, j);
                            if d < min_d { min_d = d; }
                        }
                    }
                    min_d
                })
                .collect::<Vec<_>>();
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            dists[dists.len() / 2]
        };
        for r in 0..n {
            let mut dist_sq = 0.0_f64;
            for di in 0..dims {
                let diff = data[r * d + di] - centroid[di];
                dist_sq += diff * diff;
            }
            if dist_sq.sqrt() < nn_median * 0.5 {
                fixed_points += 1;
            }
        }

        // 3. Cycle length distribution → n=6 alignment
        let mut n6_cycles = 0u32;
        let n6_values: &[usize] = &[1, 2, 3, 4, 5, 6, 8, 10, 12, 24];
        for &cl in &cycle_lengths {
            if n6_values.contains(&cl) {
                n6_cycles += 1;
            }
        }
        let cycle_n6_ratio = if cycle_lengths.is_empty() {
            0.0
        } else {
            n6_cycles as f64 / cycle_lengths.len() as f64
        };

        // 4. Scale self-similarity: compare first/second half structure
        let mid = n / 2;
        let mut self_sim = 0.0_f64;
        let check = mid.min(50);
        for i in 0..check {
            for j in (i + 1)..check {
                let d1 = shared.dist(i, j);
                let d2 = shared.dist(i + mid, (j + mid).min(n - 1));
                if d1 > 1e-12 {
                    self_sim += 1.0 - ((d1 - d2).abs() / d1).min(1.0);
                }
            }
        }
        let pairs = (check * (check - 1) / 2).max(1);
        let scale_similarity = self_sim / pairs as f64;

        // Composite self-reference score
        let self_reference_score = scale_similarity * 0.3
            + cycle_n6_ratio * 0.3
            + (fixed_points as f64 / n as f64).min(0.3) * 0.2
            + (1.0 / (1.0 + mean_recurrence)).min(1.0) * 0.2;

        let mut result = HashMap::new();
        result.insert("self_reference_score".into(), vec![self_reference_score]);
        result.insert("mean_recurrence_dist".into(), vec![mean_recurrence]);
        result.insert("fixed_points".into(), vec![fixed_points as f64]);
        result.insert("cycle_n6_ratio".into(), vec![cycle_n6_ratio]);
        result.insert("scale_similarity".into(), vec![scale_similarity]);
        result.insert(
            "mean_cycle_length".into(),
            vec![if cycle_lengths.is_empty() {
                0.0
            } else {
                cycle_lengths.iter().sum::<usize>() as f64 / cycle_lengths.len() as f64
            }],
        );
        result
    }
}
