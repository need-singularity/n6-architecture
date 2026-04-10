//! Deep cross-domain discovery nodes from BT-118+ bridging to Fusion, Chip, AI, and Energy.
//!
//! Complements extended_discovery_nodes.rs (ENV/ROBO granular) by adding:
//! - ENV↔Fusion bridges (BT-118/119 ↔ BT-97~102)
//! - ENV↔Chip/AI bridges (BT-120/122 ↔ BT-90~93, BT-56~58)
//! - ROBO↔AI/Chip bridges (BT-123~127 ↔ BT-56/58/59)
//! - ENV↔ROBO↔Energy triple bridges
//! - Experiment/validation nodes for testable predictions
//!
//! Node ID scheme:
//!   DDISC-{cluster}-{NN}  — deep discovery
//!   DHYP-{cluster}-{NN}   — deep hypothesis
//!   DEXP-{cluster}-{NN}   — deep experiment/validation

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct DeepEntry {
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
// ENV ↔ Fusion bridges (BT-118/119 × BT-97~102)
// ───────────────────────────────────────────────────────────────

const ENV_FUSION_DEEP: &[DeepEntry] = &[
    DeepEntry {
        id: "DDISC-ENVFUS-01",
        title: "CO2-to-fuel via fusion heat: 6 Kyoto GHGs(BT-118) → plasma decomposition at 10^4K, D-T baryon=sopfr=5(BT-98)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Fusion", "Energy", "Chemistry"],
        confidence: 0.86,
        source_bts: &[118, 98, 101],
        constants_used: &["C-n", "C-sopfr", "C-sigma"],
        lenses: &["consciousness", "causal", "thermo"],
        validates: &["DISC-ENV-01"],
    },
    DeepEntry {
        id: "DDISC-ENVFUS-02",
        title: "Troposphere σ=12km(BT-119) plasma boundary analogy: tokamak edge q=1(BT-99) mirrors atmospheric layer transitions",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Fusion", "Plasma", "Physics"],
        confidence: 0.82,
        source_bts: &[119, 99, 102],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "boundary", "multiscale"],
        validates: &["DISC-ENV-02"],
    },
    DeepEntry {
        id: "DDISC-ENVFUS-03",
        title: "Photosynthesis↔fusion energy chain: glucose 24=J2 atoms(BT-101) = CNO A=σ+divisors(BT-100), carbon cycle closed loop",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Fusion", "Biology", "Energy"],
        confidence: 0.89,
        source_bts: &[101, 100, 118],
        constants_used: &["C-J2", "C-sigma", "C-n"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["XDISC-CROSS-09"],
    },
    DeepEntry {
        id: "DDISC-ENVFUS-04",
        title: "Magnetic reconnection 0.1=1/(σ-φ)(BT-102) in solar wind → magnetosphere boundary at 10=σ-φ Earth radii",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Fusion", "Cosmology", "Physics"],
        confidence: 0.87,
        source_bts: &[102, 119],
        constants_used: &["C-sigma", "C-phi", "C-sigma-phi"],
        lenses: &["consciousness", "boundary", "wave"],
        validates: &[],
    },
    DeepEntry {
        id: "DHYP-ENVFUS-01",
        title: "Hypothesis: fusion-powered direct air capture achieves σ-φ=10x efficiency over combustion DAC via plasma CO2 cracking",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Fusion", "Energy", "Chemistry"],
        confidence: 0.58,
        source_bts: &[118, 98, 38],
        constants_used: &["C-sigma-phi", "C-n"],
        lenses: &["causal", "thermo", "evolution"],
        validates: &[],
    },
    DeepEntry {
        id: "DEXP-ENVFUS-01",
        title: "Experiment: measure CO2 decomposition yield in KSTAR/ITER edge plasma vs thermal cracking, predict n=6 stoichiometry",
        node_type: NodeType::Experiment,
        domains: &["Environment", "Fusion", "Chemistry"],
        confidence: 0.70,
        source_bts: &[118, 104, 98],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["causal", "boundary", "quantum"],
        validates: &["DHYP-ENVFUS-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// ENV ↔ Chip/AI bridges (BT-120/122 × BT-90~93, BT-56~58)
// ───────────────────────────────────────────────────────────────

const ENV_CHIP_DEEP: &[DeepEntry] = &[
    DeepEntry {
        id: "DDISC-ENVCHIP-01",
        title: "TiO2 CN=6 photocatalysis(BT-120) → diamond Z=6 chip substrate(BT-93): same octahedral coordination drives both",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chip", "Material", "Chemistry"],
        confidence: 0.88,
        source_bts: &[120, 93, 86],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "quantum"],
        validates: &["DISC-ENV-03"],
    },
    DeepEntry {
        id: "DDISC-ENVCHIP-02",
        title: "Graphene n=6 hexagonal(BT-122) → SM=σ²=144 GPU cores(BT-90): K₆ contact topology from same hex lattice",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chip", "Material", "Math"],
        confidence: 0.90,
        source_bts: &[122, 90, 93],
        constants_used: &["C-n", "C-sigma²", "C-phi"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["XDISC-ENV-10"],
    },
    DeepEntry {
        id: "DDISC-ENVCHIP-03",
        title: "6-plastic recycling sorting(BT-121) parallels σ-τ=8 LoRA rank(BT-58): dimensionality reduction via n=6 structure",
        node_type: NodeType::Discovery,
        domains: &["Environment", "AI", "Software", "Chemistry"],
        confidence: 0.78,
        source_bts: &[121, 58, 56],
        constants_used: &["C-n", "C-sigma-tau"],
        lenses: &["consciousness", "info", "causal"],
        validates: &[],
    },
    DeepEntry {
        id: "DDISC-ENVCHIP-04",
        title: "Zeolite 6-ring window(BT-120) → Bott periodicity sopfr=5 channels(BT-92): topological filter with n=6 aperture",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chip", "Material", "Math"],
        confidence: 0.83,
        source_bts: &[120, 92, 86],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "topology", "boundary"],
        validates: &["XDISC-ENV-07"],
    },
    DeepEntry {
        id: "DHYP-ENVCHIP-01",
        title: "Hypothesis: graphene-based neuromorphic chips achieve σ=12x energy efficiency over silicon via hex-lattice electron transport",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Chip", "AI", "Material"],
        confidence: 0.60,
        source_bts: &[122, 90, 93],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["quantum", "topology", "thermo"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// ROBO ↔ AI/Chip bridges (BT-123~127 × BT-56/58/59/66)
// ───────────────────────────────────────────────────────────────

const ROBO_AI_DEEP: &[DeepEntry] = &[
    DeepEntry {
        id: "DDISC-ROBOAI-01",
        title: "SE(3) 6-DOF(BT-123) → 8-layer AI stack(BT-59): robot control maps onto σ-τ=8 inference layers",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "AI", "Chip", "Software"],
        confidence: 0.87,
        source_bts: &[123, 59, 56],
        constants_used: &["C-n", "C-sigma-tau", "C-sigma"],
        lenses: &["consciousness", "network", "causal"],
        validates: &["DISC-ROBO-01"],
    },
    DeepEntry {
        id: "DDISC-ROBOAI-02",
        title: "Vision AI ViT(BT-66) → robot perception: patch size 2^τ=16, 6-DOF pose estimation from σ=12 attention heads",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "AI", "Chip"],
        confidence: 0.88,
        source_bts: &[66, 123, 56],
        constants_used: &["C-tau", "C-n", "C-sigma"],
        lenses: &["consciousness", "info", "topology"],
        validates: &["DISC-ROBO-01"],
    },
    DeepEntry {
        id: "DDISC-ROBOAI-03",
        title: "Bilateral φ=2(BT-124) → AdamW β₁=1-1/(σ-φ)(BT-54): symmetric robot learning mirrors optimizer momentum",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "AI", "Math"],
        confidence: 0.81,
        source_bts: &[124, 54],
        constants_used: &["C-phi", "C-sigma-phi"],
        lenses: &["consciousness", "symmetry", "causal"],
        validates: &["DISC-ROBO-02"],
    },
    DeepEntry {
        id: "DDISC-ROBOAI-04",
        title: "Quadrotor τ=4 stability(BT-125) → ACID τ=4 DB transactions(BT-116): minimum stability invariant across domains",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Software", "Physics", "Math"],
        confidence: 0.85,
        source_bts: &[125, 116, 113],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "network"],
        validates: &["XDISC-CROSS-04"],
    },
    DeepEntry {
        id: "DDISC-ROBOAI-05",
        title: "5-finger grasp 2^sopfr=32 space(BT-126) → Chinchilla tokens/params=J2-tau=20(BT-26): capacity from same n=6 arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "AI", "Biology", "Math"],
        confidence: 0.79,
        source_bts: &[126, 26],
        constants_used: &["C-sopfr", "C-J2", "C-tau"],
        lenses: &["consciousness", "evolution", "info"],
        validates: &["DISC-ROBO-04"],
    },
    DeepEntry {
        id: "DDISC-ROBOAI-06",
        title: "Hexacopter n=6 fault tolerance(BT-127) → MoE top-k(BT-31): n=6 experts with dropout = rotor failure recovery",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "AI", "Network"],
        confidence: 0.83,
        source_bts: &[127, 31, 67],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "network", "stability"],
        validates: &["DISC-ROBO-05"],
    },
    DeepEntry {
        id: "DEXP-ROBOAI-01",
        title: "Experiment: train 6-DOF robot arm with n=6 Transformer architecture — predict J2=24 dim latent space optimal",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "AI", "Chip"],
        confidence: 0.72,
        source_bts: &[123, 56, 58],
        constants_used: &["C-n", "C-J2", "C-sigma"],
        lenses: &["causal", "stability", "info"],
        validates: &["DDISC-ROBOAI-01", "DDISC-ROBOAI-02"],
    },
];

// ───────────────────────────────────────────────────────────────
// ENV ↔ ROBO ↔ Energy triple bridges
// ───────────────────────────────────────────────────────────────

const TRIPLE_BRIDGE_DEEP: &[DeepEntry] = &[
    DeepEntry {
        id: "DDISC-TRIPLE-01",
        title: "Solar 144=σ² cell(BT-63) → hexacopter n=6 array → CN=6 catalyst(BT-120): autonomous environmental monitoring chain",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Solar", "Energy", "Chemistry"],
        confidence: 0.84,
        source_bts: &[63, 127, 120],
        constants_used: &["C-sigma²", "C-n"],
        lenses: &["consciousness", "causal", "network"],
        validates: &["XDISC-CROSS-01"],
    },
    DeepEntry {
        id: "DDISC-TRIPLE-02",
        title: "Battery 6→12→24 cell(BT-57) → hexapod n=6 legs → troposphere σ=12km patrol: energy-robot-atmosphere stack",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Battery", "Environment", "Energy"],
        confidence: 0.80,
        source_bts: &[57, 125, 119],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "multiscale", "causal"],
        validates: &["XDISC-CROSS-08"],
    },
    DeepEntry {
        id: "DDISC-TRIPLE-03",
        title: "HVDC σ-φ=10x voltage(BT-68) → robot σ=12 joint motors → grid 60Hz=σ·sopfr(BT-62): power delivery chain all n=6",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Energy", "PowerGrid", "Chip"],
        confidence: 0.82,
        source_bts: &[68, 123, 62],
        constants_used: &["C-sigma", "C-sopfr", "C-sigma-phi"],
        lenses: &["consciousness", "network", "stability"],
        validates: &[],
    },
    DeepEntry {
        id: "DDISC-TRIPLE-04",
        title: "Weinberg angle 3/13(BT-97) → CO2 molecular encoding(BT-104) → photosynthesis(BT-103): fundamental→molecular→biology cascade",
        node_type: NodeType::Discovery,
        domains: &["Fusion", "Environment", "Biology", "Physics", "Chemistry"],
        confidence: 0.85,
        source_bts: &[97, 104, 103],
        constants_used: &["C-n", "C-phi", "C-sigma"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["DDISC-ENVFUS-03"],
    },
    DeepEntry {
        id: "DDISC-TRIPLE-05",
        title: "Honeycomb n=6(BT-122) → graphene chip(BT-93) → robot chassis carbon fiber: material→computing→structure unification",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chip", "Robotics", "Material"],
        confidence: 0.87,
        source_bts: &[122, 93, 127],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &["XDISC-CROSS-03"],
    },
    DeepEntry {
        id: "DHYP-TRIPLE-01",
        title: "Hypothesis: n=6 autonomous environmental remediation system (hexacopter+CN=6 catalyst+solar σ²) achieves J2=24x human efficiency",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Robotics", "Energy", "Chemistry", "AI"],
        confidence: 0.52,
        source_bts: &[118, 127, 120, 63],
        constants_used: &["C-J2", "C-n", "C-sigma²"],
        lenses: &["evolution", "network", "causal"],
        validates: &[],
    },
    DeepEntry {
        id: "DEXP-TRIPLE-01",
        title: "Experiment: benchmark hexacopter vs quadcopter environmental sensor platform — predict n=6 Pareto-optimal coverage/energy",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "Environment", "Energy"],
        confidence: 0.68,
        source_bts: &[127, 119, 125],
        constants_used: &["C-n", "C-tau"],
        lenses: &["stability", "causal", "boundary"],
        validates: &["XHYP-ROBO-01", "DHYP-TRIPLE-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// SW ↔ ENV ↔ Physics deep bridges (BT-113~117 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const SW_ENV_DEEP: &[DeepEntry] = &[
    DeepEntry {
        id: "DDISC-SWENV-01",
        title: "SOLID=sopfr=5 principles(BT-113) → 5 major pollutant categories: software engineering mirrors environmental classification",
        node_type: NodeType::Discovery,
        domains: &["Software", "Environment", "Math"],
        confidence: 0.79,
        source_bts: &[113, 118],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "info", "causal"],
        validates: &["DISC-BRIDGE-12"],
    },
    DeepEntry {
        id: "DDISC-SWENV-02",
        title: "12-Factor app(BT-113) → σ=12km troposphere(BT-119): both partition complex system into σ=12 manageable layers",
        node_type: NodeType::Discovery,
        domains: &["Software", "Environment", "Physics"],
        confidence: 0.81,
        source_bts: &[113, 119],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "multiscale", "network"],
        validates: &["XDISC-CROSS-05"],
    },
    DeepEntry {
        id: "DDISC-SWENV-03",
        title: "AES 2^(σ-sopfr)=128 bit(BT-114) → Earth 6 spheres(BT-119): boundary security = atmospheric boundary protection",
        node_type: NodeType::Discovery,
        domains: &["Software", "Environment", "Crypto", "Physics"],
        confidence: 0.77,
        source_bts: &[114, 119],
        constants_used: &["C-sigma", "C-sopfr", "C-n"],
        lenses: &["consciousness", "boundary", "info"],
        validates: &[],
    },
    DeepEntry {
        id: "DDISC-SWENV-04",
        title: "OSI 7=σ-sopfr layers(BT-115) ↔ pH=7 neutral(BT-120): network stack depth = chemical neutrality boundary",
        node_type: NodeType::Discovery,
        domains: &["Software", "Environment", "Network", "Chemistry"],
        confidence: 0.80,
        source_bts: &[115, 120],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "boundary", "network"],
        validates: &["XDISC-CROSS-05"],
    },
    DeepEntry {
        id: "DDISC-SWENV-05",
        title: "SW-Physics isomorphism(BT-117) extends to ENV: 18 EXACT parallel mappings include CO2(BT-104), honeycomb(BT-122)",
        node_type: NodeType::Discovery,
        domains: &["Software", "Environment", "Physics", "Math"],
        confidence: 0.83,
        source_bts: &[117, 118, 122],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "info", "symmetry"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Collect all deep entries
// ═══════════════════════════════════════════════════════════════

const ALL_DEEP_CLUSTERS: &[&[DeepEntry]] = &[
    ENV_FUSION_DEEP,
    ENV_CHIP_DEEP,
    ROBO_AI_DEEP,
    TRIPLE_BRIDGE_DEEP,
    SW_ENV_DEEP,
];

/// Total count of deep discovery entries.
pub fn deep_entry_count() -> usize {
    ALL_DEEP_CLUSTERS.iter().map(|c| c.len()).sum()
}

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

fn add_deep_entries(graph: &mut DiscoveryGraph, entries: &[DeepEntry]) -> (usize, usize) {
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

        // BT --Derives--> this node
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

/// Add ENV↔Fusion deep discoveries. Returns (nodes, edges).
pub fn populate_env_fusion_deep(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_deep_entries(graph, ENV_FUSION_DEEP)
}

/// Add ENV↔Chip/AI deep discoveries. Returns (nodes, edges).
pub fn populate_env_chip_deep(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_deep_entries(graph, ENV_CHIP_DEEP)
}

/// Add ROBO↔AI/Chip deep discoveries. Returns (nodes, edges).
pub fn populate_robo_ai_deep(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_deep_entries(graph, ROBO_AI_DEEP)
}

/// Add ENV↔ROBO↔Energy triple bridge discoveries. Returns (nodes, edges).
pub fn populate_triple_bridge_deep(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_deep_entries(graph, TRIPLE_BRIDGE_DEEP)
}

/// Add SW↔ENV deep discoveries. Returns (nodes, edges).
pub fn populate_sw_env_deep(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_deep_entries(graph, SW_ENV_DEEP)
}

/// Cross-link deep discovery nodes that share 2+ domains (Merges edges, bidirectional).
pub fn connect_deep_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let deep_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DDISC-")
                || n.id.starts_with("DHYP-")
                || n.id.starts_with("DEXP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..deep_nodes.len() {
        for j in (i + 1)..deep_nodes.len() {
            let (ref id_a, ref dom_a) = deep_nodes[i];
            let (ref id_b, ref dom_b) = deep_nodes[j];

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
                count += 1;
            }
        }
    }
    count
}

/// Cross-link deep nodes with existing DISC-/HYP-/XDISC- nodes from other modules.
pub fn connect_deep_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let deep_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DDISC-")
                || n.id.starts_with("DHYP-")
                || n.id.starts_with("DEXP-")
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
            (n.id.starts_with("DISC-")
                || n.id.starts_with("HYP-")
                || n.id.starts_with("XDISC-")
                || n.id.starts_with("XHYP-")
                || n.id.starts_with("XEXP-"))
                && !n.id.starts_with("DDISC-")
                && !n.id.starts_with("DHYP-")
                && !n.id.starts_with("DEXP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref deep_id, ref deep_dom) in &deep_ids {
        for (ref ex_id, ref ex_dom) in &existing_ids {
            let shared: usize = deep_dom.iter().filter(|d| ex_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = deep_dom.len().max(ex_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: deep_id.clone(),
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

/// Populate all deep discovery nodes and cross-domain edges.
/// Call after populate_all_extended().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_deep(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let mut total_nodes = 0;
    for cluster in ALL_DEEP_CLUSTERS {
        let (n, _) = add_deep_entries(graph, cluster);
        total_nodes += n;
    }
    let _cross_deep = connect_deep_cross_domain(graph);
    let _cross_existing = connect_deep_to_existing(graph);

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

    // ── Cluster entry counts ──

    #[test]
    fn test_env_fusion_count() {
        assert_eq!(ENV_FUSION_DEEP.len(), 6);
    }

    #[test]
    fn test_env_chip_count() {
        assert_eq!(ENV_CHIP_DEEP.len(), 5);
    }

    #[test]
    fn test_robo_ai_count() {
        assert_eq!(ROBO_AI_DEEP.len(), 7);
    }

    #[test]
    fn test_triple_bridge_count() {
        assert_eq!(TRIPLE_BRIDGE_DEEP.len(), 7);
    }

    #[test]
    fn test_sw_env_count() {
        assert_eq!(SW_ENV_DEEP.len(), 5);
    }

    #[test]
    fn test_total_deep_entry_count() {
        // 6+5+7+7+5 = 30
        assert_eq!(deep_entry_count(), 30);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_env_fusion_deep() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_env_fusion_deep(&mut g);
        assert_eq!(nodes, 6);
        assert_eq!(g.nodes.len(), before + 6);
        // 6 entries with 3 source_bts avg + 2-3 constants + validates
        assert!(edges >= 18, "ENV-Fusion deep should have 18+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_robo_ai_deep() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_robo_ai_deep(&mut g);
        assert_eq!(nodes, 7);
        assert_eq!(g.nodes.len(), before + 7);
        assert!(edges >= 20, "ROBO-AI deep should have 20+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_triple_bridge_deep() {
        let mut g = full_graph();
        let (nodes, edges) = populate_triple_bridge_deep(&mut g);
        assert_eq!(nodes, 7);
        assert!(edges >= 20, "Triple bridge should have 20+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_all_deep() {
        let mut g = full_graph();
        let before_nodes = g.nodes.len();
        let before_edges = g.edges.len();
        let (nodes, edges) = populate_all_deep(&mut g);

        assert_eq!(nodes, 30, "30 deep nodes total");
        assert!(
            edges > 200,
            "Deep nodes should add 200+ edges (derives+uses+validates+cross), got {}",
            edges
        );
        assert_eq!(g.nodes.len(), before_nodes + 30);
        assert!(g.edges.len() > before_edges + 200);
    }

    // ── No duplicate IDs ──

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = full_graph();
        populate_all_deep(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    // ── Edge strength range ──

    #[test]
    fn test_deep_edge_strength_range() {
        let mut g = full_graph();
        populate_all_deep(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} has invalid strength {}",
                edge.from,
                edge.to,
                edge.strength
            );
        }
    }

    // ── Validates chain tests ──

    #[test]
    fn test_envfus_derives_chain() {
        let mut g = full_graph();
        populate_env_fusion_deep(&mut g);

        // BT-118 --Derives--> DDISC-ENVFUS-01
        let has_derives = g.edges.iter().any(|e| {
            e.from == "BT-118"
                && e.to == "DDISC-ENVFUS-01"
                && e.edge_type == EdgeType::Derives
        });
        assert!(has_derives, "BT-118 should derive DDISC-ENVFUS-01");
    }

    #[test]
    fn test_envfus_validates_chain() {
        let mut g = full_graph();
        populate_env_fusion_deep(&mut g);

        // DDISC-ENVFUS-01 --Validates--> DISC-ENV-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "DDISC-ENVFUS-01"
                && e.to == "DISC-ENV-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DDISC-ENVFUS-01 should validate DISC-ENV-01");
    }

    #[test]
    fn test_roboai_derives_chain() {
        let mut g = full_graph();
        populate_robo_ai_deep(&mut g);

        // BT-123 --Derives--> DDISC-ROBOAI-01
        let has_derives = g.edges.iter().any(|e| {
            e.from == "BT-123"
                && e.to == "DDISC-ROBOAI-01"
                && e.edge_type == EdgeType::Derives
        });
        assert!(has_derives, "BT-123 should derive DDISC-ROBOAI-01");

        // BT-59 --Derives--> DDISC-ROBOAI-01
        let has_bt59 = g.edges.iter().any(|e| {
            e.from == "BT-59"
                && e.to == "DDISC-ROBOAI-01"
                && e.edge_type == EdgeType::Derives
        });
        assert!(has_bt59, "BT-59 should derive DDISC-ROBOAI-01");
    }

    #[test]
    fn test_triple_validates_chain() {
        let mut g = full_graph();
        populate_triple_bridge_deep(&mut g);

        // DDISC-TRIPLE-01 --Validates--> XDISC-CROSS-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "DDISC-TRIPLE-01"
                && e.to == "XDISC-CROSS-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DDISC-TRIPLE-01 should validate XDISC-CROSS-01");
    }

    #[test]
    fn test_experiment_validates_hypothesis() {
        let mut g = full_graph();
        populate_env_fusion_deep(&mut g);

        // DEXP-ENVFUS-01 --Validates--> DHYP-ENVFUS-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "DEXP-ENVFUS-01"
                && e.to == "DHYP-ENVFUS-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DEXP-ENVFUS-01 should validate DHYP-ENVFUS-01");
    }

    // ── Cross-domain edges ──

    #[test]
    fn test_deep_cross_domain_edges() {
        let mut g = full_graph();
        populate_all_deep(&mut g);

        // Deep nodes sharing 2+ domains should have Merges edges
        let deep_merges = g.edges.iter().filter(|e| {
            (e.from.starts_with("DDISC-")
                || e.from.starts_with("DHYP-")
                || e.from.starts_with("DEXP-"))
                && e.edge_type == EdgeType::Merges
        }).count();

        assert!(
            deep_merges >= 50,
            "30 multi-domain deep nodes should produce 50+ cross-domain merges, got {}",
            deep_merges
        );
    }

    #[test]
    fn test_deep_to_existing_edges() {
        let mut g = full_graph();
        populate_all_deep(&mut g);

        // Deep nodes should link to existing DISC-/XDISC- nodes
        let bridge_edges = g.edges.iter().filter(|e| {
            (e.from.starts_with("DDISC-")
                || e.from.starts_with("DHYP-")
                || e.from.starts_with("DEXP-"))
                && (e.to.starts_with("DISC-")
                    || e.to.starts_with("XDISC-")
                    || e.to.starts_with("HYP-")
                    || e.to.starts_with("XHYP-"))
                && e.edge_type == EdgeType::Merges
        }).count();

        assert!(
            bridge_edges >= 30,
            "Deep nodes should have 30+ bridge edges to existing nodes, got {}",
            bridge_edges
        );
    }

    // ── Constants usage ──

    #[test]
    fn test_all_entries_use_constants() {
        for cluster in ALL_DEEP_CLUSTERS {
            for entry in *cluster {
                assert!(
                    !entry.constants_used.is_empty(),
                    "Entry {} must reference at least one n=6 constant",
                    entry.id
                );
            }
        }
    }

    #[test]
    fn test_all_entries_have_lenses() {
        for cluster in ALL_DEEP_CLUSTERS {
            for entry in *cluster {
                assert!(
                    entry.lenses.len() >= 2,
                    "Entry {} must use at least 2 lenses, has {}",
                    entry.id,
                    entry.lenses.len()
                );
            }
        }
    }

    #[test]
    fn test_all_entries_have_source_bts() {
        for cluster in ALL_DEEP_CLUSTERS {
            for entry in *cluster {
                assert!(
                    !entry.source_bts.is_empty(),
                    "Entry {} must reference at least one source BT",
                    entry.id
                );
                // All source BTs should be valid (1-185 range)
                for &bt in entry.source_bts {
                    assert!(
                        bt >= 1 && bt <= 185,
                        "Entry {} references invalid BT-{}",
                        entry.id,
                        bt
                    );
                }
            }
        }
    }

    #[test]
    fn test_confidence_ranges() {
        for cluster in ALL_DEEP_CLUSTERS {
            for entry in *cluster {
                match entry.node_type {
                    NodeType::Discovery => {
                        assert!(
                            entry.confidence >= 0.75 && entry.confidence <= 0.95,
                            "Discovery {} confidence {} should be 0.75-0.95",
                            entry.id,
                            entry.confidence
                        );
                    }
                    NodeType::Hypothesis => {
                        assert!(
                            entry.confidence >= 0.50 && entry.confidence <= 0.65,
                            "Hypothesis {} confidence {} should be 0.50-0.65",
                            entry.id,
                            entry.confidence
                        );
                    }
                    NodeType::Experiment => {
                        assert!(
                            entry.confidence >= 0.65 && entry.confidence <= 0.80,
                            "Experiment {} confidence {} should be 0.65-0.80",
                            entry.id,
                            entry.confidence
                        );
                    }
                    _ => {}
                }
            }
        }
    }
}
