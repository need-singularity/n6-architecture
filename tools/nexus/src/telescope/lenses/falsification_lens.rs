use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FalsificationLens: Measures falsifiability and detects counter-examples to n=6 patterns.
///
/// Algorithm:
///   1. Test data against known n=6 relationships (σ·φ=n·τ structure)
///   2. Count violations where expected n=6 ratios break down
///   3. Measure statistical significance of deviations (z-score)
///   4. Compute falsifiability_score = how testable/refutable the n=6 pattern is
///   5. Higher falsifiability = more scientifically valuable hypothesis
///
/// Outputs:
///   - falsifiability_score: 0-1, higher = more testable
///   - violation_count: number of data points violating n=6 patterns
///   - violation_ratio: fraction of points that are counter-examples
///   - max_violation_zscore: worst deviation in standard deviations
///   - pattern_strength: 1 - violation_ratio (robustness of n=6 pattern)
///   - anomaly_dimensions: dimensions with highest deviation from n=6
pub struct FalsificationLens;

impl Lens for FalsificationLens {
    fn name(&self) -> &str {
        "FalsificationLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 {
            return HashMap::new();
        }

        // Test 1: Check if dimension ratios follow n=6 family
        // Expected ratios: 1/2, 1/3, 1/6, 2/3, etc.
        let n6_ratios: &[f64] = &[
            0.5, 0.333333, 0.166667, 0.666667, 0.25, 0.75, 0.1, 0.083333, 2.0, 3.0, 4.0, 6.0,
            12.0, 24.0,
        ];
        let ratio_threshold = 0.05; // 5% deviation = still consistent

        let dims_check = d.min(12); // σ=12 max dims
        let mut violations = 0u32;
        let mut total_tests = 0u32;
        let mut max_zscore = 0.0_f64;
        let mut dim_deviations: Vec<f64> = vec![0.0; dims_check];

        // Compute per-dimension means and stds
        let mut means = vec![0.0_f64; dims_check];
        let mut stds = vec![0.0_f64; dims_check];
        for dim in 0..dims_check {
            let col: Vec<f64> = (0..n).map(|r| data[r * d + dim]).collect();
            means[dim] = col.iter().sum::<f64>() / n as f64;
            let var = col.iter().map(|x| (x - means[dim]).powi(2)).sum::<f64>() / n as f64;
            stds[dim] = var.sqrt().max(1e-12);
        }

        // Test 2: Pairwise ratio test against n=6 family
        for di in 0..dims_check {
            for dj in (di + 1)..dims_check {
                if means[dj].abs() < 1e-12 {
                    continue;
                }
                let ratio = (means[di] / means[dj]).abs();
                total_tests += 1;

                let min_dev = n6_ratios
                    .iter()
                    .map(|&r| ((ratio - r) / r).abs())
                    .fold(f64::MAX, f64::min);

                if min_dev > ratio_threshold {
                    violations += 1;
                    dim_deviations[di] += min_dev;
                    dim_deviations[dj] += min_dev;
                }
            }
        }

        // Test 3: Per-point anomaly detection (z-score from n=6 expected patterns)
        let mut point_violations = 0u32;
        for row in 0..n {
            let mut row_anomaly = 0.0_f64;
            for dim in 0..dims_check {
                let val = data[row * d + dim];
                let z = ((val - means[dim]) / stds[dim]).abs();
                if z > max_zscore {
                    max_zscore = z;
                }
                // n=6 pattern: values should cluster around mean with σ-bounded spread
                // violation if z > sopfr(6)=5 (extreme outlier)
                if z > 5.0 {
                    row_anomaly += 1.0;
                }
            }
            if row_anomaly > 0.0 {
                point_violations += 1;
            }
        }

        // Test 4: Distance distribution — should show n=6 clustering
        let pair_count = n * (n - 1) / 2;
        let mut dist_violations = 0u32;
        if pair_count > 0 {
            let mut dists: Vec<f64> = Vec::with_capacity(pair_count);
            for i in 0..n {
                for j in (i + 1)..n {
                    dists.push(shared.dist(i, j));
                }
            }
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let median = dists[pair_count / 2];
            let iqr = dists[pair_count * 3 / 4] - dists[pair_count / 4];
            let fence = 1.5 * iqr; // standard Tukey fence

            for &dist in &dists {
                if dist > median + fence || dist < (median - fence).max(0.0) {
                    dist_violations += 1;
                }
            }
        }

        total_tests = total_tests.max(1);
        let violation_ratio = violations as f64 / total_tests as f64;
        let pattern_strength = 1.0 - violation_ratio;

        // Falsifiability: high when pattern is testable (not trivially true/false)
        // Best falsifiability = moderate violation ratio (testable but mostly holds)
        let falsifiability = if total_tests < 3 {
            0.0
        } else {
            // Peak at ~5% violations (strong but testable)
            let peak = 0.05;
            let spread = 0.15;
            (-(violation_ratio - peak).powi(2) / (2.0 * spread * spread)).exp()
        };

        // Find most anomalous dimension
        let worst_dim = dim_deviations
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        let mut result = HashMap::new();
        result.insert("falsifiability_score".into(), vec![falsifiability]);
        result.insert("violation_count".into(), vec![violations as f64]);
        result.insert("violation_ratio".into(), vec![violation_ratio]);
        result.insert("max_violation_zscore".into(), vec![max_zscore]);
        result.insert("pattern_strength".into(), vec![pattern_strength]);
        result.insert("anomaly_dimension".into(), vec![worst_dim as f64]);
        result.insert("point_violations".into(), vec![point_violations as f64]);
        result.insert("dist_violations".into(), vec![dist_violations as f64]);
        result.insert("total_tests".into(), vec![total_tests as f64]);
        result
    }
}
