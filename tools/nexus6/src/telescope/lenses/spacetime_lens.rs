use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// SpacetimeLens: Metric signature and causal structure analysis.
///
/// Analyzes the data manifold's metric structure, detecting Lorentzian-like
/// signatures (mixed +/- eigenvalues), light-cone structure, and causal
/// ordering in high-dimensional data.
///
/// Metrics:
///   1. metric_signature: ratio of positive to negative eigenvalue directions
///   2. lorentzian_score: how Lorentzian (vs Euclidean) the local metric is
///   3. causal_ordering: fraction of point pairs with clear temporal ordering
///   4. light_cone_width: spread of the null-like directions
///
/// n=6: Spacetime = 4D (3+1), but string theory lives in σ-μ=11D (10+1).
///       Extra n=6 compactified dimensions on Calabi-Yau with Euler number ~ J₂.
///       Minkowski metric signature (+---) has τ=4 components.
pub struct SpacetimeLens;

impl Lens for SpacetimeLens {
    fn name(&self) -> &str { "SpacetimeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let (means, vars) = mean_var(data, n, d);

        // 1. Covariance matrix (d x d) for metric signature
        let mut cov = vec![0.0f64; d * d];
        for i in 0..n {
            for di in 0..d {
                let a = data[i * d + di] - means[di];
                for dj in di..d {
                    let b = data[i * d + dj] - means[dj];
                    cov[di * d + dj] += a * b;
                    if di != dj {
                        cov[dj * d + di] += a * b;
                    }
                }
            }
        }
        let inv_n = 1.0 / n as f64;
        for v in cov.iter_mut() { *v *= inv_n; }

        // 2. Approximate eigenvalues via power iteration on diagonal dominance
        // (Full eigendecomposition is expensive; use diagonal + off-diagonal ratio)
        let mut positive_dims = 0u32;
        let mut negative_dims = 0u32;
        let mut eigenvalue_estimates = Vec::with_capacity(d);

        for di in 0..d {
            let diag = cov[di * d + di];
            // Off-diagonal contribution
            let off_diag_sum: f64 = (0..d)
                .filter(|&dj| dj != di)
                .map(|dj| cov[di * d + dj].abs())
                .sum();

            // Gershgorin circle: eigenvalue in [diag - off_sum, diag + off_sum]
            // If both bounds positive → positive eigenvalue
            // If lower bound negative and |lower| > upper → likely negative
            let lower = diag - off_diag_sum;
            let _upper = diag + off_diag_sum;

            if lower > 0.0 {
                positive_dims += 1;
                eigenvalue_estimates.push(diag);
            } else if diag < -1e-12 {
                negative_dims += 1;
                eigenvalue_estimates.push(diag);
            } else {
                // Near-zero: "null" direction
                eigenvalue_estimates.push(diag);
                if diag > 0.0 { positive_dims += 1; } else { negative_dims += 1; }
            }
        }

        let metric_signature = if (positive_dims + negative_dims) > 0 {
            positive_dims as f64 / (positive_dims + negative_dims) as f64
        } else {
            1.0
        };

        // Lorentzian score: closer to (d-1, 1) signature = more Lorentzian
        // Perfect Lorentzian: 1 negative + (d-1) positive
        let lorentzian_score = if d >= 2 && negative_dims >= 1 {
            let ideal_neg = 1.0;
            let actual_neg = negative_dims as f64;
            1.0 / (1.0 + (actual_neg - ideal_neg).abs())
        } else {
            0.0 // Pure Euclidean (all positive)
        };

        // 3. Causal ordering: check if first dimension can serve as "time"
        // (monotonically increasing along trajectory)
        let mut ordered_pairs = 0u64;
        let total_pairs = (n * (n - 1)) / 2;
        let max_check = n.min(100);
        let check_pairs = (max_check * (max_check - 1)) / 2;
        for i in 0..max_check {
            for j in (i + 1)..max_check {
                // Check if ordering in dim 0 implies ordering in dim 1
                let dt = columns[0][j] - columns[0][i];
                let dx = columns[1][j] - columns[1][i];
                if (dt > 0.0 && dx > 0.0) || (dt < 0.0 && dx < 0.0) || dt.abs() > dx.abs() {
                    ordered_pairs += 1;
                }
            }
        }
        let causal_ordering = if check_pairs > 0 {
            ordered_pairs as f64 / check_pairs as f64
        } else {
            0.0
        };

        // 4. Light cone width: variance ratio of "timelike" vs "spacelike" directions
        let max_var = vars.iter().cloned().fold(0.0f64, f64::max);
        let min_var = vars.iter().cloned().fold(f64::INFINITY, f64::min);
        let light_cone_width = if max_var > 1e-12 { min_var / max_var } else { 1.0 };

        let mut result = HashMap::new();
        result.insert("metric_signature".to_string(), vec![metric_signature]);
        result.insert("lorentzian_score".to_string(), vec![lorentzian_score]);
        result.insert("causal_ordering".to_string(), vec![causal_ordering]);
        result.insert("light_cone_width".to_string(), vec![light_cone_width]);
        result.insert("eigenvalue_estimates".to_string(), eigenvalue_estimates);
        result.insert("positive_dims".to_string(), vec![positive_dims as f64]);
        result.insert("negative_dims".to_string(), vec![negative_dims as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spacetime_lens_basic() {
        // Time-like data: dim0 increases monotonically, dim1 varies
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(i as f64);                          // "time"
            data.push((i as f64 * 0.3).sin() * 5.0);     // "space"
        }
        let n = 20;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = SpacetimeLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("metric_signature"));
        assert!(result.contains_key("causal_ordering"));
        // Monotonic time → high causal ordering
        assert!(result["causal_ordering"][0] > 0.5);
    }
}
