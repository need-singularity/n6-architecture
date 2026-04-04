use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// WarpLens: Alcubierre-inspired metric distortion detection.
///
/// Detects local warping of the data manifold — regions where local
/// distances are compressed or stretched relative to global structure.
/// Analogous to spacetime curvature in Alcubierre metric.
///
/// Metrics:
///   1. warp_factor: local-to-global distance ratio per point (>1 = expansion, <1 = contraction)
///   2. mean_warp: average warping across all points
///   3. warp_anisotropy: variance of warp factors (uniform=0, highly warped=high)
///   4. contraction_fraction: fraction of points in contracted regions
///
/// n=6: Alcubierre metric requires exotic matter; energy density ~ σ(6)=12 dimensions
///       of compactified space in string theory. Warp bubble thickness ~ 1/σ.
pub struct WarpLens;

impl Lens for WarpLens {
    fn name(&self) -> &str { "WarpLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }

        // Global median distance
        let mut global_dists = Vec::new();
        let step = if n > 100 { n / 100 } else { 1 };
        let mut i = 0;
        while i < n {
            let mut j = i + 1;
            while j < n {
                global_dists.push(shared.dist(i, j));
                j += step;
            }
            i += step;
        }
        if global_dists.is_empty() { return HashMap::new(); }
        global_dists.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap());
        let global_median = global_dists[global_dists.len() / 2];
        if global_median < 1e-15 { return HashMap::new(); }

        // Per-point warp factor: mean local distance / global median
        let mut warp_factors = Vec::with_capacity(n);
        for i in 0..n {
            let knn = shared.knn(i);
            if knn.is_empty() { continue; }
            let local_mean: f64 = knn.iter()
                .map(|&j| shared.dist(i, j as usize))
                .sum::<f64>() / knn.len() as f64;
            warp_factors.push(local_mean / global_median);
        }

        if warp_factors.is_empty() { return HashMap::new(); }

        let nf = warp_factors.len() as f64;
        let mean_warp = warp_factors.iter().sum::<f64>() / nf;
        let warp_var = warp_factors.iter()
            .map(|w| (w - mean_warp) * (w - mean_warp))
            .sum::<f64>() / nf;
        let contraction_frac = warp_factors.iter().filter(|&&w| w < 1.0).count() as f64 / nf;

        let mut result = HashMap::new();
        result.insert("warp_factor".to_string(), warp_factors);
        result.insert("mean_warp".to_string(), vec![mean_warp]);
        result.insert("warp_anisotropy".to_string(), vec![warp_var]);
        result.insert("contraction_fraction".to_string(), vec![contraction_frac]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_warp_lens_basic() {
        // Two dense clusters with sparse gap → high warp anisotropy
        let mut data = Vec::new();
        for i in 0..10 { data.push(i as f64 * 0.01); data.push(0.0); }
        for i in 0..10 { data.push(100.0 + i as f64 * 0.01); data.push(0.0); }
        let n = 20;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = WarpLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_warp"));
        assert!(result.contains_key("warp_anisotropy"));
    }
}
