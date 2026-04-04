use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// PolymerLens: Analyze chain statistics and random walk properties.
///
/// Treats sequential data points as a polymer chain and measures:
/// - End-to-end distance vs contour length (Flory exponent)
/// - Radius of gyration
/// - Persistence length
/// n=6 connection: Carbon Z=6 is the backbone of all organic polymers,
/// hexagonal packing is the densest 2D arrangement (BT-122),
/// Flory exponent nu ~ 3/(d+2) where d=3 gives nu=3/5=0.6.
pub struct PolymerLens;

impl Lens for PolymerLens {
    fn name(&self) -> &str { "PolymerLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let nn = n.min(200);

        // Treat rows as sequential monomers in a chain
        // Bond vectors: b_i = r_{i+1} - r_i
        let mut bond_lengths = Vec::with_capacity(nn - 1);
        let mut contour_length = 0.0;

        for i in 0..(nn - 1) {
            let mut bl2 = 0.0;
            for j in 0..d {
                let diff = data[(i + 1) * d + j] - data[i * d + j];
                bl2 += diff * diff;
            }
            let bl = bl2.sqrt();
            bond_lengths.push(bl);
            contour_length += bl;
        }

        if contour_length < 1e-12 { return HashMap::new(); }

        // End-to-end distance
        let mut end_to_end_sq = 0.0;
        for j in 0..d {
            let diff = data[(nn - 1) * d + j] - data[j];
            end_to_end_sq += diff * diff;
        }
        let end_to_end = end_to_end_sq.sqrt();

        // Radius of gyration
        let mut com = vec![0.0; d];
        for i in 0..nn {
            for j in 0..d {
                com[j] += data[i * d + j];
            }
        }
        for j in 0..d {
            com[j] /= nn as f64;
        }
        let mut rg_sq = 0.0;
        for i in 0..nn {
            for j in 0..d {
                let diff = data[i * d + j] - com[j];
                rg_sq += diff * diff;
            }
        }
        rg_sq /= nn as f64;
        let rg = rg_sq.sqrt();

        // Flory exponent estimate: R_ee ~ N^nu => nu = log(R_ee) / log(N)
        let flory_exponent = if nn > 2 && end_to_end > 1e-12 {
            end_to_end.ln() / (nn as f64).ln()
        } else {
            0.0
        };

        // Persistence length: decay of bond-bond correlations
        // <b_i . b_{i+s}> ~ exp(-s/l_p)
        let mean_bl = contour_length / (nn - 1) as f64;
        let max_s = (nn / 2).min(12); // sigma=12 max correlation lag
        let mut correlations = Vec::with_capacity(max_s);
        for s in 1..=max_s {
            let mut dot_sum = 0.0;
            let mut count = 0;
            for i in 0..(nn - 1 - s) {
                let mut dot = 0.0;
                for j in 0..d {
                    let b1 = data[(i + 1) * d + j] - data[i * d + j];
                    let b2 = data[(i + s + 1) * d + j] - data[(i + s) * d + j];
                    dot += b1 * b2;
                }
                dot_sum += dot;
                count += 1;
            }
            if count > 0 {
                correlations.push(dot_sum / count as f64);
            }
        }

        let persistence_length = if correlations.len() >= 2 {
            // Fit exponential decay
            let c0 = correlations[0].abs().max(1e-12);
            let mut lp_sum = 0.0;
            let mut lp_count = 0;
            for (s, &c) in correlations.iter().enumerate() {
                if c > 1e-12 && c0 > 1e-12 {
                    let ratio = c / c0;
                    if ratio > 0.01 {
                        lp_sum += -(s as f64 + 1.0) / ratio.ln();
                        lp_count += 1;
                    }
                }
            }
            if lp_count > 0 { lp_sum / lp_count as f64 } else { 0.0 }
        } else {
            0.0
        };

        // n=6 chain ratio: end-to-end / (6 * mean_bond_length)
        let n6_chain_ratio = if mean_bl > 1e-12 {
            end_to_end / (6.0 * mean_bl)
        } else {
            0.0
        };

        // Shape anisotropy: rg / end_to_end
        let shape_factor = if end_to_end > 1e-12 { rg / end_to_end } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("end_to_end_distance".to_string(), vec![end_to_end]);
        result.insert("contour_length".to_string(), vec![contour_length]);
        result.insert("radius_of_gyration".to_string(), vec![rg]);
        result.insert("flory_exponent".to_string(), vec![flory_exponent]);
        result.insert("persistence_length".to_string(), vec![persistence_length]);
        result.insert("n6_chain_ratio".to_string(), vec![n6_chain_ratio]);
        result.insert("shape_factor".to_string(), vec![shape_factor]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_polymer_straight_chain() {
        // Straight line: end-to-end = contour length, Flory ~ 1
        let n = 20;
        let d = 2;
        let data: Vec<f64> = (0..n).flat_map(|i| vec![i as f64, 0.0]).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = PolymerLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("flory_exponent"));
        let ee = result["end_to_end_distance"][0];
        let cl = result["contour_length"][0];
        assert!((ee - cl).abs() / cl < 0.01, "Straight chain: end-to-end should equal contour");
    }

    #[test]
    fn test_polymer_coiled() {
        // Circular/coiled chain: end-to-end << contour length
        let n = 24; // J2=24 monomers
        let d = 2;
        let data: Vec<f64> = (0..n).flat_map(|i| {
            let theta = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            vec![theta.cos(), theta.sin()]
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = PolymerLens.scan(&data, n, d, &shared);
        let ee = result["end_to_end_distance"][0];
        let cl = result["contour_length"][0];
        assert!(ee < cl * 0.5, "Coiled chain: end-to-end should be much less than contour");
    }
}
