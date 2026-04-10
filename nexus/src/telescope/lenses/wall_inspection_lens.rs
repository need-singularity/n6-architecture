use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// WallInspectionLens: Impenetrable barrier detection + bypass route analysis.
///
/// "벽이 있다. 뚫고 지나가는 건 불가능. 하지만 터널을 파거나(웜홀),
///  공간을 접는 건(워프) 이론적으로 금지되지 않았다."
///
/// Detects wall-like structures in data manifold:
///   1. Wall detection: sharp density discontinuities separating regions
///   2. Wall thickness: how wide the no-man's-land between regions is
///   3. Wall impermeability: fraction of KNN edges blocked by the wall
///   4. Tunnel count: paths that cross through the wall via intermediate points
///   5. Fold proximity: distant regions brought close by manifold folding
///   6. bypass_score: overall feasibility of circumventing the wall
///
/// n=6: Light-speed barrier has Lorentz group dim=n=6. Perfect number
///       divisors {1,2,3,6} define the wall's internal structure.
///       Exotic matter (negative energy) needed for bypass ~ Casimir at 1/σ scale.
pub struct WallInspectionLens;

impl Lens for WallInspectionLens {
    fn name(&self) -> &str { "WallInspectionLens" }
    fn category(&self) -> &str { "T1" }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 10 { return HashMap::new(); }

        let max_n = n.min(200);

        // 1. Detect walls: find density gaps (regions with very low density
        //    flanked by high-density regions on both sides)
        let mut densities: Vec<(usize, f64)> = (0..max_n)
            .map(|i| (i, shared.knn_density(i)))
            .collect();
        densities.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

        let median_density = densities[densities.len() / 2].1;
        let low_threshold = median_density * 0.2;

        // Low-density points are "wall material" (the gap itself)
        let wall_points: Vec<usize> = densities.iter()
            .filter(|(_, d)| *d < low_threshold)
            .map(|(i, _)| *i)
            .collect();

        let wall_fraction = wall_points.len() as f64 / max_n as f64;

        // 2. Wall thickness: average distance between wall points
        let wall_thickness = if wall_points.len() >= 2 {
            let mut sum = 0.0f64;
            let mut count = 0u32;
            let max_pairs = 50;
            for i in 0..wall_points.len().min(max_pairs) {
                for j in (i + 1)..wall_points.len().min(max_pairs) {
                    sum += shared.dist(wall_points[i], wall_points[j]);
                    count += 1;
                }
            }
            if count > 0 { sum / count as f64 } else { 0.0 }
        } else {
            0.0
        };

        // 3. Wall impermeability: fraction of KNN connections that cross
        //    from high-density to high-density without going through wall
        let high_density: Vec<bool> = (0..max_n)
            .map(|i| shared.knn_density(i) >= median_density)
            .collect();

        let mut blocked_edges = 0u64;
        let mut total_cross_edges = 0u64;

        for i in 0..max_n {
            if !high_density[i] { continue; }
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let j = j as usize;
                if j >= max_n { continue; }
                if !high_density[j] { continue; }

                total_cross_edges += 1;
                // Check if the path i→j passes through a low-density zone
                // Approximate: is any wall point closer to the midpoint?
                let mid_dist = shared.dist(i, j) / 2.0;
                let crosses_wall = wall_points.iter().any(|&w| {
                    let d_iw = shared.dist(i, w);
                    let d_wj = shared.dist(w, j);
                    d_iw < mid_dist * 1.5 && d_wj < mid_dist * 1.5
                });
                if crosses_wall {
                    blocked_edges += 1;
                }
            }
        }

        let impermeability = if total_cross_edges > 0 {
            blocked_edges as f64 / total_cross_edges as f64
        } else {
            0.0
        };

        // 4. Tunnel detection: paths that traverse the wall via chain of points
        //    A tunnel = sequence of connected points crossing from one side to the other
        let mut tunnel_count = 0u32;
        for &wp in wall_points.iter().take(20) {
            let knn_w = shared.knn(wp);
            // Check if wall point connects two high-density regions
            let high_neighbors: Vec<usize> = knn_w.iter()
                .map(|&j| j as usize)
                .filter(|&j| j < max_n && high_density[j])
                .collect();

            if high_neighbors.len() >= 2 {
                // Check if any pair of high-density neighbors are far from each other
                // (indicating they're on opposite sides of the wall)
                for a in 0..high_neighbors.len() {
                    for b in (a + 1)..high_neighbors.len() {
                        let d_ab = shared.dist(high_neighbors[a], high_neighbors[b]);
                        let d_aw = shared.dist(high_neighbors[a], wp);
                        let d_bw = shared.dist(high_neighbors[b], wp);
                        // Triangle inequality violation hint: if going through wall point
                        // is much shorter than direct distance → tunnel
                        if d_ab > (d_aw + d_bw) * 0.8 {
                            tunnel_count += 1;
                        }
                    }
                }
            }
        }

        // 5. Fold proximity: pairs of high-density points that are far in
        //    Euclidean distance but connected through short KNN chains
        let mut fold_count = 0u32;
        let global_dists: Vec<f64> = {
            let mut d = Vec::new();
            let step = if max_n > 50 { max_n / 50 } else { 1 };
            let mut i = 0;
            while i < max_n {
                let mut j = i + 1;
                while j < max_n {
                    d.push(shared.dist(i, j));
                    j += step;
                }
                i += step;
            }
            d
        };
        if !global_dists.is_empty() {
            let mut sorted_dists = global_dists.clone();
            sorted_dists.sort_unstable_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            let far_threshold = sorted_dists[sorted_dists.len() * 3 / 4];

            // Check far points that share KNN neighbors (manifold fold)
            for i in 0..max_n.min(50) {
                let knn_i = shared.knn(i);
                for j in (i + 1)..max_n.min(50) {
                    if shared.dist(i, j) < far_threshold { continue; }
                    let knn_j = shared.knn(j);
                    let shared_nn = knn_i.iter()
                        .filter(|&&ni| knn_j.contains(&ni))
                        .count();
                    if shared_nn >= 1 {
                        fold_count += 1;
                    }
                }
            }
        }

        // 6. Bypass score: can the wall be circumvented?
        // High tunnels + high folds + low impermeability = bypassable
        let tunnel_norm = (tunnel_count as f64).min(20.0) / 20.0;
        let fold_norm = (fold_count as f64).min(20.0) / 20.0;
        let bypass_score = (tunnel_norm * 0.4 + fold_norm * 0.3 + (1.0 - impermeability) * 0.3)
            .clamp(0.0, 1.0);

        let mut result = HashMap::new();
        result.insert("wall_fraction".to_string(), vec![wall_fraction]);
        result.insert("wall_thickness".to_string(), vec![wall_thickness]);
        result.insert("impermeability".to_string(), vec![impermeability]);
        result.insert("tunnel_count".to_string(), vec![tunnel_count as f64]);
        result.insert("fold_count".to_string(), vec![fold_count as f64]);
        result.insert("bypass_score".to_string(), vec![bypass_score]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wall_inspection_two_clusters() {
        // Two dense clusters separated by empty gap = wall
        let mut data = Vec::new();
        for i in 0..15 {
            data.push(i as f64 * 0.1); data.push(0.0);
        }
        for i in 0..15 {
            data.push(100.0 + i as f64 * 0.1); data.push(0.0);
        }
        let n = 30;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let result = WallInspectionLens.scan(&data, n, d, &shared);
        assert!(result.contains_key("wall_fraction"));
        assert!(result.contains_key("impermeability"));
        assert!(result.contains_key("bypass_score"));
    }

    #[test]
    fn test_wall_inspection_continuous() {
        // Continuous data = no wall
        let n = 30;
        let data: Vec<f64> = (0..n * 2)
            .map(|i| (i as f64 * 0.1).sin())
            .collect();
        let shared = SharedData::compute(&data, n, 2);
        let result = WallInspectionLens.scan(&data, n, 2, &shared);
        // No significant wall in continuous data
        assert!(result["wall_fraction"][0] < 0.3);
    }
}
