use nexus::ouroboros::{
    ConvergenceChecker, ConvergenceStatus, CycleResult, EvolutionConfig, EvolutionEngine,
    MutationStrategy, mutate_hypothesis,
    mutation::mutate_with_strategy,
};

#[test]
fn test_single_evolve_step() {
    let config = EvolutionConfig {
        domain: "ai-efficiency".to_string(),
        ..EvolutionConfig::default()
    };
    let seeds = vec![
        "transformer attention head count is sigma=12".to_string(),
        "MoE top-k follows n=6 divisor pattern".to_string(),
    ];
    let mut engine = EvolutionEngine::new(config, seeds);

    let result = engine.evolve_step();

    assert_eq!(result.cycle, 1);
    assert_eq!(result.domain, "ai-efficiency");
    assert!(!result.lenses_used.is_empty());
    assert!(result.verification_score >= 0.0);
    assert!(result.verification_score <= 1.0);
    // graph_nodes is usize, just verify it's accessible
    let _ = result.graph_nodes;
    assert_eq!(engine.history.len(), 1);
}

#[test]
fn test_convergence_detection() {
    // Simulate a scenario where discoveries decrease to zero
    let checker = ConvergenceChecker::new(3, 3, 0.5);

    let history = vec![
        make_result(1, 5),
        make_result(2, 4),
        make_result(3, 3),
        make_result(4, 0),
        make_result(5, 0),
        make_result(6, 0),
    ];

    let status = checker.check(&history);
    assert_eq!(status, ConvergenceStatus::Saturated);
}

#[test]
fn test_convergence_exploring() {
    let checker = ConvergenceChecker::new(3, 3, 0.5);

    let history = vec![
        make_result(1, 5),
        make_result(2, 4),
        make_result(3, 5),
        make_result(4, 4),
    ];

    let status = checker.check(&history);
    assert_eq!(status, ConvergenceStatus::Exploring);
}

#[test]
fn test_mutation_strategies() {
    let base = "bandgap is 1.34 eV for optimal SQ efficiency in solar cells";

    let shift = mutate_with_strategy(base, MutationStrategy::ParameterShift);
    let transfer = mutate_with_strategy(base, MutationStrategy::DomainTransfer);
    let combo = mutate_with_strategy(base, MutationStrategy::Combination);
    let inv = mutate_with_strategy(base, MutationStrategy::Inversion);

    // Each strategy produces non-empty results
    assert!(!shift.is_empty(), "ParameterShift should produce results");
    assert!(!transfer.is_empty(), "DomainTransfer should produce results");
    assert!(!combo.is_empty(), "Combination should produce results");
    assert!(!inv.is_empty(), "Inversion should produce results");

    // Results are different across strategies
    assert_ne!(shift[0], transfer[0]);
    assert_ne!(combo[0], inv[0]);

    // ParameterShift should reference n=6 constants
    assert!(shift.iter().any(|s| s.contains("sigma")));
    assert!(shift.iter().any(|s| s.contains("tau")));

    // DomainTransfer should reference target domains
    assert!(transfer.iter().any(|s| s.contains("chip-architecture")));

    // Inversion should contain negation keywords
    assert!(inv.iter().any(|s| s.contains("NOT")));
}

#[test]
fn test_mutate_hypothesis_all_strategies() {
    let results = mutate_hypothesis("n=6 identity constrains transformer architecture");
    // Should have results from all 4 strategies
    assert!(results.len() >= 4, "Should produce at least 4 mutations, got {}", results.len());

    // Check that different strategy markers are present
    assert!(results.iter().any(|s| s.contains("[ParameterShift]")));
    assert!(results.iter().any(|s| s.contains("[DomainTransfer")));
    assert!(results.iter().any(|s| s.contains("[Combination")));
    assert!(results.iter().any(|s| s.contains("[Inversion]")));
}

#[test]
fn test_run_loop_terminates() {
    let config = EvolutionConfig {
        domain: "test-domain".to_string(),
        max_mutations_per_cycle: 2,
        min_verification_score: 0.99, // Very high → few discoveries → converges quickly
        ..EvolutionConfig::default()
    };
    let seeds = vec!["initial hypothesis".to_string()];
    let mut engine = EvolutionEngine::new(config, seeds);

    // Set convergence checker with small windows for fast convergence
    engine.convergence_checker = ConvergenceChecker::new(2, 2, 0.5);

    let (status, history) = engine.run_loop(20);

    // Should terminate (either by convergence or max_iterations)
    assert!(!history.is_empty(), "Should have completed at least one cycle");
    assert!(history.len() <= 20, "Should not exceed max_iterations");

    // If saturated, should have stopped early
    if status == ConvergenceStatus::Saturated {
        assert!(history.len() < 20, "Saturated should stop before max");
    }
}

#[test]
fn test_run_loop_max_iterations() {
    let config = EvolutionConfig {
        domain: "bounded-test".to_string(),
        ..EvolutionConfig::default()
    };
    let seeds = vec!["seed hypothesis for bounded test".to_string()];
    let mut engine = EvolutionEngine::new(config, seeds);

    // Use very large saturation window so it won't saturate
    engine.convergence_checker = ConvergenceChecker::new(100, 100, 0.5);

    let (_status, history) = engine.run_loop(5);
    assert_eq!(history.len(), 5, "Should run exactly max_iterations cycles");
}

#[test]
fn test_graph_grows_with_cycles() {
    let config = EvolutionConfig {
        domain: "graph-growth".to_string(),
        min_verification_score: 0.1, // Low bar → more discoveries → more nodes
        ..EvolutionConfig::default()
    };
    let seeds = vec![
        "sigma=12 heads optimal".to_string(),
        "J2=24 expert capacity".to_string(),
    ];
    let mut engine = EvolutionEngine::new(config, seeds);

    engine.evolve_step();
    let nodes_after_1 = engine.graph.nodes.len();

    engine.evolve_step();
    let nodes_after_2 = engine.graph.nodes.len();

    // Graph should grow (or at least not shrink)
    assert!(nodes_after_2 >= nodes_after_1);
}

// Helper to build a CycleResult for convergence testing
fn make_result(cycle: usize, discoveries: usize) -> CycleResult {
    CycleResult {
        cycle,
        domain: "test".to_string(),
        lenses_used: vec!["void".to_string()],
        new_discoveries: discoveries,
        graph_nodes: cycle * 2,
        graph_edges: cycle,
        verification_score: 0.5,
        lens_candidates_found: 0,
        lenses_evolved: 0,
    }
}
