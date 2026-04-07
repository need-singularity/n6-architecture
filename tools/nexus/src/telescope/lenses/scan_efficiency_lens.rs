use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ScanEfficiencyLens: Measures data-lens fitness and scan cost optimization.
///
/// Determines how efficiently the current data can be analyzed:
///   1. Signal-to-noise ratio: useful pattern signal vs noise floor
///   2. Redundancy: how much data is redundant (compressible)
///   3. Lens suitability: data characteristics vs lens requirements
///   4. Diminishing returns: would more data help or is it saturated?
pub struct ScanEfficiencyLens;

impl Lens for ScanEfficiencyLens {
    fn name(&self) -> &str {
        "ScanEfficiencyLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. SNR: signal (structured variation) vs noise (random residual)
        // Signal proxy: inter-cluster variance / total variance
        let mut means = vec![0.0_f64; dims];
        for r in 0..n {
            for di in 0..dims {
                means[di] += data[r * d + di];
            }
        }
        for m in &mut means { *m /= n as f64; }

        // k-means-like split into n/φ=3 groups by distance from centroid
        let mut centroid_dists: Vec<(usize, f64)> = (0..n)
            .map(|r| {
                let d2: f64 = (0..dims)
                    .map(|di| (data[r * d + di] - means[di]).powi(2))
                    .sum();
                (r, d2.sqrt())
            })
            .collect();
        centroid_dists.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let third = n / 3;
        let groups = [
            &centroid_dists[..third],
            &centroid_dists[third..2 * third],
            &centroid_dists[2 * third..],
        ];

        let mut inter_var = 0.0_f64;
        let global_mean_dist = centroid_dists.iter().map(|x| x.1).sum::<f64>() / n as f64;
        for group in &groups {
            let group_mean = group.iter().map(|x| x.1).sum::<f64>() / group.len().max(1) as f64;
            inter_var += (group_mean - global_mean_dist).powi(2);
        }
        inter_var /= 3.0;

        let total_var = centroid_dists
            .iter()
            .map(|x| (x.1 - global_mean_dist).powi(2))
            .sum::<f64>()
            / n as f64;
        let snr = if total_var > 1e-12 {
            inter_var / total_var
        } else {
            0.0
        };

        // 2. Redundancy: duplicate/near-duplicate detection
        let nn_threshold = {
            let mut sample_dists: Vec<f64> = Vec::new();
            let check = n.min(80);
            for i in 0..check {
                for j in (i + 1)..check {
                    sample_dists.push(shared.dist(i, j));
                }
            }
            sample_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            if sample_dists.is_empty() { 1.0 } else { sample_dists[sample_dists.len() / 20].max(1e-12) }
        };
        let mut near_dupes = 0u32;
        let check = n.min(100);
        for i in 0..check {
            for j in (i + 1)..check {
                if shared.dist(i, j) < nn_threshold {
                    near_dupes += 1;
                }
            }
        }
        let max_pairs = (check * (check - 1) / 2).max(1);
        let redundancy = near_dupes as f64 / max_pairs as f64;

        // 3. Saturation: compare statistics of first 2/3 vs all data
        let cutoff = n * 2 / 3;
        let mut partial_means = vec![0.0_f64; dims];
        for r in 0..cutoff {
            for di in 0..dims {
                partial_means[di] += data[r * d + di];
            }
        }
        for m in &mut partial_means { *m /= cutoff as f64; }

        let mut mean_drift = 0.0_f64;
        for di in 0..dims {
            mean_drift += ((means[di] - partial_means[di]) / means[di].abs().max(1e-12)).abs();
        }
        mean_drift /= dims as f64;
        // Low drift = saturated (more data won't help much)
        let saturation = 1.0 - mean_drift.min(1.0);

        // 4. Information density: bits per sample
        let info_density = snr * (1.0 - redundancy);

        // Efficiency score
        let efficiency = info_density * 0.4 + snr * 0.3 + (1.0 - redundancy) * 0.3;

        let mut result = HashMap::new();
        result.insert("scan_efficiency".into(), vec![efficiency]);
        result.insert("snr".into(), vec![snr]);
        result.insert("redundancy".into(), vec![redundancy]);
        result.insert("saturation".into(), vec![saturation]);
        result.insert("info_density".into(), vec![info_density]);
        result.insert("near_duplicates_ratio".into(), vec![near_dupes as f64 / max_pairs as f64]);
        result
    }
}
