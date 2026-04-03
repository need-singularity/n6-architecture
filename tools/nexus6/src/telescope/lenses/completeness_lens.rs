use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CompletenessLens: Check whether a set of components covers the full space.
///
/// Algorithm:
///   1. Compute bounding box of data in each dimension
///   2. Partition each dimension into bins (ceil(n^{1/d}) bins per axis)
///   3. Count occupied vs total bins → coverage_ratio
///   4. Compute max gap ratio per dimension (largest empty interval / range)
///   5. Compute nearest-neighbor dispersion: max over all grid centroids
///      of the minimum distance to any data point (measures worst-case hole)
///   6. completeness_score = coverage_ratio * (1 - mean_max_gap)
pub struct UcompletenessLens;

impl Lens for UcompletenessLens {
    fn name(&self) -> &str {
        "CompletenessLens"
    }

    fn category(&self) -> &str {
        "structure"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 2 || d == 0 {
            return HashMap::new();
        }

        // --- Per-dimension analysis ---
        let mut dim_mins = vec![f64::INFINITY; d];
        let mut dim_maxs = vec![f64::NEG_INFINITY; d];

        for i in 0..n {
            for dim in 0..d {
                let v = data[i * d + dim];
                if v < dim_mins[dim] {
                    dim_mins[dim] = v;
                }
                if v > dim_maxs[dim] {
                    dim_maxs[dim] = v;
                }
            }
        }

        // Bins per axis: ceil(n^{1/d}), minimum 2
        let bins_per_axis = ((n as f64).powf(1.0 / d as f64)).ceil() as usize;
        let bins_per_axis = if bins_per_axis < 2 { 2 } else { bins_per_axis };

        // --- Max gap ratio per dimension ---
        let mut max_gap_ratios = Vec::with_capacity(d);
        for dim in 0..d {
            let range = dim_maxs[dim] - dim_mins[dim];
            if range <= 0.0 {
                max_gap_ratios.push(0.0);
                continue;
            }
            // Collect sorted values in this dimension
            let mut vals: Vec<f64> = (0..n).map(|i| data[i * d + dim]).collect();
            vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            let mut max_gap = 0.0_f64;
            for w in vals.windows(2) {
                let gap = w[1] - w[0];
                if gap > max_gap {
                    max_gap = gap;
                }
            }
            max_gap_ratios.push(max_gap / range);
        }

        let mean_max_gap =
            max_gap_ratios.iter().sum::<f64>() / max_gap_ratios.len() as f64;

        // --- Grid-based coverage ---
        // Assign each point to a bin index tuple, count unique occupied bins
        let total_bins: usize = bins_per_axis.checked_pow(d as u32).unwrap_or(usize::MAX);
        // For high d, cap total_bins to avoid huge allocation
        let use_grid = total_bins <= 1_000_000;

        let coverage_ratio = if use_grid {
            let mut occupied = vec![false; total_bins];
            for i in 0..n {
                let mut flat_idx = 0usize;
                let mut stride = 1usize;
                for dim in (0..d).rev() {
                    let range = dim_maxs[dim] - dim_mins[dim];
                    let bin = if range <= 0.0 {
                        0
                    } else {
                        let normalized = (data[i * d + dim] - dim_mins[dim]) / range;
                        let b = (normalized * bins_per_axis as f64) as usize;
                        b.min(bins_per_axis - 1)
                    };
                    flat_idx += bin * stride;
                    stride *= bins_per_axis;
                }
                occupied[flat_idx] = true;
            }
            let occupied_count = occupied.iter().filter(|&&x| x).count();
            occupied_count as f64 / total_bins as f64
        } else {
            // Fallback: per-dimension coverage average
            let mut dim_coverages = Vec::with_capacity(d);
            for dim in 0..d {
                let range = dim_maxs[dim] - dim_mins[dim];
                if range <= 0.0 {
                    dim_coverages.push(1.0);
                    continue;
                }
                let mut bins_hit = vec![false; bins_per_axis];
                for i in 0..n {
                    let normalized = (data[i * d + dim] - dim_mins[dim]) / range;
                    let b = (normalized * bins_per_axis as f64) as usize;
                    bins_hit[b.min(bins_per_axis - 1)] = true;
                }
                let hit = bins_hit.iter().filter(|&&x| x).count();
                dim_coverages.push(hit as f64 / bins_per_axis as f64);
            }
            dim_coverages.iter().sum::<f64>() / d as f64
        };

        // --- Dispersion: max nearest-neighbor distance from data ---
        // Use pairwise distances from SharedData to compute how "spread" the coverage is
        let mut nn_dists: Vec<f64> = Vec::with_capacity(n);
        for i in 0..n {
            let mut min_d = f64::INFINITY;
            for j in 0..n {
                if i == j {
                    continue;
                }
                let dist = shared.dist(i, j);
                if dist < min_d {
                    min_d = dist;
                }
            }
            nn_dists.push(min_d);
        }

        // Dispersion = max nearest-neighbor distance (worst-case hole indicator)
        let dispersion = nn_dists
            .iter()
            .cloned()
            .fold(f64::NEG_INFINITY, f64::max);

        // Mean nearest-neighbor distance
        let mean_nn = nn_dists.iter().sum::<f64>() / n as f64;

        // Uniformity: coefficient of variation of NN distances (lower = more uniform)
        let nn_var = nn_dists.iter().map(|x| (x - mean_nn).powi(2)).sum::<f64>() / n as f64;
        let nn_cv = if mean_nn > 0.0 {
            nn_var.sqrt() / mean_nn
        } else {
            0.0
        };

        // --- Completeness score ---
        // High coverage + small gaps = high completeness
        let completeness_score = coverage_ratio * (1.0 - mean_max_gap);

        let mut result = HashMap::new();
        result.insert("coverage_ratio".to_string(), vec![coverage_ratio]);
        result.insert("max_gap_ratios".to_string(), max_gap_ratios);
        result.insert("mean_max_gap".to_string(), vec![mean_max_gap]);
        result.insert("dispersion".to_string(), vec![dispersion]);
        result.insert("mean_nn_distance".to_string(), vec![mean_nn]);
        result.insert("nn_uniformity_cv".to_string(), vec![nn_cv]);
        result.insert("completeness_score".to_string(), vec![completeness_score]);
        result.insert("bins_per_axis".to_string(), vec![bins_per_axis as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_completeness_uniform_grid() {
        // 3x3 uniform grid in 2D → should have high coverage
        let data = vec![
            0.0, 0.0, 0.5, 0.0, 1.0, 0.0,
            0.0, 0.5, 0.5, 0.5, 1.0, 0.5,
            0.0, 1.0, 0.5, 1.0, 1.0, 1.0,
        ];
        let n = 9;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UcompletenessLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must be non-empty");
        let score = result.get("completeness_score").unwrap()[0];
        assert!(score >= 0.5, "uniform grid should have high completeness, got {}", score);
        let coverage = result.get("coverage_ratio").unwrap()[0];
        assert!(coverage > 0.5, "uniform grid coverage should be high, got {}", coverage);
    }

    #[test]
    fn test_completeness_clustered_data() {
        // All points clustered in one corner → low coverage, high gap
        let data = vec![
            0.0, 0.0,
            0.01, 0.01,
            0.02, 0.0,
            0.0, 0.02,
            0.01, 0.02,
            1.0, 1.0, // single outlier to create range
        ];
        let n = 6;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UcompletenessLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "result must be non-empty");
        let score = result.get("completeness_score").unwrap()[0];
        let mean_gap = result.get("mean_max_gap").unwrap()[0];
        // Clustered data should have a large gap
        assert!(mean_gap > 0.5, "clustered data should have large gap, got {}", mean_gap);
        // Completeness should be lower than the uniform case
        assert!(score < 0.5, "clustered data should have low completeness, got {}", score);
    }
}
