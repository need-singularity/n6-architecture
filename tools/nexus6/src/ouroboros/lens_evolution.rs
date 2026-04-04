//! Lens Self-Evolution — Runtime mutation of lens parameters within OUROBOROS cycles.
//!
//! Instead of only mutating hypotheses, the OUROBOROS engine can now evolve
//! the lenses themselves by adjusting their tunable parameters (tolerance,
//! threshold, sensitivity) stored in `AdaptiveWeights::lens_params`.
//!
//! Three mutation strategies (all grounded in n=6 constants):
//!   1. **ParameterMutation**: perturb a lens's params by +/- lr range
//!   2. **CrossLensMutation**: transfer successful lens params to underperformers
//!   3. **AffinityMutation**: blend params of high-affinity lens pairs
//!
//! Integration: called from `engine.rs::evolve_step()` after verification,
//! before graph update / recommend.

use std::collections::HashMap;

use crate::adaptive::AdaptiveWeights;

// n=6 arithmetic constants
const SIGMA: f64 = 12.0;
const _PHI: f64 = 2.0;
const _TAU: f64 = 4.0;
const _N: f64 = 6.0;
const SIGMA_MINUS_PHI: f64 = 10.0;
const MUTATION_RANGE: f64 = 0.1; // 1/(sigma - phi) = 1/10
const GENERATION_CYCLE: usize = 12; // sigma = 12

/// Tunable parameter names that lens evolution can mutate.
const EVOLVABLE_PARAMS: &[(&str, f64, f64, f64)] = &[
    // (name, default, min, max)
    ("tolerance", 0.02, 0.001, 0.10),
    ("threshold_scale", 1.0, 0.0833, 12.0), // min=1/sigma, max=sigma
    ("sensitivity_boost", 1.0, 0.5, 6.0),   // min=0.5, max=n
];

/// Strategy selection for lens parameter mutation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensEvolutionStrategy {
    /// Perturb parameters by +/- lr within bounds.
    ParameterMutation,
    /// Transfer winning lens parameters to underperformers.
    CrossLensMutation,
    /// Blend parameters of high-affinity lens pairs.
    AffinityMutation,
}

/// Result of one lens evolution step.
#[derive(Debug, Clone)]
pub struct LensEvolutionResult {
    /// Number of lenses whose parameters were mutated.
    pub lenses_mutated: usize,
    /// Which strategies were applied.
    pub strategies_applied: Vec<LensEvolutionStrategy>,
    /// Per-lens parameter deltas (lens_name -> param_name -> delta).
    pub deltas: HashMap<String, HashMap<String, f64>>,
}

/// The Lens Evolver — drives runtime parameter mutation of lenses.
pub struct LensEvolver {
    /// Cycle counter (triggers generation-level events every sigma=12 cycles).
    generation_counter: usize,
}

impl LensEvolver {
    pub fn new() -> Self {
        Self {
            generation_counter: 0,
        }
    }

    /// Execute one lens evolution step.
    ///
    /// Reads scan outcomes from `adaptive_weights`, applies mutation strategies,
    /// and writes mutated parameters back into `adaptive_weights.lens_params`.
    ///
    /// Called from `evolve_step()` after scan+verify, before graph_update.
    pub fn evolve_lens_params(
        &mut self,
        adaptive_weights: &mut AdaptiveWeights,
        lenses_used: &[String],
        discovery_lenses: &[String],
    ) -> LensEvolutionResult {
        self.generation_counter += 1;

        let lr = adaptive_weights.learning_rate;
        let mut result = LensEvolutionResult {
            lenses_mutated: 0,
            strategies_applied: Vec::new(),
            deltas: HashMap::new(),
        };

        // Strategy 1: ParameterMutation — always applied
        let pm_count = self.apply_parameter_mutation(
            adaptive_weights,
            lenses_used,
            discovery_lenses,
            lr,
            &mut result.deltas,
        );
        if pm_count > 0 {
            result.lenses_mutated += pm_count;
            result.strategies_applied.push(LensEvolutionStrategy::ParameterMutation);
        }

        // Strategy 2: CrossLensMutation — applied when there are both winners and losers
        if !discovery_lenses.is_empty() {
            let non_discovery: Vec<&String> = lenses_used
                .iter()
                .filter(|l| !discovery_lenses.contains(l))
                .collect();
            if !non_discovery.is_empty() {
                let cl_count = self.apply_cross_lens_mutation(
                    adaptive_weights,
                    discovery_lenses,
                    &non_discovery,
                    lr,
                    &mut result.deltas,
                );
                if cl_count > 0 {
                    result.lenses_mutated += cl_count;
                    result
                        .strategies_applied
                        .push(LensEvolutionStrategy::CrossLensMutation);
                }
            }
        }

        // Strategy 3: AffinityMutation — every generation (sigma=12 cycles)
        if self.generation_counter % GENERATION_CYCLE == 0 {
            let af_count = self.apply_affinity_mutation(
                adaptive_weights,
                lr,
                &mut result.deltas,
            );
            if af_count > 0 {
                result.lenses_mutated += af_count;
                result
                    .strategies_applied
                    .push(LensEvolutionStrategy::AffinityMutation);
            }
        }

        result
    }

    /// Strategy 1: ParameterMutation
    ///
    /// For each lens used this cycle, perturb evolvable parameters.
    /// - Discovery lenses: tighten (reduce tolerance, boost threshold)
    /// - Non-discovery lenses: widen (increase tolerance, reduce threshold)
    /// - Mutation magnitude = lr * MUTATION_RANGE = lr * 0.1
    fn apply_parameter_mutation(
        &self,
        weights: &mut AdaptiveWeights,
        lenses_used: &[String],
        discovery_lenses: &[String],
        lr: f64,
        deltas: &mut HashMap<String, HashMap<String, f64>>,
    ) -> usize {
        let mut count = 0;
        let magnitude = lr * MUTATION_RANGE;

        for lens in lenses_used {
            let contributed = discovery_lenses.contains(lens);
            let lens_deltas = deltas.entry(lens.clone()).or_default();
            let mut mutated = false;

            for &(param_name, default, min, max) in EVOLVABLE_PARAMS {
                let old_val = weights.param(lens, param_name, default);

                let delta = if contributed {
                    // Reward direction: tighten tolerance, boost threshold/sensitivity
                    match param_name {
                        "tolerance" => -magnitude * old_val, // tighten
                        _ => magnitude * old_val,            // boost
                    }
                } else if !discovery_lenses.is_empty() {
                    // Penalty direction: loosen tolerance, reduce threshold/sensitivity
                    match param_name {
                        "tolerance" => magnitude * old_val * 0.5, // loosen (gentler)
                        _ => -magnitude * old_val * 0.3,          // reduce (gentler)
                    }
                } else {
                    // No signal: tiny drift toward default (homeostasis)
                    (default - old_val) * magnitude * 0.1
                };

                let new_val = (old_val + delta).clamp(min, max);
                if (new_val - old_val).abs() > 1e-12 {
                    weights.set_param(lens, param_name, new_val);
                    lens_deltas.insert(param_name.to_string(), new_val - old_val);
                    mutated = true;
                }
            }

            if mutated {
                count += 1;
            }
        }

        count
    }

    /// Strategy 2: CrossLensMutation
    ///
    /// Transfer parameters from discovery (winning) lenses to non-discovery
    /// (underperforming) lenses. The transfer is partial: the loser lens moves
    /// a fraction (lr) toward the winner's parameter values.
    fn apply_cross_lens_mutation(
        &self,
        weights: &mut AdaptiveWeights,
        winners: &[String],
        losers: &[&String],
        lr: f64,
        deltas: &mut HashMap<String, HashMap<String, f64>>,
    ) -> usize {
        if winners.is_empty() {
            return 0;
        }

        // Compute average parameters across winning lenses
        let mut avg_params: HashMap<String, (f64, usize)> = HashMap::new();
        for winner in winners {
            for &(param_name, default, _, _) in EVOLVABLE_PARAMS {
                let val = weights.param(winner, param_name, default);
                let entry = avg_params.entry(param_name.to_string()).or_insert((0.0, 0));
                entry.0 += val;
                entry.1 += 1;
            }
        }

        let mut count = 0;
        for loser in losers {
            let lens_deltas = deltas.entry(loser.to_string()).or_default();
            let mut mutated = false;

            for &(param_name, default, min, max) in EVOLVABLE_PARAMS {
                let loser_val = weights.param(loser, param_name, default);
                let winner_avg = avg_params
                    .get(param_name)
                    .map(|(sum, n)| sum / *n as f64)
                    .unwrap_or(default);

                // Move loser toward winner by lr fraction
                let delta = (winner_avg - loser_val) * lr;
                let new_val = (loser_val + delta).clamp(min, max);

                if (new_val - loser_val).abs() > 1e-12 {
                    weights.set_param(loser, param_name, new_val);
                    lens_deltas.insert(param_name.to_string(), new_val - loser_val);
                    mutated = true;
                }
            }

            if mutated {
                count += 1;
            }
        }

        count
    }

    /// Strategy 3: AffinityMutation
    ///
    /// For high-affinity lens pairs (co-discovered frequently), blend their
    /// parameters toward each other. This reinforces synergistic lens
    /// configurations that the system has learned work well together.
    ///
    /// Triggered every sigma=12 cycles (one "generation").
    fn apply_affinity_mutation(
        &self,
        weights: &mut AdaptiveWeights,
        lr: f64,
        deltas: &mut HashMap<String, HashMap<String, f64>>,
    ) -> usize {
        // Collect high-affinity pairs (affinity > 0.5)
        let affinity_threshold = 0.5;
        let high_affinity_pairs: Vec<(String, String, f64)> = weights
            .lens_affinity
            .iter()
            .filter(|(_, &aff)| aff > affinity_threshold)
            .filter_map(|(key, &aff)| {
                let parts: Vec<&str> = key.split(':').collect();
                if parts.len() == 2 {
                    Some((parts[0].to_string(), parts[1].to_string(), aff))
                } else {
                    None
                }
            })
            .collect();

        let mut count = 0;
        for (lens_a, lens_b, affinity) in &high_affinity_pairs {
            // Blend strength proportional to affinity and lr
            let blend = lr * (affinity / SIGMA_MINUS_PHI); // normalize by sigma-phi=10

            for &(param_name, default, min, max) in EVOLVABLE_PARAMS {
                let val_a = weights.param(lens_a, param_name, default);
                let val_b = weights.param(lens_b, param_name, default);

                // Move each toward the midpoint
                let midpoint = (val_a + val_b) / 2.0;

                let delta_a = (midpoint - val_a) * blend;
                let new_a = (val_a + delta_a).clamp(min, max);
                if (new_a - val_a).abs() > 1e-12 {
                    weights.set_param(lens_a, param_name, new_a);
                    deltas
                        .entry(lens_a.clone())
                        .or_default()
                        .insert(param_name.to_string(), new_a - val_a);
                }

                let delta_b = (midpoint - val_b) * blend;
                let new_b = (val_b + delta_b).clamp(min, max);
                if (new_b - val_b).abs() > 1e-12 {
                    weights.set_param(lens_b, param_name, new_b);
                    deltas
                        .entry(lens_b.clone())
                        .or_default()
                        .insert(param_name.to_string(), new_b - val_b);
                }
            }

            count += 1;
        }

        count
    }
}

impl Default for LensEvolver {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_mutation_rewards_discovery_lenses() {
        let mut weights = AdaptiveWeights::default();
        let mut evolver = LensEvolver::new();

        let lenses = vec!["consciousness".to_string(), "topology".to_string()];
        let discoveries = vec!["consciousness".to_string()];

        let result = evolver.evolve_lens_params(&mut weights, &lenses, &discoveries);

        assert!(result.lenses_mutated > 0);
        assert!(result
            .strategies_applied
            .contains(&LensEvolutionStrategy::ParameterMutation));

        // consciousness found discoveries -> tolerance should decrease (tighten)
        let cons_tol = weights.param("consciousness", "tolerance", 0.02);
        assert!(
            cons_tol < 0.02,
            "Discovery lens tolerance should tighten: got {}",
            cons_tol
        );

        // topology did not contribute -> tolerance should increase (loosen)
        let topo_tol = weights.param("topology", "tolerance", 0.02);
        assert!(
            topo_tol > 0.02,
            "Non-discovery lens tolerance should loosen: got {}",
            topo_tol
        );
    }

    #[test]
    fn test_cross_lens_mutation_transfers_params() {
        let mut weights = AdaptiveWeights::default();
        let mut evolver = LensEvolver::new();

        // Pre-set winner lens with specific params
        weights.set_param("winner_lens", "tolerance", 0.005);
        weights.set_param("winner_lens", "threshold_scale", 3.0);

        // Loser lens starts at defaults
        // tolerance=0.02, threshold_scale=1.0

        let lenses = vec!["winner_lens".to_string(), "loser_lens".to_string()];
        let discoveries = vec!["winner_lens".to_string()];

        let result = evolver.evolve_lens_params(&mut weights, &lenses, &discoveries);

        assert!(result
            .strategies_applied
            .contains(&LensEvolutionStrategy::CrossLensMutation));

        // Loser's threshold_scale should move toward winner's 3.0 (increase from ~1.0)
        let loser_ts = weights.param("loser_lens", "threshold_scale", 1.0);
        // After ParameterMutation (penalty: reduce) + CrossLensMutation (toward 3.0),
        // net effect should still move toward winner
        // The cross-lens mutation moves by lr * (winner - loser) fraction
        assert!(
            loser_ts != 1.0,
            "Loser threshold_scale should have been mutated"
        );
    }

    #[test]
    fn test_affinity_mutation_triggers_at_generation_boundary() {
        let mut weights = AdaptiveWeights::default();
        let mut evolver = LensEvolver::new();

        // Set up high-affinity pair
        weights
            .lens_affinity
            .insert("alpha:beta".to_string(), 0.8);

        // Set divergent params
        weights.set_param("alpha", "tolerance", 0.01);
        weights.set_param("beta", "tolerance", 0.05);

        let lenses = vec!["alpha".to_string(), "beta".to_string()];

        // Run 11 cycles without affinity trigger
        for _ in 0..11 {
            let result = evolver.evolve_lens_params(&mut weights, &lenses, &[]);
            assert!(
                !result
                    .strategies_applied
                    .contains(&LensEvolutionStrategy::AffinityMutation),
                "Affinity should not trigger before generation boundary"
            );
        }

        // Cycle 12 = sigma = generation boundary -> affinity triggers
        let result = evolver.evolve_lens_params(&mut weights, &lenses, &[]);
        assert!(
            result
                .strategies_applied
                .contains(&LensEvolutionStrategy::AffinityMutation),
            "Affinity should trigger at cycle 12 (sigma generation)"
        );

        // Alpha and beta tolerances should converge toward midpoint
        let alpha_tol = weights.param("alpha", "tolerance", 0.01);
        let beta_tol = weights.param("beta", "tolerance", 0.05);
        // Midpoint was 0.03, so alpha should increase and beta should decrease
        assert!(
            alpha_tol > 0.01,
            "Alpha tolerance should move toward midpoint: got {}",
            alpha_tol
        );
        assert!(
            beta_tol < 0.05,
            "Beta tolerance should move toward midpoint: got {}",
            beta_tol
        );
    }

    #[test]
    fn test_no_mutation_when_no_lenses() {
        let mut weights = AdaptiveWeights::default();
        let mut evolver = LensEvolver::new();

        let result = evolver.evolve_lens_params(&mut weights, &[], &[]);
        assert_eq!(result.lenses_mutated, 0);
        assert!(result.strategies_applied.is_empty());
    }

    #[test]
    fn test_parameter_bounds_respected() {
        let mut weights = AdaptiveWeights::default();
        let mut evolver = LensEvolver::new();

        // Set params at extremes
        weights.set_param("extreme", "tolerance", 0.001); // at min
        weights.set_param("extreme", "threshold_scale", 12.0); // at max

        let lenses = vec!["extreme".to_string()];
        let discoveries = vec!["extreme".to_string()];

        // Even with rewards, params should stay within bounds
        for _ in 0..100 {
            evolver.evolve_lens_params(&mut weights, &lenses, &discoveries);
        }

        let tol = weights.param("extreme", "tolerance", 0.02);
        let ts = weights.param("extreme", "threshold_scale", 1.0);

        assert!(tol >= 0.001, "Tolerance below minimum: {}", tol);
        assert!(tol <= 0.10, "Tolerance above maximum: {}", tol);
        assert!(ts >= 0.0833, "Threshold below minimum: {}", ts);
        assert!(ts <= 12.0, "Threshold above maximum: {}", ts);
    }
}
