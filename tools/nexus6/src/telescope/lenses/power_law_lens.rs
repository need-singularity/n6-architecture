use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PowerLawLens: Detect power-law distributions and scale-free behavior.
///
/// Algorithm:
///   1. Sort distances and fit log-log CDF
///   2. Estimate exponent alpha via linear regression
///   3. Check if alpha matches n=6 values (e.g., 4/3, 5/3, 2)
pub struct PowerLawLens;

const N6_EXPONENTS: &[(f64, &str)] = &[
    (1.0, "mu=1"),
    (4.0 / 3.0, "tau^2/sigma = 4/3"),
    (3.0 / 2.0, "n/tau = 3/2"),
    (5.0 / 3.0, "sopfr/n_over_phi = 5/3"),
    (2.0, "phi=2"),
    (5.0 / 2.0, "sopfr/phi = 5/2"),
    (3.0, "n/phi = 3"),
];

impl Lens for PowerLawLens {
    fn name(&self) -> &str { "PowerLawLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }

        // Collect all pairwise distances
        let nn = n.min(80);
        let mut dists = Vec::new();
        for i in 0..nn {
            for j in (i + 1)..nn {
                let d = shared.dist(i, j);
                if d > 1e-12 { dists.push(d); }
            }
        }
        if dists.len() < 6 { return HashMap::new(); }

        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Log-log CDF fit
        let total = dists.len();
        let log_x: Vec<f64> = dists.iter().map(|d| d.ln()).collect();
        let log_cdf: Vec<f64> = (0..total)
            .map(|i| (1.0 - (i as f64 + 0.5) / total as f64).max(1e-15).ln())
            .collect();

        let (slope, r2) = linear_regression(&log_x, &log_cdf);
        let alpha = -slope; // Power-law exponent

        // Match against n=6 exponents
        let mut best = ("none", f64::MAX);
        for &(exp, name) in N6_EXPONENTS {
            let dist = (alpha - exp).abs();
            if dist < best.1 { best = (name, dist); }
        }
        let match_score = (-best.1 * 5.0).exp();

        let mut result = HashMap::new();
        result.insert("power_law_alpha".to_string(), vec![alpha]);
        result.insert("power_law_r2".to_string(), vec![r2]);
        result.insert("n6_exponent_match".to_string(), vec![match_score]);
        result
    }
}

fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
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
    fn test_power_law_lens() {
        // Points with power-law spacing
        let mut data = Vec::new();
        for i in 1..=20 {
            data.push(i as f64);
            data.push((i as f64).powf(1.5));
        }
        let n = 20;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = PowerLawLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("power_law_alpha"));
        assert!(result["power_law_alpha"][0].is_finite());
    }
}
