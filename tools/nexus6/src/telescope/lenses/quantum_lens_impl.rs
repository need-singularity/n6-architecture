use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// QuantumLensImpl: mutual information-based entanglement analogy.
///
/// Algorithm:
///   1. For each dimension pair, compute mutual information via 2D histogram
///   2. Entanglement score = mean MI across all dimension pairs
///   3. Max MI pair = most "entangled" dimensions
pub struct QuantumLensImpl;

impl Lens for QuantumLensImpl {
    fn name(&self) -> &str {
        "QuantumLensImpl"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 4 || d < 2 {
            return HashMap::new();
        }

        let num_bins = ((n as f64).sqrt().ceil() as usize).max(2).min(20);

        let mut mi_values: Vec<f64> = Vec::new();
        let mut max_mi = 0.0_f64;
        let mut max_pair = (0usize, 1usize);

        // Compute MI for each dimension pair
        let pairs_to_check = d.min(10); // limit for large d
        for di in 0..pairs_to_check {
            for dj in (di + 1)..pairs_to_check.min(d) {
                let col_i: Vec<f64> = (0..n).map(|r| data[r * d + di]).collect();
                let col_j: Vec<f64> = (0..n).map(|r| data[r * d + dj]).collect();

                let mi = mutual_information(&col_i, &col_j, n, num_bins);
                mi_values.push(mi);
                if mi > max_mi {
                    max_mi = mi;
                    max_pair = (di, dj);
                }
            }
        }

        let mean_mi = if mi_values.is_empty() {
            0.0
        } else {
            mi_values.iter().sum::<f64>() / mi_values.len() as f64
        };

        let mut result = HashMap::new();
        result.insert("mean_mutual_information".to_string(), vec![mean_mi]);
        result.insert(
            "max_entangled_pair".to_string(),
            vec![max_pair.0 as f64, max_pair.1 as f64, max_mi],
        );
        result
    }
}

fn mutual_information(x: &[f64], y: &[f64], n: usize, bins: usize) -> f64 {
    let x_min = x.iter().cloned().fold(f64::INFINITY, f64::min);
    let x_max = x.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let y_min = y.iter().cloned().fold(f64::INFINITY, f64::min);
    let y_max = y.iter().cloned().fold(f64::NEG_INFINITY, f64::max);

    let x_range = (x_max - x_min).max(1e-15);
    let y_range = (y_max - y_min).max(1e-15);

    // 2D histogram
    let mut joint = vec![vec![0usize; bins]; bins];
    let mut marginal_x = vec![0usize; bins];
    let mut marginal_y = vec![0usize; bins];

    for i in 0..n {
        let xi = ((x[i] - x_min) / x_range * (bins - 1) as f64).round() as usize;
        let yi = ((y[i] - y_min) / y_range * (bins - 1) as f64).round() as usize;
        let xi = xi.min(bins - 1);
        let yi = yi.min(bins - 1);
        joint[xi][yi] += 1;
        marginal_x[xi] += 1;
        marginal_y[yi] += 1;
    }

    let nf = n as f64;
    let mut mi = 0.0;
    for bx in 0..bins {
        for by in 0..bins {
            if joint[bx][by] > 0 && marginal_x[bx] > 0 && marginal_y[by] > 0 {
                let pxy = joint[bx][by] as f64 / nf;
                let px = marginal_x[bx] as f64 / nf;
                let py = marginal_y[by] as f64 / nf;
                mi += pxy * (pxy / (px * py)).ln();
            }
        }
    }

    mi.max(0.0)
}
