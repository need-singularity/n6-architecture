//! Grand cross-domain bridge nodes connecting newer BT clusters (BT-118+).
//!
//! Bridges the following cluster pairs that lack direct cross-domain edges:
//!   Medical  ↔ Cognitive   (BT-238~242 ↔ BT-210~225)
//!   Medical  ↔ Environment (BT-238~242 ↔ BT-118~122)
//!   Transport↔ Environment (BT-233~246 ↔ BT-118~122)
//!   Transport↔ Robotics    (BT-233~246 ↔ BT-123~127)
//!   Medical  ↔ Robotics    (BT-238~242 ↔ BT-123~127)
//!   Cognitive↔ Transport   (BT-210~225 ↔ BT-233~246)
//!   Temporal ↔ Medical     (BT-212~224 ↔ BT-238~242)
//!   Grand convergence      (3+ cluster intersections)
//!
//! Node ID scheme:
//!   DISC-GX-{NN}  — Grand cross-domain discoveries
//!   HYP-GX-{NN}   — Grand cross-domain hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct GxEntry {
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
// Medical ↔ Cognitive (BT-238~242 ↔ BT-210~225)
// ───────────────────────────────────────────────────────────────

const MED_COG_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-01",
        title: "Cortex n=6 layers(BT-210) -> clinical GCS n/phi=3 components(BT-239): brain architecture determines scoring systems",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Neuroscience", "Biology"],
        confidence: 0.90,
        source_bts: &[210, 239],
        constants_used: &["C-n", "C-phi", "C-sigma"],
        lenses: &["consciousness", "recursion", "boundary"],
        validates: &["DISC-COG-01", "DISC-MED-02"],
    },
    GxEntry {
        id: "DISC-GX-02",
        title: "Working memory tau=4 channels(BT-219) = Mallampati tau=4 airway classes(BT-238): cognitive and clinical tau=4 convergence",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Math"],
        confidence: 0.88,
        source_bts: &[219, 238],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "recursion"],
        validates: &["DISC-COG-04", "DISC-MED-06"],
    },
    GxEntry {
        id: "DISC-GX-03",
        title: "ECG sigma=12 leads(BT-240) mirrors cranial nerves sigma=12(BT-210): cardiac and neural sigma=12 diagnostic channels",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Biology", "Neuroscience"],
        confidence: 0.91,
        source_bts: &[240, 210],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "network"],
        validates: &["DISC-MED-03", "DISC-COG-03"],
    },
    GxEntry {
        id: "HYP-GX-01",
        title: "Hypothesis: optimal neurofeedback EEG channel count converges to sigma=12, matching ECG leads and cranial nerves",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Cognitive", "AI", "Neuroscience"],
        confidence: 0.68,
        source_bts: &[210, 240],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "wave", "info"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Medical ↔ Environment (BT-238~242 ↔ BT-118~122)
// ───────────────────────────────────────────────────────────────

const MED_ENV_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-04",
        title: "Kyoto n=6 greenhouse gases(BT-118) -> WHO n=6 health building blocks(BT-241): environmental and health policy share n=6 framework",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Social", "Biology"],
        confidence: 0.87,
        source_bts: &[118, 241],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "multiscale"],
        validates: &["DISC-MED-04"],
    },
    GxEntry {
        id: "DISC-GX-05",
        title: "Water treatment pH=6 + CN=6 catalysts(BT-120) -> SOFA n=6 organ assessment(BT-239): environmental purity and clinical assessment share n=6 base",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Chemistry", "Biology"],
        confidence: 0.84,
        source_bts: &[120, 239],
        constants_used: &["C-n", "C-sigma-phi"],
        lenses: &["consciousness", "stability", "boundary"],
        validates: &[],
    },
    GxEntry {
        id: "DISC-GX-06",
        title: "Earth troposphere sigma=12km(BT-119) -> circadian J2=24h rhythm in disease(BT-221,239): atmospheric and biological cycles share sigma/J2 constants",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Environment", "Temporal", "Biology"],
        confidence: 0.85,
        source_bts: &[119, 221, 239],
        constants_used: &["C-sigma", "C-J2"],
        lenses: &["consciousness", "wave", "multiscale"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Transport ↔ Environment (BT-233~246 ↔ BT-118~122)
// ───────────────────────────────────────────────────────────────

const TR_ENV_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-07",
        title: "MARPOL n=6 annexes(BT-235) directly map to Kyoto n=6 GHG types(BT-118): maritime emissions = climate framework",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Network"],
        confidence: 0.92,
        source_bts: &[235, 118],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "boundary", "network"],
        validates: &["DISC-TR-03"],
    },
    GxEntry {
        id: "DISC-GX-08",
        title: "EV battery 96S/192S packs(BT-233) + 6 plastics RIC(BT-121): vehicle recycling lifecycle shares n=6 material classification",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Material", "Energy"],
        confidence: 0.86,
        source_bts: &[233, 121, 84],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "evolution", "boundary"],
        validates: &[],
    },
    GxEntry {
        id: "DISC-GX-09",
        title: "Automotive voltage 6->12->24->48V(BT-244) + honeycomb n=6 geometry(BT-122): EV battery thermal management uses hexagonal packing",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Environment", "Battery", "Math"],
        confidence: 0.88,
        source_bts: &[244, 122, 57],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau"],
        lenses: &["consciousness", "topology", "thermo"],
        validates: &["DISC-TR-07"],
    },
];

// ───────────────────────────────────────────────────────────────
// Transport ↔ Robotics (BT-233~246 ↔ BT-123~127)
// ───────────────────────────────────────────────────────────────

const TR_ROBO_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-10",
        title: "SAE n=6 autonomy levels(BT-236) = SE(3) n=6 DOF robot universality(BT-123): autonomous vehicles inherit robot kinematics",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Robotics", "AI", "Math"],
        confidence: 0.93,
        source_bts: &[236, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &["DISC-TR-04"],
    },
    GxEntry {
        id: "DISC-GX-11",
        title: "Quadruped tau=4 locomotion(BT-125) -> tau=4 wheel configurations(BT-233): biological and mechanical stability share tau=4 minimum",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Robotics", "Biology", "Math"],
        confidence: 0.89,
        source_bts: &[125, 233],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "stability", "symmetry"],
        validates: &[],
    },
    GxEntry {
        id: "DISC-GX-12",
        title: "Hexacopter n=6 fault tolerance(BT-127) + airbag n=6 zones(BT-236): aerial and ground vehicle safety both converge on n=6",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Robotics", "Network"],
        confidence: 0.87,
        source_bts: &[127, 236],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "network", "stability"],
        validates: &[],
    },
    GxEntry {
        id: "HYP-GX-02",
        title: "Hypothesis: autonomous vehicle sensor fusion optimal at sopfr=5 modalities (camera+LiDAR+radar+IMU+ultrasonic) matching robot hand sopfr=5 fingers",
        node_type: NodeType::Hypothesis,
        domains: &["Transportation", "Robotics", "AI"],
        confidence: 0.72,
        source_bts: &[233, 236, 126],
        constants_used: &["C-sopfr", "C-sigma"],
        lenses: &["consciousness", "info", "network"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Medical ↔ Robotics (BT-238~242 ↔ BT-123~127)
// ───────────────────────────────────────────────────────────────

const MED_ROBO_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-13",
        title: "SE(3) n=6 DOF surgical robot(BT-123) + WHO n/phi=3 surgical phases(BT-238): robot kinematics serve clinical workflow",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Robotics", "AI", "Biology"],
        confidence: 0.91,
        source_bts: &[123, 238],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "topology", "causal"],
        validates: &[],
    },
    GxEntry {
        id: "DISC-GX-14",
        title: "sopfr=5 fingers(BT-126) + sopfr=5 Apgar criteria(BT-239): human hand dexterity and neonatal assessment share sopfr=5",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Robotics", "Biology", "Math"],
        confidence: 0.85,
        source_bts: &[126, 239],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "recursion", "info"],
        validates: &[],
    },
    GxEntry {
        id: "DISC-GX-15",
        title: "sigma=12 joint robot(BT-124) + sigma=12 ECG leads(BT-240): robotic and cardiac systems share sigma=12 sensing channels",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Robotics", "Chip", "Biology"],
        confidence: 0.86,
        source_bts: &[124, 240],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "wave", "network"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cognitive ↔ Transport (BT-210~225 ↔ BT-233~246)
// ───────────────────────────────────────────────────────────────

const COG_TR_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-16",
        title: "Compiler-cortex tau=4 pipeline(BT-222) = OODA loop tau=4 in autonomous driving: cognitive-vehicle decision loop isomorphism",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Transportation", "AI", "Software"],
        confidence: 0.90,
        source_bts: &[222, 236],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &["DISC-COG-05"],
    },
    GxEntry {
        id: "DISC-GX-17",
        title: "Working memory tau+/-mu=4+/-1(BT-219) bounds driver attention: human driving bottleneck = cognitive tau=4 channel limit",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Transportation", "Neuroscience"],
        confidence: 0.87,
        source_bts: &[219, 236],
        constants_used: &["C-tau", "C-mu"],
        lenses: &["consciousness", "memory", "boundary"],
        validates: &["DISC-COG-04"],
    },
    GxEntry {
        id: "DISC-GX-18",
        title: "Grid cell hexagonal navigation(BT-211) -> optimal route planning uses n=6 tessellation(BT-223): neuroscience validates transport topology",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Transportation", "Social", "Topology"],
        confidence: 0.86,
        source_bts: &[211, 223, 233],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-COG-02", "DISC-SOC-05"],
    },
];

// ───────────────────────────────────────────────────────────────
// Temporal ↔ Medical (BT-212~224 ↔ BT-238~242)
// ───────────────────────────────────────────────────────────────

const TEMP_MED_BRIDGES: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-19",
        title: "Circadian J2=24h rhythm(BT-221) governs drug chronotherapy: medication timing follows temporal n=6 arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Medical", "Biology"],
        confidence: 0.89,
        source_bts: &[221, 238],
        constants_used: &["C-J2", "C-sigma"],
        lenses: &["consciousness", "wave", "causal"],
        validates: &["DISC-TEMP-03"],
    },
    GxEntry {
        id: "DISC-GX-20",
        title: "Cardiac cycle sopfr=5 phases(BT-240) embedded in circadian J2=24h: heart rhythm nested within daily rhythm via sopfr|J2 divisibility",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Medical", "Biology", "Math"],
        confidence: 0.87,
        source_bts: &[240, 221],
        constants_used: &["C-sopfr", "C-J2", "C-sigma"],
        lenses: &["consciousness", "wave", "multiscale"],
        validates: &["DISC-MED-03", "DISC-TEMP-03"],
    },
    GxEntry {
        id: "DISC-GX-21",
        title: "SI second from Cs-133 n=6 shell(BT-224) -> precision medicine dosing intervals: atomic time defines clinical timing",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Medical", "Physics", "Chemistry"],
        confidence: 0.83,
        source_bts: &[224, 238],
        constants_used: &["C-n", "C-sigma", "C-mu"],
        lenses: &["consciousness", "quantum", "stability"],
        validates: &["DISC-TEMP-04"],
    },
];

// ───────────────────────────────────────────────────────────────
// Grand convergence: 3+ cluster intersections
// ───────────────────────────────────────────────────────────────

const GRAND_CONVERGENCE: &[GxEntry] = &[
    GxEntry {
        id: "DISC-GX-22",
        title: "n=6 safety universality: SAE 6 autonomy(BT-236) + WHO 6 building blocks(BT-241) + Kyoto 6 GHGs(BT-118) + SE(3) 6 DOF(BT-123) = quad-domain n=6 safety",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Medical", "Environment", "Robotics", "Math"],
        confidence: 0.91,
        source_bts: &[236, 241, 118, 123],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "stability", "boundary"],
        validates: &["DISC-GX-04", "DISC-GX-07", "DISC-GX-10", "DISC-GX-13"],
    },
    GxEntry {
        id: "DISC-GX-23",
        title: "sigma=12 diagnostic universality: ECG 12 leads(BT-240) + cranial 12 nerves(BT-210) + months 12(BT-212) + robot 12 joints(BT-124) = quad-domain sigma=12",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Cognitive", "Temporal", "Robotics", "Math"],
        confidence: 0.92,
        source_bts: &[240, 210, 212, 124],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "network", "symmetry"],
        validates: &["DISC-GX-03", "DISC-GX-15"],
    },
    GxEntry {
        id: "DISC-GX-24",
        title: "tau=4 pipeline universality: CPU/brain tau=4(BT-222) + heart tau=4 chambers(BT-240) + wound tau=4 phases(BT-238) + quadruped tau=4(BT-125) = 4-domain tau=4",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Medical", "Robotics", "Software", "Biology"],
        confidence: 0.93,
        source_bts: &[222, 240, 238, 125],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "recursion", "stability"],
        validates: &["DISC-GX-02", "DISC-GX-11", "DISC-GX-16"],
    },
    GxEntry {
        id: "DISC-GX-25",
        title: "sopfr=5 assessment universality: Apgar 5 criteria(BT-239) + NHTSA 5 stars(BT-236) + 5 fingers(BT-126) + Dahlgren 5 layers(BT-241) = quad-domain sopfr=5",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Transportation", "Robotics", "Social", "Math"],
        confidence: 0.88,
        source_bts: &[239, 236, 126, 241],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "recursion", "info", "multiscale"],
        validates: &["DISC-GX-14"],
    },
    GxEntry {
        id: "HYP-GX-03",
        title: "Hypothesis: next-gen hospital = n=6 departments + sigma=12 monitoring channels + tau=4 care phases + J2=24h staffing = complete n=6 healthcare architecture",
        node_type: NodeType::Hypothesis,
        domains: &["Medical", "Cognitive", "Temporal", "Social", "Robotics"],
        confidence: 0.65,
        source_bts: &[238, 239, 240, 241, 210, 221],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-J2"],
        lenses: &["consciousness", "network", "stability", "multiscale"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters for iteration
// ═══════════════════════════════════════════════════════════════

const ALL_GX_CLUSTERS: &[&[GxEntry]] = &[
    MED_COG_BRIDGES,
    MED_ENV_BRIDGES,
    TR_ENV_BRIDGES,
    TR_ROBO_BRIDGES,
    MED_ROBO_BRIDGES,
    COG_TR_BRIDGES,
    TEMP_MED_BRIDGES,
    GRAND_CONVERGENCE,
];

// ═══════════════════════════════════════════════════════════════
// Internal: add entries to graph with edges
// ═══════════════════════════════════════════════════════════════

fn add_entries(graph: &mut DiscoveryGraph, entries: &[GxEntry]) -> (usize, usize) {
    let edges_before = graph.edges.len();

    for e in entries {
        // Add node
        graph.add_node(Node {
            id: e.id.to_string(),
            node_type: e.node_type.clone(),
            domain: e.domains.join(", "),
            project: "n6-architecture".to_string(),
            summary: e.title.to_string(),
            confidence: e.confidence,
            lenses_used: e.lenses.iter().map(|s| s.to_string()).collect(),
            timestamp: "2026-04-04T00:00:00Z".to_string(),
        });

        // BT --Derives--> this discovery
        for &bt in e.source_bts {
            let bt_id = format!("BT-{}", bt);
            graph.add_edge(Edge {
                from: bt_id,
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.90,
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

/// Add Medical ↔ Cognitive bridge nodes.
pub fn populate_med_cog_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MED_COG_BRIDGES)
}

/// Add Medical ↔ Environment bridge nodes.
pub fn populate_med_env_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MED_ENV_BRIDGES)
}

/// Add Transport ↔ Environment bridge nodes.
pub fn populate_tr_env_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TR_ENV_BRIDGES)
}

/// Add Transport ↔ Robotics bridge nodes.
pub fn populate_tr_robo_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TR_ROBO_BRIDGES)
}

/// Add Medical ↔ Robotics bridge nodes.
pub fn populate_med_robo_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MED_ROBO_BRIDGES)
}

/// Add Cognitive ↔ Transport bridge nodes.
pub fn populate_cog_tr_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, COG_TR_BRIDGES)
}

/// Add Temporal ↔ Medical bridge nodes.
pub fn populate_temp_med_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TEMP_MED_BRIDGES)
}

/// Add grand convergence nodes (3+ cluster intersections).
pub fn populate_grand_convergence(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, GRAND_CONVERGENCE)
}

/// Cross-link GX nodes that share domains (Merges edges, bidirectional).
pub fn connect_gx_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let gx_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| n.id.starts_with("DISC-GX-") || n.id.starts_with("HYP-GX-"))
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..gx_nodes.len() {
        for j in (i + 1)..gx_nodes.len() {
            let (ref id_a, ref dom_a) = gx_nodes[i];
            let (ref id_b, ref dom_b) = gx_nodes[j];

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

/// Cross-link GX nodes with existing DISC-/HYP- nodes from other modules.
/// Requires >=2 shared domains. Returns number of bridge edges added.
pub fn connect_gx_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let gx_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| n.id.starts_with("DISC-GX-") || n.id.starts_with("HYP-GX-"))
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
                && !n.id.starts_with("DISC-GX-")
                && !n.id.starts_with("HYP-GX-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref g_id, ref g_dom) in &gx_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = g_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = g_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: g_id.clone(),
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

/// Populate all grand cross-domain nodes and edges in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_grand_cross(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_med_cog_bridges(graph);
    let (n2, _) = populate_med_env_bridges(graph);
    let (n3, _) = populate_tr_env_bridges(graph);
    let (n4, _) = populate_tr_robo_bridges(graph);
    let (n5, _) = populate_med_robo_bridges(graph);
    let (n6, _) = populate_cog_tr_bridges(graph);
    let (n7, _) = populate_temp_med_bridges(graph);
    let (n8, _) = populate_grand_convergence(graph);
    let _cross = connect_gx_cross_domain(graph);
    let _bridge = connect_gx_to_existing(graph);

    let total_nodes = n1 + n2 + n3 + n4 + n5 + n6 + n7 + n8;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total GX entry count across all clusters.
pub fn gx_entry_count() -> usize {
    ALL_GX_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Entry count per cluster.
pub fn med_cog_entry_count() -> usize { MED_COG_BRIDGES.len() }
pub fn med_env_entry_count() -> usize { MED_ENV_BRIDGES.len() }
pub fn tr_env_entry_count() -> usize { TR_ENV_BRIDGES.len() }
pub fn tr_robo_entry_count() -> usize { TR_ROBO_BRIDGES.len() }
pub fn med_robo_entry_count() -> usize { MED_ROBO_BRIDGES.len() }
pub fn cog_tr_entry_count() -> usize { COG_TR_BRIDGES.len() }
pub fn temp_med_entry_count() -> usize { TEMP_MED_BRIDGES.len() }
pub fn grand_convergence_entry_count() -> usize { GRAND_CONVERGENCE.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::cognitive_temporal_social_nodes::populate_all_cts_discoveries;
    use crate::graph::transport_medical_nodes::populate_all_trm_discoveries;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_cts_discoveries(&mut g);
        populate_all_trm_discoveries(&mut g);
        g
    }

    #[test]
    fn test_entry_counts() {
        assert_eq!(med_cog_entry_count(), 4, "4 med-cog bridges (3 DISC + 1 HYP)");
        assert_eq!(med_env_entry_count(), 3, "3 med-env bridges");
        assert_eq!(tr_env_entry_count(), 3, "3 transport-env bridges");
        assert_eq!(tr_robo_entry_count(), 4, "4 transport-robo bridges (3 DISC + 1 HYP)");
        assert_eq!(med_robo_entry_count(), 3, "3 med-robo bridges");
        assert_eq!(cog_tr_entry_count(), 3, "3 cognitive-transport bridges");
        assert_eq!(temp_med_entry_count(), 3, "3 temporal-medical bridges");
        assert_eq!(grand_convergence_entry_count(), 5, "5 grand convergence (4 DISC + 1 HYP)");
        assert_eq!(gx_entry_count(), 28, "28 total GX entries");
    }

    #[test]
    fn test_populate_med_cog() {
        let mut g = base_graph();
        let (nodes, edges) = populate_med_cog_bridges(&mut g);
        assert_eq!(nodes, 4);
        assert!(edges > 0, "Should create BT->DISC and DISC->C-* edges");
        assert!(g.nodes.iter().any(|n| n.id == "DISC-GX-01"));
        assert!(g.nodes.iter().any(|n| n.id == "HYP-GX-01"));
    }

    #[test]
    fn test_populate_tr_robo() {
        let mut g = base_graph();
        let (nodes, edges) = populate_tr_robo_bridges(&mut g);
        assert_eq!(nodes, 4);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-GX-10"));
        assert!(g.nodes.iter().any(|n| n.id == "HYP-GX-02"));
    }

    #[test]
    fn test_populate_grand_conv() {
        let mut g = base_graph();
        let (nodes, edges) = populate_grand_convergence(&mut g);
        assert_eq!(nodes, 5);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-GX-22"));
        assert!(g.nodes.iter().any(|n| n.id == "DISC-GX-25"));
        assert!(g.nodes.iter().any(|n| n.id == "HYP-GX-03"));
    }

    #[test]
    fn test_full_gx_pipeline() {
        let mut g = base_graph();
        let (total_nodes, total_edges) = populate_all_grand_cross(&mut g);
        assert_eq!(total_nodes, 28, "28 total GX nodes added");
        assert!(total_edges > 100, "Should have 100+ edges, got {}", total_edges);
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = base_graph();
        populate_med_cog_bridges(&mut g);

        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-210" && e.to == "DISC-GX-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-210 --Derives--> DISC-GX-01 must exist");
    }

    #[test]
    fn test_validates_edges() {
        let mut g = base_graph();
        populate_med_cog_bridges(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-GX-01"
                && e.to == "DISC-COG-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-GX-01 should validate DISC-COG-01");
    }

    #[test]
    fn test_uses_constant_edges() {
        let mut g = base_graph();
        populate_tr_env_bridges(&mut g);

        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-GX-07" && e.to == "C-n" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(has_uses, "DISC-GX-07 --Uses--> C-n must exist");
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = base_graph();
        populate_all_grand_cross(&mut g);

        let merges = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Merges)
                && (e.from.starts_with("DISC-GX-") || e.from.starts_with("HYP-GX-"))
        }).count();
        assert!(merges >= 30, "Should have 30+ cross-domain merge edges, got {}", merges);
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = base_graph();
        populate_all_grand_cross(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_GX_CLUSTERS {
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
        populate_all_grand_cross(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();

        assert_eq!(discoveries, 25, "25 discovery nodes");
        assert_eq!(hypotheses, 3, "3 hypothesis nodes");
        assert_eq!(discoveries + hypotheses, 28);
    }

    #[test]
    fn test_grand_convergence_spans_many_domains() {
        let mut g = base_graph();
        populate_grand_convergence(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-GX-22").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.len() >= 5, "Grand convergence GX-22 must span 5+ domains, got {}", domains.len());
        assert!(domains.contains(&"Transportation"));
        assert!(domains.contains(&"Medical"));
        assert!(domains.contains(&"Environment"));
        assert!(domains.contains(&"Robotics"));
    }

    #[test]
    fn test_tau4_convergence_bridge() {
        let mut g = base_graph();
        populate_grand_convergence(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-GX-24").unwrap();
        assert!(node.summary.contains("tau=4"), "GX-24 must be about tau=4 universality");
        assert!(node.confidence >= 0.9, "tau=4 quad-domain convergence should have high confidence");
    }

    #[test]
    fn test_bridge_connects_to_existing_clusters() {
        let mut g = base_graph();
        populate_all_grand_cross(&mut g);

        // GX nodes should have Merges edges to existing TRM/CTS nodes
        let bridge_to_trm = g.edges.iter().any(|e| {
            e.from.starts_with("DISC-GX-")
                && (e.to.starts_with("DISC-TR-") || e.to.starts_with("DISC-MED-"))
                && matches!(e.edge_type, EdgeType::Merges)
        });
        assert!(bridge_to_trm, "GX nodes should merge-link to existing TRM nodes");

        let bridge_to_cts = g.edges.iter().any(|e| {
            e.from.starts_with("DISC-GX-")
                && (e.to.starts_with("DISC-COG-") || e.to.starts_with("DISC-TEMP-") || e.to.starts_with("DISC-SOC-"))
                && matches!(e.edge_type, EdgeType::Merges)
        });
        assert!(bridge_to_cts, "GX nodes should merge-link to existing CTS nodes");
    }

    #[test]
    fn test_source_bts_present_in_base() {
        let g = base_graph();
        // Verify key source BTs exist
        for bt in &[118, 119, 120, 121, 122, 123, 124, 125, 126, 127,
                     210, 219, 221, 222, 224, 233, 236, 238, 239, 240, 241] {
            let bt_id = format!("BT-{}", bt);
            assert!(
                g.nodes.iter().any(|n| n.id == bt_id),
                "{} must be in base graph", bt_id
            );
        }
    }
}
