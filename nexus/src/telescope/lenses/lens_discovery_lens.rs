use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy, mean_var, column_vectors};

/// Meta-lens that discovers what kind of analysis would be most informative for this data.
pub struct LensDiscoveryLens;

impl Lens for LensDiscoveryLens {
    fn name(&self) -> &str { "LensDiscoveryLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Collect pairwise distances for statistics
        let mut dists = Vec::with_capacity(max_n * (max_n - 1) / 2);
        for i in 0..max_n {
            for j in (i + 1)..max_n {
                dists.push(shared.dist(i, j));
            }
        }
        let dist_entropy = shannon_entropy(&dists, 20);

        // Per-column entropy and variance analysis
        let cols = column_vectors(data, n.min(max_n), d);
        let (means, vars) = mean_var(data, n.min(max_n), d);
        let total_var: f64 = vars.iter().sum();
        let max_var = vars.iter().cloned().fold(0.0_f64, f64::max);
        let var_ratio = if total_var > 1e-12 { max_var / total_var } else { 0.0 };

        // Temporal: check autocorrelation of first column
        let auto_corr = if !cols.is_empty() && cols[0].len() > 2 {
            let c = &cols[0];
            let m = means[0];
            let v = vars[0].max(1e-12);
            let len = c.len() - 1;
            (0..len).map(|i| (c[i] - m) * (c[i + 1] - m)).sum::<f64>() / (len as f64 * v)
        } else { 0.0 };

        // KNN density spread — clustering signal
        let densities: Vec<f64> = (0..max_n).map(|i| shared.knn_density(i)).collect();
        let density_entropy = shannon_entropy(&densities, 16);

        // Spectral signal: variance concentration
        let spectral_score = var_ratio;

        // Score each analysis type
        let clustering_score = density_entropy;
        let temporal_score = auto_corr.abs();
        let topological_score = dist_entropy / (dist_entropy + 1.0);
        let statistical_score = total_var / (total_var + 1.0);

        let scores = [clustering_score, temporal_score, spectral_score, topological_score, statistical_score];
        let best_idx = scores.iter().enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i).unwrap_or(0);
        let best_val = scores[best_idx];
        let sum_scores: f64 = scores.iter().sum();
        let confidence = if sum_scores > 1e-12 { best_val / sum_scores } else { 0.0 };

        // Data complexity: entropy * dimension factor
        let complexity = dist_entropy * (1.0 + (d as f64).ln());

        // Unexplored: fraction not captured by top analysis
        let unexplored = 1.0 - confidence;

        // Novelty: how far distribution is from uniform
        let col_entropies: Vec<f64> = cols.iter().map(|c| shannon_entropy(c, 16)).collect();
        let mean_ent = if !col_entropies.is_empty() {
            col_entropies.iter().sum::<f64>() / col_entropies.len() as f64
        } else { 0.0 };
        let max_ent = (16.0_f64).ln();
        let novelty = 1.0 - (mean_ent / max_ent.max(1e-12)).min(1.0);

        result.insert("best_analysis_type".into(), vec![best_idx as f64]);
        result.insert("analysis_confidence".into(), vec![confidence]);
        result.insert("data_complexity".into(), vec![complexity]);
        result.insert("unexplored_potential".into(), vec![unexplored]);
        result.insert("novelty_score".into(), vec![novelty]);
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
        let r = LensDiscoveryLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("best_analysis_type"));
        assert!(r.contains_key("novelty_score"));
    }
}
