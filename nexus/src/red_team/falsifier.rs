use serde::{Deserialize, Serialize};

/// Report on the falsifiability of a discovery.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct FalsifiabilityReport {
    pub is_falsifiable: bool,
    pub testable_predictions: Vec<String>,
    pub potential_counterexamples: Vec<String>,
    pub popper_score: f64, // 0.0 = unfalsifiable, 1.0 = easily falsifiable
}

/// Keyword sets for heuristic falsifiability analysis.
const QUANTITATIVE_HINTS: &[&str] = &[
    "=", ">", "<", "ratio", "percent", "factor", "times", "eV",
    "nm", "Hz", "W", "V", "A", "K", "mol", "kg",
];

const PREDICTION_HINTS: &[&str] = &[
    "predict", "should be", "must be", "expect", "implies",
    "therefore", "consequently", "if then",
];

const VAGUE_HINTS: &[&str] = &[
    "may", "might", "could", "possibly", "perhaps", "sometimes",
    "generally", "often", "tends to",
];

const UNIVERSAL_HINTS: &[&str] = &[
    "all", "every", "always", "universal", "never", "no exception",
    "invariant", "conservation",
];

/// Assess the falsifiability of a discovery text.
///
/// Uses heuristics to determine:
/// 1. Whether the statement makes testable predictions
/// 2. What potential counterexamples exist
/// 3. A Popper score (higher = more falsifiable = better science)
pub fn assess_falsifiability(discovery: &str) -> FalsifiabilityReport {
    let lower = discovery.to_lowercase();

    // Count quantitative content
    let quant_count = QUANTITATIVE_HINTS
        .iter()
        .filter(|h| lower.contains(&h.to_lowercase()))
        .count();

    // Count prediction language
    let pred_count = PREDICTION_HINTS
        .iter()
        .filter(|h| lower.contains(&h.to_lowercase()))
        .count();

    // Count vague language (reduces falsifiability)
    let vague_count = VAGUE_HINTS
        .iter()
        .filter(|h| lower.contains(&h.to_lowercase()))
        .count();

    // Count universal claims (increases falsifiability)
    let universal_count = UNIVERSAL_HINTS
        .iter()
        .filter(|h| lower.contains(&h.to_lowercase()))
        .count();

    // Has numerical values?
    let has_numbers = discovery
        .split_whitespace()
        .any(|w| w.trim_matches(|c: char| !c.is_ascii_digit() && c != '.').parse::<f64>().is_ok());

    // Generate testable predictions
    let mut testable_predictions = Vec::new();
    if has_numbers {
        testable_predictions.push("Verify numerical values against empirical data".to_string());
    }
    if quant_count > 0 {
        testable_predictions.push("Compare quantitative predictions with measurements".to_string());
    }
    if universal_count > 0 {
        testable_predictions.push("Search for counterexamples to universal claim".to_string());
    }
    if pred_count > 0 {
        testable_predictions.push("Test explicit predictions against observations".to_string());
    }
    if lower.contains("n=6") || lower.contains("sigma") || lower.contains("phi") {
        testable_predictions.push("Verify n=6 constant match with independent calculation".to_string());
    }

    // Generate potential counterexamples
    let mut potential_counterexamples = Vec::new();
    if has_numbers {
        potential_counterexamples.push("Find a system where these numerical values differ significantly".to_string());
    }
    if universal_count > 0 {
        potential_counterexamples.push("Find a single exception to the universal claim".to_string());
    }
    if lower.contains("cross-domain") || lower.contains("bridge") {
        potential_counterexamples.push("Find a domain where the bridge/cross-domain pattern breaks".to_string());
    }
    if lower.contains("exact") {
        potential_counterexamples.push("Show that the match is only approximate, not exact".to_string());
    }

    // Calculate Popper score
    let positive = quant_count as f64 * 0.15
        + pred_count as f64 * 0.2
        + universal_count as f64 * 0.15
        + if has_numbers { 0.2 } else { 0.0 };
    let negative = vague_count as f64 * 0.15;
    let raw_score = (positive - negative).max(0.0).min(1.0);

    // Clamp: no predictions at all -> very low score
    let popper_score = if testable_predictions.is_empty() {
        raw_score * 0.3
    } else {
        raw_score
    };

    let is_falsifiable = popper_score >= 0.2 && !testable_predictions.is_empty();

    FalsifiabilityReport {
        is_falsifiable,
        testable_predictions,
        potential_counterexamples,
        popper_score,
    }
}

/// Format a falsifiability report for display.
pub fn format_report(report: &FalsifiabilityReport) -> String {
    let mut out = String::new();
    out.push_str(&format!(
        "Falsifiable: {}  (Popper score: {:.2})\n",
        if report.is_falsifiable { "YES" } else { "NO" },
        report.popper_score
    ));

    if !report.testable_predictions.is_empty() {
        out.push_str("Testable predictions:\n");
        for p in &report.testable_predictions {
            out.push_str(&format!("  + {}\n", p));
        }
    }

    if !report.potential_counterexamples.is_empty() {
        out.push_str("Potential counterexamples:\n");
        for c in &report.potential_counterexamples {
            out.push_str(&format!("  - {}\n", c));
        }
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_falsifiable_quantitative() {
        let report = assess_falsifiability(
            "sigma = 12 predicts all transformer heads should be divisible by 12",
        );
        assert!(report.is_falsifiable);
        assert!(report.popper_score > 0.3);
        assert!(!report.testable_predictions.is_empty());
    }

    #[test]
    fn test_unfalsifiable_vague() {
        let report = assess_falsifiability(
            "Perhaps things might sometimes tend to be somewhat related",
        );
        assert!(!report.is_falsifiable);
        assert!(report.popper_score < 0.3);
    }

    #[test]
    fn test_universal_claim() {
        let report = assess_falsifiability(
            "All battery cathodes universally have CN=6 coordination, never exception",
        );
        assert!(report.is_falsifiable);
        assert!(!report.potential_counterexamples.is_empty());
    }

    #[test]
    fn test_format_report() {
        let report = assess_falsifiability("sigma = 12 exact match in physics");
        let formatted = format_report(&report);
        assert!(formatted.contains("Popper score"));
    }

    #[test]
    fn test_n6_specific_prediction() {
        let report = assess_falsifiability("n=6 sigma constant predicts GPU SM count");
        assert!(report
            .testable_predictions
            .iter()
            .any(|p| p.contains("n=6")));
    }
}
