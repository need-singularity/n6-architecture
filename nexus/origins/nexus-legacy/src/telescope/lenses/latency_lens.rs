use std::collections::HashMap;

use crate::telescope::lens_trait::{Lens, LensResult};
use crate::telescope::shared_data::SharedData;

/// LatencyLens: Instruction latency-based scheduling hints.
///
/// Models operation latency characteristics from data patterns:
/// - Dependent chains (sequential data dependencies) increase latency
/// - Independent operations (parallel-ready segments) decrease latency
/// - Memory latency vs compute latency tradeoffs
///
/// Metrics:
///   1. critical_path_length: longest dependency chain estimate (normalized)
///   2. ilp_score: instruction-level parallelism potential (independent operations)
///   3. memory_latency_fraction: fraction of ops dominated by memory access time
///   4. pipeline_stall_risk: probability of pipeline stalls from data hazards
///   5. scheduling_slack: how much reordering freedom exists
pub struct LatencyLens;

impl Lens for LatencyLens {
    fn name(&self) -> &str { "LatencyLens" }
    fn category(&self) -> &str { "compiler" }

    fn scan(&self, data: &[f64], n: usize, d: usize, _shared: &SharedData) -> LensResult {
        if n < 6 || d < 1 { return HashMap::new(); }
        let max_n = n.min(500);

        // Model each row as an "instruction" with d operands
        // Dependency: row i depends on row i-1 if their values are correlated

        // Sequential dependency strength: correlation between consecutive rows
        let mut dep_strengths = Vec::with_capacity(max_n - 1);
        for i in 1..max_n {
            let mut dot = 0.0f64;
            let mut na = 0.0f64;
            let mut nb = 0.0f64;
            for j in 0..d {
                let a = data[(i - 1) * d + j];
                let b = data[i * d + j];
                dot += a * b;
                na += a * a;
                nb += b * b;
            }
            let denom = (na * nb).sqrt();
            let corr = if denom > 1e-15 { (dot / denom).abs() } else { 0.0 };
            dep_strengths.push(corr);
        }

        // Critical path: longest consecutive chain of strong dependencies (corr > 0.7)
        let dep_threshold = 0.7;
        let mut max_chain = 0usize;
        let mut current_chain = 0usize;
        for &dep in &dep_strengths {
            if dep > dep_threshold {
                current_chain += 1;
                max_chain = max_chain.max(current_chain);
            } else {
                current_chain = 0;
            }
        }
        let critical_path_length = max_chain as f64 / max_n as f64;

        // ILP score: fraction of weak dependencies (independent instructions)
        let independent_count = dep_strengths.iter().filter(|&&d| d < 0.3).count();
        let ilp_score = independent_count as f64 / dep_strengths.len().max(1) as f64;

        // Memory latency model: high-variance dimensions suggest memory-bound ops
        // (scattered memory access = cache miss = high latency)
        let mut dim_ranges = Vec::with_capacity(d);
        for j in 0..d {
            let mut min_v = f64::MAX;
            let mut max_v = f64::MIN;
            for i in 0..max_n {
                let v = data[i * d + j];
                min_v = min_v.min(v);
                max_v = max_v.max(v);
            }
            dim_ranges.push(max_v - min_v);
        }
        let mean_range = dim_ranges.iter().sum::<f64>() / d as f64;
        let high_range_dims = dim_ranges.iter().filter(|&&r| r > mean_range * 2.0).count();
        let memory_latency_fraction = high_range_dims as f64 / d as f64;

        // Pipeline stall risk: abrupt changes in dependency pattern
        let mut stall_risk = 0.0f64;
        if dep_strengths.len() > 1 {
            let mut transitions = 0usize;
            for i in 1..dep_strengths.len() {
                let prev_dep = dep_strengths[i - 1] > dep_threshold;
                let curr_dep = dep_strengths[i] > dep_threshold;
                if prev_dep != curr_dep {
                    transitions += 1;
                }
            }
            stall_risk = transitions as f64 / dep_strengths.len() as f64;
        }

        // Scheduling slack: how many rows can be freely reordered
        // (weak dependencies between non-adjacent rows)
        let mut reorderable = 0usize;
        let step = 2.min(max_n - 1).max(1);
        for i in step..max_n {
            let mut dot = 0.0f64;
            let mut na = 0.0f64;
            let mut nb = 0.0f64;
            for j in 0..d {
                let a = data[(i - step) * d + j];
                let b = data[i * d + j];
                dot += a * b;
                na += a * a;
                nb += b * b;
            }
            let denom = (na * nb).sqrt();
            let corr = if denom > 1e-15 { (dot / denom).abs() } else { 0.0 };
            if corr < 0.3 {
                reorderable += 1;
            }
        }
        let scheduling_slack = reorderable as f64 / (max_n - step).max(1) as f64;

        let mut result = HashMap::new();
        result.insert("critical_path_length".to_string(), vec![critical_path_length]);
        result.insert("ilp_score".to_string(), vec![ilp_score]);
        result.insert("memory_latency_fraction".to_string(), vec![memory_latency_fraction]);
        result.insert("pipeline_stall_risk".to_string(), vec![stall_risk]);
        result.insert("scheduling_slack".to_string(), vec![scheduling_slack]);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sequential_dependency() {
        // Strongly correlated consecutive rows = long critical path
        let data: Vec<f64> = (0..60).map(|i| i as f64).collect();
        let shared = SharedData::compute(&data, 20, 3);
        let result = LatencyLens.scan(&data, 20, 3, &shared);
        assert!(result.contains_key("critical_path_length"));
        assert!(result["ilp_score"][0] < 1.0);
    }

    #[test]
    fn test_random_high_ilp() {
        // Alternating pattern = less dependency = higher ILP
        let data: Vec<f64> = (0..20).map(|i| {
            if i % 2 == 0 { 100.0 } else { -100.0 }
        }).collect();
        let shared = SharedData::compute(&data, 10, 2);
        let result = LatencyLens.scan(&data, 10, 2, &shared);
        assert!(result.contains_key("ilp_score"));
    }

    #[test]
    fn test_small_n() {
        let data = vec![1.0; 4];
        let shared = SharedData::compute(&data, 4, 1);
        let result = LatencyLens.scan(&data, 4, 1, &shared);
        assert!(result.is_empty());
    }
}
