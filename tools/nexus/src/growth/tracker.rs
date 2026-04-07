//! Growth History Tracker for NEXUS-6
//!
//! Tracks metrics and benchmarks over time, computes trends, measures
//! distance to targets, and suggests priorities for growth.

use std::collections::HashMap;

use super::benchmark::BenchmarkSuite;
use super::metrics::NexusMetrics;

// ── n=6 constants ────────────────────────────────────────────────────
const _N: usize = 6;          // the perfect number
const SIGMA: usize = 12;      // σ(6) = sum of divisors
const PHI: usize = 2;         // φ(6) = Euler totient
const _TAU: usize = 4;        // τ(6) = number of divisors
const _J2: usize = 24;        // J₂(6) = Jordan totient
const SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10

/// A single growth cycle entry containing metrics, benchmarks, and actions.
#[derive(Debug, Clone)]
pub struct GrowthEntry {
    /// Cycle number (1-indexed)
    pub cycle: usize,
    /// System metrics snapshot
    pub metrics: NexusMetrics,
    /// Benchmark suite results
    pub benchmarks: BenchmarkSuite,
    /// Actions taken during this cycle
    pub actions_taken: Vec<String>,
    /// Improvements observed
    pub improvements: Vec<String>,
}

/// Target thresholds for growth goals.
#[derive(Debug, Clone)]
pub struct GrowthTargets {
    /// Target number of tests (e.g., 1000)
    pub target_tests: usize,
    /// Target number of implemented lenses (e.g., 100)
    pub target_lenses_impl: usize,
    /// Target telescope scan throughput (ops/sec)
    pub target_scan_throughput: f64,
    /// Target health score (0.0..1.0)
    pub target_health: f64,
}

impl Default for GrowthTargets {
    fn default() -> Self {
        GrowthTargets {
            target_tests: 1000,                   // ~σ² × n ≈ 864, rounded to 1000
            target_lenses_impl: 100,              // ~σ² - τ² - n = 114, rounded
            target_scan_throughput: 1000.0,        // ops/sec
            target_health: 0.95,                   // 95% = 1 - 1/(J₂-τ) ≈ 0.95
        }
    }
}

/// Overall trend direction.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GrowthTrend {
    /// Health score improving on average
    Improving,
    /// Health score roughly stable (within ±0.01)
    Stagnant,
    /// Health score declining
    Declining,
}

/// A prioritized growth suggestion.
#[derive(Debug, Clone)]
pub struct GrowthPriority {
    /// What to work on
    pub area: String,
    /// How far from target (0.0 = at target, 1.0 = maximum distance)
    pub gap: f64,
    /// Estimated impact on health score (0.0..1.0)
    pub estimated_impact: f64,
    /// Human-readable description
    pub description: String,
}

/// Tracks growth entries over time and computes trends/priorities.
pub struct GrowthTracker {
    /// History of growth entries
    pub entries: Vec<GrowthEntry>,
    /// Target thresholds
    pub targets: GrowthTargets,
}

impl GrowthTracker {
    /// Create a new tracker with the given targets.
    pub fn new(targets: GrowthTargets) -> Self {
        GrowthTracker {
            entries: Vec::new(),
            targets,
        }
    }

    /// Append a growth entry to the history.
    pub fn add_entry(&mut self, entry: GrowthEntry) {
        self.entries.push(entry);
    }

    /// Compute the overall trend from the last σ-φ=10 entries (or all if fewer).
    pub fn trend(&self) -> GrowthTrend {
        let window = SIGMA_MINUS_PHI; // σ-φ = 10
        if self.entries.len() < PHI {
            return GrowthTrend::Stagnant;
        }

        let start = if self.entries.len() > window {
            self.entries.len() - window
        } else {
            0
        };
        let recent = &self.entries[start..];

        let deltas: Vec<f64> = recent
            .windows(PHI) // φ=2 window
            .map(|w| w[1].metrics.health_score - w[0].metrics.health_score)
            .collect();

        if deltas.is_empty() {
            return GrowthTrend::Stagnant;
        }

        let avg_delta: f64 = deltas.iter().sum::<f64>() / deltas.len() as f64;

        if avg_delta > 0.01 {
            GrowthTrend::Improving
        } else if avg_delta < -0.01 {
            GrowthTrend::Declining
        } else {
            GrowthTrend::Stagnant
        }
    }

    /// Compute distance to each target as a fraction (0.0 = reached, 1.0 = maximally far).
    pub fn distance_to_targets(&self) -> HashMap<String, f64> {
        let mut distances = HashMap::new();

        let latest = match self.entries.last() {
            Some(e) => e,
            None => {
                distances.insert("tests".to_string(), 1.0);
                distances.insert("lenses_impl".to_string(), 1.0);
                distances.insert("scan_throughput".to_string(), 1.0);
                distances.insert("health".to_string(), 1.0);
                return distances;
            }
        };

        // Tests distance
        let test_ratio = latest.metrics.total_tests as f64 / self.targets.target_tests.max(1) as f64;
        distances.insert("tests".to_string(), (1.0 - test_ratio).max(0.0));

        // Lens implementation distance
        let lens_ratio = latest.metrics.total_lenses_implemented as f64
            / self.targets.target_lenses_impl.max(1) as f64;
        distances.insert("lenses_impl".to_string(), (1.0 - lens_ratio).max(0.0));

        // Scan throughput distance
        let throughput = latest
            .benchmarks
            .results
            .iter()
            .find(|r| r.name.contains("telescope_scan"))
            .map(|r| r.throughput)
            .unwrap_or(0.0);
        let tp_ratio = throughput / self.targets.target_scan_throughput.max(1.0);
        distances.insert("scan_throughput".to_string(), (1.0 - tp_ratio).max(0.0));

        // Health distance
        let health_ratio = latest.metrics.health_score / self.targets.target_health.max(0.01);
        distances.insert("health".to_string(), (1.0 - health_ratio).max(0.0));

        distances
    }

    /// Suggest priorities for what to work on next, sorted by estimated impact.
    pub fn suggest_priorities(&self) -> Vec<GrowthPriority> {
        let distances = self.distance_to_targets();
        let mut priorities = Vec::new();

        // Lenses: high impact (40% of health score weight)
        if let Some(&gap) = distances.get("lenses_impl") {
            if gap > 0.0 {
                priorities.push(GrowthPriority {
                    area: "lenses_impl".to_string(),
                    gap,
                    estimated_impact: gap * 0.4, // 40% health weight
                    description: format!(
                        "Implement more lenses (gap: {:.0}%)",
                        gap * 100.0
                    ),
                });
            }
        }

        // Tests: moderate impact (30% of health score weight)
        if let Some(&gap) = distances.get("tests") {
            if gap > 0.0 {
                priorities.push(GrowthPriority {
                    area: "tests".to_string(),
                    gap,
                    estimated_impact: gap * 0.3,
                    description: format!(
                        "Add more tests (gap: {:.0}%)",
                        gap * 100.0
                    ),
                });
            }
        }

        // Scan throughput: performance impact
        if let Some(&gap) = distances.get("scan_throughput") {
            if gap > 0.0 {
                priorities.push(GrowthPriority {
                    area: "scan_throughput".to_string(),
                    gap,
                    estimated_impact: gap * 0.2,
                    description: format!(
                        "Optimize scan throughput (gap: {:.0}%)",
                        gap * 100.0
                    ),
                });
            }
        }

        // Health: overall
        if let Some(&gap) = distances.get("health") {
            if gap > 0.0 {
                priorities.push(GrowthPriority {
                    area: "health".to_string(),
                    gap,
                    estimated_impact: gap * 0.1,
                    description: format!(
                        "Improve overall health (gap: {:.0}%)",
                        gap * 100.0
                    ),
                });
            }
        }

        // Sort by estimated impact descending
        priorities.sort_by(|a, b| {
            b.estimated_impact
                .partial_cmp(&a.estimated_impact)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        priorities
    }

    /// Format an ASCII progress report with progress bars.
    pub fn format_report(&self) -> String {
        let mut s = String::new();
        s.push_str("┌──────────────────────────────────────────────────────────────┐\n");
        s.push_str("│              NEXUS-6 Growth Report                          │\n");
        s.push_str("├──────────────────────────────────────────────────────────────┤\n");

        // Cycles
        s.push_str(&format!(
            "│  Growth cycles: {:>4}                                        │\n",
            self.entries.len()
        ));

        // Trend
        let trend = self.trend();
        let trend_str = match trend {
            GrowthTrend::Improving => "IMPROVING ↑",
            GrowthTrend::Stagnant => "STAGNANT  →",
            GrowthTrend::Declining => "DECLINING ↓",
        };
        s.push_str(&format!(
            "│  Trend: {:<20}                                  │\n",
            trend_str
        ));

        // Distance to targets with progress bars
        let distances = self.distance_to_targets();
        s.push_str("├──────────────────────────────────────────────────────────────┤\n");
        s.push_str("│  Target Progress:                                          │\n");

        let bar_width = SIGMA * PHI; // σ·φ = 24 chars

        for (name, gap) in &distances {
            let progress = 1.0 - gap;
            let filled = (progress * bar_width as f64).round() as usize;
            let empty = bar_width.saturating_sub(filled);
            let bar = format!(
                "{}{}",
                "█".repeat(filled),
                "░".repeat(empty),
            );
            s.push_str(&format!(
                "│  {:<16} {} {:>5.1}%                │\n",
                name,
                bar,
                progress * 100.0,
            ));
        }

        // Priorities
        let priorities = self.suggest_priorities();
        if !priorities.is_empty() {
            s.push_str("├──────────────────────────────────────────────────────────────┤\n");
            s.push_str("│  Priorities:                                               │\n");
            for (i, p) in priorities.iter().enumerate() {
                s.push_str(&format!(
                    "│  {}. {} (impact: {:.2})                       │\n",
                    i + 1,
                    &p.description[..p.description.len().min(40)],
                    p.estimated_impact,
                ));
            }
        }

        s.push_str("└──────────────────────────────────────────────────────────────┘\n");
        s
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;
    use crate::growth::benchmark::BenchmarkResult;

    fn make_metrics(health: f64, tests: usize, lenses_impl: usize) -> NexusMetrics {
        NexusMetrics {
            total_modules: 42,
            total_tests: tests,
            total_lenses_registered: 693,
            total_lenses_implemented: lenses_impl,
            code_lines: 10000,
            compile_warnings: 0,
            test_pass_rate: 1.0,
            health_score: health,
            timestamp: "2026-04-03T00:00:00Z".to_string(),
        }
    }

    fn make_suite(scan_throughput: f64) -> BenchmarkSuite {
        BenchmarkSuite {
            results: vec![BenchmarkResult {
                name: "telescope_scan_n24".to_string(),
                iterations: 8,
                mean_ns: if scan_throughput > 0.0 {
                    (1_000_000_000.0 / scan_throughput) as u64
                } else {
                    1_000_000
                },
                min_ns: 100,
                max_ns: 2_000_000,
                throughput: scan_throughput,
            }],
            total_time_ms: 100,
            bottleneck: Some("telescope_scan_n24".to_string()),
        }
    }

    fn make_entry(cycle: usize, health: f64, tests: usize, lenses: usize) -> GrowthEntry {
        GrowthEntry {
            cycle,
            metrics: make_metrics(health, tests, lenses),
            benchmarks: make_suite(500.0),
            actions_taken: vec!["test action".to_string()],
            improvements: vec!["test improvement".to_string()],
        }
    }

    #[test]
    fn test_tracker_new() {
        let tracker = GrowthTracker::new(GrowthTargets::default());
        assert!(tracker.entries.is_empty());
        assert_eq!(tracker.targets.target_tests, 1000);
    }

    #[test]
    fn test_add_entry_and_trend() {
        let mut tracker = GrowthTracker::new(GrowthTargets::default());

        // Empty: stagnant
        assert_eq!(tracker.trend(), GrowthTrend::Stagnant);

        // Single: stagnant
        tracker.add_entry(make_entry(1, 0.5, 100, 20));
        assert_eq!(tracker.trend(), GrowthTrend::Stagnant);

        // Improving: health increases
        tracker.add_entry(make_entry(2, 0.6, 150, 25));
        tracker.add_entry(make_entry(3, 0.7, 200, 30));
        assert_eq!(tracker.trend(), GrowthTrend::Improving);
    }

    #[test]
    fn test_declining_trend() {
        let mut tracker = GrowthTracker::new(GrowthTargets::default());
        tracker.add_entry(make_entry(1, 0.9, 500, 80));
        tracker.add_entry(make_entry(2, 0.8, 490, 78));
        tracker.add_entry(make_entry(3, 0.7, 480, 75));
        assert_eq!(tracker.trend(), GrowthTrend::Declining);
    }

    #[test]
    fn test_distance_to_targets() {
        let mut tracker = GrowthTracker::new(GrowthTargets {
            target_tests: 1000,
            target_lenses_impl: 100,
            target_scan_throughput: 1000.0,
            target_health: 1.0,
        });
        tracker.add_entry(make_entry(1, 0.5, 500, 50));

        let dist = tracker.distance_to_targets();
        assert!((dist["tests"] - 0.5).abs() < 0.01, "tests should be 50% away");
        assert!((dist["lenses_impl"] - 0.5).abs() < 0.01, "lenses should be 50% away");
        assert!((dist["health"] - 0.5).abs() < 0.01, "health should be 50% away");
    }

    #[test]
    fn test_suggest_priorities() {
        let mut tracker = GrowthTracker::new(GrowthTargets::default());
        tracker.add_entry(make_entry(1, 0.5, 100, 10));

        let priorities = tracker.suggest_priorities();
        assert!(!priorities.is_empty(), "should have suggestions");
        // Should be sorted by estimated_impact descending
        for w in priorities.windows(2) {
            assert!(w[0].estimated_impact >= w[1].estimated_impact);
        }
    }

    #[test]
    fn test_format_report() {
        let mut tracker = GrowthTracker::new(GrowthTargets::default());
        tracker.add_entry(make_entry(1, 0.5, 200, 30));
        tracker.add_entry(make_entry(2, 0.6, 300, 40));

        let report = tracker.format_report();
        assert!(report.contains("NEXUS-6 Growth Report"));
        assert!(report.contains("Growth cycles"));
        assert!(report.contains("Trend"));
        assert!(report.contains("Target Progress"));
    }
}
