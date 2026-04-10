use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// MirrorLens (대칭/거울): reflection symmetry score.
///
/// Algorithm:
///   1. Compute centroid of all points
///   2. For each point, find its "mirror" (centroid-reflected counterpart)
///   3. Find closest actual point to each mirror position
///   4. Symmetry score = 1 - mean(min_mirror_distance / max_distance)
pub struct MirrorLens;

impl Lens for MirrorLens {
    fn name(&self) -> &str {
        "MirrorLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 3 || d == 0 {
            return HashMap::new();
        }

        // Compute centroid
        let mut centroid = vec![0.0; d];
        for i in 0..n {
            for j in 0..d {
                centroid[j] += data[i * d + j];
            }
        }
        for c in centroid.iter_mut() {
            *c /= n as f64;
        }

        // Max pairwise distance for normalization
        let mut max_dist = 0.0_f64;
        for i in 0..n {
            for j in (i + 1)..n {
                let dd = shared.dist(i, j);
                if dd > max_dist {
                    max_dist = dd;
                }
            }
        }
        if max_dist < 1e-15 {
            let mut result = HashMap::new();
            result.insert("symmetry_score".to_string(), vec![1.0]);
            result.insert("mean_mirror_error".to_string(), vec![0.0]);
            return result;
        }

        // For each point, compute mirror position and find closest actual point
        let mut mirror_errors: Vec<f64> = Vec::with_capacity(n);

        for i in 0..n {
            // Mirror of point i across centroid
            let mirror: Vec<f64> = (0..d)
                .map(|dim| 2.0 * centroid[dim] - data[i * d + dim])
                .collect();

            // Find closest actual point to mirror (excluding self)
            let mut min_dist = f64::MAX;
            for j in 0..n {
                if j == i {
                    continue;
                }
                let dist: f64 = (0..d)
                    .map(|dim| {
                        let diff = data[j * d + dim] - mirror[dim];
                        diff * diff
                    })
                    .sum::<f64>()
                    .sqrt();
                if dist < min_dist {
                    min_dist = dist;
                }
            }

            mirror_errors.push(min_dist / max_dist);
        }

        let mean_error = mirror_errors.iter().sum::<f64>() / n as f64;
        let symmetry_score = (1.0 - mean_error).max(0.0);

        let mut result = HashMap::new();
        result.insert("symmetry_score".to_string(), vec![symmetry_score]);
        result.insert("mean_mirror_error".to_string(), vec![mean_error]);
        result
    }
}
