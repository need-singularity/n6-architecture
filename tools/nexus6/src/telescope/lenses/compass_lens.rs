use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CompassLens (곡률/컴퍼스): Menger curvature from point triples.
///
/// Algorithm:
///   1. Sample triples of points
///   2. For each triple, compute Menger curvature = 4*area / (d01*d02*d12)
///   3. High curvature = tightly curved region, low = flat
///   4. Reports mean curvature and curvature variance
pub struct CompassLens;

impl Lens for CompassLens {
    fn name(&self) -> &str {
        "CompassLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        let mut curvatures: Vec<f64> = Vec::new();

        // Sample triples: for efficiency, limit to O(n^2) triples
        let step = if n > 30 { n / 30 } else { 1 };
        let indices: Vec<usize> = (0..n).step_by(step).collect();
        let m = indices.len();

        for ai in 0..m {
            for bi in (ai + 1)..m {
                for ci in (bi + 1)..m {
                    let a = indices[ai];
                    let b = indices[bi];
                    let c = indices[ci];

                    let dab = shared.dist(a, b);
                    let dac = shared.dist(a, c);
                    let dbc = shared.dist(b, c);

                    // Heron's formula for area
                    let s = (dab + dac + dbc) / 2.0;
                    let area_sq = s * (s - dab) * (s - dac) * (s - dbc);
                    let area = if area_sq > 0.0 { area_sq.sqrt() } else { 0.0 };

                    let product = dab * dac * dbc;
                    if product > 1e-15 {
                        let menger = 4.0 * area / product;
                        curvatures.push(menger);
                    }
                }
            }
        }

        if curvatures.is_empty() {
            return HashMap::new();
        }

        let mean_curv = curvatures.iter().sum::<f64>() / curvatures.len() as f64;
        let var_curv = curvatures
            .iter()
            .map(|&c| (c - mean_curv).powi(2))
            .sum::<f64>()
            / curvatures.len() as f64;

        let mut result = HashMap::new();
        result.insert("mean_curvature".to_string(), vec![mean_curv]);
        result.insert("curvature_variance".to_string(), vec![var_curv]);
        result
    }
}
