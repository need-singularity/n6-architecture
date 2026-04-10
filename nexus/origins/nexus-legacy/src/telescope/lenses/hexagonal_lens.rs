use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// HexagonalLens: Detect hexagonal/6-fold symmetric patterns in data.
///
/// Checks for n=6 geometric structure by analyzing angular distributions
/// of nearest neighbors and looking for 60-degree periodicity (BT-122).
pub struct HexagonalLens;

impl Lens for HexagonalLens {
    fn name(&self) -> &str { "HexagonalLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 { return HashMap::new(); }

        let k = 6.min(n - 1); // Look for 6 nearest neighbors (hexagonal)

        let mut hex_scores = Vec::with_capacity(n);
        let mut angle_60_counts = Vec::new();

        let nn = n.min(50);
        for i in 0..nn {
            // Find k nearest neighbors
            let mut neighbors: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, shared.dist(i, j)))
                .collect();
            neighbors.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let nn_k = neighbors.iter().take(k).map(|&(j, _)| j).collect::<Vec<_>>();

            if nn_k.len() < 3 { hex_scores.push(0.0); continue; }

            // Compute angles between consecutive NN vectors (in 2D projection)
            let mut angles = Vec::new();
            for &j in &nn_k {
                let dx = data[j * d] - data[i * d];
                let dy = if d > 1 { data[j * d + 1] - data[i * d + 1] } else { 0.0 };
                angles.push(dy.atan2(dx));
            }
            angles.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

            // Check for uniform 60-degree spacing (hexagonal)
            let target_spacing = std::f64::consts::PI / 3.0; // 60 degrees
            let mut hex_score = 0.0;
            let mut count_60 = 0;

            if angles.len() >= 2 {
                for w in angles.windows(2) {
                    let diff = w[1] - w[0];
                    let deviation = (diff - target_spacing).abs();
                    if deviation < 0.3 { // Within ~17 degrees of 60
                        count_60 += 1;
                    }
                }
                // Also check wrap-around
                let wrap_diff = (angles[0] + 2.0 * std::f64::consts::PI) - angles[angles.len() - 1];
                if (wrap_diff - target_spacing).abs() < 0.3 { count_60 += 1; }

                hex_score = count_60 as f64 / nn_k.len() as f64;
            }

            hex_scores.push(hex_score);
            angle_60_counts.push(count_60 as f64);
        }

        let mean_hex_score = hex_scores.iter().sum::<f64>() / hex_scores.len().max(1) as f64;

        // Check for CN=6 (hexagonal coordination number)
        let mean_angle_60 = angle_60_counts.iter().sum::<f64>() / angle_60_counts.len().max(1) as f64;

        let mut result = HashMap::new();
        result.insert("hex_scores".to_string(), hex_scores);
        result.insert("mean_hex_score".to_string(), vec![mean_hex_score]);
        result.insert("mean_60deg_count".to_string(), vec![mean_angle_60]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hexagonal_lens_hex_grid() {
        // Create a hexagonal grid: center + 6 vertices
        let r = 1.0;
        let mut data = vec![0.0, 0.0]; // center
        for k in 0..6 {
            let angle = k as f64 * std::f64::consts::PI / 3.0;
            data.push(r * angle.cos());
            data.push(r * angle.sin());
        }
        let n = 7;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = HexagonalLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("mean_hex_score"));
        // The center point should show good hexagonal pattern
        assert!(result["hex_scores"][0] > 0.0, "Center of hex grid should detect hex pattern");
    }
}
