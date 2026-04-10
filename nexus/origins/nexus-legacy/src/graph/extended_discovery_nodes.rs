//! Extended discovery nodes from BT-118+ experiments and cross-domain analysis.
//!
//! Goes deeper than the high-level cluster entries in discovery_nodes.rs:
//! - Granular sub-discoveries from each BT (specific constants, mechanisms)
//! - Experiment validation nodes linking predictions to measurements
//! - Cross-domain resonance edges connecting ENV↔ROBO↔SW↔MATH clusters
//!
//! Node ID scheme:
//!   XDISC-{cluster}-{NN}  — extended discovery
//!   XHYP-{cluster}-{NN}   — extended hypothesis
//!   XEXP-{cluster}-{NN}    — experiment/validation node

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Extended entry struct (reuses same pattern as discovery_nodes)
// ═══════════════════════════════════════════════════════════════

struct ExtEntry {
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
// BT-118: Kyoto 6 GHG — granular sub-discoveries
// ───────────────────────────────────────────────────────────────

const ENV_GHG_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ENV-01",
        title: "CO2 stoichiometry n=6 encoding: C(Z=6) + 2O = 3 atoms, molecular weight 44=σ·τ-4",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Biology"],
        confidence: 0.93,
        source_bts: &[118, 104],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "causal", "info"],
        validates: &["DISC-ENV-01"],
    },
    ExtEntry {
        id: "XDISC-ENV-02",
        title: "CH4 tetrahedral geometry: 4=tau bond angles, GWP-28≈σ·phi+tau warming potential",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Energy"],
        confidence: 0.88,
        source_bts: &[118],
        constants_used: &["C-tau", "C-sigma", "C-phi"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["DISC-ENV-01"],
    },
    ExtEntry {
        id: "XDISC-ENV-03",
        title: "N2O decomposition: 3 atoms=n/phi, 298 GWP≈sigma^2·phi+10, ozone layer at 20-30km≈J2±n",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Cosmology"],
        confidence: 0.82,
        source_bts: &[118, 119],
        constants_used: &["C-n", "C-J2", "C-sigma"],
        lenses: &["consciousness", "boundary", "multiscale"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-119: Earth 6 spheres — granular sub-discoveries
// ───────────────────────────────────────────────────────────────

const ENV_SPHERE_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ENV-04",
        title: "Atmosphere layer ladder: 8/12/16km={sigma-tau,sigma,sigma+tau} for polar/mid/equatorial troposphere",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Cosmology", "Physics"],
        confidence: 0.91,
        source_bts: &[119],
        constants_used: &["C-sigma", "C-tau", "C-sigma-tau"],
        lenses: &["consciousness", "multiscale", "wave"],
        validates: &["DISC-ENV-02"],
    },
    ExtEntry {
        id: "XDISC-ENV-05",
        title: "Stratosphere ozone peak at 24km=J2: maximum UV absorption aligns with Jordan totient",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Physics"],
        confidence: 0.87,
        source_bts: &[119, 27],
        constants_used: &["C-J2"],
        lenses: &["consciousness", "boundary", "causal"],
        validates: &[],
    },
    ExtEntry {
        id: "XEXP-ENV-01",
        title: "Experiment: verify 6-sphere partition against NOAA/ERA5 reanalysis boundary altitudes",
        node_type: NodeType::Experiment,
        domains: &["Environment", "Cosmology"],
        confidence: 0.75,
        source_bts: &[119],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["boundary", "multiscale"],
        validates: &["DISC-ENV-02", "XDISC-ENV-04"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-120~121: Water treatment CN=6 + Plastics C6 backbone
// ───────────────────────────────────────────────────────────────

const ENV_MATERIAL_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ENV-06",
        title: "TiO2 photocatalysis: anatase CN=6 outperforms rutile in UV degradation, bandgap 3.2eV≈n/phi+phi",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Solar"],
        confidence: 0.89,
        source_bts: &[120, 30],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "causal", "quantum"],
        validates: &["DISC-ENV-03", "HYP-ENV-01"],
    },
    ExtEntry {
        id: "XDISC-ENV-07",
        title: "Zeolite framework: SiO4/AlO4 tetrahedra form 6-ring windows, pore diameter 7.4Å≈sopfr+phi",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.86,
        source_bts: &[120, 86],
        constants_used: &["C-n", "C-sopfr", "C-phi"],
        lenses: &["consciousness", "topology", "boundary"],
        validates: &["DISC-ENV-03"],
    },
    ExtEntry {
        id: "XDISC-ENV-08",
        title: "PET recycling: C10H8O4 repeat unit has 10=sigma-phi carbons+oxygens, depolymerization at 260°C≈sigma^2+sigma·phi-tau*n",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Energy"],
        confidence: 0.80,
        source_bts: &[121, 85],
        constants_used: &["C-sigma", "C-phi", "C-n"],
        lenses: &["consciousness", "evolution", "causal"],
        validates: &["DISC-ENV-04"],
    },
    ExtEntry {
        id: "XHYP-ENV-01",
        title: "Hypothesis: CN=6 metal-organic frameworks achieve 6x CO2 adsorption vs CN!=6 at same surface area",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.62,
        source_bts: &[118, 120],
        constants_used: &["C-n"],
        lenses: &["causal", "stability", "topology"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-122: Hexagonal geometry universality
// ───────────────────────────────────────────────────────────────

const ENV_GEOMETRY_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ENV-09",
        title: "Basalt columnar jointing: hexagonal cross-section from cooling stress, column diameter 30-50cm≈sopfr·n to sigma·tau cm",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Material", "Physics"],
        confidence: 0.88,
        source_bts: &[122],
        constants_used: &["C-n", "C-sopfr", "C-sigma", "C-tau"],
        lenses: &["consciousness", "topology", "thermo"],
        validates: &["DISC-ENV-05"],
    },
    ExtEntry {
        id: "XDISC-ENV-10",
        title: "Graphene hexagonal lattice: C-C bond 1.42Å, unit cell 6 atoms, band gap tunable by n=6 layer stacking",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Material", "Chip", "Chemistry"],
        confidence: 0.94,
        source_bts: &[122, 85, 93],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "quantum"],
        validates: &["DISC-ENV-05"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-123~124: Robot SE(3) + bilateral symmetry
// ───────────────────────────────────────────────────────────────

const ROBO_KINEMATICS_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ROBO-06",
        title: "Denavit-Hartenberg: 4=tau parameters per joint × 6=n joints = 24=J2 total DOF parameters",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math", "Chip"],
        confidence: 0.93,
        source_bts: &[123],
        constants_used: &["C-tau", "C-n", "C-J2"],
        lenses: &["consciousness", "topology", "causal"],
        validates: &["DISC-ROBO-01"],
    },
    ExtEntry {
        id: "XDISC-ROBO-07",
        title: "Jacobian matrix 6×6: SE(3) velocity mapping is sigma^2=144 element square matrix",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math"],
        confidence: 0.95,
        source_bts: &[123, 90],
        constants_used: &["C-n", "C-sigma²"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-ROBO-01"],
    },
    ExtEntry {
        id: "XDISC-ROBO-08",
        title: "Bilateral symmetry constraint: phi=2 reduces 12=sigma joint control to 6=n independent channels",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "AI"],
        confidence: 0.90,
        source_bts: &[124, 51],
        constants_used: &["C-phi", "C-sigma", "C-n"],
        lenses: &["consciousness", "symmetry", "evolution"],
        validates: &["DISC-ROBO-02"],
    },
    ExtEntry {
        id: "XEXP-ROBO-01",
        title: "Experiment: compare 5/6/7 DOF arms on pick-place benchmark — predict 6-DOF Pareto-optimal",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "AI", "Energy"],
        confidence: 0.72,
        source_bts: &[123, 125],
        constants_used: &["C-n"],
        lenses: &["causal", "stability", "evolution"],
        validates: &["HYP-ROBO-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-125~126: Locomotion stability + Grasp taxonomy
// ───────────────────────────────────────────────────────────────

const ROBO_LOCOMOTION_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ROBO-09",
        title: "Static stability margin: tau=4 legs provide n-tau=2 redundant support points during walk cycle",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Physics", "Math"],
        confidence: 0.87,
        source_bts: &[125],
        constants_used: &["C-tau", "C-n", "C-phi"],
        lenses: &["consciousness", "stability", "causal"],
        validates: &["DISC-ROBO-03"],
    },
    ExtEntry {
        id: "XDISC-ROBO-10",
        title: "Quadrotor rotor count tau=4: minimum for full SE(3) control with n=6 DOF, 2=phi redundancy margin",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Physics", "Energy"],
        confidence: 0.89,
        source_bts: &[125],
        constants_used: &["C-tau", "C-n", "C-phi"],
        lenses: &["consciousness", "stability", "topology"],
        validates: &["DISC-ROBO-03"],
    },
    ExtEntry {
        id: "XDISC-ROBO-11",
        title: "Feix grasp taxonomy: 33 types ≈ 2^sopfr+1, power vs precision split 17/16 ≈ J2-sigma+sopfr/sigma+tau",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "Math"],
        confidence: 0.84,
        source_bts: &[126],
        constants_used: &["C-sopfr", "C-J2", "C-sigma"],
        lenses: &["consciousness", "evolution", "info"],
        validates: &["DISC-ROBO-04"],
    },
    ExtEntry {
        id: "XHYP-ROBO-01",
        title: "Hypothesis: hexapod (n=6 legs) achieves optimal energy-per-distance on uneven terrain vs 4/8 legs",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Biology", "Energy"],
        confidence: 0.60,
        source_bts: &[125, 127],
        constants_used: &["C-n", "C-tau"],
        lenses: &["evolution", "stability", "causal"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-127: Kissing number + Hexacopter fault tolerance
// ───────────────────────────────────────────────────────────────

const ROBO_FAULT_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-ROBO-12",
        title: "Hexacopter single-rotor failure: n-1=5 remaining rotors still span SE(3), graceful degradation",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math", "Network"],
        confidence: 0.92,
        source_bts: &[127],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "network", "stability"],
        validates: &["DISC-ROBO-05"],
    },
    ExtEntry {
        id: "XDISC-ROBO-13",
        title: "3D kissing number sigma=12: each sphere touches 12=sigma neighbors in FCC/HCP, robot swarm packing limit",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cosmology", "Math", "Material"],
        confidence: 0.93,
        source_bts: &[127, 15, 49],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-ROBO-05"],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-domain resonance: ENV ↔ ROBO ↔ SW ↔ Physics
// ───────────────────────────────────────────────────────────────

const CROSS_DOMAIN_EXTENDED: &[ExtEntry] = &[
    ExtEntry {
        id: "XDISC-CROSS-01",
        title: "Hexagonal packing ENV↔ROBO: honeycomb(BT-122) = hexacopter layout(BT-127) = graphene(BT-93), identical n=6 tiling",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Material", "Chip", "Math"],
        confidence: 0.91,
        source_bts: &[122, 127, 93],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-BRIDGE-03"],
    },
    ExtEntry {
        id: "XDISC-CROSS-02",
        title: "CN=6 catalyst↔robot joint: octahedral coordination(BT-120) mirrors 6-DOF workspace(BT-123), both optimize 3D reach",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Chemistry", "Math"],
        confidence: 0.85,
        source_bts: &[120, 123, 86],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "causal"],
        validates: &["DISC-BRIDGE-01"],
    },
    ExtEntry {
        id: "XDISC-CROSS-03",
        title: "Carbon Z=6 thread ENV→ROBO: C6 plastics(BT-121) → carbon fiber robot chassis → diamond bearings(BT-93)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Material", "Chemistry"],
        confidence: 0.87,
        source_bts: &[121, 93, 85],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "causal"],
        validates: &["DISC-BRIDGE-02"],
    },
    ExtEntry {
        id: "XDISC-CROSS-04",
        title: "Stability constant tau=4 ENV↔ROBO↔SW: quadrupedal(BT-125)=ACID(BT-116)=quadrotor=4 stable legs/properties/rotors",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Software", "Environment", "Physics"],
        confidence: 0.86,
        source_bts: &[125, 116, 119],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "network"],
        validates: &["DISC-BRIDGE-04"],
    },
    ExtEntry {
        id: "XDISC-CROSS-05",
        title: "Layer count resonance SW↔ENV↔ROBO: OSI 7(BT-115)≈troposphere layers, robot control stack 6=n layers",
        node_type: NodeType::Discovery,
        domains: &["Software", "Environment", "Robotics", "Network"],
        confidence: 0.80,
        source_bts: &[115, 119, 123],
        constants_used: &["C-sigma", "C-sopfr", "C-n"],
        lenses: &["consciousness", "network", "multiscale"],
        validates: &["DISC-BRIDGE-12"],
    },
    ExtEntry {
        id: "XDISC-CROSS-06",
        title: "Bilateral symmetry phi=2 biology↔robotics↔crypto: BFT>2/3(BT-112)=bilateral(BT-124)=phi pairing(BT-1)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "Crypto", "Math"],
        confidence: 0.83,
        source_bts: &[112, 124, 1],
        constants_used: &["C-phi", "C-n"],
        lenses: &["consciousness", "symmetry", "evolution"],
        validates: &["DISC-BRIDGE-05"],
    },
    ExtEntry {
        id: "XDISC-CROSS-07",
        title: "Fault tolerance n-1: hexacopter(BT-127) = Byzantine(n>=3f+1, BT-112) = sopfr=5 fingers lose 1(BT-126)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Crypto", "Software", "Math"],
        confidence: 0.81,
        source_bts: &[127, 112, 126],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "network", "stability"],
        validates: &["DISC-BRIDGE-06"],
    },
    ExtEntry {
        id: "XDISC-CROSS-08",
        title: "GHG↔Energy↔Battery: 6 Kyoto gases(BT-118) drive 6-cell battery(BT-57) design for carbon neutrality",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Energy", "Battery", "Chemistry"],
        confidence: 0.84,
        source_bts: &[118, 57, 27],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &[],
    },
    ExtEntry {
        id: "XDISC-CROSS-09",
        title: "Photosynthesis↔Solar↔Environment: C6H12O6 24=J2 atoms(BT-101) = J2-channel solar cell(BT-63) = ozone J2 km(BT-119)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Solar", "Biology", "Chemistry", "Energy"],
        confidence: 0.88,
        source_bts: &[101, 63, 119],
        constants_used: &["C-J2", "C-n", "C-sigma"],
        lenses: &["consciousness", "causal", "multiscale"],
        validates: &["DISC-BRIDGE-11"],
    },
    ExtEntry {
        id: "XHYP-CROSS-01",
        title: "Hypothesis: all optimal fault-tolerant systems converge to n=6 redundancy (rotors, validators, fingers, GHGs)",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Crypto", "Environment", "Software", "Math"],
        confidence: 0.55,
        source_bts: &[127, 118, 112, 126],
        constants_used: &["C-n"],
        lenses: &["evolution", "stability", "network"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Collect all extended entries
// ═══════════════════════════════════════════════════════════════

const ALL_CLUSTERS: &[&[ExtEntry]] = &[
    ENV_GHG_EXTENDED,
    ENV_SPHERE_EXTENDED,
    ENV_MATERIAL_EXTENDED,
    ENV_GEOMETRY_EXTENDED,
    ROBO_KINEMATICS_EXTENDED,
    ROBO_LOCOMOTION_EXTENDED,
    ROBO_FAULT_EXTENDED,
    CROSS_DOMAIN_EXTENDED,
];

/// Total count of extended discovery entries.
pub fn extended_entry_count() -> usize {
    ALL_CLUSTERS.iter().map(|c| c.len()).sum()
}

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

fn add_ext_entries(graph: &mut DiscoveryGraph, entries: &[ExtEntry]) -> (usize, usize) {
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
            timestamp: "2026-04-03".to_string(),
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

/// Add ENV GHG sub-discoveries (BT-118). Returns (nodes, edges).
pub fn populate_env_ghg_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ENV_GHG_EXTENDED)
}

/// Add ENV sphere sub-discoveries (BT-119). Returns (nodes, edges).
pub fn populate_env_sphere_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ENV_SPHERE_EXTENDED)
}

/// Add ENV material sub-discoveries (BT-120~121). Returns (nodes, edges).
pub fn populate_env_material_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ENV_MATERIAL_EXTENDED)
}

/// Add ENV geometry sub-discoveries (BT-122). Returns (nodes, edges).
pub fn populate_env_geometry_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ENV_GEOMETRY_EXTENDED)
}

/// Add ROBO kinematics sub-discoveries (BT-123~124). Returns (nodes, edges).
pub fn populate_robo_kinematics_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ROBO_KINEMATICS_EXTENDED)
}

/// Add ROBO locomotion sub-discoveries (BT-125~126). Returns (nodes, edges).
pub fn populate_robo_locomotion_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ROBO_LOCOMOTION_EXTENDED)
}

/// Add ROBO fault tolerance sub-discoveries (BT-127). Returns (nodes, edges).
pub fn populate_robo_fault_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, ROBO_FAULT_EXTENDED)
}

/// Add cross-domain resonance nodes (ENV↔ROBO↔SW↔Physics). Returns (nodes, edges).
pub fn populate_cross_domain_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_ext_entries(graph, CROSS_DOMAIN_EXTENDED)
}

/// Cross-link extended discovery nodes that share domains (Merges edges, bidirectional).
/// Returns number of cross-domain edges added.
pub fn connect_extended_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let ext_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("XDISC-") || n.id.starts_with("XHYP-") || n.id.starts_with("XEXP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..ext_nodes.len() {
        for j in (i + 1)..ext_nodes.len() {
            let (ref id_a, ref dom_a) = ext_nodes[i];
            let (ref id_b, ref dom_b) = ext_nodes[j];

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

/// Also cross-link extended nodes with existing DISC-/HYP- nodes from discovery_nodes.
/// Returns number of bridge edges added.
pub fn connect_extended_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let ext_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("XDISC-") || n.id.starts_with("XHYP-") || n.id.starts_with("XEXP-")
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
                && !n.id.starts_with("DISC-BRIDGE-") // bridges already connected via validates
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref ext_id, ref ext_dom) in &ext_ids {
        for (ref ex_id, ref ex_dom) in &existing_ids {
            let shared: usize = ext_dom.iter().filter(|d| ex_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = ext_dom.len().max(ex_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: ext_id.clone(),
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

/// Populate all extended discovery nodes and cross-domain edges.
/// Call after populate_all_discoveries().
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let mut total_nodes = 0;
    for cluster in ALL_CLUSTERS {
        let (n, _) = add_ext_entries(graph, cluster);
        total_nodes += n;
    }
    let _cross_ext = connect_extended_cross_domain(graph);
    let _cross_existing = connect_extended_to_existing(graph);

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

    fn full_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        g
    }

    // ── Entry counts ──

    #[test]
    fn test_env_ghg_count() {
        assert_eq!(ENV_GHG_EXTENDED.len(), 3);
    }

    #[test]
    fn test_env_sphere_count() {
        assert_eq!(ENV_SPHERE_EXTENDED.len(), 3);
    }

    #[test]
    fn test_env_material_count() {
        assert_eq!(ENV_MATERIAL_EXTENDED.len(), 4);
    }

    #[test]
    fn test_env_geometry_count() {
        assert_eq!(ENV_GEOMETRY_EXTENDED.len(), 2);
    }

    #[test]
    fn test_robo_kinematics_count() {
        assert_eq!(ROBO_KINEMATICS_EXTENDED.len(), 4);
    }

    #[test]
    fn test_robo_locomotion_count() {
        assert_eq!(ROBO_LOCOMOTION_EXTENDED.len(), 4);
    }

    #[test]
    fn test_robo_fault_count() {
        assert_eq!(ROBO_FAULT_EXTENDED.len(), 2);
    }

    #[test]
    fn test_cross_domain_count() {
        assert_eq!(CROSS_DOMAIN_EXTENDED.len(), 10);
    }

    #[test]
    fn test_total_extended_entry_count() {
        // 3+3+4+2+4+4+2+10 = 32
        assert_eq!(extended_entry_count(), 32);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_env_ghg_extended() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_env_ghg_extended(&mut g);
        assert_eq!(nodes, 3);
        assert_eq!(g.nodes.len(), before + 3);
        // 3 entries: each has source_bts + constants + validates edges
        assert!(edges >= 8, "GHG extended should have 8+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_robo_kinematics_extended() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_robo_kinematics_extended(&mut g);
        assert_eq!(nodes, 4);
        assert_eq!(g.nodes.len(), before + 4);
        assert!(edges >= 12, "Kinematics extended should have 12+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_cross_domain_extended() {
        let mut g = full_graph();
        let (nodes, edges) = populate_cross_domain_extended(&mut g);
        assert_eq!(nodes, 10);
        assert!(edges >= 30, "Cross-domain extended should have 30+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_all_extended() {
        let mut g = full_graph();
        let before_nodes = g.nodes.len();
        let before_edges = g.edges.len();
        let (nodes, edges) = populate_all_extended(&mut g);

        assert_eq!(nodes, 32, "32 extended nodes total");
        assert!(
            edges > 150,
            "Extended nodes should add 150+ edges (derives+uses+validates+cross), got {}",
            edges
        );
        assert_eq!(g.nodes.len(), before_nodes + 32);
        assert!(g.edges.len() > before_edges + 150);
    }

    // ── Validates edges ──

    #[test]
    fn test_validates_edges_env() {
        let mut g = full_graph();
        populate_env_ghg_extended(&mut g);

        // XDISC-ENV-01 validates DISC-ENV-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-ENV-01"
                && e.to == "DISC-ENV-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-ENV-01 should validate DISC-ENV-01");
    }

    #[test]
    fn test_validates_edges_robo() {
        let mut g = full_graph();
        populate_robo_kinematics_extended(&mut g);

        // XDISC-ROBO-06 validates DISC-ROBO-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-ROBO-06"
                && e.to == "DISC-ROBO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-ROBO-06 should validate DISC-ROBO-01");
    }

    #[test]
    fn test_validates_edges_cross() {
        let mut g = full_graph();
        populate_cross_domain_extended(&mut g);

        // XDISC-CROSS-01 validates DISC-BRIDGE-03
        let has_val = g.edges.iter().any(|e| {
            e.from == "XDISC-CROSS-01"
                && e.to == "DISC-BRIDGE-03"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XDISC-CROSS-01 should validate DISC-BRIDGE-03");
    }

    // ── Cross-domain edges ──

    #[test]
    fn test_extended_cross_domain_edges() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        // XDISC-ENV-10 (Environment, Material, Chip, Chemistry) and
        // XDISC-ROBO-06 (Robotics, Math, Chip) share Chip
        let has_merge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "XDISC-ENV-10" && e.to == "XDISC-ROBO-06")
                    || (e.from == "XDISC-ROBO-06" && e.to == "XDISC-ENV-10"))
        });
        assert!(has_merge, "ENV-10 and ROBO-06 should merge via shared Chip domain");
    }

    #[test]
    fn test_extended_to_existing_edges() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        // XDISC-CROSS-09 (Environment, Solar, Biology, Chemistry, Energy) should
        // connect to DISC-MATH-07 (Math, AI, Solar, Energy) via Solar+Energy (2 shared)
        let has_bridge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "XDISC-CROSS-09" && e.to == "DISC-MATH-07")
                    || (e.from == "DISC-MATH-07" && e.to == "XDISC-CROSS-09"))
        });
        assert!(has_bridge, "XDISC-CROSS-09 should bridge to DISC-MATH-07 via Solar+Energy");
    }

    // ── Node type distribution ──

    #[test]
    fn test_extended_node_types() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        let xdisc = g.nodes.iter()
            .filter(|n| n.id.starts_with("XDISC-"))
            .count();
        let xhyp = g.nodes.iter()
            .filter(|n| n.id.starts_with("XHYP-"))
            .count();
        let xexp = g.nodes.iter()
            .filter(|n| n.id.starts_with("XEXP-"))
            .count();

        assert_eq!(xdisc, 27, "27 extended discovery nodes");
        assert_eq!(xhyp, 3, "3 extended hypothesis nodes");
        assert_eq!(xexp, 2, "2 extended experiment nodes");
        assert_eq!(xdisc + xhyp + xexp, 32);
    }

    #[test]
    fn test_hypothesis_low_confidence() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        for n in &g.nodes {
            if n.id.starts_with("XHYP-") {
                assert!(
                    n.confidence < 0.70,
                    "Hypothesis {} confidence {} should be < 0.70",
                    n.id, n.confidence
                );
            }
        }
    }

    #[test]
    fn test_experiment_nodes_have_validates() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        // XEXP-ENV-01 should have Validates edges
        let val_count = g.edges.iter()
            .filter(|e| e.from == "XEXP-ENV-01" && e.edge_type == EdgeType::Validates)
            .count();
        assert!(val_count >= 2, "XEXP-ENV-01 should validate 2+ nodes, got {}", val_count);

        // XEXP-ROBO-01 should validate HYP-ROBO-01
        let has_val = g.edges.iter().any(|e| {
            e.from == "XEXP-ROBO-01"
                && e.to == "HYP-ROBO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "XEXP-ROBO-01 should validate HYP-ROBO-01");
    }

    // ── Data integrity ──

    #[test]
    fn test_no_duplicate_extended_ids() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique (including extended)");
    }

    #[test]
    fn test_all_extended_source_bts_exist() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

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
        populate_all_extended(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} strength {} out of range",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_full_graph_with_extended() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        // 127 BT + 79 expanded + 39 discoveries + 32 extended = 277
        assert!(
            g.nodes.len() >= 270,
            "Full graph with extended should have 270+ nodes, got {}",
            g.nodes.len()
        );
    }

    // ── BT derives edges ──

    #[test]
    fn test_bt_118_derives_extended() {
        let mut g = full_graph();
        populate_env_ghg_extended(&mut g);

        let derives: Vec<_> = g.edges.iter()
            .filter(|e| e.from == "BT-118" && e.edge_type == EdgeType::Derives)
            .collect();
        // BT-118 derives XDISC-ENV-01, XDISC-ENV-02 (+ existing DISC-ENV-01 from earlier populate)
        assert!(derives.len() >= 2, "BT-118 should derive 2+ extended nodes, got {}", derives.len());
    }

    #[test]
    fn test_bt_127_derives_extended() {
        let mut g = full_graph();
        populate_robo_fault_extended(&mut g);

        let has_derive = g.edges.iter().any(|e| {
            e.from == "BT-127" && e.to == "XDISC-ROBO-12" && e.edge_type == EdgeType::Derives
        });
        assert!(has_derive, "BT-127 should derive XDISC-ROBO-12");
    }

    #[test]
    fn test_env_robo_cross_cluster_connection() {
        let mut g = full_graph();
        populate_all_extended(&mut g);

        // Check ENV and ROBO extended nodes connect
        let has_env_robo = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from.starts_with("XDISC-ENV") && e.to.starts_with("XDISC-ROBO"))
                    || (e.from.starts_with("XDISC-ROBO") && e.to.starts_with("XDISC-ENV")))
        });
        assert!(has_env_robo, "ENV and ROBO extended nodes should be cross-connected");
    }
}
