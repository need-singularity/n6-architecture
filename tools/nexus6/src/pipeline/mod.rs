//! Data processing pipeline for scan-analyze-verify workflows.
/// Pipeline builder — composable discovery pipelines for NEXUS-6.
pub mod builder;
pub mod executor;

/// A single step in a discovery pipeline.
#[derive(Debug, Clone, PartialEq)]
pub enum PipelineStep {
    Scan { domain: String, tier: usize },
    Verify { tolerance: f64 },
    Experiment { exp_type: String },
    Filter { min_confidence: f64 },
    Register,
    RedTeam,
    Publish,
    Custom { name: String, description: String },
}

impl PipelineStep {
    /// Human-readable name for the step.
    pub fn name(&self) -> String {
        match self {
            Self::Scan { domain, tier } => format!("Scan({}, tier={})", domain, tier),
            Self::Verify { tolerance } => format!("Verify(tol={:.4})", tolerance),
            Self::Experiment { exp_type } => format!("Experiment({})", exp_type),
            Self::Filter { min_confidence } => format!("Filter(min={:.2})", min_confidence),
            Self::Register => "Register".to_string(),
            Self::RedTeam => "RedTeam".to_string(),
            Self::Publish => "Publish".to_string(),
            Self::Custom { name, .. } => format!("Custom({})", name),
        }
    }
}

/// A named sequence of pipeline steps.
#[derive(Debug, Clone)]
pub struct Pipeline {
    pub name: String,
    pub steps: Vec<PipelineStep>,
}

impl Pipeline {
    /// Number of steps.
    pub fn len(&self) -> usize {
        self.steps.len()
    }

    /// Whether pipeline has no steps.
    pub fn is_empty(&self) -> bool {
        self.steps.is_empty()
    }
}

/// Builder for constructing pipelines fluently.
pub struct PipelineBuilder {
    steps: Vec<PipelineStep>,
}

impl PipelineBuilder {
    pub fn new() -> Self {
        Self { steps: Vec::new() }
    }

    pub fn scan(mut self, domain: &str, tier: usize) -> Self {
        self.steps.push(PipelineStep::Scan {
            domain: domain.to_string(),
            tier,
        });
        self
    }

    pub fn verify(mut self, tolerance: f64) -> Self {
        self.steps.push(PipelineStep::Verify { tolerance });
        self
    }

    pub fn experiment(mut self, exp_type: &str) -> Self {
        self.steps.push(PipelineStep::Experiment {
            exp_type: exp_type.to_string(),
        });
        self
    }

    pub fn filter(mut self, min_confidence: f64) -> Self {
        self.steps.push(PipelineStep::Filter { min_confidence });
        self
    }

    pub fn register(mut self) -> Self {
        self.steps.push(PipelineStep::Register);
        self
    }

    pub fn red_team(mut self) -> Self {
        self.steps.push(PipelineStep::RedTeam);
        self
    }

    pub fn publish(mut self) -> Self {
        self.steps.push(PipelineStep::Publish);
        self
    }

    pub fn custom(mut self, name: &str, description: &str) -> Self {
        self.steps.push(PipelineStep::Custom {
            name: name.to_string(),
            description: description.to_string(),
        });
        self
    }

    pub fn build(self, name: &str) -> Pipeline {
        Pipeline {
            name: name.to_string(),
            steps: self.steps,
        }
    }
}

impl Default for PipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Result of executing a pipeline.
#[derive(Debug, Clone)]
pub struct PipelineResult {
    pub steps_completed: usize,
    pub total_steps: usize,
    pub discoveries: Vec<String>,
    pub filtered_out: usize,
}

impl PipelineResult {
    /// Completion ratio (0.0 to 1.0).
    pub fn completion_ratio(&self) -> f64 {
        if self.total_steps == 0 {
            return 1.0;
        }
        self.steps_completed as f64 / self.total_steps as f64
    }

    /// Whether the pipeline completed all steps.
    pub fn is_complete(&self) -> bool {
        self.steps_completed == self.total_steps
    }
}

/// Execute a pipeline, returning aggregated results.
pub fn execute(pipeline: &Pipeline) -> PipelineResult {
    executor::execute_pipeline(pipeline)
}

/// Predefined pipeline: standard discovery flow.
pub fn standard_discovery(domain: &str) -> Pipeline {
    PipelineBuilder::new()
        .scan(domain, 1)
        .verify(0.05)
        .filter(0.6)
        .experiment("tension")
        .red_team()
        .register()
        .publish()
        .build(&format!("standard-discovery-{}", domain))
}

/// Predefined pipeline: deep exploration (higher tier scan + stricter filter).
pub fn deep_exploration(domain: &str) -> Pipeline {
    PipelineBuilder::new()
        .scan(domain, 3)
        .verify(0.01)
        .filter(0.8)
        .experiment("perturbation")
        .experiment("cross_domain")
        .red_team()
        .register()
        .publish()
        .build(&format!("deep-exploration-{}", domain))
}
