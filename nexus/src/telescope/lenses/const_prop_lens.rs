use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConstPropLens: Find compile-time computable expressions.
///
/// Detects data patterns that represent constant or near-constant
/// values, deterministic functions of indices, and expressions that
/// could be hoisted out of loops. LLVM's constant propagation works
/// at the SSA level but misses patterns visible in data layout
/// (e.g., repeated rows, linear functions of index, lookup tables).
///
/// Metrics:
///   1. constant_fraction: fraction of data that is effectively constant
///   2. linear_predictable: fraction fitting a linear function of index
///   3. lookup_table_score: likelihood data is a precomputed LUT
///   4. loop_invariant_dims: number of dimensions that don't vary
///   5. hoistable_fraction: total fraction of data hoistable from loops
///   6. redundant_computation: fraction of duplicate or near-duplicate rows
pub struct ConstPropLens;

impl Lens for ConstPropLens {
    fn name(&self) -> &str { "ConstPropLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // --- Constant fraction: dimensions where all values are the same ---
        let mut constant_dims = 0usize;
        let epsilon = 1e-10;
        let mut dim_is_const = vec![false; d];

        for dim in 0..d {
            let first = data[dim];
            let all_same = (1..max_n).all(|i| (data[i * d + dim] - first).abs() < epsilon);
            if all_same {
                constant_dims += 1;
                dim_is_const[dim] = true;
            }
        }
        let constant_fraction = constant_dims as f64 / d as f64;

        // --- Linear predictable: fit y = a*i + b per dimension ---
        let mut linear_dims = 0usize;
        let mut linear_r2_sum = 0.0f64;
        let d_check = d.min(32);

        for dim in 0..d_check {
            if dim_is_const[dim] { linear_dims += 1; continue; }

            let col: Vec<f64> = (0..max_n).map(|i| data[i * d + dim]).collect();
            // Simple linear regression: y = a*x + b
            let n_f = max_n as f64;
            let sum_x: f64 = (0..max_n).map(|i| i as f64).sum();
            let sum_y: f64 = col.iter().sum();
            let sum_xy: f64 = col.iter().enumerate().map(|(i, &y)| i as f64 * y).sum();
            let sum_x2: f64 = (0..max_n).map(|i| (i * i) as f64).sum();

            let denom = n_f * sum_x2 - sum_x * sum_x;
            if denom.abs() < 1e-12 { continue; }

            let a = (n_f * sum_xy - sum_x * sum_y) / denom;
            let b = (sum_y - a * sum_x) / n_f;

            // R-squared
            let mean_y = sum_y / n_f;
            let ss_tot: f64 = col.iter().map(|&y| (y - mean_y) * (y - mean_y)).sum();
            let ss_res: f64 = col.iter().enumerate()
                .map(|(i, &y)| {
                    let pred = a * i as f64 + b;
                    (y - pred) * (y - pred)
                })
                .sum();

            let r2 = if ss_tot > 1e-12 { 1.0 - ss_res / ss_tot } else { 1.0 };
            linear_r2_sum += r2;
            if r2 > 0.95 { linear_dims += 1; }
        }
        let linear_predictable = linear_dims as f64 / d_check as f64;

        // --- Lookup table score: data has small unique value set ---
        let mut all_unique_values: Vec<i64> = Vec::new();
        for dim in 0..d_check {
            let mut values: Vec<i64> = (0..max_n)
                .map(|i| (data[i * d + dim] * 1e6).round() as i64)
                .collect();
            values.sort();
            values.dedup();
            all_unique_values.extend(values.iter());
        }
        all_unique_values.sort();
        all_unique_values.dedup();
        let unique_ratio = all_unique_values.len() as f64 / (max_n * d_check).max(1) as f64;
        // Low unique ratio = LUT-like
        let lookup_table_score = (1.0 - unique_ratio).max(0.0);

        // --- Loop invariant dims: dims with very low variance ---
        let mut invariant_dims = 0usize;
        for dim in 0..d {
            let col: Vec<f64> = (0..max_n).map(|i| data[i * d + dim]).collect();
            let mean = col.iter().sum::<f64>() / max_n as f64;
            let var = col.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>() / max_n as f64;
            let range = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max)
                - col.iter().cloned().fold(f64::INFINITY, f64::min);
            // Near-zero variance relative to range
            if range < 1e-8 || var / (range * range + 1e-12) < 0.001 {
                invariant_dims += 1;
            }
        }

        // --- Hoistable fraction: combine constant + linear + invariant ---
        let hoistable_fraction = (constant_fraction * 0.4
            + linear_predictable * 0.3
            + (invariant_dims as f64 / d as f64) * 0.3)
            .min(1.0);

        // --- Redundant computation: duplicate or near-duplicate rows ---
        let mut row_hashes: HashMap<Vec<i64>, usize> = HashMap::new();
        for i in 0..max_n {
            let hash: Vec<i64> = (0..d.min(16))
                .map(|dim| (data[i * d + dim] * 1e4).round() as i64)
                .collect();
            *row_hashes.entry(hash).or_insert(0) += 1;
        }
        let duplicate_rows: usize = row_hashes.values().filter(|&&c| c > 1).map(|&c| c - 1).sum();
        let redundant_computation = duplicate_rows as f64 / max_n as f64;

        let mut result = HashMap::new();
        result.insert("constant_fraction".to_string(), vec![constant_fraction]);
        result.insert("linear_predictable".to_string(), vec![linear_predictable]);
        result.insert("lookup_table_score".to_string(), vec![lookup_table_score]);
        result.insert("loop_invariant_dims".to_string(), vec![invariant_dims as f64]);
        result.insert("hoistable_fraction".to_string(), vec![hoistable_fraction]);
        result.insert("redundant_computation".to_string(), vec![redundant_computation]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_const_prop_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = ConstPropLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("constant_fraction"));
        assert!(r.contains_key("hoistable_fraction"));
    }

    #[test]
    fn test_const_prop_constant_data() {
        // All same value = fully constant
        let data: Vec<f64> = vec![3.14; 40];
        let shared = SharedData::compute(&data, 20, 2);
        let r = ConstPropLens.scan(&data, 20, 2, &shared);
        assert_eq!(r["constant_fraction"][0], 1.0);
    }

    #[test]
    fn test_const_prop_linear() {
        // Perfectly linear data
        let mut data = Vec::new();
        for i in 0..20 {
            data.push(i as f64 * 2.0 + 1.0); // y = 2x + 1
            data.push(5.0); // constant
        }
        let shared = SharedData::compute(&data, 20, 2);
        let r = ConstPropLens.scan(&data, 20, 2, &shared);
        assert!(r["linear_predictable"][0] > 0.9);
    }
}
