use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// DimensionalBridgeLens: Cross-dimension bridge detection.
///
/// Measures mutual information between dimension pairs to find
/// strong cross-dimensional coupling, asymmetry, and bridge structures.
pub struct DimensionalBridgeLens;

impl Lens for DimensionalBridgeLens {
    fn name(&self) -> &str { "DimensionalBridgeLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let _max_n = n.min(200);

        if d < 2 { return HashMap::new(); }

        let mi_threshold = 0.1;
        let mut bridge_count = 0u32;
        let mut max_mi = 0.0_f64;
        let mut total_mi = 0.0_f64;
        let mut pair_count = 0u32;
        let mut forward_sum = 0.0_f64;
        let mut backward_sum = 0.0_f64;

        for di in 0..d {
            for dj in (di + 1)..d {
                let mi_ij = shared.mi(di, dj);
                let mi_ji = shared.mi(dj, di);
                let avg_mi = (mi_ij + mi_ji) / 2.0;

                total_mi += avg_mi;
                pair_count += 1;

                // Non-adjacent: dimensions more than 1 apart
                if (dj - di) > 1 && avg_mi > max_mi {
                    max_mi = avg_mi;
                }

                if avg_mi > mi_threshold {
                    bridge_count += 1;
                }

                forward_sum += mi_ij;
                backward_sum += mi_ji;
            }
        }

        let dimensional_coupling = if pair_count > 0 {
            total_mi / pair_count as f64
        } else {
            0.0
        };

        let bridge_asymmetry = if (forward_sum + backward_sum) > 1e-12 {
            (forward_sum - backward_sum).abs() / (forward_sum + backward_sum)
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("bridge_count".to_string(), vec![bridge_count as f64]);
        result.insert("bridge_strength".to_string(), vec![max_mi]);
        result.insert("dimensional_coupling".to_string(), vec![dimensional_coupling]);
        result.insert("bridge_asymmetry".to_string(), vec![bridge_asymmetry]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1).sin()).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = DimensionalBridgeLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("bridge_count"));
        assert!(result.contains_key("dimensional_coupling"));
    }

    #[test]
    fn test_small_n_guard() {
        let data: Vec<f64> = vec![1.0, 2.0, 3.0, 4.0];
        let shared = SharedData::compute(&data, 2, 2);
        let result = DimensionalBridgeLens.scan(&data, 2, 2, &shared);
        assert!(result.is_empty());
    }
}
