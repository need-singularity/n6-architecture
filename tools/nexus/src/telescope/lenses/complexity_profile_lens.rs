use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ComplexityProfileLens: Estimates computational complexity profile of data patterns.
///
/// Measures:
///   1. Sample complexity: minimum samples needed for reliable pattern detection
///   2. Dimensional complexity: effective dimensionality vs nominal
///   3. Algorithmic complexity proxy: compressibility of distance matrix
///   4. Interaction complexity: pairwise vs higher-order dependencies
///
/// Used to predict scan cost and optimize lens selection.
pub struct ComplexityProfileLens;

impl Lens for ComplexityProfileLens {
    fn name(&self) -> &str {
        "ComplexityProfileLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. Effective dimensionality via PCA proxy (variance explained)
        let mut means = vec![0.0_f64; dims];
        for r in 0..n {
            for di in 0..dims {
                means[di] += data[r * d + di];
            }
        }
        for m in &mut means { *m /= n as f64; }

        let mut variances: Vec<f64> = Vec::with_capacity(dims);
        for di in 0..dims {
            let v: f64 = (0..n).map(|r| (data[r * d + di] - means[di]).powi(2)).sum::<f64>() / n as f64;
            variances.push(v);
        }
        let total_var: f64 = variances.iter().sum();
        let effective_dim = if total_var > 1e-12 {
            let mut ent = 0.0_f64;
            for &v in &variances {
                if v > 1e-15 {
                    let p = v / total_var;
                    ent -= p * p.ln();
                }
            }
            ent.exp()
        } else {
            1.0
        };
        let dim_complexity = effective_dim / dims as f64; // 1.0 = all dims active

        // 2. Sample complexity: stability of statistics across subsamples
        let half = n / 2;
        let mut dist_diff = 0.0_f64;
        let check = half.min(50);
        let mut count = 0u32;
        for i in 0..check {
            for j in (i + 1)..check {
                let d1 = shared.dist(i, j);
                let d2 = shared.dist(i + half, (j + half).min(n - 1));
                dist_diff += (d1 - d2).abs() / d1.max(1e-12);
                count += 1;
            }
        }
        let subsample_instability = if count > 0 { dist_diff / count as f64 } else { 0.0 };
        // High instability = need more samples
        let sample_complexity = subsample_instability.min(1.0);

        // 3. Distance matrix compressibility
        let mut dist_vals: Vec<f64> = Vec::new();
        for i in 0..n.min(80) {
            for j in (i + 1)..n.min(80) {
                dist_vals.push(shared.dist(i, j));
            }
        }
        if dist_vals.is_empty() {
            return HashMap::new();
        }
        // Quantize to σ=12 bins and measure entropy
        dist_vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let d_min = dist_vals[0];
        let d_max = *dist_vals.last().unwrap();
        let d_range = (d_max - d_min).max(1e-12);
        let bins = 12usize;
        let mut bin_counts = vec![0u32; bins];
        for &v in &dist_vals {
            let idx = ((v - d_min) / d_range * (bins - 1) as f64).round() as usize;
            bin_counts[idx.min(bins - 1)] += 1;
        }
        let total = dist_vals.len() as f64;
        let mut dist_entropy = 0.0_f64;
        for &c in &bin_counts {
            if c > 0 {
                let p = c as f64 / total;
                dist_entropy -= p * p.ln();
            }
        }
        let max_ent = (bins as f64).ln();
        let compressibility = 1.0 - dist_entropy / max_ent;

        // 4. Interaction complexity: correlation density
        let mut high_corr_pairs = 0u32;
        let mut total_pairs = 0u32;
        let check_d = dims.min(12);
        let mut stds = vec![0.0_f64; check_d];
        for di in 0..check_d {
            stds[di] = variances[di].sqrt().max(1e-12);
        }
        for i in 0..check_d {
            for j in (i + 1)..check_d {
                let mut cov = 0.0_f64;
                for r in 0..n {
                    cov += (data[r * d + i] - means[i]) * (data[r * d + j] - means[j]);
                }
                cov /= n as f64;
                let corr = (cov / (stds[i] * stds[j])).abs();
                total_pairs += 1;
                if corr > 0.3 { high_corr_pairs += 1; }
            }
        }
        let interaction_density = if total_pairs > 0 {
            high_corr_pairs as f64 / total_pairs as f64
        } else {
            0.0
        };

        // Composite: overall complexity
        let overall_complexity = dim_complexity * 0.3
            + sample_complexity * 0.2
            + (1.0 - compressibility) * 0.25
            + interaction_density * 0.25;

        // Estimated scan cost class: 0=trivial, 1=light, 2=medium, 3=heavy
        let scan_cost_class = if overall_complexity < 0.25 {
            0.0
        } else if overall_complexity < 0.5 {
            1.0
        } else if overall_complexity < 0.75 {
            2.0
        } else {
            3.0
        };

        let mut result = HashMap::new();
        result.insert("overall_complexity".into(), vec![overall_complexity]);
        result.insert("effective_dimensionality".into(), vec![effective_dim]);
        result.insert("dim_complexity".into(), vec![dim_complexity]);
        result.insert("sample_complexity".into(), vec![sample_complexity]);
        result.insert("compressibility".into(), vec![compressibility]);
        result.insert("interaction_density".into(), vec![interaction_density]);
        result.insert("scan_cost_class".into(), vec![scan_cost_class]);
        result
    }
}
