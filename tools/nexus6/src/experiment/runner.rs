use crate::telescope::registry::LensRegistry;
use super::types::{
    ExperimentConfig, ExperimentMetrics, ExperimentResult, ExperimentType,
    ALL_EXPERIMENT_TYPES,
};

/// The experiment runner: executes experiments on synthetic data
/// and measures metrics before/after manipulation.
pub struct ExperimentRunner {
    _registry: LensRegistry,
}

impl ExperimentRunner {
    pub fn new() -> Self {
        Self {
            _registry: LensRegistry::new(),
        }
    }

    /// Run a single experiment with the given configuration.
    pub fn run(&self, config: &ExperimentConfig) -> ExperimentResult {
        // Generate synthetic probe data for the target
        let data = generate_probe_data(&config.target);

        // 1. Measure before
        let before = measure_metrics(&data);

        // 2. Apply experiment-specific manipulation
        let (modified, breakpoint) = self.apply_experiment(config, &data);

        // 3. Measure after
        let after = measure_metrics(&modified);

        // 4. Compute delta
        let delta = after.delta(&before);

        // 5. Detect discoveries
        let discoveries = detect_discoveries(config.exp_type, &before, &after, breakpoint);

        ExperimentResult {
            exp_type: config.exp_type,
            before,
            after,
            delta,
            breakpoint,
            discoveries,
        }
    }

    /// Run all 22 experiment types on the given target.
    pub fn run_all(&self, target: &str) -> Vec<ExperimentResult> {
        ALL_EXPERIMENT_TYPES
            .iter()
            .map(|&exp_type| {
                let config = ExperimentConfig::new(exp_type, target);
                self.run(&config)
            })
            .collect()
    }

    /// Run a selected battery of experiment types on the target.
    pub fn run_battery(&self, types: &[ExperimentType], target: &str) -> Vec<ExperimentResult> {
        types
            .iter()
            .map(|&exp_type| {
                let config = ExperimentConfig::new(exp_type, target);
                self.run(&config)
            })
            .collect()
    }

    /// Apply the experiment-type-specific manipulation to data.
    /// Returns (modified_data, optional_breakpoint).
    fn apply_experiment(&self, config: &ExperimentConfig, data: &[f64]) -> (Vec<f64>, Option<f64>) {
        let intensity = config.intensity;
        let duration = config.duration;

        match config.exp_type {
            ExperimentType::Acceleration => (apply_acceleration(data, intensity), None),
            ExperimentType::Collision => (apply_collision(data, intensity), None),
            ExperimentType::Separation => (apply_separation(data, intensity), None),
            ExperimentType::Fusion => (apply_fusion(data, intensity), None),
            ExperimentType::Reversal => (apply_reversal(data), None),
            ExperimentType::Destruction => (apply_destruction(data, intensity), None),
            ExperimentType::Amplification => (apply_amplification(data, intensity), None),
            ExperimentType::Suppression => (apply_suppression(data, intensity), None),
            ExperimentType::Mutation => (apply_mutation(data, intensity), None),
            ExperimentType::Crossover => (apply_crossover(data, intensity), None),
            ExperimentType::Isolation => (apply_isolation(data, intensity), None),
            ExperimentType::Overload => (apply_overload(data, duration), None),
            ExperimentType::Starvation => (apply_starvation(data, intensity), None),
            ExperimentType::TimeWarp => (apply_time_warp(data, intensity), None),
            ExperimentType::DimensionShift => (apply_dimension_shift(data, intensity), None),
            ExperimentType::SymmetryBreaking => (apply_symmetry_breaking(data, intensity), None),
            ExperimentType::Resonance => (apply_resonance(data, intensity), None),
            ExperimentType::Tension => {
                let (modified, bp) = apply_tension(data, intensity, duration);
                (modified, Some(bp))
            }
            ExperimentType::Compression => (apply_compression(data, intensity), None),
            ExperimentType::Vibration => (apply_vibration(data, intensity), None),
            ExperimentType::Elasticity => (apply_elasticity(data, intensity), None),
            ExperimentType::Friction => (apply_friction(data, intensity), None),
        }
    }
}

impl Default for ExperimentRunner {
    fn default() -> Self {
        Self::new()
    }
}

// ─── Domain-Specific Probe Data ─────────────────────────────────

/// Domain-specific seed data profiles.
/// Each domain uses characteristic values from its actual field,
/// producing genuinely different experiment outcomes.
///
/// Returns (base_values, intensity_bias, variability).
///   intensity_bias: shifts default experiment intensity per domain
///   variability: controls hash-noise amplitude (fraction of value)
fn domain_profile(target: &str) -> (&'static [f64], f64, f64) {
    let t = target.to_lowercase();

    // Physics / fundamental constants
    if t.contains("physics") || t.contains("particle") || t.contains("cosmology") {
        static P: [f64; 12] = [0.511, 105.7, 135.0, 938.3, 137.036, 0.231,
                                91.2, 125.1, 2.725, 13.8, 1.616e-10, 6.674];
        return (&P, 0.0, 0.05);
    }

    // Biology / genetics
    if t.contains("biology") || t.contains("bio") || t.contains("genetic") {
        static P: [f64; 12] = [4.0, 64.0, 20.0, 3.0, 23.0, 3.4,
                                2.0, 10.0, 46.0, 6.0, 1.5, 30000.0];
        return (&P, 0.1, 0.15);
    }

    // Chip architecture / semiconductor
    if t.contains("chip") || t.contains("semiconductor") || t.contains("gpu") || t.contains("cpu") {
        static P: [f64; 12] = [144.0, 132.0, 80.0, 192.0, 288.0, 48.0,
                                28.0, 7.0, 5.0, 3.0, 16384.0, 1024.0];
        return (&P, -0.1, 0.02);
    }

    // Energy / power grid / solar
    if t.contains("energy") || t.contains("power") || t.contains("grid") || t.contains("solar") {
        static P: [f64; 12] = [50.0, 60.0, 6.0, 12.0, 24.0, 72.0,
                                144.0, 1.34, 1.2, 120.0, 480.0, 48.0];
        return (&P, 0.05, 0.08);
    }

    // Battery
    if t.contains("battery") || t.contains("cathode") || t.contains("anode") {
        static P: [f64; 12] = [6.0, 3.6, 3.7, 4.2, 372.0, 3579.0,
                                3860.0, 96.0, 192.0, 24.0, 12.0, 48.0];
        return (&P, 0.15, 0.1);
    }

    // Fusion / plasma / tokamak
    if t.contains("fusion") || t.contains("plasma") || t.contains("tokamak") {
        static P: [f64; 12] = [17.6, 6.2, 2.0, 5.3, 10.0, 150.0,
                                1.0e8, 3.0, 0.1, 100.0, 15.0, 1.0];
        return (&P, 0.2, 0.12);
    }

    // Consciousness / IIT
    if t.contains("consciousness") || t.contains("iit") || t.contains("phi") {
        static P: [f64; 12] = [0.3, 0.8, 1.5, 3.0, 2.718, 1.618,
                                0.618, 6.0, 12.0, 3.14159, 0.5, 1.0];
        return (&P, 0.0, 0.2);
    }

    // Quantum computing
    if t.contains("quantum") {
        static P: [f64; 12] = [1000.0, 0.999, 0.9999, 100.0, 0.001, 17.0,
                                7.0, 3.0, 72.0, 53.0, 127.0, 433.0];
        return (&P, -0.05, 0.03);
    }

    // Superconductor
    if t.contains("superconductor") || t.contains("magnet") {
        static P: [f64; 12] = [39.0, 93.0, 110.0, 203.0, 250.0, 15.0,
                                45.0, 100.0, 0.16, 0.3, 4.2, 77.0];
        return (&P, 0.1, 0.07);
    }

    // Robotics
    if t.contains("robot") || t.contains("robotics") {
        static P: [f64; 12] = [6.0, 5.0, 4.0, 12.0, 32.0, 2.0,
                                96.97, 0.1, 1000.0, 50.0, 6.0, 3.0];
        return (&P, 0.05, 0.1);
    }

    // Blockchain / crypto
    if t.contains("blockchain") || t.contains("crypto") || t.contains("bitcoin") {
        static P: [f64; 12] = [21000000.0, 6.0, 10.0, 2016.0, 12.0, 32.0,
                                256.0, 128.0, 64.0, 2048.0, 0.5, 1.0];
        return (&P, -0.15, 0.05);
    }

    // Network / protocol
    if t.contains("network") || t.contains("protocol") || t.contains("tcp") {
        static P: [f64; 12] = [7.0, 4.0, 1500.0, 65535.0, 100.0, 0.5,
                                13.0, 80.0, 443.0, 8080.0, 53.0, 22.0];
        return (&P, 0.0, 0.06);
    }

    // Display / audio
    if t.contains("display") || t.contains("audio") || t.contains("music") {
        static P: [f64; 12] = [12.0, 24.0, 48000.0, 440.0, 8.0, 16.0,
                                60.0, 120.0, 144.0, 96000.0, 44100.0, 32.0];
        return (&P, 0.1, 0.15);
    }

    // Software design
    if t.contains("software") || t.contains("compiler") || t.contains("os") || t.contains("programming") {
        static P: [f64; 12] = [5.0, 6.0, 12.0, 4.0, 23.0, 11.0,
                                9.0, 7.0, 3.0, 8.0, 256.0, 1024.0];
        return (&P, -0.1, 0.04);
    }

    // Pure mathematics
    if t.contains("math") || t.contains("number") || t.contains("algebra") {
        static P: [f64; 12] = [3.14159, 2.71828, 1.61803, 1.41421, 0.57722, 0.91597,
                                1.20206, 6.0, 28.0, 496.0, 12.0, 24.0];
        return (&P, 0.0, 0.18);
    }

    // Environment / ecology
    if t.contains("environment") || t.contains("ecology") || t.contains("climate") {
        static P: [f64; 12] = [6.0, 420.0, 12.0, 7.0, 1.5, 280.0,
                                350.0, 8.0, 14.0, 1000.0, 100.0, 36.0];
        return (&P, 0.1, 0.12);
    }

    // Material synthesis
    if t.contains("material") || t.contains("synthesis") {
        static P: [f64; 12] = [6.0, 12.011, 3.567, 1.42, 348.0, 614.0,
                                4.36, 0.154, 2.46, 120.0, 109.5, 6.0];
        return (&P, 0.05, 0.09);
    }

    // Cryptography
    if t.contains("cryptograph") {
        static P: [f64; 12] = [128.0, 192.0, 256.0, 2048.0, 4096.0, 512.0,
                                160.0, 384.0, 521.0, 32.0, 48.0, 64.0];
        return (&P, -0.1, 0.03);
    }

    // Learning algorithm / AI training
    if t.contains("learning") || t.contains("training") || t.contains("ai-efficiency") || t.contains("ai") {
        static P: [f64; 12] = [0.001, 8192.0, 12.0, 96.0, 4096.0, 0.1,
                                2000.0, 0.9, 0.95, 128.0, 32000.0, 0.288];
        return (&P, 0.05, 0.08);
    }

    // Default: generic n=6 profile (fallback — still hash-differentiated)
    static DEFAULT: [f64; 12] = [6.0, 12.0, 24.0, 4.0, 2.0, 5.0,
                                  8.0, 10.0, 1.0, 3.0, 7.0, 48.0];
    (&DEFAULT, 0.0, 0.1)
}

/// Generate domain-specific probe data from a target string.
/// Each domain uses characteristic values reflecting its actual properties,
/// ensuring experiments produce genuinely different patterns per domain.
fn generate_probe_data(target: &str) -> Vec<f64> {
    let seed: u64 = target.bytes().fold(0u64, |acc, b| acc.wrapping_mul(31).wrapping_add(b as u64));
    let (profile, _intensity_bias, variability) = domain_profile(target);

    let profile_len = profile.len();
    let data_len = profile_len.max(24); // At least J_2=24 data points
    let mut data = Vec::with_capacity(data_len);

    for i in 0..data_len {
        let base = profile[i % profile_len];
        // Scale-relative noise: magnitude proportional to value scale
        let hash_val = seed.wrapping_mul(6364136223846793005).wrapping_add((i as u64).wrapping_mul(1442695040888963407));
        let noise_frac = ((hash_val % 10000) as f64 / 10000.0 - 0.5) * 2.0 * variability;
        data.push(base * (1.0 + noise_frac));
    }
    data
}

// ─── Metrics Measurement ────────────────────────────────────────

/// Compute experiment metrics from a data vector.
fn measure_metrics(data: &[f64]) -> ExperimentMetrics {
    if data.is_empty() {
        return ExperimentMetrics::zero();
    }
    let n = data.len() as f64;
    let mean = data.iter().sum::<f64>() / n;

    // Phi: integrated information proxy (variance-based)
    let variance = data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n;
    let phi = variance.sqrt();

    // Entropy: Shannon-like from normalized values
    let sum_abs: f64 = data.iter().map(|x| x.abs()).sum();
    let entropy = if sum_abs > 0.0 {
        data.iter()
            .map(|x| {
                let p = x.abs() / sum_abs;
                if p > 1e-15 { -p * p.ln() } else { 0.0 }
            })
            .sum::<f64>()
    } else {
        0.0
    };

    // Connectivity: autocorrelation at lag 1
    let connectivity = if data.len() > 1 {
        let pairs: f64 = data.windows(2)
            .map(|w| (w[0] - mean) * (w[1] - mean))
            .sum();
        let denom = variance * (n - 1.0);
        if denom > 1e-15 { (pairs / denom).abs() } else { 0.0 }
    } else {
        0.0
    };

    // Stability: 1 - coefficient of variation (clamped)
    let stability = if mean.abs() > 1e-15 {
        (1.0 - phi / mean.abs()).clamp(0.0, 1.0)
    } else {
        0.0
    };

    // Complexity: number of distinct "levels" / data length
    let mut sorted = data.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
    let mut levels = 1usize;
    for w in sorted.windows(2) {
        if (w[1] - w[0]).abs() > 0.01 {
            levels += 1;
        }
    }
    let complexity = levels as f64 / n;

    // N6 score: how close the mean is to an n=6 constant
    let n6_constants = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 24.0];
    let min_dist = n6_constants.iter()
        .map(|c| (mean - c).abs())
        .fold(f64::MAX, f64::min);
    let n6_score = 1.0 / (1.0 + min_dist);

    ExperimentMetrics {
        phi,
        entropy,
        connectivity,
        stability,
        complexity,
        n6_score,
    }
}

// ─── Experiment Manipulations ───────────────────────────────────

fn apply_acceleration(data: &[f64], intensity: f64) -> Vec<f64> {
    // Scale up data values
    let factor = 1.0 + intensity * 6.0; // up to 7x (σ-sopfr=7)
    data.iter().map(|x| x * factor).collect()
}

fn apply_collision(data: &[f64], intensity: f64) -> Vec<f64> {
    // Merge data with a phase-shifted version of itself
    let shift = (data.len() as f64 * intensity).ceil() as usize;
    let mut result = data.to_vec();
    for (i, val) in result.iter_mut().enumerate() {
        let partner = data[(i + shift) % data.len()];
        *val = (*val + partner * intensity) / (1.0 + intensity);
    }
    result
}

fn apply_separation(data: &[f64], intensity: f64) -> Vec<f64> {
    // Keep only a fraction of the data, zero out the rest
    let keep = ((1.0 - intensity) * data.len() as f64).ceil() as usize;
    let mut result = data.to_vec();
    for val in result.iter_mut().skip(keep) {
        *val = 0.0;
    }
    result
}

fn apply_fusion(data: &[f64], intensity: f64) -> Vec<f64> {
    // Average neighboring elements, reducing distinctness
    let window = (intensity * 5.0).ceil() as usize + 1; // 1..6
    data.iter().enumerate().map(|(i, _)| {
        let start = i.saturating_sub(window / 2);
        let end = (i + window / 2 + 1).min(data.len());
        let slice = &data[start..end];
        slice.iter().sum::<f64>() / slice.len() as f64
    }).collect()
}

fn apply_reversal(data: &[f64]) -> Vec<f64> {
    let mut result = data.to_vec();
    result.reverse();
    result
}

fn apply_destruction(data: &[f64], intensity: f64) -> Vec<f64> {
    // Inject pseudo-random noise proportional to intensity
    data.iter().enumerate().map(|(i, x)| {
        let noise = (i as f64 * 7.3 + 13.7).sin() * intensity * x.abs();
        x + noise
    }).collect()
}

fn apply_amplification(data: &[f64], intensity: f64) -> Vec<f64> {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let factor = 1.0 + intensity * 12.0; // sigma=12 amplification
    data.iter().map(|x| mean + (x - mean) * factor).collect()
}

fn apply_suppression(data: &[f64], intensity: f64) -> Vec<f64> {
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    let factor = 1.0 - intensity * 0.9; // reduce deviation up to 90%
    data.iter().map(|x| mean + (x - mean) * factor).collect()
}

fn apply_mutation(data: &[f64], intensity: f64) -> Vec<f64> {
    // Pseudo-random perturbation based on index
    data.iter().enumerate().map(|(i, x)| {
        let perturbation = (i as f64 * 3.7 + 2.1).cos() * intensity * 6.0;
        x + perturbation
    }).collect()
}

fn apply_crossover(data: &[f64], intensity: f64) -> Vec<f64> {
    // Swap first half and second half with blending
    let mid = data.len() / 2;
    let mut result = data.to_vec();
    for i in 0..mid.min(data.len() - mid) {
        let a = data[i];
        let b = data[mid + i];
        result[i] = a * (1.0 - intensity) + b * intensity;
        result[mid + i] = b * (1.0 - intensity) + a * intensity;
    }
    result
}

fn apply_isolation(data: &[f64], intensity: f64) -> Vec<f64> {
    // Keep only a central subset, zero out edges
    let total = data.len();
    let keep = ((1.0 - intensity) * total as f64).ceil() as usize;
    let start = (total - keep) / 2;
    let end = start + keep;
    data.iter().enumerate().map(|(i, x)| {
        if i >= start && i < end { *x } else { 0.0 }
    }).collect()
}

fn apply_overload(data: &[f64], duration: usize) -> Vec<f64> {
    // Replicate data `duration` times
    let mut result = Vec::with_capacity(data.len() * duration);
    for _ in 0..duration {
        result.extend_from_slice(data);
    }
    result
}

fn apply_starvation(data: &[f64], intensity: f64) -> Vec<f64> {
    // Keep only a small fraction
    let keep = ((1.0 - intensity * 0.9) * data.len() as f64).ceil() as usize;
    let keep = keep.max(1);
    data[..keep].to_vec()
}

fn apply_time_warp(data: &[f64], intensity: f64) -> Vec<f64> {
    // Resample at a different rate
    let rate = 1.0 + intensity * 2.0; // up to 3x stretch
    let new_len = (data.len() as f64 * rate).ceil() as usize;
    (0..new_len).map(|i| {
        let src = (i as f64 / rate).min((data.len() - 1) as f64);
        let lo = src.floor() as usize;
        let hi = (lo + 1).min(data.len() - 1);
        let frac = src - lo as f64;
        data[lo] * (1.0 - frac) + data[hi] * frac
    }).collect()
}

fn apply_dimension_shift(data: &[f64], intensity: f64) -> Vec<f64> {
    // Add derived "dimensions" (squared, sqrt values)
    let extra = (intensity * data.len() as f64).ceil() as usize;
    let mut result = data.to_vec();
    for i in 0..extra.min(data.len()) {
        result.push(data[i] * data[i] / 6.0); // normalized square
    }
    result
}

fn apply_symmetry_breaking(data: &[f64], intensity: f64) -> Vec<f64> {
    // Remove the mean (symmetric component) partially
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    data.iter().map(|x| x - mean * intensity).collect()
}

fn apply_resonance(data: &[f64], intensity: f64) -> Vec<f64> {
    // Amplify the dominant "frequency" (periodic component)
    let n = data.len();
    // Detect dominant period via autocorrelation
    let mean = data.iter().sum::<f64>() / n as f64;
    let centered: Vec<f64> = data.iter().map(|x| x - mean).collect();
    let mut best_lag = 1usize;
    let mut best_corr = f64::MIN;
    for lag in 1..n / 2 {
        let corr: f64 = centered.iter()
            .zip(centered.iter().cycle().skip(lag))
            .take(n - lag)
            .map(|(a, b)| a * b)
            .sum();
        if corr > best_corr {
            best_corr = corr;
            best_lag = lag;
        }
    }
    // Amplify that period
    data.iter().enumerate().map(|(i, x)| {
        let resonant = (i as f64 / best_lag as f64 * std::f64::consts::TAU).sin();
        x + resonant * intensity * 6.0
    }).collect()
}

fn apply_tension(data: &[f64], intensity: f64, duration: usize) -> (Vec<f64>, f64) {
    // Gradually increase stress; find the breakpoint where stability drops below threshold
    let original_metrics = measure_metrics(data);
    let base_stability = original_metrics.stability;
    let threshold = base_stability * 0.5; // break at 50% stability loss

    let mut breakpoint = 1.0_f64;
    let mut result = data.to_vec();
    let steps = duration.max(1);

    for step in 1..=steps {
        let stress = intensity * (step as f64 / steps as f64);
        let stressed: Vec<f64> = data.iter().enumerate().map(|(i, x)| {
            let strain = (i as f64 * 5.3 + step as f64).sin() * stress * x.abs();
            x + strain
        }).collect();
        let m = measure_metrics(&stressed);
        if m.stability < threshold {
            breakpoint = stress;
            result = stressed;
            break;
        }
        result = stressed;
        breakpoint = stress;
    }

    (result, breakpoint)
}

fn apply_compression(data: &[f64], intensity: f64) -> Vec<f64> {
    // Keep only the top-k most significant components (sorted by magnitude)
    let keep = ((1.0 - intensity * 0.8) * data.len() as f64).ceil() as usize;
    let keep = keep.max(1);

    // Sort by absolute magnitude, keep top-k, zero out rest
    let mut indexed: Vec<(usize, f64)> = data.iter().enumerate().map(|(i, &v)| (i, v)).collect();
    indexed.sort_by(|a, b| b.1.abs().partial_cmp(&a.1.abs()).unwrap_or(std::cmp::Ordering::Equal));

    let mut result = vec![0.0; data.len()];
    for &(idx, val) in indexed.iter().take(keep) {
        result[idx] = val;
    }
    result
}

fn apply_vibration(data: &[f64], intensity: f64) -> Vec<f64> {
    // Multi-frequency perturbation
    data.iter().enumerate().map(|(i, x)| {
        let v1 = (i as f64 * 1.0).sin() * intensity;
        let v2 = (i as f64 * 2.5).sin() * intensity * 0.5;
        let v3 = (i as f64 * 6.0).sin() * intensity * 0.25; // n=6 frequency
        x + v1 + v2 + v3
    }).collect()
}

fn apply_elasticity(data: &[f64], intensity: f64) -> Vec<f64> {
    // Deform then partially recover
    let deformed: Vec<f64> = data.iter().enumerate().map(|(i, x)| {
        let deformation = (i as f64 * 4.1).sin() * intensity * x.abs();
        x + deformation
    }).collect();
    // Recover: blend back toward original
    let recovery = 1.0 - intensity * 0.3; // partial recovery
    data.iter().zip(deformed.iter()).map(|(orig, def)| {
        orig * recovery + def * (1.0 - recovery)
    }).collect()
}

fn apply_friction(data: &[f64], intensity: f64) -> Vec<f64> {
    // Dampen differences between adjacent values (smoothing = resistance)
    let mut result = data.to_vec();
    let passes = (intensity * 6.0).ceil() as usize; // up to 6 passes
    for _ in 0..passes {
        let prev = result.clone();
        for i in 1..result.len() - 1 {
            result[i] = prev[i] * (1.0 - intensity * 0.5)
                + (prev[i - 1] + prev[i + 1]) * intensity * 0.25;
        }
    }
    result
}

// ─── Discovery Detection ────────────────────────────────────────

fn detect_discoveries(
    exp_type: ExperimentType,
    before: &ExperimentMetrics,
    after: &ExperimentMetrics,
    breakpoint: Option<f64>,
) -> Vec<String> {
    let mut discoveries = Vec::new();

    // Check for significant n6_score improvement
    let n6_delta = after.n6_score - before.n6_score;
    if n6_delta > 0.05 {
        discoveries.push(format!(
            "{}: n6_score improved by {:.3} ({:.3} -> {:.3})",
            exp_type.name(), n6_delta, before.n6_score, after.n6_score
        ));
    }

    // Check for emergent complexity
    let complexity_delta = after.complexity - before.complexity;
    if complexity_delta > 0.1 {
        discoveries.push(format!(
            "{}: complexity emerged (+{:.3})",
            exp_type.name(), complexity_delta
        ));
    }

    // Check for stability anomaly (dramatic change)
    let stability_delta = (after.stability - before.stability).abs();
    if stability_delta > 0.3 {
        discoveries.push(format!(
            "{}: stability anomaly (delta={:.3})",
            exp_type.name(), stability_delta
        ));
    }

    // Breakpoint discovery
    if let Some(bp) = breakpoint {
        // Check if breakpoint aligns with n=6 fractions
        let n6_fracs = [1.0/6.0, 1.0/3.0, 0.5, 2.0/3.0, 5.0/6.0, 1.0];
        for frac in &n6_fracs {
            if (bp - frac).abs() < 0.05 {
                discoveries.push(format!(
                    "{}: breakpoint {:.3} near n=6 fraction {:.3}",
                    exp_type.name(), bp, frac
                ));
            }
        }
    }

    discoveries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_probe_data_length_j2() {
        let data = generate_probe_data("ai");
        assert_eq!(data.len(), 24);
    }

    #[test]
    fn test_generate_probe_data_deterministic() {
        let d1 = generate_probe_data("fusion");
        let d2 = generate_probe_data("fusion");
        assert_eq!(d1, d2);
        let d3 = generate_probe_data("chip");
        assert_ne!(d1, d3);
    }

    #[test]
    fn test_runner_single_experiment() {
        let runner = ExperimentRunner::new();
        let config = ExperimentConfig::new(ExperimentType::Reversal, "energy");
        let result = runner.run(&config);
        assert_eq!(result.exp_type, ExperimentType::Reversal);
        assert!(result.breakpoint.is_none());
    }

    #[test]
    fn test_runner_run_all_22_types() {
        let runner = ExperimentRunner::new();
        let results = runner.run_all("ai");
        assert_eq!(results.len(), 22);
    }

    #[test]
    fn test_runner_tension_has_breakpoint() {
        let runner = ExperimentRunner::new();
        let config = ExperimentConfig::new(ExperimentType::Tension, "chip")
            .with_intensity(1.0)
            .with_duration(12);
        let result = runner.run(&config);
        assert_eq!(result.exp_type, ExperimentType::Tension);
        assert!(result.breakpoint.is_some());
    }

    #[test]
    fn test_apply_reversal_preserves_length() {
        let data = vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0];
        let reversed = apply_reversal(&data);
        assert_eq!(reversed.len(), 6);
        assert_eq!(reversed, vec![6.0, 5.0, 4.0, 3.0, 2.0, 1.0]);
    }

    #[test]
    fn test_apply_overload_replicates_n6() {
        let data = vec![6.0, 12.0, 24.0, 4.0];
        let overloaded = apply_overload(&data, 6);
        assert_eq!(overloaded.len(), 24);
    }

    #[test]
    fn test_measure_metrics_empty_data() {
        let m = measure_metrics(&[]);
        assert_eq!(m.phi, 0.0);
        assert_eq!(m.entropy, 0.0);
        assert_eq!(m.n6_score, 0.0);
    }

    #[test]
    fn test_measure_metrics_n6_constant_data() {
        let data = vec![6.0; 12];
        let m = measure_metrics(&data);
        assert_eq!(m.n6_score, 1.0);
        assert_eq!(m.stability, 1.0);
    }
}
