use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// FrustrationLens: Detect competing constraints that prevent global optimum.
///
/// Inspired by geometric frustration in condensed matter physics (e.g., spin
/// glasses, triangular antiferromagnets). When three points form a triangle,
/// we check whether their pairwise distance relationships are mutually
/// consistent — if two points are both "close" to a third but "far" from
/// each other, the triangle is frustrated.
///
/// Algorithm:
///   1. Compute median distance as threshold to classify edges as short/long.
///   2. For each triangle (i,j,k), count short edges. A triangle with exactly
///      2 short edges is frustrated (the two "close" pairs conflict with the
///      "far" pair — analogous to antiferromagnetic frustration on a triangle).
///   3. Compute frustration index = frustrated triangles / total triangles.
///   4. Compute per-point frustration = fraction of each point's triangles
///      that are frustrated, then report mean and std.
///   5. Compute frustration energy = sum of tension in frustrated triangles,
///      where tension = |d_long - d_short_mean| for each frustrated triangle.
pub struct UfrustrationLens;

impl Lens for UfrustrationLens {
    fn name(&self) -> &str {
        "UfrustrationLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // Collect all pairwise distances and find median
        let pair_count = n * (n - 1) / 2;
        let mut all_dists: Vec<f64> = Vec::with_capacity(pair_count);
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(shared.dist(i, j));
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median_dist = all_dists[pair_count / 2];

        // Classify each edge as short (true) or long (false)
        // short_edge[i][j] = true if dist(i,j) < median
        let mut short_edge: Vec<Vec<bool>> = vec![vec![false; n]; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let is_short = shared.dist(i, j) < median_dist;
                short_edge[i][j] = is_short;
                short_edge[j][i] = is_short;
            }
        }

        // Enumerate triangles (cap to avoid O(n^3) blow-up on large data)
        let max_n_tri = if n <= 80 { n } else { 80 };

        let mut total_triangles: u64 = 0;
        let mut frustrated_count: u64 = 0;
        let mut frustration_energy = 0.0_f64;
        let mut point_frustrated = vec![0u64; n];
        let mut point_triangles = vec![0u64; n];

        for i in 0..max_n_tri {
            for j in (i + 1)..max_n_tri {
                for k in (j + 1)..max_n_tri {
                    total_triangles += 1;

                    let s_ij = short_edge[i][j];
                    let s_ik = short_edge[i][k];
                    let s_jk = short_edge[j][k];
                    let short_count = s_ij as u8 + s_ik as u8 + s_jk as u8;

                    // A triangle with exactly 2 short edges is frustrated:
                    // two pairs want to be close, but the third is far,
                    // creating an irreconcilable tension.
                    let is_frustrated = short_count == 2;

                    point_triangles[i] += 1;
                    point_triangles[j] += 1;
                    point_triangles[k] += 1;

                    if is_frustrated {
                        frustrated_count += 1;
                        point_frustrated[i] += 1;
                        point_frustrated[j] += 1;
                        point_frustrated[k] += 1;

                        // Compute tension: |long_edge_dist - mean(short_edge_dists)|
                        let d_ij = shared.dist(i, j);
                        let d_ik = shared.dist(i, k);
                        let d_jk = shared.dist(j, k);

                        let (long_d, short_mean) = if !s_ij {
                            (d_ij, (d_ik + d_jk) / 2.0)
                        } else if !s_ik {
                            (d_ik, (d_ij + d_jk) / 2.0)
                        } else {
                            (d_jk, (d_ij + d_ik) / 2.0)
                        };
                        frustration_energy += (long_d - short_mean).abs();
                    }
                }
            }
        }

        if total_triangles == 0 {
            return HashMap::new();
        }

        let frustration_index = frustrated_count as f64 / total_triangles as f64;

        // Normalize energy by triangle count
        let norm_energy = if frustrated_count > 0 {
            frustration_energy / frustrated_count as f64
        } else {
            0.0
        };

        // Per-point frustration ratio
        let mut local_frust: Vec<f64> = Vec::with_capacity(max_n_tri);
        for i in 0..max_n_tri {
            if point_triangles[i] > 0 {
                local_frust.push(point_frustrated[i] as f64 / point_triangles[i] as f64);
            }
        }

        let (local_mean, local_std) = if !local_frust.is_empty() {
            let mean = local_frust.iter().sum::<f64>() / local_frust.len() as f64;
            let var = local_frust
                .iter()
                .map(|&x| (x - mean) * (x - mean))
                .sum::<f64>()
                / local_frust.len() as f64;
            (mean, var.sqrt())
        } else {
            (0.0, 0.0)
        };

        let mut result = HashMap::new();
        result.insert(
            "frustration_index".to_string(),
            vec![frustration_index],
        );
        result.insert(
            "frustrated_triangles".to_string(),
            vec![frustrated_count as f64, total_triangles as f64],
        );
        result.insert(
            "frustration_energy".to_string(),
            vec![norm_energy],
        );
        result.insert(
            "local_frustration".to_string(),
            vec![local_mean, local_std],
        );
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_frustration_basic_non_empty() {
        // 6 points in 2D forming a mix of close and far pairs
        let data = vec![
            0.0, 0.0,
            1.0, 0.0,
            0.5, 0.5,
            10.0, 10.0,
            11.0, 10.0,
            10.5, 10.5,
        ];
        let n = 6;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UfrustrationLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        assert!(result.contains_key("frustration_index"));
        assert!(result.contains_key("frustrated_triangles"));
        assert!(result.contains_key("frustration_energy"));
        assert!(result.contains_key("local_frustration"));

        let fi = result["frustration_index"][0];
        assert!(fi >= 0.0 && fi <= 1.0, "frustration_index must be in [0,1], got {fi}");

        let counts = &result["frustrated_triangles"];
        assert!(counts[1] > 0.0, "total triangles must be > 0");
    }

    #[test]
    fn test_frustration_cluster_separation() {
        // Two tight clusters far apart should produce frustration in cross-cluster triangles
        let mut data = Vec::new();
        // Cluster A: 4 points near origin
        for i in 0..4 {
            data.push(i as f64 * 0.1);
            data.push(0.0);
        }
        // Cluster B: 4 points far away
        for i in 0..4 {
            data.push(100.0 + i as f64 * 0.1);
            data.push(0.0);
        }
        let n = 8;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UfrustrationLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty());
        let fi = result["frustration_index"][0];
        // With two tight clusters, cross-cluster triangles (2 short intra + 1 long inter)
        // should create significant frustration
        assert!(fi > 0.0, "two-cluster layout should have frustration > 0, got {fi}");

        let local = &result["local_frustration"];
        assert!(local[0] > 0.0, "local mean frustration should be > 0");
    }

    #[test]
    fn test_frustration_too_few_points() {
        let data = vec![0.0, 1.0, 2.0, 3.0];
        let n = 2;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UfrustrationLens;
        let result = lens.scan(&data, n, d, &shared);
        assert!(result.is_empty(), "n<3 should return empty");
    }
}
