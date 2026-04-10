/// Pipeline builder utilities — template pipelines and serialization.

use super::{Pipeline, PipelineBuilder};

/// Create a pipeline from a list of step descriptions (simple DSL).
///
/// Each string: "scan:<domain>:<tier>", "verify:<tol>", "filter:<min>",
/// "experiment:<type>", "register", "red_team", "publish", "custom:<name>:<desc>"
pub fn from_dsl(name: &str, steps: &[&str]) -> Result<Pipeline, String> {
    let mut builder = PipelineBuilder::new();

    for step_str in steps {
        let parts: Vec<&str> = step_str.splitn(3, ':').collect();
        match parts[0] {
            "scan" => {
                if parts.len() < 3 {
                    return Err(format!("scan step needs domain:tier, got '{}'", step_str));
                }
                let tier: usize = parts[2].parse().map_err(|_| format!("bad tier: {}", parts[2]))?;
                builder = builder.scan(parts[1], tier);
            }
            "verify" => {
                if parts.len() < 2 {
                    return Err(format!("verify step needs tolerance, got '{}'", step_str));
                }
                let tol: f64 = parts[1].parse().map_err(|_| format!("bad tol: {}", parts[1]))?;
                builder = builder.verify(tol);
            }
            "filter" => {
                if parts.len() < 2 {
                    return Err(format!("filter step needs min_confidence, got '{}'", step_str));
                }
                let min: f64 = parts[1].parse().map_err(|_| format!("bad min: {}", parts[1]))?;
                builder = builder.filter(min);
            }
            "experiment" => {
                if parts.len() < 2 {
                    return Err(format!("experiment step needs type, got '{}'", step_str));
                }
                builder = builder.experiment(parts[1]);
            }
            "register" => {
                builder = builder.register();
            }
            "red_team" => {
                builder = builder.red_team();
            }
            "publish" => {
                builder = builder.publish();
            }
            "custom" => {
                if parts.len() < 3 {
                    return Err(format!("custom step needs name:desc, got '{}'", step_str));
                }
                builder = builder.custom(parts[1], parts[2]);
            }
            other => {
                return Err(format!("unknown pipeline step: '{}'", other));
            }
        }
    }

    Ok(builder.build(name))
}

/// Serialize a pipeline to a human-readable string representation.
pub fn to_string(pipeline: &Pipeline) -> String {
    let mut out = format!("Pipeline: {} ({} steps)\n", pipeline.name, pipeline.steps.len());
    for (i, step) in pipeline.steps.iter().enumerate() {
        out.push_str(&format!("  [{}] {}\n", i + 1, step.name()));
    }
    out
}
