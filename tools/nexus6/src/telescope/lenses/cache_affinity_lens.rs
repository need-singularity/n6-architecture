use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CacheAffinityLens: Analyze data layout fitness for CPU cache hierarchy.
///
/// Determines whether data patterns are L1/L2/L3 friendly by measuring
/// spatial locality, temporal locality, and working set size. LLVM
/// cannot restructure data layouts, but this lens identifies when
/// tiling, padding, or AoS-to-SoA transforms would help.
///
/// Metrics:
///   1. spatial_locality: how often neighbors in data are neighbors in value
///   2. temporal_locality: reuse within small windows (L1-sized)
///   3. working_set_ratio: effective data size vs cache levels
///   4. l1_fit_score: probability data fits in L1 (32-64KB)
///   5. l2_fit_score: probability data fits in L2 (256KB-1MB)
///   6. prefetch_predictability: how predictable the access pattern is
pub struct CacheAffinityLens;

// Typical cache sizes in elements (f64 = 8 bytes each)
const L1_ELEMS: usize = 4096;    // ~32KB
const L2_ELEMS: usize = 65536;   // ~512KB
const L3_ELEMS: usize = 1048576; // ~8MB

impl Lens for CacheAffinityLens {
    fn name(&self) -> &str { "CacheAffinityLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);
        let total_elems = max_n * d;

        // --- Spatial locality: adjacent rows in memory should be close in value ---
        let mut spatial_hits = 0usize;
        let mut spatial_total = 0usize;
        for i in 0..(max_n - 1) {
            let row_i = &data[i * d..(i * d + d).min(data.len())];
            let row_next = &data[(i + 1) * d..((i + 1) * d + d).min(data.len())];
            let dist_sq: f64 = row_i.iter().zip(row_next.iter())
                .map(|(a, b)| (a - b) * (a - b)).sum();
            // Compare to average KNN distance
            let knn = shared.knn(i);
            if !knn.is_empty() {
                let avg_knn_dist: f64 = knn.iter()
                    .filter_map(|&j| {
                        let j = j as usize;
                        if j < n && j != i { Some(shared.dist(i, j)) } else { None }
                    })
                    .sum::<f64>() / knn.len() as f64;
                // If next-in-memory is within KNN distance, good spatial locality
                if dist_sq.sqrt() <= avg_knn_dist * 1.5 {
                    spatial_hits += 1;
                }
                spatial_total += 1;
            }
        }
        let spatial_locality = if spatial_total > 0 {
            spatial_hits as f64 / spatial_total as f64
        } else { 0.0 };

        // --- Temporal locality: within cache-line-sized windows, measure reuse ---
        let cache_line_rows = (64 / (d * 8).max(1)).max(1).min(max_n); // 64-byte cache line
        let mut reuse_sum = 0.0f64;
        let mut reuse_count = 0usize;
        let window = cache_line_rows.max(4).min(max_n);
        for start in (0..max_n).step_by(window.max(1)) {
            let end = (start + window).min(max_n);
            if end - start < 2 { continue; }
            // Count value reuse within window (similar values)
            let mut intra_sim = 0.0f64;
            let mut pairs = 0usize;
            for i in start..end {
                for j in (i + 1)..end.min(i + 4) {
                    let row_i = &data[i * d..(i * d + d).min(data.len())];
                    let row_j = &data[j * d..(j * d + d).min(data.len())];
                    let dot: f64 = row_i.iter().zip(row_j.iter()).map(|(a, b)| a * b).sum();
                    let ni: f64 = row_i.iter().map(|v| v * v).sum::<f64>().sqrt();
                    let nj: f64 = row_j.iter().map(|v| v * v).sum::<f64>().sqrt();
                    if ni > 1e-12 && nj > 1e-12 {
                        intra_sim += (dot / (ni * nj)).abs();
                        pairs += 1;
                    }
                }
            }
            if pairs > 0 {
                reuse_sum += intra_sim / pairs as f64;
                reuse_count += 1;
            }
        }
        let temporal_locality = if reuse_count > 0 {
            reuse_sum / reuse_count as f64
        } else { 0.0 };

        // --- Working set analysis ---
        // Effective working set: count distinct "regions" using quantization
        let n_bins = 32usize;
        let mut occupied_bins = vec![false; n_bins * d.min(8)];
        for dim in 0..d.min(8) {
            let col: Vec<f64> = (0..max_n).map(|i| data[i * d + dim]).collect();
            let min_v = col.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_v = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let range = max_v - min_v + 1e-12;
            for &v in &col {
                let bin = ((v - min_v) / range * (n_bins - 1) as f64) as usize;
                occupied_bins[dim * n_bins + bin.min(n_bins - 1)] = true;
            }
        }
        let active_bins = occupied_bins.iter().filter(|&&b| b).count();
        let total_bins = occupied_bins.len();
        let working_set_ratio = active_bins as f64 / total_bins as f64;

        // --- Cache level fit scores ---
        let l1_fit = if total_elems <= L1_ELEMS { 1.0 }
            else { (L1_ELEMS as f64 / total_elems as f64).min(1.0) };
        let l2_fit = if total_elems <= L2_ELEMS { 1.0 }
            else { (L2_ELEMS as f64 / total_elems as f64).min(1.0) };

        // --- Prefetch predictability: how well can we predict next access ---
        // Measure autocorrelation of inter-row differences
        let mut delta_autocorr = 0.0f64;
        if max_n >= 4 && d >= 1 {
            let deltas: Vec<f64> = (0..max_n - 1)
                .map(|i| data[(i + 1) * d] - data[i * d])
                .collect();
            if deltas.len() >= 3 {
                let mean_d = deltas.iter().sum::<f64>() / deltas.len() as f64;
                let var_d: f64 = deltas.iter().map(|&v| (v - mean_d) * (v - mean_d)).sum::<f64>();
                if var_d > 1e-12 {
                    let cov: f64 = deltas.windows(2)
                        .map(|w| (w[0] - mean_d) * (w[1] - mean_d))
                        .sum::<f64>();
                    delta_autocorr = (cov / var_d).abs().min(1.0);
                }
            }
        }

        let mut result = HashMap::new();
        result.insert("spatial_locality".to_string(), vec![spatial_locality]);
        result.insert("temporal_locality".to_string(), vec![temporal_locality]);
        result.insert("working_set_ratio".to_string(), vec![working_set_ratio]);
        result.insert("l1_fit_score".to_string(), vec![l1_fit]);
        result.insert("l2_fit_score".to_string(), vec![l2_fit]);
        result.insert("prefetch_predictability".to_string(), vec![delta_autocorr]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_cache_affinity_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = CacheAffinityLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("spatial_locality"));
        assert!(r.contains_key("l1_fit_score"));
    }

    #[test]
    fn test_cache_affinity_small_data() {
        // Small data should fit in L1
        let data: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let r = CacheAffinityLens.scan(&data, 10, 2, &shared);
        assert_eq!(r["l1_fit_score"][0], 1.0);
    }
}
