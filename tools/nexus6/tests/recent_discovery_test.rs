//! Integration tests for recent discovery nodes (BT-89~104) and their extended sub-discoveries.
//!
//! Tests the full graph pipeline:
//!   BT → expanded → discoveries → extended → recent → recent_extended
//! Verifies node counts, edge types, cross-domain bridges, and validates chains.

use nexus6::graph::*;
use nexus6::graph::bt_nodes::populate_bt_graph;
use nexus6::graph::expanded_nodes::populate_expanded_graph;
use nexus6::graph::discovery_nodes::populate_all_discoveries;
use nexus6::graph::extended_discovery_nodes::populate_all_extended;
use nexus6::graph::recent_discoveries::{
    populate_all_recent_discoveries, recent_entry_count,
};
use nexus6::graph::recent_extended_nodes::{
    populate_all_recent_extended, recent_extended_entry_count,
};

fn base_graph() -> DiscoveryGraph {
    let mut g = DiscoveryGraph::new();
    populate_bt_graph(&mut g);
    populate_expanded_graph(&mut g);
    g
}

fn full_graph() -> DiscoveryGraph {
    let mut g = base_graph();
    populate_all_discoveries(&mut g);
    populate_all_extended(&mut g);
    g
}

fn complete_graph() -> DiscoveryGraph {
    let mut g = full_graph();
    populate_all_recent_discoveries(&mut g);
    populate_all_recent_extended(&mut g);
    g
}

// ═══════════════════════════════════════════════════════════════
// Entry counts
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_recent_entry_count() {
    assert_eq!(recent_entry_count(), 26, "26 recent discovery entries");
}

#[test]
fn test_recent_extended_entry_count() {
    assert_eq!(recent_extended_entry_count(), 24, "24 recent extended entries");
}

// ═══════════════════════════════════════════════════════════════
// Full pipeline integration
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_complete_graph_node_count() {
    let g = complete_graph();
    // 127 BT + ~79 expanded + 39 disc + 32 ext + 26 recent + 24 rext = ~327
    assert!(
        g.nodes.len() >= 320,
        "Complete graph should have 320+ nodes, got {}",
        g.nodes.len()
    );
}

#[test]
fn test_complete_graph_edge_count() {
    let g = complete_graph();
    // Massive cross-domain connectivity
    assert!(
        g.edges.len() >= 500,
        "Complete graph should have 500+ edges, got {}",
        g.edges.len()
    );
}

#[test]
fn test_no_duplicate_node_ids_complete() {
    let g = complete_graph();
    let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
    let total = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), total, "All node IDs must be unique in complete graph");
}

// ═══════════════════════════════════════════════════════════════
// Cross-layer validation chains
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_photonic_validation_chain() {
    // BT-89 --Derives--> DISC-PHOT-01 <--Validates-- XDISC-PHOT-01
    let g = complete_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-89" && e.to == "DISC-PHOT-01" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-89 should derive DISC-PHOT-01");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-PHOT-01" && e.to == "DISC-PHOT-01" && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-PHOT-01 should validate DISC-PHOT-01");
}

#[test]
fn test_topo_validation_chain() {
    // BT-90 --Derives--> DISC-TOPO-01 <--Validates-- XDISC-TOPO-01
    let g = complete_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-90" && e.to == "DISC-TOPO-01" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-90 should derive DISC-TOPO-01");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-TOPO-01" && e.to == "DISC-TOPO-01" && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-TOPO-01 should validate DISC-TOPO-01");
}

#[test]
fn test_fusion_validation_chain() {
    // BT-99 --Derives--> DISC-FUS-03 <--Validates-- XDISC-FUSN-01
    let g = complete_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-99" && e.to == "DISC-FUS-03" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-99 should derive DISC-FUS-03");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-FUSN-01" && e.to == "DISC-FUS-03" && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-FUSN-01 should validate DISC-FUS-03");
}

#[test]
fn test_bio_validation_chain() {
    // BT-101 --Derives--> DISC-BIO-01 <--Validates-- XDISC-BIOC-01
    let g = complete_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-101" && e.to == "DISC-BIO-01" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-101 should derive DISC-BIO-01");

    let xdisc_validates = g.edges.iter().any(|e| {
        e.from == "XDISC-BIOC-01" && e.to == "DISC-BIO-01" && e.edge_type == EdgeType::Validates
    });
    assert!(xdisc_validates, "XDISC-BIOC-01 should validate DISC-BIO-01");
}

// ═══════════════════════════════════════════════════════════════
// Cross-domain bridge connectivity
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_recent_bridges_connect_to_existing() {
    let g = complete_graph();

    // DISC-RBRIDGE-02 (Fusion, Environment, Chemistry, Biology) should
    // connect to DISC-ENV-01 (Environment, Chemistry, Energy, Biology) via 3 shared domains
    let has_merge = g.edges.iter().any(|e| {
        e.edge_type == EdgeType::Merges
            && ((e.from.starts_with("DISC-RBRIDGE-") && e.to.starts_with("DISC-ENV-"))
                || (e.from.starts_with("DISC-ENV-") && e.to.starts_with("DISC-RBRIDGE-")))
    });
    assert!(has_merge, "Recent bridges should connect to existing ENV discoveries");
}

#[test]
fn test_sigma_phi_universality_bridge() {
    let g = complete_graph();

    // XDISC-RCROSS-01 validates DISC-FUS-05 (reconnection rate 0.1)
    let has_val = g.edges.iter().any(|e| {
        e.from == "XDISC-RCROSS-01"
            && e.to == "DISC-FUS-05"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "sigma-phi=10 universality should validate reconnection discovery");
}

// ═══════════════════════════════════════════════════════════════
// Hub analysis with complete graph
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_c_n_is_major_hub() {
    let g = complete_graph();
    let hubs = g.hubs(10);
    let c_n = hubs.iter().find(|h| h.node_id == "C-n");
    assert!(c_n.is_some(), "C-n should be a major hub with degree >= 10");
}

#[test]
fn test_c_sigma_is_major_hub() {
    let g = complete_graph();
    let hubs = g.hubs(10);
    let c_sigma = hubs.iter().find(|h| h.node_id == "C-sigma");
    assert!(c_sigma.is_some(), "C-sigma should be a major hub with degree >= 10");
}

// ═══════════════════════════════════════════════════════════════
// Edge strength validation
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_all_edge_strengths_valid() {
    let g = complete_graph();
    for edge in &g.edges {
        assert!(
            edge.strength > 0.0 && edge.strength <= 1.0,
            "Edge {}->{} strength {} out of range",
            edge.from, edge.to, edge.strength
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// Convergence detection
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_convergences_exist() {
    let g = complete_graph();
    let convs = g.convergences();
    // With 320+ nodes, many convergences should exist
    assert!(
        convs.len() >= 10,
        "Complete graph should have 10+ convergence points, got {}",
        convs.len()
    );
}

#[test]
fn test_c_n_convergence() {
    let g = complete_graph();
    let convs = g.convergences();
    let c_n_conv = convs.iter().find(|c| c.target == "C-n");
    assert!(c_n_conv.is_some(), "C-n should be a convergence target");
    if let Some(conv) = c_n_conv {
        assert!(
            conv.sources.len() >= 5,
            "C-n should have 5+ sources, got {}",
            conv.sources.len()
        );
    }
}

// ═══════════════════════════════════════════════════════════════
// Save/load roundtrip with complete graph
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_complete_graph_roundtrip() {
    let g = complete_graph();
    let tmp = "/tmp/nexus6_complete_graph_test.json";
    g.save(tmp).expect("save failed");
    let loaded = DiscoveryGraph::load(tmp).expect("load failed");

    assert_eq!(loaded.nodes.len(), g.nodes.len());
    assert_eq!(loaded.edges.len(), g.edges.len());

    std::fs::remove_file(tmp).ok();
}
