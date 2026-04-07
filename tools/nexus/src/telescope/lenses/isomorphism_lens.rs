use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// IsomorphismLens: Find structural equivalences between seemingly different systems.
///
/// Algorithm:
///   1. Build a "distance profile" for each point: sorted distances to all others.
///   2. Compare profiles pairwise via Spearman rank correlation to measure
///      structural similarity (two points are isomorphic if they sit in
///      equivalent positions within the dataset's geometry).
///   3. Threshold correlation to partition points into equivalence classes
///      (connected components of the high-similarity graph).
///   4. Outputs: mean isomorphism score, equivalence class count + sizes,
///      and a profile entropy measuring structural diversity.
pub struct UisomorphismLens;

impl Lens for UisomorphismLens {
    fn name(&self) -> &str {
        "isomorphism"
    }

    fn category(&self) -> &str {
        "structure"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 3 {
            return HashMap::new();
        }

        // Step 1: Build distance profiles (sorted distances from each point to all others)
        let mut profiles: Vec<Vec<f64>> = Vec::with_capacity(n);
        for i in 0..n {
            let mut dists: Vec<f64> = Vec::with_capacity(n - 1);
            for j in 0..n {
                if i != j {
                    dists.push(shared.dist(i, j));
                }
            }
            dists.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
            profiles.push(dists);
        }

        // Step 2: Pairwise Spearman rank correlation between profiles.
        // Since profiles are already sorted (same ranking = identity permutation),
        // we compare the *values* via Pearson correlation on the sorted distances.
        // Two points with identical local geometry will have correlation ~1.
        let pair_count = n * (n - 1) / 2;
        let mut sim_matrix: Vec<f64> = vec![0.0; pair_count];
        let mut sim_sum = 0.0;

        for i in 0..n {
            for j in (i + 1)..n {
                let corr = pearson(&profiles[i], &profiles[j]);
                let idx = i * (2 * n - i - 1) / 2 + (j - i - 1);
                sim_matrix[idx] = corr;
                sim_sum += corr;
            }
        }

        let mean_iso_score = if pair_count > 0 {
            sim_sum / pair_count as f64
        } else {
            0.0
        };

        // Step 3: Equivalence classes via union-find on high-similarity pairs.
        // Threshold = mean + 0.5 * std of similarities (adaptive).
        let sim_mean = mean_iso_score;
        let sim_var = if pair_count > 1 {
            let mut v = 0.0;
            for &s in &sim_matrix {
                let d = s - sim_mean;
                v += d * d;
            }
            v / pair_count as f64
        } else {
            0.0
        };
        let threshold = (sim_mean + 0.5 * sim_var.sqrt()).min(0.99);

        let mut parent: Vec<usize> = (0..n).collect();
        for i in 0..n {
            for j in (i + 1)..n {
                let idx = i * (2 * n - i - 1) / 2 + (j - i - 1);
                if sim_matrix[idx] > threshold {
                    union(&mut parent, i, j);
                }
            }
        }

        // Count equivalence classes and their sizes
        let mut class_map: HashMap<usize, usize> = HashMap::new();
        for i in 0..n {
            let root = find(&mut parent, i);
            *class_map.entry(root).or_insert(0) += 1;
        }
        let num_classes = class_map.len();
        let mut class_sizes: Vec<f64> = class_map.values().map(|&v| v as f64).collect();
        class_sizes.sort_by(|a, b| b.partial_cmp(a).unwrap_or(std::cmp::Ordering::Equal));

        // Step 4: Profile entropy — how diverse are the structural roles?
        // Use distribution of equivalence class sizes as probability.
        let n_f = n as f64;
        let mut profile_entropy = 0.0;
        for &sz in &class_sizes {
            let p = sz / n_f;
            if p > 0.0 {
                profile_entropy -= p * p.ln();
            }
        }

        // Symmetry ratio: fraction of pairs above threshold (high = more global symmetry)
        let above_count = sim_matrix.iter().filter(|&&s| s > threshold).count();
        let symmetry_ratio = if pair_count > 0 {
            above_count as f64 / pair_count as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("mean_isomorphism_score".to_string(), vec![mean_iso_score]);
        result.insert("num_equivalence_classes".to_string(), vec![num_classes as f64]);
        result.insert("equivalence_class_sizes".to_string(), class_sizes);
        result.insert("profile_entropy".to_string(), vec![profile_entropy]);
        result.insert("symmetry_ratio".to_string(), vec![symmetry_ratio]);
        result
    }
}

/// Pearson correlation between two slices of equal length.
fn pearson(a: &[f64], b: &[f64]) -> f64 {
    let n = a.len().min(b.len());
    if n < 2 {
        return 0.0;
    }
    let n_f = n as f64;
    let mean_a = a[..n].iter().sum::<f64>() / n_f;
    let mean_b = b[..n].iter().sum::<f64>() / n_f;

    let mut cov = 0.0;
    let mut var_a = 0.0;
    let mut var_b = 0.0;
    for i in 0..n {
        let da = a[i] - mean_a;
        let db = b[i] - mean_b;
        cov += da * db;
        var_a += da * da;
        var_b += db * db;
    }

    let denom = (var_a * var_b).sqrt();
    if denom < 1e-12 {
        return 0.0;
    }
    cov / denom
}

/// Union-Find: find with path compression.
fn find(parent: &mut Vec<usize>, mut x: usize) -> usize {
    while parent[x] != x {
        parent[x] = parent[parent[x]];
        x = parent[x];
    }
    x
}

/// Union-Find: union by index (simple).
fn union(parent: &mut Vec<usize>, a: usize, b: usize) {
    let ra = find(parent, a);
    let rb = find(parent, b);
    if ra != rb {
        parent[rb] = ra;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    /// Test with symmetric data: 4 points forming a square should produce
    /// high isomorphism scores (all points are structurally equivalent).
    #[test]
    fn test_isomorphism_square() {
        // Square: (0,0), (1,0), (1,1), (0,1)
        let data = vec![0.0, 0.0, 1.0, 0.0, 1.0, 1.0, 0.0, 1.0];
        let shared = SharedData::compute(&data, 4, 2);
        let lens = UisomorphismLens;
        let result = lens.scan(&data, 4, 2, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        let score = result["mean_isomorphism_score"][0];
        assert!(score > 0.5, "square should have high isomorphism score, got {score}");
        assert!(result.contains_key("num_equivalence_classes"));
        assert!(result.contains_key("profile_entropy"));
    }

    /// Test with asymmetric data: two distinct cluster shapes.
    /// Tight triangle vs spread-out line → low mean isomorphism score.
    #[test]
    fn test_isomorphism_asymmetric_shapes() {
        // Triangle cluster: (0,0), (1,0), (0.5, 0.87)
        // Line cluster: (10,0), (12,0), (14,0), (16,0), (18,0)
        let data = vec![
            0.0, 0.0,
            1.0, 0.0,
            0.5, 0.87,
            10.0, 0.0,
            12.0, 0.0,
            14.0, 0.0,
            16.0, 0.0,
            18.0, 0.0,
        ];
        let n = 8;
        let d = 2;
        let shared = SharedData::compute(&data, n, d);
        let lens = UisomorphismLens;
        let result = lens.scan(&data, n, d, &shared);

        assert!(!result.is_empty(), "scan must return non-empty result");
        assert!(result.contains_key("mean_isomorphism_score"));
        assert!(result.contains_key("symmetry_ratio"));

        let sizes = &result["equivalence_class_sizes"];
        assert!(!sizes.is_empty(), "class sizes must be non-empty");
        // With two distinct shapes, we should see multiple equivalence classes
        let classes = result["num_equivalence_classes"][0];
        assert!(classes >= 2.0, "distinct shapes should produce multiple classes, got {classes}");
    }

    /// Edge case: exactly 3 points (minimum viable input).
    #[test]
    fn test_isomorphism_minimum_points() {
        let data = vec![0.0, 1.0, 2.0];
        let shared = SharedData::compute(&data, 3, 1);
        let lens = UisomorphismLens;
        let result = lens.scan(&data, 3, 1, &shared);
        assert!(!result.is_empty());
        assert!(result["mean_isomorphism_score"][0].is_finite());
    }
}
