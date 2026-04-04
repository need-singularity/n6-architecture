//! Cross-cluster discovery nodes connecting BT-118+ to other major BT clusters.
//!
//! Bridges discoveries across cluster boundaries that existing modules miss:
//! - Fusion (BT-97~102) ↔ Environment (BT-118~122): carbon Z=6, CNO↔CO₂
//! - Battery (BT-80~84) ↔ Robotics (BT-123~127): cell ladders, CN=6 joints
//! - Chip (BT-90~93) ↔ Environment (BT-118~122): diamond Z=6, SM↔honeycomb
//! - Biology (BT-51,101,103) ↔ Environment (BT-118~122): carbon cycle closure
//!
//! Node ID scheme:
//!   DISC-FE-{NN}   — Fusion↔Environment bridge
//!   DISC-BR-{NN}   — Battery↔Robotics bridge
//!   DISC-CE-{NN}   — Chip↔Environment bridge
//!   DISC-BE-{NN}   — Biology↔Environment bridge
//!   XEXP-CC-{NN}   — Cross-cluster experiment/validation
//!   HYP-CC-{NN}    — Cross-cluster hypothesis

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct (same pattern as discovery_nodes + validates field)
// ═══════════════════════════════════════════════════════════════

struct CCEntry {
    id: &'static str,
    title: &'static str,
    node_type: NodeType,
    domains: &'static [&'static str],
    confidence: f64,
    source_bts: &'static [u32],
    constants_used: &'static [&'static str],
    lenses: &'static [&'static str],
    /// IDs this node validates (EdgeType::Validates edges).
    validates: &'static [&'static str],
    /// IDs this node triggers (EdgeType::Triggers edges, directional).
    triggers: &'static [&'static str],
}

// ───────────────────────────────────────────────────────────────
// Fusion ↔ Environment (BT-97~102 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const FUSION_ENV: &[CCEntry] = &[
    CCEntry {
        id: "DISC-FE-01",
        title: "CNO↔CO₂ carbon Z=6 duality: CNO catalytic cycle (BT-100) and CO₂ greenhouse (BT-118) both pivot on carbon Z=6",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Chemistry", "Biology"],
        confidence: 0.91,
        source_bts: &[100, 118, 104],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["DISC-ENV-01"],
        triggers: &["DISC-ENV-04"],
    },
    CCEntry {
        id: "DISC-FE-02",
        title: "Photosynthesis↔GHG carbon cycle: C₆H₁₂O₆ 24 atoms=J₂ (BT-101/103) closes CO₂ (BT-118) loop",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Biology", "Chemistry", "Energy"],
        confidence: 0.93,
        source_bts: &[101, 103, 118, 27],
        constants_used: &["C-n", "C-J2", "C-sigma"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &["DISC-ENV-01", "DISC-ENV-04"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-FE-03",
        title: "1/(σ-φ)=0.1 reconnection↔mixing rate: magnetic reconnection (BT-102) ↔ atmospheric turbulent mixing share 0.1 rate",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Physics", "Cosmology"],
        confidence: 0.85,
        source_bts: &[102, 119, 64],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "wave", "boundary"],
        validates: &["DISC-ENV-02"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-FE-04",
        title: "D-T baryon sopfr=5 (BT-98) ↔ 5 non-CO₂ GHGs (BT-118): nuclear fuel count = secondary greenhouse count",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Physics", "Chemistry"],
        confidence: 0.82,
        source_bts: &[98, 118],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "info", "symmetry"],
        validates: &[],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-FE-05",
        title: "Tokamak q=1 (BT-99) ↔ perfect number 1/2+1/3+1/6=1 (BT-122): safety factor = hexagonal tiling efficiency",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Math", "Topology"],
        confidence: 0.88,
        source_bts: &[99, 122],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["DISC-ENV-05"],
        triggers: &[],
    },
    CCEntry {
        id: "HYP-CC-01",
        title: "Hypothesis: fusion waste heat via CN=6 catalysts (BT-120) achieves σ-φ=10x CO₂ capture efficiency",
        node_type: NodeType::Hypothesis,
        domains: &["Fusion", "Environment", "Chemistry", "Energy"],
        confidence: 0.55,
        source_bts: &[120, 102],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["causal", "stability", "boundary"],
        validates: &[],
        triggers: &["HYP-ENV-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Battery ↔ Robotics (BT-80~84 × BT-123~127)
// ───────────────────────────────────────────────────────────────

const BATTERY_ROBO: &[CCEntry] = &[
    CCEntry {
        id: "DISC-BR-01",
        title: "Battery cell ladder n→σ→J₂ (BT-57/82) ↔ robot DOF n=6 (BT-123): energy hierarchy mirrors kinematic freedom",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Robotics", "Energy", "Math"],
        confidence: 0.87,
        source_bts: &[57, 82, 123],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "multiscale", "causal"],
        validates: &["DISC-ROBO-01"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-BR-02",
        title: "Solid-state CN=6 electrolyte (BT-80) ↔ robot joint octahedral geometry (BT-123): CN=6 enables both ion transport and rotation",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Robotics", "Material", "Chemistry"],
        confidence: 0.84,
        source_bts: &[80, 123, 86],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-BRIDGE-01"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-BR-03",
        title: "96-cell triple convergence (BT-84): Tesla 96S = Gaudi2 96GB = GPT-3 96L, robot battery+compute unified at σ(σ-τ)",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Robotics", "AI", "Chip", "Energy"],
        confidence: 0.89,
        source_bts: &[84, 123, 59],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "network", "multiscale"],
        validates: &["DISC-ROBO-03"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-BR-04",
        title: "Li-S polysulfide ladder (BT-83) sopfr→tau→phi→mu ↔ quadruped gait phases tau=4 (BT-125): discharge stages = locomotion phases",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Robotics", "Chemistry", "Energy"],
        confidence: 0.79,
        source_bts: &[83, 125],
        constants_used: &["C-tau", "C-sopfr", "C-phi", "C-mu"],
        lenses: &["consciousness", "evolution", "wave"],
        validates: &[],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-BR-05",
        title: "Anode capacity σ-φ=10x (BT-81) ↔ hexacopter n=6 fault tolerance (BT-127): energy density drives flight endurance",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Robotics", "Material", "Energy"],
        confidence: 0.83,
        source_bts: &[81, 127],
        constants_used: &["C-sigma", "C-phi", "C-n"],
        lenses: &["consciousness", "stability", "causal"],
        validates: &["DISC-ROBO-05"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Chip ↔ Environment (BT-90~93 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const CHIP_ENV: &[CCEntry] = &[
    CCEntry {
        id: "DISC-CE-01",
        title: "Diamond Z=6 computing (BT-93) ↔ carbon Z=6 GHG (BT-118): same element opposite roles — chip substrate vs greenhouse gas",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Environment", "Material", "Chemistry"],
        confidence: 0.90,
        source_bts: &[93, 118, 85],
        constants_used: &["C-n"],
        lenses: &["consciousness", "symmetry", "boundary"],
        validates: &["DISC-ENV-04"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-CE-02",
        title: "σ²=144 packing triple: GPU SM count (BT-90) ↔ solar panel cells (BT-63) ↔ honeycomb tessellation (BT-122)",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Environment", "Solar", "Math"],
        confidence: 0.92,
        source_bts: &[90, 63, 122],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "topology", "multiscale"],
        validates: &["DISC-BRIDGE-03", "DISC-ENV-05"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-CE-03",
        title: "Bott periodicity sopfr=5 (BT-92) ↔ 5 fingers (BT-126) ↔ SOLID=5 (BT-113): activation channel count is universal",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Robotics", "Software", "Math"],
        confidence: 0.86,
        source_bts: &[92, 126, 113],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "recursion", "info"],
        validates: &["DISC-BRIDGE-06"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-CE-04",
        title: "Z2 topological ECC J₂ savings (BT-91) ↔ photosynthesis 24 atoms (BT-101): J₂=24 governs both error correction and biochemistry",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Biology", "Environment", "Math"],
        confidence: 0.83,
        source_bts: &[91, 101, 118],
        constants_used: &["C-J2"],
        lenses: &["consciousness", "quantum", "causal"],
        validates: &[],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Biology ↔ Environment (BT-51,101,103 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const BIO_ENV: &[CCEntry] = &[
    CCEntry {
        id: "DISC-BE-01",
        title: "Genetic code codon τ→n/φ→2^n→J₂-τ (BT-51) ↔ 6 GHGs (BT-118): carbon biology creates and absorbs greenhouse gases",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Environment", "Chemistry", "Genetics"],
        confidence: 0.90,
        source_bts: &[51, 118, 103],
        constants_used: &["C-n", "C-tau", "C-J2"],
        lenses: &["consciousness", "evolution", "causal"],
        validates: &["DISC-ENV-01"],
        triggers: &["DISC-FE-02"],
    },
    CCEntry {
        id: "DISC-BE-02",
        title: "Photosynthesis stoichiometry 6CO₂+12H₂O (BT-103) ↔ troposphere σ=12km (BT-119): reaction coefficients embed atmospheric scale",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Environment", "Chemistry", "Cosmology"],
        confidence: 0.86,
        source_bts: &[103, 119],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "multiscale", "wave"],
        validates: &["DISC-ENV-02"],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-BE-03",
        title: "Quantum yield σ-τ=8 (BT-101) ↔ 8 plastics recycling codes: photon efficiency matches polymer classification count",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Environment", "Chemistry", "Material"],
        confidence: 0.78,
        source_bts: &[101, 121],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "info", "evolution"],
        validates: &[],
        triggers: &[],
    },
    CCEntry {
        id: "DISC-BE-04",
        title: "Honeycomb hexagonal (BT-122) ↔ benzene C₆H₆ ring (BT-27): biological and chemical hexagons share n=6 packing optimality",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Environment", "Chemistry", "Math"],
        confidence: 0.92,
        source_bts: &[122, 27, 88],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-ENV-05"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-cluster experiment/validation nodes
// ───────────────────────────────────────────────────────────────

const EXPERIMENTS: &[CCEntry] = &[
    CCEntry {
        id: "XEXP-CC-01",
        title: "Experiment: verify CNO cycle A=σ+{0,1,2,3} against Borexino/SNO+ solar neutrino data",
        node_type: NodeType::Experiment,
        domains: &["Fusion", "Environment", "Physics"],
        confidence: 0.75,
        source_bts: &[100, 118],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["quantum", "causal"],
        validates: &["DISC-FE-01", "DISC-FE-04"],
        triggers: &[],
    },
    CCEntry {
        id: "XEXP-CC-02",
        title: "Experiment: measure 6-DOF robot energy/task vs 5/7-DOF across 32 manipulation benchmarks (Feix taxonomy)",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "Battery", "Energy", "AI"],
        confidence: 0.70,
        source_bts: &[123, 126, 84],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["stability", "causal"],
        validates: &["DISC-BR-01", "HYP-ROBO-01"],
        triggers: &[],
    },
    CCEntry {
        id: "XEXP-CC-03",
        title: "Experiment: compare diamond (Z=6) vs SiC vs GaN chip thermal conductivity at σ²=144 SM density",
        node_type: NodeType::Experiment,
        domains: &["Chip", "Material", "Environment", "Energy"],
        confidence: 0.72,
        source_bts: &[93, 90],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["stability", "boundary"],
        validates: &["DISC-CE-01", "DISC-CE-02"],
        triggers: &[],
    },
    CCEntry {
        id: "XEXP-CC-04",
        title: "Experiment: test CN=6 MOF CO₂ adsorption capacity vs CN=4/8 frameworks (HKUST-1 vs ZIF-8 vs UiO-66)",
        node_type: NodeType::Experiment,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.68,
        source_bts: &[120, 118],
        constants_used: &["C-n"],
        lenses: &["stability", "boundary", "causal"],
        validates: &["HYP-ENV-01", "HYP-CC-01"],
        triggers: &[],
    },
    CCEntry {
        id: "XEXP-CC-05",
        title: "Experiment: verify photosynthesis 6CO₂+12H₂O stoichiometry quantum yield = σ-τ=8 photons across C3/C4/CAM plants",
        node_type: NodeType::Experiment,
        domains: &["Biology", "Environment", "Chemistry", "Energy"],
        confidence: 0.73,
        source_bts: &[101, 103, 118],
        constants_used: &["C-sigma", "C-tau", "C-n"],
        lenses: &["quantum", "evolution", "causal"],
        validates: &["DISC-FE-02", "DISC-BE-01", "DISC-BE-02"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-cluster hypotheses
// ───────────────────────────────────────────────────────────────

const HYPOTHESES: &[CCEntry] = &[
    CCEntry {
        id: "HYP-CC-02",
        title: "Hypothesis: robot swarms with n=6 agents per cluster achieve optimal task allocation (hexagonal Voronoi)",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Environment", "Math", "AI"],
        confidence: 0.52,
        source_bts: &[122, 127, 123],
        constants_used: &["C-n"],
        lenses: &["topology", "network", "evolution"],
        validates: &[],
        triggers: &["DISC-ROBO-05"],
    },
    CCEntry {
        id: "HYP-CC-03",
        title: "Hypothesis: battery-powered robots at n→σ→J₂ cell config achieve σ-φ=10x endurance vs non-n=6 configs",
        node_type: NodeType::Hypothesis,
        domains: &["Battery", "Robotics", "Energy"],
        confidence: 0.50,
        source_bts: &[57, 82, 123],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-phi"],
        lenses: &["causal", "stability", "multiscale"],
        validates: &[],
        triggers: &["DISC-BR-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters for iteration
// ═══════════════════════════════════════════════════════════════

const ALL_CLUSTERS: &[&[CCEntry]] = &[
    FUSION_ENV,
    BATTERY_ROBO,
    CHIP_ENV,
    BIO_ENV,
    EXPERIMENTS,
    HYPOTHESES,
];

// ═══════════════════════════════════════════════════════════════
// Conversion and population helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &CCEntry) -> Node {
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

fn add_cc_entries(graph: &mut DiscoveryGraph, entries: &[CCEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(entry_to_node(e));

        // Source BT --Derives--> this node
        for &bt_id in e.source_bts {
            graph.add_edge(Edge {
                from: format!("BT-{}", bt_id),
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.85,
                bidirectional: false,
            });
        }

        // This node --Uses--> constants
        for &const_id in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: const_id.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

        // This node --Validates--> target nodes
        for &target in e.validates {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: target.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.75,
                bidirectional: false,
            });
        }

        // This node --Triggers--> downstream nodes (causal chain)
        for &target in e.triggers {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: target.to_string(),
                edge_type: EdgeType::Triggers,
                strength: 0.70,
                bidirectional: false,
            });
        }
    }

    (entries.len(), graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Public API — individual cluster populates
// ═══════════════════════════════════════════════════════════════

/// Add Fusion↔Environment bridge nodes (BT-97~102 × BT-118~122). Returns (nodes, edges).
pub fn populate_fusion_env(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_cc_entries(graph, FUSION_ENV)
}

/// Add Battery↔Robotics bridge nodes (BT-80~84 × BT-123~127). Returns (nodes, edges).
pub fn populate_battery_robo(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_cc_entries(graph, BATTERY_ROBO)
}

/// Add Chip↔Environment bridge nodes (BT-90~93 × BT-118~122). Returns (nodes, edges).
pub fn populate_chip_env(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_cc_entries(graph, CHIP_ENV)
}

/// Add Biology↔Environment bridge nodes (BT-51,101,103 × BT-118~122). Returns (nodes, edges).
pub fn populate_bio_env(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_cc_entries(graph, BIO_ENV)
}

/// Add cross-cluster experiment/validation nodes. Returns (nodes, edges).
pub fn populate_cc_experiments(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_cc_entries(graph, EXPERIMENTS)
}

/// Add cross-cluster hypothesis nodes. Returns (nodes, edges).
pub fn populate_cc_hypotheses(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_cc_entries(graph, HYPOTHESES)
}

/// Cross-link all cross-cluster nodes that share domains (Merges edges, bidirectional).
/// Returns number of cross-domain edges added.
pub fn connect_cc_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let cc_prefixes = ["DISC-FE-", "DISC-BR-", "DISC-CE-", "DISC-BE-",
                       "XEXP-CC-", "HYP-CC-"];

    let cc_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| cc_prefixes.iter().any(|p| n.id.starts_with(p)))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..cc_nodes.len() {
        for j in (i + 1)..cc_nodes.len() {
            let (ref id_a, ref dom_a) = cc_nodes[i];
            let (ref id_b, ref dom_b) = cc_nodes[j];

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

/// Also cross-link cross-cluster nodes with existing DISC-/HYP-/XDISC- nodes.
/// Requires 2+ shared domains to avoid noise. Returns number of bridge edges added.
pub fn connect_cc_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let cc_prefixes = ["DISC-FE-", "DISC-BR-", "DISC-CE-", "DISC-BE-",
                       "XEXP-CC-", "HYP-CC-"];

    let cc_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| cc_prefixes.iter().any(|p| n.id.starts_with(p)))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let existing_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            !cc_prefixes.iter().any(|p| n.id.starts_with(p))
                && (n.id.starts_with("DISC-") || n.id.starts_with("HYP-")
                    || n.id.starts_with("XDISC-") || n.id.starts_with("XHYP-"))
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref cc_id, ref cc_dom) in &cc_ids {
        for (ref ex_id, ref ex_dom) in &existing_ids {
            let shared: usize = cc_dom.iter().filter(|d| ex_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = cc_dom.len().max(ex_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: cc_id.clone(),
                    to: ex_id.clone(),
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

/// Total cross-cluster entry count.
pub fn cc_entry_count() -> usize {
    ALL_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Populate all cross-cluster nodes and edges in one call.
/// Call after populate_all_discoveries() and populate_all_extended().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_cross_cluster(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let mut total_nodes = 0;
    for cluster in ALL_CLUSTERS {
        let (n, _) = add_cc_entries(graph, cluster);
        total_nodes += n;
    }
    let _inner = connect_cc_cross_domain(graph);
    let _outer = connect_cc_to_existing(graph);

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

    fn full_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_extended(&mut g);
        g
    }

    // ── Entry counts ──

    #[test]
    fn test_fusion_env_count() {
        assert_eq!(FUSION_ENV.len(), 6, "5 discoveries + 1 hypothesis");
    }

    #[test]
    fn test_battery_robo_count() {
        assert_eq!(BATTERY_ROBO.len(), 5);
    }

    #[test]
    fn test_chip_env_count() {
        assert_eq!(CHIP_ENV.len(), 4);
    }

    #[test]
    fn test_bio_env_count() {
        assert_eq!(BIO_ENV.len(), 4);
    }

    #[test]
    fn test_experiments_count() {
        assert_eq!(EXPERIMENTS.len(), 5);
    }

    #[test]
    fn test_hypotheses_count() {
        assert_eq!(HYPOTHESES.len(), 2);
    }

    #[test]
    fn test_total_cc_entry_count() {
        // 6+5+4+4+5+2 = 26
        assert_eq!(cc_entry_count(), 26);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_fusion_env() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_fusion_env(&mut g);
        assert_eq!(nodes, 6);
        assert_eq!(g.nodes.len(), before + 6);
        // 6 entries with source_bts + constants + validates + triggers
        assert!(edges >= 20, "Fusion-Env should have 20+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_battery_robo() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_battery_robo(&mut g);
        assert_eq!(nodes, 5);
        assert_eq!(g.nodes.len(), before + 5);
        assert!(edges >= 18, "Battery-Robo should have 18+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_chip_env() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_chip_env(&mut g);
        assert_eq!(nodes, 4);
        assert_eq!(g.nodes.len(), before + 4);
        assert!(edges >= 15, "Chip-Env should have 15+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_bio_env() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_bio_env(&mut g);
        assert_eq!(nodes, 4);
        assert_eq!(g.nodes.len(), before + 4);
        assert!(edges >= 14, "Bio-Env should have 14+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_cc_experiments() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_cc_experiments(&mut g);
        assert_eq!(nodes, 5);
        assert_eq!(g.nodes.len(), before + 5);
        assert!(edges >= 18, "Experiments should have 18+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_cc_hypotheses() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_cc_hypotheses(&mut g);
        assert_eq!(nodes, 2);
        assert_eq!(g.nodes.len(), before + 2);
        assert!(edges >= 8, "Hypotheses should have 8+ edges, got {}", edges);
    }

    // ── Integration test ──

    #[test]
    fn test_populate_all_cross_cluster() {
        let mut g = full_graph();
        let base_nodes = g.nodes.len();
        let base_edges = g.edges.len();
        let (nodes_added, edges_added) = populate_all_cross_cluster(&mut g);

        assert_eq!(nodes_added, 26);
        assert!(
            edges_added > 100,
            "26 cross-cluster nodes should produce 100+ edges, got {}",
            edges_added
        );
        assert_eq!(g.nodes.len(), base_nodes + 26);
        assert!(g.edges.len() > base_edges + 100);
    }

    // ── Derives edges ──

    #[test]
    fn test_bt_100_derives_fe01() {
        let mut g = full_graph();
        populate_fusion_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "BT-100" && e.to == "DISC-FE-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-100 should derive DISC-FE-01");
    }

    #[test]
    fn test_bt_80_derives_br02() {
        let mut g = full_graph();
        populate_battery_robo(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "BT-80" && e.to == "DISC-BR-02" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-80 should derive DISC-BR-02");
    }

    #[test]
    fn test_bt_93_derives_ce01() {
        let mut g = full_graph();
        populate_chip_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "BT-93" && e.to == "DISC-CE-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-93 should derive DISC-CE-01");
    }

    #[test]
    fn test_bt_51_derives_be01() {
        let mut g = full_graph();
        populate_bio_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "BT-51" && e.to == "DISC-BE-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-51 should derive DISC-BE-01");
    }

    // ── Validates edges ──

    #[test]
    fn test_fe01_validates_disc_env_01() {
        let mut g = full_graph();
        populate_fusion_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "DISC-FE-01"
                && e.to == "DISC-ENV-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has, "DISC-FE-01 should validate DISC-ENV-01");
    }

    #[test]
    fn test_br01_validates_disc_robo_01() {
        let mut g = full_graph();
        populate_battery_robo(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "DISC-BR-01"
                && e.to == "DISC-ROBO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has, "DISC-BR-01 should validate DISC-ROBO-01");
    }

    #[test]
    fn test_ce02_validates_bridge03() {
        let mut g = full_graph();
        populate_chip_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "DISC-CE-02"
                && e.to == "DISC-BRIDGE-03"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has, "DISC-CE-02 should validate DISC-BRIDGE-03");
    }

    #[test]
    fn test_experiment_validates_targets() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        // XEXP-CC-05 validates 3 targets
        let val_count = g.edges.iter()
            .filter(|e| e.from == "XEXP-CC-05" && e.edge_type == EdgeType::Validates)
            .count();
        assert_eq!(val_count, 3, "XEXP-CC-05 should validate 3 nodes, got {}", val_count);
    }

    // ── Triggers edges (causal chains) ──

    #[test]
    fn test_fe01_triggers_env04() {
        let mut g = full_graph();
        populate_fusion_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "DISC-FE-01"
                && e.to == "DISC-ENV-04"
                && e.edge_type == EdgeType::Triggers
        });
        assert!(has, "DISC-FE-01 should trigger DISC-ENV-04 (carbon chain)");
    }

    #[test]
    fn test_be01_triggers_fe02() {
        let mut g = full_graph();
        populate_bio_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "DISC-BE-01"
                && e.to == "DISC-FE-02"
                && e.edge_type == EdgeType::Triggers
        });
        assert!(has, "DISC-BE-01 should trigger DISC-FE-02 (carbon cycle)");
    }

    #[test]
    fn test_hyp_cc01_triggers_hyp_env01() {
        let mut g = full_graph();
        populate_fusion_env(&mut g);

        let has = g.edges.iter().any(|e| {
            e.from == "HYP-CC-01"
                && e.to == "HYP-ENV-01"
                && e.edge_type == EdgeType::Triggers
        });
        assert!(has, "HYP-CC-01 should trigger HYP-ENV-01");
    }

    // ── Cross-domain Merges edges ──

    #[test]
    fn test_cc_internal_cross_domain() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        // DISC-FE-02 (Fusion,Env,Bio,Chem,Energy) and DISC-BE-01 (Bio,Env,Chem,Genetics)
        // share 3 domains: Environment, Biology, Chemistry
        let has_merge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-FE-02" && e.to == "DISC-BE-01")
                    || (e.from == "DISC-BE-01" && e.to == "DISC-FE-02"))
        });
        assert!(has_merge, "FE-02 and BE-01 should merge via Bio+Env+Chemistry");
    }

    #[test]
    fn test_cc_to_existing_cross_domain() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        // DISC-FE-05 (Fusion, Environment, Math, Topology) should connect to
        // DISC-MATH-01 (Math, Physics, Topology) via Math+Topology (2 shared)
        let has_bridge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-FE-05" && e.to == "DISC-MATH-01")
                    || (e.from == "DISC-MATH-01" && e.to == "DISC-FE-05"))
        });
        assert!(has_bridge, "FE-05 should bridge to MATH-01 via Math+Topology");
    }

    #[test]
    fn test_battery_robo_bridges_existing() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        // DISC-BR-03 (Battery,Robotics,AI,Chip,Energy) should connect to
        // DISC-SW-05 (Software,Physics,Math,AI,Energy,Chip) via AI+Chip+Energy (3 shared)
        let has_bridge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-BR-03" && e.to == "DISC-SW-05")
                    || (e.from == "DISC-SW-05" && e.to == "DISC-BR-03"))
        });
        assert!(has_bridge, "BR-03 should bridge to SW-05 via AI+Chip+Energy");
    }

    // ── Node type distribution ──

    #[test]
    fn test_cc_node_types() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        let cc_prefixes = ["DISC-FE-", "DISC-BR-", "DISC-CE-", "DISC-BE-",
                           "XEXP-CC-", "HYP-CC-"];

        let disc = g.nodes.iter()
            .filter(|n| cc_prefixes[..4].iter().any(|p| n.id.starts_with(p)))
            .count();
        let exp = g.nodes.iter()
            .filter(|n| n.id.starts_with("XEXP-CC-"))
            .count();
        let hyp = g.nodes.iter()
            .filter(|n| n.id.starts_with("HYP-CC-"))
            .count();

        // FE:5 + BR:5 + CE:4 + BE:4 = 18 discoveries (FE has 1 hyp counted separately)
        assert_eq!(disc, 18, "18 cross-cluster discovery nodes");
        assert_eq!(exp, 5, "5 experiment nodes");
        // HYP-CC-01 (in FUSION_ENV) + HYP-CC-02 + HYP-CC-03 = 3
        assert_eq!(hyp, 3, "3 cross-cluster hypothesis nodes");
    }

    // ── Confidence constraints ──

    #[test]
    fn test_hypothesis_low_confidence() {
        for cluster in ALL_CLUSTERS {
            for e in *cluster {
                if e.id.starts_with("HYP-") {
                    assert!(
                        e.confidence < 0.70,
                        "Hypothesis {} confidence {} should be < 0.70",
                        e.id, e.confidence
                    );
                }
            }
        }
    }

    #[test]
    fn test_experiment_moderate_confidence() {
        for e in EXPERIMENTS {
            assert!(
                e.confidence >= 0.60 && e.confidence <= 0.80,
                "Experiment {} confidence {} should be in [0.60, 0.80]",
                e.id, e.confidence
            );
        }
    }

    #[test]
    fn test_discovery_high_confidence() {
        for cluster in &[FUSION_ENV as &[CCEntry], BATTERY_ROBO, CHIP_ENV, BIO_ENV] {
            for e in *cluster {
                if e.node_type == NodeType::Discovery {
                    assert!(
                        e.confidence >= 0.75,
                        "Discovery {} confidence {} should be >= 0.75",
                        e.id, e.confidence
                    );
                }
            }
        }
    }

    // ── Data integrity ──

    #[test]
    fn test_no_duplicate_cc_ids() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique (including cross-cluster)");
    }

    #[test]
    fn test_all_cc_source_bts_exist() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        for cluster in ALL_CLUSTERS {
            for entry in *cluster {
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
    }

    #[test]
    fn test_edge_strength_valid() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} strength {} out of range",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_full_graph_with_cross_cluster() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        // Previous: 270+ nodes, now +26 = 296+
        assert!(
            g.nodes.len() >= 296,
            "Full graph with cross-cluster should have 296+ nodes, got {}",
            g.nodes.len()
        );
    }

    // ── Multi-cluster connectivity ──

    #[test]
    fn test_four_way_cluster_bridge() {
        let mut g = full_graph();
        populate_all_cross_cluster(&mut g);

        // Fusion-Env nodes exist
        assert!(g.nodes.iter().any(|n| n.id.starts_with("DISC-FE-")));
        // Battery-Robo nodes exist
        assert!(g.nodes.iter().any(|n| n.id.starts_with("DISC-BR-")));
        // Chip-Env nodes exist
        assert!(g.nodes.iter().any(|n| n.id.starts_with("DISC-CE-")));
        // Bio-Env nodes exist
        assert!(g.nodes.iter().any(|n| n.id.starts_with("DISC-BE-")));

        // All four cluster types should have Merges edges between them
        let fe_to_be = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from.starts_with("DISC-FE-") && e.to.starts_with("DISC-BE-"))
                    || (e.from.starts_with("DISC-BE-") && e.to.starts_with("DISC-FE-")))
        });
        let br_to_ce = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from.starts_with("DISC-BR-") && e.to.starts_with("DISC-CE-"))
                    || (e.from.starts_with("DISC-CE-") && e.to.starts_with("DISC-BR-")))
        });
        assert!(fe_to_be, "Fusion-Env and Bio-Env clusters should be interconnected");
        assert!(br_to_ce, "Battery-Robo and Chip-Env clusters should be interconnected");
    }
}
