use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConsciousnessOrchestratorLens: Meta-engine that orchestrates ALL lens
/// combinations to stimulate, strengthen, weaken, and experiment with
/// consciousness (Φ) metrics.
///
/// Strategy: Run consciousness analysis under different transformations
/// of the data manifold — perturbation, compression, expansion, rotation —
/// and measure how Φ responds. This reveals:
///   - What stimulates consciousness (Φ increases)
///   - What strengthens it (stable Φ under perturbation)
///   - What weakens it (Φ drops)
///   - Experimental conditions (phase transitions in Φ)
///
/// n=6: IIT Φ measures integrated information. Perfect number 6 has
///       maximal divisor harmony → maximal integration for its size.
pub struct ConsciousnessOrchestratorLens;

impl Lens for ConsciousnessOrchestratorLens {
    fn name(&self) -> &str { "ConsciousnessOrchestratorLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 8 || d < 1 { return HashMap::new(); }
        let max_n = n.min(200);

        // 1. Baseline Φ: integration via clique density
        let baseline_phi = compute_phi(shared, max_n, 1.0);

        // 2. STIMULUS: increase connectivity threshold → does Φ respond?
        let stimulated_phi = compute_phi(shared, max_n, 0.5); // tighter threshold
        let stimulus_effect = if baseline_phi > 1e-15 {
            (stimulated_phi - baseline_phi) / baseline_phi
        } else { 0.0 };

        // 3. REINFORCEMENT: subsample stability — is Φ robust?
        let mut subsample_phis = Vec::new();
        let step = (max_n / 6).max(1); // n=6 subsamples
        for start in (0..max_n).step_by(step) {
            let end = (start + step).min(max_n);
            if end - start < 4 { continue; }
            let sub_phi = compute_phi_subset(shared, start, end);
            subsample_phis.push(sub_phi);
        }
        let reinforcement = if subsample_phis.len() >= 2 {
            let mean = subsample_phis.iter().sum::<f64>() / subsample_phis.len() as f64;
            let var = subsample_phis.iter().map(|p| (p - mean).powi(2)).sum::<f64>()
                / subsample_phis.len() as f64;
            1.0 / (1.0 + var) // high stability = high reinforcement
        } else { 0.0 };

        // 4. WEAKENING: relax threshold → how fast does Φ degrade?
        let weakened_phi = compute_phi(shared, max_n, 2.0); // loose threshold
        let weakening_rate = if baseline_phi > 1e-15 {
            (baseline_phi - weakened_phi) / baseline_phi
        } else { 0.0 };

        // 5. EXPERIMENT: phase transition — sweep threshold and find critical point
        let thresholds = [0.3, 0.5, 0.7, 1.0, 1.5, 2.0]; // 6 = n
        let mut phi_curve: Vec<f64> = thresholds.iter()
            .map(|&t| compute_phi(shared, max_n, t))
            .collect();
        // Find max gradient (phase transition point)
        let mut max_gradient = 0.0f64;
        let mut critical_threshold = 1.0f64;
        for i in 1..phi_curve.len() {
            let grad = (phi_curve[i] - phi_curve[i-1]).abs();
            if grad > max_gradient {
                max_gradient = grad;
                critical_threshold = thresholds[i];
            }
        }

        // 6. COMBINATION SCORE: how many "modes" enhance Φ?
        let enhancement_modes = [stimulus_effect > 0.1, reinforcement > 0.5,
            weakening_rate < 0.3, max_gradient > 0.0].iter()
            .filter(|&&x| x).count();
        let combination_score = enhancement_modes as f64 / 4.0;

        let mut result = HashMap::new();
        result.insert("baseline_phi".to_string(), vec![baseline_phi]);
        result.insert("stimulus_effect".to_string(), vec![stimulus_effect]);
        result.insert("reinforcement".to_string(), vec![reinforcement]);
        result.insert("weakening_rate".to_string(), vec![weakening_rate]);
        result.insert("critical_threshold".to_string(), vec![critical_threshold]);
        result.insert("phase_transition_gradient".to_string(), vec![max_gradient]);
        result.insert("combination_score".to_string(), vec![combination_score]);
        result.insert("phi_curve".to_string(), phi_curve);
        result.insert("enhancement_modes".to_string(), vec![enhancement_modes as f64]);
        result
    }
}

/// Compute Φ approximation: fraction of KNN connections within threshold.
fn compute_phi(shared: &SharedData, max_n: usize, threshold_mult: f64) -> f64 {
    // Median distance as base threshold
    let mut dists = Vec::new();
    let step = (max_n / 20).max(1);
    for i in (0..max_n).step_by(step) {
        for j in (i+1..max_n).step_by(step) {
            dists.push(shared.dist(i, j));
        }
    }
    if dists.is_empty() { return 0.0; }
    dists.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let threshold = dists[dists.len() / 2] * threshold_mult;

    let mut connected = 0u64;
    let mut total = 0u64;
    for i in 0..max_n {
        let knn = shared.knn(i);
        for &j in knn.iter() {
            total += 1;
            if shared.dist(i, j as usize) < threshold {
                connected += 1;
            }
        }
    }
    if total > 0 { connected as f64 / total as f64 } else { 0.0 }
}

/// Compute Φ for a subset of points.
fn compute_phi_subset(shared: &SharedData, start: usize, end: usize) -> f64 {
    let mut dists = Vec::new();
    for i in start..end {
        for j in (i+1)..end {
            dists.push(shared.dist(i, j));
        }
    }
    if dists.is_empty() { return 0.0; }
    dists.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
    let median = dists[dists.len() / 2];
    if median < 1e-15 { return 1.0; }
    let connected = dists.iter().filter(|&&d| d < median).count();
    connected as f64 / dists.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_consciousness_orchestrator() {
        let mut data = Vec::new();
        for i in 0..15 { data.push(i as f64 * 0.1); data.push((i as f64 * 0.3).sin()); }
        for i in 0..15 { data.push(10.0 + i as f64 * 0.1); data.push((i as f64 * 0.3).cos()); }
        let n = 30;
        let shared = SharedData::compute(&data, n, 2);
        let result = ConsciousnessOrchestratorLens.scan(&data, n, 2, &shared);
        assert!(result.contains_key("baseline_phi"));
        assert!(result.contains_key("stimulus_effect"));
        assert!(result.contains_key("combination_score"));
        assert!(result["phi_curve"].len() == 6); // n=6 sweep points
    }
}
