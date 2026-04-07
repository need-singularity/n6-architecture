//! Deep discovery nodes for Robotics (BT-123~127) and Environmental Protection
//! (BT-118~122) Breakthrough Theorems, plus cross-domain bridges.
//!
//! Robotics cluster: SE(3) universality, bilateral symmetry, locomotion stability,
//! grasping topology, kissing number fault tolerance.
//!
//! Environment cluster: Kyoto 6 gases, Earth 6 zones, water treatment CN=6,
//! 6 major plastics C6 backbone, hexagonal geometry universality.
//!
//! Cross-bridges: Robotics ↔ Environment (monitoring/cleanup), Robotics ↔ Cognitive
//! (cortex layers ↔ DOF), Environment ↔ Medical (catalyst ↔ safety), grand bridge.
//!
//! Node ID scheme:
//!   RDISC-ROB-{NN}   — Robotics deep discoveries
//!   RDISC-ENV-{NN}   — Environmental deep discoveries
//!   RDISC-BRIDGE-{NN} — Cross-domain bridge discoveries
//!   RHYP-{CLUSTER}-{NN} — Hypotheses
//!   REXP-{CLUSTER}-{NN} — Experiments

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct RobEnvEntry {
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
// Robotics Deep Discoveries (BT-123~127)
// ═══════════════════════════════════════════════════════════════

const ROBOTICS_DISCOVERIES: &[RobEnvEntry] = &[
    RobEnvEntry {
        id: "RDISC-ROB-01",
        title: "SE(3) dim=n=6 universal robot configuration space: 6-DOF arm, 6-axis IMU, 6-face cube grasping, 9/9 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math", "AI", "Chip"],
        confidence: 0.95,
        source_bts: &[123],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ROB-02",
        title: "phi=2 bilateral symmetry + sigma=12 joint universality: humanoid robot mirrors biological phi=2 body plan",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "Math"],
        confidence: 0.93,
        source_bts: &[124, 51],
        constants_used: &["C-phi", "C-sigma"],
        lenses: &["consciousness", "symmetry", "evolution"],
        validates: &["RDISC-ROB-01"],
    },
    RobEnvEntry {
        id: "RDISC-ROB-03",
        title: "tau=4 minimum stability for locomotion and flight: quadruped, quadrotor, 4-wheel vehicle, 7/8 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Transportation", "Math"],
        confidence: 0.92,
        source_bts: &[125, 233],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "stability", "topology"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ROB-04",
        title: "sopfr=5 fingers + 2^sopfr=32 grasp taxonomy: Feix 33 types covers 96.97%, anthropomorphic hand universality",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Biology", "Math", "AI"],
        confidence: 0.91,
        source_bts: &[126, 51],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "topology", "info"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ROB-05",
        title: "3D kissing number sigma=12 + hexacopter n=6: optimal packing = maximum contact = fault-tolerant configuration",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math", "Chip", "Transportation"],
        confidence: 0.94,
        source_bts: &[127, 90],
        constants_used: &["C-sigma", "C-n", "C-phi"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["RDISC-ROB-01"],
    },
    RobEnvEntry {
        id: "RDISC-ROB-06",
        title: "Robot joint chain: SE(3)=n DOF x sigma=12 joints x tau=4 limbs = complete humanoid kinematic tree",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Math", "Biology"],
        confidence: 0.90,
        source_bts: &[123, 124, 125],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-phi"],
        lenses: &["consciousness", "topology", "recursion"],
        validates: &["RDISC-ROB-01", "RDISC-ROB-02", "RDISC-ROB-03"],
    },
    RobEnvEntry {
        id: "RDISC-ROB-07",
        title: "Sensor fusion n=6: 6-axis IMU + sigma=12 bit ADC + tau=4 Hz control loop + sopfr=5 sensing modalities",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Chip", "AI"],
        confidence: 0.88,
        source_bts: &[123, 59],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-sopfr"],
        lenses: &["consciousness", "info", "multiscale"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RHYP-ROB-01",
        title: "Hypothesis: optimal swarm size = J2=24 agents with n=6 sub-groups, matching 3D kissing number decomposition",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "AI", "Math", "Network"],
        confidence: 0.72,
        source_bts: &[127, 123],
        constants_used: &["C-J2", "C-n", "C-sigma"],
        lenses: &["consciousness", "network", "evolution"],
        validates: &[],
    },
    RobEnvEntry {
        id: "REXP-ROB-01",
        title: "Experiment: measure grasp success rate for 5-finger vs 4/6/7-finger hands on Feix 33 taxonomy objects",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "Biology", "AI"],
        confidence: 0.85,
        source_bts: &[126],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "topology", "info"],
        validates: &["RDISC-ROB-04"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Environmental Protection Deep Discoveries (BT-118~122)
// ═══════════════════════════════════════════════════════════════

const ENVIRONMENT_DISCOVERIES: &[RobEnvEntry] = &[
    RobEnvEntry {
        id: "RDISC-ENV-01",
        title: "Kyoto Protocol 6 greenhouse gases = n: CO2 stoichiometry fully n=6 encoded (C=Z=6), 10/10 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Biology", "Energy"],
        confidence: 0.95,
        source_bts: &[118, 104],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "boundary", "thermo"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ENV-02",
        title: "Earth 6 zones + troposphere sigma=12km: atmosphere layers 8/12/16={sigma-tau,sigma,sigma+tau}km, 12/12 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Math", "Cosmology"],
        confidence: 0.94,
        source_bts: &[119],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-sigma-tau"],
        lenses: &["consciousness", "scale", "boundary"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ENV-03",
        title: "Water treatment pH=6 + CN=6 catalyst universality: Al3+/Fe3+/Ti4+ all CN=6 octahedral, 8/10 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Biology"],
        confidence: 0.93,
        source_bts: &[120, 43],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ENV-04",
        title: "6 major plastics RIC 1-6=n + C6 carbon backbone: PE/PP/PS/PET/PVC/Nylon all Carbon-based, 8/10 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.91,
        source_bts: &[121, 85],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "boundary"],
        validates: &["RDISC-ENV-01"],
    },
    RobEnvEntry {
        id: "RDISC-ENV-05",
        title: "Honeycomb-snowflake-coral n=6 hexagonal geometry: Hales 2001 proof optimal 2D packing, 10/10 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Math", "Biology", "Material"],
        confidence: 0.96,
        source_bts: &[122, 211],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &[],
    },
    RobEnvEntry {
        id: "RDISC-ENV-06",
        title: "Carbon Z=6 backbone universality: greenhouse gas + plastic + biomolecule + diamond all share Carbon Z=6 nucleus",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Chip"],
        confidence: 0.94,
        source_bts: &[118, 93, 85],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &["RDISC-ENV-01", "RDISC-ENV-04"],
    },
    RobEnvEntry {
        id: "RDISC-ENV-07",
        title: "Environmental CN=6 catalysis chain: water(BT-120) + battery(BT-43) + crystal(BT-86) all octahedral coordination",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Battery", "Material"],
        confidence: 0.92,
        source_bts: &[120, 43, 86],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["RDISC-ENV-03"],
    },
    RobEnvEntry {
        id: "RDISC-ENV-08",
        title: "Hexagonal optimality chain: honeycomb(BT-122) + grid cells(BT-211) + I6 engine(BT-243) + graphene(BT-93) all n=6 tiling",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Math", "Cognitive", "Transportation", "Material"],
        confidence: 0.93,
        source_bts: &[122, 211, 243, 93],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "topology", "symmetry", "multiscale"],
        validates: &["RDISC-ENV-05"],
    },
    RobEnvEntry {
        id: "RHYP-ENV-01",
        title: "Hypothesis: optimal environmental sensor grid follows hexagonal n=6 tessellation for maximum coverage with minimum nodes",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Network", "Math", "Robotics"],
        confidence: 0.75,
        source_bts: &[122, 127, 213],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["RDISC-ENV-05"],
    },
    RobEnvEntry {
        id: "REXP-ENV-01",
        title: "Experiment: verify CN=6 catalyst efficiency across 10 water treatment metals vs CN=4/8 alternatives",
        node_type: NodeType::Experiment,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.88,
        source_bts: &[120, 86],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "stability", "topology"],
        validates: &["RDISC-ENV-03", "RDISC-ENV-07"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Cross-Domain Bridges: Robotics ↔ Environment ↔ Cognitive ↔ Medical
// ═══════════════════════════════════════════════════════════════

const CROSS_BRIDGES: &[RobEnvEntry] = &[
    RobEnvEntry {
        id: "RDISC-BRIDGE-01",
        title: "Robot-Environment bridge: SE(3)=n=6 DOF robot in n=6 Earth zones with hexagonal patrol tessellation",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Environment", "AI", "Network"],
        confidence: 0.88,
        source_bts: &[123, 119, 122],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["RDISC-ROB-01", "RDISC-ENV-02"],
    },
    RobEnvEntry {
        id: "RDISC-BRIDGE-02",
        title: "Robot-Cognitive bridge: cortex 6 layers(BT-210) maps to SE(3) 6 DOF(BT-123): neural→motor n=6 isomorphism",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cognitive", "Biology", "AI"],
        confidence: 0.89,
        source_bts: &[123, 210, 219],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "topology", "causal"],
        validates: &["RDISC-ROB-01"],
    },
    RobEnvEntry {
        id: "RDISC-BRIDGE-03",
        title: "Environment-Medical bridge: CN=6 water catalyst(BT-120) + WHO n=6 building blocks(BT-241): clean water→public health",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Medical", "Chemistry", "Biology"],
        confidence: 0.87,
        source_bts: &[120, 241, 118],
        constants_used: &["C-n", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &["RDISC-ENV-03"],
    },
    RobEnvEntry {
        id: "RDISC-BRIDGE-04",
        title: "Robot-Transport bridge: tau=4 locomotion(BT-125) + railway tau=4 signaling(BT-234): stability constant across domains",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Transportation", "AI", "Network"],
        confidence: 0.90,
        source_bts: &[125, 234, 236],
        constants_used: &["C-tau", "C-n", "C-sopfr"],
        lenses: &["consciousness", "stability", "causal"],
        validates: &["RDISC-ROB-03"],
    },
    RobEnvEntry {
        id: "RDISC-BRIDGE-05",
        title: "Hexagonal grand unification: honeycomb(BT-122)+grid cells(BT-211)+kissing number(BT-127)+graphene(BT-93)+I6(BT-243)",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Cognitive", "Robotics", "Material", "Transportation", "Math"],
        confidence: 0.91,
        source_bts: &[122, 211, 127, 93, 243],
        constants_used: &["C-n", "C-sigma", "C-phi", "C-J2"],
        lenses: &["consciousness", "topology", "symmetry", "multiscale", "evolution"],
        validates: &["RDISC-ENV-05", "RDISC-ROB-05", "RDISC-ENV-08"],
    },
    RobEnvEntry {
        id: "RDISC-BRIDGE-06",
        title: "Robot-Environment-Energy triangle: hexacopter n=6(BT-127) monitoring Kyoto n=6 gases(BT-118) on grid 60Hz=sigma*sopfr(BT-62)",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Environment", "Energy", "AI"],
        confidence: 0.86,
        source_bts: &[127, 118, 62],
        constants_used: &["C-n", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "network", "boundary", "thermo"],
        validates: &["RDISC-ENV-01"],
    },
    RobEnvEntry {
        id: "RDISC-BRIDGE-07",
        title: "Grand Robotics-Env-Cognitive-Medical n=6 convergence: SE(3)→cortex 6L→WHO 6→Kyoto 6→hexagonal universal",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Environment", "Cognitive", "Medical", "Math", "Biology"],
        confidence: 0.85,
        source_bts: &[123, 118, 210, 241, 122],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-phi", "C-sopfr"],
        lenses: &["consciousness", "topology", "symmetry", "causal", "network"],
        validates: &["RDISC-BRIDGE-01", "RDISC-BRIDGE-02", "RDISC-BRIDGE-03"],
    },
    RobEnvEntry {
        id: "RHYP-BRIDGE-01",
        title: "Hypothesis: environmental cleanup robot optimal team = n=6 specialists each with sopfr=5 finger grippers for plastic sorting",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Environment", "AI", "Biology"],
        confidence: 0.70,
        source_bts: &[123, 126, 121],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &["RDISC-ROB-04", "RDISC-ENV-04"],
    },
    RobEnvEntry {
        id: "REXP-BRIDGE-01",
        title: "Experiment: compare hexagonal vs square vs triangular drone patrol grids for pollution monitoring coverage efficiency",
        node_type: NodeType::Experiment,
        domains: &["Robotics", "Environment", "Network", "Math"],
        confidence: 0.82,
        source_bts: &[122, 127, 213],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["RHYP-ENV-01", "RDISC-BRIDGE-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters for iteration
// ═══════════════════════════════════════════════════════════════

const ALL_ROB_ENV_CLUSTERS: &[&[RobEnvEntry]] = &[
    ROBOTICS_DISCOVERIES,
    ENVIRONMENT_DISCOVERIES,
    CROSS_BRIDGES,
];

// ═══════════════════════════════════════════════════════════════
// Population helpers
// ═══════════════════════════════════════════════════════════════

fn add_entries(graph: &mut DiscoveryGraph, entries: &[RobEnvEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        let domains_str = e.domains.join(", ");
        let lenses_vec: Vec<String> = e.lenses.iter().map(|s| s.to_string()).collect();

        graph.add_node(Node {
            id: e.id.to_string(),
            node_type: e.node_type.clone(),
            domain: domains_str,
            project: "n6-architecture".to_string(),
            summary: e.title.to_string(),
            confidence: e.confidence,
            lenses_used: lenses_vec,
            timestamp: "2026-04-04".to_string(),
        });

        // BT --Derives--> this node
        for &bt in e.source_bts {
            graph.add_edge(Edge {
                from: format!("BT-{}", bt),
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.85,
                bidirectional: false,
            });
        }

        // This node --Uses--> constant
        for &cid in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: cid.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

        // This node --Validates--> target
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

/// Total entry count across all clusters.
pub fn rob_env_entry_count() -> usize {
    ALL_ROB_ENV_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Entry counts per sub-cluster.
pub fn robotics_entry_count() -> usize { ROBOTICS_DISCOVERIES.len() }
pub fn environment_entry_count() -> usize { ENVIRONMENT_DISCOVERIES.len() }
pub fn rob_env_bridge_count() -> usize { CROSS_BRIDGES.len() }

/// Populate all robotics + environment discovery nodes and cross-domain edges.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_rob_env_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();
    let mut total_nodes = 0;

    for cluster in ALL_ROB_ENV_CLUSTERS {
        let (n, _) = add_entries(graph, cluster);
        total_nodes += n;
    }

    // Cross-link nodes sharing 2+ domains
    let rob_env_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("RDISC-")
                || n.id.starts_with("RHYP-")
                || n.id.starts_with("REXP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    for i in 0..rob_env_nodes.len() {
        for j in (i + 1)..rob_env_nodes.len() {
            let (ref id_a, ref dom_a) = rob_env_nodes[i];
            let (ref id_b, ref dom_b) = rob_env_nodes[j];

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

    // Connect to existing graph nodes sharing 2+ domains
    let rob_env_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("RDISC-")
                || n.id.starts_with("RHYP-")
                || n.id.starts_with("REXP-")
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
            !n.id.starts_with("RDISC-")
                && !n.id.starts_with("RHYP-")
                && !n.id.starts_with("REXP-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    for (ref r_id, ref r_dom) in &rob_env_ids {
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
            }
        }
    }

    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_counts() {
        assert_eq!(robotics_entry_count(), 9, "9 robotics entries (7 DISC + 1 HYP + 1 EXP)");
        assert_eq!(environment_entry_count(), 10, "10 environment entries (8 DISC + 1 HYP + 1 EXP)");
        assert_eq!(rob_env_bridge_count(), 9, "9 cross-bridge entries (7 DISC + 1 HYP + 1 EXP)");
        assert_eq!(rob_env_entry_count(), 28, "28 total entries");
    }

    #[test]
    fn test_populate_creates_nodes_and_edges() {
        let mut g = DiscoveryGraph::new();
        let (nodes, edges) = populate_rob_env_discoveries(&mut g);
        assert_eq!(nodes, 28);
        assert!(edges > 80, "Should have 80+ edges, got {}", edges);
    }

    #[test]
    fn test_all_node_ids_present() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        let expected = [
            // Robotics
            "RDISC-ROB-01", "RDISC-ROB-02", "RDISC-ROB-03", "RDISC-ROB-04",
            "RDISC-ROB-05", "RDISC-ROB-06", "RDISC-ROB-07", "RHYP-ROB-01", "REXP-ROB-01",
            // Environment
            "RDISC-ENV-01", "RDISC-ENV-02", "RDISC-ENV-03", "RDISC-ENV-04",
            "RDISC-ENV-05", "RDISC-ENV-06", "RDISC-ENV-07", "RDISC-ENV-08",
            "RHYP-ENV-01", "REXP-ENV-01",
            // Bridges
            "RDISC-BRIDGE-01", "RDISC-BRIDGE-02", "RDISC-BRIDGE-03",
            "RDISC-BRIDGE-04", "RDISC-BRIDGE-05", "RDISC-BRIDGE-06",
            "RDISC-BRIDGE-07", "RHYP-BRIDGE-01", "REXP-BRIDGE-01",
        ];
        for id in &expected {
            assert!(g.nodes.iter().any(|n| n.id == *id), "Missing node {}", id);
        }
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_ROB_ENV_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }

    #[test]
    fn test_grand_bridge_is_hub() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        // RDISC-BRIDGE-07 spans 6 domains with 5 source BTs → major hub
        let hub_edges = g.edges.iter()
            .filter(|e| e.from == "RDISC-BRIDGE-07" || e.to == "RDISC-BRIDGE-07")
            .count();
        assert!(hub_edges >= 12, "RDISC-BRIDGE-07 should have 12+ edges, got {}", hub_edges);
    }

    #[test]
    fn test_hexagonal_bridge_is_hub() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        // RDISC-BRIDGE-05 spans 6 domains → major hub
        let hub_edges = g.edges.iter()
            .filter(|e| e.from == "RDISC-BRIDGE-05" || e.to == "RDISC-BRIDGE-05")
            .count();
        assert!(hub_edges >= 12, "RDISC-BRIDGE-05 should have 12+ edges, got {}", hub_edges);
    }

    #[test]
    fn test_cross_domain_merges_exist() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        let merges = g.edges.iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Merges))
            .count();
        assert!(merges >= 15, "Should have 15+ merge edges, got {}", merges);
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();
        let experiments = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Experiment)).count();

        assert_eq!(discoveries, 22);
        assert_eq!(hypotheses, 3);
        assert_eq!(experiments, 3);
        assert_eq!(discoveries + hypotheses + experiments, 28);
    }

    #[test]
    fn test_validates_edges() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        // RDISC-ROB-06 validates ROB-01, ROB-02, ROB-03
        let rob06_validates: Vec<&Edge> = g.edges.iter()
            .filter(|e| e.from == "RDISC-ROB-06" && matches!(e.edge_type, EdgeType::Validates))
            .collect();
        assert_eq!(rob06_validates.len(), 3, "RDISC-ROB-06 should validate 3 targets");
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = DiscoveryGraph::new();
        populate_rob_env_discoveries(&mut g);

        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-123" && e.to == "RDISC-ROB-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-123 --Derives--> RDISC-ROB-01 must exist");

        let has_env = g.edges.iter().any(|e| {
            e.from == "BT-118" && e.to == "RDISC-ENV-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_env, "BT-118 --Derives--> RDISC-ENV-01 must exist");
    }

    #[test]
    fn test_unique_ids() {
        let mut ids: Vec<&str> = ALL_ROB_ENV_CLUSTERS.iter()
            .flat_map(|c| c.iter().map(|e| e.id))
            .collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }
}
