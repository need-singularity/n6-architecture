use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// MaterialCombinationLens: Multi-feature composites like alloys.
/// Measures alloy_score, composite_strength, phase_count, defect_density.
pub struct MaterialCombinationLens;

impl Lens for MaterialCombinationLens {
    fn name(&self) -> &str { "MaterialCombinationLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let cols = column_vectors(data, max_n, d);
        let (means, vars) = mean_var(data, max_n, d);
        let max_d = d.min(24);

        // Alloy score: how uniformly mixed the features are (low CV of means)
        let active_means: Vec<f64> = (0..max_d).map(|i| means[i].abs()).collect();
        let m_mean = active_means.iter().sum::<f64>() / max_d.max(1) as f64;
        let m_var = active_means.iter().map(|v| (v - m_mean).powi(2)).sum::<f64>() / max_d.max(1) as f64;
        let alloy_score = if m_mean > 1e-12 { 1.0 - (m_var.sqrt() / m_mean).min(1.0) } else { 0.0 };

        // Composite strength: product of normalized variances (emergent from combination)
        let total_var: f64 = vars[..max_d].iter().sum();
        let composite_strength = if max_d > 0 && total_var > 1e-12 {
            let normalized: Vec<f64> = vars[..max_d].iter().map(|v| v / total_var).collect();
            // Shannon entropy of variance distribution (higher = more uniform = stronger composite)
            -normalized.iter().filter(|&&v| v > 1e-15).map(|v| v * v.ln()).sum::<f64>() / (max_d as f64).ln().max(1e-12)
        } else { 0.0 };

        // Phase count: cluster distinct variance levels
        let mut sorted_vars: Vec<f64> = vars[..max_d].to_vec();
        sorted_vars.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let mut phases = 1usize;
        for i in 1..sorted_vars.len() {
            let prev = sorted_vars[i - 1].max(1e-15);
            if (sorted_vars[i] - prev) / prev > 0.5 { phases += 1; }
        }

        // Defect density: fraction of outlier rows (distance > 3σ from centroid)
        let mut defects = 0usize;
        for row in 0..max_n {
            let mut dist_sq = 0.0;
            for j in 0..max_d {
                let diff = cols[j][row] - means[j];
                dist_sq += diff * diff;
            }
            let threshold = 9.0 * total_var; // 3σ squared
            if dist_sq > threshold { defects += 1; }
        }
        let defect_density = defects as f64 / max_n as f64;

        let mut result = HashMap::new();
        result.insert("alloy_score".into(), vec![alloy_score]);
        result.insert("composite_strength".into(), vec![composite_strength]);
        result.insert("phase_count".into(), vec![phases as f64]);
        result.insert("defect_density".into(), vec![defect_density]);
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
        let result = MaterialCombinationLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
