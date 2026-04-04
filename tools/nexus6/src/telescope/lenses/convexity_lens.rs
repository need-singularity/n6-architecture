use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ConvexityLens: Measure convexity/concavity structure of data.
///
/// Analyzes whether the data cloud is convex (all points inside
/// the convex hull) and measures curvature properties.
/// n=6 connection: regular hexagon (n=6 vertices) is the most
/// efficient convex partition of the plane (BT-122);
/// convex hull in d dimensions connects to kissing number K_d
/// where K_3 = sigma=12.
pub struct ConvexityLens;

impl Lens for ConvexityLens {
    fn name(&self) -> &str { "ConvexityLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d == 0 { return HashMap::new(); }

        let nn = n.min(100);

        // Center of mass
        let mut com = vec![0.0; d];
        for i in 0..nn {
            for j in 0..d {
                com[j] += data[i * d + j];
            }
        }
        for j in 0..d {
            com[j] /= nn as f64;
        }

        // Distance from COM for each point
        let mut dists_from_com: Vec<f64> = Vec::with_capacity(nn);
        for i in 0..nn {
            let mut dsq = 0.0;
            for j in 0..d {
                let diff = data[i * d + j] - com[j];
                dsq += diff * diff;
            }
            dists_from_com.push(dsq.sqrt());
        }

        let max_dist = dists_from_com.iter().cloned().fold(0.0_f64, f64::max);
        let mean_dist = dists_from_com.iter().sum::<f64>() / nn as f64;

        // Convexity ratio: mean_dist / max_dist
        // For a convex, uniformly filled body: ~0.5-0.7
        // For points on boundary only: ~1.0
        // For star-shaped (non-convex): lower
        let convexity_ratio = if max_dist > 1e-12 {
            mean_dist / max_dist
        } else {
            0.0
        };

        // Curvature estimation via second differences along nearest-neighbor chain
        let mut curvatures = Vec::new();
        for i in 1..(nn.min(50) - 1) {
            // Use consecutive points as chain
            let mut acc_sq = 0.0;
            for j in 0..d {
                let d2 = data[(i + 1) * d + j] - 2.0 * data[i * d + j] + data[(i - 1) * d + j];
                acc_sq += d2 * d2;
            }
            curvatures.push(acc_sq.sqrt());
        }

        let mean_curvature = if curvatures.is_empty() {
            0.0
        } else {
            curvatures.iter().sum::<f64>() / curvatures.len() as f64
        };

        let max_curvature = curvatures.iter().cloned().fold(0.0_f64, f64::max);

        // Curvature variation: low = smooth (sphere-like), high = spiky
        let curv_var = if !curvatures.is_empty() && mean_curvature > 1e-12 {
            let var: f64 = curvatures.iter()
                .map(|c| (c - mean_curvature).powi(2))
                .sum::<f64>() / curvatures.len() as f64;
            var.sqrt() / mean_curvature
        } else {
            0.0
        };

        // n=6 vertex check: does the point count on the "hull"
        // approximate n=6 or its multiples?
        // Count extreme points (dist > 0.8 * max_dist)
        let hull_count = dists_from_com.iter()
            .filter(|&&d| d > 0.8 * max_dist)
            .count() as f64;
        let n6_multiples = [6.0, 12.0, 24.0, 48.0];
        let n6_vertex_match = n6_multiples.iter()
            .map(|&m| (-(hull_count - m).abs() / m.max(1.0)).exp())
            .fold(0.0_f64, f64::max);

        let mut result = HashMap::new();
        result.insert("convexity_ratio".to_string(), vec![convexity_ratio]);
        result.insert("mean_curvature".to_string(), vec![mean_curvature]);
        result.insert("max_curvature".to_string(), vec![max_curvature]);
        result.insert("curvature_variation".to_string(), vec![curv_var]);
        result.insert("hull_point_count".to_string(), vec![hull_count]);
        result.insert("n6_vertex_match".to_string(), vec![n6_vertex_match]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_convexity_sphere() {
        // Points on a circle: all at boundary
        let n = 12; // sigma=12 points
        let d = 2;
        let data: Vec<f64> = (0..n).flat_map(|i| {
            let theta = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            vec![theta.cos(), theta.sin()]
        }).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ConvexityLens.scan(&data, n, d, &shared);
        assert!(result["convexity_ratio"][0] > 0.8, "Circle points should have high convexity ratio");
    }

    #[test]
    fn test_convexity_line() {
        // Points on a line
        let n = 12;
        let d = 2;
        let data: Vec<f64> = (0..n).flat_map(|i| vec![i as f64, 0.0]).collect();
        let shared = SharedData::compute(&data, n, d);
        let result = ConvexityLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_curvature"));
    }
}
