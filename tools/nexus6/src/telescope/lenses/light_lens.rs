use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// LightLens: Light-like propagation in data.
/// Measures propagation speed, null geodesics, shadow fraction, luminosity.
pub struct LightLens;

impl Lens for LightLens {
    fn name(&self) -> &str { "LightLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 { return HashMap::new(); }
        let max_n = n.min(200);

        // Propagation speed: how fast influence spreads through kNN graph
        // BFS-like: count average hops to reach all points from point 0
        let mut visited = vec![false; max_n];
        let mut queue = std::collections::VecDeque::new();
        visited[0] = true;
        queue.push_back((0usize, 0usize)); // (node, depth)
        let mut total_depth = 0usize;
        let mut reached = 1usize;
        while let Some((node, depth)) = queue.pop_front() {
            for &j in shared.knn(node).iter() {
                let j = j as usize;
                if j < max_n && !visited[j] {
                    visited[j] = true;
                    queue.push_back((j, depth + 1));
                    total_depth += depth + 1;
                    reached += 1;
                }
            }
        }
        let avg_hops = if reached > 1 { total_depth as f64 / (reached - 1) as f64 } else { max_n as f64 };
        let propagation_speed = 1.0 / avg_hops.max(1e-12);

        // Null geodesic score: how "straight" are the shortest paths
        // Compare kNN distance sum with direct distance for random triplets
        let mut geodesic_sum = 0.0f64;
        let mut direct_sum = 0.0f64;
        let triplets = max_n.min(50);
        for i in 0..triplets {
            let a = i;
            let b = (i + triplets / 3) % max_n;
            let c = (i + 2 * triplets / 3) % max_n;
            if a != b && b != c && a != c {
                let ab = shared.dist(a, b);
                let bc = shared.dist(b, c);
                let ac = shared.dist(a, c);
                geodesic_sum += ab + bc;
                direct_sum += ac;
            }
        }
        let null_geodesic_score = if geodesic_sum > 1e-12 {
            (direct_sum / geodesic_sum).min(1.0)
        } else {
            0.0
        };

        // Shadow fraction: fraction of points with below-median intensity
        let intensities: Vec<f64> = (0..max_n).map(|i| {
            data[i * d..(i * d + d)].iter().map(|x| x * x).sum::<f64>().sqrt()
        }).collect();
        let mut sorted_int = intensities.clone();
        sorted_int.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_int = sorted_int[max_n / 2];
        let shadow_threshold = median_int * 0.3;
        let shadow_count = intensities.iter().filter(|&&x| x < shadow_threshold).count();
        let shadow_fraction = shadow_count as f64 / max_n as f64;

        // Luminosity: total intensity
        let luminosity = intensities.iter().sum::<f64>() / max_n as f64;

        let mut result = HashMap::new();
        result.insert("propagation_speed".to_string(), vec![propagation_speed]);
        result.insert("null_geodesic_score".to_string(), vec![null_geodesic_score]);
        result.insert("shadow_fraction".to_string(), vec![shadow_fraction]);
        result.insert("luminosity".to_string(), vec![luminosity]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_basic() {
        let data: Vec<f64> = (0..20).map(|i| (i as f64 * 0.3).sin()).collect();
        let shared = SharedData::compute(&data, 20, 1);
        let result = LightLens.scan(&data, 20, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["propagation_speed"][0] > 0.0);
        assert!(result["luminosity"][0] >= 0.0);
    }
}
