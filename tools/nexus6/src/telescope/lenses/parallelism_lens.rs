use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// ParallelismLens: Detect data/task/pipeline parallelism opportunities.
///
/// Analyzes data dependencies to find independent subsets that can run
/// in parallel. LLVM's auto-parallelization is limited to simple loops;
/// this lens detects deeper parallelism: independent data partitions,
/// wavefront parallelism, and pipeline-stage boundaries.
///
/// Metrics:
///   1. data_parallelism: fraction of data that can be processed independently
///   2. task_parallelism: number of independent task clusters
///   3. pipeline_depth: number of sequential stages detected
///   4. dependency_density: fraction of point pairs with strong dependency
///   5. wavefront_width: max number of simultaneously processable points
///   6. amdahl_parallel_fraction: estimated parallel fraction (Amdahl's law)
pub struct ParallelismLens;

impl Lens for ParallelismLens {
    fn name(&self) -> &str { "ParallelismLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }

        let max_n = n.min(200);

        // Build dependency graph: two points are "dependent" if they have
        // high mutual information (their values are correlated)
        let mi_threshold = 0.1f64;
        let mut dep_count = vec![0u32; max_n];
        let mut total_deps = 0u64;

        // Use KNN + MI to detect dependencies
        for i in 0..max_n {
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let j = j as usize;
                if j >= max_n || j == i { continue; }
                // Check if rows are dependent via cosine similarity
                let row_i = &data[i * d..(i * d + d).min(data.len())];
                let row_j = &data[j * d..(j * d + d).min(data.len())];
                let dot: f64 = row_i.iter().zip(row_j.iter()).map(|(a, b)| a * b).sum();
                let norm_i: f64 = row_i.iter().map(|v| v * v).sum::<f64>().sqrt();
                let norm_j: f64 = row_j.iter().map(|v| v * v).sum::<f64>().sqrt();
                let sim = if norm_i > 1e-12 && norm_j > 1e-12 {
                    (dot / (norm_i * norm_j)).abs()
                } else { 0.0 };
                if sim > 0.8 {
                    dep_count[i] += 1;
                    total_deps += 1;
                }
            }
        }

        let max_possible = max_n as u64 * shared.knn_k as u64;
        let dependency_density = if max_possible > 0 {
            total_deps as f64 / max_possible as f64
        } else { 0.0 };

        // Data parallelism: find connected components in dependency graph
        // (union-find)
        let mut parent: Vec<usize> = (0..max_n).collect();
        fn find(parent: &mut Vec<usize>, i: usize) -> usize {
            if parent[i] != i {
                parent[i] = find(parent, parent[i]);
            }
            parent[i]
        }
        fn union(parent: &mut Vec<usize>, a: usize, b: usize) {
            let ra = find(parent, a);
            let rb = find(parent, b);
            if ra != rb { parent[ra] = rb; }
        }

        for i in 0..max_n {
            let knn = shared.knn(i);
            for &j in knn.iter() {
                let j = j as usize;
                if j >= max_n || j == i { continue; }
                let row_i = &data[i * d..(i * d + d).min(data.len())];
                let row_j = &data[j * d..(j * d + d).min(data.len())];
                let dot: f64 = row_i.iter().zip(row_j.iter()).map(|(a, b)| a * b).sum();
                let norm_i: f64 = row_i.iter().map(|v| v * v).sum::<f64>().sqrt();
                let norm_j: f64 = row_j.iter().map(|v| v * v).sum::<f64>().sqrt();
                let sim = if norm_i > 1e-12 && norm_j > 1e-12 {
                    (dot / (norm_i * norm_j)).abs()
                } else { 0.0 };
                if sim > 0.8 {
                    union(&mut parent, i, j);
                }
            }
        }

        // Count components
        let mut component_sizes: HashMap<usize, usize> = HashMap::new();
        for i in 0..max_n {
            let root = find(&mut parent, i);
            *component_sizes.entry(root).or_insert(0) += 1;
        }
        let n_components = component_sizes.len();
        let task_parallelism = n_components as f64;

        // Data parallelism: fraction of points in non-singleton components
        // that could be split across threads
        let independent_points: usize = component_sizes.values()
            .filter(|&&size| size == 1).count();
        let data_parallelism = independent_points as f64 / max_n as f64;

        // Pipeline depth: detect sequential chains using value ordering
        // Sort points by their first-dim value as a proxy for "time"
        let mut ordered: Vec<(f64, usize)> = (0..max_n)
            .map(|i| (data[i * d], i))
            .collect();
        ordered.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        // Find the longest chain where consecutive points are dependent
        let mut max_chain = 1usize;
        let mut current_chain = 1usize;
        for w in ordered.windows(2) {
            let i = w[0].1;
            let j = w[1].1;
            if find(&mut parent, i) == find(&mut parent, j) {
                current_chain += 1;
            } else {
                max_chain = max_chain.max(current_chain);
                current_chain = 1;
            }
        }
        max_chain = max_chain.max(current_chain);
        let pipeline_depth = max_chain as f64;

        // Wavefront width: max number of independent points at any "level"
        // Use row index as level proxy
        let level_size = (max_n as f64 / pipeline_depth).ceil() as usize;
        let wavefront_width = level_size.max(1) as f64;

        // Amdahl's parallel fraction
        let serial_fraction = pipeline_depth / max_n as f64;
        let amdahl_parallel = 1.0 - serial_fraction.min(1.0);

        let mut result = HashMap::new();
        result.insert("data_parallelism".to_string(), vec![data_parallelism]);
        result.insert("task_parallelism".to_string(), vec![task_parallelism]);
        result.insert("pipeline_depth".to_string(), vec![pipeline_depth]);
        result.insert("dependency_density".to_string(), vec![dependency_density]);
        result.insert("wavefront_width".to_string(), vec![wavefront_width]);
        result.insert("amdahl_parallel_fraction".to_string(), vec![amdahl_parallel]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::telescope::shared_data::SharedData;

    #[test]
    fn test_parallelism_basic() {
        let data: Vec<f64> = (0..40).map(|i| i as f64 * 0.1).collect();
        let shared = SharedData::compute(&data, 20, 2);
        let r = ParallelismLens.scan(&data, 20, 2, &shared);
        assert!(!r.is_empty());
        assert!(r.contains_key("data_parallelism"));
        assert!(r.contains_key("amdahl_parallel_fraction"));
    }

    #[test]
    fn test_parallelism_independent_clusters() {
        // Two well-separated clusters = high parallelism
        let mut data = Vec::new();
        for i in 0..10 { data.push(i as f64 * 0.01); data.push(0.0); }
        for i in 0..10 { data.push(100.0 + i as f64 * 0.01); data.push(100.0); }
        let shared = SharedData::compute(&data, 20, 2);
        let r = ParallelismLens.scan(&data, 20, 2, &shared);
        assert!(r["task_parallelism"][0] >= 2.0);
    }
}
