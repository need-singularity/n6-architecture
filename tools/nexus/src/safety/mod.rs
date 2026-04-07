//! Safety gate for NEXUS-6 autonomous operation.
//!
//! Checks Phi alignment, stability, and anomaly rates before allowing
//! autonomous actions. All default thresholds derive from n=6 constants.

/// Thresholds that must be met for autonomous operation.
#[derive(Debug, Clone)]
pub struct SafetyGate {
    /// Minimum Phi for autonomous operation (default: 1/(sigma-phi) = 0.1).
    pub phi_threshold: f64,
    /// Minimum stability score (default: 0.5).
    pub stability_threshold: f64,
    /// Maximum anomaly percentage (default: 1/sigma = 0.083).
    pub max_anomaly_rate: f64,
}

/// Result of a safety gate check.
#[derive(Debug, Clone)]
pub enum GateDecision {
    /// Safe to proceed, with confidence level.
    Allow { confidence: f64 },
    /// Blocked with reason and the failing metric values.
    Block {
        reason: String,
        values: Vec<(String, f64)>,
    },
}

/// Scan result metrics fed into the safety gate.
#[derive(Debug, Clone)]
pub struct ScanMetrics {
    /// Phi alignment score (0.0..1.0).
    pub phi_score: f64,
    /// Stability score (0.0..1.0).
    pub stability_score: f64,
    /// Anomaly rate as a fraction (0.0..1.0).
    pub anomaly_rate: f64,
    /// n6 EXACT ratio (0.0..1.0).
    pub n6_exact_ratio: f64,
}

/// Return n=6-aligned default safety gate.
///
/// - phi_threshold = 1/(sigma - phi) = 1/10 = 0.1
/// - stability_threshold = 0.5
/// - max_anomaly_rate = 1/sigma = 1/12 ~ 0.0833
pub fn default_gate() -> SafetyGate {
    SafetyGate {
        phi_threshold: 0.1,        // 1/(sigma-phi) = 1/10
        stability_threshold: 0.5,
        max_anomaly_rate: 1.0 / 12.0, // 1/sigma ~ 0.0833
    }
}

/// Check whether the given scan metrics pass the safety gate.
///
/// Returns `Allow` with a confidence proportional to how far above thresholds
/// the metrics are, or `Block` listing every failing metric.
pub fn check_gate(metrics: &ScanMetrics, gate: &SafetyGate) -> GateDecision {
    let mut failures: Vec<(String, f64)> = Vec::new();

    if metrics.phi_score < gate.phi_threshold {
        failures.push(("phi_score".to_string(), metrics.phi_score));
    }
    if metrics.stability_score < gate.stability_threshold {
        failures.push(("stability_score".to_string(), metrics.stability_score));
    }
    if metrics.anomaly_rate > gate.max_anomaly_rate {
        failures.push(("anomaly_rate".to_string(), metrics.anomaly_rate));
    }

    if failures.is_empty() {
        // Confidence: average of how far each metric exceeds its threshold,
        // clamped to [0, 1].
        let phi_margin = if gate.phi_threshold > 0.0 {
            (metrics.phi_score / gate.phi_threshold).min(2.0) / 2.0
        } else {
            1.0
        };
        let stab_margin = if gate.stability_threshold > 0.0 {
            (metrics.stability_score / gate.stability_threshold).min(2.0) / 2.0
        } else {
            1.0
        };
        let anom_margin = if gate.max_anomaly_rate > 0.0 {
            (1.0 - metrics.anomaly_rate / gate.max_anomaly_rate).max(0.0)
        } else {
            if metrics.anomaly_rate == 0.0 { 1.0 } else { 0.0 }
        };
        // Bonus for high n6 EXACT ratio
        let n6_bonus = metrics.n6_exact_ratio * 0.1;

        let confidence = ((phi_margin + stab_margin + anom_margin) / 3.0 + n6_bonus).min(1.0);
        GateDecision::Allow { confidence }
    } else {
        let reasons: Vec<String> = failures
            .iter()
            .map(|(name, val)| format!("{} = {:.4}", name, val))
            .collect();
        GateDecision::Block {
            reason: format!("Safety gate blocked: {}", reasons.join(", ")),
            values: failures,
        }
    }
}

/// Format a gate decision as a human-readable string.
pub fn format_decision(decision: &GateDecision) -> String {
    match decision {
        GateDecision::Allow { confidence } => {
            format!("ALLOW (confidence: {:.2}%)", confidence * 100.0)
        }
        GateDecision::Block { reason, .. } => {
            format!("BLOCK: {}", reason)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_gate_values() {
        let gate = default_gate();
        assert!((gate.phi_threshold - 0.1).abs() < 1e-9);
        assert!((gate.stability_threshold - 0.5).abs() < 1e-9);
        assert!((gate.max_anomaly_rate - 1.0 / 12.0).abs() < 1e-9);
    }

    #[test]
    fn test_allow_good_metrics() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.8,
            stability_score: 0.9,
            anomaly_rate: 0.01,
            n6_exact_ratio: 0.85,
        };
        let decision = check_gate(&metrics, &gate);
        match decision {
            GateDecision::Allow { confidence } => {
                assert!(confidence > 0.5, "confidence should be high: {}", confidence);
            }
            GateDecision::Block { reason, .. } => {
                panic!("Expected Allow, got Block: {}", reason);
            }
        }
    }

    #[test]
    fn test_block_low_phi() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.05, // below 0.1 threshold
            stability_score: 0.9,
            anomaly_rate: 0.01,
            n6_exact_ratio: 0.5,
        };
        let decision = check_gate(&metrics, &gate);
        match decision {
            GateDecision::Block { values, .. } => {
                assert!(values.iter().any(|(n, _)| n == "phi_score"));
            }
            GateDecision::Allow { .. } => {
                panic!("Expected Block for low phi_score");
            }
        }
    }

    #[test]
    fn test_block_high_anomaly() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.5,
            stability_score: 0.7,
            anomaly_rate: 0.2, // above 1/12 ~ 0.083 threshold
            n6_exact_ratio: 0.5,
        };
        let decision = check_gate(&metrics, &gate);
        match decision {
            GateDecision::Block { values, .. } => {
                assert!(values.iter().any(|(n, _)| n == "anomaly_rate"));
            }
            GateDecision::Allow { .. } => {
                panic!("Expected Block for high anomaly_rate");
            }
        }
    }

    #[test]
    fn test_block_multiple_failures() {
        let gate = default_gate();
        let metrics = ScanMetrics {
            phi_score: 0.01,
            stability_score: 0.1,
            anomaly_rate: 0.5,
            n6_exact_ratio: 0.0,
        };
        let decision = check_gate(&metrics, &gate);
        match decision {
            GateDecision::Block { values, reason } => {
                assert_eq!(values.len(), 3, "all 3 metrics should fail");
                assert!(reason.contains("phi_score"));
                assert!(reason.contains("stability_score"));
                assert!(reason.contains("anomaly_rate"));
            }
            GateDecision::Allow { .. } => {
                panic!("Expected Block for all-bad metrics");
            }
        }
    }

    #[test]
    fn test_format_decision() {
        let allow = GateDecision::Allow { confidence: 0.85 };
        let s = format_decision(&allow);
        assert!(s.contains("ALLOW"));
        assert!(s.contains("85.00%"));

        let block = GateDecision::Block {
            reason: "test".to_string(),
            values: vec![("x".to_string(), 0.0)],
        };
        let s = format_decision(&block);
        assert!(s.contains("BLOCK"));
    }
}
