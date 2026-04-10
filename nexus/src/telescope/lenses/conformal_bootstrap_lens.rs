use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var, column_vectors, shannon_entropy};

/// n=6 conformal/CFT constants (BT-105, BT-49)
const N: f64 = 6.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const J2: f64 = 24.0;
const SOPFR: f64 = 5.0;

/// Central charges and scaling dimensions from CFT
/// c = 0 for SLE_6 (percolation, BT-105)
/// c = 1/2 for Ising, c = 7/10 for tricritical Ising
const CFT_CENTRAL_CHARGES: &[(f64, &str)] = &[
    (0.0, "SLE_6 percolation"),
    (0.5, "Ising 2D"),
    (0.7, "tricritical Ising"),
    (1.0, "free boson"),
    (4.0, "N=4 SYM minimal"),
];

/// Known critical exponents for percolation (d=2, SLE_6)
/// All expressible as n=6 fractions (BT-105)
const PERCOLATION_EXPONENTS: &[(f64, &str)] = &[
    (5.0 / 36.0, "beta (order param)"),        // 5/36
    (43.0 / 18.0, "gamma (susceptibility)"),    // 43/18
    (5.0 / 48.0, "eta (anomalous)"),            // 5/48 ≈ 0.1042
    (4.0 / 3.0, "nu (correlation length)"),     // 4/3 = tau^2/sigma
    (91.0 / 5.0, "delta (mag field)"),          // 91/5
    (187.0 / 91.0, "dh (hull fractal)"),        // ≈ 2.055
    (5.0 / 24.0, "h_sigma (scaling dim)"),      // 5/24 = sopfr/J2
];

const REL_TOL: f64 = 0.08;

/// ConformalBootstrapLens: Detect conformal symmetry patterns in data.
///
/// Algorithm:
///   1. Estimate scaling dimensions from power-law correlations
///   2. Check for conformal invariance via scale-free distance distribution
///   3. Detect OPE coefficient patterns (fusion rule structure)
///   4. Measure central charge proxy from entanglement entropy scaling
///   5. Match against known CFT critical exponents (especially SLE_6)
///   6. Reports scaling_dimension, central_charge_proxy, ope_structure, conformal_score
pub struct ConformalBootstrapLens;

impl Lens for ConformalBootstrapLens {
    fn name(&self) -> &str {
        "ConformalBootstrapLens"
    }

    fn category(&self) -> &str {
        "T2"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 5 || d == 0 {
            return HashMap::new();
        }

        // 1. Estimate scaling dimension from two-point correlations
        //    In CFT, <O(x)O(y)> ~ |x-y|^{-2*Delta}
        //    Fit log(correlation) vs log(distance) to get -2*Delta
        let nn = n.min(50);
        let mut log_dists = Vec::new();
        let mut log_corrs = Vec::new();

        for i in 0..nn {
            for j in (i + 1)..nn {
                let dist = shared.dist(i, j);
                if dist < 1e-12 { continue; }

                // Correlation proxy: overlap of normalized feature vectors
                let mut dot_val = 0.0;
                let mut ni2 = 0.0;
                let mut nj2 = 0.0;
                for dim in 0..d {
                    let vi = data[i * d + dim];
                    let vj = data[j * d + dim];
                    dot_val += vi * vj;
                    ni2 += vi * vi;
                    nj2 += vj * vj;
                }
                let denom = (ni2 * nj2).sqrt();
                if denom < 1e-12 { continue; }
                let corr = (dot_val / denom).abs().max(1e-15);

                log_dists.push(dist.ln());
                log_corrs.push(corr.ln());
            }
        }

        // Linear regression: log_corr = slope * log_dist + intercept
        // slope ≈ -2*Delta (scaling dimension)
        let (slope, r_squared) = if log_dists.len() >= 3 {
            linear_regression(&log_dists, &log_corrs)
        } else {
            (0.0, 0.0)
        };

        let scaling_dimension = (-slope / 2.0).max(0.0);

        // Match scaling dimension against known critical exponents
        let mut best_exponent_match = ("none", f64::MAX);
        for &(exp, name) in PERCOLATION_EXPONENTS {
            let dist = (scaling_dimension - exp).abs();
            if dist < best_exponent_match.1 {
                best_exponent_match = (name, dist);
            }
        }
        let exponent_match_score = (-best_exponent_match.1 * 5.0).exp();

        // 2. Scale-free test: distance distribution should follow power law
        //    if data has conformal invariance
        let mut all_dists = Vec::new();
        for i in 0..nn {
            for j in (i + 1)..nn {
                let dd = shared.dist(i, j);
                if dd > 1e-12 {
                    all_dists.push(dd);
                }
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Check if CDF follows power law: log-log linearity
        let scale_free_score = if all_dists.len() >= 5 {
            let log_r: Vec<f64> = all_dists.iter().map(|d| d.ln()).collect();
            let log_cdf: Vec<f64> = (0..all_dists.len())
                .map(|i| (1.0 - (i as f64 + 0.5) / all_dists.len() as f64).max(1e-15).ln())
                .collect();
            let (_, r2) = linear_regression(&log_r, &log_cdf);
            r2 // R² of power law fit
        } else {
            0.0
        };

        // 3. OPE structure: check if feature correlations have fusion-rule-like structure
        //    OPE: O_i x O_j = sum_k C_{ijk} O_k
        //    Proxy: MI matrix should have structured pattern (not random)
        let mi_entropy = if d >= 2 {
            let mi_vals: Vec<f64> = (0..d)
                .flat_map(|i| (0..d).filter(move |&j| j != i).map(move |j| shared.mi(i, j)))
                .filter(|&v| v > 1e-15)
                .collect();
            if !mi_vals.is_empty() {
                let max_mi = mi_vals.iter().cloned().fold(0.0_f64, f64::max);
                if max_mi > 1e-12 {
                    // Normalized MI distribution entropy: low = structured (like OPE)
                    let normed: Vec<f64> = mi_vals.iter().map(|v| v / max_mi).collect();
                    shannon_entropy(&normed, (mi_vals.len() as f64).sqrt().max(3.0) as usize)
                } else {
                    0.0
                }
            } else {
                0.0
            }
        } else {
            0.0
        };
        // Lower MI entropy = more structured = higher OPE score
        let ope_structure = (-mi_entropy).exp();

        // 4. Central charge proxy from entanglement entropy scaling
        //    S(A) = (c/3) * ln(L_A) + const (1+1D CFT)
        //    Use data partitions at different scales
        let central_charge_proxy = if n >= 8 {
            let columns = column_vectors(data, n, d);
            let mut entropies_by_scale = Vec::new();
            let mut log_scales = Vec::new();

            for scale_factor in &[2, 4, 8, 16] {
                let block_size = *scale_factor;
                if block_size >= n { break; }
                let num_blocks = n / block_size;
                if num_blocks < 2 { break; }

                // Entropy of block means (first feature)
                if let Some(col) = columns.first() {
                    let block_means: Vec<f64> = (0..num_blocks)
                        .map(|b| {
                            let start = b * block_size;
                            let end = (start + block_size).min(n);
                            col[start..end].iter().sum::<f64>() / (end - start) as f64
                        })
                        .collect();
                    let s = shannon_entropy(&block_means, (num_blocks as f64).sqrt().max(3.0) as usize);
                    entropies_by_scale.push(s);
                    log_scales.push((block_size as f64).ln());
                }
            }

            if entropies_by_scale.len() >= 2 {
                let (slope, _) = linear_regression(&log_scales, &entropies_by_scale);
                // c/3 ≈ slope, so c ≈ 3*slope
                (slope * 3.0).abs()
            } else {
                0.0
            }
        } else {
            0.0
        };

        // Match central charge to known CFT values
        let mut best_cc_match = ("unknown", f64::MAX);
        for &(cc, name) in CFT_CENTRAL_CHARGES {
            let dist = (central_charge_proxy - cc).abs();
            if dist < best_cc_match.1 {
                best_cc_match = (name, dist);
            }
        }
        let cc_match_score = (-best_cc_match.1 * 3.0).exp();

        // 5. nu = 4/3 proximity (correlation length exponent, BT-105/BT-111)
        let nu_43_proximity = (-(scaling_dimension - 4.0 / 3.0).powi(2) * 10.0).exp();

        // Combined conformal score
        let conformal_score = r_squared * 0.25          // power-law correlation quality
            + exponent_match_score * 0.2                 // known exponent match
            + scale_free_score * 0.2                     // scale-free distance distribution
            + ope_structure * 0.15                       // OPE-like MI structure
            + cc_match_score * 0.1                       // central charge match
            + nu_43_proximity * 0.1;                     // nu=4/3 special

        let mut result = HashMap::new();
        result.insert("scaling_dimension".to_string(), vec![scaling_dimension]);
        result.insert("correlation_r_squared".to_string(), vec![r_squared]);
        result.insert("exponent_match_score".to_string(), vec![exponent_match_score]);
        result.insert("scale_free_score".to_string(), vec![scale_free_score]);
        result.insert("ope_structure".to_string(), vec![ope_structure]);
        result.insert("central_charge_proxy".to_string(), vec![central_charge_proxy]);
        result.insert("cc_match_score".to_string(), vec![cc_match_score]);
        result.insert("nu_43_proximity".to_string(), vec![nu_43_proximity]);
        result.insert("conformal_score".to_string(), vec![conformal_score]);
        result
    }
}

/// Simple linear regression: y = slope * x + intercept.
/// Returns (slope, R²).
fn linear_regression(x: &[f64], y: &[f64]) -> (f64, f64) {
    let n = x.len().min(y.len());
    if n < 2 {
        return (0.0, 0.0);
    }
    let n_f = n as f64;

    let sum_x: f64 = x[..n].iter().sum();
    let sum_y: f64 = y[..n].iter().sum();
    let sum_xy: f64 = x[..n].iter().zip(y[..n].iter()).map(|(a, b)| a * b).sum();
    let sum_x2: f64 = x[..n].iter().map(|a| a * a).sum();
    let sum_y2: f64 = y[..n].iter().map(|a| a * a).sum();

    let denom = n_f * sum_x2 - sum_x * sum_x;
    if denom.abs() < 1e-15 {
        return (0.0, 0.0);
    }

    let slope = (n_f * sum_xy - sum_x * sum_y) / denom;

    // R² = correlation coefficient squared
    let r_num = n_f * sum_xy - sum_x * sum_y;
    let r_denom = ((n_f * sum_x2 - sum_x * sum_x) * (n_f * sum_y2 - sum_y * sum_y)).sqrt();
    let r_squared = if r_denom > 1e-15 {
        (r_num / r_denom).powi(2)
    } else {
        0.0
    };

    (slope, r_squared)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_conformal_power_law_correlation() {
        // Create data with power-law correlations: f(r) ~ r^{-alpha}
        let n = 20;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let r = (i + 1) as f64;
            data.push(r);
            data.push(1.0 / r.powf(0.5)); // decaying correlation
        }
        let shared = make_shared(&data, n, d);
        let result = ConformalBootstrapLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("scaling_dimension"));
        assert!(result.contains_key("conformal_score"));
        let sd = result["scaling_dimension"][0];
        assert!(sd.is_finite(), "Scaling dimension should be finite, got {sd}");
    }

    #[test]
    fn test_conformal_central_charge_proxy() {
        // Create data with multi-scale structure
        let n = 32;
        let d = 2;
        let mut data = Vec::with_capacity(n * d);
        for i in 0..n {
            let x = i as f64 / n as f64;
            data.push(x);
            data.push((x * std::f64::consts::PI * 4.0).sin());
        }
        let shared = make_shared(&data, n, d);
        let result = ConformalBootstrapLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("central_charge_proxy"));
        let cc = result["central_charge_proxy"][0];
        assert!(cc.is_finite(), "Central charge should be finite, got {cc}");
    }

    #[test]
    fn test_conformal_linear_regression() {
        let x = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let y = vec![2.0, 4.0, 6.0, 8.0, 10.0];
        let (slope, r2) = linear_regression(&x, &y);
        assert!((slope - 2.0).abs() < 1e-10, "Slope should be 2.0, got {slope}");
        assert!((r2 - 1.0).abs() < 1e-10, "R² should be 1.0, got {r2}");
    }
}
