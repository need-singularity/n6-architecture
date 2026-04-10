use super::types::ExperimentResult;

/// Format experiment results as an ASCII table report.
pub fn format_report(results: &[ExperimentResult]) -> String {
    let mut out = String::new();

    out.push_str("┌────────────────┬──────────┬──────────┬──────────┬──────────┬──────────┬──────────┐\n");
    out.push_str("│ Experiment     │  Phi Δ   │ Entropy  │ Connect. │ Stable   │ N6 Score │ Breakpt  │\n");
    out.push_str("├────────────────┼──────────┼──────────┼──────────┼──────────┼──────────┼──────────┤\n");

    for r in results {
        let bp_str = match r.breakpoint {
            Some(v) => format!("{:.4}", v),
            None => "-".to_string(),
        };

        out.push_str(&format!(
            "│ {:<14} │ {:>+7.3} │ {:>+7.3} │ {:>+7.3} │ {:>+7.3} │ {:>7.4} │ {:>8} │\n",
            r.exp_type.name(),
            r.delta.phi,
            r.delta.entropy,
            r.delta.connectivity,
            r.delta.stability,
            r.after.n6_score,
            bp_str,
        ));
    }

    out.push_str("└────────────────┴──────────┴──────────┴──────────┴──────────┴──────────┴──────────┘\n");

    // Summary stats
    if !results.is_empty() {
        let total = results.len();
        let with_bp: usize = results.iter().filter(|r| r.breakpoint.is_some()).count();
        let total_discoveries: usize = results.iter().map(|r| r.discoveries.len()).sum();
        let avg_n6: f64 = results.iter().map(|r| r.after.n6_score).sum::<f64>() / total as f64;

        out.push_str(&format!("\n  Experiments: {}  |  Breakpoints: {}  |  Discoveries: {}  |  Avg N6: {:.4}\n",
            total, with_bp, total_discoveries, avg_n6));

        // List discoveries
        if total_discoveries > 0 {
            out.push_str("\n  Discoveries:\n");
            for r in results {
                for d in &r.discoveries {
                    out.push_str(&format!("    * {}\n", d));
                }
            }
        }
    }

    out
}

/// Format a single experiment result as a compact one-liner.
pub fn format_single(result: &ExperimentResult) -> String {
    let bp_str = match result.breakpoint {
        Some(v) => format!("bp={:.3}", v),
        None => "no-bp".to_string(),
    };
    format!(
        "[{}] phi={:+.3} entropy={:+.3} n6={:.4} {} disc={}",
        result.exp_type.name(),
        result.delta.phi,
        result.delta.entropy,
        result.after.n6_score,
        bp_str,
        result.discoveries.len(),
    )
}
