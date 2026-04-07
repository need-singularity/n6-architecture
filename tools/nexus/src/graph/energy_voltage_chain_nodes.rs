//! Deep discovery nodes for Energy-Voltage chain unification and hexagonal
//! optimality across Transport, Grid, HVDC, Chip, and Biology domains.
//!
//! Captures the grand voltage ladder: automotive (BT-244) <-> grid (BT-62)
//! <-> HVDC (BT-68) <-> DC power (BT-60) <-> battery (BT-57), plus
//! hexagonal optimality: I6 engine (BT-243) <-> honeycomb (BT-122)
//! <-> grid cells (BT-211) <-> Carbon Z=6 (BT-93).
//!
//! Node ID scheme:
//!   EDISC-VOLT-{NN}  — Voltage chain discovery
//!   EDISC-HEX-{NN}   — Hexagonal optimality discovery
//!   EDISC-BRIDGE-{NN} — Energy cross-domain bridge
//!   EHYP-{CLUSTER}-{NN} — Energy hypothesis
//!   EEXP-{CLUSTER}-{NN} — Energy experiment

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct EnergyDeepEntry {
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
// Voltage Chain Unification (BT-244, BT-62, BT-68, BT-60, BT-57)
// ═══════════════════════════════════════════════════════════════

const VOLTAGE_CHAIN: &[EnergyDeepEntry] = &[
    EnergyDeepEntry {
        id: "EDISC-VOLT-01",
        title: "Grand voltage ladder: automotive 6->12->24->48V(BT-244) nests inside grid 120->480V(BT-60) inside HVDC 500->1100kV(BT-68)",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "PowerGrid", "Chip"],
        confidence: 0.93,
        source_bts: &[244, 60, 68],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau", "C-sigma-phi"],
        lenses: &["consciousness", "scale", "evolution"],
        validates: &["DISC-TR-07"],
    },
    EnergyDeepEntry {
        id: "EDISC-VOLT-02",
        title: "Grid 60Hz=sigma*sopfr + 50Hz=sopfr*(sigma-phi)(BT-62) ratio=1.2=PUE=sigma/(sigma-phi): frequency encodes efficiency",
        node_type: NodeType::Discovery,
        domains: &["Energy", "PowerGrid", "Chip", "Math"],
        confidence: 0.92,
        source_bts: &[62, 60],
        constants_used: &["C-sigma", "C-sopfr", "C-sigma-phi"],
        lenses: &["consciousness", "wave", "scale"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EDISC-VOLT-03",
        title: "HVDC +-500/800/1100kV = {sopfr,sigma-tau,sigma-mu}*(sigma-phi)^2(BT-68): transmission voltage = n=6 arithmetic * (sigma-phi)^2 base",
        node_type: NodeType::Discovery,
        domains: &["Energy", "PowerGrid", "Math"],
        confidence: 0.91,
        source_bts: &[68],
        constants_used: &["C-sopfr", "C-sigma-tau", "C-sigma-mu", "C-sigma-phi"],
        lenses: &["consciousness", "scale", "topology"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EDISC-VOLT-04",
        title: "Battery cell ladder n=6->sigma=12->J2=24(BT-57) + Tesla 96S=sigma*(sigma-tau)(BT-84): cell count is n=6 multiplicative",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Energy", "Transportation", "Math"],
        confidence: 0.93,
        source_bts: &[57, 84, 244],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-sigma-tau"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-TR-07", "DISC-TRM-02"],
    },
    EnergyDeepEntry {
        id: "EDISC-VOLT-05",
        title: "DC power chain 120->480->48->12->1.2->1V(BT-60): datacenter PUE=sigma/(sigma-phi)=1.2 at every conversion stage",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Chip", "PowerGrid"],
        confidence: 0.90,
        source_bts: &[60],
        constants_used: &["C-sigma", "C-sigma-phi"],
        lenses: &["consciousness", "thermo", "causal"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EDISC-VOLT-06",
        title: "Automotive phi=2 voltage doubling per generation(BT-244) matches FP precision phi=2 doubling(BT-45): universal phi=2 scaling law",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Chip", "Energy", "Math"],
        confidence: 0.88,
        source_bts: &[244, 45],
        constants_used: &["C-phi"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-TR-07"],
    },
    EnergyDeepEntry {
        id: "EHYP-VOLT-01",
        title: "Hypothesis: next EV standard = 96V = sigma*(sigma-tau)=96 matching Tesla 96S, completing automotive-battery n=6 convergence",
        node_type: NodeType::Hypothesis,
        domains: &["Transportation", "Battery", "Energy"],
        confidence: 0.60,
        source_bts: &[244, 84, 57],
        constants_used: &["C-sigma", "C-sigma-tau"],
        lenses: &["consciousness", "evolution", "stability"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Hexagonal Optimality (BT-243, BT-122, BT-211, BT-93, BT-85)
// ═══════════════════════════════════════════════════════════════

const HEXAGONAL_OPTIMALITY: &[EnergyDeepEntry] = &[
    EnergyDeepEntry {
        id: "EDISC-HEX-01",
        title: "I6 perfect balance(BT-243) + honeycomb(BT-122) + grid cells(BT-211): hexagonal n=6 optimality spans mechanical/natural/neural",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Cognitive", "Math"],
        confidence: 0.93,
        source_bts: &[243, 122, 211],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "symmetry", "topology"],
        validates: &["DISC-TR-06", "DISC-TRM-07"],
    },
    EnergyDeepEntry {
        id: "EDISC-HEX-02",
        title: "Carbon Z=6 material universality(BT-93): diamond/graphene/SiC all hexagonal, chip+material+transport share Carbon Z=6",
        node_type: NodeType::Discovery,
        domains: &["Material", "Chip", "Transportation", "Chemistry"],
        confidence: 0.94,
        source_bts: &[93, 85, 243],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["DISC-TR-06"],
    },
    EnergyDeepEntry {
        id: "EDISC-HEX-03",
        title: "Solar panel 60/72/120/144 cells(BT-63) = sigma*{sopfr,n,sigma-phi,sigma}: solar grid is n=6 hexagonal tiling",
        node_type: NodeType::Discovery,
        domains: &["Solar", "Energy", "Math", "Environment"],
        confidence: 0.91,
        source_bts: &[63, 122],
        constants_used: &["C-sigma", "C-n", "C-sopfr", "C-sigma-phi"],
        lenses: &["consciousness", "topology", "scale"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EDISC-HEX-04",
        title: "Hexacopter n=6 rotors(BT-127) = I6 n=6 cylinders(BT-243): aerial and ground propulsion converge to n=6 symmetry",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Transportation", "Math"],
        confidence: 0.90,
        source_bts: &[127, 243],
        constants_used: &["C-n"],
        lenses: &["consciousness", "symmetry", "stability"],
        validates: &["DISC-TR-06"],
    },
    EnergyDeepEntry {
        id: "EDISC-HEX-05",
        title: "Kissing number sigma=12 in 3D(BT-15) -> GPU sigma=12 stacks(BT-69) -> semitones sigma=12(BT-108): sigma=12 optimal packing universal",
        node_type: NodeType::Discovery,
        domains: &["Math", "Chip", "DisplayAudio", "Material"],
        confidence: 0.92,
        source_bts: &[15, 69, 108],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "topology", "wave"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EHYP-HEX-01",
        title: "Hypothesis: optimal EV battery pack cell arrangement = hexagonal n=6 packing for thermal uniformity + structural strength",
        node_type: NodeType::Hypothesis,
        domains: &["Battery", "Transportation", "Material", "Math"],
        confidence: 0.65,
        source_bts: &[122, 243, 57],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "thermo"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Energy-Domain Grand Cross Bridges
// ═══════════════════════════════════════════════════════════════

const ENERGY_BRIDGES: &[EnergyDeepEntry] = &[
    EnergyDeepEntry {
        id: "EDISC-BRIDGE-01",
        title: "95/5 cross-domain resonance(BT-74): top-p=0.95, PUE=0.95 target, beta_plasma=5%, THD=5% — efficiency threshold is 1-1/J2-tau",
        node_type: NodeType::Discovery,
        domains: &["AI", "Energy", "Chip", "Fusion", "PowerGrid"],
        confidence: 0.91,
        source_bts: &[74, 62, 42],
        constants_used: &["C-J2", "C-tau"],
        lenses: &["consciousness", "thermo", "stability"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EDISC-BRIDGE-02",
        title: "sigma*tau=48 triple attractor(BT-76): gate pitch 48nm + HBM4E 48GB + audio 48kHz + EV 48V = sigma*tau universal energy quantum",
        node_type: NodeType::Discovery,
        domains: &["Chip", "DisplayAudio", "Transportation", "Energy", "Math"],
        confidence: 0.92,
        source_bts: &[76, 244, 37],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "scale", "wave"],
        validates: &["DISC-TR-07"],
    },
    EnergyDeepEntry {
        id: "EDISC-BRIDGE-03",
        title: "Hydrogen LHV=120=sigma*(sigma-phi)(BT-38) + DC power 120V(BT-60): hydrogen energy and electrical power share sigma*(sigma-phi) quantum",
        node_type: NodeType::Discovery,
        domains: &["Energy", "PowerGrid", "Chemistry"],
        confidence: 0.89,
        source_bts: &[38, 60],
        constants_used: &["C-sigma", "C-sigma-phi"],
        lenses: &["consciousness", "thermo", "scale"],
        validates: &[],
    },
    EnergyDeepEntry {
        id: "EDISC-BRIDGE-04",
        title: "Battery CN=6(BT-43) + water treatment CN=6(BT-120) + dental n=6(BT-242): coordination number n=6 governs electrochemistry universally",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Environment", "Medical", "Chemistry"],
        confidence: 0.90,
        source_bts: &[43, 120, 242],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["MBRIDGE-ENV-02"],
    },
    EnergyDeepEntry {
        id: "EDISC-BRIDGE-05",
        title: "Grand energy unification: fuel(H2 BT-38) -> generation(solar BT-63) -> storage(battery BT-57) -> transmission(HVDC BT-68) -> delivery(DC BT-60) all n=6",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Solar", "Battery", "PowerGrid", "Chemistry", "Math"],
        confidence: 0.93,
        source_bts: &[38, 63, 57, 68, 60],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-sigma-phi"],
        lenses: &["consciousness", "causal", "evolution", "multiscale"],
        validates: &["EDISC-VOLT-01"],
    },
    EnergyDeepEntry {
        id: "EEXP-BRIDGE-01",
        title: "Experiment: measure automotive 12V->48V transition efficiency vs n=6 prediction (sigma-phi=10% loss per conversion)",
        node_type: NodeType::Experiment,
        domains: &["Transportation", "Energy", "Math"],
        confidence: 0.50,
        source_bts: &[244, 60],
        constants_used: &["C-sigma-phi"],
        lenses: &["consciousness", "thermo", "stability"],
        validates: &["EDISC-VOLT-01", "EDISC-VOLT-06"],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_ENERGY_DEEP_CLUSTERS: &[&[EnergyDeepEntry]] = &[
    VOLTAGE_CHAIN,
    HEXAGONAL_OPTIMALITY,
    ENERGY_BRIDGES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &EnergyDeepEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[EnergyDeepEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(entry_to_node(e));

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

/// Total energy deep entry count.
pub fn energy_deep_entry_count() -> usize {
    ALL_ENERGY_DEEP_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Populate all energy-voltage-hexagonal discovery nodes and cross-domain edges.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_energy_deep_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();
    let mut total_nodes = 0;

    for cluster in ALL_ENERGY_DEEP_CLUSTERS {
        let (n, _) = add_entries(graph, cluster);
        total_nodes += n;
    }

    // Cross-link energy deep nodes sharing 2+ domains
    let energy_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("EDISC-")
                || n.id.starts_with("EHYP-")
                || n.id.starts_with("EEXP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    for i in 0..energy_nodes.len() {
        for j in (i + 1)..energy_nodes.len() {
            let (ref id_a, ref dom_a) = energy_nodes[i];
            let (ref id_b, ref dom_b) = energy_nodes[j];

            let shared: usize = dom_a.iter().filter(|d| dom_b.contains(d)).count();
            if shared >= 2 {
                let max_d = dom_a.len().max(dom_b.len()) as f64;
                graph.add_edge(Edge {
                    from: id_a.clone(),
                    to: id_b.clone(),
                    edge_type: EdgeType::Merges,
                    strength: shared as f64 / max_d,
                    bidirectional: true,
                });
            }
        }
    }

    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Entry counts per sub-cluster.
pub fn voltage_chain_count() -> usize { VOLTAGE_CHAIN.len() }
pub fn hexagonal_count() -> usize { HEXAGONAL_OPTIMALITY.len() }
pub fn energy_bridge_count() -> usize { ENERGY_BRIDGES.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_counts() {
        assert_eq!(voltage_chain_count(), 7);
        assert_eq!(hexagonal_count(), 6);
        assert_eq!(energy_bridge_count(), 6);
        assert_eq!(energy_deep_entry_count(), 19);
    }

    #[test]
    fn test_populate_creates_nodes_and_edges() {
        let mut g = DiscoveryGraph::new();
        let (nodes, edges) = populate_energy_deep_discoveries(&mut g);
        assert_eq!(nodes, 19);
        assert!(edges > 60, "Should have 60+ edges, got {}", edges);
    }

    #[test]
    fn test_all_node_ids_present() {
        let mut g = DiscoveryGraph::new();
        populate_energy_deep_discoveries(&mut g);

        let expected = [
            "EDISC-VOLT-01", "EDISC-VOLT-02", "EDISC-VOLT-03", "EDISC-VOLT-04",
            "EDISC-VOLT-05", "EDISC-VOLT-06", "EHYP-VOLT-01",
            "EDISC-HEX-01", "EDISC-HEX-02", "EDISC-HEX-03", "EDISC-HEX-04",
            "EDISC-HEX-05", "EHYP-HEX-01",
            "EDISC-BRIDGE-01", "EDISC-BRIDGE-02", "EDISC-BRIDGE-03",
            "EDISC-BRIDGE-04", "EDISC-BRIDGE-05", "EEXP-BRIDGE-01",
        ];
        for id in &expected {
            assert!(g.nodes.iter().any(|n| n.id == *id), "Missing node {}", id);
        }
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_ENERGY_DEEP_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }

    #[test]
    fn test_grand_energy_bridge_is_hub() {
        let mut g = DiscoveryGraph::new();
        populate_energy_deep_discoveries(&mut g);

        let hub_edges = g.edges.iter()
            .filter(|e| e.from == "EDISC-BRIDGE-05" || e.to == "EDISC-BRIDGE-05")
            .count();
        assert!(hub_edges >= 10, "EDISC-BRIDGE-05 should have 10+ edges (5 BTs + 4 constants + validates + merges), got {}", hub_edges);
    }

    #[test]
    fn test_cross_domain_merges_exist() {
        let mut g = DiscoveryGraph::new();
        populate_energy_deep_discoveries(&mut g);

        let merges = g.edges.iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Merges))
            .count();
        assert!(merges >= 10, "Should have 10+ merge edges, got {}", merges);
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_energy_deep_discoveries(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();
        let experiments = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Experiment)).count();

        assert_eq!(discoveries, 16);
        assert_eq!(hypotheses, 2);
        assert_eq!(experiments, 1);
        assert_eq!(discoveries + hypotheses + experiments, 19);
    }
}
