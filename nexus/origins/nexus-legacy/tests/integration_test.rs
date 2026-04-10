//! NEXUS-6 Phase 10 — End-to-End Integration Tests
//!
//! These tests verify that the full pipeline works when modules are composed
//! together, simulating real discovery workflows.

use std::collections::HashMap;

use nexus::encoder::parser::parse_hypotheses;
use nexus::encoder::vectorize::vectorize;
use nexus::graph::edge::{Edge, EdgeType};
use nexus::graph::node::{Node, NodeType};
use nexus::graph::persistence::DiscoveryGraph;
use nexus::history::recorder::ScanRecord;
use nexus::history::stats::{compute_domain_stats, compute_lens_affinity};
use nexus::history::recommend::recommend_lenses;
use nexus::ouroboros::{EvolutionConfig, EvolutionEngine};
use nexus::ouroboros::mutation::mutate_hypothesis;
use nexus::telescope::consensus::{weighted_consensus, ConsensusLevel};
use nexus::telescope::domain_combos::default_combos;
use nexus::telescope::registry::{LensCategory, LensRegistry};
use nexus::telescope::Telescope;
use nexus::verifier::feasibility;
use nexus::verifier::n6_check;

// ─── Helpers ───────────────────────────────────────────────────────────────

fn make_record(
    id: &str,
    domain: &str,
    lenses: &[&str],
    discoveries: &[&str],
    consensus: usize,
) -> ScanRecord {
    ScanRecord {
        id: id.to_string(),
        timestamp: "2026-04-03T00:00:00Z".to_string(),
        domain: domain.to_string(),
        lenses_used: lenses.iter().map(|s| s.to_string()).collect(),
        discoveries: discoveries.iter().map(|s| s.to_string()).collect(),
        consensus_level: consensus,
    }
}

fn make_node(id: &str, domain: &str, confidence: f64, lenses: &[&str]) -> Node {
    Node {
        id: id.to_string(),
        node_type: NodeType::Discovery,
        domain: domain.to_string(),
        project: "nexus".to_string(),
        summary: format!("Discovery {} in {}", id, domain),
        confidence,
        lenses_used: lenses.iter().map(|s| s.to_string()).collect(),
        timestamp: "2026-04-03T00:00:00Z".to_string(),
    }
}

fn make_edge(from: &str, to: &str, strength: f64, bidir: bool) -> Edge {
    Edge {
        from: from.to_string(),
        to: to.to_string(),
        edge_type: EdgeType::Derives,
        strength,
        bidirectional: bidir,
    }
}

// ─── Test 1: Full Scan Pipeline ────────────────────────────────────────────

#[test]
fn test_full_scan_pipeline() {
    // 1. LensRegistry: select lenses for "energy" domain
    let registry = LensRegistry::new();
    let energy_lenses = registry.for_domain("energy");
    assert!(!energy_lenses.is_empty(), "Registry should find lenses for 'energy' domain");

    let lens_names: Vec<String> = energy_lenses.iter().map(|e| e.name.clone()).collect();

    // 2. Telescope scan with sample data (n=6 constants + noise)
    let telescope = Telescope::new();
    let data = vec![6.0, 12.0, 2.0, 4.0, 24.0, 5.0, 7.3, 15.1, 3.8];
    let scan_results = telescope.scan_all(&data, data.len(), 1);
    assert!(!scan_results.is_empty(), "Telescope should return results");

    // 3. Verifier: n6_check + feasibility_score
    let n6_ratio = n6_check::n6_exact_ratio(&data);
    assert!(n6_ratio > 0.0, "Some values should match n=6 constants");

    let verification = feasibility::verify(
        0.8,      // lens_consensus
        0.6,      // cross_validation
        0.7,      // physical_check
        0.1,      // graph_bonus
        0.5,      // novelty
        n6_ratio, // n6_exact
    );
    assert!(verification.score > 0.0, "Verification score should be positive");
    assert!(!verification.grade.label().is_empty(), "Grade should have a label");

    // 4. Graph: add discovery node and edge
    let mut graph = DiscoveryGraph::new();
    graph.add_node(make_node("int-d1", "energy", verification.score, &["thermo", "stability"]));
    graph.add_node(make_node("int-d2", "energy", 0.6, &["boundary", "thermo"]));
    graph.add_edge(make_edge("int-d1", "int-d2", verification.score, true));
    assert_eq!(graph.nodes.len(), 2);
    assert_eq!(graph.edges.len(), 1);

    // 5. History: record the scan
    let record = ScanRecord {
        id: "int-scan-1".to_string(),
        timestamp: "2026-04-03T00:00:00Z".to_string(),
        domain: "energy".to_string(),
        lenses_used: lens_names,
        discoveries: vec!["Energy pattern found".to_string()],
        consensus_level: 3,
    };
    // Verify record was constructed correctly
    assert_eq!(record.domain, "energy");
    assert!(!record.discoveries.is_empty());

    // Full pipeline completed without errors
}

// ─── Test 2: Evolution with Registry ───────────────────────────────────────

#[test]
fn test_evolution_with_registry() {
    // 1. Extract Core lens names from registry
    let registry = LensRegistry::new();
    let core_lenses = registry.by_category(LensCategory::Core);
    assert_eq!(core_lenses.len(), 23, "Should have 23 Core lenses (22 + chaos)");

    let core_names: Vec<String> = core_lenses.iter().map(|e| e.name.clone()).collect();

    // 2. Create EvolutionEngine with registry lens names
    let config = EvolutionConfig {
        domain: "chip-architecture".to_string(),
        all_lenses: core_names.clone(),
        serendipity_ratio: 0.2,
        min_verification_score: 0.3,
        max_mutations_per_cycle: 6,
    };
    let seeds = vec!["SM count = sigma^2 = 144 in top GPUs".to_string()];
    let mut engine = EvolutionEngine::new(config, seeds);

    // 3. Execute one evolution step
    let result = engine.evolve_step();

    // 4. Verify CycleResult structure
    assert_eq!(result.cycle, 1);
    assert_eq!(result.domain, "chip-architecture");
    assert!(!result.lenses_used.is_empty(), "Should use some lenses");
    assert!(result.verification_score >= 0.0, "Score should be non-negative");
    // graph_nodes is usize, always >= 0; just verify it's accessible
    let _ = result.graph_nodes;
}

// ─── Test 3: History Recommend Loop ────────────────────────────────────────

#[test]
fn test_history_recommend_loop() {
    // 1. Build scan records with diverse lenses and discoveries
    let records = vec![
        make_record("r1", "physics", &["consciousness", "topology", "causal"], &["disc-A", "disc-B"], 3),
        make_record("r2", "physics", &["topology", "wave", "quantum"], &["disc-C"], 3),
        make_record("r3", "physics", &["thermo", "boundary", "stability"], &[], 2),
        make_record("r4", "physics", &["consciousness", "topology", "network"], &["disc-D"], 3),
        make_record("r5", "physics", &["wave", "quantum", "causal"], &["disc-E", "disc-F"], 3),
    ];

    // 2. Compute DomainStats
    let stats = compute_domain_stats(&records);
    assert_eq!(stats.total_scans, 5);
    assert_eq!(stats.total_discoveries, 6);
    assert!(!stats.lens_stats.is_empty());

    // 3. Recommend lenses
    let all_lenses: Vec<String> = vec![
        "consciousness", "topology", "causal", "wave", "quantum",
        "thermo", "boundary", "stability", "network", "evolution",
        "info", "em",
    ].into_iter().map(String::from).collect();

    let mut all_stats = HashMap::new();
    all_stats.insert("physics".to_string(), stats);

    let recommendation = recommend_lenses("physics", &all_stats, &all_lenses, 0.2);
    assert!(!recommendation.lenses.is_empty(), "Should recommend at least some lenses");
    assert!(recommendation.lenses.len() >= 4, "Minimum 4 lenses enforced");

    // 4. Verify recommended lenses exist in the registry
    let registry = LensRegistry::new();
    for lens_name in &recommendation.lenses {
        // All recommended lenses should either be in the registry or in the all_lenses pool
        let in_registry = registry.get(lens_name).is_some();
        let in_pool = all_lenses.contains(lens_name);
        assert!(
            in_registry || in_pool,
            "Recommended lens '{}' should exist in registry or pool",
            lens_name
        );
    }
}

// ─── Test 4: Graph Convergence Check ───────────────────────────────────────

#[test]
fn test_graph_convergence_check() {
    let mut graph = DiscoveryGraph::new();

    // Add 6 nodes (n=6) forming a connected structure
    for i in 0..6 {
        graph.add_node(make_node(
            &format!("n{}", i),
            "ai",
            0.5 + (i as f64) * 0.08,
            &["consciousness", "topology"],
        ));
    }

    // Add bidirectional edges to form triangles and a hub
    // Triangle 1: n0-n1-n2
    graph.add_edge(make_edge("n0", "n1", 0.9, true));
    graph.add_edge(make_edge("n1", "n2", 0.8, true));
    graph.add_edge(make_edge("n0", "n2", 0.85, true));

    // Triangle 2: n0-n3-n4
    graph.add_edge(make_edge("n0", "n3", 0.7, true));
    graph.add_edge(make_edge("n3", "n4", 0.75, true));
    graph.add_edge(make_edge("n0", "n4", 0.65, true));

    // Extra edges from n0 (making it a hub) and to n5
    graph.add_edge(make_edge("n0", "n5", 0.6, true));
    graph.add_edge(make_edge("n2", "n5", 0.55, true));

    // 1. Hub scores: n0 should be a hub (degree >= 5)
    let hubs = graph.hubs(3);
    assert!(!hubs.is_empty(), "Should find at least one hub");
    let top_hub = &hubs[0];
    assert_eq!(top_hub.node_id, "n0", "n0 should be the top hub");

    // 2. Find triangles
    let triangles = graph.closed_loops();
    assert!(
        !triangles.is_empty(),
        "Should find at least one closed triangle"
    );

    // Verify triangle strength is consistent (average of edge strengths)
    for tri in &triangles {
        assert_eq!(tri.nodes.len(), 3, "Each triangle has exactly 3 nodes");
        assert!(tri.strength > 0.0, "Triangle strength should be positive");
    }

    // 3. Convergences: nodes with 2+ incoming sources
    let convergences = graph.convergences();
    // With bidirectional edges, several nodes should have 2+ sources
    assert!(
        !convergences.is_empty(),
        "Should find convergence points"
    );
}

// ─── Test 5: Encoder → Materials → Verifier Chain ──────────────────────────

#[test]
fn test_encoder_materials_verifier_chain() {
    // 1. Encoder: parse hypothesis markdown
    let md = "\
## H-SC-01: MgB2 superconductor
- Tc = 39 K
- Pressure = 0 GPa
- Layers = 6

## H-SC-02: YBCO cuprate
- Tc = 93 K
- Pressure = 0 GPa
- Layers = 12

## H-SC-03: Optimal n6 material
- Tc = 24 K
- Pressure = 4 GPa
- Layers = 6
";
    let entries = parse_hypotheses(md);
    assert_eq!(entries.len(), 3, "Should parse 3 hypotheses");
    assert_eq!(entries[0].get("id").unwrap(), "H-SC-01");

    // 2. Vectorize the parsed entries
    let feature_keys = &["Tc", "Pressure", "Layers"];
    let (data, n_rows, n_cols) = vectorize(&entries, feature_keys);
    assert_eq!(n_rows, 3);
    assert_eq!(n_cols, 3);
    assert_eq!(data.len(), 9);

    // Extract individual feature values for n6 check
    let _tc_values: Vec<f64> = (0..n_rows).map(|i| data[i * n_cols]).collect();
    let layer_values: Vec<f64> = (0..n_rows).map(|i| data[i * n_cols + 2]).collect();

    // 3. Verifier: check n=6 alignment of layer counts
    // Layers = [6, 12, 6] should match n=6 constants
    let layer_ratio = n6_check::n6_exact_ratio(&layer_values);
    assert!(
        layer_ratio > 0.5,
        "Most layer values should match n=6 constants (6=n, 12=sigma), got {}",
        layer_ratio
    );

    // Check individual values
    let (name_6, quality_6) = n6_check::n6_match(6.0);
    assert_eq!(name_6, "n");
    assert_eq!(quality_6, 1.0);

    let (name_12, quality_12) = n6_check::n6_match(12.0);
    assert_eq!(name_12, "sigma");
    assert_eq!(quality_12, 1.0);

    // 4. Verify Tc=24 matches J2
    let (name_24, quality_24) = n6_check::n6_match(24.0);
    assert_eq!(quality_24, 1.0, "24 should be EXACT match");
    // J2=24 or sigma*phi=24
    assert!(name_24 == "J2" || name_24 == "sigma*phi", "24 should match J2 or sigma*phi");

    // 5. Full verification with extracted data
    let all_values: Vec<f64> = data.iter().copied().filter(|v| !v.is_nan()).collect();
    let n6_ratio = n6_check::n6_exact_ratio(&all_values);

    let verification = feasibility::verify(0.7, 0.5, 0.6, 0.1, 0.3, n6_ratio);
    assert!(verification.score > 0.3, "Chain verification score should be above minimum threshold");
}

// ─── Test 6: Domain Combo Scan ─────────────────────────────────────────────

#[test]
fn test_domain_combo_scan() {
    let combos = default_combos();
    assert_eq!(combos.len(), 10, "Should have 10 domain combos");

    // Pick the "stability" combo (lenses: stability, boundary, thermo)
    let stability_combo = combos
        .iter()
        .find(|c| c.name == "stability")
        .expect("stability combo should exist");
    assert_eq!(stability_combo.lenses.len(), 3);

    // Scan with Telescope
    let telescope = Telescope::new();
    let data = vec![6.0, 12.0, 24.0, 4.0, 8.0, 10.0, 5.0, 48.0, 144.0];
    let scan_results = telescope.scan_all(&data, data.len(), 1);

    // Build consensus from scan results
    let mut hit_rates: HashMap<String, f64> = HashMap::new();
    for lens_name in scan_results.keys() {
        hit_rates.insert(lens_name.clone(), 1.0);
    }

    let consensus = weighted_consensus(&scan_results, &hit_rates);
    // Consensus results may be empty if fewer than 3 lenses agree on a pattern,
    // but the structure should be valid
    for cr in &consensus {
        assert!(!cr.pattern_id.is_empty(), "Pattern ID should not be empty");
        assert!(!cr.agreeing_lenses.is_empty(), "Should have agreeing lenses");
        assert!(cr.weighted_score > 0.0, "Weighted score should be positive");
        match cr.level {
            ConsensusLevel::Candidate => assert!(cr.agreeing_lenses.len() >= 3),
            ConsensusLevel::High => assert!(cr.agreeing_lenses.len() >= 7),
            ConsensusLevel::Confirmed => assert!(cr.agreeing_lenses.len() >= 12),
        }
    }

    // Verify that scan_results contain expected lens names
    assert!(
        scan_results.contains_key("VoidLens") || scan_results.contains_key("BarrierLens"),
        "Should contain at least one built-in lens result"
    );
}

// ─── Test 7: Mutation → Verify → Graph Cycle ──────────────────────────────

#[test]
fn test_mutation_verify_graph_cycle() {
    let base_hypothesis = "Transformer attention head count equals sigma=12";

    // 1. Generate mutations
    let mutations = mutate_hypothesis(base_hypothesis);
    assert!(
        mutations.len() >= 10,
        "Should generate many mutations (param shifts + domain transfers + combos + inversions), got {}",
        mutations.len()
    );

    // 2. Verify each mutation and collect passing ones
    let mut graph = DiscoveryGraph::new();
    let mut pass_count = 0;

    for (i, mutation) in mutations.iter().enumerate() {
        // Generate scan data from mutation text (simple deterministic approach)
        let hash_val: usize = mutation.bytes().map(|b| b as usize).sum();
        let synthetic_values = vec![
            6.0,
            12.0,
            (hash_val % 50) as f64,
            24.0,
            (hash_val % 30) as f64 + 0.5,
        ];

        let n6_ratio = n6_check::n6_exact_ratio(&synthetic_values);
        let verification = feasibility::verify(0.5, 0.4, 0.5, 0.0, 0.3, n6_ratio);

        // 3. Add to graph only if passing threshold
        if verification.score >= 0.3 {
            pass_count += 1;
            let node_id = format!("mut-{}", i);
            graph.add_node(Node {
                id: node_id.clone(),
                node_type: NodeType::Hypothesis,
                domain: "ai".to_string(),
                project: "nexus".to_string(),
                summary: mutation.chars().take(120).collect(),
                confidence: verification.score,
                lenses_used: vec!["consciousness".to_string(), "topology".to_string()],
                timestamp: "2026-04-03T00:00:00Z".to_string(),
            });

            // Connect to previous node if exists
            if pass_count > 1 {
                let prev_id = format!("mut-{}", i - 1);
                // Only add edge if the previous node was also added
                if graph.nodes.iter().any(|n| n.id == prev_id) {
                    graph.add_edge(Edge {
                        from: prev_id,
                        to: node_id,
                        edge_type: EdgeType::Derives,
                        strength: verification.score,
                        bidirectional: false,
                    });
                }
            }
        }
    }

    // 4. Verify graph state
    assert_eq!(
        graph.nodes.len(),
        pass_count,
        "Graph nodes should match passing mutation count"
    );
    assert!(
        pass_count > 0,
        "At least some mutations should pass verification"
    );
}

// ─── Test 8: Lens Affinity with Real Data ──────────────────────────────────

#[test]
fn test_lens_affinity_with_real_data() {
    // Build diverse scan records simulating a real discovery session
    let records = vec![
        make_record("a1", "ai", &["consciousness", "topology", "causal"], &["disc-1"], 3),
        make_record("a2", "ai", &["consciousness", "topology", "quantum"], &["disc-2"], 3),
        make_record("a3", "ai", &["topology", "causal", "network"], &["disc-3"], 3),
        make_record("a4", "ai", &["consciousness", "causal", "info"], &["disc-4", "disc-5"], 3),
        make_record("a5", "ai", &["wave", "quantum", "memory"], &[], 2),
        make_record("a6", "ai", &["consciousness", "topology", "causal", "network"], &["disc-6"], 4),
        make_record("a7", "ai", &["topology", "quantum", "mirror"], &["disc-7"], 3),
        make_record("a8", "ai", &["consciousness", "causal", "evolution"], &["disc-8"], 3),
        make_record("a9", "ai", &["thermo", "boundary", "stability"], &[], 2),
        make_record("a10", "ai", &["consciousness", "topology", "info", "causal"], &["disc-9", "disc-10"], 4),
    ];

    // Compute lens affinity
    let affinity = compute_lens_affinity(&records);

    // Should have affinity pairs from discovery-producing records
    assert!(!affinity.is_empty(), "Should compute at least one affinity pair");

    // Check that high-frequency co-occurring pairs have positive affinity
    let mut has_positive = false;
    for ((a, b), score) in &affinity {
        assert!(
            *score >= 0.0 && *score <= 1.0,
            "Affinity score for ({}, {}) should be in [0,1], got {}",
            a, b, score
        );
        if *score > 0.0 {
            has_positive = true;
        }
    }
    assert!(has_positive, "At least one pair should have positive affinity");

    // consciousness+topology should have high affinity (appear together in many discovery scans)
    let ct_key = ("consciousness".to_string(), "topology".to_string());
    if let Some(&score) = affinity.get(&ct_key) {
        assert!(
            score > 0.3,
            "consciousness+topology should have high affinity, got {}",
            score
        );
    }
}
