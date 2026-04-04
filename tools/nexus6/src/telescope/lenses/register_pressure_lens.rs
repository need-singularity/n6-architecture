use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RegisterPressureLens: Register pressure analysis and spill prevention.
///
/// Analyzes data "liveness" patterns — how many values are simultaneously needed.
/// High simultaneous liveness = high register pressure = spills to memory.
///
/// Metrics:
///   1. peak_liveness: maximum number of simultaneously live values (normalized by d)
///   2. spill_risk: fraction of rows where liveness exceeds a threshold
///   3. liveness_variance: variance in liveness across the data (bursty = harder to schedule)
///   4. reuse_distance: mean distance between uses of similar values
///   5. pressure_score: overall register pressure (0=low, 1=critical)
pub struct RegisterPressureLens;

impl Lens for RegisterPressureLens {
    fn name(&self) -> &str { "RegisterPressureLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Model liveness: how many distinct "active" dimensions contribute non-trivially
        // at each row. A dimension is "live" if it has significant magnitude.
        let mut liveness_per_row = Vec::with_capacity(max_n);
        let mut dim_threshold = vec![0.0f64; d];

        // Threshold per dimension: 10% of range
        for j in 0..d {
            let mut min_v = f64::MAX;
            let mut max_v = f64::MIN;
            for i in 0..max_n {
                let v = data[i * d + j].abs();
                min_v = min_v.min(v);
                max_v = max_v.max(v);
            }
            dim_threshold[j] = min_v + (max_v - min_v) * 0.1;
        }

        for i in 0..max_n {
            let live_count = (0..d).filter(|&j| data[i * d + j].abs() > dim_threshold[j]).count();
            liveness_per_row.push(live_count as f64);
        }

        let nf = max_n as f64;
        let peak_liveness = liveness_per_row.iter().cloned().fold(0.0f64, f64::max) / d as f64;

        // Spill risk: rows where liveness exceeds 75% of dimensions
        let spill_threshold = d as f64 * 0.75;
        let spill_count = liveness_per_row.iter().filter(|&&l| l > spill_threshold).count();
        let spill_risk = spill_count as f64 / nf;

        // Liveness variance
        let mean_liveness = liveness_per_row.iter().sum::<f64>() / nf;
        let liveness_variance = liveness_per_row.iter()
            .map(|&l| (l - mean_liveness).powi(2))
            .sum::<f64>() / nf;

        // Reuse distance: for each value, find distance to next similar value
        // Use dimension 0 as proxy, quantize to bins
        let num_bins = 16usize;
        let mut col0: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let col_min = col0.iter().cloned().fold(f64::MAX, f64::min);
        let col_max = col0.iter().cloned().fold(f64::MIN, f64::max);
        let bin_width = ((col_max - col_min) / num_bins as f64).max(1e-15);

        let mut last_seen = vec![usize::MAX; num_bins];
        let mut reuse_distances = Vec::new();
        for i in 0..max_n {
            let bin = ((col0[i] - col_min) / bin_width).floor() as usize;
            let bin = bin.min(num_bins - 1);
            if last_seen[bin] != usize::MAX {
                reuse_distances.push((i - last_seen[bin]) as f64);
            }
            last_seen[bin] = i;
        }
        let _ = col0; // consumed

        let reuse_distance = if !reuse_distances.is_empty() {
            reuse_distances.iter().sum::<f64>() / reuse_distances.len() as f64
        } else {
            max_n as f64 // no reuse detected
        };

        // Overall pressure score
        let pressure_score = (peak_liveness * 0.3
            + spill_risk * 0.3
            + (liveness_variance / d.max(1) as f64).min(1.0) * 0.2
            + (1.0 - (1.0 / (reuse_distance + 1.0))) * 0.2)
            .min(1.0);

        let mut result = HashMap::new();
        result.insert("peak_liveness".to_string(), vec![peak_liveness]);
        result.insert("spill_risk".to_string(), vec![spill_risk]);
        result.insert("liveness_variance".to_string(), vec![liveness_variance]);
        result.insert("reuse_distance".to_string(), vec![reuse_distance]);
        result.insert("pressure_score".to_string(), vec![pressure_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_dims_active() {
        // All dimensions have large values = high pressure
        let data: Vec<f64> = (0..80).map(|i| (i as f64 + 1.0) * 10.0).collect();
        let shared = SharedData::compute(&data, 10, 8);
        let result = RegisterPressureLens.scan(&data, 10, 8, &shared);
        assert!(result["peak_liveness"][0] > 0.5);
    }

    #[test]
    fn test_sparse_low_pressure() {
        // Only one dimension active per row (sparse)
        let mut data = vec![0.0; 60];
        for i in 0..20 {
            data[i * 3 + (i % 3)] = 100.0;
        }
        let shared = SharedData::compute(&data, 20, 3);
        let result = RegisterPressureLens.scan(&data, 20, 3, &shared);
        assert!(result["spill_risk"][0] < 0.5);
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 5];
        let shared = SharedData::compute(&data, 5, 1);
        let result = RegisterPressureLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty());
    }
}
