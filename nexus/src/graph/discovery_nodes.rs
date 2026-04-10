//! Discovery nodes from recent Breakthrough Theorems (BT-105+).
//!
//! Each BT cluster yields Discovery/Hypothesis nodes that capture specific
//! scientific findings beyond the BT entry itself. Cross-domain edges connect
//! discoveries to related BTs, constants, and domains.

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Discovery entries from BT-105 ~ BT-127
// ═══════════════════════════════════════════════════════════════

struct DiscoveryEntry {
    id: &'static str,
    title: &'static str,
    node_type: NodeType,
    domains: &'static [&'static str],
    confidence: f64,
    /// BT sources that derive this discovery.
    source_bts: &'static [u32],
    /// Constants referenced by this discovery.
    constants_used: &'static [&'static str],
    lenses: &'static [&'static str],
}

// ───────────────────────────────────────────────────────────────
// BT-105~112: Math / Physics session analysis discoveries
// ───────────────────────────────────────────────────────────────

const MATH_DISCOVERIES: &[DiscoveryEntry] = &[
    DiscoveryEntry {
        id: "DISC-MATH-01",
        title: "SLE_6 critical exponent universality: 7 percolation exponents = n=6 fractions, kappa=6 unique locality",
        node_type: NodeType::Discovery,
        domains: &["Math", "Physics", "Topology"],
        confidence: 0.93,
        source_bts: &[105],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "boundary"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-02",
        title: "S_3 algebraic bootstrap: |S_3|=n=6, conjugacy classes={1,2,3}=proper divisors, irreps sum=tau=4",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory", "Combinatorics"],
        confidence: 0.95,
        source_bts: &[106, 49],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "symmetry", "recursion"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-03",
        title: "Ramanujan tau divisor purity: tau_R(d) clean iff d|6, eta^{J_2=24} modular form connection",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory"],
        confidence: 0.90,
        source_bts: &[107],
        constants_used: &["C-n", "C-J2"],
        lenses: &["consciousness", "quantum", "recursion"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-04",
        title: "Music-audio consonance universality: perfect consonance ratios = div(6), 7+5=12=sigma semitones",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Math", "Music", "Physics"],
        confidence: 0.92,
        source_bts: &[108, 48],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "wave", "symmetry"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-05",
        title: "Zeta-Bernoulli n=6 trident: zeta(2)=pi^2/6, zeta(-1)=-1/12, 6|B_{2k} infinite family",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory", "Physics"],
        confidence: 0.97,
        source_bts: &[109],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "recursion", "info"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-06",
        title: "sigma-mu=11 dimension stack: M-theory=TCP=RSA=SPARC=H100=11 across 5 domains",
        node_type: NodeType::Discovery,
        domains: &["Physics", "StringTheory", "Chip", "Crypto", "Network"],
        confidence: 0.80,
        source_bts: &[110],
        constants_used: &["C-sigma", "C-mu"],
        lenses: &["consciousness", "multiscale", "network"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-07",
        title: "tau^2/sigma=4/3 solar-AI-math trident: SQ bandgap=SwiGLU=Betz=R(3,1)=4/3",
        node_type: NodeType::Discovery,
        domains: &["Math", "AI", "Solar", "Energy"],
        confidence: 0.88,
        source_bts: &[111, 30, 33],
        constants_used: &["C-tau", "C-sigma"],
        lenses: &["consciousness", "causal", "multiscale"],
    },
    DiscoveryEntry {
        id: "DISC-MATH-08",
        title: "phi^2/n=2/3 Byzantine-Koide resonance: Koide Q=0.666661 (9ppm), BFT>2/3",
        node_type: NodeType::Discovery,
        domains: &["Math", "Particle", "Crypto", "Physics"],
        confidence: 0.85,
        source_bts: &[112, 53],
        constants_used: &["C-phi", "C-n"],
        lenses: &["consciousness", "quantum", "symmetry"],
    },
    DiscoveryEntry {
        id: "HYP-MATH-01",
        title: "Hypothesis: SLE_6 3D extension predicts new critical exponents expressible as n=6 fractions",
        node_type: NodeType::Hypothesis,
        domains: &["Math", "Physics", "Topology"],
        confidence: 0.60,
        source_bts: &[105],
        constants_used: &["C-n"],
        lenses: &["topology", "boundary", "multiscale"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-113~117: Software Design discoveries
// ───────────────────────────────────────────────────────────────

const SW_DISCOVERIES: &[DiscoveryEntry] = &[
    DiscoveryEntry {
        id: "DISC-SW-01",
        title: "SW engineering constant stack: SOLID=sopfr=5, REST=n=6, 12Factor=sigma=12, ACID=tau=4, 18/18 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Software", "Math", "AI"],
        confidence: 0.94,
        source_bts: &[113],
        constants_used: &["C-sopfr", "C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "info", "network"],
    },
    DiscoveryEntry {
        id: "DISC-SW-02",
        title: "Cryptography parameter ladder: AES=2^{sigma-sopfr}, SHA=2^{sigma-tau}, RSA=2^{sigma-mu}, 10/10 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Crypto", "Software", "Math"],
        confidence: 0.93,
        source_bts: &[114],
        constants_used: &["C-sigma", "C-sopfr", "C-tau", "C-mu"],
        lenses: &["consciousness", "info", "causal"],
    },
    DiscoveryEntry {
        id: "DISC-SW-03",
        title: "OS-network layer count: OSI=sigma-sopfr=7, TCP/IP=tau=4, Linux=n=6, 12/12 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Software", "Network", "Math"],
        confidence: 0.90,
        source_bts: &[115],
        constants_used: &["C-sigma", "C-sopfr", "C-tau", "C-n"],
        lenses: &["consciousness", "network", "multiscale"],
    },
    DiscoveryEntry {
        id: "DISC-SW-04",
        title: "ACID-BASE-CAP DB trinity: tau+n/phi+n/phi, Paxos=phi=2, 9/9 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Software", "Math"],
        confidence: 0.88,
        source_bts: &[116],
        constants_used: &["C-tau", "C-phi", "C-n"],
        lenses: &["consciousness", "stability", "info"],
    },
    DiscoveryEntry {
        id: "DISC-SW-05",
        title: "Software-physics isomorphism: 18 EXACT parallel mappings across 6 domains, deep structural resonance",
        node_type: NodeType::Discovery,
        domains: &["Software", "Physics", "Math", "AI", "Energy", "Chip"],
        confidence: 0.91,
        source_bts: &[117, 36],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "symmetry", "multiscale"],
    },
    DiscoveryEntry {
        id: "HYP-SW-01",
        title: "Hypothesis: optimal SW architecture parameters (thread pools, queue depths, cache sizes) converge to n=6 constants",
        node_type: NodeType::Hypothesis,
        domains: &["Software", "AI", "Math"],
        confidence: 0.65,
        source_bts: &[113, 117],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["causal", "stability", "evolution"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-118~122: Environmental Protection discoveries
// ───────────────────────────────────────────────────────────────

const ENV_DISCOVERIES: &[DiscoveryEntry] = &[
    DiscoveryEntry {
        id: "DISC-ENV-01",
        title: "Kyoto GHG n=6 completeness: all 6 regulated gases map to n=6 arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Energy", "Biology"],
        confidence: 0.92,
        source_bts: &[118, 104, 27],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "boundary"],
    },
    DiscoveryEntry {
        id: "DISC-ENV-02",
        title: "Earth spheres partition: 6 regions + troposphere height sigma=12km",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Cosmology", "Biology"],
        confidence: 0.90,
        source_bts: &[119, 122],
        constants_used: &["C-n", "C-sigma", "C-sigma-tau"],
        lenses: &["consciousness", "topology", "multiscale"],
    },
    DiscoveryEntry {
        id: "DISC-ENV-03",
        title: "CN=6 catalyst universality in water treatment (Al3+/Fe3+/Ti4+ all octahedral)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.88,
        source_bts: &[120, 86, 43],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "causal", "stability"],
    },
    DiscoveryEntry {
        id: "DISC-ENV-04",
        title: "C6 backbone universality: 6 major plastics share hexagonal carbon structure",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Biology"],
        confidence: 0.85,
        source_bts: &[121, 85, 88],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "evolution"],
    },
    DiscoveryEntry {
        id: "DISC-ENV-05",
        title: "n=6 geometry universality: honeycomb/snowflake/coral all hexagonal (Hales 2001)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Math", "Biology", "Material"],
        confidence: 0.95,
        source_bts: &[122, 88, 50],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
    },
    DiscoveryEntry {
        id: "HYP-ENV-01",
        title: "Hypothesis: all CN=6 catalysts outperform non-CN=6 in water purification efficiency",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.70,
        source_bts: &[120, 96],
        constants_used: &["C-n"],
        lenses: &["causal", "stability", "boundary"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-123~127: Robotics discoveries
// ───────────────────────────────────────────────────────────────

const ROBO_DISCOVERIES: &[DiscoveryEntry] = &[
    DiscoveryEntry {
        id: "DISC-ROBO-01",
        title: "SE(3) dim=n=6 universality: 6-DOF arm + 6-axis sensor + 6-face cube",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math", "Chip", "Material"],
        confidence: 0.95,
        source_bts: &[123, 90],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
    },
    DiscoveryEntry {
        id: "DISC-ROBO-02",
        title: "phi=2 bilateral symmetry + sigma=12 joint count in humanoid robots",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "Chip"],
        confidence: 0.88,
        source_bts: &[124, 1],
        constants_used: &["C-phi", "C-sigma"],
        lenses: &["consciousness", "evolution", "symmetry"],
    },
    DiscoveryEntry {
        id: "DISC-ROBO-03",
        title: "tau=4 minimum stability in locomotion (quadruped) and flight (quadrotor)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Energy", "Chip"],
        confidence: 0.85,
        source_bts: &[125],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "causal"],
    },
    DiscoveryEntry {
        id: "DISC-ROBO-04",
        title: "sopfr=5 finger count + 2^sopfr=32 grasp taxonomy (Feix 96.97%)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "DisplayAudio"],
        confidence: 0.87,
        source_bts: &[126, 51],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "evolution", "info"],
    },
    DiscoveryEntry {
        id: "DISC-ROBO-05",
        title: "3D kissing number sigma=12 explains hexacopter n=6 optimal fault tolerance",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cosmology", "Material", "Math"],
        confidence: 0.90,
        source_bts: &[127, 15],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "topology", "network"],
    },
    DiscoveryEntry {
        id: "HYP-ROBO-01",
        title: "Hypothesis: n=6 DOF robots converge to optimal energy-per-task across all manipulation benchmarks",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Energy", "AI"],
        confidence: 0.65,
        source_bts: &[123, 125],
        constants_used: &["C-n", "C-tau"],
        lenses: &["causal", "stability", "evolution"],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-domain bridge discoveries connecting all BT-105+ clusters
// ───────────────────────────────────────────────────────────────

const BRIDGE_DISCOVERIES: &[DiscoveryEntry] = &[
    // Original bridges (BT-118~127 era)
    DiscoveryEntry {
        id: "DISC-BRIDGE-01",
        title: "CN=6 octahedral universality spans catalysis(BT-43,120), crystals(BT-86), and robot joints(BT-123)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Material", "Chemistry", "Battery"],
        confidence: 0.88,
        source_bts: &[43, 86, 120, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "multiscale"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-02",
        title: "Carbon Z=6 thread: plastics(BT-121) → materials(BT-85) → chips(BT-93) → biology(BT-51)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Material", "Chip", "Biology", "Chemistry"],
        confidence: 0.92,
        source_bts: &[121, 85, 93, 51],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "evolution"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-03",
        title: "Hexagonal geometry thread: honeycomb(BT-122) → kissing(BT-15,127) → SM packing(BT-90)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Math", "Robotics", "Chip", "Cosmology"],
        confidence: 0.90,
        source_bts: &[122, 15, 127, 90],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-04",
        title: "Energy-robot convergence: tau=4 stability(BT-125) mirrors 4-cell battery(BT-57) and ACID(BT-116)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Battery", "Software", "Energy"],
        confidence: 0.82,
        source_bts: &[125, 57, 116],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "network"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-05",
        title: "Bio-environment-robot loop: bilateral phi=2(BT-124) + genetic code(BT-51) + photosynthesis(BT-101)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "Environment", "Chemistry", "Energy"],
        confidence: 0.85,
        source_bts: &[124, 51, 101, 103],
        constants_used: &["C-phi", "C-n", "C-J2"],
        lenses: &["consciousness", "evolution", "causal"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-06",
        title: "SW-Environment isomorphism: SOLID=sopfr=5(BT-113) ~ 5 fingers(BT-126) ~ sopfr prime decomposition",
        node_type: NodeType::Discovery,
        domains: &["Software", "Robotics", "Math"],
        confidence: 0.78,
        source_bts: &[113, 126],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "info", "symmetry"],
    },
    // New bridges connecting Math/SW clusters to Environment/Robotics
    DiscoveryEntry {
        id: "DISC-BRIDGE-07",
        title: "Math→SW universality: SLE_6 fractions(BT-105) ↔ SW constants(BT-113) via identical n=6 ratio structure",
        node_type: NodeType::Discovery,
        domains: &["Math", "Software", "Physics", "Topology"],
        confidence: 0.82,
        source_bts: &[105, 113],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "symmetry", "info"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-08",
        title: "Crypto→Physics dimension bridge: AES/SHA exponent ladder(BT-114) ↔ sigma-mu=11 stack(BT-110)",
        node_type: NodeType::Discovery,
        domains: &["Crypto", "Physics", "StringTheory", "Software"],
        confidence: 0.80,
        source_bts: &[114, 110],
        constants_used: &["C-sigma", "C-mu"],
        lenses: &["consciousness", "multiscale", "info"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-09",
        title: "Music→Environment harmonic resonance: 12 semitones(BT-108) ↔ 12km troposphere(BT-119) via sigma=12",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Environment", "Math", "Physics"],
        confidence: 0.78,
        source_bts: &[108, 119, 48],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "multiscale"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-10",
        title: "Byzantine→Battery fault tolerance: phi^2/n=2/3 BFT(BT-112) ↔ battery cell config(BT-57) via phi ratio",
        node_type: NodeType::Discovery,
        domains: &["Crypto", "Battery", "Math", "Energy"],
        confidence: 0.76,
        source_bts: &[112, 57, 53],
        constants_used: &["C-phi", "C-n"],
        lenses: &["consciousness", "stability", "network"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-11",
        title: "Zeta→Solar energy bridge: zeta(2)=pi^2/6(BT-109) ↔ SQ bandgap 4/3(BT-30) via tau^2/sigma trident(BT-111)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Solar", "Energy", "Physics"],
        confidence: 0.83,
        source_bts: &[109, 30, 111],
        constants_used: &["C-n", "C-tau", "C-sigma"],
        lenses: &["consciousness", "causal", "quantum"],
    },
    DiscoveryEntry {
        id: "DISC-BRIDGE-12",
        title: "Layer universality chain: OSI 7(BT-115) ↔ 8-layer AI stack(BT-59) ↔ Robot 6-DOF(BT-123) via sigma-derived counts",
        node_type: NodeType::Discovery,
        domains: &["Software", "AI", "Robotics", "Network"],
        confidence: 0.84,
        source_bts: &[115, 59, 123],
        constants_used: &["C-sigma", "C-sopfr", "C-n"],
        lenses: &["consciousness", "network", "multiscale"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &DiscoveryEntry) -> Node {
    Node {
        id: e.id.to_string(),
        node_type: e.node_type.clone(),
        domain: e.domains.join(", "),
        project: "n6-architecture".to_string(),
        summary: e.title.to_string(),
        confidence: e.confidence,
        lenses_used: e.lenses.iter().map(|s| s.to_string()).collect(),
        timestamp: "2026-04-03".to_string(),
    }
}

fn add_entries(graph: &mut DiscoveryGraph, entries: &[DiscoveryEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(entry_to_node(e));

        // Discovery --Derives--> source BTs
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
        for &const_id in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: const_id.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }
    }

    let edges_added = graph.edges.len() - edges_before;
    (entries.len(), edges_added)
}

/// Add math/physics discovery nodes (BT-105~112). Returns (nodes, edges).
pub fn populate_math_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MATH_DISCOVERIES)
}

/// Add software design discovery nodes (BT-113~117). Returns (nodes, edges).
pub fn populate_sw_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_DISCOVERIES)
}

/// Add environmental protection discovery nodes (BT-118~122). Returns (nodes, edges).
pub fn populate_env_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ENV_DISCOVERIES)
}

/// Add robotics discovery nodes (BT-123~127). Returns (nodes, edges).
pub fn populate_robo_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ROBO_DISCOVERIES)
}

/// Add cross-domain bridge discoveries connecting all BT-105+ clusters. Returns (nodes, edges).
pub fn populate_bridge_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, BRIDGE_DISCOVERIES)
}

/// Cross-link discovery nodes that share domains (Merges edges, bidirectional).
/// Must be called after all discovery nodes are populated.
/// Returns the number of cross-domain edges added.
pub fn connect_discovery_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let disc_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-") || n.id.starts_with("HYP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..disc_nodes.len() {
        for j in (i + 1)..disc_nodes.len() {
            let (ref id_a, ref dom_a) = disc_nodes[i];
            let (ref id_b, ref dom_b) = disc_nodes[j];

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

/// Populate all BT-105+ discovery nodes and cross-domain edges in one call.
/// Call after populate_bt_graph() and populate_expanded_graph().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_math_discoveries(graph);
    let (n2, _) = populate_sw_discoveries(graph);
    let (n3, _) = populate_env_discoveries(graph);
    let (n4, _) = populate_robo_discoveries(graph);
    let (n5, _) = populate_bridge_discoveries(graph);
    let _cross = connect_discovery_cross_domain(graph);

    let total_nodes = n1 + n2 + n3 + n4 + n5;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total discovery entry count across all clusters.
pub fn discovery_entry_count() -> usize {
    MATH_DISCOVERIES.len()
        + SW_DISCOVERIES.len()
        + ENV_DISCOVERIES.len()
        + ROBO_DISCOVERIES.len()
        + BRIDGE_DISCOVERIES.len()
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;

    fn full_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        g
    }

    // ── Cluster size tests ──

    #[test]
    fn test_math_discovery_count() {
        assert_eq!(MATH_DISCOVERIES.len(), 9, "8 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_sw_discovery_count() {
        assert_eq!(SW_DISCOVERIES.len(), 6, "5 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_env_discovery_count() {
        assert_eq!(ENV_DISCOVERIES.len(), 6, "5 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_robo_discovery_count() {
        assert_eq!(ROBO_DISCOVERIES.len(), 6, "5 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_bridge_discovery_count() {
        assert_eq!(BRIDGE_DISCOVERIES.len(), 12, "6 original + 6 new cross-cluster bridges");
    }

    #[test]
    fn test_total_discovery_entry_count() {
        // 9 + 6 + 6 + 6 + 12 = 39
        assert_eq!(discovery_entry_count(), 39);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_math_discoveries() {
        let mut g = full_graph();
        let nodes_before = g.nodes.len();
        let (nodes, edges) = populate_math_discoveries(&mut g);
        assert_eq!(nodes, 9);
        assert_eq!(g.nodes.len(), nodes_before + 9);
        // Each entry has source_bts + constants_used edges
        assert!(edges > 15, "Should have 15+ edges from math discoveries, got {}", edges);
    }

    #[test]
    fn test_populate_sw_discoveries() {
        let mut g = full_graph();
        let nodes_before = g.nodes.len();
        let (nodes, edges) = populate_sw_discoveries(&mut g);
        assert_eq!(nodes, 6);
        assert_eq!(g.nodes.len(), nodes_before + 6);
        assert!(edges > 15, "Should have 15+ edges from sw discoveries, got {}", edges);
    }

    #[test]
    fn test_populate_env_discoveries() {
        let mut g = full_graph();
        let nodes_before = g.nodes.len();
        let (nodes, edges) = populate_env_discoveries(&mut g);
        assert_eq!(nodes, 6);
        assert_eq!(g.nodes.len(), nodes_before + 6);
        assert!(edges > 10, "Should have 10+ edges from env discoveries, got {}", edges);
    }

    #[test]
    fn test_populate_robo_discoveries() {
        let mut g = full_graph();
        let nodes_before = g.nodes.len();
        let (nodes, edges) = populate_robo_discoveries(&mut g);
        assert_eq!(nodes, 6);
        assert_eq!(g.nodes.len(), nodes_before + 6);
        assert!(edges > 10, "Should have 10+ edges from robo discoveries, got {}", edges);
    }

    #[test]
    fn test_populate_bridge_discoveries() {
        let mut g = full_graph();
        let (nodes, edges) = populate_bridge_discoveries(&mut g);
        assert_eq!(nodes, 12);
        assert!(edges > 30, "12 bridges should have many edges, got {}", edges);
    }

    // ── Integration tests ──

    #[test]
    fn test_all_discoveries_integrated() {
        let mut g = full_graph();
        let base_nodes = g.nodes.len();
        let base_edges = g.edges.len();
        let (nodes_added, edges_added) = populate_all_discoveries(&mut g);

        assert_eq!(nodes_added, 39, "39 discovery nodes total");
        assert!(
            edges_added > 150,
            "Should add 150+ edges (derives + uses + cross-domain), got {}",
            edges_added
        );
        assert_eq!(g.nodes.len(), base_nodes + 39);
        assert!(g.edges.len() > base_edges + 150);
    }

    #[test]
    fn test_cross_domain_edges_created() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        // DISC-ENV-05 (Environment, Math, Biology, Material) and
        // DISC-ROBO-05 (Robotics, Cosmology, Material, Math) share Math + Material
        let cross_edge = g.edges.iter().any(|e| {
            (e.from == "DISC-ENV-05" && e.to == "DISC-ROBO-05")
                || (e.from == "DISC-ROBO-05" && e.to == "DISC-ENV-05")
        });
        assert!(cross_edge, "ENV-05 and ROBO-05 should be connected via shared Math/Material");
    }

    #[test]
    fn test_math_sw_cross_domain_edge() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        // DISC-MATH-01 (Math, Physics, Topology) and DISC-SW-01 (Software, Math, AI)
        // share Math domain
        let cross_edge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-MATH-01" && e.to == "DISC-SW-01")
                    || (e.from == "DISC-SW-01" && e.to == "DISC-MATH-01"))
        });
        assert!(cross_edge, "MATH-01 and SW-01 should be connected via shared Math domain");
    }

    #[test]
    fn test_new_bridges_connect_all_clusters() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        // DISC-BRIDGE-07 connects Math and Software clusters
        let bridge07 = g.nodes.iter().find(|n| n.id == "DISC-BRIDGE-07").unwrap();
        assert!(bridge07.domain.contains("Math"));
        assert!(bridge07.domain.contains("Software"));

        // DISC-BRIDGE-12 connects Software, AI, and Robotics
        let bridge12 = g.nodes.iter().find(|n| n.id == "DISC-BRIDGE-12").unwrap();
        assert!(bridge12.domain.contains("Software"));
        assert!(bridge12.domain.contains("AI"));
        assert!(bridge12.domain.contains("Robotics"));
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = full_graph();
        populate_env_discoveries(&mut g);

        // BT-118 should derive DISC-ENV-01
        let has_derive = g.edges.iter().any(|e| {
            e.from == "BT-118" && e.to == "DISC-ENV-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has_derive, "BT-118 should derive DISC-ENV-01");
    }

    #[test]
    fn test_bt_derives_math_discoveries() {
        let mut g = full_graph();
        populate_math_discoveries(&mut g);

        // BT-105 should derive DISC-MATH-01
        let has_derive = g.edges.iter().any(|e| {
            e.from == "BT-105" && e.to == "DISC-MATH-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has_derive, "BT-105 should derive DISC-MATH-01");

        // BT-109 should derive DISC-MATH-05
        let has_derive = g.edges.iter().any(|e| {
            e.from == "BT-109" && e.to == "DISC-MATH-05" && e.edge_type == EdgeType::Derives
        });
        assert!(has_derive, "BT-109 should derive DISC-MATH-05");
    }

    #[test]
    fn test_bt_derives_sw_discoveries() {
        let mut g = full_graph();
        populate_sw_discoveries(&mut g);

        // BT-113 should derive DISC-SW-01
        let has_derive = g.edges.iter().any(|e| {
            e.from == "BT-113" && e.to == "DISC-SW-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has_derive, "BT-113 should derive DISC-SW-01");
    }

    #[test]
    fn test_constant_uses_edges() {
        let mut g = full_graph();
        populate_robo_discoveries(&mut g);

        // DISC-ROBO-03 uses C-tau
        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-ROBO-03" && e.to == "C-tau" && e.edge_type == EdgeType::Uses
        });
        assert!(has_uses, "DISC-ROBO-03 should use C-tau");
    }

    #[test]
    fn test_math_constant_uses() {
        let mut g = full_graph();
        populate_math_discoveries(&mut g);

        // DISC-MATH-03 uses C-J2 (Ramanujan tau, J₂=24)
        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-MATH-03" && e.to == "C-J2" && e.edge_type == EdgeType::Uses
        });
        assert!(has_uses, "DISC-MATH-03 should use C-J2");

        // DISC-MATH-02 uses C-tau
        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-MATH-02" && e.to == "C-tau" && e.edge_type == EdgeType::Uses
        });
        assert!(has_uses, "DISC-MATH-02 should use C-tau");
    }

    #[test]
    fn test_sw_constant_uses() {
        let mut g = full_graph();
        populate_sw_discoveries(&mut g);

        // DISC-SW-01 uses all four: C-sopfr, C-n, C-sigma, C-tau
        for cid in &["C-sopfr", "C-n", "C-sigma", "C-tau"] {
            let has_uses = g.edges.iter().any(|e| {
                e.from == "DISC-SW-01" && e.to == *cid && e.edge_type == EdgeType::Uses
            });
            assert!(has_uses, "DISC-SW-01 should use {}", cid);
        }
    }

    #[test]
    fn test_hypothesis_nodes_lower_confidence() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        let hypotheses: Vec<_> = g.nodes.iter()
            .filter(|n| n.id.starts_with("HYP-"))
            .collect();
        assert_eq!(hypotheses.len(), 4, "Should have 4 hypothesis nodes (MATH+SW+ENV+ROBO)");
        for h in &hypotheses {
            assert!(
                h.confidence < 0.75,
                "Hypothesis {} confidence {} should be < 0.75",
                h.id, h.confidence
            );
        }
    }

    #[test]
    fn test_bridge_connects_env_and_robo() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        // DISC-BRIDGE-01 domains include both Environment and Robotics
        let bridge = g.nodes.iter().find(|n| n.id == "DISC-BRIDGE-01").unwrap();
        assert!(bridge.domain.contains("Environment"));
        assert!(bridge.domain.contains("Robotics"));
    }

    #[test]
    fn test_edge_strength_in_range() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} strength {} out of range",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_discovery_node_types() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        let disc_count = g.nodes.iter()
            .filter(|n| n.node_type == NodeType::Discovery && n.id.starts_with("DISC-"))
            .count();
        let hyp_count = g.nodes.iter()
            .filter(|n| n.node_type == NodeType::Hypothesis && n.id.starts_with("HYP-"))
            .count();

        // 8+5+5+5+12 = 35 Discovery nodes
        assert_eq!(disc_count, 35, "35 Discovery nodes");
        // 1+1+1+1 = 4 Hypothesis nodes
        assert_eq!(hyp_count, 4, "4 Hypothesis nodes");
    }

    #[test]
    fn test_full_graph_node_count_with_discoveries() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        // 127 BT + 79 expanded + 39 discoveries = 245
        assert!(
            g.nodes.len() >= 240,
            "Full graph with discoveries should have 240+ nodes, got {}",
            g.nodes.len()
        );
    }

    #[test]
    fn test_no_duplicate_node_ids() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_all_source_bts_exist_in_graph() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        let all_entries: Vec<&DiscoveryEntry> = MATH_DISCOVERIES.iter()
            .chain(SW_DISCOVERIES.iter())
            .chain(ENV_DISCOVERIES.iter())
            .chain(ROBO_DISCOVERIES.iter())
            .chain(BRIDGE_DISCOVERIES.iter())
            .collect();

        for entry in all_entries {
            for &bt_id in entry.source_bts {
                let bt_node_id = format!("BT-{}", bt_id);
                assert!(
                    g.nodes.iter().any(|n| n.id == bt_node_id),
                    "Source BT {} referenced by {} not found in graph",
                    bt_node_id, entry.id
                );
            }
        }
    }

    #[test]
    fn test_cross_domain_edge_count_increased() {
        // With 39 discovery nodes sharing many domains, cross-domain edges should be substantial
        let mut g = full_graph();
        populate_math_discoveries(&mut g);
        populate_sw_discoveries(&mut g);
        populate_env_discoveries(&mut g);
        populate_robo_discoveries(&mut g);
        populate_bridge_discoveries(&mut g);
        let cross_count = connect_discovery_cross_domain(&mut g);

        // 39 nodes with significant domain overlap → many cross edges
        assert!(
            cross_count > 100,
            "39 discovery nodes should generate 100+ cross-domain edges, got {}",
            cross_count
        );
    }

    #[test]
    fn test_four_cluster_interconnection() {
        let mut g = full_graph();
        populate_all_discoveries(&mut g);

        // Verify that all four clusters are interconnected via Merges edges
        let has_math_env = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from.starts_with("DISC-MATH") && e.to.starts_with("DISC-ENV"))
                    || (e.from.starts_with("DISC-ENV") && e.to.starts_with("DISC-MATH")))
        });
        let has_sw_robo = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from.starts_with("DISC-SW") && e.to.starts_with("DISC-ROBO"))
                    || (e.from.starts_with("DISC-ROBO") && e.to.starts_with("DISC-SW")))
        });
        let has_math_sw = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from.starts_with("DISC-MATH") && e.to.starts_with("DISC-SW"))
                    || (e.from.starts_with("DISC-SW") && e.to.starts_with("DISC-MATH")))
        });

        assert!(has_math_env, "Math and Environment clusters should be interconnected");
        assert!(has_sw_robo, "Software and Robotics clusters should be interconnected");
        assert!(has_math_sw, "Math and Software clusters should be interconnected");
    }
}
