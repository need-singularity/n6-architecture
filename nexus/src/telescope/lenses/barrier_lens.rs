use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// BarrierLens: find energy barriers between clusters.
///
/// Algorithm:
///   1. Run simple k-means for k=2..5, pick best silhouette
///   2. Barrier height = min inter-cluster distance / mean intra-cluster distance
///   3. Returns barrier heights and cluster pair indices
pub struct BarrierLens;

impl Lens for BarrierLens {
    fn name(&self) -> &str {
        "BarrierLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        let max_k = 5.min(n);
        let mut best_k = 2;
        let mut best_silhouette = f64::NEG_INFINITY;
        let mut best_assignments = vec![0usize; n];

        for k in 2..=max_k {
            let assignments = kmeans(data, n, d, k, 20);
            let sil = silhouette_score(&assignments, n, k, shared);
            if sil > best_silhouette {
                best_silhouette = sil;
                best_k = k;
                best_assignments = assignments;
            }
        }

        // Compute barriers between all cluster pairs
        let mut barrier_heights = Vec::new();
        let mut barrier_pairs = Vec::new();

        for ca in 0..best_k {
            for cb in (ca + 1)..best_k {
                let members_a: Vec<usize> = (0..n)
                    .filter(|&i| best_assignments[i] == ca)
                    .collect();
                let members_b: Vec<usize> = (0..n)
                    .filter(|&i| best_assignments[i] == cb)
                    .collect();

                if members_a.is_empty() || members_b.is_empty() {
                    continue;
                }

                // Min inter-cluster distance
                let mut min_inter = f64::MAX;
                for &ia in &members_a {
                    for &ib in &members_b {
                        let d = shared.dist(ia, ib);
                        if d < min_inter {
                            min_inter = d;
                        }
                    }
                }

                // Mean intra-cluster distance for both clusters
                let intra_a = mean_intra_distance(&members_a, shared);
                let intra_b = mean_intra_distance(&members_b, shared);
                let mean_intra = (intra_a + intra_b) / 2.0;

                let barrier = if mean_intra > 0.0 {
                    min_inter / mean_intra
                } else {
                    min_inter
                };

                barrier_heights.push(barrier);
                barrier_pairs.push(ca as f64);
                barrier_pairs.push(cb as f64);
            }
        }

        let mut result = HashMap::new();
        result.insert("barrier_heights".to_string(), barrier_heights);
        result.insert("barrier_pairs".to_string(), barrier_pairs);
        result
    }
}

/// Simple k-means clustering.
fn kmeans(data: &[f64], n: usize, d: usize, k: usize, max_iter: usize) -> Vec<usize> {
    // Initialize centroids: pick first k points (deterministic)
    let mut centroids: Vec<Vec<f64>> = (0..k)
        .map(|i| {
            let idx = i * n / k; // spread across data
            data[idx * d..(idx + 1) * d].to_vec()
        })
        .collect();

    let mut assignments = vec![0usize; n];

    for _ in 0..max_iter {
        let mut changed = false;

        // Assign each point to nearest centroid
        for i in 0..n {
            let point = &data[i * d..(i + 1) * d];
            let mut best_c = 0;
            let mut best_dist = f64::MAX;
            for (c, centroid) in centroids.iter().enumerate() {
                let dist: f64 = point
                    .iter()
                    .zip(centroid.iter())
                    .map(|(a, b)| (a - b) * (a - b))
                    .sum();
                if dist < best_dist {
                    best_dist = dist;
                    best_c = c;
                }
            }
            if assignments[i] != best_c {
                assignments[i] = best_c;
                changed = true;
            }
        }

        if !changed {
            break;
        }

        // Update centroids
        for c in 0..k {
            let members: Vec<usize> = (0..n).filter(|&i| assignments[i] == c).collect();
            if members.is_empty() {
                continue;
            }
            for dim in 0..d {
                centroids[c][dim] =
                    members.iter().map(|&i| data[i * d + dim]).sum::<f64>() / members.len() as f64;
            }
        }
    }

    assignments
}

/// Compute silhouette score for a clustering.
fn silhouette_score(
    assignments: &[usize],
    n: usize,
    k: usize,
    shared: &SharedData,
) -> f64 {
    if k < 2 || n < 2 {
        return -1.0;
    }

    let mut total_sil = 0.0;
    let mut count = 0;

    for i in 0..n {
        let ci = assignments[i];

        // a(i) = mean distance to same-cluster points
        let same: Vec<usize> = (0..n)
            .filter(|&j| j != i && assignments[j] == ci)
            .collect();
        if same.is_empty() {
            continue;
        }
        let a_i = same.iter().map(|&j| shared.dist(i, j)).sum::<f64>() / same.len() as f64;

        // b(i) = min over other clusters of mean distance
        let mut b_i = f64::MAX;
        for ck in 0..k {
            if ck == ci {
                continue;
            }
            let others: Vec<usize> = (0..n).filter(|&j| assignments[j] == ck).collect();
            if others.is_empty() {
                continue;
            }
            let mean_dist =
                others.iter().map(|&j| shared.dist(i, j)).sum::<f64>() / others.len() as f64;
            if mean_dist < b_i {
                b_i = mean_dist;
            }
        }

        if b_i == f64::MAX {
            continue;
        }

        let s_i = (b_i - a_i) / a_i.max(b_i);
        total_sil += s_i;
        count += 1;
    }

    if count > 0 {
        total_sil / count as f64
    } else {
        -1.0
    }
}

/// Mean pairwise distance within a set of points.
fn mean_intra_distance(members: &[usize], shared: &SharedData) -> f64 {
    if members.len() < 2 {
        return 0.0;
    }
    let mut sum = 0.0;
    let mut count = 0;
    for i in 0..members.len() {
        for j in (i + 1)..members.len() {
            sum += shared.dist(members[i], members[j]);
            count += 1;
        }
    }
    if count > 0 {
        sum / count as f64
    } else {
        0.0
    }
}
