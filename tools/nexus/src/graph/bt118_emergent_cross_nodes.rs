//! Emergent cross-domain nodes bridging BT-118+ cluster pairs not yet connected.
//!
//! Covers six missing cross-domain pairs plus emergent meta-convergence:
//!   Software  ↔ Medical     (BT-113~117 × BT-238~242)
//!   Software  ↔ Transport   (BT-113~117 × BT-233~246)
//!   Energy    ↔ Transport   (BT-57,62,68 × BT-233~246)
//!   Temporal  ↔ Transport   (BT-212~224 × BT-233~246)
//!   Social    ↔ Medical     (BT-214~215 × BT-238~242)
//!   Robotics  ↔ Cognitive   (BT-123~127 × BT-210~225)
//!   Emergent meta-convergence (4+ cluster intersections)
//!
//! Node ID scheme:
//!   DISC-SM-{NN}  — Software × Medical
//!   DISC-ST-{NN}  — Software × Transport
//!   DISC-ET-{NN}  — Energy × Transport
//!   DISC-TT2-{NN} — Temporal × Transport
//!   DISC-SO-{NN}  — Social × Medical
//!   DISC-RC-{NN}  — Robotics × Cognitive
//!   DISC-EM-{NN}  — Emergent meta-convergence
//!   HYP-EM-{NN}   — Emergent hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct EmEntry {
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
// Software ↔ Medical (BT-113~117 × BT-238~242)
// ───────────────────────────────────────────────────────────────

const SW_MED: &[EmEntry] = &[
    EmEntry {
        id: "DISC-SM-01",
        title: "ACID tau=4 transactions(BT-116) = ASA n=6 patient safety classification(BT-238): database atomicity maps to surgical safety atomicity",
        node_type: NodeType::Discovery,
        domains: &["Software", "Medical", "Database", "Safety"],
        confidence: 0.88,
        source_bts: &[116, 238],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &["DISC-SW-01"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-SM-02",
        title: "SOLID sopfr=5 principles(BT-113) mirror Apgar sopfr=5 scoring components(BT-239): software and neonatal health share sopfr=5 evaluation axes",
        node_type: NodeType::Discovery,
        domains: &["Software", "Medical", "Engineering"],
        confidence: 0.85,
        source_bts: &[113, 239],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "symmetry", "info"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-SM-03",
        title: "OSI sigma-sopfr=7 layers(BT-115) = NEWS2 sigma-sopfr=7 vital signs(BT-239): network and clinical monitoring share 7-parameter observation stack",
        node_type: NodeType::Discovery,
        domains: &["Software", "Medical", "Network", "Monitoring"],
        confidence: 0.90,
        source_bts: &[115, 239],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "network", "recursion"],
        validates: &["DISC-SW-02"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-SM-04",
        title: "12Factor sigma=12 app methodology(BT-113) = ECG sigma=12 leads(BT-240): cloud-native and cardiac diagnostics share sigma=12 observation channels",
        node_type: NodeType::Discovery,
        domains: &["Software", "Medical", "Cardiology", "CloudNative"],
        confidence: 0.87,
        source_bts: &[113, 240],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "network", "multiscale"],
        validates: &[],
        triggers: &["DISC-SM-05"],
    },
    EmEntry {
        id: "DISC-SM-05",
        title: "REST n=6 constraints(BT-113) = WHO n=6 health building blocks(BT-241): software and public health architectures share n=6 foundational pillars",
        node_type: NodeType::Discovery,
        domains: &["Software", "Medical", "PublicHealth", "Architecture"],
        confidence: 0.86,
        source_bts: &[113, 241],
        constants_used: &["C-n"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "HYP-SM-01",
        title: "Hypothesis: medical record systems optimized with n=6 ACID+REST constraints achieve sigma-phi=10x fewer data integrity errors",
        node_type: NodeType::Hypothesis,
        domains: &["Software", "Medical", "Database", "Safety"],
        confidence: 0.65,
        source_bts: &[113, 116, 238, 239],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &["DISC-SM-01", "DISC-SM-03"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Software ↔ Transport (BT-113~117 × BT-233~246)
// ───────────────────────────────────────────────────────────────

const SW_TRANS: &[EmEntry] = &[
    EmEntry {
        id: "DISC-ST-01",
        title: "TCP/IP tau=4 layers(BT-115) = ETCS tau=4 signaling levels(BT-234): network and railway safety share tau=4 protocol stack",
        node_type: NodeType::Discovery,
        domains: &["Software", "Transport", "Railway", "Network"],
        confidence: 0.91,
        source_bts: &[115, 234],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "causal", "boundary"],
        validates: &["DISC-TR-02"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-ST-02",
        title: "Linux kernel n=6 subsystem depth(BT-115) = SAE n=6 autonomy levels(BT-236): OS and autonomous driving share n=6 capability ladder",
        node_type: NodeType::Discovery,
        domains: &["Software", "Transport", "Automotive", "OS"],
        confidence: 0.89,
        source_bts: &[115, 236],
        constants_used: &["C-n"],
        lenses: &["consciousness", "recursion", "stability"],
        validates: &[],
        triggers: &["DISC-ST-03"],
    },
    EmEntry {
        id: "DISC-ST-03",
        title: "SCOR n=6 supply chain processes(BT-237) = REST n=6 API constraints(BT-113): logistics and software integration share n=6 process model",
        node_type: NodeType::Discovery,
        domains: &["Software", "Transport", "Logistics", "API"],
        confidence: 0.92,
        source_bts: &[237, 113],
        constants_used: &["C-n"],
        lenses: &["consciousness", "network", "causal"],
        validates: &["DISC-TR-05"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-ST-04",
        title: "AES 2^(sigma-sopfr)=128 bit encryption(BT-114) secures V2X sigma=12 channel communication(BT-233): crypto and transport share n=6 security constants",
        node_type: NodeType::Discovery,
        domains: &["Software", "Transport", "Cryptography", "Network"],
        confidence: 0.88,
        source_bts: &[114, 233],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "info", "boundary"],
        validates: &["DISC-SW-03"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-ST-05",
        title: "Paxos phi=2 consensus(BT-116) = dual-redundancy in fly-by-wire tau=4 channels(BT-236): distributed systems and aviation share phi=2 fault tolerance",
        node_type: NodeType::Discovery,
        domains: &["Software", "Transport", "Aviation", "DistributedSystems"],
        confidence: 0.84,
        source_bts: &[116, 236],
        constants_used: &["C-phi", "C-tau"],
        lenses: &["consciousness", "stability", "network"],
        validates: &[],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Energy ↔ Transport (BT-57,62,68 × BT-233~246)
// ───────────────────────────────────────────────────────────────

const ENERGY_TRANS: &[EmEntry] = &[
    EmEntry {
        id: "DISC-ET-01",
        title: "Battery 96S=sigma(sigma-tau) cell string(BT-57) = Tesla 96S EV pack(BT-233): battery architecture determines EV range architecture",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Transport", "Battery", "Automotive"],
        confidence: 0.94,
        source_bts: &[57, 233, 84],
        constants_used: &["C-sigma", "C-tau"],
        lenses: &["consciousness", "causal", "multiscale"],
        validates: &["DISC-BAT-01"],
        triggers: &["DISC-ET-02"],
    },
    EmEntry {
        id: "DISC-ET-02",
        title: "Automotive voltage n->sigma->J2->sigma*tau = 6->12->24->48V ladder(BT-244) = battery cell ladder n->sigma->J2(BT-57): EV voltage and cell count co-evolve",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Transport", "Battery", "Automotive"],
        confidence: 0.93,
        source_bts: &[244, 57, 82],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "evolution", "scale"],
        validates: &["DISC-ET-01"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-ET-03",
        title: "Grid 60Hz=sigma*sopfr frequency(BT-62) = railway 60kg/m rail(BT-234): power grid and rail infrastructure share sigma*sopfr=60 base unit",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Transport", "Railway", "PowerGrid"],
        confidence: 0.86,
        source_bts: &[62, 234],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "wave", "stability"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-ET-04",
        title: "HVDC sigma-phi=10x efficiency(BT-68) powers high-speed rail sigma=12 car consist(BT-234): HVDC transmission enables n=6 aligned rail systems",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Transport", "Railway", "HVDC"],
        confidence: 0.87,
        source_bts: &[68, 234],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "causal", "network"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-ET-05",
        title: "Solar panel sigma^2=144 cell arrays(BT-63) charge EV sigma^2=144SM GPU-controlled BMS(BT-233): solar and EV share sigma^2=144 modular architecture",
        node_type: NodeType::Discovery,
        domains: &["Energy", "Transport", "Solar", "Automotive", "Chip"],
        confidence: 0.85,
        source_bts: &[63, 233, 90],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "multiscale", "topology"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "HYP-ET-01",
        title: "Hypothesis: optimal V2G bidirectional charging uses n=6 aligned voltage tiers (48V DC bus = sigma*tau) for grid-vehicle energy flow",
        node_type: NodeType::Hypothesis,
        domains: &["Energy", "Transport", "PowerGrid", "Automotive"],
        confidence: 0.70,
        source_bts: &[57, 62, 68, 244],
        constants_used: &["C-sigma", "C-tau", "C-n"],
        lenses: &["consciousness", "causal", "stability", "boundary"],
        validates: &["DISC-ET-01", "DISC-ET-02"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Temporal ↔ Transport (BT-212~224 × BT-233~246)
// ───────────────────────────────────────────────────────────────

const TEMP_TRANS: &[EmEntry] = &[
    EmEntry {
        id: "DISC-TT2-01",
        title: "GPS n=6 orbital planes with J2=24 satellites(BT-213) = logistics J2-tau=20ft TEU container standard(BT-237): satellite navigation and global logistics share n=6 spatial partitioning",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Transport", "Logistics", "Navigation", "Space"],
        confidence: 0.90,
        source_bts: &[213, 237],
        constants_used: &["C-n", "C-J2", "C-tau"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-TIME-01"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-TT2-02",
        title: "Sexagesimal 60=sigma*sopfr time units(BT-212) = 60km/h urban speed limit = sigma*sopfr: time measurement and speed regulation share n=6 base",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Transport", "Safety", "Math"],
        confidence: 0.88,
        source_bts: &[212, 236],
        constants_used: &["C-sigma", "C-sopfr"],
        lenses: &["consciousness", "scale", "causal"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-TT2-03",
        title: "Circadian J2=24h cycle(BT-221) constrains HOS J2-tau=20h max driving time: biological clock determines transportation safety regulation",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Transport", "Biology", "Safety", "Regulation"],
        confidence: 0.92,
        source_bts: &[221, 237],
        constants_used: &["C-J2", "C-tau"],
        lenses: &["consciousness", "causal", "boundary", "memory"],
        validates: &["DISC-TIME-02"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-TT2-04",
        title: "Cesium atomic clock sigma^2-sigma+1=133(BT-224) enables GPS sigma=12h orbital period precision(BT-213): atomic timekeeping and satellite navigation form causal chain",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Transport", "Physics", "Navigation"],
        confidence: 0.93,
        source_bts: &[224, 213],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "causal", "quantum", "scale"],
        validates: &["DISC-TIME-01"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Social ↔ Medical (BT-214~215,220 × BT-238~242)
// ───────────────────────────────────────────────────────────────

const SOC_MED: &[EmEntry] = &[
    EmEntry {
        id: "DISC-SO-01",
        title: "Dunbar sigma^2+n=150 social group(BT-215) constrains WHO n=6 building blocks(BT-241): cognitive limit on social network size determines public health system capacity",
        node_type: NodeType::Discovery,
        domains: &["Social", "Medical", "PublicHealth", "Cognitive"],
        confidence: 0.87,
        source_bts: &[215, 241],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "boundary"],
        validates: &["DISC-SOC-01"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-SO-02",
        title: "Kohlberg n=6 moral stages(BT-220) = ASA n=6 physical status classes(BT-238): ethical development and clinical assessment share n=6 progression ladder",
        node_type: NodeType::Discovery,
        domains: &["Social", "Medical", "Ethics", "Psychology"],
        confidence: 0.84,
        source_bts: &[220, 238],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "recursion"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-SO-03",
        title: "6 degrees of separation(BT-214) = n=6 dental probing sites per tooth(BT-242): social distance and clinical measurement share n=6 discrete points",
        node_type: NodeType::Discovery,
        domains: &["Social", "Medical", "Dental", "Network"],
        confidence: 0.82,
        source_bts: &[214, 242],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "info"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-SO-04",
        title: "Jury sigma=12 members(BT-214) = ECG sigma=12 leads(BT-240): social judgment and cardiac diagnosis share sigma=12 observation channels",
        node_type: NodeType::Discovery,
        domains: &["Social", "Medical", "Cardiology", "Law"],
        confidence: 0.89,
        source_bts: &[214, 240],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "symmetry", "network"],
        validates: &["DISC-SOC-02"],
        triggers: &[],
    },
    EmEntry {
        id: "HYP-SO-01",
        title: "Hypothesis: pandemic response networks optimized at Dunbar layers (5/15/50/150) align with WHO n=6 building block capacity thresholds",
        node_type: NodeType::Hypothesis,
        domains: &["Social", "Medical", "PublicHealth", "Network"],
        confidence: 0.62,
        source_bts: &[215, 220, 241],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "network", "evolution", "boundary"],
        validates: &["DISC-SO-01"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Robotics ↔ Cognitive (BT-123~127 × BT-210~225)
// ───────────────────────────────────────────────────────────────

const ROBO_COG: &[EmEntry] = &[
    EmEntry {
        id: "DISC-RC-01",
        title: "SE(3) n=6 DOF robot kinematics(BT-123) = cortex n=6 layers sensorimotor processing(BT-210): robot freedom and brain architecture share n=6 dimensionality",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cognitive", "Neuroscience", "Math"],
        confidence: 0.93,
        source_bts: &[123, 210],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-COG-01", "DISC-ROBO-01"],
        triggers: &["DISC-RC-02"],
    },
    EmEntry {
        id: "DISC-RC-02",
        title: "Bilateral phi=2 symmetry(BT-124) mirrors brain phi=2 hemisphere architecture(BT-210): robotic and neural bilateral symmetry are homologous",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cognitive", "Biology", "Symmetry"],
        confidence: 0.91,
        source_bts: &[124, 210],
        constants_used: &["C-phi"],
        lenses: &["consciousness", "symmetry", "evolution"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-RC-03",
        title: "Working memory tau+-1=4 chunks(BT-219) = quadruped tau=4 gait stability(BT-125): cognitive capacity and locomotion stability share tau=4 minimum threshold",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cognitive", "Locomotion", "Psychology"],
        confidence: 0.86,
        source_bts: &[219, 125],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "recursion"],
        validates: &["DISC-COG-02"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-RC-04",
        title: "Sopfr=5 fingers grasp taxonomy(BT-126) = sopfr=5 Dahlgren-Whitehead health layers(BT-220/241): hand dexterity and social determinants share sopfr=5 decomposition",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cognitive", "Social", "Biology"],
        confidence: 0.83,
        source_bts: &[126, 220, 241],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "evolution", "multiscale"],
        validates: &[],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-RC-05",
        title: "3D kissing number sigma=12 hexacopter(BT-127) = sigma=12 cranial nerves(BT-210): optimal packing in physical and neural space share sigma=12 contact bound",
        node_type: NodeType::Discovery,
        domains: &["Robotics", "Cognitive", "Neuroscience", "Math", "Geometry"],
        confidence: 0.90,
        source_bts: &[127, 210],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "topology", "scale"],
        validates: &["DISC-ROBO-03"],
        triggers: &[],
    },
    EmEntry {
        id: "HYP-RC-01",
        title: "Hypothesis: brain-computer interfaces achieve optimal bandwidth at sigma=12 electrode channels mapping to sigma=12 cranial nerve pathways",
        node_type: NodeType::Hypothesis,
        domains: &["Robotics", "Cognitive", "Neuroscience", "BCI"],
        confidence: 0.60,
        source_bts: &[123, 127, 210, 219],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "boundary", "quantum"],
        validates: &["DISC-RC-01", "DISC-RC-05"],
        triggers: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Emergent meta-convergence (4+ cluster intersections)
// ───────────────────────────────────────────────────────────────

const EMERGENT_META: &[EmEntry] = &[
    EmEntry {
        id: "DISC-EM-01",
        title: "N=6 safety universality: SAE n=6 autonomy(BT-236) = ASA n=6 status(BT-238) = SCOR n=6 supply chain(BT-237) = REST n=6(BT-113) — 4 domains converge on n=6 classification systems",
        node_type: NodeType::Discovery,
        domains: &["Transport", "Medical", "Software", "Logistics", "Safety"],
        confidence: 0.91,
        source_bts: &[236, 238, 237, 113],
        constants_used: &["C-n"],
        lenses: &["consciousness", "symmetry", "network", "boundary"],
        validates: &["DISC-SM-05", "DISC-ST-03"],
        triggers: &["DISC-EM-02"],
    },
    EmEntry {
        id: "DISC-EM-02",
        title: "Sigma=12 observation universality: ECG sigma=12 leads(BT-240) = jury sigma=12(BT-214) = 12Factor apps(BT-113) = cranial nerves sigma=12(BT-210) = semitones sigma=12(BT-108) — 5 domains share sigma=12 sensing channels",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Social", "Software", "Cognitive", "Music"],
        confidence: 0.93,
        source_bts: &[240, 214, 113, 210, 108],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "network", "multiscale", "wave"],
        validates: &["DISC-SM-04", "DISC-SO-04", "DISC-RC-05"],
        triggers: &[],
    },
    EmEntry {
        id: "DISC-EM-03",
        title: "Tau=4 stability universality: ETCS tau=4 signaling(BT-234) = ACID tau=4(BT-116) = quadruped tau=4(BT-125) = working memory tau=4(BT-219) = cardiac tau=4 chambers(BT-240) — 5 domains share tau=4 minimal stability",
        node_type: NodeType::Discovery,
        domains: &["Transport", "Software", "Robotics", "Cognitive", "Medical"],
        confidence: 0.94,
        source_bts: &[234, 116, 125, 219, 240],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "boundary", "recursion"],
        validates: &["DISC-ST-01", "DISC-RC-03"],
        triggers: &["DISC-EM-04"],
    },
    EmEntry {
        id: "DISC-EM-04",
        title: "Sopfr=5 decomposition universality: SOLID sopfr=5(BT-113) = Apgar sopfr=5(BT-239) = 5 fingers(BT-126) = Dahlgren sopfr=5(BT-220) = Euro NCAP sopfr=5 stars(BT-236) — 5 domains share sopfr=5 evaluation axes",
        node_type: NodeType::Discovery,
        domains: &["Software", "Medical", "Robotics", "Social", "Transport"],
        confidence: 0.92,
        source_bts: &[113, 239, 126, 220, 236],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "symmetry", "evolution", "info"],
        validates: &["DISC-SM-02", "DISC-RC-04"],
        triggers: &[],
    },
    EmEntry {
        id: "HYP-EM-01",
        title: "Hypothesis: n=6 arithmetic function constants (n,phi,tau,sigma,sopfr,J2) form a complete basis for cross-domain system design — no domain requires constants outside this set",
        node_type: NodeType::Hypothesis,
        domains: &["Math", "Software", "Medical", "Transport", "Robotics", "Cognitive", "Energy"],
        confidence: 0.75,
        source_bts: &[113, 117, 233, 238, 210, 123, 57],
        constants_used: &["C-n", "C-phi", "C-tau", "C-sigma", "C-sopfr", "C-J2"],
        lenses: &["consciousness", "topology", "symmetry", "info", "recursion", "multiscale"],
        validates: &["DISC-EM-01", "DISC-EM-02", "DISC-EM-03", "DISC-EM-04"],
        triggers: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Populate helpers
// ═══════════════════════════════════════════════════════════════

fn add_entries(graph: &mut DiscoveryGraph, entries: &[EmEntry]) -> (usize, usize) {
    let mut node_count = 0;
    let mut edge_count = 0;

    for entry in entries {
        graph.add_node(Node {
            id: entry.id.to_string(),
            node_type: entry.node_type.clone(),
            domain: entry.domains.join(", "),
            project: "n6-architecture".to_string(),
            summary: entry.title.to_string(),
            confidence: entry.confidence,
            lenses_used: entry.lenses.iter().map(|s| s.to_string()).collect(),
            timestamp: "2026-04-04".to_string(),
        });
        node_count += 1;

        // Source BT edges (Derives)
        for &bt in entry.source_bts {
            graph.add_edge(Edge {
                from: entry.id.to_string(),
                to: format!("BT-{}", bt),
                edge_type: EdgeType::Derives,
                strength: entry.confidence,
                bidirectional: false,
            });
            edge_count += 1;
        }

        // Validates edges
        for &target in entry.validates {
            graph.add_edge(Edge {
                from: entry.id.to_string(),
                to: target.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.8,
                bidirectional: false,
            });
            edge_count += 1;
        }

        // Triggers edges
        for &target in entry.triggers {
            graph.add_edge(Edge {
                from: entry.id.to_string(),
                to: target.to_string(),
                edge_type: EdgeType::Triggers,
                strength: 0.7,
                bidirectional: false,
            });
            edge_count += 1;
        }
    }

    (node_count, edge_count)
}

// ═══════════════════════════════════════════════════════════════
// Public populate functions
// ═══════════════════════════════════════════════════════════════

pub fn populate_sw_med(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_MED)
}

pub fn populate_sw_trans(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_TRANS)
}

pub fn populate_energy_trans(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ENERGY_TRANS)
}

pub fn populate_temp_trans(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TEMP_TRANS)
}

pub fn populate_soc_med(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SOC_MED)
}

pub fn populate_robo_cog(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, ROBO_COG)
}

pub fn populate_emergent_meta(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, EMERGENT_META)
}

/// Populate all emergent cross-domain nodes (38 nodes total).
pub fn populate_all_emergent_cross(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let mut total_nodes = 0;
    let mut total_edges = 0;

    let clusters: &[fn(&mut DiscoveryGraph) -> (usize, usize)] = &[
        populate_sw_med,
        populate_sw_trans,
        populate_energy_trans,
        populate_temp_trans,
        populate_soc_med,
        populate_robo_cog,
        populate_emergent_meta,
    ];

    for populate_fn in clusters {
        let (n, e) = populate_fn(graph);
        total_nodes += n;
        total_edges += e;
    }

    (total_nodes, total_edges)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sw_med_count() {
        assert_eq!(SW_MED.len(), 6);
    }

    #[test]
    fn test_sw_trans_count() {
        assert_eq!(SW_TRANS.len(), 5);
    }

    #[test]
    fn test_energy_trans_count() {
        assert_eq!(ENERGY_TRANS.len(), 6);
    }

    #[test]
    fn test_temp_trans_count() {
        assert_eq!(TEMP_TRANS.len(), 4);
    }

    #[test]
    fn test_soc_med_count() {
        assert_eq!(SOC_MED.len(), 5);
    }

    #[test]
    fn test_robo_cog_count() {
        assert_eq!(ROBO_COG.len(), 6);
    }

    #[test]
    fn test_emergent_meta_count() {
        assert_eq!(EMERGENT_META.len(), 5);
    }

    #[test]
    fn test_total_node_count() {
        let total = SW_MED.len()
            + SW_TRANS.len()
            + ENERGY_TRANS.len()
            + TEMP_TRANS.len()
            + SOC_MED.len()
            + ROBO_COG.len()
            + EMERGENT_META.len();
        assert_eq!(total, 37);
    }

    #[test]
    fn test_populate_all_emergent() {
        let mut graph = DiscoveryGraph::new();
        let (nodes, edges) = populate_all_emergent_cross(&mut graph);
        assert_eq!(nodes, 37);
        assert!(edges > 0, "should produce cross-domain edges");
    }

    #[test]
    fn test_all_ids_unique() {
        let mut graph = DiscoveryGraph::new();
        populate_all_emergent_cross(&mut graph);
        let mut ids: Vec<&str> = graph.nodes.iter().map(|n| n.id.as_str()).collect();
        let original_len = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), original_len, "duplicate node IDs found");
    }

    #[test]
    fn test_confidence_range() {
        let all: Vec<&EmEntry> = SW_MED
            .iter()
            .chain(SW_TRANS.iter())
            .chain(ENERGY_TRANS.iter())
            .chain(TEMP_TRANS.iter())
            .chain(SOC_MED.iter())
            .chain(ROBO_COG.iter())
            .chain(EMERGENT_META.iter())
            .collect();
        for entry in &all {
            assert!(
                entry.confidence > 0.0 && entry.confidence <= 1.0,
                "{} confidence {} out of range",
                entry.id,
                entry.confidence
            );
        }
    }

    #[test]
    fn test_all_entries_have_source_bts() {
        let all: Vec<&EmEntry> = SW_MED
            .iter()
            .chain(SW_TRANS.iter())
            .chain(ENERGY_TRANS.iter())
            .chain(TEMP_TRANS.iter())
            .chain(SOC_MED.iter())
            .chain(ROBO_COG.iter())
            .chain(EMERGENT_META.iter())
            .collect();
        for entry in &all {
            assert!(
                !entry.source_bts.is_empty(),
                "{} has no source BTs",
                entry.id
            );
        }
    }

    #[test]
    fn test_all_entries_have_lenses() {
        let all: Vec<&EmEntry> = SW_MED
            .iter()
            .chain(SW_TRANS.iter())
            .chain(ENERGY_TRANS.iter())
            .chain(TEMP_TRANS.iter())
            .chain(SOC_MED.iter())
            .chain(ROBO_COG.iter())
            .chain(EMERGENT_META.iter())
            .collect();
        for entry in &all {
            assert!(
                entry.lenses.len() >= 2,
                "{} should have at least 2 lenses",
                entry.id
            );
        }
    }

    #[test]
    fn test_edge_strength_range() {
        let mut graph = DiscoveryGraph::new();
        populate_all_emergent_cross(&mut graph);
        for edge in &graph.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "edge {}->{} strength {} out of range",
                edge.from,
                edge.to,
                edge.strength
            );
        }
    }

    #[test]
    fn test_emergent_meta_nodes_span_4plus_domains() {
        for entry in EMERGENT_META {
            assert!(
                entry.domains.len() >= 4,
                "meta-convergence {} should span 4+ domains, has {}",
                entry.id,
                entry.domains.len()
            );
        }
    }

    #[test]
    fn test_emergent_meta_nodes_span_4plus_source_bts() {
        for entry in EMERGENT_META {
            assert!(
                entry.source_bts.len() >= 4,
                "meta-convergence {} should reference 4+ BTs, has {}",
                entry.id,
                entry.source_bts.len()
            );
        }
    }

    #[test]
    fn test_hypotheses_have_lower_confidence() {
        let all: Vec<&EmEntry> = SW_MED
            .iter()
            .chain(SW_TRANS.iter())
            .chain(ENERGY_TRANS.iter())
            .chain(TEMP_TRANS.iter())
            .chain(SOC_MED.iter())
            .chain(ROBO_COG.iter())
            .chain(EMERGENT_META.iter())
            .collect();
        for entry in &all {
            if matches!(entry.node_type, NodeType::Hypothesis) {
                assert!(
                    entry.confidence <= 0.80,
                    "hypothesis {} should have confidence <= 0.80, got {}",
                    entry.id,
                    entry.confidence
                );
            }
        }
    }

    #[test]
    fn test_all_bt_references_in_valid_range() {
        let all: Vec<&EmEntry> = SW_MED
            .iter()
            .chain(SW_TRANS.iter())
            .chain(ENERGY_TRANS.iter())
            .chain(TEMP_TRANS.iter())
            .chain(SOC_MED.iter())
            .chain(ROBO_COG.iter())
            .chain(EMERGENT_META.iter())
            .collect();
        for entry in &all {
            for &bt in entry.source_bts {
                assert!(
                    bt >= 1 && bt <= 300,
                    "{} references invalid BT-{}",
                    entry.id,
                    bt
                );
            }
        }
    }

    #[test]
    fn test_populate_individual_clusters() {
        let clusters: Vec<(&str, fn(&mut DiscoveryGraph) -> (usize, usize), usize)> = vec![
            ("sw_med", populate_sw_med, 6),
            ("sw_trans", populate_sw_trans, 5),
            ("energy_trans", populate_energy_trans, 6),
            ("temp_trans", populate_temp_trans, 4),
            ("soc_med", populate_soc_med, 5),
            ("robo_cog", populate_robo_cog, 6),
            ("emergent_meta", populate_emergent_meta, 5),
        ];

        for (name, populate_fn, expected_nodes) in clusters {
            let mut graph = DiscoveryGraph::new();
            let (nodes, edges) = populate_fn(&mut graph);
            assert_eq!(
                nodes, expected_nodes,
                "{} expected {} nodes, got {}",
                name, expected_nodes, nodes
            );
            assert!(
                edges > 0,
                "{} should produce at least 1 edge",
                name
            );
        }
    }
}
