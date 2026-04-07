use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BigBangLens: Detect expansion-from-origin patterns (Big Bang-like).
///
/// n=6 connection: CMB temperature 2.725K, Hubble constant ~70 km/s/Mpc,
/// observable universe age ~13.8Gyr. Expansion patterns appear across all scales.
pub struct BigBangLens;

impl Lens for BigBangLens {
    fn name(&self) -> &str {
        "BigBangLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 {
            return HashMap::new();
        }

        let max_n = n.min(200);
        let mut result = HashMap::new();

        // Compute centroid and radii
        let (means, _) = crate::telescope::shared_data::mean_var(data, n, d);
        let mut radii = Vec::with_capacity(max_n);
        for i in 0..max_n {
            let mut r2 = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - means[j];
                r2 += diff * diff;
            }
            radii.push(r2.sqrt());
        }

        // 1. Expansion rate (Hubble-like): velocity vs distance.
        //    Treat index order as "time": later indices = further from origin = faster.
        //    Compute correlation between radius and index (proxy for recession velocity).
        let mean_r = radii.iter().sum::<f64>() / radii.len() as f64;
        let mean_i = (max_n - 1) as f64 / 2.0;
        let mut cov_ri = 0.0;
        let mut var_i = 0.0;
        let mut var_r = 0.0;
        for (idx, &r) in radii.iter().enumerate() {
            let di = idx as f64 - mean_i;
            let dr = r - mean_r;
            cov_ri += di * dr;
            var_i += di * di;
            var_r += dr * dr;
        }
        let expansion_rate = if var_i > 1e-15 && var_r > 1e-15 {
            cov_ri / (var_i.sqrt() * var_r.sqrt())
        } else {
            0.0
        };
        result.insert("expansion_rate".to_string(), vec![expansion_rate]);

        // 2. Origin concentration: fraction of points within 1/6 of max radius.
        let max_r = radii.iter().cloned().fold(0.0_f64, f64::max);
        if max_r > 1e-12 {
            let inner_radius = max_r / 6.0; // n=6
            let near_origin = radii.iter().filter(|&&r| r <= inner_radius).count();
            let origin_concentration = near_origin as f64 / radii.len() as f64;
            result.insert("origin_concentration".to_string(), vec![origin_concentration]);
        }

        // 3. Age gradient: Spearman-like rank correlation between index and radius.
        //    Strong positive correlation = expansion from origin over time.
        let mut indexed_radii: Vec<(usize, f64)> = radii.iter().enumerate().map(|(i, &r)| (i, r)).collect();
        indexed_radii.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        let mut rank_r = vec![0.0; max_n];
        for (rank, &(orig_idx, _)) in indexed_radii.iter().enumerate() {
            rank_r[orig_idx] = rank as f64;
        }
        let mean_rank = (max_n - 1) as f64 / 2.0;
        let mut cov_ranks = 0.0;
        let mut var_rank_i = 0.0;
        let mut var_rank_r = 0.0;
        for i in 0..max_n {
            let di = i as f64 - mean_rank;
            let dr = rank_r[i] - mean_rank;
            cov_ranks += di * dr;
            var_rank_i += di * di;
            var_rank_r += dr * dr;
        }
        let age_gradient = if var_rank_i > 1e-15 && var_rank_r > 1e-15 {
            cov_ranks / (var_rank_i.sqrt() * var_rank_r.sqrt())
        } else {
            0.0
        };
        result.insert("age_gradient".to_string(), vec![age_gradient]);

        // 4. Isotropy: measure how uniform the expansion is in all directions.
        //    Compute directional variance of unit vectors from centroid.
        if d >= 2 {
            let mut dir_sums = vec![0.0; d];
            let mut valid_count = 0usize;
            for i in 0..max_n {
                let r = radii[i];
                if r > 1e-12 {
                    for j in 0..d {
                        dir_sums[j] += (data[i * d + j] - means[j]) / r;
                    }
                    valid_count += 1;
                }
            }
            if valid_count > 0 {
                let inv_n = 1.0 / valid_count as f64;
                let dir_magnitude: f64 = dir_sums.iter().map(|&s| (s * inv_n).powi(2)).sum::<f64>();
                let isotropy = 1.0 - dir_magnitude.sqrt().min(1.0);
                result.insert("isotropy".to_string(), vec![isotropy]);
            }
        }

        // 5. CMB uniformity: after removing the mean radial trend, measure
        //    uniformity of residuals (analogous to CMB anisotropy).
        if radii.len() >= 6 {
            let residuals: Vec<f64> = radii.iter().map(|&r| r - mean_r).collect();
            let res_var = residuals.iter().map(|&r| r * r).sum::<f64>() / residuals.len() as f64;
            let cmb_uniformity = if mean_r > 1e-12 {
                (1.0 - (res_var.sqrt() / mean_r)).max(0.0)
            } else {
                0.0
            };
            result.insert("cmb_uniformity".to_string(), vec![cmb_uniformity]);
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_big_bang_lens_expanding() {
        // Data that expands from origin: index determines radius
        let n = 20;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let r = (i + 1) as f64;
            let angle = i as f64 * 0.5;
            data.push(r * angle.cos());
            data.push(r * angle.sin());
        }
        let shared = SharedData::compute(&data, n, d);
        let result = BigBangLens.scan(&data, n, d, &shared);
        assert!(!result.is_empty());
        assert!(result.contains_key("expansion_rate"));
        assert!(result.contains_key("isotropy"));
        assert!(result.contains_key("cmb_uniformity"));

        // Should detect age gradient (later indices → larger radius)
        let ag = result.get("age_gradient").unwrap()[0];
        assert!(ag > 0.5, "Expanding data should have positive age gradient, got {}", ag);
    }

    #[test]
    fn test_big_bang_lens_uniform_sphere() {
        // Points on a sphere — high isotropy, high CMB uniformity
        let n = 24; // J₂ = 24
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            data.push(5.0 * angle.cos());
            data.push(5.0 * angle.sin());
        }
        let shared = SharedData::compute(&data, n, d);
        let result = BigBangLens.scan(&data, n, d, &shared);
        assert!(!result.is_empty());

        let iso = result.get("isotropy").unwrap()[0];
        assert!(iso > 0.8, "Uniform sphere should have high isotropy, got {}", iso);

        let cmb = result.get("cmb_uniformity").unwrap()[0];
        assert!(cmb > 0.9, "Uniform sphere should have high CMB uniformity, got {}", cmb);
    }

    #[test]
    fn test_big_bang_lens_small_n() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let shared = SharedData::compute(&data, 5, 1);
        let result = BigBangLens.scan(&data, 5, 1, &shared);
        assert!(result.is_empty());
    }
}
