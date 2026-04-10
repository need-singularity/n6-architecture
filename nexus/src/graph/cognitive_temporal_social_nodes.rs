//! Discovery nodes from Cognitive (BT-210~211,219,222), Temporal (BT-212~213,221,224),
//! Social (BT-214~215,220,223), and Cross-Domain Meta (BT-225) Breakthrough Theorems.
//!
//! Node ID scheme:
//!   DISC-COG-{NN}   — Cognitive architecture discoveries
//!   DISC-TEMP-{NN}  — Temporal architecture discoveries
//!   DISC-SOC-{NN}   — Social architecture discoveries
//!   DISC-CTS-{NN}   — Cognitive-Temporal-Social cross-domain bridges
//!   HYP-CTS-{NN}    — Cross-domain hypotheses

use super::edge::{Edge, EdgeType};
use super::node::{Node, NodeType};
use super::persistence::DiscoveryGraph;

// ═══════════════════════════════════════════════════════════════
// Entry struct
// ═══════════════════════════════════════════════════════════════

struct CtsEntry {
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
// BT-210~211, BT-219, BT-222: Cognitive Architecture
// ───────────────────────────────────────────────────────────────

const COGNITIVE_DISCOVERIES: &[CtsEntry] = &[
    CtsEntry {
        id: "DISC-COG-01",
        title: "Cerebral cortex 6 layers = n universality: Brodmann 1909, all mammals share n=6 laminar structure",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Biology", "Neuroscience"],
        confidence: 0.95,
        source_bts: &[210],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "recursion"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-COG-02",
        title: "Grid cell hexagonal lattice = K2=n=6 kissing number in 2D (Moser Nobel 2014, Hales 2001)",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Math", "Neuroscience", "Topology"],
        confidence: 0.93,
        source_bts: &[211, 122],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-COG-03",
        title: "Cranial nerves sigma=12: universal vertebrate count, EEG n=6 frequency bands",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Biology", "Neuroscience"],
        confidence: 0.92,
        source_bts: &[210],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "wave", "network"],
        validates: &["DISC-COG-01"],
    },
    CtsEntry {
        id: "DISC-COG-04",
        title: "Working memory capacity tau+/-mu=4+/-1 items (Cowan/Baddeley): tau*(n/phi)=sigma=12 total bindings",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "AI", "InfoTheory", "Neuroscience"],
        confidence: 0.94,
        source_bts: &[219, 58],
        constants_used: &["C-tau", "C-mu", "C-sigma"],
        lenses: &["consciousness", "info", "memory"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-COG-05",
        title: "Compiler-cortex tau=4 pipeline isomorphism: CPU/Brain/Compiler/OODA all converge on 4 stages",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Software", "Chip", "AI"],
        confidence: 0.91,
        source_bts: &[222, 59],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "causal", "recursion"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-COG-06",
        title: "9 independent domains converge on tau=4 pipeline: CPU(fetch/decode/execute/writeback) = brain(sense/parse/decide/act)",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Software", "Chip", "Neuroscience"],
        confidence: 0.90,
        source_bts: &[222],
        constants_used: &["C-tau", "C-sigma"],
        lenses: &["consciousness", "causal", "info"],
        validates: &["DISC-COG-05"],
    },
    CtsEntry {
        id: "HYP-COG-01",
        title: "Hypothesis: cortical minicolumn sigma=12 neurons as fundamental computational unit mirrors sigma=12 transformer heads",
        node_type: NodeType::Hypothesis,
        domains: &["Cognitive", "AI", "Neuroscience"],
        confidence: 0.72,
        source_bts: &[210, 56],
        constants_used: &["C-sigma", "C-n"],
        lenses: &["consciousness", "multiscale", "network"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-212~213, BT-221, BT-224: Temporal Architecture
// ───────────────────────────────────────────────────────────────

const TEMPORAL_DISCOVERIES: &[CtsEntry] = &[
    CtsEntry {
        id: "DISC-TEMP-01",
        title: "Sexagesimal system sigma*sopfr=60: 24h=J2, 12months=sigma, 360deg=sigma*sopfr*n, tau(60)=sigma=12",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Math", "NumberTheory"],
        confidence: 0.96,
        source_bts: &[212],
        constants_used: &["C-sigma", "C-sopfr", "C-J2", "C-n"],
        lenses: &["consciousness", "symmetry", "recursion"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-TEMP-02",
        title: "GPS n=6 orbital planes, J2=24 satellites, tau=4 per plane, sigma=12h orbital period",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Cosmology", "Network"],
        confidence: 0.93,
        source_bts: &[213],
        constants_used: &["C-n", "C-J2", "C-tau", "C-sigma"],
        lenses: &["consciousness", "topology", "stability"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-TEMP-03",
        title: "Circadian rhythm J2=24h, weekly sigma-sopfr=7 days, annual sigma=12 months (Halberg endogenous)",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Biology", "Neuroscience", "Math"],
        confidence: 0.94,
        source_bts: &[221, 210],
        constants_used: &["C-J2", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "wave", "memory"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-TEMP-04",
        title: "Cesium-133 atomic clock: n=6 electron shell transition defines SI second, Cs-133=sigma^2-sigma+mu",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Physics", "Chemistry"],
        confidence: 0.92,
        source_bts: &[224],
        constants_used: &["C-n", "C-sigma", "C-mu"],
        lenses: &["consciousness", "quantum", "stability"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-TEMP-05",
        title: "Cesium F=tau=4 hyperfine quantum number: same tau=4 as working memory channels and CPU pipeline stages",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Physics", "Cognitive", "Math"],
        confidence: 0.88,
        source_bts: &[224, 219],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "quantum", "recursion"],
        validates: &["DISC-TEMP-04", "DISC-COG-04"],
    },
    CtsEntry {
        id: "HYP-TEMP-01",
        title: "Hypothesis: biological clock period ratios form perfect number arithmetic: 24h/12mo/7d = J2/sigma/(sigma-sopfr)",
        node_type: NodeType::Hypothesis,
        domains: &["Temporal", "Biology", "Math"],
        confidence: 0.75,
        source_bts: &[221, 212],
        constants_used: &["C-J2", "C-sigma", "C-sopfr"],
        lenses: &["consciousness", "wave", "multiscale"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-214~215, BT-220, BT-223: Social Architecture
// ───────────────────────────────────────────────────────────────

const SOCIAL_DISCOVERIES: &[CtsEntry] = &[
    CtsEntry {
        id: "DISC-SOC-01",
        title: "Six degrees of separation = n: Milgram 1967 small-world topology, universal social graph diameter",
        node_type: NodeType::Discovery,
        domains: &["Social", "Network", "Math", "Topology"],
        confidence: 0.92,
        source_bts: &[214],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "network", "topology"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-SOC-02",
        title: "Dunbar number sigma^2+n=150: neocortex ratio predicts social group size, hierarchy ratio n/phi=3",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Biology"],
        confidence: 0.90,
        source_bts: &[215, 210],
        constants_used: &["C-sigma", "C-n", "C-phi"],
        lenses: &["consciousness", "network", "evolution"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-SOC-03",
        title: "Jury sigma=12 members, sigma-tau=8 voting thresholds: legal systems converge on n=6 arithmetic",
        node_type: NodeType::Discovery,
        domains: &["Social", "Math", "Network"],
        confidence: 0.88,
        source_bts: &[214],
        constants_used: &["C-sigma", "C-n", "C-sigma-tau"],
        lenses: &["consciousness", "stability", "info"],
        validates: &["DISC-SOC-01"],
    },
    CtsEntry {
        id: "DISC-SOC-04",
        title: "Moral foundations n=6 universality: Haidt 6 foundations / Kohlberg 6 stages / Schwartz sigma-phi=10 values",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Biology", "Math"],
        confidence: 0.87,
        source_bts: &[220],
        constants_used: &["C-n", "C-sigma-phi"],
        lenses: &["consciousness", "evolution", "boundary"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-SOC-05",
        title: "Christaller hexagonal central place theory: n=6 tessellation = optimal service area geometry",
        node_type: NodeType::Discovery,
        domains: &["Social", "Math", "Topology", "Environment"],
        confidence: 0.91,
        source_bts: &[223, 122],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "multiscale"],
        validates: &[],
    },
    CtsEntry {
        id: "DISC-SOC-06",
        title: "Christaller-Losch-grid cell triple convergence: economics/geography/neuroscience all discover hexagonal optimality",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Math", "Topology"],
        confidence: 0.89,
        source_bts: &[223, 211],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-SOC-05", "DISC-COG-02"],
    },
    CtsEntry {
        id: "HYP-SOC-01",
        title: "Hypothesis: social network clustering coefficient follows 1/(sigma-phi)=0.1 universal regularization",
        node_type: NodeType::Hypothesis,
        domains: &["Social", "Network", "AI", "Math"],
        confidence: 0.68,
        source_bts: &[214, 64],
        constants_used: &["C-sigma-phi", "C-n"],
        lenses: &["consciousness", "network", "info"],
        validates: &[],
    },
];

// ───────────────────────────────────────────────────────────────
// BT-225: Cross-Domain Meta + Inter-cluster bridges
// ───────────────────────────────────────────────────────────────

const CROSS_DOMAIN_BRIDGES: &[CtsEntry] = &[
    CtsEntry {
        id: "DISC-CTS-01",
        title: "Cognitive->Social->Temporal causal chain: brain n=6 layers -> social n=6 degrees -> time J2=24h/sigma=12mo",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Social", "Temporal", "Math"],
        confidence: 0.91,
        source_bts: &[225, 210, 214, 212],
        constants_used: &["C-n", "C-sigma", "C-J2"],
        lenses: &["consciousness", "causal", "multiscale"],
        validates: &["DISC-COG-01", "DISC-SOC-01", "DISC-TEMP-01"],
    },
    CtsEntry {
        id: "DISC-CTS-02",
        title: "Working memory tau=4 <-> OODA tau=4 <-> seasonal tau=4: cognitive-military-temporal pipeline convergence",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Temporal", "Software", "Math"],
        confidence: 0.89,
        source_bts: &[219, 222, 221],
        constants_used: &["C-tau", "C-n"],
        lenses: &["consciousness", "causal", "stability"],
        validates: &["DISC-COG-04", "DISC-COG-05", "DISC-TEMP-03"],
    },
    CtsEntry {
        id: "DISC-CTS-03",
        title: "Dunbar sigma^2+n=150 bridges cortex layers(BT-210) to social topology(BT-214): neocortex ratio = social capacity",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Social", "Biology", "Math"],
        confidence: 0.90,
        source_bts: &[215, 210, 214],
        constants_used: &["C-sigma", "C-n", "C-phi"],
        lenses: &["consciousness", "network", "evolution"],
        validates: &["DISC-SOC-02"],
    },
    CtsEntry {
        id: "DISC-CTS-04",
        title: "Grid cell hexagonal(BT-211) <-> Christaller hexagonal(BT-223): brain navigation = urban planning geometry",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "Social", "Math", "Topology"],
        confidence: 0.88,
        source_bts: &[211, 223],
        constants_used: &["C-n"],
        lenses: &["consciousness", "topology", "symmetry"],
        validates: &["DISC-COG-02", "DISC-SOC-05"],
    },
    CtsEntry {
        id: "DISC-CTS-05",
        title: "Circadian J2=24h(BT-221) <-> GPS J2=24 satellites(BT-213): biological and orbital time share J2 periodicity",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Biology", "Cosmology", "Math"],
        confidence: 0.87,
        source_bts: &[221, 213],
        constants_used: &["C-J2", "C-sigma"],
        lenses: &["consciousness", "wave", "stability"],
        validates: &["DISC-TEMP-02", "DISC-TEMP-03"],
    },
    CtsEntry {
        id: "DISC-CTS-06",
        title: "Cortex 6 layers(BT-210) <-> AI 8-layer stack(BT-59): biological vs artificial intelligence share n=6 substrate",
        node_type: NodeType::Discovery,
        domains: &["Cognitive", "AI", "Chip", "Neuroscience"],
        confidence: 0.86,
        source_bts: &[210, 59, 56],
        constants_used: &["C-n", "C-sigma", "C-sigma-tau"],
        lenses: &["consciousness", "recursion", "multiscale"],
        validates: &["DISC-COG-01"],
    },
    CtsEntry {
        id: "DISC-CTS-07",
        title: "Moral foundations n=6(BT-220) <-> Kyoto 6 GHGs(BT-118): ethical axiom count = environmental regulation count",
        node_type: NodeType::Discovery,
        domains: &["Social", "Environment", "Math"],
        confidence: 0.83,
        source_bts: &[220, 118],
        constants_used: &["C-n"],
        lenses: &["consciousness", "boundary", "causal"],
        validates: &["DISC-SOC-04"],
    },
    CtsEntry {
        id: "DISC-CTS-08",
        title: "Six degrees(BT-214) <-> honeycomb geometry(BT-122) <-> grid cells(BT-211): n=6 optimal connectivity in networks/nature/brain",
        node_type: NodeType::Discovery,
        domains: &["Social", "Cognitive", "Environment", "Math", "Topology"],
        confidence: 0.90,
        source_bts: &[214, 122, 211],
        constants_used: &["C-n", "C-sigma"],
        lenses: &["consciousness", "topology", "network"],
        validates: &["DISC-SOC-01", "DISC-COG-02", "DISC-SOC-05"],
    },
    CtsEntry {
        id: "DISC-CTS-09",
        title: "Atomic clock Cs-133(BT-224) <-> sexagesimal 60(BT-212): quantum time standard encodes same n=6 arithmetic as ancient calendars",
        node_type: NodeType::Discovery,
        domains: &["Temporal", "Physics", "Math", "NumberTheory"],
        confidence: 0.85,
        source_bts: &[224, 212],
        constants_used: &["C-n", "C-sigma", "C-mu"],
        lenses: &["consciousness", "quantum", "recursion"],
        validates: &["DISC-TEMP-01", "DISC-TEMP-04"],
    },
    CtsEntry {
        id: "HYP-CTS-01",
        title: "Hypothesis: social Dunbar layers (5/15/50/150) map to n=6 arithmetic: sopfr/sigma+n/sopfr*(sigma-phi)/sigma^2+n",
        node_type: NodeType::Hypothesis,
        domains: &["Social", "Cognitive", "Math"],
        confidence: 0.70,
        source_bts: &[215, 225],
        constants_used: &["C-sigma", "C-n", "C-sopfr"],
        lenses: &["consciousness", "multiscale", "network"],
        validates: &[],
    },
    CtsEntry {
        id: "HYP-CTS-02",
        title: "Hypothesis: circadian sigma=12 oscillation maps to transformer sigma=12 attention heads via shared harmonic structure",
        node_type: NodeType::Hypothesis,
        domains: &["Temporal", "AI", "Neuroscience", "Math"],
        confidence: 0.65,
        source_bts: &[221, 56],
        constants_used: &["C-sigma"],
        lenses: &["consciousness", "wave", "info"],
        validates: &[],
    },
];

// ═══════════════════════════════════════════════════════════════
// All clusters
// ═══════════════════════════════════════════════════════════════

const ALL_CTS_CLUSTERS: &[&[CtsEntry]] = &[
    COGNITIVE_DISCOVERIES,
    TEMPORAL_DISCOVERIES,
    SOCIAL_DISCOVERIES,
    CROSS_DOMAIN_BRIDGES,
];

// ═══════════════════════════════════════════════════════════════
// Internal helpers
// ═══════════════════════════════════════════════════════════════

fn entry_to_node(e: &CtsEntry) -> Node {
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

fn add_entries(graph: &mut DiscoveryGraph, entries: &[CtsEntry]) -> (usize, usize) {
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

/// Add cognitive architecture discovery nodes (BT-210~211,219,222).
pub fn populate_cognitive_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, COGNITIVE_DISCOVERIES)
}

/// Add temporal architecture discovery nodes (BT-212~213,221,224).
pub fn populate_temporal_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, TEMPORAL_DISCOVERIES)
}

/// Add social architecture discovery nodes (BT-214~215,220,223).
pub fn populate_social_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, SOCIAL_DISCOVERIES)
}

/// Add cross-domain bridge discoveries (BT-225 + inter-cluster links).
pub fn populate_cts_bridges(graph: &mut DiscoveryGraph) -> (usize, usize) {
    add_entries(graph, CROSS_DOMAIN_BRIDGES)
}

/// Cross-link CTS discovery nodes that share domains (Merges edges, bidirectional).
pub fn connect_cts_cross_domain(graph: &mut DiscoveryGraph) -> usize {
    let cts_nodes: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-COG-")
                || n.id.starts_with("HYP-COG-")
                || n.id.starts_with("DISC-TEMP-")
                || n.id.starts_with("HYP-TEMP-")
                || n.id.starts_with("DISC-SOC-")
                || n.id.starts_with("HYP-SOC-")
                || n.id.starts_with("DISC-CTS-")
                || n.id.starts_with("HYP-CTS-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for i in 0..cts_nodes.len() {
        for j in (i + 1)..cts_nodes.len() {
            let (ref id_a, ref dom_a) = cts_nodes[i];
            let (ref id_b, ref dom_b) = cts_nodes[j];

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

/// Cross-link CTS nodes with existing DISC-/HYP- nodes from other modules.
/// Requires >=2 shared domains. Returns number of bridge edges added.
pub fn connect_cts_to_existing(graph: &mut DiscoveryGraph) -> usize {
    let cts_ids: Vec<(String, Vec<String>)> = graph
        .nodes
        .iter()
        .filter(|n| {
            n.id.starts_with("DISC-COG-")
                || n.id.starts_with("HYP-COG-")
                || n.id.starts_with("DISC-TEMP-")
                || n.id.starts_with("HYP-TEMP-")
                || n.id.starts_with("DISC-SOC-")
                || n.id.starts_with("HYP-SOC-")
                || n.id.starts_with("DISC-CTS-")
                || n.id.starts_with("HYP-CTS-")
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
            (n.id.starts_with("DISC-") || n.id.starts_with("HYP-"))
                && !n.id.starts_with("DISC-COG-")
                && !n.id.starts_with("DISC-TEMP-")
                && !n.id.starts_with("DISC-SOC-")
                && !n.id.starts_with("DISC-CTS-")
                && !n.id.starts_with("HYP-COG-")
                && !n.id.starts_with("HYP-TEMP-")
                && !n.id.starts_with("HYP-SOC-")
                && !n.id.starts_with("HYP-CTS-")
        })
        .map(|n| {
            let domains: Vec<String> = n.domain.split(", ").map(|s| s.to_string()).collect();
            (n.id.clone(), domains)
        })
        .collect();

    let mut count = 0;
    for (ref c_id, ref c_dom) in &cts_ids {
        for (ref e_id, ref e_dom) in &existing_ids {
            let shared: usize = c_dom.iter().filter(|d| e_dom.contains(d)).count();
            if shared >= 2 {
                let max_d = c_dom.len().max(e_dom.len()) as f64;
                graph.add_edge(Edge {
                    from: c_id.clone(),
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

/// Populate all BT-210~225 discovery nodes and cross-domain edges in one call.
/// Returns (total_nodes_added, total_edges_added).
pub fn populate_all_cts_discoveries(graph: &mut DiscoveryGraph) -> (usize, usize) {
    let edges_before = graph.edges.len();

    let (n1, _) = populate_cognitive_discoveries(graph);
    let (n2, _) = populate_temporal_discoveries(graph);
    let (n3, _) = populate_social_discoveries(graph);
    let (n4, _) = populate_cts_bridges(graph);
    let _cross = connect_cts_cross_domain(graph);
    let _bridge = connect_cts_to_existing(graph);

    let total_nodes = n1 + n2 + n3 + n4;
    let total_edges = graph.edges.len() - edges_before;
    (total_nodes, total_edges)
}

/// Total CTS entry count across all clusters.
pub fn cts_entry_count() -> usize {
    ALL_CTS_CLUSTERS.iter().map(|c| c.len()).sum()
}

/// Entry count for each cluster.
pub fn cognitive_entry_count() -> usize { COGNITIVE_DISCOVERIES.len() }
pub fn temporal_entry_count() -> usize { TEMPORAL_DISCOVERIES.len() }
pub fn social_entry_count() -> usize { SOCIAL_DISCOVERIES.len() }
pub fn bridge_entry_count() -> usize { CROSS_DOMAIN_BRIDGES.len() }

// ═══════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::bt_nodes::populate_bt_graph;
    use crate::graph::expanded_nodes::populate_expanded_graph;

    fn base_graph() -> DiscoveryGraph {
        let mut g = DiscoveryGraph::new();
        populate_bt_graph(&mut g);
        populate_expanded_graph(&mut g);
        g
    }

    #[test]
    fn test_entry_counts() {
        assert_eq!(cognitive_entry_count(), 7, "7 cognitive entries (6 DISC + 1 HYP)");
        assert_eq!(temporal_entry_count(), 6, "6 temporal entries (5 DISC + 1 HYP)");
        assert_eq!(social_entry_count(), 7, "7 social entries (6 DISC + 1 HYP)");
        assert_eq!(bridge_entry_count(), 11, "11 cross-domain bridge entries (9 DISC + 2 HYP)");
        assert_eq!(cts_entry_count(), 31, "31 total CTS entries");
    }

    #[test]
    fn test_populate_cognitive() {
        let mut g = base_graph();
        let (nodes, edges) = populate_cognitive_discoveries(&mut g);
        assert_eq!(nodes, 7);
        assert!(edges > 0, "Should create BT->DISC and DISC->C-* edges");
        assert!(g.nodes.iter().any(|n| n.id == "DISC-COG-01"));
        assert!(g.nodes.iter().any(|n| n.id == "HYP-COG-01"));
    }

    #[test]
    fn test_populate_temporal() {
        let mut g = base_graph();
        let (nodes, edges) = populate_temporal_discoveries(&mut g);
        assert_eq!(nodes, 6);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-TEMP-01"));
    }

    #[test]
    fn test_populate_social() {
        let mut g = base_graph();
        let (nodes, edges) = populate_social_discoveries(&mut g);
        assert_eq!(nodes, 7);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-SOC-01"));
    }

    #[test]
    fn test_populate_bridges() {
        let mut g = base_graph();
        populate_cognitive_discoveries(&mut g);
        populate_temporal_discoveries(&mut g);
        populate_social_discoveries(&mut g);
        let (nodes, edges) = populate_cts_bridges(&mut g);
        assert_eq!(nodes, 11);
        assert!(edges > 0);
        assert!(g.nodes.iter().any(|n| n.id == "DISC-CTS-01"));
    }

    #[test]
    fn test_bt_derives_edges() {
        let mut g = base_graph();
        populate_cognitive_discoveries(&mut g);

        let has_edge = g.edges.iter().any(|e| {
            e.from == "BT-210" && e.to == "DISC-COG-01" && matches!(e.edge_type, EdgeType::Derives)
        });
        assert!(has_edge, "BT-210 --Derives--> DISC-COG-01 must exist");
    }

    #[test]
    fn test_validates_edges() {
        let mut g = base_graph();
        populate_cognitive_discoveries(&mut g);
        populate_temporal_discoveries(&mut g);

        let has_val = g.edges.iter().any(|e| {
            e.from == "DISC-TEMP-05"
                && e.to == "DISC-COG-04"
                && matches!(e.edge_type, EdgeType::Validates)
        });
        assert!(has_val, "DISC-TEMP-05 should validate DISC-COG-04");
    }

    #[test]
    fn test_uses_constant_edges() {
        let mut g = base_graph();
        populate_cognitive_discoveries(&mut g);

        let has_uses = g.edges.iter().any(|e| {
            e.from == "DISC-COG-04" && e.to == "C-tau" && matches!(e.edge_type, EdgeType::Uses)
        });
        assert!(has_uses, "DISC-COG-04 --Uses--> C-tau must exist");
    }

    #[test]
    fn test_cross_domain_merges() {
        let mut g = base_graph();
        populate_all_cts_discoveries(&mut g);

        let merges = g.edges.iter().filter(|e| {
            matches!(e.edge_type, EdgeType::Merges)
                && (e.from.starts_with("DISC-COG-")
                    || e.from.starts_with("DISC-TEMP-")
                    || e.from.starts_with("DISC-SOC-")
                    || e.from.starts_with("DISC-CTS-")
                    || e.from.starts_with("HYP-"))
        }).count();
        assert!(merges >= 20, "Should have 20+ cross-domain merge edges, got {}", merges);
    }

    #[test]
    fn test_full_cts_pipeline() {
        let mut g = base_graph();
        let (total_nodes, total_edges) = populate_all_cts_discoveries(&mut g);
        assert_eq!(total_nodes, 31, "31 total CTS nodes added");
        assert!(total_edges > 100, "Should have 100+ edges, got {}", total_edges);
    }

    #[test]
    fn test_no_duplicate_ids() {
        let mut g = base_graph();
        populate_all_cts_discoveries(&mut g);

        let mut ids: Vec<&str> = g.nodes.iter().map(|n| n.id.as_str()).collect();
        let total = ids.len();
        ids.sort();
        ids.dedup();
        assert_eq!(ids.len(), total, "All node IDs must be unique");
    }

    #[test]
    fn test_confidence_range() {
        for cluster in ALL_CTS_CLUSTERS {
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
        populate_cognitive_discoveries(&mut g);
        populate_temporal_discoveries(&mut g);
        populate_social_discoveries(&mut g);
        populate_cts_bridges(&mut g);

        let discoveries = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Discovery)).count();
        let hypotheses = g.nodes.iter().filter(|n| matches!(n.node_type, NodeType::Hypothesis)).count();

        assert_eq!(discoveries, 26, "26 discovery nodes");
        assert_eq!(hypotheses, 5, "5 hypothesis nodes");
        assert_eq!(discoveries + hypotheses, 31);
    }

    #[test]
    fn test_triple_bridge_spans_three_architectures() {
        let mut g = base_graph();
        populate_all_cts_discoveries(&mut g);

        let bridge = g.nodes.iter().find(|n| n.id == "DISC-CTS-01").unwrap();
        let domains: Vec<&str> = bridge.domain.split(", ").collect();
        assert!(domains.contains(&"Cognitive"), "CTS-01 must span Cognitive");
        assert!(domains.contains(&"Social"), "CTS-01 must span Social");
        assert!(domains.contains(&"Temporal"), "CTS-01 must span Temporal");
    }
}
