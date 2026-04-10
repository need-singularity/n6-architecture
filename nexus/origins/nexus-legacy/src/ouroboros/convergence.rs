use super::engine::CycleResult;

/// Convergence status of the evolution loop.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConvergenceStatus {
    /// Still finding new discoveries at a healthy rate.
    Exploring,
    /// Discovery rate is decreasing — approaching saturation.
    Converging,
    /// No new discoveries in recent cycles — evolution complete.
    Saturated,
    /// Discovery rate is increasing unexpectedly (possible runaway).
    Divergent,
}

/// Checks whether the evolution loop has converged based on cycle history.
pub struct ConvergenceChecker {
    /// Minimum number of cycles before convergence can be declared.
    pub min_cycles: usize,
    /// Number of consecutive zero-discovery cycles to declare saturation.
    pub saturation_window: usize,
    /// Threshold for rate-of-change to declare convergence (0.0..1.0).
    /// If discovery rate drops by more than this fraction, we're converging.
    pub convergence_threshold: f64,
}

impl Default for ConvergenceChecker {
    fn default() -> Self {
        Self {
            min_cycles: 3,
            saturation_window: 3,
            convergence_threshold: 0.5,
        }
    }
}

impl ConvergenceChecker {
    pub fn new(min_cycles: usize, saturation_window: usize, convergence_threshold: f64) -> Self {
        Self {
            min_cycles,
            saturation_window,
            convergence_threshold,
        }
    }

    /// Evaluate convergence status from cycle history.
    ///
    /// Logic:
    /// 1. If fewer than min_cycles completed, always Exploring.
    /// 2. If the last `saturation_window` cycles all have 0 new discoveries → Saturated.
    /// 3. Compare average discovery rate of the recent half vs. the earlier half:
    ///    - If recent is significantly lower → Converging.
    ///    - If recent is significantly higher → Divergent.
    ///    - Otherwise → Exploring.
    pub fn check(&self, history: &[CycleResult]) -> ConvergenceStatus {
        if history.len() < self.min_cycles {
            return ConvergenceStatus::Exploring;
        }

        // Check saturation: last N cycles all zero discoveries
        let window = self.saturation_window.min(history.len());
        let tail = &history[history.len() - window..];
        if tail.iter().all(|c| c.new_discoveries == 0) {
            return ConvergenceStatus::Saturated;
        }

        // Compare recent half vs earlier half
        if history.len() < 2 {
            return ConvergenceStatus::Exploring;
        }

        let mid = history.len() / 2;
        let early = &history[..mid];
        let recent = &history[mid..];

        let early_avg = avg_discoveries(early);
        let recent_avg = avg_discoveries(recent);

        if early_avg == 0.0 && recent_avg == 0.0 {
            return ConvergenceStatus::Saturated;
        }

        if early_avg > 0.0 {
            let change_ratio = recent_avg / early_avg;
            if change_ratio < (1.0 - self.convergence_threshold) {
                // Recent discoveries dropped by more than threshold
                return ConvergenceStatus::Converging;
            }
            if change_ratio > (1.0 + self.convergence_threshold) {
                // Recent discoveries increased unexpectedly
                return ConvergenceStatus::Divergent;
            }
        } else if recent_avg > 0.0 {
            // Was zero, now finding things → divergent/expanding
            return ConvergenceStatus::Divergent;
        }

        ConvergenceStatus::Exploring
    }
}

fn avg_discoveries(cycles: &[CycleResult]) -> f64 {
    if cycles.is_empty() {
        return 0.0;
    }
    let total: usize = cycles.iter().map(|c| c.new_discoveries).sum();
    total as f64 / cycles.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cycle(discoveries: usize) -> CycleResult {
        CycleResult {
            cycle: 0,
            domain: "test".to_string(),
            lenses_used: vec![],
            new_discoveries: discoveries,
            graph_nodes: 0,
            graph_edges: 0,
            verification_score: 0.0,
            lens_candidates_found: 0,
            lenses_evolved: 0,
        }
    }

    #[test]
    fn test_too_few_cycles() {
        let checker = ConvergenceChecker::default();
        let history = vec![make_cycle(5), make_cycle(3)];
        assert_eq!(checker.check(&history), ConvergenceStatus::Exploring);
    }

    #[test]
    fn test_saturation() {
        let checker = ConvergenceChecker::new(2, 3, 0.5);
        let history = vec![
            make_cycle(5),
            make_cycle(3),
            make_cycle(0),
            make_cycle(0),
            make_cycle(0),
        ];
        assert_eq!(checker.check(&history), ConvergenceStatus::Saturated);
    }

    #[test]
    fn test_converging() {
        let checker = ConvergenceChecker::new(2, 3, 0.5);
        // Early half: 10,10 → avg 10. Recent half: 2,1 → avg 1.5. ratio=0.15 < 0.5
        let history = vec![
            make_cycle(10),
            make_cycle(10),
            make_cycle(2),
            make_cycle(1),
        ];
        assert_eq!(checker.check(&history), ConvergenceStatus::Converging);
    }

    #[test]
    fn test_divergent() {
        let checker = ConvergenceChecker::new(2, 3, 0.5);
        // Early half: 1,1 → avg 1. Recent half: 10,10 → avg 10. ratio=10 > 1.5
        let history = vec![
            make_cycle(1),
            make_cycle(1),
            make_cycle(10),
            make_cycle(10),
        ];
        assert_eq!(checker.check(&history), ConvergenceStatus::Divergent);
    }
}
