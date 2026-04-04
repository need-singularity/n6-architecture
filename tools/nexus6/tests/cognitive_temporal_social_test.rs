//! Integration tests for Cognitive/Temporal/Social discovery nodes (BT-210~225).
//!
//! Tests the graph pipeline extension:
//!   BT → expanded → ... → CTS discoveries → cross-domain edges

use nexus6::graph::*;
use nexus6::graph::bt_nodes::populate_bt_graph;
use nexus6::graph::expanded_nodes::populate_expanded_graph;
use nexus6::graph::discovery_nodes::populate_all_discoveries;
use nexus6::graph::extended_discovery_nodes::populate_all_extended;
use nexus6::graph::recent_discoveries::populate_all_recent_discoveries;
use nexus6::graph::recent_extended_nodes::populate_all_recent_extended;
use nexus6::graph::bt185_discovery_nodes::populate_all_bt185_discoveries;
use nexus6::graph::math_sw_extended_nodes::populate_all_sw_math_extended;
use nexus6::graph::cognitive_temporal_social_nodes::{
    populate_all_cts_discoveries, cts_entry_count,
    populate_cognitive_discoveries, populate_temporal_discoveries,
    populate_social_discoveries, populate_cts_bridges,
    cognitive_entry_count, temporal_entry_count, social_entry_count, bridge_entry_count,
};

fn base_graph() -> DiscoveryGraph {
    let mut g = DiscoveryGraph::new();
    populate_bt_graph(&mut g);
    populate_expanded_graph(&mut g);
    g
}

fn ultimate_graph() -> DiscoveryGraph {
    let mut g = base_graph();
    populate_all_discoveries(&mut g);
    populate_all_extended(&mut g);
    populate_all_recent_discoveries(&mut g);
    populate_all_recent_extended(&mut g);
    populate_all_bt185_discoveries(&mut g);
    populate_all_sw_math_extended(&mut g);
    g
}

fn full_graph_with_cts() -> DiscoveryGraph {
    let mut g = ultimate_graph();
    populate_all_cts_discoveries(&mut g);
    g
}

// ═══════════════════════════════════════════════════════════════
// Entry counts
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_cts_entry_count() {
    assert_eq!(cts_entry_count(), 31, "31 total CTS entries");
}

#[test]
fn test_cluster_entry_counts() {
    assert_eq!(cognitive_entry_count(), 7);
    assert_eq!(temporal_entry_count(), 6);
    assert_eq!(social_entry_count(), 7);
    assert_eq!(bridge_entry_count(), 11);
}

// ═══════════════════════════════════════════════════════════════
// BT node count (155 = 128 original + 13 CTS + 14 TRM)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_bt_count_with_new_bts() {
    let mut g = DiscoveryGraph::new();
    populate_bt_graph(&mut g);
    let bt_nodes: Vec<_> = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Bt)).collect();
    assert_eq!(bt_nodes.len(), 155, "155 BT nodes (128 + 13 CTS + 14 TRM)");

    // Verify specific new BTs exist
    for id in &[210, 211, 212, 213, 214, 215, 219, 220, 221, 222, 223, 224, 225] {
        assert!(
            g.nodes.iter().any(|n| n.id == format!("BT-{}", id)),
            "BT-{} should exist", id
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// Full pipeline with CTS layer
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_full_graph_with_cts_node_count() {
    let g = full_graph_with_cts();
    // ultimate ~395+ nodes + 32 CTS = 427+
    assert!(
        g.nodes.len() >= 425,
        "Full graph with CTS should have 425+ nodes, got {}",
        g.nodes.len()
    );
}

#[test]
fn test_full_graph_with_cts_edge_count() {
    let g = full_graph_with_cts();
    // CTS adds 100+ edges (derives, uses, validates, merges)
    assert!(
        g.edges.len() >= 700,
        "Full graph with CTS should have 700+ edges, got {}",
        g.edges.len()
    );
}

#[test]
fn test_no_duplicate_ids_with_cts() {
    let g = full_graph_with_cts();
    let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
    let total = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), total, "All node IDs must be unique in CTS-extended graph");
}

// ═══════════════════════════════════════════════════════════════
// Cross-domain edge verification
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_bt210_derives_disc_cog_01() {
    let mut g = base_graph();
    populate_cognitive_discoveries(&mut g);

    let has_edge = g.edges.iter().any(|e| {
        e.from == "BT-210" && e.to == "DISC-COG-01" && matches!(e.edge_type, EdgeType::Derives)
    });
    assert!(has_edge, "BT-210 --Derives--> DISC-COG-01");
}

#[test]
fn test_bt225_derives_triple_bridge() {
    let mut g = base_graph();
    populate_cognitive_discoveries(&mut g);
    populate_temporal_discoveries(&mut g);
    populate_social_discoveries(&mut g);
    populate_cts_bridges(&mut g);

    let has_edge = g.edges.iter().any(|e| {
        e.from == "BT-225" && e.to == "DISC-CTS-01" && matches!(e.edge_type, EdgeType::Derives)
    });
    assert!(has_edge, "BT-225 --Derives--> DISC-CTS-01");
}

#[test]
fn test_cross_validates_across_clusters() {
    let mut g = base_graph();
    populate_cognitive_discoveries(&mut g);
    populate_temporal_discoveries(&mut g);

    // DISC-TEMP-05 validates DISC-COG-04 (cross-cluster validation)
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-TEMP-05"
            && e.to == "DISC-COG-04"
            && matches!(e.edge_type, EdgeType::Validates)
    });
    assert!(has_val, "DISC-TEMP-05 should validate DISC-COG-04 (cross-cluster)");
}

#[test]
fn test_social_validates_cognitive() {
    let mut g = base_graph();
    populate_cognitive_discoveries(&mut g);
    populate_social_discoveries(&mut g);

    // DISC-SOC-06 validates DISC-COG-02 (Christaller↔grid cell)
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-SOC-06"
            && e.to == "DISC-COG-02"
            && matches!(e.edge_type, EdgeType::Validates)
    });
    assert!(has_val, "DISC-SOC-06 should validate DISC-COG-02");
}

#[test]
fn test_cts_bridge_validates_all_three_architectures() {
    let mut g = base_graph();
    populate_all_cts_discoveries(&mut g);

    // DISC-CTS-01 validates nodes in all three architectures
    let val_cog = g.edges.iter().any(|e| {
        e.from == "DISC-CTS-01" && e.to == "DISC-COG-01" && matches!(e.edge_type, EdgeType::Validates)
    });
    let val_soc = g.edges.iter().any(|e| {
        e.from == "DISC-CTS-01" && e.to == "DISC-SOC-01" && matches!(e.edge_type, EdgeType::Validates)
    });
    let val_temp = g.edges.iter().any(|e| {
        e.from == "DISC-CTS-01" && e.to == "DISC-TEMP-01" && matches!(e.edge_type, EdgeType::Validates)
    });
    assert!(val_cog, "DISC-CTS-01 should validate DISC-COG-01");
    assert!(val_soc, "DISC-CTS-01 should validate DISC-SOC-01");
    assert!(val_temp, "DISC-CTS-01 should validate DISC-TEMP-01");
}

// ═══════════════════════════════════════════════════════════════
// Hub and convergence analysis
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_c_n_is_major_hub_with_cts() {
    let g = full_graph_with_cts();
    let hubs = g.hubs(5);
    let c_n_hub = hubs.iter().find(|h| h.node_id == "C-n");
    assert!(c_n_hub.is_some(), "C-n should be a hub with degree >= 5");
}

#[test]
fn test_new_domains_in_bt_graph() {
    let mut g = DiscoveryGraph::new();
    populate_bt_graph(&mut g);

    // New domains: Cognitive, Temporal, Social, Neuroscience
    for domain in &["Cognitive", "Temporal", "Social", "Neuroscience"] {
        let has_domain = g.nodes.iter().any(|n| n.domain.contains(domain));
        assert!(has_domain, "Domain '{}' should exist in BT graph", domain);
    }
}

#[test]
fn test_cross_domain_bt_edges_new_domains() {
    let mut g = DiscoveryGraph::new();
    populate_bt_graph(&mut g);

    // BT-215 (Social, Cognitive) should connect to BT-210 (Cognitive) via shared Cognitive
    let has_edge = g.edges.iter().any(|e| {
        (e.from == "BT-210" && e.to == "BT-215") || (e.from == "BT-215" && e.to == "BT-210")
    });
    assert!(has_edge, "BT-210 and BT-215 should share Cognitive domain edge");
}

// ═══════════════════════════════════════════════════════════════
// Roundtrip persistence
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_cts_graph_roundtrip() {
    let g = full_graph_with_cts();
    let tmp = "/tmp/nexus6_cts_graph_test.json";
    g.save(tmp).expect("save failed");
    let loaded = DiscoveryGraph::load(tmp).expect("load failed");
    assert_eq!(loaded.nodes.len(), g.nodes.len());
    assert_eq!(loaded.edges.len(), g.edges.len());
    std::fs::remove_file(tmp).ok();
}

// ═══════════════════════════════════════════════════════════════
// Edge strength validation
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_all_edge_strengths_valid() {
    let g = full_graph_with_cts();
    for edge in &g.edges {
        assert!(
            edge.strength > 0.0 && edge.strength <= 1.0,
            "Edge {}->{} has invalid strength {}", edge.from, edge.to, edge.strength
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// CTS-specific domain spanning
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_disc_cts_08_spans_five_domains() {
    let mut g = base_graph();
    populate_all_cts_discoveries(&mut g);

    let bridge = g.nodes.iter().find(|n| n.id == "DISC-CTS-08").unwrap();
    let domains: Vec<&str> = bridge.domain.split(", ").collect();
    assert_eq!(domains.len(), 5, "DISC-CTS-08 spans 5 domains: Social, Cognitive, Environment, Math, Topology");
}
