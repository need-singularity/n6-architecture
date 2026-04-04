use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MemoryPatternLens: Classify memory access patterns in data flow.
///
/// Analyzes whether data exhibits sequential, strided, random, or
/// tiled access patterns. This matters because LLVM's alias analysis
/// and prefetch hints are weak for non-trivial patterns, and the
/// difference between sequential and random access is 10-100x on
/// modern CPUs due to cache hierarchy.
///
/// Metrics:
///   1. sequential_score: fraction of data following sequential order
///   2. strided_score: regular stride detected (and stride value)
///   3. random_score: entropy of access order (1.0 = fully random)
///   4. tiled_score: block/tile structure detected
///   5. stride_value: detected dominant stride
///   6. reuse_distance: average distance between accesses to similar values
pub struct MemoryPatternLens;

impl Lens for MemoryPatternLens {
    fn name(&self) -> &str { "MemoryPatternLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // Treat each column as an "address stream" — analyze order patterns
        let d_check = d.min(16);
        let mut seq_scores = Vec::with_capacity(d_check);
        let mut stride_scores = Vec::with_capacity(d_check);
        let mut best_stride = 0.0f64;
        let mut best_stride_score = 0.0f64;

        for dim in 0..d_check {
            let col: Vec<f64> = (0..max_n).map(|i| data[i * d + dim]).collect();
            if col.len() < 3 { continue; }

            // Sequential: check monotonic tendency
            let mut increasing = 0usize;
            let mut decreasing = 0usize;
            for w in col.windows(2) {
                if w[1] >= w[0] - 1e-12 { increasing += 1; }
                if w[1] <= w[0] + 1e-12 { decreasing += 1; }
            }
            let mono_frac = increasing.max(decreasing) as f64 / (col.len() - 1) as f64;
            seq_scores.push(mono_frac);

            // Stride detection: compute deltas, find dominant stride
            let deltas: Vec<f64> = col.windows(2).map(|w| w[1] - w[0]).collect();
            if deltas.is_empty() { continue; }

            // Bin deltas to find dominant stride
            let mut delta_counts: HashMap<i64, usize> = HashMap::new();
            for &delta in &deltas {
                let key = (delta * 1000.0).round() as i64; // quantize
                *delta_counts.entry(key).or_insert(0) += 1;
            }
            let (dom_key, dom_count) = delta_counts.iter()
                .max_by_key(|(_, &c)| c)
                .map(|(&k, &c)| (k, c))
                .unwrap_or((0, 0));

            let stride_frac = dom_count as f64 / deltas.len() as f64;
            stride_scores.push(stride_frac);

            if stride_frac > best_stride_score {
                best_stride_score = stride_frac;
                best_stride = dom_key as f64 / 1000.0;
            }
        }

        let sequential_score = if seq_scores.is_empty() { 0.0 }
            else { seq_scores.iter().sum::<f64>() / seq_scores.len() as f64 };
        let strided_score = if stride_scores.is_empty() { 0.0 }
            else { stride_scores.iter().sum::<f64>() / stride_scores.len() as f64 };

        // Random score: based on entropy of inter-point distances
        // High entropy in neighbor distances = random-like access
        let mut dist_entropies = Vec::new();
        for i in 0..max_n.min(80) {
            let knn = shared.knn(i);
            let dists: Vec<f64> = knn.iter()
                .filter_map(|&j| {
                    let j = j as usize;
                    if j < n && j != i { Some(shared.dist(i, j)) } else { None }
                })
                .collect();
            if dists.len() >= 2 {
                let mean = dists.iter().sum::<f64>() / dists.len() as f64;
                let var = dists.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>()
                    / dists.len() as f64;
                // Coefficient of variation as entropy proxy
                let cv = if mean > 1e-12 { var.sqrt() / mean } else { 0.0 };
                dist_entropies.push(cv.min(2.0));
            }
        }
        let random_score = if dist_entropies.is_empty() { 0.0 }
            else { (dist_entropies.iter().sum::<f64>() / dist_entropies.len() as f64).min(1.0) };

        // Tiled score: detect block structure by checking if data forms clusters
        // with regular spacing between cluster centers
        let mut tile_score = 0.0f64;
        if d >= 2 && max_n >= 8 {
            // Check if rows naturally partition into equal-sized blocks
            let block_sizes = [2, 4, 8, 16];
            let mut best_regularity = 0.0f64;
            for &bs in &block_sizes {
                if max_n < bs * 2 { continue; }
                let n_blocks = max_n / bs;
                let mut block_means = Vec::with_capacity(n_blocks);
                for b in 0..n_blocks {
                    let mut mean = vec![0.0f64; d];
                    for row in 0..bs {
                        let idx = (b * bs + row) * d;
                        for dim in 0..d {
                            mean[dim] += data[idx + dim];
                        }
                    }
                    for m in &mut mean { *m /= bs as f64; }
                    block_means.push(mean);
                }
                // Check regularity: variance of inter-block distances
                if block_means.len() >= 2 {
                    let mut inter_dists = Vec::new();
                    for w in block_means.windows(2) {
                        let dist: f64 = w[0].iter().zip(w[1].iter())
                            .map(|(a, b)| (a - b) * (a - b)).sum::<f64>().sqrt();
                        inter_dists.push(dist);
                    }
                    let mean_d = inter_dists.iter().sum::<f64>() / inter_dists.len() as f64;
                    let var_d = inter_dists.iter()
                        .map(|&v| (v - mean_d) * (v - mean_d)).sum::<f64>()
                        / inter_dists.len() as f64;
                    let regularity = if mean_d > 1e-12 {
                        1.0 - (var_d.sqrt() / mean_d).min(1.0)
                    } else { 0.0 };
                    if regularity > best_regularity { best_regularity = regularity; }
                }
            }
            tile_score = best_regularity;
        }

        // Reuse distance: average "distance" between points with similar values
        let mut reuse_distances = Vec::new();
        for i in 0..max_n.min(50) {
            let knn = shared.knn(i);
            if let Some(&nearest) = knn.first() {
                let nearest = nearest as usize;
                // "Reuse distance" = index gap between point and its nearest neighbor
                let gap = if nearest > i { nearest - i } else { i - nearest };
                reuse_distances.push(gap as f64);
            }
        }
        let reuse_distance = if reuse_distances.is_empty() { 0.0 }
            else { reuse_distances.iter().sum::<f64>() / reuse_distances.len() as f64 };

        let mut result = HashMap::new();
        result.insert("sequential_score".to_string(), vec![sequential_score]);
        result.insert("strided_score".to_string(), vec![strided_score]);
        result.insert("random_score".to_string(), vec![random_score]);
        result.insert("tiled_score".to_string(), vec![tile_score]);
        result.insert("stride_value".to_string(), vec![best_stride]);
        result.insert("reuse_distance".to_string(), vec![reuse_distance]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_memory_pattern_sequential() {
        // Sequential data: 0,1,2,3,...
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.5).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = MemoryPatternLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r["sequential_score"][0] > 0.5);
    }

    #[test]
    fn test_memory_pattern_strided() {
        // Strided: every other element with constant stride
        let mut data = Vec::new();
        for i in 0..20 {
            data.push((i * 3) as f64); // stride = 3
            data.push(0.0);
        }
        let shared = SharedData::compute(&data, 20, 2);
        let r = MemoryPatternLens.scan(&data, 20, 2, &shared);
        assert!(r["strided_score"][0] > 0.5);
    }
}
