//! Compiler Benchmark Tools for NEXUS-6
//!
//! 8 benchmark tools that measure compiler optimization characteristics
//! on real f64 data arrays.
//!
//! n=6 constants:
//!   - σ(6) = 12 (cache line simulation)
//!   - τ(6) = 4  (pipeline stages)
//!   - sopfr(6) = 5 (unroll factor)
//!   - φ(6) = 2  (dual-issue)

use std::collections::HashMap;

const SIGMA: usize = 12;
const TAU: usize = 4;
const N: usize = 6;
const SOPFR: usize = 5;
const PHI: usize = 2;

/// Result of a benchmark measurement: named f64 vectors.
pub type BenchResult = HashMap<String, Vec<f64>>;

/// Trait for compiler benchmark tools.
pub trait CompilerBench {
    /// Human-readable name of this benchmark.
    fn name(&self) -> &str;
    /// Measure characteristics of `data` (n rows x d cols, row-major).
    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult;
}

// ---------------------------------------------------------------------------
// 1. MicroBench — Per-optimization effect measurement
// ---------------------------------------------------------------------------

/// Measures the effect of each basic operation (add, mul, fma, branch)
/// on the data, reporting throughput estimates per operation type.
pub struct MicroBench;

impl CompilerBench for MicroBench {
    fn name(&self) -> &str { "micro_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;
        if total == 0 {
            let mut r = BenchResult::new();
            r.insert("ops".into(), vec![]);
            return r;
        }

        // Simulate add chain
        let mut add_result = 0.0_f64;
        for &v in data.iter() {
            add_result += v;
        }

        // Simulate mul chain
        let mut mul_result = 1.0_f64;
        for &v in data.iter().take(total.min(64)) {
            mul_result *= 1.0 + v * 0.001;
        }

        // Simulate FMA (fused multiply-add)
        let mut fma_result = 0.0_f64;
        for i in (0..total).step_by(2) {
            let a = data[i];
            let b = if i + 1 < total { data[i + 1] } else { 1.0 };
            fma_result = fma_result.mul_add(a, b);
        }

        // Simulate branch cost: count sign changes
        let mut branches = 0usize;
        for i in 1..total {
            if (data[i] >= 0.0) != (data[i - 1] >= 0.0) {
                branches += 1;
            }
        }

        // Throughput estimates (ops / element)
        let add_throughput = total as f64; // 1 add per element
        let mul_throughput = total.min(64) as f64;
        let fma_throughput = (total / 2) as f64; // 1 FMA per 2 elements
        let branch_mispredict_rate = if total > 1 {
            branches as f64 / (total - 1) as f64
        } else { 0.0 };

        let mut r = BenchResult::new();
        r.insert("add_sum".into(), vec![add_result]);
        r.insert("mul_product".into(), vec![mul_result]);
        r.insert("fma_result".into(), vec![fma_result]);
        r.insert("branch_count".into(), vec![branches as f64]);
        r.insert("throughput_add".into(), vec![add_throughput]);
        r.insert("throughput_mul".into(), vec![mul_throughput]);
        r.insert("throughput_fma".into(), vec![fma_throughput]);
        r.insert("branch_mispredict_rate".into(), vec![branch_mispredict_rate]);
        r
    }
}

// ---------------------------------------------------------------------------
// 2. CacheBench — Cache hit rate estimation
// ---------------------------------------------------------------------------

/// Estimates cache behaviour by simulating a σ(6)=12 element cache line.
/// Measures spatial and temporal locality of access patterns.
pub struct CacheBench;

impl CompilerBench for CacheBench {
    fn name(&self) -> &str { "cache_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;
        let cache_line = SIGMA; // 12 elements per line
        let cache_lines = (total + cache_line - 1) / cache_line;

        // Spatial locality: fraction of sequential accesses
        let mut sequential = 0usize;
        for i in 1..total {
            // Sequential if same cache line or adjacent line
            let prev_line = (i - 1) / cache_line;
            let curr_line = i / cache_line;
            if curr_line == prev_line || curr_line == prev_line + 1 {
                sequential += 1;
            }
        }
        let spatial_locality = if total > 1 {
            sequential as f64 / (total - 1) as f64
        } else { 1.0 };

        // Temporal locality: measure value reuse (how many values appear near duplicates)
        let mut reuse_count = 0usize;
        let window = cache_line * PHI; // look-back window of 24
        for i in window..total {
            let target = (data[i] * 1000.0).round();
            for j in (i - window)..i {
                if (data[j] * 1000.0).round() == target {
                    reuse_count += 1;
                    break;
                }
            }
        }
        let temporal_locality = if total > window {
            reuse_count as f64 / (total - window) as f64
        } else { 0.0 };

        // Estimated hit rate: weighted combination
        let estimated_hit_rate = 0.6 * spatial_locality + 0.4 * temporal_locality;

        // Working set size: unique cache lines touched
        let working_set = cache_lines;

        let mut r = BenchResult::new();
        r.insert("spatial_locality".into(), vec![spatial_locality]);
        r.insert("temporal_locality".into(), vec![temporal_locality]);
        r.insert("estimated_hit_rate".into(), vec![estimated_hit_rate]);
        r.insert("cache_lines_total".into(), vec![cache_lines as f64]);
        r.insert("cache_line_size".into(), vec![cache_line as f64]);
        r.insert("working_set_lines".into(), vec![working_set as f64]);
        r.insert("working_set_bytes".into(), vec![(working_set * cache_line * 8) as f64]);
        r
    }
}

// ---------------------------------------------------------------------------
// 3. CompileBench — Compilation speed measurement
// ---------------------------------------------------------------------------

/// Estimates compilation complexity based on data characteristics:
/// expression complexity, type inference cost, and optimization passes.
pub struct CompileBench;

impl CompilerBench for CompileBench {
    fn name(&self) -> &str { "compile_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;

        // Expression complexity: count distinct value ranges (proxy for AST nodes)
        let num_buckets = SIGMA * N; // 72
        let mut bucket_hits = vec![false; num_buckets];
        for &v in data.iter() {
            let b = ((v.abs() * num_buckets as f64) as usize) % num_buckets;
            bucket_hits[b] = true;
        }
        let distinct_ranges = bucket_hits.iter().filter(|&&h| h).count();

        // Type inference cost: O(n * d) proportional to dimensions
        let type_inference_cost = (n as f64) * (d as f64).ln().max(1.0);

        // Optimization passes: proportional to data complexity
        let entropy = {
            let mut counts = vec![0usize; num_buckets];
            for &v in data.iter() {
                let b = ((v.abs() * num_buckets as f64) as usize) % num_buckets;
                counts[b] += 1;
            }
            let mut h = 0.0_f64;
            for &c in &counts {
                if c > 0 {
                    let p = c as f64 / total.max(1) as f64;
                    h -= p * p.ln();
                }
            }
            h
        };
        let opt_passes = (entropy * TAU as f64).ceil() as usize; // ~4x entropy

        // Estimated compile time (arbitrary units)
        let compile_time_estimate = type_inference_cost
            + (opt_passes as f64) * (total as f64).sqrt()
            + distinct_ranges as f64;

        let mut r = BenchResult::new();
        r.insert("distinct_value_ranges".into(), vec![distinct_ranges as f64]);
        r.insert("type_inference_cost".into(), vec![type_inference_cost]);
        r.insert("data_entropy".into(), vec![entropy]);
        r.insert("optimization_passes".into(), vec![opt_passes as f64]);
        r.insert("compile_time_estimate".into(), vec![compile_time_estimate]);
        r.insert("ast_complexity".into(), vec![total as f64 + distinct_ranges as f64]);
        r
    }
}

// ---------------------------------------------------------------------------
// 4. RealWorldBench — Real application pattern simulation
// ---------------------------------------------------------------------------

/// Simulates real-world computation patterns: matrix multiply, reduction,
/// scatter-gather, and stencil operations on the data.
pub struct RealWorldBench;

impl CompilerBench for RealWorldBench {
    fn name(&self) -> &str { "real_world_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;

        // Pattern 1: Row reduction (sum per row)
        let mut row_sums = Vec::with_capacity(n);
        for row in 0..n {
            let start = row * d;
            let end = (start + d).min(total);
            if start >= total { break; }
            let sum: f64 = data[start..end].iter().sum();
            row_sums.push(sum);
        }

        // Pattern 2: Dot product (first row · second row)
        let dot_product = if n >= 2 && d <= total {
            let min_d = d.min(total).min(total - d);
            (0..min_d).map(|j| data[j] * data[d + j]).sum::<f64>()
        } else { 0.0 };

        // Pattern 3: Stencil (3-point average)
        let mut stencil_result = Vec::with_capacity(total.saturating_sub(2));
        for i in 1..total.saturating_sub(1) {
            stencil_result.push((data[i - 1] + data[i] + data[i + 1]) / 3.0);
        }

        // Pattern 4: Scatter-gather (indirect access simulation)
        let mut gather_sum = 0.0_f64;
        let mut gather_count = 0usize;
        for i in 0..total.min(n) {
            let idx = ((data[i].abs() * total as f64) as usize) % total;
            gather_sum += data[idx];
            gather_count += 1;
        }

        // Compute arithmetic intensity (ops / bytes accessed)
        let total_ops = total                  // reductions
            + d.min(total / 2)                // dot product
            + total.saturating_sub(2) * 3     // stencil (3 reads + 2 adds)
            + gather_count;                   // scatter-gather
        let bytes_accessed = total * 8; // f64 = 8 bytes
        let arithmetic_intensity = total_ops as f64 / bytes_accessed.max(1) as f64;

        let mut r = BenchResult::new();
        r.insert("row_sums".into(), row_sums);
        r.insert("dot_product".into(), vec![dot_product]);
        r.insert("stencil_elements".into(), vec![stencil_result.len() as f64]);
        r.insert("gather_sum".into(), vec![gather_sum]);
        r.insert("total_ops".into(), vec![total_ops as f64]);
        r.insert("arithmetic_intensity".into(), vec![arithmetic_intensity]);
        r
    }
}

// ---------------------------------------------------------------------------
// 5. EnergyBench — Energy consumption estimation
// ---------------------------------------------------------------------------

/// Estimates relative energy consumption based on operation mix:
/// memory accesses (expensive), arithmetic (moderate), and branches (cheap).
/// Uses n=6 constants for energy ratios.
pub struct EnergyBench;

impl CompilerBench for EnergyBench {
    fn name(&self) -> &str { "energy_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;

        // Energy model (relative pJ per operation):
        //   memory load:  σ(6) = 12 pJ
        //   arithmetic:   φ(6) = 2 pJ
        //   branch:       1 pJ
        let mem_energy = SIGMA as f64;  // 12
        let alu_energy = PHI as f64;    // 2
        let branch_energy = 1.0;

        // Count operation types
        let mem_ops = total; // every element accessed at least once
        let alu_ops = total; // assume 1 arith op per element

        // Branch count: sign changes
        let mut branch_ops = 0usize;
        for i in 1..total {
            if (data[i] >= 0.0) != (data[i - 1] >= 0.0) {
                branch_ops += 1;
            }
        }

        let total_energy = (mem_ops as f64 * mem_energy)
            + (alu_ops as f64 * alu_energy)
            + (branch_ops as f64 * branch_energy);

        // Energy efficiency: useful work / total energy
        let useful_work: f64 = data.iter().map(|v| v.abs()).sum();
        let energy_efficiency = if total_energy > 0.0 {
            useful_work / total_energy
        } else { 0.0 };

        // Energy breakdown by component
        let mem_fraction = (mem_ops as f64 * mem_energy) / total_energy.max(1e-15);
        let alu_fraction = (alu_ops as f64 * alu_energy) / total_energy.max(1e-15);
        let branch_fraction = (branch_ops as f64 * branch_energy) / total_energy.max(1e-15);

        let mut r = BenchResult::new();
        r.insert("total_energy_pj".into(), vec![total_energy]);
        r.insert("mem_ops".into(), vec![mem_ops as f64]);
        r.insert("alu_ops".into(), vec![alu_ops as f64]);
        r.insert("branch_ops".into(), vec![branch_ops as f64]);
        r.insert("mem_fraction".into(), vec![mem_fraction]);
        r.insert("alu_fraction".into(), vec![alu_fraction]);
        r.insert("branch_fraction".into(), vec![branch_fraction]);
        r.insert("energy_efficiency".into(), vec![energy_efficiency]);
        r.insert("energy_per_element".into(), vec![
            if total > 0 { total_energy / total as f64 } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 6. SizeBench — Binary size estimation
// ---------------------------------------------------------------------------

/// Estimates the binary footprint of the data when compiled:
/// code section, data section, symbol table, and relocation entries.
pub struct SizeBench;

impl CompilerBench for SizeBench {
    fn name(&self) -> &str { "size_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;

        // Data section: raw f64 storage
        let data_section = total * 8; // 8 bytes per f64

        // Code section estimate: proportional to unique patterns
        let pattern_window = SOPFR; // 5-element patterns
        let mut unique_patterns = std::collections::HashSet::new();
        for i in 0..total.saturating_sub(pattern_window) {
            // Quantize pattern to detect duplicates
            let key: Vec<i64> = data[i..i + pattern_window]
                .iter()
                .map(|v| (v * 1000.0).round() as i64)
                .collect();
            unique_patterns.insert(key);
        }
        let code_section = unique_patterns.len() * TAU * 8; // ~4 instructions per pattern

        // Symbol table: one entry per row (function)
        let symbol_table = n * 24; // name_offset(8) + addr(8) + size(8)

        // Relocation entries: cross-row references
        let mut relocations = 0usize;
        for row in 1..n {
            let start = row * d;
            if start >= total { break; }
            let prev_start = (row - 1) * d;
            // Reference if first element matches any in previous row
            for j in 0..d.min(total - start) {
                for k in 0..d.min(total - prev_start) {
                    if (data[start + j] - data[prev_start + k]).abs() < 1e-10 {
                        relocations += 1;
                        break;
                    }
                }
            }
        }
        let relocation_section = relocations * 16; // 16 bytes per reloc

        let total_size = data_section + code_section + symbol_table + relocation_section;

        let mut r = BenchResult::new();
        r.insert("data_section_bytes".into(), vec![data_section as f64]);
        r.insert("code_section_bytes".into(), vec![code_section as f64]);
        r.insert("symbol_table_bytes".into(), vec![symbol_table as f64]);
        r.insert("relocation_bytes".into(), vec![relocation_section as f64]);
        r.insert("total_binary_size".into(), vec![total_size as f64]);
        r.insert("unique_patterns".into(), vec![unique_patterns.len() as f64]);
        r.insert("relocations".into(), vec![relocations as f64]);
        r.insert("code_density".into(), vec![
            if total_size > 0 { code_section as f64 / total_size as f64 } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 7. StartupBench — Cold start time estimation
// ---------------------------------------------------------------------------

/// Estimates cold-start latency by simulating initialization cost:
/// zero-initialization, page faults, and first-touch overhead.
pub struct StartupBench;

impl CompilerBench for StartupBench {
    fn name(&self) -> &str { "startup_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;
        let page_size = SIGMA * TAU; // 48 elements per "page"

        // Page fault simulation
        let num_pages = (total + page_size - 1) / page_size;
        let page_fault_cost = 1000.0; // ns per fault (simulated)
        let page_faults_total = num_pages as f64 * page_fault_cost;

        // Zero-init cost: count zero/near-zero elements that need init
        let near_zero = data.iter().filter(|v| v.abs() < 1e-10).count();
        let zero_init_cost = near_zero as f64 * 0.5; // ns per zero-init

        // First-touch: cost of first access per cache line
        let cache_line = SIGMA;
        let num_cache_lines = (total + cache_line - 1) / cache_line;
        let first_touch_cost = num_cache_lines as f64 * 10.0; // ns per first touch

        // Dynamic linking simulation: cost proportional to cross-row references
        let mut extern_refs = 0usize;
        for row in 0..n {
            let start = row * d;
            if start >= total { break; }
            let end = (start + d).min(total);
            // "External" if value range differs significantly from row mean
            let mean: f64 = data[start..end].iter().sum::<f64>() / (end - start).max(1) as f64;
            for &v in &data[start..end] {
                if (v - mean).abs() > mean.abs() * 2.0 {
                    extern_refs += 1;
                }
            }
        }
        let link_cost = extern_refs as f64 * 5.0; // ns per external resolution

        let total_startup_ns = page_faults_total + zero_init_cost + first_touch_cost + link_cost;

        let mut r = BenchResult::new();
        r.insert("page_faults_ns".into(), vec![page_faults_total]);
        r.insert("zero_init_ns".into(), vec![zero_init_cost]);
        r.insert("first_touch_ns".into(), vec![first_touch_cost]);
        r.insert("link_resolution_ns".into(), vec![link_cost]);
        r.insert("total_startup_ns".into(), vec![total_startup_ns]);
        r.insert("num_pages".into(), vec![num_pages as f64]);
        r.insert("extern_refs".into(), vec![extern_refs as f64]);
        r.insert("startup_per_element_ns".into(), vec![
            if total > 0 { total_startup_ns / total as f64 } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 8. LatencyBench — P99 latency estimation
// ---------------------------------------------------------------------------

/// Estimates per-row processing latency distribution and reports
/// P50, P90, P95, P99 percentiles.
pub struct LatencyBench;

impl CompilerBench for LatencyBench {
    fn name(&self) -> &str { "latency_bench" }

    fn measure(&self, data: &[f64], n: usize, d: usize) -> BenchResult {
        let total = n * d;
        if n == 0 || total == 0 {
            let mut r = BenchResult::new();
            r.insert("latencies".into(), vec![]);
            return r;
        }

        // Compute per-row "latency" based on computational complexity
        let mut latencies = Vec::with_capacity(n);
        for row in 0..n {
            let start = row * d;
            let end = (start + d).min(total);
            if start >= total { break; }
            let slice = &data[start..end];

            // Latency model: base + data-dependent component
            let base_latency = d as f64 * 0.5; // 0.5ns per element

            // Data-dependent: variance increases latency (branch misprediction)
            let mean = slice.iter().sum::<f64>() / slice.len().max(1) as f64;
            let variance = slice.iter()
                .map(|v| (v - mean).powi(2))
                .sum::<f64>() / slice.len().max(1) as f64;

            // Non-zero density affects memory latency
            let density = slice.iter().filter(|v| v.abs() > 1e-10).count() as f64
                / slice.len().max(1) as f64;

            let row_latency = base_latency
                + variance * SIGMA as f64      // variance penalty
                + (1.0 - density) * TAU as f64; // sparsity penalty

            latencies.push(row_latency);
        }

        // Sort for percentile computation
        let mut sorted = latencies.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

        let percentile = |p: f64| -> f64 {
            if sorted.is_empty() { return 0.0; }
            let idx = ((p / 100.0) * (sorted.len() - 1) as f64).round() as usize;
            sorted[idx.min(sorted.len() - 1)]
        };

        let mean_latency = if latencies.is_empty() { 0.0 }
            else { latencies.iter().sum::<f64>() / latencies.len() as f64 };

        let jitter = if latencies.len() > 1 {
            let var = latencies.iter()
                .map(|l| (l - mean_latency).powi(2))
                .sum::<f64>() / (latencies.len() - 1) as f64;
            var.sqrt()
        } else { 0.0 };

        let mut r = BenchResult::new();
        r.insert("latencies".into(), latencies);
        r.insert("p50".into(), vec![percentile(50.0)]);
        r.insert("p90".into(), vec![percentile(90.0)]);
        r.insert("p95".into(), vec![percentile(95.0)]);
        r.insert("p99".into(), vec![percentile(99.0)]);
        r.insert("mean_latency".into(), vec![mean_latency]);
        r.insert("jitter".into(), vec![jitter]);
        r.insert("min_latency".into(), vec![sorted.first().copied().unwrap_or(0.0)]);
        r.insert("max_latency".into(), vec![sorted.last().copied().unwrap_or(0.0)]);
        r
    }
}

// ---------------------------------------------------------------------------
// Registry: all 8 benchmarks
// ---------------------------------------------------------------------------

/// Returns all 8 compiler benchmarks.
pub fn all_benchmarks() -> Vec<Box<dyn CompilerBench>> {
    vec![
        Box::new(MicroBench),
        Box::new(CacheBench),
        Box::new(CompileBench),
        Box::new(RealWorldBench),
        Box::new(EnergyBench),
        Box::new(SizeBench),
        Box::new(StartupBench),
        Box::new(LatencyBench),
    ]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;

    fn make_data(n: usize, d: usize) -> Vec<f64> {
        (0..n * d).map(|i| ((i as f64) * 0.1).sin() * 0.5 + 0.5).collect()
    }

    #[test]
    fn test_micro_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = MicroBench.measure(&data, n, d);
        assert!(r["add_sum"][0].is_finite());
        assert!(r["mul_product"][0].is_finite());
        assert!(r["fma_result"][0].is_finite());
        assert!(r["branch_mispredict_rate"][0] >= 0.0);
        assert!(r["branch_mispredict_rate"][0] <= 1.0);
    }

    #[test]
    fn test_cache_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = CacheBench.measure(&data, n, d);
        assert!(r["spatial_locality"][0] >= 0.0 && r["spatial_locality"][0] <= 1.0);
        assert!(r["estimated_hit_rate"][0] >= 0.0 && r["estimated_hit_rate"][0] <= 1.0);
        assert_eq!(r["cache_line_size"][0], SIGMA as f64);
    }

    #[test]
    fn test_compile_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = CompileBench.measure(&data, n, d);
        assert!(r["distinct_value_ranges"][0] > 0.0);
        assert!(r["data_entropy"][0] > 0.0);
        assert!(r["compile_time_estimate"][0] > 0.0);
    }

    #[test]
    fn test_real_world_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = RealWorldBench.measure(&data, n, d);
        assert_eq!(r["row_sums"].len(), n);
        assert!(r["dot_product"][0].is_finite());
        assert!(r["arithmetic_intensity"][0] > 0.0);
    }

    #[test]
    fn test_energy_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = EnergyBench.measure(&data, n, d);
        assert!(r["total_energy_pj"][0] > 0.0);
        // Memory should dominate (σ=12 >> φ=2)
        assert!(r["mem_fraction"][0] > r["alu_fraction"][0]);
        let fractions = r["mem_fraction"][0] + r["alu_fraction"][0] + r["branch_fraction"][0];
        assert!((fractions - 1.0).abs() < 0.01, "fractions should sum to 1.0");
    }

    #[test]
    fn test_size_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = SizeBench.measure(&data, n, d);
        assert!(r["total_binary_size"][0] > 0.0);
        assert!(r["data_section_bytes"][0] == (n * d * 8) as f64);
        assert!(r["code_density"][0] >= 0.0 && r["code_density"][0] <= 1.0);
    }

    #[test]
    fn test_startup_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = StartupBench.measure(&data, n, d);
        assert!(r["total_startup_ns"][0] > 0.0);
        assert!(r["num_pages"][0] > 0.0);
        assert!(r["startup_per_element_ns"][0] > 0.0);
    }

    #[test]
    fn test_latency_bench() {
        let (n, d) = (6, 12);
        let data = make_data(n, d);
        let r = LatencyBench.measure(&data, n, d);
        assert_eq!(r["latencies"].len(), n);
        // P50 <= P90 <= P95 <= P99
        assert!(r["p50"][0] <= r["p90"][0] + 1e-10);
        assert!(r["p90"][0] <= r["p95"][0] + 1e-10);
        assert!(r["p95"][0] <= r["p99"][0] + 1e-10);
        assert!(r["jitter"][0] >= 0.0);
    }

    #[test]
    fn test_all_benchmarks_registry() {
        let benches = all_benchmarks();
        assert_eq!(benches.len(), 8);
        let names: Vec<&str> = benches.iter().map(|b| b.name()).collect();
        assert!(names.contains(&"micro_bench"));
        assert!(names.contains(&"latency_bench"));
    }

    #[test]
    fn test_all_benchmarks_run() {
        let (n, d) = (12, 12);
        let data = make_data(n, d);
        let benches = all_benchmarks();
        for b in &benches {
            let r = b.measure(&data, n, d);
            assert!(!r.is_empty(), "bench {} returned empty result", b.name());
        }
    }

    #[test]
    fn test_empty_data() {
        let data: Vec<f64> = vec![];
        let benches = all_benchmarks();
        for b in &benches {
            let r = b.measure(&data, 0, 0);
            // Should not panic on empty data
            let _ = r;
        }
    }
}
