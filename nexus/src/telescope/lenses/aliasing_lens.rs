use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// AliasingLens: Pointer alias analysis via data pattern detection.
///
/// Detects aliasing patterns in data that would prevent LLVM from
/// optimizing. Works by finding overlapping value ranges, shared
/// memory regions, and identity relationships between data points.
/// Stricter than LLVM's alias analysis because it uses statistical
/// ownership inference.
///
/// Metrics:
///   1. alias_density: fraction of point pairs that could alias
///   2. unique_ownership: fraction of data with exclusive access pattern
///   3. overlap_score: degree of value range overlap between dimensions
///   4. restrict_safe_fraction: fraction safe to mark as restrict/noalias
///   5. write_conflict_risk: estimated probability of write-after-write
///   6. alias_clusters: number of distinct aliasing groups
pub struct AliasingLens;

impl Lens for AliasingLens {
    fn name(&self) -> &str { "AliasingLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(200);

        // --- Alias density: pairs with near-identical values ---
        // Two data points "alias" if they are extremely close (same memory location)
        let mut alias_pairs = 0u64;
        let mut total_checked = 0u64;
        let epsilon = 1e-10;

        for i in 0..max_n.min(100) {
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let j = j as usize;
                if j >= max_n || j <= i { continue; }
                let dist = shared.dist(i, j);
                if dist < epsilon {
                    alias_pairs += 1;
                }
                total_checked += 1;
            }
        }
        let alias_density = if total_checked > 0 {
            alias_pairs as f64 / total_checked as f64
        } else { 0.0 };

        // --- Unique ownership: how many points have distinct, non-overlapping values ---
        // Quantize each point to a "cell" and check if multiple points map to same cell
        let n_bins = 32usize;
        let d_check = d.min(8);
        let mut cell_counts: HashMap<Vec<u16>, usize> = HashMap::new();
        let mut dim_ranges = Vec::with_capacity(d_check);

        for dim in 0..d_check {
            let mut min_v = f64::INFINITY;
            let mut max_v = f64::NEG_INFINITY;
            for i in 0..max_n {
                let v = data[i * d + dim];
                if v < min_v { min_v = v; }
                if v > max_v { max_v = v; }
            }
            dim_ranges.push((min_v, max_v));
        }

        for i in 0..max_n {
            let cell: Vec<u16> = (0..d_check).map(|dim| {
                let (lo, hi) = dim_ranges[dim];
                let range = hi - lo + 1e-12;
                let bin = ((data[i * d + dim] - lo) / range * (n_bins - 1) as f64) as u16;
                bin.min(n_bins as u16 - 1)
            }).collect();
            *cell_counts.entry(cell).or_insert(0) += 1;
        }

        let unique_cells = cell_counts.values().filter(|&&c| c == 1).count();
        let unique_ownership = unique_cells as f64 / max_n as f64;

        // --- Overlap score: value range overlap between dimensions ---
        let mut overlap_sum = 0.0f64;
        let mut overlap_pairs = 0usize;
        for di in 0..d_check {
            for dj in (di + 1)..d_check {
                let (lo_i, hi_i) = dim_ranges[di];
                let (lo_j, hi_j) = dim_ranges[dj];
                let overlap_lo = lo_i.max(lo_j);
                let overlap_hi = hi_i.min(hi_j);
                let range_union = (hi_i.max(hi_j)) - (lo_i.min(lo_j));
                if overlap_hi > overlap_lo && range_union > 1e-12 {
                    overlap_sum += (overlap_hi - overlap_lo) / range_union;
                }
                overlap_pairs += 1;
            }
        }
        let overlap_score = if overlap_pairs > 0 {
            overlap_sum / overlap_pairs as f64
        } else { 0.0 };

        // --- Restrict-safe fraction ---
        // Points with high unique ownership and no close neighbors are restrict-safe
        let mut restrict_safe = 0usize;
        for i in 0..max_n {
            let knn = shared.knn(i);
            let nearest_dist = knn.iter()
                .filter_map(|&j| {
                    let j = j as usize;
                    if j < n && j != i { Some(shared.dist(i, j)) } else { None }
                })
                .fold(f64::INFINITY, f64::min);
            // If nearest neighbor is far away, this point is "owned" exclusively
            if nearest_dist > 0.1 {
                restrict_safe += 1;
            }
        }
        let restrict_safe_fraction = restrict_safe as f64 / max_n as f64;

        // --- Write conflict risk: based on clustering of similar values ---
        // Many points in same cell = high write conflict risk
        let max_cell_count = cell_counts.values().copied().max().unwrap_or(1);
        let write_conflict_risk = if max_n > 1 {
            (max_cell_count as f64 - 1.0) / (max_n as f64 - 1.0)
        } else { 0.0 };

        // --- Alias clusters: groups of points that could alias each other ---
        let alias_clusters = cell_counts.values().filter(|&&c| c > 1).count() as f64;

        let mut result = HashMap::new();
        result.insert("alias_density".to_string(), vec![alias_density]);
        result.insert("unique_ownership".to_string(), vec![unique_ownership]);
        result.insert("overlap_score".to_string(), vec![overlap_score]);
        result.insert("restrict_safe_fraction".to_string(), vec![restrict_safe_fraction]);
        result.insert("write_conflict_risk".to_string(), vec![write_conflict_risk]);
        result.insert("alias_clusters".to_string(), vec![alias_clusters]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_aliasing_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = AliasingLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("alias_density"));
        assert!(r.contains_key("unique_ownership"));
    }

    #[test]
    fn test_aliasing_duplicates() {
        // Duplicate rows should increase alias density
        let mut data = Vec::new();
        for _ in 0..10 { data.push(1.0); data.push(2.0); } // 10 identical rows
        for i in 0..10 { data.push(i as f64 * 10.0); data.push(i as f64 * 10.0); }
        let shared = SharedData::compute(&data, 20, 2);
        let r = AliasingLens.scan(&data, 20, 2, &shared);
        assert!(r["write_conflict_risk"][0] > 0.0);
    }
}
