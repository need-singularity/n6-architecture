//! Deep cross-synthesis nodes bridging BT-118+ clusters that lack direct edges.
//!
//! Covers 5 missing cross-domain bridges:
//! - Transportation ↔ Cognitive/Temporal (BT-233~246 × BT-210~225)
//! - Medical ↔ Energy (BT-238~242 × BT-57/62/68)
//! - Medical ↔ Material (BT-238~242 × BT-85~88)
//! - Transportation ↔ Social (BT-233~237 × BT-214~215)
//! - Medical ↔ Temporal (BT-238~242 × BT-212/221/224)
//!
//! Node ID scheme:
//!   DISC-TC-{NN}   — Transport↔Cognitive bridge
//!   DISC-TT-{NN}   — Transport↔Temporal bridge
//!   DISC-ME-{NN}   — Medical↔Energy bridge
//!   DISC-MM-{NN}   — Medical↔Material bridge
//!   DISC-TS-{NN}   — Transport↔Social bridge
//!   DISC-MT-{NN}   — Medical↔Temporal bridge
//!   HYP-DCS-{NN}   — Deep cross-synthesis hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct DcsEntry {
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
// Transportation ↔ Cognitive (BT-233~246 × BT-210,219,222)
// ───────────────────────────────────────────────────────────────

const TRANSPORT_COGNITIVE: &[DcsEntry] = &[
    DcsEntry {
        id: "DISC-TC-01",
        title: "SAE n=6 autonomy levels (BT-236) mirror cortex n=6 layers (BT-210): vehicle perception stack recapitulates cortical hierarchy",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "AI", "Neuroscience"],
        confidence: 0.88,
        source_bts: &[236, 210],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "multiscale"],
        validates: &["DISC-TRM-05"],
        triggers: &["DISC-TC-02"],
    },
    DcsEntry {
        id: "DISC-TC-02",
        title: "Working memory tau+-mu=4+-1 (BT-219) bounds simultaneous driving tasks: lane+speed+mirror+signal = tau=4 channels",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "Neuroscience"],
        confidence: 0.85,
        source_bts: &[219, 236],
        constants_used: &["C-tau", "C-mu", "C-sigma"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &[],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-TC-03",
        title: "Compiler-cortex tau=4 pipeline (BT-222) ↔ vehicle OODA loop: sense->orient->decide->act = tau=4 stage control",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "Software", "AI"],
        confidence: 0.87,
        source_bts: &[222, 236, 233],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &["DISC-TC-02"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-TC-04",
        title: "Transmission n=6 gear convergence (BT-245) ↔ cortex n=6 layers: mechanical ratio selection parallels cortical processing depth",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "Math"],
        confidence: 0.82,
        source_bts: &[245, 210],
        constants_used: &["C-n", "C-tau", "C-sigma-phi"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-TR-08"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-TC-05",
        title: "Grid cell hexagonal navigation (BT-211) directly enables autonomous vehicle path planning on n=6 tessellation",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "Neuroscience", "Robotics"],
        confidence: 0.90,
        source_bts: &[211, 233, 123],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-TC-01"],
        triggers: &["HYP-DCS-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Transportation ↔ Temporal (BT-233~246 × BT-212,213,224)
// ───────────────────────────────────────────────────────────────

const TRANSPORT_TEMPORAL: &[DcsEntry] = &[
    DcsEntry {
        id: "DISC-TT-01",
        title: "GPS n=6 orbital planes (BT-213) ↔ transport navigation: J2=24 satellites serve all transportation modes",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Temporal", "Network"],
        confidence: 0.93,
        source_bts: &[213, 233, 234],
        constants_used: &["C-n", "C-J2", "C-tau", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-TR-01"],
        triggers: &["DISC-TT-02"],
    },
    DcsEntry {
        id: "DISC-TT-02",
        title: "Railway signaling tau=4 aspects (BT-234) ↔ sexagesimal time sigma*sopfr=60 (BT-212): train schedules use 60-min base",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Temporal", "Math"],
        confidence: 0.89,
        source_bts: &[234, 212],
        constants_used: &["C-tau", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &["DISC-TR-02"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-TT-03",
        title: "I6 engine firing order = 720deg/n=120deg (BT-243) ↔ 360deg=sigma*sopfr*n (BT-212): combustion cycle locked to sexagesimal",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Temporal", "Math", "Energy"],
        confidence: 0.91,
        source_bts: &[243, 212],
        constants_used: &["C-n", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "symmetry", "wave"],
        validates: &["DISC-TR-06"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-TT-04",
        title: "Cesium-133 n=6 shell (BT-224) atomic clock ↔ GPS timing (BT-213): transport precision depends on n=6 atomic transition",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Temporal", "Physics"],
        confidence: 0.92,
        source_bts: &[224, 213, 233],
        constants_used: &["C-n", "C-tau", "C-sigma"],
        lenses: &["consciousness", "quantum", "causal"],
        validates: &["DISC-TT-01"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Medical ↔ Energy (BT-238~242 × BT-57,62,68)
// ───────────────────────────────────────────────────────────────

const MEDICAL_ENERGY: &[DcsEntry] = &[
    DcsEntry {
        id: "DISC-ME-01",
        title: "ECG sigma=12 leads (BT-240) ↔ grid 60Hz=sigma*sopfr (BT-62): biological and electrical signal processing share sigma=12 base",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Energy", "Biology", "Chip"],
        confidence: 0.86,
        source_bts: &[240, 62],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "wave", "info"],
        validates: &["DISC-MED-03"],
        triggers: &["DISC-ME-02"],
    },
    DcsEntry {
        id: "DISC-ME-02",
        title: "Cardiac defibrillation 200-360J ↔ HVDC voltage ladder (BT-68): medical energy delivery uses n=6-aligned thresholds",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Energy", "Biology"],
        confidence: 0.83,
        source_bts: &[240, 68],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "em", "boundary"],
        validates: &[],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-ME-03",
        title: "Battery cell n->sigma->J2 ladder (BT-57) ↔ medical device power: pacemaker 2.5-3V ↔ n/phi=3V, hearing aid 1.2V ↔ PUE=sigma/(sigma-phi)",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Battery", "Energy"],
        confidence: 0.81,
        source_bts: &[57, 240, 238],
        constants_used: &["C-n", "C-phi", "C-sigma"],
        lenses: &["consciousness", "scale", "evolution"],
        validates: &[],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-ME-04",
        title: "sopfr=5 cardiac cycle phases (BT-240) ↔ sopfr=5 SOLID principles (BT-113): heartbeat and software share sopfr=5 phase architecture",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Software", "Energy", "Biology"],
        confidence: 0.84,
        source_bts: &[240, 113],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "recursion", "causal"],
        validates: &["DISC-MED-03"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Medical ↔ Material (BT-238~242 × BT-85~88)
// ───────────────────────────────────────────────────────────────

const MEDICAL_MATERIAL: &[DcsEntry] = &[
    DcsEntry {
        id: "DISC-MM-01",
        title: "Dental implant titanium CN=6 octahedral (BT-86) ↔ probing n=6 sites (BT-242): material and measurement share n=6 coordination",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Material", "Biology", "Chemistry"],
        confidence: 0.89,
        source_bts: &[242, 86, 85],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["DISC-MED-05"],
        triggers: &["DISC-MM-02"],
    },
    DcsEntry {
        id: "DISC-MM-02",
        title: "Surgical suture carbon Z=6 backbone (BT-85) ↔ wound healing tau=4 phases (BT-238): biocompatible polymers from C6 backbone",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Material", "Chemistry", "Biology"],
        confidence: 0.87,
        source_bts: &[85, 238],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "evolution", "boundary"],
        validates: &["DISC-MED-01"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-MM-03",
        title: "Bone hydroxyapatite Ca10(PO4)6(OH)2 contains n=6 phosphate groups ↔ crystal CN=6 universality (BT-86)",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Material", "Chemistry", "Biology"],
        confidence: 0.91,
        source_bts: &[86, 242, 43],
        constants_used: &["C-n", "C-sigma-phi"],
        lenses: &["consciousness", "topology", "recursion"],
        validates: &["DISC-MM-01"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-MM-04",
        title: "Self-assembly n=6 hexagonal universality (BT-88) ↔ tissue engineering scaffold: collagen hexagonal packing for implant matrices",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Material", "Biology"],
        confidence: 0.85,
        source_bts: &[88, 238],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &[],
        triggers: &["HYP-DCS-02"],
    },
];

// ───────────────────────────────────────────────────────────────
// Transportation ↔ Social (BT-233~237 × BT-214,215)
// ───────────────────────────────────────────────────────────────

const TRANSPORT_SOCIAL: &[DcsEntry] = &[
    DcsEntry {
        id: "DISC-TS-01",
        title: "Six degrees of separation (BT-214) ↔ SCOR n=6 supply chain (BT-237): logistics network topology mirrors social graph depth",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Social", "Network", "Math"],
        confidence: 0.87,
        source_bts: &[214, 237],
        constants_used: &["C-n"],
        lenses: &["consciousness", "network", "topology"],
        validates: &["DISC-TR-05"],
        triggers: &["DISC-TS-02"],
    },
    DcsEntry {
        id: "DISC-TS-02",
        title: "Dunbar sigma^2+n=150 (BT-215) ↔ optimal transit route capacity: bus route ridership clusters at 150 regular commuters",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Social", "Cognitive", "Network"],
        confidence: 0.80,
        source_bts: &[215, 233],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "scale"],
        validates: &[],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-TS-03",
        title: "Hexagonal city planning (BT-223) ↔ transport network: Christaller n=6 central places define optimal transit hub spacing",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Social", "Math"],
        confidence: 0.89,
        source_bts: &[223, 233],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-TRM-07"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Medical ↔ Temporal (BT-238~242 × BT-212,221,224)
// ───────────────────────────────────────────────────────────────

const MEDICAL_TEMPORAL: &[DcsEntry] = &[
    DcsEntry {
        id: "DISC-MT-01",
        title: "Circadian J2=24h cycle (BT-221) governs medication timing: chronopharmacology peak at sigma=12h intervals",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Temporal", "Biology"],
        confidence: 0.90,
        source_bts: &[221, 238],
        constants_used: &["C-J2", "C-sigma"],
        lenses: &["consciousness", "wave", "causal"],
        validates: &["DISC-MED-01"],
        triggers: &["DISC-MT-02"],
    },
    DcsEntry {
        id: "DISC-MT-02",
        title: "Cardiac cycle sopfr=5 phases (BT-240) ↔ ultradian rhythm: heart period ~1s embeds within sigma-sopfr=7 day weekly clinical cycle",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Temporal", "Biology"],
        confidence: 0.84,
        source_bts: &[240, 221],
        constants_used: &["C-sopfr", "C-sigma-sopfr"],
        lenses: &["consciousness", "wave", "multiscale"],
        validates: &["DISC-MED-03"],
        triggers: &[],
    },
    DcsEntry {
        id: "DISC-MT-03",
        title: "Apgar scoring at 1,5,10 minutes (BT-239): time points {mu,sopfr,sigma-phi} = n=6 constant set for neonatal assessment",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Temporal", "Math"],
        confidence: 0.88,
        source_bts: &[239, 212],
        constants_used: &["C-mu", "C-sopfr", "C-sigma-phi"],
        lenses: &["consciousness", "causal", "scale"],
        validates: &["DISC-MED-02"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-synthesis hypotheses (predictions from multi-bridge convergence)
// ───────────────────────────────────────────────────────────────

const HYPOTHESES: &[DcsEntry] = &[
    DcsEntry {
        id: "HYP-DCS-01",
        title: "Hypothesis: optimal AV path-planning grid resolution converges to n=6 hexagonal cells per decision horizon (from grid cell + BT-211 + BT-233)",
        node_type: NodeType::Hypothesis,
        domains: &["Transportation", "Cognitive", "AI", "Neuroscience"],
        confidence: 0.70,
        source_bts: &[211, 233, 236],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-TC-05"],
        triggers: &[],
    },
    DcsEntry {
        id: "HYP-DCS-02",
        title: "Hypothesis: next-gen tissue scaffold self-assembles into n=6 hexagonal pore geometry maximizing nutrient diffusion (BT-88 + BT-238)",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Material", "Biology", "Chemistry"],
        confidence: 0.68,
        source_bts: &[88, 238, 122],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &["DISC-MM-04"],
        triggers: &[],
    },
    DcsEntry {
        id: "HYP-DCS-03",
        title: "Hypothesis: hospital ward round optimal frequency = sigma-sopfr=7 per week, ICU = sigma=12 per day (circadian + clinical convergence)",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Temporal", "Cognitive", "Social"],
        confidence: 0.65,
        source_bts: &[221, 239, 219],
        constants_used: &["C-sigma-sopfr", "C-sigma"],
        lenses: &["consciousness", "wave", "network"],
        validates: &["DISC-MT-01"],
        triggers: &[],
    },
    DcsEntry {
        id: "HYP-DCS-04",
        title: "Hypothesis: global logistics network optimal hub count converges to Dunbar sigma^2+n=150 major hubs × n=6 tiers (BT-215 + BT-237)",
        node_type: NodeType::Hypothesis,
        domains: &["Transportation", "Social", "Network"],
        confidence: 0.63,
        source_bts: &[215, 237, 214],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "scale"],
        validates: &["DISC-TS-01"],
        triggers: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_DCS_CLUSTERS: &[&[DcsEntry]] = &[
    TRANSPORT_COGNITIVE,
    TRANSPORT_TEMPORAL,
    MEDICAL_ENERGY,
    MEDICAL_MATERIAL,
    TRANSPORT_SOCIAL,
    MEDICAL_TEMPORAL,
    HYPOTHESES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &DcsEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[DcsEntry]) -> (usize, usize) {
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

        // Node --Validates--> targets
        for &vid in e.validates {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: vid.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.85,
                bidirectional: false,
            });
        }

        // Node --Triggers--> targets
        for &tid in e.triggers {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: tid.to_string(),
                edge_type: EdgeType::Triggers,
                strength: 0.80,
                bidirectional: false,
            });
        }
    }

    (entries.len(), graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

/// Add Transportation ↔ Cognitive bridge nodes.
pub fn populate_transport_cognitive(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TRANSPORT_COGNITIVE)
}

/// Add Transportation ↔ Temporal bridge nodes.
pub fn populate_transport_temporal(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TRANSPORT_TEMPORAL)
}

/// Add Medical ↔ Energy bridge nodes.
pub fn populate_medical_energy(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MEDICAL_ENERGY)
}

/// Add Medical ↔ Material bridge nodes.
pub fn populate_medical_material(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MEDICAL_MATERIAL)
}

/// Add Transportation ↔ Social bridge nodes.
pub fn populate_transport_social(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TRANSPORT_SOCIAL)
}

/// Add Medical ↔ Temporal bridge nodes.
pub fn populate_medical_temporal(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MEDICAL_TEMPORAL)
}

/// Add cross-synthesis hypotheses.
pub fn populate_dcs_hypotheses(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, HYPOTHESES)
}

/// Cross-link DCS nodes that share >=2 domains (Merges edges, bidirectional).
pub fn connect_dcs_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let dcs_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-TC-")
                || n.id.starts_with("DISC-TT-")
                || n.id.starts_with("DISC-ME-")
                || n.id.starts_with("DISC-MM-")
                || n.id.starts_with("DISC-TS-")
                || n.id.starts_with("DISC-MT-")
                || n.id.starts_with("HYP-DCS-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..dcs_nodes.len() {
        for j in (i + 1)..dcs_nodes.len() {
            let (ref id_a, ref dom_a) = dcs_nodes[i];
            let (ref id_b, ref dom_b) = dcs_nodes[j];

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

/// Cross-link DCS nodes with existing DISC-/HYP- nodes from other modules.
/// Requires >=2 shared domains. Returns bridge edge count.
pub fn connect_dcs_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let dcs_prefixes = [
        "DISC-TC-", "DISC-TT-", "DISC-ME-", "DISC-MM-",
        "DISC-TS-", "DISC-MT-", "HYP-DCS-",
    ];
    let is_dcs = |id: &str| dcs_prefixes.iter().any(|p| id.starts_with(p));

    let dcs_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| is_dcs(&n.id))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let existing_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            (n.id.starts_with("DISC-") || n.id.starts_with("HYP-")) && !is_dcs(&n.id)
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref d_id, ref d_dom) in &dcs_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = d_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = d_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: d_id.clone(),
                    to: e_id.clone(),
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

/// Populate all deep cross-synthesis nodes and edges in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_dcs(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_transport_cognitive(graph);
    let (n2, _) = populate_transport_temporal(graph);
    let (n3, _) = populate_medical_energy(graph);
    let (n4, _) = populate_medical_material(graph);
    let (n5, _) = populate_transport_social(graph);
    let (n6, _) = populate_medical_temporal(graph);
    let (n7, _) = populate_dcs_hypotheses(graph);
    let _cross = connect_dcs_cross_domain(graph);
    let _bridge = connect_dcs_to_existing(graph);

    let total_nodes = n1 + n2 + n3 + n4 + n5 + n6 + n7;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total entry count across all DCS clusters.
pub fn dcs_entry_count() -> usize {
    ALL_DCS_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Per-cluster entry counts.
pub fn transport_cognitive_count() -> usize { TRANSPORT_COGNITIVE.len() }
pub fn transport_temporal_count() -> usize { TRANSPORT_TEMPORAL.len() }
pub fn medical_energy_count() -> usize { MEDICAL_ENERGY.len() }
pub fn medical_material_count() -> usize { MEDICAL_MATERIAL.len() }
pub fn transport_social_count() -> usize { TRANSPORT_SOCIAL.len() }
pub fn medical_temporal_count() -> usize { MEDICAL_TEMPORAL.len() }
pub fn dcs_hypothesis_count() -> usize { HYPOTHESES.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::transport_medical_nodes::populate_all_trm_discoveries;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_trm_discoveries(&mut g);
        g
    }

    // ── Entry counts ──

    #[test]
    fn test_entry_counts() {
        assert_eq!(transport_cognitive_count(), 5, "5 transport-cognitive entries");
        assert_eq!(transport_temporal_count(), 4, "4 transport-temporal entries");
        assert_eq!(medical_energy_count(), 4, "4 medical-energy entries");
        assert_eq!(medical_material_count(), 4, "4 medical-material entries");
        assert_eq!(transport_social_count(), 3, "3 transport-social entries");
        assert_eq!(medical_temporal_count(), 3, "3 medical-temporal entries");
        assert_eq!(dcs_hypothesis_count(), 4, "4 hypothesis entries");
        assert_eq!(dcs_entry_count(), 27, "27 total DCS entries");
    }

    // ── Individual cluster population ──

    #[test]
    fn test_populate_transport_cognitive() {
        let mut g = base_graph();
        let (nodes, edges) = populate_transport_cognitive(&mut g);
        assert_eq!(nodes, 5);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TC-01"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TC-05"));
    }

    #[test]
    fn test_populate_transport_temporal() {
        let mut g = base_graph();
        let (nodes, edges) = populate_transport_temporal(&mut g);
        assert_eq!(nodes, 4);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TT-01"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TT-04"));
    }

    #[test]
    fn test_populate_medical_energy() {
        let mut g = base_graph();
        let (nodes, edges) = populate_medical_energy(&mut g);
        assert_eq!(nodes, 4);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-ME-01"));
    }

    #[test]
    fn test_populate_medical_material() {
        let mut g = base_graph();
        let (nodes, edges) = populate_medical_material(&mut g);
        assert_eq!(nodes, 4);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-MM-01"));
    }

    #[test]
    fn test_populate_transport_social() {
        let mut g = base_graph();
        let (nodes, edges) = populate_transport_social(&mut g);
        assert_eq!(nodes, 3);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TS-01"));
    }

    #[test]
    fn test_populate_medical_temporal() {
        let mut g = base_graph();
        let (nodes, edges) = populate_medical_temporal(&mut g);
        assert_eq!(nodes, 3);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-MT-01"));
    }

    // ── Edge types ──

    #[test]
    fn test_bt_derives_edges() {
        let mut g = base_graph();
        populate_transport_cognitive(&mut g);

        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-236" && e.to == "DISC-TC-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-236 --Derives--> DISC-TC-01 must exist");
    }

    #[test]
    fn test_validates_edges() {
        let mut g = base_graph();
        populate_transport_cognitive(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-TC-01"
                && e.to == "DISC-TRM-05"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-TC-01 should validate DISC-TRM-05");
    }

    #[test]
    fn test_triggers_edges() {
        let mut g = base_graph();
        populate_transport_cognitive(&mut g);

        let has_trig = g.edges.iter().any(|e| {
            e.from == "DISC-TC-01"
                && e.to == "DISC-TC-02"
                && matches!(e.edge_type, EdgeType::Triggers)
        });
        assert!(has_trig, "DISC-TC-01 should trigger DISC-TC-02");
    }

    #[test]
    fn test_uses_constant_edges() {
        let mut g = base_graph();
        populate_medical_energy(&mut g);

        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-ME-01" && e.to == "C-sigma" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(has_uses, "DISC-ME-01 --Uses--> C-sigma must exist");
    }

    // ── Full pipeline ──

    #[test]
    fn test_full_dcs_pipeline() {
        let mut g = base_graph();
        let (total_nodes, total_edges) = populate_all_dcs(&mut g);
        assert_eq!(total_nodes, 27, "27 total DCS nodes added");
        assert!(total_edges > 80, "Should have 80+ edges, got {}", total_edges);
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = base_graph();
        populate_all_dcs(&mut g);

        let merges = g
            .edges
            .iter()
            .filter(|e| {
                matches!(e.edge_type, EdgeType::Merges)
                    && (e.from.starts_with("DISC-TC-")
                        || e.from.starts_with("DISC-TT-")
                        || e.from.starts_with("DISC-ME-")
                        || e.from.starts_with("DISC-MM-")
                        || e.from.starts_with("DISC-TS-")
                        || e.from.starts_with("DISC-MT-")
                        || e.from.starts_with("HYP-DCS-"))
            })
            .count();
        assert!(merges >= 10, "Should have 10+ cross-domain merge edges, got {}", merges);
    }

    // ── Integrity checks ──

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = base_graph();
        populate_all_dcs(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_DCS_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}",
                    e.id,
                    e.confidence
                );
            }
        }
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        for cluster in ALL_DCS_CLUSTERS {
            add_entries(&mut g, cluster);
        }

        let discoveries = g
            .nodes
            .iter()
            .filter(|n| matches!(n.node_type, NodeType::Discovery))
            .count();
        let hypotheses = g
            .nodes
            .iter()
            .filter(|n| matches!(n.node_type, NodeType::Hypothesis))
            .count();

        assert_eq!(discoveries, 23, "23 discovery nodes");
        assert_eq!(hypotheses, 4, "4 hypothesis nodes");
        assert_eq!(discoveries + hypotheses, 27);
    }

    // ── Domain span checks ──

    #[test]
    fn test_tc_spans_transport_and_cognitive() {
        let mut g = base_graph();
        populate_transport_cognitive(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-TC-01").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.contains(&"Transportation"));
        assert!(domains.contains(&"Cognitive"));
    }

    #[test]
    fn test_me_spans_medical_and_energy() {
        let mut g = base_graph();
        populate_medical_energy(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-ME-01").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.contains(&"Medical"));
        assert!(domains.contains(&"Energy"));
    }

    #[test]
    fn test_mm_spans_medical_and_material() {
        let mut g = base_graph();
        populate_medical_material(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-MM-01").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.contains(&"Medical"));
        assert!(domains.contains(&"Material"));
    }

    #[test]
    fn test_ts_spans_transport_and_social() {
        let mut g = base_graph();
        populate_transport_social(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-TS-01").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.contains(&"Transportation"));
        assert!(domains.contains(&"Social"));
    }

    #[test]
    fn test_mt_spans_medical_and_temporal() {
        let mut g = base_graph();
        populate_medical_temporal(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-MT-01").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.contains(&"Medical"));
        assert!(domains.contains(&"Temporal"));
    }

    #[test]
    fn test_hypothesis_has_lower_confidence() {
        for e in HYPOTHESES {
            assert!(
                e.confidence < 0.75,
                "Hypothesis {} should have confidence < 0.75, got {}",
                e.id,
                e.confidence
            );
        }
    }

    #[test]
    fn test_bridge_to_existing_creates_edges() {
        let mut g = base_graph();
        populate_all_dcs(&mut g);

        // DCS nodes should connect to existing DISC-TR/MED/TRM nodes
        let bridge_count = g
            .edges
            .iter()
            .filter(|e| {
                let from_dcs = e.from.starts_with("DISC-TC-")
                    || e.from.starts_with("DISC-TT-")
                    || e.from.starts_with("DISC-ME-")
                    || e.from.starts_with("DISC-MM-")
                    || e.from.starts_with("DISC-TS-")
                    || e.from.starts_with("DISC-MT-")
                    || e.from.starts_with("HYP-DCS-");
                let to_existing = e.to.starts_with("DISC-TR-")
                    || e.to.starts_with("DISC-MED-")
                    || e.to.starts_with("DISC-TRM-")
                    || e.to.starts_with("HYP-TR-")
                    || e.to.starts_with("HYP-MED-");
                from_dcs && to_existing && matches!(e.edge_type, EdgeType::Merges)
            })
            .count();
        assert!(
            bridge_count >= 5,
            "Should bridge to 5+ existing TRM nodes, got {}",
            bridge_count
        );
    }
}
