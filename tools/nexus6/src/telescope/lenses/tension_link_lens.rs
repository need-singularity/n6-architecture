use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TensionLinkLens: Tension connections between distant points.
///
/// Measures stressed connections in the kNN graph: link count, mean tension,
/// breaking threshold, elastic energy stored in stretched links, and
/// overall network stress.
pub struct TensionLinkLens;

impl Lens for TensionLinkLens {
    fn name(&self) -> &str { "TensionLinkLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Compute mean and std of kNN edge distances
        let mut edge_dists: Vec<f64> = Vec::with_capacity(max_n * shared.knn_k);
        for i in 0..max_n {
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let d = shared.dist(i, j as usize);
                edge_dists.push(d);
            }
        }

        if edge_dists.is_empty() { return HashMap::new(); }

        let mean_edge = edge_dists.iter().sum::<f64>() / edge_dists.len() as f64;
        let var_edge = edge_dists.iter()
            .map(|&d| (d - mean_edge).powi(2)).sum::<f64>() / edge_dists.len() as f64;
        let std_edge = var_edge.sqrt();

        // Tension threshold: edges longer than mean + 1 std are under tension
        let tension_threshold = mean_edge + std_edge;

        // Identify stressed links
        let mut link_count = 0u32;
        let mut tension_sum = 0.0_f64;
        let mut max_tension = 0.0_f64;
        let mut elastic_energy = 0.0_f64;

        for i in 0..max_n {
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let d = shared.dist(i, j as usize);
                if d > tension_threshold {
                    link_count += 1;
                    let stretch = d - mean_edge;
                    tension_sum += stretch;
                    if stretch > max_tension { max_tension = stretch; }
                    // Elastic energy: 0.5 * k * x^2, with k=1
                    elastic_energy += 0.5 * stretch * stretch;
                }
            }
        }

        let link_tension = if link_count > 0 {
            tension_sum / link_count as f64
        } else { 0.0 };

        // Breaking threshold: the tension at which links would snap
        // Estimated as mean + 3*std (3-sigma rule)
        let breaking_threshold = mean_edge + 3.0 * std_edge;

        // Network stress: fraction of total edges that are stressed
        let total_edges = edge_dists.len().max(1) as f64;
        let network_stress = link_count as f64 / total_edges;

        let mut result = HashMap::new();
        result.insert("link_count".to_string(), vec![link_count as f64]);
        result.insert("link_tension".to_string(), vec![link_tension]);
        result.insert("breaking_threshold".to_string(), vec![breaking_threshold]);
        result.insert("elastic_energy".to_string(), vec![elastic_energy]);
        result.insert("network_stress".to_string(), vec![network_stress]);
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
        let result = TensionLinkLens.scan(&data, 20, 2, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("link_count"));
        assert!(result.contains_key("network_stress"));
        assert!(result.contains_key("elastic_energy"));
    }

    #[test]
    fn test_stress_range() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.1)).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let result = TensionLinkLens.scan(&data, 20, 2, &shared);
        let stress = result["network_stress"][0];
        assert!(stress >= 0.0 && stress <= 1.0, "stress should be in [0,1]: {stress}");
    }
}
