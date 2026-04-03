use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ExtrapolationLens: Project observed trends beyond measured boundaries.
///
/// Algorithm:
///   1. Per-dimension linear regression (OLS) on point ordering by distance from centroid
///   2. Compute R² goodness-of-fit per dimension → extrapolation confidence
///   3. Extrapolate each dimension beyond observed range by trend slope
///   4. Measure distance-trend gradient: how pairwise distances evolve along the ordering
///   5. Estimate boundary expansion rate and divergence risk
pub struct UextrapolationLens;

impl Lens for UextrapolationLens {
    fn name(&self) -> &str {
        "extrapolation"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        // --- Step 1: Order points by distance from centroid ---
        let mut centroid = vec![0.0f64; d];
        for i in 0..n {
            let row = i * d;
            for j in 0..d {
                centroid[j] += data[row + j];
            }
        }
        let inv_n = 1.0 / n as f64;
        for j in 0..d {
            centroid[j] *= inv_n;
        }

        // Compute distance from centroid for each point
        let mut centroid_dists: Vec<(f64, usize)> = (0..n)
            .map(|i| {
                let row = i * d;
                let mut sq = 0.0;
                for j in 0..d {
                    let diff = data[row + j] - centroid[j];
                    sq += diff * diff;
                }
                (sq.sqrt(), i)
            })
            .collect();
        centroid_dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));

        let ordering: Vec<usize> = centroid_dists.iter().map(|&(_, idx)| idx).collect();

        // --- Step 2: Per-dimension linear regression along ordering ---
        // For each dimension, fit y = a*t + b where t = ordering index
        let mut slopes = Vec::with_capacity(d);
        let mut intercepts = Vec::with_capacity(d);
        let mut r_squared = Vec::with_capacity(d);

        let t_mean = (n - 1) as f64 / 2.0;
        let t_var: f64 = (0..n).map(|t| {
            let diff = t as f64 - t_mean;
            diff * diff
        }).sum::<f64>();

        for dim in 0..d {
            // Gather values in ordering
            let mut y_mean = 0.0;
            for &idx in &ordering {
                y_mean += data[idx * d + dim];
            }
            y_mean *= inv_n;

            let mut cov_ty = 0.0;
            let mut ss_tot = 0.0;
            for (t, &idx) in ordering.iter().enumerate() {
                let y = data[idx * d + dim];
                cov_ty += (t as f64 - t_mean) * (y - y_mean);
                ss_tot += (y - y_mean) * (y - y_mean);
            }

            let slope = if t_var > 1e-15 { cov_ty / t_var } else { 0.0 };
            let intercept = y_mean - slope * t_mean;

            // R²
            let ss_res: f64 = ordering.iter().enumerate().map(|(t, &idx)| {
                let y = data[idx * d + dim];
                let pred = slope * t as f64 + intercept;
                (y - pred) * (y - pred)
            }).sum();

            let r2 = if ss_tot > 1e-15 { 1.0 - ss_res / ss_tot } else { 0.0 };

            slopes.push(slope);
            intercepts.push(intercept);
            r_squared.push(r2.max(0.0));
        }

        // --- Step 3: Extrapolation beyond observed boundaries ---
        // Project forward by n/2 steps and backward by n/2 steps
        let horizon = (n as f64 / 2.0).ceil() as usize;
        let mut forward_extrap = Vec::with_capacity(d);
        let mut backward_extrap = Vec::with_capacity(d);
        let mut boundary_expansion = Vec::with_capacity(d);

        for dim in 0..d {
            // Current observed range
            let mut dim_min = f64::INFINITY;
            let mut dim_max = f64::NEG_INFINITY;
            for i in 0..n {
                let v = data[i * d + dim];
                if v < dim_min { dim_min = v; }
                if v > dim_max { dim_max = v; }
            }
            let range = dim_max - dim_min;

            // Forward extrapolation value (at t = n - 1 + horizon)
            let t_fwd = (n - 1 + horizon) as f64;
            let fwd_val = slopes[dim] * t_fwd + intercepts[dim];
            forward_extrap.push(fwd_val);

            // Backward extrapolation value (at t = -horizon)
            let t_bwd = -(horizon as f64);
            let bwd_val = slopes[dim] * t_bwd + intercepts[dim];
            backward_extrap.push(bwd_val);

            // Boundary expansion: how much the extrapolated range exceeds observed range
            let extrap_min = fwd_val.min(bwd_val).min(dim_min);
            let extrap_max = fwd_val.max(bwd_val).max(dim_max);
            let extrap_range = extrap_max - extrap_min;
            let expansion = if range > 1e-15 { extrap_range / range } else { 1.0 };
            boundary_expansion.push(expansion);
        }

        // --- Step 4: Distance-trend gradient ---
        // Measure how pairwise distances change along the radial ordering
        // Split points into inner half and outer half, compare mean distances
        let half = n / 2;
        let inner = &ordering[..half];
        let outer = &ordering[half..];

        let mut inner_mean_dist = 0.0;
        let mut inner_count = 0u64;
        for (i, &a) in inner.iter().enumerate() {
            for &b in &inner[(i + 1)..] {
                inner_mean_dist += shared.dist(a, b);
                inner_count += 1;
            }
        }
        if inner_count > 0 {
            inner_mean_dist /= inner_count as f64;
        }

        let mut outer_mean_dist = 0.0;
        let mut outer_count = 0u64;
        for (i, &a) in outer.iter().enumerate() {
            for &b in &outer[(i + 1)..] {
                outer_mean_dist += shared.dist(a, b);
                outer_count += 1;
            }
        }
        if outer_count > 0 {
            outer_mean_dist /= outer_count as f64;
        }

        // Divergence ratio: how much outer points spread vs inner
        let divergence_ratio = if inner_mean_dist > 1e-15 {
            outer_mean_dist / inner_mean_dist
        } else {
            1.0
        };

        // --- Step 5: Overall extrapolation confidence ---
        let mean_r2 = r_squared.iter().sum::<f64>() / d as f64;
        let mean_expansion = boundary_expansion.iter().sum::<f64>() / d as f64;

        // Confidence: high R² + low divergence = trustworthy extrapolation
        let confidence = mean_r2 / (1.0 + (divergence_ratio - 1.0).abs());

        // --- Assemble results ---
        let mut result = HashMap::new();
        result.insert("slopes".to_string(), slopes);
        result.insert("r_squared".to_string(), r_squared);
        result.insert("forward_extrapolation".to_string(), forward_extrap);
        result.insert("backward_extrapolation".to_string(), backward_extrap);
        result.insert("boundary_expansion".to_string(), boundary_expansion);
        result.insert("divergence_ratio".to_string(), vec![divergence_ratio]);
        result.insert("mean_r_squared".to_string(), vec![mean_r2]);
        result.insert("mean_boundary_expansion".to_string(), vec![mean_expansion]);
        result.insert("extrapolation_confidence".to_string(), vec![confidence]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_linear_trend_detected() {
        // Points along y = 2x line: should get high R² and positive slope
        let data: Vec<f64> = (0..10)
            .flat_map(|i| vec![i as f64, 2.0 * i as f64])
            .collect();
        let n = 10;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UextrapolationLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty results");
        let r2 = &result["mean_r_squared"];
        assert!(r2[0] > 0.5, "linear data should have high R², got {}", r2[0]);
        let conf = &result["extrapolation_confidence"];
        assert!(conf[0] > 0.0, "confidence should be positive, got {}", conf[0]);
        assert!(result.contains_key("slopes"));
        assert!(result.contains_key("forward_extrapolation"));
        assert!(result.contains_key("backward_extrapolation"));
    }

    #[test]
    fn test_clustered_data_divergence() {
        // Two clusters: inner tight cluster + outer spread cluster
        // Inner cluster near origin
        let data = vec![
            0.0, 0.0,
            0.1, 0.1,
            0.2, 0.0,
            0.0, 0.2,
            // Outer cluster far away
            5.0, 5.0,
            5.5, 4.5,
            6.0, 6.0,
            4.5, 5.5,
        ];
        let n = 8;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UextrapolationLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty results");
        let div = &result["divergence_ratio"];
        assert!(div[0].is_finite(), "divergence ratio should be finite");
        assert!(result["boundary_expansion"].len() == d);
        // Boundary expansion should be > 1 (extrapolation extends beyond observed)
        let expansion = &result["mean_boundary_expansion"];
        assert!(expansion[0] >= 1.0, "expansion should be >= 1.0, got {}", expansion[0]);
    }
}
