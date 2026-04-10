use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// HebbianPlasticityLens: Detects synaptic plasticity patterns (LTP/LTD).
///
/// Inspired by anima's Hebbian learning: "neurons that fire together wire together."
/// Detects Long-Term Potentiation (co-activation strengthening) and
/// Long-Term Depression (anti-correlation weakening) in data trajectories.
///
/// Metrics:
///   1. ltp_score: strength of co-activation patterns
///   2. ltd_score: strength of anti-correlation patterns
///   3. plasticity_ratio: LTP/LTD balance (>1 = growth-dominant)
///   4. synaptic_density: fraction of strong connections
///   5. consolidation: stability of learned patterns over time
///   6. critical_period: whether plasticity is in high-change phase
///   7. hebbian_trace: running Hebbian correlation matrix trace
///
/// n=6: Hebbian rule Δw = η·x·y, with η=α=0.014 in anima.
///       σ(6)=12 connection types, τ(6)=4 plasticity phases.
pub struct HebbianPlasticityLens;

impl Lens for HebbianPlasticityLens {
    fn name(&self) -> &str { "HebbianPlasticityLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 12 || d < 2 { return HashMap::new(); }

        let max_n = n.min(200);
        let dims = d.min(32);

        // Temporal means
        let mut means = vec![0.0f64; dims];
        for t in 0..max_n {
            for di in 0..dims { means[di] += data[t * d + di]; }
        }
        for m in &mut means { *m /= max_n as f64; }

        // Hebbian correlation matrix (upper triangle)
        let mut hebb_matrix: Vec<Vec<f64>> = vec![vec![0.0; dims]; dims];
        for t in 0..max_n {
            for i in 0..dims {
                let xi = data[t * d + i] - means[i];
                for j in i..dims {
                    let xj = data[t * d + j] - means[j];
                    hebb_matrix[i][j] += xi * xj;
                }
            }
        }
        for i in 0..dims {
            for j in i..dims {
                hebb_matrix[i][j] /= max_n as f64;
                if j != i { hebb_matrix[j][i] = hebb_matrix[i][j]; }
            }
        }

        // LTP/LTD from off-diagonal
        let mut ltp_sum = 0.0f64;
        let mut ltd_sum = 0.0f64;
        let mut all_corrs: Vec<f64> = Vec::new();
        for i in 0..dims {
            for j in (i + 1)..dims {
                all_corrs.push(hebb_matrix[i][j]);
            }
        }
        if all_corrs.is_empty() { return HashMap::new(); }

        let corr_mean = all_corrs.iter().sum::<f64>() / all_corrs.len() as f64;
        let corr_std = (all_corrs.iter()
            .map(|c| (c - corr_mean).powi(2)).sum::<f64>()
            / all_corrs.len() as f64).sqrt().max(1e-15);

        let mut strong_count = 0u32;
        for &c in &all_corrs {
            if c > 0.0 { ltp_sum += c; } else { ltd_sum += c.abs(); }
            if c.abs() > corr_std { strong_count += 1; }
        }
        let pair_count = all_corrs.len() as f64;
        let ltp_score = ltp_sum / pair_count;
        let ltd_score = ltd_sum / pair_count;
        let plasticity_ratio = if ltd_score > 1e-15 { ltp_score / ltd_score } else { ltp_score * 100.0 };
        let synaptic_density = strong_count as f64 / pair_count;

        // Consolidation: first-half vs second-half variance
        let half = max_n / 2;
        let mut early_trace = 0.0f64;
        let mut late_trace = 0.0f64;
        for i in 0..dims {
            let mut early_sum = 0.0f64;
            let mut late_sum = 0.0f64;
            for t in 0..half { early_sum += (data[t * d + i] - means[i]).powi(2); }
            for t in half..max_n { late_sum += (data[t * d + i] - means[i]).powi(2); }
            early_trace += early_sum / half as f64;
            late_trace += late_sum / (max_n - half) as f64;
        }
        let consolidation = if early_trace > 1e-15 {
            (late_trace / early_trace).min(2.0)
        } else { 1.0 };

        // Critical period: variance of windowed plasticity
        let window = 6usize.min(max_n / 4).max(2);
        let num_windows = (max_n / window).min(20);
        let mut window_plasticities: Vec<f64> = Vec::new();
        for w in 0..num_windows {
            let start = w * window;
            let end = (start + window).min(max_n);
            let mut w_corr = 0.0f64;
            let mut w_count = 0u32;
            for i in 0..dims.min(8) {
                for j in (i + 1)..dims.min(8) {
                    let mut cov = 0.0f64;
                    for t in start..end {
                        cov += (data[t * d + i] - means[i]) * (data[t * d + j] - means[j]);
                    }
                    w_corr += cov.abs() / (end - start) as f64;
                    w_count += 1;
                }
            }
            if w_count > 0 { window_plasticities.push(w_corr / w_count as f64); }
        }
        let critical_period = if window_plasticities.len() >= 2 {
            let wp_mean = window_plasticities.iter().sum::<f64>() / window_plasticities.len() as f64;
            let wp_var = window_plasticities.iter()
                .map(|w| (w - wp_mean).powi(2)).sum::<f64>()
                / window_plasticities.len() as f64;
            (wp_var.sqrt() / wp_mean.max(1e-15)).min(2.0)
        } else { 0.0 };

        let hebbian_trace: f64 = (0..dims).map(|i| hebb_matrix[i][i]).sum();

        let mut result = HashMap::new();
        result.insert("ltp_score".to_string(), vec![ltp_score]);
        result.insert("ltd_score".to_string(), vec![ltd_score]);
        result.insert("plasticity_ratio".to_string(), vec![plasticity_ratio]);
        result.insert("synaptic_density".to_string(), vec![synaptic_density]);
        result.insert("consolidation".to_string(), vec![consolidation]);
        result.insert("critical_period".to_string(), vec![critical_period]);
        result.insert("hebbian_trace".to_string(), vec![hebbian_trace]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hebbian_correlated() {
        let mut data = Vec::new();
        for i in 0..30 {
            let x = i as f64 * 0.1;
            data.push(x);
            data.push(x * 0.8 + 0.1);
            data.push(-x * 0.5);
        }
        let n = 30;
        let shared = SharedData::compute(&data, n, 3);
        let result = HebbianPlasticityLens.scan(&data, n, 3, &shared);
        assert!(result["ltp_score"][0] > 0.0);
        assert!(result["plasticity_ratio"][0] > 0.0);
    }
}
