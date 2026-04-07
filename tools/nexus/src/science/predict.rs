/// Prediction engine — predict experiment outcomes before running them.

#[derive(Debug, Clone)]
pub struct Prediction {
    pub experiment_type: String,
    pub target: String,
    pub predicted_phi_delta: f64,
    pub predicted_entropy_delta: f64,
    pub predicted_n6_score: f64,
    pub confidence: f64,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub struct PredictionResult {
    pub prediction: Prediction,
    pub actual_phi_delta: f64,
    pub actual_entropy_delta: f64,
    pub actual_n6_score: f64,
    pub accuracy: f64,
    pub surprise: f64,
}

/// Predict experiment outcome based on historical data.
///
/// `history` contains tuples of (experiment_type, phi_delta, entropy_delta).
/// When no matching history exists, returns a conservative baseline prediction.
pub fn predict_experiment(
    experiment_type: &str,
    target: &str,
    history: &[(String, f64, f64)],
) -> Prediction {
    // Filter history for similar experiment types
    let similar: Vec<&(String, f64, f64)> = history
        .iter()
        .filter(|(t, _, _)| t == experiment_type)
        .collect();

    if similar.is_empty() {
        // No prior data — conservative baseline using n=6 defaults
        return Prediction {
            experiment_type: experiment_type.to_string(),
            target: target.to_string(),
            predicted_phi_delta: 0.0,
            predicted_entropy_delta: 0.0,
            predicted_n6_score: 0.5, // neutral
            confidence: 0.1,         // very low confidence
            reasoning: "No matching history — baseline prediction".to_string(),
        };
    }

    // Compute mean phi_delta and entropy_delta from similar experiments
    let n = similar.len() as f64;
    let mean_phi: f64 = similar.iter().map(|(_, p, _)| p).sum::<f64>() / n;
    let mean_entropy: f64 = similar.iter().map(|(_, _, e)| e).sum::<f64>() / n;

    // Compute standard deviation for confidence estimation
    let var_phi: f64 = similar.iter().map(|(_, p, _)| (p - mean_phi).powi(2)).sum::<f64>() / n;
    let std_phi = var_phi.sqrt();

    // Confidence scales with sample size and inversely with variance
    // More data + less variance = higher confidence (capped at 0.95)
    let confidence = (1.0 - 1.0 / (n + 1.0)) * (1.0 / (1.0 + std_phi)).min(1.0);
    let confidence = confidence.min(0.95);

    // Predicted n6_score: if phi_delta trends toward n=6 constants, score is higher
    let n6_score = compute_n6_affinity(mean_phi, mean_entropy);

    let reasoning = format!(
        "Based on {} similar '{}' experiments: mean phi_delta={:.4}, mean entropy_delta={:.4}, std_phi={:.4}",
        similar.len(), experiment_type, mean_phi, mean_entropy, std_phi
    );

    Prediction {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        predicted_phi_delta: mean_phi,
        predicted_entropy_delta: mean_entropy,
        predicted_n6_score: n6_score,
        confidence,
        reasoning,
    }
}

/// Evaluate a prediction against actual results.
pub fn evaluate_prediction(
    prediction: &Prediction,
    actual_phi: f64,
    actual_entropy: f64,
    actual_n6: f64,
) -> PredictionResult {
    // Accuracy: 1.0 - normalized error across all dimensions
    let phi_err = if actual_phi.abs() > 1e-12 {
        ((prediction.predicted_phi_delta - actual_phi) / actual_phi.abs()).abs()
    } else {
        prediction.predicted_phi_delta.abs()
    };

    let entropy_err = if actual_entropy.abs() > 1e-12 {
        ((prediction.predicted_entropy_delta - actual_entropy) / actual_entropy.abs()).abs()
    } else {
        prediction.predicted_entropy_delta.abs()
    };

    let n6_err = ((prediction.predicted_n6_score - actual_n6) / 1.0_f64.max(actual_n6.abs())).abs();

    let mean_err = (phi_err + entropy_err + n6_err) / 3.0;
    let accuracy = (1.0 - mean_err).max(0.0).min(1.0);

    // Surprise: how unexpected the result was (high = very different from prediction)
    let surprise = mean_err.min(1.0);

    PredictionResult {
        prediction: prediction.clone(),
        actual_phi_delta: actual_phi,
        actual_entropy_delta: actual_entropy,
        actual_n6_score: actual_n6,
        accuracy,
        surprise,
    }
}

/// Compute n=6 affinity score for given phi_delta and entropy_delta.
/// Higher score means closer alignment to n=6 constants.
fn compute_n6_affinity(phi_delta: f64, entropy_delta: f64) -> f64 {
    let n6_constants = [1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 11.0, 12.0, 24.0, 48.0, 144.0];

    // Find minimum relative distance to any n=6 constant
    let phi_abs = phi_delta.abs();
    let best_phi_dist = if phi_abs < 1e-12 {
        1.0 // zero delta is neutral
    } else {
        n6_constants
            .iter()
            .map(|&c| ((phi_abs - c) / c).abs())
            .fold(f64::INFINITY, f64::min)
    };

    let entropy_abs = entropy_delta.abs();
    let best_entropy_dist = if entropy_abs < 1e-12 {
        1.0
    } else {
        n6_constants
            .iter()
            .map(|&c| ((entropy_abs - c) / c).abs())
            .fold(f64::INFINITY, f64::min)
    };

    let avg_dist = (best_phi_dist + best_entropy_dist) / 2.0;
    (1.0 / (1.0 + avg_dist)).min(1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_predict_empty_history() {
        let pred = predict_experiment("tension", "physics", &[]);
        assert_eq!(pred.confidence, 0.1);
        assert_eq!(pred.predicted_phi_delta, 0.0);
    }

    #[test]
    fn test_predict_with_history() {
        let history = vec![
            ("tension".to_string(), 6.0, 12.0),
            ("tension".to_string(), 6.0, 12.0),
            ("fusion".to_string(), 1.0, 2.0),
        ];
        let pred = predict_experiment("tension", "physics", &history);
        assert!((pred.predicted_phi_delta - 6.0).abs() < 1e-9);
        assert!((pred.predicted_entropy_delta - 12.0).abs() < 1e-9);
        assert!(pred.confidence > 0.1);
    }

    #[test]
    fn test_evaluate_prediction() {
        let pred = Prediction {
            experiment_type: "tension".to_string(),
            target: "physics".to_string(),
            predicted_phi_delta: 6.0,
            predicted_entropy_delta: 12.0,
            predicted_n6_score: 0.8,
            confidence: 0.5,
            reasoning: "test".to_string(),
        };
        let result = evaluate_prediction(&pred, 6.0, 12.0, 0.8);
        assert!(result.accuracy > 0.99);
        assert!(result.surprise < 0.01);
    }
}
