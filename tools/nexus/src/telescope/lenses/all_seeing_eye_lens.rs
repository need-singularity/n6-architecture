use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// AllSeeingEyeLens: Comprehensive omnidirectional scan.
///
/// Measures coverage (fraction of data space explored), blind spots,
/// visibility radius per point, and panopticon score (mutual observability).
pub struct AllSeeingEyeLens;

impl Lens for AllSeeingEyeLens {
    fn name(&self) -> &str { "AllSeeingEyeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // coverage: fraction of data space explored (entropy-based)
        // Bin each dimension and count occupied cells
        let n_bins = 8_usize;
        let mut occupied = vec![false; n_bins.pow(d.min(4) as u32)];
        let total_cells = occupied.len();

        // Compute per-dim min/max for binning
        let mut mins = vec![f64::INFINITY; d];
        let mut maxs = vec![f64::NEG_INFINITY; d];
        for i in 0..max_n {
            for j in 0..d {
                let v = data[i * d + j];
                if v < mins[j] { mins[j] = v; }
                if v > maxs[j] { maxs[j] = v; }
            }
        }

        let dims_used = d.min(4);
        for i in 0..max_n {
            let mut cell_idx = 0usize;
            for j in 0..dims_used {
                let range = (maxs[j] - mins[j]).max(1e-12);
                let bin = ((data[i * d + j] - mins[j]) / range * (n_bins - 1) as f64) as usize;
                let bin = bin.min(n_bins - 1);
                cell_idx = cell_idx * n_bins + bin;
            }
            if cell_idx < total_cells {
                occupied[cell_idx] = true;
            }
        }
        let occupied_count = occupied.iter().filter(|&&b| b).count();
        let coverage = occupied_count as f64 / total_cells as f64;

        // blind_spots: fraction of unoccupied cells
        let blind_spots = 1.0 - coverage;

        // visibility_radius: mean distance to k-th nearest neighbor
        let mut vis_sum = 0.0_f64;
        for i in 0..max_n {
            let knn = shared.knn(i);
            if !knn.is_empty() {
                let k_neighbor = knn[knn.len() - 1] as usize;
                vis_sum += shared.dist(i, k_neighbor);
            }
        }
        let visibility_radius = vis_sum / max_n as f64;

        // panopticon_score: can all points observe all others?
        // Measure: fraction of pairs within 2x mean distance
        let mut total_dist = 0.0_f64;
        let mut pair_count = 0u64;
        let sample_step = (max_n / 40).max(1);
        for i in (0..max_n).step_by(sample_step) {
            for j in ((i + 1)..max_n).step_by(sample_step) {
                total_dist += shared.dist(i, j);
                pair_count += 1;
            }
        }
        let mean_pair_dist = if pair_count > 0 { total_dist / pair_count as f64 } else { 1.0 };
        let threshold = 2.0 * mean_pair_dist;

        let mut visible_pairs = 0u64;
        let mut check_count = 0u64;
        for i in (0..max_n).step_by(sample_step) {
            for j in ((i + 1)..max_n).step_by(sample_step) {
                if shared.dist(i, j) < threshold {
                    visible_pairs += 1;
                }
                check_count += 1;
            }
        }
        let panopticon_score = if check_count > 0 {
            visible_pairs as f64 / check_count as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("coverage".to_string(), vec![coverage]);
        result.insert("blind_spots".to_string(), vec![blind_spots]);
        result.insert("visibility_radius".to_string(), vec![visibility_radius]);
        result.insert("panopticon_score".to_string(), vec![panopticon_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = AllSeeingEyeLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("coverage"));
        assert!(result.contains_key("blind_spots"));
        assert!(result.contains_key("panopticon_score"));
    }

    #[test]
    fn test_coverage_bounds() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.25)).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = AllSeeingEyeLens.scan(&data, 20, 2, &shared);
        let cov = result["coverage"][0];
        let blind = result["blind_spots"][0];
        assert!(cov >= 0.0 && cov <= 1.0);
        assert!((cov + blind - 1.0).abs() < 1e-10, "coverage + blind_spots should = 1.0");
    }
}
