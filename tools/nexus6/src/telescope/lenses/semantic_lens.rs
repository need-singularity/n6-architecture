use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// SemanticLens: Detect high-level computational patterns in data flow.
///
/// Recognizes reduction, scan (prefix-sum), stencil, matrix-multiply,
/// and gather/scatter patterns by analyzing the structure of data
/// dependencies and access patterns. These patterns are the ones
/// LLVM often fails to auto-vectorize or parallelize because they
/// require semantic understanding beyond SSA-level IR.
///
/// Metrics:
///   1. reduction_score: likelihood data contains a reduction pattern
///   2. scan_score: prefix-sum / scan pattern detection
///   3. stencil_score: neighbor-dependent computation (convolution-like)
///   4. matmul_score: outer-product / dot-product accumulation pattern
///   5. gather_scatter_score: indirect access pattern strength
///   6. dominant_pattern: index of strongest pattern (0-4)
pub struct SemanticLens;

impl Lens for SemanticLens {
    fn name(&self) -> &str { "SemanticLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // --- Reduction pattern: values converge toward a single accumulator ---
        // Check if successive rows collapse variance (partial sums converging)
        let mut row_vars = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let row = &data[i * d..(i * d + d).min(data.len())];
            let mean = row.iter().sum::<f64>() / d as f64;
            let var = row.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / d as f64;
            row_vars.push(var);
        }
        // Reduction signature: variance monotonically decreasing
        let mut reduction_decreasing = 0usize;
        for w in row_vars.windows(2) {
            if w[1] <= w[0] + 1e-12 { reduction_decreasing += 1; }
        }
        let reduction_score = if max_n > 1 {
            reduction_decreasing as f64 / (max_n - 1) as f64
        } else { 0.0 };

        // --- Scan (prefix-sum) pattern: each row ≈ cumulative sum of previous ---
        let mut scan_hits = 0usize;
        let mut scan_total = 0usize;
        for dim in 0..d.min(8) {
            let col: Vec<f64> = (0..max_n).map(|i| data[i * d + dim]).collect();
            // Check if col[i] ≈ col[i-1] + delta for consistent delta
            if col.len() >= 3 {
                let deltas: Vec<f64> = col.windows(2).map(|w| w[1] - w[0]).collect();
                let mean_delta = deltas.iter().sum::<f64>() / deltas.len() as f64;
                let delta_var = deltas.iter()
                    .map(|&d| (d - mean_delta) * (d - mean_delta))
                    .sum::<f64>() / deltas.len() as f64;
                // Low variance in deltas = scan/prefix-sum pattern
                let col_range = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                    - col.iter().cloned().fold(f64::INFINITY, f64::min);
                if col_range > 1e-12 && delta_var / (col_range * col_range + 1e-12) < 0.01 {
                    scan_hits += 1;
                }
                scan_total += 1;
            }
        }
        let scan_score = if scan_total > 0 { scan_hits as f64 / scan_total as f64 } else { 0.0 };

        // --- Stencil pattern: each point's value correlates with its neighbors ---
        let mut stencil_corr_sum = 0.0f64;
        let mut stencil_count = 0usize;
        for i in 0..max_n.min(100) {
            let knn = shared.knn(i);
            if knn.is_empty() { continue; }
            let row_i = &data[i * d..(i * d + d).min(data.len())];
            let norm_i: f64 = row_i.iter().map(|v| v * v).sum::<f64>().sqrt();
            if norm_i < 1e-15 { continue; }
            for &j in knn.iter().take(3) {
                let j = j as usize;
                if j >= n { continue; }
                let row_j = &data[j * d..(j * d + d).min(data.len())];
                let norm_j: f64 = row_j.iter().map(|v| v * v).sum::<f64>().sqrt();
                if norm_j < 1e-15 { continue; }
                let dot: f64 = row_i.iter().zip(row_j.iter()).map(|(a, b)| a * b).sum();
                stencil_corr_sum += dot / (norm_i * norm_j);
                stencil_count += 1;
            }
        }
        let stencil_score = if stencil_count > 0 {
            (stencil_corr_sum / stencil_count as f64).max(0.0)
        } else { 0.0 };

        // --- Matmul pattern: detect outer-product structure via MI between dims ---
        let mut matmul_mi_sum = 0.0f64;
        let mut matmul_pairs = 0usize;
        let d_check = d.min(16);
        for di in 0..d_check {
            for dj in (di + 1)..d_check {
                matmul_mi_sum += shared.mi(di, dj);
                matmul_pairs += 1;
            }
        }
        let matmul_score = if matmul_pairs > 0 {
            (matmul_mi_sum / matmul_pairs as f64).min(1.0)
        } else { 0.0 };

        // --- Gather/scatter: high variance in neighbor distances = indirect access ---
        let mut dist_vars = Vec::with_capacity(max_n);
        for i in 0..max_n.min(100) {
            let knn = shared.knn(i);
            if knn.len() < 2 { continue; }
            let dists: Vec<f64> = knn.iter()
                .filter_map(|&j| {
                    let j = j as usize;
                    if j < n && j != i { Some(shared.dist(i, j)) } else { None }
                })
                .collect();
            if dists.len() < 2 { continue; }
            let mean_d = dists.iter().sum::<f64>() / dists.len() as f64;
            let var_d = dists.iter().map(|&v| (v - mean_d) * (v - mean_d)).sum::<f64>()
                / dists.len() as f64;
            dist_vars.push(var_d / (mean_d * mean_d + 1e-12));
        }
        let gather_scatter_score = if dist_vars.is_empty() { 0.0 } else {
            let mean_cv = dist_vars.iter().sum::<f64>() / dist_vars.len() as f64;
            mean_cv.min(1.0)
        };

        // Dominant pattern
        let scores = [reduction_score, scan_score, stencil_score, matmul_score, gather_scatter_score];
        let dominant = scores.iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let mut result = HashMap::new();
        result.insert("reduction_score".to_string(), vec![reduction_score]);
        result.insert("scan_score".to_string(), vec![scan_score]);
        result.insert("stencil_score".to_string(), vec![stencil_score]);
        result.insert("matmul_score".to_string(), vec![matmul_score]);
        result.insert("gather_scatter_score".to_string(), vec![gather_scatter_score]);
        result.insert("dominant_pattern".to_string(), vec![dominant as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_semantic_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = SemanticLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("reduction_score"));
        assert!(r.contains_key("dominant_pattern"));
    }

    #[test]
    fn test_semantic_scan_pattern() {
        // Prefix-sum like data: each row is cumulative
        let mut data = Vec::new();
        for i in 0..20 {
            for j in 0..3 {
                data.push((i * 3 + j) as f64); // monotonically increasing
            }
        }
        let shared = SharedData::compute(&data, 20, 3);
        let r = SemanticLens.scan(&data, 20, 3, &shared);
        assert!(r["scan_score"][0] > 0.0);
    }
}
