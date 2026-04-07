use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PowerConsumptionLens: Energy consumption prediction for code paths.
///
/// Models computational energy cost based on data access patterns:
/// - Memory-bound operations (large strides, cache misses) cost more energy
/// - Compute-bound operations (many arithmetic ops on local data) are cheaper
/// - SIMD-friendly patterns reduce energy per element
///
/// Metrics:
///   1. energy_per_element: estimated relative energy cost per data point
///   2. memory_energy_ratio: fraction of energy spent on memory vs compute
///   3. idle_fraction: fraction of data that is zero/near-zero (wasted energy)
///   4. dynamic_range_cost: wider value ranges need more bits, more energy
///   5. batch_efficiency: energy savings from processing in batches
pub struct PowerConsumptionLens;

impl Lens for PowerConsumptionLens {
    fn name(&self) -> &str { "PowerConsumptionLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let total = n * d;
        let max_total = total.min(5000);

        // Idle fraction: near-zero elements (|x| < 1e-10)
        let idle_count = data[..max_total].iter().filter(|&&x| x.abs() < 1e-10).count();
        let idle_fraction = idle_count as f64 / max_total as f64;

        // Dynamic range: log2(max/min) for non-zero values
        let mut abs_vals: Vec<f64> = data[..max_total].iter()
            .map(|x| x.abs())
            .filter(|&x| x > 1e-15)
            .collect();
        abs_vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let dynamic_range_cost = if abs_vals.len() > 1 {
            let min_val = abs_vals[0];
            let max_val = abs_vals[abs_vals.len() - 1];
            if min_val > 1e-15 {
                (max_val / min_val).log2().min(64.0) / 64.0 // normalize to [0,1]
            } else {
                1.0
            }
        } else {
            0.0
        };

        // Memory energy: based on stride analysis (irregular access = more energy)
        let max_n = n.min(500);
        let mut stride_irregularity = 0.0f64;
        if max_n > 1 && d > 0 {
            // Look at first dimension as proxy for access pattern
            let mut diffs = Vec::with_capacity(max_n - 1);
            for i in 1..max_n {
                diffs.push((data[i * d] - data[(i - 1) * d]).abs());
            }
            let mean_diff = diffs.iter().sum::<f64>() / diffs.len() as f64;
            let var_diff: f64 = diffs.iter().map(|&x| (x - mean_diff).powi(2)).sum::<f64>()
                / diffs.len() as f64;
            stride_irregularity = if mean_diff > 1e-15 {
                (var_diff.sqrt() / mean_diff).min(2.0) / 2.0
            } else {
                0.0
            };
        }

        // Memory energy model: base 0.3 + irregularity bonus
        let memory_energy_ratio = 0.3 + 0.7 * stride_irregularity;

        // Compute energy: based on value magnitudes (larger values = more FP energy)
        let mean_magnitude = if !abs_vals.is_empty() {
            abs_vals.iter().sum::<f64>() / abs_vals.len() as f64
        } else {
            0.0
        };
        let compute_energy = (mean_magnitude.log2().abs() + 1.0).min(20.0) / 20.0;

        // Total energy per element
        let energy_per_element = memory_energy_ratio * 0.6 + compute_energy * 0.4
            + idle_fraction * 0.1; // idle elements still cost cache space

        // Batch efficiency: how much energy is saved by batching consecutive elements
        // Higher row similarity = better batching (amortize cache load)
        let mut batch_sim = 0.0f64;
        let mut batch_count = 0;
        for i in 1..max_n.min(100) {
            let mut dot = 0.0f64;
            let mut na = 0.0f64;
            let mut nb = 0.0f64;
            for j in 0..d {
                let a = data[(i - 1) * d + j];
                let b = data[i * d + j];
                dot += a * b;
                na += a * a;
                nb += b * b;
            }
            let denom = (na * nb).sqrt();
            if denom > 1e-15 {
                batch_sim += (dot / denom).max(0.0);
                batch_count += 1;
            }
        }
        let batch_efficiency = if batch_count > 0 {
            batch_sim / batch_count as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("energy_per_element".to_string(), vec![energy_per_element]);
        result.insert("memory_energy_ratio".to_string(), vec![memory_energy_ratio]);
        result.insert("idle_fraction".to_string(), vec![idle_fraction]);
        result.insert("dynamic_range_cost".to_string(), vec![dynamic_range_cost]);
        result.insert("batch_efficiency".to_string(), vec![batch_efficiency]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sparse_data_high_idle() {
        // Mostly zeros
        let mut data = vec![0.0; 20];
        data[5] = 1.0;
        data[15] = 2.0;
        let shared = SharedData::compute(&data, 20, 1);
        let result = PowerConsumptionLens.scan(&data, 20, 1, &shared);
        assert!(result["idle_fraction"][0] > 0.8);
    }

    #[test]
    fn test_regular_stride_low_memory_energy() {
        // Perfectly regular data
        let data: Vec<f64> = (0..60).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 20, 3);
        let result = PowerConsumptionLens.scan(&data, 20, 3, &shared);
        // Regular stride => lower memory_energy_ratio
        assert!(result["memory_energy_ratio"][0] < 0.8);
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 3];
        let shared = SharedData::compute(&data, 3, 1);
        let result = PowerConsumptionLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty());
    }
}
