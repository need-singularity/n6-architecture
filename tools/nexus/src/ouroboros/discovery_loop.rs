//! Discovery Loop — OUROBOROS → Claude Code CLI → Code Generation → Feedback
//!
//! When a scan cycle produces discoveries with high confidence,
//! this module automatically invokes Claude Code CLI to:
//!   1. Analyze the discovery deeper
//!   2. Generate code (new lens, experiment, calculator, hypothesis)
//!   3. Verify the generated code (cargo check/test)
//!   4. Record findings to docs/
//!   5. Feed results back into the next OUROBOROS cycle
//!
//! n=6 constants in loop dynamics:
//!   - trigger threshold: min_confidence = 1/(sigma-phi) = 0.1 (broad)
//!   - high confidence:   >= 1 - 1/e = 0.632
//!   - max concurrent:    n = 6 CLI calls per cycle
//!   - cooldown:          tau = 4 cycles after failure
//!   - retry limit:       phi = 2 attempts per discovery

use std::collections::HashMap;
use std::path::PathBuf;
use std::process::Command;

use serde::{Deserialize, Serialize};

// n=6 constants
const N: usize = 6;
const SIGMA: f64 = 12.0;
const PHI: usize = 2;
const TAU: usize = 4;
const HIGH_CONFIDENCE: f64 = 0.632; // 1 - 1/e

/// What kind of code to generate from a discovery.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DiscoveryAction {
    /// Generate a new telescope lens
    NewLens { pattern_description: String },
    /// Generate a new experiment to verify the discovery
    NewExperiment { hypothesis: String },
    /// Generate a new Rust calculator
    NewCalculator { computation: String },
    /// Record a breakthrough theorem candidate
    NewBreakthrough { bt_summary: String },
    /// Deepen analysis of an existing discovery
    DeepenAnalysis { node_id: String, question: String },
    /// Generate cross-domain resonance analysis
    CrossDomain { domains: Vec<String>, pattern: String },
}

/// Result of a Claude CLI invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CliResult {
    pub action: DiscoveryAction,
    pub success: bool,
    pub files_modified: Vec<String>,
    pub summary: String,
    pub exit_code: i32,
}

/// Configuration for the discovery loop.
#[derive(Debug, Clone)]
pub struct DiscoveryLoopConfig {
    /// Path to Claude Code CLI binary.
    pub claude_cli_path: String,
    /// Project root directory.
    pub project_root: String,
    /// NEXUS-6 root directory.
    pub nexus_root: String,
    /// Minimum confidence to trigger CLI call.
    pub min_confidence: f64,
    /// Maximum CLI calls per cycle.
    pub max_calls_per_cycle: usize,
    /// Allowed tools for Claude CLI.
    pub allowed_tools: String,
    /// Whether CLI calls are enabled (can be disabled for testing).
    pub enabled: bool,
    /// Cooldown tracker: domain -> cycles remaining.
    pub cooldowns: HashMap<String, usize>,
}

impl Default for DiscoveryLoopConfig {
    fn default() -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/tmp".to_string());
        Self {
            claude_cli_path: format!("{}/.local/bin/claude", home),
            project_root: format!("{}/Dev/n6-architecture", home),
            nexus_root: format!("{}/Dev/n6-architecture/tools/nexus", home),
            min_confidence: 0.1, // 1/(sigma-phi) = broad trigger
            max_calls_per_cycle: N,
            allowed_tools: "Edit,Write,Read,Bash,Grep,Glob".to_string(),
            enabled: true,
            cooldowns: HashMap::new(),
        }
    }
}

/// The Discovery Loop engine.
pub struct DiscoveryLoop {
    pub config: DiscoveryLoopConfig,
    /// History of CLI invocations.
    pub cli_history: Vec<CliResult>,
    /// Retry counts per discovery node.
    retries: HashMap<String, usize>,
}

impl DiscoveryLoop {
    pub fn new() -> Self {
        Self {
            config: DiscoveryLoopConfig::default(),
            cli_history: Vec::new(),
            retries: HashMap::new(),
        }
    }

    pub fn with_config(config: DiscoveryLoopConfig) -> Self {
        Self {
            config,
            cli_history: Vec::new(),
            retries: HashMap::new(),
        }
    }

    /// Determine what actions to take for discoveries from a cycle.
    ///
    /// Takes the list of discovered hypotheses with their confidence scores
    /// and graph context, returns a prioritized list of actions.
    pub fn plan_actions(
        &self,
        discoveries: &[(String, f64)], // (summary, confidence)
        domain: &str,
        cycle: usize,
    ) -> Vec<(DiscoveryAction, f64)> {
        let mut actions: Vec<(DiscoveryAction, f64)> = Vec::new();

        // Check cooldown
        if let Some(&remaining) = self.config.cooldowns.get(domain) {
            if remaining > 0 {
                return actions; // domain is in cooldown
            }
        }

        for (summary, confidence) in discoveries {
            if *confidence < self.config.min_confidence {
                continue;
            }

            // High confidence: generate experiment + breakthrough candidate
            if *confidence >= HIGH_CONFIDENCE {
                actions.push((
                    DiscoveryAction::NewExperiment {
                        hypothesis: summary.clone(),
                    },
                    *confidence,
                ));
                actions.push((
                    DiscoveryAction::NewBreakthrough {
                        bt_summary: summary.clone(),
                    },
                    *confidence,
                ));
            }

            // Medium confidence: deepen analysis
            if *confidence >= 0.3 && *confidence < HIGH_CONFIDENCE {
                actions.push((
                    DiscoveryAction::DeepenAnalysis {
                        node_id: format!("ouroboros-c{}", cycle),
                        question: format!(
                            "Analyze this discovery deeper and find n=6 connections: {}",
                            summary
                        ),
                    },
                    *confidence,
                ));
            }

            // Any confidence with "pattern" or "lens" keywords: suggest new lens
            let lower = summary.to_lowercase();
            if lower.contains("pattern") || lower.contains("lens") || lower.contains("metric") {
                actions.push((
                    DiscoveryAction::NewLens {
                        pattern_description: summary.clone(),
                    },
                    *confidence,
                ));
            }

            // Cross-domain signals
            if lower.contains("cross") || lower.contains("bridge") || lower.contains("resonance") {
                actions.push((
                    DiscoveryAction::CrossDomain {
                        domains: vec![domain.to_string(), "general".to_string()],
                        pattern: summary.clone(),
                    },
                    *confidence,
                ));
            }
        }

        // Sort by confidence descending, limit to max_calls_per_cycle
        actions.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        actions.truncate(self.config.max_calls_per_cycle);

        actions
    }

    /// Build a Claude CLI prompt for a given action.
    pub fn build_prompt(&self, action: &DiscoveryAction) -> String {
        let root = &self.config.project_root;
        let nexus = &self.config.nexus_root;

        match action {
            DiscoveryAction::NewLens { pattern_description } => {
                format!(
                    "In {nexus}/, implement a new telescope lens based on this discovered pattern: \
                     \"{pattern_description}\". \
                     Check src/telescope/lenses/ for existing lenses. \
                     Create the .rs file implementing the Lens trait with scan() method. \
                     Use n=6 constants (sigma=12, phi=2, tau=4, J2=24). Add 2+ tests. \
                     Register in src/telescope/lenses/mod.rs. Run cargo test to verify."
                )
            }
            DiscoveryAction::NewExperiment { hypothesis } => {
                format!(
                    "In {root}/, create a new experiment to verify this hypothesis: \
                     \"{hypothesis}\". \
                     Check experiments/ for existing experiment patterns. \
                     Create a Python experiment file with: setup, run, measure, compare. \
                     Use n=6 constants. Include statistical significance test. \
                     Output results as a table."
                )
            }
            DiscoveryAction::NewCalculator { computation } => {
                format!(
                    "In {root}/tools/, create a new Rust calculator for: \"{computation}\". \
                     Follow the pattern in existing tools/*-calc/ directories. \
                     Create main.rs with the computation. Use n=6 constants. \
                     Build with: ~/.cargo/bin/rustc main.rs -o output"
                )
            }
            DiscoveryAction::NewBreakthrough { bt_summary } => {
                format!(
                    "In {root}/, analyze this discovery as a potential breakthrough theorem: \
                     \"{bt_summary}\". \
                     Read docs/breakthrough-theorems.md for existing BTs and format. \
                     If it qualifies (n=6 EXACT match, cross-domain, 3+ lens consensus), \
                     add it as BT-128+ with proper grading (EXACT/CLOSE/WEAK). \
                     Also update docs/atlas-constants.md if new constants found."
                )
            }
            DiscoveryAction::DeepenAnalysis { node_id, question } => {
                format!(
                    "In {root}/, deepen analysis of discovery {node_id}: \"{question}\". \
                     Use NEXUS-6 scan (import nexus; nexus.scan_all()) on related data. \
                     Check for n=6 constant matches. \
                     Write findings to docs/hypotheses/ as a new hypothesis file. \
                     Cross-reference with existing BTs in docs/breakthrough-theorems.md."
                )
            }
            DiscoveryAction::CrossDomain { domains, pattern } => {
                format!(
                    "In {root}/, analyze cross-domain resonance between {:?}: \
                     \"{pattern}\". \
                     Check docs/cross-domain-resonance-2026-03-31.md for existing patterns. \
                     Run NEXUS-6 scan on the pattern data across domains. \
                     If 3+ lenses agree, record in cross-domain resonance doc. \
                     Look for formula reuse between domains.",
                    domains
                )
            }
        }
    }

    /// Execute a single Claude CLI call for a discovery action.
    /// Returns the result (success/failure + files modified).
    pub fn execute_action(&mut self, action: &DiscoveryAction) -> CliResult {
        // Check retry limit (phi = 2) — must precede enabled check
        // so exhausted actions are rejected even when CLI is disabled
        let action_key = format!("{:?}", action);
        let retries = self.retries.get(&action_key).copied().unwrap_or(0);
        if retries >= PHI {
            return CliResult {
                action: action.clone(),
                success: false,
                files_modified: Vec::new(),
                summary: format!("Retry limit reached (phi={})", PHI),
                exit_code: -2,
            };
        }

        if !self.config.enabled {
            return CliResult {
                action: action.clone(),
                success: false,
                files_modified: Vec::new(),
                summary: "CLI disabled".to_string(),
                exit_code: -1,
            };
        }

        let prompt = self.build_prompt(action);

        let output = Command::new(&self.config.claude_cli_path)
            .arg("-p")
            .arg(&prompt)
            .arg("--allowedTools")
            .arg(&self.config.allowed_tools)
            .current_dir(&self.config.project_root)
            .output();

        let result = match output {
            Ok(out) => {
                let exit_code = out.status.code().unwrap_or(-1);
                let stdout = String::from_utf8_lossy(&out.stdout).to_string();
                let success = out.status.success();

                CliResult {
                    action: action.clone(),
                    success,
                    files_modified: extract_modified_files(&stdout),
                    summary: truncate_str(&stdout, 500),
                    exit_code,
                }
            }
            Err(e) => CliResult {
                action: action.clone(),
                success: false,
                files_modified: Vec::new(),
                summary: format!("CLI execution error: {}", e),
                exit_code: -1,
            },
        };

        // Track retries on failure
        if !result.success {
            *self.retries.entry(action_key).or_insert(0) += 1;
        }

        self.cli_history.push(result.clone());
        result
    }

    /// Execute all planned actions for a cycle's discoveries.
    /// Returns (successes, failures).
    pub fn execute_cycle(
        &mut self,
        discoveries: &[(String, f64)],
        domain: &str,
        cycle: usize,
    ) -> (Vec<CliResult>, Vec<CliResult>) {
        let actions = self.plan_actions(discoveries, domain, cycle);

        let mut successes = Vec::new();
        let mut failures = Vec::new();

        for (action, _confidence) in &actions {
            let result = self.execute_action(action);
            if result.success {
                successes.push(result);
            } else {
                failures.push(result);
            }
        }

        // Apply cooldown on too many failures (tau = 4 cycles)
        if failures.len() > successes.len() && !failures.is_empty() {
            self.config.cooldowns.insert(domain.to_string(), TAU);
        }

        // Decrement all cooldowns
        let domains: Vec<String> = self.config.cooldowns.keys().cloned().collect();
        for d in domains {
            if let Some(c) = self.config.cooldowns.get_mut(&d) {
                *c = c.saturating_sub(1);
            }
        }

        (successes, failures)
    }

    /// Get summary statistics.
    pub fn summary(&self) -> DiscoveryLoopSummary {
        let total = self.cli_history.len();
        let successes = self.cli_history.iter().filter(|r| r.success).count();
        let total_files: usize = self.cli_history
            .iter()
            .filter(|r| r.success)
            .map(|r| r.files_modified.len())
            .sum();

        DiscoveryLoopSummary {
            total_calls: total,
            successes,
            failures: total - successes,
            files_generated: total_files,
            success_rate: if total > 0 {
                successes as f64 / total as f64
            } else {
                0.0
            },
        }
    }
}

#[derive(Debug, Clone)]
pub struct DiscoveryLoopSummary {
    pub total_calls: usize,
    pub successes: usize,
    pub failures: usize,
    pub files_generated: usize,
    pub success_rate: f64,
}

/// Extract file paths from CLI output (heuristic: lines containing "Created" or "Modified").
fn extract_modified_files(output: &str) -> Vec<String> {
    let mut files = Vec::new();
    for line in output.lines() {
        let lower = line.to_lowercase();
        if lower.contains("created") || lower.contains("modified") || lower.contains("wrote") {
            // Try to extract a path-like token
            for token in line.split_whitespace() {
                if token.contains('/') && (token.ends_with(".rs") || token.ends_with(".py") || token.ends_with(".md") || token.ends_with(".toml")) {
                    files.push(token.trim_matches(|c: char| !c.is_alphanumeric() && c != '/' && c != '.' && c != '_' && c != '-').to_string());
                }
            }
        }
    }
    files.dedup();
    files
}

fn truncate_str(s: &str, max_len: usize) -> String {
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

    #[test]
    fn test_plan_actions_empty() {
        let dl = DiscoveryLoop::new();
        let actions = dl.plan_actions(&[], "test", 1);
        assert!(actions.is_empty());
    }

    #[test]
    fn test_plan_actions_high_confidence() {
        let dl = DiscoveryLoop::new();
        let discoveries = vec![("n=6 pattern in AI".to_string(), 0.8)];
        let actions = dl.plan_actions(&discoveries, "ai", 1);
        // High confidence should generate experiment + breakthrough
        assert!(actions.len() >= 2);
        assert!(actions.iter().any(|(a, _)| matches!(a, DiscoveryAction::NewExperiment { .. })));
        assert!(actions.iter().any(|(a, _)| matches!(a, DiscoveryAction::NewBreakthrough { .. })));
    }

    #[test]
    fn test_plan_actions_medium_confidence() {
        let dl = DiscoveryLoop::new();
        let discoveries = vec![("possible resonance".to_string(), 0.4)];
        let actions = dl.plan_actions(&discoveries, "physics", 1);
        assert!(actions.iter().any(|(a, _)| matches!(a, DiscoveryAction::DeepenAnalysis { .. })));
    }

    #[test]
    fn test_plan_actions_lens_keyword() {
        let dl = DiscoveryLoop::new();
        let discoveries = vec![("new metric pattern detected".to_string(), 0.5)];
        let actions = dl.plan_actions(&discoveries, "ai", 1);
        assert!(actions.iter().any(|(a, _)| matches!(a, DiscoveryAction::NewLens { .. })));
    }

    #[test]
    fn test_plan_actions_cross_domain() {
        let dl = DiscoveryLoop::new();
        let discoveries = vec![("cross-domain bridge between AI and energy".to_string(), 0.6)];
        let actions = dl.plan_actions(&discoveries, "ai", 1);
        assert!(actions.iter().any(|(a, _)| matches!(a, DiscoveryAction::CrossDomain { .. })));
    }

    #[test]
    fn test_plan_actions_max_limit() {
        let dl = DiscoveryLoop::new();
        // Many high confidence discoveries
        let discoveries: Vec<(String, f64)> = (0..20)
            .map(|i| (format!("discovery pattern lens {}", i), 0.9))
            .collect();
        let actions = dl.plan_actions(&discoveries, "ai", 1);
        assert!(actions.len() <= N); // max n=6
    }

    #[test]
    fn test_plan_actions_cooldown() {
        let mut dl = DiscoveryLoop::new();
        dl.config.cooldowns.insert("frozen".to_string(), 3);
        let discoveries = vec![("important finding".to_string(), 0.9)];
        let actions = dl.plan_actions(&discoveries, "frozen", 1);
        assert!(actions.is_empty()); // cooldown blocks
    }

    #[test]
    fn test_build_prompt_new_lens() {
        let dl = DiscoveryLoop::new();
        let action = DiscoveryAction::NewLens {
            pattern_description: "hexagonal clustering in data".to_string(),
        };
        let prompt = dl.build_prompt(&action);
        assert!(prompt.contains("telescope lens"));
        assert!(prompt.contains("hexagonal clustering"));
        assert!(prompt.contains("n=6"));
    }

    #[test]
    fn test_execute_disabled() {
        let mut dl = DiscoveryLoop::new();
        dl.config.enabled = false;
        let action = DiscoveryAction::NewLens {
            pattern_description: "test".to_string(),
        };
        let result = dl.execute_action(&action);
        assert!(!result.success);
        assert_eq!(result.exit_code, -1);
    }

    #[test]
    fn test_retry_limit() {
        let mut dl = DiscoveryLoop::new();
        dl.config.enabled = false; // prevent actual CLI calls
        let action = DiscoveryAction::NewLens {
            pattern_description: "test".to_string(),
        };
        // Manually set retries to phi=2
        dl.retries.insert(format!("{:?}", action), PHI);
        let result = dl.execute_action(&action);
        assert!(!result.success);
        assert_eq!(result.exit_code, -2); // retry limit code
    }

    #[test]
    fn test_extract_modified_files() {
        let output = "Created /path/to/new_lens.rs\nModified src/telescope/mod.rs\nDone.";
        let files = extract_modified_files(output);
        assert!(files.iter().any(|f| f.contains("new_lens.rs")));
        assert!(files.iter().any(|f| f.contains("mod.rs")));
    }

    #[test]
    fn test_summary_empty() {
        let dl = DiscoveryLoop::new();
        let s = dl.summary();
        assert_eq!(s.total_calls, 0);
        assert_eq!(s.success_rate, 0.0);
    }
}
