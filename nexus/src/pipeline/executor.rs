/// Pipeline executor — runs each step and aggregates results.

use super::{Pipeline, PipelineStep, PipelineResult};

/// Execute a pipeline step-by-step.
///
/// Each step is simulated deterministically (no external I/O).
/// In a full system, steps would invoke telescope scans, experiments, etc.
pub fn execute_pipeline(pipeline: &Pipeline) -> PipelineResult {
    let total_steps = pipeline.steps.len();
    let mut steps_completed = 0;
    let mut discoveries: Vec<String> = Vec::new();
    let mut filtered_out = 0;

    // Simulated discovery pool that steps operate on
    let mut pool: Vec<(String, f64)> = Vec::new(); // (id, confidence)

    for step in &pipeline.steps {
        match step {
            PipelineStep::Scan { domain, tier } => {
                // Simulate discovering items proportional to tier
                let n_found = (*tier + 1) * 3; // tier 1 -> 6 items (n=6!)
                for i in 0..n_found {
                    let confidence = 0.5 + (i as f64 * 0.05).min(0.45);
                    let id = format!("{}-{}-{}", domain, tier, i);
                    pool.push((id, confidence));
                }
            }
            PipelineStep::Verify { tolerance } => {
                // Remove items that fail verification
                let before = pool.len();
                pool.retain(|(_, conf)| *conf > *tolerance);
                filtered_out += before - pool.len();
            }
            PipelineStep::Filter { min_confidence } => {
                let before = pool.len();
                pool.retain(|(_, conf)| *conf >= *min_confidence);
                filtered_out += before - pool.len();
            }
            PipelineStep::Experiment { .. } => {
                // Boost confidence of remaining items slightly
                for (_, conf) in pool.iter_mut() {
                    *conf = (*conf * 1.1).min(1.0);
                }
            }
            PipelineStep::RedTeam => {
                // Red team challenge: remove weakest 1/6 (n=6)
                if pool.len() > 6 {
                    pool.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
                    let cut = pool.len() / 6;
                    let removed: Vec<_> = pool.drain(..cut).collect();
                    filtered_out += removed.len();
                }
            }
            PipelineStep::Register => {
                // Mark surviving items as discoveries
                for (id, _) in &pool {
                    discoveries.push(id.clone());
                }
            }
            PipelineStep::Publish => {
                // Final step — nothing extra to do
            }
            PipelineStep::Custom { .. } => {
                // Custom steps are no-ops in simulation
            }
        }
        steps_completed += 1;
    }

    PipelineResult {
        steps_completed,
        total_steps,
        discoveries,
        filtered_out,
    }
}
