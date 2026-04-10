//! Autonomous Agent — self-directed discovery engine.
//!
//! An AutonomousAgent runs a configurable loop of scan/evolve/forge/experiment
//! cycles, automatically switching modes when saturation is detected.

use std::time::Instant;

use crate::lens_forge::forge_engine::{self, ForgeConfig};
use crate::ouroboros::engine::{EvolutionConfig, EvolutionEngine};
use crate::ouroboros::convergence::ConvergenceStatus;
use crate::telescope::registry::LensRegistry;
use crate::history::recorder::ScanRecord;

/// Operating mode of an autonomous agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentMode {
    /// Explore new domains — broad scan with serendipity.
    Explore,
    /// Deepen existing discoveries — focused evolution.
    Deepen,
    /// Verify unverified discoveries — strict confirmation.
    Verify,
    /// Attempt to disprove existing discoveries — adversarial.
    RedTeam,
    /// Create new lenses via LensForge.
    Forge,
    /// Run automated experiments.
    Experiment,
}

impl std::fmt::Display for AgentMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AgentMode::Explore => write!(f, "explore"),
            AgentMode::Deepen => write!(f, "deepen"),
            AgentMode::Verify => write!(f, "verify"),
            AgentMode::RedTeam => write!(f, "redteam"),
            AgentMode::Forge => write!(f, "forge"),
            AgentMode::Experiment => write!(f, "experiment"),
        }
    }
}

/// Current status of the agent.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AgentStatus {
    Idle,
    Running,
    Paused,
    Completed,
}

/// Configuration for creating an autonomous agent.
#[derive(Debug, Clone)]
pub struct AgentConfig {
    /// Maximum cycles before stopping.
    pub max_cycles: usize,
    /// Domains to operate in.
    pub domains: Vec<String>,
    /// Starting mode.
    pub mode: AgentMode,
    /// Automatically register discoveries.
    pub auto_register: bool,
    /// Serendipity ratio for exploration (0.0..1.0).
    pub serendipity: f64,
}

impl Default for AgentConfig {
    fn default() -> Self {
        Self {
            max_cycles: 6, // n=6
            domains: vec!["general".to_string()],
            mode: AgentMode::Explore,
            auto_register: true,
            serendipity: 0.2,
        }
    }
}

/// Report produced after an agent run.
#[derive(Debug, Clone)]
pub struct AgentReport {
    pub agent_id: String,
    pub mode: String,
    pub cycles_completed: usize,
    pub discoveries: Vec<String>,
    pub experiments_run: usize,
    pub lenses_forged: usize,
    pub time_elapsed_ms: u64,
    pub final_status: String,
    pub domains_covered: Vec<String>,
}

/// An autonomous agent that runs discovery cycles.
pub struct AutonomousAgent {
    pub id: String,
    pub mode: AgentMode,
    pub status: AgentStatus,
    config: AgentConfig,
}

impl AutonomousAgent {
    /// Create a new agent with the given configuration.
    pub fn new(config: AgentConfig) -> Self {
        let id = format!(
            "agent-{}-{}",
            config.mode,
            config.domains.first().map(|s| s.as_str()).unwrap_or("general")
        );
        let mode = config.mode.clone();
        Self {
            id,
            mode,
            status: AgentStatus::Idle,
            config,
        }
    }

    /// Create an agent with a specific ID.
    pub fn with_id(id: String, config: AgentConfig) -> Self {
        let mode = config.mode.clone();
        Self {
            id,
            mode,
            status: AgentStatus::Idle,
            config,
        }
    }

    /// Run the agent through its configured cycles.
    ///
    /// The agent loops through its mode-specific behavior:
    /// 1. Select action based on mode
    /// 2. Execute scan/evolve/forge/experiment
    /// 3. Collect discoveries
    /// 4. Auto-switch mode on saturation
    /// 5. Return cumulative report
    pub fn run(&mut self) -> AgentReport {
        let start = Instant::now();
        self.status = AgentStatus::Running;

        let mut all_discoveries: Vec<String> = Vec::new();
        let mut total_experiments = 0usize;
        let mut total_lenses_forged = 0usize;
        let mut domains_covered: Vec<String> = Vec::new();
        let mut cycles_completed = 0usize;

        for domain in &self.config.domains {
            if !domains_covered.contains(domain) {
                domains_covered.push(domain.clone());
            }

            let result = self.run_domain(domain);
            cycles_completed += result.cycles;
            all_discoveries.extend(result.discoveries);
            total_experiments += result.experiments;
            total_lenses_forged += result.lenses_forged;

            // Mode auto-switch: if saturated in one mode, try the next
            if result.saturated && self.config.auto_register {
                self.mode = self.next_mode();
            }
        }

        self.status = AgentStatus::Completed;
        let elapsed = start.elapsed().as_millis() as u64;

        AgentReport {
            agent_id: self.id.clone(),
            mode: self.mode.to_string(),
            cycles_completed,
            discoveries: all_discoveries,
            experiments_run: total_experiments,
            lenses_forged: total_lenses_forged,
            time_elapsed_ms: elapsed,
            final_status: format!("{:?}", self.status),
            domains_covered,
        }
    }

    /// Run cycles on a single domain.
    fn run_domain(&self, domain: &str) -> DomainRunResult {
        let mut discoveries: Vec<String> = Vec::new();
        let mut experiments = 0usize;
        let mut lenses_forged = 0usize;
        let mut cycles = 0usize;
        let mut saturated = false;

        match self.mode {
            AgentMode::Explore | AgentMode::Deepen | AgentMode::Verify | AgentMode::RedTeam => {
                let mut evo_config = EvolutionConfig::default();
                evo_config.domain = domain.to_string();

                // Adjust config based on mode
                match self.mode {
                    AgentMode::Explore => {
                        evo_config.serendipity_ratio = self.config.serendipity.max(0.3);
                        evo_config.min_verification_score = 0.2;
                    }
                    AgentMode::Deepen => {
                        evo_config.serendipity_ratio = 0.05;
                        evo_config.min_verification_score = 0.4;
                    }
                    AgentMode::Verify => {
                        evo_config.serendipity_ratio = 0.0;
                        evo_config.min_verification_score = 0.6;
                    }
                    AgentMode::RedTeam => {
                        // RedTeam: high bar, try to find weaknesses
                        evo_config.serendipity_ratio = 0.1;
                        evo_config.min_verification_score = 0.8;
                    }
                    _ => {}
                }

                let seeds = vec![format!(
                    "n=6 pattern in {} ({} mode)",
                    domain, self.mode
                )];
                let mut engine = EvolutionEngine::new(evo_config, seeds);

                let max = self.config.max_cycles;
                let (status, history) = engine.run_loop(max);

                for cr in &history {
                    cycles += 1;
                    if cr.new_discoveries > 0 {
                        discoveries.push(format!(
                            "[{}] cycle {} in {}: {} discoveries (score {:.3})",
                            self.mode, cr.cycle, domain, cr.new_discoveries, cr.verification_score
                        ));
                    }
                }

                saturated = status == ConvergenceStatus::Saturated;
            }
            AgentMode::Forge => {
                let registry = LensRegistry::new();
                let records: Vec<ScanRecord> = Vec::new();
                let forge_config = ForgeConfig::default();
                let result = forge_engine::forge_cycle(&registry, &records, &forge_config);
                lenses_forged = result.candidates_accepted;
                cycles = 1;

                for lens in &result.new_lenses {
                    discoveries.push(format!("[forge] new lens: {}", lens.name));
                }
            }
            AgentMode::Experiment => {
                // Run a simple experiment cycle per domain
                let evo_config = EvolutionConfig {
                    domain: domain.to_string(),
                    ..EvolutionConfig::default()
                };
                let seeds = vec![format!("experiment probe: {} domain", domain)];
                let mut engine = EvolutionEngine::new(evo_config, seeds);

                let result = engine.evolve_step();
                experiments = 1;
                cycles = 1;
                if result.new_discoveries > 0 {
                    discoveries.push(format!(
                        "[experiment] {} in {}: {} discoveries",
                        "probe", domain, result.new_discoveries
                    ));
                }
            }
        }

        DomainRunResult {
            discoveries,
            experiments,
            lenses_forged,
            cycles,
            saturated,
        }
    }

    /// Determine the next mode when current mode saturates.
    fn next_mode(&self) -> AgentMode {
        match self.mode {
            AgentMode::Explore => AgentMode::Deepen,
            AgentMode::Deepen => AgentMode::Verify,
            AgentMode::Verify => AgentMode::Forge,
            AgentMode::Forge => AgentMode::Experiment,
            AgentMode::Experiment => AgentMode::RedTeam,
            AgentMode::RedTeam => AgentMode::Explore,
        }
    }
}

/// Internal result for a single domain run.
struct DomainRunResult {
    discoveries: Vec<String>,
    experiments: usize,
    lenses_forged: usize,
    cycles: usize,
    saturated: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_creation() {
        let config = AgentConfig::default();
        let agent = AutonomousAgent::new(config);
        assert_eq!(agent.status, AgentStatus::Idle);
        assert_eq!(agent.mode, AgentMode::Explore);
        assert!(agent.id.contains("explore"));
    }

    #[test]
    fn test_agent_with_id() {
        let config = AgentConfig {
            mode: AgentMode::RedTeam,
            ..AgentConfig::default()
        };
        let agent = AutonomousAgent::with_id("custom-id".to_string(), config);
        assert_eq!(agent.id, "custom-id");
        assert_eq!(agent.mode, AgentMode::RedTeam);
    }

    #[test]
    fn test_agent_run_explore() {
        let config = AgentConfig {
            max_cycles: 3,
            domains: vec!["physics".to_string()],
            mode: AgentMode::Explore,
            auto_register: true,
            serendipity: 0.3,
        };
        let mut agent = AutonomousAgent::new(config);
        let report = agent.run();

        assert_eq!(agent.status, AgentStatus::Completed);
        assert!(report.cycles_completed > 0);
        assert_eq!(report.domains_covered, vec!["physics".to_string()]);
        assert!(report.time_elapsed_ms < 10_000); // should be fast
    }

    #[test]
    fn test_agent_run_forge() {
        let config = AgentConfig {
            max_cycles: 1,
            domains: vec!["general".to_string()],
            mode: AgentMode::Forge,
            auto_register: false,
            serendipity: 0.0,
        };
        let mut agent = AutonomousAgent::new(config);
        let report = agent.run();

        assert_eq!(agent.status, AgentStatus::Completed);
        assert_eq!(report.cycles_completed, 1);
    }

    #[test]
    fn test_mode_auto_switch() {
        let agent = AutonomousAgent::new(AgentConfig::default());
        assert_eq!(agent.next_mode(), AgentMode::Deepen);

        let config2 = AgentConfig {
            mode: AgentMode::RedTeam,
            ..AgentConfig::default()
        };
        let agent2 = AutonomousAgent::new(config2);
        assert_eq!(agent2.next_mode(), AgentMode::Explore);
    }

    #[test]
    fn test_agent_mode_display() {
        assert_eq!(AgentMode::Explore.to_string(), "explore");
        assert_eq!(AgentMode::RedTeam.to_string(), "redteam");
        assert_eq!(AgentMode::Forge.to_string(), "forge");
    }
}
