use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// HotPathLens: Static hot path prediction without profiling.
///
/// Uses entropy and information-theoretic measures to predict which
/// data regions are "hot" (frequently accessed / high impact) without
/// runtime profiling. This finds optimization targets that PGO misses
/// because it works on the data structure itself rather than execution
/// traces.
///
/// Metrics:
///   1. hot_region_fraction: fraction of data in hot paths
///   2. entropy_gradient: rate of entropy change across data
///   3. information_density: bits per data point in hot regions
///   4. cold_region_fraction: fraction of rarely-touched data
///   5. hotspot_indices: indices of detected hot points
///   6. heat_concentration: Gini coefficient of "heat" distribution
pub struct HotPathLens;

impl Lens for HotPathLens {
    fn name(&self) -> &str { "HotPathLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // Compute per-point "heat" based on multiple signals:
        // 1. KNN density (high density = hot)
        // 2. Local variance (high variance = interesting/hot)
        // 3. Centrality (close to center of mass = hot)

        let mut heat = vec![0.0f64; max_n];

        // Global center of mass
        let mut center = vec![0.0f64; d];
        for i in 0..max_n {
            for dim in 0..d {
                center[dim] += data[i * d + dim];
            }
        }
        for c in &mut center { *c /= max_n as f64; }

        // Per-point signals
        let mut densities = Vec::with_capacity(max_n);
        let mut local_vars = Vec::with_capacity(max_n);
        let mut centralities = Vec::with_capacity(max_n);

        for i in 0..max_n {
            // Density from KNN
            let density = shared.knn_density(i);
            densities.push(density);

            // Local variance: variance among neighbors
            let knn = shared.knn(i);
            let mut local_var = 0.0f64;
            if knn.len() >= 2 {
                for dim in 0..d.min(8) {
                    let vals: Vec<f64> = knn.iter()
                        .filter_map(|&j| {
                            let j = j as usize;
                            if j < n { Some(data[j * d + dim]) } else { None }
                        })
                        .collect();
                    if vals.is_empty() { continue; }
                    let mean = vals.iter().sum::<f64>() / vals.len() as f64;
                    local_var += vals.iter().map(|&v| (v - mean) * (v - mean)).sum::<f64>()
                        / vals.len() as f64;
                }
            }
            local_vars.push(local_var);

            // Centrality: inverse distance to center
            let dist_to_center: f64 = (0..d)
                .map(|dim| {
                    let diff = data[i * d + dim] - center[dim];
                    diff * diff
                })
                .sum::<f64>()
                .sqrt();
            centralities.push(if dist_to_center > 1e-12 { 1.0 / dist_to_center } else { 1.0 });
        }

        // Normalize each signal to [0,1] and combine
        fn normalize(vals: &[f64]) -> Vec<f64> {
            if vals.is_empty() { return vec![]; }
            let min = vals.iter().cloned().fold(f64::INFINITY, f64::min);
            let max = vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let range = max - min;
            if range < 1e-15 { return vec![0.5; vals.len()]; }
            vals.iter().map(|&v| (v - min) / range).collect()
        }

        let norm_dens = normalize(&densities);
        let norm_var = normalize(&local_vars);
        let norm_cent = normalize(&centralities);

        for i in 0..max_n {
            heat[i] = 0.4 * norm_dens[i] + 0.35 * norm_var[i] + 0.25 * norm_cent[i];
        }

        // Classify hot vs cold
        let mean_heat = heat.iter().sum::<f64>() / max_n as f64;
        let hot_threshold = mean_heat * 1.5;
        let cold_threshold = mean_heat * 0.5;

        let hot_count = heat.iter().filter(|&&h| h > hot_threshold).count();
        let cold_count = heat.iter().filter(|&&h| h < cold_threshold).count();
        let hot_region_fraction = hot_count as f64 / max_n as f64;
        let cold_region_fraction = cold_count as f64 / max_n as f64;

        // Hotspot indices (top 10%)
        let mut indexed: Vec<(usize, f64)> = heat.iter().copied().enumerate().collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        let top_k = (max_n / 10).max(1).min(20);
        let hotspot_indices: Vec<f64> = indexed[..top_k].iter().map(|(i, _)| *i as f64).collect();

        // Entropy gradient: how quickly entropy changes across data windows
        let window = (max_n / 4).max(4);
        let mut window_entropies = Vec::new();
        for start in (0..max_n).step_by(window.max(1)) {
            let end = (start + window).min(max_n);
            if end - start < 3 { continue; }
            let segment: Vec<f64> = (start..end).flat_map(|i| {
                (0..d.min(4)).map(move |dim| data[i * d + dim])
            }).collect();
            // Compute entropy of segment
            let n_bins = 16usize;
            let min_v = segment.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_v = segment.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let range = max_v - min_v + 1e-12;
            let mut counts = vec![0u32; n_bins];
            for &v in &segment {
                let bin = ((v - min_v) / range * (n_bins - 1) as f64) as usize;
                counts[bin.min(n_bins - 1)] += 1;
            }
            let inv = 1.0 / segment.len() as f64;
            let ent: f64 = counts.iter()
                .filter(|&&c| c > 0)
                .map(|&c| { let p = c as f64 * inv; -p * p.ln() })
                .sum();
            window_entropies.push(ent);
        }
        let entropy_gradient = if window_entropies.len() >= 2 {
            let deltas: Vec<f64> = window_entropies.windows(2).map(|w| (w[1] - w[0]).abs()).collect();
            deltas.iter().sum::<f64>() / deltas.len() as f64
        } else { 0.0 };

        // Information density in hot regions
        let hot_info: f64 = heat.iter().filter(|&&h| h > hot_threshold).sum::<f64>();
        let info_density = if hot_count > 0 { hot_info / hot_count as f64 } else { 0.0 };

        // Gini coefficient of heat distribution
        let mut sorted_heat = heat.clone();
        sorted_heat.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let total = sorted_heat.iter().sum::<f64>();
        let gini = if total > 1e-12 {
            let n_f = max_n as f64;
            let mut sum = 0.0f64;
            for (i, &h) in sorted_heat.iter().enumerate() {
                sum += (2.0 * (i + 1) as f64 - n_f - 1.0) * h;
            }
            sum / (n_f * total)
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("hot_region_fraction".to_string(), vec![hot_region_fraction]);
        result.insert("entropy_gradient".to_string(), vec![entropy_gradient]);
        result.insert("information_density".to_string(), vec![info_density]);
        result.insert("cold_region_fraction".to_string(), vec![cold_region_fraction]);
        result.insert("hotspot_indices".to_string(), hotspot_indices);
        result.insert("heat_concentration".to_string(), vec![gini]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_hot_path_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = HotPathLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("hot_region_fraction"));
        assert!(r.contains_key("heat_concentration"));
    }

    #[test]
    fn test_hot_path_clustered() {
        // Clustered data should have clear hot/cold regions
        let mut data = Vec::new();
        // Dense cluster
        for i in 0..15 { data.push(i as f64 * 0.01); data.push(i as f64 * 0.01); }
        // Sparse outliers
        for i in 0..5 { data.push(10.0 + i as f64 * 5.0); data.push(10.0 + i as f64 * 5.0); }
        let shared = SharedData::compute(&data, 20, 2);
        let r = HotPathLens.scan(&data, 20, 2, &shared);
        assert!(r["heat_concentration"][0] > 0.0);
    }
}
