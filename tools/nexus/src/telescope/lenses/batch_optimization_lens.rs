use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BatchOptimizationLens: Optimize batch size and composition for throughput and convergence.
///
/// Algorithm:
///   1. Compute local density per point via mean distance to neighbors
///   2. Partition data into density-based strata (low/mid/high density)
///   3. For candidate batch sizes (powers of 2), estimate:
///      - coverage: fraction of strata represented in a batch
///      - gradient variance proxy: intra-batch distance variance
///      - throughput score: batch_size / (1 + gradient_variance)
///   4. Select optimal batch size maximizing throughput * coverage
///   5. Compute composition ratios (how many from each stratum per batch)
pub struct UbatchUoptimizationLens;

impl Lens for UbatchUoptimizationLens {
    fn name(&self) -> &str {
        "batch_optimization"
    }

    fn category(&self) -> &str {
        "AI"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // 1. Compute local density per point: inverse mean distance to all others
        let mut densities = Vec::with_capacity(n);
        for i in 0..n {
            let mut sum = 0.0;
            for j in 0..n {
                if i != j {
                    sum += shared.dist(i, j);
                }
            }
            let mean_dist = sum / (n - 1) as f64;
            densities.push(if mean_dist > 1e-12 { 1.0 / mean_dist } else { 1e12 });
        }

        // 2. Partition into 3 strata by density terciles
        let mut sorted_densities = densities.clone();
        sorted_densities.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let t1 = sorted_densities[n / 3];
        let t2 = sorted_densities[2 * n / 3];

        let mut strata = vec![0u8; n]; // 0=low, 1=mid, 2=high
        let mut strata_counts = [0usize; 3];
        for i in 0..n {
            let s = if densities[i] < t1 {
                0
            } else if densities[i] < t2 {
                1
            } else {
                2
            };
            strata[i] = s;
            strata_counts[s as usize] += 1;
        }

        // 3. Compute per-feature variance for gradient noise estimation
        let mut feature_means = vec![0.0f64; d];
        for i in 0..n {
            let row = i * d;
            for j in 0..d {
                feature_means[j] += data[row + j];
            }
        }
        let inv_n = 1.0 / n as f64;
        for j in 0..d {
            feature_means[j] *= inv_n;
        }
        let mut total_variance = 0.0;
        for i in 0..n {
            let row = i * d;
            for j in 0..d {
                let diff = data[row + j] - feature_means[j];
                total_variance += diff * diff;
            }
        }
        total_variance /= (n * d) as f64;

        // 4. Evaluate candidate batch sizes
        let mut candidates: Vec<usize> = Vec::new();
        let mut bs = 2;
        while bs <= n {
            candidates.push(bs);
            bs *= 2;
        }
        if candidates.is_empty() {
            candidates.push(n);
        }

        let mut best_score = f64::NEG_INFINITY;
        let mut best_batch_size = candidates[0];
        let mut all_scores = Vec::with_capacity(candidates.len());

        for &batch_size in &candidates {
            // Estimate gradient variance reduction: var / batch_size (CLT approximation)
            let grad_var = total_variance / batch_size as f64;

            // Coverage: probability that all 3 strata appear in a random batch
            // Using inclusion-exclusion for hypergeometric sampling
            let coverage = if batch_size >= n {
                1.0
            } else {
                let mut missing_prob = 0.0;
                for s in 0..3 {
                    if strata_counts[s] == 0 {
                        continue;
                    }
                    // Probability stratum s is completely absent from batch of size batch_size
                    // = C(n - strata_counts[s], batch_size) / C(n, batch_size)
                    let absent = hypergeom_absent(n, strata_counts[s], batch_size);
                    missing_prob += absent;
                }
                (1.0 - missing_prob).max(0.0).min(1.0)
            };

            // Throughput: samples processed per unit gradient noise
            let throughput = batch_size as f64 / (1.0 + grad_var);

            // Combined score: throughput * coverage (balanced objective)
            let score = throughput * coverage;
            all_scores.push(score);

            if score > best_score {
                best_score = score;
                best_batch_size = batch_size;
            }
        }

        // 5. Compute optimal composition ratios for best batch
        let composition: Vec<f64> = strata_counts
            .iter()
            .map(|&c| (c as f64 / n as f64) * best_batch_size as f64)
            .collect();

        // 6. Compute diversity metric: mean pairwise distance within the dataset
        let pair_count = n * (n - 1) / 2;
        let mut dist_sum = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                dist_sum += shared.dist(i, j);
            }
        }
        let mean_pairwise_dist = dist_sum / pair_count as f64;

        // Convergence speed estimate: inversely proportional to gradient variance at optimal batch
        let optimal_grad_var = total_variance / best_batch_size as f64;
        let convergence_speed = 1.0 / (1.0 + optimal_grad_var);

        let mut result = HashMap::new();
        result.insert("optimal_batch_size".to_string(), vec![best_batch_size as f64]);
        result.insert("convergence_speed".to_string(), vec![convergence_speed]);
        result.insert("gradient_variance".to_string(), vec![optimal_grad_var]);
        result.insert("total_data_variance".to_string(), vec![total_variance]);
        result.insert("mean_pairwise_distance".to_string(), vec![mean_pairwise_dist]);
        result.insert("strata_counts".to_string(), strata_counts.iter().map(|&c| c as f64).collect());
        result.insert("composition_ratios".to_string(), composition);
        result.insert(
            "candidate_scores".to_string(),
            candidates.iter().zip(all_scores.iter()).flat_map(|(&b, &s)| vec![b as f64, s]).collect(),
        );
        result
    }
}

/// Approximate probability that a stratum of size `k` is completely absent
/// from a random batch of size `b` drawn from population `n`.
/// P(absent) = C(n-k, b) / C(n, b) = product_{i=0..b} (n-k-i)/(n-i)
fn hypergeom_absent(n: usize, k: usize, b: usize) -> f64 {
    if k == 0 || b == 0 {
        return 1.0;
    }
    if b > n - k {
        return 0.0;
    }
    let mut p = 1.0;
    for i in 0..b {
        p *= (n - k - i) as f64 / (n - i) as f64;
    }
    p
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    fn make_test_data() -> (Vec<f64>, usize, usize, SharedData) {
        // 10 points in 2D: two clusters + some outliers
        let data = vec![
            0.0, 0.0,
            0.1, 0.1,
            0.2, 0.0,
            0.0, 0.2,
            0.1, 0.15,
            5.0, 5.0,
            5.1, 5.1,
            5.2, 5.0,
            5.0, 5.2,
            10.0, 10.0, // outlier
        ];
        let n = 10;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        (data, n, d, shared)
    }

    #[test]
    fn test_scan_returns_nonempty() {
        let (data, n, d, shared) = make_test_data();
        let lens = UbatchUoptimizationLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan() must return non-empty result");
        assert!(result.contains_key("optimal_batch_size"));
        assert!(result.contains_key("convergence_speed"));
        assert!(result.contains_key("gradient_variance"));
        assert!(result.contains_key("strata_counts"));
        assert!(result.contains_key("composition_ratios"));

        let batch_size = result["optimal_batch_size"][0];
        assert!(batch_size >= 2.0, "batch size must be >= 2, got {batch_size}");
        assert!(batch_size <= n as f64, "batch size must be <= n");

        let speed = result["convergence_speed"][0];
        assert!(speed > 0.0 && speed <= 1.0, "convergence_speed must be in (0,1], got {speed}");
    }

    #[test]
    fn test_strata_composition_sums_to_batch() {
        let (data, n, d, shared) = make_test_data();
        let lens = UbatchUoptimizationLens;
        let result = lens.scan(&data, n, d, &shared);

        let batch_size = result["optimal_batch_size"][0];
        let composition = &result["composition_ratios"];
        let comp_sum: f64 = composition.iter().sum();
        assert!(
            (comp_sum - batch_size).abs() < 1e-9,
            "composition must sum to batch_size: {} vs {}",
            comp_sum,
            batch_size
        );
    }
}
