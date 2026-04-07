use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TelepathyLens: Distant correlation / non-local entanglement.
///
/// Measures correlated distant point pairs (telepathy pairs), entanglement
/// strength, spooky action at distance, channel capacity, and decoherence
/// rate (how fast correlation decays with distance).
pub struct TelepathyLens;

impl Lens for TelepathyLens {
    fn name(&self) -> &str { "TelepathyLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Compute mean distance as baseline for "distant"
        let mut total_dist = 0.0_f64;
        let mut pair_count = 0u64;
        let sample_step = (max_n / 30).max(1);
        for i in (0..max_n).step_by(sample_step) {
            for j in ((i + 1)..max_n).step_by(sample_step) {
                total_dist += shared.dist(i, j);
                pair_count += 1;
            }
        }
        let mean_dist = if pair_count > 0 { total_dist / pair_count as f64 } else { 1.0 };

        // For each pair of distant points, compute correlation via feature similarity
        // "Distant" = distance > mean_dist
        let mut telepathy_pairs = 0u32;
        let mut entanglement_sum = 0.0_f64;
        let mut max_entanglement = 0.0_f64;
        let mut near_corr_sum = 0.0_f64;
        let mut near_count = 0u32;
        let mut far_corr_sum = 0.0_f64;
        let mut far_count = 0u32;

        // Bin distances into near/far and compute feature correlation
        for i in (0..max_n).step_by(sample_step) {
            for j in ((i + 1)..max_n).step_by(sample_step) {
                let dist = shared.dist(i, j);

                // Feature correlation: dot product of normalized feature vectors
                let row_i = &data[(i * d)..((i + 1) * d)];
                let row_j = &data[(j * d)..((j + 1) * d)];
                let norm_i = row_i.iter().map(|&v| v * v).sum::<f64>().sqrt().max(1e-12);
                let norm_j = row_j.iter().map(|&v| v * v).sum::<f64>().sqrt().max(1e-12);
                let corr: f64 = row_i.iter().zip(row_j.iter())
                    .map(|(&a, &b)| a * b)
                    .sum::<f64>() / (norm_i * norm_j);
                let corr = corr.abs();

                if dist > mean_dist {
                    far_corr_sum += corr;
                    far_count += 1;
                    // Telepathy: distant but correlated
                    if corr > 0.5 {
                        telepathy_pairs += 1;
                        entanglement_sum += corr;
                        if corr > max_entanglement {
                            max_entanglement = corr;
                        }
                    }
                } else {
                    near_corr_sum += corr;
                    near_count += 1;
                }
            }
        }

        let entanglement_strength = if telepathy_pairs > 0 {
            entanglement_sum / telepathy_pairs as f64
        } else { 0.0 };

        // spooky_action_score: ratio of far-correlation to near-correlation
        let near_corr = if near_count > 0 { near_corr_sum / near_count as f64 } else { 0.0 };
        let far_corr = if far_count > 0 { far_corr_sum / far_count as f64 } else { 0.0 };
        let spooky_action_score = if near_corr > 1e-12 {
            (far_corr / near_corr).min(2.0)
        } else if far_corr > 0.0 { 2.0 } else { 0.0 };

        // channel_capacity: estimated bits from distant correlations (Shannon-like)
        let channel_capacity = if telepathy_pairs > 0 {
            (telepathy_pairs as f64).log2() * entanglement_strength
        } else { 0.0 };

        // decoherence_rate: decay of correlation with distance (reuse near/far)
        let decoherence_rate = if near_corr > 1e-12 {
            ((near_corr - far_corr) / near_corr).clamp(0.0, 1.0)
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("telepathy_pairs".to_string(), vec![telepathy_pairs as f64]);
        result.insert("entanglement_strength".to_string(), vec![entanglement_strength]);
        result.insert("spooky_action_score".to_string(), vec![spooky_action_score]);
        result.insert("channel_capacity".to_string(), vec![channel_capacity]);
        result.insert("decoherence_rate".to_string(), vec![decoherence_rate]);
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
        let result = TelepathyLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("telepathy_pairs"));
        assert!(result.contains_key("entanglement_strength"));
        assert!(result.contains_key("decoherence_rate"));
    }

    #[test]
    fn test_identical_points() {
        // All identical points: high correlation everywhere
        let data: Vec<f64> = (0..20).map(|i| if i % 2 == 0 { 1.0 } else { 2.0 }).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let result = TelepathyLens.scan(&data, 10, 2, &shared);
        assert!(result.contains_key("spooky_action_score"));
    }
}
