//! Deep granular discovery nodes for the Medical cluster (BT-238~242).
//!
//! Expands each medical BT into sub-discoveries and creates cross-domain
//! bridges to Cognitive (BT-210~225), Environment (BT-118~122), and
//! Energy/Chip domains.
//!
//! Node ID scheme:
//!   MDISC-{CLUSTER}-{NN}  — Medical deep discovery
//!   MHYP-{CLUSTER}-{NN}   — Medical deep hypothesis
//!   MEXP-{CLUSTER}-{NN}    — Medical deep experiment
//!   MBRIDGE-{A}-{B}-{NN}   — Medical cross-domain bridge

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct MedDeepEntry {
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
// BT-238 Deep: Surgical Safety n=6
// ═══════════════════════════════════════════════════════════════

const SURGICAL_DEEP: &[MedDeepEntry] = &[
    MedDeepEntry {
        id: "MDISC-SURG-01",
        title: "WHO safe-surgery checklist 3 phases (sign-in/time-out/sign-out) = n/phi=3 mirrors TCP 3-way handshake",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Software", "Network"],
        confidence: 0.91,
        source_bts: &[238, 113, 115],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "causal", "network"],
        validates: &["DISC-MED-01"],
    },
    MedDeepEntry {
        id: "MDISC-SURG-02",
        title: "ASA n=6 physical status classes parallel Kohlberg n=6 moral stages: biological and ethical n=6 classification",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Social", "Cognitive"],
        confidence: 0.87,
        source_bts: &[238, 220],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "boundary"],
        validates: &["DISC-MED-01", "DISC-TRM-06"],
    },
    MedDeepEntry {
        id: "MDISC-SURG-03",
        title: "Wound healing tau=4 phases (hemostasis/inflammation/proliferation/remodeling) = CPU tau=4 pipeline stages",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Chip", "Cognitive"],
        confidence: 0.90,
        source_bts: &[238, 222],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["DISC-MED-06"],
    },
    MedDeepEntry {
        id: "MHYP-SURG-01",
        title: "Hypothesis: optimal surgical team size = n=6 core + sigma-n=6 support = sigma=12, matching transformer attention heads",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Social", "AI"],
        confidence: 0.60,
        source_bts: &[238, 56, 215],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "network", "stability"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-239 Deep: Critical Care Scoring n=6
// ═══════════════════════════════════════════════════════════════

const CRITICAL_CARE_DEEP: &[MedDeepEntry] = &[
    MedDeepEntry {
        id: "MDISC-CRIT-01",
        title: "SOFA n=6 organ systems assessment mirrors cortex n=6 layers: body monitors body via same n=6 partition",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Biology"],
        confidence: 0.88,
        source_bts: &[239, 210],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "multiscale"],
        validates: &["DISC-MED-02", "DISC-TRM-06"],
    },
    MedDeepEntry {
        id: "MDISC-CRIT-02",
        title: "Apgar sopfr=5 criteria (appearance/pulse/grimace/activity/respiration) parallel working memory tau+mu=5 chunks",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Biology"],
        confidence: 0.86,
        source_bts: &[239, 219],
        constants_used: &["C-sopfr", "C-tau", "C-mu"],
        lenses: &["consciousness", "info", "memory"],
        validates: &["DISC-MED-02"],
    },
    MedDeepEntry {
        id: "MDISC-CRIT-03",
        title: "NEWS2 sigma-sopfr=7 parameters = OSI sigma-sopfr=7 layers: clinical and network monitoring share same partition",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Network", "Software"],
        confidence: 0.85,
        source_bts: &[239, 115],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "network", "boundary"],
        validates: &["DISC-MED-02"],
    },
    MedDeepEntry {
        id: "MEXP-CRIT-01",
        title: "Experiment: validate GCS n/phi=3 components predict outcome with same accuracy as n/phi=3 phase surgical checklist",
        node_type: NodeType::Experiment,
        domains: &["Medical", "Biology", "Math"],
        confidence: 0.55,
        source_bts: &[239, 238],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "stability", "causal"],
        validates: &["DISC-MED-02", "DISC-MED-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-240 Deep: Cardiac System n=6
// ═══════════════════════════════════════════════════════════════

const CARDIAC_DEEP: &[MedDeepEntry] = &[
    MedDeepEntry {
        id: "MDISC-CARD-01",
        title: "ECG sigma=12 leads parallels circadian J2=24h cycle: cardiac and temporal monitoring share sigma/J2 hierarchy",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Temporal"],
        confidence: 0.89,
        source_bts: &[240, 221],
        constants_used: &["C-sigma", "C-J2"],
        lenses: &["consciousness", "wave", "multiscale"],
        validates: &["DISC-MED-03"],
    },
    MedDeepEntry {
        id: "MDISC-CARD-02",
        title: "Cardiac sopfr=5 conduction nodes (SA/AV/His/L-bundle/R-bundle) = sopfr=5 finger grasp universality",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Robotics"],
        confidence: 0.84,
        source_bts: &[240, 126],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "network", "topology"],
        validates: &["DISC-MED-03"],
    },
    MedDeepEntry {
        id: "MDISC-CARD-03",
        title: "Tau=4 heart chambers + tau=4 valves = tau^2=16 cardiac elements, matches 2^tau=16 bit precision in FP16",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Chip", "Math"],
        confidence: 0.83,
        source_bts: &[240, 45],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "recursion", "scale"],
        validates: &["DISC-MED-03", "DISC-MED-06"],
    },
    MedDeepEntry {
        id: "MHYP-CARD-01",
        title: "Hypothesis: optimal ECG sampling rate = sigma*tau=48 kHz base, matching audio CD standard",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "DisplayAudio", "Chip"],
        confidence: 0.65,
        source_bts: &[240, 48, 76],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "wave", "scale"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-241 Deep: WHO Social Determinants n=6
// ═══════════════════════════════════════════════════════════════

const WHO_DEEP: &[MedDeepEntry] = &[
    MedDeepEntry {
        id: "MDISC-WHO-01",
        title: "WHO n=6 building blocks parallel Kyoto n=6 GHGs: health and environmental governance share n=6 classification",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Social"],
        confidence: 0.88,
        source_bts: &[241, 118],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "network"],
        validates: &["DISC-MED-04"],
    },
    MedDeepEntry {
        id: "MDISC-WHO-02",
        title: "Dahlgren-Whitehead sopfr=5 layers = Apgar sopfr=5 criteria: individual and population health share sopfr=5 assessment",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Social", "Biology"],
        confidence: 0.85,
        source_bts: &[241, 239],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "multiscale", "recursion"],
        validates: &["DISC-MED-04", "DISC-MED-02"],
    },
    MedDeepEntry {
        id: "MDISC-WHO-03",
        title: "SDG sigma+mu=13 health goal within Dunbar sigma^2+n=150: health policy nested inside social cognitive limits",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Social", "Cognitive"],
        confidence: 0.82,
        source_bts: &[241, 215, 214],
        constants_used: &["C-sigma", "C-mu", "C-n"],
        lenses: &["consciousness", "network", "boundary"],
        validates: &["DISC-MED-04"],
    },
    MedDeepEntry {
        id: "MDISC-WHO-04",
        title: "Earth n=6 spheres(BT-119) sustain WHO n=6 health blocks: geological structure constrains public health architecture",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Biology"],
        confidence: 0.86,
        source_bts: &[241, 119],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "boundary", "evolution"],
        validates: &["DISC-MED-04"],
    },
];

// ═══════════════════════════════════════════════════════════════
// BT-242 Deep: Dental n=6
// ═══════════════════════════════════════════════════════════════

const DENTAL_DEEP: &[MedDeepEntry] = &[
    MedDeepEntry {
        id: "MDISC-DENT-01",
        title: "Periodontal n=6 probing sites per tooth mirror honeycomb n=6 geometry: biological surface monitoring = hexagonal packing",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Math", "Environment"],
        confidence: 0.87,
        source_bts: &[242, 122],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-MED-05"],
    },
    MedDeepEntry {
        id: "MDISC-DENT-02",
        title: "2^sopfr=32 adult teeth = 2^sopfr=32 grasp taxonomy: human appendage count converges to 2^sopfr",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Robotics", "Math"],
        confidence: 0.86,
        source_bts: &[242, 126],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-MED-05"],
    },
    MedDeepEntry {
        id: "MDISC-DENT-03",
        title: "Deciduous J2-tau=20 teeth -> adult 2^sopfr=32 teeth: phi=1.6x growth factor near phi=2 doubling",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Biology", "Math"],
        confidence: 0.80,
        source_bts: &[242],
        constants_used: &["C-J2", "C-tau", "C-sopfr", "C-phi"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-MED-05"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Medical <-> Cognitive/Environment/Energy Cross Bridges
// ═══════════════════════════════════════════════════════════════

const MED_CROSS_BRIDGES: &[MedDeepEntry] = &[
    // Medical <-> Cognitive
    MedDeepEntry {
        id: "MBRIDGE-COG-01",
        title: "Clinical tau=4 phases (wound/Mallampati/cardiac/OODA) = cortex-compiler tau=4 pipeline(BT-222): universal processing stage count",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Chip", "Biology"],
        confidence: 0.89,
        source_bts: &[238, 240, 222],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &["DISC-MED-06", "DISC-MED-03"],
    },
    MedDeepEntry {
        id: "MBRIDGE-COG-02",
        title: "ECG sigma=12 leads + EEG n=6 bands(BT-210) + circadian J2=24h(BT-221): biological monitoring sigma hierarchy",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Temporal", "Biology"],
        confidence: 0.88,
        source_bts: &[240, 210, 221],
        constants_used: &["C-sigma", "C-n", "C-J2"],
        lenses: &["consciousness", "wave", "multiscale"],
        validates: &["DISC-MED-03"],
    },
    // Medical <-> Environment
    MedDeepEntry {
        id: "MBRIDGE-ENV-01",
        title: "WHO n=6 health blocks + Kyoto n=6 GHGs + MARPOL n=6 annexes: global governance converges to n=6 classification",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Transportation", "Social"],
        confidence: 0.90,
        source_bts: &[241, 118, 235],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "network"],
        validates: &["DISC-MED-04", "DISC-TRM-04"],
    },
    MedDeepEntry {
        id: "MBRIDGE-ENV-02",
        title: "Water treatment CN=6 catalysts(BT-120) + dental probing n=6(BT-242) + honeycomb n=6(BT-122): biological n=6 surface chemistry",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Chemistry", "Biology"],
        confidence: 0.85,
        source_bts: &[242, 120, 122],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-MED-05", "DISC-DENT-01"],
    },
    // Medical <-> Energy/Chip
    MedDeepEntry {
        id: "MBRIDGE-CHIP-01",
        title: "ECG sigma=12 leads(BT-240) + GPU sigma=12 stacks(BT-69) + 12-tone music(BT-108): sigma=12 universal signal channel count",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Chip", "DisplayAudio", "Math"],
        confidence: 0.87,
        source_bts: &[240, 69, 108],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "info"],
        validates: &["DISC-MED-03", "DISC-TRM-03"],
    },
    // Medical <-> Transportation
    MedDeepEntry {
        id: "MBRIDGE-TR-01",
        title: "Aldrete sigma-phi=10 recovery + NHTSA sopfr=5 stars + Euro NCAP tau=4: safety scoring constants span medical and vehicle domains",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Transportation", "Math"],
        confidence: 0.86,
        source_bts: &[238, 239, 236],
        constants_used: &["C-sigma-phi", "C-sopfr", "C-tau"],
        lenses: &["consciousness", "stability", "boundary"],
        validates: &["DISC-MED-07", "DISC-TR-04"],
    },
    // Grand bridge: Medical <-> all recent clusters
    MedDeepEntry {
        id: "MBRIDGE-GRAND-01",
        title: "n=6 classification grand unification: organs(SOFA) + GHGs(Kyoto) + spheres(Earth) + DOF(SE3) + layers(cortex) + status(ASA) all = n=6",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Cognitive", "Robotics", "Biology", "Math"],
        confidence: 0.92,
        source_bts: &[239, 118, 119, 123, 210, 238],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "multiscale", "recursion"],
        validates: &["DISC-TRM-06"],
    },
    MedDeepEntry {
        id: "MHYP-GRAND-01",
        title: "Hypothesis: any future clinical scoring system with >sopfr=5 criteria will converge to n=6 or sigma=12 parameters",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Math", "Biology"],
        confidence: 0.55,
        source_bts: &[238, 239, 240, 242],
        constants_used: &["C-sopfr", "C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "stability"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_MED_DEEP_CLUSTERS: &[&[MedDeepEntry]] = &[
    SURGICAL_DEEP,
    CRITICAL_CARE_DEEP,
    CARDIAC_DEEP,
    WHO_DEEP,
    DENTAL_DEEP,
    MED_CROSS_BRIDGES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &MedDeepEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[MedDeepEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(entry_to_node(e));

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

        // Node --Uses--> constants
        for &cid in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: cid.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

        // Node --Validates--> target nodes
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

/// Total medical deep entry count across all clusters.
pub fn med_deep_entry_count() -> usize {
    ALL_MED_DEEP_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Populate all medical deep discovery nodes and cross-domain edges.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_medical_deep_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();
    let mut total_nodes = 0;

    for cluster in ALL_MED_DEEP_CLUSTERS {
        let (n, _) = add_entries(graph, cluster);
        total_nodes += n;
    }

    // Cross-link medical deep nodes that share domains (Merges edges)
    let med_deep_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("MDISC-")
                || n.id.starts_with("MHYP-")
                || n.id.starts_with("MEXP-")
                || n.id.starts_with("MBRIDGE-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    for i in 0..med_deep_nodes.len() {
        for j in (i + 1)..med_deep_nodes.len() {
            let (ref id_a, ref dom_a) = med_deep_nodes[i];
            let (ref id_b, ref dom_b) = med_deep_nodes[j];

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
pub fn surgical_deep_count() -> usize { SURGICAL_DEEP.len() }
pub fn critical_care_deep_count() -> usize { CRITICAL_CARE_DEEP.len() }
pub fn cardiac_deep_count() -> usize { CARDIAC_DEEP.len() }
pub fn who_deep_count() -> usize { WHO_DEEP.len() }
pub fn dental_deep_count() -> usize { DENTAL_DEEP.len() }
pub fn med_cross_bridge_count() -> usize { MED_CROSS_BRIDGES.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_entry_counts() {
        assert_eq!(surgical_deep_count(), 4);
        assert_eq!(critical_care_deep_count(), 4);
        assert_eq!(cardiac_deep_count(), 4);
        assert_eq!(who_deep_count(), 4);
        assert_eq!(dental_deep_count(), 3);
        assert_eq!(med_cross_bridge_count(), 8);
        assert_eq!(med_deep_entry_count(), 27);
    }

    #[test]
    fn test_populate_creates_nodes_and_edges() {
        let mut g = DiscoveryGraph::new();
        let (nodes, edges) = populate_medical_deep_discoveries(&mut g);
        assert_eq!(nodes, 27);
        assert!(edges > 80, "Should have 80+ edges, got {}", edges);
    }

    #[test]
    fn test_all_node_ids_present() {
        let mut g = DiscoveryGraph::new();
        populate_medical_deep_discoveries(&mut g);

        let expected = [
            "MDISC-SURG-01", "MDISC-SURG-02", "MDISC-SURG-03", "MHYP-SURG-01",
            "MDISC-CRIT-01", "MDISC-CRIT-02", "MDISC-CRIT-03", "MEXP-CRIT-01",
            "MDISC-CARD-01", "MDISC-CARD-02", "MDISC-CARD-03", "MHYP-CARD-01",
            "MDISC-WHO-01", "MDISC-WHO-02", "MDISC-WHO-03", "MDISC-WHO-04",
            "MDISC-DENT-01", "MDISC-DENT-02", "MDISC-DENT-03",
            "MBRIDGE-COG-01", "MBRIDGE-COG-02", "MBRIDGE-ENV-01", "MBRIDGE-ENV-02",
            "MBRIDGE-CHIP-01", "MBRIDGE-TR-01", "MBRIDGE-GRAND-01", "MHYP-GRAND-01",
        ];
        for id in &expected {
            assert!(g.nodes.iter().any(|n| n.id == *id), "Missing node {}", id);
        }
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_MED_DEEP_CLUSTERS {
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
        populate_medical_deep_discoveries(&mut g);

        let grand_edges = g.edges.iter()
            .filter(|e| e.from == "MBRIDGE-GRAND-01" || e.to == "MBRIDGE-GRAND-01")
            .count();
        assert!(grand_edges >= 8, "MBRIDGE-GRAND-01 should have 8+ edges (6 BTs + constants + validates + merges), got {}", grand_edges);
    }

    #[test]
    fn test_cross_domain_merges_exist() {
        let mut g = DiscoveryGraph::new();
        populate_medical_deep_discoveries(&mut g);

        let merges = g.edges.iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Merges))
            .count();
        assert!(merges >= 15, "Should have 15+ merge edges, got {}", merges);
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_medical_deep_discoveries(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();
        let experiments = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Experiment)).count();

        assert_eq!(discoveries, 23);
        assert_eq!(hypotheses, 3);
        assert_eq!(experiments, 1);
        assert_eq!(discoveries + hypotheses + experiments, 27);
    }
}
