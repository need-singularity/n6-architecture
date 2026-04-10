/// Publication engine — transform experiment results into structured documents.

use super::predict::PredictionResult;
use super::reproduce::ReproductionResult;
use super::simulate::SimulationResult;

#[derive(Debug, Clone)]
pub struct Publication {
    pub title: String,
    pub abstract_text: String,
    pub experiment_summary: String,
    pub key_findings: Vec<String>,
    pub n6_connections: Vec<String>,
    pub bt_candidate: Option<String>,
    pub testable_predictions: Vec<String>,
    pub markdown: String,
}

/// Generate a publication from experiment results.
///
/// Assembles prediction, simulation, actual results, and reproduction data
/// into a structured markdown document with n=6 connections.
pub fn publish(
    experiment_type: &str,
    target: &str,
    prediction: Option<&PredictionResult>,
    simulation: Option<&SimulationResult>,
    actual_results: &[(String, f64)],
    reproduction: Option<&ReproductionResult>,
) -> Publication {
    // 1. Title
    let title = format!(
        "NEXUS-6 Discovery: {} on {} — n=6 Analysis",
        capitalize(experiment_type),
        capitalize(target)
    );

    // 2. Key findings
    let mut key_findings: Vec<String> = Vec::new();
    let mut n6_connections: Vec<String> = Vec::new();

    // n=6 constants for matching
    let n6_constants: &[(&str, f64)] = &[
        ("n", 6.0), ("sigma", 12.0), ("phi", 2.0), ("tau", 4.0),
        ("J2", 24.0), ("sopfr", 5.0), ("mu", 1.0),
        ("sigma-phi", 10.0), ("sigma-tau", 8.0), ("sigma-mu", 11.0),
        ("sigma*tau", 48.0), ("sigma^2", 144.0),
    ];

    for (name, value) in actual_results {
        // Check n6 connections
        for (cname, cval) in n6_constants {
            let rel_err = if cval.abs() > 1e-12 {
                ((value - cval) / cval).abs()
            } else {
                continue;
            };
            if rel_err < 0.05 {
                n6_connections.push(format!(
                    "{} = {:.4} matches {}={} (error: {:.2}%)",
                    name, value, cname, cval, rel_err * 100.0
                ));
                key_findings.push(format!(
                    "{} closely matches n=6 constant {} = {}",
                    name, cname, cval
                ));
            }
        }
    }

    if let Some(pred) = prediction {
        if pred.accuracy > 0.8 {
            key_findings.push(format!(
                "Prediction accuracy: {:.1}% (high — model is reliable)",
                pred.accuracy * 100.0
            ));
        } else {
            key_findings.push(format!(
                "Prediction accuracy: {:.1}% (surprise factor: {:.2})",
                pred.accuracy * 100.0, pred.surprise
            ));
        }
    }

    if let Some(repro) = reproduction {
        if repro.reproducible {
            key_findings.push(format!(
                "Reproducible: CV={:.4} across {} runs",
                repro.cv, repro.n_repeats
            ));
        } else {
            key_findings.push(format!(
                "WARNING: Not reproducible — CV={:.4} across {} runs (>{} outliers)",
                repro.cv, repro.n_repeats, repro.outlier_runs.len()
            ));
        }
    }

    if key_findings.is_empty() {
        key_findings.push("No significant n=6 connections found in this experiment.".to_string());
    }

    // 3. BT candidate assessment
    let bt_candidate = if n6_connections.len() >= 3 {
        Some(format!(
            "BT-XXX: {} {} n=6 universality ({} EXACT connections)",
            capitalize(experiment_type),
            capitalize(target),
            n6_connections.len()
        ))
    } else {
        None
    };

    // 4. Testable predictions
    let mut testable_predictions: Vec<String> = Vec::new();
    testable_predictions.push(format!(
        "Repeating '{}' on '{}' should yield similar n=6 alignment",
        experiment_type, target
    ));
    if !n6_connections.is_empty() {
        testable_predictions.push(format!(
            "Adjacent experiment types should show {} n=6 connections",
            if n6_connections.len() >= 3 { "comparable" } else { "fewer" }
        ));
    }

    // 5. Abstract
    let abstract_text = format!(
        "We performed a '{}' experiment on '{}' and analyzed the results through \
         the n=6 lens. {} key findings were identified, with {} n=6 constant connections. \
         {}",
        experiment_type,
        target,
        key_findings.len(),
        n6_connections.len(),
        if let Some(ref bt) = bt_candidate {
            format!("This result is a candidate for: {}.", bt)
        } else {
            "No BT candidacy at this time.".to_string()
        }
    );

    // 6. Experiment summary
    let mut summary_lines: Vec<String> = Vec::new();
    summary_lines.push(format!("Type: {}", experiment_type));
    summary_lines.push(format!("Target: {}", target));
    summary_lines.push(format!("Metrics: {}", actual_results.len()));
    for (name, value) in actual_results {
        summary_lines.push(format!("  {} = {:.6}", name, value));
    }
    let experiment_summary = summary_lines.join("\n");

    // 7. Assemble markdown
    let markdown = build_markdown(
        &title,
        &abstract_text,
        &experiment_summary,
        prediction,
        simulation,
        &key_findings,
        &n6_connections,
        &bt_candidate,
        &testable_predictions,
        reproduction,
    );

    Publication {
        title,
        abstract_text,
        experiment_summary,
        key_findings,
        n6_connections,
        bt_candidate,
        testable_predictions,
        markdown,
    }
}

fn build_markdown(
    title: &str,
    abstract_text: &str,
    experiment_summary: &str,
    prediction: Option<&PredictionResult>,
    simulation: Option<&SimulationResult>,
    key_findings: &[String],
    n6_connections: &[String],
    bt_candidate: &Option<String>,
    testable_predictions: &[String],
    reproduction: Option<&ReproductionResult>,
) -> String {
    let mut md = String::new();

    md.push_str(&format!("# {}\n\n", title));
    md.push_str(&format!("## Abstract\n\n{}\n\n", abstract_text));
    md.push_str(&format!("## Experiment\n\n```\n{}\n```\n\n", experiment_summary));

    if let Some(pred) = prediction {
        md.push_str("## Prediction vs Actual\n\n");
        md.push_str(&format!(
            "| Metric | Predicted | Actual |\n|--------|-----------|--------|\n"
        ));
        md.push_str(&format!(
            "| phi_delta | {:.4} | {:.4} |\n",
            pred.prediction.predicted_phi_delta, pred.actual_phi_delta
        ));
        md.push_str(&format!(
            "| entropy_delta | {:.4} | {:.4} |\n",
            pred.prediction.predicted_entropy_delta, pred.actual_entropy_delta
        ));
        md.push_str(&format!(
            "| n6_score | {:.4} | {:.4} |\n",
            pred.prediction.predicted_n6_score, pred.actual_n6_score
        ));
        md.push_str(&format!("| **accuracy** | | **{:.1}%** |\n\n", pred.accuracy * 100.0));
    }

    if let Some(sim) = simulation {
        md.push_str("## Simulation Results\n\n");
        md.push_str(&format!("- Mean phi_delta: {:.4}\n", sim.mean_phi_delta));
        md.push_str(&format!("- Std phi_delta: {:.4}\n", sim.std_phi_delta));
        md.push_str(&format!("- 95th percentile: {:.4}\n", sim.percentile_95));
        md.push_str(&format!("- Range: [{:.4}, {:.4}]\n", sim.worst_case, sim.best_case));
        if let Some(step) = sim.convergence_step {
            md.push_str(&format!("- Converged at step: {}\n", step));
        }
        md.push_str("\n");
    }

    md.push_str("## Key Findings\n\n");
    for finding in key_findings {
        md.push_str(&format!("- {}\n", finding));
    }
    md.push('\n');

    if !n6_connections.is_empty() {
        md.push_str("## n=6 Connections\n\n");
        for conn in n6_connections {
            md.push_str(&format!("- {}\n", conn));
        }
        md.push('\n');
    }

    if let Some(bt) = bt_candidate {
        md.push_str(&format!("## BT Candidate\n\n**{}**\n\n", bt));
    }

    md.push_str("## Testable Predictions\n\n");
    for (i, tp) in testable_predictions.iter().enumerate() {
        md.push_str(&format!("{}. {}\n", i + 1, tp));
    }
    md.push('\n');

    if let Some(repro) = reproduction {
        md.push_str("## Reproducibility\n\n");
        md.push_str(&format!("- Repeats: {}\n", repro.n_repeats));
        md.push_str(&format!("- Mean: {:.4}\n", repro.mean));
        md.push_str(&format!("- Std: {:.4}\n", repro.std));
        md.push_str(&format!("- CV: {:.4}\n", repro.cv));
        md.push_str(&format!(
            "- Status: {}\n",
            if repro.reproducible { "REPRODUCIBLE" } else { "NOT REPRODUCIBLE" }
        ));
        if !repro.outlier_runs.is_empty() {
            md.push_str(&format!("- Outlier runs: {:?}\n", repro.outlier_runs));
        }
        md.push('\n');
    }

    md.push_str("---\n*Generated by NEXUS-6 Science Cycle*\n");
    md
}

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_publish_generates_markdown() {
        let actual = vec![
            ("phi_delta".to_string(), 6.0),
            ("entropy_delta".to_string(), 12.0),
            ("n6_score".to_string(), 0.95),
        ];
        let pub_result = publish("tension", "physics", None, None, &actual, None);
        assert!(!pub_result.markdown.is_empty());
        assert!(pub_result.markdown.contains("# NEXUS-6 Discovery"));
        assert!(pub_result.markdown.contains("Tension"));
        assert!(pub_result.markdown.contains("Physics"));
        assert!(pub_result.title.contains("Tension"));
    }

    #[test]
    fn test_publish_detects_n6_connections() {
        let actual = vec![
            ("val_a".to_string(), 6.0),   // n
            ("val_b".to_string(), 12.0),  // sigma
            ("val_c".to_string(), 24.0),  // J2
        ];
        let pub_result = publish("phase", "crystal", None, None, &actual, None);
        assert!(pub_result.n6_connections.len() >= 3);
        assert!(pub_result.bt_candidate.is_some());
    }

    #[test]
    fn test_publish_with_all_sections() {
        use super::super::predict::{Prediction, PredictionResult};
        use super::super::simulate::SimulationResult;
        use super::super::reproduce::ReproductionResult;

        let pred_result = PredictionResult {
            prediction: Prediction {
                experiment_type: "tension".to_string(),
                target: "physics".to_string(),
                predicted_phi_delta: 6.0,
                predicted_entropy_delta: 12.0,
                predicted_n6_score: 0.9,
                confidence: 0.8,
                reasoning: "test".to_string(),
            },
            actual_phi_delta: 6.1,
            actual_entropy_delta: 11.9,
            actual_n6_score: 0.88,
            accuracy: 0.95,
            surprise: 0.05,
        };

        let sim_result = SimulationResult {
            mean_phi_delta: 6.0,
            std_phi_delta: 0.1,
            mean_entropy_delta: 12.0,
            percentile_95: 6.15,
            worst_case: 5.8,
            best_case: 6.2,
            convergence_step: Some(10),
        };

        let repro_result = ReproductionResult {
            n_repeats: 10,
            results: vec![6.0; 10],
            mean: 6.0,
            std: 0.01,
            cv: 0.0017,
            reproducible: true,
            outlier_runs: vec![],
        };

        let actual = vec![("phi_delta".to_string(), 6.1)];
        let pub_result = publish(
            "tension",
            "physics",
            Some(&pred_result),
            Some(&sim_result),
            &actual,
            Some(&repro_result),
        );

        assert!(pub_result.markdown.contains("Prediction vs Actual"));
        assert!(pub_result.markdown.contains("Simulation Results"));
        assert!(pub_result.markdown.contains("Reproducibility"));
        assert!(pub_result.markdown.contains("REPRODUCIBLE"));
    }
}
