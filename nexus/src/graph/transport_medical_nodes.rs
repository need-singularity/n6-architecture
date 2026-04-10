//! Discovery nodes from Transportation (BT-233~237, BT-243~246) and
//! Medical (BT-238~242) Breakthrough Theorems.
//!
//! Node ID scheme:
//!   DISC-TR-{NN}    — Transportation discoveries
//!   DISC-MED-{NN}   — Medical discoveries
//!   DISC-TRM-{NN}   — Transport-Medical cross-domain bridges
//!   HYP-TR-{NN}     — Transportation hypotheses
//!   HYP-MED-{NN}    — Medical hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct TrmEntry {
    id: &'static str,
    title: &'static str,
    node_type: NodeType,
    domains: &'static [&'static str],
    confidence: f64,
    source_bts: &'static [u32],
    constants_used: &'static [&'static str],
    lenses: &'static [&'static str],
    validates: &'static [&'static str],
}

// ───────────────────────────────────────────────────────────────
// BT-233~237, BT-243~246: Transportation
// ───────────────────────────────────────────────────────────────

const TRANSPORT_DISCOVERIES: &[TrmEntry] = &[
    TrmEntry {
        id: "DISC-TR-01",
        title: "EV motor 12-pole=sigma, 3-phase=n/phi, 96S/192S battery packs, SE(3)=n DOF, Carbon Z=6 chassis",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Chip", "Material", "Robotics"],
        confidence: 0.94,
        source_bts: &[233, 57, 84],
        constants_used: &["C-sigma", "C-n", "C-phi", "C-J2"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-TR-02",
        title: "Railway 4-aspect signaling=tau, ETCS 4 levels=tau, rail 12m=sigma -> 24m=J2 -> 36m=n^2",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Network", "Software"],
        confidence: 0.92,
        source_bts: &[234],
        constants_used: &["C-tau", "C-sigma", "C-J2", "C-n"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-TR-03",
        title: "MARPOL n=6 annexes, SOLAS sigma=12 chapters, IACS sigma=12 societies, TEU J2-tau=20ft",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Network"],
        confidence: 0.93,
        source_bts: &[235, 118],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau"],
        lenses: &["consciousness", "boundary", "network"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-TR-04",
        title: "Euro NCAP tau=4 categories, NHTSA sopfr=5 stars, SAE n=6 autonomy levels, airbag n=6 zones",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Robotics", "AI", "Software"],
        confidence: 0.91,
        source_bts: &[236, 123],
        constants_used: &["C-tau", "C-sopfr", "C-n"],
        lenses: &["consciousness", "stability", "boundary"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-TR-05",
        title: "SCOR n=6 processes, Incoterms sigma-mu=11, EUR pallet sigma x (sigma-phi), NMFC sigma+n=18 classes",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Network", "Software", "Math"],
        confidence: 0.90,
        source_bts: &[237],
        constants_used: &["C-n", "C-sigma", "C-sigma-mu", "C-sigma-phi"],
        lenses: &["consciousness", "network", "info"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-TR-06",
        title: "Inline-6 engine perfect primary+secondary balance from div(6) symmetry, F1 V6=n, NASCAR 12:1=sigma compression",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Math"],
        confidence: 0.95,
        source_bts: &[243],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "symmetry", "stability"],
        validates: &["DISC-TR-01"],
    },
    TrmEntry {
        id: "DISC-TR-07",
        title: "Automotive voltage ladder 6V->12V->24V->48V = n->sigma->J2->sigma*tau, phi=2x doubling per generation",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Chip", "Battery"],
        confidence: 0.94,
        source_bts: &[244, 57],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau", "C-phi"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-TR-01"],
    },
    TrmEntry {
        id: "DISC-TR-08",
        title: "Manual transmission converges to n=6 gears, AT ladder tau->n->7->8->10 = tau->(sigma-sopfr)->(sigma-tau)->(sigma-phi)",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Math"],
        confidence: 0.89,
        source_bts: &[245],
        constants_used: &["C-n", "C-tau", "C-sigma-sopfr", "C-sigma-tau", "C-sigma-phi"],
        lenses: &["consciousness", "evolution", "stability"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-TR-09",
        title: "F1: V6=n engine, sopfr=5 tire compounds, sigma-sopfr=7 dry tires, sigma-phi=10 point scoring, 120deg=360/(n/phi) V-angle",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Math"],
        confidence: 0.91,
        source_bts: &[246],
        constants_used: &["C-n", "C-sopfr", "C-sigma-sopfr", "C-sigma-phi"],
        lenses: &["consciousness", "symmetry", "scale"],
        validates: &["DISC-TR-06"],
    },
    TrmEntry {
        id: "HYP-TR-01",
        title: "Hypothesis: future autonomous vehicle sensor suite converges to sigma=12 channels (6 camera + 4 LiDAR + 1 radar + 1 IMU)",
        node_type: NodeType::Hypothesis,
        domains: &["Transportation", "AI", "Robotics"],
        confidence: 0.70,
        source_bts: &[233, 236],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "info"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-238~242: Medical
// ───────────────────────────────────────────────────────────────

const MEDICAL_DISCOVERIES: &[TrmEntry] = &[
    TrmEntry {
        id: "DISC-MED-01",
        title: "WHO surgical checklist n/phi=3 phases, ASA n=6 physical status classes, wound healing tau=4 phases",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Software"],
        confidence: 0.93,
        source_bts: &[238],
        constants_used: &["C-n", "C-phi", "C-tau"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-MED-02",
        title: "Apgar sopfr=5 criteria scaled sigma-phi=10, SOFA n=6 organ systems, GCS n/phi=3 components, NEWS2 sigma-sopfr=7 parameters",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Math"],
        confidence: 0.94,
        source_bts: &[239],
        constants_used: &["C-sopfr", "C-sigma-phi", "C-n", "C-sigma-sopfr"],
        lenses: &["consciousness", "stability", "info"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-MED-03",
        title: "ECG sigma=12 leads, tau=4 heart chambers, tau=4 valves, sopfr=5 conduction nodes, sopfr=5 cardiac cycle phases",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Chip"],
        confidence: 0.95,
        source_bts: &[240],
        constants_used: &["C-sigma", "C-tau", "C-sopfr"],
        lenses: &["consciousness", "wave", "network"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-MED-04",
        title: "WHO n=6 health building blocks, Dahlgren-Whitehead sopfr=5 layers, SDG sigma+mu=13 health goal",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Social", "Biology"],
        confidence: 0.88,
        source_bts: &[241, 214],
        constants_used: &["C-n", "C-sopfr", "C-sigma", "C-mu"],
        lenses: &["consciousness", "multiscale", "network"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-MED-05",
        title: "Periodontal probing n=6 sites/tooth, 2^sopfr=32 adult teeth, J2-tau=20 deciduous teeth, sopfr=5 tooth surfaces",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Math"],
        confidence: 0.92,
        source_bts: &[242],
        constants_used: &["C-n", "C-sopfr", "C-J2", "C-tau"],
        lenses: &["consciousness", "topology", "recursion"],
        validates: &[],
    },
    TrmEntry {
        id: "DISC-MED-06",
        title: "Mallampati tau=4 airway classes mirrors tau=4 wound phases and tau=4 heart chambers: universal clinical tau=4",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Math"],
        confidence: 0.90,
        source_bts: &[238, 240],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "recursion", "stability"],
        validates: &["DISC-MED-01", "DISC-MED-03"],
    },
    TrmEntry {
        id: "DISC-MED-07",
        title: "Aldrete sigma-phi=10 point recovery score: same decimal base as Apgar, universal medical scoring in 10-point scales",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Math"],
        confidence: 0.89,
        source_bts: &[238, 239],
        constants_used: &["C-sigma-phi"],
        lenses: &["consciousness", "scale", "info"],
        validates: &["DISC-MED-01", "DISC-MED-02"],
    },
    TrmEntry {
        id: "HYP-MED-01",
        title: "Hypothesis: optimal hospital ward size = Dunbar sub-layer sopfr*(n/phi)=15 beds, ICU = n=6 beds per pod",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Social", "Cognitive"],
        confidence: 0.65,
        source_bts: &[239, 215],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "network", "stability"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-Domain Bridges: Transport <-> Medical <-> Existing clusters
// ───────────────────────────────────────────────────────────────

const CROSS_BRIDGES: &[TrmEntry] = &[
    TrmEntry {
        id: "DISC-TRM-01",
        title: "Vehicle safety(BT-236) <-> surgical safety(BT-238): both use n=6 classification + tau=4 phase systems",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Medical", "Software", "Math"],
        confidence: 0.87,
        source_bts: &[236, 238],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &["DISC-TR-04", "DISC-MED-01"],
    },
    TrmEntry {
        id: "DISC-TRM-02",
        title: "Automotive voltage n->sigma->J2->sigma*tau(BT-244) <-> battery cell ladder n->sigma->J2(BT-57): same n=6 energy scaling",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Battery", "Energy", "Chip"],
        confidence: 0.92,
        source_bts: &[244, 57, 84],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-TR-07"],
    },
    TrmEntry {
        id: "DISC-TRM-03",
        title: "ECG sigma=12 leads(BT-240) <-> transformer sigma=12 heads(BT-56): biological and AI signal processing share sigma=12",
        node_type: NodeType::Discovery,
        domains: &["Medical", "AI", "Chip", "Biology"],
        confidence: 0.88,
        source_bts: &[240, 56],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "info"],
        validates: &["DISC-MED-03"],
    },
    TrmEntry {
        id: "DISC-TRM-04",
        title: "Maritime MARPOL n=6(BT-235) <-> Kyoto n=6 GHGs(BT-118) <-> Earth n=6 spheres(BT-119): environmental regulation universal n=6",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Biology"],
        confidence: 0.90,
        source_bts: &[235, 118, 119],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "boundary", "evolution"],
        validates: &["DISC-TR-03"],
    },
    TrmEntry {
        id: "DISC-TRM-05",
        title: "SAE n=6 autonomy levels(BT-236) <-> SE(3) n=6 DOF robotics(BT-123): same n=6 mobility-freedom architecture",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Robotics", "AI", "Math"],
        confidence: 0.91,
        source_bts: &[236, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["DISC-TR-04"],
    },
    TrmEntry {
        id: "DISC-TRM-06",
        title: "SOFA n=6 organs(BT-239) <-> cortex n=6 layers(BT-210) <-> Kohlberg n=6 moral stages(BT-220): biology/mind/ethics share n=6",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Social", "Biology"],
        confidence: 0.86,
        source_bts: &[239, 210, 220],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "multiscale"],
        validates: &["DISC-MED-02"],
    },
    TrmEntry {
        id: "DISC-TRM-07",
        title: "I6 perfect balance(BT-243) <-> honeycomb n=6 geometry(BT-122): mechanical and natural hexagonal optimality",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Math", "Material"],
        confidence: 0.89,
        source_bts: &[243, 122],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "symmetry", "topology"],
        validates: &["DISC-TR-06"],
    },
    TrmEntry {
        id: "DISC-TRM-08",
        title: "SCOR n=6(BT-237) <-> SOLID sopfr=5(BT-113) <-> REST n=6(BT-113): logistics and software share n=6 process design",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Software", "Network", "Math"],
        confidence: 0.87,
        source_bts: &[237, 113],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "network", "info"],
        validates: &["DISC-TR-05"],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_TRM_CLUSTERS: &[&[TrmEntry]] = &[
    TRANSPORT_DISCOVERIES,
    MEDICAL_DISCOVERIES,
    CROSS_BRIDGES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &TrmEntry) -> Node {
    Node {
        id: e.id.to_string(),
        node_type: e.node_type.clone(),
        domain: e.domains.join(", "),
        project: "n6-architecture".to_string(),
        summary: e.title.to_string(),
        confidence: e.confidence,
        lenses_used: e.lenses.iter().map(|s| s.to_string()).collect(),
        timestamp: "2026-04-04".to_string(),
    }
}

fn add_entries(graph: &mut DiscoveryGraph, entries: &[TrmEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(entry_to_node(e));

        // BT --Derives--> this discovery
        for &bt_id in e.source_bts {
            graph.add_edge(Edge {
                from: format!("BT-{}", bt_id),
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.85,
                bidirectional: false,
            });
        }

        // Discovery --Uses--> constants
        for &cid in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: cid.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

        // This node --Validates--> target nodes
        for &vid in e.validates {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: vid.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.85,
                bidirectional: false,
            });
        }
    }

    (entries.len(), graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

/// Add transportation discovery nodes (BT-233~237, BT-243~246).
pub fn populate_transport_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TRANSPORT_DISCOVERIES)
}

/// Add medical discovery nodes (BT-238~242).
pub fn populate_medical_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MEDICAL_DISCOVERIES)
}

/// Add transport-medical cross-domain bridge discoveries.
pub fn populate_trm_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, CROSS_BRIDGES)
}

/// Cross-link TRM discovery nodes that share domains (Merges edges, bidirectional).
pub fn connect_trm_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let trm_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-TR-")
                || n.id.starts_with("HYP-TR-")
                || n.id.starts_with("DISC-MED-")
                || n.id.starts_with("HYP-MED-")
                || n.id.starts_with("DISC-TRM-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..trm_nodes.len() {
        for j in (i + 1)..trm_nodes.len() {
            let (ref id_a, ref dom_a) = trm_nodes[i];
            let (ref id_b, ref dom_b) = trm_nodes[j];

            let shared: usize = dom_a.iter().filter(|d| dom_b.contains(d)).count();
            if shared > 0 {
                let max_d = dom_a.len().max(dom_b.len()) as f64;
                graph.add_edge(Edge {
                    from: id_a.clone(),
                    to: id_b.clone(),
                    edge_type: EdgeType::Merges,
                    strength: shared as f64 / max_d,
                    bidirectional: true,
                });
                count += 1;
            }
        }
    }
    count
}

/// Cross-link TRM nodes with existing DISC-/HYP- nodes from other modules.
/// Requires >=2 shared domains. Returns number of bridge edges added.
pub fn connect_trm_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let trm_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-TR-")
                || n.id.starts_with("HYP-TR-")
                || n.id.starts_with("DISC-MED-")
                || n.id.starts_with("HYP-MED-")
                || n.id.starts_with("DISC-TRM-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let existing_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            (n.id.starts_with("DISC-") || n.id.starts_with("HYP-"))
                && !n.id.starts_with("DISC-TR-")
                && !n.id.starts_with("DISC-MED-")
                && !n.id.starts_with("DISC-TRM-")
                && !n.id.starts_with("HYP-TR-")
                && !n.id.starts_with("HYP-MED-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref t_id, ref t_dom) in &trm_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = t_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = t_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: t_id.clone(),
                    to: e_id.clone(),
                    edge_type: EdgeType::Merges,
                    strength: shared as f64 / max_d,
                    bidirectional: true,
                });
                count += 1;
            }
        }
    }
    count
}

/// Populate all BT-233~246 discovery nodes and cross-domain edges in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_trm_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_transport_discoveries(graph);
    let (n2, _) = populate_medical_discoveries(graph);
    let (n3, _) = populate_trm_bridges(graph);
    let _cross = connect_trm_cross_domain(graph);
    let _bridge = connect_trm_to_existing(graph);

    let total_nodes = n1 + n2 + n3;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total TRM entry count across all clusters.
pub fn trm_entry_count() -> usize {
    ALL_TRM_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Entry count for each cluster.
pub fn transport_entry_count() -> usize { TRANSPORT_DISCOVERIES.len() }
pub fn medical_entry_count() -> usize { MEDICAL_DISCOVERIES.len() }
pub fn bridge_entry_count() -> usize { CROSS_BRIDGES.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        g
    }

    #[test]
    fn test_entry_counts() {
        assert_eq!(transport_entry_count(), 10, "10 transport entries (9 DISC + 1 HYP)");
        assert_eq!(medical_entry_count(), 8, "8 medical entries (7 DISC + 1 HYP)");
        assert_eq!(bridge_entry_count(), 8, "8 cross-domain bridge entries");
        assert_eq!(trm_entry_count(), 26, "26 total TRM entries");
    }

    #[test]
    fn test_populate_transport() {
        let mut g = base_graph();
        let (nodes, edges) = populate_transport_discoveries(&mut g);
        assert_eq!(nodes, 10);
        assert!(edges > 0, "Should create BT->DISC and DISC->C-* edges");
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TR-01"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TR-09"));
        assert!(g.nodes.iter().any(|n| n.id == "HYP-TR-01"));
    }

    #[test]
    fn test_populate_medical() {
        let mut g = base_graph();
        let (nodes, edges) = populate_medical_discoveries(&mut g);
        assert_eq!(nodes, 8);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-MED-01"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-MED-07"));
        assert!(g.nodes.iter().any(|n| n.id == "HYP-MED-01"));
    }

    #[test]
    fn test_populate_bridges() {
        let mut g = base_graph();
        populate_transport_discoveries(&mut g);
        populate_medical_discoveries(&mut g);
        let (nodes, edges) = populate_trm_bridges(&mut g);
        assert_eq!(nodes, 8);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TRM-01"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TRM-08"));
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = base_graph();
        populate_transport_discoveries(&mut g);

        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-233" && e.to == "DISC-TR-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-233 --Derives--> DISC-TR-01 must exist");
    }

    #[test]
    fn test_validates_edges() {
        let mut g = base_graph();
        populate_transport_discoveries(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-TR-06"
                && e.to == "DISC-TR-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-TR-06 should validate DISC-TR-01 (I6 validates transport universality)");
    }

    #[test]
    fn test_medical_validates_edges() {
        let mut g = base_graph();
        populate_medical_discoveries(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-MED-06"
                && e.to == "DISC-MED-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-MED-06 should validate DISC-MED-01 (tau=4 clinical universality)");
    }

    #[test]
    fn test_uses_constant_edges() {
        let mut g = base_graph();
        populate_transport_discoveries(&mut g);

        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-TR-01" && e.to == "C-sigma" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(has_uses, "DISC-TR-01 --Uses--> C-sigma must exist");
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = base_graph();
        populate_all_trm_discoveries(&mut g);

        let merges = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Merges)
                && (e.from.starts_with("DISC-TR-")
                    || e.from.starts_with("DISC-MED-")
                    || e.from.starts_with("DISC-TRM-")
                    || e.from.starts_with("HYP-TR-")
                    || e.from.starts_with("HYP-MED-"))
        }).count();
        assert!(merges >= 20, "Should have 20+ cross-domain merge edges, got {}", merges);
    }

    #[test]
    fn test_full_trm_pipeline() {
        let mut g = base_graph();
        let (total_nodes, total_edges) = populate_all_trm_discoveries(&mut g);
        assert_eq!(total_nodes, 26, "26 total TRM nodes added");
        assert!(total_edges > 80, "Should have 80+ edges, got {}", total_edges);
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = base_graph();
        populate_all_trm_discoveries(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_TRM_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_transport_discoveries(&mut g);
        populate_medical_discoveries(&mut g);
        populate_trm_bridges(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();

        assert_eq!(discoveries, 24, "24 discovery nodes");
        assert_eq!(hypotheses, 2, "2 hypothesis nodes");
        assert_eq!(discoveries + hypotheses, 26);
    }

    #[test]
    fn test_bridge_spans_transport_and_medical() {
        let mut g = base_graph();
        populate_all_trm_discoveries(&mut g);

        let bridge = g.nodes.iter().find(|n| n.id == "DISC-TRM-01").unwrap();
        let domains: Vec<&str> = bridge.domain.split(", ").collect();
        assert!(domains.contains(&"Transportation"), "TRM-01 must span Transportation");
        assert!(domains.contains(&"Medical"), "TRM-01 must span Medical");
    }

    #[test]
    fn test_bridge_ecg_transformer() {
        let mut g = base_graph();
        populate_all_trm_discoveries(&mut g);

        let bridge = g.nodes.iter().find(|n| n.id == "DISC-TRM-03").unwrap();
        let domains: Vec<&str> = bridge.domain.split(", ").collect();
        assert!(domains.contains(&"Medical"), "TRM-03 ECG-transformer bridge must span Medical");
        assert!(domains.contains(&"AI"), "TRM-03 ECG-transformer bridge must span AI");
    }

    #[test]
    fn test_transport_bts_present() {
        let g = base_graph();
        assert!(g.nodes.iter().any(|n| n.id == "BT-233"), "BT-233 must be in base graph");
        assert!(g.nodes.iter().any(|n| n.id == "BT-246"), "BT-246 must be in base graph");
    }

    #[test]
    fn test_medical_bts_present() {
        let g = base_graph();
        assert!(g.nodes.iter().any(|n| n.id == "BT-238"), "BT-238 must be in base graph");
        assert!(g.nodes.iter().any(|n| n.id == "BT-242"), "BT-242 must be in base graph");
    }
}
