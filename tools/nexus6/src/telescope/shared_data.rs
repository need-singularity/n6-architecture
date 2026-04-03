use crate::gpu::fallback;

// ---------------------------------------------------------------------------
// Hot-path utility functions (migrated from telescope-rs common.rs)
// ---------------------------------------------------------------------------

/// Min and max of a slice. Returns (min, max).
#[inline]
pub fn min_max(s: &[f64]) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for &v in s {
        if v < lo { lo = v; }
        if v > hi { hi = v; }
    }
    (lo, hi)
}

/// Min and max of column j in row-major data (n_samples x n_features).
#[inline]
pub fn col_min_max(data: &[f64], n_samples: usize, n_features: usize, j: usize) -> (f64, f64) {
    let mut lo = f64::INFINITY;
    let mut hi = f64::NEG_INFINITY;
    for i in 0..n_samples {
        let v = data[i * n_features + j];
        if v < lo { lo = v; }
        if v > hi { hi = v; }
    }
    (lo, hi)
}

/// Extract column vectors from row-major data. Returns Vec of column Vecs.
pub fn column_vectors(data: &[f64], n_samples: usize, n_features: usize) -> Vec<Vec<f64>> {
    (0..n_features)
        .map(|j| {
            let mut col = Vec::with_capacity(n_samples);
            for i in 0..n_samples {
                col.push(data[i * n_features + j]);
            }
            col
        })
        .collect()
}

/// Compute mean and variance per feature. Returns (means, vars).
pub fn mean_var(data: &[f64], n_samples: usize, n_features: usize) -> (Vec<f64>, Vec<f64>) {
    let mut means = vec![0.0f64; n_features];
    for i in 0..n_samples {
        let row = i * n_features;
        for j in 0..n_features {
            means[j] += data[row + j];
        }
    }
    let inv_n = 1.0 / n_samples as f64;
    for j in 0..n_features {
        means[j] *= inv_n;
    }

    let mut vars = vec![0.0f64; n_features];
    for i in 0..n_samples {
        let row = i * n_features;
        for j in 0..n_features {
            let d = data[row + j] - means[j];
            vars[j] += d * d;
        }
    }
    for j in 0..n_features {
        vars[j] *= inv_n;
    }
    (means, vars)
}

/// Z-score normalize data into output buffer.
/// Requires pre-computed means and stds (sqrt of vars).
pub fn normalize_zscore_into(
    data: &[f64],
    n_samples: usize,
    n_features: usize,
    means: &[f64],
    stds: &[f64],
    out: &mut [f64],
) {
    for i in 0..n_samples {
        let row = i * n_features;
        for j in 0..n_features {
            out[row + j] = (data[row + j] - means[j]) / stds[j];
        }
    }
}

/// Shannon entropy of a slice via histogram binning.
pub fn shannon_entropy(values: &[f64], n_bins: usize) -> f64 {
    if values.is_empty() || n_bins < 2 {
        return 0.0;
    }
    let (lo, hi) = min_max(values);
    let range = (hi - lo).max(1e-12);
    let scale = (n_bins - 1) as f64 / range;

    let mut counts = vec![0u32; n_bins];
    for &v in values {
        let bin = ((v - lo) * scale) as usize;
        counts[bin.min(n_bins - 1)] += 1;
    }

    let inv_n = 1.0 / values.len() as f64;
    let mut entropy = 0.0;
    for &c in &counts {
        if c > 0 {
            let p = c as f64 * inv_n;
            entropy -= p * p.ln();
        }
    }
    entropy
}

/// K-nearest neighbor indices for each point (using squared distances).
/// Uses partial sort for efficiency when k << n.
pub fn knn_indices_from_data(data: &[f64], n_samples: usize, n_features: usize, k: usize) -> Vec<Vec<usize>> {
    let k = k.min(n_samples.saturating_sub(1));
    if k == 0 {
        return vec![vec![]; n_samples];
    }
    (0..n_samples)
        .map(|i| {
            let mut dists: Vec<(f64, usize)> = (0..n_samples)
                .filter(|&j| j != i)
                .map(|j| (dist_sq(data, i, j, n_features), j))
                .collect();
            let nth = k.min(dists.len().saturating_sub(1));
            dists.select_nth_unstable_by(nth, |a, b| {
                a.0.partial_cmp(&b.0).unwrap()
            });
            dists[..k].iter().map(|&(_, j)| j).collect()
        })
        .collect()
}

/// Squared Euclidean distance between row i and row j in row-major data.
#[inline(always)]
pub fn dist_sq(data: &[f64], i: usize, j: usize, n_features: usize) -> f64 {
    let ri = i * n_features;
    let rj = j * n_features;
    let mut sum = 0.0;
    let chunks = n_features / 4;
    let rem = n_features % 4;
    for c in 0..chunks {
        let base = c * 4;
        let d0 = data[ri + base] - data[rj + base];
        let d1 = data[ri + base + 1] - data[rj + base + 1];
        let d2 = data[ri + base + 2] - data[rj + base + 2];
        let d3 = data[ri + base + 3] - data[rj + base + 3];
        sum += d0 * d0 + d1 * d1 + d2 * d2 + d3 * d3;
    }
    let base = chunks * 4;
    for k in 0..rem {
        let d = data[ri + base + k] - data[rj + base + k];
        sum += d * d;
    }
    sum
}

/// Dot product of two slices.
#[inline(always)]
pub fn dot(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    let mut sum = 0.0;
    let chunks = n / 4;
    let rem = n % 4;
    for c in 0..chunks {
        let base = c * 4;
        sum += a[base] * b[base]
            + a[base + 1] * b[base + 1]
            + a[base + 2] * b[base + 2]
            + a[base + 3] * b[base + 3];
    }
    let base = chunks * 4;
    for k in 0..rem {
        sum += a[base + k] * b[base + k];
    }
    sum
}

/// Norm (L2) of a slice.
#[inline(always)]
pub fn norm(a: &[f64]) -> f64 {
    dot(a, a).sqrt()
}

/// Cosine similarity between two slices.
#[inline]
pub fn cosine_sim(a: &[f64], b: &[f64]) -> f64 {
    let d = dot(a, b);
    let na = norm(a);
    let nb = norm(b);
    if na < 1e-12 || nb < 1e-12 {
        return 0.0;
    }
    d / (na * nb)
}

/// Compute mutual information between two f64 slices via binned histogram.
pub fn mutual_info(a: &[f64], b: &[f64], n_bins: usize) -> f64 {
    let n = a.len().min(b.len());
    if n < 2 || n_bins < 2 {
        return 0.0;
    }
    let n = n.min(32);

    let (a_min, a_max) = min_max(&a[..n]);
    let (b_min, b_max) = min_max(&b[..n]);

    let a_range = a_max - a_min + 1e-12;
    let b_range = b_max - b_min + 1e-12;
    let a_scale = n_bins as f64 / a_range;
    let b_scale = n_bins as f64 / b_range;

    let mut joint = vec![0u32; n_bins * n_bins];
    for i in 0..n {
        let ai = ((a[i] - a_min) * a_scale) as usize;
        let bi = ((b[i] - b_min) * b_scale) as usize;
        let ai = ai.min(n_bins - 1);
        let bi = bi.min(n_bins - 1);
        joint[ai * n_bins + bi] += 1;
    }

    let mut pa = vec![0u32; n_bins];
    let mut pb = vec![0u32; n_bins];
    for ai in 0..n_bins {
        for bi in 0..n_bins {
            let c = joint[ai * n_bins + bi];
            pa[ai] += c;
            pb[bi] += c;
        }
    }

    let n_f = n as f64;
    let mut mi = 0.0;
    for ai in 0..n_bins {
        if pa[ai] == 0 { continue; }
        let p_a = pa[ai] as f64 / n_f;
        for bi in 0..n_bins {
            let c = joint[ai * n_bins + bi];
            if c == 0 || pb[bi] == 0 { continue; }
            let p_ab = c as f64 / n_f;
            let p_b = pb[bi] as f64 / n_f;
            mi += p_ab * (p_ab / (p_a * p_b)).ln();
        }
    }
    mi.max(0.0)
}

/// Compute pairwise MI matrix for an (n_rows, n_cols) matrix.
/// Returns (flat MI values, average MI).
pub fn pairwise_mi(data: &[f64], n_rows: usize, n_cols: usize, n_bins: usize, max_pairs: usize) -> (Vec<f64>, f64) {
    if n_rows < 2 {
        return (vec![], 0.0);
    }

    use std::collections::HashSet;
    let mut pairs: HashSet<(usize, usize)> = HashSet::with_capacity(max_pairs);
    for i in 0..n_rows {
        pairs.insert((i, (i + 1) % n_rows));
    }

    let max_possible = n_rows * (n_rows - 1) / 2;
    let target = max_pairs.min(max_possible);
    'outer: for i in 0..n_rows {
        for j in (i + 1)..n_rows {
            if pairs.len() >= target { break 'outer; }
            pairs.insert((i, j));
        }
    }

    let pairs_vec: Vec<(usize, usize)> = pairs.into_iter().collect();

    let mi_vals: Vec<f64> = pairs_vec
        .iter()
        .map(|&(i, j)| {
            let row_i = &data[i * n_cols..(i + 1) * n_cols];
            let row_j = &data[j * n_cols..(j + 1) * n_cols];
            mutual_info(row_i, row_j, n_bins)
        })
        .collect();

    let avg = if mi_vals.is_empty() {
        0.0
    } else {
        mi_vals.iter().sum::<f64>() / mi_vals.len() as f64
    };

    (mi_vals, avg)
}

// ---------------------------------------------------------------------------
// SharedData — pre-computed data for all lenses
// ---------------------------------------------------------------------------

/// Pre-computed shared data that all lenses can use.
/// The distance matrix is computed once and shared read-only.
/// KNN indices and mutual information are lazily available via GPU fallback.
pub struct SharedData {
    /// Lower-triangle pairwise Euclidean distances, length = N*(N-1)/2
    pub distance_matrix: Vec<f64>,
    /// Pre-computed k-NN indices per point (k = sqrt(N)), length = N * k.
    /// Index into this as: knn_indices[i * knn_k + j] = j-th nearest neighbor of point i.
    pub knn_indices: Vec<u32>,
    /// The k used for knn_indices.
    pub knn_k: usize,
    /// Mutual information matrix (D x D), computed via histogram binning.
    /// mi_matrix[di * d + dj] = MI(dim_di, dim_dj).
    pub mi_matrix: Vec<f64>,
    /// Number of data points
    pub n: usize,
    /// Dimensionality of each point
    pub d: usize,
}

impl SharedData {
    /// Compute the shared data from raw row-major data.
    /// `n` = number of points, `d` = dimensions per point.
    ///
    /// Uses gpu::fallback functions (rayon-parallel CPU) for distance matrix,
    /// KNN, and mutual information. These will be replaced by Metal GPU kernels
    /// when available.
    pub fn compute(data: &[f64], n: usize, d: usize) -> Self {
        assert_eq!(data.len(), n * d, "data length must equal n*d");

        // Convert f64 -> f32 for GPU/fallback pipeline
        let data_f32: Vec<f32> = data.iter().map(|&x| x as f32).collect();

        // 1. Distance matrix via GPU fallback (f32, rayon-parallel)
        let dist_f32 = fallback::distance_matrix_cpu(&data_f32, n as u32, d as u32);

        // Convert back to f64 for lens compatibility
        let distance_matrix: Vec<f64> = dist_f32.iter().map(|&x| x as f64).collect();

        // 2. KNN indices via GPU fallback
        let k = ((n as f64).sqrt().ceil() as usize).max(1).min(n.saturating_sub(1)).max(1);
        let knn_indices = if n > 1 {
            fallback::knn_cpu(&dist_f32, n as u32, k as u32)
        } else {
            Vec::new()
        };

        // 3. Mutual information matrix via GPU fallback
        let n_bins = 10u32; // standard bin count for MI estimation
        let mi_f32 = if n >= 2 && d >= 2 {
            fallback::mutual_info_cpu(&data_f32, n as u32, d as u32, n_bins)
        } else {
            vec![0.0f32; d * d]
        };
        let mi_matrix: Vec<f64> = mi_f32.iter().map(|&x| x as f64).collect();

        SharedData {
            distance_matrix,
            knn_indices,
            knn_k: k,
            mi_matrix,
            n,
            d,
        }
    }

    /// Get distance between point i and point j.
    /// Panics if i == j or indices out of range.
    pub fn dist(&self, i: usize, j: usize) -> f64 {
        assert_ne!(i, j, "distance to self is zero, use 0.0 directly");
        let (big, small) = if i > j { (i, j) } else { (j, i) };
        let idx = big * (big - 1) / 2 + small;
        self.distance_matrix[idx]
    }

    /// Get the pre-computed k-nearest neighbor indices for point `i`.
    /// Returns a slice of length `knn_k`.
    pub fn knn(&self, i: usize) -> &[u32] {
        let start = i * self.knn_k;
        let end = start + self.knn_k;
        &self.knn_indices[start..end]
    }

    /// Get mutual information between dimension `di` and dimension `dj`.
    pub fn mi(&self, di: usize, dj: usize) -> f64 {
        self.mi_matrix[di * self.d + dj]
    }

    /// Get the k-NN density for point `i` (inverse of distance to k-th neighbor).
    pub fn knn_density(&self, i: usize) -> f64 {
        if self.knn_k == 0 || self.knn_indices.is_empty() {
            return 0.0;
        }
        // The last neighbor in the KNN list is the k-th nearest
        let k_neighbor = self.knn_indices[i * self.knn_k + self.knn_k - 1] as usize;
        let d = self.dist(i, k_neighbor);
        if d > 0.0 { 1.0 / d } else { f64::MAX }
    }
}

/// Convert a flat lower-triangle index to (i, j) pair where i > j.
#[cfg(test)]
fn flat_to_pair(idx: usize, _n: usize) -> (usize, usize) {
    let i = ((1.0 + (1.0 + 8.0 * idx as f64).sqrt()) / 2.0).floor() as usize;
    let j = idx - i * (i - 1) / 2;
    (i, j)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_to_pair() {
        assert_eq!(flat_to_pair(0, 4), (1, 0));
        assert_eq!(flat_to_pair(1, 4), (2, 0));
        assert_eq!(flat_to_pair(2, 4), (2, 1));
        assert_eq!(flat_to_pair(3, 4), (3, 0));
        assert_eq!(flat_to_pair(4, 4), (3, 1));
        assert_eq!(flat_to_pair(5, 4), (3, 2));
    }

    #[test]
    fn test_shared_data_distance() {
        let data = vec![0.0, 0.0, 3.0, 4.0, 6.0, 0.0];
        let sd = SharedData::compute(&data, 3, 2);
        assert!((sd.dist(1, 0) - 5.0).abs() < 0.01);
        assert!((sd.dist(2, 0) - 6.0).abs() < 0.01);
        assert!((sd.dist(2, 1) - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_shared_data_knn() {
        let data = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 0.0];
        let sd = SharedData::compute(&data, 4, 2);
        assert_eq!(sd.knn_k, 2);
        let knn0 = sd.knn(0);
        assert_eq!(knn0[0], 1);
    }

    #[test]
    fn test_shared_data_knn_density() {
        let data = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 0.0];
        let sd = SharedData::compute(&data, 4, 2);
        let d0 = sd.knn_density(0);
        let d3 = sd.knn_density(3);
        assert!(d0 > d3, "point 0 should be denser than isolated point 3");
    }

    #[test]
    fn test_shared_data_mi_matrix_shape() {
        let data = vec![0.0, 1.0, 1.0, 0.0, 2.0, 3.0];
        let sd = SharedData::compute(&data, 3, 2);
        assert_eq!(sd.mi_matrix.len(), 4);
    }

    // --- Utility function tests (migrated from telescope-rs common.rs) ---

    #[test]
    fn test_min_max() {
        let v = vec![3.0, 1.0, 4.0, 1.0, 5.0];
        let (lo, hi) = min_max(&v);
        assert!((lo - 1.0).abs() < 1e-12);
        assert!((hi - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_col_min_max() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let (lo, hi) = col_min_max(&data, 2, 3, 0);
        assert!((lo - 1.0).abs() < 1e-12);
        assert!((hi - 4.0).abs() < 1e-12);
        let (lo2, hi2) = col_min_max(&data, 2, 3, 2);
        assert!((lo2 - 3.0).abs() < 1e-12);
        assert!((hi2 - 6.0).abs() < 1e-12);
    }

    #[test]
    fn test_column_vectors() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let cols = column_vectors(&data, 2, 2);
        assert_eq!(cols.len(), 2);
        assert_eq!(cols[0], vec![1.0, 3.0]);
        assert_eq!(cols[1], vec![2.0, 4.0]);
    }

    #[test]
    fn test_mean_var() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let (means, vars) = mean_var(&data, 2, 2);
        assert!((means[0] - 2.0).abs() < 1e-12);
        assert!((means[1] - 3.0).abs() < 1e-12);
        assert!((vars[0] - 1.0).abs() < 1e-12);
        assert!((vars[1] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_normalize_zscore_into() {
        let data = vec![1.0, 2.0, 3.0, 4.0];
        let means = vec![2.0, 3.0];
        let stds = vec![1.0, 1.0];
        let mut out = vec![0.0; 4];
        normalize_zscore_into(&data, 2, 2, &means, &stds, &mut out);
        assert!((out[0] - (-1.0)).abs() < 1e-12);
        assert!((out[1] - (-1.0)).abs() < 1e-12);
        assert!((out[2] - 1.0).abs() < 1e-12);
        assert!((out[3] - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_shannon_entropy_uniform() {
        let v: Vec<f64> = (0..400).map(|i| (i % 4) as f64).collect();
        let e = shannon_entropy(&v, 4);
        assert!((e - 4.0_f64.ln()).abs() < 0.1);
    }

    #[test]
    fn test_dist_sq() {
        let data = vec![1.0, 0.0, 0.0, 1.0];
        assert!((dist_sq(&data, 0, 1, 2) - 2.0).abs() < 1e-12);
    }

    #[test]
    fn test_dist_sq_4way() {
        let data = vec![
            1.0, 2.0, 3.0, 4.0, 5.0,
            2.0, 3.0, 4.0, 5.0, 6.0,
        ];
        let d = dist_sq(&data, 0, 1, 5);
        assert!((d - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_dot_product() {
        let a = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        let b = vec![5.0, 4.0, 3.0, 2.0, 1.0];
        assert!((dot(&a, &b) - 35.0).abs() < 1e-12);
    }

    #[test]
    fn test_cosine_sim_identical() {
        let a = vec![1.0, 2.0, 3.0];
        assert!((cosine_sim(&a, &a) - 1.0).abs() < 1e-12);
    }

    #[test]
    fn test_cosine_sim_orthogonal() {
        let a = vec![1.0, 0.0];
        let b = vec![0.0, 1.0];
        assert!(cosine_sim(&a, &b).abs() < 1e-12);
    }

    #[test]
    fn test_norm() {
        let a = vec![3.0, 4.0];
        assert!((norm(&a) - 5.0).abs() < 1e-12);
    }

    #[test]
    fn test_knn_indices_simple() {
        let data = vec![0.0, 1.0, 10.0];
        let knn = knn_indices_from_data(&data, 3, 1, 1);
        assert_eq!(knn.len(), 3);
        assert_eq!(knn[0], vec![1]);
        assert_eq!(knn[1], vec![0]);
        assert_eq!(knn[2], vec![1]);
    }

    #[test]
    fn test_mutual_info_identical() {
        let a = vec![0.0, 0.1, 0.2, 0.3, 0.4, 0.5, 0.6, 0.7, 0.8, 0.9];
        let mi = mutual_info(&a, &a, 16);
        assert!(mi > 0.0, "MI of identical vectors should be > 0, got {mi}");
    }

    #[test]
    fn test_mutual_info_independent() {
        let a: Vec<f64> = (0..100).map(|i| (i as f64 * 0.1).sin()).collect();
        let b: Vec<f64> = (0..100).map(|i| (i as f64 * 0.37 + 2.0).cos()).collect();
        let mi = mutual_info(&a, &b, 16);
        assert!(mi >= 0.0 && mi.is_finite(), "MI should be finite, got {mi}");
    }

    #[test]
    fn test_pairwise_mi() {
        let n = 8;
        let dim = 16;
        let data: Vec<f64> = (0..n * dim).map(|i| (i as f64 * 0.1).sin()).collect();
        let (vals, avg) = pairwise_mi(&data, n, dim, 16, 20);
        assert!(!vals.is_empty());
        assert!(avg >= 0.0);
    }
}
