use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// QuantumJumpLens: Discrete state transition (quantum jump) detection.
///
/// Detects discontinuous jumps between discrete states in data — analogous
/// to quantum jumps where a system instantaneously transitions between
/// energy levels with no intermediate states.
///
/// Metrics:
///   1. jump_count: number of detected discontinuous transitions
///   2. jump_magnitude: mean magnitude of jumps (normalized)
///   3. dwell_time: mean time spent in each state before jumping
///   4. state_count: number of distinct quasi-stable states
///   5. jump_rate: jumps per unit time (transition frequency)
///   6. quantum_discreteness: how discrete (vs continuous) the transitions are
///
/// n=6: Bohr model has n=1,2,3... energy levels; hydrogen Balmer series
///       visible lines = τ=4. Transition rates ~ 1/n³. Energy spacing for
///       n=6 shell: E = -13.6/36 eV. Quantum numbers {n,l,m,s} = τ=4 values.
pub struct QuantumJumpLens;

impl Lens for QuantumJumpLens {
    fn name(&self) -> &str { "QuantumJumpLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 10 { return HashMap::new(); }

        // Use first column as primary signal
        let ts: Vec<f64> = (0..n).map(|i| data[i * d]).collect();

        // 1. Detect jumps: points where |Δx| >> median |Δx|
        let mut deltas: Vec<f64> = Vec::with_capacity(n - 1);
        for i in 1..n {
            deltas.push((ts[i] - ts[i - 1]).abs());
        }
        if deltas.is_empty() { return HashMap::new(); }

        let mut sorted_deltas = deltas.clone();
        sorted_deltas.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let median_delta = sorted_deltas[sorted_deltas.len() / 2];
        let jump_threshold = (median_delta * 3.0).max(1e-12);

        // Identify jumps
        let mut jump_indices: Vec<usize> = Vec::new();
        let mut jump_magnitudes: Vec<f64> = Vec::new();
        for (i, &d) in deltas.iter().enumerate() {
            if d > jump_threshold {
                jump_indices.push(i);
                jump_magnitudes.push(d);
            }
        }

        let jump_count = jump_indices.len();
        let mean_magnitude = if jump_count > 0 {
            jump_magnitudes.iter().sum::<f64>() / jump_count as f64
        } else {
            0.0
        };

        // 2. Dwell time: gaps between consecutive jumps
        let mut dwell_times = Vec::new();
        if jump_indices.len() >= 2 {
            for i in 1..jump_indices.len() {
                dwell_times.push((jump_indices[i] - jump_indices[i - 1]) as f64);
            }
        } else if jump_count == 1 {
            dwell_times.push(n as f64);
        }
        let mean_dwell = if dwell_times.is_empty() {
            n as f64
        } else {
            dwell_times.iter().sum::<f64>() / dwell_times.len() as f64
        };

        // 3. State count: cluster the values between jumps
        let mut segments: Vec<f64> = Vec::new();
        let mut prev = 0;
        for &ji in &jump_indices {
            if ji > prev {
                let seg_mean = ts[prev..=ji].iter().sum::<f64>() / (ji - prev + 1) as f64;
                segments.push(seg_mean);
            }
            prev = ji + 1;
        }
        if prev < n {
            let seg_mean = ts[prev..n].iter().sum::<f64>() / (n - prev) as f64;
            segments.push(seg_mean);
        }

        // Count distinct states (cluster segment means)
        let state_count = count_distinct_states(&segments, jump_threshold * 0.5);

        // 4. Jump rate
        let jump_rate = jump_count as f64 / n as f64;

        // 5. Quantum discreteness: ratio of jump variance to continuous variance
        // High discreteness = jumps are sharp, not gradual
        let continuous_var = if !deltas.is_empty() {
            let mean_d = deltas.iter().sum::<f64>() / deltas.len() as f64;
            deltas.iter().map(|d| (d - mean_d) * (d - mean_d)).sum::<f64>() / deltas.len() as f64
        } else {
            0.0
        };

        let jump_var = if !jump_magnitudes.is_empty() {
            let mean_j = jump_magnitudes.iter().sum::<f64>() / jump_magnitudes.len() as f64;
            jump_magnitudes.iter().map(|j| (j - mean_j) * (j - mean_j)).sum::<f64>()
                / jump_magnitudes.len() as f64
        } else {
            0.0
        };

        // Discreteness: if all jumps are similar size → high discreteness
        // (quantized energy levels produce uniform jump sizes)
        let discreteness = if continuous_var > 1e-15 {
            1.0 / (1.0 + jump_var / continuous_var)
        } else {
            if jump_count > 0 { 1.0 } else { 0.0 }
        };

        let mut result = HashMap::new();
        result.insert("jump_count".to_string(), vec![jump_count as f64]);
        result.insert("jump_magnitude".to_string(), vec![mean_magnitude]);
        result.insert("dwell_time".to_string(), vec![mean_dwell]);
        result.insert("state_count".to_string(), vec![state_count as f64]);
        result.insert("jump_rate".to_string(), vec![jump_rate]);
        result.insert("quantum_discreteness".to_string(), vec![discreteness]);
        result
    }
}

/// Count distinct states by simple 1D clustering.
fn count_distinct_states(values: &[f64], tolerance: f64) -> usize {
    if values.is_empty() { return 0; }
    let mut sorted = values.to_vec();
    sorted.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());

    let mut count = 1;
    let mut last = sorted[0];
    for &v in &sorted[1..] {
        if (v - last).abs() > tolerance {
            count += 1;
            last = v;
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_quantum_jump_staircase() {
        // Staircase signal: discrete jumps between levels
        let mut ts = Vec::new();
        for _ in 0..20 { ts.push(0.0); }  // state 0
        for _ in 0..20 { ts.push(5.0); }  // state 1 (jump!)
        for _ in 0..20 { ts.push(10.0); } // state 2 (jump!)
        for _ in 0..20 { ts.push(5.0); }  // back to state 1 (jump!)
        let n = ts.len();
        let shared = SharedData::compute(&ts, n, 1);
        let result = QuantumJumpLens.scan(&ts, n, 1, &shared);

        assert!(result.contains_key("jump_count"));
        assert!(result["jump_count"][0] >= 2.0, "Should detect jumps");
        assert!(result["state_count"][0] >= 2.0, "Should detect multiple states");
        assert!(result["quantum_discreteness"][0] > 0.3, "Uniform jumps = high discreteness");
    }

    #[test]
    fn test_quantum_jump_smooth() {
        // Smooth sine wave: no jumps
        let n = 80;
        let ts: Vec<f64> = (0..n)
            .map(|i| (i as f64 * 0.1).sin())
            .collect();
        let shared = SharedData::compute(&ts, n, 1);
        let result = QuantumJumpLens.scan(&ts, n, 1, &shared);

        // Smooth signal should have few/no jumps
        assert!(result["jump_rate"][0] < 0.2, "Smooth signal should have low jump rate");
    }
}
