use nexus::experiment::types::{
    ExperimentConfig, ExperimentType, ALL_EXPERIMENT_TYPES,
};
use nexus::experiment::runner::ExperimentRunner;
use nexus::experiment::report;

#[test]
fn test_experiment_types_count() {
    // There must be exactly 22 experiment types (n=6 design: J_2+phi-tau = 22)
    assert_eq!(ALL_EXPERIMENT_TYPES.len(), 22);
}

#[test]
fn test_experiment_type_descriptions() {
    // Every type must have a non-empty description
    for t in &ALL_EXPERIMENT_TYPES {
        let desc = t.description();
        assert!(!desc.is_empty(), "{:?} has empty description", t);
    }
}

#[test]
fn test_experiment_type_recommended_lenses() {
    // Every type must recommend at least one lens
    for t in &ALL_EXPERIMENT_TYPES {
        let lenses = t.recommended_lenses();
        assert!(!lenses.is_empty(), "{:?} has no recommended lenses", t);
    }
}

#[test]
fn test_experiment_type_names_unique() {
    let names: Vec<&str> = ALL_EXPERIMENT_TYPES.iter().map(|t| t.name()).collect();
    let mut sorted = names.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(names.len(), sorted.len(), "Duplicate experiment type names detected");
}

#[test]
fn test_experiment_type_from_str() {
    assert_eq!(ExperimentType::from_str("acceleration"), Some(ExperimentType::Acceleration));
    assert_eq!(ExperimentType::from_str("COLLISION"), Some(ExperimentType::Collision));
    assert_eq!(ExperimentType::from_str("timewarp"), Some(ExperimentType::TimeWarp));
    assert_eq!(ExperimentType::from_str("symmetry_breaking"), Some(ExperimentType::SymmetryBreaking));
    assert_eq!(ExperimentType::from_str("nonexistent"), None);
}

#[test]
fn test_run_acceleration() {
    let runner = ExperimentRunner::new();
    let config = ExperimentConfig::new(ExperimentType::Acceleration, "physics")
        .with_intensity(0.7)
        .with_duration(6);
    let result = runner.run(&config);

    assert_eq!(result.exp_type, ExperimentType::Acceleration);
    assert!(result.before.phi >= 0.0);
    assert!(result.after.phi >= 0.0);
    assert!(result.breakpoint.is_none());
}

#[test]
fn test_run_collision() {
    let runner = ExperimentRunner::new();
    let config = ExperimentConfig::new(ExperimentType::Collision, "biology");
    let result = runner.run(&config);

    assert_eq!(result.exp_type, ExperimentType::Collision);
    assert!(result.after.n6_score > 0.0);
}

#[test]
fn test_run_tension_breakpoint() {
    let runner = ExperimentRunner::new();
    let config = ExperimentConfig::new(ExperimentType::Tension, "energy")
        .with_intensity(0.9)
        .with_duration(12);
    let result = runner.run(&config);

    assert_eq!(result.exp_type, ExperimentType::Tension);
    // Tension experiments always produce a breakpoint
    assert!(result.breakpoint.is_some());
    let bp = result.breakpoint.unwrap();
    assert!(bp >= 0.0 && bp <= 1.0, "breakpoint={} out of [0,1]", bp);
}

#[test]
fn test_run_all() {
    let runner = ExperimentRunner::new();
    let results = runner.run_all("cosmology");

    // Must produce exactly 22 results
    assert_eq!(results.len(), 22);

    // Each result should have the correct type
    for (result, &expected_type) in results.iter().zip(ALL_EXPERIMENT_TYPES.iter()) {
        assert_eq!(result.exp_type, expected_type);
    }
}

#[test]
fn test_run_battery() {
    let runner = ExperimentRunner::new();
    let types = vec![
        ExperimentType::Tension,
        ExperimentType::Compression,
        ExperimentType::Elasticity,
    ];
    let results = runner.run_battery(&types, "materials");

    assert_eq!(results.len(), 3);
    assert_eq!(results[0].exp_type, ExperimentType::Tension);
    assert_eq!(results[1].exp_type, ExperimentType::Compression);
    assert_eq!(results[2].exp_type, ExperimentType::Elasticity);
}

#[test]
fn test_report_format() {
    let runner = ExperimentRunner::new();
    let results = runner.run_all("test-domain");
    let report_str = report::format_report(&results);

    // Report should contain the table structure
    assert!(report_str.contains("Experiment"));
    assert!(report_str.contains("Phi"));
    assert!(report_str.contains("N6 Score"));
    assert!(report_str.contains("Breakpt"));

    // Should contain all experiment names
    assert!(report_str.contains("Acceleration"));
    assert!(report_str.contains("Collision"));
    assert!(report_str.contains("Tension"));
    assert!(report_str.contains("Friction"));

    // Summary line
    assert!(report_str.contains("Experiments: 22"));
}

#[test]
fn test_report_single() {
    let runner = ExperimentRunner::new();
    let config = ExperimentConfig::new(ExperimentType::Resonance, "audio");
    let result = runner.run(&config);
    let single = report::format_single(&result);

    assert!(single.contains("Resonance"));
    assert!(single.contains("n6="));
}

#[test]
fn test_metrics_delta() {
    use nexus::experiment::types::ExperimentMetrics;

    let a = ExperimentMetrics {
        phi: 1.0, entropy: 2.0, connectivity: 0.5,
        stability: 0.8, complexity: 0.3, n6_score: 0.9,
    };
    let b = ExperimentMetrics {
        phi: 0.5, entropy: 1.5, connectivity: 0.3,
        stability: 0.9, complexity: 0.2, n6_score: 0.7,
    };
    let delta = a.delta(&b);

    assert!((delta.phi - 0.5).abs() < 1e-10);
    assert!((delta.entropy - 0.5).abs() < 1e-10);
    assert!((delta.n6_score - 0.2).abs() < 1e-10);
}

#[test]
fn test_experiment_config_builder() {
    let config = ExperimentConfig::new(ExperimentType::Vibration, "test")
        .with_intensity(0.8)
        .with_duration(12);

    assert_eq!(config.exp_type, ExperimentType::Vibration);
    assert!((config.intensity - 0.8).abs() < 1e-10);
    assert_eq!(config.duration, 12);
    assert_eq!(config.target, "test");
}

#[test]
fn test_intensity_clamp() {
    let config = ExperimentConfig::new(ExperimentType::Mutation, "test")
        .with_intensity(5.0);
    assert!((config.intensity - 1.0).abs() < 1e-10);

    let config2 = ExperimentConfig::new(ExperimentType::Mutation, "test")
        .with_intensity(-1.0);
    assert!((config2.intensity - 0.0).abs() < 1e-10);
}

// CLI parser tests for experiment command
#[test]
fn test_parse_experiment_single() {
    use nexus::cli::parser::{parse_args, CliCommand, ExperimentMode};

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(|w| w.to_string()).collect()
    }

    let cmd = parse_args(&args("nexus experiment acceleration physics")).unwrap();
    match cmd {
        CliCommand::Experiment { exp_type, target, intensity, duration } => {
            assert_eq!(exp_type, ExperimentMode::Single("acceleration".to_string()));
            assert_eq!(target, "physics");
            assert!((intensity - 0.5).abs() < 1e-10); // default
            assert_eq!(duration, 6); // n=6 default
        }
        _ => panic!("Expected Experiment command"),
    }
}

#[test]
fn test_parse_experiment_all() {
    use nexus::cli::parser::{parse_args, CliCommand, ExperimentMode};

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(|w| w.to_string()).collect()
    }

    let cmd = parse_args(&args("nexus experiment all cosmology")).unwrap();
    match cmd {
        CliCommand::Experiment { exp_type, target, .. } => {
            assert_eq!(exp_type, ExperimentMode::All);
            assert_eq!(target, "cosmology");
        }
        _ => panic!("Expected Experiment command"),
    }
}

#[test]
fn test_parse_experiment_battery() {
    use nexus::cli::parser::{parse_args, CliCommand, ExperimentMode};

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(|w| w.to_string()).collect()
    }

    let cmd = parse_args(&args("nexus experiment battery tension,compression,elasticity materials")).unwrap();
    match cmd {
        CliCommand::Experiment { exp_type, target, .. } => {
            assert_eq!(exp_type, ExperimentMode::Battery(vec![
                "tension".to_string(),
                "compression".to_string(),
                "elasticity".to_string(),
            ]));
            assert_eq!(target, "materials");
        }
        _ => panic!("Expected Experiment command"),
    }
}

#[test]
fn test_parse_experiment_with_options() {
    use nexus::cli::parser::{parse_args, CliCommand, ExperimentMode};

    fn args(s: &str) -> Vec<String> {
        s.split_whitespace().map(|w| w.to_string()).collect()
    }

    let cmd = parse_args(&args("nexus experiment collision energy --intensity 0.8 --duration 12")).unwrap();
    match cmd {
        CliCommand::Experiment { exp_type, target, intensity, duration } => {
            assert_eq!(exp_type, ExperimentMode::Single("collision".to_string()));
            assert_eq!(target, "energy");
            assert!((intensity - 0.8).abs() < 1e-10);
            assert_eq!(duration, 12);
        }
        _ => panic!("Expected Experiment command"),
    }
}
