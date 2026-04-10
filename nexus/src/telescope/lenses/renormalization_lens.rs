use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RenormalizationLens: Apply scale-elimination to extract effective parameters.
///
/// Inspired by the renormalization group in physics, this lens iteratively
/// coarse-grains data by merging nearest-neighbor pairs and tracking how
/// statistical properties (variance, correlation length, effective dimension)
/// flow across scales.
///
/// Algorithm:
///   1. Start with all N points as individual "blocks"
///   2. At each RG step, merge the two closest blocks (weighted centroid)
///   3. Track observables at each scale: variance, mean distance, block count
///   4. Compute the RG flow: how observables change per coarse-graining step
///   5. Extract effective parameters:
///      - critical_exponent: log-log slope of variance vs block count (nu)
///      - anomalous_dimension: deviation from mean-field scaling
///      - fixed_point_proximity: how close the flow converges to a fixed point
///      - correlation_length: scale at which variance plateaus
pub struct RenormalizationLens;

impl Lens for RenormalizationLens {
    fn name(&self) -> &str {
        "RenormalizationLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // Initialize blocks: each point is its own block
        let mut block_centers: Vec<Vec<f64>> = (0..n)
            .map(|i| {
                let start = i * d;
                data[start..start + d].to_vec()
            })
            .collect();
        let mut block_sizes: Vec<usize> = vec![1; n];

        // Track observables at each RG scale
        let mut scale_variance: Vec<f64> = Vec::new();
        let mut scale_mean_dist: Vec<f64> = Vec::new();
        let mut scale_block_count: Vec<f64> = Vec::new();

        // Record initial scale
        scale_variance.push(compute_variance(&block_centers, d));
        scale_mean_dist.push(compute_mean_interblock_dist(&block_centers, d));
        scale_block_count.push(block_centers.len() as f64);

        // Use pre-computed distances for the first merge step
        // After that, recompute from centroids
        let max_steps = n - 2; // stop when 2 blocks remain
        let steps = max_steps.min(n - 1);

        for step in 0..steps {
            let num_blocks = block_centers.len();
            if num_blocks < 3 {
                break;
            }

            // Find closest pair of blocks
            let (mi, mj) = if step == 0 && num_blocks == n {
                // Use shared distance matrix for initial step
                find_closest_pair_shared(n, shared)
            } else {
                find_closest_pair_direct(&block_centers, d)
            };

            // Merge blocks mi and mj (weighted centroid)
            let size_i = block_sizes[mi];
            let size_j = block_sizes[mj];
            let total = size_i + size_j;
            let mut merged = vec![0.0; d];
            for dim in 0..d {
                merged[dim] = (block_centers[mi][dim] * size_i as f64
                    + block_centers[mj][dim] * size_j as f64)
                    / total as f64;
            }

            // Remove the higher index first to avoid shifting
            let (remove_first, remove_second) = if mi > mj { (mi, mj) } else { (mj, mi) };
            block_centers.remove(remove_first);
            block_sizes.remove(remove_first);
            block_centers[remove_second] = merged;
            block_sizes[remove_second] = total;

            // Record observables at this scale
            scale_variance.push(compute_variance(&block_centers, d));
            scale_mean_dist.push(compute_mean_interblock_dist(&block_centers, d));
            scale_block_count.push(block_centers.len() as f64);
        }

        // Extract effective parameters from the RG flow

        // 1. Critical exponent (nu): log-log slope of variance vs block count
        //    variance ~ N^(-1/nu) in the scaling regime
        let critical_exponent = compute_log_log_slope(&scale_block_count, &scale_variance);

        // 2. Anomalous dimension (eta): deviation of distance scaling from mean-field
        //    mean_dist ~ N^(-(2-eta)/d_eff) => eta measures anomalous scaling
        let dist_slope = compute_log_log_slope(&scale_block_count, &scale_mean_dist);
        let mean_field_slope = -1.0 / d.max(1) as f64;
        let anomalous_dimension = if mean_field_slope.abs() > 1e-12 {
            (dist_slope - mean_field_slope) / mean_field_slope.abs()
        } else {
            0.0
        };

        // 3. Fixed-point proximity: how stable the flow becomes at late steps
        //    Measure the ratio of consecutive variance changes — convergence to 1.0 = fixed point
        let fixed_point_proximity = compute_fixed_point_proximity(&scale_variance);

        // 4. Correlation length: scale (as fraction of N) where variance change flattens
        let correlation_length = compute_correlation_length(&scale_variance, n);

        // 5. RG flow curve: normalized variance at each scale for detailed analysis
        let max_var = scale_variance
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);
        let rg_flow: Vec<f64> = if max_var > 1e-15 {
            scale_variance.iter().map(|v| v / max_var).collect()
        } else {
            scale_variance.clone()
        };

        let mut result = HashMap::new();
        result.insert("critical_exponent".to_string(), vec![critical_exponent]);
        result.insert("anomalous_dimension".to_string(), vec![anomalous_dimension]);
        result.insert(
            "fixed_point_proximity".to_string(),
            vec![fixed_point_proximity],
        );
        result.insert("correlation_length".to_string(), vec![correlation_length]);
        result.insert("rg_flow".to_string(), rg_flow);
        result
    }
}

/// Compute total variance of block centers across all dimensions.
fn compute_variance(centers: &[Vec<f64>], d: usize) -> f64 {
    if centers.is_empty() {
        return 0.0;
    }
    let n = centers.len() as f64;
    let mut total_var = 0.0;
    for dim in 0..d {
        let mean = centers.iter().map(|c| c[dim]).sum::<f64>() / n;
        let var = centers.iter().map(|c| (c[dim] - mean).powi(2)).sum::<f64>() / n;
        total_var += var;
    }
    total_var
}

/// Compute mean inter-block Euclidean distance.
fn compute_mean_interblock_dist(centers: &[Vec<f64>], d: usize) -> f64 {
    let n = centers.len();
    if n < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut count = 0;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut dist_sq = 0.0;
            for dim in 0..d {
                let diff = centers[i][dim] - centers[j][dim];
                dist_sq += diff * diff;
            }
            sum += dist_sq.sqrt();
            count += 1;
        }
    }
    sum / count as f64
}

/// Find closest pair using the shared (pre-computed) distance matrix.
fn find_closest_pair_shared(n: usize, shared: &SharedData) -> (usize, usize) {
    let mut best_i = 0;
    let mut best_j = 1;
    let mut best_dist = shared.dist(0, 1);
    for i in 0..n {
        for j in (i + 1)..n {
            let d = shared.dist(i, j);
            if d < best_dist {
                best_dist = d;
                best_i = i;
                best_j = j;
            }
        }
    }
    (best_i, best_j)
}

/// Find closest pair by direct computation from block centers.
fn find_closest_pair_direct(centers: &[Vec<f64>], d: usize) -> (usize, usize) {
    let n = centers.len();
    let mut best_i = 0;
    let mut best_j = 1;
    let mut best_dist = f64::MAX;
    for i in 0..n {
        for j in (i + 1)..n {
            let mut dist_sq = 0.0;
            for dim in 0..d {
                let diff = centers[i][dim] - centers[j][dim];
                dist_sq += diff * diff;
            }
            let dist = dist_sq.sqrt();
            if dist < best_dist {
                best_dist = dist;
                best_i = i;
                best_j = j;
            }
        }
    }
    (best_i, best_j)
}

/// Compute log-log slope via least-squares on log(x) vs log(y).
/// Filters out non-positive values.
fn compute_log_log_slope(x: &[f64], y: &[f64]) -> f64 {
    let pairs: Vec<(f64, f64)> = x
        .iter()
        .zip(y.iter())
        .filter(|(&xi, &yi)| xi > 0.0 && yi > 0.0)
        .map(|(&xi, &yi)| (xi.ln(), yi.ln()))
        .collect();

    if pairs.len() < 2 {
        return 0.0;
    }

    let n = pairs.len() as f64;
    let sum_x: f64 = pairs.iter().map(|(x, _)| x).sum();
    let sum_y: f64 = pairs.iter().map(|(_, y)| y).sum();
    let sum_xy: f64 = pairs.iter().map(|(x, y)| x * y).sum();
    let sum_xx: f64 = pairs.iter().map(|(x, _)| x * x).sum();

    let denom = n * sum_xx - sum_x * sum_x;
    if denom.abs() < 1e-15 {
        return 0.0;
    }
    (n * sum_xy - sum_x * sum_y) / denom
}

/// Measure convergence to a fixed point by checking ratio stability at late steps.
/// Returns value in [0, 1]: 1.0 = perfect fixed point, 0.0 = no convergence.
fn compute_fixed_point_proximity(variances: &[f64]) -> f64 {
    if variances.len() < 4 {
        return 0.0;
    }
    // Look at the last third of the flow
    let start = variances.len() * 2 / 3;
    let tail = &variances[start..];
    if tail.len() < 2 {
        return 0.0;
    }

    // Compute consecutive ratios
    let mut ratios: Vec<f64> = Vec::new();
    for i in 1..tail.len() {
        if tail[i - 1].abs() > 1e-15 {
            ratios.push(tail[i] / tail[i - 1]);
        }
    }
    if ratios.is_empty() {
        return 0.0;
    }

    // Fixed point = ratios converge to a constant
    let mean_ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;
    let ratio_var = ratios
        .iter()
        .map(|r| (r - mean_ratio).powi(2))
        .sum::<f64>()
        / ratios.len() as f64;

    // Low variance in ratios => close to fixed point
    // Map variance to [0, 1] via exp(-var * scale)
    let proximity = (-ratio_var * 10.0).exp();
    proximity.clamp(0.0, 1.0)
}

/// Find the scale (as fraction of N) where variance change drops below threshold.
fn compute_correlation_length(variances: &[f64], n: usize) -> f64 {
    if variances.len() < 3 {
        return 1.0;
    }

    // Compute absolute relative change at each step
    let total_range = variances[0] - variances[variances.len() - 1];
    if total_range.abs() < 1e-15 {
        return 0.0; // no change => zero correlation length
    }

    let threshold = 0.05; // 5% of total range per step
    for i in 1..variances.len() {
        let change = (variances[i] - variances[i - 1]).abs() / total_range.abs();
        if change < threshold {
            // Scale = fraction of original N at which flow flattens
            return (n - i) as f64 / n as f64;
        }
    }
    1.0 // never flattened
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    /// Helper: create data and SharedData for testing.
    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_renormalization_clustered_data() {
        // Two tight clusters far apart: should show clear scaling behavior
        // Cluster 1: points near (0, 0)
        // Cluster 2: points near (10, 10)
        let data = vec![
            0.0, 0.0, 0.1, 0.1, 0.2, 0.0, 0.0, 0.2, // cluster 1
            10.0, 10.0, 10.1, 10.1, 10.2, 10.0, 10.0, 10.2, // cluster 2
        ];
        let n = 8;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = RenormalizationLens;
        let result = lens.scan(&data, n, d, &shared);

        // Must return non-empty with all expected keys
        assert!(!result.is_empty(), "result should not be empty");
        assert!(result.contains_key("critical_exponent"));
        assert!(result.contains_key("anomalous_dimension"));
        assert!(result.contains_key("fixed_point_proximity"));
        assert!(result.contains_key("correlation_length"));
        assert!(result.contains_key("rg_flow"));

        // RG flow should have multiple entries (one per coarse-graining step + initial)
        let rg_flow = &result["rg_flow"];
        assert!(rg_flow.len() >= 3, "rg_flow should have at least 3 entries");

        // Critical exponent should be a finite number
        let nu = result["critical_exponent"][0];
        assert!(nu.is_finite(), "critical_exponent must be finite");

        // Fixed-point proximity should be in [0, 1]
        let fp = result["fixed_point_proximity"][0];
        assert!(fp >= 0.0 && fp <= 1.0, "fixed_point_proximity in [0,1]");
    }

    #[test]
    fn test_renormalization_uniform_line() {
        // Points on a line: 0, 1, 2, 3, 4, 5 in 1D
        // Uniform spacing should give smooth, predictable RG flow
        let data: Vec<f64> = (0..6).map(|i| i as f64).collect();
        let n = 6;
        let d = 1;
        let shared = make_shared(&data, n, d);
        let lens = RenormalizationLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result should not be empty");

        // With uniform spacing, the critical exponent should be meaningful
        let nu = result["critical_exponent"][0];
        assert!(nu.is_finite(), "critical_exponent must be finite for uniform data");

        // Correlation length should be positive
        let cl = result["correlation_length"][0];
        assert!(cl >= 0.0, "correlation_length should be non-negative");

        // RG flow should be monotonically related to coarse-graining
        let rg_flow = &result["rg_flow"];
        assert!(rg_flow.len() >= 2);
        // Last entry should be normalized to <= 1.0
        assert!(*rg_flow.last().unwrap() <= 1.0 + 1e-10);
    }

    #[test]
    fn test_renormalization_too_few_points() {
        // n < 4 should return empty
        let data = vec![0.0, 1.0, 2.0];
        let n = 3;
        let d = 1;
        let shared = make_shared(&data, n, d);
        let lens = RenormalizationLens;
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.is_empty(), "n < 4 should return empty");
    }
}
