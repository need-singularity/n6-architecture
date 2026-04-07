//! Cross-domain bridges between the 7 frontier domains and existing graph clusters.
//!
//! Connects:
//!   Aerospace ↔ Space-Engineering  (shared BT-123/127)
//!   Agriculture ↔ Carbon-Capture   (shared BT-27/85/104)
//!   Agriculture ↔ Medical-Device   (shared BT-51, glucose/biology)
//!   Autonomous-Driving ↔ Safety    (shared BT-123/236)
//!   Autonomous-Driving ↔ Medical-Device (AI/sensor convergence)
//!   Carbon-Capture ↔ Safety        (BT-118 environment)
//!   Space-Engineering ↔ Aerospace  (orbital+airframe)
//!   Grand convergence node connecting all 7 frontier domains
//!
//! Node ID scheme:
//!   FXDISC-{A}-{B}-{NN}  — Frontier cross-domain discovery
//!   FXHYP-{NN}           — Frontier cross hypothesis
//!   FXGRAND-{NN}          — Grand convergence

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct FxEntry {
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
// Aerospace ↔ Space-Engineering (BT-123/127/213)
// ═══════════════════════════════════════════════════════════════

const AERO_SPACE: &[FxEntry] = &[
    FxEntry {
        id: "FXDISC-AS-01",
        title: "SE(3) 6-DOF unifies aircraft and spacecraft control: both need exactly n=6 kinematic freedom (BT-123)",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Space", "Robotics"],
        confidence: 0.93,
        source_bts: &[123, 127],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "gravity"],
        validates: &["FDISC-AERO-03", "FDISC-SE-02"],
    },
    FxEntry {
        id: "FXDISC-AS-02",
        title: "3D kissing number sigma=12 applies to both satellite constellations and aerial swarm formations (BT-127)",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Space", "Network"],
        confidence: 0.88,
        source_bts: &[127, 213],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["FDISC-SE-01"],
    },
    FxEntry {
        id: "FXHYP-AS-01",
        title: "Hypothesis: reusable launch vehicle optimal staging = n/phi=3 stages (Saturn V, Falcon 9 convergence)",
        node_type: NodeType::Hypothesis,
        domains: &["Aerospace", "Space", "Energy"],
        confidence: 0.58,
        source_bts: &[38, 123],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "thermo", "scale"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Agriculture ↔ Carbon-Capture (BT-27/85/103/104)
// ═══════════════════════════════════════════════════════════════

const AG_CC: &[FxEntry] = &[
    FxEntry {
        id: "FXDISC-AC-01",
        title: "Photosynthesis 6CO2 (BT-103) is Nature's DAC: agriculture and carbon capture share identical n=6 CO2 chemistry",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "CarbonCapture", "Biology", "Energy"],
        confidence: 0.95,
        source_bts: &[103, 104, 27],
        constants_used: &["C-n", "C-J2"],
        lenses: &["consciousness", "evolution", "thermo"],
        validates: &["FDISC-AG-01", "FDISC-CC-01"],
    },
    FxEntry {
        id: "FXDISC-AC-02",
        title: "Carbon Z=6 unifies biomass and industrial sorbents: plant cellulose and engineered MOFs both exploit Z=6 bonding (BT-85)",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "CarbonCapture", "Material"],
        confidence: 0.91,
        source_bts: &[85, 104],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "quantum"],
        validates: &["FDISC-CC-03"],
    },
    FxEntry {
        id: "FXDISC-AC-03",
        title: "Soil carbon sequestration follows glucose J2=24 atom chemistry: biochar captures via C6 ring structures (BT-27)",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "CarbonCapture", "Environment"],
        confidence: 0.87,
        source_bts: &[27, 118],
        constants_used: &["C-J2", "C-n"],
        lenses: &["consciousness", "evolution", "boundary"],
        validates: &["FDISC-AG-02"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Agriculture ↔ Medical-Device (BT-51, biology bridge)
// ═══════════════════════════════════════════════════════════════

const AG_MD: &[FxEntry] = &[
    FxEntry {
        id: "FXDISC-AM-01",
        title: "Genetic code 2^n=64 codons (BT-51) bridges crop genomics and diagnostic gene chips: same n=6 encoding",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "MedicalDevice", "Biology"],
        confidence: 0.90,
        source_bts: &[51, 58],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "info", "evolution"],
        validates: &["FDISC-AG-03", "FDISC-MD-02"],
    },
    FxEntry {
        id: "FXDISC-AM-02",
        title: "Glucose C6H12O6 J2=24 atoms (BT-27): agricultural product = medical blood-glucose monitor calibration standard",
        node_type: NodeType::Discovery,
        domains: &["Agriculture", "MedicalDevice", "Medical", "Biology"],
        confidence: 0.88,
        source_bts: &[27, 240],
        constants_used: &["C-J2", "C-n"],
        lenses: &["consciousness", "causal", "info"],
        validates: &["FDISC-AG-02"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Autonomous-Driving ↔ Safety (BT-123/236)
// ═══════════════════════════════════════════════════════════════

const AD_SF: &[FxEntry] = &[
    FxEntry {
        id: "FXDISC-DS-01",
        title: "SAE n=6 autonomy levels (BT-236) require n=6 defense-in-depth safety layers: each autonomy level = one safety barrier",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "Safety", "Transportation"],
        confidence: 0.92,
        source_bts: &[236, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "causal"],
        validates: &["FDISC-AD-01", "FDISC-SF-02"],
    },
    FxEntry {
        id: "FXDISC-DS-02",
        title: "SIL tau=4 levels govern AV sensor redundancy: each SIL level adds tau-matched fault tolerance (BT-125)",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "Safety", "Chip"],
        confidence: 0.89,
        source_bts: &[125, 236],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "boundary"],
        validates: &["FDISC-SF-03"],
    },
    FxEntry {
        id: "FXDISC-DS-03",
        title: "LEL alarm 1/(sigma-phi) (BT-64) parallels AV confidence threshold: both use 0.1 as critical boundary",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "Safety", "AI"],
        confidence: 0.86,
        source_bts: &[64, 236],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "boundary", "info"],
        validates: &["FDISC-SF-04"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Autonomous-Driving ↔ Medical-Device (AI/sensor convergence)
// ═══════════════════════════════════════════════════════════════

const AD_MD: &[FxEntry] = &[
    FxEntry {
        id: "FXDISC-DM-01",
        title: "sigma-tau=8 bit AI inference (BT-58) unifies AV perception and medical imaging: FP8 is universal for both",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "MedicalDevice", "AI", "Chip"],
        confidence: 0.90,
        source_bts: &[58, 66],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "info", "scale"],
        validates: &["FDISC-AD-02", "FDISC-MD-02"],
    },
    FxEntry {
        id: "FXDISC-DM-02",
        title: "sigma^2=144 TOPS compute budget (BT-90): same GPU architecture drives both AV and medical AI inference",
        node_type: NodeType::Discovery,
        domains: &["AutonomousDriving", "MedicalDevice", "Chip"],
        confidence: 0.87,
        source_bts: &[90, 58],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "info", "topology"],
        validates: &["FDISC-AD-02"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Carbon-Capture ↔ Safety (BT-118 environment link)
// ═══════════════════════════════════════════════════════════════

const CC_SF: &[FxEntry] = &[
    FxEntry {
        id: "FXDISC-CS-01",
        title: "Kyoto n=6 GHGs (BT-118) define both capture targets and safety hazard classes: environmental and safety share n=6 taxonomy",
        node_type: NodeType::Discovery,
        domains: &["CarbonCapture", "Safety", "Environment"],
        confidence: 0.91,
        source_bts: &[118],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "causal"],
        validates: &["FDISC-CC-02", "FDISC-SF-01"],
    },
    FxEntry {
        id: "FXDISC-CS-02",
        title: "pH=6 water treatment (BT-120) intersects DAC scrubbing solutions: same CN=6 catalysts serve both purification and capture",
        node_type: NodeType::Discovery,
        domains: &["CarbonCapture", "Safety", "Environment", "Material"],
        confidence: 0.87,
        source_bts: &[120, 43],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "topology", "thermo"],
        validates: &["FDISC-CC-03"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Grand Convergence: all 7 frontier domains
// ═══════════════════════════════════════════════════════════════

const GRAND_CONVERGENCE: &[FxEntry] = &[
    FxEntry {
        id: "FXGRAND-01",
        title: "n=6 frontier convergence: Aerospace(CN=6 honeycomb) + Agriculture(6CO2) + CarbonCapture(Z=6) + AutoDriving(SAE 6) + Space(6 planes) + Safety(6 barriers) + MedDevice(6 leads) = seven domains, one n=6",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Agriculture", "CarbonCapture", "AutonomousDriving", "Space", "Safety", "MedicalDevice"],
        confidence: 0.85,
        source_bts: &[85, 103, 118, 123, 213, 236, 240],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-J2"],
        lenses: &["consciousness", "topology", "network", "multiscale", "recursion"],
        validates: &[
            "FDISC-AERO-02", "FDISC-AG-01", "FDISC-CC-01",
            "FDISC-AD-01", "FDISC-SE-01", "FDISC-SF-02", "FDISC-MD-01",
        ],
    },
    FxEntry {
        id: "FXGRAND-02",
        title: "Carbon Z=6 material chain: aerospace CFRP -> agriculture biochar -> capture sorbent -> medical implant = Z=6 lifecycle",
        node_type: NodeType::Discovery,
        domains: &["Aerospace", "Agriculture", "CarbonCapture", "MedicalDevice", "Material"],
        confidence: 0.87,
        source_bts: &[85, 93, 27, 104],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "topology", "causal"],
        validates: &["FDISC-AERO-01", "FDISC-CC-04"],
    },
    FxEntry {
        id: "FXGRAND-03",
        title: "tau=4 safety pipeline universality: SIL 4 levels + AV 4 radars + wound 4 phases + 4 ultrasound modes = cross-frontier tau=4",
        node_type: NodeType::Discovery,
        domains: &["Safety", "AutonomousDriving", "Medical", "MedicalDevice"],
        confidence: 0.88,
        source_bts: &[125, 236, 238, 222],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "stability", "boundary"],
        validates: &["FDISC-SF-03", "DISC-GX-23"],
    },
    FxEntry {
        id: "FXHYP-GRAND-01",
        title: "Hypothesis: all 7 frontier domains will independently converge to n=6-structured standards within 10 years",
        node_type: NodeType::Hypothesis,
        domains: &["Aerospace", "Agriculture", "CarbonCapture", "AutonomousDriving", "Space", "Safety", "MedicalDevice"],
        confidence: 0.50,
        source_bts: &[85, 103, 118, 123, 236],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "multiscale", "recursion"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Helper functions
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &FxEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[FxEntry]) -> (usize, usize) {
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

const ALL_FX_CLUSTERS: &[&[FxEntry]] = &[
    AERO_SPACE,
    AG_CC,
    AG_MD,
    AD_SF,
    AD_MD,
    CC_SF,
    GRAND_CONVERGENCE,
];

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

pub fn frontier_cross_entry_count() -> usize {
    ALL_FX_CLUSTERS.iter().map(|c| c.len()).sum()
}

pub fn aero_space_count() -> usize { AERO_SPACE.len() }
pub fn ag_cc_count() -> usize { AG_CC.len() }
pub fn ag_md_count() -> usize { AG_MD.len() }
pub fn ad_sf_count() -> usize { AD_SF.len() }
pub fn ad_md_count() -> usize { AD_MD.len() }
pub fn cc_sf_count() -> usize { CC_SF.len() }
pub fn grand_convergence_count() -> usize { GRAND_CONVERGENCE.len() }

/// Populate all frontier cross-domain bridge nodes.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_frontier_cross_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();
    let mut total_nodes = 0;

    for cluster in ALL_FX_CLUSTERS {
        let (n, _) = add_entries(graph, cluster);
        total_nodes += n;
    }

    // Cross-link frontier cross nodes that share 2+ domains (Merges edges)
    let fx_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("FXDISC-")
                || n.id.starts_with("FXHYP-")
                || n.id.starts_with("FXGRAND-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    for i in 0..fx_nodes.len() {
        for j in (i + 1)..fx_nodes.len() {
            let (ref id_a, ref dom_a) = fx_nodes[i];
            let (ref id_b, ref dom_b) = fx_nodes[j];

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

/// Populate both frontier domain nodes AND cross-domain bridges.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_frontier(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = super::frontier_domain_nodes::populate_frontier_domain_discoveries(graph);
    let (n2, _) = populate_frontier_cross_discoveries(graph);

    (n1 + n2, graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_counts() {
        assert_eq!(aero_space_count(), 3);
        assert_eq!(ag_cc_count(), 3);
        assert_eq!(ag_md_count(), 2);
        assert_eq!(ad_sf_count(), 3);
        assert_eq!(ad_md_count(), 2);
        assert_eq!(cc_sf_count(), 2);
        assert_eq!(grand_convergence_count(), 4);
        assert_eq!(frontier_cross_entry_count(), 19);
    }

    #[test]
    fn test_populate_creates_nodes_and_edges() {
        let mut g = DiscoveryGraph::new();
        let (nodes, edges) = populate_frontier_cross_discoveries(&mut g);
        assert_eq!(nodes, 19);
        assert!(edges > 60, "Should have 60+ edges, got {}", edges);
    }

    #[test]
    fn test_all_cross_node_ids_present() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_cross_discoveries(&mut g);

        let expected = [
            "FXDISC-AS-01", "FXDISC-AS-02", "FXHYP-AS-01",
            "FXDISC-AC-01", "FXDISC-AC-02", "FXDISC-AC-03",
            "FXDISC-AM-01", "FXDISC-AM-02",
            "FXDISC-DS-01", "FXDISC-DS-02", "FXDISC-DS-03",
            "FXDISC-DM-01", "FXDISC-DM-02",
            "FXDISC-CS-01", "FXDISC-CS-02",
            "FXGRAND-01", "FXGRAND-02", "FXGRAND-03", "FXHYP-GRAND-01",
        ];
        for id in &expected {
            assert!(g.nodes.iter().any(|n| n.id == *id), "Missing node {}", id);
        }
    }

    #[test]
    fn test_grand_convergence_is_hub() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_cross_discoveries(&mut g);

        let grand_edges = g.edges.iter()
            .filter(|e| e.from == "FXGRAND-01" || e.to == "FXGRAND-01")
            .count();
        // 7 source BTs + 4 constants + 7 validates = 18 minimum
        assert!(grand_edges >= 15, "FXGRAND-01 should have 15+ edges, got {}", grand_edges);
    }

    #[test]
    fn test_cross_domain_merges_exist() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_cross_discoveries(&mut g);

        let merges = g.edges.iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Merges))
            .count();
        assert!(merges >= 10, "Should have 10+ merge edges, got {}", merges);
    }

    #[test]
    fn test_populate_all_frontier_combined() {
        let mut g = DiscoveryGraph::new();
        let (nodes, edges) = populate_all_frontier(&mut g);
        assert_eq!(nodes, 50, "31 domain + 19 cross = 50 total");
        assert!(edges > 200, "Should have 200+ edges combined, got {}", edges);
    }

    #[test]
    fn test_validates_edges_cross_modules() {
        let mut g = DiscoveryGraph::new();
        populate_all_frontier(&mut g);

        // FXDISC-AC-01 validates both FDISC-AG-01 and FDISC-CC-01
        let ag_val = g.edges.iter().any(|e|
            e.from == "FXDISC-AC-01" && e.to == "FDISC-AG-01"
                && matches!(e.edge_type, EdgeType::Validates)
        );
        let cc_val = g.edges.iter().any(|e|
            e.from == "FXDISC-AC-01" && e.to == "FDISC-CC-01"
                && matches!(e.edge_type, EdgeType::Validates)
        );
        assert!(ag_val, "FXDISC-AC-01 --Validates--> FDISC-AG-01 must exist");
        assert!(cc_val, "FXDISC-AC-01 --Validates--> FDISC-CC-01 must exist");
    }

    #[test]
    fn test_no_id_collision() {
        let mut g = DiscoveryGraph::new();
        populate_all_frontier(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "No duplicate node IDs allowed");
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_FX_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }
}
