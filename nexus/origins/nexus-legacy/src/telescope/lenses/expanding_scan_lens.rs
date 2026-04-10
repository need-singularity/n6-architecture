use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// ExpandingScanLens: Multi-scale expanding scan.
/// Starts from local (small k-NN) and expands outward.
/// Measures density/entropy/structure at each scale, finds transitions and critical scale.
pub struct ExpandingScanLens;

impl Lens for ExpandingScanLens {
    fn name(&self) -> &str { "ExpandingScanLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Define scales: k = 1, 2, 4, 8, ... up to max_n/2
        let mut scales = Vec::new();
        let mut k = 1usize;
        while k <= max_n / 2 {
            scales.push(k);
            k *= 2;
        }
        if scales.is_empty() { scales.push(1); }

        let mut density_per_scale = Vec::new();
        let mut entropy_per_scale = Vec::new();
        let mut structure_per_scale = Vec::new();

        for &scale in &scales {
            // Density at scale: average local density using k nearest neighbors
            let mut densities = Vec::with_capacity(max_n);
            for i in 0..max_n {
                let knn = shared.knn(i);
                let k_eff = scale.min(knn.len());
                if k_eff == 0 { densities.push(0.0); continue; }
                let last_j = knn[k_eff - 1] as usize;
                let dist = if last_j < max_n && last_j != i { shared.dist(i, last_j) } else { 1.0 };
                densities.push(if dist > 1e-12 { 1.0 / dist } else { 1e6 });
            }
            let avg_density = densities.iter().sum::<f64>() / max_n as f64;
            density_per_scale.push(avg_density);

            // Entropy at scale: entropy of density distribution
            let ent = shannon_entropy(&densities, 10);
            entropy_per_scale.push(ent);

            // Structure at scale: variance of densities (high = structured, low = uniform)
            let var = densities.iter().map(|&dd| (dd - avg_density).powi(2)).sum::<f64>() / max_n as f64;
            structure_per_scale.push(var.sqrt());
        }

        // Scale transition: find largest change in entropy between consecutive scales
        let mut max_transition = 0.0f64;
        let mut transition_scale = 0usize;
        for i in 1..entropy_per_scale.len() {
            let change = (entropy_per_scale[i] - entropy_per_scale[i - 1]).abs();
            if change > max_transition {
                max_transition = change;
                transition_scale = i;
            }
        }
        let scale_transition = if transition_scale < scales.len() { scales[transition_scale] as f64 } else { 1.0 };

        // Expansion rate: average change in density across scales
        let expansion_rate = if density_per_scale.len() >= 2 {
            let first = density_per_scale[0].max(1e-12);
            let last = *density_per_scale.last().unwrap();
            (first - last).abs() / first
        } else {
            0.0
        };

        // Critical scale: most informative = highest entropy
        let (critical_idx, _) = entropy_per_scale.iter().enumerate()
            .fold((0, 0.0f64), |(bi, bv), (i, &v)| if v > bv { (i, v) } else { (bi, bv) });
        let critical_scale = scales[critical_idx] as f64;

        let mut result = HashMap::new();
        result.insert("scale_transition".to_string(), vec![scale_transition]);
        result.insert("expansion_rate".to_string(), vec![expansion_rate]);
        result.insert("critical_scale".to_string(), vec![critical_scale]);
        result.insert("density_at_scales".to_string(), density_per_scale);
        result.insert("entropy_at_scales".to_string(), entropy_per_scale);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..30).map(|i| (i as f64 * 0.2).sin()).collect();
        let shared = SharedData::compute(&data, 30, 1);
        let result = ExpandingScanLens.scan(&data, 30, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["critical_scale"][0] >= 1.0);
        assert!(!result["density_at_scales"].is_empty());
    }
}
