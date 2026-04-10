use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors, mean_var};

/// MetricDiscoveryLens: Finds the most informative distance metric for the data.
/// Measures best_metric_type, metric_advantage, anisotropy_direction, effective_dimensionality.
pub struct MetricDiscoveryLens;

impl Lens for MetricDiscoveryLens {
    fn name(&self) -> &str { "MetricDiscoveryLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }
        let max_n = n.min(200);
        let cols = column_vectors(data, max_n, d);
        let (_means, vars) = mean_var(data, max_n, d);
        let max_d = d.min(20);
        let sample = max_n.min(50);

        // Compute pairwise distances under different metrics
        let mut eucl_dists = Vec::new();
        let mut manh_dists = Vec::new();
        let mut cos_dists = Vec::new();

        for i in 0..sample {
            for j in (i + 1)..sample {
                let mut e2 = 0.0_f64;
                let mut m1 = 0.0_f64;
                let mut dot = 0.0_f64;
                let mut ni = 0.0_f64;
                let mut nj = 0.0_f64;
                for k in 0..max_d {
                    let a = cols[k][i];
                    let b = cols[k][j];
                    e2 += (a - b).powi(2);
                    m1 += (a - b).abs();
                    dot += a * b;
                    ni += a * a;
                    nj += b * b;
                }
                eucl_dists.push(e2.sqrt());
                manh_dists.push(m1);
                let cos_sim = if ni > 1e-15 && nj > 1e-15 { dot / (ni.sqrt() * nj.sqrt()) } else { 0.0 };
                cos_dists.push(1.0 - cos_sim);
            }
        }

        // Metric quality = variance of distances (higher spread = more informative)
        fn dist_quality(dists: &[f64]) -> f64 {
            if dists.is_empty() { return 0.0; }
            let mean = dists.iter().sum::<f64>() / dists.len() as f64;
            if mean.abs() < 1e-15 { return 0.0; }
            let var = dists.iter().map(|d| (d - mean).powi(2)).sum::<f64>() / dists.len() as f64;
            var.sqrt() / mean // coefficient of variation
        }

        let q_eucl = dist_quality(&eucl_dists);
        let q_manh = dist_quality(&manh_dists);
        let q_cos = dist_quality(&cos_dists);

        // 0=euclidean, 1=manhattan, 2=cosine
        let (best_type, best_q) = if q_manh > q_eucl && q_manh > q_cos { (1.0, q_manh) }
            else if q_cos > q_eucl { (2.0, q_cos) }
            else { (0.0, q_eucl) };
        let metric_advantage = if q_eucl > 1e-15 { best_q / q_eucl } else { 1.0 };

        // Anisotropy: direction of maximum variance
        let max_var_idx = vars[..max_d].iter().enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i).unwrap_or(0);

        // Effective dimensionality via variance participation ratio
        let total_var: f64 = vars[..max_d].iter().sum();
        let sum_sq: f64 = vars[..max_d].iter().map(|v| v * v).sum();
        let eff_dim = if sum_sq > 1e-15 { (total_var * total_var) / sum_sq } else { 1.0 };

        let mut result = HashMap::new();
        result.insert("best_metric_type".into(), vec![best_type]);
        result.insert("metric_advantage".into(), vec![metric_advantage]);
        result.insert("anisotropy_direction".into(), vec![max_var_idx as f64]);
        result.insert("effective_dimensionality".into(), vec![eff_dim]);
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
        let result = MetricDiscoveryLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
    }
}
