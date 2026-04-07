use nexus::science::predict;
use nexus::science::simulate;
use nexus::science::compare;
use nexus::science::reproduce;
use nexus::science::publish;

#[test]
fn test_predict_from_empty_history() {
    let pred = predict::predict_experiment("tension", "physics", &[]);
    assert_eq!(pred.experiment_type, "tension");
    assert_eq!(pred.target, "physics");
    assert_eq!(pred.predicted_phi_delta, 0.0);
    assert_eq!(pred.predicted_entropy_delta, 0.0);
    assert_eq!(pred.confidence, 0.1);
    assert!(pred.reasoning.contains("No matching history"));
}

#[test]
fn test_predict_with_history() {
    let history = vec![
        ("tension".to_string(), 6.0, 12.0),
        ("tension".to_string(), 6.2, 11.8),
        ("tension".to_string(), 5.8, 12.2),
        ("fusion".to_string(), 24.0, 48.0), // different type, should be ignored
    ];
    let pred = predict::predict_experiment("tension", "physics", &history);
    // Mean of 6.0, 6.2, 5.8 = 6.0
    assert!((pred.predicted_phi_delta - 6.0).abs() < 0.01);
    // Mean of 12.0, 11.8, 12.2 = 12.0
    assert!((pred.predicted_entropy_delta - 12.0).abs() < 0.01);
    // With 3 data points, confidence should be > baseline
    assert!(pred.confidence > 0.1);
    assert!(pred.confidence <= 0.95);
}

#[test]
fn test_simulate_basic() {
    let config = simulate::SimulationConfig {
        experiment_type: "tension".to_string(),
        target: "physics".to_string(),
        n_simulations: 100,
        noise_level: 0.1,
        time_steps: 10,
    };
    let result = simulate::simulate(&config);

    // Mean should be near baseline 6.0 for tension
    assert!((result.mean_phi_delta - 6.0).abs() < 2.0,
        "mean_phi_delta={} expected near 6.0", result.mean_phi_delta);
    assert!(result.std_phi_delta >= 0.0);
    assert!(result.best_case >= result.worst_case);
    assert!(result.percentile_95 >= result.worst_case);
    assert!(result.percentile_95 <= result.best_case);
}

#[test]
fn test_compare_a_wins() {
    let a_metrics = vec![
        ("phi".to_string(), 10.0),
        ("entropy".to_string(), 8.0),
        ("n6_score".to_string(), 0.9),
    ];
    let b_metrics = vec![
        ("phi".to_string(), 5.0),
        ("entropy".to_string(), 4.0),
        ("n6_score".to_string(), 0.4),
    ];
    let result = compare::compare("experiment_A", &a_metrics, "experiment_B", &b_metrics);
    assert_eq!(result.winner, "A");
    assert!(result.effect_size > 0.0);
    assert!(result.statistically_significant);
    assert_eq!(result.details.len(), 3);
}

#[test]
fn test_compare_tie() {
    let a_metrics = vec![
        ("phi".to_string(), 6.0),
        ("entropy".to_string(), 12.0),
    ];
    let b_metrics = vec![
        ("phi".to_string(), 6.0),
        ("entropy".to_string(), 12.0),
    ];
    let result = compare::compare("X", &a_metrics, "Y", &b_metrics);
    assert_eq!(result.winner, "Tie");
}

#[test]
fn test_reproduce_consistent() {
    let config = reproduce::ReproductionConfig {
        experiment_type: "tension".to_string(),
        target: "physics".to_string(),
        n_repeats: 20,
        variation: 0.01,
    };
    let result = reproduce::reproduce(&config);
    assert_eq!(result.n_repeats, 20);
    assert_eq!(result.results.len(), 20);
    assert!(result.cv < 0.1, "CV={} should be < 0.1 for consistent results", result.cv);
    assert!(result.reproducible);
    assert!((result.mean - 6.0).abs() < 1.0);
}

#[test]
fn test_reproduce_outlier() {
    // Large variation should produce wider spread
    let config = reproduce::ReproductionConfig {
        experiment_type: "tension".to_string(),
        target: "physics".to_string(),
        n_repeats: 100,
        variation: 0.5,
    };
    let result = reproduce::reproduce(&config);
    assert_eq!(result.n_repeats, 100);
    assert_eq!(result.results.len(), 100);
    // Mean should still be somewhat near baseline
    assert!((result.mean - 6.0).abs() < 3.0);
    // Std should be larger than the consistent case
    assert!(result.std > 0.0);
}

#[test]
fn test_publish_generates_markdown() {
    let actual = vec![
        ("phi_delta".to_string(), 6.0),
        ("entropy_delta".to_string(), 12.0),
        ("n6_score".to_string(), 0.95),
    ];
    let publication = publish::publish("tension", "physics", None, None, &actual, None);

    assert!(!publication.markdown.is_empty());
    assert!(publication.markdown.contains("# NEXUS-6 Discovery"));
    assert!(publication.markdown.contains("Tension"));
    assert!(publication.markdown.contains("Physics"));
    assert!(publication.title.contains("Tension"));
    assert!(!publication.key_findings.is_empty());
    // 6.0 and 12.0 should match n=6 constants
    assert!(!publication.n6_connections.is_empty());
}

#[test]
fn test_full_cycle() {
    // Test the full pipeline: predict -> simulate -> evaluate -> compare -> reproduce -> publish
    let experiment_type = "tension";
    let target = "physics";

    // 1. Predict
    let prediction = predict::predict_experiment(experiment_type, target, &[]);
    assert_eq!(prediction.confidence, 0.1); // no history

    // 2. Simulate
    let sim_config = simulate::SimulationConfig {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        n_simulations: 50,
        noise_level: 0.1,
        time_steps: 6,
    };
    let sim_result = simulate::simulate(&sim_config);
    assert!((sim_result.mean_phi_delta - 6.0).abs() < 2.0);

    // 3. Evaluate prediction
    let pred_result = predict::evaluate_prediction(
        &prediction,
        sim_result.mean_phi_delta,
        sim_result.mean_entropy_delta,
        0.85,
    );
    assert!(pred_result.accuracy >= 0.0 && pred_result.accuracy <= 1.0);
    assert!(pred_result.surprise >= 0.0 && pred_result.surprise <= 1.0);

    // 4. Compare
    let a_metrics = vec![("phi".to_string(), sim_result.mean_phi_delta)];
    let b_metrics = vec![("phi".to_string(), prediction.predicted_phi_delta)];
    let cmp = compare::compare("simulation", &a_metrics, "prediction", &b_metrics);
    assert!(!cmp.winner.is_empty());

    // 5. Reproduce
    let repro_config = reproduce::ReproductionConfig {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        n_repeats: 10,
        variation: 0.05,
    };
    let repro = reproduce::reproduce(&repro_config);
    assert_eq!(repro.n_repeats, 10);
    assert!(repro.reproducible);

    // 6. Publish
    let actual = vec![
        ("phi_delta".to_string(), sim_result.mean_phi_delta),
        ("entropy_delta".to_string(), sim_result.mean_entropy_delta),
    ];
    let publication = publish::publish(
        experiment_type,
        target,
        Some(&pred_result),
        Some(&sim_result),
        &actual,
        Some(&repro),
    );
    assert!(publication.markdown.contains("Prediction vs Actual"));
    assert!(publication.markdown.contains("Simulation Results"));
    assert!(publication.markdown.contains("Reproducibility"));
    assert!(!publication.testable_predictions.is_empty());
}
