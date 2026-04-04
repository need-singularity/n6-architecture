use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{self, SharedData};

/// EntropyProductionLens: Measure irreversibility in data evolution.
///
/// Computes entropy production rate by comparing forward vs backward
/// transition probabilities. High entropy production = far from equilibrium.
/// n=6 connection: R(6) = sigma*phi / (n*tau) = 1 (perfect reversibility),
/// thermodynamic arrow tied to n=6 uniqueness.
pub struct EntropyProductionLens;

impl Lens for EntropyProductionLens {
    fn name(&self) -> &str { "EntropyProductionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 8 || d == 0 { return HashMap::new(); }

        // Treat data as time series (rows = time steps)
        let signal: Vec<f64> = (0..n).map(|i| data[i * d]).collect();

        // Forward entropy: H(x_{t+1} | x_t)
        // Backward entropy: H(x_{t-1} | x_t)
        // Entropy production = forward - backward

        let n_bins = 12; // sigma=12 bins

        // Bin the signal
        let (lo, hi) = shared_data::min_max(&signal);
        let range = (hi - lo).max(1e-12);
        let scale = (n_bins - 1) as f64 / range;
        let binned: Vec<usize> = signal.iter()
            .map(|&v| ((v - lo) * scale) as usize)
            .map(|b| b.min(n_bins - 1))
            .collect();

        // Forward transition matrix
        let mut fwd = vec![vec![0u32; n_bins]; n_bins];
        let mut bwd = vec![vec![0u32; n_bins]; n_bins];
        for t in 0..(n - 1) {
            fwd[binned[t]][binned[t + 1]] += 1;
            bwd[binned[t + 1]][binned[t]] += 1;
        }

        // Compute conditional entropies
        let fwd_entropy = conditional_entropy(&fwd, n_bins);
        let bwd_entropy = conditional_entropy(&bwd, n_bins);

        // Entropy production rate
        let entropy_production = (fwd_entropy - bwd_entropy).abs();

        // Reversibility score: R(6) = 1 means perfect reversibility
        let reversibility = (-entropy_production * 6.0).exp(); // n=6 scaling

        // KL divergence between forward and backward
        let kl_fwd_bwd = transition_kl(&fwd, &bwd, n_bins);

        // Steady state entropy (stationary distribution entropy)
        let mut state_counts = vec![0u32; n_bins];
        for &b in &binned {
            state_counts[b] += 1;
        }
        let state_entropy = {
            let total = n as f64;
            state_counts.iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / total;
                    -p * p.ln()
                })
                .sum::<f64>()
        };

        let mut result = HashMap::new();
        result.insert("entropy_production_rate".to_string(), vec![entropy_production]);
        result.insert("reversibility_score".to_string(), vec![reversibility]);
        result.insert("forward_conditional_entropy".to_string(), vec![fwd_entropy]);
        result.insert("backward_conditional_entropy".to_string(), vec![bwd_entropy]);
        result.insert("kl_divergence_fwd_bwd".to_string(), vec![kl_fwd_bwd]);
        result.insert("stationary_entropy".to_string(), vec![state_entropy]);
        result
    }
}

fn conditional_entropy(trans: &[Vec<u32>], n_bins: usize) -> f64 {
    let mut h = 0.0;
    for i in 0..n_bins {
        let row_sum: u32 = trans[i].iter().sum();
        if row_sum == 0 { continue; }
        let inv = 1.0 / row_sum as f64;
        for j in 0..n_bins {
            if trans[i][j] > 0 {
                let p = trans[i][j] as f64 * inv;
                h -= (row_sum as f64 / trans.iter().map(|r| r.iter().sum::<u32>()).sum::<u32>() as f64)
                    * p * p.ln();
            }
        }
    }
    h
}

fn transition_kl(p: &[Vec<u32>], q: &[Vec<u32>], n_bins: usize) -> f64 {
    let mut kl = 0.0;
    for i in 0..n_bins {
        let p_sum: u32 = p[i].iter().sum();
        let q_sum: u32 = q[i].iter().sum();
        if p_sum == 0 { continue; }
        for j in 0..n_bins {
            if p[i][j] > 0 && q[i][j] > 0 {
                let pp = p[i][j] as f64 / p_sum as f64;
                let qq = q[i][j] as f64 / q_sum.max(1) as f64;
                kl += pp * (pp / qq).ln();
            }
        }
    }
    kl / n_bins as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entropy_production_monotone() {
        // Monotonically increasing: highly irreversible
        let n = 30;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = EntropyProductionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("entropy_production_rate"));
    }

    #[test]
    fn test_entropy_production_symmetric() {
        // Symmetric oscillation: more reversible
        let n = 30;
        let d = 1;
        let data: Vec<f64> = (0..n).map(|i| {
            (2.0 * std::f64::consts::PI * i as f64 / 6.0).sin()
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = EntropyProductionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("reversibility_score"));
    }
}
