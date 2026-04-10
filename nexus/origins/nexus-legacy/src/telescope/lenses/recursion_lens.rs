use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// RecursionLens: self-similarity detection via sub-sample comparison.
///
/// Algorithm:
///   1. Compute distance distribution for full dataset
///   2. Compute distance distribution for random sub-samples (1/2, 1/3, 1/4)
///   3. Self-similarity = KL-divergence between sub-sample and full distributions
///   4. Low divergence = fractal/self-similar structure
pub struct RecursionLens;

impl Lens for RecursionLens {
    fn name(&self) -> &str {
        "RecursionLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 8 {
            return HashMap::new();
        }

        let num_bins = ((n as f64).sqrt().ceil() as usize).max(5).min(30);

        // Full dataset distance distribution
        let full_hist = distance_histogram(shared, &(0..n).collect::<Vec<_>>(), num_bins);

        // Sub-samples at different scales
        let scales = [2usize, 3, 4];
        let mut kl_divergences: Vec<f64> = Vec::new();

        for &scale in &scales {
            let sub_indices: Vec<usize> = (0..n).step_by(scale).collect();
            if sub_indices.len() < 4 {
                continue;
            }

            let sub_hist = distance_histogram(shared, &sub_indices, num_bins);
            let kl = kl_divergence(&full_hist, &sub_hist);
            kl_divergences.push(kl);
        }

        if kl_divergences.is_empty() {
            return HashMap::new();
        }

        let mean_kl = kl_divergences.iter().sum::<f64>() / kl_divergences.len() as f64;
        // Self-similarity score: inverse of KL (high score = self-similar)
        let self_similarity = 1.0 / (1.0 + mean_kl);

        let mut result = HashMap::new();
        result.insert("self_similarity_score".to_string(), vec![self_similarity]);
        result.insert("mean_kl_divergence".to_string(), vec![mean_kl]);
        result
    }
}

fn distance_histogram(shared: &SharedData, indices: &[usize], bins: usize) -> Vec<f64> {
    let m = indices.len();
    if m < 2 {
        return vec![0.0; bins];
    }

    let mut dists: Vec<f64> = Vec::new();
    for i in 0..m {
        for j in (i + 1)..m {
            dists.push(shared.dist(indices[i], indices[j]));
        }
    }

    if dists.is_empty() {
        return vec![0.0; bins];
    }

    let min_d = dists.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_d = dists.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_d - min_d;

    let mut hist = vec![0.0; bins];
    if range < 1e-15 {
        hist[0] = 1.0;
        return hist;
    }

    for &d in &dists {
        let idx = ((d - min_d) / range * (bins - 1) as f64).round() as usize;
        hist[idx.min(bins - 1)] += 1.0;
    }

    // Normalize to probability
    let total: f64 = hist.iter().sum();
    if total > 0.0 {
        for h in hist.iter_mut() {
            *h /= total;
        }
    }

    hist
}

fn kl_divergence(p: &[f64], q: &[f64]) -> f64 {
    let eps = 1e-10;
    p.iter()
        .zip(q.iter())
        .map(|(&pi, &qi)| {
            let pi = pi.max(eps);
            let qi = qi.max(eps);
            pi * (pi / qi).ln()
        })
        .sum::<f64>()
        .max(0.0)
}
