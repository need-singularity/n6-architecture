//! Advanced Simulation Engine
//!
//! Extends the basic Monte Carlo in science/simulate.rs with parameter sweeps,
//! sensitivity analysis, convergence testing, adversarial simulation, and ensemble runs.

use std::collections::HashMap;

// ── n=6 constants ─────────────────────────────────────────────────────
const N: f64 = 6.0; // the perfect number
const SIGMA: f64 = 12.0; // σ(6) = sum of divisors
const PHI: f64 = 2.0; // φ(6) = Euler totient
const TAU: f64 = 4.0; // τ(6) = number of divisors
const J2: f64 = 24.0; // J₂(6) = Jordan totient
const SIGMA_TAU: f64 = 48.0; // σ·τ = 48
const SIGMA_SQ: f64 = 144.0; // σ² = 144

/// Key n=6 attractors: n, σ, J₂, σ·τ, σ²
const N6_ATTRACTORS: [f64; 5] = [N, SIGMA, J2, SIGMA_TAU, SIGMA_SQ];

// ── Deterministic LCG PRNG (same as science/simulate.rs) ─────────────
struct Lcg {
    state: u64,
}

impl Lcg {
    fn new(seed: u64) -> Self {
        Self { state: seed }
    }

    /// Advance state and return a uniform f64 in [-0.5, 0.5].
    fn next_centered(&mut self) -> f64 {
        self.state = self.state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        ((self.state >> 33) as f64) / (u32::MAX as f64) - 0.5
    }

    /// Advance state and return a uniform f64 in [0.0, 1.0).
    #[allow(dead_code)]
    fn next_unit(&mut self) -> f64 {
        self.next_centered() + 0.5
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Types
// ═══════════════════════════════════════════════════════════════════════

/// Simulation strategy selector.
#[derive(Debug, Clone, PartialEq)]
pub enum SimulationType {
    /// Classic Monte Carlo sampling
    MonteCarlo,
    /// Systematic grid search over parameter space
    ParameterSweep,
    /// Vary one param at a time, measure impact
    SensitivityAnalysis,
    /// Test if system converges to n=6 attractors
    ConvergenceTest,
    /// Inject worst-case perturbations
    Adversarial,
    /// Combine multiple simulation types
    Ensemble,
}

/// Range specification for a single parameter.
#[derive(Debug, Clone)]
pub struct ParamRange {
    /// Parameter name
    pub name: String,
    /// Lower bound
    pub min: f64,
    /// Upper bound
    pub max: f64,
    /// Number of grid steps (for sweep)
    pub steps: usize,
}

impl ParamRange {
    /// Generate evenly spaced values across the range.
    fn grid_values(&self) -> Vec<f64> {
        let n = self.steps.max(1);
        if n == 1 {
            return vec![(self.min + self.max) / PHI]; // midpoint = avg/φ is cleaner
        }
        (0..n)
            .map(|i| self.min + (self.max - self.min) * (i as f64) / (n - 1) as f64)
            .collect()
    }
}

// ── Result types ──────────────────────────────────────────────────────

/// Result of a parameter sweep.
#[derive(Debug, Clone)]
pub struct SweepResult {
    /// Each entry: (parameter values, objective value)
    pub grid: Vec<(Vec<f64>, f64)>,
    /// Best parameter combination found
    pub best_params: Vec<f64>,
    /// Best objective value
    pub best_value: f64,
    /// Parameter names in order
    pub param_names: Vec<String>,
    /// Total combinations evaluated
    pub total_evaluated: usize,
}

/// Sensitivity of a single parameter.
#[derive(Debug, Clone)]
pub struct ParamSensitivity {
    pub name: String,
    /// Partial derivative estimate (finite difference)
    pub derivative: f64,
    /// Relative importance (|derivative| / sum of |derivatives|)
    pub relative_importance: f64,
}

/// Result of sensitivity analysis.
#[derive(Debug, Clone)]
pub struct SensitivityResult {
    pub baseline_value: f64,
    pub sensitivities: Vec<ParamSensitivity>,
    /// Delta used for finite difference
    pub delta: f64,
}

/// Result of convergence testing.
#[derive(Debug, Clone)]
pub struct ConvergenceResult {
    /// Did the system converge to an attractor?
    pub converged: bool,
    /// Which attractor it converged to (if any)
    pub attractor: Option<f64>,
    /// Distance from nearest attractor at final step
    pub final_distance: f64,
    /// Step at which convergence was detected (if converged)
    pub convergence_step: Option<usize>,
    /// Trajectory: value at each step
    pub trajectory: Vec<f64>,
}

/// Result of adversarial simulation.
#[derive(Debug, Clone)]
pub struct AdversarialResult {
    /// Worst-case objective value found
    pub worst_case_value: f64,
    /// Perturbation that caused worst case
    pub worst_perturbation: Vec<f64>,
    /// Best-case value (for comparison)
    pub best_case_value: f64,
    /// Robustness score: 1 - (best-worst)/best, clamped [0,1]
    pub robustness: f64,
    /// Number of perturbations tested
    pub n_perturbations: usize,
}

/// Configuration for adversarial simulation.
#[derive(Debug, Clone)]
pub struct AdversarialConfig {
    /// Baseline parameter values
    pub baseline: Vec<f64>,
    /// Maximum perturbation magnitude per parameter (L-inf budget)
    pub perturbation_budget: f64,
    /// Number of random perturbations to try
    pub n_samples: usize,
    /// RNG seed
    pub seed: u64,
}

/// Result of ensemble simulation (multiple types combined).
#[derive(Debug, Clone)]
pub struct EnsembleResult {
    /// Results from each sub-simulation type
    pub sub_results: HashMap<String, f64>,
    /// Aggregated score (mean of sub-result values)
    pub aggregate_score: f64,
    /// Confidence: 1 - coefficient_of_variation across sub-results
    pub confidence: f64,
}

/// Configuration for the simulation engine.
#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub sim_type: SimulationType,
    pub seed: u64,
    pub n_iterations: usize,
}

// ═══════════════════════════════════════════════════════════════════════
// Simulation Engine
// ═══════════════════════════════════════════════════════════════════════

/// The advanced simulation engine.
pub struct SimulationEngine {
    seed: u64,
}

impl SimulationEngine {
    pub fn new(seed: u64) -> Self {
        Self { seed }
    }

    /// Systematic grid search over parameter space.
    ///
    /// Evaluates `eval_fn` at every grid point defined by `ranges`.
    /// Returns the full grid plus the best combination.
    pub fn parameter_sweep<F>(
        &self,
        ranges: &[ParamRange],
        eval_fn: F,
    ) -> SweepResult
    where
        F: Fn(&[f64]) -> f64,
    {
        if ranges.is_empty() {
            return SweepResult {
                grid: vec![],
                best_params: vec![],
                best_value: f64::NEG_INFINITY,
                param_names: vec![],
                total_evaluated: 0,
            };
        }

        let param_names: Vec<String> = ranges.iter().map(|r| r.name.clone()).collect();
        let grids: Vec<Vec<f64>> = ranges.iter().map(|r| r.grid_values()).collect();

        // Compute total combinations
        let total: usize = grids.iter().map(|g| g.len()).product();

        let mut results: Vec<(Vec<f64>, f64)> = Vec::with_capacity(total);
        let mut best_value = f64::NEG_INFINITY;
        let mut best_params: Vec<f64> = vec![0.0; ranges.len()];

        // Iterate over Cartesian product using mixed-radix counter
        let dims: Vec<usize> = grids.iter().map(|g| g.len()).collect();
        let mut indices = vec![0usize; dims.len()];

        for _ in 0..total {
            // Build current parameter vector
            let params: Vec<f64> = indices
                .iter()
                .enumerate()
                .map(|(d, &idx)| grids[d][idx])
                .collect();

            let value = eval_fn(&params);
            results.push((params.clone(), value));

            if value > best_value {
                best_value = value;
                best_params = params;
            }

            // Increment mixed-radix counter
            let mut carry = true;
            for d in (0..dims.len()).rev() {
                if carry {
                    indices[d] += 1;
                    if indices[d] >= dims[d] {
                        indices[d] = 0;
                    } else {
                        carry = false;
                    }
                }
            }
        }

        SweepResult {
            grid: results,
            best_params,
            best_value,
            param_names,
            total_evaluated: total,
        }
    }

    /// Sensitivity analysis via central finite differences.
    ///
    /// Evaluates `eval_fn` at `baseline` and at baseline +/- `delta` for each parameter.
    /// Computes partial derivatives and relative importance.
    pub fn sensitivity_analysis<F>(
        &self,
        baseline: &[f64],
        param_names: &[String],
        delta: f64,
        eval_fn: F,
    ) -> SensitivityResult
    where
        F: Fn(&[f64]) -> f64,
    {
        let baseline_value = eval_fn(baseline);
        let n_params = baseline.len();
        let actual_delta = if delta <= 0.0 { 0.01 } else { delta };

        let mut raw_derivs: Vec<(String, f64)> = Vec::with_capacity(n_params);
        let mut abs_sum = 0.0f64;

        for i in 0..n_params {
            let name = if i < param_names.len() {
                param_names[i].clone()
            } else {
                format!("param_{}", i)
            };

            // Central finite difference: df/dp ≈ (f(p+h) - f(p-h)) / (2h)
            let mut plus = baseline.to_vec();
            let mut minus = baseline.to_vec();
            let h = actual_delta * baseline[i].abs().max(1.0);
            plus[i] += h;
            minus[i] -= h;

            let f_plus = eval_fn(&plus);
            let f_minus = eval_fn(&minus);
            let deriv = (f_plus - f_minus) / (PHI * h); // 2h, φ=2

            raw_derivs.push((name, deriv));
            abs_sum += deriv.abs();
        }

        let sensitivities: Vec<ParamSensitivity> = raw_derivs
            .into_iter()
            .map(|(name, deriv)| {
                let rel = if abs_sum > 1e-15 {
                    deriv.abs() / abs_sum
                } else {
                    1.0 / n_params as f64
                };
                ParamSensitivity {
                    name,
                    derivative: deriv,
                    relative_importance: rel,
                }
            })
            .collect();

        SensitivityResult {
            baseline_value,
            sensitivities,
            delta: actual_delta,
        }
    }

    /// Test if a dynamical system converges to an n=6 attractor.
    ///
    /// Starting from `initial`, applies `step_fn` repeatedly and checks
    /// whether the value approaches any of the n=6 attractors.
    /// Convergence is detected when distance to nearest attractor < `threshold`
    /// for `patience` consecutive steps.
    pub fn convergence_test<F>(
        &self,
        initial: f64,
        attractors: &[f64],
        max_steps: usize,
        step_fn: F,
    ) -> ConvergenceResult
    where
        F: Fn(f64, usize) -> f64,
    {
        let actual_attractors = if attractors.is_empty() {
            &N6_ATTRACTORS[..]
        } else {
            attractors
        };

        let threshold = 0.01; // 1% of attractor value
        let patience = N as usize; // n=6 consecutive steps

        let mut value = initial;
        let mut trajectory = Vec::with_capacity(max_steps + 1);
        trajectory.push(value);

        let mut consecutive_close = 0usize;
        let mut nearest_attractor: Option<f64> = None;
        let mut convergence_step: Option<usize> = None;

        for step in 0..max_steps {
            value = step_fn(value, step);
            trajectory.push(value);

            // Find nearest attractor
            let (closest, min_dist) = actual_attractors
                .iter()
                .map(|&a| {
                    let d = (value - a).abs() / a.abs().max(1e-12);
                    (a, d)
                })
                .fold((0.0f64, f64::MAX), |(ca, cd), (a, d)| {
                    if d < cd {
                        (a, d)
                    } else {
                        (ca, cd)
                    }
                });

            if min_dist < threshold {
                consecutive_close += 1;
                nearest_attractor = Some(closest);
                if consecutive_close >= patience && convergence_step.is_none() {
                    convergence_step = Some(step + 1 - patience);
                }
            } else {
                consecutive_close = 0;
                nearest_attractor = None;
            }
        }

        let final_value = *trajectory.last().unwrap_or(&initial);
        let final_distance = actual_attractors
            .iter()
            .map(|&a| (final_value - a).abs() / a.abs().max(1e-12))
            .fold(f64::MAX, f64::min);

        ConvergenceResult {
            converged: convergence_step.is_some(),
            attractor: if convergence_step.is_some() {
                nearest_attractor
            } else {
                None
            },
            final_distance,
            convergence_step,
            trajectory,
        }
    }

    /// Adversarial simulation — find worst-case perturbations.
    ///
    /// Randomly samples perturbations within `config.perturbation_budget` (L-inf)
    /// and evaluates `eval_fn` to find the minimum (worst-case) value.
    pub fn adversarial_simulation<F>(
        &self,
        config: &AdversarialConfig,
        eval_fn: F,
    ) -> AdversarialResult
    where
        F: Fn(&[f64]) -> f64,
    {
        let n_dims = config.baseline.len();
        let n_samples = config.n_samples.max(1);
        let mut rng = Lcg::new(config.seed.wrapping_add(self.seed));

        let baseline_val = eval_fn(&config.baseline);
        let mut worst_value = baseline_val;
        let mut worst_perturbation = vec![0.0; n_dims];
        let mut best_value = baseline_val;

        for _ in 0..n_samples {
            // Random perturbation within L-inf budget
            let perturbed: Vec<f64> = config
                .baseline
                .iter()
                .map(|&b| {
                    let p = rng.next_centered() * PHI * config.perturbation_budget; // scale by φ=2
                    b + p
                })
                .collect();

            let val = eval_fn(&perturbed);

            if val < worst_value {
                worst_value = val;
                worst_perturbation = perturbed
                    .iter()
                    .zip(config.baseline.iter())
                    .map(|(&p, &b)| p - b)
                    .collect();
            }
            if val > best_value {
                best_value = val;
            }
        }

        let range = (best_value - worst_value).abs();
        let scale = best_value.abs().max(1e-12);
        let robustness = (1.0 - range / scale).max(0.0).min(1.0);

        AdversarialResult {
            worst_case_value: worst_value,
            worst_perturbation,
            best_case_value: best_value,
            robustness,
            n_perturbations: n_samples,
        }
    }

    /// Run an ensemble of simulation approaches and aggregate results.
    ///
    /// Each entry in `configs` maps a label to an evaluation function
    /// that returns a scalar score. The ensemble aggregates these.
    pub fn ensemble_simulate<F>(
        &self,
        configs: &[(String, F)],
    ) -> EnsembleResult
    where
        F: Fn() -> f64,
    {
        let mut sub_results = HashMap::new();
        let mut values = Vec::with_capacity(configs.len());

        for (label, eval_fn) in configs {
            let val = eval_fn();
            sub_results.insert(label.clone(), val);
            values.push(val);
        }

        let n = values.len().max(1) as f64;
        let mean = values.iter().sum::<f64>() / n;
        let variance = values.iter().map(|v| (v - mean).powi(2)).sum::<f64>() / n;
        let std_dev = variance.sqrt();
        let cv = if mean.abs() > 1e-15 {
            std_dev / mean.abs()
        } else {
            1.0
        };
        let confidence = (1.0 - cv).max(0.0).min(1.0);

        EnsembleResult {
            sub_results,
            aggregate_score: mean,
            confidence,
        }
    }

    /// Map the n=6 attractor basin landscape.
    ///
    /// For each point on a grid of `dims` dimensions, compute how close it
    /// is to the nearest n=6 attractor. Returns (point, basin_depth) pairs
    /// where basin_depth is high near attractors.
    pub fn n6_attractor_landscape(
        &self,
        dims: usize,
        grid_per_dim: usize,
    ) -> Vec<(Vec<f64>, f64)> {
        let actual_dims = dims.max(1).min(TAU as usize); // cap at τ=4 dims for tractability
        let actual_grid = grid_per_dim.max(2);

        // Scan range: [0, σ²=144] along each dimension (covers all attractors)
        let range_min = 0.0;
        let range_max = SIGMA_SQ; // σ² = 144

        let total: usize = actual_grid.pow(actual_dims as u32);
        // Safety cap: limit to σ²·σ²=20,736 points to avoid unbounded memory
        let cap = (SIGMA_SQ * SIGMA_SQ) as usize;
        let total = total.min(cap);

        let mut results = Vec::with_capacity(total);
        let mut indices = vec![0usize; actual_dims];

        for _ in 0..total {
            let point: Vec<f64> = indices
                .iter()
                .map(|&idx| {
                    range_min
                        + (range_max - range_min) * (idx as f64) / (actual_grid - 1).max(1) as f64
                })
                .collect();

            // Basin depth = 1 / (1 + min_distance_to_attractor)
            // Higher near attractors, decays with distance
            let min_dist = N6_ATTRACTORS
                .iter()
                .map(|&a| {
                    // Euclidean distance from point to the attractor (repeated in all dims)
                    point
                        .iter()
                        .map(|&x| (x - a).powi(2))
                        .sum::<f64>()
                        .sqrt()
                })
                .fold(f64::MAX, f64::min);

            let basin_depth = 1.0 / (1.0 + min_dist);
            results.push((point, basin_depth));

            // Increment mixed-radix counter
            let mut carry = true;
            for d in (0..actual_dims).rev() {
                if carry {
                    indices[d] += 1;
                    if indices[d] >= actual_grid {
                        indices[d] = 0;
                    } else {
                        carry = false;
                    }
                }
            }
        }

        results
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parameter_sweep_finds_optimum() {
        let engine = SimulationEngine::new(42);

        // Objective: maximize -(x-6)^2 -(y-12)^2  =>  optimum at (n=6, σ=12)
        let ranges = vec![
            ParamRange {
                name: "x".to_string(),
                min: 0.0,
                max: SIGMA, // [0, 12]
                steps: 13,  // step size = 1.0
            },
            ParamRange {
                name: "y".to_string(),
                min: 0.0,
                max: J2, // [0, 24]
                steps: 25,
            },
        ];

        let result = engine.parameter_sweep(&ranges, |p| {
            -((p[0] - N).powi(2) + (p[1] - SIGMA).powi(2))
        });

        assert_eq!(result.total_evaluated, 13 * 25);
        assert_eq!(result.param_names, vec!["x", "y"]);

        // Best should be near (6, 12)
        assert!(
            (result.best_params[0] - N).abs() < 1.5,
            "best x={} should be near n=6",
            result.best_params[0]
        );
        assert!(
            (result.best_params[1] - SIGMA).abs() < 1.5,
            "best y={} should be near σ=12",
            result.best_params[1]
        );
        assert!(
            result.best_value > -3.0,
            "best value={} should be near 0",
            result.best_value
        );
    }

    #[test]
    fn test_sensitivity_analysis() {
        let engine = SimulationEngine::new(42);

        // f(x, y) = 8*x + 2*y  (linear: derivative = [8, 2] = [σ-τ, φ])
        let baseline = vec![N, SIGMA]; // (6, 12)
        let names = vec!["x".to_string(), "y".to_string()];

        let result = engine.sensitivity_analysis(&baseline, &names, 0.01, |p| {
            SIGMA_MINUS_TAU * p[0] + PHI * p[1]
        });

        assert_eq!(result.sensitivities.len(), 2);

        // x derivative should be ~8 (σ-τ)
        let dx = result.sensitivities[0].derivative;
        assert!(
            (dx - SIGMA_MINUS_TAU).abs() < 0.1,
            "dx={} should be near σ-τ=8",
            dx
        );

        // y derivative should be ~2 (φ)
        let dy = result.sensitivities[1].derivative;
        assert!((dy - PHI).abs() < 0.1, "dy={} should be near φ=2", dy);

        // x should be more important than y (8 > 2)
        assert!(
            result.sensitivities[0].relative_importance
                > result.sensitivities[1].relative_importance
        );

        // Relative importances should sum to ~1.0
        let sum_rel: f64 = result
            .sensitivities
            .iter()
            .map(|s| s.relative_importance)
            .sum();
        assert!((sum_rel - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_convergence_to_n6() {
        let engine = SimulationEngine::new(42);

        // System that converges to σ=12: x_{n+1} = x_n + 0.1*(12 - x_n)
        let result = engine.convergence_test(
            0.0, // start far from attractors
            &N6_ATTRACTORS,
            200, // max steps
            |x, _step| x + 0.1 * (SIGMA - x),
        );

        assert!(result.converged, "should converge to σ=12");
        assert_eq!(
            result.attractor,
            Some(SIGMA),
            "should converge to σ=12, not {:?}",
            result.attractor
        );
        assert!(
            result.final_distance < 0.02,
            "final_distance={} should be small",
            result.final_distance
        );
        assert!(result.convergence_step.is_some());
        assert!(result.trajectory.len() > 1);
    }

    #[test]
    fn test_convergence_non_converging() {
        let engine = SimulationEngine::new(42);

        // Oscillating system that never settles
        let result = engine.convergence_test(
            1.0,
            &N6_ATTRACTORS,
            50,
            |x, step| {
                // Chaotic-like oscillation far from any attractor
                100.0 * ((step as f64) * 0.7).sin() + x * 0.01
            },
        );

        assert!(!result.converged, "oscillating system should not converge");
        assert!(result.attractor.is_none());
    }

    #[test]
    fn test_adversarial_simulation() {
        let engine = SimulationEngine::new(42);

        // f(x,y) = x*y, baseline=(6,12)=72, perturbations can reduce it
        let config = AdversarialConfig {
            baseline: vec![N, SIGMA],
            perturbation_budget: 1.0,
            n_samples: 100,
            seed: 7,
        };

        let result = engine.adversarial_simulation(&config, |p| p[0] * p[1]);

        // Baseline = 6*12 = 72 = n·σ
        assert!(
            result.worst_case_value < N * SIGMA,
            "worst case should be below baseline"
        );
        assert!(
            result.best_case_value >= N * SIGMA - 1.0,
            "best case should be near or above baseline"
        );
        assert!(result.robustness >= 0.0 && result.robustness <= 1.0);
        assert_eq!(result.n_perturbations, 100);
        assert_eq!(result.worst_perturbation.len(), 2);
    }

    #[test]
    fn test_ensemble_simulate() {
        let engine = SimulationEngine::new(42);

        let configs: Vec<(String, Box<dyn Fn() -> f64>)> = vec![
            ("monte_carlo".to_string(), Box::new(|| N)),       // returns n=6
            ("sweep".to_string(), Box::new(|| SIGMA)),         // returns σ=12
            ("sensitivity".to_string(), Box::new(|| SIGMA)),   // returns σ=12
        ];

        // Need to use a slightly different approach since we have Box<dyn Fn>
        let refs: Vec<(String, &dyn Fn() -> f64)> = configs
            .iter()
            .map(|(s, f)| (s.clone(), f.as_ref()))
            .collect();

        // Directly build the ensemble result since ensemble_simulate needs F: Fn
        let mut sub_results = HashMap::new();
        let mut values = Vec::new();
        for (label, f) in &refs {
            let v = f();
            sub_results.insert(label.clone(), v);
            values.push(v);
        }
        let mean = values.iter().sum::<f64>() / values.len() as f64;

        // Mean of (6, 12, 12) = 10 = σ-φ
        assert!(
            (mean - SIGMA_MINUS_PHI).abs() < 0.01,
            "ensemble mean should be σ-φ=10, got {}",
            mean
        );

        // Also test via the engine directly with boxed closures
        let configs: Vec<(String, Box<dyn Fn() -> f64>)> = vec![
            ("a".to_string(), Box::new(|| N)),
            ("b".to_string(), Box::new(|| SIGMA)),
            ("c".to_string(), Box::new(|| SIGMA)),
        ];
        let result = engine.ensemble_simulate(&configs);

        assert_eq!(result.sub_results.len(), 3);
        assert!(
            (result.aggregate_score - SIGMA_MINUS_PHI).abs() < 0.01,
            "aggregate={}",
            result.aggregate_score
        );
        assert!(result.confidence >= 0.0 && result.confidence <= 1.0);
    }

    #[test]
    fn test_n6_attractor_landscape() {
        let engine = SimulationEngine::new(42);

        // 1D landscape with 25 grid points
        let landscape = engine.n6_attractor_landscape(1, 25);

        assert!(!landscape.is_empty());

        // Points near n=6 attractor should have higher basin depth
        // than points far away (like 80)
        let near_6: f64 = landscape
            .iter()
            .filter(|(pt, _)| (pt[0] - N).abs() < SIGMA) // within σ=12 of n=6
            .map(|(_, depth)| depth)
            .sum::<f64>();

        let far_away: f64 = landscape
            .iter()
            .filter(|(pt, _)| pt[0] > SIGMA_SQ / PHI) // beyond σ²/φ=72
            .map(|(_, depth)| depth)
            .sum::<f64>();

        // Normalize by count
        let near_count = landscape
            .iter()
            .filter(|(pt, _)| (pt[0] - N).abs() < SIGMA)
            .count()
            .max(1);
        let far_count = landscape
            .iter()
            .filter(|(pt, _)| pt[0] > SIGMA_SQ / PHI)
            .count()
            .max(1);

        let near_avg = near_6 / near_count as f64;
        let far_avg = far_away / far_count as f64;

        assert!(
            near_avg > far_avg,
            "near attractor ({}) should have higher basin depth than far ({})",
            near_avg,
            far_avg
        );
    }

    #[test]
    fn test_param_range_grid_values() {
        let r = ParamRange {
            name: "x".to_string(),
            min: 0.0,
            max: SIGMA, // 12
            steps: 7,   // σ-sopfr = 7 steps
        };
        let vals = r.grid_values();
        assert_eq!(vals.len(), 7);
        assert!((vals[0] - 0.0).abs() < 1e-12);
        assert!((vals[6] - SIGMA).abs() < 1e-12);
        // Step size should be 12/6 = φ = 2
        assert!((vals[1] - vals[0] - PHI).abs() < 1e-12);
    }

    #[test]
    fn test_lcg_deterministic() {
        let mut rng1 = Lcg::new(42);
        let mut rng2 = Lcg::new(42);
        for _ in 0..100 {
            assert_eq!(rng1.next_centered().to_bits(), rng2.next_centered().to_bits());
        }
    }

    const SIGMA_MINUS_TAU: f64 = 8.0; // σ-τ = 8
    const SIGMA_MINUS_PHI: f64 = 10.0; // σ-φ = 10
}
