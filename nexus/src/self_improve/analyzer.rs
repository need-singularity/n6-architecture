//! Performance analyzer — identify bottlenecks and improvement opportunities.

use crate::autonomous::agent::AgentReport;
use crate::telescope::registry::LensRegistry;

use super::suggestion::ImprovementSuggestion;

/// Full performance analysis result.
#[derive(Debug, Clone)]
pub struct PerformanceAnalysis {
    /// Identified bottlenecks.
    pub bottlenecks: Vec<String>,
    /// Lenses that were never used in any scan.
    pub unused_lenses: Vec<String>,
    /// Lenses with low hit rates (name, hit_rate).
    pub low_hit_lenses: Vec<(String, f64)>,
    /// Slow operations (name, time_ms).
    pub slow_operations: Vec<(String, u64)>,
    /// Generated improvement suggestions.
    pub suggestions: Vec<ImprovementSuggestion>,
    /// Overall health score (0.0..1.0).
    pub health_score: f64,
}

/// Analyze performance based on agent reports and registry state.
pub fn analyze_performance(reports: &[AgentReport]) -> PerformanceAnalysis {
    let registry = LensRegistry::new();
    let _all_lens_names: Vec<String> = registry.iter().map(|(name, _)| name.clone()).collect();

    // Track which lenses were used across all reports
    let mut used_lenses: Vec<String> = Vec::new();
    for report in reports {
        // Parse lens names from discovery descriptions if available
        for domain in &report.domains_covered {
            if !used_lenses.contains(domain) {
                used_lenses.push(domain.clone());
            }
        }
    }

    // Find unused lenses (from the core 22 — check a subset for analysis)
    let core_lenses = vec![
        "consciousness", "gravity", "topology", "thermo", "wave",
        "evolution", "info", "quantum", "em", "ruler", "triangle",
        "compass", "mirror", "scale", "causal", "quantum_microscope",
        "stability", "network", "memory", "recursion", "boundary", "multiscale",
    ];
    let unused_lenses: Vec<String> = core_lenses
        .iter()
        .filter(|l| !used_lenses.iter().any(|u| u.contains(*l)))
        .map(|l| l.to_string())
        .collect();

    // Identify bottlenecks
    let mut bottlenecks: Vec<String> = Vec::new();
    let mut slow_operations: Vec<(String, u64)> = Vec::new();

    for report in reports {
        if report.cycles_completed > 0 {
            let avg_time = report.time_elapsed_ms / report.cycles_completed as u64;
            if avg_time > 1000 {
                bottlenecks.push(format!(
                    "Agent '{}' slow: {}ms/cycle (threshold: 1000ms)",
                    report.agent_id, avg_time
                ));
                slow_operations.push((report.agent_id.clone(), avg_time));
            }
        }

        if report.discoveries.is_empty() && report.cycles_completed > 3 {
            bottlenecks.push(format!(
                "Agent '{}' unproductive: 0 discoveries in {} cycles",
                report.agent_id, report.cycles_completed
            ));
        }
    }

    // Low hit lenses (simulated from report data)
    let low_hit_lenses: Vec<(String, f64)> = Vec::new(); // Would be populated from scan history

    // Generate suggestions
    let mut suggestions: Vec<ImprovementSuggestion> = Vec::new();

    if !unused_lenses.is_empty() {
        suggestions.push(ImprovementSuggestion {
            category: "lens".to_string(),
            description: format!(
                "Activate {} unused lenses: {}",
                unused_lenses.len(),
                unused_lenses.iter().take(6).cloned().collect::<Vec<_>>().join(", ")
            ),
            expected_improvement: format!(
                "{}% more coverage ({} new lenses)",
                (unused_lenses.len() * 100) / core_lenses.len().max(1),
                unused_lenses.len()
            ),
            difficulty: "easy".to_string(),
        });
    }

    if !bottlenecks.is_empty() {
        suggestions.push(ImprovementSuggestion {
            category: "engine".to_string(),
            description: format!("{} bottleneck(s) detected — consider reducing max_cycles or adjusting serendipity", bottlenecks.len()),
            expected_improvement: "30-50% cycle time reduction".to_string(),
            difficulty: "medium".to_string(),
        });
    }

    let total_discoveries: usize = reports.iter().map(|r| r.discoveries.len()).sum();
    let total_cycles: usize = reports.iter().map(|r| r.cycles_completed).sum();

    if total_cycles > 0 && (total_discoveries as f64 / total_cycles as f64) < 0.5 {
        suggestions.push(ImprovementSuggestion {
            category: "algorithm".to_string(),
            description: "Low discovery rate — try increasing serendipity or switching to Explore mode".to_string(),
            expected_improvement: "2-3x discovery rate improvement".to_string(),
            difficulty: "easy".to_string(),
        });
    }

    // Compute health score
    let discovery_score = if total_cycles > 0 {
        (total_discoveries as f64 / total_cycles as f64).min(1.0)
    } else {
        0.0
    };
    let bottleneck_penalty = (bottlenecks.len() as f64 * 0.1).min(0.5);
    let unused_penalty = (unused_lenses.len() as f64 * 0.02).min(0.3);
    let health_score = (discovery_score - bottleneck_penalty - unused_penalty).max(0.0).min(1.0);

    PerformanceAnalysis {
        bottlenecks,
        unused_lenses,
        low_hit_lenses,
        slow_operations,
        suggestions,
        health_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report(id: &str, cycles: usize, discoveries: usize, time_ms: u64) -> AgentReport {
        AgentReport {
            agent_id: id.to_string(),
            mode: "explore".to_string(),
            cycles_completed: cycles,
            discoveries: (0..discoveries).map(|i| format!("disc-{}", i)).collect(),
            experiments_run: 0,
            lenses_forged: 0,
            time_elapsed_ms: time_ms,
            final_status: "Completed".to_string(),
            domains_covered: vec!["physics".to_string()],
        }
    }

    #[test]
    fn test_analyze_empty() {
        let analysis = analyze_performance(&[]);
        assert!(analysis.bottlenecks.is_empty());
        assert_eq!(analysis.health_score, 0.0);
    }

    #[test]
    fn test_analyze_healthy() {
        let reports = vec![make_report("agent-1", 6, 6, 300)];
        let analysis = analyze_performance(&reports);
        assert!(analysis.bottlenecks.is_empty());
        assert!(analysis.health_score > 0.0);
    }

    #[test]
    fn test_analyze_slow_agent() {
        let reports = vec![make_report("slow-agent", 3, 3, 30_000)];
        let analysis = analyze_performance(&reports);
        assert!(!analysis.bottlenecks.is_empty());
        assert!(!analysis.slow_operations.is_empty());
    }

    #[test]
    fn test_analyze_unproductive_agent() {
        let reports = vec![make_report("lazy-agent", 6, 0, 600)];
        let analysis = analyze_performance(&reports);
        assert!(!analysis.bottlenecks.is_empty());
    }

    #[test]
    fn test_suggestions_generated() {
        let reports = vec![make_report("agent-1", 6, 1, 600)];
        let analysis = analyze_performance(&reports);
        // Should have suggestions about unused lenses at minimum
        assert!(!analysis.suggestions.is_empty());
    }
}
