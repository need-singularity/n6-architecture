use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// CombinatorialLens: Enumerate and score discrete design-space combinations.
///
/// Algorithm:
///   1. Discretize each dimension into K bins (levels), forming a "design signature"
///      per data point (which bin it falls into on each axis).
///   2. Count unique design signatures → coverage ratio = unique / total.
///   3. Compute Shannon entropy of signature frequencies → combination diversity.
///   4. For each populated signature, measure the mean intra-cluster compactness
///      (mean pairwise distance among points sharing the same signature).
///   5. Compute a Pareto dominance count: for every pair of unique signatures,
///      check if one dominates the other (all dimension-centroids ≤ or ≥).
///   6. Output coverage, diversity, compactness, dominance fraction, and per-signature scores.
pub struct UcombinatorialLens;

impl UcombinatorialLens {
    /// Number of bins per dimension for discretization.
    const NUM_BINS: usize = 6; // n=6

    /// Discretize a value into a bin index given the column min and range.
    #[inline]
    fn bin_index(value: f64, col_min: f64, col_range: f64) -> usize {
        if col_range < 1e-15 {
            return 0;
        }
        let idx = ((value - col_min) / col_range * Self::NUM_BINS as f64) as usize;
        idx.min(Self::NUM_BINS - 1)
    }

    /// Encode a multi-dimensional bin signature as a single u64 key.
    /// Each dimension gets ceil(log2(NUM_BINS)) = 3 bits, supporting up to 21 dimensions
    /// in a u64. For higher d, we wrap (which is fine—collisions just merge clusters).
    #[inline]
    fn encode_signature(bins: &[usize]) -> u64 {
        let mut key: u64 = 0;
        for (i, &b) in bins.iter().enumerate() {
            let shift = (i * 3) % 64;
            key ^= (b as u64) << shift;
        }
        key
    }
}

impl Lens for UcombinatorialLens {
    fn name(&self) -> &str {
        "combinatorial"
    }

    fn category(&self) -> &str {
        "DSE"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 2 || d < 1 {
            return HashMap::new();
        }

        // 1. Compute per-column min and range for binning
        let mut col_min = vec![f64::INFINITY; d];
        let mut col_max = vec![f64::NEG_INFINITY; d];
        for i in 0..n {
            let row = i * d;
            for j in 0..d {
                let v = data[row + j];
                if v < col_min[j] { col_min[j] = v; }
                if v > col_max[j] { col_max[j] = v; }
            }
        }
        let col_range: Vec<f64> = (0..d).map(|j| col_max[j] - col_min[j]).collect();

        // 2. Assign each point a discrete design signature
        let mut signatures: Vec<u64> = Vec::with_capacity(n);
        let mut bin_vecs: Vec<Vec<usize>> = Vec::with_capacity(n);
        for i in 0..n {
            let row = i * d;
            let bins: Vec<usize> = (0..d)
                .map(|j| Self::bin_index(data[row + j], col_min[j], col_range[j]))
                .collect();
            signatures.push(Self::encode_signature(&bins));
            bin_vecs.push(bins);
        }

        // 3. Group points by signature
        let mut clusters: HashMap<u64, Vec<usize>> = HashMap::new();
        for (i, &sig) in signatures.iter().enumerate() {
            clusters.entry(sig).or_insert_with(Vec::new).push(i);
        }

        let num_unique = clusters.len();
        let max_possible = Self::NUM_BINS.pow(d.min(10) as u32).min(n * 10); // theoretical max
        let coverage = num_unique as f64 / max_possible.max(1) as f64;
        let coverage_ratio = (num_unique as f64 / n as f64).min(1.0);

        // 4. Shannon entropy of signature frequencies → diversity
        let inv_n = 1.0 / n as f64;
        let mut diversity = 0.0;
        for members in clusters.values() {
            let p = members.len() as f64 * inv_n;
            if p > 0.0 {
                diversity -= p * p.ln();
            }
        }
        // Normalize by max entropy (all unique → ln(n))
        let max_entropy = (n as f64).ln();
        let normalized_diversity = if max_entropy > 1e-12 {
            diversity / max_entropy
        } else {
            0.0
        };

        // 5. Intra-cluster compactness: mean pairwise distance within each cluster
        let mut compactness_values: Vec<f64> = Vec::new();
        let mut total_compactness = 0.0;
        let mut compact_count = 0usize;

        for members in clusters.values() {
            if members.len() >= 2 {
                let mut sum_dist = 0.0;
                let mut pair_count = 0usize;
                for a in 0..members.len() {
                    for b in (a + 1)..members.len() {
                        sum_dist += shared.dist(members[a], members[b]);
                        pair_count += 1;
                    }
                }
                let mean_dist = sum_dist / pair_count as f64;
                compactness_values.push(mean_dist);
                total_compactness += mean_dist;
                compact_count += 1;
            }
        }
        let avg_compactness = if compact_count > 0 {
            total_compactness / compact_count as f64
        } else {
            0.0
        };

        // 6. Compute centroids per cluster (mean bin vector as f64)
        let cluster_keys: Vec<u64> = clusters.keys().copied().collect();
        let num_clusters = cluster_keys.len();
        let mut centroids: Vec<Vec<f64>> = Vec::with_capacity(num_clusters);
        for key in &cluster_keys {
            let members = &clusters[key];
            let mut centroid = vec![0.0; d];
            for &idx in members {
                for j in 0..d {
                    centroid[j] += bin_vecs[idx][j] as f64;
                }
            }
            let inv_m = 1.0 / members.len() as f64;
            for j in 0..d {
                centroid[j] *= inv_m;
            }
            centroids.push(centroid);
        }

        // 7. Pareto dominance analysis among cluster centroids
        //    A dominates B if all centroid dimensions of A >= B and at least one is strictly >.
        let mut dominance_count = 0usize;
        let mut dominated = vec![false; num_clusters];
        let total_pairs = if num_clusters >= 2 {
            num_clusters * (num_clusters - 1) / 2
        } else {
            1
        };

        for a in 0..num_clusters {
            for b in (a + 1)..num_clusters {
                let a_dom_b = centroids[a].iter().zip(centroids[b].iter())
                    .all(|(&ca, &cb)| ca >= cb)
                    && centroids[a].iter().zip(centroids[b].iter())
                    .any(|(&ca, &cb)| ca > cb);
                let b_dom_a = centroids[b].iter().zip(centroids[a].iter())
                    .all(|(&cb, &ca)| cb >= ca)
                    && centroids[b].iter().zip(centroids[a].iter())
                    .any(|(&cb, &ca)| cb > ca);
                if a_dom_b {
                    dominance_count += 1;
                    dominated[b] = true;
                }
                if b_dom_a {
                    dominance_count += 1;
                    dominated[a] = true;
                }
            }
        }

        let pareto_front_size = dominated.iter().filter(|&&d| !d).count();
        let dominance_fraction = dominance_count as f64 / total_pairs as f64;

        // 8. Inter-cluster separation: mean distance between cluster centroids
        let mut inter_sep = 0.0;
        let mut sep_count = 0usize;
        for a in 0..num_clusters {
            for b in (a + 1)..num_clusters {
                let dist_sq: f64 = centroids[a].iter().zip(centroids[b].iter())
                    .map(|(&ca, &cb)| (ca - cb) * (ca - cb))
                    .sum();
                inter_sep += dist_sq.sqrt();
                sep_count += 1;
            }
        }
        let avg_separation = if sep_count > 0 {
            inter_sep / sep_count as f64
        } else {
            0.0
        };

        // 9. Composite DSE score: balance between coverage, diversity, and separation
        let dse_score = coverage_ratio * normalized_diversity * (1.0 + avg_separation);

        // Build result
        let mut result = HashMap::new();
        result.insert("coverage_ratio".to_string(), vec![coverage_ratio]);
        result.insert("coverage_absolute".to_string(), vec![coverage, num_unique as f64]);
        result.insert("diversity".to_string(), vec![diversity, normalized_diversity]);
        result.insert("compactness".to_string(), compactness_values);
        result.insert("avg_compactness".to_string(), vec![avg_compactness]);
        result.insert("pareto_front_size".to_string(), vec![pareto_front_size as f64]);
        result.insert("dominance_fraction".to_string(), vec![dominance_fraction]);
        result.insert("avg_separation".to_string(), vec![avg_separation]);
        result.insert("dse_score".to_string(), vec![dse_score]);
        result.insert("num_bins".to_string(), vec![Self::NUM_BINS as f64]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    /// Helper: create test data and SharedData.
    fn make_shared(data: &[f64], n: usize, d: usize) -> SharedData {
        SharedData::compute(data, n, d)
    }

    #[test]
    fn test_scan_returns_non_empty() {
        // 6 points in 2D forming a grid-like pattern
        let data = vec![
            0.0, 0.0,
            1.0, 0.0,
            2.0, 0.0,
            0.0, 1.0,
            1.0, 1.0,
            2.0, 1.0,
        ];
        let n = 6;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = UcombinatorialLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        assert!(result.contains_key("coverage_ratio"));
        assert!(result.contains_key("diversity"));
        assert!(result.contains_key("dse_score"));
        assert!(result.contains_key("pareto_front_size"));

        let coverage = result["coverage_ratio"][0];
        assert!(coverage > 0.0, "coverage must be positive for spread-out data");

        let dse = result["dse_score"][0];
        assert!(dse.is_finite(), "dse_score must be finite");
    }

    #[test]
    fn test_identical_points_low_diversity() {
        // All points identical → single cluster, zero diversity
        let data = vec![
            1.0, 2.0,
            1.0, 2.0,
            1.0, 2.0,
            1.0, 2.0,
        ];
        let n = 4;
        let d = 2;
        let shared = make_shared(&data, n, d);
        let lens = UcombinatorialLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty());

        // All points in same bin → normalized diversity = 0
        let norm_div = result["diversity"][1];
        assert!(
            norm_div < 1e-9,
            "identical points must have ~0 normalized diversity, got {norm_div}"
        );

        // Single cluster → pareto front = 1
        let pareto = result["pareto_front_size"][0];
        assert!(
            (pareto - 1.0).abs() < 1e-9,
            "single cluster must yield pareto front size = 1, got {pareto}"
        );
    }

    #[test]
    fn test_high_diversity_spread_data() {
        // 12 well-separated points across 3D → should have higher diversity than clustered
        let data = vec![
            0.0, 0.0, 0.0,
            5.0, 0.0, 0.0,
            0.0, 5.0, 0.0,
            0.0, 0.0, 5.0,
            5.0, 5.0, 0.0,
            5.0, 0.0, 5.0,
            0.0, 5.0, 5.0,
            5.0, 5.0, 5.0,
            2.5, 2.5, 0.0,
            2.5, 0.0, 2.5,
            0.0, 2.5, 2.5,
            2.5, 2.5, 2.5,
        ];
        let n = 12;
        let d = 3;
        let shared = make_shared(&data, n, d);
        let lens = UcombinatorialLens;
        let result = lens.scan(&data, n, d, &shared);

        let norm_div = result["diversity"][1];
        assert!(
            norm_div > 0.3,
            "spread data should have meaningful diversity, got {norm_div}"
        );

        let pareto = result["pareto_front_size"][0] as usize;
        assert!(
            pareto >= 1,
            "pareto front must have at least 1 non-dominated cluster"
        );
    }
}
