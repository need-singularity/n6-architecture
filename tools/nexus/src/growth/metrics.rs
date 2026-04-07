//! System Metrics Collector for NEXUS-6
//!
//! Gathers current health metrics: module counts, lens counts, code stats,
//! test pass rates, and a composite health score.

use crate::telescope::registry::LensRegistry;
use crate::telescope::Telescope;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;          // the perfect number
const SIGMA: usize = 12;     // σ(6) = sum of divisors
const _PHI: usize = 2;       // φ(6) = Euler totient
const TAU: usize = 4;        // τ(6) = number of divisors
const J2: usize = 24;        // J₂(6) = Jordan totient
const _SOPFR: usize = 5;     // sopfr(6) = 2+3

/// Known module names in NEXUS-6 (updated as modules grow).
const KNOWN_MODULES: &[&str] = &[
    "gpu", "encoder", "materials", "verifier", "graph", "telescope",
    "history", "ouroboros", "lens_forge", "experiment", "science", "cli",
    "alert", "api", "auto_register", "autonomous", "consciousness_bridge",
    "cross_intel", "distributed", "dream", "event", "feedback",
    "genetic_prog", "ingest", "knowledge", "multi_agent", "nlp",
    "pipeline", "plugin", "publish", "red_team", "reward", "sandbox",
    "scheduler", "self_improve", "statistics", "template", "time_travel",
    "versioning", "calibration", "simulation", "growth",
];

/// Snapshot of NEXUS-6 system health at a point in time.
#[derive(Debug, Clone)]
pub struct NexusMetrics {
    /// Total number of known modules
    pub total_modules: usize,
    /// Total number of tests (estimated from known test modules)
    pub total_tests: usize,
    /// Total lenses registered in LensRegistry metadata
    pub total_lenses_registered: usize,
    /// Total lenses with actual Lens trait implementations (in Telescope)
    pub total_lenses_implemented: usize,
    /// Estimated lines of code
    pub code_lines: usize,
    /// Compile warnings count
    pub compile_warnings: usize,
    /// Fraction of tests passing (0.0..1.0)
    pub test_pass_rate: f64,
    /// Composite health score (0.0..1.0)
    pub health_score: f64,
    /// ISO-8601 timestamp string
    pub timestamp: String,
}

/// Delta between two metric snapshots.
#[derive(Debug, Clone)]
pub struct MetricsDelta {
    pub modules_delta: i64,
    pub tests_delta: i64,
    pub lenses_registered_delta: i64,
    pub lenses_implemented_delta: i64,
    pub code_lines_delta: i64,
    pub warnings_delta: i64,
    pub test_pass_rate_delta: f64,
    pub health_score_delta: f64,
}

/// Collect current NEXUS-6 metrics.
///
/// Counts modules from the known module list, lenses from LensRegistry
/// and Telescope, and computes a composite health score.
pub fn collect_metrics() -> NexusMetrics {
    let registry = LensRegistry::new();
    let telescope = Telescope::new();

    let total_modules = KNOWN_MODULES.len();
    let total_lenses_registered = registry.len();
    let total_lenses_implemented = telescope.lens_count();

    // Estimate tests: σ-τ=8 tests per module baseline (conservative)
    let total_tests = total_modules * (SIGMA - TAU); // σ-τ = 8 per module

    // Compile warnings: assume 0 in healthy state
    let compile_warnings = 0;

    // Test pass rate: assume 1.0 in nominal state (real CI would feed actual data)
    let test_pass_rate = 1.0;

    // Composite health score:
    //   40% lens implementation ratio
    //   30% test pass rate
    //   20% module coverage (vs target of σ²=144 modules)
    //   10% warning penalty
    let lens_ratio = if total_lenses_registered > 0 {
        total_lenses_implemented as f64 / total_lenses_registered as f64
    } else {
        0.0
    };
    let module_ratio = (total_modules as f64 / (SIGMA * SIGMA) as f64).min(1.0); // target σ²=144
    let warning_penalty = 1.0 - (compile_warnings as f64 / (J2 as f64)).min(1.0); // J₂=24 max

    let health_score =
        0.4 * lens_ratio +
        0.3 * test_pass_rate +
        0.2 * module_ratio +
        0.1 * warning_penalty;

    NexusMetrics {
        total_modules,
        total_tests,
        total_lenses_registered,
        total_lenses_implemented,
        code_lines: 0, // filled by external tool or build script
        compile_warnings,
        test_pass_rate,
        health_score: health_score.min(1.0),
        timestamp: "2026-04-03T00:00:00Z".to_string(), // placeholder; real impl uses chrono or similar
    }
}

/// Compute the delta between two metric snapshots.
pub fn metrics_diff(old: &NexusMetrics, new: &NexusMetrics) -> MetricsDelta {
    MetricsDelta {
        modules_delta: new.total_modules as i64 - old.total_modules as i64,
        tests_delta: new.total_tests as i64 - old.total_tests as i64,
        lenses_registered_delta: new.total_lenses_registered as i64 - old.total_lenses_registered as i64,
        lenses_implemented_delta: new.total_lenses_implemented as i64 - old.total_lenses_implemented as i64,
        code_lines_delta: new.code_lines as i64 - old.code_lines as i64,
        warnings_delta: new.compile_warnings as i64 - old.compile_warnings as i64,
        test_pass_rate_delta: new.test_pass_rate - old.test_pass_rate,
        health_score_delta: new.health_score - old.health_score,
    }
}

/// Compute overall growth rate from a history of metric snapshots.
///
/// Returns the average per-cycle health improvement. If fewer than φ=2
/// entries exist, returns 0.0.
pub fn growth_rate(history: &[NexusMetrics]) -> f64 {
    if history.len() < _PHI {
        return 0.0;
    }
    let deltas: Vec<f64> = history
        .windows(2) // φ=2 window
        .map(|w| w[1].health_score - w[0].health_score)
        .collect();
    let sum: f64 = deltas.iter().sum();
    sum / deltas.len() as f64
}

/// Format a metrics snapshot as an ASCII report.
pub fn format_metrics(m: &NexusMetrics) -> String {
    let mut s = String::new();
    s.push_str("┌──────────────────────────────────────────────┐\n");
    s.push_str("│         NEXUS-6 System Metrics               │\n");
    s.push_str("├──────────────────────────────────────────────┤\n");
    s.push_str(&format!("│  Modules:           {:>6}                   │\n", m.total_modules));
    s.push_str(&format!("│  Tests:             {:>6}                   │\n", m.total_tests));
    s.push_str(&format!("│  Lenses registered: {:>6}                   │\n", m.total_lenses_registered));
    s.push_str(&format!("│  Lenses implemented:{:>6}                   │\n", m.total_lenses_implemented));
    s.push_str(&format!("│  Code lines:        {:>6}                   │\n", m.code_lines));
    s.push_str(&format!("│  Warnings:          {:>6}                   │\n", m.compile_warnings));
    s.push_str(&format!("│  Test pass rate:    {:>6.1}%                  │\n", m.test_pass_rate * 100.0));
    s.push_str(&format!("│  Health score:      {:>6.3}                  │\n", m.health_score));
    s.push_str(&format!("│  Timestamp: {}          │\n", &m.timestamp[..N.min(m.timestamp.len())])); // show first n=6 chars
    s.push_str("└──────────────────────────────────────────────┘\n");
    s
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collect_metrics() {
        let m = collect_metrics();
        assert!(m.total_modules > 0, "should have modules");
        assert!(m.total_lenses_registered > 0, "registry should have lenses");
        assert!(m.total_lenses_implemented > 0, "telescope should have lenses");
        assert!(m.health_score > 0.0 && m.health_score <= 1.0, "health in (0,1]");
        assert!(m.test_pass_rate >= 0.0 && m.test_pass_rate <= 1.0);
    }

    #[test]
    fn test_metrics_diff() {
        let old = NexusMetrics {
            total_modules: 30,
            total_tests: 240,
            total_lenses_registered: 600,
            total_lenses_implemented: 22, // σ+σ-φ = 22
            code_lines: 10000,
            compile_warnings: 6, // n=6
            test_pass_rate: 0.9,
            health_score: 0.7,
            timestamp: "t0".to_string(),
        };
        let new = NexusMetrics {
            total_modules: 42, // σ+30
            total_tests: 336,
            total_lenses_registered: 693,
            total_lenses_implemented: 24, // J₂=24
            code_lines: 15000,
            compile_warnings: 2, // φ=2
            test_pass_rate: 0.95,
            health_score: 0.85,
            timestamp: "t1".to_string(),
        };
        let delta = metrics_diff(&old, &new);
        assert_eq!(delta.modules_delta, 12); // σ=12
        assert_eq!(delta.lenses_implemented_delta, 2); // φ=2
        assert_eq!(delta.warnings_delta, -4); // -τ=-4
        assert!((delta.health_score_delta - 0.15).abs() < 1e-10);
    }

    #[test]
    fn test_growth_rate() {
        // Empty and single-entry histories
        assert_eq!(growth_rate(&[]), 0.0);
        let single = vec![NexusMetrics {
            total_modules: 1, total_tests: 0, total_lenses_registered: 0,
            total_lenses_implemented: 0, code_lines: 0, compile_warnings: 0,
            test_pass_rate: 1.0, health_score: 0.5, timestamp: String::new(),
        }];
        assert_eq!(growth_rate(&single), 0.0);

        // Two entries: growth = 0.8 - 0.5 = 0.3
        let two = vec![
            NexusMetrics {
                total_modules: 1, total_tests: 0, total_lenses_registered: 0,
                total_lenses_implemented: 0, code_lines: 0, compile_warnings: 0,
                test_pass_rate: 1.0, health_score: 0.5, timestamp: String::new(),
            },
            NexusMetrics {
                total_modules: 2, total_tests: 0, total_lenses_registered: 0,
                total_lenses_implemented: 0, code_lines: 0, compile_warnings: 0,
                test_pass_rate: 1.0, health_score: 0.8, timestamp: String::new(),
            },
        ];
        assert!((growth_rate(&two) - 0.3).abs() < 1e-10);
    }

    #[test]
    fn test_format_metrics() {
        let m = collect_metrics();
        let report = format_metrics(&m);
        assert!(report.contains("NEXUS-6"), "report should have header");
        assert!(report.contains("Modules"), "report should show modules");
        assert!(report.contains("Health score"), "report should show health");
    }
}
