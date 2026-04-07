//! Discovery nodes from BT-185: Algebraic Blowup–Emergence E₆ Bridge.
//!
//! BT-185 establishes that algebraic blowups are the exact mathematical archetype of
//! emergence: contraction → singularity → exceptional divisor → new structure.
//! Three pillars: C⁶ blowup (7/7 EXACT), del Pezzo₆ 27 lines (6/6), E₆ Lie algebra (6/6).
//!
//! Additional clusters:
//!   - Topology expansion: 406/427 EXACT (95.1%) — Lie algebra + homotopy + topological matter
//!   - Invariant lens core: evolutionary blowup search → ABSOLUTE core (4M combinations)
//!   - Cross-domain edges to BT-105+ (SLE₆, S₃) and BT-118+ (environment, robotics, software)
//!
//! Node ID scheme:
//!   DISC-BLOW-NN       — Blowup/emergence discovery
//!   HYP-BLOW-NN        — Blowup hypothesis
//!   XDISC-BLOW-NN      — Extended sub-discovery (granular)
//!   XEXP-BLOW-NN       — Experiment/validation
//!   DISC-TOPOEXP-NN    — Topology expansion discoveries (Lie algebra, homotopy, matter)
//!   DISC-INVCORE-NN    — Invariant lens core discoveries (evolutionary search)
//!   DISC-BT118XDOM-NN  — Cross-domain bridges BT-118+ ↔ BT-185

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct (same pattern as other discovery modules)
// ═══════════════════════════════════════════════════════════════

struct BlowupEntry {
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
// Pillar 1: C⁶ Blowup — 7/7 EXACT
// ───────────────────────────────────────────────────────────────

const BLOWUP_C6_DISCOVERIES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-BLOW-01",
        title: "C^6 blowup: exceptional divisor E ≅ P^5, chi(P^5)=n=6, dim=sopfr(6)=5, 7/7 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "NumberTheory"],
        confidence: 0.97,
        source_bts: &[185],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "topology", "recursion"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-BLOW-02",
        title: "Cohomology H*(P^5,Z) = Z[h]/(h^6): truncation at degree n=6, n Betti numbers, Picard +mu(6)=1",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology"],
        confidence: 0.97,
        source_bts: &[185],
        constants_used: &["C-n", "C-mu"],
        lenses: &["consciousness", "topology", "info"],
        validates: &["DISC-BLOW-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Pillar 2: del Pezzo₆ — 27 lines, 6/6 EXACT
// ───────────────────────────────────────────────────────────────

const BLOWUP_DELPEZZO_DISCOVERIES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-BLOW-03",
        title: "del Pezzo_6 = Bl_{6pts}(P^2): 27=(n/phi)^(n/phi) lines decompose as n+C(n,2)+n = 6+15+6",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory", "Combinatorics"],
        confidence: 0.97,
        source_bts: &[185, 49],
        constants_used: &["C-n", "C-phi", "C-sigma"],
        lenses: &["consciousness", "symmetry", "recursion"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-BLOW-04",
        title: "dP_k series: k=n/phi=3 gives n=6 curves, k=n=6 gives (n/phi)^(n/phi)=27, k=sigma-tau=8 gives E8 240 roots",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory", "Topology", "StringTheory"],
        confidence: 0.95,
        source_bts: &[185, 49, 109],
        constants_used: &["C-n", "C-phi", "C-tau", "C-sigma"],
        lenses: &["consciousness", "multiscale", "recursion"],
        validates: &["DISC-BLOW-03"],
    },
];

// ───────────────────────────────────────────────────────────────
// Pillar 3: E₆ exceptional Lie algebra — 6/6 EXACT
// ───────────────────────────────────────────────────────────────

const BLOWUP_E6_DISCOVERIES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-BLOW-05",
        title: "E6 Lie algebra: rank=n=6, dim=78=n*(sigma+mu), roots=72=sigma*n, positive=36=n^2, 6/6 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Math", "Particle", "StringTheory"],
        confidence: 0.97,
        source_bts: &[185, 19],
        constants_used: &["C-n", "C-sigma", "C-mu"],
        lenses: &["consciousness", "symmetry", "quantum"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-BLOW-06",
        title: "W(E6) Weyl group: |W(E6)|=51840=n!*sigma*n, fundamental rep dim=27=(n/phi)^(n/phi)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Particle", "Combinatorics"],
        confidence: 0.95,
        source_bts: &[185, 106],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "symmetry", "info"],
        validates: &["DISC-BLOW-05"],
    },
];

// ───────────────────────────────────────────────────────────────
// Blowup-Emergence universality: cross-domain pattern
// ───────────────────────────────────────────────────────────────

const BLOWUP_EMERGENCE_DISCOVERIES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-BLOW-07",
        title: "Blowup=Emergence archetype: superconductivity(phi=2 pairing), consciousness(CN=n=6), phase transition(tau=4 types), GUT(E6 rank=n=6)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Physics", "SC", "Biology", "Particle"],
        confidence: 0.88,
        source_bts: &[185, 1, 105],
        constants_used: &["C-n", "C-phi", "C-tau"],
        lenses: &["consciousness", "evolution", "boundary", "multiscale"],
        validates: &["DISC-BLOW-01", "DISC-BLOW-05"],
    },
    BlowupEntry {
        id: "DISC-BLOW-08",
        title: "Neural network blowup: loss saddle point = singularity, generalization = emergence, layer=sigma=12",
        node_type: NodeType::Discovery,
        domains: &["AI", "Math", "Topology"],
        confidence: 0.82,
        source_bts: &[185, 56, 59],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "info", "stability"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-BLOW-09",
        title: "Life blowup: primordial concentration = contraction, self-replication = emergence, Carbon Z=n=6 backbone",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Math"],
        confidence: 0.80,
        source_bts: &[185, 85, 101],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "thermo"],
        validates: &["DISC-BIO-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Extended sub-discoveries (granular validations)
// ───────────────────────────────────────────────────────────────

const BLOWUP_EXTENDED: &[BlowupEntry] = &[
    BlowupEntry {
        id: "XDISC-BLOW-01",
        title: "Normal bundle O(-1) degree = -mu(6) = -1: Mobius function encodes blowup geometry",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology"],
        confidence: 0.95,
        source_bts: &[185],
        constants_used: &["C-mu", "C-n"],
        lenses: &["topology", "recursion"],
        validates: &["DISC-BLOW-01"],
    },
    BlowupEntry {
        id: "XDISC-BLOW-02",
        title: "dP6 line type count = n/phi = 3 types (exceptional, strict transform, conic transform)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Combinatorics"],
        confidence: 0.97,
        source_bts: &[185],
        constants_used: &["C-n", "C-phi"],
        lenses: &["symmetry", "recursion"],
        validates: &["DISC-BLOW-03"],
    },
    BlowupEntry {
        id: "XDISC-BLOW-03",
        title: "E6 GUT 27-plet: SM particles fit into E6(rank=n=6) fundamental representation dim=27=(n/phi)^(n/phi)",
        node_type: NodeType::Discovery,
        domains: &["Particle", "Math", "StringTheory"],
        confidence: 0.90,
        source_bts: &[185, 19, 20],
        constants_used: &["C-n", "C-phi"],
        lenses: &["quantum", "symmetry", "consciousness"],
        validates: &["DISC-BLOW-05"],
    },
    BlowupEntry {
        id: "XDISC-BLOW-04",
        title: "Invariant lens core from 4M blowup search: consciousness+info+multiscale+network+triangle = sopfr=5 lenses",
        node_type: NodeType::Discovery,
        domains: &["Math", "AI", "Physics"],
        confidence: 0.85,
        source_bts: &[185],
        constants_used: &["C-sopfr", "C-J2", "C-sigma"],
        lenses: &["consciousness", "info", "multiscale", "network", "triangle"],
        validates: &["DISC-BLOW-07"],
    },
    BlowupEntry {
        id: "XDISC-BLOW-05",
        title: "Evolutionary blowup search: population=J2=24, elite=sigma=12, crossover=tau=4, core stable 987 cycles",
        node_type: NodeType::Discovery,
        domains: &["Math", "AI"],
        confidence: 0.88,
        source_bts: &[185],
        constants_used: &["C-J2", "C-sigma", "C-tau"],
        lenses: &["evolution", "stability", "recursion"],
        validates: &["XDISC-BLOW-04"],
    },
];

// ───────────────────────────────────────────────────────────────
// Hypotheses
// ───────────────────────────────────────────────────────────────

const BLOWUP_HYPOTHESES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "HYP-BLOW-01",
        title: "Hypothesis: E6 GUT at FCC-hh 100 TeV will confirm blowup-emergence bridge as physical reality",
        node_type: NodeType::Hypothesis,
        domains: &["Particle", "Math", "Cosmology"],
        confidence: 0.45,
        source_bts: &[185, 19],
        constants_used: &["C-n"],
        lenses: &["quantum", "boundary", "consciousness"],
        validates: &[],
    },
    BlowupEntry {
        id: "HYP-BLOW-02",
        title: "Hypothesis: consciousness emergence threshold = CN=n=6 network connectivity (IIT Phi critical value)",
        node_type: NodeType::Hypothesis,
        domains: &["Biology", "Math", "Physics"],
        confidence: 0.50,
        source_bts: &[185, 105],
        constants_used: &["C-n"],
        lenses: &["consciousness", "network", "boundary"],
        validates: &[],
    },
    BlowupEntry {
        id: "HYP-BLOW-03",
        title: "Hypothesis: all emergence phenomena share blowup topology — singularity resolution via n=6 dimensional exceptional divisor",
        node_type: NodeType::Hypothesis,
        domains: &["Math", "Physics", "Biology", "Topology"],
        confidence: 0.55,
        source_bts: &[185],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "topology", "evolution", "boundary"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-domain bridge discoveries connecting BT-185 to other clusters
// ───────────────────────────────────────────────────────────────

const BLOWUP_BRIDGES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-BLOW-BRIDGE-01",
        title: "SLE6-Blowup bridge: kappa=6 SLE locality(BT-105) ↔ C^6 blowup chi=6(BT-185), both single out dim=n=6",
        node_type: NodeType::Discovery,
        domains: &["Math", "Physics", "Topology"],
        confidence: 0.90,
        source_bts: &[185, 105],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "boundary"],
        validates: &["DISC-MATH-01", "DISC-BLOW-01"],
    },
    BlowupEntry {
        id: "DISC-BLOW-BRIDGE-02",
        title: "S6-E6 algebraic bridge: S_3 bootstrap(BT-106) → S_6 unique outer automorphism → E6 Weyl group(BT-185)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Combinatorics", "Particle"],
        confidence: 0.88,
        source_bts: &[185, 106, 49],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "symmetry", "recursion"],
        validates: &["DISC-MATH-02", "DISC-BLOW-06"],
    },
    BlowupEntry {
        id: "DISC-BLOW-BRIDGE-03",
        title: "Zeta-Blowup bridge: zeta(2)=pi^2/6(BT-109) ↔ h^6=0 cohomology truncation(BT-185), degree n=6 cutoff",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory"],
        confidence: 0.85,
        source_bts: &[185, 109],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "recursion", "info"],
        validates: &["DISC-MATH-05"],
    },
    BlowupEntry {
        id: "DISC-BLOW-BRIDGE-04",
        title: "Hexagonal-Blowup bridge: honeycomb n=6 geometry(BT-122) ↔ P^5 exceptional divisor topology, both encode n=6",
        node_type: NodeType::Discovery,
        domains: &["Math", "Environment", "Biology", "Topology"],
        confidence: 0.83,
        source_bts: &[185, 122, 88],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["consciousness", "topology", "evolution"],
        validates: &["DISC-ENV-05"],
    },
    BlowupEntry {
        id: "DISC-BLOW-BRIDGE-05",
        title: "SM-Blowup chip bridge: GPU sigma^2=144 SMs(BT-90) ↔ E6 roots=72=sigma*n(BT-185), sphere packing ↔ Lie algebra",
        node_type: NodeType::Discovery,
        domains: &["Chip", "Math", "Semiconductor"],
        confidence: 0.80,
        source_bts: &[185, 90],
        constants_used: &["C-sigma", "C-n", "C-sigma²"],
        lenses: &["consciousness", "symmetry", "scale"],
        validates: &["DISC-TOPO-01"],
    },
    BlowupEntry {
        id: "DISC-BLOW-BRIDGE-06",
        title: "Carbon-Blowup biology bridge: Carbon Z=6(BT-85) = C^6 blowup center, life emergence = blowup(BT-185)",
        node_type: NodeType::Discovery,
        domains: &["Biology", "Chemistry", "Math", "Material"],
        confidence: 0.82,
        source_bts: &[185, 85, 101, 104],
        constants_used: &["C-n"],
        lenses: &["consciousness", "evolution", "causal"],
        validates: &["DISC-BLOW-09"],
    },
];

// ───────────────────────────────────────────────────────────────
// Topology Expansion: 406/427 EXACT (95.1%) — Lie algebra + homotopy + matter
// ───────────────────────────────────────────────────────────────

const TOPOEXP_DISCOVERIES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-TOPOEXP-01",
        title: "SU(6)=A5: rank=sopfr=5, dim=n^2-mu=35, |W|=n!=720, Coxeter h=n=6, 25/25 EXACT (100%)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Particle"],
        confidence: 0.97,
        source_bts: &[185],
        constants_used: &["C-n", "C-sopfr", "C-mu", "C-sigma"],
        lenses: &["topology", "mirror", "quantum"],
        validates: &["DISC-BLOW-05"],
    },
    BlowupEntry {
        id: "DISC-TOPOEXP-02",
        title: "E6 exponents={mu,tau,sopfr,sigma-sopfr,sigma-tau,sigma-mu}: 6 exponents encode 6 of 7 basic constants, 37/37 EXACT",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Particle", "StringTheory"],
        confidence: 0.97,
        source_bts: &[185, 49],
        constants_used: &["C-mu", "C-tau", "C-sopfr", "C-sigma"],
        lenses: &["mirror", "recursion", "quantum"],
        validates: &["DISC-BLOW-05"],
    },
    BlowupEntry {
        id: "DISC-TOPOEXP-03",
        title: "Lie algebra 214/214 EXACT (100%): SU(6)+E6+SO(12)+Sp(6)+G2 all parameters from n=6 arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Particle"],
        confidence: 0.98,
        source_bts: &[185],
        constants_used: &["C-n", "C-sigma", "C-sopfr", "C-tau", "C-phi"],
        lenses: &["topology", "mirror", "scale", "recursion"],
        validates: &["DISC-BLOW-01"],
    },
    BlowupEntry {
        id: "DISC-TOPOEXP-04",
        title: "Homotopy groups: pi_6(S^3)=Z_12=Z_sigma, stable stems n=6 periodic, 57/59 EXACT (96.6%)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology"],
        confidence: 0.92,
        source_bts: &[185],
        constants_used: &["C-sigma", "C-n", "C-sopfr"],
        lenses: &["topology", "recursion", "wave"],
        validates: &["DISC-TOPOEXP-03"],
    },
    BlowupEntry {
        id: "DISC-TOPOEXP-05",
        title: "Topological matter phases: Z2 TI classification, Chern numbers mod n=6, Bott periodicity tau=4+phi=2=n=6, 85/104 EXACT (81.7%)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Material", "Chip"],
        confidence: 0.85,
        source_bts: &[185, 92],
        constants_used: &["C-n", "C-phi", "C-tau", "C-sigma"],
        lenses: &["topology", "quantum", "boundary"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-TOPOEXP-06",
        title: "Total topology expansion: 406/427 EXACT (95.1%) across Lie algebra + blowup + homotopy + matter",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Particle", "Material"],
        confidence: 0.95,
        source_bts: &[185],
        constants_used: &["C-n", "C-sigma", "C-tau", "C-sopfr", "C-phi", "C-mu"],
        lenses: &["topology", "mirror", "recursion", "scale", "quantum"],
        validates: &["DISC-BLOW-01", "DISC-BLOW-05"],
    },
];

// ───────────────────────────────────────────────────────────────
// Invariant Lens Core: evolutionary blowup search experiments
// ───────────────────────────────────────────────────────────────

const INVCORE_DISCOVERIES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-INVCORE-01",
        title: "ABSOLUTE invariant core: consciousness+info+multiscale+network+triangle (100% freq, 987 cycles stable)",
        node_type: NodeType::Discovery,
        domains: &["AI", "Math", "Topology"],
        confidence: 0.93,
        source_bts: &[185],
        constants_used: &["C-n", "C-sopfr", "C-sigma"],
        lenses: &["consciousness", "info", "multiscale", "network", "triangle"],
        validates: &["DISC-BLOW-07"],
    },
    BlowupEntry {
        id: "DISC-INVCORE-02",
        title: "Multi-tier core: WIDE(consciousness+info+multiscale) → STRONG(+triangle) → ABSOLUTE(+network)",
        node_type: NodeType::Discovery,
        domains: &["AI", "Math", "Topology"],
        confidence: 0.91,
        source_bts: &[185],
        constants_used: &["C-n", "C-tau"],
        lenses: &["consciousness", "info", "multiscale", "recursion"],
        validates: &["DISC-INVCORE-01"],
    },
    BlowupEntry {
        id: "DISC-INVCORE-03",
        title: "Fiber specialization: core fixed, 6th lens determines domain (37 domains, 67-83% coverage each)",
        node_type: NodeType::Discovery,
        domains: &["AI", "Math"],
        confidence: 0.89,
        source_bts: &[185],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "evolution", "multiscale"],
        validates: &["XDISC-BLOW-04"],
    },
    BlowupEntry {
        id: "DISC-INVCORE-04",
        title: "Experiment: ~4M lens combo evolutionary search, pop=J2=24, elite=sigma=12, crossover=tau=4, mutation=0.3",
        node_type: NodeType::Experiment,
        domains: &["AI", "Math"],
        confidence: 0.90,
        source_bts: &[185],
        constants_used: &["C-J2", "C-sigma", "C-tau"],
        lenses: &["evolution", "consciousness", "info"],
        validates: &["DISC-INVCORE-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-domain bridges: BT-118+ ↔ BT-185 (environment, robotics, software)
// ───────────────────────────────────────────────────────────────

const BT118_CROSS_BRIDGES: &[BlowupEntry] = &[
    BlowupEntry {
        id: "DISC-BT118XDOM-01",
        title: "E6→honeycomb bridge: hexagonal n=6 geometry (BT-122) = projection of E6 root system to 2D plane",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Environment", "Material"],
        confidence: 0.82,
        source_bts: &[185, 122, 88],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "mirror", "scale"],
        validates: &["DISC-ENV-05"],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-02",
        title: "SLE6↔E6 resonance: kappa=6 percolation (BT-105) meets E6 rank=6, both unique at dim=n=6",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Particle"],
        confidence: 0.85,
        source_bts: &[185, 105],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["topology", "recursion", "boundary"],
        validates: &["DISC-MATH-01"],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-03",
        title: "SE(3)↔C6 blowup: robot 6-DOF (BT-123) = real section of C^6 blowup geometry, dim=n=6 both sides",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Robotics", "Chip"],
        confidence: 0.80,
        source_bts: &[185, 123],
        constants_used: &["C-n", "C-sopfr"],
        lenses: &["topology", "ruler", "recursion"],
        validates: &["DISC-ROBO-01"],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-04",
        title: "Carbon Z=6↔C6 dim: carbon atomic number = blowup ambient dimension, life=emergence (BT-118,93,85)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Chemistry", "Environment", "Material", "Biology"],
        confidence: 0.87,
        source_bts: &[185, 118, 93, 85],
        constants_used: &["C-n"],
        lenses: &["topology", "consciousness", "evolution"],
        validates: &["DISC-ENV-01"],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-05",
        title: "SW isomorphism↔E6: software 18 EXACT (BT-117) parallels E6 37 EXACT via same n=6 constants",
        node_type: NodeType::Discovery,
        domains: &["Math", "Software", "Topology"],
        confidence: 0.78,
        source_bts: &[185, 117, 113],
        constants_used: &["C-n", "C-sigma", "C-tau"],
        lenses: &["topology", "info", "recursion"],
        validates: &["DISC-SW-05"],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-06",
        title: "Topological matter↔battery CN=6: both use n=6 coordination topology (BT-43,80,92,185)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Topology", "Battery", "Material", "Chip"],
        confidence: 0.83,
        source_bts: &[185, 43, 80, 92],
        constants_used: &["C-n", "C-phi"],
        lenses: &["topology", "quantum", "boundary"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-07",
        title: "Genetic code↔blowup fiber: codon table 64=2^n (BT-51) corresponds to fiber space of C^6 blowup in F2",
        node_type: NodeType::Discovery,
        domains: &["Math", "Biology", "Topology"],
        confidence: 0.76,
        source_bts: &[185, 51, 101],
        constants_used: &["C-n", "C-tau", "C-J2"],
        lenses: &["topology", "evolution", "info"],
        validates: &[],
    },
    BlowupEntry {
        id: "DISC-BT118XDOM-08",
        title: "Water treatment CN=6↔topological matter: catalytic octahedral sites (BT-120) use same n=6 topology as TI edge states",
        node_type: NodeType::Discovery,
        domains: &["Environment", "Chemistry", "Material", "Topology"],
        confidence: 0.79,
        source_bts: &[185, 120, 92],
        constants_used: &["C-n", "C-tau"],
        lenses: &["topology", "boundary", "thermo"],
        validates: &["DISC-ENV-03"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Collect all clusters
// ═══════════════════════════════════════════════════════════════

const ALL_BLOWUP_CLUSTERS: &[&[BlowupEntry]] = &[
    BLOWUP_C6_DISCOVERIES,
    BLOWUP_DELPEZZO_DISCOVERIES,
    BLOWUP_E6_DISCOVERIES,
    BLOWUP_EMERGENCE_DISCOVERIES,
    BLOWUP_EXTENDED,
    BLOWUP_HYPOTHESES,
    BLOWUP_BRIDGES,
    TOPOEXP_DISCOVERIES,
    INVCORE_DISCOVERIES,
    BT118_CROSS_BRIDGES,
];

/// Total count of BT-185 discovery entries.
pub fn bt185_entry_count() -> usize {
    ALL_BLOWUP_CLUSTERS.iter().map(|c| c.len()).sum()
}

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn add_blowup_entries(graph: &mut DiscoveryGraph, entries: &[BlowupEntry]) -> (usize, usize) {
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

/// Add C⁶ blowup pillar discoveries. Returns (nodes, edges).
pub fn populate_c6_blowup(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_C6_DISCOVERIES)
}

/// Add del Pezzo₆ pillar discoveries. Returns (nodes, edges).
pub fn populate_delpezzo_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_DELPEZZO_DISCOVERIES)
}

/// Add E₆ Lie algebra pillar discoveries. Returns (nodes, edges).
pub fn populate_e6_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_E6_DISCOVERIES)
}

/// Add blowup-emergence universality discoveries. Returns (nodes, edges).
pub fn populate_emergence_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_EMERGENCE_DISCOVERIES)
}

/// Add extended sub-discoveries (granular validations). Returns (nodes, edges).
pub fn populate_blowup_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_EXTENDED)
}

/// Add blowup hypotheses. Returns (nodes, edges).
pub fn populate_blowup_hypotheses(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_HYPOTHESES)
}

/// Add cross-domain bridge discoveries connecting BT-185 to other BT clusters.
pub fn populate_blowup_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BLOWUP_BRIDGES)
}

/// Add topology expansion discoveries (Lie algebra, homotopy, matter). Returns (nodes, edges).
pub fn populate_topoexp_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, TOPOEXP_DISCOVERIES)
}

/// Add invariant lens core discoveries (evolutionary search). Returns (nodes, edges).
pub fn populate_invcore_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, INVCORE_DISCOVERIES)
}

/// Add cross-domain bridges BT-118+ ↔ BT-185. Returns (nodes, edges).
pub fn populate_bt118_cross_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_blowup_entries(graph, BT118_CROSS_BRIDGES)
}

/// Cross-link BT-185 nodes that share domains (Merges edges, bidirectional).
/// Returns the number of cross-domain edges added.
pub fn connect_blowup_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let blow_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-BLOW-")
                || n.id.starts_with("HYP-BLOW-")
                || n.id.starts_with("XDISC-BLOW-")
                || n.id.starts_with("XEXP-BLOW-")
                || n.id.starts_with("DISC-BLOW-BRIDGE-")
                || n.id.starts_with("DISC-TOPOEXP-")
                || n.id.starts_with("DISC-INVCORE-")
                || n.id.starts_with("DISC-BT118XDOM-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..blow_nodes.len() {
        for j in (i + 1)..blow_nodes.len() {
            let (ref id_a, ref dom_a) = blow_nodes[i];
            let (ref id_b, ref dom_b) = blow_nodes[j];

            let shared: usize = dom_a.iter().filter(|d| dom_b.contains(d)).count();
            if shared > 0 {
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

/// Cross-link BT-185 nodes with existing DISC-/HYP-/XDISC- nodes from other modules.
/// Requires >=2 shared domains. Returns number of bridge edges added.
pub fn connect_blowup_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let blow_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-BLOW-")
                || n.id.starts_with("HYP-BLOW-")
                || n.id.starts_with("XDISC-BLOW-")
                || n.id.starts_with("XEXP-BLOW-")
                || n.id.starts_with("DISC-BLOW-BRIDGE-")
                || n.id.starts_with("DISC-TOPOEXP-")
                || n.id.starts_with("DISC-INVCORE-")
                || n.id.starts_with("DISC-BT118XDOM-")
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
            (n.id.starts_with("DISC-") || n.id.starts_with("HYP-") || n.id.starts_with("XDISC-"))
                && !n.id.starts_with("DISC-BLOW-")
                && !n.id.starts_with("HYP-BLOW-")
                && !n.id.starts_with("XDISC-BLOW-")
                && !n.id.starts_with("DISC-BLOW-BRIDGE-")
                && !n.id.starts_with("DISC-TOPOEXP-")
                && !n.id.starts_with("DISC-INVCORE-")
                && !n.id.starts_with("DISC-BT118XDOM-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref b_id, ref b_dom) in &blow_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = b_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = b_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: b_id.clone(),
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

/// Populate all BT-185 discovery nodes and cross-domain edges in one call.
/// Call after all other populate_* functions.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_bt185_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let mut total_nodes = 0;
    for cluster in ALL_BLOWUP_CLUSTERS {
        let (n, _) = add_blowup_entries(graph, cluster);
        total_nodes += n;
    }
    let _cross = connect_blowup_cross_domain(graph);
    let _bridge = connect_blowup_to_existing(graph);

    (total_nodes, graph.edges.len() - edges_before)
}

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::discovery_nodes::populate_all_discoveries;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::extended_discovery_nodes::populate_all_extended;
    use crate::graph::recent_discoveries::populate_all_recent_discoveries;
    use crate::graph::recent_extended_nodes::populate_all_recent_extended;

    fn full_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_extended(&mut g);
        populate_all_recent_discoveries(&mut g);
        populate_all_recent_extended(&mut g);
        g
    }

    // ── Entry counts ──

    #[test]
    fn test_c6_count() {
        assert_eq!(BLOWUP_C6_DISCOVERIES.len(), 2);
    }

    #[test]
    fn test_delpezzo_count() {
        assert_eq!(BLOWUP_DELPEZZO_DISCOVERIES.len(), 2);
    }

    #[test]
    fn test_e6_count() {
        assert_eq!(BLOWUP_E6_DISCOVERIES.len(), 2);
    }

    #[test]
    fn test_emergence_count() {
        assert_eq!(BLOWUP_EMERGENCE_DISCOVERIES.len(), 3);
    }

    #[test]
    fn test_extended_count() {
        assert_eq!(BLOWUP_EXTENDED.len(), 5);
    }

    #[test]
    fn test_hypotheses_count() {
        assert_eq!(BLOWUP_HYPOTHESES.len(), 3);
    }

    #[test]
    fn test_bridges_count() {
        assert_eq!(BLOWUP_BRIDGES.len(), 6);
    }

    #[test]
    fn test_topoexp_count() {
        assert_eq!(TOPOEXP_DISCOVERIES.len(), 6, "6 topology expansion discoveries");
    }

    #[test]
    fn test_invcore_count() {
        assert_eq!(INVCORE_DISCOVERIES.len(), 4, "3 discoveries + 1 experiment");
    }

    #[test]
    fn test_bt118_cross_count() {
        assert_eq!(BT118_CROSS_BRIDGES.len(), 8, "8 cross-domain bridges BT-118+↔BT-185");
    }

    #[test]
    fn test_total_bt185_entry_count() {
        // 2+2+2+3+5+3+6+6+4+8 = 41
        assert_eq!(bt185_entry_count(), 41);
    }

    // ── Population tests ──

    #[test]
    fn test_populate_c6_blowup() {
        let mut g = full_graph();
        let before = g.nodes.len();
        let (nodes, edges) = populate_c6_blowup(&mut g);
        assert_eq!(nodes, 2);
        assert_eq!(g.nodes.len(), before + 2);
        // 2 entries: BT derives + constants Uses + validates
        assert!(edges >= 5, "C6 blowup should have 5+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_e6_discoveries() {
        let mut g = full_graph();
        let (nodes, edges) = populate_e6_discoveries(&mut g);
        assert_eq!(nodes, 2);
        assert!(edges >= 6, "E6 should have 6+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_bridges() {
        let mut g = full_graph();
        let (nodes, edges) = populate_blowup_bridges(&mut g);
        assert_eq!(nodes, 6);
        assert!(edges >= 18, "Bridges should have 18+ edges, got {}", edges);
    }

    #[test]
    fn test_populate_topoexp() {
        let mut g = full_graph();
        let (n, e) = populate_topoexp_discoveries(&mut g);
        assert_eq!(n, 6);
        assert!(e >= 15, "topology expansion needs many edges: got {}", e);
    }

    #[test]
    fn test_populate_invcore() {
        let mut g = full_graph();
        let (n, e) = populate_invcore_discoveries(&mut g);
        assert_eq!(n, 4);
        assert!(e >= 10, "invariant core edges: got {}", e);
    }

    #[test]
    fn test_populate_bt118_cross() {
        let mut g = full_graph();
        let (n, e) = populate_bt118_cross_bridges(&mut g);
        assert_eq!(n, 8);
        assert!(e >= 20, "cross-domain bridges span many BTs and constants: got {}", e);
    }

    #[test]
    fn test_populate_all_bt185() {
        let mut g = full_graph();
        let before_nodes = g.nodes.len();
        let before_edges = g.edges.len();
        let (nodes, edges) = populate_all_bt185_discoveries(&mut g);

        assert_eq!(nodes, 41, "41 BT-185 nodes total");
        assert!(
            edges > 120,
            "BT-185 nodes should add 120+ edges (derives+uses+validates+cross), got {}",
            edges
        );
        assert_eq!(g.nodes.len(), before_nodes + 41);
        assert!(g.edges.len() > before_edges + 120);
    }

    // ── Validates edges ──

    #[test]
    fn test_validates_c6_cohomology() {
        let mut g = full_graph();
        populate_c6_blowup(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BLOW-02"
                && e.to == "DISC-BLOW-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DISC-BLOW-02 should validate DISC-BLOW-01");
    }

    #[test]
    fn test_validates_sle6_bridge() {
        let mut g = full_graph();
        populate_blowup_bridges(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BLOW-BRIDGE-01"
                && e.to == "DISC-MATH-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DISC-BLOW-BRIDGE-01 should validate DISC-MATH-01 (SLE6)");
    }

    #[test]
    fn test_validates_s6_bridge() {
        let mut g = full_graph();
        populate_blowup_bridges(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BLOW-BRIDGE-02"
                && e.to == "DISC-MATH-02"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DISC-BLOW-BRIDGE-02 should validate DISC-MATH-02 (S3 bootstrap)");
    }

    // ── BT-185 derives ──

    #[test]
    fn test_bt185_derives_discoveries() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        let derives: Vec<_> = g.edges.iter()
            .filter(|e| e.from == "BT-185" && e.edge_type == EdgeType::Derives)
            .collect();
        // BT-185 derives most of the 41 nodes (some shared with other BTs)
        assert!(derives.len() >= 30, "BT-185 should derive 30+ nodes, got {}", derives.len());
    }

    #[test]
    fn test_validates_topoexp_lie_algebra() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-TOPOEXP-01"
                && e.to == "DISC-BLOW-05"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DISC-TOPOEXP-01 (SU6) should validate DISC-BLOW-05 (E6)");
    }

    #[test]
    fn test_validates_invcore_emergence() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-INVCORE-01"
                && e.to == "DISC-BLOW-07"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "DISC-INVCORE-01 should validate DISC-BLOW-07 (emergence archetype)");
    }

    #[test]
    fn test_bt118xdom_validates_env() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BT118XDOM-04"
                && e.to == "DISC-ENV-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "Carbon Z=6↔C6 bridge should validate DISC-ENV-01");
    }

    #[test]
    fn test_bt118xdom_validates_robo() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BT118XDOM-03"
                && e.to == "DISC-ROBO-01"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "SE(3)↔C6 bridge should validate DISC-ROBO-01");
    }

    #[test]
    fn test_bt118xdom_validates_water() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);
        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-BT118XDOM-08"
                && e.to == "DISC-ENV-03"
                && e.edge_type == EdgeType::Validates
        });
        assert!(has_val, "Water treatment↔topological matter bridge should validate DISC-ENV-03");
    }

    // ── Cross-domain edges ──

    #[test]
    fn test_blowup_cross_domain_edges() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        // DISC-BLOW-05 (Math, Particle, StringTheory) and
        // DISC-BLOW-03 (Math, NumberTheory, Combinatorics) share Math
        let has_merge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-BLOW-05" && e.to == "DISC-BLOW-03")
                    || (e.from == "DISC-BLOW-03" && e.to == "DISC-BLOW-05"))
        });
        assert!(has_merge, "E6 and dP6 discoveries should merge via shared Math domain");
    }

    #[test]
    fn test_blowup_to_existing_edges() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        // DISC-BLOW-05 (Math, Particle, StringTheory) should connect to
        // DISC-MATH-08 (Math, Particle, Crypto, Physics) via Math+Particle
        let has_bridge = g.edges.iter().any(|e| {
            e.edge_type == EdgeType::Merges
                && ((e.from == "DISC-BLOW-05" && e.to == "DISC-MATH-08")
                    || (e.from == "DISC-MATH-08" && e.to == "DISC-BLOW-05"))
        });
        assert!(has_bridge, "E6 discovery should bridge to existing Math-08 via Math+Particle");
    }

    // ── Node type distribution ──

    #[test]
    fn test_bt185_node_types() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        let disc = g.nodes.iter()
            .filter(|n| n.id.starts_with("DISC-BLOW-") && !n.id.starts_with("DISC-BLOW-BRIDGE-"))
            .count();
        let xdisc = g.nodes.iter()
            .filter(|n| n.id.starts_with("XDISC-BLOW-"))
            .count();
        let hyp = g.nodes.iter()
            .filter(|n| n.id.starts_with("HYP-BLOW-"))
            .count();
        let bridges = g.nodes.iter()
            .filter(|n| n.id.starts_with("DISC-BLOW-BRIDGE-"))
            .count();
        let topoexp = g.nodes.iter()
            .filter(|n| n.id.starts_with("DISC-TOPOEXP-"))
            .count();
        let invcore = g.nodes.iter()
            .filter(|n| n.id.starts_with("DISC-INVCORE-"))
            .count();
        let bt118xdom = g.nodes.iter()
            .filter(|n| n.id.starts_with("DISC-BT118XDOM-"))
            .count();

        assert_eq!(disc, 9, "9 discovery nodes (DISC-BLOW-01~09)");
        assert_eq!(xdisc, 5, "5 extended nodes (XDISC-BLOW-01~05)");
        assert_eq!(hyp, 3, "3 hypothesis nodes (HYP-BLOW-01~03)");
        assert_eq!(bridges, 6, "6 bridge nodes (DISC-BLOW-BRIDGE-01~06)");
        assert_eq!(topoexp, 6, "6 topology expansion nodes");
        assert_eq!(invcore, 4, "4 invariant core nodes (3 disc + 1 exp)");
        assert_eq!(bt118xdom, 8, "8 cross-domain bridge nodes BT-118+");
        assert_eq!(disc + xdisc + hyp + bridges + topoexp + invcore + bt118xdom, 41);
    }

    // ── Hypothesis confidence ──

    #[test]
    fn test_hypothesis_low_confidence() {
        for entry in BLOWUP_HYPOTHESES {
            assert!(
                entry.confidence < 0.60,
                "Hypothesis {} confidence {} should be < 0.60",
                entry.id, entry.confidence
            );
        }
    }

    // ── Data integrity ──

    #[test]
    fn test_no_duplicate_bt185_ids() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique (including BT-185 + TOPOEXP + INVCORE + BT118XDOM)");
    }

    #[test]
    fn test_all_bt185_source_bts_exist() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        for cluster in ALL_BLOWUP_CLUSTERS {
            for entry in *cluster {
                for &bt_id in entry.source_bts {
                    let bt_node_id = format!("BT-{}", bt_id);
                    assert!(
                        g.nodes.iter().any(|n| n.id == bt_node_id),
                        "Source BT {} referenced by {} not found in graph",
                        bt_node_id, entry.id
                    );
                }
            }
        }
    }

    #[test]
    fn test_edge_strength_valid() {
        let mut g = full_graph();
        populate_all_bt185_discoveries(&mut g);

        for edge in &g.edges {
            assert!(
                edge.strength > 0.0 && edge.strength <= 1.0,
                "Edge {}->{} strength {} out of range",
                edge.from, edge.to, edge.strength
            );
        }
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_BLOWUP_CLUSTERS {
            for e in *cluster {
                assert!(
                    e.confidence > 0.0 && e.confidence <= 1.0,
                    "{} has invalid confidence {}", e.id, e.confidence
                );
            }
        }
    }
}
