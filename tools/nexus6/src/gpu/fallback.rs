use rayon::prelude::*;

/// Pairwise Euclidean distance matrix (lower triangle, row-major).
///
/// Given `n` points of dimension `d` packed in `data` (length n*d),
/// returns a Vec of length n*(n-1)/2 containing dist(i,j) for i>j.
/// Index mapping: flat_idx(i,j) = i*(i-1)/2 + j  (i > j).
pub fn distance_matrix_cpu(data: &[f32], n: u32, d: u32) -> Vec<f32> {
    let n = n as usize;
    let d = d as usize;
    assert_eq!(data.len(), n * d, "data length must equal n * d");

    let pair_count = n * (n - 1) / 2;
    let mut result = vec![0.0f32; pair_count];

    // Parallel over rows
    result
        .par_chunks_mut(1) // we'll index manually for better granularity
        .enumerate()
        .for_each(|(flat, slot)| {
            // Recover (i, j) from flat index: i*(i-1)/2 + j = flat
            let i = ((1.0 + (1.0 + 8.0 * flat as f64).sqrt()) / 2.0).floor() as usize;
            let j = flat - i * (i - 1) / 2;

            let row_i = &data[i * d..(i + 1) * d];
            let row_j = &data[j * d..(j + 1) * d];

            let sum_sq: f32 = row_i
                .iter()
                .zip(row_j.iter())
                .map(|(a, b)| (a - b) * (a - b))
                .sum();

            slot[0] = sum_sq.sqrt();
        });

    result
}

/// Mutual information matrix (D x D) via histogram binning.
///
/// `data` has `n` samples of `d` dimensions. Returns D*D MI matrix.
/// MI(X,Y) = sum_xy p(x,y) * log(p(x,y) / (p(x)*p(y)))
pub fn mutual_info_cpu(data: &[f32], n: u32, d: u32, n_bins: u32) -> Vec<f32> {
    let n = n as usize;
    let d = d as usize;
    let nb = n_bins as usize;

    // Precompute per-dimension min/max for binning
    let mut mins = vec![f32::INFINITY; d];
    let mut maxs = vec![f32::NEG_INFINITY; d];
    for i in 0..n {
        for j in 0..d {
            let v = data[i * d + j];
            if v < mins[j] {
                mins[j] = v;
            }
            if v > maxs[j] {
                maxs[j] = v;
            }
        }
    }

    // Bin assignment helper
    let bin_of = |val: f32, dim: usize| -> usize {
        let range = maxs[dim] - mins[dim];
        if range <= 0.0 {
            return 0;
        }
        let b = ((val - mins[dim]) / range * nb as f32) as usize;
        b.min(nb - 1)
    };

    // Precompute bin indices: bins[sample][dim]
    let bins: Vec<Vec<usize>> = (0..n)
        .map(|i| (0..d).map(|j| bin_of(data[i * d + j], j)).collect())
        .collect();

    // Compute MI for each dimension pair in parallel
    let mi: Vec<f32> = (0..d * d)
        .into_par_iter()
        .map(|idx| {
            let di = idx / d;
            let dj = idx % d;

            if di == dj {
                return 0.0; // Self-MI = entropy, but we return 0 for simplicity
            }

            // Joint histogram
            let mut joint = vec![0u32; nb * nb];
            for s in 0..n {
                let bi = bins[s][di];
                let bj = bins[s][dj];
                joint[bi * nb + bj] += 1;
            }

            // Marginals
            let mut margin_i = vec![0u32; nb];
            let mut margin_j = vec![0u32; nb];
            for bi in 0..nb {
                for bj in 0..nb {
                    let c = joint[bi * nb + bj];
                    margin_i[bi] += c;
                    margin_j[bj] += c;
                }
            }

            let n_f = n as f32;
            let mut mi_val = 0.0f32;
            for bi in 0..nb {
                let pi = margin_i[bi] as f32 / n_f;
                if pi <= 0.0 {
                    continue;
                }
                for bj in 0..nb {
                    let pj = margin_j[bj] as f32 / n_f;
                    let pij = joint[bi * nb + bj] as f32 / n_f;
                    if pij > 0.0 && pj > 0.0 {
                        mi_val += pij * (pij / (pi * pj)).ln();
                    }
                }
            }

            mi_val.max(0.0) // Clamp numerical noise
        })
        .collect();

    mi
}

/// K-nearest neighbor indices per sample from a lower-triangle distance matrix.
///
/// `dist` has n*(n-1)/2 entries (lower triangle). Returns n*k indices.
pub fn knn_cpu(dist: &[f32], n: u32, k: u32) -> Vec<u32> {
    let n = n as usize;
    let k = k as usize;
    assert!(k < n, "k must be less than n");

    let get_dist = |i: usize, j: usize| -> f32 {
        if i == j {
            return f32::INFINITY;
        }
        let (big, small) = if i > j { (i, j) } else { (j, i) };
        dist[big * (big - 1) / 2 + small]
    };

    (0..n)
        .into_par_iter()
        .flat_map_iter(|i| {
            // Collect distances to all other points
            let mut dists: Vec<(f32, u32)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (get_dist(i, j), j as u32))
                .collect();

            // Partial sort to get k nearest
            dists.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap_or(std::cmp::Ordering::Equal));
            dists.truncate(k);

            dists.into_iter().map(|(_, idx)| idx)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn distance_basic() {
        // 2 points: (0,0) and (3,4) -> distance = 5
        let data = vec![0.0, 0.0, 3.0, 4.0];
        let result = distance_matrix_cpu(&data, 2, 2);
        assert_eq!(result.len(), 1);
        assert!((result[0] - 5.0).abs() < 1e-5);
    }

    #[test]
    fn distance_single_point() {
        // 1 point -> 0 pairs
        let data = vec![1.0, 2.0, 3.0];
        let result = distance_matrix_cpu(&data, 1, 3);
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn distance_three_points() {
        // (0,0), (1,0), (0,1) -> d(1,0)=1, d(2,0)=1, d(2,1)=sqrt(2)
        let data = vec![0.0, 0.0, 1.0, 0.0, 0.0, 1.0];
        let result = distance_matrix_cpu(&data, 3, 2);
        assert_eq!(result.len(), 3);
        assert!((result[0] - 1.0).abs() < 1e-5); // d(1,0)
        assert!((result[1] - 1.0).abs() < 1e-5); // d(2,0)
        assert!((result[2] - std::f32::consts::SQRT_2).abs() < 1e-5); // d(2,1)
    }

    #[test]
    fn distance_identical_points() {
        let data = vec![5.0, 5.0, 5.0, 5.0];
        let result = distance_matrix_cpu(&data, 2, 2);
        assert_eq!(result.len(), 1);
        assert!((result[0]).abs() < 1e-5);
    }

    #[test]
    fn distance_1d() {
        // 3 points in 1D: 0, 3, 7
        let data = vec![0.0, 3.0, 7.0];
        let result = distance_matrix_cpu(&data, 3, 1);
        assert_eq!(result.len(), 3);
        assert!((result[0] - 3.0).abs() < 1e-5); // d(1,0)
        assert!((result[1] - 7.0).abs() < 1e-5); // d(2,0)
        assert!((result[2] - 4.0).abs() < 1e-5); // d(2,1)
    }

    #[test]
    #[should_panic(expected = "data length must equal n * d")]
    fn distance_bad_length() {
        distance_matrix_cpu(&[1.0, 2.0, 3.0], 2, 2);
    }

    #[test]
    fn mutual_info_independent_dims() {
        // Constant column -> MI with anything should be 0
        // dim0 varies, dim1 constant
        let mut data = Vec::new();
        for i in 0..100 {
            data.push(i as f32);
            data.push(42.0);
        }
        let mi = mutual_info_cpu(&data, 100, 2, 10);
        assert_eq!(mi.len(), 4); // 2x2
        assert_eq!(mi[0], 0.0); // self
        assert_eq!(mi[3], 0.0); // self
        // MI(0,1) and MI(1,0) should be ~0 since dim1 is constant
        assert!(mi[1].abs() < 1e-5);
        assert!(mi[2].abs() < 1e-5);
    }

    #[test]
    fn mutual_info_correlated_dims() {
        // dim0 = dim1 -> high MI
        let mut data = Vec::new();
        for i in 0..200 {
            let v = (i as f32) / 200.0;
            data.push(v);
            data.push(v);
        }
        let mi = mutual_info_cpu(&data, 200, 2, 10);
        assert!(mi[1] > 0.5, "correlated dims should have high MI, got {}", mi[1]);
        // Symmetry
        assert!((mi[1] - mi[2]).abs() < 1e-5);
    }

    #[test]
    fn mutual_info_single_bin() {
        // With 1 bin, everything lands in the same cell -> MI = 0
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let mi = mutual_info_cpu(&data, 3, 2, 1);
        for v in &mi {
            assert!(v.abs() < 1e-5);
        }
    }

    #[test]
    fn knn_basic() {
        // 3 points: (0,0), (1,0), (10,0). k=1
        // dists: d(1,0)=1, d(2,0)=10, d(2,1)=9
        let dist = vec![1.0, 10.0, 9.0];
        let neighbors = knn_cpu(&dist, 3, 1);
        assert_eq!(neighbors.len(), 3);
        assert_eq!(neighbors[0], 1); // point 0's nearest = point 1
        assert_eq!(neighbors[1], 0); // point 1's nearest = point 0
        assert_eq!(neighbors[2], 1); // point 2's nearest = point 1
    }

    #[test]
    fn knn_k2() {
        // 4 points, k=2
        // Flat lower triangle: d(1,0), d(2,0), d(2,1), d(3,0), d(3,1), d(3,2)
        let dist = vec![1.0, 5.0, 4.0, 2.0, 3.0, 6.0];
        let neighbors = knn_cpu(&dist, 4, 2);
        assert_eq!(neighbors.len(), 8); // 4 * 2
        // Point 0: dists to 1=1, 2=5, 3=2 -> nearest: 1, 3
        assert_eq!(neighbors[0], 1);
        assert_eq!(neighbors[1], 3);
        // Point 3: dists to 0=2, 1=3, 2=6 -> nearest: 0, 1
        assert_eq!(neighbors[6], 0);
        assert_eq!(neighbors[7], 1);
    }

    #[test]
    #[should_panic(expected = "k must be less than n")]
    fn knn_k_too_large() {
        knn_cpu(&[1.0], 2, 2);
    }
}
