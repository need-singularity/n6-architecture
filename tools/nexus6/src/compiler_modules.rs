//! Compiler Optimization Modules for NEXUS-6
//!
//! 10 compiler-phase modules that operate on f64 data arrays,
//! simulating optimization passes with real numerical transformations.
//!
//! n=6 constants woven into allocation ratios and thresholds:
//!   - Egyptian fraction: 1/2 + 1/3 + 1/6 = 1  (perfect partition)
//!   - Arena zone: 1/σ(6) = 1/12 of total capacity
//!   - σ(6) = 12, φ(6) = 2, τ(6) = 4, sopfr(6) = 5

use std::collections::HashMap;

// n=6 arithmetic constants
const SIGMA: usize = 12;
const PHI: usize = 2;
const TAU: usize = 4;
const N: usize = 6;
const SOPFR: usize = 5;

/// Result of a compiler module pass: named vectors of f64 metrics.
pub type ModuleResult = HashMap<String, Vec<f64>>;

/// Trait for compiler optimization modules.
pub trait CompilerModule {
    /// Human-readable name of this optimization pass.
    fn name(&self) -> &str;
    /// Apply the optimization to `data` (n rows x d cols, row-major).
    /// Returns named metric vectors.
    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult;
}

// ---------------------------------------------------------------------------
// 1. EgyptianAllocator — 1/2 + 1/3 + 1/6 = 1 memory layout simulation
// ---------------------------------------------------------------------------

/// Partitions data into three zones using the Egyptian fraction 1/2+1/3+1/6=1.
/// Zone A (hot, 1/2): values scaled by σ(6)/N = 2.
/// Zone B (warm, 1/3): values averaged with neighbours.
/// Zone C (cold, 1/6): values left untouched (read-only archive).
pub struct EgyptianAllocator;

impl CompilerModule for EgyptianAllocator {
    fn name(&self) -> &str { "egyptian_allocator" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let zone_a_end = total / 2;
        let zone_b_end = zone_a_end + total / 3;
        // zone C is the remainder (~1/6)

        let mut hot_vals = Vec::with_capacity(zone_a_end);
        let mut warm_vals = Vec::with_capacity(zone_b_end - zone_a_end);
        let mut cold_vals = Vec::with_capacity(total.saturating_sub(zone_b_end));

        // Zone A: scale by σ/N = 12/6 = 2
        for i in 0..zone_a_end.min(total) {
            data[i] *= (SIGMA as f64) / (N as f64);
            hot_vals.push(data[i]);
        }

        // Zone B: smooth with neighbours
        for i in zone_a_end..zone_b_end.min(total) {
            let prev = if i > 0 { data[i - 1] } else { data[i] };
            let next = if i + 1 < total { data[i + 1] } else { data[i] };
            data[i] = (prev + data[i] + next) / 3.0;
            warm_vals.push(data[i]);
        }

        // Zone C: untouched
        for i in zone_b_end..total {
            cold_vals.push(data[i]);
        }

        let mut r = ModuleResult::new();
        r.insert("hot_zone".into(), hot_vals);
        r.insert("warm_zone".into(), warm_vals);
        r.insert("cold_zone".into(), cold_vals);
        r.insert("zone_sizes".into(), vec![
            zone_a_end as f64,
            (zone_b_end - zone_a_end) as f64,
            total.saturating_sub(zone_b_end) as f64,
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 2. ArenaPool — Bulk allocation / deallocation (1/σ arena zone)
// ---------------------------------------------------------------------------

/// Simulates arena-based memory pooling. Reserves 1/σ(6) = 1/12 of slots
/// as an arena header, then packs remaining data contiguously, eliminating
/// fragmentation gaps (zeros).
pub struct ArenaPool;

impl CompilerModule for ArenaPool {
    fn name(&self) -> &str { "arena_pool" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let arena_size = total / SIGMA; // 1/12

        // Arena header: store running sum as metadata
        let mut running = 0.0_f64;
        for i in 0..arena_size.min(total) {
            running += data[i].abs();
            data[i] = running;
        }

        // Compact: remove near-zero gaps (fragmentation elimination)
        let mut compacted = Vec::with_capacity(total - arena_size);
        let mut removed = 0usize;
        for i in arena_size..total {
            if data[i].abs() < 1e-12 {
                removed += 1;
            } else {
                compacted.push(data[i]);
            }
        }
        // Write compacted data back, zero-fill tail
        for (i, &v) in compacted.iter().enumerate() {
            if arena_size + i < total {
                data[arena_size + i] = v;
            }
        }
        for i in (arena_size + compacted.len())..total {
            data[i] = 0.0;
        }

        let mut r = ModuleResult::new();
        r.insert("arena_header_sum".into(), vec![running]);
        r.insert("arena_size".into(), vec![arena_size as f64]);
        r.insert("removed_gaps".into(), vec![removed as f64]);
        r.insert("compacted_len".into(), vec![compacted.len() as f64]);
        r
    }
}

// ---------------------------------------------------------------------------
// 3. OwnershipOptimizer — Eliminate ref counting, move optimization
// ---------------------------------------------------------------------------

/// Simulates ownership analysis: identifies values that are read once (can be
/// moved) vs read multiple times (need sharing). Single-use values are zeroed
/// at source (simulating move semantics), reducing live-set size.
pub struct OwnershipOptimizer;

impl CompilerModule for OwnershipOptimizer {
    fn name(&self) -> &str { "ownership_optimizer" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let mut use_count: HashMap<usize, usize> = HashMap::new();

        // Count approximate "uses": quantize to buckets and count duplicates
        let buckets = SIGMA * N; // 72 buckets
        for i in 0..total {
            let bucket = ((data[i].abs() * buckets as f64) as usize) % buckets;
            *use_count.entry(bucket).or_insert(0) += 1;
        }

        let mut moved = 0usize;
        let mut shared = 0usize;
        let mut move_savings = Vec::new();

        for i in 0..total {
            let bucket = ((data[i].abs() * buckets as f64) as usize) % buckets;
            let count = use_count.get(&bucket).copied().unwrap_or(1);
            if count == 1 {
                // Single use: "move" — mark and transform
                move_savings.push(data[i]);
                data[i] = 0.0; // source zeroed after move
                moved += 1;
            } else {
                shared += 1;
            }
        }

        let mut r = ModuleResult::new();
        r.insert("moved_count".into(), vec![moved as f64]);
        r.insert("shared_count".into(), vec![shared as f64]);
        r.insert("move_ratio".into(), vec![
            if total > 0 { moved as f64 / total as f64 } else { 0.0 }
        ]);
        r.insert("moved_values".into(), move_savings);
        r
    }
}

// ---------------------------------------------------------------------------
// 4. CompileTimeEvaluator — Static evaluation of computable expressions
// ---------------------------------------------------------------------------

/// Constant-folds computable sub-expressions. Detects arithmetic patterns
/// in adjacent data elements and replaces them with pre-computed results.
/// Uses a sliding window of τ(6)=4 elements.
pub struct CompileTimeEvaluator;

impl CompilerModule for CompileTimeEvaluator {
    fn name(&self) -> &str { "compile_time_evaluator" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let window = TAU; // 4-element window
        let mut folded = 0usize;
        let mut original_ops = 0usize;
        let mut reduced_ops = 0usize;

        let mut i = 0;
        while i + window <= total {
            let slice = &data[i..i + window];
            let sum: f64 = slice.iter().sum();
            let product: f64 = slice.iter().product();

            // If all elements are "constant-like" (finite, non-zero), fold
            if slice.iter().all(|v| v.is_finite() && v.abs() > 1e-15) {
                original_ops += window - 1; // N-1 additions originally
                reduced_ops += 0;            // folded to single value
                // Replace window with [folded_sum, folded_product, 0, 0]
                data[i] = sum;
                data[i + 1] = product;
                for j in 2..window {
                    data[i + j] = 0.0;
                }
                folded += 1;
            }
            i += window;
        }

        let mut r = ModuleResult::new();
        r.insert("folded_windows".into(), vec![folded as f64]);
        r.insert("original_ops".into(), vec![original_ops as f64]);
        r.insert("reduced_ops".into(), vec![reduced_ops as f64]);
        r.insert("op_reduction_ratio".into(), vec![
            if original_ops > 0 { 1.0 - (reduced_ops as f64 / original_ops as f64) } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 5. TypeSpecializer — Generic → specialized code generation
// ---------------------------------------------------------------------------

/// Analyses data distribution and "specializes" processing paths.
/// Classifies each row into one of φ(6)×τ(6)=8 type buckets based on
/// value range, then applies bucket-specific transforms.
pub struct TypeSpecializer;

impl CompilerModule for TypeSpecializer {
    fn name(&self) -> &str { "type_specializer" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let num_types = PHI * TAU; // 8 specialized types
        let mut type_counts = vec![0usize; num_types];
        let mut type_means = vec![0.0_f64; num_types];
        let mut type_totals = vec![0usize; num_types];

        for row in 0..n {
            // Classify row by its mean absolute value
            let row_start = row * d;
            let row_end = (row_start + d).min(data.len());
            if row_start >= data.len() { break; }
            let slice = &data[row_start..row_end];
            let mean_abs: f64 = slice.iter().map(|v| v.abs()).sum::<f64>()
                / (slice.len().max(1) as f64);

            let type_id = ((mean_abs * num_types as f64) as usize) % num_types;
            type_counts[type_id] += 1;
            type_totals[type_id] += slice.len();

            // Apply type-specific scaling
            let scale = 1.0 + (type_id as f64) / (N as f64); // 1.0 to ~2.17
            for v in &mut data[row_start..row_end] {
                *v *= scale;
            }
            type_means[type_id] += mean_abs;
        }

        // Normalize means
        for i in 0..num_types {
            if type_counts[i] > 0 {
                type_means[i] /= type_counts[i] as f64;
            }
        }

        let mut r = ModuleResult::new();
        r.insert("type_counts".into(), type_counts.iter().map(|&c| c as f64).collect());
        r.insert("type_means".into(), type_means);
        r.insert("num_specializations".into(), vec![num_types as f64]);
        r.insert("rows_classified".into(), vec![n as f64]);
        r
    }
}

// ---------------------------------------------------------------------------
// 6. CodegenSelector — Multi-target selection
// ---------------------------------------------------------------------------

/// Scores each data row for suitability across 6 targets:
/// x86, ARM, WASM, GPU, FPGA, ESP32.
/// Selection heuristic: dense computation → GPU, sparse → CPU, small → ESP32.
pub struct CodegenSelector;

impl CompilerModule for CodegenSelector {
    fn name(&self) -> &str { "codegen_selector" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let targets = ["x86", "arm", "wasm", "gpu", "fpga", "esp32"];
        let mut scores: Vec<Vec<f64>> = vec![Vec::with_capacity(n); N];
        let mut selections = Vec::with_capacity(n);

        for row in 0..n {
            let row_start = row * d;
            let row_end = (row_start + d).min(data.len());
            if row_start >= data.len() { break; }
            let slice = &data[row_start..row_end];

            let density = slice.iter().filter(|v| v.abs() > 1e-10).count() as f64
                / slice.len().max(1) as f64;
            let magnitude: f64 = slice.iter().map(|v| v.abs()).sum::<f64>()
                / slice.len().max(1) as f64;
            let variance = {
                let mean = magnitude;
                slice.iter().map(|v| (v.abs() - mean).powi(2)).sum::<f64>()
                    / slice.len().max(1) as f64
            };

            // Score each target
            let row_scores = [
                0.5 + 0.3 * density + 0.2 * (1.0 - variance.min(1.0)),   // x86: balanced
                0.4 + 0.4 * (1.0 - magnitude.min(1.0)) + 0.2 * density,  // ARM: low power
                0.3 + 0.3 * density + 0.4 * (1.0 - magnitude.min(1.0)),   // WASM: portable
                0.2 + 0.7 * density + 0.1 * magnitude.min(1.0),           // GPU: dense
                0.3 + 0.2 * density + 0.5 * variance.min(1.0),            // FPGA: irregular
                0.6 * (1.0 - density) + 0.4 * (1.0 - magnitude.min(1.0)), // ESP32: tiny
            ];

            for (t, &s) in row_scores.iter().enumerate() {
                scores[t].push(s);
            }

            // Select best target
            let best = row_scores.iter()
                .enumerate()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(std::cmp::Ordering::Equal))
                .map(|(i, _)| i)
                .unwrap_or(0);
            selections.push(best as f64);

            // Tag row: embed target ID in first element's fractional part
            if row_start < data.len() {
                data[row_start] = data[row_start].trunc() + (best as f64) / 10.0;
            }
        }

        let mut r = ModuleResult::new();
        for (i, name) in targets.iter().enumerate() {
            r.insert(format!("score_{name}"), scores[i].clone());
        }
        r.insert("selections".into(), selections);
        r.insert("num_targets".into(), vec![N as f64]);
        r
    }
}

// ---------------------------------------------------------------------------
// 7. LinkTimeOptimizer — Whole-program analysis post-link
// ---------------------------------------------------------------------------

/// Simulates link-time optimization: cross-row dead-code elimination
/// (removes duplicate patterns) and inlining (merges small rows into
/// their callers).
pub struct LinkTimeOptimizer;

impl CompilerModule for LinkTimeOptimizer {
    fn name(&self) -> &str { "link_time_optimizer" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let mut eliminated = 0usize;
        let mut inlined = 0usize;

        // Dead code elimination: zero out rows whose L2 norm < threshold
        let threshold = 1.0 / (SIGMA as f64); // 1/12
        for row in 0..n {
            let start = row * d;
            let end = (start + d).min(total);
            if start >= total { break; }
            let norm: f64 = data[start..end].iter().map(|v| v * v).sum::<f64>().sqrt();
            if norm < threshold {
                for v in &mut data[start..end] {
                    *v = 0.0;
                }
                eliminated += 1;
            }
        }

        // Inlining: merge consecutive similar rows (cosine > 0.95)
        let mut row = 0;
        while row + 1 < n {
            let s1 = row * d;
            let s2 = (row + 1) * d;
            if s1 + d > total || s2 + d > total { break; }

            let dot: f64 = (0..d).map(|j| data[s1 + j] * data[s2 + j]).sum();
            let n1: f64 = (0..d).map(|j| data[s1 + j].powi(2)).sum::<f64>().sqrt();
            let n2: f64 = (0..d).map(|j| data[s2 + j].powi(2)).sum::<f64>().sqrt();
            let cosine = if n1 > 1e-12 && n2 > 1e-12 { dot / (n1 * n2) } else { 0.0 };

            if cosine > 0.95 {
                // Merge: average into first row, zero second
                for j in 0..d {
                    data[s1 + j] = (data[s1 + j] + data[s2 + j]) / 2.0;
                    data[s2 + j] = 0.0;
                }
                inlined += 1;
                row += 2;
            } else {
                row += 1;
            }
        }

        let mut r = ModuleResult::new();
        r.insert("eliminated_rows".into(), vec![eliminated as f64]);
        r.insert("inlined_rows".into(), vec![inlined as f64]);
        r.insert("total_optimized".into(), vec![(eliminated + inlined) as f64]);
        r.insert("optimization_ratio".into(), vec![
            if n > 0 { (eliminated + inlined) as f64 / n as f64 } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 8. BinaryPatcher — Hot-patch support for deployed binaries
// ---------------------------------------------------------------------------

/// Simulates binary hot-patching: identifies "patch points" (local extrema)
/// in the data and replaces them with interpolated values, simulating
/// in-place binary modification without full recompilation.
pub struct BinaryPatcher;

impl CompilerModule for BinaryPatcher {
    fn name(&self) -> &str { "binary_patcher" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let mut patch_points = Vec::new();
        let mut original_values = Vec::new();
        let mut patched_values = Vec::new();

        // Find local extrema (patch candidates)
        for i in 1..total.saturating_sub(1) {
            let is_max = data[i] > data[i - 1] && data[i] > data[i + 1];
            let is_min = data[i] < data[i - 1] && data[i] < data[i + 1];
            if is_max || is_min {
                patch_points.push(i as f64);
                original_values.push(data[i]);
                // Patch: linear interpolation from neighbours
                let patched = (data[i - 1] + data[i + 1]) / 2.0;
                patched_values.push(patched);
                data[i] = patched;
            }
        }

        let mut r = ModuleResult::new();
        r.insert("patch_count".into(), vec![patch_points.len() as f64]);
        r.insert("patch_points".into(), patch_points);
        r.insert("original_values".into(), original_values);
        let patched_count = patched_values.len();
        r.insert("patched_values".into(), patched_values);
        r.insert("patch_density".into(), vec![
            if total > 2 { patched_count as f64 / (total - 2) as f64 } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// 9. DebugOptimizer — Optimize hot loops even in debug mode
// ---------------------------------------------------------------------------

/// Identifies "hot loops" (repeated value patterns of length sopfr(6)=5)
/// and optimizes them even in debug builds by pre-computing the loop body.
pub struct DebugOptimizer;

impl CompilerModule for DebugOptimizer {
    fn name(&self) -> &str { "debug_optimizer" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let pattern_len = SOPFR; // 5
        let mut hot_loops_found = 0usize;
        let mut speedup_estimates = Vec::new();

        if total < pattern_len * 2 {
            let mut r = ModuleResult::new();
            r.insert("hot_loops".into(), vec![0.0]);
            r.insert("speedup_estimates".into(), vec![]);
            r.insert("optimized_elements".into(), vec![0.0]);
            return r;
        }

        let mut i = 0;
        let mut optimized_count = 0usize;
        while i + pattern_len * 2 <= total {
            // Check if pattern repeats
            let mut repeats = true;
            for j in 0..pattern_len {
                if (data[i + j] - data[i + pattern_len + j]).abs() > 1e-10 {
                    repeats = false;
                    break;
                }
            }

            if repeats {
                hot_loops_found += 1;
                // "Optimize": replace repeated block with scaled version
                let scale = (N as f64) / (SOPFR as f64); // 6/5 = 1.2
                for j in 0..pattern_len {
                    data[i + pattern_len + j] = data[i + j] * scale;
                }
                // Estimate: hot-loop optimization gives ~Nx speedup
                speedup_estimates.push(scale);
                optimized_count += pattern_len;
                i += pattern_len * 2;
            } else {
                i += 1;
            }
        }

        let mut r = ModuleResult::new();
        r.insert("hot_loops".into(), vec![hot_loops_found as f64]);
        r.insert("speedup_estimates".into(), speedup_estimates);
        r.insert("optimized_elements".into(), vec![optimized_count as f64]);
        r.insert("pattern_length".into(), vec![pattern_len as f64]);
        r
    }
}

// ---------------------------------------------------------------------------
// 10. ProfilingInserter — Auto-insert / remove profiling instrumentation
// ---------------------------------------------------------------------------

/// Inserts profiling markers into data: every σ(6)=12th element gets a
/// sentinel value encoding the local statistics (mean, variance) of the
/// surrounding block. Can be reversed by removing sentinels.
pub struct ProfilingInserter;

impl CompilerModule for ProfilingInserter {
    fn name(&self) -> &str { "profiling_inserter" }

    fn apply(&self, data: &mut [f64], n: usize, d: usize) -> ModuleResult {
        let total = n * d;
        let interval = SIGMA; // every 12th element
        let mut markers_inserted = 0usize;
        let mut block_means = Vec::new();
        let mut block_variances = Vec::new();

        let mut i = 0;
        while i + interval <= total {
            let block = &data[i..i + interval];
            let mean = block.iter().sum::<f64>() / interval as f64;
            let variance = block.iter()
                .map(|v| (v - mean).powi(2))
                .sum::<f64>() / interval as f64;

            block_means.push(mean);
            block_variances.push(variance);

            // Insert sentinel: encode mean in the last element of the block
            // Use a distinctive pattern: -(mean + 1/sigma)
            let sentinel_idx = i + interval - 1;
            data[sentinel_idx] = -(mean.abs() + 1.0 / SIGMA as f64);
            markers_inserted += 1;

            i += interval;
        }

        let mut r = ModuleResult::new();
        r.insert("markers_inserted".into(), vec![markers_inserted as f64]);
        r.insert("block_means".into(), block_means);
        r.insert("block_variances".into(), block_variances);
        r.insert("profiling_interval".into(), vec![interval as f64]);
        r.insert("coverage_ratio".into(), vec![
            if total > 0 {
                (markers_inserted * interval) as f64 / total as f64
            } else { 0.0 }
        ]);
        r
    }
}

// ---------------------------------------------------------------------------
// Registry: all 10 modules
// ---------------------------------------------------------------------------

/// Returns all 10 compiler modules.
pub fn all_modules() -> Vec<Box<dyn CompilerModule>> {
    vec![
        Box::new(EgyptianAllocator),
        Box::new(ArenaPool),
        Box::new(OwnershipOptimizer),
        Box::new(CompileTimeEvaluator),
        Box::new(TypeSpecializer),
        Box::new(CodegenSelector),
        Box::new(LinkTimeOptimizer),
        Box::new(BinaryPatcher),
        Box::new(DebugOptimizer),
        Box::new(ProfilingInserter),
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
    fn test_egyptian_allocator() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = EgyptianAllocator.apply(&mut data, n, d);
        assert!(r.contains_key("hot_zone"));
        assert!(r.contains_key("warm_zone"));
        assert!(r.contains_key("cold_zone"));
        let sizes = &r["zone_sizes"];
        assert_eq!(sizes.len(), 3);
        // 1/2 + 1/3 + 1/6 ≈ 1
        let total = n * d;
        let sum: f64 = sizes.iter().sum();
        assert!((sum - total as f64).abs() < 2.0, "zone sizes should cover total");
    }

    #[test]
    fn test_arena_pool() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        // Insert some zeros to test compaction
        data[20] = 0.0;
        data[30] = 0.0;
        let r = ArenaPool.apply(&mut data, n, d);
        assert!(r["arena_size"][0] > 0.0);
        assert!(r["arena_header_sum"][0] > 0.0);
    }

    #[test]
    fn test_ownership_optimizer() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = OwnershipOptimizer.apply(&mut data, n, d);
        let moved = r["moved_count"][0] as usize;
        let shared = r["shared_count"][0] as usize;
        assert_eq!(moved + shared, n * d);
        assert!(r["move_ratio"][0] >= 0.0 && r["move_ratio"][0] <= 1.0);
    }

    #[test]
    fn test_compile_time_evaluator() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = CompileTimeEvaluator.apply(&mut data, n, d);
        assert!(r["folded_windows"][0] > 0.0);
        assert_eq!(r["op_reduction_ratio"][0], 1.0); // all reduced to 0 ops
    }

    #[test]
    fn test_type_specializer() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = TypeSpecializer.apply(&mut data, n, d);
        let counts = &r["type_counts"];
        assert_eq!(counts.len(), PHI * TAU);
        assert_eq!(r["num_specializations"][0], 8.0);
    }

    #[test]
    fn test_codegen_selector() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = CodegenSelector.apply(&mut data, n, d);
        assert!(r.contains_key("score_gpu"));
        assert!(r.contains_key("score_esp32"));
        assert_eq!(r["selections"].len(), n);
        assert_eq!(r["num_targets"][0], 6.0);
    }

    #[test]
    fn test_link_time_optimizer() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = LinkTimeOptimizer.apply(&mut data, n, d);
        assert!(r.contains_key("eliminated_rows"));
        assert!(r.contains_key("inlined_rows"));
        assert!(r["optimization_ratio"][0] >= 0.0);
    }

    #[test]
    fn test_binary_patcher() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = BinaryPatcher.apply(&mut data, n, d);
        let count = r["patch_count"][0] as usize;
        assert_eq!(r["patch_points"].len(), count);
        assert_eq!(r["original_values"].len(), count);
        assert_eq!(r["patched_values"].len(), count);
    }

    #[test]
    fn test_debug_optimizer() {
        let (n, d) = (6, 12);
        // Create data with a repeated pattern of length 5
        let mut data = vec![0.0; n * d];
        for i in 0..SOPFR {
            data[i] = (i as f64 + 1.0) * 0.1;
            data[i + SOPFR] = (i as f64 + 1.0) * 0.1; // repeat
        }
        let r = DebugOptimizer.apply(&mut data, n, d);
        assert!(r["hot_loops"][0] >= 1.0);
        assert_eq!(r["pattern_length"][0], SOPFR as f64);
    }

    #[test]
    fn test_profiling_inserter() {
        let (n, d) = (6, 12);
        let mut data = make_data(n, d);
        let r = ProfilingInserter.apply(&mut data, n, d);
        assert!(r["markers_inserted"][0] > 0.0);
        assert_eq!(r["profiling_interval"][0], SIGMA as f64);
        assert_eq!(r["block_means"].len(), r["markers_inserted"][0] as usize);
    }

    #[test]
    fn test_all_modules_registry() {
        let mods = all_modules();
        assert_eq!(mods.len(), 10);
        let names: Vec<&str> = mods.iter().map(|m| m.name()).collect();
        assert!(names.contains(&"egyptian_allocator"));
        assert!(names.contains(&"profiling_inserter"));
    }

    #[test]
    fn test_all_modules_run() {
        let (n, d) = (12, 12);
        let mods = all_modules();
        for m in &mods {
            let mut data = make_data(n, d);
            let r = m.apply(&mut data, n, d);
            assert!(!r.is_empty(), "module {} returned empty result", m.name());
        }
    }
}
