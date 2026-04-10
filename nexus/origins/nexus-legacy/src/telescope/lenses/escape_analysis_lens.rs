use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// EscapeAnalysisLens: Detect heap-to-stack demotion opportunities.
///
/// Analyzes data lifetime and scope patterns to find allocations that
/// could be demoted from heap to stack. LLVM's escape analysis is
/// conservative; this lens detects additional cases where data has
/// bounded lifetime, small size, and no external references.
///
/// Metrics:
///   1. stack_demotable: fraction of data with bounded, local lifetime
///   2. escape_fraction: fraction of data that "escapes" to wider scope
///   3. lifetime_locality: how localized data lifetimes are
///   4. size_fitness: fraction small enough for stack allocation
///   5. reference_depth: average depth of reference chains
///   6. arena_candidate: fraction suitable for arena/bump allocation
pub struct EscapeAnalysisLens;

impl Lens for EscapeAnalysisLens {
    fn name(&self) -> &str { "EscapeAnalysisLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(256);

        // Model "lifetime" via temporal locality: points that are similar to
        // their sequential neighbors have short, local lifetimes (stack-like).
        // Points that are similar to distant points have long lifetimes (heap-like).

        let mut local_sim = Vec::with_capacity(max_n);
        let mut distant_sim = Vec::with_capacity(max_n);

        for i in 0..max_n {
            let row_i = &data[i * d..(i * d + d).min(data.len())];
            let norm_i: f64 = row_i.iter().map(|v| v * v).sum::<f64>().sqrt();
            if norm_i < 1e-15 {
                local_sim.push(0.0);
                distant_sim.push(0.0);
                continue;
            }

            // Local: similarity to immediate neighbors (within 3 rows)
            let mut local_s = 0.0f64;
            let mut local_count = 0usize;
            for delta in 1..=3 {
                for &offset in &[i.wrapping_sub(delta), i + delta] {
                    if offset < max_n {
                        let row_j = &data[offset * d..(offset * d + d).min(data.len())];
                        let norm_j: f64 = row_j.iter().map(|v| v * v).sum::<f64>().sqrt();
                        if norm_j > 1e-15 {
                            let dot: f64 = row_i.iter().zip(row_j.iter())
                                .map(|(a, b)| a * b).sum();
                            local_s += (dot / (norm_i * norm_j)).abs();
                            local_count += 1;
                        }
                    }
                }
            }
            local_sim.push(if local_count > 0 { local_s / local_count as f64 } else { 0.0 });

            // Distant: similarity to KNN that are far away in index
            let knn = shared.knn(i);
            let mut dist_s = 0.0f64;
            let mut dist_count = 0usize;
            for &j in knn.iter() {
                let j = j as usize;
                if j >= max_n { continue; }
                let index_dist = if j > i { j - i } else { i - j };
                if index_dist > max_n / 4 {
                    let row_j = &data[j * d..(j * d + d).min(data.len())];
                    let norm_j: f64 = row_j.iter().map(|v| v * v).sum::<f64>().sqrt();
                    if norm_j > 1e-15 {
                        let dot: f64 = row_i.iter().zip(row_j.iter())
                            .map(|(a, b)| a * b).sum();
                        dist_s += (dot / (norm_i * norm_j)).abs();
                        dist_count += 1;
                    }
                }
            }
            distant_sim.push(if dist_count > 0 { dist_s / dist_count as f64 } else { 0.0 });
        }

        // Stack-demotable: high local similarity, low distant similarity
        let mut stack_count = 0usize;
        let mut escape_count = 0usize;
        for i in 0..max_n {
            if local_sim[i] > 0.5 && distant_sim[i] < 0.3 {
                stack_count += 1;
            }
            if distant_sim[i] > 0.5 {
                escape_count += 1;
            }
        }
        let stack_demotable = stack_count as f64 / max_n as f64;
        let escape_fraction = escape_count as f64 / max_n as f64;

        // Lifetime locality: ratio of local to distant similarity
        let mean_local = local_sim.iter().sum::<f64>() / max_n as f64;
        let mean_distant = distant_sim.iter().sum::<f64>() / max_n as f64;
        let lifetime_locality = if mean_local + mean_distant > 1e-12 {
            mean_local / (mean_local + mean_distant)
        } else { 0.5 };

        // Size fitness: row size vs typical stack frame (8KB)
        let row_bytes = d * 8; // f64 = 8 bytes
        let stack_limit = 8192; // typical stack frame limit
        let size_fitness = if row_bytes <= stack_limit { 1.0 }
            else { (stack_limit as f64 / row_bytes as f64).min(1.0) };

        // Reference depth: chain length in KNN graph
        // Follow KNN links and measure longest non-repeating chain
        let mut max_depth = 0usize;
        for start in 0..max_n.min(30) {
            let mut visited = vec![false; max_n];
            let mut current = start;
            let mut depth = 0usize;
            loop {
                if visited[current] { break; }
                visited[current] = true;
                depth += 1;
                let knn = shared.knn(current);
                // Follow nearest unvisited neighbor
                let next = knn.iter()
                    .map(|&j| j as usize)
                    .find(|&j| j < max_n && !visited[j]);
                match next {
                    Some(j) => current = j,
                    None => break,
                }
                if depth > max_n { break; }
            }
            if depth > max_depth { max_depth = depth; }
        }
        let reference_depth = max_depth as f64 / max_n as f64;

        // Arena candidate: groups of points created and destroyed together
        // (high local similarity in windows)
        let window = (max_n / 8).max(4);
        let mut arena_windows = 0usize;
        let mut total_windows = 0usize;
        for start in (0..max_n).step_by(window.max(1)) {
            let end = (start + window).min(max_n);
            if end - start < 3 { continue; }
            // Check if window points are internally similar but different from outside
            let mut internal_sim = 0.0f64;
            let mut internal_count = 0usize;
            for i in start..end {
                for j in (i + 1)..end.min(i + 4) {
                    if i < max_n && j < max_n && i != j {
                        let dist = shared.dist(i, j);
                        internal_sim += 1.0 / (1.0 + dist);
                        internal_count += 1;
                    }
                }
            }
            let avg_internal = if internal_count > 0 {
                internal_sim / internal_count as f64
            } else { 0.0 };
            if avg_internal > 0.5 { arena_windows += 1; }
            total_windows += 1;
        }
        let arena_candidate = if total_windows > 0 {
            arena_windows as f64 / total_windows as f64
        } else { 0.0 };

        let mut result = HashMap::new();
        result.insert("stack_demotable".to_string(), vec![stack_demotable]);
        result.insert("escape_fraction".to_string(), vec![escape_fraction]);
        result.insert("lifetime_locality".to_string(), vec![lifetime_locality]);
        result.insert("size_fitness".to_string(), vec![size_fitness]);
        result.insert("reference_depth".to_string(), vec![reference_depth]);
        result.insert("arena_candidate".to_string(), vec![arena_candidate]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_escape_analysis_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = EscapeAnalysisLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("stack_demotable"));
        assert!(r.contains_key("escape_fraction"));
    }

    #[test]
    fn test_escape_small_rows() {
        // Small d = stack-friendly
        let data: Vec<f64> = (0..20).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let r = EscapeAnalysisLens.scan(&data, 10, 2, &shared);
        assert_eq!(r["size_fitness"][0], 1.0);
    }
}
