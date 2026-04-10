use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// TriangleLens (비율/삼각자): simple fraction matching in distance ratios.
///
/// Algorithm:
///   1. Compute all pairwise distance ratios
///   2. Check proximity to simple fractions p/q (p,q in 1..6)
///   3. Count and score matches — high score = data has simple ratio structure
///   4. Reports best-matching ratio and fraction match density
pub struct TriangleLens;

impl Lens for TriangleLens {
    fn name(&self) -> &str {
        "TriangleLens"
    }

    fn category(&self) -> &str {
        "T0"
    }

    fn scan(&self, _data: &[f64], n: usize, _d: usize, shared: &SharedData) -> LensResult {
        if n < 4 {
            return HashMap::new();
        }

        // Generate simple fractions p/q for p,q in 1..=6
        let mut fractions: Vec<(f64, u32, u32)> = Vec::new();
        for p in 1..=6u32 {
            for q in 1..=6u32 {
                if p != q {
                    let f = p as f64 / q as f64;
                    // Avoid duplicates (e.g., 2/4 = 1/2)
                    if !fractions.iter().any(|(v, _, _)| (v - f).abs() < 1e-10) {
                        fractions.push((f, p, q));
                    }
                }
            }
        }

        let tolerance = 0.05; // 5% tolerance

        // Sample distance pairs and compute ratios
        let max_pairs = n.min(50);
        let mut match_count = 0usize;
        let mut total_ratios = 0usize;
        let mut best_fraction = (0u32, 0u32);
        let mut best_fraction_count = 0usize;
        let mut fraction_hits: HashMap<(u32, u32), usize> = HashMap::new();

        for i in 0..max_pairs {
            for j in (i + 1)..max_pairs.min(n) {
                let d1 = shared.dist(i, j);
                if d1 < 1e-15 {
                    continue;
                }
                for kk in (j + 1)..max_pairs.min(n) {
                    let d2 = shared.dist(i, kk);
                    if d2 < 1e-15 {
                        continue;
                    }
                    let ratio = d1 / d2;
                    total_ratios += 1;

                    for &(frac, p, q) in &fractions {
                        if (ratio - frac).abs() / frac < tolerance {
                            match_count += 1;
                            let count = fraction_hits.entry((p, q)).or_insert(0);
                            *count += 1;
                            if *count > best_fraction_count {
                                best_fraction_count = *count;
                                best_fraction = (p, q);
                            }
                            break;
                        }
                    }
                }
            }
        }

        let match_density = if total_ratios > 0 {
            match_count as f64 / total_ratios as f64
        } else {
            0.0
        };

        let mut result = HashMap::new();
        result.insert("fraction_match_density".to_string(), vec![match_density]);
        result.insert(
            "best_fraction".to_string(),
            vec![best_fraction.0 as f64, best_fraction.1 as f64],
        );
        result
    }
}
