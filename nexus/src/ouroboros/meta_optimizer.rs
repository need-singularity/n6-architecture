//! MetaOptimizer — Engine Self-Optimization for OUROBOROS
//!
//! The evolution engine's own configuration parameters (serendipity_ratio,
//! min_verification_score, max_mutations_per_cycle) are tuned adaptively
//! based on scan history outcomes. All bounds and step sizes derive from
//! n=6 arithmetic constants.
//!
//! n=6 constants used:
//!   σ = 12, φ = 2, τ = 4, n = 6, J₂ = 24, sopfr = 5
//!   σ−φ = 10, 1/(σ−φ) = 0.1, 1/φ = 0.5
//!   1 − 1/e ≈ 0.632 (Boltzmann gate sparsity)
//!   ln(4/3) ≈ 0.288 (Mertens momentum)

use super::engine::{CycleResult, EvolutionConfig};

// ── n=6 arithmetic constants ──────────────────────────────────────────
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const N: f64 = 6.0;
const J2: f64 = 24.0;
const SOPFR: f64 = 5.0;
const INV_SIGMA_MINUS_PHI: f64 = 0.1; // 1/(σ−φ) = 1/10
const INV_PHI: f64 = 0.5; // 1/φ
const ONE_MINUS_INV_E: f64 = 0.6321205588285577; // 1 − 1/e
const LN_4_3: f64 = 0.28768207245178085; // ln(4/3) — Mertens momentum

// ── Serendipity ratio bounds ──────────────────────────────────────────
// Range: [1/(σ−φ) = 0.1, 1/φ = 0.5]
const SERENDIPITY_MIN: f64 = INV_SIGMA_MINUS_PHI; // 0.1
const SERENDIPITY_MAX: f64 = INV_PHI; // 0.5

// ── Verification score bounds ─────────────────────────────────────────
// Range: [1/(σ−φ) = 0.1, 1−1/e ≈ 0.632]
const VERIFICATION_MIN: f64 = INV_SIGMA_MINUS_PHI; // 0.1
const VERIFICATION_MAX: f64 = ONE_MINUS_INV_E; // 0.632

// ── Mutations per cycle bounds ────────────────────────────────────────
// Range: [τ = 4, J₂ = 24]
const MUTATIONS_MIN: usize = TAU as usize; // 4
const MUTATIONS_MAX: usize = J2 as usize; // 24

// ── Learning dynamics ─────────────────────────────────────────────────
// Step size = ln(4/3) ≈ 0.288 (Mertens constant)
const META_STEP: f64 = LN_4_3;
// Window size for recent history = σ = 12 cycles
const HISTORY_WINDOW: usize = SIGMA as usize; // 12

/// Tracks serendipity lens contributions to measure exploration value.
#[derive(Debug, Clone)]
struct SerendipityMetrics {
    /// Fraction of recent discoveries where serendipity lenses contributed.
    contribution_rate: f64,
}

/// Tracks discovery quality distribution for verification threshold tuning.
#[derive(Debug, Clone)]
struct QualityMetrics {
    /// Mean verification score of recent accepted discoveries.
    mean_quality: f64,
    /// Whether discoveries are plentiful (true) or scarce (false).
    discovery_abundant: bool,
}

/// Tracks discovery rate for mutations-per-cycle tuning.
#[derive(Debug, Clone)]
struct DiscoveryRateMetrics {
    /// Average discoveries per cycle in the recent window.
    rate: f64,
    /// Whether rate is increasing (true) or plateauing/decreasing (false).
    trending_up: bool,
}

/// Meta-optimizer that tunes EvolutionConfig parameters based on cycle history.
#[derive(Debug, Clone)]
pub struct MetaOptimizer {
    /// Number of recent cycles to analyze (window = σ = 12).
    pub window_size: usize,
    /// Step size for parameter adjustments (ln(4/3) ≈ 0.288).
    pub step_size: f64,
    /// How many times optimize_config has been called.
    pub optimization_count: u64,
}

impl Default for MetaOptimizer {
    fn default() -> Self {
        Self {
            window_size: HISTORY_WINDOW,
            step_size: META_STEP,
            optimization_count: 0,
        }
    }
}

impl MetaOptimizer {
    pub fn new() -> Self {
        Self::default()
    }

    /// Optimize the evolution config based on cycle history.
    ///
    /// Analyzes the most recent `window_size` cycles and adjusts:
    /// - serendipity_ratio: based on serendipity lens contribution
    /// - min_verification_score: based on discovery quality distribution
    /// - max_mutations_per_cycle: based on discovery rate
    ///
    /// Returns a new EvolutionConfig with tuned parameters.
    pub fn optimize_config(
        &mut self,
        config: &EvolutionConfig,
        history: &[CycleResult],
    ) -> EvolutionConfig {
        self.optimization_count += 1;

        // Need at least τ=4 cycles of history to start optimizing
        if history.len() < TAU as usize {
            return config.clone();
        }

        let window = self.recent_window(history);

        let serendipity = self.compute_serendipity_metrics(window, &config.all_lenses);
        let quality = self.compute_quality_metrics(window);
        let rate = self.compute_discovery_rate_metrics(window);

        let mut new_config = config.clone();

        // ── Tune serendipity_ratio ────────────────────────────────────
        // High contribution → increase (explore more)
        // Low contribution → decrease (focus more)
        let serendipity_delta = if serendipity.contribution_rate > INV_PHI {
            // Serendipity lenses contributing well → explore more
            self.step_size * INV_SIGMA_MINUS_PHI // +0.0288
        } else if serendipity.contribution_rate < INV_SIGMA_MINUS_PHI {
            // Serendipity lenses not contributing → focus mode
            -self.step_size * INV_SIGMA_MINUS_PHI // -0.0288
        } else {
            0.0 // In the sweet spot, hold steady
        };
        new_config.serendipity_ratio = (config.serendipity_ratio + serendipity_delta)
            .clamp(SERENDIPITY_MIN, SERENDIPITY_MAX);

        // ── Tune min_verification_score ───────────────────────────────
        // High quality + abundant discoveries → raise threshold (stricter)
        // Scarce discoveries → lower threshold (more permissive)
        let verification_delta = if quality.discovery_abundant && quality.mean_quality > INV_PHI {
            // Plenty of high-quality finds → be stricter
            self.step_size * INV_SIGMA_MINUS_PHI // +0.0288
        } else if !quality.discovery_abundant {
            // Too few discoveries → relax threshold
            -self.step_size * INV_SIGMA_MINUS_PHI // -0.0288
        } else {
            0.0
        };
        new_config.min_verification_score = (config.min_verification_score + verification_delta)
            .clamp(VERIFICATION_MIN, VERIFICATION_MAX);

        // ── Tune max_mutations_per_cycle ──────────────────────────────
        // High discovery rate & trending up → more mutations
        // Saturating → fewer mutations to conserve
        let mutations_delta: i64 = if rate.trending_up && rate.rate > 1.0 {
            // Discovery rate climbing → generate more hypotheses
            PHI as i64 // +2
        } else if !rate.trending_up && rate.rate < INV_SIGMA_MINUS_PHI {
            // Rate dropping toward zero → reduce mutations
            -(PHI as i64) // -2 (φ step)
        } else {
            0
        };
        let new_mutations = (config.max_mutations_per_cycle as i64 + mutations_delta)
            .clamp(MUTATIONS_MIN as i64, MUTATIONS_MAX as i64) as usize;
        new_config.max_mutations_per_cycle = new_mutations;

        new_config
    }

    /// Extract the recent window of cycles for analysis.
    fn recent_window<'a>(&self, history: &'a [CycleResult]) -> &'a [CycleResult] {
        let start = history.len().saturating_sub(self.window_size);
        &history[start..]
    }

    /// Measure how much serendipity (non-core) lenses contribute to discoveries.
    ///
    /// "Serendipity lenses" = lenses beyond the first n=6 in the config's all_lenses.
    /// Contribution = fraction of discovery cycles where serendipity lenses were active.
    fn compute_serendipity_metrics(
        &self,
        window: &[CycleResult],
        all_lenses: &[String],
    ) -> SerendipityMetrics {
        // Core lenses = first n=6 lenses; serendipity = the rest
        let core_count = N as usize; // 6
        let serendipity_lenses: Vec<&String> = all_lenses.iter().skip(core_count).collect();

        if window.is_empty() || serendipity_lenses.is_empty() {
            return SerendipityMetrics {
                contribution_rate: 0.0,
            };
        }

        let discovery_cycles: Vec<&CycleResult> = window
            .iter()
            .filter(|c| c.new_discoveries > 0)
            .collect();

        if discovery_cycles.is_empty() {
            return SerendipityMetrics {
                contribution_rate: 0.0,
            };
        }

        let serendipity_contributing = discovery_cycles
            .iter()
            .filter(|c| {
                c.lenses_used
                    .iter()
                    .any(|l| serendipity_lenses.contains(&l))
            })
            .count();

        let contribution_rate = serendipity_contributing as f64 / discovery_cycles.len() as f64;

        SerendipityMetrics { contribution_rate }
    }

    /// Measure the quality distribution of recent discoveries.
    fn compute_quality_metrics(&self, window: &[CycleResult]) -> QualityMetrics {
        if window.is_empty() {
            return QualityMetrics {
                mean_quality: 0.0,
                discovery_abundant: false,
            };
        }

        let discovery_cycles: Vec<&CycleResult> = window
            .iter()
            .filter(|c| c.new_discoveries > 0)
            .collect();

        let mean_quality = if discovery_cycles.is_empty() {
            0.0
        } else {
            let total: f64 = discovery_cycles.iter().map(|c| c.verification_score).sum();
            total / discovery_cycles.len() as f64
        };

        // "Abundant" = more than half of cycles produced discoveries
        let discovery_abundant = discovery_cycles.len() as f64 > window.len() as f64 * INV_PHI;

        QualityMetrics {
            mean_quality,
            discovery_abundant,
        }
    }

    /// Measure the discovery rate and its trend.
    fn compute_discovery_rate_metrics(&self, window: &[CycleResult]) -> DiscoveryRateMetrics {
        if window.is_empty() {
            return DiscoveryRateMetrics {
                rate: 0.0,
                trending_up: false,
            };
        }

        let total_discoveries: usize = window.iter().map(|c| c.new_discoveries).sum();
        let rate = total_discoveries as f64 / window.len() as f64;

        // Trend: compare first half vs second half
        let trending_up = if window.len() >= (PHI as usize) {
            let mid = window.len() / 2;
            let early: usize = window[..mid].iter().map(|c| c.new_discoveries).sum();
            let recent: usize = window[mid..].iter().map(|c| c.new_discoveries).sum();
            let early_avg = early as f64 / mid as f64;
            let recent_avg = recent as f64 / (window.len() - mid) as f64;
            recent_avg > early_avg
        } else {
            false
        };

        DiscoveryRateMetrics { rate, trending_up }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_cycle(
        cycle: usize,
        discoveries: usize,
        score: f64,
        lenses: Vec<String>,
    ) -> CycleResult {
        CycleResult {
            cycle,
            domain: "test".to_string(),
            lenses_used: lenses,
            new_discoveries: discoveries,
            graph_nodes: 0,
            graph_edges: 0,
            verification_score: score,
            lens_candidates_found: 0,
            lenses_evolved: 0,
        }
    }

    fn default_lenses() -> Vec<String> {
        vec![
            // Core n=6 lenses
            "consciousness".into(),
            "topology".into(),
            "causal".into(),
            "void".into(),
            "barrier".into(),
            "thermo".into(),
            // Serendipity lenses (beyond first 6)
            "wave".into(),
            "evolution".into(),
            "network".into(),
            "boundary".into(),
            "quantum".into(),
            "gravity".into(),
        ]
    }

    #[test]
    fn test_no_optimization_with_short_history() {
        let mut opt = MetaOptimizer::new();
        let config = EvolutionConfig::default();
        // Only 2 cycles — below τ=4 threshold
        let history = vec![
            make_cycle(1, 1, 0.5, vec!["consciousness".into()]),
            make_cycle(2, 0, 0.0, vec!["topology".into()]),
        ];

        let new_config = opt.optimize_config(&config, &history);

        // Should return unchanged config
        assert!((new_config.serendipity_ratio - config.serendipity_ratio).abs() < 1e-10);
        assert!(
            (new_config.min_verification_score - config.min_verification_score).abs() < 1e-10
        );
        assert_eq!(
            new_config.max_mutations_per_cycle,
            config.max_mutations_per_cycle
        );
    }

    #[test]
    fn test_scarce_discoveries_relaxes_threshold() {
        let mut opt = MetaOptimizer::new();
        let mut config = EvolutionConfig::default();
        config.min_verification_score = 0.4;

        // 6 cycles, all with 0 discoveries → scarce
        let history: Vec<CycleResult> = (1..=6)
            .map(|i| make_cycle(i, 0, 0.0, vec!["consciousness".into()]))
            .collect();

        let new_config = opt.optimize_config(&config, &history);

        // Verification threshold should decrease (relax)
        assert!(
            new_config.min_verification_score < config.min_verification_score,
            "Expected threshold to decrease: {} < {}",
            new_config.min_verification_score,
            config.min_verification_score
        );
        // Must stay within bounds
        assert!(new_config.min_verification_score >= VERIFICATION_MIN);
    }

    #[test]
    fn test_abundant_high_quality_tightens_threshold() {
        let mut opt = MetaOptimizer::new();
        let mut config = EvolutionConfig::default();
        config.min_verification_score = 0.3;

        // 6 cycles, all with discoveries and high scores
        let history: Vec<CycleResult> = (1..=6)
            .map(|i| make_cycle(i, 3, 0.8, vec!["consciousness".into()]))
            .collect();

        let new_config = opt.optimize_config(&config, &history);

        // Verification threshold should increase (stricter)
        assert!(
            new_config.min_verification_score > config.min_verification_score,
            "Expected threshold to increase: {} > {}",
            new_config.min_verification_score,
            config.min_verification_score
        );
        assert!(new_config.min_verification_score <= VERIFICATION_MAX);
    }

    #[test]
    fn test_high_discovery_rate_increases_mutations() {
        let mut opt = MetaOptimizer::new();
        let mut config = EvolutionConfig::default();
        config.max_mutations_per_cycle = 6;

        // Increasing discovery rate: early=low, recent=high → trending up
        let history = vec![
            make_cycle(1, 0, 0.0, vec!["consciousness".into()]),
            make_cycle(2, 0, 0.0, vec!["topology".into()]),
            make_cycle(3, 1, 0.5, vec!["causal".into()]),
            make_cycle(4, 2, 0.6, vec!["thermo".into()]),
            make_cycle(5, 3, 0.7, vec!["wave".into()]),
            make_cycle(6, 4, 0.8, vec!["network".into()]),
        ];

        let new_config = opt.optimize_config(&config, &history);

        // Mutations should increase by φ=2
        assert_eq!(
            new_config.max_mutations_per_cycle,
            config.max_mutations_per_cycle + PHI as usize,
            "Expected mutations to increase by phi=2"
        );
    }

    #[test]
    fn test_bounds_are_respected() {
        let mut opt = MetaOptimizer::new();
        let mut config = EvolutionConfig::default();

        // Push all params to extremes
        config.serendipity_ratio = 0.01; // below min
        config.min_verification_score = 0.01; // below min
        config.max_mutations_per_cycle = 2; // below min

        // Scarce discoveries → would push everything down further
        let history: Vec<CycleResult> = (1..=6)
            .map(|i| make_cycle(i, 0, 0.0, vec!["consciousness".into()]))
            .collect();

        let new_config = opt.optimize_config(&config, &history);

        assert!(
            new_config.serendipity_ratio >= SERENDIPITY_MIN,
            "serendipity {} >= {}",
            new_config.serendipity_ratio,
            SERENDIPITY_MIN
        );
        assert!(
            new_config.min_verification_score >= VERIFICATION_MIN,
            "verification {} >= {}",
            new_config.min_verification_score,
            VERIFICATION_MIN
        );
        assert!(
            new_config.max_mutations_per_cycle >= MUTATIONS_MIN,
            "mutations {} >= {}",
            new_config.max_mutations_per_cycle,
            MUTATIONS_MIN
        );

        // Now push to upper extremes
        config.serendipity_ratio = 0.99;
        config.min_verification_score = 0.99;
        config.max_mutations_per_cycle = 50;

        // Abundant high-quality discoveries → would push everything up
        let history: Vec<CycleResult> = (1..=6)
            .map(|i| {
                make_cycle(
                    i,
                    5,
                    0.9,
                    vec!["consciousness".into(), "wave".into()],
                )
            })
            .collect();

        let new_config = opt.optimize_config(&config, &history);

        assert!(
            new_config.serendipity_ratio <= SERENDIPITY_MAX,
            "serendipity {} <= {}",
            new_config.serendipity_ratio,
            SERENDIPITY_MAX
        );
        assert!(
            new_config.min_verification_score <= VERIFICATION_MAX,
            "verification {} <= {}",
            new_config.min_verification_score,
            VERIFICATION_MAX
        );
        assert!(
            new_config.max_mutations_per_cycle <= MUTATIONS_MAX,
            "mutations {} <= {}",
            new_config.max_mutations_per_cycle,
            MUTATIONS_MAX
        );
    }

    #[test]
    fn test_optimization_count_increments() {
        let mut opt = MetaOptimizer::new();
        let config = EvolutionConfig::default();
        let history: Vec<CycleResult> = (1..=6)
            .map(|i| make_cycle(i, 1, 0.5, vec!["consciousness".into()]))
            .collect();

        assert_eq!(opt.optimization_count, 0);
        opt.optimize_config(&config, &history);
        assert_eq!(opt.optimization_count, 1);
        opt.optimize_config(&config, &history);
        assert_eq!(opt.optimization_count, 2);
    }

    #[test]
    fn test_serendipity_lenses_high_contribution() {
        let mut opt = MetaOptimizer::new();
        let mut config = EvolutionConfig::default();
        config.all_lenses = default_lenses();
        config.serendipity_ratio = 0.2;

        // All discovery cycles use serendipity lenses (beyond first 6)
        let history: Vec<CycleResult> = (1..=6)
            .map(|i| {
                make_cycle(
                    i,
                    2,
                    0.6,
                    // "wave" is the 7th lens → serendipity
                    vec!["consciousness".into(), "wave".into()],
                )
            })
            .collect();

        let new_config = opt.optimize_config(&config, &history);

        // Serendipity contributing well → ratio should increase
        assert!(
            new_config.serendipity_ratio > config.serendipity_ratio,
            "Expected serendipity ratio to increase: {} > {}",
            new_config.serendipity_ratio,
            config.serendipity_ratio
        );
    }
}
