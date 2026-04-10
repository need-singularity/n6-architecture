//! Performance Benchmarking for NEXUS-6
//!
//! Runs micro-benchmarks on core operations (telescope scan, registry lookup,
//! consensus computation, calibration) using std::time::Instant only.

use std::collections::HashMap;
use std::time::Instant;

use crate::calibration::{calibrate_lens, generate_synthetic_datasets};
use crate::telescope::consensus::weighted_consensus;
use crate::telescope::lens_trait::LensResult;
use crate::telescope::registry::LensRegistry;
use crate::telescope::shared_data::SharedData;
use crate::telescope::Telescope;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;           // the perfect number
const SIGMA: usize = 12;      // σ(6) = sum of divisors
const PHI: usize = 2;         // φ(6) = Euler totient
const TAU: usize = 4;         // τ(6) = number of divisors
const J2: usize = 24;         // J₂(6) = Jordan totient
const SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// Result from a single benchmark run.
#[derive(Debug, Clone)]
pub struct BenchmarkResult {
    /// Name of the benchmarked operation
    pub name: String,
    /// Number of iterations run
    pub iterations: usize,
    /// Mean time per iteration in nanoseconds
    pub mean_ns: u64,
    /// Minimum iteration time in nanoseconds
    pub min_ns: u64,
    /// Maximum iteration time in nanoseconds
    pub max_ns: u64,
    /// Throughput: operations per second
    pub throughput: f64,
}

/// Aggregated suite of benchmark results.
#[derive(Debug, Clone)]
pub struct BenchmarkSuite {
    /// Individual benchmark results
    pub results: Vec<BenchmarkResult>,
    /// Total wall-clock time for the entire suite in milliseconds
    pub total_time_ms: u64,
    /// Name of the slowest operation (bottleneck)
    pub bottleneck: Option<String>,
}

/// Time a closure over `iterations`, returning a BenchmarkResult.
pub fn time_fn<F: FnMut()>(name: &str, iterations: usize, mut f: F) -> BenchmarkResult {
    let iters = iterations.max(1);
    let mut times_ns = Vec::with_capacity(iters);

    // Warmup: run n=6 times before measuring
    for _ in 0..N {
        f();
    }

    for _ in 0..iters {
        let start = Instant::now();
        f();
        let elapsed = start.elapsed();
        times_ns.push(elapsed.as_nanos() as u64);
    }

    let min_ns = *times_ns.iter().min().unwrap_or(&0);
    let max_ns = *times_ns.iter().max().unwrap_or(&0);
    let sum_ns: u64 = times_ns.iter().sum();
    let mean_ns = sum_ns / iters as u64;

    let throughput = if mean_ns > 0 {
        1_000_000_000.0 / mean_ns as f64
    } else {
        f64::INFINITY
    };

    BenchmarkResult {
        name: name.to_string(),
        iterations: iters,
        mean_ns,
        min_ns,
        max_ns,
        throughput,
    }
}

/// Benchmark telescope scan with synthetic data.
///
/// `data_size` controls the number of data points (n). Dimensionality = φ=2.
pub fn bench_telescope_scan(data_size: usize) -> BenchmarkResult {
    let n = data_size;
    let d = PHI; // φ=2 dimensions
    let data: Vec<f64> = (0..n * d)
        .map(|i| (i as f64 * 0.1).sin() * SIGMA as f64)
        .collect();

    let telescope = Telescope::new();

    // Run σ-τ=8 iterations for scan benchmark
    time_fn(
        &format!("telescope_scan_n{}", n),
        SIGMA_MINUS_TAU, // σ-τ=8 iterations
        || {
            let _ = telescope.scan_all(&data, n, d);
        },
    )
}

/// Benchmark LensRegistry creation and lookup.
pub fn bench_lens_registry() -> BenchmarkResult {
    time_fn(
        "lens_registry_create_lookup",
        J2, // J₂=24 iterations
        || {
            let registry = LensRegistry::new();
            // Lookup a few known lenses
            let _ = registry.get("consciousness");
            let _ = registry.get("gravity");
            let _ = registry.get("topology");
            let _ = registry.len();
        },
    )
}

/// Benchmark weighted consensus computation.
///
/// `n_lenses` controls how many synthetic lens results to include.
pub fn bench_consensus(n_lenses: usize) -> BenchmarkResult {
    // Build synthetic results: each lens produces τ=4 metrics
    let mut results: HashMap<String, LensResult> = HashMap::new();
    let mut hit_rates: HashMap<String, f64> = HashMap::new();

    for i in 0..n_lenses {
        let lens_name = format!("lens_{}", i);
        let mut lr = LensResult::new();
        for m in 0..TAU {
            // Every lens reports on the same τ=4 metrics with slight variation
            lr.insert(
                format!("metric_{}", m),
                vec![(i as f64 + m as f64) * 0.1],
            );
        }
        results.insert(lens_name.clone(), lr);
        hit_rates.insert(lens_name, 0.8); // default weight
    }

    time_fn(
        &format!("consensus_{}lenses", n_lenses),
        SIGMA, // σ=12 iterations
        || {
            let _ = weighted_consensus(&results, &hit_rates);
        },
    )
}

/// Benchmark the calibration pipeline on synthetic datasets.
pub fn bench_calibration() -> BenchmarkResult {
    let datasets = generate_synthetic_datasets();

    // Use a lightweight internal lens for calibration benchmark
    struct BenchLens;
    impl crate::telescope::lens_trait::Lens for BenchLens {
        fn name(&self) -> &str { "BenchLens" }
        fn category(&self) -> &str { "T0" }
        fn scan(
            &self,
            _data: &[f64],
            _n: usize,
            _d: usize,
            _shared: &SharedData,
        ) -> LensResult {
            let mut r = LensResult::new();
            r.insert("period".to_string(), vec![12.0]); // σ=12
            r
        }
    }

    let lens = BenchLens;

    time_fn(
        "calibration_pipeline",
        N, // n=6 iterations
        || {
            let _ = calibrate_lens(&lens, &datasets);
        },
    )
}

/// Run the full benchmark suite and identify the bottleneck.
pub fn run_full_suite() -> BenchmarkSuite {
    let suite_start = Instant::now();

    let mut results = Vec::new();

    // Benchmark 1: telescope scan (J₂=24 data points)
    results.push(bench_telescope_scan(J2));

    // Benchmark 2: registry operations
    results.push(bench_lens_registry());

    // Benchmark 3: consensus with σ=12 lenses
    results.push(bench_consensus(SIGMA));

    // Benchmark 4: calibration pipeline
    results.push(bench_calibration());

    let total_time_ms = suite_start.elapsed().as_millis() as u64;

    // Identify bottleneck: operation with highest mean_ns
    let bottleneck = results
        .iter()
        .max_by_key(|r| r.mean_ns)
        .map(|r| r.name.clone());

    BenchmarkSuite {
        results,
        total_time_ms,
        bottleneck,
    }
}

/// Format a BenchmarkSuite as an ASCII report.
pub fn format_suite(suite: &BenchmarkSuite) -> String {
    let mut s = String::new();
    s.push_str("┌──────────────────────────────────────────────────────────────┐\n");
    s.push_str("│              NEXUS-6 Benchmark Suite                        │\n");
    s.push_str("├──────────────────────────────────┬───────┬──────────┬───────┤\n");
    s.push_str("│  Operation                       │ Iters │ Mean(ns) │ Op/s  │\n");
    s.push_str("├──────────────────────────────────┼───────┼──────────┼───────┤\n");
    for r in &suite.results {
        s.push_str(&format!(
            "│  {:<32} │ {:>5} │ {:>8} │ {:>5.0} │\n",
            truncate_str(&r.name, 32),
            r.iterations,
            r.mean_ns,
            r.throughput.min(99999.0),
        ));
    }
    s.push_str("├──────────────────────────────────┴───────┴──────────┴───────┤\n");
    s.push_str(&format!(
        "│  Total time: {}ms                                          │\n",
        suite.total_time_ms
    ));
    if let Some(ref bn) = suite.bottleneck {
        s.push_str(&format!(
            "│  Bottleneck: {:<45} │\n",
            truncate_str(bn, 45)
        ));
    }
    s.push_str("└──────────────────────────────────────────────────────────────┘\n");
    s
}

fn truncate_str(s: &str, max_len: usize) -> &str {
    if s.len() <= max_len {
        s
    } else {
        &s[..max_len]
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_time_fn() {
        let mut counter = 0u64;
        let result = time_fn("increment", SIGMA, || {
            counter += 1;
        });
        assert_eq!(result.name, "increment");
        assert_eq!(result.iterations, SIGMA); // σ=12
        assert!(result.min_ns <= result.mean_ns);
        assert!(result.mean_ns <= result.max_ns);
        assert!(result.throughput > 0.0);
    }

    #[test]
    fn test_bench_telescope_scan() {
        let result = bench_telescope_scan(N); // n=6 data points
        assert!(result.name.contains("telescope_scan"));
        assert!(result.iterations == SIGMA_MINUS_TAU); // σ-τ=8
        assert!(result.mean_ns > 0);
    }

    #[test]
    fn test_bench_lens_registry() {
        let result = bench_lens_registry();
        assert_eq!(result.iterations, J2); // J₂=24
        assert!(result.throughput > 0.0);
    }

    #[test]
    fn test_bench_consensus() {
        let result = bench_consensus(N); // n=6 lenses
        assert!(result.name.contains("consensus"));
        assert!(result.mean_ns > 0);
    }

    #[test]
    fn test_bench_calibration() {
        let result = bench_calibration();
        assert_eq!(result.name, "calibration_pipeline");
        assert_eq!(result.iterations, N); // n=6
    }

    #[test]
    fn test_run_full_suite() {
        let suite = run_full_suite();
        assert_eq!(suite.results.len(), TAU); // τ=4 benchmarks
        assert!(suite.total_time_ms > 0 || suite.results.iter().all(|r| r.mean_ns == 0));
        assert!(suite.bottleneck.is_some(), "should identify a bottleneck");
    }

    #[test]
    fn test_format_suite() {
        let suite = run_full_suite();
        let report = format_suite(&suite);
        assert!(report.contains("NEXUS-6 Benchmark Suite"));
        assert!(report.contains("Bottleneck"));
    }
}
