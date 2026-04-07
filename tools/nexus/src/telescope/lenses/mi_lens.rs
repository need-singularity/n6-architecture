//! MiLens — Mutual Information lens (migrated from telescope-rs mi.rs)
//!
//! Computes pairwise mutual information between data dimensions to detect
//! information-theoretic dependencies. Uses binned histogram MI estimation.

use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{self, SharedData};

/// MiLens: Mutual Information analysis between feature dimensions.
///
/// Algorithm:
///   1. Extract column vectors from row-major data
///   2. For each pair of dimensions, compute MI via binned histogram
///   3. Identify high-MI pairs (strong dependencies)
///   4. Compute average MI as integration proxy (Phi-like measure)
pub struct MiLens;

impl Lens for MiLens {
    fn name(&self) -> &str {
        "MiLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 2 || d < 2 {
            return HashMap::new();
        }

        let n_bins = 16usize;

        // Use pre-computed MI matrix from SharedData if available
        let mi_matrix = &shared.mi_matrix;

        // Compute average MI (Phi proxy)
        let mut total_mi = 0.0;
        let mut pair_count = 0usize;
        let mut high_mi_pairs_i = Vec::new();
        let mut high_mi_pairs_j = Vec::new();
        let mut high_mi_values = Vec::new();

        // Also compute per-column MI using our utility for finer-grained analysis
        let cols = shared_data::column_vectors(data, n, d);

        for di in 0..d {
            for dj in (di + 1)..d {
                // Use shared MI matrix (from GPU fallback)
                let mi_val = if mi_matrix.len() == d * d {
                    mi_matrix[di * d + dj]
                } else {
                    // Fallback: compute directly
                    shared_data::mutual_info(&cols[di], &cols[dj], n_bins)
                };

                total_mi += mi_val;
                pair_count += 1;

                // High MI threshold: > 0.1 nats
                if mi_val > 0.1 {
                    high_mi_pairs_i.push(di as f64);
                    high_mi_pairs_j.push(dj as f64);
                    high_mi_values.push(mi_val);
                }
            }
        }

        let avg_mi = if pair_count > 0 {
            total_mi / pair_count as f64
        } else {
            0.0
        };

        // Flatten MI matrix to Vec<f64> for output
        let mi_flat: Vec<f64> = if mi_matrix.len() == d * d {
            mi_matrix.clone()
        } else {
            // Build from scratch
            let mut flat = vec![0.0; d * d];
            for di in 0..d {
                for dj in 0..d {
                    if di != dj {
                        flat[di * d + dj] = shared_data::mutual_info(&cols[di], &cols[dj], n_bins);
                    }
                }
            }
            flat
        };

        let mut result = HashMap::new();
        result.insert("mi_matrix".to_string(), mi_flat);
        result.insert("avg_mi".to_string(), vec![avg_mi]);
        result.insert("n_high_mi_pairs".to_string(), vec![high_mi_pairs_i.len() as f64]);
        result.insert("high_mi_dim_i".to_string(), high_mi_pairs_i);
        result.insert("high_mi_dim_j".to_string(), high_mi_pairs_j);
        result.insert("high_mi_values".to_string(), high_mi_values);
        result.insert("total_mi".to_string(), vec![total_mi]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mi_lens_basic() {
        // 4 points, 3 dimensions
        let data = vec![
            0.0, 0.0, 0.0,
            1.0, 1.0, 0.5,
            2.0, 2.0, 1.0,
            3.0, 3.0, 1.5,
        ];
        let shared = SharedData::compute(&data, 4, 3);
        let lens = MiLens;
        let result = lens.scan(&data, 4, 3, &shared);

        assert!(result.contains_key("avg_mi"));
        assert!(result.contains_key("mi_matrix"));
        let avg = result["avg_mi"][0];
        assert!(avg >= 0.0, "avg MI should be non-negative");
    }

    #[test]
    fn test_mi_lens_name() {
        let lens = MiLens;
        assert_eq!(lens.name(), "MiLens");
        assert_eq!(lens.category(), "T0");
    }

    #[test]
    fn test_mi_lens_too_small() {
        let data = vec![1.0, 2.0]; // 1 point, 2 dims
        let shared = SharedData::compute(&data, 1, 2);
        let lens = MiLens;
        let result = lens.scan(&data, 1, 2, &shared);
        assert!(result.is_empty());
    }
}
