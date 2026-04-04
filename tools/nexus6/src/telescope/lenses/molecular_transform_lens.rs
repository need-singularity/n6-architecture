use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MolecularTransformLens: Detects transformations/reactions in data over time.
/// Measures transform_count, reaction_energy, catalyst_presence, product_diversity, reversibility.
pub struct MolecularTransformLens;

impl Lens for MolecularTransformLens {
    fn name(&self) -> &str { "MolecularTransformLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Detect structural changes between consecutive rows
        let change_thresh = 0.3;
        let mut transforms = 0usize;
        let mut energy_sum = 0.0_f64;
        let mut feature_changes = vec![0.0_f64; d]; // track per-feature total change

        for i in 1..max_n {
            let mut row_change = 0.0;
            for j in 0..d {
                let diff = (data[i * d + j] - data[(i - 1) * d + j]).abs();
                row_change += diff * diff;
                feature_changes[j] += diff;
            }
            let row_dist = row_change.sqrt();
            if row_dist > change_thresh {
                transforms += 1;
                energy_sum += row_dist;
            }
        }

        let steps = (max_n - 1).max(1) as f64;
        let reaction_energy = energy_sum / steps;

        // Catalyst presence: features that enable transforms without changing much themselves
        // Low self-change but present during high-change rows
        let total_change: f64 = feature_changes.iter().sum();
        let mean_change = total_change / d.max(1) as f64;
        let catalyst_count = feature_changes.iter()
            .filter(|&&c| c < mean_change * 0.3 && c > 1e-12)
            .count();
        let catalyst_presence = catalyst_count as f64 / d.max(1) as f64;

        // Product diversity: how varied the outputs are (entropy of last-row distribution)
        let last_row = &data[((max_n - 1) * d)..((max_n - 1) * d + d).min(data.len())];
        let first_row = &data[..d.min(data.len())];
        let mut unique_bins = std::collections::HashSet::new();
        let n_bins = 16;
        let (mn, mx) = last_row.iter().fold((f64::MAX, f64::MIN), |(lo, hi), &v| (lo.min(v), hi.max(v)));
        let rng = (mx - mn).max(1e-15);
        for &v in last_row {
            unique_bins.insert(((v - mn) / rng * (n_bins - 1) as f64) as u32);
        }
        let product_diversity = unique_bins.len() as f64 / n_bins as f64;

        // Reversibility: can we get from last row back to first row?
        // Measure cosine similarity between forward and backward change vectors
        let mut fwd_dot = 0.0_f64;
        let mut fwd_norm = 0.0_f64;
        let mut bwd_norm = 0.0_f64;
        for j in 0..d {
            let fwd = data[(max_n - 1) * d + j] - data[j];
            let bwd = data[j] - data[(max_n - 1) * d + j];
            fwd_dot += fwd * bwd;
            fwd_norm += fwd * fwd;
            bwd_norm += bwd * bwd;
        }
        let reversibility = if fwd_norm > 1e-15 && bwd_norm > 1e-15 {
            // Perfect reversibility = vectors are opposite = cosine = -1
            let cos = fwd_dot / (fwd_norm.sqrt() * bwd_norm.sqrt());
            ((-cos) + 1.0) / 2.0 // map -1→1 (reversible), 1→0 (irreversible)
        } else { 0.5 };

        let mut result = HashMap::new();
        result.insert("transform_count".into(), vec![transforms as f64]);
        result.insert("reaction_energy".into(), vec![reaction_energy]);
        result.insert("catalyst_presence".into(), vec![catalyst_presence]);
        result.insert("product_diversity".into(), vec![product_diversity]);
        result.insert("reversibility".into(), vec![reversibility]);
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
        let result = MolecularTransformLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
