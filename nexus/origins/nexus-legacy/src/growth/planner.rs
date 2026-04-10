//! Growth Plan Generator for NEXUS-6
//!
//! Analyzes current metrics and benchmarks against targets to produce
//! prioritized, actionable growth plans — including Claude Code CLI prompts.

use std::collections::HashMap;

use super::benchmark::BenchmarkSuite;
use super::metrics::NexusMetrics;
use super::tracker::GrowthTargets;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;           // the perfect number
const SIGMA: usize = 12;      // σ(6) = sum of divisors
const _PHI: usize = 2;        // φ(6) = Euler totient
const TAU: usize = 4;         // τ(6) = number of divisors
const _J2: usize = 24;        // J₂(6) = Jordan totient
const SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// A concrete action the growth engine recommends.
#[derive(Debug, Clone)]
pub enum GrowthAction {
    /// Implement a new lens with the given name and category.
    ImplementLens {
        lens_name: String,
        category: String,
    },
    /// Add tests to a specific module.
    AddTests {
        module: String,
        count: usize,
    },
    /// Optimize a bottleneck operation.
    OptimizeBottleneck {
        operation: String,
        current_ns: u64,
    },
    /// Fix compiler warnings.
    FixWarnings {
        count: usize,
    },
    /// Add a new module.
    AddModule {
        name: String,
        purpose: String,
    },
}

/// A complete growth plan with prioritized actions and estimated impacts.
#[derive(Debug, Clone)]
pub struct GrowthPlan {
    /// Ordered list of actions
    pub actions: Vec<GrowthAction>,
    /// Estimated impact per target area (area_name -> delta improvement)
    pub estimated_impact: HashMap<String, f64>,
    /// Priority order (indices into actions, highest priority first)
    pub priority_order: Vec<usize>,
}

/// Generate a prioritized growth plan from current state and targets.
pub fn generate_plan(
    metrics: &NexusMetrics,
    benchmarks: &BenchmarkSuite,
    targets: &GrowthTargets,
) -> GrowthPlan {
    let mut actions = Vec::new();
    let mut estimated_impact: HashMap<String, f64> = HashMap::new();

    // ── 1. Lens implementation gap ───────────────────────────────────
    if metrics.total_lenses_implemented < targets.target_lenses_impl {
        let gap = targets.target_lenses_impl - metrics.total_lenses_implemented;
        // Suggest implementing lenses in batches of n=6
        let batch_size = N;
        let batches = (gap + batch_size - 1) / batch_size;

        // Suggest first batch of lens categories
        let categories = ["Extended", "DomainCombo", "Custom"];
        for i in 0..batches.min(N) {
            // n=6 max suggestions
            let cat = categories[i % categories.len()];
            actions.push(GrowthAction::ImplementLens {
                lens_name: format!("growth_lens_{}", i + 1),
                category: cat.to_string(),
            });
        }

        let lens_impact = (gap as f64 / targets.target_lenses_impl as f64) * 0.4; // 40% health weight
        estimated_impact.insert("lenses_impl".to_string(), lens_impact);
    }

    // ── 2. Test coverage gap ─────────────────────────────────────────
    if metrics.total_tests < targets.target_tests {
        let gap = targets.target_tests - metrics.total_tests;
        // Suggest tests per module, σ-τ=8 tests per batch
        let modules = [
            "growth", "telescope", "calibration", "graph",
            "verifier", "encoder",
        ]; // n=6 modules
        let tests_per_module = (gap / modules.len()).max(SIGMA_MINUS_TAU); // at least σ-τ=8

        for module in &modules {
            actions.push(GrowthAction::AddTests {
                module: module.to_string(),
                count: tests_per_module,
            });
        }

        let test_impact = (gap as f64 / targets.target_tests as f64) * 0.3; // 30% health weight
        estimated_impact.insert("tests".to_string(), test_impact);
    }

    // ── 3. Bottleneck optimization ───────────────────────────────────
    if let Some(ref bottleneck_name) = benchmarks.bottleneck {
        if let Some(br) = benchmarks.results.iter().find(|r| &r.name == bottleneck_name) {
            // Only suggest optimization if throughput is below target
            if br.throughput < targets.target_scan_throughput {
                actions.push(GrowthAction::OptimizeBottleneck {
                    operation: bottleneck_name.clone(),
                    current_ns: br.mean_ns,
                });
                let perf_impact = 0.2; // 20% potential impact
                estimated_impact.insert("performance".to_string(), perf_impact);
            }
        }
    }

    // ── 4. Warning fixes ─────────────────────────────────────────────
    if metrics.compile_warnings > 0 {
        actions.push(GrowthAction::FixWarnings {
            count: metrics.compile_warnings,
        });
        let warn_impact = (metrics.compile_warnings as f64 / (SIGMA as f64)).min(0.1);
        estimated_impact.insert("warnings".to_string(), warn_impact);
    }

    // ── 5. Module suggestions ────────────────────────────────────────
    // If module count is below σ²=144 target, suggest new modules
    if metrics.total_modules < SIGMA * SIGMA {
        let suggested_modules = [
            ("monitoring", "Real-time system health monitoring"),
            ("caching", "Result caching for repeated scans"),
            ("export", "Export results to multiple formats"),
        ];
        for (name, purpose) in &suggested_modules {
            if metrics.total_modules + actions.len() < SIGMA * SIGMA {
                actions.push(GrowthAction::AddModule {
                    name: name.to_string(),
                    purpose: purpose.to_string(),
                });
            }
        }
    }

    // ── Priority ordering ────────────────────────────────────────────
    // Score each action by estimated impact: lenses > tests > bottleneck > warnings > modules
    let mut scored: Vec<(usize, f64)> = actions
        .iter()
        .enumerate()
        .map(|(i, action)| {
            let score = match action {
                GrowthAction::ImplementLens { .. } => 1.0,
                GrowthAction::AddTests { .. } => 0.8,
                GrowthAction::OptimizeBottleneck { .. } => 0.7,
                GrowthAction::FixWarnings { .. } => 0.5,
                GrowthAction::AddModule { .. } => 0.3,
            };
            (i, score)
        })
        .collect();
    scored.sort_by(|a, b| {
        b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal)
    });
    let priority_order: Vec<usize> = scored.into_iter().map(|(i, _)| i).collect();

    GrowthPlan {
        actions,
        estimated_impact,
        priority_order,
    }
}

/// Format a growth plan as a human-readable string.
pub fn format_plan(plan: &GrowthPlan) -> String {
    let mut s = String::new();
    s.push_str("┌──────────────────────────────────────────────────────────────┐\n");
    s.push_str("│              NEXUS-6 Growth Plan                            │\n");
    s.push_str("├──────────────────────────────────────────────────────────────┤\n");
    s.push_str(&format!(
        "│  Total actions: {:>3}                                         │\n",
        plan.actions.len()
    ));
    s.push_str("├──────────────────────────────────────────────────────────────┤\n");

    for (rank, &idx) in plan.priority_order.iter().enumerate() {
        if rank >= SIGMA {
            // Show at most σ=12 actions
            s.push_str(&format!(
                "│  ... and {} more actions                                    │\n",
                plan.priority_order.len() - SIGMA
            ));
            break;
        }
        let action = &plan.actions[idx];
        let desc = match action {
            GrowthAction::ImplementLens { lens_name, category } => {
                format!("LENS: {} ({})", lens_name, category)
            }
            GrowthAction::AddTests { module, count } => {
                format!("TEST: +{} tests for {}", count, module)
            }
            GrowthAction::OptimizeBottleneck { operation, current_ns } => {
                format!("PERF: optimize {} ({}ns)", operation, current_ns)
            }
            GrowthAction::FixWarnings { count } => {
                format!("WARN: fix {} warnings", count)
            }
            GrowthAction::AddModule { name, purpose } => {
                format!("MOD: add {} — {}", name, purpose)
            }
        };
        s.push_str(&format!(
            "│  #{:<2} {:<55} │\n",
            rank + 1,
            &desc[..desc.len().min(55)],
        ));
    }

    // Estimated impacts
    s.push_str("├──────────────────────────────────────────────────────────────┤\n");
    s.push_str("│  Estimated Impacts:                                        │\n");
    for (area, &impact) in &plan.estimated_impact {
        s.push_str(&format!(
            "│    {:<20} +{:.2} health                            │\n",
            area, impact
        ));
    }
    s.push_str("└──────────────────────────────────────────────────────────────┘\n");
    s
}

/// Generate a Claude Code CLI prompt for a specific growth action.
///
/// These prompts can be fed to `claude -p "..."` to auto-execute.
pub fn format_claude_prompt(action: &GrowthAction) -> String {
    match action {
        GrowthAction::ImplementLens { lens_name, category } => {
            format!(
                "Implement the {lens_name} lens in tools/nexus/src/telescope/lenses/{lens_name}.rs. \
                 It must implement the Lens trait with scan() method returning LensResult. \
                 Category: {category}. Include n=6 constants with comments. \
                 Add at least {TAU} tests (tau=4). Register in telescope/registry.rs.",
                lens_name = lens_name,
                category = category,
                TAU = TAU,
            )
        }
        GrowthAction::AddTests { module, count } => {
            format!(
                "Add {count} tests to the {module} module in tools/nexus/src/{module}/. \
                 Each test should verify a specific behavior. Use n=6 constants \
                 (N=6, sigma=12, phi=2, tau=4, J2=24) in test data where appropriate. \
                 Ensure all tests pass with `cargo test -p nexus`.",
                count = count,
                module = module,
            )
        }
        GrowthAction::OptimizeBottleneck { operation, current_ns } => {
            format!(
                "Optimize the {operation} operation in tools/nexus/ which currently takes \
                 {current_ns}ns per iteration. Profile the hot path and reduce allocations, \
                 use batch processing, or apply SIMD where possible. Target: at least \
                 {PHI}x speedup (phi=2). Add a benchmark test to verify improvement.",
                operation = operation,
                current_ns = current_ns,
                PHI = _PHI,
            )
        }
        GrowthAction::FixWarnings { count } => {
            format!(
                "Fix {count} compiler warnings in tools/nexus/. Run `cargo build -p nexus 2>&1` \
                 to see all warnings. Fix unused imports, dead code, and type mismatches. \
                 Goal: zero warnings.",
                count = count,
            )
        }
        GrowthAction::AddModule { name, purpose } => {
            format!(
                "Add a new module '{name}' to tools/nexus/src/{name}/ for: {purpose}. \
                 Create mod.rs with the module structure. Add `pub mod {name};` to lib.rs. \
                 Include at least {N} functions (n=6) and {TAU} tests (tau=4). \
                 Use n=6 constants throughout.",
                name = name,
                purpose = purpose,
                N = N,
                TAU = TAU,
            )
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    use crate::growth::benchmark::{BenchmarkResult, BenchmarkSuite};

    fn make_metrics(lenses_impl: usize, tests: usize, warnings: usize) -> NexusMetrics {
        NexusMetrics {
            total_modules: 42,
            total_tests: tests,
            total_lenses_registered: 693,
            total_lenses_implemented: lenses_impl,
            code_lines: 10000,
            compile_warnings: warnings,
            test_pass_rate: 1.0,
            health_score: 0.7,
            timestamp: "2026-04-03T00:00:00Z".to_string(),
        }
    }

    fn make_suite() -> BenchmarkSuite {
        BenchmarkSuite {
            results: vec![BenchmarkResult {
                name: "telescope_scan_n24".to_string(),
                iterations: 8,
                mean_ns: 5_000_000,
                min_ns: 4_000_000,
                max_ns: 6_000_000,
                throughput: 200.0, // below 1000 target
            }],
            total_time_ms: 50,
            bottleneck: Some("telescope_scan_n24".to_string()),
        }
    }

    #[test]
    fn test_generate_plan_with_gaps() {
        let metrics = make_metrics(20, 100, 3);
        let benchmarks = make_suite();
        let targets = GrowthTargets::default();

        let plan = generate_plan(&metrics, &benchmarks, &targets);

        assert!(!plan.actions.is_empty(), "should produce actions");
        assert!(!plan.priority_order.is_empty(), "should have priority order");
        assert_eq!(plan.priority_order.len(), plan.actions.len());

        // Should have lens, test, bottleneck, and warning actions
        let has_lens = plan.actions.iter().any(|a| matches!(a, GrowthAction::ImplementLens { .. }));
        let has_test = plan.actions.iter().any(|a| matches!(a, GrowthAction::AddTests { .. }));
        let has_perf = plan.actions.iter().any(|a| matches!(a, GrowthAction::OptimizeBottleneck { .. }));
        let has_warn = plan.actions.iter().any(|a| matches!(a, GrowthAction::FixWarnings { .. }));
        assert!(has_lens, "should suggest lens implementation");
        assert!(has_test, "should suggest adding tests");
        assert!(has_perf, "should suggest bottleneck optimization");
        assert!(has_warn, "should suggest fixing warnings");
    }

    #[test]
    fn test_generate_plan_all_targets_met() {
        let metrics = NexusMetrics {
            total_modules: 200, // above σ²=144
            total_tests: 2000,  // above 1000
            total_lenses_registered: 693,
            total_lenses_implemented: 200, // above 100
            code_lines: 50000,
            compile_warnings: 0,
            test_pass_rate: 1.0,
            health_score: 0.99,
            timestamp: "2026-04-03T00:00:00Z".to_string(),
        };
        let benchmarks = BenchmarkSuite {
            results: vec![BenchmarkResult {
                name: "telescope_scan_n24".to_string(),
                iterations: 8,
                mean_ns: 100_000,
                min_ns: 90_000,
                max_ns: 110_000,
                throughput: 10000.0, // above 1000 target
            }],
            total_time_ms: 10,
            bottleneck: Some("telescope_scan_n24".to_string()),
        };
        let targets = GrowthTargets::default();

        let plan = generate_plan(&metrics, &benchmarks, &targets);
        // With all targets met, plan may still suggest modules but no urgent gaps
        // No lens, test, perf, or warning actions needed
        let urgent = plan.actions.iter().filter(|a| {
            matches!(a, GrowthAction::ImplementLens { .. }
                | GrowthAction::AddTests { .. }
                | GrowthAction::OptimizeBottleneck { .. }
                | GrowthAction::FixWarnings { .. })
        }).count();
        assert_eq!(urgent, 0, "no urgent actions when targets are met");
    }

    #[test]
    fn test_format_plan() {
        let metrics = make_metrics(20, 100, 3);
        let benchmarks = make_suite();
        let targets = GrowthTargets::default();
        let plan = generate_plan(&metrics, &benchmarks, &targets);

        let formatted = format_plan(&plan);
        assert!(formatted.contains("NEXUS-6 Growth Plan"));
        assert!(formatted.contains("Total actions"));
        assert!(formatted.contains("Estimated Impacts"));
    }

    #[test]
    fn test_format_claude_prompt_lens() {
        let action = GrowthAction::ImplementLens {
            lens_name: "entropy_flow".to_string(),
            category: "Extended".to_string(),
        };
        let prompt = format_claude_prompt(&action);
        assert!(prompt.contains("entropy_flow"));
        assert!(prompt.contains("Lens trait"));
        assert!(prompt.contains("Extended"));
    }

    #[test]
    fn test_format_claude_prompt_tests() {
        let action = GrowthAction::AddTests {
            module: "telescope".to_string(),
            count: 8, // σ-τ=8
        };
        let prompt = format_claude_prompt(&action);
        assert!(prompt.contains("8 tests"));
        assert!(prompt.contains("telescope"));
    }

    #[test]
    fn test_format_claude_prompt_optimize() {
        let action = GrowthAction::OptimizeBottleneck {
            operation: "scan_all".to_string(),
            current_ns: 5_000_000,
        };
        let prompt = format_claude_prompt(&action);
        assert!(prompt.contains("scan_all"));
        assert!(prompt.contains("5000000ns"));
    }

    #[test]
    fn test_format_claude_prompt_module() {
        let action = GrowthAction::AddModule {
            name: "monitoring".to_string(),
            purpose: "Real-time health".to_string(),
        };
        let prompt = format_claude_prompt(&action);
        assert!(prompt.contains("monitoring"));
        assert!(prompt.contains("Real-time health"));
    }
}
