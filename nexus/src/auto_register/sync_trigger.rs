use serde::{Deserialize, Serialize};

use super::registrar::RegistrationResult;

/// Result of determining which sync commands are needed.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncResult {
    pub commands: Vec<SyncCommand>,
    pub summary: String,
}

/// A sync command to execute (returned as data, not executed).
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SyncCommand {
    pub target: String,
    pub command: String,
    pub description: String,
}

/// Determine which sync commands should be run based on registration results.
///
/// This function does NOT execute commands — it returns a list of commands
/// that the caller (CLI or automation) should run.
pub fn trigger_sync(registrations: &[RegistrationResult]) -> SyncResult {
    let mut commands: Vec<SyncCommand> = Vec::new();
    let mut seen_targets = std::collections::HashSet::new();

    for reg in registrations {
        if !reg.success {
            continue;
        }
        for target in &reg.registered_to {
            if seen_targets.contains(target.as_str()) {
                continue;
            }
            seen_targets.insert(target.as_str());

            match target.as_str() {
                "math_atlas" => {
                    commands.push(SyncCommand {
                        target: "math_atlas".to_string(),
                        command: "python3 .shared/scan_math_atlas.py --save --summary".to_string(),
                        description: "Sync math atlas with new constants/formulas".to_string(),
                    });
                }
                "bt_list" | "bt_candidates" => {
                    if !seen_targets.contains("bt_sync") {
                        seen_targets.insert("bt_sync");
                        commands.push(SyncCommand {
                            target: "bt_list".to_string(),
                            command: "# Update docs/breakthrough-theorems.md with new BT candidate"
                                .to_string(),
                            description: "Register new breakthrough theorem candidate".to_string(),
                        });
                    }
                }
                "lens_forge" => {
                    commands.push(SyncCommand {
                        target: "lens_forge".to_string(),
                        command: "# Run lens forge validation pipeline".to_string(),
                        description: "Validate and register new lens candidate".to_string(),
                    });
                }
                "graph" => {
                    if !seen_targets.contains("graph_sync") {
                        seen_targets.insert("graph_sync");
                        commands.push(SyncCommand {
                            target: "graph".to_string(),
                            command: "# Save updated discovery graph".to_string(),
                            description: "Persist discovery graph changes".to_string(),
                        });
                    }
                }
                "hypotheses" => {
                    commands.push(SyncCommand {
                        target: "hypotheses".to_string(),
                        command: "# Update docs/<domain>/hypotheses.md".to_string(),
                        description: "Register new hypothesis".to_string(),
                    });
                }
                _ => {}
            }
        }
    }

    let summary = format!(
        "{} sync commands from {} registrations ({} targets)",
        commands.len(),
        registrations.len(),
        seen_targets.len()
    );

    SyncResult { commands, summary }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sync_math_atlas() {
        let regs = vec![RegistrationResult {
            discovery_id: "disc-abc".to_string(),
            registered_to: vec!["math_atlas".to_string()],
            success: true,
            details: "test".to_string(),
        }];
        let result = trigger_sync(&regs);
        assert!(!result.commands.is_empty());
        assert!(result.commands[0].command.contains("scan_math_atlas"));
    }

    #[test]
    fn test_sync_deduplicates() {
        let regs = vec![
            RegistrationResult {
                discovery_id: "disc-1".to_string(),
                registered_to: vec!["graph".to_string()],
                success: true,
                details: "a".to_string(),
            },
            RegistrationResult {
                discovery_id: "disc-2".to_string(),
                registered_to: vec!["graph".to_string()],
                success: true,
                details: "b".to_string(),
            },
        ];
        let result = trigger_sync(&regs);
        // graph command should appear only once
        let graph_cmds: Vec<_> = result.commands.iter().filter(|c| c.target == "graph").collect();
        assert_eq!(graph_cmds.len(), 1);
    }

    #[test]
    fn test_sync_skips_failed() {
        let regs = vec![RegistrationResult {
            discovery_id: "disc-fail".to_string(),
            registered_to: vec!["math_atlas".to_string()],
            success: false,
            details: "failed".to_string(),
        }];
        let result = trigger_sync(&regs);
        assert!(result.commands.is_empty());
    }
}
