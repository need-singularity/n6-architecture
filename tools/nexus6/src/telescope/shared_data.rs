use crate::gpu::fallback;

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
    pub fn mutual_info(&self, di: usize, dj: usize) -> f64 {
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
    // i*(i-1)/2 + j = idx, find i such that i*(i-1)/2 <= idx
    let i = ((1.0 + (1.0 + 8.0 * idx as f64).sqrt()) / 2.0).floor() as usize;
    let j = idx - i * (i - 1) / 2;
    (i, j)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flat_to_pair() {
        // For n=4, pairs: (1,0), (2,0), (2,1), (3,0), (3,1), (3,2)
        assert_eq!(flat_to_pair(0, 4), (1, 0));
        assert_eq!(flat_to_pair(1, 4), (2, 0));
        assert_eq!(flat_to_pair(2, 4), (2, 1));
        assert_eq!(flat_to_pair(3, 4), (3, 0));
        assert_eq!(flat_to_pair(4, 4), (3, 1));
        assert_eq!(flat_to_pair(5, 4), (3, 2));
    }

    #[test]
    fn test_shared_data_distance() {
        // 3 points in 2D: (0,0), (3,4), (6,0) -> dist(1,0)=5, dist(2,0)=6, dist(2,1)=5
        let data = vec![0.0, 0.0, 3.0, 4.0, 6.0, 0.0];
        let sd = SharedData::compute(&data, 3, 2);
        assert!((sd.dist(1, 0) - 5.0).abs() < 0.01);
        assert!((sd.dist(2, 0) - 6.0).abs() < 0.01);
        assert!((sd.dist(2, 1) - 5.0).abs() < 0.1);
    }

    #[test]
    fn test_shared_data_knn() {
        // 4 points: (0,0), (1,0), (2,0), (10,0) -> knn_k = ceil(sqrt(4)) = 2
        let data = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 0.0];
        let sd = SharedData::compute(&data, 4, 2);
        assert_eq!(sd.knn_k, 2);
        // Point 0's nearest should be point 1 (dist=1)
        let knn0 = sd.knn(0);
        assert_eq!(knn0[0], 1); // nearest neighbor of point 0 is point 1
    }

    #[test]
    fn test_shared_data_knn_density() {
        let data = vec![0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 10.0, 0.0];
        let sd = SharedData::compute(&data, 4, 2);
        // Point 3 (at x=10) is far from others, should have low density
        let d0 = sd.knn_density(0);
        let d3 = sd.knn_density(3);
        assert!(d0 > d3, "point 0 should be denser than isolated point 3");
    }

    #[test]
    fn test_shared_data_mi_matrix_shape() {
        let data = vec![0.0, 1.0, 1.0, 0.0, 2.0, 3.0];
        let sd = SharedData::compute(&data, 3, 2);
        assert_eq!(sd.mi_matrix.len(), 4); // 2x2 = 4
    }
}
