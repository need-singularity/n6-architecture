//! Multi-agent collaboration — merge and analyze results from multiple agents.

use std::collections::{HashMap, HashSet};

use crate::autonomous::agent::AgentReport;

/// Result of multi-agent collaboration.
#[derive(Debug, Clone)]
pub struct MultiAgentResult {
    /// Per-agent results.
    pub agent_results: Vec<(String, AgentReport)>,
    /// All discoveries merged (deduplicated by content).
    pub merged_discoveries: Vec<String>,
    /// Discoveries confirmed by 2+ agents.
    pub consensus_discoveries: Vec<String>,
    /// Conflicting findings between agents.
    pub conflicts: Vec<String>,
    /// New discoveries that emerged from combining agent results.
    pub synergies: Vec<String>,
    /// Total cycles across all agents.
    pub total_cycles: usize,
    /// Total time across all agents (ms).
    pub total_time_ms: u64,
}

/// Run collaboration analysis on a set of agent reports.
///
/// Steps:
/// 1. Merge all discoveries, deduplicating by normalized content.
/// 2. Find consensus: discoveries appearing in 2+ agents.
/// 3. Detect conflicts: discoveries from one agent that contradict another.
/// 4. Find synergies: cross-domain patterns emerging from combined results.
pub fn collaborate(reports: &[(String, AgentReport)]) -> MultiAgentResult {
    let mut all_discoveries: Vec<String> = Vec::new();
    let mut discovery_agents: HashMap<String, Vec<String>> = HashMap::new();
    let mut domain_sets: HashMap<String, HashSet<String>> = HashMap::new();

    // Collect all discoveries and track which agent found each
    for (agent_name, report) in reports {
        for discovery in &report.discoveries {
            let normalized = normalize_discovery(discovery);
            discovery_agents
                .entry(normalized.clone())
                .or_default()
                .push(agent_name.clone());

            if !all_discoveries.contains(&normalized) {
                all_discoveries.push(normalized);
            }
        }

        // Track domains per agent
        for domain in &report.domains_covered {
            domain_sets
                .entry(agent_name.clone())
                .or_default()
                .insert(domain.clone());
        }
    }

    // Consensus: found by 2+ agents
    let consensus_discoveries: Vec<String> = discovery_agents
        .iter()
        .filter(|(_, agents)| agents.len() >= 2)
        .map(|(disc, _)| disc.clone())
        .collect();

    // Conflicts: opposite conclusions in different agents
    // (simplified: look for "redteam" vs other agent disagreements)
    let conflicts = detect_conflicts(reports);

    // Synergies: cross-domain patterns
    let synergies = detect_synergies(reports, &domain_sets);

    let total_cycles: usize = reports.iter().map(|(_, r)| r.cycles_completed).sum();
    let total_time_ms: u64 = reports.iter().map(|(_, r)| r.time_elapsed_ms).sum();

    MultiAgentResult {
        agent_results: reports.to_vec(),
        merged_discoveries: all_discoveries,
        consensus_discoveries,
        conflicts,
        synergies,
        total_cycles,
        total_time_ms,
    }
}

/// Normalize a discovery string for deduplication.
fn normalize_discovery(discovery: &str) -> String {
    discovery.trim().to_lowercase()
}

/// Detect conflicts between agent results.
///
/// A conflict is when a RedTeam agent's findings overlap with an Explore/Deepen
/// agent's findings (the RedTeam found a counter-example to something others accepted).
fn detect_conflicts(reports: &[(String, AgentReport)]) -> Vec<String> {
    let mut conflicts = Vec::new();

    let redteam_discoveries: Vec<&str> = reports
        .iter()
        .filter(|(_, r)| r.mode == "redteam")
        .flat_map(|(_, r)| r.discoveries.iter().map(|d| d.as_str()))
        .collect();

    let other_discoveries: Vec<&str> = reports
        .iter()
        .filter(|(_, r)| r.mode != "redteam")
        .flat_map(|(_, r)| r.discoveries.iter().map(|d| d.as_str()))
        .collect();

    // Check for domain overlap between redteam and other findings
    for rt_disc in &redteam_discoveries {
        for other_disc in &other_discoveries {
            // Simple heuristic: if they share a domain keyword, it's a potential conflict
            let rt_words: HashSet<&str> = rt_disc.split_whitespace().collect();
            let other_words: HashSet<&str> = other_disc.split_whitespace().collect();
            let overlap: HashSet<&&str> = rt_words.intersection(&other_words).collect();

            if overlap.len() >= 3 {
                conflicts.push(format!(
                    "Conflict: redteam '{}' vs '{}'",
                    truncate(rt_disc, 60),
                    truncate(other_disc, 60),
                ));
            }
        }
    }

    conflicts
}

/// Detect synergies from cross-domain agent results.
///
/// If agents cover different domains and both find discoveries, the
/// combination may reveal cross-domain patterns.
fn detect_synergies(
    reports: &[(String, AgentReport)],
    domain_sets: &HashMap<String, HashSet<String>>,
) -> Vec<String> {
    let mut synergies = Vec::new();

    // Collect all unique domain pairs where both agents made discoveries
    let agents_with_discoveries: Vec<(&String, &HashSet<String>)> = domain_sets
        .iter()
        .filter(|(name, _)| {
            reports
                .iter()
                .any(|(n, r)| n == *name && !r.discoveries.is_empty())
        })
        .collect();

    for i in 0..agents_with_discoveries.len() {
        for j in (i + 1)..agents_with_discoveries.len() {
            let (a_name, a_domains) = agents_with_discoveries[i];
            let (b_name, b_domains) = agents_with_discoveries[j];

            // Cross-domain synergy: agents cover different domains
            let shared: HashSet<&String> = a_domains.intersection(b_domains).collect();
            let a_unique: HashSet<&String> = a_domains.difference(b_domains).collect();
            let b_unique: HashSet<&String> = b_domains.difference(a_domains).collect();

            if !a_unique.is_empty() && !b_unique.is_empty() {
                synergies.push(format!(
                    "Cross-domain synergy: {} ({:?}) x {} ({:?})",
                    a_name, a_unique, b_name, b_unique
                ));
            }

            if !shared.is_empty() {
                synergies.push(format!(
                    "Shared domain convergence: {} + {} on {:?}",
                    a_name, b_name, shared
                ));
            }
        }
    }

    synergies
}

fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut end = max_len;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report(mode: &str, discoveries: Vec<&str>, domains: Vec<&str>) -> AgentReport {
        AgentReport {
            agent_id: format!("agent-{}", mode),
            mode: mode.to_string(),
            cycles_completed: 6,
            discoveries: discoveries.into_iter().map(|s| s.to_string()).collect(),
            experiments_run: 0,
            lenses_forged: 0,
            time_elapsed_ms: 100,
            final_status: "Completed".to_string(),
            domains_covered: domains.into_iter().map(|s| s.to_string()).collect(),
        }
    }

    #[test]
    fn test_collaborate_empty() {
        let result = collaborate(&[]);
        assert!(result.merged_discoveries.is_empty());
        assert!(result.consensus_discoveries.is_empty());
        assert_eq!(result.total_cycles, 0);
    }

    #[test]
    fn test_collaborate_merge() {
        let reports = vec![
            ("explorer".to_string(), make_report("explore", vec!["discovery A", "discovery B"], vec!["physics"])),
            ("deepener".to_string(), make_report("deepen", vec!["discovery A", "discovery C"], vec!["physics"])),
        ];
        let result = collaborate(&reports);

        assert_eq!(result.merged_discoveries.len(), 3); // A, B, C
        assert_eq!(result.consensus_discoveries.len(), 1); // A (found by both)
        assert_eq!(result.total_cycles, 12);
    }

    #[test]
    fn test_collaborate_cross_domain_synergy() {
        let reports = vec![
            ("agent-a".to_string(), make_report("explore", vec!["found X"], vec!["physics"])),
            ("agent-b".to_string(), make_report("explore", vec!["found Y"], vec!["biology"])),
        ];
        let result = collaborate(&reports);

        assert!(!result.synergies.is_empty());
        assert!(result.synergies.iter().any(|s| s.contains("Cross-domain")));
    }

    #[test]
    fn test_collaborate_no_conflicts_without_redteam() {
        let reports = vec![
            ("a".to_string(), make_report("explore", vec!["disc 1"], vec!["physics"])),
            ("b".to_string(), make_report("deepen", vec!["disc 2"], vec!["physics"])),
        ];
        let result = collaborate(&reports);
        assert!(result.conflicts.is_empty());
    }

    #[test]
    fn test_normalize_discovery() {
        assert_eq!(normalize_discovery("  Hello World  "), "hello world");
    }
}
