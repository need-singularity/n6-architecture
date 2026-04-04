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
use nexus6::graph::bt185_discovery_nodes::{
    populate_all_bt185_discoveries, bt185_entry_count,
};
use nexus6::graph::math_sw_extended_nodes::{
    populate_all_sw_math_extended, sw_math_extended_entry_count,
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

fn ultimate_graph() -> DiscoveryGraph {
    let mut g = complete_graph();
    populate_all_bt185_discoveries(&mut g);
    populate_all_sw_math_extended(&mut g);
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
    // 128 BT + ~79 expanded + 39 disc + 32 ext + 26 recent + 24 rext = ~328
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

// ═══════════════════════════════════════════════════════════════
// BT-185: Algebraic Blowup–Emergence E₆ Bridge
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_bt185_entry_count() {
    // 2+2+2+3+5+3+6+6+4+8 = 41
    assert_eq!(bt185_entry_count(), 41, "41 BT-185 entries (original 23 + 6 topoexp + 4 invcore + 8 bt118xdom)");
}

#[test]
fn test_ultimate_graph_node_count() {
    let g = ultimate_graph();
    // complete_graph 320+ nodes + 41 BT-185 nodes + 36 Math/SW extended = 397+
    assert!(
        g.nodes.len() >= 395,
        "Ultimate graph should have 395+ nodes, got {}",
        g.nodes.len()
    );
}

#[test]
fn test_ultimate_graph_edge_count() {
    let g = ultimate_graph();
    // 600+ from before + 100+ from math_sw_extended
    assert!(
        g.edges.len() >= 700,
        "Ultimate graph should have 700+ edges, got {}",
        g.edges.len()
    );
}

#[test]
fn test_no_duplicate_node_ids_ultimate() {
    let g = ultimate_graph();
    let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
    let total = ids.len();
    ids.sort();
    ids.dedup();
    assert_eq!(ids.len(), total, "All node IDs must be unique in ultimate graph");
}

#[test]
fn test_bt185_validation_chain() {
    // BT-185 --Derives--> DISC-BLOW-01 <--Validates-- DISC-BLOW-02
    let g = ultimate_graph();

    let bt_derives = g.edges.iter().any(|e| {
        e.from == "BT-185" && e.to == "DISC-BLOW-01" && e.edge_type == EdgeType::Derives
    });
    assert!(bt_derives, "BT-185 should derive DISC-BLOW-01");

    let disc_validates = g.edges.iter().any(|e| {
        e.from == "DISC-BLOW-02" && e.to == "DISC-BLOW-01" && e.edge_type == EdgeType::Validates
    });
    assert!(disc_validates, "DISC-BLOW-02 should validate DISC-BLOW-01");
}

#[test]
fn test_bt185_e6_pillar() {
    let g = ultimate_graph();

    // E6 rank=n=6 discovery exists
    let e6_node = g.nodes.iter().find(|n| n.id == "DISC-BLOW-05");
    assert!(e6_node.is_some(), "E6 Lie algebra discovery DISC-BLOW-05 should exist");

    // BT-185 derives it
    let derives = g.edges.iter().any(|e| {
        e.from == "BT-185" && e.to == "DISC-BLOW-05" && e.edge_type == EdgeType::Derives
    });
    assert!(derives, "BT-185 should derive DISC-BLOW-05");
}

#[test]
fn test_bt185_bridges_to_existing_clusters() {
    let g = ultimate_graph();

    // Bridge to SLE6 (DISC-MATH-01)
    let sle6_bridge = g.edges.iter().any(|e| {
        e.from == "DISC-BLOW-BRIDGE-01"
            && e.to == "DISC-MATH-01"
            && e.edge_type == EdgeType::Validates
    });
    assert!(sle6_bridge, "BT-185 should bridge to SLE6 discovery via DISC-BLOW-BRIDGE-01");

    // Bridge to SM packing (DISC-TOPO-01)
    let sm_bridge = g.edges.iter().any(|e| {
        e.from == "DISC-BLOW-BRIDGE-05"
            && e.to == "DISC-TOPO-01"
            && e.edge_type == EdgeType::Validates
    });
    assert!(sm_bridge, "BT-185 should bridge to topological chip DISC-TOPO-01");
}

// ═══════════════════════════════════════════════════════════════
// BT-185: Topology Expansion (406/427 EXACT)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_topoexp_su6_validates_e6() {
    let g = ultimate_graph();
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-TOPOEXP-01"
            && e.to == "DISC-BLOW-05"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "DISC-TOPOEXP-01 (SU6) should validate DISC-BLOW-05 (E6)");
}

#[test]
fn test_topoexp_total_node() {
    let g = ultimate_graph();
    let topoexp_total = g.nodes.iter().find(|n| n.id == "DISC-TOPOEXP-06");
    assert!(topoexp_total.is_some(), "406/427 EXACT total node DISC-TOPOEXP-06 should exist");
}

#[test]
fn test_topoexp_homotopy() {
    let g = ultimate_graph();
    let has = g.nodes.iter().any(|n| n.id == "DISC-TOPOEXP-04");
    assert!(has, "Homotopy discovery DISC-TOPOEXP-04 should exist");
}

// ═══════════════════════════════════════════════════════════════
// BT-185: Invariant Lens Core
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_invcore_absolute_core() {
    let g = ultimate_graph();
    let core = g.nodes.iter().find(|n| n.id == "DISC-INVCORE-01");
    assert!(core.is_some(), "ABSOLUTE invariant core DISC-INVCORE-01 should exist");
    let core = core.unwrap();
    assert!(core.confidence >= 0.90, "Invariant core should have high confidence");
}

#[test]
fn test_invcore_validates_emergence() {
    let g = ultimate_graph();
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-INVCORE-01"
            && e.to == "DISC-BLOW-07"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "Invariant core should validate blowup-emergence archetype");
}

#[test]
fn test_invcore_experiment_node() {
    let g = ultimate_graph();
    let exp = g.nodes.iter().find(|n| n.id == "DISC-INVCORE-04");
    assert!(exp.is_some(), "Experiment node DISC-INVCORE-04 should exist");
    assert!(matches!(exp.unwrap().node_type, NodeType::Experiment));
}

// ═══════════════════════════════════════════════════════════════
// BT-185: Cross-domain bridges BT-118+ ↔ BT-185
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_bt118xdom_carbon_bridge() {
    let g = ultimate_graph();
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-BT118XDOM-04"
            && e.to == "DISC-ENV-01"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "Carbon Z=6↔C^6 bridge should validate DISC-ENV-01");
}

#[test]
fn test_bt118xdom_robot_bridge() {
    let g = ultimate_graph();
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-BT118XDOM-03"
            && e.to == "DISC-ROBO-01"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "SE(3)↔C^6 bridge should validate DISC-ROBO-01");
}

#[test]
fn test_bt118xdom_water_bridge() {
    let g = ultimate_graph();
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-BT118XDOM-08"
            && e.to == "DISC-ENV-03"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "Water treatment↔topo matter bridge should validate DISC-ENV-03");
}

#[test]
fn test_bt118xdom_software_bridge() {
    let g = ultimate_graph();
    let has_val = g.edges.iter().any(|e| {
        e.from == "DISC-BT118XDOM-05"
            && e.to == "DISC-SW-05"
            && e.edge_type == EdgeType::Validates
    });
    assert!(has_val, "SW isomorphism↔E6 bridge should validate DISC-SW-05");
}

#[test]
fn test_bt185_is_hub() {
    let g = ultimate_graph();
    let hubs = g.hubs(15);
    let bt185 = hubs.iter().find(|h| h.node_id == "BT-185");
    assert!(bt185.is_some(), "BT-185 should be a major hub with degree >= 15");
}

#[test]
fn test_bt185_all_edge_strengths_valid() {
    let g = ultimate_graph();
    for edge in &g.edges {
        assert!(
            edge.strength > 0.0 && edge.strength <= 1.0,
            "Edge {}->{} strength {} out of range",
            edge.from, edge.to, edge.strength
        );
    }
}

#[test]
fn test_ultimate_graph_roundtrip() {
    let g = ultimate_graph();
    let tmp = "/tmp/nexus6_ultimate_graph_test.json";
    g.save(tmp).expect("save failed");
    let loaded = DiscoveryGraph::load(tmp).expect("load failed");

    assert_eq!(loaded.nodes.len(), g.nodes.len());
    assert_eq!(loaded.edges.len(), g.edges.len());

    std::fs::remove_file(tmp).ok();
}

// ═══════════════════════════════════════════════════════════════
// Math/SW Extended Nodes (BT-105~117)
// ══════════════════════════════════════════════��════════════════

#[test]
fn test_sw_math_extended_entry_count() {
    // 3+3+4+3+3+4+4+3+9 = 36
    assert_eq!(sw_math_extended_entry_count(), 36, "36 Math/SW extended entries");
}

#[test]
fn test_math_sw_sle6_validation_chain() {
    let g = ultimate_graph();
    // BT-105 --Derives--> XDISC-MATH-01 --Validates--> DISC-MATH-01
    let derives = g.edges.iter().any(|e| {
        e.from == "BT-105" && e.to == "XDISC-MATH-01" && e.edge_type == EdgeType::Derives
    });
    assert!(derives, "BT-105 should derive XDISC-MATH-01");

    let validates = g.edges.iter().any(|e| {
        e.from == "XDISC-MATH-01" && e.to == "DISC-MATH-01" && e.edge_type == EdgeType::Validates
    });
    assert!(validates, "XDISC-MATH-01 should validate DISC-MATH-01");
}

#[test]
fn test_math_sw_ramanujan_chain() {
    let g = ultimate_graph();
    // BT-107 --Derives--> XDISC-MATH-04 (Ramanujan Delta eta^24) --Validates--> DISC-MATH-03
    let derives = g.edges.iter().any(|e| {
        e.from == "BT-107" && e.to == "XDISC-MATH-04" && e.edge_type == EdgeType::Derives
    });
    assert!(derives, "BT-107 should derive XDISC-MATH-04");

    let validates = g.edges.iter().any(|e| {
        e.from == "XDISC-MATH-04" && e.to == "DISC-MATH-03" && e.edge_type == EdgeType::Validates
    });
    assert!(validates, "XDISC-MATH-04 should validate DISC-MATH-03");
}

#[test]
fn test_math_sw_crypto_chain() {
    let g = ultimate_graph();
    // BT-114 --Derives--> XDISC-SW-04 --Validates--> DISC-SW-02
    let derives = g.edges.iter().any(|e| {
        e.from == "BT-114" && e.to == "XDISC-SW-04" && e.edge_type == EdgeType::Derives
    });
    assert!(derives, "BT-114 should derive XDISC-SW-04");

    let validates = g.edges.iter().any(|e| {
        e.from == "XDISC-SW-04" && e.to == "DISC-SW-02" && e.edge_type == EdgeType::Validates
    });
    assert!(validates, "XDISC-SW-04 should validate DISC-SW-02");
}

#[test]
fn test_math_sw_mswx_bridge() {
    let g = ultimate_graph();
    // XDISC-MSWX-07 (sigma=12 universal count) validates DISC-BRIDGE-09 and DISC-BRIDGE-12
    let node = g.nodes.iter().find(|n| n.id == "XDISC-MSWX-07");
    assert!(node.is_some(), "sigma=12 universal count node should exist");

    let val_bridge = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-07" && e.to == "DISC-BRIDGE-09" && e.edge_type == EdgeType::Validates
    });
    assert!(val_bridge, "MSWX-07 should validate DISC-BRIDGE-09");
}

#[test]
fn test_math_sw_acid_quadruped_bridge() {
    let g = ultimate_graph();
    // XDISC-MSWX-05 (tau=4 ACID↔quadruped) validates DISC-SW-04 and DISC-ROBO-03
    let val_sw = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-05" && e.to == "DISC-SW-04" && e.edge_type == EdgeType::Validates
    });
    let val_robo = g.edges.iter().any(|e| {
        e.from == "XDISC-MSWX-05" && e.to == "DISC-ROBO-03" && e.edge_type == EdgeType::Validates
    });
    assert!(val_sw, "MSWX-05 should validate DISC-SW-04");
    assert!(val_robo, "MSWX-05 should validate DISC-ROBO-03");
}

#[test]
fn test_math_sw_hypothesis_nodes() {
    let g = ultimate_graph();
    let hyp_math = g.nodes.iter().find(|n| n.id == "XHYP-MATH-01");
    assert!(hyp_math.is_some(), "XHYP-MATH-01 (zeta special values) should exist");
    assert!(matches!(hyp_math.unwrap().node_type, NodeType::Hypothesis));

    let hyp_sw = g.nodes.iter().find(|n| n.id == "XHYP-SW-01");
    assert!(hyp_sw.is_some(), "XHYP-SW-01 (thread pool sigma/J2) should exist");
}

#[test]
fn test_math_sw_experiment_nodes() {
    let g = ultimate_graph();
    let exp_math = g.nodes.iter().find(|n| n.id == "XEXP-MATH-01");
    assert!(exp_math.is_some(), "XEXP-MATH-01 (Monte Carlo SLE) should exist");
    assert!(matches!(exp_math.unwrap().node_type, NodeType::Experiment));

    let exp_sw = g.nodes.iter().find(|n| n.id == "XEXP-SW-01");
    assert!(exp_sw.is_some(), "XEXP-SW-01 (NIST crypto survey) should exist");
    assert!(matches!(exp_sw.unwrap().node_type, NodeType::Experiment));
}
