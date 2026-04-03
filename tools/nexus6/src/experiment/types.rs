/// 22 experiment types for the NEXUS-6 unified experiment engine.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ExperimentType {
    Acceleration,       // Accelerate — learning speed/efficiency
    Collision,          // Collision — emergent behavior from merging two systems
    Separation,         // Separation — split one into two
    Fusion,             // Fusion — merge many small parts into one
    Reversal,           // Reversal — run the process backwards
    Destruction,        // Destruction — intentional destruction, observe resilience
    Amplification,      // Amplification — maximize a weak signal
    Suppression,        // Suppression — suppress a strong signal
    Mutation,           // Mutation — random transformation, observe outcome
    Crossover,          // Crossover — swap traits between two systems
    Isolation,          // Isolation — isolate a subsystem
    Overload,           // Overload — push beyond limits
    Starvation,         // Starvation — minimize input
    TimeWarp,           // TimeWarp — speed changes / temporal resampling
    DimensionShift,     // DimensionShift — add or remove dimensions
    SymmetryBreaking,   // SymmetryBreaking — intentionally break symmetry
    Resonance,          // Resonance — find and inject resonant frequency
    Tension,            // Tension — gradual stress to find breakpoint
    Compression,        // Compression — extreme compression to extract essence
    Vibration,          // Vibration — inject vibrations at various frequencies
    Elasticity,         // Elasticity — deform then measure recovery
    Friction,           // Friction — interface resistance simulation
}

/// All 22 experiment types in canonical order.
pub const ALL_EXPERIMENT_TYPES: [ExperimentType; 22] = [
    ExperimentType::Acceleration,
    ExperimentType::Collision,
    ExperimentType::Separation,
    ExperimentType::Fusion,
    ExperimentType::Reversal,
    ExperimentType::Destruction,
    ExperimentType::Amplification,
    ExperimentType::Suppression,
    ExperimentType::Mutation,
    ExperimentType::Crossover,
    ExperimentType::Isolation,
    ExperimentType::Overload,
    ExperimentType::Starvation,
    ExperimentType::TimeWarp,
    ExperimentType::DimensionShift,
    ExperimentType::SymmetryBreaking,
    ExperimentType::Resonance,
    ExperimentType::Tension,
    ExperimentType::Compression,
    ExperimentType::Vibration,
    ExperimentType::Elasticity,
    ExperimentType::Friction,
];

impl ExperimentType {
    /// Human-readable description of this experiment type.
    pub fn description(&self) -> &str {
        match self {
            Self::Acceleration => "Accelerate data scale to observe speed/efficiency changes",
            Self::Collision => "Merge two systems and observe emergent properties",
            Self::Separation => "Split a system in two and compare halves",
            Self::Fusion => "Fuse multiple subsets into one unified structure",
            Self::Reversal => "Reverse the data ordering and observe invariance",
            Self::Destruction => "Inject noise to destroy structure, measure resilience",
            Self::Amplification => "Scale up weak signals to reveal hidden patterns",
            Self::Suppression => "Dampen strong signals to expose underlying structure",
            Self::Mutation => "Apply random perturbations and observe stability",
            Self::Crossover => "Swap segments between two datasets",
            Self::Isolation => "Isolate a subset and measure independent behavior",
            Self::Overload => "Duplicate/expand data beyond normal capacity",
            Self::Starvation => "Reduce data to minimum and observe core behavior",
            Self::TimeWarp => "Resample along time axis at different rates",
            Self::DimensionShift => "Add or remove data dimensions",
            Self::SymmetryBreaking => "Remove symmetric components intentionally",
            Self::Resonance => "Find and amplify resonant frequencies",
            Self::Tension => "Gradually increase stress until breakpoint",
            Self::Compression => "Compress data to extract essential structure (PCA/SVD-like)",
            Self::Vibration => "Inject perturbations at varying frequencies",
            Self::Elasticity => "Deform then release, measure recovery fidelity",
            Self::Friction => "Simulate interface resistance between subsystems",
        }
    }

    /// Recommended telescope lenses for this experiment type.
    pub fn recommended_lenses(&self) -> Vec<&str> {
        match self {
            Self::Acceleration => vec!["causal", "wave", "scale"],
            Self::Collision => vec!["consciousness", "topology", "network"],
            Self::Separation => vec!["boundary", "topology", "info"],
            Self::Fusion => vec!["consciousness", "network", "stability"],
            Self::Reversal => vec!["causal", "memory", "recursion"],
            Self::Destruction => vec!["stability", "boundary", "thermo"],
            Self::Amplification => vec!["wave", "scale", "em"],
            Self::Suppression => vec!["wave", "scale", "info"],
            Self::Mutation => vec!["evolution", "stability", "quantum"],
            Self::Crossover => vec!["evolution", "network", "topology"],
            Self::Isolation => vec!["boundary", "info", "recursion"],
            Self::Overload => vec!["thermo", "stability", "scale"],
            Self::Starvation => vec!["info", "boundary", "multiscale"],
            Self::TimeWarp => vec!["memory", "wave", "causal", "multiscale"],
            Self::DimensionShift => vec!["multiscale", "topology", "recursion"],
            Self::SymmetryBreaking => vec!["mirror", "topology", "quantum"],
            Self::Resonance => vec!["wave", "em", "quantum_microscope"],
            Self::Tension => vec!["stability", "boundary", "thermo"],
            Self::Compression => vec!["info", "scale", "recursion"],
            Self::Vibration => vec!["wave", "quantum", "em"],
            Self::Elasticity => vec!["stability", "memory", "thermo"],
            Self::Friction => vec!["boundary", "thermo", "network"],
        }
    }

    /// Short name for display.
    pub fn name(&self) -> &str {
        match self {
            Self::Acceleration => "Acceleration",
            Self::Collision => "Collision",
            Self::Separation => "Separation",
            Self::Fusion => "Fusion",
            Self::Reversal => "Reversal",
            Self::Destruction => "Destruction",
            Self::Amplification => "Amplification",
            Self::Suppression => "Suppression",
            Self::Mutation => "Mutation",
            Self::Crossover => "Crossover",
            Self::Isolation => "Isolation",
            Self::Overload => "Overload",
            Self::Starvation => "Starvation",
            Self::TimeWarp => "TimeWarp",
            Self::DimensionShift => "DimensionShift",
            Self::SymmetryBreaking => "SymmetryBreak",
            Self::Resonance => "Resonance",
            Self::Tension => "Tension",
            Self::Compression => "Compression",
            Self::Vibration => "Vibration",
            Self::Elasticity => "Elasticity",
            Self::Friction => "Friction",
        }
    }

    /// Parse from a string (case-insensitive).
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "acceleration" => Some(Self::Acceleration),
            "collision" => Some(Self::Collision),
            "separation" => Some(Self::Separation),
            "fusion" => Some(Self::Fusion),
            "reversal" => Some(Self::Reversal),
            "destruction" => Some(Self::Destruction),
            "amplification" => Some(Self::Amplification),
            "suppression" => Some(Self::Suppression),
            "mutation" => Some(Self::Mutation),
            "crossover" => Some(Self::Crossover),
            "isolation" => Some(Self::Isolation),
            "overload" => Some(Self::Overload),
            "starvation" => Some(Self::Starvation),
            "timewarp" => Some(Self::TimeWarp),
            "dimensionshift" | "dimension_shift" => Some(Self::DimensionShift),
            "symmetrybreaking" | "symmetry_breaking" | "symmetrybreak" => Some(Self::SymmetryBreaking),
            "resonance" => Some(Self::Resonance),
            "tension" => Some(Self::Tension),
            "compression" => Some(Self::Compression),
            "vibration" => Some(Self::Vibration),
            "elasticity" => Some(Self::Elasticity),
            "friction" => Some(Self::Friction),
            _ => None,
        }
    }
}

impl std::fmt::Display for ExperimentType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name())
    }
}

/// Configuration for running an experiment.
pub struct ExperimentConfig {
    pub exp_type: ExperimentType,
    pub intensity: f64,         // Experiment intensity (0.0~1.0)
    pub duration: usize,        // Number of experiment steps
    pub target: String,         // Target (domain, lens, etc.)
    pub measure_interval: usize, // Measurement interval
}

impl ExperimentConfig {
    /// Create a domain-aware config for the given type and target.
    /// Intensity and duration are tuned per domain to produce differentiated results.
    pub fn new(exp_type: ExperimentType, target: &str) -> Self {
        let (base_intensity, base_duration) = Self::domain_params(target);

        // Per-experiment-type modulation: some experiments are naturally
        // more/less intense depending on the domain's characteristics.
        let (intensity, duration) = Self::experiment_modulation(
            exp_type, base_intensity, base_duration, target,
        );

        Self {
            exp_type,
            intensity: intensity.clamp(0.01, 1.0),
            duration: duration.max(1),
            target: target.to_string(),
            measure_interval: 1,
        }
    }

    /// Domain-specific base parameters.
    fn domain_params(target: &str) -> (f64, usize) {
        let t = target.to_lowercase();

        // (base_intensity, base_duration)
        if t.contains("physics") || t.contains("particle") || t.contains("cosmology") {
            (0.50, 6)   // balanced, n=6 steps
        } else if t.contains("biology") || t.contains("bio") || t.contains("genetic") {
            (0.60, 8)   // biological systems tolerate more perturbation, longer evolution
        } else if t.contains("chip") || t.contains("semiconductor") || t.contains("gpu") {
            (0.40, 4)   // precise, short cycles (τ=4)
        } else if t.contains("energy") || t.contains("power") || t.contains("grid") || t.contains("solar") {
            (0.55, 6)   // moderate intensity, standard duration
        } else if t.contains("battery") {
            (0.65, 8)   // electrochemical cycles, higher stress
        } else if t.contains("fusion") || t.contains("plasma") || t.contains("tokamak") {
            (0.70, 10)  // extreme conditions, long confinement
        } else if t.contains("consciousness") || t.contains("iit") {
            (0.45, 12)  // subtle, many integration steps (σ=12)
        } else if t.contains("quantum") {
            (0.35, 5)   // fragile, short coherence (sopfr=5)
        } else if t.contains("superconductor") {
            (0.55, 6)
        } else if t.contains("robot") {
            (0.50, 6)
        } else if t.contains("blockchain") || t.contains("crypto") {
            (0.35, 6)   // low intensity, deterministic
        } else if t.contains("network") || t.contains("protocol") {
            (0.45, 7)   // 7 OSI layers
        } else if t.contains("display") || t.contains("audio") {
            (0.55, 12)  // σ=12 semitones/frames
        } else if t.contains("software") || t.contains("compiler") {
            (0.40, 5)   // SOLID=5 principles
        } else if t.contains("math") {
            (0.50, 6)   // perfect number steps
        } else if t.contains("environment") {
            (0.60, 6)
        } else if t.contains("material") || t.contains("synthesis") {
            (0.55, 6)
        } else if t.contains("cryptograph") {
            (0.35, 4)   // minimal, precise
        } else if t.contains("learning") || t.contains("training") || t.contains("ai") {
            (0.55, 8)   // training epochs
        } else {
            (0.50, 6)   // default
        }
    }

    /// Modulate intensity/duration per experiment type and domain.
    /// This ensures e.g. Destruction doesn't always dominate.
    fn experiment_modulation(
        exp_type: ExperimentType,
        base_intensity: f64,
        base_duration: usize,
        target: &str,
    ) -> (f64, usize) {
        let t = target.to_lowercase();

        // Domain-experiment affinity: some experiments are more/less
        // effective depending on the domain's nature.
        let affinity = match exp_type {
            // Destruction: less effective on robust/large-scale systems
            ExperimentType::Destruction => {
                if t.contains("chip") || t.contains("blockchain") || t.contains("cryptograph") {
                    0.6  // robust to noise
                } else if t.contains("consciousness") || t.contains("quantum") {
                    1.2  // fragile
                } else if t.contains("biology") {
                    0.8  // resilient through redundancy
                } else {
                    1.0
                }
            }
            // Tension: more effective on physical/mechanical systems
            ExperimentType::Tension => {
                if t.contains("fusion") || t.contains("plasma") || t.contains("material") {
                    1.3  // mechanical stress relevant
                } else if t.contains("software") || t.contains("blockchain") || t.contains("math") {
                    0.5  // abstract, less physical tension
                } else {
                    0.9
                }
            }
            // Resonance: strong for wave/frequency domains
            ExperimentType::Resonance => {
                if t.contains("audio") || t.contains("display") || t.contains("wave") {
                    1.4
                } else if t.contains("quantum") || t.contains("physics") {
                    1.2
                } else if t.contains("software") || t.contains("blockchain") {
                    0.5
                } else {
                    0.8
                }
            }
            // Mutation: strong for biological/evolutionary systems
            ExperimentType::Mutation => {
                if t.contains("biology") || t.contains("genetic") {
                    1.4
                } else if t.contains("learning") || t.contains("ai") {
                    1.1
                } else if t.contains("chip") || t.contains("cryptograph") {
                    0.5
                } else {
                    0.8
                }
            }
            // Fusion: strong for plasma/energy domains
            ExperimentType::Fusion => {
                if t.contains("fusion") || t.contains("plasma") || t.contains("energy") {
                    1.4
                } else if t.contains("consciousness") {
                    1.2
                } else {
                    0.7
                }
            }
            // Crossover: strong for biology, AI
            ExperimentType::Crossover => {
                if t.contains("biology") || t.contains("genetic") {
                    1.3
                } else if t.contains("learning") || t.contains("ai") {
                    1.2
                } else {
                    0.7
                }
            }
            // Compression: strong for information-rich domains
            ExperimentType::Compression => {
                if t.contains("software") || t.contains("network") || t.contains("cryptograph") {
                    1.3
                } else if t.contains("audio") || t.contains("display") {
                    1.2
                } else {
                    0.8
                }
            }
            // SymmetryBreaking: strong for physics/math
            ExperimentType::SymmetryBreaking => {
                if t.contains("physics") || t.contains("math") || t.contains("quantum") {
                    1.3
                } else if t.contains("consciousness") {
                    1.1
                } else {
                    0.7
                }
            }
            // DimensionShift: strong for quantum/math/consciousness
            ExperimentType::DimensionShift => {
                if t.contains("quantum") || t.contains("math") || t.contains("consciousness") {
                    1.3
                } else if t.contains("chip") || t.contains("battery") {
                    0.6
                } else {
                    0.8
                }
            }
            // Amplification: strong for signal-based domains
            ExperimentType::Amplification => {
                if t.contains("audio") || t.contains("display") || t.contains("network") {
                    1.3
                } else if t.contains("physics") || t.contains("quantum") {
                    1.1
                } else {
                    0.8
                }
            }
            // Starvation: strong for resource-dependent systems
            ExperimentType::Starvation => {
                if t.contains("energy") || t.contains("battery") || t.contains("fusion") {
                    1.3
                } else if t.contains("biology") {
                    1.2
                } else if t.contains("math") || t.contains("software") {
                    0.5
                } else {
                    0.8
                }
            }
            // Elasticity: strong for physical/material systems
            ExperimentType::Elasticity => {
                if t.contains("material") || t.contains("robot") || t.contains("superconductor") {
                    1.3
                } else if t.contains("software") || t.contains("blockchain") {
                    0.5
                } else {
                    0.8
                }
            }
            // Friction: strong for mechanical/interface systems
            ExperimentType::Friction => {
                if t.contains("robot") || t.contains("material") || t.contains("chip") {
                    1.3
                } else if t.contains("quantum") || t.contains("consciousness") {
                    0.5
                } else {
                    0.8
                }
            }
            // Overload: strong for capacity-limited systems
            ExperimentType::Overload => {
                if t.contains("network") || t.contains("chip") || t.contains("energy") {
                    1.2
                } else if t.contains("math") {
                    0.5
                } else {
                    0.8
                }
            }
            // Others: moderate domain sensitivity
            _ => 0.9,
        };

        let final_intensity = base_intensity * affinity;
        let duration_scale = if affinity > 1.1 { 1.5 } else if affinity < 0.7 { 0.7 } else { 1.0 };
        let final_duration = (base_duration as f64 * duration_scale).round() as usize;

        (final_intensity, final_duration)
    }

    pub fn with_intensity(mut self, intensity: f64) -> Self {
        self.intensity = intensity.clamp(0.0, 1.0);
        self
    }

    pub fn with_duration(mut self, duration: usize) -> Self {
        self.duration = duration;
        self
    }
}

/// Metrics measured before and after an experiment.
#[derive(Debug, Clone)]
pub struct ExperimentMetrics {
    pub phi: f64,               // Phi (consciousness indicator)
    pub entropy: f64,           // Entropy
    pub connectivity: f64,      // Connectivity
    pub stability: f64,         // Stability
    pub complexity: f64,        // Complexity
    pub n6_score: f64,          // n=6 alignment score
}

impl ExperimentMetrics {
    /// Zero metrics.
    pub fn zero() -> Self {
        Self {
            phi: 0.0,
            entropy: 0.0,
            connectivity: 0.0,
            stability: 0.0,
            complexity: 0.0,
            n6_score: 0.0,
        }
    }

    /// Compute delta (self - other).
    pub fn delta(&self, other: &ExperimentMetrics) -> ExperimentMetrics {
        ExperimentMetrics {
            phi: self.phi - other.phi,
            entropy: self.entropy - other.entropy,
            connectivity: self.connectivity - other.connectivity,
            stability: self.stability - other.stability,
            complexity: self.complexity - other.complexity,
            n6_score: self.n6_score - other.n6_score,
        }
    }
}

/// Result of a single experiment run.
pub struct ExperimentResult {
    pub exp_type: ExperimentType,
    pub before: ExperimentMetrics,
    pub after: ExperimentMetrics,
    pub delta: ExperimentMetrics,
    pub breakpoint: Option<f64>,  // Breakpoint/critical point (if applicable)
    pub discoveries: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_experiment_types_count_22() {
        assert_eq!(ALL_EXPERIMENT_TYPES.len(), 22);
    }

    #[test]
    fn test_experiment_type_from_str_roundtrip() {
        for exp_type in &ALL_EXPERIMENT_TYPES {
            let name = exp_type.name();
            let parsed = ExperimentType::from_str(name);
            assert!(parsed.is_some(), "Failed to parse: {}", name);
            assert_eq!(parsed.unwrap(), *exp_type);
        }
    }

    #[test]
    fn test_experiment_type_from_str_case_insensitive() {
        assert_eq!(ExperimentType::from_str("FUSION"), Some(ExperimentType::Fusion));
        assert_eq!(ExperimentType::from_str("dimension_shift"), Some(ExperimentType::DimensionShift));
        assert_eq!(ExperimentType::from_str("symmetry_breaking"), Some(ExperimentType::SymmetryBreaking));
        assert_eq!(ExperimentType::from_str("nonexistent"), None);
    }

    #[test]
    fn test_experiment_config_defaults_n6() {
        // "math" domain gives base (0.50, 6) = n=6 default duration
        let config = ExperimentConfig::new(ExperimentType::Reversal, "math");
        assert_eq!(config.target, "math");
        // Duration should be positive and intensity should be in (0,1]
        assert!(config.duration >= 1);
        assert!(config.intensity > 0.0 && config.intensity <= 1.0);
    }

    #[test]
    fn test_experiment_config_intensity_clamp() {
        let config = ExperimentConfig::new(ExperimentType::Tension, "chip")
            .with_intensity(2.5);
        assert_eq!(config.intensity, 1.0);
        let config2 = ExperimentConfig::new(ExperimentType::Tension, "chip")
            .with_intensity(-1.0);
        assert_eq!(config2.intensity, 0.0);
    }

    #[test]
    fn test_experiment_metrics_delta() {
        let before = ExperimentMetrics {
            phi: 6.0, entropy: 12.0, connectivity: 0.5,
            stability: 0.8, complexity: 0.3, n6_score: 0.9,
        };
        let after = ExperimentMetrics {
            phi: 12.0, entropy: 24.0, connectivity: 0.7,
            stability: 0.6, complexity: 0.5, n6_score: 1.0,
        };
        let delta = after.delta(&before);
        assert!((delta.phi - 6.0).abs() < 1e-10);
        assert!((delta.entropy - 12.0).abs() < 1e-10);
        assert!((delta.n6_score - 0.1).abs() < 1e-10);
    }

    #[test]
    fn test_experiment_metrics_zero() {
        let z = ExperimentMetrics::zero();
        assert_eq!(z.phi, 0.0);
        assert_eq!(z.entropy, 0.0);
        assert_eq!(z.n6_score, 0.0);
    }

    #[test]
    fn test_recommended_lenses_non_empty() {
        for exp_type in &ALL_EXPERIMENT_TYPES {
            let lenses = exp_type.recommended_lenses();
            assert!(lenses.len() >= 3, "{} has only {} lenses", exp_type.name(), lenses.len());
        }
    }
}
