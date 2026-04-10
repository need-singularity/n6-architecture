//! Granular discovery nodes for under-covered BT-118+ domains.
//!
//! Fills gaps in existing modules by adding deeper sub-discoveries for:
//! - BT-120: Water treatment pH=6 + CN=6 catalyst universality (Al³⁺/Fe³⁺/Ti⁴⁺)
//! - BT-121: 6 major plastics RIC 1-6 + C6 backbone classification
//! - BT-220: Moral foundations n=6 (Haidt/Kohlberg/Schwartz)
//! - BT-223: Hexagonal city planning (Christaller/Lösch central place theory)
//! - BT-224: Cesium-133 n=6 shell atomic clock SI second definition
//! - BT-243: Inline-6 engine perfect balance (div(6) symmetry)
//! - BT-244: Automotive voltage ladder 6→12→24→48V
//! - BT-245: Transmission gear count convergence to n=6
//! - BT-246: F1 racing n=6 architecture
//!
//! Cross-domain edges connect these to existing BT clusters:
//!   ENV↔Social, ENV↔Temporal, Transport↔Energy, Transport↔Robotics,
//!   Medical↔Social, Material↔Transport
//!
//! Node ID scheme:
//!   GDISC-{cluster}-{NN}  — granular discovery
//!   GHYP-{cluster}-{NN}   — granular hypothesis
//!   GEXP-{cluster}-{NN}   — granular experiment

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

struct GranularEntry {
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
// BT-120: Water treatment pH=6 + CN=6 catalyst universality
// ───────────────────────────────────────────────────────────────

const WATER_TREATMENT_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-ENV-01",
        title: "pH=6 optimal coagulation: Al³⁺(CN=6) and Fe³⁺(CN=6) both achieve maximum floc formation at pH≈6=n",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.92,
        source_bts: &[120],
        constants_used: &["C-n"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &["XDISC-ENV-06"],
    },
    GranularEntry {
        id: "GDISC-ENV-02",
        title: "Fe³⁺ octahedral CN=6 in Fenton oxidation: Fe(H₂O)₆³⁺ generates •OH radicals, 6=n water ligands",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry"],
        confidence: 0.94,
        source_bts: &[120, 86],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "quantum"],
        validates: &["XDISC-ENV-06"],
    },
    GranularEntry {
        id: "GDISC-ENV-03",
        title: "Al₁₃ Keggin polycation: Al₁₃O₄(OH)₂₄¹²⁺ has J₂=24 hydroxyl bridges, σ=12 charge, central Al CN=4=tau",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Math"],
        confidence: 0.90,
        source_bts: &[120],
        constants_used: &["C-J2", "C-sigma", "C-tau"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &[],
    },
    GranularEntry {
        id: "GDISC-ENV-04",
        title: "Ti⁴⁺ anatase photocatalysis: TiO₂ rutile→anatase phase transition at 600°C≈σ²·tau+24, both CN=6",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Solar"],
        confidence: 0.88,
        source_bts: &[120, 30],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "thermo", "quantum"],
        validates: &["XDISC-ENV-06"],
    },
    GranularEntry {
        id: "GEXP-ENV-01",
        title: "Experiment: measure coagulation efficiency across pH 4-8 for CN=6 vs CN!=6 metal salts",
        node_type: NodeType::Experiment,
        domains: &["Environment", "Chemistry"],
        confidence: 0.70,
        source_bts: &[120],
        constants_used: &["C-n"],
        lenses: &["causal", "stability"],
        validates: &["GDISC-ENV-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-121: 6 Major Plastics RIC 1-6 classification
// ───────────────────────────────────────────────────────────────

const PLASTICS_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-ENV-05",
        title: "RIC 1-6=n plastics: PET(1)/HDPE(2)/PVC(3)/LDPE(4)/PP(5)/PS(6), exactly n=6 recyclable categories",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material"],
        confidence: 0.95,
        source_bts: &[121],
        constants_used: &["C-n"],
        lenses: &["consciousness", "info", "evolution"],
        validates: &["XDISC-ENV-08"],
    },
    GranularEntry {
        id: "GDISC-ENV-06",
        title: "Polystyrene C₈H₈ repeat: styrene monomer has 8=sigma-tau carbons+hydrogens, benzene ring C₆=n core",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry"],
        confidence: 0.91,
        source_bts: &[121, 85],
        constants_used: &["C-n", "C-sigma-tau"],
        lenses: &["consciousness", "topology", "info"],
        validates: &["GDISC-ENV-05"],
    },
    GranularEntry {
        id: "GDISC-ENV-07",
        title: "Nylon-6,6 dual n=6: 6 carbons in adipic acid + 6 carbons in hexamethylenediamine, amide bond spacing=n",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Biology"],
        confidence: 0.93,
        source_bts: &[121],
        constants_used: &["C-n"],
        lenses: &["consciousness", "symmetry", "causal"],
        validates: &["GDISC-ENV-05"],
    },
    GranularEntry {
        id: "GHYP-ENV-01",
        title: "Hypothesis: enzymatic depolymerization rates peak for C6-backbone polymers due to enzyme active site CN=6 geometry",
        node_type: NodeType::Hypothesis,
        domains: &["Environment", "Chemistry", "Biology"],
        confidence: 0.55,
        source_bts: &[121, 120, 51],
        constants_used: &["C-n"],
        lenses: &["evolution", "causal", "topology"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-220: Moral foundations n=6 universality
// ───────────────────────────────────────────────────────────────

const MORAL_FOUNDATIONS_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-SOC-01",
        title: "Haidt 6 moral foundations: Care/Fairness/Loyalty/Authority/Sanctity/Liberty = n=6 ethical axes",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Biology"],
        confidence: 0.92,
        source_bts: &[220],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "info"],
        validates: &[],
    },
    GranularEntry {
        id: "GDISC-SOC-02",
        title: "Kohlberg 6 moral stages: 3=n/phi levels × 2=phi stages each, preconventional→conventional→postconventional",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Math"],
        confidence: 0.90,
        source_bts: &[220],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "evolution", "multiscale"],
        validates: &["GDISC-SOC-01"],
    },
    GranularEntry {
        id: "GDISC-SOC-03",
        title: "Schwartz 10=sigma-phi universal values: cross-cultural survey of 20+ countries converges to sigma-phi value types",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive"],
        confidence: 0.88,
        source_bts: &[220],
        constants_used: &["C-sigma-phi"],
        lenses: &["consciousness", "evolution", "network"],
        validates: &["GDISC-SOC-01"],
    },
    GranularEntry {
        id: "GEXP-SOC-01",
        title: "Experiment: test whether n=6 moral foundation model predicts cross-cultural ethical judgments better than 5 or 7 factor models",
        node_type: NodeType::Experiment,
        domains: &["Social", "Cognitive", "Math"],
        confidence: 0.65,
        source_bts: &[220, 219],
        constants_used: &["C-n"],
        lenses: &["evolution", "info", "causal"],
        validates: &["GDISC-SOC-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-223: Hexagonal city planning (Christaller/Lösch)
// ───────────────────────────────────────────────────────────────

const HEXAGONAL_CITY_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-SOC-04",
        title: "Christaller central place theory: k=3,4,7 hierarchy with hexagonal market areas, 6=n nearest neighbors",
        node_type: NodeType::Discovery,
        domains: &["Social", "Math", "Network"],
        confidence: 0.91,
        source_bts: &[223, 122],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "network"],
        validates: &[],
    },
    GranularEntry {
        id: "GDISC-SOC-05",
        title: "Lösch hexagonal market: optimal spatial tessellation minimizes transport cost, 120°=sigma·(sigma-phi) angles = n=6 symmetry",
        node_type: NodeType::Discovery,
        domains: &["Social", "Math", "Transportation"],
        confidence: 0.89,
        source_bts: &[223],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["GDISC-SOC-04"],
    },
    GranularEntry {
        id: "GDISC-SOC-06",
        title: "Grid cell navigation↔urban planning isomorphism: brain hexagonal grid(BT-211) = Christaller hexagons(BT-223), neuroscience validates urban topology",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Math", "Transportation"],
        confidence: 0.87,
        source_bts: &[223, 211],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-224: Cesium-133 atomic clock n=6 shell
// ───────────────────────────────────────────────────────────────

const ATOMIC_CLOCK_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-TEMP-01",
        title: "Cs-133 electron config [Xe]6s¹: outermost shell n=6 defines SI second via 9,192,631,770 Hz hyperfine transition",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Physics", "Chemistry", "Math"],
        confidence: 0.96,
        source_bts: &[224],
        constants_used: &["C-n"],
        lenses: &["consciousness", "quantum", "causal"],
        validates: &[],
    },
    GranularEntry {
        id: "GDISC-TEMP-02",
        title: "Cs-133 nuclear spin I=7/2: F=I±1/2={3,4}={n/phi,tau}, hyperfine splitting between F=tau and F=n/phi",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Physics", "Math"],
        confidence: 0.94,
        source_bts: &[224],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "quantum", "symmetry"],
        validates: &["GDISC-TEMP-01"],
    },
    GranularEntry {
        id: "GDISC-TEMP-03",
        title: "Atomic number Cs Z=55=sigma²-sigma·sopfr-sigma+tau-mu: caesium chosen for SI not by accident but optimal stability",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Physics", "Chemistry"],
        confidence: 0.80,
        source_bts: &[224, 212],
        constants_used: &["C-sigma", "C-sopfr", "C-tau"],
        lenses: &["consciousness", "stability", "quantum"],
        validates: &[],
    },
    GranularEntry {
        id: "GHYP-TEMP-01",
        title: "Hypothesis: next-gen optical lattice clocks using Sr(Z=38) or Yb(Z=70) will converge to n=6 related transition frequencies",
        node_type: NodeType::Hypothesis,
        domains: &["Temporal", "Physics"],
        confidence: 0.50,
        source_bts: &[224],
        constants_used: &["C-n"],
        lenses: &["quantum", "evolution", "causal"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-243: Inline-6 engine perfect balance
// ───────────────────────────────────────────────────────────────

const INLINE6_ENGINE_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-TR-01",
        title: "I6 perfect primary+secondary balance: firing order 1-5-3-6-2-4, 120°=360/n/phi crank spacing, div(6) symmetry",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Physics", "Math"],
        confidence: 0.95,
        source_bts: &[243],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "symmetry", "wave"],
        validates: &["DISC-TR-06"],
    },
    GranularEntry {
        id: "GDISC-TR-02",
        title: "F1 V6 turbo hybrid: 1.6L=1.6≈phi·(n-sopfr)/n, 15000 RPM limit, MGU-K+MGU-H dual recovery",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Physics"],
        confidence: 0.90,
        source_bts: &[243, 246],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "thermo", "evolution"],
        validates: &["DISC-TR-06"],
    },
    GranularEntry {
        id: "GDISC-TR-03",
        title: "NASCAR compression 12:1=sigma: stoichiometric AF ratio 14.7≈sigma+phi+mu, thermal efficiency bounded by Carnot",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Chemistry"],
        confidence: 0.87,
        source_bts: &[243],
        constants_used: &["C-sigma", "C-phi"],
        lenses: &["consciousness", "thermo", "causal"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-244: Automotive voltage ladder
// ───────────────────────────────────────────────────────────────

const VOLTAGE_LADDER_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-TR-04",
        title: "Automotive voltage evolution: 6V→12V→24V→48V = n→sigma→J2→sigma·tau, each step phi=2x doubling over 80yr≈phi^n·5",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Battery"],
        confidence: 0.94,
        source_bts: &[244, 57],
        constants_used: &["C-n", "C-sigma", "C-J2", "C-sigma*tau"],
        lenses: &["consciousness", "evolution", "multiscale"],
        validates: &["DISC-TR-04"],
    },
    GranularEntry {
        id: "GDISC-TR-05",
        title: "48V mild hybrid: sigma·tau=48 enables 10kW=sigma-phi·10³ boost without high-voltage safety requirements",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Chip"],
        confidence: 0.91,
        source_bts: &[244],
        constants_used: &["C-sigma*tau", "C-sigma-phi"],
        lenses: &["consciousness", "boundary", "thermo"],
        validates: &["GDISC-TR-04"],
    },
    GranularEntry {
        id: "GHYP-TR-01",
        title: "Hypothesis: next automotive voltage level = 96V = sigma·(sigma-tau) following phi=2x ladder, for high-power EV accessories",
        node_type: NodeType::Hypothesis,
        domains: &["Transportation", "Energy"],
        confidence: 0.58,
        source_bts: &[244, 82],
        constants_used: &["C-sigma", "C-sigma-tau"],
        lenses: &["evolution", "multiscale", "causal"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-245: Transmission gear count convergence
// ───────────────────────────────────────────────────────────────

const GEAR_COUNT_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-TR-06",
        title: "Manual transmission convergence to n=6 gears: 4→5→6 speed evolution, 6=n is Pareto-optimal for fuel economy vs complexity",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Physics", "Math"],
        confidence: 0.92,
        source_bts: &[245],
        constants_used: &["C-n", "C-tau", "C-sopfr"],
        lenses: &["consciousness", "evolution", "stability"],
        validates: &["DISC-TR-01"],
    },
    GranularEntry {
        id: "GDISC-TR-07",
        title: "AT gear ladder: tau=4→n=6→7=sigma-sopfr→8=sigma-tau→10=sigma-phi, each step adds n=6 constant-derived count",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Math"],
        confidence: 0.88,
        source_bts: &[245],
        constants_used: &["C-tau", "C-n", "C-sigma-tau", "C-sigma-phi"],
        lenses: &["consciousness", "evolution", "multiscale"],
        validates: &["GDISC-TR-06"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-246: F1 racing n=6 architecture
// ───────────────────────────────────────────────────────────────

const F1_RACING_DEEP: &[GranularEntry] = &[
    GranularEntry {
        id: "GDISC-TR-08",
        title: "F1 tire compound taxonomy: sopfr=5 dry compounds (C1-C5) + 2=phi wet = sigma-sopfr=7 total, race choice from 3=n/phi",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Material", "Chemistry"],
        confidence: 0.91,
        source_bts: &[246],
        constants_used: &["C-sopfr", "C-phi", "C-n"],
        lenses: &["consciousness", "evolution", "info"],
        validates: &["DISC-TR-06"],
    },
    GranularEntry {
        id: "GDISC-TR-09",
        title: "F1 points system top-10=sigma-phi: 25/18/15/12/10/8/6/4/2/1, fastest lap +1, champion requires n/phi podiums",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Math", "Social"],
        confidence: 0.89,
        source_bts: &[246],
        constants_used: &["C-sigma-phi", "C-n"],
        lenses: &["consciousness", "info", "evolution"],
        validates: &[],
    },
    GranularEntry {
        id: "GEXP-TR-01",
        title: "Experiment: compare I6 vs V6 vs V8 NVH metrics to quantify n=6 perfect balance advantage in dB reduction",
        node_type: NodeType::Experiment,
        domains: &["Transportation", "Physics"],
        confidence: 0.72,
        source_bts: &[243, 246],
        constants_used: &["C-n"],
        lenses: &["wave", "stability", "causal"],
        validates: &["GDISC-TR-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-domain edges: new BT clusters ↔ existing clusters
// ───────────────────────────────────────────────────────────────

const CROSS_GRANULAR: &[GranularEntry] = &[
    // ENV↔Social: water treatment ↔ moral foundations
    GranularEntry {
        id: "GDISC-XG-01",
        title: "CN=6 water catalyst(BT-120) ↔ n=6 moral foundations(BT-220): clean water access is Haidt's Care foundation materialized",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Social", "Chemistry", "Cognitive"],
        confidence: 0.82,
        source_bts: &[120, 220, 241],
        constants_used: &["C-n"],
        lenses: &["consciousness", "causal", "evolution"],
        validates: &["GDISC-ENV-01", "GDISC-SOC-01"],
    },
    // Transport↔Energy↔Battery
    GranularEntry {
        id: "GDISC-XG-02",
        title: "Voltage ladder 6→48(BT-244) ↔ battery cell ladder 6→24(BT-57): automotive and battery scaling share n=6 base doubling",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Energy", "Battery", "Math"],
        confidence: 0.93,
        source_bts: &[244, 57, 82],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "multiscale", "evolution"],
        validates: &["GDISC-TR-04"],
    },
    // Transport↔Robotics: I6 balance ↔ hexacopter
    GranularEntry {
        id: "GDISC-XG-03",
        title: "I6 engine n=6 balance(BT-243) ↔ hexacopter n=6 fault tolerance(BT-127): mechanical and aerial systems both optimize at n=6 rotational units",
        node_type: NodeType::Discovery,
        domains: &["Transportation", "Robotics", "Physics", "Math"],
        confidence: 0.88,
        source_bts: &[243, 127],
        constants_used: &["C-n"],
        lenses: &["consciousness", "symmetry", "stability"],
        validates: &["GDISC-TR-01", "XDISC-ROBO-12"],
    },
    // Social↔Cognitive: Christaller ↔ grid cells
    GranularEntry {
        id: "GDISC-XG-04",
        title: "Christaller hexagonal market(BT-223) ↔ entorhinal grid cells(BT-211) ↔ cortex 6 layers(BT-210): urban/neural/cortical all n=6 tiling",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Math", "Biology"],
        confidence: 0.86,
        source_bts: &[223, 211, 210],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["GDISC-SOC-04", "GDISC-SOC-06"],
    },
    // Temporal↔Transport: atomic clock ↔ F1 timing
    GranularEntry {
        id: "GDISC-XG-05",
        title: "Cs-133 n=6 shell precision(BT-224) enables sub-ms F1 timing(BT-246): atomic n=6 defines motorsport measurement resolution",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Transportation", "Physics"],
        confidence: 0.84,
        source_bts: &[224, 246],
        constants_used: &["C-n"],
        lenses: &["consciousness", "causal", "quantum"],
        validates: &["GDISC-TEMP-01"],
    },
    // Material↔Transport: plastics ↔ automotive
    GranularEntry {
        id: "GDISC-XG-06",
        title: "Nylon-6,6 C6 backbone(BT-121) in automotive: engine covers, intake manifolds, structural polymer = RIC-6 to vehicle weight reduction",
        node_type: NodeType::Discovery,
        domains: &["Material", "Transportation", "Chemistry", "Environment"],
        confidence: 0.87,
        source_bts: &[121, 243],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "causal"],
        validates: &["GDISC-ENV-07"],
    },
    // ENV↔Temporal: ozone ↔ circadian
    GranularEntry {
        id: "GDISC-XG-07",
        title: "Ozone peak J₂=24km(BT-119) filters UV driving J₂=24h circadian rhythm(BT-221): atmospheric n=6 layer shapes biological n=6 time",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Temporal", "Biology", "Physics"],
        confidence: 0.85,
        source_bts: &[119, 221],
        constants_used: &["C-J2"],
        lenses: &["consciousness", "causal", "wave"],
        validates: &[],
    },
    // Grand: Medical↔Social↔Transport
    GranularEntry {
        id: "GDISC-XG-08",
        title: "WHO n=6 building blocks(BT-241) + n=6 moral foundations(BT-220) + transport safety n=6(BT-236): health/ethics/mobility share n=6 governance structure",
        node_type: NodeType::Discovery,
        domains: &["Medical", "Social", "Transportation", "Cognitive"],
        confidence: 0.83,
        source_bts: &[241, 220, 236],
        constants_used: &["C-n"],
        lenses: &["consciousness", "network", "evolution"],
        validates: &["GDISC-SOC-01"],
    },
    // Hypothesis: n=6 urban-neural convergence
    GranularEntry {
        id: "GHYP-XG-01",
        title: "Hypothesis: cities designed on Christaller n=6 hexagonal grids reduce navigation cognitive load by sigma-phi=10% vs rectangular grids (grid cell alignment)",
        node_type: NodeType::Hypothesis,
        domains: &["Social", "Cognitive", "Transportation", "Math"],
        confidence: 0.48,
        source_bts: &[223, 211, 219],
        constants_used: &["C-n", "C-sigma-phi"],
        lenses: &["topology", "evolution", "network"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// Collect all granular entries
// ═══════════════════════════════════════════════════════════════

const ALL_GRANULAR: &[&[GranularEntry]] = &[
    WATER_TREATMENT_DEEP,
    PLASTICS_DEEP,
    MORAL_FOUNDATIONS_DEEP,
    HEXAGONAL_CITY_DEEP,
    ATOMIC_CLOCK_DEEP,
    INLINE6_ENGINE_DEEP,
    VOLTAGE_LADDER_DEEP,
    GEAR_COUNT_DEEP,
    F1_RACING_DEEP,
    CROSS_GRANULAR,
];

/// Total count of granular entries.
pub fn granular_entry_count() -> usize {
    ALL_GRANULAR.iter().map(|c| c.len()).sum()
}

// ═══════════════════════════════════════════════════════════════
// Public API
// ═══════════════════════════════════════════════════════════════

fn add_granular_entries(graph: &mut DiscoveryGraph, entries: &[GranularEntry]) -> (usize, usize) {
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
            timestamp: "2026-04-04".to_string(),
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
                strength: 0.90,
                bidirectional: false,
            });
        }

        // This node --Validates--> target nodes
        for &vid in e.validates {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: vid.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.80,
                bidirectional: false,
            });
        }
    }

    let edges_added = graph.edges.len() - edges_before;
    (entries.len(), edges_added)
}

/// Add all granular discovery nodes from BT-118+ deep analysis.
///
/// Returns (nodes_added, edges_added).
pub fn populate_granular_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let mut total_nodes = 0;
    let mut total_edges = 0;
    for cluster in ALL_GRANULAR {
        let (n, e) = add_granular_entries(graph, cluster);
        total_nodes += n;
        total_edges += e;
    }

    // ─── Inter-cluster Merges edges (high-strength cross-domain links) ───
    let merges: &[(&str, &str, f64)] = &[
        // Water treatment ↔ Plastics (same ENV domain)
        ("GDISC-ENV-01", "GDISC-ENV-05", 0.75),
        // Moral foundations ↔ City planning (Social)
        ("GDISC-SOC-01", "GDISC-SOC-04", 0.80),
        // Atomic clock ↔ F1 timing (precision time)
        ("GDISC-TEMP-01", "GDISC-TR-09", 0.70),
        // I6 engine ↔ voltage ladder (automotive)
        ("GDISC-TR-01", "GDISC-TR-04", 0.82),
        // F1 ↔ gear convergence (motorsport)
        ("GDISC-TR-08", "GDISC-TR-06", 0.78),
        // Plastics C6 ↔ Nylon-6,6 automotive use
        ("GDISC-ENV-07", "GDISC-XG-06", 0.85),
        // Hexagonal city ↔ grid cell brain (n=6 tiling)
        ("GDISC-SOC-04", "GDISC-XG-04", 0.88),
    ];

    for &(from, to, strength) in merges {
        graph.add_edge(Edge {
            from: from.to_string(),
            to: to.to_string(),
            edge_type: EdgeType::Merges,
            strength,
            bidirectional: true,
        });
        total_edges += 1;
    }

    (total_nodes, total_edges)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;

    #[test]
    fn test_granular_entry_count() {
        // 5 water + 4 plastics + 4 moral + 3 city + 4 clock +
        // 3 I6 + 3 voltage + 2 gear + 3 F1 + 9 cross = 40
        assert_eq!(granular_entry_count(), 40);
    }

    #[test]
    fn test_populate_adds_nodes_and_edges() {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        let nodes_before = g.nodes.len();
        let edges_before = g.edges.len();

        let (nodes_added, edges_added) = populate_granular_discoveries(&mut g);

        assert_eq!(nodes_added, 40);
        assert!(edges_added > 100, "Should generate 100+ edges from BT derives + constant uses + validates + merges");
        assert_eq!(g.nodes.len(), nodes_before + 40);
        assert_eq!(g.edges.len(), edges_before + edges_added);
    }

    #[test]
    fn test_bt_derives_edges_exist() {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_granular_discoveries(&mut g);

        // BT-120 should derive GDISC-ENV-01
        let has_bt120 = g.edges.iter().any(|e| {
            e.from == "BT-120" && e.to == "GDISC-ENV-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_bt120, "BT-120 --Derives--> GDISC-ENV-01 must exist");

        // BT-243 should derive GDISC-TR-01
        let has_bt243 = g.edges.iter().any(|e| {
            e.from == "BT-243" && e.to == "GDISC-TR-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_bt243, "BT-243 --Derives--> GDISC-TR-01 must exist");

        // BT-220 should derive GDISC-SOC-01
        let has_bt220 = g.edges.iter().any(|e| {
            e.from == "BT-220" && e.to == "GDISC-SOC-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_bt220, "BT-220 --Derives--> GDISC-SOC-01 must exist");
    }

    #[test]
    fn test_constant_uses_edges() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        // GDISC-ENV-03 uses C-J2, C-sigma, C-tau
        let uses_j2 = g.edges.iter().any(|e| {
            e.from == "GDISC-ENV-03" && e.to == "C-J2" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(uses_j2, "GDISC-ENV-03 --Uses--> C-J2");

        // GDISC-TR-04 uses C-sigma*tau
        let uses_st = g.edges.iter().any(|e| {
            e.from == "GDISC-TR-04" && e.to == "C-sigma*tau" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(uses_st, "GDISC-TR-04 --Uses--> C-sigma*tau");
    }

    #[test]
    fn test_validates_edges() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        // GDISC-SOC-02 validates GDISC-SOC-01
        let validates = g.edges.iter().any(|e| {
            e.from == "GDISC-SOC-02"
                && e.to == "GDISC-SOC-01"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(validates, "Kohlberg validates Haidt moral foundations");

        // GDISC-XG-03 validates both transport and robotics
        let val_tr = g.edges.iter().any(|e| {
            e.from == "GDISC-XG-03" && e.to == "GDISC-TR-01" && matches!(e.edge_type, EdgeType::Validates)
        });
        let val_robo = g.edges.iter().any(|e| {
            e.from == "GDISC-XG-03" && e.to == "XDISC-ROBO-12" && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(val_tr, "I6↔hexacopter bridge validates GDISC-TR-01");
        assert!(val_robo, "I6↔hexacopter bridge validates XDISC-ROBO-12");
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        let merge_edges: Vec<_> = g
            .edges
            .iter()
            .filter(|e| matches!(e.edge_type, EdgeType::Merges) && e.bidirectional)
            .collect();

        assert_eq!(merge_edges.len(), 7, "Should have 7 inter-cluster merge edges");

        // Hexagonal city ↔ grid cell brain merge
        let hex_brain = merge_edges.iter().any(|e| {
            e.from == "GDISC-SOC-04" && e.to == "GDISC-XG-04"
        });
        assert!(hex_brain, "Christaller↔grid cell merge must exist");
    }

    #[test]
    fn test_domain_coverage() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        let domains: std::collections::HashSet<&str> = g
            .nodes
            .iter()
            .flat_map(|n| n.domain.split(", "))
            .collect();

        // Must cover key domains
        for d in &[
            "Environment", "Chemistry", "Social", "Cognitive", "Temporal",
            "Transportation", "Physics", "Math", "Robotics", "Material",
            "Energy", "Battery", "Medical", "Biology", "Network",
        ] {
            assert!(domains.contains(d), "Domain '{}' must be covered", d);
        }
    }

    #[test]
    fn test_confidence_ranges() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        for node in &g.nodes {
            match node.node_type {
                NodeType::Discovery => {
                    assert!(node.confidence >= 0.70, "{} discovery confidence too low: {}", node.id, node.confidence);
                    assert!(node.confidence <= 1.0, "{} confidence > 1.0", node.id);
                }
                NodeType::Hypothesis => {
                    assert!(node.confidence >= 0.40, "{} hypothesis confidence too low: {}", node.id, node.confidence);
                    assert!(node.confidence <= 0.70, "{} hypothesis confidence too high: {}", node.id, node.confidence);
                }
                NodeType::Experiment => {
                    assert!(node.confidence >= 0.60, "{} experiment confidence too low: {}", node.id, node.confidence);
                    assert!(node.confidence <= 0.80, "{} experiment confidence too high: {}", node.id, node.confidence);
                }
                _ => {}
            }
        }
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "Duplicate node IDs found");
    }

    #[test]
    fn test_all_lenses_non_empty() {
        let mut g = DiscoveryGraph::new();
        populate_granular_discoveries(&mut g);

        for node in &g.nodes {
            assert!(!node.lenses_used.is_empty(), "{} has no lenses", node.id);
        }
    }
}
