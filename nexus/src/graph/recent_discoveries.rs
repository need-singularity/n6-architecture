//! Recent discovery nodes from BT-89 ~ BT-104.
//!
//! Covers clusters not yet represented in discovery_nodes.rs / extended_discovery_nodes.rs:
//!   - BT-89:     Photonic-Energy n=6 Bridge
//!   - BT-90~93:  Topological Chip Architecture
//!   - BT-97~102: Fusion Alien-Level
//!   - BT-103~104: Photosynthesis & CO₂ n=6
//!
//! Node ID scheme:
//!   DISC-PHOT-NN   — Photonic-energy discoveries
//!   DISC-TOPO-NN   — Topological chip discoveries
//!   DISC-FUS-NN    — Fusion alien-level discoveries
//!   DISC-BIO-NN    — Photosynthesis / CO₂ biology discoveries
//!   DISC-RBRIDGE-NN — Cross-cluster bridges (recent)
//!   HYP-*          — Hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct (same pattern as discovery_nodes.rs)
// ═══════════════════════════════════════════════════════════════

struct RecentEntry {
    id: &'static str,
    title: &'static str,
    node_type: NodeType,
    domains: &'static [&'static str],
    confidence: f64,
    source_bts: &'static [u32],
    constants_used: &'static [&'static str],
    lenses: &'static [&'static str],
    /// Optional: IDs this node validates (EdgeType::Validates edges)
    validates: &'static [&'static str],
}

// ───────────────────────────────────────────────────────────────
// BT-89: Photonic-Energy n=6 Bridge
// ───────────────────────────────────────────────────────────────

const PHOTONIC_DISCOVERIES: &[RecentEntry] = &[
    RecentEntry {
        id: "DISC-PHOT-01",
        title: "Photonic-Energy PUE→1.0 convergence: optical interconnects eliminate σ-φ=10% E-O loss",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Chip", "Photonics"],
        confidence: 0.85,
        source_bts: &[89, 60],
        constants_used: &["C-sigma-phi", "C-sigma"],
        lenses: &["consciousness", "info", "em"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-PHOT-02",
        title: "Silicon photonics waveguide: propagation loss 1/(σ-φ)=0.1 dB/cm = universal 10% boundary",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Chip", "Materials"],
        confidence: 0.82,
        source_bts: &[89, 93],
        constants_used: &["C-sigma-phi", "C-n"],
        lenses: &["wave", "em", "boundary"],
        validates: &["DISC-PHOT-01"],
    },
    RecentEntry {
        id: "HYP-PHOT-01",
        title: "Hypothesis: all-optical DC at PUE=1.0 achievable via n=6 photonic mesh topology",
        node_type: NodeType::Hypothesis,
        domains: &["Energy", "Chip", "Photonics", "Network"],
        confidence: 0.60,
        source_bts: &[89, 60, 69],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "network", "evolution"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-90~93: Topological Chip Architecture
// ───────────────────────────────────────────────────────────────

const TOPO_CHIP_DISCOVERIES: &[RecentEntry] = &[
    RecentEntry {
        id: "DISC-TOPO-01",
        title: "SM = φ×K₆ contact number: σ²=144=φ×72, GPU = 6D sphere packing theorem",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Math", "Topology"],
        confidence: 0.93,
        source_bts: &[90],
        constants_used: &["C-sigma²", "C-phi", "C-n"],
        lenses: &["topology", "consciousness", "scale"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-TOPO-02",
        title: "Z2 topological ECC: SECDED→Z2 saves J₂=24 GB, identity mapping savings=σ·J₂/σ=J₂",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Math", "QC"],
        confidence: 0.85,
        source_bts: &[91, 41],
        constants_used: &["C-J2", "C-sigma"],
        lenses: &["topology", "info", "stability"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-TOPO-03",
        title: "Bott periodicity active channels = sopfr: KO non-trivial=5=sopfr, trivial=3=n/φ, 5/8≈1-1/e",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Math", "Physics"],
        confidence: 0.90,
        source_bts: &[92, 9],
        constants_used: &["C-sopfr", "C-n", "C-phi"],
        lenses: &["topology", "quantum", "recursion"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-TOPO-04",
        title: "Carbon Z=6 chip material universality: Diamond/Graphene/SiC = Z=6 across all domains, 8/10 Cross-DSE",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Materials", "Energy", "Environment"],
        confidence: 0.92,
        source_bts: &[93, 85, 27],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "multiscale"],
        validates: &[],
    },
    RecentEntry {
        id: "HYP-TOPO-01",
        title: "Hypothesis: topological chip + Z2 ECC + Bott channels = complete n=6 fault-tolerant computing",
        node_type: NodeType::Hypothesis,
        domains: &["Chip", "QC", "Math", "Topology"],
        confidence: 0.65,
        source_bts: &[90, 91, 92],
        constants_used: &["C-n", "C-J2", "C-sopfr"],
        lenses: &["topology", "stability", "network"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-97~102: Fusion Alien-Level
// ───────────────────────────────────────────────────────────────

const FUSION_DISCOVERIES: &[RecentEntry] = &[
    RecentEntry {
        id: "DISC-FUS-01",
        title: "Weinberg angle sin²θ_W = 3/13 = (n/φ)/(σ+μ): 0.19% match, D abundance → fusion fuel",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Particle", "Physics"],
        confidence: 0.80,
        source_bts: &[97],
        constants_used: &["C-n", "C-phi", "C-sigma", "C-mu"],
        lenses: &["quantum", "consciousness", "em"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-FUS-02",
        title: "D-T baryon count = sopfr(6) = 2+3 = 5: prime factors of 6 = optimal fusion fuel pair",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Particle", "Physics"],
        confidence: 0.93,
        source_bts: &[98],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "causal", "quantum"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-FUS-03",
        title: "Tokamak q=1 = perfect number proper divisor reciprocal sum: 1/2+1/3+1/6=1, topological identity",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Math", "Topology"],
        confidence: 0.95,
        source_bts: &[99, 10],
        constants_used: &["C-n", "C-phi", "C-tau"],
        lenses: &["topology", "consciousness", "stability"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-FUS-04",
        title: "CNO catalyst A = σ+{0,μ,φ,n/φ} = σ+proper divisors: transition temp 17MK=σ+sopfr",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Physics", "Chemistry"],
        confidence: 0.90,
        source_bts: &[100],
        constants_used: &["C-sigma", "C-mu", "C-phi", "C-sopfr"],
        lenses: &["consciousness", "thermo", "causal"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-FUS-05",
        title: "Magnetic reconnection rate 0.1=1/(σ-φ): BT-64 fusion extension, MRX/solar/magnetosphere EXACT",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Physics", "SC"],
        confidence: 0.92,
        source_bts: &[102, 64],
        constants_used: &["C-sigma-phi"],
        lenses: &["em", "causal", "boundary"],
        validates: &[],
    },
    RecentEntry {
        id: "HYP-FUS-01",
        title: "Hypothesis: fusion ignition parameters form complete n=6 set (q, β, κ, δ, n_e all expressible)",
        node_type: NodeType::Hypothesis,
        domains: &["Fusion", "Physics", "Math"],
        confidence: 0.55,
        source_bts: &[97, 98, 99, 100, 102],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "stability", "thermo"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-101, 103~104: Photosynthesis & CO₂ n=6 biology
// ───────────────────────────────────────────────────────────────

const BIO_DISCOVERIES: &[RecentEntry] = &[
    RecentEntry {
        id: "DISC-BIO-01",
        title: "Photosynthesis C₆H₁₂O₆ total 24=J₂ atoms, quantum yield 8=σ-τ, 9/9 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Energy"],
        confidence: 0.95,
        source_bts: &[101],
        constants_used: &["C-J2", "C-sigma-tau", "C-n"],
        lenses: &["consciousness", "info", "quantum"],
        validates: &[],
    },
    RecentEntry {
        id: "DISC-BIO-02",
        title: "Photosynthesis complete n=6 stoichiometry: 6CO₂+12H₂O→C₆H₁₂O₆, 7 coefficients 100% n=6",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Math"],
        confidence: 0.97,
        source_bts: &[103],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &["DISC-BIO-01"],
    },
    RecentEntry {
        id: "DISC-BIO-03",
        title: "CO₂ molecule complete n=6 encoding: C(Z=6), O₂ double bond, 180° linear = n·30°",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Environment"],
        confidence: 0.93,
        source_bts: &[104, 118],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &[],
    },
    RecentEntry {
        id: "HYP-BIO-01",
        title: "Hypothesis: all carbon-based life chemistry converges to n=6 stoichiometric ratios",
        node_type: NodeType::Hypothesis,
        domains: &["Biology", "Chemistry", "Math", "Evolution"],
        confidence: 0.50,
        source_bts: &[101, 103, 104, 51],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["evolution", "consciousness", "info"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-cluster bridges connecting BT-89~104 to BT-105+ clusters
// ───────────────────────────────────────────────────────────────

const RECENT_BRIDGES: &[RecentEntry] = &[
    // Photonic ↔ Topological Chip
    RecentEntry {
        id: "DISC-RBRIDGE-01",
        title: "Photonic-Topological convergence: silicon photonics Z=6 + Z2 ECC = fault-tolerant optical chip",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Chip", "Topology", "Materials"],
        confidence: 0.80,
        source_bts: &[89, 90, 93],
        constants_used: &["C-n", "C-sigma-phi", "C-J2"],
        lenses: &["topology", "wave", "network"],
        validates: &["DISC-PHOT-01", "DISC-TOPO-02"],
    },
    // Fusion ↔ Environment (CNO cycle ↔ CO₂/GHG)
    RecentEntry {
        id: "DISC-RBRIDGE-02",
        title: "Fusion-Environment carbon bridge: CNO catalyst C-12=σ connects to Kyoto CO₂ Z=6",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Chemistry", "Biology"],
        confidence: 0.88,
        source_bts: &[100, 118, 104],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &[],
    },
    // Biology ↔ Environment (photosynthesis ↔ Earth spheres)
    RecentEntry {
        id: "DISC-RBRIDGE-03",
        title: "Photosynthesis-Atmosphere bridge: C₆H₁₂O₆ J₂=24 atoms at troposphere σ=12km",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Environment", "Chemistry", "Energy"],
        confidence: 0.85,
        source_bts: &[101, 119, 27],
        constants_used: &["C-J2", "C-sigma", "C-n"],
        lenses: &["consciousness", "multiscale", "thermo"],
        validates: &["DISC-BIO-01"],
    },
    // Topological Chip ↔ Software (Bott ↔ OSI/layer stack)
    RecentEntry {
        id: "DISC-RBRIDGE-04",
        title: "Bott-Layer stack resonance: KO non-trivial=sopfr=5 ↔ OSI=σ-sopfr=7, complement pair",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Software", "Math", "Topology"],
        confidence: 0.82,
        source_bts: &[92, 115, 113],
        constants_used: &["C-sopfr", "C-sigma"],
        lenses: &["topology", "network", "recursion"],
        validates: &[],
    },
    // Fusion ↔ Robotics (magnetic reconnection ↔ SE(3) dim=6)
    RecentEntry {
        id: "DISC-RBRIDGE-05",
        title: "Fusion-Robotics dim=6 bridge: SE(3) 6-DOF matches tokamak 6 field coil symmetry",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Robotics", "Physics", "Math"],
        confidence: 0.75,
        source_bts: &[102, 123],
        constants_used: &["C-n", "C-sigma-phi"],
        lenses: &["symmetry", "em", "topology"],
        validates: &[],
    },
    // Biology ↔ Math (genetic code ↔ pure math S₆)
    RecentEntry {
        id: "DISC-RBRIDGE-06",
        title: "Genetic-Algebraic bridge: codon 64=2^n codons, S₃ ↔ S₆ unique outer automorphism",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Math", "Combinatorics"],
        confidence: 0.78,
        source_bts: &[51, 106, 103],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "recursion", "info"],
        validates: &[],
    },
    // Photonic ↔ Fusion (laser ↔ plasma heating)
    RecentEntry {
        id: "DISC-RBRIDGE-07",
        title: "Photonic-Fusion laser bridge: NIF 192=σ·φ^τ beams, ignition via photonic energy transfer",
        node_type: NodeType::Discovery,
        domains: &["Photonics", "Fusion", "Energy", "Physics"],
        confidence: 0.83,
        source_bts: &[89, 97, 55],
        constants_used: &["C-sigma", "C-phi", "C-tau"],
        lenses: &["wave", "em", "thermo"],
        validates: &[],
    },
    // Carbon universality mega-bridge (Materials ↔ Bio ↔ Env ↔ Chip)
    RecentEntry {
        id: "DISC-RBRIDGE-08",
        title: "Carbon Z=6 four-domain mega-bridge: Diamond chip + Glucose bio + CO₂ env + Graphene materials",
        node_type: NodeType::Discovery,
        domains: &["Materials", "Biology", "Environment", "Chip", "Chemistry"],
        confidence: 0.90,
        source_bts: &[85, 93, 101, 104, 118],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "multiscale", "network"],
        validates: &["DISC-TOPO-04", "DISC-BIO-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Constants for counts
// ═══════════════════════════════════════════════════════════════

const ALL_RECENT_CLUSTERS: &[&[RecentEntry]] = &[
    PHOTONIC_DISCOVERIES,
    TOPO_CHIP_DISCOVERIES,
    FUSION_DISCOVERIES,
    BIO_DISCOVERIES,
    RECENT_BRIDGES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &RecentEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[RecentEntry]) -> (usize, usize) {
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

/// Add photonic-energy discovery nodes (BT-89). Returns (nodes, edges).
pub fn populate_photonic_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, PHOTONIC_DISCOVERIES)
}

/// Add topological chip discovery nodes (BT-90~93). Returns (nodes, edges).
pub fn populate_topo_chip_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TOPO_CHIP_DISCOVERIES)
}

/// Add fusion alien-level discovery nodes (BT-97~102). Returns (nodes, edges).
pub fn populate_fusion_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, FUSION_DISCOVERIES)
}

/// Add photosynthesis / CO₂ biology discovery nodes (BT-101,103~104). Returns (nodes, edges).
pub fn populate_bio_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, BIO_DISCOVERIES)
}

/// Add cross-cluster bridge discoveries connecting BT-89~104 to BT-105+ clusters.
pub fn populate_recent_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, RECENT_BRIDGES)
}

/// Cross-link recent discovery nodes that share domains (Merges edges, bidirectional).
/// Returns the number of cross-domain edges added.
pub fn connect_recent_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let recent_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-PHOT-")
                || n.id.starts_with("HYP-PHOT-")
                || n.id.starts_with("DISC-TOPO-")
                || n.id.starts_with("HYP-TOPO-")
                || n.id.starts_with("DISC-FUS-")
                || n.id.starts_with("HYP-FUS-")
                || n.id.starts_with("DISC-BIO-")
                || n.id.starts_with("HYP-BIO-")
                || n.id.starts_with("DISC-RBRIDGE-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..recent_nodes.len() {
        for j in (i + 1)..recent_nodes.len() {
            let (ref id_a, ref dom_a) = recent_nodes[i];
            let (ref id_b, ref dom_b) = recent_nodes[j];

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

/// Also cross-link recent nodes with existing DISC-/HYP- nodes from discovery_nodes.
/// Requires >=2 shared domains. Returns number of bridge edges added.
pub fn connect_recent_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let recent_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-PHOT-")
                || n.id.starts_with("HYP-PHOT-")
                || n.id.starts_with("DISC-TOPO-")
                || n.id.starts_with("HYP-TOPO-")
                || n.id.starts_with("DISC-FUS-")
                || n.id.starts_with("HYP-FUS-")
                || n.id.starts_with("DISC-BIO-")
                || n.id.starts_with("HYP-BIO-")
                || n.id.starts_with("DISC-RBRIDGE-")
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
                && !n.id.starts_with("DISC-PHOT-")
                && !n.id.starts_with("DISC-TOPO-")
                && !n.id.starts_with("DISC-FUS-")
                && !n.id.starts_with("DISC-BIO-")
                && !n.id.starts_with("DISC-RBRIDGE-")
                && !n.id.starts_with("HYP-PHOT-")
                && !n.id.starts_with("HYP-TOPO-")
                && !n.id.starts_with("HYP-FUS-")
                && !n.id.starts_with("HYP-BIO-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref r_id, ref r_dom) in &recent_ids {
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

/// Populate all BT-89~104 discovery nodes and cross-domain edges in one call.
/// Call after populate_bt_graph(), populate_expanded_graph(), and populate_all_discoveries().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_recent_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_photonic_discoveries(graph);
    let (n2, _) = populate_topo_chip_discoveries(graph);
    let (n3, _) = populate_fusion_discoveries(graph);
    let (n4, _) = populate_bio_discoveries(graph);
    let (n5, _) = populate_recent_bridges(graph);
    let _cross = connect_recent_cross_domain(graph);
    let _bridge = connect_recent_to_existing(graph);

    let total_nodes = n1 + n2 + n3 + n4 + n5;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total recent entry count across all clusters.
pub fn recent_entry_count() -> usize {
    ALL_RECENT_CLUSTERS.iter().map(|c| c.len()).sum()
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::discovery_nodes::populate_all_discoveries;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        g
    }

    fn full_graph() -> DiscoveryGraph {
        let mut g = base_graph();
        populate_all_discoveries(&mut g);
        g
    }

    // ── Cluster size tests ──

    #[test]
    fn test_photonic_count() {
        assert_eq!(PHOTONIC_DISCOVERIES.len(), 3, "2 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_topo_chip_count() {
        assert_eq!(TOPO_CHIP_DISCOVERIES.len(), 5, "4 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_fusion_count() {
        assert_eq!(FUSION_DISCOVERIES.len(), 6, "5 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_bio_count() {
        assert_eq!(BIO_DISCOVERIES.len(), 4, "3 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_recent_bridge_count() {
        assert_eq!(RECENT_BRIDGES.len(), 8, "8 cross-cluster bridges");
    }

    #[test]
    fn test_total_recent_entry_count() {
        // 3 + 5 + 6 + 4 + 8 = 26
        assert_eq!(recent_entry_count(), 26);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_photonic() {
        let mut g = base_graph();
        let (n, e) = populate_photonic_discoveries(&mut g);
        assert_eq!(n, 3);
        assert!(e >= 6, "at least 2 BT derives + 2 Uses per node on avg");
    }

    #[test]
    fn test_populate_topo_chip() {
        let mut g = base_graph();
        let (n, e) = populate_topo_chip_discoveries(&mut g);
        assert_eq!(n, 5);
        assert!(e >= 10, "each node has BT derives + Uses edges");
    }

    #[test]
    fn test_populate_fusion() {
        let mut g = base_graph();
        let (n, e) = populate_fusion_discoveries(&mut g);
        assert_eq!(n, 6);
        assert!(e >= 12);
    }

    #[test]
    fn test_populate_bio() {
        let mut g = base_graph();
        let (n, e) = populate_bio_discoveries(&mut g);
        assert_eq!(n, 4);
        assert!(e >= 8);
    }

    #[test]
    fn test_populate_recent_bridges() {
        let mut g = base_graph();
        let (n, e) = populate_recent_bridges(&mut g);
        assert_eq!(n, 8);
        assert!(e >= 16, "bridges span multiple BTs and constants");
    }

    // ── Integration test ──

    #[test]
    fn test_all_recent_discoveries_integrated() {
        let mut g = full_graph();
        let nodes_before = g.nodes.len();
        let edges_before = g.edges.len();

        let (n_added, e_added) = populate_all_recent_discoveries(&mut g);

        assert_eq!(n_added, 26, "26 total recent nodes");
        assert!(e_added >= 80, "derives + uses + cross-domain + bridge edges: got {}", e_added);
        assert_eq!(g.nodes.len(), nodes_before + 26);
        assert_eq!(g.edges.len(), edges_before + e_added);
    }

    // ── Cross-domain edge tests ──

    #[test]
    fn test_recent_cross_domain_edges() {
        let mut g = base_graph();
        populate_photonic_discoveries(&mut g);
        populate_topo_chip_discoveries(&mut g);
        populate_fusion_discoveries(&mut g);
        populate_bio_discoveries(&mut g);
        populate_recent_bridges(&mut g);

        let cross = connect_recent_cross_domain(&mut g);
        // Many nodes share domains (Chip, Math, Physics, Energy, Chemistry, etc.)
        assert!(cross >= 30, "expect 30+ cross-domain merges, got {}", cross);
    }

    #[test]
    fn test_recent_to_existing_bridge_edges() {
        let mut g = full_graph();
        populate_photonic_discoveries(&mut g);
        populate_topo_chip_discoveries(&mut g);
        populate_fusion_discoveries(&mut g);
        populate_bio_discoveries(&mut g);
        populate_recent_bridges(&mut g);

        let bridges = connect_recent_to_existing(&mut g);
        // Recent clusters share domains with existing BT-105+ discoveries
        assert!(bridges >= 5, "expect 5+ bridges to existing nodes, got {}", bridges);
    }

    // ── Specific edge verification ──

    #[test]
    fn test_bt_derives_edges_present() {
        let mut g = base_graph();
        populate_fusion_discoveries(&mut g);

        // BT-99 should derive DISC-FUS-03 (tokamak q=1)
        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-99" && e.to == "DISC-FUS-03" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-99 --Derives--> DISC-FUS-03 must exist");
    }

    #[test]
    fn test_uses_constant_edges_present() {
        let mut g = base_graph();
        populate_topo_chip_discoveries(&mut g);

        // DISC-TOPO-01 should use C-sigma² (144 SMs)
        let has_edge = g.edges.iter().any(|e| {
            e.from == "DISC-TOPO-01" && e.to == "C-sigma²" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(has_edge, "DISC-TOPO-01 --Uses--> C-sigma² must exist");
    }

    #[test]
    fn test_carbon_mega_bridge_spans_five_domains() {
        let mut g = base_graph();
        populate_recent_bridges(&mut g);

        let bridge = g.nodes.iter().find(|n| n.id == "DISC-RBRIDGE-08").unwrap();
        let domains: Vec<&str> = bridge.domain.split(", ").collect();
        assert_eq!(domains.len(), 5, "Carbon mega-bridge spans 5 domains");
        assert!(domains.contains(&"Materials"));
        assert!(domains.contains(&"Biology"));
        assert!(domains.contains(&"Environment"));
        assert!(domains.contains(&"Chip"));
        assert!(domains.contains(&"Chemistry"));
    }

    // ── Node type distribution ──

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_photonic_discoveries(&mut g);
        populate_topo_chip_discoveries(&mut g);
        populate_fusion_discoveries(&mut g);
        populate_bio_discoveries(&mut g);
        populate_recent_bridges(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();

        assert_eq!(discoveries, 22, "22 discovery nodes (DISC-*)");
        assert_eq!(hypotheses, 4, "4 hypothesis nodes (HYP-*)");
        assert_eq!(discoveries + hypotheses, 26);
    }

    // ── Confidence range validation ──

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_RECENT_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }

    // ── Validates edge tests ──

    #[test]
    fn test_validates_edges_present() {
        let mut g = base_graph();
        populate_photonic_discoveries(&mut g);

        // DISC-PHOT-02 validates DISC-PHOT-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-PHOT-02"
                && e.to == "DISC-PHOT-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-PHOT-02 should validate DISC-PHOT-01");
    }

    #[test]
    fn test_validates_bio_stoichiometry() {
        let mut g = base_graph();
        populate_bio_discoveries(&mut g);

        // DISC-BIO-02 validates DISC-BIO-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BIO-02"
                && e.to == "DISC-BIO-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-BIO-02 should validate DISC-BIO-01");
    }

    #[test]
    fn test_validates_mega_bridge() {
        let mut g = full_graph();
        populate_all_recent_discoveries(&mut g);

        // DISC-RBRIDGE-08 validates DISC-TOPO-04 and DISC-BIO-01
        let val_topo = g.edges.iter().any(|e| {
            e.from == "DISC-RBRIDGE-08"
                && e.to == "DISC-TOPO-04"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(val_topo, "DISC-RBRIDGE-08 should validate DISC-TOPO-04");

        let val_bio = g.edges.iter().any(|e| {
            e.from == "DISC-RBRIDGE-08"
                && e.to == "DISC-BIO-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(val_bio, "DISC-RBRIDGE-08 should validate DISC-BIO-01");
    }

    // ── Hub detection with recent nodes ──

    #[test]
    fn test_carbon_is_hub() {
        let mut g = base_graph();
        populate_all_discoveries(&mut g);
        let (_, _) = populate_all_recent_discoveries(&mut g);

        let hubs = g.hubs(5);
        // C-n (n=6) should be a major hub — used by nearly every discovery
        let c_n_hub = hubs.iter().find(|h| h.node_id == "C-n");
        assert!(c_n_hub.is_some(), "C-n should be a hub with degree >= 5");
    }
}
