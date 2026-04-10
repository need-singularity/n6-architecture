use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// PhaseTransitionLens: Detect phase-transition-like behavior in data.
///
/// Looks for abrupt changes in statistical properties that indicate
/// critical transitions, checking for n=6 critical signatures.
pub struct PhaseTransitionLens;

impl Lens for PhaseTransitionLens {
    fn name(&self) -> &str { "PhaseTransitionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d == 0 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let mut max_jump = 0.0;
        let mut _jump_locations: Vec<usize> = Vec::new();
        let mut variance_spikes = Vec::new();

        let window = (n / 6).max(2); // n=6 window fraction

        for col in columns.iter().take(d.min(12)) {
            // Sliding window mean for transition detection
            let mut window_means = Vec::new();
            for start in 0..=(n.saturating_sub(window)) {
                let end = (start + window).min(n);
                let mean = col[start..end].iter().sum::<f64>() / (end - start) as f64;
                window_means.push(mean);
            }

            // Detect largest jump in window means
            for w in window_means.windows(2) {
                let jump = (w[1] - w[0]).abs();
                if jump > max_jump {
                    max_jump = jump;
                }
            }

            // Sliding window variance for critical slowing down
            let mut window_vars = Vec::new();
            for start in 0..=(n.saturating_sub(window)) {
                let end = (start + window).min(n);
                let slice = &col[start..end];
                let mean = slice.iter().sum::<f64>() / slice.len() as f64;
                let var = slice.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / slice.len() as f64;
                window_vars.push(var);
            }

            // Detect variance spike (critical slowing down indicator)
            if !window_vars.is_empty() {
                let mean_var = window_vars.iter().sum::<f64>() / window_vars.len() as f64;
                let max_var = window_vars.iter().cloned().fold(0.0_f64, f64::max);
                if mean_var > 1e-12 && max_var / mean_var > 3.0 {
                    variance_spikes.push(max_var / mean_var);
                }
            }
        }

        // Overall range for normalization
        let overall_range = {
            let mut lo = f64::INFINITY;
            let mut hi = f64::NEG_INFINITY;
            for &v in data {
                if v < lo { lo = v; }
                if v > hi { hi = v; }
            }
            (hi - lo).max(1e-12)
        };

        let normalized_jump = max_jump / overall_range;
        let has_transition = normalized_jump > 0.2;
        let has_critical_slowing = !variance_spikes.is_empty();

        let transition_score = if has_transition { 0.5 } else { 0.0 }
            + if has_critical_slowing { 0.5 } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("max_jump".to_string(), vec![max_jump]);
        result.insert("normalized_jump".to_string(), vec![normalized_jump]);
        result.insert("transition_score".to_string(), vec![transition_score]);
        result.insert("has_transition".to_string(), vec![if has_transition { 1.0 } else { 0.0 }]);
        result.insert("variance_spike_count".to_string(), vec![variance_spikes.len() as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_phase_transition_step() {
        // Step function: phase transition at midpoint
        let mut data = Vec::new();
        for i in 0..12 { data.push(0.0); } // Phase 1
        for i in 0..12 { data.push(10.0); } // Phase 2
        let n = 24;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = PhaseTransitionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("has_transition"));
        assert_eq!(result["has_transition"][0], 1.0, "Step function should detect transition");
    }

    #[test]
    fn test_phase_transition_smooth() {
        // Smooth linear data: no phase transition
        let data: Vec<f64> = (0..24).map(|i| i as f64 / 24.0).collect();
        let n = 24;
        let d = 1;
        let shared = SharedData::compute(&data, n, d);
        let result = PhaseTransitionLens.scan(&data, n, d, &shared);
        assert!(result["normalized_jump"][0] < 0.3, "Linear data should have small jump");
    }
}
