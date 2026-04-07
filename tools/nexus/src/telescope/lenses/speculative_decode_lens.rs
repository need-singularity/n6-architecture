use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SpeculativeDecodeLens: Detect speculative decoding opportunities for autoregressive speedup.
///
/// Speculative decoding accelerates autoregressive generation by using a cheap "draft"
/// model for predictable tokens and a full "verifier" for hard ones. This lens identifies
/// data regions amenable to drafting by measuring:
///
/// Algorithm:
///   1. Sequential prediction error: for each point i (i>=2), extrapolate from (i-2, i-1)
///      and measure how close the prediction is to the actual point i. Low error = high
///      predictability = good candidate for speculative decoding draft acceptance.
///   2. Local smoothness (Lipschitz ratio): ratio of consecutive distance changes.
///      Smooth sequences (ratio ≈ 1) are easier for a draft model to predict.
///   3. Acceptance rate: fraction of points where prediction error < median distance.
///      Maps directly to speculative decoding's draft token acceptance probability.
///   4. Theoretical speedup: 1 / (1 - α) where α = acceptance rate (geometric model).
///   5. Entropy of prediction errors: low entropy = consistent predictability pattern.
///   6. Neighbor continuation similarity: measures whether similar contexts produce
///      similar next tokens — the key property for draft model accuracy.
pub struct SpeculativeDecodeLens;

impl Lens for SpeculativeDecodeLens {
    fn name(&self) -> &str {
        "SpeculativeDecodeLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // --- 1. Compute median pairwise distance as reference scale ---
        let pair_count = n * (n - 1) / 2;
        let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_dist = all_dists[pair_count / 2].max(1e-12);

        // --- 2. Sequential prediction errors via linear extrapolation ---
        // For each point i >= 2, predict from points i-2, i-1:
        //   predicted[dim] = 2 * data[i-1][dim] - data[i-2][dim]
        // Then measure Euclidean distance from prediction to actual point i.
        let mut pred_errors: Vec<f64> = Vec::with_capacity(n - 2);
        for i in 2..n {
            let mut err_sq = 0.0;
            for dim in 0..d {
                let v_prev2 = data[(i - 2) * d + dim];
                let v_prev1 = data[(i - 1) * d + dim];
                let predicted = 2.0 * v_prev1 - v_prev2;
                let actual = data[i * d + dim];
                let diff = actual - predicted;
                err_sq += diff * diff;
            }
            pred_errors.push(err_sq.sqrt());
        }

        // --- 3. Local smoothness (Lipschitz ratio) ---
        // Ratio of consecutive step distances: dist(i, i+1) / dist(i-1, i).
        // Values near 1.0 indicate smooth, predictable progression.
        let mut smoothness_ratios: Vec<f64> = Vec::with_capacity(n - 2);
        for i in 1..(n - 1) {
            let d_prev = shared.dist(i - 1, i);
            let d_next = shared.dist(i, i + 1);
            if d_prev > 1e-12 {
                smoothness_ratios.push(d_next / d_prev);
            }
        }

        let mean_smoothness = if smoothness_ratios.is_empty() {
            1.0
        } else {
            smoothness_ratios.iter().sum::<f64>() / smoothness_ratios.len() as f64
        };

        // Smoothness consistency: std dev of log-ratios (lower = more uniform steps)
        let log_ratios: Vec<f64> = smoothness_ratios
            .iter()
            .filter(|&&r| r > 0.0)
            .map(|&r| r.ln())
            .collect();
        let smoothness_consistency = if log_ratios.len() >= 2 {
            let mean_lr = log_ratios.iter().sum::<f64>() / log_ratios.len() as f64;
            let var = log_ratios
                .iter()
                .map(|&x| (x - mean_lr).powi(2))
                .sum::<f64>()
                / log_ratios.len() as f64;
            var.sqrt()
        } else {
            0.0
        };

        // --- 4. Draft acceptance rate ---
        // Fraction of predictions within median_dist (normalized threshold).
        let threshold = median_dist * 0.5; // strict: half the median distance
        let accepted = pred_errors.iter().filter(|&&e| e < threshold).count();
        let acceptance_rate = accepted as f64 / pred_errors.len().max(1) as f64;

        // --- 5. Theoretical speedup (geometric model) ---
        // In speculative decoding with draft length K and acceptance rate α:
        //   E[accepted] = (1 - α^(K+1)) / (1 - α)
        // We report speedup for K=4 (common draft length):
        let alpha = acceptance_rate.min(0.999);
        let k_draft = 4.0;
        let expected_accepted = if alpha < 1e-6 {
            1.0
        } else {
            (1.0 - alpha.powf(k_draft + 1.0)) / (1.0 - alpha)
        };
        let speedup = expected_accepted; // tokens verified per step

        // --- 6. Prediction error entropy ---
        // Bin the prediction errors and compute Shannon entropy.
        // Low entropy = errors are concentrated = consistent draft quality.
        let n_bins = 10;
        let pred_entropy = if pred_errors.len() >= 2 {
            let (e_min, e_max) = pred_errors.iter().fold(
                (f64::INFINITY, f64::NEG_INFINITY),
                |(lo, hi), &v| (lo.min(v), hi.max(v)),
            );
            let range = (e_max - e_min).max(1e-12);
            let scale = (n_bins - 1) as f64 / range;
            let mut counts = vec![0u32; n_bins];
            for &e in &pred_errors {
                let bin = ((e - e_min) * scale) as usize;
                counts[bin.min(n_bins - 1)] += 1;
            }
            let inv_n = 1.0 / pred_errors.len() as f64;
            let mut entropy = 0.0;
            for &c in &counts {
                if c > 0 {
                    let p = c as f64 * inv_n;
                    entropy -= p * p.ln();
                }
            }
            entropy
        } else {
            0.0
        };

        // --- 7. Neighbor draft potential ---
        // For each point, check if its nearest neighbor's successor is close.
        // This measures whether similar contexts produce similar continuations
        // (key property for draft model accuracy).
        let mut neighbor_continuation_errors: Vec<f64> = Vec::new();
        for i in 0..(n - 1) {
            let knn = shared.knn(i);
            // Find nearest neighbor that also has a successor (index < n-1)
            let mut best_j: Option<usize> = None;
            for &k_idx in knn.iter() {
                let j = k_idx as usize;
                if j != i && j + 1 < n && j + 1 != i + 1 {
                    best_j = Some(j);
                    break;
                }
            }
            if let Some(j) = best_j {
                let continuation_dist = shared.dist(i + 1, j + 1);
                neighbor_continuation_errors.push(continuation_dist);
            }
        }

        let mean_continuation_error = if neighbor_continuation_errors.is_empty() {
            median_dist
        } else {
            neighbor_continuation_errors.iter().sum::<f64>()
                / neighbor_continuation_errors.len() as f64
        };

        // Normalized: lower = better draft potential
        let draft_quality = 1.0 - (mean_continuation_error / median_dist).min(1.0);

        // --- Assemble results ---
        let mut result = HashMap::new();
        result.insert("acceptance_rate".to_string(), vec![acceptance_rate]);
        result.insert("speedup_k4".to_string(), vec![speedup]);
        result.insert(
            "smoothness".to_string(),
            vec![mean_smoothness, smoothness_consistency],
        );
        result.insert("pred_error_entropy".to_string(), vec![pred_entropy]);
        result.insert("draft_quality".to_string(), vec![draft_quality]);
        result.insert("median_distance".to_string(), vec![median_dist]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    /// Linear sequence should be highly predictable (high acceptance rate, high speedup).
    #[test]
    fn test_linear_sequence_high_predictability() {
        // 8 points in 2D on a straight line: (0,0), (1,1), (2,2), ...
        let data: Vec<f64> = (0..8).flat_map(|i| vec![i as f64, i as f64]).collect();
        let n = 8;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = SpeculativeDecodeLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must not be empty");
        assert!(result.contains_key("acceptance_rate"));
        assert!(result.contains_key("speedup_k4"));
        assert!(result.contains_key("smoothness"));
        assert!(result.contains_key("draft_quality"));

        let acc = result["acceptance_rate"][0];
        assert!(
            acc > 0.5,
            "linear sequence should have high acceptance, got {acc}"
        );

        let speedup = result["speedup_k4"][0];
        assert!(speedup >= 1.0, "speedup must be >= 1.0, got {speedup}");
    }

    /// Random-looking data should have lower predictability than a smooth curve.
    #[test]
    fn test_noisy_vs_smooth() {
        // Smooth sine curve
        let n = 10;
        let d = 2;
        let smooth: Vec<f64> = (0..n)
            .flat_map(|i| {
                let t = i as f64 * 0.3;
                vec![t.sin(), t.cos()]
            })
            .collect();
        let shared_smooth = make_shared(&smooth, n, d);
        let lens = SpeculativeDecodeLens;
        let r_smooth = lens.scan(&smooth, n, d, &shared_smooth);

        // Jagged / high-variance sequence
        let noisy: Vec<f64> = (0..n)
            .flat_map(|i| {
                let sign = if i % 2 == 0 { 1.0 } else { -1.0 };
                vec![sign * (i as f64) * 3.7, sign * (i as f64) * 2.1]
            })
            .collect();
        let shared_noisy = make_shared(&noisy, n, d);
        let r_noisy = lens.scan(&noisy, n, d, &shared_noisy);

        assert!(!r_smooth.is_empty());
        assert!(!r_noisy.is_empty());

        let smooth_acc = r_smooth["acceptance_rate"][0];
        let noisy_acc = r_noisy["acceptance_rate"][0];
        // Smooth should generally be more predictable
        assert!(
            smooth_acc >= noisy_acc || (smooth_acc - noisy_acc).abs() < 0.3,
            "smooth({smooth_acc}) should be >= noisy({noisy_acc}) or close"
        );

        // Both must have valid speedup
        assert!(r_smooth["speedup_k4"][0] >= 1.0);
        assert!(r_noisy["speedup_k4"][0] >= 1.0);
    }
}
