//! Conflict resolution — detect and resolve contradictions between agents.

use std::collections::HashSet;

use crate::autonomous::agent::AgentReport;

/// A detected conflict between two agent findings.
#[derive(Debug, Clone)]
pub struct Conflict {
    pub agent_a: String,
    pub agent_b: String,
    pub discovery_a: String,
    pub discovery_b: String,
    pub overlap_words: Vec<String>,
    pub resolution: ConflictResolution,
}

/// How a conflict was resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConflictResolution {
    /// Not yet resolved.
    Unresolved,
    /// Agent A's finding is accepted.
    AcceptA,
    /// Agent B's finding is accepted.
    AcceptB,
    /// Both findings are compatible (false conflict).
    Compatible,
}

/// Detect conflicts between agent reports.
///
/// Two findings are considered conflicting if:
/// 1. They come from agents in different modes (especially redteam vs others).
/// 2. They share significant word overlap (same domain/topic).
pub fn detect_conflicts(reports: &[(String, AgentReport)]) -> Vec<Conflict> {
    let mut conflicts = Vec::new();

    for i in 0..reports.len() {
        for j in (i + 1)..reports.len() {
            let (name_a, report_a) = &reports[i];
            let (name_b, report_b) = &reports[j];

            // Only check for conflicts between different modes
            if report_a.mode == report_b.mode {
                continue;
            }

            for disc_a in &report_a.discoveries {
                for disc_b in &report_b.discoveries {
                    let overlap = find_overlap(disc_a, disc_b);
                    if overlap.len() >= 3 {
                        conflicts.push(Conflict {
                            agent_a: name_a.clone(),
                            agent_b: name_b.clone(),
                            discovery_a: disc_a.clone(),
                            discovery_b: disc_b.clone(),
                            overlap_words: overlap,
                            resolution: ConflictResolution::Unresolved,
                        });
                    }
                }
            }
        }
    }

    conflicts
}

/// Auto-resolve conflicts using heuristics.
///
/// Resolution rules:
/// - If one agent has more cycles (more evidence), prefer it.
/// - If one agent is in verify/redteam mode, weight its findings more heavily.
/// - Otherwise, mark as unresolved.
pub fn auto_resolve(
    conflicts: &mut [Conflict],
    reports: &[(String, AgentReport)],
) {
    for conflict in conflicts.iter_mut() {
        let report_a = reports.iter().find(|(n, _)| *n == conflict.agent_a);
        let report_b = reports.iter().find(|(n, _)| *n == conflict.agent_b);

        if let (Some((_, ra)), Some((_, rb))) = (report_a, report_b) {
            // Verify/RedTeam mode gets priority
            let a_priority = mode_priority(&ra.mode);
            let b_priority = mode_priority(&rb.mode);

            if a_priority > b_priority {
                conflict.resolution = ConflictResolution::AcceptA;
            } else if b_priority > a_priority {
                conflict.resolution = ConflictResolution::AcceptB;
            } else if ra.cycles_completed > rb.cycles_completed * 2 {
                conflict.resolution = ConflictResolution::AcceptA;
            } else if rb.cycles_completed > ra.cycles_completed * 2 {
                conflict.resolution = ConflictResolution::AcceptB;
            }
            // else remains Unresolved
        }
    }
}

/// Priority score for mode-based conflict resolution.
fn mode_priority(mode: &str) -> usize {
    match mode {
        "redteam" => 3,
        "verify" => 2,
        "deepen" => 1,
        _ => 0,
    }
}

/// Find overlapping words between two discovery strings.
fn find_overlap(a: &str, b: &str) -> Vec<String> {
    let words_a: HashSet<String> = a
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() > 2)
        .collect();
    let words_b: HashSet<String> = b
        .split_whitespace()
        .map(|w| w.to_lowercase())
        .filter(|w| w.len() > 2)
        .collect();

    words_a.intersection(&words_b).cloned().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report(mode: &str, discoveries: Vec<&str>, cycles: usize) -> AgentReport {
        AgentReport {
            agent_id: format!("agent-{}", mode),
            mode: mode.to_string(),
            cycles_completed: cycles,
            discoveries: discoveries.into_iter().map(|s| s.to_string()).collect(),
            experiments_run: 0,
            lenses_forged: 0,
            time_elapsed_ms: 100,
            final_status: "Completed".to_string(),
            domains_covered: vec!["test".to_string()],
        }
    }

    #[test]
    fn test_no_conflicts_same_mode() {
        let reports = vec![
            ("a".to_string(), make_report("explore", vec!["found pattern alpha beta gamma"], 6)),
            ("b".to_string(), make_report("explore", vec!["found pattern alpha beta gamma"], 6)),
        ];
        let conflicts = detect_conflicts(&reports);
        assert!(conflicts.is_empty()); // Same mode = no conflict
    }

    #[test]
    fn test_conflict_detected() {
        let reports = vec![
            ("explorer".to_string(), make_report("explore", vec!["n=6 pattern found in physics domain verified"], 6)),
            ("redteam".to_string(), make_report("redteam", vec!["n=6 pattern found in physics domain disproved"], 6)),
        ];
        let conflicts = detect_conflicts(&reports);
        assert!(!conflicts.is_empty());
    }

    #[test]
    fn test_auto_resolve_redteam_priority() {
        let reports = vec![
            ("explorer".to_string(), make_report("explore", vec!["the quick brown fox jumps high"], 6)),
            ("redteam".to_string(), make_report("redteam", vec!["the quick brown fox does not jump"], 6)),
        ];
        let mut conflicts = detect_conflicts(&reports);
        auto_resolve(&mut conflicts, &reports);

        if !conflicts.is_empty() {
            assert_eq!(conflicts[0].resolution, ConflictResolution::AcceptB); // redteam wins
        }
    }

    #[test]
    fn test_find_overlap() {
        let overlap = find_overlap("alpha beta gamma delta", "gamma delta epsilon zeta");
        assert!(overlap.contains(&"gamma".to_string()));
        assert!(overlap.contains(&"delta".to_string()));
    }

    #[test]
    fn test_mode_priority() {
        assert!(mode_priority("redteam") > mode_priority("verify"));
        assert!(mode_priority("verify") > mode_priority("deepen"));
        assert!(mode_priority("deepen") > mode_priority("explore"));
    }
}
