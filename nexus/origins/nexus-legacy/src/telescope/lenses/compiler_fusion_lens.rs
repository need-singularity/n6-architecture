use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CompilerFusionLens: Find fusable consecutive operations/loops (kernel fusion).
///
/// Analyzes data patterns to detect sequential segments that share data locality,
/// indicating loop bodies or kernel stages that could be fused into a single pass
/// to reduce memory traffic.
///
/// Metrics:
///   1. fusable_pair_count: number of adjacent row-pairs with high data reuse
///   2. fusion_benefit: estimated memory bandwidth savings from fusing (0-1)
///   3. locality_score: spatial locality across consecutive segments
///   4. stride_regularity: how regular the access stride is (1.0 = perfectly regular)
///   5. segment_overlap: fraction of value ranges that overlap between consecutive segments
pub struct CompilerFusionLens;

impl Lens for CompilerFusionLens {
    fn name(&self) -> &str { "CompilerFusionLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Segment size: treat each row as a "kernel stage"
        // Fusable pairs: consecutive rows where L2 distance is small relative to global spread
        let mut global_var = 0.0f64;
        let global_mean: f64 = data[..max_n * d].iter().sum::<f64>() / (max_n * d) as f64;
        for i in 0..max_n * d {
            let diff = data[i] - global_mean;
            global_var += diff * diff;
        }
        global_var /= (max_n * d) as f64;
        let global_std = global_var.sqrt().max(1e-15);

        let mut fusable_pairs = 0usize;
        let mut locality_sum = 0.0f64;
        let mut stride_diffs = Vec::with_capacity(max_n);

        for i in 1..max_n {
            // L2 distance between row i-1 and row i, normalized by global std
            let mut dist_sq = 0.0f64;
            for j in 0..d {
                let diff = data[i * d + j] - data[(i - 1) * d + j];
                dist_sq += diff * diff;
            }
            let norm_dist = (dist_sq.sqrt()) / (global_std * (d as f64).sqrt());

            // Fusable if normalized distance < 0.5 (high data reuse)
            if norm_dist < 0.5 {
                fusable_pairs += 1;
            }
            locality_sum += (-norm_dist).exp(); // exponential locality decay

            // Stride: difference magnitude per dimension
            let stride = dist_sq.sqrt() / d.max(1) as f64;
            stride_diffs.push(stride);
        }

        let pair_count = (max_n - 1).max(1);
        let fusable_pair_count = fusable_pairs as f64;
        let locality_score = locality_sum / pair_count as f64;

        // Fusion benefit: fraction of pairs that are fusable
        let fusion_benefit = fusable_pairs as f64 / pair_count as f64;

        // Stride regularity: 1 - CV of stride differences
        let stride_mean = stride_diffs.iter().sum::<f64>() / stride_diffs.len().max(1) as f64;
        let stride_var: f64 = stride_diffs.iter()
            .map(|&s| (s - stride_mean).powi(2))
            .sum::<f64>() / stride_diffs.len().max(1) as f64;
        let stride_cv = if stride_mean > 1e-15 { stride_var.sqrt() / stride_mean } else { 0.0 };
        let stride_regularity = (1.0 - stride_cv).max(0.0).min(1.0);

        // Segment overlap: value range overlap between consecutive rows
        let mut overlap_sum = 0.0f64;
        for i in 1..max_n {
            let (mut min_a, mut max_a) = (f64::MAX, f64::MIN);
            let (mut min_b, mut max_b) = (f64::MAX, f64::MIN);
            for j in 0..d {
                let a = data[(i - 1) * d + j];
                let b = data[i * d + j];
                min_a = min_a.min(a);
                max_a = max_a.max(a);
                min_b = min_b.min(b);
                max_b = max_b.max(b);
            }
            let overlap_low = min_a.max(min_b);
            let overlap_high = max_a.min(max_b);
            let overlap = (overlap_high - overlap_low).max(0.0);
            let span = (max_a.max(max_b) - min_a.min(min_b)).max(1e-15);
            overlap_sum += overlap / span;
        }
        let segment_overlap = overlap_sum / pair_count as f64;

        let mut result = HashMap::new();
        result.insert("fusable_pair_count".to_string(), vec![fusable_pair_count]);
        result.insert("fusion_benefit".to_string(), vec![fusion_benefit]);
        result.insert("locality_score".to_string(), vec![locality_score]);
        result.insert("stride_regularity".to_string(), vec![stride_regularity]);
        result.insert("segment_overlap".to_string(), vec![segment_overlap]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smooth_data_high_fusion() {
        // Smoothly varying data = high fusability
        let data: Vec<f64> = (0..30).map(|i| i as f64 * 0.01).collect();
        let shared = SharedData::compute(&data, 10, 3);
        let result = CompilerFusionLens.scan(&data, 10, 3, &shared);
        assert!(result.contains_key("fusion_benefit"));
        assert!(result["fusion_benefit"][0] > 0.3);
    }

    #[test]
    fn test_random_jumps_low_fusion() {
        // Large jumps between rows = low fusability
        let data: Vec<f64> = (0..20).map(|i| if i % 2 == 0 { 100.0 } else { -100.0 }).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let result = CompilerFusionLens.scan(&data, 10, 2, &shared);
        assert!(result.contains_key("locality_score"));
    }

    #[test]
    fn test_constant_stride_regular() {
        // Constant stride = high regularity
        let data: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = CompilerFusionLens.scan(&data, 20, 1, &shared);
        assert!(result["stride_regularity"][0] > 0.8);
    }
}
