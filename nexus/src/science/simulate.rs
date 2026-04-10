/// Monte Carlo simulation engine — run experiments in silico before real execution.

#[derive(Debug, Clone)]
pub struct SimulationConfig {
    pub experiment_type: String,
    pub target: String,
    pub n_simulations: usize,
    pub noise_level: f64,
    pub time_steps: usize,
}

#[derive(Debug, Clone)]
pub struct SimulationResult {
    pub mean_phi_delta: f64,
    pub std_phi_delta: f64,
    pub mean_entropy_delta: f64,
    pub percentile_95: f64,
    pub worst_case: f64,
    pub best_case: f64,
    pub convergence_step: Option<usize>,
}

/// Run Monte Carlo simulation for an experiment.
///
/// Uses a simple deterministic pseudo-random model (no external crate).
/// Each simulation run adds noise to a baseline model and tracks convergence.
pub fn simulate(config: &SimulationConfig) -> SimulationResult {
    let n = config.n_simulations.max(1);
    let steps = config.time_steps.max(1);
    let noise = config.noise_level;

    // Baseline model: experiment type determines the attractor
    let (base_phi, base_entropy) = baseline_for_type(&config.experiment_type);

    let mut phi_results: Vec<f64> = Vec::with_capacity(n);
    let mut entropy_results: Vec<f64> = Vec::with_capacity(n);

    // Simple LCG PRNG (deterministic, no external deps)
    let mut rng_state: u64 = 42 + config.target.len() as u64;

    for i in 0..n {
        let mut phi = base_phi;
        let mut entropy = base_entropy;

        for step in 0..steps {
            // LCG: next = (a * state + c) mod m
            rng_state = rng_state.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
            let r = ((rng_state >> 33) as f64) / (u32::MAX as f64) - 0.5; // uniform [-0.5, 0.5]

            // Apply noise perturbation
            phi += r * noise * base_phi.abs().max(1.0) / (step as f64 + 1.0);
            entropy += r * noise * 0.5 * base_entropy.abs().max(1.0) / (step as f64 + 1.0);
        }

        phi_results.push(phi);
        entropy_results.push(entropy);

        // Seed next run differently
        rng_state = rng_state.wrapping_add(i as u64 * 7 + 13);
    }

    // Statistics
    let mean_phi = phi_results.iter().sum::<f64>() / n as f64;
    let mean_entropy = entropy_results.iter().sum::<f64>() / n as f64;

    let var_phi: f64 = phi_results.iter().map(|x| (x - mean_phi).powi(2)).sum::<f64>() / n as f64;
    let std_phi = var_phi.sqrt();

    // Sort for percentiles
    let mut sorted_phi = phi_results.clone();
    sorted_phi.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));

    let p95_idx = ((n as f64 * 0.95) as usize).min(n - 1);
    let percentile_95 = sorted_phi[p95_idx];
    let worst_case = sorted_phi[0];
    let best_case = sorted_phi[n - 1];

    // Convergence detection: check if running mean stabilizes
    let convergence_step = detect_convergence(&phi_results, 0.01);

    SimulationResult {
        mean_phi_delta: mean_phi,
        std_phi_delta: std_phi,
        mean_entropy_delta: mean_entropy,
        percentile_95,
        worst_case,
        best_case,
        convergence_step,
    }
}

/// Get baseline attractor values for a given experiment type.
fn baseline_for_type(experiment_type: &str) -> (f64, f64) {
    match experiment_type {
        "tension" => (6.0, 12.0),       // n, sigma
        "fusion" => (24.0, 48.0),        // J2, sigma*tau
        "substitution" => (2.0, 4.0),    // phi, tau
        "doping" => (8.0, 10.0),         // sigma-tau, sigma-phi
        "phase" => (12.0, 24.0),         // sigma, J2
        _ => (6.0, 6.0),                 // default: n=6
    }
}

/// Detect when running mean stabilizes (relative change < threshold).
fn detect_convergence(values: &[f64], threshold: f64) -> Option<usize> {
    if values.len() < 6 {
        return None;
    }

    let mut running_sum = 0.0;
    let mut prev_mean: f64 = 0.0;

    for (i, &v) in values.iter().enumerate() {
        running_sum += v;
        let mean = running_sum / (i + 1) as f64;

        if i >= 5 {
            let rel_change = if prev_mean.abs() > 1e-12 {
                ((mean - prev_mean) / prev_mean).abs()
            } else {
                mean.abs()
            };

            if rel_change < threshold {
                return Some(i);
            }
        }
        prev_mean = mean;
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulate_basic() {
        let config = SimulationConfig {
            experiment_type: "tension".to_string(),
            target: "physics".to_string(),
            n_simulations: 100,
            noise_level: 0.1,
            time_steps: 10,
        };
        let result = simulate(&config);
        // Mean should be near baseline (6.0 for tension)
        assert!((result.mean_phi_delta - 6.0).abs() < 2.0, "mean_phi={}", result.mean_phi_delta);
        assert!(result.best_case >= result.worst_case);
        assert!(result.std_phi_delta >= 0.0);
    }

    #[test]
    fn test_simulate_deterministic() {
        let config = SimulationConfig {
            experiment_type: "tension".to_string(),
            target: "physics".to_string(),
            n_simulations: 50,
            noise_level: 0.05,
            time_steps: 5,
        };
        let r1 = simulate(&config);
        let r2 = simulate(&config);
        assert!((r1.mean_phi_delta - r2.mean_phi_delta).abs() < 1e-12);
    }

    #[test]
    fn test_convergence_detection() {
        // Stable series should converge
        let stable: Vec<f64> = (0..20).map(|i| 6.0 + 0.001 * (i as f64 - 10.0)).collect();
        assert!(detect_convergence(&stable, 0.01).is_some());

        // Too short series should not converge
        assert!(detect_convergence(&[1.0, 2.0], 0.01).is_none());
    }
}
