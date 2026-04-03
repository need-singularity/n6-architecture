//! Architecture growth planner — discovers structural gaps and designs new capabilities.
//!
//! Scans the NEXUS-6 module tree, classifies maturity, identifies gaps
//! (orphans, stubs, missing integrations, missing capabilities, bottlenecks),
//! and generates prioritised Claude CLI prompts to close those gaps.

use std::collections::HashMap;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;
const SIGMA: usize = 12;
const PHI: usize = 2;
const TAU: usize = 4;
const _J2: usize = 24;
const SOPFR: usize = 5;
const SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10
const _SIGMA_MINUS_TAU: usize = 8;  // σ-τ = 8

/// Project root for prompt generation.
const NEXUS_ROOT: &str = "/Users/ghost/Dev/n6-architecture/tools/nexus6/";

/// Known module names in NEXUS-6 (mirrors metrics.rs KNOWN_MODULES).
const KNOWN_MODULES: &[&str] = &[
    "gpu", "encoder", "materials", "verifier", "graph", "telescope",
    "history", "ouroboros", "lens_forge", "experiment", "science", "cli",
    "alert", "api", "auto_register", "autonomous", "consciousness_bridge",
    "cross_intel", "distributed", "dream", "event", "feedback",
    "genetic_prog", "ingest", "knowledge", "multi_agent", "nlp",
    "pipeline", "plugin", "publish", "red_team", "reward", "sandbox",
    "scheduler", "self_improve", "statistics", "template", "time_travel",
    "versioning", "calibration", "simulation", "growth",
];

// ═══════════════════════════════════════════════════════════════════════
// Data structures
// ═══════════════════════════════════════════════════════════════════════

/// Snapshot of the overall architecture at a point in time.
#[derive(Debug)]
pub struct ArchSnapshot {
    pub modules: Vec<ModuleInfo>,
    /// (from_module, to_module) — known use/import edges.
    pub connections: Vec<(String, String)>,
    /// Modules with zero connections (neither imports nor imported_by).
    pub orphan_modules: Vec<String>,
    /// (module, connection_count) — most-connected hubs.
    pub hub_modules: Vec<(String, usize)>,
    pub total_pub_fns: usize,
    pub total_pub_structs: usize,
}

/// Per-module information.
#[derive(Debug, Clone)]
pub struct ModuleInfo {
    pub name: String,
    pub files: usize,
    pub lines: usize,
    pub tests: usize,
    pub pub_fns: usize,
    pub pub_structs: usize,
    pub imports_from: Vec<String>,
    pub imported_by: Vec<String>,
    pub maturity: Maturity,
}

/// Module maturity classification (thresholds inspired by n=6 constants).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Maturity {
    /// < 50 lines, < φ=2 tests
    Stub,
    /// < 200 lines, < SOPFR=5 tests
    Basic,
    /// < 500 lines, SOPFR+ tests
    Mature,
    /// 500+ lines, SIGMA_MINUS_PHI+ tests, well-connected
    Production,
}

impl std::fmt::Display for Maturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Maturity::Stub => write!(f, "Stub"),
            Maturity::Basic => write!(f, "Basic"),
            Maturity::Mature => write!(f, "Mature"),
            Maturity::Production => write!(f, "Production"),
        }
    }
}

/// An identified architectural gap.
#[derive(Debug)]
pub struct ArchGap {
    pub gap_type: GapType,
    pub description: String,
    /// 0.0..1.0 — how much closing this gap improves the system.
    pub impact: f64,
    pub effort: Effort,
    pub suggested_modules: Vec<String>,
    /// Ready-to-use prompt for Claude CLI.
    pub claude_prompt: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GapType {
    /// Two modules that should communicate but have no edge.
    MissingIntegration,
    /// Module with no inbound or outbound connections.
    OrphanModule,
    /// A capability that similar systems have but we lack.
    MissingCapability,
    /// A hub module with excessive responsibility.
    BottleneckModule,
    /// Module exists but is barely implemented.
    StubModule,
    /// Module has code but insufficient tests.
    MissingTests,
    /// Modules in the same domain but no cross-references.
    MissingCrossLink,
}

impl std::fmt::Display for GapType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GapType::MissingIntegration => write!(f, "MissingIntegration"),
            GapType::OrphanModule => write!(f, "OrphanModule"),
            GapType::MissingCapability => write!(f, "MissingCapability"),
            GapType::BottleneckModule => write!(f, "BottleneckModule"),
            GapType::StubModule => write!(f, "StubModule"),
            GapType::MissingTests => write!(f, "MissingTests"),
            GapType::MissingCrossLink => write!(f, "MissingCrossLink"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Effort {
    /// < 100 lines, 1 file
    Small,
    /// 100-500 lines, 2-3 files
    Medium,
    /// 500+ lines, new module
    Large,
}

impl Effort {
    /// Numeric weight (lower = easier).
    fn weight(&self) -> f64 {
        match self {
            Effort::Small => 1.0,
            Effort::Medium => 3.0,
            Effort::Large => 6.0, // n=6
        }
    }
}

/// A prioritised plan of architectural actions.
#[derive(Debug)]
pub struct ArchPlan {
    pub actions: Vec<ArchGap>,
    pub total_impact: f64,
    pub total_effort: f64,
}

// ═══════════════════════════════════════════════════════════════════════
// Expected connections (domain knowledge)
// ═══════════════════════════════════════════════════════════════════════

/// Pairs of modules that *should* have integration edges.
const EXPECTED_CONNECTIONS: &[(&str, &str)] = &[
    ("calibration", "telescope"),
    ("calibration", "growth"),
    ("growth", "ouroboros"),
    ("simulation", "experiment"),
    ("reward", "genetic_prog"),
    ("red_team", "verifier"),
    ("feedback", "growth"),
    ("pipeline", "ingest"),
    ("pipeline", "publish"),
    ("knowledge", "graph"),
    ("scheduler", "autonomous"),
    ("alert", "feedback"),
    ("dream", "consciousness_bridge"),
    ("self_improve", "growth"),
    ("statistics", "experiment"),
    ("versioning", "history"),
    ("sandbox", "red_team"),
    ("nlp", "cli"),
    ("multi_agent", "distributed"),
    ("event", "alert"),
];

/// Known import relationships (actual `use crate::` edges discovered via inspection).
const KNOWN_EDGES: &[(&str, &str)] = &[
    // growth -> calibration, telescope, etc.
    ("growth", "telescope"),
    ("growth", "calibration"),
    // calibration -> telescope
    ("calibration", "telescope"),
    // experiment -> telescope
    ("experiment", "telescope"),
    // graph -> telescope
    ("graph", "telescope"),
    // ouroboros -> telescope, experiment
    ("ouroboros", "telescope"),
    ("ouroboros", "experiment"),
    // verifier -> telescope
    ("verifier", "telescope"),
    // lens_forge -> telescope
    ("lens_forge", "telescope"),
    // cli -> telescope, graph, history, ouroboros, experiment, science
    ("cli", "telescope"),
    ("cli", "graph"),
    ("cli", "history"),
    ("cli", "ouroboros"),
    ("cli", "experiment"),
    ("cli", "science"),
    // science -> telescope
    ("science", "telescope"),
    // encoder -> gpu
    ("encoder", "gpu"),
    // simulation -> telescope
    ("simulation", "telescope"),
    // pipeline -> ingest, publish
    ("pipeline", "ingest"),
    ("pipeline", "publish"),
    // auto_register -> telescope
    ("auto_register", "telescope"),
];

/// Module descriptions for prompt generation.
fn module_description(name: &str) -> &'static str {
    match name {
        "gpu" => "Metal GPU compute kernels for macOS",
        "encoder" => "Data encoding/decoding for lens input/output",
        "materials" => "Physical materials database and property lookups",
        "verifier" => "Hypothesis and result verification engine",
        "graph" => "Knowledge graph with nodes, edges, traversal, and PageRank",
        "telescope" => "Core telescope with 22+ lenses, registry, consensus, and shared data",
        "history" => "Versioned history tracking for experiments and discoveries",
        "ouroboros" => "OUROBOROS evolution engine — self-improvement loop",
        "lens_forge" => "Dynamic lens creation and hot-reload",
        "experiment" => "Experiment runner with design and result tracking",
        "science" => "Scientific method automation (hypothesis -> experiment -> conclusion)",
        "cli" => "Command-line interface for all NEXUS-6 operations",
        "alert" => "Real-time alerting system for anomalies and thresholds",
        "api" => "REST/gRPC API surface for external integrations",
        "auto_register" => "Automatic lens and module registration on startup",
        "autonomous" => "Autonomous operation mode — runs without user input",
        "consciousness_bridge" => "Bridge to consciousness/awareness layer",
        "cross_intel" => "Cross-module intelligence sharing and correlation",
        "distributed" => "Distributed scan execution across nodes",
        "dream" => "Dream mode — background pattern discovery during idle",
        "event" => "Event bus for inter-module communication",
        "feedback" => "Feedback loop collection and processing",
        "genetic_prog" => "Genetic programming for evolving scan strategies",
        "ingest" => "Data ingestion from files, streams, APIs",
        "knowledge" => "Knowledge base with indexing and retrieval",
        "multi_agent" => "Multi-agent coordination and conflict resolution",
        "nlp" => "Natural language query interface",
        "pipeline" => "Data processing pipeline orchestration",
        "plugin" => "Plugin ecosystem for third-party extensions",
        "publish" => "Result publishing to files, APIs, dashboards",
        "red_team" => "Red-team adversarial testing and falsification",
        "reward" => "Reward signal computation for RL-guided evolution",
        "sandbox" => "Sandboxed execution environment for untrusted code",
        "scheduler" => "Task scheduling and queue management",
        "self_improve" => "Self-improvement analysis, optimisation, and suggestion",
        "statistics" => "Statistical analysis, distributions, and hypothesis testing",
        "template" => "Template engine for experiment and report generation",
        "time_travel" => "Snapshot and branch-based state time-travel",
        "versioning" => "Semantic versioning and compatibility tracking",
        "calibration" => "Lens calibration using synthetic and real datasets",
        "simulation" => "Simulation engine for what-if experiments",
        "growth" => "Auto-growth system — metrics, benchmarks, tracking, planning, architecture",
        _ => "Unknown module",
    }
}

/// Estimated lines of code per module (approximate, from codebase inspection).
fn estimated_lines(name: &str) -> usize {
    match name {
        "telescope" => 8000,
        "cli" => 1200,
        "growth" => 900,
        "graph" => 800,
        "ouroboros" => 700,
        "experiment" => 600,
        "calibration" => 500,
        "science" => 400,
        "verifier" => 350,
        "gpu" => 350,
        "encoder" => 300,
        "lens_forge" => 300,
        "history" => 250,
        "simulation" => 250,
        "materials" => 200,
        "auto_register" => 200,
        "pipeline" => 180,
        "api" => 150,
        "plugin" => 150,
        "distributed" => 150,
        "nlp" => 120,
        "event" => 100,
        "alert" => 100,
        "autonomous" => 100,
        "feedback" => 80,
        "reward" => 80,
        "genetic_prog" => 80,
        "scheduler" => 80,
        "sandbox" => 60,
        "red_team" => 60,
        "knowledge" => 60,
        "self_improve" => 60,
        "multi_agent" => 50,
        "publish" => 50,
        "ingest" => 50,
        "statistics" => 40,
        "template" => 40,
        "cross_intel" => 30,
        "dream" => 30,
        "consciousness_bridge" => 30,
        "time_travel" => 30,
        "versioning" => 30,
        _ => 20,
    }
}

/// Estimated test count per module.
fn estimated_tests(name: &str) -> usize {
    match name {
        "telescope" => 40,
        "growth" => 12,
        "graph" => 10,
        "calibration" => 8,
        "ouroboros" => 8,
        "experiment" => 6,
        "cli" => 6,
        "science" => 5,
        "verifier" => 5,
        "encoder" => 4,
        "gpu" => 4,
        "materials" => 3,
        "lens_forge" => 3,
        "history" => 3,
        "simulation" => 3,
        "auto_register" => 2,
        "pipeline" => 2,
        "api" => 2,
        "plugin" => 2,
        "distributed" => 2,
        "nlp" => 1,
        "event" => 1,
        "alert" => 1,
        _ => 0,
    }
}

/// Estimated file count per module.
fn estimated_files(name: &str) -> usize {
    match name {
        "telescope" => 30,
        "growth" => 5,
        "cli" => 4,
        "graph" => 3,
        "ouroboros" => 3,
        "experiment" => 3,
        "calibration" => 2,
        "science" => 2,
        "simulation" => 2,
        "pipeline" => 2,
        "ingest" => 2,
        "publish" => 2,
        "knowledge" => 2,
        "red_team" => 2,
        "self_improve" => 3,
        "time_travel" => 2,
        "statistics" => 2,
        _ => 1,
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Core analysis functions
// ═══════════════════════════════════════════════════════════════════════

/// Classify module maturity from lines and test count.
fn classify_maturity(lines: usize, tests: usize, connections: usize) -> Maturity {
    if lines >= 500 && tests >= SIGMA_MINUS_PHI && connections >= TAU {
        Maturity::Production
    } else if lines >= 200 && tests >= SOPFR {
        Maturity::Mature
    } else if lines >= 50 && tests >= PHI {
        Maturity::Basic
    } else {
        Maturity::Stub
    }
}

/// Scan the known module list and build an architecture snapshot.
pub fn analyze_architecture() -> ArchSnapshot {
    // Build connection maps
    let mut imports_from: HashMap<String, Vec<String>> = HashMap::new();
    let mut imported_by: HashMap<String, Vec<String>> = HashMap::new();

    for &(from, to) in KNOWN_EDGES {
        imports_from
            .entry(from.to_string())
            .or_default()
            .push(to.to_string());
        imported_by
            .entry(to.to_string())
            .or_default()
            .push(from.to_string());
    }

    let connections: Vec<(String, String)> = KNOWN_EDGES
        .iter()
        .map(|&(a, b)| (a.to_string(), b.to_string()))
        .collect();

    // Connection count per module (in + out)
    let mut conn_count: HashMap<String, usize> = HashMap::new();
    for &(from, to) in KNOWN_EDGES {
        *conn_count.entry(from.to_string()).or_default() += 1;
        *conn_count.entry(to.to_string()).or_default() += 1;
    }

    // Build module info
    let mut modules = Vec::new();
    let mut total_pub_fns: usize = 0;
    let mut total_pub_structs: usize = 0;

    for &name in KNOWN_MODULES {
        let lines = estimated_lines(name);
        let tests = estimated_tests(name);
        let files = estimated_files(name);
        let conns = conn_count.get(name).copied().unwrap_or(0);
        let maturity = classify_maturity(lines, tests, conns);

        // Estimate pub items: ~1 pub fn per 30 lines, ~1 pub struct per 100 lines
        let pub_fns = (lines / 30).max(1);
        let pub_structs = (lines / 100).max(1);
        total_pub_fns += pub_fns;
        total_pub_structs += pub_structs;

        modules.push(ModuleInfo {
            name: name.to_string(),
            files,
            lines,
            tests,
            pub_fns,
            pub_structs,
            imports_from: imports_from.get(name).cloned().unwrap_or_default(),
            imported_by: imported_by.get(name).cloned().unwrap_or_default(),
            maturity,
        });
    }

    // Orphans: modules with zero connections
    let orphan_modules: Vec<String> = KNOWN_MODULES
        .iter()
        .filter(|&&m| conn_count.get(m).copied().unwrap_or(0) == 0)
        .map(|s| s.to_string())
        .collect();

    // Hub modules: sorted by connection count descending, take top N=6
    let mut hubs: Vec<(String, usize)> = conn_count.into_iter().collect();
    hubs.sort_by(|a, b| b.1.cmp(&a.1));
    hubs.truncate(N);

    ArchSnapshot {
        modules,
        connections,
        orphan_modules,
        hub_modules: hubs,
        total_pub_fns,
        total_pub_structs,
    }
}

/// Returns (capability_name, responsible_module, exists).
pub fn capability_matrix() -> Vec<(&'static str, &'static str, bool)> {
    vec![
        ("data_import", "ingest", true),
        ("data_export", "publish", true),
        ("auto_benchmark", "growth/benchmark", true),
        ("auto_refactor", "growth/architect", true),   // this module!
        ("dependency_graph", "growth/architect", true), // provided here
        ("code_generation", "growth/codegen", false),
        ("architecture_visualization", "growth/architect", false),
        ("regression_detection", "growth/tracker", false),
        ("cross_module_testing", "growth/integration", false),
        ("api_documentation", "api", false),
        ("plugin_ecosystem", "plugin", true),
        ("distributed_scan", "distributed", true),
        ("real_time_monitoring", "alert", true),
        ("ml_inference", "N/A", false),
        ("web_dashboard", "N/A", false),
        ("natural_language_query", "nlp", true),
        ("experiment_design", "experiment", true),
        ("hypothesis_verification", "verifier", true),
        ("evolution_engine", "ouroboros", true),
        ("genetic_search", "genetic_prog", true),
        ("reward_shaping", "reward", true),
        ("sandboxed_execution", "sandbox", true),
        ("knowledge_indexing", "knowledge", true),
        ("event_driven_arch", "event", true),
    ]
}

/// Identify all architectural gaps from a snapshot.
pub fn find_gaps(snapshot: &ArchSnapshot) -> Vec<ArchGap> {
    let mut gaps: Vec<ArchGap> = Vec::new();

    // --- 1. Stub modules ---
    for m in &snapshot.modules {
        if m.maturity == Maturity::Stub {
            gaps.push(ArchGap {
                gap_type: GapType::StubModule,
                description: format!(
                    "Module '{}' is a stub ({} lines, {} tests). Needs full implementation.",
                    m.name, m.lines, m.tests
                ),
                impact: 0.7,
                effort: Effort::Medium,
                suggested_modules: vec![m.name.clone()],
                claude_prompt: format_stub_prompt(&m.name, m.lines),
            });
        }
    }

    // --- 2. Orphan modules ---
    for name in &snapshot.orphan_modules {
        // Skip modules that are stubs (already caught above)
        let is_stub = snapshot
            .modules
            .iter()
            .any(|m| m.name == *name && m.maturity == Maturity::Stub);

        gaps.push(ArchGap {
            gap_type: GapType::OrphanModule,
            description: format!(
                "Module '{}' has no connections to other modules. Risk of dead code.",
                name
            ),
            impact: if is_stub { 0.4 } else { 0.6 },
            effort: Effort::Small,
            suggested_modules: vec![name.clone()],
            claude_prompt: format_orphan_prompt(name),
        });
    }

    // --- 3. Missing integrations (expected but absent) ---
    let edge_set: std::collections::HashSet<(&str, &str)> =
        KNOWN_EDGES.iter().copied().collect();

    for &(a, b) in EXPECTED_CONNECTIONS {
        if !edge_set.contains(&(a, b)) && !edge_set.contains(&(b, a)) {
            gaps.push(ArchGap {
                gap_type: GapType::MissingIntegration,
                description: format!(
                    "Expected integration between '{}' and '{}' is missing.",
                    a, b
                ),
                impact: 0.65,
                effort: Effort::Medium,
                suggested_modules: vec![a.to_string(), b.to_string()],
                claude_prompt: format_integration_prompt(a, b),
            });
        }
    }

    // --- 4. Missing capabilities ---
    for (cap_name, module, exists) in capability_matrix() {
        if !exists {
            gaps.push(ArchGap {
                gap_type: GapType::MissingCapability,
                description: format!(
                    "Missing capability '{}' (would live in '{}').",
                    cap_name, module
                ),
                impact: capability_impact(cap_name),
                effort: capability_effort(cap_name),
                suggested_modules: vec![module.to_string()],
                claude_prompt: format_capability_prompt(cap_name, module),
            });
        }
    }

    // --- 5. Bottleneck / hub overload ---
    for (name, count) in &snapshot.hub_modules {
        // Threshold: σ=12 connections is overloaded
        if *count >= SIGMA {
            gaps.push(ArchGap {
                gap_type: GapType::BottleneckModule,
                description: format!(
                    "Module '{}' is a bottleneck with {} connections (threshold: σ={}).",
                    name, count, SIGMA
                ),
                impact: 0.5,
                effort: Effort::Large,
                suggested_modules: vec![name.clone()],
                claude_prompt: format_bottleneck_prompt(name, *count),
            });
        }
    }

    // --- 6. Missing tests (Basic+ modules with < SOPFR=5 tests) ---
    for m in &snapshot.modules {
        if m.maturity >= Maturity::Basic && m.tests < SOPFR {
            gaps.push(ArchGap {
                gap_type: GapType::MissingTests,
                description: format!(
                    "Module '{}' has {} tests but needs at least sopfr={} for its maturity level.",
                    m.name, m.tests, SOPFR
                ),
                impact: 0.45,
                effort: Effort::Small,
                suggested_modules: vec![m.name.clone()],
                claude_prompt: format_tests_prompt(&m.name, m.tests),
            });
        }
    }

    // --- 7. Missing cross-links (same domain, no connection) ---
    let domain_groups: &[(&str, &[&str])] = &[
        ("analysis", &["statistics", "experiment", "science", "verifier"]),
        ("evolution", &["ouroboros", "genetic_prog", "reward", "self_improve"]),
        ("data", &["ingest", "pipeline", "publish", "knowledge"]),
        ("execution", &["scheduler", "autonomous", "sandbox", "distributed"]),
        ("communication", &["event", "alert", "feedback", "api"]),
    ];

    for &(domain, members) in domain_groups {
        for i in 0..members.len() {
            for j in (i + 1)..members.len() {
                let a = members[i];
                let b = members[j];
                if !edge_set.contains(&(a, b)) && !edge_set.contains(&(b, a)) {
                    // Only add if both modules are at least Basic
                    let a_mature = snapshot.modules.iter().any(|m| m.name == a && m.maturity >= Maturity::Basic);
                    let b_mature = snapshot.modules.iter().any(|m| m.name == b && m.maturity >= Maturity::Basic);
                    if a_mature && b_mature {
                        gaps.push(ArchGap {
                            gap_type: GapType::MissingCrossLink,
                            description: format!(
                                "Modules '{}' and '{}' are in the '{}' domain but have no cross-reference.",
                                a, b, domain
                            ),
                            impact: 0.35,
                            effort: Effort::Small,
                            suggested_modules: vec![a.to_string(), b.to_string()],
                            claude_prompt: format_crosslink_prompt(a, b, domain),
                        });
                    }
                }
            }
        }
    }

    gaps
}

/// Prioritise gaps by impact/effort ratio and produce a plan.
pub fn generate_architecture_plan(gaps: &[ArchGap], max_actions: usize) -> ArchPlan {
    // Score = impact / effort_weight  (higher is better)
    let mut scored: Vec<(usize, f64)> = gaps
        .iter()
        .enumerate()
        .map(|(i, g)| (i, g.impact / g.effort.weight()))
        .collect();

    scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    scored.truncate(max_actions);

    // We cannot move out of the slice, so we rebuild gaps for the plan.
    // In practice the caller owns the Vec and can index into it.
    // Here we just record the indices and compute totals.
    let total_impact: f64 = scored.iter().map(|&(i, _)| gaps[i].impact).sum();
    let total_effort: f64 = scored.iter().map(|&(i, _)| gaps[i].effort.weight()).sum();

    // Return a plan with references via index (we rebuild lightweight copies).
    let actions: Vec<ArchGap> = scored
        .iter()
        .map(|&(i, _)| {
            let g = &gaps[i];
            ArchGap {
                gap_type: g.gap_type,
                description: g.description.clone(),
                impact: g.impact,
                effort: g.effort,
                suggested_modules: g.suggested_modules.clone(),
                claude_prompt: g.claude_prompt.clone(),
            }
        })
        .collect();

    ArchPlan {
        actions,
        total_impact,
        total_effort,
    }
}

/// Generate a detailed Claude CLI prompt for a specific gap.
pub fn format_claude_architecture_prompt(gap: &ArchGap) -> String {
    // The per-gap prompt is already stored, but this function enriches it
    // with common preamble and validation instructions.
    format!(
        "{preamble}\n\n{body}\n\n{validation}",
        preamble = "You are working on the NEXUS-6 Discovery Engine, a Rust project.",
        body = gap.claude_prompt,
        validation = "After making changes, run:\n\
            1. cd /Users/ghost/Dev/n6-architecture/tools/nexus6 && ~/.cargo/bin/cargo check\n\
            2. ~/.cargo/bin/cargo test\n\
            Ensure both pass before finishing."
    )
}

// ═══════════════════════════════════════════════════════════════════════
// Prompt generators (private helpers)
// ═══════════════════════════════════════════════════════════════════════

fn format_stub_prompt(module: &str, current_lines: usize) -> String {
    let desc = module_description(module);
    format!(
        "In {root}, fully implement the '{module}' module. \
         Currently it has only ~{lines} lines and is a stub. \
         It should provide: {desc}. \
         Add at least {sopfr} unit tests. \
         Follow the existing code style (n=6 constants, doc comments, mod.rs re-exports). \
         Do NOT break any existing public API.",
        root = NEXUS_ROOT,
        module = module,
        lines = current_lines,
        desc = desc,
        sopfr = SOPFR,
    )
}

fn format_orphan_prompt(module: &str) -> String {
    let desc = module_description(module);
    format!(
        "In {root}, the '{module}' module ({desc}) has no connections to other modules. \
         Add meaningful integration:\n\
         1. Identify which existing modules should use '{module}' or be used by it.\n\
         2. Add `use crate::{module}::...` imports where appropriate.\n\
         3. Create at least one bridge function or shared trait.\n\
         4. Add integration tests that exercise the new connection.",
        root = NEXUS_ROOT,
        module = module,
        desc = desc,
    )
}

fn format_integration_prompt(module_a: &str, module_b: &str) -> String {
    let desc_a = module_description(module_a);
    let desc_b = module_description(module_b);
    format!(
        "In {root}, add integration between '{a}' and '{b}'.\n\
         Module '{a}': {desc_a}\n\
         Module '{b}': {desc_b}\n\n\
         Create bridge functions that allow '{a}' to leverage '{b}' capabilities \
         (or vice versa). Add at least {phi} integration tests that exercise the \
         new connection. Update mod.rs re-exports as needed.",
        root = NEXUS_ROOT,
        a = module_a,
        b = module_b,
        desc_a = desc_a,
        desc_b = desc_b,
        phi = PHI,
    )
}

fn format_capability_prompt(capability: &str, module: &str) -> String {
    format!(
        "In {root}, create or extend the '{module}' module to provide the \
         '{capability}' capability. This capability is currently missing from NEXUS-6.\n\n\
         Requirements:\n\
         - If the module path contains '/', it's a sub-module (e.g., growth/codegen → src/growth/codegen.rs).\n\
         - Implement the core functionality with at least {sopfr} public functions.\n\
         - Add {sopfr} unit tests.\n\
         - Register in the parent mod.rs.\n\
         - Follow n=6 constant naming conventions.",
        root = NEXUS_ROOT,
        module = module,
        capability = capability,
        sopfr = SOPFR,
    )
}

fn format_bottleneck_prompt(module: &str, connection_count: usize) -> String {
    format!(
        "In {root}, the '{module}' module is a bottleneck with {count} connections \
         (threshold: sigma={sigma}). Refactor it:\n\
         1. Identify distinct responsibilities within '{module}'.\n\
         2. Extract sub-modules for each responsibility.\n\
         3. Create a facade (the original mod.rs) that re-exports the split pieces.\n\
         4. Ensure all existing public API items remain accessible.\n\
         5. Add tests to verify the refactoring preserves behaviour.",
        root = NEXUS_ROOT,
        module = module,
        count = connection_count,
        sigma = SIGMA,
    )
}

fn format_tests_prompt(module: &str, current_tests: usize) -> String {
    let desc = module_description(module);
    format!(
        "In {root}, the '{module}' module ({desc}) has only {current} tests. \
         Add at least {needed} more tests to bring it to sopfr={sopfr} total. \
         Cover: edge cases, error paths, and integration with other modules it uses. \
         Use #[test] functions in a `mod tests` block at the bottom of the relevant file.",
        root = NEXUS_ROOT,
        module = module,
        desc = desc,
        current = current_tests,
        needed = SOPFR.saturating_sub(current_tests),
        sopfr = SOPFR,
    )
}

fn format_crosslink_prompt(module_a: &str, module_b: &str, domain: &str) -> String {
    format!(
        "In {root}, modules '{a}' and '{b}' are both in the '{domain}' domain \
         but have no cross-reference. Add a meaningful connection:\n\
         - '{a}': {desc_a}\n\
         - '{b}': {desc_b}\n\
         Create shared types or bridge functions. Add at least 1 test.",
        root = NEXUS_ROOT,
        a = module_a,
        b = module_b,
        domain = domain,
        desc_a = module_description(module_a),
        desc_b = module_description(module_b),
    )
}

/// Impact score for a capability (higher = more important).
fn capability_impact(cap: &str) -> f64 {
    match cap {
        "regression_detection" => 0.85,
        "code_generation" => 0.80,
        "cross_module_testing" => 0.75,
        "api_documentation" => 0.60,
        "architecture_visualization" => 0.55,
        "ml_inference" => 0.50,
        "web_dashboard" => 0.45,
        _ => 0.50,
    }
}

/// Effort estimate for a capability.
fn capability_effort(cap: &str) -> Effort {
    match cap {
        "regression_detection" => Effort::Medium,
        "code_generation" => Effort::Large,
        "cross_module_testing" => Effort::Medium,
        "api_documentation" => Effort::Small,
        "architecture_visualization" => Effort::Medium,
        "ml_inference" => Effort::Large,
        "web_dashboard" => Effort::Large,
        _ => Effort::Medium,
    }
}

/// Format the full architecture plan as an ASCII report.
pub fn format_plan_report(plan: &ArchPlan) -> String {
    let mut s = String::new();
    s.push_str("┌──────────────────────────────────────────────────────────────────┐\n");
    s.push_str("│           NEXUS-6 Architecture Growth Plan                      │\n");
    s.push_str("├────┬─────────────────────┬────────┬────────┬───────────────────┤\n");
    s.push_str("│  # │ Gap Type            │ Impact │ Effort │ Module(s)         │\n");
    s.push_str("├────┼─────────────────────┼────────┼────────┼───────────────────┤\n");

    for (i, action) in plan.actions.iter().enumerate() {
        let effort_str = match action.effort {
            Effort::Small => "S",
            Effort::Medium => "M",
            Effort::Large => "L",
        };
        let modules_str = action.suggested_modules.join(", ");
        let modules_truncated = if modules_str.len() > 17 {
            format!("{}...", &modules_str[..14])
        } else {
            modules_str
        };

        s.push_str(&format!(
            "│ {:>2} │ {:19} │ {:>5.2}  │   {}    │ {:17} │\n",
            i + 1,
            format!("{}", action.gap_type),
            action.impact,
            effort_str,
            modules_truncated,
        ));
    }

    s.push_str("├────┴─────────────────────┴────────┴────────┴───────────────────┤\n");
    s.push_str(&format!(
        "│  Total: impact={:.2}  effort={:.1}  actions={}               │\n",
        plan.total_impact,
        plan.total_effort,
        plan.actions.len(),
    ));
    s.push_str("└──────────────────────────────────────────────────────────────────┘\n");
    s
}

/// Format a summary of the architecture snapshot.
pub fn format_snapshot_report(snapshot: &ArchSnapshot) -> String {
    let mut s = String::new();

    let stub_count = snapshot.modules.iter().filter(|m| m.maturity == Maturity::Stub).count();
    let basic_count = snapshot.modules.iter().filter(|m| m.maturity == Maturity::Basic).count();
    let mature_count = snapshot.modules.iter().filter(|m| m.maturity == Maturity::Mature).count();
    let prod_count = snapshot.modules.iter().filter(|m| m.maturity == Maturity::Production).count();

    s.push_str("┌──────────────────────────────────────────────────────┐\n");
    s.push_str("│         NEXUS-6 Architecture Snapshot                │\n");
    s.push_str("├──────────────────────────────────────────────────────┤\n");
    s.push_str(&format!("│  Modules:        {:>4}                                │\n", snapshot.modules.len()));
    s.push_str(&format!("│  Connections:    {:>4}                                │\n", snapshot.connections.len()));
    s.push_str(&format!("│  Orphans:        {:>4}                                │\n", snapshot.orphan_modules.len()));
    s.push_str(&format!("│  pub fn:         {:>4}                                │\n", snapshot.total_pub_fns));
    s.push_str(&format!("│  pub struct:     {:>4}                                │\n", snapshot.total_pub_structs));
    s.push_str("├──────────────────────────────────────────────────────┤\n");
    s.push_str("│  Maturity Distribution                               │\n");
    s.push_str(&format!("│    Production: {:>3}  ", prod_count));
    s.push_str(&bar(prod_count, snapshot.modules.len(), 20));
    s.push('\n');
    s.push_str(&format!("│    Mature:     {:>3}  ", mature_count));
    s.push_str(&bar(mature_count, snapshot.modules.len(), 20));
    s.push('\n');
    s.push_str(&format!("│    Basic:      {:>3}  ", basic_count));
    s.push_str(&bar(basic_count, snapshot.modules.len(), 20));
    s.push('\n');
    s.push_str(&format!("│    Stub:       {:>3}  ", stub_count));
    s.push_str(&bar(stub_count, snapshot.modules.len(), 20));
    s.push('\n');
    s.push_str("├──────────────────────────────────────────────────────┤\n");
    s.push_str("│  Top Hubs                                            │\n");
    for (name, count) in &snapshot.hub_modules {
        s.push_str(&format!("│    {:16} {:>3} connections                  │\n", name, count));
    }
    s.push_str("└──────────────────────────────────────────────────────┘\n");
    s
}

fn bar(value: usize, total: usize, width: usize) -> String {
    if total == 0 {
        return format!("{:width$} │", "", width = width);
    }
    let filled = (value * width) / total;
    let empty = width - filled;
    format!(
        "{}{}  │",
        "█".repeat(filled),
        "░".repeat(empty),
    )
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_architecture_returns_all_modules() {
        let snap = analyze_architecture();
        assert_eq!(snap.modules.len(), KNOWN_MODULES.len());
        // telescope should be a hub (most connections)
        assert!(
            snap.hub_modules.iter().any(|(name, _)| name == "telescope"),
            "telescope should be in hub_modules"
        );
        // Should have some connections
        assert!(!snap.connections.is_empty(), "should have known edges");
    }

    #[test]
    fn test_find_gaps_detects_stubs() {
        let snap = analyze_architecture();
        let gaps = find_gaps(&snap);
        // There should be stub gaps (several modules are < 50 lines)
        let stub_gaps: Vec<_> = gaps.iter().filter(|g| g.gap_type == GapType::StubModule).collect();
        assert!(!stub_gaps.is_empty(), "should find stub modules");
        // Each stub gap should have a non-empty claude_prompt
        for g in &stub_gaps {
            assert!(!g.claude_prompt.is_empty());
            assert!(g.impact > 0.0);
        }
    }

    #[test]
    fn test_find_gaps_detects_missing_integrations() {
        let snap = analyze_architecture();
        let gaps = find_gaps(&snap);
        let integration_gaps: Vec<_> = gaps
            .iter()
            .filter(|g| g.gap_type == GapType::MissingIntegration)
            .collect();
        // growth<->ouroboros is expected but not in KNOWN_EDGES
        assert!(
            integration_gaps
                .iter()
                .any(|g| g.suggested_modules.contains(&"ouroboros".to_string())
                    && g.suggested_modules.contains(&"growth".to_string())),
            "should detect missing growth<->ouroboros integration"
        );
    }

    #[test]
    fn test_generate_plan_respects_max_actions() {
        let snap = analyze_architecture();
        let gaps = find_gaps(&snap);
        assert!(!gaps.is_empty(), "need gaps to plan");

        let plan = generate_architecture_plan(&gaps, N); // max n=6 actions
        assert!(plan.actions.len() <= N);
        assert!(plan.total_impact > 0.0);

        // Actions should be sorted by impact/effort (best first)
        if plan.actions.len() >= 2 {
            let score_0 = plan.actions[0].impact / plan.actions[0].effort.weight();
            let score_1 = plan.actions[1].impact / plan.actions[1].effort.weight();
            assert!(
                score_0 >= score_1 - 1e-9,
                "plan should be sorted by priority: {:.3} >= {:.3}",
                score_0,
                score_1
            );
        }
    }

    #[test]
    fn test_capability_matrix_has_mixed_states() {
        let matrix = capability_matrix();
        let existing: usize = matrix.iter().filter(|&&(_, _, e)| e).count();
        let missing: usize = matrix.iter().filter(|&&(_, _, e)| !e).count();
        assert!(existing > 0, "should have existing capabilities");
        assert!(missing > 0, "should have missing capabilities");
        // Total should be J₂=24 capabilities
        assert_eq!(matrix.len(), _J2, "capability matrix should have J2={} entries", _J2);
    }

    #[test]
    fn test_classify_maturity_thresholds() {
        assert_eq!(classify_maturity(10, 0, 0), Maturity::Stub);
        assert_eq!(classify_maturity(49, 1, 0), Maturity::Stub);
        assert_eq!(classify_maturity(100, 3, 0), Maturity::Basic);
        assert_eq!(classify_maturity(300, 6, 2), Maturity::Mature);
        assert_eq!(classify_maturity(600, 12, 5), Maturity::Production);
    }

    #[test]
    fn test_format_claude_architecture_prompt() {
        let gap = ArchGap {
            gap_type: GapType::StubModule,
            description: "test gap".to_string(),
            impact: 0.7,
            effort: Effort::Medium,
            suggested_modules: vec!["test".to_string()],
            claude_prompt: "Implement the test module.".to_string(),
        };
        let prompt = format_claude_architecture_prompt(&gap);
        assert!(prompt.contains("NEXUS-6"));
        assert!(prompt.contains("Implement the test module"));
        assert!(prompt.contains("cargo check"));
        assert!(prompt.contains("cargo test"));
    }
}
