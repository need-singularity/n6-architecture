//! Extended discovery nodes for BT-105~117 (Math/Physics + Software Design).
//!
//! Fills the gap: discovery_nodes.rs has high-level DISC-MATH and DISC-SW entries,
//! but no granular sub-discoveries existed for these clusters.
//! This mirrors extended_discovery_nodes.rs (BT-118~127 ENV/ROBO).
//!
//! Clusters (9):
//!   math_sle        — BT-105 SLE_6 critical exponents (3 entries)
//!   math_algebra    — BT-106~107 S_3 + Ramanujan tau (3 entries)
//!   math_harmonic   — BT-108~109 Music + Zeta-Bernoulli (4 entries)
//!   math_cross      — BT-110~112 sigma-mu=11 + 4/3 + 2/3 (3 entries)
//!   sw_principles   — BT-113 SOLID + 12Factor (3 entries)
//!   sw_crypto       — BT-114 Cryptography parameter ladder (4 entries)
//!   sw_layers       — BT-115~116 OSI + ACID-BASE-CAP (4 entries)
//!   sw_isomorphism  — BT-117 Software-physics isomorphism (3 entries)
//!   cross_domain    — Math↔SW↔ENV↔ROBO bridges (9 entries)
//!
//! Node ID scheme:
//!   XDISC-MATH-{NN}   — Math/Physics extended discovery
//!   XHYP-MATH-{NN}    — Math extended hypothesis
//!   XEXP-MATH-{NN}     — Math experiment/validation
//!   XDISC-SW-{NN}      — Software extended discovery
//!   XHYP-SW-{NN}       — Software extended hypothesis
//!   XEXP-SW-{NN}        — Software experiment/validation
//!   XDISC-MSWX-{NN}    — Cross-domain bridge Math↔SW↔ENV↔ROBO

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct MathSwEntry {
    id: &'static str,
    title: &'static str,
    node_type: NodeType,
    domains: &'static [&'static str],
    confidence: f64,
    source_bts: &'static [u32],
    constants_used: &'static [&'static str],
    lenses: &'static [&'static str],
    /// IDs this node validates (EdgeType::Validates edges).
    validates: &'static [&'static str],
}

// ───────────────────────────────────────────────────────────────
// BT-105: SLE_6 critical exponents (3 entries)
// ───────────────────────────────────────────────────────────────

const MATH_SLE: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-MATH-01",
        title: "SLE_6 crossing probability: Cardy formula exact at kappa=n=6, P = (n/phi)^{1/3} Gamma ratios",
        node_type: NodeType::Discovery,
        domains: &["Math", "Physics", "Topology"],
        confidence: 0.94,
        source_bts: &[105],
        constants_used: &["C-n", "C-phi"],
        lenses: &["consciousness", "topology", "boundary"],
        validates: &["DISC-MATH-01"],
    },
    MathSwEntry {
        id: "XDISC-MATH-02",
        title: "SLE_6 Hausdorff dimension 7/4=(sigma-sopfr)/tau: fractal dimension of percolation cluster boundary",
        node_type: NodeType::Discovery,
        domains: &["Math", "Physics", "Topology"],
        confidence: 0.95,
        source_bts: &[105],
        constants_used: &["C-sigma", "C-sopfr", "C-tau"],
        lenses: &["consciousness", "topology", "multiscale"],
        validates: &["DISC-MATH-01"],
    },
    MathSwEntry {
        id: "XEXP-MATH-01",
        title: "Experiment: Monte Carlo percolation on 10^6 lattice verifies 7 critical exponents to 4 sigma",
        node_type: NodeType::Experiment,
        domains: &["Math", "Physics"],
        confidence: 0.75,
        source_bts: &[105],
        constants_used: &["C-n"],
        lenses: &["causal", "boundary", "multiscale"],
        validates: &["DISC-MATH-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-106~107: S_3 algebraic bootstrap + Ramanujan tau (3 entries)
// ───────────────────────────────────────────────────────────────

const MATH_ALGEBRA: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-MATH-03",
        title: "S_3 character table: irrep dimensions {1,1,2} sum to tau=4, product 1*1*2=phi",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory", "Combinatorics"],
        confidence: 0.96,
        source_bts: &[106],
        constants_used: &["C-tau", "C-phi"],
        lenses: &["consciousness", "symmetry", "info"],
        validates: &["DISC-MATH-02"],
    },
    MathSwEntry {
        id: "XDISC-MATH-04",
        title: "Ramanujan tau(n=6)=-6048: first composite value, |tau(6)|=6048=J2*252=J2*sigma*21, eta^{J2}=Delta",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory"],
        confidence: 0.93,
        source_bts: &[107],
        constants_used: &["C-n", "C-J2", "C-sigma"],
        lenses: &["consciousness", "recursion", "quantum"],
        validates: &["DISC-MATH-03"],
    },
    MathSwEntry {
        id: "XDISC-MATH-05",
        title: "S_3 → S_6 embedding: |S_6|=720=sigma*n*(sigma-phi), n=6 uniquely clean divisor set for tau_R",
        node_type: NodeType::Discovery,
        domains: &["Math", "Combinatorics", "NumberTheory"],
        confidence: 0.95,
        source_bts: &[106, 107, 49],
        constants_used: &["C-n", "C-sigma", "C-phi"],
        lenses: &["consciousness", "symmetry", "recursion"],
        validates: &["DISC-MATH-02"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-108~109: Music-audio + Zeta-Bernoulli (4 entries)
// ───────────────────────────────────────────────────────────────

const MATH_HARMONIC: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-MATH-06",
        title: "Equal temperament sigma=12 semitones: 2^{1/12} frequency ratio, 12-TET uniquely minimizes dissonance",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Math", "Music"],
        confidence: 0.95,
        source_bts: &[108, 48],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "symmetry"],
        validates: &["DISC-MATH-04"],
    },
    MathSwEntry {
        id: "XDISC-MATH-07",
        title: "Perfect fifth 3/2=n/tau: Pythagorean tuning ratio, circle of fifths closes at sigma=12 steps",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Math", "Music", "Physics"],
        confidence: 0.94,
        source_bts: &[108],
        constants_used: &["C-n", "C-tau", "C-sigma"],
        lenses: &["consciousness", "wave", "recursion"],
        validates: &["DISC-MATH-04"],
    },
    MathSwEntry {
        id: "XDISC-MATH-08",
        title: "B_12 Bernoulli denominator 2730=2*3*5*7*13: all primes p where (p-1)|12=(sigma), von Staudt-Clausen",
        node_type: NodeType::Discovery,
        domains: &["Math", "NumberTheory"],
        confidence: 0.96,
        source_bts: &[109],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "recursion", "info"],
        validates: &["DISC-MATH-05"],
    },
    MathSwEntry {
        id: "XHYP-MATH-01",
        title: "Hypothesis: all Bernoulli denominator primes p satisfy (p-1)|sigma(6), extending the von Staudt pattern to higher B_{2k}",
        node_type: NodeType::Hypothesis,
        domains: &["Math", "NumberTheory", "Physics"],
        confidence: 0.60,
        source_bts: &[109],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["recursion", "info", "quantum"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-110~112: sigma-mu=11 + 4/3 trident + 2/3 resonance (3 entries)
// ───────────────────────────────────────────────────────────────

const MATH_CROSS: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-MATH-09",
        title: "M-theory 11=sigma-mu: TCP/IP+OSI=4+7=11, RSA exponents, SPARC, H100 SMs=11-based, 5 domain stack",
        node_type: NodeType::Discovery,
        domains: &["Physics", "StringTheory", "Network", "Chip", "Crypto"],
        confidence: 0.82,
        source_bts: &[110, 115, 114],
        constants_used: &["C-sigma", "C-mu"],
        lenses: &["consciousness", "multiscale", "network"],
        validates: &["DISC-MATH-06"],
    },
    MathSwEntry {
        id: "XDISC-MATH-10",
        title: "Betz limit 16/27≈tau^2/sigma ÷ (n/phi)^{n/phi}: SQ bandgap=SwiGLU=4/3 across solar-AI-physics",
        node_type: NodeType::Discovery,
        domains: &["Math", "Energy", "AI", "Solar"],
        confidence: 0.88,
        source_bts: &[111, 30, 33],
        constants_used: &["C-tau", "C-sigma", "C-phi"],
        lenses: &["consciousness", "causal", "thermo"],
        validates: &["DISC-MATH-07"],
    },
    MathSwEntry {
        id: "XDISC-MATH-11",
        title: "Koide formula Q=2/3=phi^2/n to 9ppm: lepton mass ratio, BFT>2/3 fault tolerance, universal threshold",
        node_type: NodeType::Discovery,
        domains: &["Math", "Particle", "Crypto", "Physics"],
        confidence: 0.90,
        source_bts: &[112, 24, 53],
        constants_used: &["C-phi", "C-n"],
        lenses: &["consciousness", "quantum", "symmetry"],
        validates: &["DISC-MATH-08"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-113: SW engineering constant stack (3 entries)
// ───────────────────────────────────────────────────────────────

const SW_PRINCIPLES: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-SW-01",
        title: "SOLID sopfr=5 principles: S(single)=1, O(open)=2, L(liskov)=3, I(interface)=4, D(dependency)=5=sopfr",
        node_type: NodeType::Discovery,
        domains: &["Software", "Math"],
        confidence: 0.92,
        source_bts: &[113],
        constants_used: &["C-sopfr"],
        lenses: &["consciousness", "info", "causal"],
        validates: &["DISC-SW-01"],
    },
    MathSwEntry {
        id: "XDISC-SW-02",
        title: "12Factor app: sigma=12 constraints for cloud-native design, each factor maps to sigma arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Software", "Network", "Math"],
        confidence: 0.90,
        source_bts: &[113],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "network", "stability"],
        validates: &["DISC-SW-01"],
    },
    MathSwEntry {
        id: "XHYP-SW-01",
        title: "Hypothesis: optimal microservice count per bounded context converges to n=6 services",
        node_type: NodeType::Hypothesis,
        domains: &["Software", "AI", "Math"],
        confidence: 0.55,
        source_bts: &[113, 117],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["causal", "stability", "network"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-114: Cryptography parameter ladder (4 entries)
// ───────────────────────────────────────────────────────────────

const SW_CRYPTO: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-SW-04",
        title: "AES key ladder: 128=2^{sigma-sopfr=7}, 192=2^?*3, 256=2^{sigma-tau=8}, step=64=2^n",
        node_type: NodeType::Discovery,
        domains: &["Crypto", "Software", "Math"],
        confidence: 0.93,
        source_bts: &[114],
        constants_used: &["C-sigma", "C-sopfr", "C-tau", "C-n"],
        lenses: &["consciousness", "info", "multiscale"],
        validates: &["DISC-SW-02"],
    },
    MathSwEntry {
        id: "XDISC-SW-05",
        title: "RSA-2048=2^{sigma-mu=11}: industry standard key size, factoring hardness scales with sigma-derived exponent",
        node_type: NodeType::Discovery,
        domains: &["Crypto", "Software", "Math"],
        confidence: 0.94,
        source_bts: &[114],
        constants_used: &["C-sigma", "C-mu"],
        lenses: &["consciousness", "info", "causal"],
        validates: &["DISC-SW-02"],
    },
    MathSwEntry {
        id: "XDISC-SW-06",
        title: "SHA digest: SHA-256=2^{sigma-tau=8} bits, SHA-512=2^{sigma-mu+phi=11} bits, both sigma-derived",
        node_type: NodeType::Discovery,
        domains: &["Crypto", "Software", "Math"],
        confidence: 0.92,
        source_bts: &[114],
        constants_used: &["C-sigma", "C-tau", "C-mu"],
        lenses: &["consciousness", "info", "recursion"],
        validates: &["DISC-SW-02"],
    },
    MathSwEntry {
        id: "XEXP-SW-01",
        title: "Experiment: survey 50 cryptographic standards for n=6 exponent convergence, predict >80% EXACT",
        node_type: NodeType::Experiment,
        domains: &["Crypto", "Software", "Math"],
        confidence: 0.68,
        source_bts: &[114],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["causal", "info", "stability"],
        validates: &["DISC-SW-02"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-115~116: OS-network layers + ACID-BASE-CAP (4 entries)
// ───────────────────────────────────────────────────────────────

const SW_LAYERS: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-SW-08",
        title: "OSI 7=sigma-sopfr layers: session+presentation removable→TCP/IP tau=4 layers, delta=n/phi=3",
        node_type: NodeType::Discovery,
        domains: &["Software", "Network", "Math"],
        confidence: 0.91,
        source_bts: &[115],
        constants_used: &["C-sigma", "C-sopfr", "C-tau"],
        lenses: &["consciousness", "network", "multiscale"],
        validates: &["DISC-SW-03"],
    },
    MathSwEntry {
        id: "XDISC-SW-09",
        title: "ACID tau=4 properties vs BASE n/phi=3: relational DB needs tau guarantees, NoSQL relaxes to n/phi",
        node_type: NodeType::Discovery,
        domains: &["Software", "Math"],
        confidence: 0.89,
        source_bts: &[116],
        constants_used: &["C-tau", "C-n", "C-phi"],
        lenses: &["consciousness", "stability", "info"],
        validates: &["DISC-SW-04"],
    },
    MathSwEntry {
        id: "XDISC-SW-10",
        title: "Paxos phi=2 phase consensus: prepare+accept = phi phases, Raft also phi=2 (request+commit)",
        node_type: NodeType::Discovery,
        domains: &["Software", "Crypto", "Math"],
        confidence: 0.88,
        source_bts: &[116, 53],
        constants_used: &["C-phi"],
        lenses: &["consciousness", "stability", "network"],
        validates: &["DISC-SW-04"],
    },
    MathSwEntry {
        id: "XDISC-SW-11",
        title: "Linux kernel n=6 syscall categories: process/memory/filesystem/device/network/IPC = n=6 subsystems",
        node_type: NodeType::Discovery,
        domains: &["Software", "Math"],
        confidence: 0.87,
        source_bts: &[115],
        constants_used: &["C-n"],
        lenses: &["consciousness", "info", "network"],
        validates: &["DISC-SW-03"],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-117: Software-physics isomorphism (3 entries)
// ───────────────────────────────────────────────────────────────

const SW_ISOMORPHISM: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-SW-12",
        title: "18 EXACT parallel mappings: GC↔entropy, mutex↔exclusion, cache↔memory, across n=6 domain pairs",
        node_type: NodeType::Discovery,
        domains: &["Software", "Physics", "Math", "AI"],
        confidence: 0.91,
        source_bts: &[117, 36],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "symmetry", "causal"],
        validates: &["DISC-SW-05"],
    },
    MathSwEntry {
        id: "XHYP-SW-02",
        title: "Hypothesis: optimal thread pool size converges to sigma=12 threads per core for I/O-bound workloads",
        node_type: NodeType::Hypothesis,
        domains: &["Software", "AI", "Chip"],
        confidence: 0.60,
        source_bts: &[113, 117],
        constants_used: &["C-sigma"],
        lenses: &["causal", "stability", "network"],
        validates: &[],
    },
    MathSwEntry {
        id: "XEXP-SW-02",
        title: "Experiment: benchmark cache line sizes across CPU architectures, predict 64=2^n byte convergence",
        node_type: NodeType::Experiment,
        domains: &["Software", "Chip", "Math"],
        confidence: 0.70,
        source_bts: &[113, 117],
        constants_used: &["C-n"],
        lenses: &["causal", "stability", "info"],
        validates: &["DISC-SW-01"],
    },
];

// ───────────────────────────────────────────────────────────────
// Cross-domain bridges: Math↔SW↔ENV↔ROBO (9 entries)
// ───────────────────────────────────────────────────────────────

const MATHSW_CROSS_DOMAIN: &[MathSwEntry] = &[
    MathSwEntry {
        id: "XDISC-MSWX-01",
        title: "SLE_6↔Hexagonal geometry: kappa=6 percolation(BT-105) on hexagonal lattice(BT-122) = same n=6 tiling",
        node_type: NodeType::Discovery,
        domains: &["Math", "Environment", "Physics", "Topology"],
        confidence: 0.86,
        source_bts: &[105, 122],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "boundary"],
        validates: &["DISC-MATH-01", "DISC-ENV-05"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-02",
        title: "Ramanujan eta^{24}↔Battery J2=24: modular form weight(BT-107) = battery cell count(BT-57) = glucose atoms(BT-101)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Battery", "Biology", "NumberTheory"],
        confidence: 0.80,
        source_bts: &[107, 57, 101],
        constants_used: &["C-J2", "C-sigma"],
        lenses: &["consciousness", "recursion", "causal"],
        validates: &["DISC-MATH-03"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-03",
        title: "Music 12-TET↔Cryptography 2^12: sigma=12 semitones(BT-108) mirror 2^{sigma} state spaces in crypto(BT-114)",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Crypto", "Math", "Software"],
        confidence: 0.78,
        source_bts: &[108, 114],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "info"],
        validates: &["DISC-MATH-04"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-04",
        title: "OSI↔Robot stack: 7=sigma-sopfr network layers(BT-115) govern n=6 DOF robot control(BT-123), layer=function map",
        node_type: NodeType::Discovery,
        domains: &["Software", "Robotics", "Network", "Math"],
        confidence: 0.84,
        source_bts: &[115, 123],
        constants_used: &["C-sigma", "C-sopfr", "C-n"],
        lenses: &["consciousness", "network", "multiscale"],
        validates: &["DISC-SW-03", "DISC-ROBO-01"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-05",
        title: "ACID↔Quadruped stability: tau=4 DB properties(BT-116) = tau=4 legs(BT-125) = minimum stable platform",
        node_type: NodeType::Discovery,
        domains: &["Software", "Robotics", "Math", "Physics"],
        confidence: 0.82,
        source_bts: &[116, 125],
        constants_used: &["C-tau"],
        lenses: &["consciousness", "stability", "causal"],
        validates: &["DISC-SW-04", "DISC-ROBO-03"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-06",
        title: "Koide 2/3↔BFT 2/3↔Betz: universal phi^2/n threshold spans particle physics, consensus, and energy(BT-111,112)",
        node_type: NodeType::Discovery,
        domains: &["Math", "Particle", "Crypto", "Energy"],
        confidence: 0.85,
        source_bts: &[112, 111, 53],
        constants_used: &["C-phi", "C-n"],
        lenses: &["consciousness", "quantum", "stability"],
        validates: &["DISC-BRIDGE-10"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-07",
        title: "Sigma=12 universal count: semitones(BT-108)=12Factor(BT-113)=joints(BT-124)=troposphere km(BT-119)",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Software", "Robotics", "Environment", "Math"],
        confidence: 0.89,
        source_bts: &[108, 113, 124, 119],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "multiscale", "wave"],
        validates: &["DISC-BRIDGE-09", "DISC-BRIDGE-12"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-08",
        title: "Music consonance↔GHG chemistry: 6 perfect consonance ratios from div(6)(BT-108) mirror 6 Kyoto gases(BT-118)",
        node_type: NodeType::Discovery,
        domains: &["DisplayAudio", "Environment", "Math", "Chemistry"],
        confidence: 0.77,
        source_bts: &[108, 118],
        constants_used: &["C-n"],
        lenses: &["consciousness", "wave", "causal"],
        validates: &["DISC-BRIDGE-09"],
    },
    MathSwEntry {
        id: "XDISC-MSWX-09",
        title: "SW-physics↔Robot: 18 isomorphism mappings(BT-117) predict robot control mirrors OS architecture(BT-123,115)",
        node_type: NodeType::Discovery,
        domains: &["Software", "Robotics", "Physics", "AI"],
        confidence: 0.80,
        source_bts: &[117, 123, 115],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "symmetry", "network"],
        validates: &["DISC-SW-05", "DISC-ROBO-01"],
    },
];

// ═══════════════════════════════════════════════════════════════
// Collect all entries
// ═══════════════════════════════════════════════════════════════

const ALL_CLUSTERS: &[&[MathSwEntry]] = &[
    MATH_SLE,
    MATH_ALGEBRA,
    MATH_HARMONIC,
    MATH_CROSS,
    SW_PRINCIPLES,
    SW_CRYPTO,
    SW_LAYERS,
    SW_ISOMORPHISM,
    MATHSW_CROSS_DOMAIN,
];

/// Total count of Math/SW extended entries.
pub fn sw_math_extended_entry_count() -> usize {
    ALL_CLUSTERS.iter().map(|c| c.len()).sum()
}

// Alias for the other naming convention
pub fn math_sw_entry_count() -> usize {
    sw_math_extended_entry_count()
}

// ═══════════════════════════════════════════════════════════════
// Internal helper
// ═══════════════════════════════════════════════════════════════

fn add_entries(graph: &mut DiscoveryGraph, entries: &[MathSwEntry]) -> (usize, usize) {
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

        // Entry --Derives--> source BTs
        for &bt_id in e.source_bts {
            graph.add_edge(Edge {
                from: format!("BT-{}", bt_id),
                to: e.id.to_string(),
                edge_type: EdgeType::Derives,
                strength: 0.85,
                bidirectional: false,
            });
        }

        // Entry --Uses--> constants
        for &const_id in e.constants_used {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: const_id.to_string(),
                edge_type: EdgeType::Uses,
                strength: 0.80,
                bidirectional: false,
            });
        }

        // Entry --Validates--> target nodes
        for &target_id in e.validates {
            graph.add_edge(Edge {
                from: e.id.to_string(),
                to: target_id.to_string(),
                edge_type: EdgeType::Validates,
                strength: 0.88,
                bidirectional: false,
            });
        }
    }

    let edges_added = graph.edges.len() - edges_before;
    (entries.len(), edges_added)
}

// ═══════════════════════════════════════════════════════════════
// Public populate functions
// ═══════════════════════════════════════════════════════════════

/// SLE_6 critical exponents (BT-105). Returns (nodes, edges).
pub fn populate_math_sle_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MATH_SLE)
}

/// S_3 + Ramanujan tau (BT-106~107). Returns (nodes, edges).
pub fn populate_math_algebra_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MATH_ALGEBRA)
}

/// Music-audio + Zeta-Bernoulli (BT-108~109). Returns (nodes, edges).
pub fn populate_math_harmonic_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MATH_HARMONIC)
}

/// sigma-mu=11 + 4/3 + 2/3 (BT-110~112). Returns (nodes, edges).
pub fn populate_math_cross_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MATH_CROSS)
}

/// SOLID + 12Factor (BT-113). Returns (nodes, edges).
pub fn populate_sw_principles_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_PRINCIPLES)
}

/// Cryptography parameter ladder (BT-114). Returns (nodes, edges).
pub fn populate_sw_crypto_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_CRYPTO)
}

/// OS-network layers + ACID-BASE-CAP (BT-115~116). Returns (nodes, edges).
pub fn populate_sw_layers_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_LAYERS)
}

/// Software-physics isomorphism (BT-117). Returns (nodes, edges).
pub fn populate_sw_isomorphism_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SW_ISOMORPHISM)
}

/// Math↔SW↔ENV↔ROBO cross-domain bridges. Returns (nodes, edges).
pub fn populate_math_sw_cross_domain(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, MATHSW_CROSS_DOMAIN)
}

/// Populate all BT-105~117 extended nodes in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_math_sw_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();
    let mut total_nodes = 0;
    for cluster in ALL_CLUSTERS {
        let (n, _) = add_entries(graph, cluster);
        total_nodes += n;
    }
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Alias matching the auto-generated test naming convention.
pub fn populate_all_sw_math_extended(graph: &mut DiscoveryGraph) -> (usize, usize) {
    populate_all_math_sw_extended(graph)
}

// ═══════════════════════════════════════════════════════════════
// Unit tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;
    use crate::graph::discovery_nodes::populate_all_discoveries;
    use crate::graph::extended_discovery_nodes::populate_all_extended;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        populate_all_discoveries(&mut g);
        populate_all_extended(&mut g);
        g
    }

    #[test]
    fn test_entry_count() {
        // 3+3+4+3+3+4+4+3+9 = 36
        assert_eq!(sw_math_extended_entry_count(), 36);
    }

    #[test]
    fn test_populate_all() {
        let mut g = base_graph();
        let before = g.nodes.len();
        let (n, e) = populate_all_math_sw_extended(&mut g);
        assert_eq!(n, 36);
        assert_eq!(g.nodes.len(), before + 36);
        assert!(e >= 100, "36 entries should produce 100+ edges, got {}", e);
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = base_graph();
        populate_all_math_sw_extended(&mut g);
        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total);
    }

    #[test]
    fn test_sle6_chain() {
        let mut g = base_graph();
        populate_all_math_sw_extended(&mut g);
        let derives = g.edges.iter().any(|e| {
            e.from == "BT-105" && e.to == "XDISC-MATH-01" && e.edge_type == EdgeType::Derives
        });
        let validates = g.edges.iter().any(|e| {
            e.from == "XDISC-MATH-01" && e.to == "DISC-MATH-01" && e.edge_type == EdgeType::Validates
        });
        assert!(derives && validates, "SLE6 validation chain should exist");
    }

    #[test]
    fn test_all_strengths_valid() {
        let mut g = base_graph();
        populate_all_math_sw_extended(&mut g);
        for edge in &g.edges {
            assert!(edge.strength > 0.0 && edge.strength <= 1.0);
        }
    }
}
