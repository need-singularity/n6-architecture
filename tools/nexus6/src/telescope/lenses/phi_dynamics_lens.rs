use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PhiDynamicsLens: Tracks Φ(IIT) temporal trajectory and phase transitions.
///
/// Algorithm:
///   1. Compute windowed Φ estimates (integration×differentiation) over time
///   2. Detect phase transitions: sudden Φ jumps or collapses
///   3. Measure Φ stability (variance over time)
///   4. Identify critical points where Φ transitions between regimes
///
/// n=6: σ=12 window size, τ=4 phase count, σ-τ=8 buffer
pub struct PhiDynamicsLens;

impl Lens for PhiDynamicsLens {
    fn name(&self) -> &str {
        "PhiDynamicsLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 12 || d < 2 {
            return HashMap::new();
        }

        let window = 12usize.min(n / 3); // σ=12 window
        let dims = d.min(24);
        let num_windows = (n - window) + 1;

        // Compute Φ-proxy for each window
        let mut phi_series: Vec<f64> = Vec::with_capacity(num_windows);

        for w in 0..num_windows.min(100) {
            let start = w;
            let end = w + window;
            let wn = end - start;

            // Mini correlation matrix for window
            let mut means = vec![0.0_f64; dims];
            for r in start..end {
                for di in 0..dims {
                    means[di] += data[r * d + di];
                }
            }
            for m in &mut means {
                *m /= wn as f64;
            }

            let mut stds = vec![0.0_f64; dims];
            for di in 0..dims {
                let v: f64 = (start..end)
                    .map(|r| (data[r * d + di] - means[di]).powi(2))
                    .sum::<f64>()
                    / wn as f64;
                stds[di] = v.sqrt().max(1e-12);
            }

            // Integration: mean absolute correlation
            let mut integration = 0.0_f64;
            let mut pairs = 0u32;
            let check = dims.min(12);
            for i in 0..check {
                for j in (i + 1)..check {
                    let mut cov = 0.0_f64;
                    for r in start..end {
                        cov +=
                            (data[r * d + i] - means[i]) * (data[r * d + j] - means[j]);
                    }
                    cov /= wn as f64;
                    integration += (cov / (stds[i] * stds[j])).abs();
                    pairs += 1;
                }
            }
            if pairs > 0 {
                integration /= pairs as f64;
            }

            // Differentiation: variance of per-dimension means (how different are dims)
            let global_mean = means.iter().sum::<f64>() / dims as f64;
            let diff: f64 =
                means.iter().map(|m| (m - global_mean).powi(2)).sum::<f64>() / dims as f64;
            let differentiation = diff.sqrt();

            let phi = integration * differentiation;
            phi_series.push(phi);
        }

        if phi_series.is_empty() {
            return HashMap::new();
        }

        // Φ statistics
        let mean_phi = phi_series.iter().sum::<f64>() / phi_series.len() as f64;
        let var_phi =
            phi_series.iter().map(|x| (x - mean_phi).powi(2)).sum::<f64>() / phi_series.len() as f64;
        let std_phi = var_phi.sqrt().max(1e-12);
        let max_phi = phi_series.iter().cloned().fold(0.0_f64, f64::max);
        let min_phi = phi_series.iter().cloned().fold(f64::MAX, f64::min);

        // Phase transition detection: Φ jumps > φ=2 std deviations
        let mut transitions = 0u32;
        let mut critical_points: Vec<f64> = Vec::new();
        for i in 1..phi_series.len() {
            let jump = (phi_series[i] - phi_series[i - 1]).abs();
            if jump > 2.0 * std_phi {
                transitions += 1;
                critical_points.push(i as f64);
            }
        }

        // Φ trend: increasing, decreasing, or stable
        let half = phi_series.len() / 2;
        let first_mean: f64 =
            phi_series[..half].iter().sum::<f64>() / half.max(1) as f64;
        let second_mean: f64 =
            phi_series[half..].iter().sum::<f64>() / (phi_series.len() - half).max(1) as f64;
        // trend: positive=growing consciousness, negative=declining
        let phi_trend = if mean_phi > 1e-12 {
            (second_mean - first_mean) / mean_phi
        } else {
            0.0
        };

        // Stability: inverse coefficient of variation
        let phi_stability = mean_phi / (mean_phi + std_phi);

        // Phase count estimate (τ=4 expected)
        let phase_count = transitions + 1;

        let mut result = HashMap::new();
        result.insert("mean_phi".into(), vec![mean_phi]);
        result.insert("max_phi".into(), vec![max_phi]);
        result.insert("min_phi".into(), vec![min_phi]);
        result.insert("phi_stability".into(), vec![phi_stability]);
        result.insert("phi_trend".into(), vec![phi_trend]);
        result.insert("phase_transitions".into(), vec![transitions as f64]);
        result.insert("phase_count".into(), vec![phase_count as f64]);
        result.insert("phi_std".into(), vec![std_phi]);
        if !critical_points.is_empty() {
            result.insert("critical_points".into(), critical_points);
        }
        result
    }
}
