use std::collections::HashMap;
use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, shannon_entropy};

/// Discovers reusable structural modules in data.
pub struct ModuleDiscoveryLens;

impl Lens for ModuleDiscoveryLens {
    fn name(&self) -> &str { "ModuleDiscoveryLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Build distance-threshold adjacency for module detection
        let mut dists = Vec::with_capacity(max_n * (max_n - 1) / 2);
        for i in 0..max_n {
            for j in (i + 1)..max_n {
                dists.push(shared.dist(i, j));
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let threshold = dists.get(dists.len() / 3).copied().unwrap_or(1.0);

        // Greedy clustering: assign points to modules by proximity
        let mut labels = vec![usize::MAX; max_n];
        let mut module_id = 0_usize;
        for i in 0..max_n {
            if labels[i] != usize::MAX { continue; }
            labels[i] = module_id;
            for j in (i + 1)..max_n {
                if labels[j] == usize::MAX && shared.dist(i, j) < threshold {
                    labels[j] = module_id;
                }
            }
            module_id += 1;
        }
        let module_count = module_id;

        // Module sizes
        let mut sizes = vec![0usize; module_count];
        for &l in &labels {
            if l < module_count { sizes[l] += 1; }
        }
        let non_empty: Vec<usize> = sizes.iter().copied().filter(|&s| s > 0).collect();

        // Coverage: fraction of points in modules with size > 1
        let covered = non_empty.iter().filter(|&&s| s > 1).sum::<usize>();
        let coverage = covered as f64 / max_n as f64;

        // Diversity: entropy of module size distribution
        let size_floats: Vec<f64> = non_empty.iter().map(|&s| s as f64).collect();
        let diversity = shannon_entropy(&size_floats, non_empty.len().max(2).min(32));

        // Composability: how often KNN neighbors share module labels
        let mut shared_knn = 0usize;
        let mut total_knn = 0usize;
        for i in 0..max_n {
            let knn = shared.knn(i);
            for &k in knn {
                let k = k as usize;
                if k < max_n {
                    total_knn += 1;
                    if labels[i] == labels[k] { shared_knn += 1; }
                }
            }
        }
        let composability = if total_knn > 0 {
            shared_knn as f64 / total_knn as f64
        } else { 0.0 };

        // Abstraction level: 0=raw (many singletons), 1=pattern, 2=structure, 3=meta
        let singleton_frac = non_empty.iter().filter(|&&s| s == 1).count() as f64
            / non_empty.len().max(1) as f64;
        let abstraction = if singleton_frac > 0.8 { 0.0 }
            else if singleton_frac > 0.5 { 1.0 }
            else if module_count < max_n / 4 { 2.0 }
            else { 3.0 };

        result.insert("module_count".into(), vec![module_count as f64]);
        result.insert("module_coverage".into(), vec![coverage]);
        result.insert("module_diversity".into(), vec![diversity]);
        result.insert("composability_score".into(), vec![composability]);
        result.insert("abstraction_level".into(), vec![abstraction]);
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
        let r = ModuleDiscoveryLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("module_count"));
        assert!(r.contains_key("composability_score"));
    }
}
