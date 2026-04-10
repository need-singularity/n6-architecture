use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, column_vectors};

/// DivergenceLens: KL and Jensen-Shannon divergence between feature distributions.
///
/// Detects distributional asymmetries and information-theoretic distances
/// between dimensions, useful for identifying n=6 structural breaks.
pub struct DivergenceLens;

impl Lens for DivergenceLens {
    fn name(&self) -> &str { "DivergenceLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d < 2 { return HashMap::new(); }

        let columns = column_vectors(data, n, d);
        let n_bins = (n as f64).sqrt().max(4.0) as usize;

        let mut js_divergences = Vec::new();
        let pairs = d.min(10);
        for i in 0..pairs {
            for j in (i + 1)..pairs.min(d) {
                let js = jensen_shannon(&columns[i], &columns[j], n_bins);
                js_divergences.push(js);
            }
        }

        let mean_js = if !js_divergences.is_empty() {
            js_divergences.iter().sum::<f64>() / js_divergences.len() as f64
        } else { 0.0 };

        let max_js = js_divergences.iter().cloned().fold(0.0_f64, f64::max);

        let mut result = HashMap::new();
        result.insert("js_divergences".to_string(), js_divergences);
        result.insert("mean_js".to_string(), vec![mean_js]);
        result.insert("max_js".to_string(), vec![max_js]);
        result
    }
}

fn histogram(values: &[f64], n_bins: usize) -> Vec<f64> {
    if values.is_empty() || n_bins < 2 { return vec![1.0 / n_bins as f64; n_bins]; }
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in values { if v < lo { lo = v; } if v > hi { hi = v; } }
    let range = (hi - lo).max(1e-12);
    let scale = (n_bins - 1) as f64 / range;
    let mut counts = vec![0u32; n_bins];
    for &v in values {
        let bin = ((v - lo) * scale) as usize;
        counts[bin.min(n_bins - 1)] += 1;
    }
    let total = values.len() as f64;
    counts.iter().map(|&c| (c as f64 + 1e-10) / (total + n_bins as f64 * 1e-10)).collect()
}

fn jensen_shannon(a: &[f64], b: &[f64], n_bins: usize) -> f64 {
    let pa = histogram(a, n_bins);
    let pb = histogram(b, n_bins);
    let mut js = 0.0;
    for i in 0..pa.len() {
        let m = (pa[i] + pb[i]) / 2.0;
        if pa[i] > 0.0 && m > 0.0 { js += pa[i] * (pa[i] / m).ln(); }
        if pb[i] > 0.0 && m > 0.0 { js += pb[i] * (pb[i] / m).ln(); }
    }
    (js / 2.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_divergence_lens_basic() {
        let data: Vec<f64> = (0..40).map(|i| (i as f64 * 0.5).sin()).collect();
        let n = 10;
        let d = 4;
        let shared = SharedData::compute(&data, n, d);
        let result = DivergenceLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_js"));
        assert!(result["mean_js"][0] >= 0.0);
    }
}
