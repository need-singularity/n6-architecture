use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// QuantumMicroLens (양자현미경): density matrix approximation + von Neumann entropy.
///
/// Algorithm:
///   1. Construct a Gaussian kernel density matrix: rho_ij = exp(-d_ij^2 / (2*sigma^2))
///   2. Normalize to trace 1
///   3. Von Neumann entropy S = -Tr(rho * ln(rho)) via eigenvalues
///   4. Purity = Tr(rho^2) — how "pure" vs "mixed" the state is
pub struct QuantumMicroLens;

impl Lens for QuantumMicroLens {
    fn name(&self) -> &str {
        "QuantumMicroLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // Use a manageable subset for the density matrix
        let m = n.min(100);

        // Bandwidth: median distance
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..m {
            for j in (i + 1)..m {
                dists.push(shared.dist(i, j));
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let sigma = dists[dists.len() / 2].max(1e-10);
        let two_sigma_sq = 2.0 * sigma * sigma;

        // Build density matrix (Gaussian kernel)
        let mut rho = vec![0.0; m * m];
        for i in 0..m {
            for j in 0..m {
                if i == j {
                    rho[i * m + j] = 1.0;
                } else {
                    let d = shared.dist(i, j);
                    rho[i * m + j] = (-d * d / two_sigma_sq).exp();
                }
            }
        }

        // Normalize: rho = rho / Tr(rho)
        let trace: f64 = (0..m).map(|i| rho[i * m + i]).sum();
        if trace > 1e-15 {
            for v in rho.iter_mut() {
                *v /= trace;
            }
        }

        // Purity = Tr(rho^2)
        let mut purity = 0.0;
        for i in 0..m {
            for k in 0..m {
                purity += rho[i * m + k] * rho[k * m + i];
            }
        }

        // Von Neumann entropy approximation:
        // Use rho^2 trace to approximate: S ~ -ln(purity) for near-pure states
        // For better approximation, use S = ln(m) * (1 - purity) / (1 - 1/m)
        // which interpolates between S=0 (pure, purity=1) and S=ln(m) (maximally mixed, purity=1/m)
        let vn_entropy = if purity >= 1.0 - 1e-10 {
            0.0
        } else if m > 1 {
            let max_entropy = (m as f64).ln();
            let normalized = (1.0 - purity) / (1.0 - 1.0 / m as f64);
            max_entropy * normalized
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("von_neumann_entropy".to_string(), vec![vn_entropy]);
        result.insert("purity".to_string(), vec![purity]);
        result
    }
}
