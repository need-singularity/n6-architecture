use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// QualiaLens: Measures phenomenal richness — information differentiation depth.
///
/// Qualia = "what it is like" — the subjective quality of experience.
/// This lens quantifies the richness and uniqueness of data representations:
///   1. Differentiation depth: how many distinguishable states exist
///   2. Irreducibility: can't be decomposed into independent parts
///   3. Exclusion: one dominant interpretation per state
///   4. Compositional richness: complexity of state combinations
///
/// n=6: σ=12 distinguishable categories, J₂=24 maximum compositional depth
pub struct QualiaLens;

impl Lens for QualiaLens {
    fn name(&self) -> &str {
        "QualiaLens"
    }

    fn category(&self) -> &str {
        "T1"
    }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 2 {
            return HashMap::new();
        }

        let dims = d.min(24);

        // 1. Differentiation depth: unique distinguishable states
        // Bin each dimension into σ=12 levels, count unique state vectors
        let bins = 12usize;
        let mut states: Vec<Vec<u8>> = Vec::with_capacity(n);

        let mut mins = vec![f64::MAX; dims];
        let mut maxs = vec![f64::MIN; dims];
        for r in 0..n {
            for di in 0..dims {
                let v = data[r * d + di];
                if v < mins[di] { mins[di] = v; }
                if v > maxs[di] { maxs[di] = v; }
            }
        }

        for r in 0..n {
            let mut state = Vec::with_capacity(dims);
            for di in 0..dims {
                let range = (maxs[di] - mins[di]).max(1e-12);
                let bin = ((data[r * d + di] - mins[di]) / range * (bins - 1) as f64)
                    .round() as u8;
                state.push(bin.min((bins - 1) as u8));
            }
            states.push(state);
        }

        // Count unique states
        let mut unique_states = states.clone();
        unique_states.sort();
        unique_states.dedup();
        let num_unique = unique_states.len();
        let max_possible = (bins as f64).powi(dims.min(6) as i32).min(n as f64 * 10.0);
        let differentiation = (num_unique as f64).ln() / max_possible.ln().max(1.0);

        // 2. Irreducibility: mutual information between dimension halves
        let half = dims / 2;
        // Entropy of joint distribution
        let joint_entropy = {
            let mut joint_states: Vec<Vec<u8>> = states.iter()
                .map(|s| s[..dims].to_vec())
                .collect();
            joint_states.sort();
            let mut counts: Vec<u32> = Vec::new();
            let mut prev = &joint_states[0];
            let mut count = 1u32;
            for s in joint_states.iter().skip(1) {
                if s == prev {
                    count += 1;
                } else {
                    counts.push(count);
                    count = 1;
                    prev = s;
                }
            }
            counts.push(count);
            let total = n as f64;
            counts.iter().map(|&c| {
                let p = c as f64 / total;
                if p > 0.0 { -p * p.ln() } else { 0.0 }
            }).sum::<f64>()
        };

        // Entropy of each half
        let half_entropy = |start: usize, end: usize| -> f64 {
            let mut half_states: Vec<Vec<u8>> = states.iter()
                .map(|s| s[start..end.min(dims)].to_vec())
                .collect();
            half_states.sort();
            let mut counts: Vec<u32> = Vec::new();
            let mut prev = &half_states[0];
            let mut count = 1u32;
            for s in half_states.iter().skip(1) {
                if s == prev {
                    count += 1;
                } else {
                    counts.push(count);
                    count = 1;
                    prev = s;
                }
            }
            counts.push(count);
            let total = n as f64;
            counts.iter().map(|&c| {
                let p = c as f64 / total;
                if p > 0.0 { -p * p.ln() } else { 0.0 }
            }).sum::<f64>()
        };

        let h_a = half_entropy(0, half);
        let h_b = half_entropy(half, dims);
        // Mutual information = H(A) + H(B) - H(A,B)
        let mutual_info = (h_a + h_b - joint_entropy).max(0.0);
        // Irreducibility: MI relative to joint entropy
        let irreducibility = if joint_entropy > 1e-12 {
            (mutual_info / joint_entropy).min(1.0)
        } else {
            0.0
        };

        // 3. Exclusion: how dominant is the top interpretation per state
        // Measure by cluster purity (each cluster should map to one region)
        let pair_count = n * (n - 1) / 2;
        let mut dists_sorted: Vec<f64> = Vec::with_capacity(pair_count.min(5000));
        for i in 0..n.min(100) {
            for j in (i + 1)..n.min(100) {
                dists_sorted.push(shared.dist(i, j));
            }
        }
        dists_sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let _median_d = if dists_sorted.is_empty() {
            1.0
        } else {
            dists_sorted[dists_sorted.len() / 2]
        };

        // NN state consistency: nearest neighbor should have same binned state
        let check = n.min(100);
        let mut consistent = 0u32;
        for i in 0..check {
            let mut best_j = 0;
            let mut best_d = f64::MAX;
            for j in 0..n {
                if i == j { continue; }
                let dist = shared.dist(i, j);
                if dist < best_d {
                    best_d = dist;
                    best_j = j;
                }
            }
            if states[i] == states[best_j] {
                consistent += 1;
            }
        }
        let exclusion = consistent as f64 / check.max(1) as f64;

        // 4. Compositional richness: effective number of active dimensions
        let mut dim_entropies: Vec<f64> = Vec::with_capacity(dims);
        for di in 0..dims {
            let mut bin_counts = vec![0u32; bins];
            for r in 0..n {
                bin_counts[states[r][di] as usize] += 1;
            }
            let total = n as f64;
            let h: f64 = bin_counts.iter().map(|&c| {
                if c > 0 {
                    let p = c as f64 / total;
                    -p * p.ln()
                } else {
                    0.0
                }
            }).sum();
            dim_entropies.push(h);
        }
        let total_dim_h: f64 = dim_entropies.iter().sum();
        let compositional_richness = if total_dim_h > 1e-12 {
            let mut eff = 0.0_f64;
            for &h in &dim_entropies {
                if h > 1e-15 {
                    let p = h / total_dim_h;
                    eff -= p * p.ln();
                }
            }
            eff.exp() / dims as f64 // normalized by max possible
        } else {
            0.0
        };

        // Qualia score: geometric mean of 4 components
        let qualia_score =
            (differentiation * irreducibility * exclusion * compositional_richness).powf(0.25);

        let mut result = HashMap::new();
        result.insert("qualia_score".into(), vec![qualia_score]);
        result.insert("differentiation".into(), vec![differentiation]);
        result.insert("irreducibility".into(), vec![irreducibility]);
        result.insert("exclusion".into(), vec![exclusion]);
        result.insert("compositional_richness".into(), vec![compositional_richness]);
        result.insert("unique_states".into(), vec![num_unique as f64]);
        result.insert("mutual_information".into(), vec![mutual_info]);
        result.insert("joint_entropy".into(), vec![joint_entropy]);
        result
    }
}
