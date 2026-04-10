//! Watchdog — monitors agent health and progress.
//!
//! Detects stalled agents, runaway cycles, and anomalous behavior.

use super::agent::AgentReport;

/// Watchdog alert levels.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AlertLevel {
    /// Everything normal.
    Ok,
    /// Agent is slow but progressing.
    Warning,
    /// Agent appears stalled or runaway.
    Critical,
}

/// Result of a watchdog health check.
#[derive(Debug, Clone)]
pub struct HealthCheck {
    pub agent_id: String,
    pub alert: AlertLevel,
    pub message: String,
    pub discoveries_per_cycle: f64,
    pub avg_cycle_time_ms: f64,
}

/// Configuration for watchdog thresholds.
#[derive(Debug, Clone)]
pub struct WatchdogConfig {
    /// Maximum allowed time per cycle (ms) before warning.
    pub max_cycle_time_ms: u64,
    /// Minimum expected discoveries per cycle.
    pub min_discoveries_per_cycle: f64,
    /// Maximum cycles with zero progress before critical.
    pub max_stall_cycles: usize,
}

impl Default for WatchdogConfig {
    fn default() -> Self {
        Self {
            max_cycle_time_ms: 60_000, // 1 minute
            min_discoveries_per_cycle: 0.1,
            max_stall_cycles: 6, // n=6
        }
    }
}

/// Check health of an agent based on its report.
pub fn check_health(report: &AgentReport, config: &WatchdogConfig) -> HealthCheck {
    let discoveries_per_cycle = if report.cycles_completed > 0 {
        report.discoveries.len() as f64 / report.cycles_completed as f64
    } else {
        0.0
    };

    let avg_cycle_time_ms = if report.cycles_completed > 0 {
        report.time_elapsed_ms as f64 / report.cycles_completed as f64
    } else {
        0.0
    };

    // Determine alert level
    let (alert, message) = if report.cycles_completed == 0 {
        (AlertLevel::Critical, "Agent completed zero cycles".to_string())
    } else if discoveries_per_cycle < config.min_discoveries_per_cycle
        && report.discoveries.is_empty()
    {
        (
            AlertLevel::Critical,
            format!(
                "No discoveries in {} cycles (stalled)",
                report.cycles_completed
            ),
        )
    } else if avg_cycle_time_ms > config.max_cycle_time_ms as f64 {
        (
            AlertLevel::Warning,
            format!(
                "Slow cycles: {:.0}ms avg (limit {}ms)",
                avg_cycle_time_ms, config.max_cycle_time_ms
            ),
        )
    } else if discoveries_per_cycle < config.min_discoveries_per_cycle {
        (
            AlertLevel::Warning,
            format!(
                "Low discovery rate: {:.2}/cycle (min {:.2})",
                discoveries_per_cycle, config.min_discoveries_per_cycle
            ),
        )
    } else {
        (
            AlertLevel::Ok,
            format!(
                "Healthy: {:.2} discoveries/cycle, {:.0}ms/cycle",
                discoveries_per_cycle, avg_cycle_time_ms
            ),
        )
    };

    HealthCheck {
        agent_id: report.agent_id.clone(),
        alert,
        message,
        discoveries_per_cycle,
        avg_cycle_time_ms,
    }
}

/// Check health of multiple agent reports.
pub fn check_all(reports: &[AgentReport], config: &WatchdogConfig) -> Vec<HealthCheck> {
    reports.iter().map(|r| check_health(r, config)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_report(cycles: usize, discoveries: usize, time_ms: u64) -> AgentReport {
        AgentReport {
            agent_id: "test-agent".to_string(),
            mode: "explore".to_string(),
            cycles_completed: cycles,
            discoveries: (0..discoveries)
                .map(|i| format!("discovery-{}", i))
                .collect(),
            experiments_run: 0,
            lenses_forged: 0,
            time_elapsed_ms: time_ms,
            final_status: "Completed".to_string(),
            domains_covered: vec!["test".to_string()],
        }
    }

    #[test]
    fn test_healthy_agent() {
        let report = make_report(6, 3, 600);
        let check = check_health(&report, &WatchdogConfig::default());
        assert_eq!(check.alert, AlertLevel::Ok);
        assert!(check.discoveries_per_cycle > 0.0);
    }

    #[test]
    fn test_stalled_agent() {
        let report = make_report(6, 0, 600);
        let check = check_health(&report, &WatchdogConfig::default());
        assert_eq!(check.alert, AlertLevel::Critical);
    }

    #[test]
    fn test_slow_agent() {
        let report = make_report(2, 2, 200_000);
        let check = check_health(&report, &WatchdogConfig::default());
        assert_eq!(check.alert, AlertLevel::Warning);
    }

    #[test]
    fn test_zero_cycles() {
        let report = make_report(0, 0, 0);
        let check = check_health(&report, &WatchdogConfig::default());
        assert_eq!(check.alert, AlertLevel::Critical);
    }

    #[test]
    fn test_check_all() {
        let reports = vec![
            make_report(6, 3, 600),
            make_report(6, 0, 600),
        ];
        let checks = check_all(&reports, &WatchdogConfig::default());
        assert_eq!(checks.len(), 2);
        assert_eq!(checks[0].alert, AlertLevel::Ok);
        assert_eq!(checks[1].alert, AlertLevel::Critical);
    }
}
