use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FractalLens: Estimate fractal dimension via box-counting method.
///
/// Algorithm:
///   1. Compute correlation integral C(r) at multiple scales
///   2. Fit log(C(r)) vs log(r) to estimate correlation dimension
///   3. Check if dimension matches n=6 constants (e.g., 2, 4/3, 6)
pub struct FractalLens;

const N6_DIMS: &[(f64, &str)] = &[
    (1.0, "mu=1 (line)"),
    (4.0 / 3.0, "tau^2/sigma (percolation nu)"),
    (3.0 / 2.0, "n/tau=1.5"),
    (2.0, "phi=2 (plane)"),
    (3.0, "n/phi=3 (space)"),
    (6.0, "n=6"),
];

impl Lens for FractalLens {
    fn name(&self) -> &str { "FractalLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 8 { return HashMap::new(); }

        let nn = n.min(100);

        // Collect all pairwise distances
        let mut all_dists = Vec::new();
        for i in 0..nn {
            for j in (i + 1)..nn {
                let d = shared.dist(i, j);
                if d > 1e-12 { all_dists.push(d); }
            }
        }
        if all_dists.len() < 10 { return HashMap::new(); }

        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let max_dist = all_dists[all_dists.len() - 1];
        let min_dist = all_dists[0];
        if max_dist <= min_dist { return HashMap::new(); }

        // Correlation integral at multiple radii
        let n_scales = 12; // sigma=12 scales
        let mut log_r = Vec::with_capacity(n_scales);
        let mut log_c = Vec::with_capacity(n_scales);

        for s in 0..n_scales {
            let frac = (s as f64 + 1.0) / (n_scales as f64 + 1.0);
            let r = min_dist * (max_dist / min_dist).powf(frac);

            // Count pairs within distance r
            let count = all_dists.iter().take_while(|&&d| d <= r).count();
            let cr = count as f64 / all_dists.len() as f64;
            if cr > 1e-12 {
                log_r.push(r.ln());
                log_c.push(cr.ln());
            }
        }

        if log_r.len() < 3 { return HashMap::new(); }

        // Linear regression for correlation dimension
        let (slope, r2) = linear_reg(&log_r, &log_c);
        let corr_dim = slope.max(0.0);

        // Match to n=6 dimensions
        let mut best = ("none", f64::MAX);
        for &(dim, name) in N6_DIMS {
            let dist = (corr_dim - dim).abs();
            if dist < best.1 { best = (name, dist); }
        }
        let dim_match_score = (-best.1 * 3.0).exp();

        let mut result = HashMap::new();
        result.insert("correlation_dimension".to_string(), vec![corr_dim]);
        result.insert("dimension_r2".to_string(), vec![r2]);
        result.insert("n6_dimension_match".to_string(), vec![dim_match_score]);
        result
    }
}

fn linear_reg(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len().min(y.len());
    if n < 2 { return (0.0, 0.0); }
    let nf = n as f64;
    let sx: f64 = x[..n].iter().sum();
    let sy: f64 = y[..n].iter().sum();
    let sxy: f64 = x[..n].iter().zip(y[..n].iter()).map(|(a, b)| a * b).sum();
    let sx2: f64 = x[..n].iter().map(|a| a * a).sum();
    let sy2: f64 = y[..n].iter().map(|a| a * a).sum();
    let denom = nf * sx2 - sx * sx;
    if denom.abs() < 1e-15 { return (0.0, 0.0); }
    let slope = (nf * sxy - sx * sy) / denom;
    let r_num = nf * sxy - sx * sy;
    let r_denom = ((nf * sx2 - sx * sx) * (nf * sy2 - sy * sy)).sqrt();
    let r2 = if r_denom > 1e-15 { (r_num / r_denom).powi(2) } else { 0.0 };
    (slope, r2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fractal_lens_line() {
        // Points on a line -> dimension should be ~1
        let mut data = Vec::new();
        for i in 0..30 {
            data.push(i as f64);
            data.push(0.0);
        }
        let n = 30;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = FractalLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("correlation_dimension"));
        let dim = result["correlation_dimension"][0];
        assert!(dim < 2.0, "Line should have dim < 2, got {}", dim);
    }
}
