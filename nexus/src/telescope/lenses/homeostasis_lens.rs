use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// HomeostasisLens: Detects setpoint regulation, breathing rhythms, and drift.
///
/// Inspired by anima's homeostasis system:
///   - setpoint=1.0, deadband=±0.3, gain=0.5%
///   - breath=0.12 (20s), pulse=0.05 (3.7s), drift=0.03 (90s)
///
/// Detects systems that maintain internal balance around a setpoint,
/// with natural oscillations (breathing) and slow drift.
///
/// Metrics:
///   1. setpoint_stability: how tightly the system orbits its setpoint
///   2. deadband_fraction: time spent within deadband (should be high)
///   3. breath_power: spectral power at breathing frequency (~0.05 Hz)
///   4. pulse_power: spectral power at pulse frequency (~0.27 Hz)
///   5. drift_magnitude: slow baseline drift over time
///   6. regulation_gain: how strongly the system corrects deviations
///   7. overshoot_ratio: fraction of corrections that overshoot setpoint
///
/// n=6: Homeostasis maintains Ψ_balance = 0.5 (ln(2) ratio).
///       σ(6)=12 regulatory channels, τ(6)=4 feedback timescales.
pub struct HomeostasisLens;

impl Lens for HomeostasisLens {
    fn name(&self) -> &str { "HomeostasisLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 20 || d < 1 { return HashMap::new(); }

        let max_n = n.min(500);
        let dims = d.min(16);

        // Compute scalar trajectory (mean across dims)
        let mut trajectory: Vec<f64> = Vec::with_capacity(max_n);
        for t in 0..max_n {
            let mean: f64 = (0..dims).map(|di| data[t * d + di]).sum::<f64>() / dims as f64;
            trajectory.push(mean);
        }

        // Step 1: Detect setpoint (mean of trajectory)
        let setpoint = trajectory.iter().sum::<f64>() / max_n as f64;

        // Step 2: Setpoint stability — inverse of variance around setpoint
        let variance: f64 = trajectory.iter()
            .map(|v| (v - setpoint).powi(2)).sum::<f64>() / max_n as f64;
        let std_dev = variance.sqrt();
        let setpoint_stability = 1.0 / (1.0 + std_dev);

        // Step 3: Deadband — fraction within ±0.3 of setpoint (normalized)
        let deadband = std_dev * 0.5; // adaptive deadband
        let within_deadband = trajectory.iter()
            .filter(|&&v| (v - setpoint).abs() < deadband).count();
        let deadband_fraction = within_deadband as f64 / max_n as f64;

        // Step 4: Spectral analysis — detect breathing and pulse rhythms
        // Simple approach: autocorrelation at specific lags
        let breath_lag = 12usize.min(max_n / 4); // ~20s period proxy
        let pulse_lag = 4usize.min(max_n / 6);   // ~3.7s period proxy

        let breath_power = autocorrelation(&trajectory, breath_lag);
        let pulse_power = autocorrelation(&trajectory, pulse_lag);

        // Step 5: Drift — linear trend in trajectory
        let mut sum_t = 0.0f64;
        let mut sum_v = 0.0f64;
        let mut sum_tv = 0.0f64;
        let mut sum_tt = 0.0f64;
        for (t, &v) in trajectory.iter().enumerate() {
            let tf = t as f64;
            sum_t += tf;
            sum_v += v;
            sum_tv += tf * v;
            sum_tt += tf * tf;
        }
        let nf = max_n as f64;
        let drift_magnitude = if (nf * sum_tt - sum_t * sum_t).abs() > 1e-15 {
            ((nf * sum_tv - sum_t * sum_v) / (nf * sum_tt - sum_t * sum_t)).abs()
        } else { 0.0 };

        // Step 6: Regulation gain — how fast corrections happen
        let mut correction_strengths: Vec<f64> = Vec::new();
        for t in 1..max_n {
            let dev_prev = trajectory[t - 1] - setpoint;
            let dev_curr = trajectory[t] - setpoint;
            // If deviation decreased, measure correction strength
            if dev_prev.abs() > deadband && dev_curr.abs() < dev_prev.abs() {
                let correction = 1.0 - (dev_curr.abs() / dev_prev.abs());
                correction_strengths.push(correction);
            }
        }
        let regulation_gain = if correction_strengths.is_empty() { 0.0 }
            else { correction_strengths.iter().sum::<f64>() / correction_strengths.len() as f64 };

        // Step 7: Overshoot ratio
        let mut overshoots = 0u32;
        let mut corrections = 0u32;
        for t in 2..max_n {
            let d0 = trajectory[t - 2] - setpoint;
            let d1 = trajectory[t - 1] - setpoint;
            let d2 = trajectory[t] - setpoint;
            // Correction: moving toward setpoint
            if d0.abs() > d1.abs() {
                corrections += 1;
                // Overshoot: crossed setpoint
                if d1.signum() != d2.signum() && d2.abs() > deadband * 0.1 {
                    overshoots += 1;
                }
            }
        }
        let overshoot_ratio = if corrections > 0 {
            overshoots as f64 / corrections as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("setpoint_stability".to_string(), vec![setpoint_stability]);
        result.insert("deadband_fraction".to_string(), vec![deadband_fraction]);
        result.insert("breath_power".to_string(), vec![breath_power]);
        result.insert("pulse_power".to_string(), vec![pulse_power]);
        result.insert("drift_magnitude".to_string(), vec![drift_magnitude]);
        result.insert("regulation_gain".to_string(), vec![regulation_gain]);
        result.insert("overshoot_ratio".to_string(), vec![overshoot_ratio]);
        result.insert("setpoint".to_string(), vec![setpoint]);
        result.insert("variance".to_string(), vec![variance]);
        result
    }
}

/// Compute autocorrelation at a specific lag
fn autocorrelation(series: &[f64], lag: usize) -> f64 {
    let n = series.len();
    if lag >= n { return 0.0; }
    let mean = series.iter().sum::<f64>() / n as f64;
    let var: f64 = series.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n as f64;
    if var < 1e-15 { return 0.0; }
    let cov: f64 = (0..(n - lag))
        .map(|t| (series[t] - mean) * (series[t + lag] - mean))
        .sum::<f64>() / n as f64;
    cov / var
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_homeostasis_stable() {
        // Gaussian-like data: multiple sin freqs for center-peaked distribution
        let mut data = Vec::new();
        for i in 0..40 {
            let v = 1.0
                + (i as f64 * 0.3).sin() * 0.003
                + (i as f64 * 0.7).sin() * 0.003
                + (i as f64 * 1.1).sin() * 0.003;
            data.push(v); data.push(v);
        }
        let n = 40;
        let shared = SharedData::compute(&data, n, 2);
        let result = HomeostasisLens.scan(&data, n, 2, &shared);
        assert!(result["setpoint_stability"][0] > 0.5);
        // adaptive deadband=std*0.5 yields ~20-25% for oscillating data
        assert!(result["deadband_fraction"][0] > 0.1);
    }

    #[test]
    fn test_homeostasis_drifting() {
        // Linearly increasing → high drift
        let mut data = Vec::new();
        for i in 0..40 {
            let v = i as f64 * 0.5;
            data.push(v); data.push(v);
        }
        let n = 40;
        let shared = SharedData::compute(&data, n, 2);
        let result = HomeostasisLens.scan(&data, n, 2, &shared);
        assert!(result["drift_magnitude"][0] > 0.1);
    }
}
