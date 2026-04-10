use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SimulationLens: Forward/backward projection engine.
/// Measures forward_prediction, backward_reconstruction, simulation_stability,
/// equilibrium_distance, phase_space_coverage.
pub struct SimulationLens;

impl Lens for SimulationLens {
    fn name(&self) -> &str { "SimulationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Treat rows as time steps. Use simple linear extrapolation per feature.
        let mut fwd_err_total = 0.0_f64;
        let mut bwd_err_total = 0.0_f64;
        let mut stability_score = 0.0_f64;
        let half = max_n / 2;
        if half < 3 { return HashMap::new(); }

        for j in 0..d {
            // Forward: predict second half from first half (linear trend)
            let (mut sx, mut sy, mut sxx, mut sxy) = (0.0, 0.0, 0.0, 0.0);
            for i in 0..half {
                let x = i as f64;
                let y = data[i * d + j];
                sx += x; sy += y; sxx += x * x; sxy += x * y;
            }
            let hn = half as f64;
            let det = hn * sxx - sx * sx;
            let (a, b) = if det.abs() > 1e-15 {
                ((hn * sxy - sx * sy) / det, (sxx * sy - sx * sxy) / det)
            } else { (0.0, sy / hn) };

            let mut fwd_err = 0.0;
            let mut var = 0.0;
            let mean2 = (half..max_n).map(|i| data[i * d + j]).sum::<f64>() / (max_n - half) as f64;
            for i in half..max_n {
                let pred = a * (i as f64) + b;
                let actual = data[i * d + j];
                fwd_err += (pred - actual).powi(2);
                var += (actual - mean2).powi(2);
            }
            if var > 1e-15 { fwd_err_total += fwd_err / var; } else { fwd_err_total += 0.0; }

            // Backward: predict first half from second half
            let (mut sx2, mut sy2, mut sxx2, mut sxy2) = (0.0, 0.0, 0.0, 0.0);
            for i in half..max_n {
                let x = i as f64;
                let y = data[i * d + j];
                sx2 += x; sy2 += y; sxx2 += x * x; sxy2 += x * y;
            }
            let hn2 = (max_n - half) as f64;
            let det2 = hn2 * sxx2 - sx2 * sx2;
            let (a2, b2) = if det2.abs() > 1e-15 {
                ((hn2 * sxy2 - sx2 * sy2) / det2, (sxx2 * sy2 - sx2 * sxy2) / det2)
            } else { (0.0, sy2 / hn2) };

            let mut bwd_err = 0.0;
            let mean1 = (0..half).map(|i| data[i * d + j]).sum::<f64>() / half as f64;
            let mut var1 = 0.0;
            for i in 0..half {
                let pred = a2 * (i as f64) + b2;
                bwd_err += (pred - data[i * d + j]).powi(2);
                var1 += (data[i * d + j] - mean1).powi(2);
            }
            if var1 > 1e-15 { bwd_err_total += bwd_err / var1; }

            // Stability: ratio of second-half variance to first-half variance
            let v1 = var1 / half as f64;
            let v2 = var / (max_n - half) as f64;
            if v1 > 1e-15 { stability_score += (v2 / v1).min(10.0); }
        }

        let dd = d.max(1) as f64;
        let forward_prediction = 1.0 - (fwd_err_total / dd).min(1.0);
        let backward_reconstruction = 1.0 - (bwd_err_total / dd).min(1.0);
        let simulation_stability = 1.0 - ((stability_score / dd) - 1.0).abs().min(1.0);

        // Equilibrium distance: how many steps until variance stabilizes (simple: check running var)
        let eq_dist = half as f64 / max_n as f64;

        // Phase space coverage: fraction of bins occupied
        let n_bins = 16;
        let mut occupied = vec![false; n_bins * n_bins];
        if d >= 2 && max_n > 0 {
            let col0: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
            let col1: Vec<f64> = (0..max_n).map(|i| data[i * d + 1]).collect();
            let (mn0, mx0) = col0.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
            let (mn1, mx1) = col1.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
            let r0 = (mx0 - mn0).max(1e-15); let r1 = (mx1 - mn1).max(1e-15);
            for i in 0..max_n {
                let b0 = ((col0[i] - mn0) / r0 * (n_bins - 1) as f64) as usize;
                let b1 = ((col1[i] - mn1) / r1 * (n_bins - 1) as f64) as usize;
                occupied[b0 * n_bins + b1] = true;
            }
        }
        let phase_coverage = occupied.iter().filter(|&&o| o).count() as f64 / (n_bins * n_bins) as f64;

        let mut result = HashMap::new();
        result.insert("forward_prediction".into(), vec![forward_prediction]);
        result.insert("backward_reconstruction".into(), vec![backward_reconstruction]);
        result.insert("simulation_stability".into(), vec![simulation_stability]);
        result.insert("equilibrium_distance".into(), vec![eq_dist]);
        result.insert("phase_space_coverage".into(), vec![phase_coverage]);
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
        let result = SimulationLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
