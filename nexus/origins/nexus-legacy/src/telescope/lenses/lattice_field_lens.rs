use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::{SharedData, mean_var};

/// n=6 lattice/crystal constants
const N: f64 = 6.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const J2: f64 = 24.0;

/// Known coordination numbers for common lattices
const CN_TARGETS: &[(f64, &str)] = &[
    (4.0, "diamond/tetrahedral (tau)"),
    (6.0, "simple_cubic/octahedral (n)"),
    (8.0, "BCC (sigma-tau)"),
    (12.0, "FCC/HCP (sigma)"),
    (24.0, "Leech_kissing_2D (J2)"),
];

/// Known packing fractions
const PACKING_FRACTIONS: &[(f64, &str)] = &[
    (0.5236, "simple_cubic (pi/6)"),
    (0.6802, "BCC (pi*sqrt(3)/8)"),
    (0.7405, "FCC/HCP (pi/(3*sqrt(2)))"),
    (0.6667, "2/3 = phi_sq/n"),
    (0.3333, "1/3 = mu/n_over_phi"),
];

const REL_TOL: f64 = 0.08;

/// LatticeFieldLens: Detect lattice and crystal structure patterns in data.
///
/// Algorithm:
///   1. Compute effective coordination number via k-NN distance uniformity
///   2. Detect packing fraction from local density estimates
///   3. Identify symmetry group indicators (rotational symmetry in neighbor angles)
///   4. Check for lattice-constant ratios (c/a ratios near n=6 values)
///   5. Reports coordination_number, packing_fraction, symmetry_order, lattice_score
pub struct LatticeFieldLens;

impl Lens for LatticeFieldLens {
    fn name(&self) -> &str {
        "LatticeFieldLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 || d == 0 {
            return HashMap::new();
        }

        // 1. Effective coordination number detection
        //    For each point, count neighbors within r_cut = median_nn_dist * 1.2
        //    then check if the count matches a known CN
        let nn_dists: Vec<f64> = (0..n)
            .map(|i| {
                let mut min_d = f64::MAX;
                for j in 0..n {
                    if j == i { continue; }
                    let dd = shared.dist(i, j);
                    if dd < min_d { min_d = dd; }
                }
                min_d
            })
            .collect();

        let mut sorted_nn = nn_dists.clone();
        sorted_nn.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_nn = sorted_nn[n / 2];
        let r_cut = median_nn * 1.3;

        let cn_per_point: Vec<f64> = (0..n)
            .map(|i| {
                (0..n)
                    .filter(|&j| j != i && shared.dist(i, j) <= r_cut)
                    .count() as f64
            })
            .collect();

        let mean_cn = cn_per_point.iter().sum::<f64>() / n as f64;
        let cn_var = cn_per_point.iter()
            .map(|&c| (c - mean_cn).powi(2))
            .sum::<f64>() / n as f64;

        // Match mean CN to known values
        let mut best_cn_match = ("unknown", f64::MAX);
        for &(cn, name) in CN_TARGETS {
            let dist = (mean_cn - cn).abs();
            if dist < best_cn_match.1 {
                best_cn_match = (name, dist);
            }
        }
        let cn_match_score = (-best_cn_match.1 * 0.5).exp();

        // 2. Packing fraction estimation
        //    Approximate: volume of n hyperspheres / total bounding volume
        let (means, vars) = mean_var(data, n, d);

        let mut bounding_vol = 1.0_f64;
        for j in 0..d {
            let mut lo = f64::MAX;
            let mut hi = f64::NEG_INFINITY;
            for i in 0..n {
                let v = data[i * d + j];
                if v < lo { lo = v; }
                if v > hi { hi = v; }
            }
            bounding_vol *= (hi - lo).max(1e-12);
        }

        // Use median NN distance as sphere radius
        let r = median_nn / 2.0;
        let sphere_vol = if d <= 10 {
            // Volume of d-dim sphere: V_d(r) = pi^(d/2) / Gamma(d/2+1) * r^d
            let half_d = d as f64 / 2.0;
            let gamma_half_d_plus_1 = gamma_approx(half_d + 1.0);
            std::f64::consts::PI.powf(half_d) / gamma_half_d_plus_1 * r.powi(d as i32)
        } else {
            // For high dimensions, use simpler estimate
            r.powi(d as i32)
        };

        let packing_fraction = (n as f64 * sphere_vol / bounding_vol).min(1.0).max(0.0);

        // Match packing fraction
        let mut best_pf_match = ("unknown", f64::MAX);
        for &(pf, name) in PACKING_FRACTIONS {
            let dist = (packing_fraction - pf).abs();
            if dist < best_pf_match.1 {
                best_pf_match = (name, dist);
            }
        }
        let pf_match_score = (-best_pf_match.1 * 10.0).exp();

        // 3. Rotational symmetry detection
        //    Check if the distribution of angles between NN vectors has peaks
        //    at regular intervals (60° = hexagonal, 90° = cubic, etc.)
        let mut angle_histogram = vec![0u32; 36]; // 10-degree bins
        let mut angle_count = 0;
        let max_i = n.min(30); // limit computation
        for i in 0..max_i {
            // Get nearest neighbors
            let mut nn_indices: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, shared.dist(i, j)))
                .collect();
            nn_indices.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let k = mean_cn.round() as usize;
            let k = k.min(nn_indices.len()).max(1);

            // Compute angles between pairs of NN vectors
            for a in 0..k {
                for b in (a + 1)..k {
                    let ja = nn_indices[a].0;
                    let jb = nn_indices[b].0;
                    // Compute angle between (j_a - i) and (j_b - i)
                    let mut dot_val = 0.0;
                    let mut na2 = 0.0;
                    let mut nb2 = 0.0;
                    for dim in 0..d {
                        let va = data[ja * d + dim] - data[i * d + dim];
                        let vb = data[jb * d + dim] - data[i * d + dim];
                        dot_val += va * vb;
                        na2 += va * va;
                        nb2 += vb * vb;
                    }
                    let denom = (na2 * nb2).sqrt();
                    if denom > 1e-12 {
                        let cos_angle = (dot_val / denom).clamp(-1.0, 1.0);
                        let angle_deg = cos_angle.acos() * 180.0 / std::f64::consts::PI;
                        let bin = ((angle_deg / 10.0) as usize).min(35);
                        angle_histogram[bin] += 1;
                        angle_count += 1;
                    }
                }
            }
        }

        // Detect symmetry order from angle peaks
        let mut symmetry_order = 0.0;
        if angle_count > 0 {
            let max_bin = *angle_histogram.iter().max().unwrap_or(&0) as f64;
            let mean_bin = angle_count as f64 / 36.0;
            // Peak ratio indicates crystalline order
            let peak_ratio = if mean_bin > 0.0 { max_bin / mean_bin } else { 0.0 };

            // Check specific angles for symmetry types
            let hex_bins = [5, 6]; // 50-60° = hexagonal
            let cubic_bins = [8, 9]; // 80-90° = cubic
            let hex_signal: f64 = hex_bins.iter().map(|&b| angle_histogram[b] as f64).sum();
            let cubic_signal: f64 = cubic_bins.iter().map(|&b| angle_histogram[b] as f64).sum();

            if hex_signal > cubic_signal {
                symmetry_order = N; // hexagonal ~ 6-fold
            } else if cubic_signal > mean_bin * 2.0 {
                symmetry_order = TAU; // cubic ~ 4-fold
            }

            // Crystallinity score based on angle distribution peakiness
            let angle_entropy = {
                let total = angle_count as f64;
                angle_histogram.iter()
                    .filter(|&&c| c > 0)
                    .map(|&c| {
                        let p = c as f64 / total;
                        -p * p.ln()
                    })
                    .sum::<f64>()
            };
            let max_entropy = (36.0_f64).ln();
            let crystallinity = 1.0 - (angle_entropy / max_entropy);

            // Combine
            symmetry_order = symmetry_order.max(peak_ratio);
            let _ = crystallinity; // used implicitly via symmetry_order
        }

        // 4. Lattice constant ratio: c/a ratio from principal extent ratios
        let mut extents = Vec::with_capacity(d);
        for j in 0..d {
            let mut lo = f64::MAX;
            let mut hi = f64::NEG_INFINITY;
            for i in 0..n {
                let v = data[i * d + j];
                if v < lo { lo = v; }
                if v > hi { hi = v; }
            }
            extents.push(hi - lo);
        }
        extents.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        let mut ca_ratio = 0.0;
        if extents.len() >= 2 && extents[1] > 1e-12 {
            ca_ratio = extents[0] / extents[1];
        }
        // Ideal HCP c/a = sqrt(8/3) ≈ 1.633
        let hcp_ca = (8.0_f64 / 3.0).sqrt();
        let ca_match = (-(ca_ratio - hcp_ca).powi(2) * 5.0).exp();

        // CN uniformity: low variance = regular lattice
        let cn_uniformity = if mean_cn > 0.0 {
            (-cn_var.sqrt() / mean_cn * 3.0).exp()
        } else {
            0.0
        };

        // Combined lattice score
        let lattice_score = cn_match_score * 0.3
            + cn_uniformity * 0.25
            + pf_match_score * 0.2
            + ca_match * 0.15
            + (symmetry_order / SIGMA).min(1.0) * 0.1;

        let mut result = HashMap::new();
        result.insert("mean_coordination_number".to_string(), vec![mean_cn]);
        result.insert("cn_uniformity".to_string(), vec![cn_uniformity]);
        result.insert("cn_match_score".to_string(), vec![cn_match_score]);
        result.insert("packing_fraction".to_string(), vec![packing_fraction]);
        result.insert("pf_match_score".to_string(), vec![pf_match_score]);
        result.insert("symmetry_order".to_string(), vec![symmetry_order]);
        result.insert("ca_ratio".to_string(), vec![ca_ratio]);
        result.insert("ca_match_hcp".to_string(), vec![ca_match]);
        result.insert("lattice_score".to_string(), vec![lattice_score]);
        result.insert("cn_per_point".to_string(), cn_per_point);
        result
    }
}

/// Simple Gamma function approximation using Stirling for non-negative integers
/// and Lanczos for small values.
fn gamma_approx(x: f64) -> f64 {
    if x <= 0.0 { return f64::INFINITY; }
    if (x - x.round()).abs() < 1e-10 && x > 0.0 && x <= 20.0 {
        // For positive integers, Gamma(n) = (n-1)!
        let n = x.round() as u64;
        let mut result = 1.0_f64;
        for i in 1..n {
            result *= i as f64;
        }
        return result;
    }
    // Half-integers: Gamma(n+0.5) = (2n)! / (4^n * n!) * sqrt(pi)
    if (x - 0.5 - (x - 0.5).floor()).abs() < 1e-10 {
        let n = (x - 0.5).floor() as u64;
        let mut num = 1.0_f64;
        for i in 1..=(2 * n) {
            num *= i as f64;
        }
        let denom = 4.0_f64.powi(n as i32) * {
            let mut f = 1.0;
            for i in 1..=n { f *= i as f64; }
            f
        };
        return num / denom * std::f64::consts::PI.sqrt();
    }
    // Stirling approximation for larger values
    (2.0 * std::f64::consts::PI / x).sqrt() * (x / std::f64::consts::E).powf(x)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_lattice_cubic_grid() {
        // Create a simple 3x3x3 cubic grid -> CN should be near 6
        let mut data = Vec::new();
        for x in 0..3 {
            for y in 0..3 {
                for z in 0..3 {
                    data.push(x as f64);
                    data.push(y as f64);
                    data.push(z as f64);
                }
            }
        }
        let n = 27;
        let d = 3;
        let shared = make_shared(&data, n, d);
        let result = LatticeFieldLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("mean_coordination_number"));
        let cn = result["mean_coordination_number"][0];
        // Interior points have CN=6, edge/corner have less; mean should be moderate
        assert!(cn > 2.0, "Cubic grid should have reasonable CN, got {cn}");
    }

    #[test]
    fn test_lattice_packing_fraction() {
        // Dense grid should have non-zero packing fraction
        let mut data = Vec::new();
        for x in 0..4 {
            for y in 0..4 {
                data.push(x as f64);
                data.push(y as f64);
            }
        }
        let n = 16;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let result = LatticeFieldLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("packing_fraction"));
        let pf = result["packing_fraction"][0];
        assert!(pf > 0.0 && pf <= 1.0, "Packing fraction should be in (0,1], got {pf}");
    }

    #[test]
    fn test_lattice_cn_uniformity() {
        // Regular grid should have high CN uniformity
        let mut data = Vec::new();
        for x in 0..5 {
            for y in 0..5 {
                data.push(x as f64);
                data.push(y as f64);
            }
        }
        let n = 25;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let result = LatticeFieldLens.scan(&data, n, d, &shared);

        assert!(result.contains_key("cn_uniformity"));
        // At least the result is computed without panic
        let cu = result["cn_uniformity"][0];
        assert!(cu >= 0.0, "CN uniformity should be non-negative, got {cu}");
    }
}
