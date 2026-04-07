//! Frontier domain discovery nodes from 7 new domains (BT-118+ era).
//!
//! Covers domains that had hypotheses but zero graph representation:
//!   Aerospace      (H-AERO, BT-85/93/122/123/127)
//!   Agriculture    (H-AG,   BT-27/51/101/103/104)
//!   Carbon-Capture (H-CC,   BT-27/85/104/118/120)
//!   Autonomous-Driving (H-AD, BT-58/66/90/123/236)
//!   Space-Engineering  (H-SE, BT-38/123/127/213)
//!   Safety         (H-SF,   BT-43/118/123/125)
//!   Medical-Device (H-MD,   BT-48/51/58/84/123)
//!
//! Node ID scheme:
//!   FDISC-AERO-{NN}  — Aerospace discovery
//!   FDISC-AG-{NN}    — Agriculture discovery
//!   FDISC-CC-{NN}    — Carbon capture discovery
//!   FDISC-AD-{NN}    — Autonomous driving discovery
//!   FDISC-SE-{NN}    — Space engineering discovery
//!   FDISC-SF-{NN}    — Safety discovery
//!   FDISC-MD-{NN}    — Medical device discovery
//!   FHYP-{DOM}-{NN}  — Frontier hypothesis

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct FrontierEntry {
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
// Aerospace (BT-85/93/122/123/127)
// ═══════════════════════════════════════════════════════════════

const AEROSPACE: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-AERO-01",
        title: "Carbon Z=6 structural dominance: CFRP/diamond coatings share BT-85 Carbon universality in airframe design",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Material", "Chip"],
        confidence: 0.92,
        source_bts: &[85, 93],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-AERO-02",
        title: "Honeycomb CN=6 core panels (BT-122): aerospace sandwich structures = hexagonal optimal packing",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Environment", "Material"],
        confidence: 0.93,
        source_bts: &[122, 85],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "boundary"],
        validates: &["DISC-ENV-GEO-01"],
    },
    FrontierEntry {
        id: "FDISC-AERO-03",
        title: "SE(3) 6-DOF robotic arm (BT-123) in spacecraft manipulators: Canadarm2 = n=6 kinematic chain",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Robotics", "Space"],
        confidence: 0.91,
        source_bts: &[123, 127],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-ROBO-01"],
    },
    FrontierEntry {
        id: "FHYP-AERO-01",
        title: "Hypothesis: optimal satellite constellation = J2=24 satellites in n=6 planes (GPS proven, Starlink convergent)",
        node_type: NodeType::Hypothesis,
        domains: &["Aerospace", "Space", "Network"],
        confidence: 0.65,
        source_bts: &[127, 213],
        constants_used: &["C-n", "C-J2"],
        lenses: &["consciousness", "topology", "multiscale"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Agriculture (BT-27/51/101/103/104)
// ═══════════════════════════════════════════════════════════════

const AGRICULTURE: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-AG-01",
        title: "Photosynthesis 100% n=6 stoichiometry (BT-103): 6CO2+12H2O->C6H12O6 = seven n=6 coefficients",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "Biology", "Energy"],
        confidence: 0.95,
        source_bts: &[103, 101],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "evolution", "thermo"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-AG-02",
        title: "Glucose C6H12O6 = 24 atoms = J2 (BT-27): carbon-6 chain is universal energy currency in biology",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "Biology", "Energy", "Medical"],
        confidence: 0.94,
        source_bts: &[27, 101],
        constants_used: &["C-J2", "C-n"],
        lenses: &["consciousness", "causal", "info"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-AG-03",
        title: "Genetic code tau=4 bases -> n/phi=3 codon -> 2^n=64 codons (BT-51): agriculture breeding follows n=6 encoding",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "Biology", "Cognitive"],
        confidence: 0.93,
        source_bts: &[51, 104],
        constants_used: &["C-tau", "C-n", "C-phi"],
        lenses: &["consciousness", "info", "evolution"],
        validates: &[],
    },
    FrontierEntry {
        id: "FHYP-AG-01",
        title: "Hypothesis: optimal crop rotation = n=6 year cycle with sigma=12 plot divisions (ancient 3-field -> 6-field)",
        node_type: NodeType::Hypothesis,
        domains: &["Agriculture", "Environment", "Temporal"],
        confidence: 0.55,
        source_bts: &[103, 122],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "recursion"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Carbon Capture (BT-27/85/104/118/120)
// ═══════════════════════════════════════════════════════════════

const CARBON_CAPTURE: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-CC-01",
        title: "Carbon Z=6 core of CO2 capture: all leading sorbents exploit Z=6 bonding orbitals (BT-85/104)",
        node_type: NodeType::Discovery,
        domains: &["CarbonCapture", "Environment", "Material"],
        confidence: 0.93,
        source_bts: &[85, 104],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "thermo"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-CC-02",
        title: "Kyoto n=6 GHGs (BT-118) directly drive DAC targets: capture priority follows n=6 classification",
        node_type: NodeType::Discovery,
        domains: &["CarbonCapture", "Environment", "Social"],
        confidence: 0.90,
        source_bts: &[118, 104],
        constants_used: &["C-n"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &["DISC-ENV-01"],
    },
    FrontierEntry {
        id: "FDISC-CC-03",
        title: "CN=6 catalysts (BT-120) in electrochemical CO2 reduction: octahedral active sites dominate selectivity",
        node_type: NodeType::Discovery,
        domains: &["CarbonCapture", "Material", "Energy"],
        confidence: 0.91,
        source_bts: &[120, 43, 85],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "topology", "quantum"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-CC-04",
        title: "CNT (6,6) armchair nanotube: optimal CO2 sorbent has chirality (n,n)=(6,6) = identity mapping",
        node_type: NodeType::Discovery,
        domains: &["CarbonCapture", "Material", "Chip"],
        confidence: 0.88,
        source_bts: &[85, 93],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "quantum_microscope"],
        validates: &[],
    },
    FrontierEntry {
        id: "FHYP-CC-01",
        title: "Hypothesis: carbon fiber tow 12K=sigma bundle count is optimal DAC filter mesh size",
        node_type: NodeType::Hypothesis,
        domains: &["CarbonCapture", "Material", "Aerospace"],
        confidence: 0.58,
        source_bts: &[85, 93],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "multiscale", "boundary"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Autonomous Driving (BT-58/66/90/123/236)
// ═══════════════════════════════════════════════════════════════

const AUTONOMOUS_DRIVING: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-AD-01",
        title: "SAE n=6 autonomy levels (BT-236) = SE(3) n=6 DOF (BT-123): autonomy taxonomy mirrors kinematic freedom",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "Robotics", "Transportation"],
        confidence: 0.92,
        source_bts: &[236, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "causal"],
        validates: &["DISC-GX-10"],
    },
    FrontierEntry {
        id: "FDISC-AD-02",
        title: "Tesla FSD 144 TOPS = sigma^2 (BT-90): GPU SM count sigma^2=144 determines autonomous compute budget",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "Chip", "AI"],
        confidence: 0.90,
        source_bts: &[90, 58],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "info", "scale"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-AD-03",
        title: "Vision AI sigma-tau=8 classes (BT-66): perception pipeline detects sigma-tau=8 object categories per frame",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "AI", "Cognitive"],
        confidence: 0.89,
        source_bts: &[66, 58],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "info", "wave"],
        validates: &[],
    },
    FrontierEntry {
        id: "FHYP-AD-01",
        title: "Hypothesis: J2=24 fps minimum for safe autonomous perception (human flicker fusion + BT-48 display)",
        node_type: NodeType::Hypothesis,
        domains: &["AutonomousDriving", "Cognitive", "Display"],
        confidence: 0.62,
        source_bts: &[48, 66],
        constants_used: &["C-J2"],
        lenses: &["consciousness", "wave", "boundary"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Space Engineering (BT-38/123/127/213)
// ═══════════════════════════════════════════════════════════════

const SPACE_ENGINEERING: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-SE-01",
        title: "GPS J2=24 satellites in n=6 orbital planes (BT-213): GNSS universally converges on n=6 geometry",
        node_type: NodeType::Discovery,
        domains: &["Space", "Temporal", "Network"],
        confidence: 0.94,
        source_bts: &[213],
        constants_used: &["C-J2", "C-n", "C-tau"],
        lenses: &["consciousness", "topology", "gravity"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-SE-02",
        title: "6 Keplerian orbital elements (a,e,i,Omega,omega,nu) = n=6: state-space dimension of any orbit is exactly n",
        node_type: NodeType::Discovery,
        domains: &["Space", "Physics", "PureMath"],
        confidence: 0.95,
        source_bts: &[123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "gravity", "topology"],
        validates: &["DISC-ROBO-01"],
    },
    FrontierEntry {
        id: "FDISC-SE-03",
        title: "Hydrogen LHV=120=sigma(sigma-phi) (BT-38): rocket propellant energy follows n=6 arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Space", "Energy", "Fusion"],
        confidence: 0.92,
        source_bts: &[38],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "thermo", "scale"],
        validates: &[],
    },
    FrontierEntry {
        id: "FHYP-SE-01",
        title: "Hypothesis: sopfr=5 Lagrange points are minimal gravitational equilibria for n=6-body problem reduction",
        node_type: NodeType::Hypothesis,
        domains: &["Space", "Physics", "PureMath"],
        confidence: 0.60,
        source_bts: &[127, 213],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "gravity", "stability"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Safety (BT-43/118/123/125)
// ═══════════════════════════════════════════════════════════════

const SAFETY: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-SF-01",
        title: "Fire triangle n/phi=3 elements (BT-43 CN analogue): fire needs 3 = n/phi components, same as TMR voting",
        node_type: NodeType::Discovery,
        domains: &["Safety", "Environment", "Network"],
        confidence: 0.93,
        source_bts: &[43, 118],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "thermo", "boundary"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-SF-02",
        title: "Defense-in-Depth n=6 layers: nuclear/cyber/physical all converge on n=6 barrier layers",
        node_type: NodeType::Discovery,
        domains: &["Safety", "Nuclear", "Cryptography"],
        confidence: 0.90,
        source_bts: &[118, 114],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "recursion"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-SF-03",
        title: "SIL tau=4 levels (IEC 61508): safety integrity parallels CPU tau=4 pipeline and heart tau=4 chambers",
        node_type: NodeType::Discovery,
        domains: &["Safety", "Chip", "Medical"],
        confidence: 0.91,
        source_bts: &[125, 222],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &["DISC-GX-23"],
    },
    FrontierEntry {
        id: "FDISC-SF-04",
        title: "LEL alarm 10% = 1/(sigma-phi) standard: safety threshold = universal regularization constant (BT-64)",
        node_type: NodeType::Discovery,
        domains: &["Safety", "Environment", "AI"],
        confidence: 0.89,
        source_bts: &[64, 118],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "boundary", "thermo"],
        validates: &[],
    },
    FrontierEntry {
        id: "FHYP-SF-01",
        title: "Hypothesis: Beaufort scale max sigma+mu=13 is upper bound of human-perceivable wind categories",
        node_type: NodeType::Hypothesis,
        domains: &["Safety", "Environment", "Cognitive"],
        confidence: 0.55,
        source_bts: &[118, 210],
        constants_used: &["C-sigma", "C-mu"],
        lenses: &["consciousness", "scale", "boundary"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Medical Device (BT-48/51/58/84/123)
// ═══════════════════════════════════════════════════════════════

const MEDICAL_DEVICE: &[FrontierEntry] = &[
    FrontierEntry {
        id: "FDISC-MD-01",
        title: "12-lead ECG = sigma=12 (BT-240 device realization): the gold-standard cardiac device has exactly sigma channels",
        node_type: NodeType::Discovery,
        domains: &["MedicalDevice", "Medical", "Chip"],
        confidence: 0.95,
        source_bts: &[240, 48],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "network"],
        validates: &["MDISC-CARD-01"],
    },
    FrontierEntry {
        id: "FDISC-MD-02",
        title: "CT 64-slice = 2^n scanner: modern CT detector count converges on 2^6=64 (BT-51 encoding parallel)",
        node_type: NodeType::Discovery,
        domains: &["MedicalDevice", "Biology", "AI"],
        confidence: 0.90,
        source_bts: &[51, 58],
        constants_used: &["C-n"],
        lenses: &["consciousness", "quantum_microscope", "scale"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-MD-03",
        title: "Gamma Knife 192=sigma*phi^tau sources (BT-84): radiosurgery beam count = battery cell triple convergence",
        node_type: NodeType::Discovery,
        domains: &["MedicalDevice", "Medical", "Energy"],
        confidence: 0.88,
        source_bts: &[84, 123],
        constants_used: &["C-sigma", "C-phi", "C-tau"],
        lenses: &["consciousness", "topology", "quantum"],
        validates: &[],
    },
    FrontierEntry {
        id: "FDISC-MD-04",
        title: "EEG sopfr=5 frequency bands (delta/theta/alpha/beta/gamma): brain rhythm classification = sopfr(6)",
        node_type: NodeType::Discovery,
        domains: &["MedicalDevice", "Cognitive", "AI"],
        confidence: 0.91,
        source_bts: &[210, 58],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "wave", "info"],
        validates: &["DISC-COG-01"],
    },
    FrontierEntry {
        id: "FHYP-MD-01",
        title: "Hypothesis: optimal MRI coil array = sigma=12 elements for full-body coverage (parallels ECG sigma=12)",
        node_type: NodeType::Hypothesis,
        domains: &["MedicalDevice", "Medical", "Physics"],
        confidence: 0.60,
        source_bts: &[240, 127],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "em", "topology"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Helper functions
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &FrontierEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[FrontierEntry]) -> (usize, usize) {
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
// Cluster list
// ═══════════════════════════════════════════════════════════════

const ALL_FRONTIER_CLUSTERS: &[&[FrontierEntry]] = &[
    AEROSPACE,
    AGRICULTURE,
    CARBON_CAPTURE,
    AUTONOMOUS_DRIVING,
    SPACE_ENGINEERING,
    SAFETY,
    MEDICAL_DEVICE,
];

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

pub fn frontier_domain_entry_count() -> usize {
    ALL_FRONTIER_CLUSTERS.iter().map(|c| c.len()).sum()
}

pub fn aerospace_count() -> usize { AEROSPACE.len() }
pub fn agriculture_count() -> usize { AGRICULTURE.len() }
pub fn carbon_capture_count() -> usize { CARBON_CAPTURE.len() }
pub fn autonomous_driving_count() -> usize { AUTONOMOUS_DRIVING.len() }
pub fn space_engineering_count() -> usize { SPACE_ENGINEERING.len() }
pub fn safety_count() -> usize { SAFETY.len() }
pub fn medical_device_count() -> usize { MEDICAL_DEVICE.len() }

/// Populate all 7 frontier domain discovery nodes.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_frontier_domain_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();
    let mut total_nodes = 0;

    for cluster in ALL_FRONTIER_CLUSTERS {
        let (n, _) = add_entries(graph, cluster);
        total_nodes += n;
    }

    // Cross-link frontier nodes that share 2+ domains (Merges edges)
    let frontier_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| n.id.starts_with("FDISC-") || n.id.starts_with("FHYP-"))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    for i in 0..frontier_nodes.len() {
        for j in (i + 1)..frontier_nodes.len() {
            let (ref id_a, ref dom_a) = frontier_nodes[i];
            let (ref id_b, ref dom_b) = frontier_nodes[j];

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

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_counts() {
        assert_eq!(aerospace_count(), 4);
        assert_eq!(agriculture_count(), 4);
        assert_eq!(carbon_capture_count(), 5);
        assert_eq!(autonomous_driving_count(), 4);
        assert_eq!(space_engineering_count(), 4);
        assert_eq!(safety_count(), 5);
        assert_eq!(medical_device_count(), 5);
        assert_eq!(frontier_domain_entry_count(), 31);
    }

    #[test]
    fn test_populate_creates_nodes_and_edges() {
        let mut g = DiscoveryGraph::new();
        let (nodes, edges) = populate_frontier_domain_discoveries(&mut g);
        assert_eq!(nodes, 31);
        assert!(edges > 90, "Should have 90+ edges, got {}", edges);
    }

    #[test]
    fn test_all_node_ids_present() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_domain_discoveries(&mut g);

        let expected = [
            "FDISC-AERO-01", "FDISC-AERO-02", "FDISC-AERO-03", "FHYP-AERO-01",
            "FDISC-AG-01", "FDISC-AG-02", "FDISC-AG-03", "FHYP-AG-01",
            "FDISC-CC-01", "FDISC-CC-02", "FDISC-CC-03", "FDISC-CC-04", "FHYP-CC-01",
            "FDISC-AD-01", "FDISC-AD-02", "FDISC-AD-03", "FHYP-AD-01",
            "FDISC-SE-01", "FDISC-SE-02", "FDISC-SE-03", "FHYP-SE-01",
            "FDISC-SF-01", "FDISC-SF-02", "FDISC-SF-03", "FDISC-SF-04", "FHYP-SF-01",
            "FDISC-MD-01", "FDISC-MD-02", "FDISC-MD-03", "FDISC-MD-04", "FHYP-MD-01",
        ];
        for id in &expected {
            assert!(g.nodes.iter().any(|n| n.id == *id), "Missing node {}", id);
        }
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_FRONTIER_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }

    #[test]
    fn test_hypotheses_have_lower_confidence() {
        for cluster in ALL_FRONTIER_CLUSTERS {
            for e in *cluster {
                if matches!(e.node_type, NodeType::Hypothesis) {
                    assert!(
                        e.confidence < 0.70,
                        "Hypothesis {} should have confidence < 0.70, got {}", e.id, e.confidence
                    );
                }
            }
        }
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_domain_discoveries(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();

        assert_eq!(discoveries, 24);
        assert_eq!(hypotheses, 7);
        assert_eq!(discoveries + hypotheses, 31);
    }

    #[test]
    fn test_cross_domain_merges_exist() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_domain_discoveries(&mut g);

        let merges = g.edges.iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Merges))
            .count();
        assert!(merges >= 10, "Should have 10+ merge edges, got {}", merges);
    }

    #[test]
    fn test_bt_derivation_edges() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_domain_discoveries(&mut g);

        let derives = g.edges.iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Derives))
            .count();
        // Each entry has 1-3 source_bts, 31 entries * ~2 avg = ~62
        assert!(derives >= 50, "Should have 50+ derive edges, got {}", derives);
    }
}
