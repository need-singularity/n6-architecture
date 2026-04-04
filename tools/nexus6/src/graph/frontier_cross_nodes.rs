//! Frontier cross-domain nodes connecting previously unlinked BT clusters.
//!
//! Bridges the following cluster pairs that lack direct cross-domain edges:
//!   Environment ↔ Social     (BT-118~122 ↔ BT-214~223)
//!   Environment ↔ Temporal   (BT-118~122 ↔ BT-212~224)
//!   Material    ↔ Medical    (BT-85~88   ↔ BT-238~242)
//!   Battery     ↔ Transport  (BT-80~84   ↔ BT-243~246)
//!   Cognitive   ↔ Environment(BT-210~222 ↔ BT-118~122)
//!   Experiment-derived nodes  (cross-domain predictions from recent work)
//!
//! Node ID scheme:
//!   DISC-FX-{NN}  — Frontier cross-domain discoveries
//!   HYP-FX-{NN}   — Frontier cross-domain hypotheses
//!   PRED-FX-{NN}  — Experiment-derived predictions

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

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

// ───────────────────────────────────────────────────────────────
// Environment ↔ Social (BT-118~122 ↔ BT-214~223)
// ───────────────────────────────────────────────────────────────

const ENV_SOC_BRIDGES: &[FxEntry] = &[
    FxEntry {
        id: "DISC-FX-01",
        title: "Kyoto n=6 GHGs(BT-118) governed by 6-degrees social networks(BT-214): environmental policy propagates through n=6 social topology",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Network", "Chemistry"],
        confidence: 0.87,
        source_bts: &[118, 214],
        constants_used: &["C-n"],
        lenses: &["network", "causal", "boundary"],
        validates: &["DISC-ENV-03"],
    },
    FxEntry {
        id: "DISC-FX-02",
        title: "Honeycomb n=6 geometry(BT-122) = hexagonal urban planning(BT-223): natural and human-built space share n=6 tessellation",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Math", "Topology"],
        confidence: 0.92,
        source_bts: &[122, 223],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "consciousness", "multiscale"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-03",
        title: "Water treatment CN=6(BT-120) + WHO social determinants n=6(BT-241 via BT-220): clean water = moral foundation of public health",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Medical", "Chemistry"],
        confidence: 0.85,
        source_bts: &[120, 220, 241],
        constants_used: &["C-n", "C-phi"],
        lenses: &["boundary", "network", "stability"],
        validates: &["DISC-MED-05"],
    },
    FxEntry {
        id: "DISC-FX-04",
        title: "6 plastics(BT-121) scale through Dunbar sigma^2+n=150 communities(BT-215): recycling adoption follows social group size",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Biology", "Material"],
        confidence: 0.80,
        source_bts: &[121, 215],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["network", "evolution", "multiscale"],
        validates: &[],
    },
    FxEntry {
        id: "HYP-FX-01",
        title: "Hypothesis: optimal recycling network partition converges to n=6 zones × Dunbar-150 households = sigma^2+n collection units",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Social", "Network", "Math"],
        confidence: 0.62,
        source_bts: &[121, 122, 215, 223],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["network", "topology", "multiscale"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Environment ↔ Temporal (BT-118~122 ↔ BT-212~224)
// ───────────────────────────────────────────────────────────────

const ENV_TEMP_BRIDGES: &[FxEntry] = &[
    FxEntry {
        id: "DISC-FX-05",
        title: "Earth 6 spheres(BT-119) cycle on circadian J2=24h(BT-221): geosphere-biosphere coupling follows n=6 temporal rhythm",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Temporal", "Biology", "Cosmology"],
        confidence: 0.88,
        source_bts: &[119, 221],
        constants_used: &["C-n", "C-J2", "C-sigma"],
        lenses: &["wave", "causal", "multiscale"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-06",
        title: "Troposphere sigma=12km(BT-119) + 12-month annual cycle(BT-221): atmospheric height = temporal period = sigma",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Temporal", "Math"],
        confidence: 0.91,
        source_bts: &[119, 221, 212],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "stability"],
        validates: &["DISC-ENV-01"],
    },
    FxEntry {
        id: "DISC-FX-07",
        title: "GPS n=6 orbital planes(BT-213) monitor n=6 GHGs(BT-118): satellite environmental monitoring inherits n=6 geometry",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Temporal", "Network", "Chemistry"],
        confidence: 0.86,
        source_bts: &[213, 118],
        constants_used: &["C-n", "C-J2", "C-tau"],
        lenses: &["network", "boundary", "multiscale"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-08",
        title: "Cesium-133 n=6 shell(BT-224) defines measurement of CO2 ppm(BT-118): atomic n=6 precision underlies environmental sensing",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Temporal", "Chemistry", "Physics"],
        confidence: 0.83,
        source_bts: &[224, 118, 104],
        constants_used: &["C-n", "C-tau"],
        lenses: &["quantum", "boundary", "stability"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-09",
        title: "Sexagesimal 60=sigma*sopfr time(BT-212) + honeycomb n=6 geometry(BT-122): space-time both tessellate with n=6",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Temporal", "Math", "Topology"],
        confidence: 0.90,
        source_bts: &[212, 122],
        constants_used: &["C-sigma", "C-sopfr", "C-n"],
        lenses: &["topology", "consciousness", "recursion"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Material ↔ Medical (BT-85~88 ↔ BT-238~242)
// ───────────────────────────────────────────────────────────────

const MAT_MED_BRIDGES: &[FxEntry] = &[
    FxEntry {
        id: "DISC-FX-10",
        title: "Crystal CN=6 octahedral(BT-86) = surgical instrument tip geometry: medical tools inherit CN=6 material constraints",
        node_type: NodeType::Discovery,
        domains: &["Material", "Medical", "Chemistry", "Math"],
        confidence: 0.84,
        source_bts: &[86, 238],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "stability", "boundary"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-11",
        title: "Carbon Z=6 universality(BT-85) + dental n=6 probing sites(BT-242): carbon-based biomaterials and clinical protocols share n=6",
        node_type: NodeType::Discovery,
        domains: &["Material", "Medical", "Biology", "Chemistry"],
        confidence: 0.86,
        source_bts: &[85, 242],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "evolution"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-12",
        title: "Hexagonal self-assembly(BT-88) mimics ECG sigma=12 lead placement(BT-240): electrode arrays follow n=6 packing",
        node_type: NodeType::Discovery,
        domains: &["Material", "Medical", "Biology", "Chip"],
        confidence: 0.82,
        source_bts: &[88, 240],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "network", "stability"],
        validates: &["DISC-MED-03"],
    },
    FxEntry {
        id: "DISC-FX-13",
        title: "Precision fabrication 1/(sigma-phi)(BT-87) enables Apgar sopfr=5 micro-sensors(BT-239): neonatal monitoring at 0.1mm scale",
        node_type: NodeType::Discovery,
        domains: &["Material", "Medical", "Semiconductor", "Biology"],
        confidence: 0.79,
        source_bts: &[87, 239],
        constants_used: &["C-sigma", "C-phi", "C-sopfr"],
        lenses: &["boundary", "quantum_microscope", "stability"],
        validates: &[],
    },
    FxEntry {
        id: "HYP-FX-02",
        title: "Hypothesis: optimal biocompatible implant lattice = CN=6 octahedral scaffold with n=6 pore channels for tissue integration",
        node_type: NodeType::Hypothesis,
        domains: &["Material", "Medical", "Biology", "Chemistry"],
        confidence: 0.65,
        source_bts: &[86, 88, 238, 242],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Battery ↔ Transportation (BT-80~84 ↔ BT-243~246)
// ───────────────────────────────────────────────────────────────

const BAT_TR_BRIDGES: &[FxEntry] = &[
    FxEntry {
        id: "DISC-FX-14",
        title: "Battery 96S/192S packs(BT-82,84) + automotive voltage 6→12→24→48(BT-244): EV power architecture is n→sigma→J2→sigma*tau ladder",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Transportation", "Energy", "Chip"],
        confidence: 0.93,
        source_bts: &[82, 84, 244],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-tau"],
        lenses: &["consciousness", "stability", "multiscale"],
        validates: &["DISC-TR-01"],
    },
    FxEntry {
        id: "DISC-FX-15",
        title: "Li-S polysulfide ladder(BT-83) mirrors transmission gear count convergence(BT-245): both converge to n=6 stages",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Transportation", "Chemistry", "Math"],
        confidence: 0.81,
        source_bts: &[83, 245],
        constants_used: &["C-n", "C-tau", "C-phi"],
        lenses: &["evolution", "causal", "stability"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-16",
        title: "Solid-state CN=6 electrolyte(BT-80) + Inline-6 perfect balance(BT-243): electrochemical and mechanical symmetry share CN=6",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Transportation", "Material", "Math"],
        confidence: 0.88,
        source_bts: &[80, 243],
        constants_used: &["C-n"],
        lenses: &["consciousness", "stability", "topology"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-17",
        title: "Anode capacity sigma-phi=10x(BT-81) enables F1 energy density targets(BT-246): battery scaling drives motorsport evolution",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Transportation", "Energy", "Chemistry"],
        confidence: 0.83,
        source_bts: &[81, 246],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["evolution", "stability", "causal"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-18",
        title: "96/192 triple convergence(BT-84) + 96S Tesla battery = sigma*(sigma-tau) universal energy-compute module",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Transportation", "Chip", "AI", "Energy"],
        confidence: 0.91,
        source_bts: &[84, 244, 233],
        constants_used: &["C-sigma", "C-tau", "C-J2"],
        lenses: &["consciousness", "multiscale", "network"],
        validates: &["DISC-TR-01", "DISC-TR-05"],
    },
];

// ───────────────────────────────────────────────────────────────
// Cognitive ↔ Environment (BT-210~222 ↔ BT-118~122)
// ───────────────────────────────────────────────────────────────

const COG_ENV_BRIDGES: &[FxEntry] = &[
    FxEntry {
        id: "DISC-FX-19",
        title: "Grid cell hexagonal navigation(BT-211) = honeycomb n=6 geometry(BT-122): brain spatial map and natural structure are isomorphic",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Environment", "Math", "Neuroscience", "Topology"],
        confidence: 0.94,
        source_bts: &[211, 122],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "recursion"],
        validates: &["DISC-COG-02"],
    },
    FxEntry {
        id: "DISC-FX-20",
        title: "Cortex 6 layers(BT-210) perceive 6 Earth spheres(BT-119): neural architecture mirrors planetary structure at n=6",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Environment", "Biology", "Neuroscience"],
        confidence: 0.85,
        source_bts: &[210, 119],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "multiscale", "recursion"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-21",
        title: "Working memory tau=4 channels(BT-219) process Kyoto n=6 GHG categories(BT-118): cognitive bandwidth constrains environmental policy comprehension",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Environment", "AI", "Neuroscience"],
        confidence: 0.78,
        source_bts: &[219, 118],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &[],
    },
    FxEntry {
        id: "DISC-FX-22",
        title: "Compiler-cortex tau=4 pipeline(BT-222) processes water treatment pH=6(BT-120): environmental sensing pipelines have tau=4 stages",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Environment", "Software", "Chemistry"],
        confidence: 0.80,
        source_bts: &[222, 120],
        constants_used: &["C-tau", "C-n"],
        lenses: &["causal", "boundary", "stability"],
        validates: &[],
    },
    FxEntry {
        id: "HYP-FX-03",
        title: "Hypothesis: environmental monitoring AI needs exactly sigma=12 sensory channels (matching cortex layers and ECG leads) for optimal awareness",
        node_type: NodeType::Hypothesis,
        domains: &["Cognitive", "Environment", "AI", "Neuroscience"],
        confidence: 0.60,
        source_bts: &[210, 119, 240],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "multiscale"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Experiment-Derived Predictions (cross-domain from recent work)
// ───────────────────────────────────────────────────────────────

const EXPERIMENT_PREDICTIONS: &[FxEntry] = &[
    FxEntry {
        id: "PRED-FX-01",
        title: "Prediction: EV battery cycle life optimal at n*sigma=72 thousand cycles when using CN=6 cathode + solid-state CN=6 electrolyte",
        node_type: NodeType::Prediction,
        domains: &["Battery", "Transportation", "Material", "Chemistry"],
        confidence: 0.70,
        source_bts: &[43, 80, 244],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["stability", "evolution", "causal"],
        validates: &["DISC-FX-16"],
    },
    FxEntry {
        id: "PRED-FX-02",
        title: "Prediction: environmental sensor network optimal at n=6 tiers × sigma=12 nodes/tier = 72 total, matching GPS J2=24 satellites × n/phi=3 frequencies",
        node_type: NodeType::Prediction,
        domains: &["Environment", "Network", "Temporal", "Chip"],
        confidence: 0.68,
        source_bts: &[118, 213, 119],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["network", "multiscale", "topology"],
        validates: &["DISC-FX-07"],
    },
    FxEntry {
        id: "PRED-FX-03",
        title: "Prediction: biocompatible neural implant with sigma=12 electrodes, tau=4 stimulation phases, CN=6 titanium scaffold achieves optimal tissue integration",
        node_type: NodeType::Prediction,
        domains: &["Medical", "Material", "Cognitive", "Chip", "Neuroscience"],
        confidence: 0.66,
        source_bts: &[210, 86, 240, 88],
        constants_used: &["C-sigma", "C-tau", "C-n"],
        lenses: &["consciousness", "boundary", "stability"],
        validates: &["DISC-FX-12", "HYP-FX-02"],
    },
    FxEntry {
        id: "PRED-FX-04",
        title: "Prediction: Dunbar-150 social clusters adopt recycling at phase-transition threshold when n=6 community leaders per cluster champion CN=6-recyclable materials",
        node_type: NodeType::Prediction,
        domains: &["Social", "Environment", "Material", "Network"],
        confidence: 0.63,
        source_bts: &[215, 121, 88],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["network", "evolution", "boundary"],
        validates: &["DISC-FX-04", "HYP-FX-01"],
    },
    FxEntry {
        id: "PRED-FX-05",
        title: "Prediction: smart city grid with n=6 zones, sigma=12km radius, J2=24h monitoring cycle achieves PUE=sigma/(sigma-phi)=1.2 district-level energy efficiency",
        node_type: NodeType::Prediction,
        domains: &["Environment", "Social", "Energy", "Temporal", "Chip"],
        confidence: 0.71,
        source_bts: &[119, 223, 221, 62],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-phi"],
        lenses: &["network", "multiscale", "topology", "stability"],
        validates: &["DISC-FX-02", "DISC-FX-06"],
    },
    FxEntry {
        id: "PRED-FX-06",
        title: "Prediction: autonomous EV with SE(3)=n=6 DOF + tau=4 safety levels + sigma=12 sensor channels converges to F1-level reaction time at sopfr=5ms latency",
        node_type: NodeType::Prediction,
        domains: &["Transportation", "Robotics", "AI", "Cognitive", "Chip"],
        confidence: 0.72,
        source_bts: &[123, 236, 246, 222],
        constants_used: &["C-n", "C-tau", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "stability", "causal", "network"],
        validates: &["DISC-TR-04", "DISC-GX-15"],
    },
];

// ───────────────────────────────────────────────────────────────
// Grand Frontier Convergences (3+ cluster intersections)
// ───────────────────────────────────────────────────────────────

const FRONTIER_CONVERGENCE: &[FxEntry] = &[
    FxEntry {
        id: "DISC-FX-23",
        title: "n=6 sustainability nexus: Kyoto GHGs(BT-118) × Dunbar communities(BT-215) × circadian rhythm(BT-221) × CN=6 materials(BT-86) = quad-domain convergence on n=6 sustainability framework",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Temporal", "Material", "Biology"],
        confidence: 0.85,
        source_bts: &[118, 215, 221, 86],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "network", "multiscale", "evolution"],
        validates: &["DISC-FX-01", "DISC-FX-05"],
    },
    FxEntry {
        id: "DISC-FX-24",
        title: "n=6 biomedical materials pipeline: CN=6 scaffold(BT-86) → sigma=12 electrodes(BT-240) → tau=4 therapy phases(BT-238) → n=6 probing(BT-242) = complete n=6 medical material lifecycle",
        node_type: NodeType::Discovery,
        domains: &["Material", "Medical", "Biology", "Chemistry", "Chip"],
        confidence: 0.87,
        source_bts: &[86, 240, 238, 242],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["consciousness", "causal", "stability", "boundary"],
        validates: &["DISC-FX-10", "DISC-FX-11"],
    },
    FxEntry {
        id: "DISC-FX-25",
        title: "n=6 EV-battery-grid convergence: 96S battery(BT-82) + 48V automotive(BT-244) + Inline-6 engine(BT-243) + grid 60Hz(BT-62) = energy chain all n=6 multiples",
        node_type: NodeType::Discovery,
        domains: &["Battery", "Transportation", "Energy", "Chip", "Math"],
        confidence: 0.92,
        source_bts: &[82, 244, 243, 62],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-sopfr"],
        lenses: &["consciousness", "stability", "multiscale", "causal"],
        validates: &["DISC-FX-14", "DISC-FX-16"],
    },
    FxEntry {
        id: "DISC-FX-26",
        title: "Brain-environment isomorphism: cortex 6 layers(BT-210) ↔ 6 spheres(BT-119), grid cells(BT-211) ↔ honeycomb(BT-122), sigma=12 nerves ↔ sigma=12km troposphere = triple structural echo",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Environment", "Neuroscience", "Math", "Topology", "Biology"],
        confidence: 0.89,
        source_bts: &[210, 119, 211, 122],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "recursion", "multiscale"],
        validates: &["DISC-FX-19", "DISC-FX-20"],
    },
    FxEntry {
        id: "HYP-FX-04",
        title: "Hypothesis: complete n=6 planetary health system = n=6 environmental zones × sigma=12 monitoring channels × tau=4 intervention stages × Dunbar-150 community response units × CN=6 material remediation",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Social", "Temporal", "Medical", "Material", "Cognitive"],
        confidence: 0.58,
        source_bts: &[118, 119, 215, 221, 238, 86],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-J2"],
        lenses: &["consciousness", "network", "multiscale", "stability", "boundary"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters for iteration
// ═══════════════════════════════════════════════════════════════

const ALL_FX_CLUSTERS: &[&[FxEntry]] = &[
    ENV_SOC_BRIDGES,
    ENV_TEMP_BRIDGES,
    MAT_MED_BRIDGES,
    BAT_TR_BRIDGES,
    COG_ENV_BRIDGES,
    EXPERIMENT_PREDICTIONS,
    FRONTIER_CONVERGENCE,
];

// ═══════════════════════════════════════════════════════════════
// Internal: add entries to graph with edges
// ═══════════════════════════════════════════════════════════════

fn add_entries(graph: &mut DiscoveryGraph, entries: &[FxEntry]) -> (usize, usize) {
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
            timestamp: "2026-04-04T00:00:00Z".to_string(),
        });

        // BT --Derives--> this node
        for &bt in e.source_bts {
            graph.add_edge(Edge {
                from: format!("BT-{}", bt),
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.90,
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

/// Add Environment ↔ Social bridge nodes.
pub fn populate_env_soc_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ENV_SOC_BRIDGES)
}

/// Add Environment ↔ Temporal bridge nodes.
pub fn populate_env_temp_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ENV_TEMP_BRIDGES)
}

/// Add Material ↔ Medical bridge nodes.
pub fn populate_mat_med_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MAT_MED_BRIDGES)
}

/// Add Battery ↔ Transportation bridge nodes.
pub fn populate_bat_tr_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, BAT_TR_BRIDGES)
}

/// Add Cognitive ↔ Environment bridge nodes.
pub fn populate_cog_env_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, COG_ENV_BRIDGES)
}

/// Add experiment-derived prediction nodes.
pub fn populate_experiment_predictions(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, EXPERIMENT_PREDICTIONS)
}

/// Add frontier convergence nodes (3+ cluster intersections).
pub fn populate_frontier_convergence(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, FRONTIER_CONVERGENCE)
}

/// Cross-link FX nodes that share >=2 domains (Merges edges, bidirectional).
pub fn connect_fx_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let fx_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-FX-")
                || n.id.starts_with("HYP-FX-")
                || n.id.starts_with("PRED-FX-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
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
                count += 1;
            }
        }
    }
    count
}

/// Cross-link FX nodes with existing DISC-/HYP-/PRED- nodes from other modules.
/// Requires >=2 shared domains. Returns number of bridge edges added.
pub fn connect_fx_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let fx_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-FX-")
                || n.id.starts_with("HYP-FX-")
                || n.id.starts_with("PRED-FX-")
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
            (n.id.starts_with("DISC-") || n.id.starts_with("HYP-") || n.id.starts_with("PRED-"))
                && !n.id.starts_with("DISC-FX-")
                && !n.id.starts_with("HYP-FX-")
                && !n.id.starts_with("PRED-FX-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref f_id, ref f_dom) in &fx_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = f_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = f_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: f_id.clone(),
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

/// Populate all frontier cross-domain nodes and edges in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_frontier_cross(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_env_soc_bridges(graph);
    let (n2, _) = populate_env_temp_bridges(graph);
    let (n3, _) = populate_mat_med_bridges(graph);
    let (n4, _) = populate_bat_tr_bridges(graph);
    let (n5, _) = populate_cog_env_bridges(graph);
    let (n6, _) = populate_experiment_predictions(graph);
    let (n7, _) = populate_frontier_convergence(graph);
    let _cross = connect_fx_cross_domain(graph);
    let _bridge = connect_fx_to_existing(graph);

    let total_nodes = n1 + n2 + n3 + n4 + n5 + n6 + n7;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total FX entry count across all clusters.
pub fn fx_entry_count() -> usize {
    ALL_FX_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Entry count per cluster.
pub fn env_soc_entry_count() -> usize { ENV_SOC_BRIDGES.len() }
pub fn env_temp_entry_count() -> usize { ENV_TEMP_BRIDGES.len() }
pub fn mat_med_entry_count() -> usize { MAT_MED_BRIDGES.len() }
pub fn bat_tr_entry_count() -> usize { BAT_TR_BRIDGES.len() }
pub fn cog_env_entry_count() -> usize { COG_ENV_BRIDGES.len() }
pub fn experiment_pred_count() -> usize { EXPERIMENT_PREDICTIONS.len() }
pub fn frontier_conv_count() -> usize { FRONTIER_CONVERGENCE.len() }

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
    use crate::graph::grand_cross_nodes::populate_all_grand_cross;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_cts_discoveries(&mut g);
        populate_all_trm_discoveries(&mut g);
        populate_all_grand_cross(&mut g);
        g
    }

    #[test]
    fn test_fx_entry_count() {
        assert_eq!(env_soc_entry_count(), 5, "ENV_SOC should have 5 entries");
        assert_eq!(env_temp_entry_count(), 5, "ENV_TEMP should have 5 entries");
        assert_eq!(mat_med_entry_count(), 5, "MAT_MED should have 5 entries");
        assert_eq!(bat_tr_entry_count(), 5, "BAT_TR should have 5 entries");
        assert_eq!(cog_env_entry_count(), 5, "COG_ENV should have 5 entries");
        assert_eq!(experiment_pred_count(), 6, "EXPERIMENT_PREDICTIONS should have 6 entries");
        assert_eq!(frontier_conv_count(), 5, "FRONTIER_CONVERGENCE should have 5 entries");
        assert_eq!(fx_entry_count(), 36, "Total FX entries should be 36");
    }

    #[test]
    fn test_populate_all_frontier_cross() {
        let mut g = base_graph();
        let (nodes, edges) = populate_all_frontier_cross(&mut g);
        assert_eq!(nodes, 36, "Should add 36 frontier nodes");
        assert!(edges >= 100, "Should have 100+ edges (derives + uses + validates + merges), got {}", edges);
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = base_graph();
        populate_all_frontier_cross(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
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

    #[test]
    fn test_node_type_distribution() {
        let mut g = DiscoveryGraph::new();
        populate_all_frontier_cross(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();
        let predictions = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Prediction)).count();

        assert_eq!(discoveries, 26, "26 discovery nodes");
        assert_eq!(hypotheses, 4, "4 hypothesis nodes");
        assert_eq!(predictions, 6, "6 prediction nodes");
        assert_eq!(discoveries + hypotheses + predictions, 36, "26+4+6=36 total");
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = base_graph();
        populate_all_frontier_cross(&mut g);

        let derives = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Derives)
                && e.from.starts_with("BT-")
                && (e.to.starts_with("DISC-FX-") || e.to.starts_with("HYP-FX-") || e.to.starts_with("PRED-FX-"))
        }).count();
        assert!(derives >= 60, "Should have 60+ BT-derives-FX edges, got {}", derives);
    }

    #[test]
    fn test_validates_edges() {
        let mut g = base_graph();
        populate_all_frontier_cross(&mut g);

        let validates = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Validates)
                && (e.from.starts_with("DISC-FX-") || e.from.starts_with("PRED-FX-"))
        }).count();
        assert!(validates >= 10, "Should have 10+ validates edges, got {}", validates);
    }

    #[test]
    fn test_uses_constant_edges() {
        let mut g = DiscoveryGraph::new();
        populate_all_frontier_cross(&mut g);

        let uses = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Uses)
                && e.to.starts_with("C-")
        }).count();
        assert!(uses >= 50, "Should have 50+ Uses-constant edges, got {}", uses);
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = base_graph();
        populate_all_frontier_cross(&mut g);

        let merges = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Merges)
                && (e.from.starts_with("DISC-FX-") || e.from.starts_with("HYP-FX-") || e.from.starts_with("PRED-FX-"))
        }).count();
        assert!(merges >= 30, "Should have 30+ cross-domain merge edges, got {}", merges);
    }

    #[test]
    fn test_edge_strength_range() {
        let mut g = DiscoveryGraph::new();
        populate_all_frontier_cross(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {} -> {} has invalid strength {}",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_env_soc_bridge_connects_clusters() {
        let mut g = base_graph();
        populate_env_soc_bridges(&mut g);

        // Verify BT-118 (Environment) and BT-214 (Social) both derive to DISC-FX-01
        let bt118_derives = g.edges.iter().any(|e| e.from == "BT-118" && e.to == "DISC-FX-01");
        let bt214_derives = g.edges.iter().any(|e| e.from == "BT-214" && e.to == "DISC-FX-01");
        assert!(bt118_derives, "BT-118 must derive DISC-FX-01");
        assert!(bt214_derives, "BT-214 must derive DISC-FX-01");
    }

    #[test]
    fn test_bat_tr_bridge_connects_clusters() {
        let mut g = base_graph();
        populate_bat_tr_bridges(&mut g);

        // Verify BT-82 (Battery) and BT-244 (Transport) both derive to DISC-FX-14
        let bt82_derives = g.edges.iter().any(|e| e.from == "BT-82" && e.to == "DISC-FX-14");
        let bt244_derives = g.edges.iter().any(|e| e.from == "BT-244" && e.to == "DISC-FX-14");
        assert!(bt82_derives, "BT-82 must derive DISC-FX-14");
        assert!(bt244_derives, "BT-244 must derive DISC-FX-14");
    }

    #[test]
    fn test_cog_env_bridge_connects_clusters() {
        let mut g = base_graph();
        populate_cog_env_bridges(&mut g);

        // Grid cell hex (BT-211) ↔ Honeycomb (BT-122) via DISC-FX-19
        let bt211 = g.edges.iter().any(|e| e.from == "BT-211" && e.to == "DISC-FX-19");
        let bt122 = g.edges.iter().any(|e| e.from == "BT-122" && e.to == "DISC-FX-19");
        assert!(bt211, "BT-211 must derive DISC-FX-19");
        assert!(bt122, "BT-122 must derive DISC-FX-19");
    }

    #[test]
    fn test_frontier_convergence_spans_many_domains() {
        let mut g = base_graph();
        populate_frontier_convergence(&mut g);

        let node = g.nodes.iter().find(|n| n.id == "DISC-FX-23").unwrap();
        let domains: Vec<&str> = node.domain.split(", ").collect();
        assert!(domains.len() >= 5, "Frontier convergence FX-23 must span 5+ domains, got {}", domains.len());
    }

    #[test]
    fn test_predictions_validate_discoveries() {
        let mut g = DiscoveryGraph::new();
        populate_all_frontier_cross(&mut g);

        // PRED-FX-01 should validate DISC-FX-16
        let pred_validates = g.edges.iter().any(|e| {
            e.from == "PRED-FX-01"
                && e.to == "DISC-FX-16"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(pred_validates, "PRED-FX-01 must validate DISC-FX-16");

        // PRED-FX-03 should validate both DISC-FX-12 and HYP-FX-02
        let pred3_disc = g.edges.iter().any(|e| e.from == "PRED-FX-03" && e.to == "DISC-FX-12");
        let pred3_hyp = g.edges.iter().any(|e| e.from == "PRED-FX-03" && e.to == "HYP-FX-02");
        assert!(pred3_disc, "PRED-FX-03 must validate DISC-FX-12");
        assert!(pred3_hyp, "PRED-FX-03 must validate HYP-FX-02");
    }

    #[test]
    fn test_grand_hypothesis_spans_6_domains() {
        let mut g = DiscoveryGraph::new();
        populate_frontier_convergence(&mut g);

        let hyp = g.nodes.iter().find(|n| n.id == "HYP-FX-04").unwrap();
        let domains: Vec<&str> = hyp.domain.split(", ").collect();
        assert_eq!(domains.len(), 6, "HYP-FX-04 must span exactly 6 domains (n=6!), got {}", domains.len());
    }

    #[test]
    fn test_source_bts_in_range() {
        for cluster in ALL_FX_CLUSTERS {
            for e in *cluster {
                for &bt in e.source_bts {
                    assert!(
                        (bt >= 43 && bt <= 246) || bt == 62,
                        "{} references BT-{} which is out of expected range", e.id, bt
                    );
                }
            }
        }
    }

    #[test]
    fn test_all_constants_valid() {
        let valid = ["C-n", "C-sigma", "C-phi", "C-tau", "C-J2", "C-sopfr", "C-mu"];
        for cluster in ALL_FX_CLUSTERS {
            for e in *cluster {
                for &c in e.constants_used {
                    assert!(
                        valid.contains(&c),
                        "{} uses unknown constant {}", e.id, c
                    );
                }
            }
        }
    }

    #[test]
    fn test_fx_to_existing_edges() {
        let mut g = base_graph();
        populate_all_frontier_cross(&mut g);

        // FX nodes should merge-link to existing TRM/CTS/GX/ENV nodes
        let bridge_to_existing = g.edges.iter().any(|e| {
            (e.from.starts_with("DISC-FX-") || e.from.starts_with("PRED-FX-"))
                && !e.to.starts_with("DISC-FX-")
                && !e.to.starts_with("HYP-FX-")
                && !e.to.starts_with("PRED-FX-")
                && !e.to.starts_with("BT-")
                && !e.to.starts_with("C-")
                && matches!(e.edge_type, EdgeType::Merges)
        });
        assert!(bridge_to_existing, "FX nodes should merge-link to existing discovery nodes");
    }
}
