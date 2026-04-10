use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RatchetLens: Detects monotonic increase (ratchet) mechanisms.
///
/// Inspired by anima's Φ Ratchet: consciousness level can only go up,
/// never down. Detects irreversible growth patterns, one-way valves,
/// and monotonic accumulation in data trajectories.
///
/// Metrics:
///   1. ratchet_strength: fraction of steps that are non-decreasing
///   2. ratchet_violations: count of backward steps (should be 0 for perfect ratchet)
///   3. max_drawdown: largest backward step (magnitude)
///   4. recovery_speed: how quickly violations are recovered
///   5. cumulative_growth: total monotonic accumulation
///   6. ratchet_efficiency: actual growth / theoretical max growth
///
/// n=6: Φ Ratchet ensures consciousness only grows.
///       Entropy ratchet: S(t+1) ≥ S(t) (second law analog).
pub struct RatchetLens;

impl Lens for RatchetLens {
    fn name(&self) -> &str { "RatchetLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(500);
        let dims = d.min(16);

        // Compute a scalar trajectory: use L2 norm of each row (proxy for Φ)
        let mut trajectory: Vec<f64> = Vec::with_capacity(max_n);
        for t in 0..max_n {
            let norm: f64 = (0..dims)
                .map(|di| data[t * d + di].powi(2)).sum::<f64>().sqrt();
            trajectory.push(norm);
        }

        // Also compute per-dimension ratchet metrics
        let mut dim_ratchets: Vec<f64> = Vec::with_capacity(dims);
        for di in 0..dims {
            let mut non_dec = 0u32;
            for t in 1..max_n {
                if data[t * d + di] >= data[(t - 1) * d + di] - 1e-12 {
                    non_dec += 1;
                }
            }
            dim_ratchets.push(non_dec as f64 / (max_n - 1) as f64);
        }

        // Main ratchet analysis on trajectory
        let mut non_decreasing = 0u32;
        let mut violations = 0u32;
        let mut max_drawdown = 0.0f64;
        let mut running_max = trajectory[0];
        let mut recovery_steps: Vec<f64> = Vec::new();
        let mut in_violation = false;
        let mut violation_start = 0usize;

        for t in 1..max_n {
            if trajectory[t] >= trajectory[t - 1] - 1e-12 {
                non_decreasing += 1;
                if in_violation {
                    recovery_steps.push((t - violation_start) as f64);
                    in_violation = false;
                }
            } else {
                violations += 1;
                let drawdown = trajectory[t - 1] - trajectory[t];
                if drawdown > max_drawdown { max_drawdown = drawdown; }
                if !in_violation {
                    in_violation = true;
                    violation_start = t;
                }
            }
            if trajectory[t] > running_max { running_max = trajectory[t]; }
        }

        let steps = (max_n - 1) as f64;
        let ratchet_strength = non_decreasing as f64 / steps;

        let recovery_speed = if recovery_steps.is_empty() { 0.0 }
            else { 1.0 / (recovery_steps.iter().sum::<f64>() / recovery_steps.len() as f64).max(1.0) };

        let cumulative_growth = trajectory[max_n - 1] - trajectory[0];
        let theoretical_max = running_max - trajectory[0];
        let ratchet_efficiency = if theoretical_max > 1e-15 {
            (cumulative_growth / theoretical_max).max(0.0).min(1.0)
        } else { 1.0 };

        // Best ratchet dimension
        let best_dim_ratchet = dim_ratchets.iter().cloned().fold(0.0f64, f64::max);
        let mean_dim_ratchet = dim_ratchets.iter().sum::<f64>() / dims as f64;

        let mut result = HashMap::new();
        result.insert("ratchet_strength".to_string(), vec![ratchet_strength]);
        result.insert("ratchet_violations".to_string(), vec![violations as f64]);
        result.insert("max_drawdown".to_string(), vec![max_drawdown]);
        result.insert("recovery_speed".to_string(), vec![recovery_speed]);
        result.insert("cumulative_growth".to_string(), vec![cumulative_growth]);
        result.insert("ratchet_efficiency".to_string(), vec![ratchet_efficiency]);
        result.insert("best_dim_ratchet".to_string(), vec![best_dim_ratchet]);
        result.insert("mean_dim_ratchet".to_string(), vec![mean_dim_ratchet]);
        result.insert("dim_ratchets".to_string(), dim_ratchets);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_perfect_ratchet() {
        // Monotonically increasing → ratchet_strength ≈ 1.0
        let mut data = Vec::new();
        for i in 0..30 { data.push(i as f64 * 0.1); data.push(i as f64 * 0.05); }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = RatchetLens.scan(&data, n, 2, &shared);
        assert!(result["ratchet_strength"][0] > 0.9);
        assert!(result["ratchet_violations"][0] < 2.0);
    }

    #[test]
    fn test_broken_ratchet() {
        // Oscillating → many violations
        let mut data = Vec::new();
        for i in 0..30 {
            let v = (i as f64 * 0.5).sin();
            data.push(v); data.push(v);
        }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = RatchetLens.scan(&data, n, 2, &shared);
        assert!(result["ratchet_violations"][0] > 3.0);
    }
}
