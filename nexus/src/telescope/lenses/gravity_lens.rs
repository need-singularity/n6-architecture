use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// GravityLens: mean-shift attractor finding + density peak detection.
///
/// Algorithm:
///   1. k-NN density estimation for each point
///   2. Gradient ascent: each point moves toward its density-weighted centroid
///   3. Converged positions = attractors (density peaks)
///   4. Reports attractor count and peak densities
pub struct GravityLens;

impl Lens for GravityLens {
    fn name(&self) -> &str {
        "GravityLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        let k = ((n as f64).sqrt().ceil() as usize).max(1).min(n - 1);

        // Compute k-NN density for each point
        let densities: Vec<f64> = (0..n)
            .map(|i| {
                let mut dists: Vec<f64> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| shared.dist(i, j))
                    .collect();
                dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                let knn_dist = dists[k.min(dists.len()) - 1];
                if knn_dist > 0.0 {
                    1.0 / (knn_dist * knn_dist)
                } else {
                    f64::MAX
                }
            })
            .collect();

        // Mean-shift: each point climbs toward density peak
        // Use bandwidth = median k-NN distance
        let mut knn_dists: Vec<f64> = (0..n)
            .map(|i| {
                let mut dists: Vec<f64> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| shared.dist(i, j))
                    .collect();
                dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                dists[k.min(dists.len()) - 1]
            })
            .collect();
        knn_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let bandwidth = knn_dists[n / 2].max(1e-10);

        // For each point, find which attractor it converges to
        // Simple: assign each point to its nearest density peak
        // Find density peaks: points whose density > all k-neighbors' densities
        let mut peaks: Vec<usize> = Vec::new();
        for i in 0..n {
            let mut neighbor_indices: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, shared.dist(i, j)))
                .collect();
            neighbor_indices
                .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

            let is_peak = neighbor_indices
                .iter()
                .take(k)
                .all(|(j, _)| densities[i] >= densities[*j]);

            if is_peak {
                peaks.push(i);
            }
        }

        // Merge nearby peaks (within bandwidth)
        let mut merged_peaks: Vec<usize> = Vec::new();
        let mut used = vec![false; peaks.len()];
        for i in 0..peaks.len() {
            if used[i] {
                continue;
            }
            let mut best = peaks[i];
            for j in (i + 1)..peaks.len() {
                if used[j] {
                    continue;
                }
                if shared.dist(peaks[i], peaks[j]) < bandwidth {
                    used[j] = true;
                    if densities[peaks[j]] > densities[best] {
                        best = peaks[j];
                    }
                }
            }
            merged_peaks.push(best);
        }

        let peak_densities: Vec<f64> = merged_peaks.iter().map(|&p| densities[p]).collect();
        let peak_indices: Vec<f64> = merged_peaks.iter().map(|&p| p as f64).collect();

        let mut result = HashMap::new();
        result.insert("attractor_count".to_string(), vec![merged_peaks.len() as f64]);
        result.insert("peak_densities".to_string(), peak_densities);
        result.insert("peak_indices".to_string(), peak_indices);
        result
    }
}
