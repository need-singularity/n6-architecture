use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PrefetchLens: Software prefetch insertion point detection.
///
/// Analyzes data access stride patterns to identify where software prefetch
/// instructions (__builtin_prefetch) would be beneficial.
///
/// Metrics:
///   1. stride_predictability: how predictable the access stride is (1.0 = constant stride)
///   2. prefetch_distance: optimal lookahead distance in rows
///   3. miss_rate_estimate: estimated cache miss rate without prefetching
///   4. prefetch_benefit: estimated speedup from prefetching (ratio)
///   5. stream_count: number of independent data streams detected
pub struct PrefetchLens;

impl Lens for PrefetchLens {
    fn name(&self) -> &str { "PrefetchLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Analyze stride pattern per dimension
        let mut strides_per_dim = vec![Vec::new(); d];
        for j in 0..d {
            for i in 1..max_n {
                strides_per_dim[j].push(data[i * d + j] - data[(i - 1) * d + j]);
            }
        }

        // Stride predictability: 1 - CV of strides (averaged across dims)
        let mut predictability_sum = 0.0f64;
        for j in 0..d {
            let strides = &strides_per_dim[j];
            if strides.is_empty() { continue; }
            let mean = strides.iter().sum::<f64>() / strides.len() as f64;
            let var: f64 = strides.iter().map(|&s| (s - mean).powi(2)).sum::<f64>()
                / strides.len() as f64;
            let cv = if mean.abs() > 1e-15 { var.sqrt() / mean.abs() } else { 1.0 };
            predictability_sum += (1.0 - cv.min(1.0)).max(0.0);
        }
        let stride_predictability = predictability_sum / d as f64;

        // Optimal prefetch distance: based on autocorrelation peak
        // Find lag where cumulative correlation is maximized for first dim
        let col0: Vec<f64> = (0..max_n).map(|i| data[i * d]).collect();
        let c0_mean = col0.iter().sum::<f64>() / max_n as f64;
        let c0_var: f64 = col0.iter().map(|&x| (x - c0_mean).powi(2)).sum::<f64>() / max_n as f64;

        let mut best_lag = 4usize; // default prefetch distance
        let mut best_ac = 0.0f64;
        let max_lag = (max_n / 4).min(32);
        if c0_var > 1e-15 {
            for lag in 1..=max_lag {
                let mut ac = 0.0f64;
                let count = max_n - lag;
                for i in 0..count {
                    ac += (col0[i] - c0_mean) * (col0[i + lag] - c0_mean);
                }
                ac /= count as f64 * c0_var;
                if ac > best_ac {
                    best_ac = ac;
                    best_lag = lag;
                }
            }
        }
        let prefetch_distance = best_lag as f64;

        // Cache miss rate estimate: fraction of accesses with stride > cache line
        // Model: 8 f64s per cache line (64 bytes)
        let cache_line_size = 8.0f64; // in f64 units
        let mut miss_count = 0usize;
        let mut total_accesses = 0usize;
        for j in 0..d {
            for &stride in &strides_per_dim[j] {
                total_accesses += 1;
                if stride.abs() > cache_line_size {
                    miss_count += 1;
                }
            }
        }
        let miss_rate_estimate = if total_accesses > 0 {
            miss_count as f64 / total_accesses as f64
        } else {
            0.0
        };

        // Prefetch benefit: higher with predictable strides + high miss rate
        let prefetch_benefit = stride_predictability * miss_rate_estimate * 3.0; // up to 3x

        // Stream detection: count independent stride patterns
        // Cluster dimensions by their dominant stride
        let mut dominant_strides = Vec::with_capacity(d);
        for j in 0..d {
            let strides = &strides_per_dim[j];
            if strides.is_empty() { dominant_strides.push(0.0); continue; }
            let mean = strides.iter().sum::<f64>() / strides.len() as f64;
            dominant_strides.push(mean);
        }

        // Count distinct streams: strides within 10% of each other = same stream
        let mut streams = Vec::new();
        for &s in &dominant_strides {
            let mut found = false;
            for existing in &streams {
                if (s - existing).abs() < (s.abs() * 0.1 + 1e-10) {
                    found = true;
                    break;
                }
            }
            if !found {
                streams.push(s);
            }
        }
        let stream_count = streams.len() as f64;

        let mut result = HashMap::new();
        result.insert("stride_predictability".to_string(), vec![stride_predictability]);
        result.insert("prefetch_distance".to_string(), vec![prefetch_distance]);
        result.insert("miss_rate_estimate".to_string(), vec![miss_rate_estimate]);
        result.insert("prefetch_benefit".to_string(), vec![prefetch_benefit]);
        result.insert("stream_count".to_string(), vec![stream_count]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_constant_stride_predictable() {
        // Linear data = perfectly predictable stride
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 2.0).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = PrefetchLens.scan(&data, 20, 2, &shared);
        assert!(result["stride_predictability"][0] > 0.8);
    }

    #[test]
    fn test_large_stride_high_miss() {
        // Large jumps = cache misses
        let data: Vec<f64> = (0..20).map(|i| i as f64 * 1000.0).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = PrefetchLens.scan(&data, 20, 1, &shared);
        assert!(result["miss_rate_estimate"][0] > 0.5);
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 3];
        let shared = SharedData::compute(&data, 3, 1);
        let result = PrefetchLens.scan(&data, 3, 1, &shared);
        assert!(result.is_empty());
    }
}
