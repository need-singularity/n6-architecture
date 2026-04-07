use regex::Regex;
use serde::Serialize;
use std::collections::{BTreeSet, HashMap, HashSet};
use std::env;
use std::fs;
use std::path::Path;

// ---------------------------------------------------------------------------
// Data model
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum NodeKind {
    Hypothesis,
    Constant,
    Domain,
}

#[derive(Debug, Clone, Serialize)]
struct Node {
    id: String,
    kind: NodeKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    grade: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error_pct: Option<f64>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
enum EdgeKind {
    Uses,
    Spans,
    CoOccurs,
}

#[derive(Debug, Clone, Serialize)]
struct Edge {
    source: String,
    target: String,
    kind: EdgeKind,
}

#[derive(Debug, Clone, Serialize)]
struct GraphStats {
    total_nodes: usize,
    total_edges: usize,
    hypothesis_count: usize,
    constant_count: usize,
    domain_count: usize,
    degree_min: usize,
    degree_max: usize,
    degree_avg: f64,
    degree_median: f64,
    clustering_coefficient: f64,
    connected_components: usize,
    hub_nodes: Vec<(String, usize)>,
    isolated_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<Edge>,
    stats: GraphStats,
    suggestions: Suggestions,
}

#[derive(Debug, Clone, Serialize)]
struct Suggestions {
    bridge_targets: Vec<BridgeTarget>,
    high_cooccurrence: Vec<CoOccurrence>,
    low_connectivity: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct BridgeTarget {
    domain_a: String,
    domain_b: String,
    bridging_constants: Vec<String>,
}

#[derive(Debug, Clone, Serialize)]
struct CoOccurrence {
    constant_a: String,
    constant_b: String,
    count: usize,
}

// ---------------------------------------------------------------------------
// Constants and domains
// ---------------------------------------------------------------------------

const BASE_CONSTANTS: &[&str] = &["sigma", "tau", "phi", "n", "sopfr", "J2", "mu"];

const DERIVED_CONSTANTS: &[(&str, &str)] = &[
    ("sigma-tau", "sigma-tau=8"),
    ("sigma/tau", "sigma/tau=3"),
    ("sigma*phi", "sigma*phi=24"),
    ("sigma*tau", "sigma*tau=48"),
    ("sigma*n", "sigma*n=72"),
    ("phi^sigma", "phi^sigma=4096"),
    ("sigma^2", "sigma^2=144"),
    ("tau^2", "tau^2=16"),
];

const DOMAINS: &[&str] = &[
    "particle",
    "cosmology",
    "seti",
    "nuclear",
    "qcd",
    "electroweak",
    "neutrino",
    "exoplanet",
    "consciousness",
    "biology",
    "mathematics",
];

fn domain_keywords(domain: &str) -> Vec<&'static str> {
    match domain {
        "particle" => vec![
            "quark", "lepton", "fermion", "boson", "higgs", "muon", "tau particle",
            "standard model", "gauge", "generation", "mass",
        ],
        "cosmology" => vec![
            "cmb", "cosmic", "planck", "hubble", "dark matter", "dark energy",
            "inflation", "cosmolog", "tensor-to-scalar", "spectral index",
            "baryon", "baryogenesis",
        ],
        "seti" => vec![
            "seti", "wow signal", "breakthrough listen", "voyager",
            "extraterrestrial", "alien", "drake", "signal",
        ],
        "nuclear" => vec![
            "nuclear", "binding energy", "fission", "fusion", "isotope",
            "neutron", "proton mass",
        ],
        "qcd" => vec![
            "qcd", "gluon", "color charge", "confinement", "asymptotic",
            "meson", "rho", "j/psi", "upsilon", "pion", "resonance",
        ],
        "electroweak" => vec![
            "electroweak", "weinberg", "weak mixing", "w boson", "z boson",
            "sin2", "theta_w", "alpha_em", "fine structure",
        ],
        "neutrino" => vec![
            "neutrino", "oscillation", "pmns", "mixing angle", "mass splitting",
        ],
        "exoplanet" => vec![
            "exoplanet", "habitable", "kepler", "transit", "radial velocity",
            "planet", "orbit",
        ],
        "consciousness" => vec![
            "consciousness", "conscious", "anima", "phi_max", "awareness",
            "cognit", "neural", "brain", "eeg", "psyche", "psi",
        ],
        "biology" => vec![
            "biology", "biological", "dna", "codon", "amino acid", "protein",
            "cell", "mitosis", "genetic",
        ],
        "mathematics" => vec![
            "theorem", "proof", "conjecture", "modular", "lattice",
            "lie algebra", "moonshine", "ramanujan", "riemann",
            "elliptic", "polynomial", "topology", "bott",
        ],
        _ => vec![],
    }
}

fn category_to_domains(cat: &str) -> Vec<&'static str> {
    match cat {
        "CX" => vec!["particle", "cosmology", "mathematics"],
        "CS" => vec!["consciousness"],
        "CA" => vec!["consciousness", "biology"],
        "AF" => vec!["seti", "exoplanet", "consciousness"],
        "CERN" => vec!["particle", "qcd", "electroweak"],
        _ => vec![],
    }
}

// ---------------------------------------------------------------------------
// Hypothesis parser
// ---------------------------------------------------------------------------

#[derive(Debug)]
struct ParsedHypothesis {
    id: String,
    category: String,
    grade: String,
    error_pcts: Vec<f64>,
    constants_used: HashSet<String>,
    domains_matched: HashSet<String>,
}

fn parse_hypothesis(path: &Path) -> Option<ParsedHypothesis> {
    let filename = path.file_stem()?.to_str()?;
    let content = fs::read_to_string(path).ok()?;
    let content_lower = content.to_lowercase();

    // Extract ID from filename: H-CX-1000-..., H-CA-001-..., etc.
    let id = filename
        .split('-')
        .take(3)
        .collect::<Vec<_>>()
        .join("-");

    // Extract category
    let category = filename
        .split('-')
        .nth(1)
        .unwrap_or("XX")
        .to_string();

    // Extract grade
    let grade_re = Regex::new(r"(?m)^##\s+Grade:\s*(.+)$").unwrap();
    let grade = grade_re
        .captures(&content)
        .map(|c| c[1].trim().to_string())
        .unwrap_or_else(|| "UNKNOWN".to_string());

    // Extract error percentages
    let err_re = Regex::new(r"(\d+\.?\d*)\s*%").unwrap();
    let error_pcts: Vec<f64> = err_re
        .captures_iter(&content)
        .filter_map(|c| c[1].parse::<f64>().ok())
        .filter(|&v| v > 0.0 && v < 100.0) // reasonable error range
        .collect();

    // Detect constants used
    let mut constants_used = HashSet::new();

    // Base constants — match carefully to avoid false positives
    let const_patterns: &[(&str, &str)] = &[
        (r"\bsigma\b|σ\(?6\)?|σ\b", "sigma"),
        (r"\btau\b|τ\(?6\)?|τ\b", "tau"),
        (r"\bphi\b|φ\(?6\)?|φ\b|totient", "phi"),
        (r"\bsopfr\b|prime\s+sum", "sopfr"),
        (r"\bn\s*=\s*6\b|P[_₁]1?\s*=\s*6|first\s+perfect", "n"),
        (r"\bmu\b|μ\b|möbius|mobius", "mu"),
        (r"J[_₂]2?", "J2"),
    ];
    for (pat, name) in const_patterns {
        let re = Regex::new(pat).unwrap();
        if re.is_match(&content) {
            constants_used.insert(name.to_string());
        }
    }

    // Derived constants
    let derived_patterns: &[(&str, &str)] = &[
        (r"sigma\s*-\s*tau|σ\s*-\s*τ|=\s*8\s*(gluon|color|SU\(3\))", "sigma-tau"),
        (r"sigma\s*/\s*tau|σ\s*/\s*τ|=\s*3\s*(generation|color|SU\(2\))", "sigma/tau"),
        (r"sigma\s*\*?\s*phi|σ\s*[·*]\s*φ|=\s*24\s*(fermion|total)", "sigma*phi"),
        (r"sigma\s*\*?\s*tau|σ\s*[·*]\s*τ|=\s*48", "sigma*tau"),
        (r"sigma\s*\*?\s*n|σ\s*[·*]\s*n|=\s*72", "sigma*n"),
        (r"phi\^\s*sigma|φ\^\s*σ|2\^12|=\s*4096", "phi^sigma"),
        (r"sigma\^2|σ²|=\s*144", "sigma^2"),
    ];
    for (pat, name) in derived_patterns {
        let re = Regex::new(pat).unwrap();
        if re.is_match(&content) {
            constants_used.insert(name.to_string());
        }
    }

    // Detect domains via keywords
    let mut domains_matched = HashSet::new();

    // Category-based defaults
    for d in category_to_domains(&category) {
        domains_matched.insert(d.to_string());
    }

    // Keyword-based matching
    for &domain in DOMAINS {
        for kw in domain_keywords(domain) {
            if content_lower.contains(kw) {
                domains_matched.insert(domain.to_string());
                break;
            }
        }
    }

    Some(ParsedHypothesis {
        id,
        category,
        grade,
        error_pcts,
        constants_used,
        domains_matched,
    })
}

// ---------------------------------------------------------------------------
// Graph builder
// ---------------------------------------------------------------------------

fn build_graph(hyp_dir: &Path) -> Graph {
    // 1. Parse all hypothesis files
    let mut hypotheses = Vec::new();
    if let Ok(entries) = fs::read_dir(hyp_dir) {
        let mut paths: Vec<_> = entries
            .filter_map(|e| e.ok())
            .map(|e| e.path())
            .filter(|p| {
                p.extension().map_or(false, |ext| ext == "md")
                    && p.file_name()
                        .map_or(false, |n| n.to_str().map_or(false, |s| s.starts_with("H-")))
            })
            .collect();
        paths.sort();
        for path in &paths {
            if let Some(h) = parse_hypothesis(path) {
                hypotheses.push(h);
            }
        }
    }

    // 2. Build nodes
    let mut nodes = Vec::new();
    let mut node_ids: HashSet<String> = HashSet::new();

    // Hypothesis nodes
    for h in &hypotheses {
        let min_err = h.error_pcts.iter().cloned().fold(f64::MAX, f64::min);
        nodes.push(Node {
            id: h.id.clone(),
            kind: NodeKind::Hypothesis,
            grade: Some(h.grade.clone()),
            category: Some(h.category.clone()),
            error_pct: if min_err < f64::MAX { Some(min_err) } else { None },
        });
        node_ids.insert(h.id.clone());
    }

    // Constant nodes
    for &c in BASE_CONSTANTS {
        nodes.push(Node {
            id: c.to_string(),
            kind: NodeKind::Constant,
            grade: None,
            category: None,
            error_pct: None,
        });
        node_ids.insert(c.to_string());
    }
    for &(name, _) in DERIVED_CONSTANTS {
        nodes.push(Node {
            id: name.to_string(),
            kind: NodeKind::Constant,
            grade: None,
            category: None,
            error_pct: None,
        });
        node_ids.insert(name.to_string());
    }

    // Domain nodes
    for &d in DOMAINS {
        nodes.push(Node {
            id: d.to_string(),
            kind: NodeKind::Domain,
            grade: None,
            category: None,
            error_pct: None,
        });
        node_ids.insert(d.to_string());
    }

    // 3. Build edges
    let mut edges = Vec::new();
    let mut edge_set: HashSet<(String, String, EdgeKind)> = HashSet::new();

    let add_edge = |src: &str, tgt: &str, kind: EdgeKind, edges: &mut Vec<Edge>, edge_set: &mut HashSet<(String, String, EdgeKind)>| {
        let key = (src.to_string(), tgt.to_string(), kind);
        if edge_set.insert(key) {
            edges.push(Edge {
                source: src.to_string(),
                target: tgt.to_string(),
                kind,
            });
        }
    };

    // Track constant co-occurrence per hypothesis
    let mut cooccur_count: HashMap<(String, String), usize> = HashMap::new();

    for h in &hypotheses {
        // USES edges: hypothesis -> constant
        for c in &h.constants_used {
            if node_ids.contains(c.as_str()) {
                add_edge(&h.id, c, EdgeKind::Uses, &mut edges, &mut edge_set);
            }
        }

        // SPANS edges: hypothesis -> domain
        for d in &h.domains_matched {
            if node_ids.contains(d.as_str()) {
                add_edge(&h.id, d, EdgeKind::Spans, &mut edges, &mut edge_set);
            }
        }

        // CO_OCCURS: track pairs
        let consts: Vec<&String> = h.constants_used.iter().collect();
        for i in 0..consts.len() {
            for j in (i + 1)..consts.len() {
                let (a, b) = if consts[i] < consts[j] {
                    (consts[i].clone(), consts[j].clone())
                } else {
                    (consts[j].clone(), consts[i].clone())
                };
                *cooccur_count.entry((a, b)).or_insert(0) += 1;
            }
        }
    }

    // Add CO_OCCURS edges (threshold >= 3 to avoid clutter)
    for ((a, b), count) in &cooccur_count {
        if *count >= 3 {
            add_edge(a, b, EdgeKind::CoOccurs, &mut edges, &mut edge_set);
        }
    }

    // 4. Compute stats
    let stats = compute_stats(&nodes, &edges);

    // 5. Compute suggestions
    let suggestions = compute_suggestions(&hypotheses, &cooccur_count, &nodes, &edges);

    Graph {
        nodes,
        edges,
        stats,
        suggestions,
    }
}

// ---------------------------------------------------------------------------
// Statistics
// ---------------------------------------------------------------------------

fn compute_stats(nodes: &[Node], edges: &[Edge]) -> GraphStats {
    let n = nodes.len();

    // Build degree map
    let mut degree: HashMap<&str, usize> = HashMap::new();
    for node in nodes {
        degree.entry(node.id.as_str()).or_insert(0);
    }
    for e in edges {
        *degree.entry(e.source.as_str()).or_insert(0) += 1;
        *degree.entry(e.target.as_str()).or_insert(0) += 1;
    }

    let mut degrees: Vec<usize> = degree.values().copied().collect();
    degrees.sort();

    let deg_min = degrees.first().copied().unwrap_or(0);
    let deg_max = degrees.last().copied().unwrap_or(0);
    let deg_sum: usize = degrees.iter().sum();
    let deg_avg = if n > 0 { deg_sum as f64 / n as f64 } else { 0.0 };
    let deg_median = if degrees.is_empty() {
        0.0
    } else if degrees.len() % 2 == 0 {
        (degrees[degrees.len() / 2 - 1] + degrees[degrees.len() / 2]) as f64 / 2.0
    } else {
        degrees[degrees.len() / 2] as f64
    };

    // Hub nodes (top 10 by degree)
    let mut deg_vec: Vec<(&str, usize)> = degree.iter().map(|(&k, &v)| (k, v)).collect();
    deg_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let hub_nodes: Vec<(String, usize)> = deg_vec
        .iter()
        .take(10)
        .map(|(k, v)| (k.to_string(), *v))
        .collect();

    // Isolated nodes (degree <= 1)
    let isolated: Vec<String> = deg_vec
        .iter()
        .filter(|(_, d)| *d <= 1)
        .map(|(k, _)| k.to_string())
        .collect();

    // Adjacency list for clustering + components
    let mut adj: HashMap<&str, HashSet<&str>> = HashMap::new();
    for e in edges {
        adj.entry(e.source.as_str())
            .or_default()
            .insert(e.target.as_str());
        adj.entry(e.target.as_str())
            .or_default()
            .insert(e.source.as_str());
    }

    // Clustering coefficient (global approximation)
    // For each node with degree >= 2, count triangles
    let mut total_triplets = 0u64;
    let mut closed_triplets = 0u64;
    for node in nodes {
        let neighbors = match adj.get(node.id.as_str()) {
            Some(n) => n,
            None => continue,
        };
        let deg = neighbors.len();
        if deg < 2 {
            continue;
        }
        let neighbor_vec: Vec<&str> = neighbors.iter().copied().collect();
        // Sample for large neighborhoods
        let limit = neighbor_vec.len().min(50);
        for i in 0..limit {
            for j in (i + 1)..limit {
                total_triplets += 1;
                if adj
                    .get(neighbor_vec[i])
                    .map_or(false, |s| s.contains(neighbor_vec[j]))
                {
                    closed_triplets += 1;
                }
            }
        }
    }
    let clustering = if total_triplets > 0 {
        closed_triplets as f64 / total_triplets as f64
    } else {
        0.0
    };

    // Connected components (BFS)
    let mut visited: HashSet<&str> = HashSet::new();
    let mut components = 0usize;
    for node in nodes {
        if visited.contains(node.id.as_str()) {
            continue;
        }
        components += 1;
        let mut queue = vec![node.id.as_str()];
        visited.insert(node.id.as_str());
        while let Some(cur) = queue.pop() {
            if let Some(neighbors) = adj.get(cur) {
                for &nb in neighbors {
                    if visited.insert(nb) {
                        queue.push(nb);
                    }
                }
            }
        }
    }

    let hyp_count = nodes.iter().filter(|n| n.kind == NodeKind::Hypothesis).count();
    let const_count = nodes.iter().filter(|n| n.kind == NodeKind::Constant).count();
    let dom_count = nodes.iter().filter(|n| n.kind == NodeKind::Domain).count();

    GraphStats {
        total_nodes: n,
        total_edges: edges.len(),
        hypothesis_count: hyp_count,
        constant_count: const_count,
        domain_count: dom_count,
        degree_min: deg_min,
        degree_max: deg_max,
        degree_avg: deg_avg,
        degree_median: deg_median,
        clustering_coefficient: clustering,
        connected_components: components,
        hub_nodes,
        isolated_nodes: isolated,
    }
}

// ---------------------------------------------------------------------------
// Discovery suggestions
// ---------------------------------------------------------------------------

fn compute_suggestions(
    hypotheses: &[ParsedHypothesis],
    cooccur_count: &HashMap<(String, String), usize>,
    nodes: &[Node],
    edges: &[Edge],
) -> Suggestions {
    // 1. Bridge targets: domain pairs connected only through constants
    let mut domain_direct: HashSet<(String, String)> = HashSet::new();
    let mut domain_via_const: HashMap<(String, String), BTreeSet<String>> = HashMap::new();

    for h in hypotheses {
        let doms: Vec<&String> = h.domains_matched.iter().collect();
        // Direct domain connections through same hypothesis
        for i in 0..doms.len() {
            for j in (i + 1)..doms.len() {
                let (a, b) = if doms[i] < doms[j] {
                    (doms[i].clone(), doms[j].clone())
                } else {
                    (doms[j].clone(), doms[i].clone())
                };
                domain_direct.insert((a, b));
            }
        }
        // Domains connected via constants
        for d in &h.domains_matched {
            for c in &h.constants_used {
                // Find other domains that use the same constant
                for h2 in hypotheses {
                    if h2.id == h.id {
                        continue;
                    }
                    if h2.constants_used.contains(c) {
                        for d2 in &h2.domains_matched {
                            if d != d2 {
                                let (a, b) = if d < d2 {
                                    (d.clone(), d2.clone())
                                } else {
                                    (d2.clone(), d.clone())
                                };
                                domain_via_const
                                    .entry((a, b))
                                    .or_default()
                                    .insert(c.clone());
                            }
                        }
                    }
                }
            }
        }
    }

    // Bridge targets: connected via constants but not directly through many shared hypotheses
    let mut bridge_targets = Vec::new();
    for ((a, b), consts) in &domain_via_const {
        // Only include if this is a sparse connection
        if consts.len() <= 3 {
            bridge_targets.push(BridgeTarget {
                domain_a: a.clone(),
                domain_b: b.clone(),
                bridging_constants: consts.iter().cloned().collect(),
            });
        }
    }
    bridge_targets.sort_by(|a, b| a.bridging_constants.len().cmp(&b.bridging_constants.len()));
    bridge_targets.truncate(10);

    // 2. High co-occurrence constants
    let mut cooccur_vec: Vec<CoOccurrence> = cooccur_count
        .iter()
        .map(|((a, b), &count)| CoOccurrence {
            constant_a: a.clone(),
            constant_b: b.clone(),
            count,
        })
        .collect();
    cooccur_vec.sort_by(|a, b| b.count.cmp(&a.count));
    cooccur_vec.truncate(10);

    // 3. Low connectivity hypotheses
    let mut hyp_edge_count: HashMap<&str, usize> = HashMap::new();
    for e in edges {
        for node in nodes {
            if node.kind == NodeKind::Hypothesis {
                if e.source == node.id || e.target == node.id {
                    *hyp_edge_count.entry(node.id.as_str()).or_insert(0) += 1;
                }
            }
        }
    }
    // All hypothesis IDs
    let mut low_conn: Vec<(String, usize)> = hypotheses
        .iter()
        .map(|h| {
            let count = hyp_edge_count.get(h.id.as_str()).copied().unwrap_or(0);
            (h.id.clone(), count)
        })
        .collect();
    low_conn.sort_by_key(|(_, c)| *c);
    let low_connectivity: Vec<String> = low_conn
        .iter()
        .take(10)
        .map(|(id, _)| id.clone())
        .collect();

    Suggestions {
        bridge_targets,
        high_cooccurrence: cooccur_vec,
        low_connectivity,
    }
}

// ---------------------------------------------------------------------------
// Output formatters
// ---------------------------------------------------------------------------

fn print_stats(graph: &Graph) {
    let s = &graph.stats;
    println!("=== SEDI Discovery Graph Statistics ===");
    println!();
    println!("Nodes:  {} total", s.total_nodes);
    println!("  Hypotheses: {}", s.hypothesis_count);
    println!("  Constants:  {}", s.constant_count);
    println!("  Domains:    {}", s.domain_count);
    println!();
    println!("Edges:  {} total", s.total_edges);
    println!();
    println!("Degree distribution:");
    println!("  min:    {}", s.degree_min);
    println!("  max:    {}", s.degree_max);
    println!("  avg:    {:.2}", s.degree_avg);
    println!("  median: {:.1}", s.degree_median);
    println!();
    println!("Clustering coefficient: {:.4}", s.clustering_coefficient);
    println!("Connected components:   {}", s.connected_components);
    println!();
    println!("Hub nodes (top 10 by degree):");
    for (i, (name, deg)) in s.hub_nodes.iter().enumerate() {
        println!("  {:2}. {} (degree {})", i + 1, name, deg);
    }
    println!();
    println!("Isolated nodes (degree <= 1): {}", s.isolated_nodes.len());
    if !s.isolated_nodes.is_empty() {
        for n in s.isolated_nodes.iter().take(20) {
            println!("  - {}", n);
        }
        if s.isolated_nodes.len() > 20 {
            println!("  ... and {} more", s.isolated_nodes.len() - 20);
        }
    }

    // Suggestions
    println!();
    println!("=== Discovery Suggestions ===");
    println!();

    println!("Bridge targets (domain pairs connected only through few constants):");
    if graph.suggestions.bridge_targets.is_empty() {
        println!("  (none found)");
    }
    for bt in &graph.suggestions.bridge_targets {
        println!(
            "  {} <-> {} via [{}]",
            bt.domain_a,
            bt.domain_b,
            bt.bridging_constants.join(", ")
        );
    }

    println!();
    println!("High co-occurrence constant pairs:");
    for co in &graph.suggestions.high_cooccurrence {
        println!(
            "  {} + {} (co-occur in {} hypotheses)",
            co.constant_a, co.constant_b, co.count
        );
    }

    println!();
    println!("Low connectivity hypotheses (candidates for more testing):");
    for h in &graph.suggestions.low_connectivity {
        println!("  - {}", h);
    }
}

fn print_dot(graph: &Graph) {
    println!("digraph SEDI {{");
    println!("  rankdir=LR;");
    println!("  node [fontname=\"Helvetica\"];");
    println!("  edge [fontname=\"Helvetica\", fontsize=9];");
    println!();

    // Style by node kind
    for node in &graph.nodes {
        let (shape, color) = match node.kind {
            NodeKind::Hypothesis => ("ellipse", "#4a90d9"),
            NodeKind::Constant => ("diamond", "#e8a838"),
            NodeKind::Domain => ("box", "#50c878"),
        };
        let label = &node.id;
        println!(
            "  \"{}\" [shape={}, style=filled, fillcolor=\"{}\", label=\"{}\"];",
            node.id, shape, color, label
        );
    }

    println!();
    for edge in &graph.edges {
        let style = match edge.kind {
            EdgeKind::Uses => "solid",
            EdgeKind::Spans => "dashed",
            EdgeKind::CoOccurs => "dotted",
        };
        let label = match edge.kind {
            EdgeKind::Uses => "uses",
            EdgeKind::Spans => "spans",
            EdgeKind::CoOccurs => "co-occurs",
        };
        println!(
            "  \"{}\" -> \"{}\" [style={}, label=\"{}\"];",
            edge.source, edge.target, style, label
        );
    }

    println!("}}");
}

fn print_json(graph: &Graph) {
    println!("{}", serde_json::to_string_pretty(graph).unwrap());
}

// ---------------------------------------------------------------------------
// Main
// ---------------------------------------------------------------------------

fn main() {
    let args: Vec<String> = env::args().collect();

    // Determine hypothesis directory
    let default_dir = find_hypotheses_dir();
    let hyp_dir = args
        .iter()
        .position(|a| a == "--dir")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or(&default_dir);

    let path = Path::new(hyp_dir);
    if !path.is_dir() {
        eprintln!("Error: hypothesis directory not found: {}", hyp_dir);
        eprintln!("Use --dir <path> to specify the hypotheses directory.");
        std::process::exit(1);
    }

    eprintln!("Scanning hypotheses in: {}", hyp_dir);
    let graph = build_graph(path);
    eprintln!(
        "Parsed {} hypotheses, built {} nodes and {} edges",
        graph.stats.hypothesis_count, graph.stats.total_nodes, graph.stats.total_edges
    );

    // Output mode
    let mode = if args.contains(&"--json".to_string()) {
        "json"
    } else if args.contains(&"--dot".to_string()) {
        "dot"
    } else {
        "stats"
    };

    match mode {
        "json" => print_json(&graph),
        "dot" => print_dot(&graph),
        _ => print_stats(&graph),
    }
}

fn find_hypotheses_dir() -> String {
    // Try common locations relative to the binary or current dir
    let candidates = [
        "docs/hypotheses",
        "../docs/hypotheses",
        "../../docs/hypotheses",
    ];
    for c in &candidates {
        if Path::new(c).is_dir() {
            return c.to_string();
        }
    }
    // Absolute fallback
    let home = env::var("HOME").unwrap_or_else(|_| "/Users/ghost".to_string());
    let abs = format!("{}/Dev/sedi/docs/hypotheses", home);
    if Path::new(&abs).is_dir() {
        return abs;
    }
    "docs/hypotheses".to_string()
}
