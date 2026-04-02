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
    /// Create a default config for the given type and target.
    pub fn new(exp_type: ExperimentType, target: &str) -> Self {
        Self {
            exp_type,
            intensity: 0.5,
            duration: 6,  // n=6 default
            target: target.to_string(),
            measure_interval: 1,
        }
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
