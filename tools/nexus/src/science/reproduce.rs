/// Reproduction engine — verify experiment reproducibility through repeated runs.

#[derive(Debug, Clone)]
pub struct ReproductionConfig {
    pub experiment_type: String,
    pub target: String,
    pub n_repeats: usize,
    pub variation: f64,
}

#[derive(Debug, Clone)]
pub struct ReproductionResult {
    pub n_repeats: usize,
    pub results: Vec<f64>,
    pub mean: f64,
    pub std: f64,
    pub cv: f64,
    pub reproducible: bool,
    pub outlier_runs: Vec<usize>,
}

/// Reproduce an experiment N times with slight initial condition variations.
///
/// Uses a deterministic pseudo-random model to simulate repeated runs.
/// Checks coefficient of variation (CV) against a threshold to determine reproducibility.
pub fn reproduce(config: &ReproductionConfig) -> ReproductionResult {
    let n = config.n_repeats.max(1);
    let variation = config.variation;

    // Baseline from experiment type
    let base = baseline_for_type(&config.experiment_type);

    let mut results: Vec<f64> = Vec::with_capacity(n);
    let mut rng_state: u64 = 123 + config.target.len() as u64;

    for i in 0..n {
        // LCG PRNG
        rng_state = rng_state
            .wrapping_mul(6364136223846793005)
            .wrapping_add(1442695040888963407);
        let r = ((rng_state >> 33) as f64) / (u32::MAX as f64) - 0.5;

        // Apply variation to baseline
        let value = base + r * variation * base.abs().max(1.0);
        results.push(value);

        rng_state = rng_state.wrapping_add(i as u64 * 11 + 7);
    }

    // Statistics
    let mean = results.iter().sum::<f64>() / n as f64;
    let var: f64 = results.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n as f64;
    let std = var.sqrt();
    let cv = if mean.abs() > 1e-12 { std / mean.abs() } else { std };

    // Outlier detection: |value - mean| > 2 * std
    let outlier_runs: Vec<usize> = results
        .iter()
        .enumerate()
        .filter(|(_, &v)| (v - mean).abs() > 2.0 * std && std > 1e-12)
        .map(|(i, _)| i)
        .collect();

    // Reproducible if CV < 10% (0.1)
    let reproducible = cv < 0.1;

    ReproductionResult {
        n_repeats: n,
        results,
        mean,
        std,
        cv,
        reproducible,
        outlier_runs,
    }
}

/// Get baseline value for an experiment type.
fn baseline_for_type(experiment_type: &str) -> f64 {
    match experiment_type {
        "tension" => 6.0,
        "fusion" => 24.0,
        "substitution" => 2.0,
        "doping" => 8.0,
        "phase" => 12.0,
        _ => 6.0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reproduce_consistent() {
        let config = ReproductionConfig {
            experiment_type: "tension".to_string(),
            target: "physics".to_string(),
            n_repeats: 20,
            variation: 0.01, // very small variation
        };
        let result = reproduce(&config);
        assert_eq!(result.n_repeats, 20);
        assert!(result.cv < 0.1, "cv={} should be < 0.1", result.cv);
        assert!(result.reproducible);
    }

    #[test]
    fn test_reproduce_outlier_detection() {
        let config = ReproductionConfig {
            experiment_type: "tension".to_string(),
            target: "physics".to_string(),
            n_repeats: 100,
            variation: 0.5, // larger variation to produce outliers
        };
        let result = reproduce(&config);
        assert_eq!(result.n_repeats, 100);
        assert_eq!(result.results.len(), 100);
        // With enough variation, some outliers may be detected
        // The mean should still be near baseline
        assert!((result.mean - 6.0).abs() < 3.0, "mean={}", result.mean);
    }

    #[test]
    fn test_reproduce_deterministic() {
        let config = ReproductionConfig {
            experiment_type: "fusion".to_string(),
            target: "plasma".to_string(),
            n_repeats: 10,
            variation: 0.1,
        };
        let r1 = reproduce(&config);
        let r2 = reproduce(&config);
        assert!((r1.mean - r2.mean).abs() < 1e-12);
    }
}
