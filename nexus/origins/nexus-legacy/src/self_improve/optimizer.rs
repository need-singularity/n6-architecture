//! Optimizer — apply improvement suggestions to agent configurations.

use crate::autonomous::agent::{AgentConfig, AgentMode};
use super::analyzer::PerformanceAnalysis;

/// Optimization result with adjusted configurations.
#[derive(Debug, Clone)]
pub struct OptimizationResult {
    /// Original config.
    pub original: AgentConfig,
    /// Optimized config.
    pub optimized: AgentConfig,
    /// Changes applied.
    pub changes: Vec<String>,
}

/// Optimize an agent configuration based on performance analysis.
///
/// Adjusts:
/// - max_cycles based on saturation patterns
/// - serendipity based on discovery rate
/// - mode based on bottleneck analysis
pub fn optimize(config: &AgentConfig, analysis: &PerformanceAnalysis) -> OptimizationResult {
    let mut optimized = config.clone();
    let mut changes: Vec<String> = Vec::new();

    // If health score is very low, switch to explore mode
    if analysis.health_score < 0.2 && config.mode != AgentMode::Explore {
        optimized.mode = AgentMode::Explore;
        changes.push(format!(
            "Mode: {:?} -> Explore (health {:.2} too low)",
            config.mode, analysis.health_score
        ));
    }

    // If there are slow operations, reduce max_cycles
    if !analysis.slow_operations.is_empty() && config.max_cycles > 3 {
        optimized.max_cycles = config.max_cycles / 2;
        changes.push(format!(
            "max_cycles: {} -> {} (slow operations detected)",
            config.max_cycles, optimized.max_cycles
        ));
    }

    // If discovery rate is low, increase serendipity
    let has_low_rate = analysis.suggestions.iter().any(|s| s.category == "algorithm");
    if has_low_rate && config.serendipity < 0.5 {
        optimized.serendipity = (config.serendipity + 0.15).min(0.6);
        changes.push(format!(
            "serendipity: {:.2} -> {:.2} (low discovery rate)",
            config.serendipity, optimized.serendipity
        ));
    }

    // If there are many unused lenses, add broader domains
    if analysis.unused_lenses.len() > 10 && config.domains.len() < 3 {
        let new_domains = vec![
            "physics".to_string(),
            "biology".to_string(),
            "computing".to_string(),
        ];
        for d in &new_domains {
            if !optimized.domains.contains(d) {
                optimized.domains.push(d.clone());
                changes.push(format!("Added domain: {}", d));
            }
        }
    }

    if changes.is_empty() {
        changes.push("No changes needed — configuration is optimal".to_string());
    }

    OptimizationResult {
        original: config.clone(),
        optimized,
        changes,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::self_improve::suggestion::ImprovementSuggestion;

    fn make_analysis(health: f64, bottlenecks: usize, slow: bool, low_rate: bool) -> PerformanceAnalysis {
        let mut suggestions = Vec::new();
        if low_rate {
            suggestions.push(ImprovementSuggestion {
                category: "algorithm".to_string(),
                description: "Low rate".to_string(),
                expected_improvement: "2x".to_string(),
                difficulty: "easy".to_string(),
            });
        }

        PerformanceAnalysis {
            bottlenecks: (0..bottlenecks).map(|i| format!("bottleneck-{}", i)).collect(),
            unused_lenses: Vec::new(),
            low_hit_lenses: Vec::new(),
            slow_operations: if slow {
                vec![("slow-op".to_string(), 5000)]
            } else {
                Vec::new()
            },
            suggestions,
            health_score: health,
        }
    }

    #[test]
    fn test_optimize_healthy() {
        let config = AgentConfig::default();
        let analysis = make_analysis(0.8, 0, false, false);
        let result = optimize(&config, &analysis);
        assert!(result.changes.iter().any(|c| c.contains("optimal")));
    }

    #[test]
    fn test_optimize_low_health() {
        let config = AgentConfig {
            mode: AgentMode::Deepen,
            ..AgentConfig::default()
        };
        let analysis = make_analysis(0.1, 2, false, false);
        let result = optimize(&config, &analysis);
        assert_eq!(result.optimized.mode, AgentMode::Explore);
    }

    #[test]
    fn test_optimize_slow() {
        let config = AgentConfig {
            max_cycles: 12,
            ..AgentConfig::default()
        };
        let analysis = make_analysis(0.5, 0, true, false);
        let result = optimize(&config, &analysis);
        assert_eq!(result.optimized.max_cycles, 6);
    }

    #[test]
    fn test_optimize_low_rate() {
        let config = AgentConfig {
            serendipity: 0.1,
            ..AgentConfig::default()
        };
        let analysis = make_analysis(0.5, 0, false, true);
        let result = optimize(&config, &analysis);
        assert!(result.optimized.serendipity > config.serendipity);
    }

    #[test]
    fn test_optimize_preserves_original() {
        let config = AgentConfig::default();
        let analysis = make_analysis(0.1, 1, true, true);
        let result = optimize(&config, &analysis);
        // Original should be unchanged
        assert_eq!(result.original.max_cycles, config.max_cycles);
        assert!((result.original.serendipity - config.serendipity).abs() < f64::EPSILON);
    }
}
