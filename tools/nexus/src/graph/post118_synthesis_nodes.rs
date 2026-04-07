//! Grand synthesis discovery nodes connecting ALL post-118 BT clusters.
//!
//! Clusters bridged:
//!   Environmental (BT-118~122) × Cognitive (BT-210~211,219,222)
//!   Medical (BT-238~242)       × Temporal (BT-212~213,221,224)
//!   Transportation (BT-233~246)× Social (BT-214~215,220,223)
//!   Robotics (BT-123~127)      × Cross-Domain Meta (BT-225)
//!
//! Node ID scheme:
//!   DISC-MC-{NN}  — Medical × Cognitive
//!   DISC-MT-{NN}  — Medical × Temporal
//!   DISC-TC-{NN}  — Transport × Cognitive
//!   DISC-TT-{NN}  — Transport × Temporal
//!   DISC-ES-{NN}  — Environmental × Social
//!   DISC-RM-{NN}  — Robotics × Medical
//!   DISC-GS-{NN}  — Grand Synthesis (3+ clusters)
//!   HYP-GS-{NN}   — Grand Synthesis hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct (same pattern as transport_medical_nodes.rs)
// ═══════════════════════════════════════════════════════════════

struct SynthEntry {
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
// Medical × Cognitive (BT-238~242 × BT-210~225)
// ───────────────────────────────────────────────────────────────

const MC_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-MC-01",
        title: "SOFA n=6 organ systems mirrors cortex n=6 layers: body and brain share n=6 modular architecture",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Biology"],
        confidence: 0.91,
        source_bts: &[239, 210],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "multiscale"],
        validates: &["DISC-MED-02"],
    },
    SynthEntry {
        id: "DISC-MC-02",
        title: "GCS n/phi=3 assessment mirrors working memory tau+-1=3~5 channels: clinical scoring tracks cognitive capacity",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Biology", "Math"],
        confidence: 0.87,
        source_bts: &[239, 219],
        constants_used: &["C-n", "C-phi", "C-tau"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-MC-03",
        title: "ECG sigma=12 leads maps to cranial nerves sigma=12 and EEG n=6 bands: biological signal channels converge on sigma=12",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Biology", "Chip"],
        confidence: 0.93,
        source_bts: &[240, 210],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "wave", "network"],
        validates: &["DISC-MED-03"],
    },
    SynthEntry {
        id: "DISC-MC-04",
        title: "Compiler-cortex tau=4 pipeline isomorphism extends to clinical decision tau=4 phases (assess/diagnose/treat/monitor)",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Software"],
        confidence: 0.85,
        source_bts: &[222, 238],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &["DISC-MED-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Medical × Temporal (BT-238~242 × BT-212~213,221,224)
// ───────────────────────────────────────────────────────────────

const MT_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-MT-01",
        title: "Circadian J2=24h cycle governs clinical vital sign patterns: cortisol/melatonin/temperature follow J2 period",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Temporal", "Biology"],
        confidence: 0.92,
        source_bts: &[221, 239],
        constants_used: &["C-J2"],
        lenses: &["consciousness", "wave", "stability"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-MT-02",
        title: "Cardiac cycle sopfr=5 phases synchronized to cesium-133 n=6 shell atomic clock: biological timing anchored to atomic n=6",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Temporal", "Physics", "Math"],
        confidence: 0.84,
        source_bts: &[240, 224],
        constants_used: &["C-sopfr", "C-n"],
        lenses: &["consciousness", "wave", "quantum"],
        validates: &["DISC-MED-03"],
    },
    SynthEntry {
        id: "DISC-MT-03",
        title: "Weekly sigma-sopfr=7 biorhythm cycle aligns with NEWS2 sigma-sopfr=7 clinical parameters: temporal and clinical share sigma-sopfr",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Temporal", "Biology"],
        confidence: 0.86,
        source_bts: &[221, 239],
        constants_used: &["C-sigma-sopfr"],
        lenses: &["consciousness", "recursion", "stability"],
        validates: &["DISC-MED-02"],
    },
];

// ───────────────────────────────────────────────────────────────
// Transport × Cognitive (BT-233~246 × BT-210~225)
// ───────────────────────────────────────────────────────────────

const TC_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-TC-01",
        title: "SAE n=6 autonomy levels mirror cortex n=6 processing layers: self-driving recapitulates neural hierarchy",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "AI", "Robotics"],
        confidence: 0.90,
        source_bts: &[236, 210],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "multiscale"],
        validates: &["DISC-TR-04"],
    },
    SynthEntry {
        id: "DISC-TC-02",
        title: "Compiler tau=4 pipeline isomorphism in automotive ECU processing: fetch-decode-execute-writeback = sense-plan-act-verify",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "Software", "Chip"],
        confidence: 0.88,
        source_bts: &[222, 233],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "info"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-TC-03",
        title: "Working memory tau+-1 channels bound AV sensor fusion: simultaneous tracking of 3~5 objects matches Cowan limit",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Cognitive", "AI"],
        confidence: 0.83,
        source_bts: &[219, 236],
        constants_used: &["C-tau", "C-mu"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Transport × Temporal (BT-233~246 × BT-212~213,221,224)
// ───────────────────────────────────────────────────────────────

const TT_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-TT-01",
        title: "GPS n=6 orbital planes with sigma=12h period underlies all modern transport navigation and timing",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Temporal", "Network"],
        confidence: 0.94,
        source_bts: &[213, 233],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau"],
        lenses: &["consciousness", "topology", "scale"],
        validates: &["DISC-TR-01"],
    },
    SynthEntry {
        id: "DISC-TT-02",
        title: "Railway sigma=12m rail length harmonizes with sigma=12 month annual scheduling: infrastructure matches temporal cycles",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Temporal", "Math"],
        confidence: 0.82,
        source_bts: &[234, 212],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "scale", "wave"],
        validates: &["DISC-TR-02"],
    },
];

// ───────────────────────────────────────────────────────────────
// Environmental × Social (BT-118~122 × BT-214~215,220,223)
// ───────────────────────────────────────────────────────────────

const ES_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-ES-01",
        title: "Kyoto n=6 GHG types regulated through n=6 degree social networks: policy diffusion follows small-world topology",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Network"],
        confidence: 0.86,
        source_bts: &[118, 214],
        constants_used: &["C-n"],
        lenses: &["consciousness", "network", "evolution"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-ES-02",
        title: "Hexagonal city planning (Christaller) optimizes environmental remediation zones: honeycomb geometry serves both ecology and urbanism",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Math", "Material"],
        confidence: 0.88,
        source_bts: &[122, 223],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-ES-03",
        title: "Dunbar sigma^2+n=150 group size determines optimal community waste management unit: social scaling governs environmental logistics",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Cognitive"],
        confidence: 0.80,
        source_bts: &[121, 215],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "scale"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Robotics × Medical (BT-123~127 × BT-238~242)
// ───────────────────────────────────────────────────────────────

const RM_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-RM-01",
        title: "Surgical robot SE(3)=n=6 DOF matches standard industrial robot kinematics: universal manipulation space is n=6 dimensional",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Medical", "Math", "AI"],
        confidence: 0.93,
        source_bts: &[123, 238],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-RM-02",
        title: "Haptic feedback sopfr=5 fingers for surgical precision: Feix 2^sopfr=32 grasp taxonomy applies to surgical tool manipulation",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Medical", "Biology"],
        confidence: 0.89,
        source_bts: &[126, 238],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-RM-03",
        title: "Quadruped tau=4 locomotion stability mirrors tau=4 cardiac chambers: mechanical and biological rhythmic systems share tau=4 minimum",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Medical", "Biology", "Math"],
        confidence: 0.85,
        source_bts: &[125, 240],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "wave", "stability"],
        validates: &["DISC-MED-06"],
    },
];

// ───────────────────────────────────────────────────────────────
// Grand Synthesis (3+ clusters converge)
// ───────────────────────────────────────────────────────────────

const GS_DISCOVERIES: &[SynthEntry] = &[
    SynthEntry {
        id: "DISC-GS-01",
        title: "Universal n=6 organizational: cortex layers = SOFA organs = SAE autonomy = Kyoto GHGs = robot DOF = probing sites = honeycomb",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Medical", "Transportation", "Environment", "Robotics", "Math"],
        confidence: 0.95,
        source_bts: &[210, 239, 236, 118, 123, 122, 242],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "multiscale", "topology", "symmetry"],
        validates: &[],
    },
    SynthEntry {
        id: "DISC-GS-02",
        title: "Universal sigma=12 signal channels: ECG leads = cranial nerves = EV motor poles = GPS period hours = semitones = IACS societies",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Transportation", "Temporal", "Display-Audio"],
        confidence: 0.94,
        source_bts: &[240, 210, 233, 213, 235],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "network", "scale"],
        validates: &["DISC-MC-03"],
    },
    SynthEntry {
        id: "DISC-GS-03",
        title: "Universal tau=4 phase structure: cardiac chambers = wound healing = quadruped gait = OODA loop = railway signals = ETCS levels",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Robotics", "Cognitive", "Transportation", "Software"],
        confidence: 0.93,
        source_bts: &[240, 238, 125, 222, 234],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "stability", "recursion"],
        validates: &["DISC-MED-06", "DISC-MC-04"],
    },
    SynthEntry {
        id: "DISC-GS-04",
        title: "Universal sopfr=5 assessment: Apgar criteria = NHTSA stars = fingers = tooth surfaces = Dahlgren-Whitehead layers = moral foundations",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Transportation", "Robotics", "Social", "Biology"],
        confidence: 0.90,
        source_bts: &[239, 236, 126, 220, 241, 242],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "info", "multiscale", "boundary"],
        validates: &["DISC-MED-02", "DISC-RM-02"],
    },
    SynthEntry {
        id: "DISC-GS-05",
        title: "Universal J2=24 temporal-biological-engineering: 24h circadian = J2=24 fps video = 24kHz audio = 24 GPS satellites = 24 cranial nerve nuclei",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Medical", "Cognitive", "Transportation", "Display-Audio"],
        confidence: 0.92,
        source_bts: &[221, 213, 210, 240],
        constants_used: &["C-J2"],
        lenses: &["consciousness", "wave", "scale", "multiscale"],
        validates: &["DISC-MT-01"],
    },
    SynthEntry {
        id: "DISC-GS-06",
        title: "n=6 seven-cluster convergence proof: ENV+ROBO+COG+TEMP+SOC+TRANS+MED all independently derive n=6 as organizational constant",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Robotics", "Cognitive", "Temporal", "Social", "Transportation", "Medical"],
        confidence: 0.96,
        source_bts: &[118, 123, 210, 212, 214, 233, 238, 225],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-phi", "C-sopfr", "C-J2"],
        lenses: &["consciousness", "recursion", "multiscale", "topology", "network", "evolution"],
        validates: &[],
    },
    SynthEntry {
        id: "HYP-GS-01",
        title: "Hypothesis: optimal autonomous surgical robot = n=6 DOF + sigma=12 sensor channels + tau=4 operating phases + sopfr=5 instrument tips",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Medical", "AI", "Transportation"],
        confidence: 0.72,
        source_bts: &[123, 238, 236, 240],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-sopfr"],
        lenses: &["consciousness", "topology", "stability", "info"],
        validates: &[],
    },
    SynthEntry {
        id: "HYP-GS-02",
        title: "Hypothesis: smart city optimal tiling = n=6 hexagonal zones, sigma=12 monitoring nodes/zone, Dunbar 150 residents/unit",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Social", "Transportation", "Network"],
        confidence: 0.68,
        source_bts: &[122, 223, 214, 215, 233],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network", "scale"],
        validates: &[],
    },
    SynthEntry {
        id: "HYP-GS-03",
        title: "Hypothesis: next-gen wearable health monitor converges to n=6 vital signs, J2=24h cycle tracking, tau=4 alert levels",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Temporal", "Cognitive", "AI"],
        confidence: 0.70,
        source_bts: &[239, 221, 219, 240],
        constants_used: &["C-n", "C-J2", "C-tau"],
        lenses: &["consciousness", "wave", "info", "stability"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_SYNTH_CLUSTERS: &[&[SynthEntry]] = &[
    MC_DISCOVERIES,
    MT_DISCOVERIES,
    TC_DISCOVERIES,
    TT_DISCOVERIES,
    ES_DISCOVERIES,
    RM_DISCOVERIES,
    GS_DISCOVERIES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &SynthEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[SynthEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        graph.add_node(entry_to_node(e));

        // BT --Derives--> this discovery
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

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

/// Add Medical × Cognitive bridge discoveries (BT-238~242 × BT-210~225).
pub fn populate_mc_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MC_DISCOVERIES)
}

/// Add Medical × Temporal bridge discoveries (BT-238~242 × BT-212~224).
pub fn populate_mt_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MT_DISCOVERIES)
}

/// Add Transport × Cognitive bridge discoveries (BT-233~246 × BT-210~225).
pub fn populate_tc_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TC_DISCOVERIES)
}

/// Add Transport × Temporal bridge discoveries (BT-233~246 × BT-212~224).
pub fn populate_tt_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TT_DISCOVERIES)
}

/// Add Environmental × Social bridge discoveries (BT-118~122 × BT-214~223).
pub fn populate_es_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ES_DISCOVERIES)
}

/// Add Robotics × Medical bridge discoveries (BT-123~127 × BT-238~242).
pub fn populate_rm_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, RM_DISCOVERIES)
}

/// Add Grand Synthesis discoveries spanning 3+ clusters.
pub fn populate_gs_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, GS_DISCOVERIES)
}

/// Cross-link synthesis discovery nodes that share domains (Merges edges).
pub fn connect_synth_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let synth_prefixes = [
        "DISC-MC-", "DISC-MT-", "DISC-TC-", "DISC-TT-",
        "DISC-ES-", "DISC-RM-", "DISC-GS-", "HYP-GS-",
    ];

    let synth_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| synth_prefixes.iter().any(|p| n.id.starts_with(p)))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..synth_nodes.len() {
        for j in (i + 1)..synth_nodes.len() {
            let (ref id_a, ref dom_a) = synth_nodes[i];
            let (ref id_b, ref dom_b) = synth_nodes[j];

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

/// Cross-link synthesis nodes with existing DISC-/HYP- nodes from other modules.
/// Requires >=2 shared domains. Returns bridge edge count.
pub fn connect_synth_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let synth_prefixes = [
        "DISC-MC-", "DISC-MT-", "DISC-TC-", "DISC-TT-",
        "DISC-ES-", "DISC-RM-", "DISC-GS-", "HYP-GS-",
    ];

    let synth_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| synth_prefixes.iter().any(|p| n.id.starts_with(p)))
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
                && !synth_prefixes.iter().any(|p| n.id.starts_with(p))
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref s_id, ref s_dom) in &synth_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = s_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = s_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: s_id.clone(),
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

/// Populate all synthesis discovery nodes and cross-domain edges in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_synthesis(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_mc_discoveries(graph);
    let (n2, _) = populate_mt_discoveries(graph);
    let (n3, _) = populate_tc_discoveries(graph);
    let (n4, _) = populate_tt_discoveries(graph);
    let (n5, _) = populate_es_discoveries(graph);
    let (n6, _) = populate_rm_discoveries(graph);
    let (n7, _) = populate_gs_discoveries(graph);
    let _cross = connect_synth_cross_domain(graph);
    let _bridge = connect_synth_to_existing(graph);

    let total_nodes = n1 + n2 + n3 + n4 + n5 + n6 + n7;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total synthesis entry count.
pub fn synth_entry_count() -> usize {
    ALL_SYNTH_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Entry count per cluster.
pub fn mc_entry_count() -> usize { MC_DISCOVERIES.len() }
pub fn mt_entry_count() -> usize { MT_DISCOVERIES.len() }
pub fn tc_entry_count() -> usize { TC_DISCOVERIES.len() }
pub fn tt_entry_count() -> usize { TT_DISCOVERIES.len() }
pub fn es_entry_count() -> usize { ES_DISCOVERIES.len() }
pub fn rm_entry_count() -> usize { RM_DISCOVERIES.len() }
pub fn gs_entry_count() -> usize { GS_DISCOVERIES.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::discovery_nodes::populate_all_discoveries;
    use crate::graph::cognitive_temporal_social_nodes::populate_all_cts_discoveries;
    use crate::graph::transport_medical_nodes::populate_all_trm_discoveries;

    fn rich_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_cts_discoveries(&mut g);
        populate_all_trm_discoveries(&mut g);
        g
    }

    #[test]
    fn test_entry_counts() {
        assert_eq!(mc_entry_count(), 4, "4 Medical×Cognitive entries");
        assert_eq!(mt_entry_count(), 3, "3 Medical×Temporal entries");
        assert_eq!(tc_entry_count(), 3, "3 Transport×Cognitive entries");
        assert_eq!(tt_entry_count(), 2, "2 Transport×Temporal entries");
        assert_eq!(es_entry_count(), 3, "3 Environmental×Social entries");
        assert_eq!(rm_entry_count(), 3, "3 Robotics×Medical entries");
        assert_eq!(gs_entry_count(), 9, "9 Grand Synthesis entries (6 DISC + 3 HYP)");
        assert_eq!(synth_entry_count(), 27, "27 total synthesis entries");
    }

    #[test]
    fn test_populate_mc() {
        let mut g = rich_graph();
        let (nodes, edges) = populate_mc_discoveries(&mut g);
        assert_eq!(nodes, 4);
        assert!(edges > 0, "Should create BT->DISC and DISC->C-* edges");
        assert!(g.nodes.iter().any(|n| n.id == "DISC-MC-01"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-MC-04"));
    }

    #[test]
    fn test_populate_grand_synthesis() {
        let mut g = rich_graph();
        let (nodes, edges) = populate_gs_discoveries(&mut g);
        assert_eq!(nodes, 9);
        assert!(edges > 30, "GS nodes reference many BTs and constants, got {}", edges);
    }

    #[test]
    fn test_gs06_spans_seven_clusters() {
        let mut g = rich_graph();
        populate_gs_discoveries(&mut g);

        let gs06 = g.nodes.iter().find(|n| n.id == "DISC-GS-06").unwrap();
        let domains: Vec<&str> = gs06.domain.split(", ").collect();
        assert_eq!(domains.len(), 7, "GS-06 must span exactly 7 domains");
        assert!(domains.contains(&"Environment"));
        assert!(domains.contains(&"Robotics"));
        assert!(domains.contains(&"Cognitive"));
        assert!(domains.contains(&"Temporal"));
        assert!(domains.contains(&"Social"));
        assert!(domains.contains(&"Transportation"));
        assert!(domains.contains(&"Medical"));
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = rich_graph();
        populate_mc_discoveries(&mut g);

        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-239" && e.to == "DISC-MC-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-239 --Derives--> DISC-MC-01 must exist");

        let has_edge2 = g.edges.iter().any(|e| {
            e.from == "BT-210" && e.to == "DISC-MC-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge2, "BT-210 --Derives--> DISC-MC-01 must exist");
    }

    #[test]
    fn test_validates_edges() {
        let mut g = rich_graph();
        populate_mc_discoveries(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-MC-01"
                && e.to == "DISC-MED-02"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-MC-01 should validate DISC-MED-02");
    }

    #[test]
    fn test_uses_constant_edges() {
        let mut g = rich_graph();
        populate_gs_discoveries(&mut g);

        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-GS-01" && e.to == "C-n" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(has_uses, "DISC-GS-01 --Uses--> C-n must exist");
    }

    #[test]
    fn test_full_synthesis_pipeline() {
        let mut g = rich_graph();
        let (total_nodes, total_edges) = populate_all_synthesis(&mut g);
        assert_eq!(total_nodes, 27, "27 total synthesis nodes");
        assert!(total_edges > 100, "Should have 100+ edges, got {}", total_edges);
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = rich_graph();
        populate_all_synthesis(&mut g);

        let synth_merges = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Merges)
                && (e.from.starts_with("DISC-MC-")
                    || e.from.starts_with("DISC-MT-")
                    || e.from.starts_with("DISC-TC-")
                    || e.from.starts_with("DISC-TT-")
                    || e.from.starts_with("DISC-ES-")
                    || e.from.starts_with("DISC-RM-")
                    || e.from.starts_with("DISC-GS-")
                    || e.from.starts_with("HYP-GS-"))
        }).count();
        assert!(synth_merges >= 30, "Should have 30+ cross-domain merge edges, got {}", synth_merges);
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = rich_graph();
        populate_all_synthesis(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_SYNTH_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        for cluster in ALL_SYNTH_CLUSTERS {
            add_entries(&mut g, cluster);
        }

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();

        assert_eq!(discoveries, 24, "24 discovery nodes");
        assert_eq!(hypotheses, 3, "3 hypothesis nodes");
        assert_eq!(discoveries + hypotheses, 27);
    }

    #[test]
    fn test_mc_bridges_span_correct_domains() {
        let mut g = rich_graph();
        populate_mc_discoveries(&mut g);

        for node in g.nodes.iter().filter(|n| n.id.starts_with("DISC-MC-")) {
            let domains: Vec<&str> = node.domain.split(", ").collect();
            assert!(
                domains.contains(&"Medical") || domains.contains(&"Cognitive"),
                "{} must span Medical or Cognitive, got: {}", node.id, node.domain
            );
        }
    }

    #[test]
    fn test_rm_bridges_span_correct_domains() {
        let mut g = rich_graph();
        populate_rm_discoveries(&mut g);

        for node in g.nodes.iter().filter(|n| n.id.starts_with("DISC-RM-")) {
            let domains: Vec<&str> = node.domain.split(", ").collect();
            assert!(
                domains.contains(&"Robotics") && domains.contains(&"Medical"),
                "{} must span both Robotics and Medical", node.id
            );
        }
    }

    #[test]
    fn test_hypothesis_nodes_lower_confidence() {
        for cluster in ALL_SYNTH_CLUSTERS {
            for e in *cluster {
                if matches!(e.node_type, NodeType::Hypothesis) {
                    assert!(
                        e.confidence < 0.80,
                        "Hypothesis {} should have confidence < 0.80, got {}",
                        e.id, e.confidence
                    );
                }
            }
        }
    }

    #[test]
    fn test_bridge_to_existing_creates_edges() {
        let mut g = rich_graph();
        populate_all_synthesis(&mut g);

        // GS-01 spans 6 domains - should connect to many existing nodes
        let gs01_bridges = g.edges.iter().filter(|e| {
            (e.from == "DISC-GS-01" || e.to == "DISC-GS-01")
                && matches!(e.edge_type, EdgeType::Merges)
        }).count();
        assert!(gs01_bridges >= 5, "DISC-GS-01 (6 domains) should bridge to 5+ existing nodes, got {}", gs01_bridges);
    }
}
