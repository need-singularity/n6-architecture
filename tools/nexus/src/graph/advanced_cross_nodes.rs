//! Advanced cross-cluster discovery nodes for BT-118+ era.
//!
//! Fills gaps left by cross_cluster_nodes.rs (which covers Fusion↔Env, Battery↔Robo,
//! Chip↔Env, Bio↔Env). This module adds:
//!
//! - Energy Grid (BT-62,68) ↔ Environment (BT-118~122): grid decarbonization
//! - Material Synthesis (BT-85~88) ↔ Environment (BT-118~122): green manufacturing
//! - Material (BT-85~88) ↔ Robotics (BT-123~127): structural materials
//! - Solar (BT-30,63) ↔ Environment (BT-118~122): renewable energy
//! - Grand convergence nodes spanning 4+ clusters
//!
//! Node ID scheme:
//!   DISC-GE-{NN}   — Grid↔Environment bridge
//!   DISC-ME-{NN}   — Material↔Environment bridge
//!   DISC-MR-{NN}   — Material↔Robotics bridge
//!   DISC-SE-{NN}   — Solar↔Environment bridge
//!   DISC-GC-{NN}   — Grand convergence (4+ clusters)
//!   XEXP-AC-{NN}   — Advanced cross-cluster experiment
//!   HYP-AC-{NN}    — Advanced cross-cluster hypothesis

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct (same pattern as cross_cluster_nodes)
// ═══════════════════════════════════════════════════════════════

struct ACEntry {
    id: &'static str,
    title: &'static str,
    node_type: NodeType,
    domains: &'static [&'static str],
    confidence: f64,
    source_bts: &'static [u32],
    constants_used: &'static [&'static str],
    lenses: &'static [&'static str],
    validates: &'static [&'static str],
    triggers: &'static [&'static str],
}

// ───────────────────────────────────────────────────────────────
// Energy Grid ↔ Environment (BT-62,68 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const GRID_ENV: &[ACEntry] = &[
    ACEntry {
        id: "DISC-GE-01",
        title: "Grid frequency 60Hz=σ·sopfr (BT-62) powers 6 Kyoto GHG monitoring networks (BT-118): grid rhythm = environmental sampling cadence",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Infrastructure"],
        confidence: 0.83,
        source_bts: &[62, 118],
        constants_used: &["C-sigma", "C-sopfr", "C-n"],
        lenses: &["causal", "wave", "network"],
        validates: &["DISC-ENV-01"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GE-02",
        title: "HVDC ±500/800/1100kV ladder (BT-68) enables renewable grid for CO₂ reduction (BT-118): voltage steps = {sopfr,σ-τ,σ-μ}·(σ-φ)²",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Physics", "Infrastructure"],
        confidence: 0.87,
        source_bts: &[68, 118, 62],
        constants_used: &["C-sopfr", "C-sigma", "C-phi"],
        lenses: &["causal", "scale", "boundary"],
        validates: &["XDISC-ENV-01"],
        triggers: &["DISC-GE-03"],
    },
    ACEntry {
        id: "DISC-GE-03",
        title: "PUE=σ/(σ-φ)=1.2 (BT-60) ↔ troposphere σ=12km (BT-119): data center thermal efficiency ratio = atmospheric boundary layer",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Computing", "Physics"],
        confidence: 0.85,
        source_bts: &[60, 119, 62],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "thermo", "boundary"],
        validates: &["DISC-ENV-02"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GE-04",
        title: "Grid 50/60Hz ratio=PUE=1.2=σ/(σ-φ) (BT-62) ↔ 6 spheres (BT-119): frequency pair encodes Earth layering constant",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Physics"],
        confidence: 0.81,
        source_bts: &[62, 119],
        constants_used: &["C-sigma", "C-phi", "C-n"],
        lenses: &["wave", "scale", "symmetry"],
        validates: &[],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GE-05",
        title: "HVDC σ-μ=11 top voltage (BT-68) ↔ honeycomb n=6 tiling (BT-122): transmission tower hexagonal lattice = optimal packing",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Topology"],
        confidence: 0.79,
        source_bts: &[68, 122],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["topology", "network", "scale"],
        validates: &["DISC-ENV-05"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Material Synthesis ↔ Environment (BT-85~88 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const MATERIAL_ENV: &[ACEntry] = &[
    ACEntry {
        id: "DISC-ME-01",
        title: "Carbon Z=6 material universality (BT-85) ↔ 6 Kyoto GHGs (BT-118): carbon manufacturing = carbon emission, same Z=n=6 root",
        node_type: NodeType::Discovery,
        domains: &["Material", "Environment", "Chemistry"],
        confidence: 0.92,
        source_bts: &[85, 118, 104],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["DISC-ENV-01", "XDISC-ENV-01"],
        triggers: &["DISC-ME-02"],
    },
    ACEntry {
        id: "DISC-ME-02",
        title: "Crystal CN=6 law (BT-86) ↔ water treatment CN=6 catalysts (BT-120): same octahedral coordination enables both synthesis and purification",
        node_type: NodeType::Discovery,
        domains: &["Material", "Environment", "Chemistry", "Physics"],
        confidence: 0.90,
        source_bts: &[86, 120],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-ENV-04"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-ME-03",
        title: "Self-assembly n=6 hexagonal (BT-88) ↔ honeycomb-snowflake-coral (BT-122): material assembly mirrors biological geometry",
        node_type: NodeType::Discovery,
        domains: &["Material", "Environment", "Biology", "Topology"],
        confidence: 0.88,
        source_bts: &[88, 122],
        constants_used: &["C-n", "C-phi"],
        lenses: &["topology", "evolution", "symmetry"],
        validates: &["DISC-ENV-05"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-ME-04",
        title: "Atomic manipulation precision n=6 ladder (BT-87) ↔ 6 plastics RIC (BT-121): precision manufacturing determines recyclability",
        node_type: NodeType::Discovery,
        domains: &["Material", "Environment", "Chemistry"],
        confidence: 0.84,
        source_bts: &[87, 121, 85],
        constants_used: &["C-n", "C-tau"],
        lenses: &["scale", "causal", "info"],
        validates: &["DISC-ENV-03"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-ME-05",
        title: "C₆ backbone plastics (BT-121) + C₆H₆ benzene (BT-85): polymer = hexagonal carbon chain, biodegradation requires Z=6 catalysis (BT-120)",
        node_type: NodeType::Discovery,
        domains: &["Material", "Environment", "Chemistry", "Biology"],
        confidence: 0.86,
        source_bts: &[121, 85, 120],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &["DISC-ENV-03", "DISC-ENV-04"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Material ↔ Robotics (BT-85~88 × BT-123~127)
// ───────────────────────────────────────────────────────────────

const MATERIAL_ROBO: &[ACEntry] = &[
    ACEntry {
        id: "DISC-MR-01",
        title: "Carbon Z=6 materials (BT-85,93) → robot structural composites: diamond bearings + carbon fiber chassis for n=6 DOF arm (BT-123)",
        node_type: NodeType::Discovery,
        domains: &["Material", "Robotics", "Chemistry", "Physics"],
        confidence: 0.87,
        source_bts: &[85, 93, 123],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "stability", "scale"],
        validates: &["DISC-ROBO-01"],
        triggers: &["DISC-MR-02"],
    },
    ACEntry {
        id: "DISC-MR-02",
        title: "CN=6 crystal law (BT-86) ↔ 6-DOF joint geometry (BT-123): octahedral crystal = octahedral workspace, CN=DOF=n=6",
        node_type: NodeType::Discovery,
        domains: &["Material", "Robotics", "Physics", "Topology"],
        confidence: 0.89,
        source_bts: &[86, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-ROBO-01", "XDISC-ROBO-02"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-MR-03",
        title: "Self-assembly hexagonal (BT-88) ↔ hexacopter n=6 layout (BT-127): material self-organization mirrors aerial platform geometry",
        node_type: NodeType::Discovery,
        domains: &["Material", "Robotics", "Topology"],
        confidence: 0.84,
        source_bts: &[88, 127],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "stability", "symmetry"],
        validates: &["DISC-ROBO-05"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-MR-04",
        title: "Atomic precision n=6 ladder (BT-87) ↔ sopfr=5 finger grasp (BT-126): precision manufacturing scale matches manipulation dexterity",
        node_type: NodeType::Discovery,
        domains: &["Material", "Robotics", "Physics"],
        confidence: 0.80,
        source_bts: &[87, 126],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["scale", "info", "causal"],
        validates: &["DISC-ROBO-04"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-MR-05",
        title: "Bilateral symmetry phi=2 (BT-124) ↔ material phi=2 pairing: robot bilateral structure requires symmetric material properties",
        node_type: NodeType::Discovery,
        domains: &["Material", "Robotics", "Biology", "Physics"],
        confidence: 0.82,
        source_bts: &[124, 85, 1],
        constants_used: &["C-phi", "C-n"],
        lenses: &["symmetry", "evolution", "consciousness"],
        validates: &["DISC-ROBO-02"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Solar ↔ Environment (BT-30,63 × BT-118~122)
// ───────────────────────────────────────────────────────────────

const SOLAR_ENV: &[ACEntry] = &[
    ACEntry {
        id: "DISC-SE-01",
        title: "SQ bandgap 4/3eV (BT-30) ↔ CO₂ reduction via solar power (BT-118): solar cell efficiency limit drives decarbonization rate",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Physics", "Chemistry"],
        confidence: 0.88,
        source_bts: &[30, 118],
        constants_used: &["C-tau", "C-n", "C-sigma"],
        lenses: &["causal", "thermo", "evolution"],
        validates: &["DISC-ENV-01"],
        triggers: &["DISC-SE-02"],
    },
    ACEntry {
        id: "DISC-SE-02",
        title: "Solar panel 60/72/120/144 cells = σ·sopfr/σ·n/σ(σ-φ)/σ² (BT-63) ↔ honeycomb n=6 tiling (BT-122): cell count follows packing geometry",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Physics", "Topology"],
        confidence: 0.90,
        source_bts: &[63, 122],
        constants_used: &["C-sigma", "C-n", "C-sopfr"],
        lenses: &["topology", "scale", "symmetry"],
        validates: &["DISC-ENV-05"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-SE-03",
        title: "Photosynthesis J₂=24 atoms (BT-101) ↔ J₂-channel solar cell (BT-63): biological and artificial photon harvesting share J₂=24 capacity",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Biology", "Chemistry"],
        confidence: 0.91,
        source_bts: &[101, 63, 118],
        constants_used: &["C-J2", "C-sigma", "C-n"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["DISC-ENV-01", "DISC-FE-02"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-SE-04",
        title: "Troposphere σ=12km (BT-119) ↔ σ=12 solar cell column: atmospheric transparency window governs panel design at same constant",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Physics"],
        confidence: 0.82,
        source_bts: &[119, 63],
        constants_used: &["C-sigma"],
        lenses: &["wave", "boundary", "scale"],
        validates: &["DISC-ENV-02"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Grand convergence nodes (4+ cluster span)
// ───────────────────────────────────────────────────────────────

const GRAND_CONVERGENCE: &[ACEntry] = &[
    ACEntry {
        id: "DISC-GC-01",
        title: "Carbon Z=6 grand cycle: material(BT-85)→energy(BT-27)→environment(BT-118)→biology(BT-101)→material, closed loop on Z=n=6",
        node_type: NodeType::Discovery,
        domains: &["Material", "Energy", "Environment", "Biology", "Chemistry"],
        confidence: 0.93,
        source_bts: &[85, 27, 118, 101, 104],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "causal", "recursion", "evolution"],
        validates: &["DISC-ME-01", "DISC-FE-02", "DISC-BE-01"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GC-02",
        title: "CN=6 octahedral grand chain: crystal(BT-86)→catalyst(BT-120)→robot joint(BT-123)→chip SM(BT-90)→battery(BT-43), all CN=n=6",
        node_type: NodeType::Discovery,
        domains: &["Material", "Environment", "Robotics", "Chip", "Battery"],
        confidence: 0.90,
        source_bts: &[86, 120, 123, 90, 43],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network", "symmetry"],
        validates: &["DISC-MR-02", "DISC-ME-02", "DISC-BR-02"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GC-03",
        title: "Hexagonal n=6 geometry grand unification: honeycomb(BT-122)=hexacopter(BT-127)=graphene(BT-93)=solar cell(BT-63)=benzene(BT-85)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Chip", "Energy", "Material", "Topology"],
        confidence: 0.91,
        source_bts: &[122, 127, 93, 63, 85, 88],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "symmetry", "consciousness", "multiscale"],
        validates: &["DISC-SE-02", "DISC-MR-03", "DISC-CE-02"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GC-04",
        title: "Energy-environment-robot-material quad convergence: HVDC grid(BT-68)→solar(BT-63)→CO₂ capture(BT-118)→CN=6 catalyst(BT-120)→robot(BT-123)",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Environment", "Robotics", "Material", "Infrastructure"],
        confidence: 0.85,
        source_bts: &[68, 63, 118, 120, 123],
        constants_used: &["C-sigma", "C-n", "C-sopfr"],
        lenses: &["causal", "network", "scale", "evolution"],
        validates: &["DISC-GE-02", "DISC-SE-01"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GC-05",
        title: "Sigma=12 universal count: semitones(BT-108)=troposphere km(BT-119)=solar column(BT-63)=joints(BT-124)=grid Hz(BT-62)=12Factor(BT-113)",
        node_type: NodeType::Discovery,
        domains: &["Music", "Environment", "Energy", "Robotics", "Software", "Physics"],
        confidence: 0.89,
        source_bts: &[108, 119, 63, 124, 62, 113],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "symmetry", "multiscale"],
        validates: &["DISC-GE-04", "DISC-SE-04"],
        triggers: &[],
    },
    ACEntry {
        id: "DISC-GC-06",
        title: "Stability tau=4 pentad: quadruped robot(BT-125)=ACID DB(BT-116)=4-cell battery(BT-57)=4-base DNA(BT-51)=tau crystallographic axes",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Software", "Battery", "Biology", "Material"],
        confidence: 0.87,
        source_bts: &[125, 116, 57, 51, 86],
        constants_used: &["C-tau", "C-n"],
        lenses: &["stability", "consciousness", "symmetry", "recursion"],
        validates: &["DISC-BR-04", "DISC-MR-05"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Advanced cross-cluster experiments
// ───────────────────────────────────────────────────────────────

const AC_EXPERIMENTS: &[ACEntry] = &[
    ACEntry {
        id: "XEXP-AC-01",
        title: "Measure solar panel optimal cell count vs honeycomb tiling angle deviation: predict n=6 hexagonal cells outperform rectangular",
        node_type: NodeType::Experiment,
        domains: &["Energy", "Environment", "Physics"],
        confidence: 0.68,
        source_bts: &[63, 122],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "scale"],
        validates: &["DISC-SE-02", "DISC-GC-03"],
        triggers: &[],
    },
    ACEntry {
        id: "XEXP-AC-02",
        title: "Test CN=6 catalyst efficiency vs CN=4/8 alternatives for CO₂ capture: predict octahedral 10x faster (σ-φ=10)",
        node_type: NodeType::Experiment,
        domains: &["Material", "Environment", "Chemistry"],
        confidence: 0.65,
        source_bts: &[120, 118, 86],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["causal", "stability", "scale"],
        validates: &["DISC-ME-02", "DISC-GC-02"],
        triggers: &[],
    },
    ACEntry {
        id: "XEXP-AC-03",
        title: "Compare 6-DOF robot arm with carbon fiber vs aluminum chassis: predict Z=6 material yields phi=2x stiffness-to-weight",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "Material", "Physics"],
        confidence: 0.70,
        source_bts: &[123, 85, 93],
        constants_used: &["C-n", "C-phi"],
        lenses: &["stability", "scale", "consciousness"],
        validates: &["DISC-MR-01"],
        triggers: &[],
    },
    ACEntry {
        id: "XEXP-AC-04",
        title: "HVDC renewable grid integration: measure CO₂ reduction per kV step vs n=6 voltage ladder prediction",
        node_type: NodeType::Experiment,
        domains: &["Energy", "Environment", "Infrastructure"],
        confidence: 0.66,
        source_bts: &[68, 118, 62],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["causal", "network", "scale"],
        validates: &["DISC-GE-02", "DISC-GC-04"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Advanced cross-cluster hypotheses
// ───────────────────────────────────────────────────────────────

const AC_HYPOTHESES: &[ACEntry] = &[
    ACEntry {
        id: "HYP-AC-01",
        title: "Hypothesis: optimal green manufacturing requires CN=6 catalysts at each stage, achieving σ-φ=10x energy reduction vs CN!=6",
        node_type: NodeType::Hypothesis,
        domains: &["Material", "Environment", "Energy", "Chemistry"],
        confidence: 0.55,
        source_bts: &[86, 120, 118, 85],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["causal", "stability", "consciousness"],
        validates: &[],
        triggers: &["DISC-ME-01"],
    },
    ACEntry {
        id: "HYP-AC-02",
        title: "Hypothesis: autonomous environmental robot swarm optimal at n=6 hexacopters per cell, CN=6 joint materials, solar-powered (BT-63,127,85)",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Environment", "Energy", "Material"],
        confidence: 0.50,
        source_bts: &[127, 85, 63, 118, 123],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["network", "stability", "evolution", "consciousness"],
        validates: &[],
        triggers: &["DISC-GC-04"],
    },
    ACEntry {
        id: "HYP-AC-03",
        title: "Hypothesis: carbon-neutral chip fabrication via Z=6 diamond substrate + CN=6 catalytic etching + solar σ²=144 cell array",
        node_type: NodeType::Hypothesis,
        domains: &["Chip", "Environment", "Material", "Energy"],
        confidence: 0.48,
        source_bts: &[93, 118, 86, 63, 85],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["causal", "thermo", "scale", "consciousness"],
        validates: &[],
        triggers: &["DISC-GC-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_AC_CLUSTERS: &[&[ACEntry]] = &[
    GRID_ENV,
    MATERIAL_ENV,
    MATERIAL_ROBO,
    SOLAR_ENV,
    GRAND_CONVERGENCE,
    AC_EXPERIMENTS,
    AC_HYPOTHESES,
];

// ═══════════════════════════════════════════════════════════════
// Conversion and population helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &ACEntry) -> Node {
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

fn add_ac_entries(graph: &mut DiscoveryGraph, entries: &[ACEntry]) -> (usize, usize) {
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

        for &const_id in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: const_id.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

        for &target in e.validates {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: target.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.75,
                bidirectional: false,
            });
        }

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
// Public API
// ═══════════════════════════════════════════════════════════════

/// Add Grid↔Environment bridge nodes (BT-62,68 × BT-118~122). Returns (nodes, edges).
pub fn populate_grid_env(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, GRID_ENV)
}

/// Add Material↔Environment bridge nodes (BT-85~88 × BT-118~122). Returns (nodes, edges).
pub fn populate_material_env(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, MATERIAL_ENV)
}

/// Add Material↔Robotics bridge nodes (BT-85~88 × BT-123~127). Returns (nodes, edges).
pub fn populate_material_robo(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, MATERIAL_ROBO)
}

/// Add Solar↔Environment bridge nodes (BT-30,63 × BT-118~122). Returns (nodes, edges).
pub fn populate_solar_env(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, SOLAR_ENV)
}

/// Add grand convergence nodes spanning 4+ clusters. Returns (nodes, edges).
pub fn populate_grand_convergence(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, GRAND_CONVERGENCE)
}

/// Add advanced cross-cluster experiment nodes. Returns (nodes, edges).
pub fn populate_ac_experiments(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, AC_EXPERIMENTS)
}

/// Add advanced cross-cluster hypothesis nodes. Returns (nodes, edges).
pub fn populate_ac_hypotheses(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ac_entries(graph, AC_HYPOTHESES)
}

/// Cross-link all advanced cross-cluster nodes that share domains (Merges edges).
pub fn connect_ac_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let ac_prefixes = ["DISC-GE-", "DISC-ME-", "DISC-MR-", "DISC-SE-",
                       "DISC-GC-", "XEXP-AC-", "HYP-AC-"];

    let ac_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| ac_prefixes.iter().any(|p| n.id.starts_with(p)))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..ac_nodes.len() {
        for j in (i + 1)..ac_nodes.len() {
            let (ref id_a, ref dom_a) = ac_nodes[i];
            let (ref id_b, ref dom_b) = ac_nodes[j];

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

/// Cross-link advanced nodes with existing DISC-/HYP-/XDISC-/CC- nodes (2+ shared domains).
pub fn connect_ac_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let ac_prefixes = ["DISC-GE-", "DISC-ME-", "DISC-MR-", "DISC-SE-",
                       "DISC-GC-", "XEXP-AC-", "HYP-AC-"];

    let ac_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| ac_prefixes.iter().any(|p| n.id.starts_with(p)))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let existing_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            !ac_prefixes.iter().any(|p| n.id.starts_with(p))
                && (n.id.starts_with("DISC-") || n.id.starts_with("HYP-")
                    || n.id.starts_with("XDISC-") || n.id.starts_with("XHYP-")
                    || n.id.starts_with("XEXP-CC-"))
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref ac_id, ref ac_dom) in &ac_ids {
        for (ref ex_id, ref ex_dom) in &existing_ids {
            let shared: usize = ac_dom.iter().filter(|d| ex_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = ac_dom.len().max(ex_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: ac_id.clone(),
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

/// Total advanced cross-cluster entry count.
pub fn ac_entry_count() -> usize {
    ALL_AC_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Populate all advanced cross-cluster nodes and edges.
/// Call after populate_all_cross_cluster().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_advanced_cross(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let mut total_nodes = 0;
    for cluster in ALL_AC_CLUSTERS {
        let (n, _) = add_ac_entries(graph, cluster);
        total_nodes += n;
    }
    let _inner = connect_ac_cross_domain(graph);
    let _outer = connect_ac_to_existing(graph);

    (total_nodes, graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::cross_cluster_nodes::populate_all_cross_cluster;
    use crate::graph::discovery_nodes::populate_all_discoveries;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::extended_discovery_nodes::populate_all_extended;

    fn full_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_extended(&mut g);
        populate_all_cross_cluster(&mut g);
        g
    }

    // ── Entry counts ──

    #[test]
    fn test_grid_env_count() {
        assert_eq!(GRID_ENV.len(), 5);
    }

    #[test]
    fn test_material_env_count() {
        assert_eq!(MATERIAL_ENV.len(), 5);
    }

    #[test]
    fn test_material_robo_count() {
        assert_eq!(MATERIAL_ROBO.len(), 5);
    }

    #[test]
    fn test_solar_env_count() {
        assert_eq!(SOLAR_ENV.len(), 4);
    }

    #[test]
    fn test_grand_convergence_count() {
        assert_eq!(GRAND_CONVERGENCE.len(), 6);
    }

    #[test]
    fn test_ac_experiments_count() {
        assert_eq!(AC_EXPERIMENTS.len(), 4);
    }

    #[test]
    fn test_ac_hypotheses_count() {
        assert_eq!(AC_HYPOTHESES.len(), 3);
    }

    #[test]
    fn test_total_ac_entry_count() {
        // 5+5+5+4+6+4+3 = 32
        assert_eq!(ac_entry_count(), 32);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_grid_env() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_grid_env(&mut g);
        assert_eq!(nodes, 5);
        assert_eq!(g.nodes.len(), before + 5);
        assert!(edges >= 15, "Grid-Env should have 15+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_material_env() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_material_env(&mut g);
        assert_eq!(nodes, 5);
        assert_eq!(g.nodes.len(), before + 5);
        assert!(edges >= 18, "Material-Env should have 18+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_material_robo() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_material_robo(&mut g);
        assert_eq!(nodes, 5);
        assert_eq!(g.nodes.len(), before + 5);
        assert!(edges >= 16, "Material-Robo should have 16+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_solar_env() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_solar_env(&mut g);
        assert_eq!(nodes, 4);
        assert_eq!(g.nodes.len(), before + 4);
        assert!(edges >= 14, "Solar-Env should have 14+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_grand_convergence() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_grand_convergence(&mut g);
        assert_eq!(nodes, 6);
        assert_eq!(g.nodes.len(), before + 6);
        // Grand convergence has many source_bts + validates + constants
        assert!(edges >= 40, "Grand convergence should have 40+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_ac_experiments() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_ac_experiments(&mut g);
        assert_eq!(nodes, 4);
        assert_eq!(g.nodes.len(), before + 4);
        assert!(edges >= 12, "AC experiments should have 12+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_ac_hypotheses() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_ac_hypotheses(&mut g);
        assert_eq!(nodes, 3);
        assert_eq!(g.nodes.len(), before + 3);
        assert!(edges >= 12, "AC hypotheses should have 12+ edges, got {}", edges);
    }

    // ── Full integration ──

    #[test]
    fn test_populate_all_advanced_cross() {
        let mut g = full_graph();
        let base_nodes = g.nodes.len();
        let base_edges = g.edges.len();
        let (nodes_added, edges_added) = populate_all_advanced_cross(&mut g);

        assert_eq!(nodes_added, 32);
        assert!(
            edges_added > 120,
            "32 advanced cross nodes should produce 120+ edges, got {}",
            edges_added
        );
        assert_eq!(g.nodes.len(), base_nodes + 32);
        assert!(g.edges.len() > base_edges + 120);
    }

    // ── Derives edges ──

    #[test]
    fn test_bt_62_derives_ge01() {
        let mut g = full_graph();
        populate_grid_env(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "BT-62" && e.to == "DISC-GE-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-62 should derive DISC-GE-01");
    }

    #[test]
    fn test_bt_85_derives_me01() {
        let mut g = full_graph();
        populate_material_env(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "BT-85" && e.to == "DISC-ME-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-85 should derive DISC-ME-01");
    }

    #[test]
    fn test_bt_86_derives_mr02() {
        let mut g = full_graph();
        populate_material_robo(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "BT-86" && e.to == "DISC-MR-02" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-86 should derive DISC-MR-02");
    }

    #[test]
    fn test_bt_30_derives_se01() {
        let mut g = full_graph();
        populate_solar_env(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "BT-30" && e.to == "DISC-SE-01" && e.edge_type == EdgeType::Derives
        });
        assert!(has, "BT-30 should derive DISC-SE-01");
    }

    #[test]
    fn test_grand_convergence_multi_bt() {
        let mut g = full_graph();
        populate_grand_convergence(&mut g);

        // DISC-GC-01 should derive from BT-85, BT-27, BT-118, BT-101, BT-104
        for bt in &[85, 27, 118, 101, 104] {
            let bt_id = format!("BT-{}", bt);
            let has = g.edges.iter().any(|e| {
                e.from == bt_id && e.to == "DISC-GC-01" && e.edge_type == EdgeType::Derives
            });
            assert!(has, "{} should derive DISC-GC-01", bt_id);
        }
    }

    // ── Validates edges ──

    #[test]
    fn test_me01_validates_disc_env_01() {
        let mut g = full_graph();
        populate_material_env(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "DISC-ME-01" && e.to == "DISC-ENV-01" && e.edge_type == EdgeType::Validates
        });
        assert!(has, "DISC-ME-01 should validate DISC-ENV-01");
    }

    #[test]
    fn test_gc01_validates_multiple() {
        let mut g = full_graph();
        populate_grand_convergence(&mut g);
        let val_count = g.edges.iter()
            .filter(|e| e.from == "DISC-GC-01" && e.edge_type == EdgeType::Validates)
            .count();
        assert_eq!(val_count, 3, "DISC-GC-01 should validate 3 nodes, got {}", val_count);
    }

    // ── Triggers edges ──

    #[test]
    fn test_ge02_triggers_ge03() {
        let mut g = full_graph();
        populate_grid_env(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "DISC-GE-02" && e.to == "DISC-GE-03" && e.edge_type == EdgeType::Triggers
        });
        assert!(has, "DISC-GE-02 should trigger DISC-GE-03");
    }

    #[test]
    fn test_me01_triggers_me02() {
        let mut g = full_graph();
        populate_material_env(&mut g);
        let has = g.edges.iter().any(|e| {
            e.from == "DISC-ME-01" && e.to == "DISC-ME-02" && e.edge_type == EdgeType::Triggers
        });
        assert!(has, "DISC-ME-01 should trigger DISC-ME-02");
    }

    // ── Cross-domain Merges ──

    #[test]
    fn test_ac_internal_cross_domain() {
        let mut g = full_graph();
        populate_all_advanced_cross(&mut g);

        // DISC-GC-01 (Material,Energy,Environment,Biology,Chemistry) and
        // DISC-ME-01 (Material,Environment,Chemistry) share 3 domains
        let has_merge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-GC-01" && e.to == "DISC-ME-01")
                    || (e.from == "DISC-ME-01" && e.to == "DISC-GC-01"))
        });
        assert!(has_merge, "GC-01 and ME-01 should merge via Material+Environment+Chemistry");
    }

    #[test]
    fn test_ac_to_existing_cross_domain() {
        let mut g = full_graph();
        populate_all_advanced_cross(&mut g);

        // DISC-SE-01 (Energy,Environment,Physics,Chemistry) should bridge
        // to DISC-FE-01 (Fusion,Environment,Chemistry,Biology) via Environment+Chemistry (2+)
        let has_bridge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-SE-01" && e.to == "DISC-FE-01")
                    || (e.from == "DISC-FE-01" && e.to == "DISC-SE-01"))
        });
        assert!(has_bridge, "SE-01 should bridge to FE-01 via Environment+Chemistry");
    }

    // ── Node type constraints ──

    #[test]
    fn test_hypothesis_low_confidence() {
        for cluster in ALL_AC_CLUSTERS {
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
        for e in AC_EXPERIMENTS {
            assert!(
                e.confidence >= 0.60 && e.confidence <= 0.80,
                "Experiment {} confidence {} should be in [0.60, 0.80]",
                e.id, e.confidence
            );
        }
    }

    #[test]
    fn test_discovery_high_confidence() {
        for cluster in &[GRID_ENV as &[ACEntry], MATERIAL_ENV, MATERIAL_ROBO,
                         SOLAR_ENV, GRAND_CONVERGENCE] {
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
    fn test_no_duplicate_ac_ids() {
        let mut g = full_graph();
        populate_all_advanced_cross(&mut g);

        let mut ids: Vec<String> = g.nodes.iter().map(|n| n.id.clone()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique (including advanced cross)");
    }

    #[test]
    fn test_all_ac_source_bts_exist() {
        let mut g = full_graph();
        populate_all_advanced_cross(&mut g);

        for cluster in ALL_AC_CLUSTERS {
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
        populate_all_advanced_cross(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} strength {} out of range",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_grand_convergence_domain_span() {
        // Grand convergence nodes should each span 5+ domains
        for e in GRAND_CONVERGENCE {
            assert!(
                e.domains.len() >= 5,
                "Grand convergence {} spans only {} domains, expected 5+",
                e.id, e.domains.len()
            );
        }
    }

    #[test]
    fn test_grand_convergence_multi_source() {
        // Grand convergence nodes should each derive from 4+ BTs
        for e in GRAND_CONVERGENCE {
            assert!(
                e.source_bts.len() >= 4,
                "Grand convergence {} has only {} source BTs, expected 4+",
                e.id, e.source_bts.len()
            );
        }
    }

    #[test]
    fn test_full_graph_with_all_advanced() {
        let mut g = full_graph();
        populate_all_advanced_cross(&mut g);

        // Previous: ~320+ nodes (base + cross_cluster), now +32 = 350+
        assert!(
            g.nodes.len() >= 320,
            "Full graph with advanced cross should have 320+ nodes, got {}",
            g.nodes.len()
        );
    }
}
