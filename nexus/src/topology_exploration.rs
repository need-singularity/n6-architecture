//! Topology exploration: 25 structs (10 lenses + 7 engines + 8 modules)
//! for discovering and transforming topological structure of consciousness networks.

use std::collections::HashMap;

/// Result from topology exploration: metric_name -> values
pub type ExploreResult = HashMap<String, Vec<f64>>;

/// Every topology explorer must implement this trait.
pub trait TopologyExplorer: Send + Sync {
    /// Human-readable name
    fn name(&self) -> &str;
    /// Explore topology of data (N points x D dimensions, row-major).
    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult;
}

// ─────────────────────────────────────────────────────────────
// Helpers
// ─────────────────────────────────────────────────────────────

/// Euclidean distance between row i and row j in row-major data.
#[inline]
fn dist(data: &[f64], n: usize, d: usize, i: usize, j: usize) -> f64 {
    let _ = n; // used for bounds clarity
    let mut s = 0.0;
    for k in 0..d {
        let diff = data[i * d + k] - data[j * d + k];
        s += diff * diff;
    }
    s.sqrt()
}

/// Build full NxN distance matrix (flat, row-major).
fn dist_matrix(data: &[f64], n: usize, d: usize) -> Vec<f64> {
    let mut dm = vec![0.0; n * n];
    for i in 0..n {
        for j in (i + 1)..n {
            let v = dist(data, n, d, i, j);
            dm[i * n + j] = v;
            dm[j * n + i] = v;
        }
    }
    dm
}

/// Union-Find: find with path compression.
fn uf_find(parent: &mut [usize], x: usize) -> usize {
    if parent[x] != x {
        parent[x] = uf_find(parent, parent[x]);
    }
    parent[x]
}

/// Union-Find: union by rank.
fn uf_union(parent: &mut [usize], rank: &mut [usize], a: usize, b: usize) -> bool {
    let ra = uf_find(parent, a);
    let rb = uf_find(parent, b);
    if ra == rb {
        return false;
    }
    if rank[ra] < rank[rb] {
        parent[ra] = rb;
    } else if rank[ra] > rank[rb] {
        parent[rb] = ra;
    } else {
        parent[rb] = ra;
        rank[ra] += 1;
    }
    true
}

/// Mean of a slice.
#[inline]
fn mean(v: &[f64]) -> f64 {
    if v.is_empty() {
        return 0.0;
    }
    v.iter().sum::<f64>() / v.len() as f64
}

/// Variance of a slice.
#[inline]
fn variance(v: &[f64]) -> f64 {
    if v.len() < 2 {
        return 0.0;
    }
    let m = mean(v);
    v.iter().map(|x| (x - m).powi(2)).sum::<f64>() / v.len() as f64
}

/// Standard deviation.
#[inline]
fn std_dev(v: &[f64]) -> f64 {
    variance(v).sqrt()
}

/// Gini coefficient of non-negative values.
fn gini(vals: &[f64]) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    let mut sorted = vals.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let n = sorted.len() as f64;
    let s: f64 = sorted.iter().sum();
    if s == 0.0 {
        return 0.0;
    }
    let mut num = 0.0;
    for (i, &v) in sorted.iter().enumerate() {
        num += (2.0 * (i as f64 + 1.0) - n - 1.0) * v;
    }
    num / (n * s)
}

/// Build adjacency from distance matrix at a given threshold.
fn adjacency_at_threshold(dm: &[f64], n: usize, threshold: f64) -> Vec<Vec<usize>> {
    let mut adj = vec![vec![]; n];
    for i in 0..n {
        for j in (i + 1)..n {
            if dm[i * n + j] <= threshold {
                adj[i].push(j);
                adj[j].push(i);
            }
        }
    }
    adj
}

/// Count edges in adjacency.
fn count_edges(adj: &[Vec<usize>]) -> usize {
    adj.iter().map(|v| v.len()).sum::<usize>() / 2
}

/// Count triangles in adjacency.
fn count_triangles(adj: &[Vec<usize>], n: usize) -> usize {
    let mut t = 0usize;
    for i in 0..n {
        for &j in &adj[i] {
            if j > i {
                for &k in &adj[j] {
                    if k > j && adj[i].contains(&k) {
                        t += 1;
                    }
                }
            }
        }
    }
    t
}

/// Eigenvalues of a symmetric matrix (Jacobi rotation, small matrices).
/// Returns sorted eigenvalues.
fn eigenvalues_symmetric(mat: &[f64], n: usize) -> Vec<f64> {
    if n == 0 {
        return vec![];
    }
    // Simple power iteration for top eigenvalues is sufficient for our scale.
    // For small n, use direct Jacobi-like approach.
    let mut a = mat.to_vec();
    let max_iter = 100 * n;
    for _ in 0..max_iter {
        // Find largest off-diagonal
        let mut max_val = 0.0f64;
        let mut p = 0;
        let mut q = 1;
        for i in 0..n {
            for j in (i + 1)..n {
                if a[i * n + j].abs() > max_val {
                    max_val = a[i * n + j].abs();
                    p = i;
                    q = j;
                }
            }
        }
        if max_val < 1e-12 {
            break;
        }
        // Compute rotation
        let diff = a[q * n + q] - a[p * n + p];
        let t = if diff.abs() < 1e-15 {
            1.0
        } else {
            let phi = diff / (2.0 * a[p * n + q]);
            1.0 / (phi.abs() + (1.0 + phi * phi).sqrt()) * phi.signum()
        };
        let c = 1.0 / (1.0 + t * t).sqrt();
        let s = t * c;
        // Apply rotation
        let app = a[p * n + p];
        let aqq = a[q * n + q];
        let apq = a[p * n + q];
        a[p * n + p] = app - t * apq;
        a[q * n + q] = aqq + t * apq;
        a[p * n + q] = 0.0;
        a[q * n + p] = 0.0;
        for r in 0..n {
            if r != p && r != q {
                let arp = a[r * n + p];
                let arq = a[r * n + q];
                a[r * n + p] = c * arp - s * arq;
                a[p * n + r] = a[r * n + p];
                a[r * n + q] = s * arp + c * arq;
                a[q * n + r] = a[r * n + q];
            }
        }
    }
    let mut eigs: Vec<f64> = (0..n).map(|i| a[i * n + i]).collect();
    eigs.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));
    eigs
}

// ═════════════════════════════════════════════════════════════
// 10 TOPOLOGY LENSES
// ═════════════════════════════════════════════════════════════

/// 1. TopologyDiscoveryLens — Classify topology via Betti-number proxy and Euler characteristic.
pub struct TopologyDiscoveryLens;

impl TopologyExplorer for TopologyDiscoveryLens {
    fn name(&self) -> &str {
        "TopologyDiscoveryLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Median distance as threshold
        let mut dists: Vec<f64> = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = dists[dists.len() / 2];

        let adj = adjacency_at_threshold(&dm, n, median);
        let v = n;
        let e = count_edges(&adj);
        let f = count_triangles(&adj, n);

        // Euler characteristic: V - E + F
        let euler = v as f64 - e as f64 + f as f64;

        // Betti_0 proxy: connected components at median threshold
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank_arr = vec![0usize; n];
        for i in 0..n {
            for &j in &adj[i] {
                if j > i {
                    uf_union(&mut parent, &mut rank_arr, i, j);
                }
            }
        }
        let betti0 = (0..n)
            .filter(|&i| uf_find(&mut parent, i) == i)
            .count() as f64;

        // Betti_1 proxy: independent cycles = E - V + components
        let betti1 = (e as f64 - v as f64 + betti0).max(0.0);

        // Degree distribution statistics
        let degrees: Vec<f64> = adj.iter().map(|a| a.len() as f64).collect();
        let deg_mean = mean(&degrees);
        let deg_std = std_dev(&degrees);

        // Classify: scale-free (high std/mean), small-world (high clustering), ring (low std)
        let classification = if deg_std / (deg_mean + 1e-15) > 1.0 {
            3.0 // scale-free
        } else if f as f64 / (e as f64 + 1.0) > 0.3 {
            2.0 // small-world
        } else if deg_std / (deg_mean + 1e-15) < 0.3 {
            1.0 // ring-like
        } else {
            0.0 // generic
        };

        result.insert("euler_characteristic".into(), vec![euler]);
        result.insert("betti_0".into(), vec![betti0]);
        result.insert("betti_1_proxy".into(), vec![betti1]);
        result.insert("vertices".into(), vec![v as f64]);
        result.insert("edges".into(), vec![e as f64]);
        result.insert("triangles".into(), vec![f as f64]);
        result.insert("degree_mean".into(), vec![deg_mean]);
        result.insert("degree_std".into(), vec![deg_std]);
        result.insert("classification".into(), vec![classification]);
        result
    }
}

/// 2. DimensionLens — Intrinsic dimension estimation (correlation dimension + PCA effective rank).
pub struct DimensionLens;

impl TopologyExplorer for DimensionLens {
    fn name(&self) -> &str {
        "DimensionLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 4 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Correlation dimension via Grassberger-Procaccia
        let mut dists: Vec<f64> = Vec::with_capacity(n * (n - 1) / 2);
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Two radii at 25% and 75% of distances for log-log slope
        let r1 = dists[dists.len() / 4].max(1e-15);
        let r2 = dists[3 * dists.len() / 4].max(r1 + 1e-15);
        let total = dists.len() as f64;
        let c1 = dists.iter().filter(|&&x| x < r1).count() as f64 / total;
        let c2 = dists.iter().filter(|&&x| x < r2).count() as f64 / total;
        let corr_dim = if c1 > 1e-15 && c2 > 1e-15 {
            (c2.ln() - c1.ln()) / (r2.ln() - r1.ln())
        } else {
            d as f64
        };

        // PCA effective rank via eigenvalues of covariance
        // Compute mean
        let mut mu = vec![0.0; d];
        for i in 0..n {
            for k in 0..d {
                mu[k] += data[i * d + k];
            }
        }
        for k in 0..d {
            mu[k] /= n as f64;
        }
        // Covariance matrix (d x d)
        let dd = d.min(32); // cap for eigenvalue computation
        let mut cov = vec![0.0; dd * dd];
        for i in 0..n {
            for a in 0..dd {
                for b in 0..dd {
                    cov[a * dd + b] +=
                        (data[i * d + a] - mu[a]) * (data[i * d + b] - mu[b]);
                }
            }
        }
        for v in cov.iter_mut() {
            *v /= n as f64;
        }
        let eigs = eigenvalues_symmetric(&cov, dd);
        let eig_sum: f64 = eigs.iter().map(|x| x.max(0.0)).sum();
        // Effective rank: exp(entropy of normalized eigenvalues)
        let eff_rank = if eig_sum > 1e-15 {
            let entropy: f64 = eigs
                .iter()
                .map(|&e| {
                    let p = e.max(0.0) / eig_sum;
                    if p > 1e-15 {
                        -p * p.ln()
                    } else {
                        0.0
                    }
                })
                .sum();
            entropy.exp()
        } else {
            1.0
        };

        // Explained variance ratio (top eigenvalue / sum)
        let top_ratio = if eig_sum > 1e-15 {
            eigs[0].max(0.0) / eig_sum
        } else {
            1.0
        };

        result.insert("correlation_dimension".into(), vec![corr_dim]);
        result.insert("pca_effective_rank".into(), vec![eff_rank]);
        result.insert("ambient_dimension".into(), vec![d as f64]);
        result.insert("top_eigenvalue_ratio".into(), vec![top_ratio]);
        result.insert("eigenvalues".into(), eigs.iter().map(|x| x.max(0.0)).collect());
        result
    }
}

/// 3. FoldingLens — Evaluate 1D→2D folding quality (local density maximization).
pub struct FoldingLens;

impl TopologyExplorer for FoldingLens {
    fn name(&self) -> &str {
        "FoldingLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 4 || d == 0 {
            return result;
        }

        // Sort points by first dimension (1D ordering)
        let mut indices: Vec<usize> = (0..n).collect();
        indices.sort_by(|&a, &b| {
            data[a * d]
                .partial_cmp(&data[b * d])
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        // Compute local density: for each point, count neighbors within radius
        let dm = dist_matrix(data, n, d);
        let mut all_d: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                all_d.push(dm[i * n + j]);
            }
        }
        all_d.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let radius = all_d[all_d.len() / 4]; // 25th percentile

        let local_density: Vec<f64> = (0..n)
            .map(|i| {
                (0..n)
                    .filter(|&j| j != i && dm[i * n + j] <= radius)
                    .count() as f64
            })
            .collect();

        let mean_density = mean(&local_density);
        let max_density = local_density.iter().cloned().fold(0.0f64, f64::max);

        // Try folding: boustrophedon (zigzag) in 1D ordering
        // Compare sequential vs zigzag neighbor distances
        let mut seq_sum = 0.0;
        let mut zig_sum = 0.0;
        let width = (n as f64).sqrt().ceil() as usize;
        for i in 0..(n - 1) {
            let a = indices[i];
            let b = indices[i + 1];
            seq_sum += dm[a * n + b];
        }
        // Zigzag: reverse every other row
        let mut zigzag = indices.clone();
        for row in 0.. {
            let start = row * width;
            if start >= n {
                break;
            }
            let end = (start + width).min(n);
            if row % 2 == 1 {
                zigzag[start..end].reverse();
            }
        }
        for i in 0..(n - 1) {
            let a = zigzag[i];
            let b = zigzag[i + 1];
            zig_sum += dm[a * n + b];
        }

        let fold_improvement = if zig_sum > 1e-15 {
            seq_sum / zig_sum
        } else {
            1.0
        };

        result.insert("mean_local_density".into(), vec![mean_density]);
        result.insert("max_local_density".into(), vec![max_density]);
        result.insert("fold_improvement".into(), vec![fold_improvement]);
        result.insert("optimal_width".into(), vec![width as f64]);
        result.insert("local_density".into(), local_density);
        result
    }
}

/// 4. HoleLens — Detect cycles/cavities via persistent homology proxy (distance filtration).
pub struct HoleLens;

impl TopologyExplorer for HoleLens {
    fn name(&self) -> &str {
        "HoleLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Collect edges sorted by distance
        let mut edges: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j, dm[i * n + j]));
            }
        }
        edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        // H0: connected component birth/death
        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank_arr = vec![0usize; n];
        let h0_births = vec![0.0f64; n]; // all born at 0
        let mut h0_deaths: Vec<f64> = Vec::new();

        for &(i, j, d_val) in &edges {
            if uf_union(&mut parent, &mut rank_arr, i, j) {
                h0_deaths.push(d_val);
            }
        }

        // H1 proxy: edges that create a cycle (don't merge components)
        // These are birth events for 1-cycles
        let mut parent2: Vec<usize> = (0..n).collect();
        let mut rank2 = vec![0usize; n];
        let mut h1_births: Vec<f64> = Vec::new();
        for &(i, j, d_val) in &edges {
            if !uf_union(&mut parent2, &mut rank2, i, j) {
                h1_births.push(d_val);
            }
        }

        // Persistence: longer-lived features are more significant
        let h0_persistence: Vec<f64> = h0_deaths.iter().map(|&d_val| d_val).collect();
        let h1_count = h1_births.len() as f64;

        let total_persistence: f64 = h0_persistence.iter().sum::<f64>();
        let max_persistence = h0_persistence
            .iter()
            .cloned()
            .fold(0.0f64, f64::max);

        result.insert("h0_components_at_start".into(), vec![n as f64]);
        result.insert("h0_death_distances".into(), h0_deaths);
        result.insert("h1_cycle_count".into(), vec![h1_count]);
        result.insert("h1_birth_distances".into(), h1_births);
        result.insert("total_h0_persistence".into(), vec![total_persistence]);
        result.insert("max_h0_persistence".into(), vec![max_persistence]);
        result.insert("h0_birth_distances".into(), h0_births);
        result
    }
}

/// 5. BoundaryLens — Boundary vs interior analysis (surface cells vs bulk).
pub struct BoundaryLens;

impl TopologyExplorer for BoundaryLens {
    fn name(&self) -> &str {
        "BoundaryLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Compute centroid
        let mut centroid = vec![0.0; d];
        for i in 0..n {
            for k in 0..d {
                centroid[k] += data[i * d + k];
            }
        }
        for k in 0..d {
            centroid[k] /= n as f64;
        }

        // Distance from centroid for each point
        let centroid_dists: Vec<f64> = (0..n)
            .map(|i| {
                let mut s = 0.0;
                for k in 0..d {
                    let diff = data[i * d + k] - centroid[k];
                    s += diff * diff;
                }
                s.sqrt()
            })
            .collect();

        // k-nearest neighbors average distance
        let k = (n / 4).max(2).min(n - 1);
        let knn_density: Vec<f64> = (0..n)
            .map(|i| {
                let mut row_dists: Vec<f64> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| dm[i * n + j])
                    .collect();
                row_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
                mean(&row_dists[..k])
            })
            .collect();

        // Boundary = high centroid distance AND high knn distance (low density)
        let cd_mean = mean(&centroid_dists);
        let knn_mean = mean(&knn_density);

        let boundary_scores: Vec<f64> = (0..n)
            .map(|i| {
                let cd_norm = centroid_dists[i] / (cd_mean + 1e-15);
                let knn_norm = knn_density[i] / (knn_mean + 1e-15);
                (cd_norm + knn_norm) / 2.0
            })
            .collect();

        let boundary_threshold = 1.2;
        let boundary_count = boundary_scores
            .iter()
            .filter(|&&s| s > boundary_threshold)
            .count() as f64;
        let interior_count = n as f64 - boundary_count;
        let boundary_ratio = boundary_count / n as f64;

        result.insert("boundary_scores".into(), boundary_scores);
        result.insert("boundary_count".into(), vec![boundary_count]);
        result.insert("interior_count".into(), vec![interior_count]);
        result.insert("boundary_ratio".into(), vec![boundary_ratio]);
        result.insert("centroid_distances".into(), centroid_dists);
        result.insert("knn_density".into(), knn_density);
        result
    }
}

/// 6. SymmetryBreakingLens — Detect symmetry breaking via cluster size Gini.
pub struct SymmetryBreakingLens;

impl TopologyExplorer for SymmetryBreakingLens {
    fn name(&self) -> &str {
        "SymmetryBreakingLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Cluster at median distance
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let threshold = dists[dists.len() / 2];

        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank_arr = vec![0usize; n];
        for i in 0..n {
            for j in (i + 1)..n {
                if dm[i * n + j] <= threshold {
                    uf_union(&mut parent, &mut rank_arr, i, j);
                }
            }
        }

        // Cluster sizes
        let mut size_map: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let root = uf_find(&mut parent, i);
            *size_map.entry(root).or_insert(0) += 1;
        }
        let sizes: Vec<f64> = size_map.values().map(|&s| s as f64).collect();
        let num_clusters = sizes.len() as f64;

        // Gini of cluster sizes: 0 = perfect symmetry, 1 = total asymmetry
        let gini_val = gini(&sizes);

        // Largest cluster ratio
        let largest = sizes.iter().cloned().fold(0.0f64, f64::max);
        let largest_ratio = largest / n as f64;

        // Entropy of cluster size distribution
        let total: f64 = sizes.iter().sum();
        let entropy: f64 = sizes
            .iter()
            .map(|&s| {
                let p = s / total;
                if p > 1e-15 {
                    -p * p.ln()
                } else {
                    0.0
                }
            })
            .sum();

        // Symmetry breaking score: high Gini + high largest ratio = strong breaking
        let breaking_score = (gini_val + largest_ratio) / 2.0;

        result.insert("num_clusters".into(), vec![num_clusters]);
        result.insert("cluster_sizes".into(), sizes);
        result.insert("gini_coefficient".into(), vec![gini_val]);
        result.insert("largest_cluster_ratio".into(), vec![largest_ratio]);
        result.insert("size_entropy".into(), vec![entropy]);
        result.insert("breaking_score".into(), vec![breaking_score]);
        result
    }
}

/// 7. ConnectednessLens — Connected components via union-find on distance threshold.
pub struct ConnectednessLens;

impl TopologyExplorer for ConnectednessLens {
    fn name(&self) -> &str {
        "ConnectednessLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 2 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Sweep thresholds from min to max distance
        let mut all_dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                all_dists.push(dm[i * n + j]);
            }
        }
        all_dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let num_thresholds = 10;
        let mut thresholds = Vec::new();
        let mut components_at_threshold = Vec::new();

        for t in 0..num_thresholds {
            let idx = (t * all_dists.len() / num_thresholds).min(all_dists.len() - 1);
            let threshold = all_dists[idx];
            thresholds.push(threshold);

            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank_arr = vec![0usize; n];
            for i in 0..n {
                for j in (i + 1)..n {
                    if dm[i * n + j] <= threshold {
                        uf_union(&mut parent, &mut rank_arr, i, j);
                    }
                }
            }
            let comps = (0..n)
                .filter(|&i| uf_find(&mut parent, i) == i)
                .count() as f64;
            components_at_threshold.push(comps);
        }

        // Critical threshold: where components drop to 1
        let critical_idx = components_at_threshold
            .iter()
            .position(|&c| c <= 1.0)
            .unwrap_or(num_thresholds - 1);
        let critical_threshold = thresholds[critical_idx];

        // Connectivity robustness: AUC of components curve (normalized)
        let max_comp = n as f64;
        let robustness: f64 = components_at_threshold
            .iter()
            .map(|&c| 1.0 - c / max_comp)
            .sum::<f64>()
            / num_thresholds as f64;

        result.insert("thresholds".into(), thresholds);
        result.insert("components".into(), components_at_threshold);
        result.insert("critical_threshold".into(), vec![critical_threshold]);
        result.insert("robustness".into(), vec![robustness]);
        result
    }
}

/// 8. CurvatureLens — Local curvature estimation (positive/negative/zero).
pub struct CurvatureLens;

impl TopologyExplorer for CurvatureLens {
    fn name(&self) -> &str {
        "CurvatureLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 5 || d < 2 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Ollivier-Ricci curvature proxy:
        // For each pair (i,j), compare d(i,j) with average distance of
        // their k-neighborhoods. Positive curvature = neighborhoods closer
        // than expected. Negative = farther.
        let k = (n / 4).max(2).min(n - 1);

        let knn: Vec<Vec<usize>> = (0..n)
            .map(|i| {
                let mut nbrs: Vec<(usize, f64)> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| (j, dm[i * n + j]))
                    .collect();
                nbrs.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                nbrs.iter().take(k).map(|&(j, _)| j).collect()
            })
            .collect();

        let mut curvatures = Vec::with_capacity(n);
        for i in 0..n {
            // Average curvature around point i
            let mut curv_sum = 0.0;
            let mut count = 0;
            for &j in &knn[i] {
                let d_ij = dm[i * n + j];
                if d_ij < 1e-15 {
                    continue;
                }
                // W1 distance proxy between neighborhoods
                let mut w1 = 0.0;
                let ki = &knn[i];
                let kj = &knn[j];
                let m = ki.len().min(kj.len());
                for t in 0..m {
                    w1 += dm[ki[t] * n + kj[t]];
                }
                w1 /= m as f64;
                // Curvature = 1 - W1/d(i,j)
                curv_sum += 1.0 - w1 / d_ij;
                count += 1;
            }
            curvatures.push(if count > 0 {
                curv_sum / count as f64
            } else {
                0.0
            });
        }

        let mean_curv = mean(&curvatures);
        let positive = curvatures.iter().filter(|&&c| c > 0.05).count() as f64;
        let negative = curvatures.iter().filter(|&&c| c < -0.05).count() as f64;
        let flat = n as f64 - positive - negative;

        result.insert("curvatures".into(), curvatures);
        result.insert("mean_curvature".into(), vec![mean_curv]);
        result.insert("positive_count".into(), vec![positive]);
        result.insert("negative_count".into(), vec![negative]);
        result.insert("flat_count".into(), vec![flat]);
        result
    }
}

/// 9. EmbeddingLens — Dimensionality reduction quality (stress, trustworthiness).
pub struct EmbeddingLens;

impl TopologyExplorer for EmbeddingLens {
    fn name(&self) -> &str {
        "EmbeddingLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d < 2 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Project to 2D via first 2 coordinates (simple projection baseline)
        let mut proj = vec![0.0; n * 2];
        for i in 0..n {
            proj[i * 2] = data[i * d];
            proj[i * 2 + 1] = if d > 1 { data[i * d + 1] } else { 0.0 };
        }
        let dm2 = dist_matrix(&proj, n, 2);

        // Kruskal stress: sqrt(sum((d_orig - d_proj)^2) / sum(d_orig^2))
        let mut num = 0.0;
        let mut den = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                let orig = dm[i * n + j];
                let proj_d = dm2[i * n + j];
                num += (orig - proj_d).powi(2);
                den += orig.powi(2);
            }
        }
        let stress = if den > 1e-15 {
            (num / den).sqrt()
        } else {
            0.0
        };

        // Trustworthiness: fraction of k-NN preserved
        let k = (n / 4).max(2).min(n - 1);
        let mut trust_sum = 0.0;
        for i in 0..n {
            let mut orig_nn: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, dm[i * n + j]))
                .collect();
            orig_nn.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let orig_set: Vec<usize> = orig_nn.iter().take(k).map(|&(j, _)| j).collect();

            let mut proj_nn: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, dm2[i * n + j]))
                .collect();
            proj_nn.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let proj_set: Vec<usize> = proj_nn.iter().take(k).map(|&(j, _)| j).collect();

            let preserved = proj_set.iter().filter(|j| orig_set.contains(j)).count();
            trust_sum += preserved as f64 / k as f64;
        }
        let trustworthiness = trust_sum / n as f64;

        // Continuity: how well original neighborhoods are preserved in projection
        // (inverse of trustworthiness — check if original NN are NN in projection)
        let mut cont_sum = 0.0;
        for i in 0..n {
            let mut orig_nn: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, dm[i * n + j]))
                .collect();
            orig_nn.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let orig_set: Vec<usize> = orig_nn.iter().take(k).map(|&(j, _)| j).collect();

            let mut proj_nn: Vec<(usize, f64)> = (0..n)
                .filter(|&j| j != i)
                .map(|j| (j, dm2[i * n + j]))
                .collect();
            proj_nn.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
            let proj_set: Vec<usize> = proj_nn.iter().take(k).map(|&(j, _)| j).collect();

            let preserved = orig_set.iter().filter(|j| proj_set.contains(j)).count();
            cont_sum += preserved as f64 / k as f64;
        }
        let continuity = cont_sum / n as f64;

        result.insert("stress".into(), vec![stress]);
        result.insert("trustworthiness".into(), vec![trustworthiness]);
        result.insert("continuity".into(), vec![continuity]);
        result.insert("target_dim".into(), vec![2.0]);
        result.insert("source_dim".into(), vec![d as f64]);
        result
    }
}

/// 10. MorphismLens — Topology transformation feasibility (invariant comparison).
pub struct MorphismLens;

impl TopologyExplorer for MorphismLens {
    fn name(&self) -> &str {
        "MorphismLens"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Compute topological invariants at multiple thresholds
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        // Invariants at 3 thresholds: 25%, 50%, 75%
        let mut betti0_curve = Vec::new();
        let mut edge_curve = Vec::new();
        let mut triangle_curve = Vec::new();

        for pct in &[25usize, 50, 75] {
            let idx = (*pct * dists.len() / 100).min(dists.len() - 1);
            let threshold = dists[idx];
            let adj = adjacency_at_threshold(&dm, n, threshold);

            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank_arr = vec![0usize; n];
            for i in 0..n {
                for &j in &adj[i] {
                    if j > i {
                        uf_union(&mut parent, &mut rank_arr, i, j);
                    }
                }
            }
            let comps = (0..n)
                .filter(|&i| uf_find(&mut parent, i) == i)
                .count() as f64;
            betti0_curve.push(comps);
            edge_curve.push(count_edges(&adj) as f64);
            triangle_curve.push(count_triangles(&adj, n) as f64);
        }

        // Homeomorphism feasibility: can this topology be continuously
        // deformed to a simpler one? Measure via Betti number stability.
        let betti_stability = if betti0_curve.len() >= 2 {
            let diff: f64 = betti0_curve
                .windows(2)
                .map(|w| (w[1] - w[0]).abs())
                .sum();
            1.0 / (1.0 + diff)
        } else {
            1.0
        };

        // Degree of graph homomorphism to complete graph
        let adj_50 = adjacency_at_threshold(&dm, n, dists[dists.len() / 2]);
        let actual_edges = count_edges(&adj_50) as f64;
        let max_edges = (n * (n - 1) / 2) as f64;
        let completeness = actual_edges / (max_edges + 1e-15);

        result.insert("betti0_curve".into(), betti0_curve);
        result.insert("edge_curve".into(), edge_curve);
        result.insert("triangle_curve".into(), triangle_curve);
        result.insert("betti_stability".into(), vec![betti_stability]);
        result.insert("completeness".into(), vec![completeness]);
        result
    }
}

// ═════════════════════════════════════════════════════════════
// 7 TOPOLOGY ENGINES
// ═════════════════════════════════════════════════════════════

/// 11. TopologyEvolver — Evolve topology via mutation, fitness = Phi proxy.
pub struct TopologyEvolver {
    pub generations: usize,
    pub mutation_rate: f64,
}

impl Default for TopologyEvolver {
    fn default() -> Self {
        Self {
            generations: 20,
            mutation_rate: 0.1,
        }
    }
}

impl TopologyExplorer for TopologyEvolver {
    fn name(&self) -> &str {
        "TopologyEvolver"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);
        let median = {
            let mut dists: Vec<f64> = Vec::new();
            for i in 0..n {
                for j in (i + 1)..n {
                    dists.push(dm[i * n + j]);
                }
            }
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            dists[dists.len() / 2]
        };

        // Start with adjacency at median
        let mut adj_matrix = vec![false; n * n];
        for i in 0..n {
            for j in (i + 1)..n {
                if dm[i * n + j] <= median {
                    adj_matrix[i * n + j] = true;
                    adj_matrix[j * n + i] = true;
                }
            }
        }

        // Phi proxy: variance of cell states (global - faction)
        let phi_proxy = |adj: &[bool]| -> f64 {
            // Degree-weighted variance as proxy for integration
            let degrees: Vec<f64> = (0..n)
                .map(|i| (0..n).filter(|&j| adj[i * n + j]).count() as f64)
                .collect();
            let global_var = variance(&degrees);
            // Faction variance: split by connectivity
            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank_arr = vec![0usize; n];
            for i in 0..n {
                for j in (i + 1)..n {
                    if adj[i * n + j] {
                        uf_union(&mut parent, &mut rank_arr, i, j);
                    }
                }
            }
            let mut factions: HashMap<usize, Vec<f64>> = HashMap::new();
            for i in 0..n {
                let root = uf_find(&mut parent, i);
                factions.entry(root).or_default().push(degrees[i]);
            }
            let faction_var: f64 = factions
                .values()
                .map(|v| variance(v) * v.len() as f64)
                .sum::<f64>()
                / n as f64;
            (global_var - faction_var).max(0.0)
        };

        let mut best_phi = phi_proxy(&adj_matrix);
        let mut fitness_history = vec![best_phi];

        // Simple deterministic mutation schedule
        let seed_base: u64 = data.len() as u64 ^ n as u64;
        for gen in 0..self.generations {
            let mut candidate = adj_matrix.clone();
            // Mutate: flip edges based on deterministic hash
            let num_mutations =
                ((n * n) as f64 * self.mutation_rate).ceil() as usize;
            for m in 0..num_mutations {
                let hash = seed_base
                    .wrapping_mul(gen as u64 + 1)
                    .wrapping_add(m as u64 * 6971);
                let i = (hash % n as u64) as usize;
                let j = ((hash / n as u64) % n as u64) as usize;
                if i != j {
                    candidate[i * n + j] = !candidate[i * n + j];
                    candidate[j * n + i] = candidate[i * n + j];
                }
            }
            let phi = phi_proxy(&candidate);
            if phi > best_phi {
                best_phi = phi;
                adj_matrix = candidate;
            }
            fitness_history.push(best_phi);
        }

        let final_edges = {
            let mut c = 0usize;
            for i in 0..n { for j in (i+1)..n { if adj_matrix[i * n + j] { c += 1; } } }
            c as f64
        };

        result.insert("fitness_history".into(), fitness_history);
        result.insert("best_phi_proxy".into(), vec![best_phi]);
        result.insert("final_edges".into(), vec![final_edges]);
        result.insert("generations".into(), vec![self.generations as f64]);
        result
    }
}

/// 12. DimensionClimber — Try 1D→2D→3D..., measure Phi at each, find optimal dimension.
pub struct DimensionClimber;

impl TopologyExplorer for DimensionClimber {
    fn name(&self) -> &str {
        "DimensionClimber"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }

        // For each target dimension t from 1..d, project data and measure Phi proxy
        let max_dim = d.min(16);
        let mut dim_values = Vec::new();
        let mut phi_values = Vec::new();

        for target_d in 1..=max_dim {
            // Project: take first target_d columns
            let mut proj = vec![0.0; n * target_d];
            for i in 0..n {
                for k in 0..target_d {
                    proj[i * target_d + k] = data[i * d + k];
                }
            }

            // Phi proxy: global variance - mean local variance
            let mut col_vals = vec![vec![]; target_d];
            for i in 0..n {
                for k in 0..target_d {
                    col_vals[k].push(proj[i * target_d + k]);
                }
            }
            let global_var: f64 = col_vals.iter().map(|c| variance(c)).sum::<f64>();

            // Local variance: k-nn neighborhoods
            let dm = dist_matrix(&proj, n, target_d);
            let k = (n / 3).max(2).min(n - 1);
            let mut local_var_sum = 0.0;
            for i in 0..n {
                let mut nbr_dists: Vec<(usize, f64)> = (0..n)
                    .filter(|&j| j != i)
                    .map(|j| (j, dm[i * n + j]))
                    .collect();
                nbr_dists
                    .sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                let nbrs: Vec<usize> = nbr_dists.iter().take(k).map(|&(j, _)| j).collect();
                for kk in 0..target_d {
                    let vals: Vec<f64> = nbrs.iter().map(|&j| proj[j * target_d + kk]).collect();
                    local_var_sum += variance(&vals);
                }
            }
            local_var_sum /= n as f64;

            let phi = (global_var - local_var_sum).max(0.0);
            dim_values.push(target_d as f64);
            phi_values.push(phi);
        }

        // Find optimal dimension (max Phi)
        let optimal = phi_values
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| dim_values[i])
            .unwrap_or(1.0);

        result.insert("dimensions".into(), dim_values);
        result.insert("phi_at_dim".into(), phi_values);
        result.insert("optimal_dimension".into(), vec![optimal]);
        result.insert("ambient_dimension".into(), vec![d as f64]);
        result
    }
}

/// 13. FoldingEngine — Exhaustive fold search, minimize energy.
pub struct FoldingEngine;

impl TopologyExplorer for FoldingEngine {
    fn name(&self) -> &str {
        "FoldingEngine"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }

        // Energy function: sum of distance between consecutive points in ordering
        let dm = dist_matrix(data, n, d);

        // Try different "fold widths" for 1D→2D boustrophedon
        let mut best_energy = f64::MAX;
        let mut best_width = 1usize;
        let mut energy_curve = Vec::new();

        let max_width = (n as f64).sqrt().ceil() as usize + 2;
        for width in 2..=max_width.min(n) {
            // Create boustrophedon ordering
            let mut order: Vec<usize> = (0..n).collect();
            for row in 0.. {
                let start = row * width;
                if start >= n {
                    break;
                }
                let end = (start + width).min(n);
                if row % 2 == 1 {
                    order[start..end].reverse();
                }
            }

            // Total path energy
            let mut energy = 0.0;
            for i in 0..(n - 1) {
                energy += dm[order[i] * n + order[i + 1]];
            }
            // Inter-row proximity bonus
            for row in 0.. {
                let start = row * width;
                if start + width >= n {
                    break;
                }
                for col in 0..width.min(n - start - width) {
                    let a = order[start + col];
                    let b = order[start + width + col];
                    energy -= dm[a * n + b] * 0.1; // bonus for vertical neighbors
                }
            }

            energy_curve.push(energy);
            if energy < best_energy {
                best_energy = energy;
                best_width = width;
            }
        }

        // Compare with no folding (sequential)
        let mut seq_energy = 0.0;
        for i in 0..(n - 1) {
            seq_energy += dm[i * n + (i + 1)];
        }

        let improvement = if best_energy.abs() > 1e-15 {
            seq_energy / best_energy
        } else {
            1.0
        };

        result.insert("best_width".into(), vec![best_width as f64]);
        result.insert("best_energy".into(), vec![best_energy]);
        result.insert("sequential_energy".into(), vec![seq_energy]);
        result.insert("improvement".into(), vec![improvement]);
        result.insert("energy_curve".into(), energy_curve);
        result
    }
}

/// 14. SurgeryEngine — Cut and reattach topology, measure Phi change.
pub struct SurgeryEngine;

impl TopologyExplorer for SurgeryEngine {
    fn name(&self) -> &str {
        "SurgeryEngine"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 6 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Build adjacency at median distance
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let threshold = dists[dists.len() / 2];

        let mut adj = vec![false; n * n];
        for i in 0..n {
            for j in (i + 1)..n {
                if dm[i * n + j] <= threshold {
                    adj[i * n + j] = true;
                    adj[j * n + i] = true;
                }
            }
        }

        // Phi proxy
        let phi_of = |a: &[bool]| -> f64 {
            let degrees: Vec<f64> = (0..n)
                .map(|i| (0..n).filter(|&j| a[i * n + j]).count() as f64)
                .collect();
            let gv = variance(&degrees);
            let mut parent: Vec<usize> = (0..n).collect();
            let mut rank_arr = vec![0usize; n];
            for i in 0..n {
                for j in (i + 1)..n {
                    if a[i * n + j] {
                        uf_union(&mut parent, &mut rank_arr, i, j);
                    }
                }
            }
            let mut fac: HashMap<usize, Vec<f64>> = HashMap::new();
            for i in 0..n {
                let r = uf_find(&mut parent, i);
                fac.entry(r).or_default().push(degrees[i]);
            }
            let fv: f64 = fac
                .values()
                .map(|v| variance(v) * v.len() as f64)
                .sum::<f64>()
                / n as f64;
            (gv - fv).max(0.0)
        };

        let original_phi = phi_of(&adj);

        // Surgery: cut the graph in half, reconnect differently
        let mid = n / 2;
        let mut surgeries = Vec::new();
        let mut phi_changes = Vec::new();

        // Surgery 1: disconnect halves, reconnect with cross-bridges
        let mut s1 = adj.clone();
        for i in 0..mid {
            for j in mid..n {
                s1[i * n + j] = false;
                s1[j * n + i] = false;
            }
        }
        // Add bridges: connect nearest pairs across halves
        let bridges = (n / 6).max(1);
        let mut cross_dists: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..mid {
            for j in mid..n {
                cross_dists.push((i, j, dm[i * n + j]));
            }
        }
        cross_dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        for k in 0..bridges.min(cross_dists.len()) {
            let (i, j, _) = cross_dists[k];
            s1[i * n + j] = true;
            s1[j * n + i] = true;
        }
        let phi1 = phi_of(&s1);
        surgeries.push(1.0);
        phi_changes.push(phi1 - original_phi);

        // Surgery 2: reverse connectivity of one half
        let mut s2 = adj.clone();
        for i in 0..mid {
            for j in (i + 1)..mid {
                s2[i * n + j] = !s2[i * n + j];
                s2[j * n + i] = s2[i * n + j];
            }
        }
        let phi2 = phi_of(&s2);
        surgeries.push(2.0);
        phi_changes.push(phi2 - original_phi);

        // Surgery 3: add shortcuts (random long-range connections)
        let mut s3 = adj.clone();
        let num_shortcuts = (n / 4).max(1);
        for k in 0..num_shortcuts {
            let i = (k * 7 + 3) % n;
            let j = (i + n / 3) % n;
            if i != j {
                s3[i * n + j] = true;
                s3[j * n + i] = true;
            }
        }
        let phi3 = phi_of(&s3);
        surgeries.push(3.0);
        phi_changes.push(phi3 - original_phi);

        result.insert("original_phi".into(), vec![original_phi]);
        result.insert("surgery_type".into(), surgeries);
        result.insert("phi_change".into(), phi_changes);
        result.insert("best_surgery".into(), vec![
            if phi1 >= phi2 && phi1 >= phi3 {
                1.0
            } else if phi2 >= phi3 {
                2.0
            } else {
                3.0
            },
        ]);
        result
    }
}

/// 15. StackingEngine — Auto-stack layers, find optimal depth.
pub struct StackingEngine;

impl TopologyExplorer for StackingEngine {
    fn name(&self) -> &str {
        "StackingEngine"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 4 || d == 0 {
            return result;
        }

        // Simulate stacking: partition n nodes into L layers
        // Measure inter-layer vs intra-layer connectivity
        let dm = dist_matrix(data, n, d);

        let max_layers = (n / 2).min(8);
        let mut layer_counts = Vec::new();
        let mut phi_at_layers = Vec::new();

        for num_layers in 1..=max_layers {
            let nodes_per_layer = n / num_layers;
            if nodes_per_layer < 2 {
                break;
            }

            // Assign nodes to layers
            let mut layers: Vec<Vec<usize>> = vec![vec![]; num_layers];
            for i in 0..n {
                layers[i % num_layers].push(i);
            }

            // Intra-layer average distance
            let mut intra = 0.0;
            let mut intra_count = 0;
            for layer in &layers {
                for i in 0..layer.len() {
                    for j in (i + 1)..layer.len() {
                        intra += dm[layer[i] * n + layer[j]];
                        intra_count += 1;
                    }
                }
            }
            intra /= intra_count.max(1) as f64;

            // Inter-layer average distance
            let mut inter = 0.0;
            let mut inter_count = 0;
            for l1 in 0..num_layers {
                for l2 in (l1 + 1)..num_layers {
                    for &i in &layers[l1] {
                        for &j in &layers[l2] {
                            inter += dm[i * n + j];
                            inter_count += 1;
                        }
                    }
                }
            }
            inter /= inter_count.max(1) as f64;

            // Phi proxy: high intra-layer cohesion + inter-layer differentiation
            let phi = (inter - intra).max(0.0) * num_layers as f64;

            layer_counts.push(num_layers as f64);
            phi_at_layers.push(phi);
        }

        let optimal = phi_at_layers
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| layer_counts[i])
            .unwrap_or(1.0);

        result.insert("layer_counts".into(), layer_counts);
        result.insert("phi_at_layers".into(), phi_at_layers);
        result.insert("optimal_layers".into(), vec![optimal]);
        result
    }
}

/// 16. HybridTopologyEngine — Mix topologies (ring + tree + torus combinations).
pub struct HybridTopologyEngine;

impl TopologyExplorer for HybridTopologyEngine {
    fn name(&self) -> &str {
        "HybridTopologyEngine"
    }

    fn explore(&self, _data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 4 || d == 0 {
            return result;
        }

        let phi_of_adj = |adj: &[bool], sz: usize| -> f64 {
            let degrees: Vec<f64> = (0..sz)
                .map(|i| (0..sz).filter(|&j| adj[i * sz + j]).count() as f64)
                .collect();
            let gv = variance(&degrees);
            let mut parent: Vec<usize> = (0..sz).collect();
            let mut rank_arr = vec![0usize; sz];
            for i in 0..sz {
                for j in (i + 1)..sz {
                    if adj[i * sz + j] {
                        uf_union(&mut parent, &mut rank_arr, i, j);
                    }
                }
            }
            let mut fac: HashMap<usize, Vec<f64>> = HashMap::new();
            for i in 0..sz {
                let r = uf_find(&mut parent, i);
                fac.entry(r).or_default().push(degrees[i]);
            }
            let fv: f64 = fac
                .values()
                .map(|v| variance(v) * v.len() as f64)
                .sum::<f64>()
                / sz as f64;
            (gv - fv).max(0.0)
        };

        let set_edge =
            |adj: &mut Vec<bool>, sz: usize, i: usize, j: usize| {
                adj[i * sz + j] = true;
                adj[j * sz + i] = true;
            };

        let mut topo_names: Vec<String> = Vec::new();
        let mut topo_phis: Vec<f64> = Vec::new();
        let mut topo_edges: Vec<f64> = Vec::new();

        // Topology 1: Ring
        let mut ring = vec![false; n * n];
        for i in 0..n {
            set_edge(&mut ring, n, i, (i + 1) % n);
        }
        topo_names.push("ring".into());
        topo_phis.push(phi_of_adj(&ring, n));
        topo_edges.push(n as f64);

        // Topology 2: Star (hub-spoke)
        let mut star = vec![false; n * n];
        for i in 1..n {
            set_edge(&mut star, n, 0, i);
        }
        topo_names.push("star".into());
        topo_phis.push(phi_of_adj(&star, n));
        topo_edges.push((n - 1) as f64);

        // Topology 3: Ring + shortcuts (small-world)
        let mut sw = ring.clone();
        let num_shortcuts = (n / 4).max(1);
        for k in 0..num_shortcuts {
            let i = k * 3 % n;
            let j = (i + n / 3) % n;
            if i != j {
                set_edge(&mut sw, n, i, j);
            }
        }
        topo_names.push("small_world".into());
        topo_phis.push(phi_of_adj(&sw, n));
        {
            let mut ec = 0usize;
            for i in 0..n { for j in (i+1)..n { if sw[i * n + j] { ec += 1; } } }
            topo_edges.push(ec as f64);
        }

        // Topology 4: Grid (2D torus)
        let cols = (n as f64).sqrt().ceil() as usize;
        let mut grid = vec![false; n * n];
        for i in 0..n {
            let right = if (i % cols) + 1 < cols && i + 1 < n {
                Some(i + 1)
            } else {
                None
            };
            let down = if i + cols < n { Some(i + cols) } else { None };
            if let Some(r) = right {
                set_edge(&mut grid, n, i, r);
            }
            if let Some(dw) = down {
                set_edge(&mut grid, n, i, dw);
            }
        }
        topo_names.push("grid".into());
        topo_phis.push(phi_of_adj(&grid, n));
        {
            let mut ec = 0usize;
            for i in 0..n { for j in (i+1)..n { if grid[i * n + j] { ec += 1; } } }
            topo_edges.push(ec as f64);
        }

        // Topology 5: Hybrid (ring + star)
        let mut hybrid = ring.clone();
        for i in 1..n {
            set_edge(&mut hybrid, n, 0, i);
        }
        topo_names.push("ring_star_hybrid".into());
        topo_phis.push(phi_of_adj(&hybrid, n));
        {
            let mut ec = 0usize;
            for i in 0..n { for j in (i+1)..n { if hybrid[i * n + j] { ec += 1; } } }
            topo_edges.push(ec as f64);
        }

        // Find best topology
        let best_idx = topo_phis
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
            .map(|(i, _)| i)
            .unwrap_or(0);

        result.insert("topology_names_encoded".into(), (0..topo_names.len()).map(|i| i as f64).collect());
        result.insert("phi_values".into(), topo_phis);
        result.insert("edge_counts".into(), topo_edges);
        result.insert("best_topology_index".into(), vec![best_idx as f64]);
        result
    }
}

/// 17. ContinuousDeformEngine — Continuously deform topology, measure Phi preservation.
pub struct ContinuousDeformEngine {
    pub steps: usize,
}

impl Default for ContinuousDeformEngine {
    fn default() -> Self {
        Self { steps: 20 }
    }
}

impl TopologyExplorer for ContinuousDeformEngine {
    fn name(&self) -> &str {
        "ContinuousDeformEngine"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }

        // Start with original data, continuously deform by interpolating toward
        // a uniform sphere, measure Phi at each step.
        let dm_orig = dist_matrix(data, n, d);

        // Target: uniform points on unit sphere (deterministic)
        let mut target = vec![0.0; n * d];
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            target[i * d] = angle.cos();
            if d > 1 {
                target[i * d + 1] = angle.sin();
            }
            for k in 2..d {
                let a2 = angle * (k as f64 + 0.5);
                target[i * d + k] = a2.sin() * 0.5;
            }
        }

        let mut t_values = Vec::new();
        let mut phi_values = Vec::new();

        for step in 0..=self.steps {
            let t = step as f64 / self.steps as f64;
            // Interpolate
            let mut interp = vec![0.0; n * d];
            for i in 0..n * d {
                interp[i] = (1.0 - t) * data[i] + t * target[i];
            }

            // Phi proxy via distance matrix comparison
            let dm_interp = dist_matrix(&interp, n, d);

            // Correlation between original and deformed distance matrices
            let mut pairs_orig = Vec::new();
            let mut pairs_interp = Vec::new();
            for i in 0..n {
                for j in (i + 1)..n {
                    pairs_orig.push(dm_orig[i * n + j]);
                    pairs_interp.push(dm_interp[i * n + j]);
                }
            }

            let mo = mean(&pairs_orig);
            let mi = mean(&pairs_interp);
            let mut cov_oi = 0.0;
            let mut var_o = 0.0;
            let mut var_i = 0.0;
            for k in 0..pairs_orig.len() {
                let do_ = pairs_orig[k] - mo;
                let di_ = pairs_interp[k] - mi;
                cov_oi += do_ * di_;
                var_o += do_ * do_;
                var_i += di_ * di_;
            }
            let corr = if var_o > 1e-15 && var_i > 1e-15 {
                cov_oi / (var_o.sqrt() * var_i.sqrt())
            } else {
                0.0
            };

            t_values.push(t);
            phi_values.push(corr);
        }

        // Find breaking point: where correlation drops below 0.5
        let breaking_t = t_values
            .iter()
            .zip(phi_values.iter())
            .find(|(_, &p)| p < 0.5)
            .map(|(&t, _)| t)
            .unwrap_or(1.0);

        result.insert("deformation_t".into(), t_values);
        result.insert("phi_preservation".into(), phi_values);
        result.insert("breaking_point".into(), vec![breaking_t]);
        result
    }
}

// ═════════════════════════════════════════════════════════════
// 8 TOPOLOGY MODULES (infrastructure)
// ═════════════════════════════════════════════════════════════

/// 18. TopologyLibrary — Registry of known topologies with predicted Phi.
pub struct TopologyLibrary;

impl TopologyExplorer for TopologyLibrary {
    fn name(&self) -> &str {
        "TopologyLibrary"
    }

    fn explore(&self, _data: &[f64], n: usize, _d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 2 {
            return result;
        }

        // Known topologies and their predicted Phi scaling
        // Based on consciousness network research (anima Laws 33-39)
        let topos = vec![
            ("ring", 0.8),
            ("small_world", 1.2),
            ("scale_free", 0.9),
            ("hypercube", 1.4),
            ("grid_2d", 0.7),
            ("torus", 1.0),
            ("star", 0.3),
            ("complete", 0.5),
            ("tree", 0.4),
            ("random_er", 0.6),
        ];

        let mut predicted_phi = Vec::new();
        let mut topo_indices = Vec::new();

        for (i, (_, scaling)) in topos.iter().enumerate() {
            predicted_phi.push(scaling * n as f64);
            topo_indices.push(i as f64);
        }

        result.insert("topology_index".into(), topo_indices);
        result.insert("predicted_phi".into(), predicted_phi);
        result.insert("num_topologies".into(), vec![topos.len() as f64]);
        result.insert("n".into(), vec![n as f64]);
        result
    }
}

/// 19. TopologyCompare — Quantitative comparison (spectral distance between adjacency matrices).
pub struct TopologyCompare;

impl TopologyExplorer for TopologyCompare {
    fn name(&self) -> &str {
        "TopologyCompare"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d < 2 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Build adjacency at two thresholds and compare spectrally
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let t1 = dists[dists.len() / 3];
        let t2 = dists[2 * dists.len() / 3];

        // Adjacency matrices as f64
        let mut adj1 = vec![0.0; n * n];
        let mut adj2 = vec![0.0; n * n];
        for i in 0..n {
            for j in (i + 1)..n {
                if dm[i * n + j] <= t1 {
                    adj1[i * n + j] = 1.0;
                    adj1[j * n + i] = 1.0;
                }
                if dm[i * n + j] <= t2 {
                    adj2[i * n + j] = 1.0;
                    adj2[j * n + i] = 1.0;
                }
            }
        }

        // Spectral distance: L2 distance between eigenvalue vectors
        let nn = n.min(16); // cap for eigenvalue computation
        let eigs1 = if nn <= n {
            eigenvalues_symmetric(&adj1[..nn * nn], nn)
        } else {
            vec![]
        };
        let eigs2 = if nn <= n {
            eigenvalues_symmetric(&adj2[..nn * nn], nn)
        } else {
            vec![]
        };

        let spectral_dist: f64 = eigs1
            .iter()
            .zip(eigs2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        // Frobenius distance between adjacency matrices
        let frob: f64 = adj1
            .iter()
            .zip(adj2.iter())
            .map(|(a, b)| (a - b).powi(2))
            .sum::<f64>()
            .sqrt();

        // Edge overlap (Jaccard)
        let mut both = 0.0;
        let mut either = 0.0;
        for i in 0..n {
            for j in (i + 1)..n {
                let a = adj1[i * n + j] > 0.5;
                let b = adj2[i * n + j] > 0.5;
                if a && b {
                    both += 1.0;
                }
                if a || b {
                    either += 1.0;
                }
            }
        }
        let jaccard = if either > 0.0 {
            both / either
        } else {
            1.0
        };

        result.insert("spectral_distance".into(), vec![spectral_dist]);
        result.insert("frobenius_distance".into(), vec![frob]);
        result.insert("jaccard_similarity".into(), vec![jaccard]);
        result.insert("eigenvalues_t1".into(), eigs1);
        result.insert("eigenvalues_t2".into(), eigs2);
        result
    }
}

/// 20. TopologyGenerator — Random/regular/hybrid topology generation.
pub struct TopologyGenerator;

impl TopologyExplorer for TopologyGenerator {
    fn name(&self) -> &str {
        "TopologyGenerator"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }

        // Generate Erdos-Renyi adjacency with p = average_density from data
        let dm = dist_matrix(data, n, d);
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let median = dists[dists.len() / 2];

        // Data-derived density
        let actual_edges = dists.iter().filter(|&&x| x <= median).count();
        let max_edges = n * (n - 1) / 2;
        let p = actual_edges as f64 / max_edges as f64;

        // ER model: deterministic edge selection based on data hash
        let mut er_adj = vec![0.0; n * n];
        let mut er_edges = 0;
        for i in 0..n {
            for j in (i + 1)..n {
                // Use distance as pseudo-random selector
                let normalized = dm[i * n + j] / (dists.last().unwrap_or(&1.0) + 1e-15);
                if normalized <= p {
                    er_adj[i * n + j] = 1.0;
                    er_adj[j * n + i] = 1.0;
                    er_edges += 1;
                }
            }
        }

        // Barabasi-Albert model: preferential attachment
        let mut ba_adj = vec![0.0; n * n];
        let m0 = 3.min(n); // initial complete clique
        let mut degrees = vec![0usize; n];
        for i in 0..m0 {
            for j in (i + 1)..m0 {
                ba_adj[i * n + j] = 1.0;
                ba_adj[j * n + i] = 1.0;
                degrees[i] += 1;
                degrees[j] += 1;
            }
        }
        for new_node in m0..n {
            // Connect to m0-1 existing nodes with highest degree (deterministic)
            let mut candidates: Vec<(usize, usize)> =
                (0..new_node).map(|i| (i, degrees[i])).collect();
            candidates.sort_by(|a, b| b.1.cmp(&a.1));
            let connects = (m0 - 1).min(candidates.len());
            for k in 0..connects {
                let target = candidates[k].0;
                ba_adj[new_node * n + target] = 1.0;
                ba_adj[target * n + new_node] = 1.0;
                degrees[new_node] += 1;
                degrees[target] += 1;
            }
        }
        let ba_edges: usize = {
            let mut c = 0usize;
            for i in 0..n { for j in (i+1)..n { if ba_adj[i * n + j] > 0.5 { c += 1; } } }
            c
        };

        // Watts-Strogatz model: ring + rewiring
        let k_ws = 4.min(n - 1);
        let mut ws_adj = vec![0.0; n * n];
        for i in 0..n {
            for s in 1..=(k_ws / 2) {
                let j = (i + s) % n;
                ws_adj[i * n + j] = 1.0;
                ws_adj[j * n + i] = 1.0;
            }
        }
        // Rewire based on data distances (deterministic)
        for i in 0..n {
            for s in 1..=(k_ws / 2) {
                let j = (i + s) % n;
                // Rewire probability based on distance to farther node
                let far = (i + n / 2) % n;
                if dm[i * n + far] < dm[i * n + j] && far != i {
                    ws_adj[i * n + j] = 0.0;
                    ws_adj[j * n + i] = 0.0;
                    ws_adj[i * n + far] = 1.0;
                    ws_adj[far * n + i] = 1.0;
                }
            }
        }
        let ws_edges: usize = {
            let mut c = 0usize;
            for i in 0..n { for j in (i+1)..n { if ws_adj[i * n + j] > 0.5 { c += 1; } } }
            c
        };

        result.insert("er_edges".into(), vec![er_edges as f64]);
        result.insert("er_density".into(), vec![p]);
        result.insert("ba_edges".into(), vec![ba_edges as f64]);
        result.insert("ws_edges".into(), vec![ws_edges as f64]);
        result.insert("n".into(), vec![n as f64]);
        result
    }
}

/// 21. TopologyVisualizer — ASCII topology visualization.
pub struct TopologyVisualizer;

impl TopologyExplorer for TopologyVisualizer {
    fn name(&self) -> &str {
        "TopologyVisualizer"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 2 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Build adjacency at median
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let threshold = dists[dists.len() / 2];

        // Degree distribution for ASCII histogram
        let adj = adjacency_at_threshold(&dm, n, threshold);
        let degrees: Vec<f64> = adj.iter().map(|a| a.len() as f64).collect();
        let max_deg = degrees.iter().cloned().fold(0.0f64, f64::max) as usize;

        // Degree histogram (bins 0..max_deg)
        let num_bins = (max_deg + 1).min(20);
        let mut histogram = vec![0.0; num_bins];
        for &deg in &degrees {
            let bin = (deg as usize).min(num_bins - 1);
            histogram[bin] += 1.0;
        }

        // Adjacency matrix density per row (for visualization)
        let row_density: Vec<f64> = (0..n)
            .map(|i| {
                (0..n)
                    .filter(|&j| dm[i * n + j] <= threshold && i != j)
                    .count() as f64
                    / (n - 1).max(1) as f64
            })
            .collect();

        let total_edges = count_edges(&adj) as f64;
        let density = total_edges / (n * (n - 1) / 2).max(1) as f64;

        result.insert("degree_histogram".into(), histogram);
        result.insert("row_density".into(), row_density);
        result.insert("total_edges".into(), vec![total_edges]);
        result.insert("density".into(), vec![density]);
        result.insert("max_degree".into(), vec![max_deg as f64]);
        result
    }
}

/// 22. TopologyPredictor — Predict Phi from topology without running engine.
pub struct TopologyPredictor;

impl TopologyExplorer for TopologyPredictor {
    fn name(&self) -> &str {
        "TopologyPredictor"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Features for Phi prediction (based on Law 22: structure -> Phi)
        let mut dists: Vec<f64> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                dists.push(dm[i * n + j]);
            }
        }
        dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let threshold = dists[dists.len() / 2];

        let adj = adjacency_at_threshold(&dm, n, threshold);
        let degrees: Vec<f64> = adj.iter().map(|a| a.len() as f64).collect();
        let deg_mean = mean(&degrees);
        let deg_var = variance(&degrees);

        let edges = count_edges(&adj) as f64;
        let triangles = count_triangles(&adj, n) as f64;

        // Clustering coefficient
        let clustering: f64 = (0..n)
            .map(|i| {
                let k = adj[i].len();
                if k < 2 {
                    return 0.0;
                }
                let mut t = 0;
                for &a in &adj[i] {
                    for &b in &adj[i] {
                        if a < b && adj[a].contains(&b) {
                            t += 1;
                        }
                    }
                }
                2.0 * t as f64 / (k * (k - 1)) as f64
            })
            .sum::<f64>()
            / n as f64;

        // Phi prediction formula (empirical, based on CX106):
        // Phi ~ 0.78 * N * clustering * (1 + deg_var/deg_mean^2)
        let phi_predicted = 0.78
            * n as f64
            * (clustering + 0.1)
            * (1.0 + deg_var / (deg_mean * deg_mean + 1e-15));

        // Confidence based on how well-connected the graph is
        let confidence = (edges / (n * (n - 1) / 2).max(1) as f64).min(1.0);

        result.insert("predicted_phi".into(), vec![phi_predicted]);
        result.insert("confidence".into(), vec![confidence]);
        result.insert("clustering_coefficient".into(), vec![clustering]);
        result.insert("degree_mean".into(), vec![deg_mean]);
        result.insert("degree_variance".into(), vec![deg_var]);
        result.insert("edge_count".into(), vec![edges]);
        result.insert("triangle_count".into(), vec![triangles]);
        result
    }
}

/// 23. TopologyConstraint — Hardware/memory/latency constraints checker.
pub struct TopologyConstraint {
    pub max_memory_bytes: usize,
    pub max_latency_ms: f64,
    pub max_edges: usize,
}

impl Default for TopologyConstraint {
    fn default() -> Self {
        Self {
            max_memory_bytes: 1_000_000_000, // 1GB
            max_latency_ms: 100.0,
            max_edges: 1_000_000,
        }
    }
}

impl TopologyExplorer for TopologyConstraint {
    fn name(&self) -> &str {
        "TopologyConstraint"
    }

    fn explore(&self, _data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 2 || d == 0 {
            return result;
        }

        // Memory estimate: adjacency matrix (n*n * 8 bytes for f64)
        let adj_memory = n * n * 8;
        // Distance matrix memory
        let dist_memory = n * n * 8;
        // State vectors: n * d * 8
        let state_memory = n * d * 8;
        let total_memory = adj_memory + dist_memory + state_memory;

        let memory_ok = total_memory <= self.max_memory_bytes;

        // Latency estimate: O(n^2) for distance computation
        let ops = (n * n * d) as f64;
        let estimated_latency_ms = ops / 1e9 * 1000.0; // rough: 1 GFLOP/s baseline
        let latency_ok = estimated_latency_ms <= self.max_latency_ms;

        // Edge constraint
        let max_possible_edges = n * (n - 1) / 2;
        let edges_ok = max_possible_edges <= self.max_edges;

        // Maximum feasible n given constraints
        let max_n_memory = ((self.max_memory_bytes / 24) as f64).sqrt() as usize;
        let max_n_latency = ((self.max_latency_ms * 1e6 / d as f64).sqrt()) as usize;
        let max_n_edges = ((2.0 * self.max_edges as f64).sqrt()) as usize;
        let max_n = max_n_memory.min(max_n_latency).min(max_n_edges);

        let all_ok = memory_ok && latency_ok && edges_ok;

        result.insert("total_memory_bytes".into(), vec![total_memory as f64]);
        result.insert("memory_ok".into(), vec![if memory_ok { 1.0 } else { 0.0 }]);
        result.insert(
            "estimated_latency_ms".into(),
            vec![estimated_latency_ms],
        );
        result.insert("latency_ok".into(), vec![if latency_ok { 1.0 } else { 0.0 }]);
        result.insert("max_possible_edges".into(), vec![max_possible_edges as f64]);
        result.insert("edges_ok".into(), vec![if edges_ok { 1.0 } else { 0.0 }]);
        result.insert("all_constraints_ok".into(), vec![if all_ok { 1.0 } else { 0.0 }]);
        result.insert("max_feasible_n".into(), vec![max_n as f64]);
        result
    }
}

/// 24. PersistentHomology — Simplified persistent homology (birth-death pairs).
pub struct PersistentHomology;

impl TopologyExplorer for PersistentHomology {
    fn name(&self) -> &str {
        "PersistentHomology"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 3 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Vietoris-Rips filtration (H0 only for efficiency)
        let mut edges: Vec<(usize, usize, f64)> = Vec::new();
        for i in 0..n {
            for j in (i + 1)..n {
                edges.push((i, j, dm[i * n + j]));
            }
        }
        edges.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));

        let mut parent: Vec<usize> = (0..n).collect();
        let mut rank_arr = vec![0usize; n];
        let birth_times = vec![0.0; n]; // all born at 0
        let mut death_times = vec![f64::INFINITY; n];

        let mut births = Vec::new();
        let mut deaths = Vec::new();
        let mut persistence = Vec::new();

        for &(i, j, dist_val) in &edges {
            let ri = uf_find(&mut parent, i);
            let rj = uf_find(&mut parent, j);
            if ri != rj {
                // Younger component dies (higher birth time, or arbitrary if same)
                let dying = if birth_times[ri] > birth_times[rj] {
                    ri
                } else {
                    rj
                };
                death_times[dying] = dist_val;
                births.push(birth_times[dying]);
                deaths.push(dist_val);
                persistence.push(dist_val - birth_times[dying]);
                uf_union(&mut parent, &mut rank_arr, ri, rj);
            }
        }

        // Sort persistence by lifetime (most persistent first)
        let mut indexed: Vec<(usize, f64)> =
            persistence.iter().enumerate().map(|(i, &p)| (i, p)).collect();
        indexed.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let total_persistence: f64 = persistence.iter().sum();
        let max_persistence = persistence.iter().cloned().fold(0.0f64, f64::max);
        let mean_persistence = mean(&persistence);

        // Wasserstein distance proxy (from empty diagram): sum of all persistence values
        let wasserstein = total_persistence;

        // Persistence entropy
        let p_entropy = if total_persistence > 1e-15 {
            persistence
                .iter()
                .map(|&p| {
                    let pp = p / total_persistence;
                    if pp > 1e-15 {
                        -pp * pp.ln()
                    } else {
                        0.0
                    }
                })
                .sum::<f64>()
        } else {
            0.0
        };

        result.insert("births".into(), births);
        result.insert("deaths".into(), deaths);
        result.insert("persistence".into(), persistence);
        result.insert("total_persistence".into(), vec![total_persistence]);
        result.insert("max_persistence".into(), vec![max_persistence]);
        result.insert("mean_persistence".into(), vec![mean_persistence]);
        result.insert("wasserstein_proxy".into(), vec![wasserstein]);
        result.insert("persistence_entropy".into(), vec![p_entropy]);
        result
    }
}

/// 25. TopologyBridge — Inter-topology bridge (connect different topology regions).
pub struct TopologyBridge;

impl TopologyExplorer for TopologyBridge {
    fn name(&self) -> &str {
        "TopologyBridge"
    }

    fn explore(&self, data: &[f64], n: usize, d: usize) -> ExploreResult {
        let mut result = ExploreResult::new();
        if n < 6 || d == 0 {
            return result;
        }
        let dm = dist_matrix(data, n, d);

        // Split data into two regions (by first principal axis)
        let median_val = {
            let mut col0: Vec<f64> = (0..n).map(|i| data[i * d]).collect();
            col0.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            col0[n / 2]
        };

        let region_a: Vec<usize> = (0..n).filter(|&i| data[i * d] <= median_val).collect();
        let region_b: Vec<usize> = (0..n).filter(|&i| data[i * d] > median_val).collect();

        if region_a.is_empty() || region_b.is_empty() {
            return result;
        }

        // Intra-region distances
        let intra_a: f64 = {
            let mut s = 0.0;
            let mut c = 0;
            for i in 0..region_a.len() {
                for j in (i + 1)..region_a.len() {
                    s += dm[region_a[i] * n + region_a[j]];
                    c += 1;
                }
            }
            if c > 0 { s / c as f64 } else { 0.0 }
        };

        let intra_b: f64 = {
            let mut s = 0.0;
            let mut c = 0;
            for i in 0..region_b.len() {
                for j in (i + 1)..region_b.len() {
                    s += dm[region_b[i] * n + region_b[j]];
                    c += 1;
                }
            }
            if c > 0 { s / c as f64 } else { 0.0 }
        };

        // Inter-region distances
        let mut cross_dists: Vec<(usize, usize, f64)> = Vec::new();
        for &i in &region_a {
            for &j in &region_b {
                cross_dists.push((i, j, dm[i * n + j]));
            }
        }
        cross_dists.sort_by(|a, b| a.2.partial_cmp(&b.2).unwrap_or(std::cmp::Ordering::Equal));
        let inter_mean = mean(
            &cross_dists
                .iter()
                .map(|&(_, _, d_val)| d_val)
                .collect::<Vec<f64>>(),
        );

        // Bridge nodes: k shortest cross-edges
        let k_bridges = (n / 6).max(1).min(cross_dists.len());
        let bridge_distances: Vec<f64> = cross_dists
            .iter()
            .take(k_bridges)
            .map(|&(_, _, d_val)| d_val)
            .collect();

        // Bridge quality: how much shorter are bridges vs average crossing
        let bridge_quality = if !bridge_distances.is_empty() {
            inter_mean / (mean(&bridge_distances) + 1e-15)
        } else {
            0.0
        };

        // Integration score: bridges should make the whole graph more connected
        // Ratio of inter to intra distance (lower = better connected)
        let integration = (intra_a + intra_b) / (2.0 * inter_mean + 1e-15);

        result.insert("region_a_size".into(), vec![region_a.len() as f64]);
        result.insert("region_b_size".into(), vec![region_b.len() as f64]);
        result.insert("intra_a_distance".into(), vec![intra_a]);
        result.insert("intra_b_distance".into(), vec![intra_b]);
        result.insert("inter_distance".into(), vec![inter_mean]);
        result.insert("bridge_distances".into(), bridge_distances);
        result.insert("bridge_quality".into(), vec![bridge_quality]);
        result.insert("integration_score".into(), vec![integration]);
        result.insert("num_bridges".into(), vec![k_bridges as f64]);
        result
    }
}

// ═════════════════════════════════════════════════════════════
// Convenience: run all explorers
// ═════════════════════════════════════════════════════════════

/// Returns all 25 topology explorers.
pub fn all_explorers() -> Vec<Box<dyn TopologyExplorer>> {
    vec![
        // 10 Lenses
        Box::new(TopologyDiscoveryLens),
        Box::new(DimensionLens),
        Box::new(FoldingLens),
        Box::new(HoleLens),
        Box::new(BoundaryLens),
        Box::new(SymmetryBreakingLens),
        Box::new(ConnectednessLens),
        Box::new(CurvatureLens),
        Box::new(EmbeddingLens),
        Box::new(MorphismLens),
        // 7 Engines
        Box::new(TopologyEvolver::default()),
        Box::new(DimensionClimber),
        Box::new(FoldingEngine),
        Box::new(SurgeryEngine),
        Box::new(StackingEngine),
        Box::new(HybridTopologyEngine),
        Box::new(ContinuousDeformEngine::default()),
        // 8 Modules
        Box::new(TopologyLibrary),
        Box::new(TopologyCompare),
        Box::new(TopologyGenerator),
        Box::new(TopologyVisualizer),
        Box::new(TopologyPredictor),
        Box::new(TopologyConstraint::default()),
        Box::new(PersistentHomology),
        Box::new(TopologyBridge),
    ]
}

/// Run all explorers on data, return combined results keyed by explorer name.
pub fn explore_all(
    data: &[f64],
    n: usize,
    d: usize,
) -> HashMap<String, ExploreResult> {
    let mut combined = HashMap::new();
    for explorer in all_explorers() {
        let name = explorer.name().to_string();
        let res = explorer.explore(data, n, d);
        combined.insert(name, res);
    }
    combined
}

// ═════════════════════════════════════════════════════════════
// TESTS
// ═════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    /// Generate a simple ring dataset (n points on a circle in 2D).
    fn ring_data(n: usize) -> Vec<f64> {
        let mut data = vec![0.0; n * 2];
        for i in 0..n {
            let angle = 2.0 * std::f64::consts::PI * i as f64 / n as f64;
            data[i * 2] = angle.cos();
            data[i * 2 + 1] = angle.sin();
        }
        data
    }

    /// Generate a cluster dataset (two clusters).
    fn cluster_data(n: usize) -> Vec<f64> {
        let mut data = vec![0.0; n * 3];
        let half = n / 2;
        for i in 0..half {
            data[i * 3] = i as f64 * 0.1;
            data[i * 3 + 1] = i as f64 * 0.05;
            data[i * 3 + 2] = 0.0;
        }
        for i in half..n {
            data[i * 3] = 10.0 + (i - half) as f64 * 0.1;
            data[i * 3 + 1] = 10.0 + (i - half) as f64 * 0.05;
            data[i * 3 + 2] = 5.0;
        }
        data
    }

    #[test]
    fn test_topology_discovery_lens() {
        let data = ring_data(12);
        let lens = TopologyDiscoveryLens;
        let res = lens.explore(&data, 12, 2);
        assert!(res.contains_key("euler_characteristic"));
        assert!(res.contains_key("betti_0"));
        assert!(res.contains_key("classification"));
        assert!(!res["euler_characteristic"].is_empty());
    }

    #[test]
    fn test_dimension_lens() {
        let data = ring_data(10);
        let lens = DimensionLens;
        let res = lens.explore(&data, 10, 2);
        assert!(res.contains_key("correlation_dimension"));
        assert!(res.contains_key("pca_effective_rank"));
        let rank = res["pca_effective_rank"][0];
        assert!(rank > 0.0, "effective rank should be positive");
    }

    #[test]
    fn test_folding_lens() {
        let data = ring_data(16);
        let lens = FoldingLens;
        let res = lens.explore(&data, 16, 2);
        assert!(res.contains_key("fold_improvement"));
        assert!(res.contains_key("mean_local_density"));
    }

    #[test]
    fn test_hole_lens() {
        let data = ring_data(10);
        let lens = HoleLens;
        let res = lens.explore(&data, 10, 2);
        assert!(res.contains_key("h1_cycle_count"));
        assert!(res["h0_components_at_start"][0] == 10.0);
    }

    #[test]
    fn test_boundary_lens() {
        let data = cluster_data(12);
        let lens = BoundaryLens;
        let res = lens.explore(&data, 12, 3);
        assert!(res.contains_key("boundary_ratio"));
        let ratio = res["boundary_ratio"][0];
        assert!(ratio >= 0.0 && ratio <= 1.0);
    }

    #[test]
    fn test_symmetry_breaking_lens() {
        let data = cluster_data(12);
        let lens = SymmetryBreakingLens;
        let res = lens.explore(&data, 12, 3);
        assert!(res.contains_key("gini_coefficient"));
        assert!(res.contains_key("breaking_score"));
    }

    #[test]
    fn test_connectedness_lens() {
        let data = ring_data(10);
        let lens = ConnectednessLens;
        let res = lens.explore(&data, 10, 2);
        assert!(res.contains_key("critical_threshold"));
        assert!(res.contains_key("robustness"));
        assert!(res["robustness"][0] >= 0.0);
    }

    #[test]
    fn test_curvature_lens() {
        let data = ring_data(10);
        let lens = CurvatureLens;
        let res = lens.explore(&data, 10, 2);
        assert!(res.contains_key("mean_curvature"));
        assert!(res.contains_key("curvatures"));
        assert_eq!(res["curvatures"].len(), 10);
    }

    #[test]
    fn test_embedding_lens() {
        let data = ring_data(10);
        let lens = EmbeddingLens;
        let res = lens.explore(&data, 10, 2);
        assert!(res.contains_key("stress"));
        assert!(res.contains_key("trustworthiness"));
        // 2D->2D projection should be near-perfect
        let trust = res["trustworthiness"][0];
        assert!(trust > 0.5, "2D->2D trust should be high, got {}", trust);
    }

    #[test]
    fn test_morphism_lens() {
        let data = ring_data(10);
        let lens = MorphismLens;
        let res = lens.explore(&data, 10, 2);
        assert!(res.contains_key("betti_stability"));
        assert!(res.contains_key("completeness"));
    }

    #[test]
    fn test_topology_evolver() {
        let data = ring_data(8);
        let engine = TopologyEvolver {
            generations: 5,
            mutation_rate: 0.1,
        };
        let res = engine.explore(&data, 8, 2);
        assert!(res.contains_key("best_phi_proxy"));
        assert!(res.contains_key("fitness_history"));
        assert_eq!(res["fitness_history"].len(), 6); // initial + 5 generations
    }

    #[test]
    fn test_dimension_climber() {
        let mut data = vec![0.0; 10 * 4]; // 10 points x 4D
        for i in 0..10 {
            for k in 0..4 {
                data[i * 4 + k] = ((i * 7 + k * 3) as f64).sin();
            }
        }
        let engine = DimensionClimber;
        let res = engine.explore(&data, 10, 4);
        assert!(res.contains_key("optimal_dimension"));
        assert!(res["optimal_dimension"][0] >= 1.0);
    }

    #[test]
    fn test_folding_engine() {
        let data = ring_data(12);
        let engine = FoldingEngine;
        let res = engine.explore(&data, 12, 2);
        assert!(res.contains_key("best_width"));
        assert!(res.contains_key("improvement"));
    }

    #[test]
    fn test_surgery_engine() {
        let data = ring_data(12);
        let engine = SurgeryEngine;
        let res = engine.explore(&data, 12, 2);
        assert!(res.contains_key("original_phi"));
        assert!(res.contains_key("phi_change"));
        assert_eq!(res["phi_change"].len(), 3);
    }

    #[test]
    fn test_stacking_engine() {
        let data = ring_data(12);
        let engine = StackingEngine;
        let res = engine.explore(&data, 12, 2);
        assert!(res.contains_key("optimal_layers"));
        assert!(res["optimal_layers"][0] >= 1.0);
    }

    #[test]
    fn test_hybrid_topology_engine() {
        let data = ring_data(10);
        let engine = HybridTopologyEngine;
        let res = engine.explore(&data, 10, 2);
        assert!(res.contains_key("phi_values"));
        assert_eq!(res["phi_values"].len(), 5); // ring, star, sw, grid, hybrid
    }

    #[test]
    fn test_continuous_deform_engine() {
        let data = ring_data(8);
        let engine = ContinuousDeformEngine { steps: 5 };
        let res = engine.explore(&data, 8, 2);
        assert!(res.contains_key("breaking_point"));
        assert!(res.contains_key("phi_preservation"));
        assert_eq!(res["phi_preservation"].len(), 6); // 0..=5
        // At t=0, preservation should be 1.0 (no deformation)
        assert!((res["phi_preservation"][0] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_topology_library() {
        let lib = TopologyLibrary;
        let res = lib.explore(&[], 16, 0);
        assert!(res.contains_key("predicted_phi"));
        assert_eq!(res["num_topologies"][0], 10.0);
    }

    #[test]
    fn test_topology_compare() {
        let data = ring_data(10);
        let cmp = TopologyCompare;
        let res = cmp.explore(&data, 10, 2);
        assert!(res.contains_key("spectral_distance"));
        assert!(res.contains_key("jaccard_similarity"));
    }

    #[test]
    fn test_topology_generator() {
        let data = ring_data(10);
        let gen = TopologyGenerator;
        let res = gen.explore(&data, 10, 2);
        assert!(res.contains_key("er_edges"));
        assert!(res.contains_key("ba_edges"));
        assert!(res.contains_key("ws_edges"));
    }

    #[test]
    fn test_topology_visualizer() {
        let data = ring_data(10);
        let vis = TopologyVisualizer;
        let res = vis.explore(&data, 10, 2);
        assert!(res.contains_key("degree_histogram"));
        assert!(res.contains_key("density"));
    }

    #[test]
    fn test_topology_predictor() {
        let data = ring_data(10);
        let pred = TopologyPredictor;
        let res = pred.explore(&data, 10, 2);
        assert!(res.contains_key("predicted_phi"));
        assert!(res["predicted_phi"][0] > 0.0);
        assert!(res.contains_key("confidence"));
    }

    #[test]
    fn test_topology_constraint() {
        let data = ring_data(10);
        let tc = TopologyConstraint::default();
        let res = tc.explore(&data, 10, 2);
        assert!(res.contains_key("all_constraints_ok"));
        assert_eq!(res["all_constraints_ok"][0], 1.0); // small data should pass
        assert!(res.contains_key("max_feasible_n"));
    }

    #[test]
    fn test_persistent_homology() {
        let data = ring_data(10);
        let ph = PersistentHomology;
        let res = ph.explore(&data, 10, 2);
        assert!(res.contains_key("persistence"));
        assert!(res.contains_key("persistence_entropy"));
        assert!(res["total_persistence"][0] > 0.0);
    }

    #[test]
    fn test_topology_bridge() {
        let data = cluster_data(12);
        let bridge = TopologyBridge;
        let res = bridge.explore(&data, 12, 3);
        assert!(res.contains_key("bridge_quality"));
        assert!(res.contains_key("integration_score"));
        assert!(res["region_a_size"][0] > 0.0);
        assert!(res["region_b_size"][0] > 0.0);
    }

    #[test]
    fn test_all_explorers_count() {
        let explorers = all_explorers();
        assert_eq!(explorers.len(), 25);
    }

    #[test]
    fn test_explore_all() {
        let data = ring_data(10);
        let results = explore_all(&data, 10, 2);
        assert_eq!(results.len(), 25);
        for (name, res) in &results {
            assert!(!res.is_empty(), "{} returned empty result", name);
        }
    }

    #[test]
    fn test_edge_case_small_n() {
        let data = vec![1.0, 2.0]; // 1 point x 2D
        for explorer in all_explorers() {
            let res = explorer.explore(&data, 1, 2);
            // Should not panic, may return empty
            let _ = res;
        }
    }

    #[test]
    fn test_edge_case_zero_data() {
        let data = vec![0.0; 20]; // 10 points x 2D, all zeros
        let lens = TopologyDiscoveryLens;
        let res = lens.explore(&data, 10, 2);
        // Should handle degenerate case without panic
        assert!(res.contains_key("euler_characteristic"));
    }
}
