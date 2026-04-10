use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ThermoLens: Shannon entropy + free energy approximation.
///
/// Algorithm:
///   1. Bin data values to estimate probability distribution
///   2. Shannon entropy H = -sum(p * ln(p))
///   3. Free energy proxy F = E - T*S where E = mean distance, S = entropy
///   4. Reports entropy per dimension and free energy
pub struct ThermoLens;

impl Lens for ThermoLens {
    fn name(&self) -> &str {
        "ThermoLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        // Compute Shannon entropy per dimension using histogram binning
        let num_bins = ((n as f64).sqrt().ceil() as usize).max(2);
        let mut entropies: Vec<f64> = Vec::with_capacity(d);

        for dim in 0..d {
            let col: Vec<f64> = (0..n).map(|i| data[i * d + dim]).collect();
            let min_val = col.iter().cloned().fold(f64::INFINITY, f64::min);
            let max_val = col.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
            let range = max_val - min_val;

            if range < 1e-15 {
                entropies.push(0.0);
                continue;
            }

            let mut bins = vec![0usize; num_bins];
            for &v in &col {
                let idx = ((v - min_val) / range * (num_bins - 1) as f64).round() as usize;
                bins[idx.min(num_bins - 1)] += 1;
            }

            let h: f64 = bins
                .iter()
                .filter(|&&c| c > 0)
                .map(|&c| {
                    let p = c as f64 / n as f64;
                    -p * p.ln()
                })
                .sum();

            entropies.push(h);
        }

        let total_entropy: f64 = entropies.iter().sum();

        // Mean pairwise distance as "energy"
        let pair_count = n * (n - 1) / 2;
        let mean_dist: f64 = (0..n)
            .flat_map(|i| ((i + 1)..n).map(move |j| shared.dist(i, j)))
            .sum::<f64>()
            / pair_count as f64;

        // Free energy proxy: F = E - S (temperature = 1)
        let free_energy = mean_dist - total_entropy;

        let mut result = HashMap::new();
        result.insert("shannon_entropy".to_string(), vec![total_entropy]);
        result.insert("free_energy_proxy".to_string(), vec![free_energy]);
        result.insert("entropy_per_dim".to_string(), entropies);
        result
    }
}
