use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// TimeReversalLens: Time-reversal symmetry (T-symmetry).
/// Measures T-symmetry score, irreversibility, arrow of time, detailed balance.
pub struct TimeReversalLens;

impl Lens for TimeReversalLens {
    fn name(&self) -> &str { "TimeReversalLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        let signal: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let reversed: Vec<f64> = signal.iter().rev().cloned().collect();

        // T-symmetry score: correlation between forward and time-reversed signal
        let mean_f = signal.iter().sum::<f64>() / max_n as f64;
        let mean_r = mean_f; // same values, just reversed
        let mut cov = 0.0f64;
        let mut var_f = 0.0f64;
        let mut var_r = 0.0f64;
        for i in 0..max_n {
            let df = signal[i] - mean_f;
            let dr = reversed[i] - mean_r;
            cov += df * dr;
            var_f += df * df;
            var_r += dr * dr;
        }
        let t_symmetry_score = if var_f > 1e-12 && var_r > 1e-12 {
            (cov / (var_f * var_r).sqrt()).clamp(-1.0, 1.0)
        } else {
            1.0 // constant signal is perfectly T-symmetric
        };

        // Irreversibility: entropy production = difference in transition entropy
        // forward transitions vs backward transitions
        let forward_diffs: Vec<f64> = signal.windows(2).map(|w| w[1] - w[0]).collect();
        let backward_diffs: Vec<f64> = reversed.windows(2).map(|w| w[1] - w[0]).collect();
        let ent_fwd = shannon_entropy(&forward_diffs, 10);
        let ent_bwd = shannon_entropy(&backward_diffs, 10);
        let irreversibility = (ent_fwd - ent_bwd).abs();

        // Arrow of time: asymmetry in third moment (skewness of increments)
        let mean_diff = forward_diffs.iter().sum::<f64>() / forward_diffs.len().max(1) as f64;
        let var_diff = forward_diffs.iter().map(|&x| (x - mean_diff).powi(2)).sum::<f64>()
            / forward_diffs.len().max(1) as f64;
        let std_diff = var_diff.sqrt().max(1e-12);
        let skew = forward_diffs.iter().map(|&x| ((x - mean_diff) / std_diff).powi(3)).sum::<f64>()
            / forward_diffs.len().max(1) as f64;
        let arrow_of_time = skew.abs();

        // Detailed balance: for transitions i->j, check if P(i->j) ~ P(j->i)
        // Discretize signal into bins and check transition matrix symmetry
        let n_bins = 5;
        let (lo, hi) = {
            let mut lo = f64::MAX;
            let mut hi = f64::MIN;
            for &v in &signal { if v < lo { lo = v; } if v > hi { hi = v; } }
            (lo, hi)
        };
        let range = (hi - lo).max(1e-12);
        let scale = (n_bins - 1) as f64 / range;
        let bins: Vec<usize> = signal.iter().map(|&v| ((v - lo) * scale) as usize).collect();

        let mut trans = vec![0u32; n_bins * n_bins];
        for i in 0..(max_n - 1) {
            let a = bins[i].min(n_bins - 1);
            let b = bins[i + 1].min(n_bins - 1);
            trans[a * n_bins + b] += 1;
        }
        let mut asym = 0.0f64;
        let mut total = 0.0f64;
        for a in 0..n_bins {
            for b in (a + 1)..n_bins {
                let fab = trans[a * n_bins + b] as f64;
                let fba = trans[b * n_bins + a] as f64;
                asym += (fab - fba).abs();
                total += fab + fba;
            }
        }
        let detailed_balance = if total > 0.0 { 1.0 - asym / total } else { 1.0 };

        let mut result = HashMap::new();
        result.insert("t_symmetry_score".to_string(), vec![t_symmetry_score]);
        result.insert("irreversibility".to_string(), vec![irreversibility]);
        result.insert("arrow_of_time".to_string(), vec![arrow_of_time]);
        result.insert("detailed_balance".to_string(), vec![detailed_balance]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_symmetric_signal() {
        // Palindromic signal should have high T-symmetry
        let mut data: Vec<f64> = (0..10).map(|i| i as f64).collect();
        let rev: Vec<f64> = data.iter().rev().cloned().collect();
        data.extend(rev);
        let shared = SharedData::compute(&data, 20, 1);
        let result = TimeReversalLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["t_symmetry_score"][0] > 0.5);
    }
}
