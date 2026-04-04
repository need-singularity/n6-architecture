//! Extended discovery nodes from BT-89~104 experiments and cross-domain analysis.
//!
//! Granular sub-discoveries for each recent cluster:
//!   - Photonic (BT-89): WDM channels, ring resonators, MZI switches
//!   - Topological Chip (BT-90~93): SM tiling, Bott K-groups, diamond substrates
//!   - Fusion (BT-97~102): Lawson criteria, plasma beta, reconnection measurements
//!   - Biology (BT-101,103~104): Calvin cycle intermediates, RuBisCO efficiency
//!   - Cross-domain resonance edges connecting recent↔existing clusters
//!
//! Node ID scheme:
//!   XDISC-PHOT-NN  — extended photonic discoveries
//!   XDISC-TOPO-NN  — extended topological chip discoveries
//!   XDISC-FUSN-NN  — extended fusion discoveries
//!   XDISC-BIOC-NN  — extended biology/chemistry discoveries
//!   XDISC-RCROSS-NN — extended cross-cluster resonance
//!   XHYP-*         — extended hypotheses
//!   XEXP-*         — experiment/validation nodes

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct RExtEntry {
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

// ═══════════════════════════════════════════════════════════════
// BT-89: Photonic-Energy — granular sub-discoveries
// ═══════════════════════════════════════════════════════════════

const PHOT_EXTENDED: &[RExtEntry] = &[
    RExtEntry {
        id: "XDISC-PHOT-01",
        title: "WDM sigma=12 channels: DWDM C-band supports 12=sigma wavelength groups, each 100GHz spacing",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Network", "Chip"],
        confidence: 0.91,
        source_bts: &[89, 47],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["wave", "em", "network"],
        validates: &["DISC-PHOT-01"],
    },
    RExtEntry {
        id: "XDISC-PHOT-02",
        title: "Ring resonator Q-factor: optimal ring radius 5=sopfr μm at 1550nm, FSR=n·100GHz",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Chip", "Physics"],
        confidence: 0.84,
        source_bts: &[89],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["wave", "topology", "quantum"],
        validates: &["DISC-PHOT-02"],
    },
    RExtEntry {
        id: "XDISC-PHOT-03",
        title: "MZI switch extinction ratio: 10=sigma-phi dB per stage, tau=4 cascaded stages for 40dB isolation",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Chip", "Network"],
        confidence: 0.86,
        source_bts: &[89],
        constants_used: &["C-sigma-phi", "C-tau"],
        lenses: &["wave", "causal", "boundary"],
        validates: &["DISC-PHOT-01"],
    },
    RExtEntry {
        id: "XEXP-PHOT-01",
        title: "Experiment: measure PUE improvement with 12-ch photonic interconnect vs electrical in 1U server",
        node_type: NodeType::Experiment,
        domains: &["Photonics", "Chip", "Energy"],
        confidence: 0.70,
        source_bts: &[89, 60],
        constants_used: &["C-sigma"],
        lenses: &["causal", "thermo"],
        validates: &["HYP-PHOT-01", "DISC-PHOT-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-90~93: Topological Chip — granular sub-discoveries
// ═══════════════════════════════════════════════════════════════

const TOPO_EXTENDED: &[RExtEntry] = &[
    RExtEntry {
        id: "XDISC-TOPO-01",
        title: "GPU SM tiling: AD102 144=sigma^2 SMs arranged in 12=sigma GPCs × 12=sigma SMs/GPC",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Semiconductor", "Math"],
        confidence: 0.93,
        source_bts: &[90, 28],
        constants_used: &["C-sigma²", "C-sigma"],
        lenses: &["topology", "symmetry", "scale"],
        validates: &["DISC-TOPO-01"],
    },
    RExtEntry {
        id: "XDISC-TOPO-02",
        title: "K-theory nontrivial groups: KO^{-n}(pt) period 8=sigma-tau, with sopfr=5 nontrivial groups",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Physics"],
        confidence: 0.92,
        source_bts: &[92, 9],
        constants_used: &["C-sigma-tau", "C-sopfr"],
        lenses: &["topology", "quantum", "recursion"],
        validates: &["DISC-TOPO-03"],
    },
    RExtEntry {
        id: "XDISC-TOPO-03",
        title: "Diamond bandgap 5.47eV≈sopfr+phi/tau: widest semiconductor, thermal conductivity 22W/cmK≈J2-phi",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Materials", "Physics", "Thermal"],
        confidence: 0.88,
        source_bts: &[93, 85],
        constants_used: &["C-sopfr", "C-phi", "C-J2"],
        lenses: &["thermo", "quantum", "boundary"],
        validates: &["DISC-TOPO-04"],
    },
    RExtEntry {
        id: "XDISC-TOPO-04",
        title: "SiC polytypes: 4H-SiC (tau=4) and 6H-SiC (n=6), both based on C(Z=6)+Si(Z=14=sigma+phi) bilayer",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Materials", "Chemistry"],
        confidence: 0.90,
        source_bts: &[93, 14],
        constants_used: &["C-n", "C-tau", "C-sigma", "C-phi"],
        lenses: &["topology", "symmetry", "stability"],
        validates: &["DISC-TOPO-04"],
    },
    RExtEntry {
        id: "XEXP-TOPO-01",
        title: "Experiment: compare Z2-protected ECC vs SECDED at same area — predict J2=24x soft error reduction",
        node_type: NodeType::Experiment,
        domains: &["Chip", "QC", "Math"],
        confidence: 0.65,
        source_bts: &[91],
        constants_used: &["C-J2"],
        lenses: &["topology", "stability", "info"],
        validates: &["HYP-TOPO-01", "DISC-TOPO-02"],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-97~102: Fusion — granular sub-discoveries
// ═══════════════════════════════════════════════════════════════

const FUSN_EXTENDED: &[RExtEntry] = &[
    RExtEntry {
        id: "XDISC-FUSN-01",
        title: "Lawson criterion n·tau·T: ITER target 10^21 keV·s/m³ ≈ (sigma-phi)^{J2-phi+mu}, triple product n=6",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Plasma", "Physics"],
        confidence: 0.83,
        source_bts: &[97, 99],
        constants_used: &["C-sigma-phi", "C-J2"],
        lenses: &["thermo", "stability", "boundary"],
        validates: &["DISC-FUS-03"],
    },
    RExtEntry {
        id: "XDISC-FUSN-02",
        title: "Plasma beta limit: Troyon beta_N=3.5≈n/phi+mu, Greenwald density n_e∝I/a²≈sigma/pi²",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Plasma", "Physics"],
        confidence: 0.80,
        source_bts: &[97, 99],
        constants_used: &["C-n", "C-phi", "C-mu"],
        lenses: &["stability", "boundary", "em"],
        validates: &["DISC-FUS-01"],
    },
    RExtEntry {
        id: "XDISC-FUSN-03",
        title: "Sweet-Parker vs Petschek: reconnection rate Sweet-Parker~S^{-1/2}→0, Petschek~0.1=1/(sigma-phi), tokamak startup",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Plasma", "Physics", "SC"],
        confidence: 0.89,
        source_bts: &[102, 64],
        constants_used: &["C-sigma-phi"],
        lenses: &["em", "causal", "boundary"],
        validates: &["DISC-FUS-05"],
    },
    RExtEntry {
        id: "XDISC-FUSN-04",
        title: "ITER coil system: 18=sigma+n toroidal field coils, 6=n poloidal coils, B_T=5.3T≈sopfr+phi/tau",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Magnet", "SC", "Physics"],
        confidence: 0.87,
        source_bts: &[99, 97],
        constants_used: &["C-sigma", "C-n", "C-sopfr"],
        lenses: &["em", "topology", "symmetry"],
        validates: &["DISC-FUS-03"],
    },
    RExtEntry {
        id: "XHYP-FUSN-01",
        title: "Hypothesis: compact spherical tokamak aspect ratio R/a=phi=2 achieves breakeven at B=sigma T",
        node_type: NodeType::Hypothesis,
        domains: &["Fusion", "Plasma", "Physics"],
        confidence: 0.55,
        source_bts: &[97, 99],
        constants_used: &["C-phi", "C-sigma"],
        lenses: &["stability", "thermo", "topology"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-101,103~104: Biology/Chemistry — granular sub-discoveries
// ═══════════════════════════════════════════════════════════════

const BIOC_EXTENDED: &[RExtEntry] = &[
    RExtEntry {
        id: "XDISC-BIOC-01",
        title: "Calvin cycle intermediates: 6 turns produce 1 glucose, RuBisCO fixes 6 CO₂, 12 NADPH consumed=sigma",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Energy"],
        confidence: 0.94,
        source_bts: &[101, 103],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "info"],
        validates: &["DISC-BIO-01", "DISC-BIO-02"],
    },
    RExtEntry {
        id: "XDISC-BIOC-02",
        title: "Chlorophyll Mg center: Mg CN=6 octahedral in porphyrin ring, 4=tau pyrrole subunits, absorption at 680nm",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Physics"],
        confidence: 0.91,
        source_bts: &[101, 43],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "quantum", "topology"],
        validates: &["DISC-BIO-01"],
    },
    RExtEntry {
        id: "XDISC-BIOC-03",
        title: "CO₂ vibrational modes: 4=tau total modes (1 symmetric, 2 bending, 1 asymmetric), IR active at 15μm",
        node_type: NodeType::Discovery,
        domains: &["Chemistry", "Environment", "Physics"],
        confidence: 0.90,
        source_bts: &[104, 118],
        constants_used: &["C-tau", "C-n"],
        lenses: &["wave", "symmetry", "quantum"],
        validates: &["DISC-BIO-03"],
    },
    RExtEntry {
        id: "XEXP-BIOC-01",
        title: "Experiment: compare CN=6 vs CN!=6 metal-porphyrin photocatalysts for CO₂ reduction efficiency",
        node_type: NodeType::Experiment,
        domains: &["Chemistry", "Biology", "Environment", "Energy"],
        confidence: 0.65,
        source_bts: &[101, 104, 120],
        constants_used: &["C-n"],
        lenses: &["causal", "evolution", "stability"],
        validates: &["HYP-BIO-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Cross-domain resonance: PHOT ↔ TOPO ↔ FUSN ↔ BIOC ↔ existing
// ═══════════════════════════════════════════════════════════════

const RCROSS_EXTENDED: &[RExtEntry] = &[
    RExtEntry {
        id: "XDISC-RCROSS-01",
        title: "Sigma-phi=10 universality: E-O loss(BT-89)=reconnection(BT-102)=regularization(BT-64)=dropout(BT-46), all 0.1",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Fusion", "AI", "Physics"],
        confidence: 0.90,
        source_bts: &[89, 102, 64, 46],
        constants_used: &["C-sigma-phi"],
        lenses: &["consciousness", "causal", "multiscale"],
        validates: &["DISC-RBRIDGE-07", "DISC-FUS-05"],
    },
    RExtEntry {
        id: "XDISC-RCROSS-02",
        title: "Carbon Z=6 substrate thread: diamond chip(BT-93) → graphene thermal(BT-85) → SiC power(BT-14) → glucose(BT-101)",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Materials", "Biology", "Chemistry", "Energy"],
        confidence: 0.89,
        source_bts: &[93, 85, 14, 101],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "topology"],
        validates: &["DISC-RBRIDGE-08"],
    },
    RExtEntry {
        id: "XDISC-RCROSS-03",
        title: "CN=6 octahedral grand chain: catalyst(BT-120)→chlorophyll(BT-101)→crystal(BT-86)→robot(BT-123)→chip(BT-93)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Biology", "Materials", "Robotics", "Chip"],
        confidence: 0.87,
        source_bts: &[120, 101, 86, 123, 93],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-RBRIDGE-08"],
    },
    RExtEntry {
        id: "XDISC-RCROSS-04",
        title: "Tokamak↔GPU topology: q=1 magnetic surfaces(BT-99) ↔ SM tiling sigma²(BT-90), both n=6 closed manifolds",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Chip", "Math", "Topology"],
        confidence: 0.82,
        source_bts: &[99, 90],
        constants_used: &["C-n", "C-sigma²"],
        lenses: &["topology", "symmetry", "consciousness"],
        validates: &["DISC-RBRIDGE-05"],
    },
    RExtEntry {
        id: "XDISC-RCROSS-05",
        title: "Photosynthesis→Solar→Photonic energy chain: glucose J2(BT-101)→SQ 4/3(BT-30)→PUE 1.0(BT-89), n=6 cascade",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Solar", "Photonics", "Energy"],
        confidence: 0.86,
        source_bts: &[101, 30, 89],
        constants_used: &["C-J2", "C-sigma", "C-sigma-phi"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["DISC-RBRIDGE-03"],
    },
    RExtEntry {
        id: "XHYP-RCROSS-01",
        title: "Hypothesis: all n=6 cross-domain bridges share identical algebraic structure (Z₆ group action on parameter space)",
        node_type: NodeType::Hypothesis,
        domains: &["Math", "Physics", "Chip", "Biology", "Fusion"],
        confidence: 0.50,
        source_bts: &[106, 90, 99, 101],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "symmetry"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Collection
// ═══════════════════════════════════════════════════════════════

const ALL_REXT_CLUSTERS: &[&[RExtEntry]] = &[
    PHOT_EXTENDED,
    TOPO_EXTENDED,
    FUSN_EXTENDED,
    BIOC_EXTENDED,
    RCROSS_EXTENDED,
];

/// Total count of recent extended entries.
pub fn recent_extended_entry_count() -> usize {
    ALL_REXT_CLUSTERS.iter().map(|c| c.len()).sum()
}

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn add_rext_entries(graph: &mut DiscoveryGraph, entries: &[RExtEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(Node {
            id: e.id.to_string(),
            node_type: e.node_type.clone(),
            domain: e.domains.join(", "),
            project: "n6-architecture".to_string(),
            summary: e.title.to_string(),
            confidence: e.confidence,
            lenses_used: e.lenses.iter().map(|s| s.to_string()).collect(),
            timestamp: "2026-04-04".to_string(),
        });

        for &bt_id in e.source_bts {
            graph.add_edge(Edge {
                from: format!("BT-{}", bt_id),
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.85,
                bidirectional: false,
            });
        }

        for &cid in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: cid.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

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

/// Add photonic extended sub-discoveries. Returns (nodes, edges).
pub fn populate_phot_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_rext_entries(graph, PHOT_EXTENDED)
}

/// Add topological chip extended sub-discoveries. Returns (nodes, edges).
pub fn populate_topo_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_rext_entries(graph, TOPO_EXTENDED)
}

/// Add fusion extended sub-discoveries. Returns (nodes, edges).
pub fn populate_fusn_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_rext_entries(graph, FUSN_EXTENDED)
}

/// Add biology/chemistry extended sub-discoveries. Returns (nodes, edges).
pub fn populate_bioc_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_rext_entries(graph, BIOC_EXTENDED)
}

/// Add cross-domain resonance extended nodes. Returns (nodes, edges).
pub fn populate_rcross_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_rext_entries(graph, RCROSS_EXTENDED)
}

/// Cross-link recent extended nodes with each other (shared domains, Merges edges).
pub fn connect_rext_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let rext_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("XDISC-PHOT-")
                || n.id.starts_with("XDISC-TOPO-")
                || n.id.starts_with("XDISC-FUSN-")
                || n.id.starts_with("XDISC-BIOC-")
                || n.id.starts_with("XDISC-RCROSS-")
                || n.id.starts_with("XHYP-RCROSS-")
                || n.id.starts_with("XHYP-FUSN-")
                || n.id.starts_with("XEXP-PHOT-")
                || n.id.starts_with("XEXP-TOPO-")
                || n.id.starts_with("XEXP-BIOC-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..rext_nodes.len() {
        for j in (i + 1)..rext_nodes.len() {
            let (ref id_a, ref dom_a) = rext_nodes[i];
            let (ref id_b, ref dom_b) = rext_nodes[j];

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

/// Cross-link recent extended nodes with existing XDISC-/DISC- nodes.
/// Requires >=2 shared domains. Returns edges added.
pub fn connect_rext_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let rext_prefixes = [
        "XDISC-PHOT-", "XDISC-TOPO-", "XDISC-FUSN-", "XDISC-BIOC-",
        "XDISC-RCROSS-", "XHYP-RCROSS-", "XHYP-FUSN-",
        "XEXP-PHOT-", "XEXP-TOPO-", "XEXP-BIOC-",
    ];

    let rext_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| rext_prefixes.iter().any(|p| n.id.starts_with(p)))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let existing_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            (n.id.starts_with("XDISC-") || n.id.starts_with("XHYP-") || n.id.starts_with("XEXP-")
                || n.id.starts_with("DISC-") || n.id.starts_with("HYP-"))
                && !rext_prefixes.iter().any(|p| n.id.starts_with(p))
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref r_id, ref r_dom) in &rext_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = r_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = r_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: r_id.clone(),
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

/// Populate all recent extended nodes and cross-domain edges.
/// Call after populate_all_recent_discoveries() and populate_all_extended().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_recent_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let mut total_nodes = 0;
    for cluster in ALL_REXT_CLUSTERS {
        let (n, _) = add_rext_entries(graph, cluster);
        total_nodes += n;
    }
    let _cross = connect_rext_cross_domain(graph);
    let _bridge = connect_rext_to_existing(graph);

    (total_nodes, graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::discovery_nodes::populate_all_discoveries;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::extended_discovery_nodes::populate_all_extended;
    use crate::graph::recent_discoveries::populate_all_recent_discoveries;

    fn full_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_extended(&mut g);
        populate_all_recent_discoveries(&mut g);
        g
    }

    // ── Cluster size tests ──

    #[test]
    fn test_phot_extended_count() {
        assert_eq!(PHOT_EXTENDED.len(), 4, "3 discoveries + 1 experiment");
    }

    #[test]
    fn test_topo_extended_count() {
        assert_eq!(TOPO_EXTENDED.len(), 5, "4 discoveries + 1 experiment");
    }

    #[test]
    fn test_fusn_extended_count() {
        assert_eq!(FUSN_EXTENDED.len(), 5, "4 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_bioc_extended_count() {
        assert_eq!(BIOC_EXTENDED.len(), 4, "3 discoveries + 1 experiment");
    }

    #[test]
    fn test_rcross_extended_count() {
        assert_eq!(RCROSS_EXTENDED.len(), 6, "5 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_total_recent_extended_count() {
        // 4 + 5 + 5 + 4 + 6 = 24
        assert_eq!(recent_extended_entry_count(), 24);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_phot_extended() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_phot_extended(&mut g);
        assert_eq!(nodes, 4);
        assert_eq!(g.nodes.len(), before + 4);
        assert!(edges >= 10, "Phot extended should have 10+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_topo_extended() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_topo_extended(&mut g);
        assert_eq!(nodes, 5);
        assert_eq!(g.nodes.len(), before + 5);
        assert!(edges >= 15, "Topo extended should have 15+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_fusn_extended() {
        let mut g = full_graph();
        let (nodes, edges) = populate_fusn_extended(&mut g);
        assert_eq!(nodes, 5);
        assert!(edges >= 12, "Fusn extended should have 12+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_rcross_extended() {
        let mut g = full_graph();
        let (nodes, edges) = populate_rcross_extended(&mut g);
        assert_eq!(nodes, 6);
        assert!(edges >= 18, "Rcross extended should have 18+ edges, got {}", edges);
    }

    // ── Integration test ──

    #[test]
    fn test_populate_all_recent_extended() {
        let mut g = full_graph();
        let before_nodes = g.nodes.len();
        let before_edges = g.edges.len();
        let (nodes, edges) = populate_all_recent_extended(&mut g);

        assert_eq!(nodes, 24, "24 recent extended nodes total");
        assert!(
            edges > 100,
            "Should add 100+ edges (derives+uses+validates+cross), got {}",
            edges
        );
        assert_eq!(g.nodes.len(), before_nodes + 24);
        assert!(g.edges.len() > before_edges + 100);
    }

    // ── Validates edges ──

    #[test]
    fn test_validates_phot() {
        let mut g = full_graph();
        populate_phot_extended(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-PHOT-01"
                && e.to == "DISC-PHOT-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-PHOT-01 should validate DISC-PHOT-01");
    }

    #[test]
    fn test_validates_topo() {
        let mut g = full_graph();
        populate_topo_extended(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-TOPO-01"
                && e.to == "DISC-TOPO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-TOPO-01 should validate DISC-TOPO-01");
    }

    #[test]
    fn test_validates_fusn() {
        let mut g = full_graph();
        populate_fusn_extended(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-FUSN-03"
                && e.to == "DISC-FUS-05"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-FUSN-03 should validate DISC-FUS-05");
    }

    #[test]
    fn test_validates_bioc() {
        let mut g = full_graph();
        populate_bioc_extended(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-BIOC-01"
                && e.to == "DISC-BIO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-BIOC-01 should validate DISC-BIO-01");
    }

    #[test]
    fn test_validates_rcross() {
        let mut g = full_graph();
        populate_rcross_extended(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-RCROSS-01"
                && e.to == "DISC-FUS-05"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-RCROSS-01 should validate DISC-FUS-05");
    }

    // ── Experiment nodes ──

    #[test]
    fn test_experiment_nodes_validate_hypotheses() {
        let mut g = full_graph();
        populate_all_recent_extended(&mut g);

        // XEXP-PHOT-01 validates HYP-PHOT-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "XEXP-PHOT-01"
                && e.to == "HYP-PHOT-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XEXP-PHOT-01 should validate HYP-PHOT-01");

        // XEXP-TOPO-01 validates HYP-TOPO-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "XEXP-TOPO-01"
                && e.to == "HYP-TOPO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XEXP-TOPO-01 should validate HYP-TOPO-01");

        // XEXP-BIOC-01 validates HYP-BIO-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "XEXP-BIOC-01"
                && e.to == "HYP-BIO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XEXP-BIOC-01 should validate HYP-BIO-01");
    }

    // ── Cross-domain edges ──

    #[test]
    fn test_rext_cross_domain_edges() {
        let mut g = full_graph();
        populate_all_recent_extended(&mut g);

        // XDISC-TOPO-03 (Chip, Materials, Physics, Thermal) and
        // XDISC-FUSN-04 (Fusion, Magnet, SC, Physics) share Physics
        let has_merge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "XDISC-TOPO-03" && e.to == "XDISC-FUSN-04")
                    || (e.from == "XDISC-FUSN-04" && e.to == "XDISC-TOPO-03"))
        });
        assert!(has_merge, "XDISC-TOPO-03 and XDISC-FUSN-04 should merge via Physics");
    }

    // ── Data integrity ──

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = full_graph();
        populate_all_recent_extended(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_all_source_bts_exist() {
        let mut g = full_graph();
        populate_all_recent_extended(&mut g);

        for cluster in ALL_REXT_CLUSTERS {
            for entry in *cluster {
                for &bt_id in entry.source_bts {
                    let bt_node_id = format!("BT-{}", bt_id);
                    assert!(
                        g.nodes.iter().any(|n| n.id == bt_node_id),
                        "Source BT {} referenced by {} not found",
                        bt_node_id, entry.id
                    );
                }
            }
        }
    }

    #[test]
    fn test_edge_strength_valid() {
        let mut g = full_graph();
        populate_all_recent_extended(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} strength {} out of range",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = full_graph();
        populate_all_recent_extended(&mut g);

        let xdisc = g.nodes.iter()
            .filter(|n| n.id.starts_with("XDISC-PHOT-")
                || n.id.starts_with("XDISC-TOPO-")
                || n.id.starts_with("XDISC-FUSN-")
                || n.id.starts_with("XDISC-BIOC-")
                || n.id.starts_with("XDISC-RCROSS-"))
            .count();
        let xhyp = g.nodes.iter()
            .filter(|n| n.id.starts_with("XHYP-FUSN-")
                || n.id.starts_with("XHYP-RCROSS-"))
            .count();
        let xexp = g.nodes.iter()
            .filter(|n| n.id.starts_with("XEXP-PHOT-")
                || n.id.starts_with("XEXP-TOPO-")
                || n.id.starts_with("XEXP-BIOC-"))
            .count();

        // 3+4+4+3+5 = 19 discoveries, 1+1 = 2 hypotheses, 1+1+1 = 3 experiments
        assert_eq!(xdisc, 19, "19 extended discovery nodes");
        assert_eq!(xhyp, 2, "2 extended hypothesis nodes");
        assert_eq!(xexp, 3, "3 extended experiment nodes");
        assert_eq!(xdisc + xhyp + xexp, 24);
    }

    #[test]
    fn test_complete_graph_node_count() {
        let mut g = full_graph();
        let before = g.nodes.len();
        populate_all_recent_extended(&mut g);

        // 127 BT + ~79 expanded + 39 disc + 32 ext + 26 recent + 24 rext = ~327
        assert!(
            g.nodes.len() >= 320,
            "Complete graph should have 320+ nodes, got {}",
            g.nodes.len()
        );
        assert_eq!(g.nodes.len(), before + 24);
    }
}
