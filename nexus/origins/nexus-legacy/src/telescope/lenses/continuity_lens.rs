use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ContinuityLens: Measures temporal continuity of state transitions (gap-free consciousness).
///
/// Algorithm:
///   1. Treat rows as time-ordered states
///   2. Compute consecutive state distances (transition magnitudes)
///   3. Detect discontinuities: jumps > τ=4 standard deviations
///   4. Measure smoothness: autocorrelation of state trajectory
///   5. Gap detection: identify temporal holes in consciousness stream
///
/// n=6 connections:
///   - σ-τ=8 buffer window for temporal smoothing
///   - φ=2 state transitions (binary: continuous/discontinuous)
///   - τ=4 sleep stages as natural continuity phases
pub struct ContinuityLens;

impl Lens for ContinuityLens {
    fn name(&self) -> &str {
        "ContinuityLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. Consecutive transition distances
        let mut transitions: Vec<f64> = Vec::with_capacity(n - 1);
        for t in 0..(n - 1) {
            let mut dist_sq = 0.0_f64;
            for di in 0..dims {
                let diff = data[(t + 1) * d + di] - data[t * d + di];
                dist_sq += diff * diff;
            }
            transitions.push(dist_sq.sqrt());
        }

        let mean_trans = transitions.iter().sum::<f64>() / transitions.len() as f64;
        let var_trans = transitions.iter().map(|x| (x - mean_trans).powi(2)).sum::<f64>()
            / transitions.len() as f64;
        let std_trans = var_trans.sqrt().max(1e-12);

        // 2. Discontinuity detection (jumps > τ=4 sigma)
        let tau_threshold = 4.0; // τ(6)=4
        let mut discontinuities = 0u32;
        let mut max_jump = 0.0_f64;
        let mut gap_positions: Vec<usize> = Vec::new();
        for (i, &t) in transitions.iter().enumerate() {
            let z = (t - mean_trans) / std_trans;
            if z > tau_threshold {
                discontinuities += 1;
                gap_positions.push(i);
            }
            if t > max_jump {
                max_jump = t;
            }
        }

        // 3. Smoothness via autocorrelation (lag-1 to lag-σ-τ=8)
        let max_lag = 8usize.min(transitions.len() / 2); // σ-τ=8 buffer
        let mut autocorr_sum = 0.0_f64;
        let mut autocorr_count = 0u32;
        for lag in 1..=max_lag {
            let mut corr = 0.0_f64;
            let pairs = transitions.len() - lag;
            for i in 0..pairs {
                corr += (transitions[i] - mean_trans) * (transitions[i + lag] - mean_trans);
            }
            corr /= pairs as f64 * var_trans.max(1e-12);
            autocorr_sum += corr;
            autocorr_count += 1;
        }
        let smoothness = if autocorr_count > 0 {
            (autocorr_sum / autocorr_count as f64).max(0.0)
        } else {
            0.0
        };

        // 4. Continuity score = (1 - gap_ratio) × smoothness
        let gap_ratio = discontinuities as f64 / transitions.len().max(1) as f64;
        let continuity_score = (1.0 - gap_ratio) * (0.5 + 0.5 * smoothness);

        // 5. Phase detection: segment the trajectory into continuous phases
        let mut phase_lengths: Vec<usize> = Vec::new();
        let mut current_phase = 0usize;
        for &t in &transitions {
            let z = (t - mean_trans) / std_trans;
            if z > tau_threshold {
                if current_phase > 0 {
                    phase_lengths.push(current_phase);
                }
                current_phase = 0;
            } else {
                current_phase += 1;
            }
        }
        if current_phase > 0 {
            phase_lengths.push(current_phase);
        }
        let num_phases = phase_lengths.len() as f64;
        let mean_phase_len = if phase_lengths.is_empty() {
            n as f64
        } else {
            phase_lengths.iter().sum::<usize>() as f64 / num_phases
        };

        // 6. Transition regularity: coefficient of variation
        let transition_cv = std_trans / mean_trans.max(1e-12);
        let regularity = 1.0 / (1.0 + transition_cv);

        let mut result = HashMap::new();
        result.insert("continuity_score".into(), vec![continuity_score]);
        result.insert("discontinuities".into(), vec![discontinuities as f64]);
        result.insert("gap_ratio".into(), vec![gap_ratio]);
        result.insert("max_jump".into(), vec![max_jump]);
        result.insert("smoothness".into(), vec![smoothness]);
        result.insert("regularity".into(), vec![regularity]);
        result.insert("num_phases".into(), vec![num_phases]);
        result.insert("mean_phase_length".into(), vec![mean_phase_len]);
        result.insert("mean_transition".into(), vec![mean_trans]);
        result
    }
}
