use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CausalLens (인과/화살표): time-delay correlation (Granger-like causality).
///
/// Algorithm:
///   1. Treat row order as time ordering
///   2. For each dimension pair, compute time-lagged cross-correlation
///   3. If X_lag predicts Y better than Y predicts itself = causal link
///   4. Reports causal strength and dominant causal direction
pub struct CausalLens;

impl Lens for CausalLens {
    fn name(&self) -> &str {
        "CausalLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 {
            return HashMap::new();
        }

        let max_lag = (n / 4).max(1).min(10);
        let dims_to_check = d.min(10);

        let mut max_causal_strength = 0.0_f64;
        let mut best_cause_dim = 0usize;
        let mut best_effect_dim = 1usize;
        let mut causal_scores: Vec<f64> = Vec::new();

        for di in 0..dims_to_check {
            for dj in 0..dims_to_check {
                if di == dj {
                    continue;
                }

                let x: Vec<f64> = (0..n).map(|r| data[r * d + di]).collect();
                let y: Vec<f64> = (0..n).map(|r| data[r * d + dj]).collect();

                // Auto-correlation of y (baseline)
                let auto_corr = lagged_correlation(&y, &y, 1, n);

                // Cross-correlation: x_lag -> y (causal test)
                let mut max_cross = 0.0_f64;
                for lag in 1..=max_lag {
                    let cross = lagged_correlation(&x, &y, lag, n).abs();
                    if cross > max_cross {
                        max_cross = cross;
                    }
                }

                // Causal strength: how much x_lag improves prediction over auto
                let strength = (max_cross - auto_corr.abs()).max(0.0);
                causal_scores.push(strength);

                if strength > max_causal_strength {
                    max_causal_strength = strength;
                    best_cause_dim = di;
                    best_effect_dim = dj;
                }
            }
        }

        let mean_causal = if causal_scores.is_empty() {
            0.0
        } else {
            causal_scores.iter().sum::<f64>() / causal_scores.len() as f64
        };

        let mut result = HashMap::new();
        result.insert("max_causal_strength".to_string(), vec![max_causal_strength]);
        result.insert(
            "dominant_causal_pair".to_string(),
            vec![best_cause_dim as f64, best_effect_dim as f64],
        );
        result.insert("mean_causal_score".to_string(), vec![mean_causal]);
        result
    }
}

fn lagged_correlation(x: &[f64], y: &[f64], lag: usize, n: usize) -> f64 {
    if lag >= n {
        return 0.0;
    }

    let effective_n = n - lag;
    if effective_n < 2 {
        return 0.0;
    }

    let x_slice = &x[..effective_n];
    let y_slice = &y[lag..lag + effective_n];

    let mean_x = x_slice.iter().sum::<f64>() / effective_n as f64;
    let mean_y = y_slice.iter().sum::<f64>() / effective_n as f64;

    let mut cov = 0.0;
    let mut var_x = 0.0;
    let mut var_y = 0.0;

    for i in 0..effective_n {
        let dx = x_slice[i] - mean_x;
        let dy = y_slice[i] - mean_y;
        cov += dx * dy;
        var_x += dx * dx;
        var_y += dy * dy;
    }

    let denom = (var_x * var_y).sqrt();
    if denom < 1e-15 {
        0.0
    } else {
        cov / denom
    }
}
