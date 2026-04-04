//! Integration tests for SW/Math extended discovery nodes (BT-105~117).
//!
//! Tests the full graph pipeline including the new sw_math_extended_nodes layer:
//!   BT → expanded → discoveries → extended → recent → recent_extended → bt185 → sw_math_extended
//! Verifies node counts, edge types, cross-domain bridges, and validates chains.

use nexus6::graph::*;
use nexus6::graph::bt_nodes::populate_bt_graph;
use nexus6::graph::expanded_nodes::populate_expanded_graph;
use nexus6::graph::discovery_nodes::populate_all_discoveries;
use nexus6::graph::extended_discovery_nodes::populate_all_extended;
use nexus6::graph::recent_discoveries::populate_all_recent_discoveries;
use nexus6::graph::recent_extended_nodes::populate_all_recent_extended;
use nexus6::graph::bt185_discovery_nodes::populate_all_bt185_discoveries;
use nexus6::graph::math_sw_extended_nodes::{
    populate_all_sw_math_extended, sw_math_extended_entry_count,
    populate_math_sle_extended, populate_math_algebra_extended,
    populate_math_harmonic_extended, populate_math_cross_extended,
    populate_sw_principles_extended, populate_sw_crypto_extended,
    populate_sw_layers_extended, populate_sw_isomorphism_extended,
    populate_math_sw_cross_domain,
};

fn base_graph() -> DiscoveryGraph {
    let mut g = DiscoveryGraph::new();
    populate_bt_graph(&mut g);
    populate_expanded_graph(&mut g);
    populate_all_discoveries(&mut g);
    populate_all_extended(&mut g);
    g
}

fn full_pipeline_graph() -> DiscoveryGraph {
    let mut g = base_graph();
    populate_all_recent_discoveries(&mut g);
    populate_all_recent_extended(&mut g);
    populate_all_bt185_discoveries(&mut g);
    populate_all_sw_math_extended(&mut g);
    g
}

// ═══════════════════════════════════════════════════════════════
// Entry counts
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_sw_math_entry_count() {
    // 3+3+4+3+3+4+4+3+9 = 36
    assert_eq!(sw_math_extended_entry_count(), 36, "36 SW/Math extended entries");
}

// ═══════════════════════════════════════════════════════════════
// Individual cluster population
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_populate_math_sle() {
    let mut g = base_graph();
    let before = g.nodes.len();
    let (nodes, edges) = populate_math_sle_extended(&mut g);
    assert_eq!(nodes, 3);
    assert_eq!(g.nodes.len(), before + 3);
    assert!(edges >= 7, "SLE cluster needs 7+ edges, got {}", edges);
}

#[test]
fn test_populate_math_algebra() {
    let mut g = base_graph();
    let (nodes, edges) = populate_math_algebra_extended(&mut g);
    assert_eq!(nodes, 3);
    assert!(edges >= 8, "Algebra cluster needs 8+ edges, got {}", edges);
}

#[test]
fn test_populate_math_harmonic() {
    let mut g = base_graph();
    let (nodes, edges) = populate_math_harmonic_extended(&mut g);
    assert_eq!(nodes, 4);
    assert!(edges >= 10, "Harmonic cluster needs 10+ edges, got {}", edges);
}

#[test]
fn test_populate_math_cross() {
    let mut g = base_graph();
    let (nodes, edges) = populate_math_cross_extended(&mut g);
    assert_eq!(nodes, 3);
    assert!(edges >= 8, "Math cross cluster needs 8+ edges, got {}", edges);
}

#[test]
fn test_populate_sw_principles() {
    let mut g = base_graph();
    let (nodes, edges) = populate_sw_principles_extended(&mut g);
    assert_eq!(nodes, 3);
    assert!(edges >= 6, "SW principles cluster needs 6+ edges, got {}", edges);
}

#[test]
fn test_populate_sw_crypto() {
    let mut g = base_graph();
    let (nodes, edges) = populate_sw_crypto_extended(&mut g);
    assert_eq!(nodes, 4);
    assert!(edges >= 12, "Crypto cluster needs 12+ edges, got {}", edges);
}

#[test]
fn test_populate_sw_layers() {
    let mut g = base_graph();
    let (nodes, edges) = populate_sw_layers_extended(&mut g);
    assert_eq!(nodes, 4);
    assert!(edges >= 10, "Layers cluster needs 10+ edges, got {}", edges);
}

#[test]
fn test_populate_sw_isomorphism() {
    let mut g = base_graph();
    let (nodes, edges) = populate_sw_isomorphism_extended(&mut g);
    assert_eq!(nodes, 3);
    assert!(edges >= 7, "Isomorphism cluster needs 7+ edges, got {}", edges);
}

#[test]
fn test_populate_math_sw_cross_domain() {
    let mut g = base_graph();
    let (nodes, edges) = populate_math_sw_cross_domain(&mut g);
    assert_eq!(nodes, 9);
    assert!(edges >= 25, "Cross-domain needs 25+ edges, got {}", edges);
}

// ═══════════════════════════════════════════════════════════════
// Full population
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_populate_all_sw_math_extended() {
    let mut g = base_graph();
    let before_nodes = g.nodes.len();
    let (nodes, edges) = populate_all_sw_math_extended(&mut g);

    assert_eq!(nodes, 36, "36 SW/Math extended nodes");
    assert!(edges > 150, "Should add 150+ edges, got {}", edges);
    assert_eq!(g.nodes.len(), before_nodes + 36);
}

// ═══════════════════════════════════════════════════════════════
// Full pipeline integration
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_full_pipeline_node_count() {
    let g = full_pipeline_graph();
    // base ~247 + recent 26 + rext 24 + bt185 41 + sw_math 36 = ~374+
    assert!(
        g.nodes.len() >= 370,
        "Full pipeline should have 370+ nodes, got {}",
        g.nodes.len()
    );
}

#[test]
fn test_full_pipeline_edge_count() {
    let g = full_pipeline_graph();
    assert!(
        g.edges.len() >= 700,
        "Full pipeline should have 700+ edges, got {}",
        g.edges.len()
    );
}

#[test]
fn test_no_duplicate_ids_full_pipeline() {
    let g = full_pipeline_graph();
    let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
    let total = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), total, "All node IDs must be unique in full pipeline");
}

// ═══════════════════════════════════════════════════════════════
// Validation chains (BT → DISC → XDISC)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_sle_validation_chain() {
    // BT-105 --Derives--> DISC-MATH-01 <--Validates-- XDISC-MATH-01
    let g = full_pipeline_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-105" && e.to == "DISC-MATH-01" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-105 should derive DISC-MATH-01");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-MATH-01"
            && e.to == "DISC-MATH-01"
            && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-MATH-01 should validate DISC-MATH-01");
}

#[test]
fn test_ramanujan_validation_chain() {
    // BT-107 --Derives--> DISC-MATH-03 <--Validates-- XDISC-MATH-04
    let g = full_pipeline_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-107" && e.to == "DISC-MATH-03" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-107 should derive DISC-MATH-03");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-MATH-04"
            && e.to == "DISC-MATH-03"
            && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-MATH-04 should validate DISC-MATH-03");
}

#[test]
fn test_sw_crypto_validation_chain() {
    // BT-114 --Derives--> DISC-SW-02 <--Validates-- XDISC-SW-04
    let g = full_pipeline_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-114" && e.to == "DISC-SW-02" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-114 should derive DISC-SW-02");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-SW-04"
            && e.to == "DISC-SW-02"
            && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-SW-04 should validate DISC-SW-02");
}

#[test]
fn test_sw_acid_validation_chain() {
    // BT-116 --Derives--> DISC-SW-04 <--Validates-- XDISC-SW-09
    let g = full_pipeline_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-116" && e.to == "DISC-SW-04" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-116 should derive DISC-SW-04");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-SW-09"
            && e.to == "DISC-SW-04"
            && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-SW-09 should validate DISC-SW-04");
}

// ═══════════════════════════════════════════════════════════════
// Cross-domain bridge validation
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_mswx_acid_quadruped_bridge() {
    // XDISC-MSWX-05 validates both DISC-SW-04 (ACID) and DISC-ROBO-03 (quadruped)
    let g = full_pipeline_graph();

    let val_sw = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-05"
            && e.to == "DISC-SW-04"
            && e.edge_type == EdgeType::Validates
    });
    let val_robo = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-05"
            && e.to == "DISC-ROBO-03"
            && e.edge_type == EdgeType::Validates
    });
    assert!(val_sw, "MSWX-05 should validate DISC-SW-04 (tau=4 ACID)");
    assert!(val_robo, "MSWX-05 should validate DISC-ROBO-03 (tau=4 quadruped)");
}

#[test]
fn test_mswx_sle_hexagonal_bridge() {
    // XDISC-MSWX-01 validates both DISC-MATH-01 (SLE_6) and DISC-ENV-05 (hexagonal)
    let g = full_pipeline_graph();

    let val_math = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-01"
            && e.to == "DISC-MATH-01"
            && e.edge_type == EdgeType::Validates
    });
    let val_env = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-01"
            && e.to == "DISC-ENV-05"
            && e.edge_type == EdgeType::Validates
    });
    assert!(val_math, "MSWX-01 should validate DISC-MATH-01 (SLE_6)");
    assert!(val_env, "MSWX-01 should validate DISC-ENV-05 (hexagonal geometry)");
}

#[test]
fn test_mswx_osi_robot_bridge() {
    // XDISC-MSWX-04 validates both DISC-SW-03 (OSI) and DISC-ROBO-01 (SE(3))
    let g = full_pipeline_graph();

    let val_sw = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-04"
            && e.to == "DISC-SW-03"
            && e.edge_type == EdgeType::Validates
    });
    let val_robo = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-04"
            && e.to == "DISC-ROBO-01"
            && e.edge_type == EdgeType::Validates
    });
    assert!(val_sw, "MSWX-04 should validate DISC-SW-03 (OSI layers)");
    assert!(val_robo, "MSWX-04 should validate DISC-ROBO-01 (SE(3))");
}

// ═══════════════════════════════════════════════════════════════
// Hub analysis with full pipeline
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_c_n_hub_grows_with_sw_math() {
    let g_before = {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_extended(&mut g);
        populate_all_recent_discoveries(&mut g);
        populate_all_recent_extended(&mut g);
        populate_all_bt185_discoveries(&mut g);
        g
    };
    let g_after = full_pipeline_graph();

    let hubs_before = g_before.hubs(1);
    let hubs_after = g_after.hubs(1);

    let cn_before = hubs_before.iter().find(|h| h.node_id == "C-n").map(|h| h.degree).unwrap_or(0);
    let cn_after = hubs_after.iter().find(|h| h.node_id == "C-n").map(|h| h.degree).unwrap_or(0);

    assert!(
        cn_after > cn_before,
        "C-n hub degree should increase: before={}, after={}",
        cn_before, cn_after
    );
}

// ═══════════════════════════════════════════════════════════════
// Edge strength validation
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_all_edge_strengths_valid() {
    let g = full_pipeline_graph();
    for edge in &g.edges {
        assert!(
            edge.strength > 0.0 && edge.strength <= 1.0,
            "Edge {}->{} strength {} out of range",
            edge.from, edge.to, edge.strength
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// Roundtrip persistence
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_full_pipeline_roundtrip() {
    let g = full_pipeline_graph();
    let tmp = "/tmp/nexus6_full_pipeline_test.json";
    g.save(tmp).expect("save failed");
    let loaded = DiscoveryGraph::load(tmp).expect("load failed");

    assert_eq!(loaded.nodes.len(), g.nodes.len());
    assert_eq!(loaded.edges.len(), g.edges.len());

    std::fs::remove_file(tmp).ok();
}
