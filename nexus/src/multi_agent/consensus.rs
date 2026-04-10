//! Consensus engine — determine agreement level across agent findings.

use std::collections::HashMap;

use crate::autonomous::agent::AgentReport;

/// Consensus level for a discovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusLevel {
    /// Single agent found it.
    Single,
    /// 2 agents agree.
    Pair,
    /// 3+ agents agree (high confidence).
    Strong,
    /// All agents agree (unanimous).
    Unanimous,
}

/// A discovery with its consensus information.
#[derive(Debug, Clone)]
pub struct ConsensusEntry {
    pub discovery: String,
    pub level: ConsensusLevel,
    pub agent_count: usize,
    pub total_agents: usize,
    pub agents: Vec<String>,
}

/// Compute consensus across agent reports.
///
/// Returns discoveries sorted by consensus level (strongest first).
pub fn compute_consensus(reports: &[(String, AgentReport)]) -> Vec<ConsensusEntry> {
    let total_agents = reports.len();
    let mut discovery_agents: HashMap<String, Vec<String>> = HashMap::new();

    for (agent_name, report) in reports {
        for discovery in &report.discoveries {
            let normalized = discovery.trim().to_lowercase();
            discovery_agents
                .entry(normalized)
                .or_default()
                .push(agent_name.clone());
        }
    }

    let mut entries: Vec<ConsensusEntry> = discovery_agents
        .into_iter()
        .map(|(discovery, agents)| {
            let agent_count = agents.len();
            let level = if agent_count >= total_agents && total_agents > 1 {
                ConsensusLevel::Unanimous
            } else if agent_count >= 3 {
                ConsensusLevel::Strong
            } else if agent_count >= 2 {
                ConsensusLevel::Pair
            } else {
                ConsensusLevel::Single
            };

            ConsensusEntry {
                discovery,
                level,
                agent_count,
                total_agents,
                agents,
            }
        })
        .collect();

    // Sort: Unanimous > Strong > Pair > Single, then by agent count
    entries.sort_by(|a, b| {
        let level_ord = |l: &ConsensusLevel| -> usize {
            match l {
                ConsensusLevel::Unanimous => 0,
                ConsensusLevel::Strong => 1,
                ConsensusLevel::Pair => 2,
                ConsensusLevel::Single => 3,
            }
        };
        level_ord(&a.level)
            .cmp(&level_ord(&b.level))
            .then(b.agent_count.cmp(&a.agent_count))
    });

    entries
}

/// Filter consensus entries to only those at or above the given level.
pub fn filter_by_level(entries: &[ConsensusEntry], min_level: ConsensusLevel) -> Vec<&ConsensusEntry> {
    let min_ord = match min_level {
        ConsensusLevel::Single => 1,
        ConsensusLevel::Pair => 2,
        ConsensusLevel::Strong => 3,
        ConsensusLevel::Unanimous => usize::MAX,
    };

    entries.iter().filter(|e| e.agent_count >= min_ord).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report(discoveries: Vec<&str>) -> AgentReport {
        AgentReport {
            agent_id: "test".to_string(),
            mode: "explore".to_string(),
            cycles_completed: 1,
            discoveries: discoveries.into_iter().map(|s| s.to_string()).collect(),
            experiments_run: 0,
            lenses_forged: 0,
            time_elapsed_ms: 10,
            final_status: "Completed".to_string(),
            domains_covered: vec!["test".to_string()],
        }
    }

    #[test]
    fn test_consensus_empty() {
        let entries = compute_consensus(&[]);
        assert!(entries.is_empty());
    }

    #[test]
    fn test_consensus_single_agent() {
        let reports = vec![
            ("a".to_string(), make_report(vec!["disc 1", "disc 2"])),
        ];
        let entries = compute_consensus(&reports);
        assert_eq!(entries.len(), 2);
        assert!(entries.iter().all(|e| e.level == ConsensusLevel::Single));
    }

    #[test]
    fn test_consensus_pair() {
        let reports = vec![
            ("a".to_string(), make_report(vec!["shared discovery"])),
            ("b".to_string(), make_report(vec!["shared discovery", "unique"])),
        ];
        let entries = compute_consensus(&reports);

        let shared = entries.iter().find(|e| e.discovery == "shared discovery").unwrap();
        assert_eq!(shared.level, ConsensusLevel::Unanimous); // 2/2 = unanimous
        assert_eq!(shared.agent_count, 2);
    }

    #[test]
    fn test_consensus_strong() {
        let reports = vec![
            ("a".to_string(), make_report(vec!["common"])),
            ("b".to_string(), make_report(vec!["common"])),
            ("c".to_string(), make_report(vec!["common"])),
            ("d".to_string(), make_report(vec!["unique"])),
        ];
        let entries = compute_consensus(&reports);

        let common = entries.iter().find(|e| e.discovery == "common").unwrap();
        assert_eq!(common.level, ConsensusLevel::Strong);
        assert_eq!(common.agent_count, 3);
    }

    #[test]
    fn test_filter_by_level() {
        let reports = vec![
            ("a".to_string(), make_report(vec!["shared", "only-a"])),
            ("b".to_string(), make_report(vec!["shared"])),
        ];
        let entries = compute_consensus(&reports);
        let filtered = filter_by_level(&entries, ConsensusLevel::Pair);
        assert_eq!(filtered.len(), 1);
        assert_eq!(filtered[0].discovery, "shared");
    }
}
