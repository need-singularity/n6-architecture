//! Agent Scheduler — orchestrate multiple autonomous agents.

use super::agent::{AgentConfig, AgentReport, AutonomousAgent};

/// Orchestrates multiple autonomous agents.
pub struct AgentScheduler {
    agents: Vec<AutonomousAgent>,
}

impl AgentScheduler {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
        }
    }

    /// Add an agent with the given configuration.
    pub fn add_agent(&mut self, config: AgentConfig) {
        self.agents.push(AutonomousAgent::new(config));
    }

    /// Add an agent with a specific ID.
    pub fn add_agent_with_id(&mut self, id: String, config: AgentConfig) {
        self.agents.push(AutonomousAgent::with_id(id, config));
    }

    /// Number of agents registered.
    pub fn agent_count(&self) -> usize {
        self.agents.len()
    }

    /// Run all agents sequentially and collect reports.
    pub fn run_all(&mut self) -> Vec<AgentReport> {
        let mut reports = Vec::new();
        for agent in &mut self.agents {
            reports.push(agent.run());
        }
        reports
    }

    /// Run agents in round-robin fashion for a given number of rounds.
    ///
    /// Each round, every agent runs one domain cycle. This is useful for
    /// interleaving agent work to detect cross-agent synergies.
    pub fn run_round_robin(&mut self, rounds: usize) -> Vec<AgentReport> {
        // For round-robin, we set each agent's max_cycles to 1 per round
        // and run `rounds` times. The final report is from the last run.
        let mut final_reports: Vec<AgentReport> = Vec::new();

        for _round in 0..rounds {
            for agent in &mut self.agents {
                let report = agent.run();
                // Update or insert the latest report for this agent
                if let Some(existing) = final_reports.iter_mut().find(|r| r.agent_id == report.agent_id) {
                    existing.cycles_completed += report.cycles_completed;
                    existing.discoveries.extend(report.discoveries.clone());
                    existing.experiments_run += report.experiments_run;
                    existing.lenses_forged += report.lenses_forged;
                    existing.time_elapsed_ms += report.time_elapsed_ms;
                    existing.final_status = report.final_status.clone();
                } else {
                    final_reports.push(report);
                }
            }
        }

        final_reports
    }
}

impl Default for AgentScheduler {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::agent::AgentMode;

    #[test]
    fn test_scheduler_creation() {
        let scheduler = AgentScheduler::new();
        assert_eq!(scheduler.agent_count(), 0);
    }

    #[test]
    fn test_scheduler_add_agents() {
        let mut scheduler = AgentScheduler::new();
        scheduler.add_agent(AgentConfig::default());
        scheduler.add_agent(AgentConfig {
            mode: AgentMode::RedTeam,
            ..AgentConfig::default()
        });
        assert_eq!(scheduler.agent_count(), 2);
    }

    #[test]
    fn test_scheduler_run_all() {
        let mut scheduler = AgentScheduler::new();
        scheduler.add_agent(AgentConfig {
            max_cycles: 2,
            domains: vec!["physics".to_string()],
            mode: AgentMode::Explore,
            ..AgentConfig::default()
        });
        scheduler.add_agent(AgentConfig {
            max_cycles: 2,
            domains: vec!["physics".to_string()],
            mode: AgentMode::Deepen,
            ..AgentConfig::default()
        });

        let reports = scheduler.run_all();
        assert_eq!(reports.len(), 2);
        for report in &reports {
            assert!(report.cycles_completed > 0);
        }
    }

    #[test]
    fn test_scheduler_round_robin() {
        let mut scheduler = AgentScheduler::new();
        scheduler.add_agent(AgentConfig {
            max_cycles: 2,
            domains: vec!["test".to_string()],
            mode: AgentMode::Explore,
            ..AgentConfig::default()
        });

        let reports = scheduler.run_round_robin(2);
        assert_eq!(reports.len(), 1);
        // Round robin ran 2 rounds, so cycles should accumulate
        assert!(reports[0].cycles_completed >= 2);
    }

    #[test]
    fn test_scheduler_default() {
        let scheduler = AgentScheduler::default();
        assert_eq!(scheduler.agent_count(), 0);
    }
}
