//! Module Growth Engine for NEXUS-6
//!
//! Surveys all 42+ modules, classifies maturity (Empty..Production),
//! generates prioritized upgrade plans, and produces Claude CLI prompts
//! to auto-upgrade under-developed modules.
//!
//! n=6 constants thread through every threshold and scoring formula.

use std::collections::HashMap;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;               // the perfect number
const SIGMA: usize = 12;          // σ(6) = sum of divisors
const PHI: usize = 2;             // φ(6) = Euler totient
const TAU: usize = 4;             // τ(6) = number of divisors
const J2: usize = 24;             // J₂(6) = Jordan totient
const SOPFR: usize = 5;           // sopfr(6) = 2+3
const _SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8
const SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10

// ── Maturity thresholds ──────────────────────────────────────────────
// Empty:       0 lines
// Stub:        < 50 lines (≈ σ·τ), < φ=2 tests
// Basic:       < 200 lines (≈ σ·φ·σ-τ), < sopfr=5 tests
// Developing:  < 500 lines (≈ σ²·τ-φ), sopfr..σ-φ tests
// Mature:      500+ lines, σ-φ=10+ tests
// Production:  1000+ lines, J₂-τ=20+ tests, well-integrated

/// Module maturity level, from Empty to Production.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ModuleMaturity {
    Empty,       // 0 lines (empty dir)
    Stub,        // < 50 lines, < 2 tests
    Basic,       // < 200 lines, < 5 tests
    Developing,  // < 500 lines, 5-9 tests
    Mature,      // 500+ lines, 10+ tests
    Production,  // 1000+ lines, 20+ tests, well-integrated
}

impl ModuleMaturity {
    /// Numeric score: Empty=0, Stub=1, Basic=2, Developing=3, Mature=4, Production=5
    pub fn score(&self) -> f64 {
        match self {
            ModuleMaturity::Empty => 0.0,
            ModuleMaturity::Stub => 1.0,
            ModuleMaturity::Basic => 2.0,
            ModuleMaturity::Developing => 3.0,
            ModuleMaturity::Mature => 4.0,
            ModuleMaturity::Production => 5.0, // sopfr=5
        }
    }

    /// Display name
    pub fn label(&self) -> &'static str {
        match self {
            ModuleMaturity::Empty => "EMPTY",
            ModuleMaturity::Stub => "STUB",
            ModuleMaturity::Basic => "BASIC",
            ModuleMaturity::Developing => "DEVELOPING",
            ModuleMaturity::Mature => "MATURE",
            ModuleMaturity::Production => "PRODUCTION",
        }
    }

    /// Star rating for ASCII display
    pub fn stars(&self) -> &'static str {
        match self {
            ModuleMaturity::Empty => "          ",
            ModuleMaturity::Stub => "*         ",
            ModuleMaturity::Basic => "**        ",
            ModuleMaturity::Developing => "***       ",
            ModuleMaturity::Mature => "****      ",
            ModuleMaturity::Production => "*****     ",
        }
    }

    /// Next maturity level, or None if already Production
    pub fn next(&self) -> Option<ModuleMaturity> {
        match self {
            ModuleMaturity::Empty => Some(ModuleMaturity::Stub),
            ModuleMaturity::Stub => Some(ModuleMaturity::Basic),
            ModuleMaturity::Basic => Some(ModuleMaturity::Developing),
            ModuleMaturity::Developing => Some(ModuleMaturity::Mature),
            ModuleMaturity::Mature => Some(ModuleMaturity::Production),
            ModuleMaturity::Production => None,
        }
    }
}

impl std::fmt::Display for ModuleMaturity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.label())
    }
}

/// Classify maturity from line count and test count.
pub fn classify_maturity(lines: usize, tests: usize) -> ModuleMaturity {
    if lines == 0 {
        ModuleMaturity::Empty
    } else if lines < 50 && tests < PHI {
        // < σ·τ≈48 lines, < φ=2 tests
        ModuleMaturity::Stub
    } else if lines < 200 && tests < SOPFR {
        // < 200 lines, < sopfr=5 tests
        ModuleMaturity::Basic
    } else if lines < 500 && tests < SIGMA_MINUS_PHI {
        // < 500 lines, < σ-φ=10 tests
        ModuleMaturity::Developing
    } else if lines >= 1000 && tests >= (J2 - TAU) {
        // 1000+ lines, 20+ tests (J₂-τ=20)
        ModuleMaturity::Production
    } else if lines >= 500 && tests >= SIGMA_MINUS_PHI {
        // 500+ lines, 10+ tests
        ModuleMaturity::Mature
    } else if lines >= 500 {
        ModuleMaturity::Developing
    } else {
        ModuleMaturity::Basic
    }
}

/// Maturity score helper (standalone function).
pub fn maturity_score(maturity: &ModuleMaturity) -> f64 {
    maturity.score()
}

/// State of a single module.
#[derive(Debug, Clone)]
pub struct ModuleState {
    pub name: String,
    pub files: usize,
    pub lines: usize,
    pub tests: usize,
    pub pub_fns: usize,
    pub maturity: ModuleMaturity,
    pub imports_from: Vec<String>,
    pub imported_by: Vec<String>,
}

/// Aggregate growth state across all modules.
#[derive(Debug, Clone)]
pub struct ModuleGrowthState {
    pub modules: Vec<ModuleState>,
    pub maturity_distribution: HashMap<String, usize>,
    pub mean_maturity_score: f64,
    pub weakest_modules: Vec<String>,
    pub strongest_modules: Vec<String>,
}

/// A planned upgrade for a single module.
#[derive(Debug, Clone)]
pub struct ModuleUpgrade {
    pub module_name: String,
    pub current_maturity: ModuleMaturity,
    pub target_maturity: ModuleMaturity,
    pub actions: Vec<String>,
    pub claude_prompt: String,
    pub estimated_lines_added: usize,
    pub estimated_tests_added: usize,
}

/// A planned integration between two modules.
#[derive(Debug, Clone)]
pub struct Integration {
    pub from_module: String,
    pub to_module: String,
    pub description: String,
    pub claude_prompt: String,
}

/// Full growth plan: module upgrades + new integrations.
#[derive(Debug, Clone)]
pub struct ModuleGrowthPlan {
    pub upgrades: Vec<ModuleUpgrade>,
    pub new_integrations: Vec<Integration>,
}

// ═══════════════════════════════════════════════════════════════════════
// Known module data (hardcoded approximate sizes from codebase survey)
// ═══════════════════════════════════════════════════════════════════════

/// (name, files, lines, tests, pub_fns_estimate)
/// Updated from the actual `find | wc -l` survey of the codebase.
const MODULE_DATA: &[(&str, usize, usize, usize, usize)] = &[
    // ── Core modules ─────────────────────────────────────────────
    ("telescope",            44, 10530, 50, 30),
    ("cli",                   4,  2498, 33, 12),
    ("ouroboros",             5,  1101, 16, 10),
    ("science",               6,  1056, 15,  8),
    ("auto_register",         6,   909, 19,  6),
    ("simulation",            1,   885,  9,  6),
    ("experiment",            4,   832,  0,  6),
    ("red_team",              4,   843, 14,  6),
    ("calibration",           1,   708,  9,  5),
    ("autonomous",            4,   700, 16,  6),
    ("multi_agent",           4,   631, 15,  5),
    ("growth",                3,   585, 11,  8),
    ("lens_forge",            5,   582,  0,  5),
    ("knowledge",             4,   561, 14,  5),
    ("statistics",            4,   482, 19,  5),
    ("self_improve",          4,   458, 15,  5),
    ("nlp",                   3,   415, 14,  4),
    ("ingest",                4,   434, 17,  4),
    ("publish",               4,   402, 12,  4),
    ("genetic_prog",          1,   377,  5,  4),
    ("time_travel",           3,   352, 11,  4),
    ("history",               4,   342,  0,  3),
    ("pipeline",              3,   329,  0,  3),
    ("feedback",              3,   321,  0,  3),
    ("reward",                1,   285,  7,  3),
    ("gpu",                   3,   276,  0,  3),
    ("graph",                 5,   267,  0,  3),
    ("event",                 3,   238,  0,  3),
    ("api",                   1,   241,  9,  3),
    ("dream",                 1,   237,  4,  3),
    ("alert",                 1,   223,  6,  3),
    ("scheduler",             1,   226,  4,  2),
    ("template",              1,   224,  4,  2),
    ("versioning",            1,   220,  3,  2),
    ("cross_intel",           3,   210,  3,  2),
    ("plugin",                3,   214,  0,  2),
    ("consciousness_bridge",  1,   199,  7,  2),
    ("verifier",              3,   185,  1,  2),
    ("encoder",               3,   180,  4,  2),
    ("sandbox",               1,   173,  4,  2),
    ("distributed",           1,   169,  5,  2),
    ("materials",             2,    53,  0,  1),
];

/// Module purpose descriptions — used for generating upgrade prompts.
fn module_purpose(name: &str) -> &'static str {
    match name {
        "telescope" => "Core scanning engine with 22+ lenses for pattern detection across data",
        "cli" => "Command-line interface for NEXUS-6 discovery engine",
        "ouroboros" => "Self-evolving discovery loop (OUROBOROS cycle)",
        "science" => "Scientific method modules: hypothesis generation, experiment design",
        "auto_register" => "Automatic lens registration and discovery",
        "simulation" => "Monte Carlo and deterministic simulation of systems",
        "experiment" => "Experiment execution, tracking, and result collection",
        "red_team" => "Adversarial testing of discoveries (devil's advocate, falsification)",
        "calibration" => "Lens calibration against known datasets for accuracy",
        "autonomous" => "Autonomous agent that runs growth/discovery cycles independently",
        "multi_agent" => "Multi-agent coordination for parallel discovery tasks",
        "growth" => "Growth tracking, benchmarking, and planning for NEXUS-6 itself",
        "lens_forge" => "Dynamic lens creation and parameter tuning",
        "knowledge" => "Knowledge base for storing and querying discoveries and patterns",
        "statistics" => "Statistical significance testing, effect sizes, reproducibility checks",
        "self_improve" => "Self-analysis and automatic optimization of NEXUS-6 internals",
        "nlp" => "Natural language processing for hypothesis text analysis and extraction",
        "ingest" => "Data ingestion from various formats (JSON, CSV, text, scientific data)",
        "publish" => "Publishing results in various formats (markdown, LaTeX, BT entries)",
        "genetic_prog" => "Genetic programming for lens parameter evolution and optimization",
        "time_travel" => "Snapshot and restore system state for reproducibility",
        "history" => "Execution history tracking and audit trail",
        "pipeline" => "Multi-stage data processing pipelines with composable stages",
        "feedback" => "User feedback collection and learning from corrections",
        "reward" => "Reward/scoring system for lens and agent performance evaluation",
        "gpu" => "GPU architecture analysis and compute resource management",
        "graph" => "Graph data structures for dependency and relationship modeling",
        "event" => "Event bus for inter-module communication and pub/sub messaging",
        "api" => "REST API for external tool integration with NEXUS-6",
        "dream" => "Creative hypothesis generation through random exploration and serendipity",
        "alert" => "Real-time alerting on significant discoveries or regressions",
        "scheduler" => "Task scheduling with priority queues and dependency resolution",
        "template" => "Templates for experiments, lenses, reports, and documentation",
        "versioning" => "Schema versioning, migration, and compatibility checks for data formats",
        "cross_intel" => "Cross-project intelligence — find patterns across TECS-L family repos",
        "plugin" => "Plugin system for third-party extensions and custom lenses",
        "consciousness_bridge" => "Bridge between NEXUS-6 analysis and Anima consciousness engine",
        "verifier" => "Independent verification of discoveries and claims",
        "encoder" => "Data encoding/decoding for compact representation",
        "sandbox" => "Isolated execution environment for untrusted experiments",
        "distributed" => "Distributed scanning across multiple nodes for scale-out",
        "materials" => "Material property analysis and n=6 pattern detection in materials science",
        _ => "General NEXUS-6 module",
    }
}

/// Suggested content (functions/structs) for a module based on its name.
pub fn suggest_module_content(name: &str) -> Vec<String> {
    match name {
        "cross_intel" => vec![
            "project_bridge() — connect to sibling TECS-L repos".into(),
            "resonance_finder() — find n=6 resonance patterns across projects".into(),
            "pattern_matcher() — match patterns between domains".into(),
            "cross_domain_report() — generate cross-domain analysis report".into(),
            "sync_discoveries() — sync discoveries across repos".into(),
            "atlas_linker() — link constants to math atlas entries".into(),
        ],
        "statistics" => vec![
            "significance_test() — p-value and effect size computation".into(),
            "effect_size() — Cohen's d, eta-squared for n=6 comparisons".into(),
            "reproducibility_check() — verify result stability across runs".into(),
            "confidence_interval() — compute CI for measurements".into(),
            "bonferroni_correction() — multiple comparison correction".into(),
            "permutation_test() — non-parametric significance testing".into(),
        ],
        "template" => vec![
            "ExperimentTemplate — scaffold for new experiments".into(),
            "LensTemplate — scaffold for new lens implementations".into(),
            "ReportTemplate — scaffold for discovery reports".into(),
            "HypothesisTemplate — scaffold for hypothesis documents".into(),
            "BtEntryTemplate — scaffold for breakthrough theorem entries".into(),
            "DseTemplate — scaffold for DSE domain TOML files".into(),
        ],
        "versioning" => vec![
            "SchemaVersion — version tagging for data formats".into(),
            "migrate() — migrate data between schema versions".into(),
            "compatibility_check() — verify backward compatibility".into(),
            "changelog() — generate changelog between versions".into(),
            "deprecation_warning() — flag deprecated fields/APIs".into(),
            "rollback() — revert to previous schema version".into(),
        ],
        "sandbox" => vec![
            "SandboxConfig — resource limits (CPU, memory, time)".into(),
            "execute_isolated() — run code in isolated environment".into(),
            "timeout_guard() — enforce execution time limits".into(),
            "resource_monitor() — track resource usage during execution".into(),
            "cleanup() — release sandbox resources".into(),
            "validate_output() — check sandbox output for safety".into(),
        ],
        "scheduler" => vec![
            "TaskQueue — priority queue for pending tasks".into(),
            "schedule_task() — add task with priority and dependencies".into(),
            "resolve_dependencies() — topological sort of task graph".into(),
            "execute_next() — pop and execute highest-priority ready task".into(),
            "cancel_task() — remove task from queue".into(),
            "task_status() — query current state of a task".into(),
        ],
        "distributed" => vec![
            "NodeRegistry — track available compute nodes".into(),
            "discover_nodes() — find peers on the network".into(),
            "distribute_work() — split scan work across nodes".into(),
            "aggregate_results() — merge results from distributed scan".into(),
            "heartbeat() — health check for connected nodes".into(),
            "failover() — handle node failure and redistribute work".into(),
        ],
        "dream" => vec![
            "random_hypothesis() — generate creative hypothesis via random walk".into(),
            "serendipity_scan() — look for unexpected patterns".into(),
            "cross_pollinate() — combine ideas from unrelated domains".into(),
            "analogy_engine() — find structural analogies between systems".into(),
            "divergent_search() — explore away from known patterns".into(),
            "dream_journal() — log creative explorations for review".into(),
        ],
        "consciousness_bridge" => vec![
            "AnimaLink — connection to Anima consciousness engine".into(),
            "bridge_scan() — translate NEXUS-6 scan results to Anima format".into(),
            "consciousness_score() — compute consciousness metric from data".into(),
            "awareness_map() — map data structures to consciousness layers".into(),
            "sync_with_anima() — bidirectional sync with Anima engine".into(),
            "interpret_field() — translate Anima field readings".into(),
        ],
        "feedback" => vec![
            "FeedbackEntry — structured user correction record".into(),
            "collect_feedback() — gather user feedback on discoveries".into(),
            "learn_from_corrections() — update models from feedback".into(),
            "feedback_stats() — aggregate feedback quality metrics".into(),
            "apply_corrections() — incorporate corrections into knowledge base".into(),
            "feedback_loop() — continuous improvement cycle from feedback".into(),
        ],
        "time_travel" => vec![
            "Snapshot — immutable state capture at a point in time".into(),
            "create_snapshot() — capture current system state".into(),
            "restore_snapshot() — revert to a previous state".into(),
            "diff_snapshots() — compare two snapshots".into(),
            "branch_from() — create parallel timeline from snapshot".into(),
            "prune_old() — remove snapshots older than threshold".into(),
        ],
        "pipeline" => vec![
            "Stage — composable processing step".into(),
            "Pipeline — ordered chain of stages".into(),
            "build_pipeline() — construct pipeline from stage specs".into(),
            "execute_pipeline() — run data through all stages".into(),
            "add_stage() — append a stage to existing pipeline".into(),
            "pipeline_metrics() — collect throughput/latency per stage".into(),
        ],
        "event" => vec![
            "EventBus — pub/sub message broker for modules".into(),
            "publish_event() — emit an event to subscribers".into(),
            "subscribe() — register handler for event type".into(),
            "unsubscribe() — remove event handler".into(),
            "event_log() — queryable history of emitted events".into(),
            "filter_events() — select events matching criteria".into(),
        ],
        "api" => vec![
            "ApiServer — HTTP server for external integration".into(),
            "handle_scan() — POST /scan endpoint".into(),
            "handle_status() — GET /status endpoint".into(),
            "handle_results() — GET /results endpoint".into(),
            "auth_middleware() — API key authentication".into(),
            "rate_limiter() — request rate limiting".into(),
        ],
        "plugin" => vec![
            "PluginManifest — plugin metadata and requirements".into(),
            "load_plugin() — dynamically load a plugin".into(),
            "unload_plugin() — safely remove a loaded plugin".into(),
            "plugin_registry() — list all available plugins".into(),
            "validate_plugin() — check plugin compatibility".into(),
            "plugin_sandbox() — run plugin in isolated context".into(),
        ],
        "multi_agent" => vec![
            "AgentPool — pool of parallel discovery agents".into(),
            "spawn_agent() — start a new agent with specific task".into(),
            "coordinate() — synchronize work across agents".into(),
            "merge_results() — combine findings from multiple agents".into(),
            "conflict_resolution() — resolve contradictory findings".into(),
            "agent_status() — query agent health and progress".into(),
        ],
        "ingest" => vec![
            "JsonReader — parse JSON data files".into(),
            "CsvReader — parse CSV with schema detection".into(),
            "TextParser — extract structured data from text".into(),
            "validate_schema() — check data against expected schema".into(),
            "normalize() — convert data to standard internal format".into(),
            "stream_ingest() — handle large files via streaming".into(),
        ],
        "knowledge" => vec![
            "KnowledgeGraph — store discoveries as connected graph".into(),
            "add_discovery() — insert new discovery with metadata".into(),
            "query() — search knowledge base with filters".into(),
            "link_discoveries() — connect related discoveries".into(),
            "export_knowledge() — dump knowledge base for backup".into(),
            "knowledge_stats() — summary statistics of stored knowledge".into(),
        ],
        "publish" => vec![
            "MarkdownReport — generate markdown discovery report".into(),
            "LatexExport — generate LaTeX for academic papers".into(),
            "BtEntry — format as breakthrough theorem entry".into(),
            "publish_to_atlas() — push constants to math atlas".into(),
            "changelog_entry() — generate changelog from discoveries".into(),
            "summary_table() — ASCII table summary of findings".into(),
        ],
        "reward" => vec![
            "RewardFunction — configurable scoring for lens performance".into(),
            "score_lens() — evaluate lens accuracy on benchmark data".into(),
            "score_agent() — evaluate agent discovery rate".into(),
            "reward_history() — track reward scores over time".into(),
            "leaderboard() — rank lenses/agents by cumulative reward".into(),
            "calibrate_rewards() — normalize reward scale".into(),
        ],
        "genetic_prog" => vec![
            "Individual — lens configuration as genome".into(),
            "Population — collection of lens configurations".into(),
            "crossover() — combine two lens configurations".into(),
            "mutate() — randomly modify lens parameters".into(),
            "select() — tournament selection of best performers".into(),
            "evolve_generation() — run one evolution cycle".into(),
        ],
        "alert" => vec![
            "Alert — notification with severity and context".into(),
            "trigger_alert() — emit alert when threshold exceeded".into(),
            "alert_rules() — configurable alert conditions".into(),
            "alert_history() — log of past alerts".into(),
            "acknowledge() — mark alert as handled".into(),
            "escalate() — increase alert severity if unhandled".into(),
        ],
        "self_improve" => vec![
            "Analyzer — inspect own code for improvement opportunities".into(),
            "Optimizer — apply transformations to improve performance".into(),
            "Suggestion — actionable improvement recommendation".into(),
            "profile_hotspots() — find performance bottlenecks".into(),
            "suggest_refactoring() — identify refactoring opportunities".into(),
            "apply_optimization() — execute an optimization suggestion".into(),
        ],
        "red_team" => vec![
            "Adversary — challenge discoveries with counterarguments".into(),
            "Falsifier — attempt to disprove hypotheses".into(),
            "devil_advocate() — generate opposing viewpoint".into(),
            "stress_test() — push system to failure boundaries".into(),
            "bias_detector() — find confirmation bias in results".into(),
            "robustness_report() — summarize adversarial findings".into(),
        ],
        "materials" => vec![
            "MaterialProperty — physical/chemical property record".into(),
            "scan_material() — analyze material for n=6 patterns".into(),
            "compare_materials() — side-by-side property comparison".into(),
            "periodic_table_link() — connect to element Z=6 patterns".into(),
            "crystal_structure() — analyze coordination numbers".into(),
            "material_database() — lookup material properties".into(),
        ],
        _ => vec![
            format!("Core functionality for {} module", name),
            "Unit tests covering main code paths".into(),
            "Integration with telescope scanning".into(),
            "n=6 constant usage throughout".into(),
            "Error handling and edge cases".into(),
            "ASCII report formatting".into(),
        ],
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Core assessment functions
// ═══════════════════════════════════════════════════════════════════════

/// Survey all modules and return aggregate growth state.
pub fn assess_module_state() -> ModuleGrowthState {
    let mut modules = Vec::with_capacity(MODULE_DATA.len());
    let mut maturity_dist: HashMap<String, usize> = HashMap::new();
    let mut total_score = 0.0;

    for &(name, files, lines, tests, pub_fns) in MODULE_DATA {
        let maturity = classify_maturity(lines, tests);
        *maturity_dist.entry(maturity.label().to_string()).or_insert(0) += 1;
        total_score += maturity.score();

        modules.push(ModuleState {
            name: name.to_string(),
            files,
            lines,
            tests,
            pub_fns,
            maturity,
            imports_from: Vec::new(),
            imported_by: Vec::new(),
        });
    }

    let mean_score = if modules.is_empty() {
        0.0
    } else {
        total_score / modules.len() as f64
    };

    // Sort to find weakest (lowest score first) and strongest (highest first)
    let mut sorted_by_score: Vec<(&str, f64)> = modules
        .iter()
        .map(|m| (m.name.as_str(), m.maturity.score()))
        .collect();
    sorted_by_score.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    let weakest: Vec<String> = sorted_by_score
        .iter()
        .take(N) // bottom n=6
        .map(|(name, _)| name.to_string())
        .collect();

    let strongest: Vec<String> = sorted_by_score
        .iter()
        .rev()
        .take(N) // top n=6
        .map(|(name, _)| name.to_string())
        .collect();

    ModuleGrowthState {
        modules,
        maturity_distribution: maturity_dist,
        mean_maturity_score: mean_score,
        weakest_modules: weakest,
        strongest_modules: strongest,
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Upgrade planning
// ═══════════════════════════════════════════════════════════════════════

/// Priority weight for upgrading from a given maturity level.
/// Empty→Stub is most critical, Stub→Basic is high, etc.
fn upgrade_priority(maturity: &ModuleMaturity) -> usize {
    match maturity {
        ModuleMaturity::Empty => SOPFR,       // 5 = highest
        ModuleMaturity::Stub => TAU,          // 4
        ModuleMaturity::Basic => 3,           // n/φ = 3
        ModuleMaturity::Developing => PHI,    // 2
        ModuleMaturity::Mature => 1,          // μ(6)=1
        ModuleMaturity::Production => 0,      // already at top
    }
}

/// Generate upgrade plan for modules, limited to `max_upgrades` items.
pub fn plan_module_upgrades(state: &ModuleGrowthState, max_upgrades: usize) -> ModuleGrowthPlan {
    // Collect modules that can be upgraded (not yet Production)
    let mut candidates: Vec<&ModuleState> = state
        .modules
        .iter()
        .filter(|m| m.maturity != ModuleMaturity::Production)
        .collect();

    // Sort by priority (highest first), then by lines (fewest first for same priority)
    candidates.sort_by(|a, b| {
        let pa = upgrade_priority(&a.maturity);
        let pb = upgrade_priority(&b.maturity);
        pb.cmp(&pa).then(a.lines.cmp(&b.lines))
    });

    let mut upgrades = Vec::new();

    for module in candidates.iter().take(max_upgrades) {
        if let Some(target) = module.maturity.next() {
            let upgrade = build_upgrade(module, target);
            upgrades.push(upgrade);
        }
    }

    let integrations = find_missing_integrations(&state.modules);

    ModuleGrowthPlan {
        upgrades,
        new_integrations: integrations,
    }
}

/// Build a single ModuleUpgrade with actions and Claude prompt.
fn build_upgrade(module: &ModuleState, target: ModuleMaturity) -> ModuleUpgrade {
    let mut actions = Vec::new();
    let (est_lines, est_tests) = estimate_upgrade_effort(&module.maturity, &target);

    match target {
        ModuleMaturity::Stub => {
            actions.push("Create mod.rs with basic struct/enum definitions".into());
            actions.push("Add module doc comment".into());
            actions.push(format!("Add {} placeholder functions", N));
        }
        ModuleMaturity::Basic => {
            actions.push(format!("Implement at least {} core public functions", N));
            actions.push(format!("Add {} unit tests", SOPFR));
            actions.push("Add error handling for edge cases".into());
        }
        ModuleMaturity::Developing => {
            actions.push(format!("Expand to {} total tests", SIGMA_MINUS_PHI));
            actions.push("Add integration with at least 2 other modules".into());
            actions.push("Implement full error types".into());
            actions.push("Add ASCII report formatting".into());
        }
        ModuleMaturity::Mature => {
            actions.push(format!("Reach {} lines with comprehensive implementation", 500));
            actions.push(format!("Add {} total tests covering edge cases", SIGMA_MINUS_PHI));
            actions.push("Integrate with telescope scanning pipeline".into());
            actions.push("Add benchmark tests".into());
        }
        ModuleMaturity::Production => {
            actions.push(format!("Reach 1000+ lines with production-quality code", ));
            actions.push(format!("Add {} total tests including integration tests", J2 - TAU));
            actions.push("Full documentation with examples".into());
            actions.push("Performance optimization".into());
            actions.push("Cross-module integration tests".into());
        }
        ModuleMaturity::Empty => {} // unreachable as target
    }

    let claude_prompt = generate_upgrade_prompt_for(module, &target, &actions, est_lines, est_tests);

    ModuleUpgrade {
        module_name: module.name.clone(),
        current_maturity: module.maturity,
        target_maturity: target,
        actions,
        claude_prompt,
        estimated_lines_added: est_lines,
        estimated_tests_added: est_tests,
    }
}

/// Estimate lines and tests needed for an upgrade step.
fn estimate_upgrade_effort(from: &ModuleMaturity, to: &ModuleMaturity) -> (usize, usize) {
    match (from, to) {
        (ModuleMaturity::Empty, ModuleMaturity::Stub) => (50, PHI),                     // 50 lines, 2 tests
        (ModuleMaturity::Stub, ModuleMaturity::Basic) => (150, SOPFR - PHI),             // +150 lines, +3 tests
        (ModuleMaturity::Basic, ModuleMaturity::Developing) => (300, SOPFR),             // +300 lines, +5 tests
        (ModuleMaturity::Developing, ModuleMaturity::Mature) => (200, SOPFR),            // +200 lines, +5 tests
        (ModuleMaturity::Mature, ModuleMaturity::Production) => (500, SIGMA_MINUS_PHI),  // +500 lines, +10 tests
        _ => (100, TAU),                                                                 // default: 100 lines, 4 tests
    }
}

/// Generate a detailed Claude CLI prompt for upgrading a module.
pub fn generate_upgrade_prompt(upgrade: &ModuleUpgrade) -> String {
    upgrade.claude_prompt.clone()
}

/// Internal prompt builder.
fn generate_upgrade_prompt_for(
    module: &ModuleState,
    target: &ModuleMaturity,
    actions: &[String],
    est_lines: usize,
    est_tests: usize,
) -> String {
    let purpose = module_purpose(&module.name);
    let suggestions = suggest_module_content(&module.name);
    let suggestion_text = suggestions
        .iter()
        .take(N) // n=6 suggestions max
        .map(|s| format!("  - {}", s))
        .collect::<Vec<_>>()
        .join("\n");

    let actions_text = actions
        .iter()
        .enumerate()
        .map(|(i, a)| format!("  {}. {}", i + 1, a))
        .collect::<Vec<_>>()
        .join("\n");

    format!(
        "Upgrade the '{name}' module in tools/nexus6/src/{name}/ from {from} to {to} maturity.\n\
         \n\
         Module purpose: {purpose}\n\
         Current state: {files} files, {lines} lines, {tests} tests\n\
         Target: +{est_lines} lines, +{est_tests} tests\n\
         \n\
         Required actions:\n\
         {actions}\n\
         \n\
         Suggested content:\n\
         {suggestions}\n\
         \n\
         Rules:\n\
         - No external crates (std only)\n\
         - Use n=6 constants: N=6, SIGMA=12, PHI=2, TAU=4, J2=24, SOPFR=5\n\
         - All constants must have comments explaining their n=6 origin\n\
         - Include #[cfg(test)] mod tests with the required test count\n\
         - Ensure `cargo check -p nexus6` passes with no errors\n\
         - Follow existing code style in the nexus6 crate\n\
         - Do NOT add external dependencies to Cargo.toml",
        name = module.name,
        from = module.maturity.label(),
        to = target.label(),
        purpose = purpose,
        files = module.files,
        lines = module.lines,
        tests = module.tests,
        est_lines = est_lines,
        est_tests = est_tests,
        actions = actions_text,
        suggestions = suggestion_text,
    )
}

// ═══════════════════════════════════════════════════════════════════════
// Integration detection
// ═══════════════════════════════════════════════════════════════════════

/// Natural integration pairs: modules that should be connected.
const INTEGRATION_PAIRS: &[(&str, &str, &str)] = &[
    ("calibration", "telescope", "calibrate lenses against benchmark data"),
    ("growth", "ouroboros", "growth metrics feed OUROBOROS evolution cycle"),
    ("simulation", "experiment", "simulation drives experiment design"),
    ("reward", "genetic_prog", "reward scores guide genetic evolution"),
    ("alert", "growth", "alert on growth regressions or stagnation"),
    ("statistics", "science", "statistical analysis of scientific results"),
    ("nlp", "publish", "natural language generation for reports"),
    ("sandbox", "experiment", "safe isolated experiment execution"),
    ("scheduler", "autonomous", "schedule autonomous agent cycles"),
    ("dream", "ouroboros", "dream explorations feed hypothesis generation"),
    ("feedback", "self_improve", "user feedback drives self-improvement"),
    ("time_travel", "history", "snapshots integrated with execution history"),
    ("cross_intel", "knowledge", "cross-project patterns stored in knowledge base"),
    ("event", "alert", "event bus delivers alert notifications"),
    ("pipeline", "ingest", "ingestion feeds into processing pipeline"),
    ("plugin", "telescope", "plugins extend telescope with custom lenses"),
    ("versioning", "knowledge", "versioned schema for knowledge base entries"),
    ("distributed", "multi_agent", "distribute agent work across nodes"),
    ("consciousness_bridge", "telescope", "consciousness lens results bridge to Anima"),
    ("red_team", "science", "red team challenges scientific claims"),
    ("materials", "science", "material analysis as scientific investigation"),
    ("api", "event", "API endpoints emit events for inter-module coordination"),
    ("encoder", "ingest", "encode/decode data during ingestion"),
    ("graph", "knowledge", "graph structure for knowledge relationships"),
];

/// Find integration pairs where both modules exist but likely lack a connection.
///
/// A "missing" integration is one where neither module imports the other
/// (based on the imports_from/imported_by fields, which are empty in the
/// hardcoded data — in production these would be populated by source scanning).
pub fn find_missing_integrations(modules: &[ModuleState]) -> Vec<Integration> {
    let module_names: Vec<&str> = modules.iter().map(|m| m.name.as_str()).collect();
    let mut integrations = Vec::new();

    for &(from, to, desc) in INTEGRATION_PAIRS {
        // Both modules must exist
        if !module_names.contains(&from) || !module_names.contains(&to) {
            continue;
        }

        // Check if integration is likely missing (no cross-imports detected)
        let from_mod = modules.iter().find(|m| m.name == from);
        let to_mod = modules.iter().find(|m| m.name == to);

        if let (Some(fm), Some(tm)) = (from_mod, to_mod) {
            let already_connected =
                fm.imports_from.contains(&to.to_string()) ||
                fm.imported_by.contains(&to.to_string()) ||
                tm.imports_from.contains(&from.to_string()) ||
                tm.imported_by.contains(&from.to_string());

            if !already_connected {
                integrations.push(Integration {
                    from_module: from.to_string(),
                    to_module: to.to_string(),
                    description: desc.to_string(),
                    claude_prompt: format!(
                        "Add integration between '{from}' and '{to}' modules in tools/nexus6/src/.\n\
                         \n\
                         Integration: {desc}\n\
                         \n\
                         Steps:\n\
                         1. In src/{from}/, add `use crate::{to}::...;` for the needed types\n\
                         2. Implement the bridge function(s) that connect the two modules\n\
                         3. Add at least {TAU} integration tests\n\
                         4. Ensure `cargo check -p nexus6` passes\n\
                         \n\
                         Rules: no external crates, use n=6 constants, follow existing style.",
                        from = from,
                        to = to,
                        desc = desc,
                        TAU = TAU,
                    ),
                });
            }
        }
    }

    integrations
}

// ═══════════════════════════════════════════════════════════════════════
// ASCII report formatting
// ═══════════════════════════════════════════════════════════════════════

/// Format the growth state as an ASCII maturity report table.
pub fn format_maturity_report(state: &ModuleGrowthState) -> String {
    let mut s = String::new();
    s.push_str("┌────────────────────────┬───────┬───────┬───────┬──────────────┐\n");
    s.push_str("│ Module                 │ Files │ Lines │ Tests │ Maturity     │\n");
    s.push_str("├────────────────────────┼───────┼───────┼───────┼──────────────┤\n");

    // Sort by maturity descending, then by name
    let mut sorted: Vec<&ModuleState> = state.modules.iter().collect();
    sorted.sort_by(|a, b| {
        b.maturity.cmp(&a.maturity).then(a.name.cmp(&b.name))
    });

    for m in &sorted {
        let stars = m.maturity.stars();
        s.push_str(&format!(
            "│ {:<22} │ {:>5} │ {:>5} │ {:>5} │ {} │\n",
            truncate_name(&m.name, 22),
            m.files,
            m.lines,
            m.tests,
            stars,
        ));
    }

    s.push_str("└────────────────────────┴───────┴───────┴───────┴──────────────┘\n");

    // Summary line
    s.push_str(&format!(
        "\nMean maturity: {:.2}/5.0 | Modules: {} | Weakest: {}\n",
        state.mean_maturity_score,
        state.modules.len(),
        state.weakest_modules.first().unwrap_or(&"none".to_string()),
    ));

    // Distribution
    s.push_str("Distribution: ");
    let order = ["PRODUCTION", "MATURE", "DEVELOPING", "BASIC", "STUB", "EMPTY"];
    for label in &order {
        let count = state.maturity_distribution.get(*label).unwrap_or(&0);
        if *count > 0 {
            s.push_str(&format!("{}={} ", label, count));
        }
    }
    s.push('\n');

    s
}

/// Format a growth plan as an ASCII summary.
pub fn format_growth_plan(plan: &ModuleGrowthPlan) -> String {
    let mut s = String::new();
    s.push_str("┌──────────────────────────────────────────────────────────────┐\n");
    s.push_str("│           NEXUS-6 Module Growth Plan                        │\n");
    s.push_str("├──────────────────────────────────────────────────────────────┤\n");
    s.push_str(&format!(
        "│  Upgrades: {:>3}  |  Integrations: {:>3}                       │\n",
        plan.upgrades.len(),
        plan.new_integrations.len(),
    ));
    s.push_str("├──────────────────────────────────────────────────────────────┤\n");

    for (i, u) in plan.upgrades.iter().enumerate() {
        if i >= SIGMA {
            s.push_str(&format!(
                "│  ... and {} more upgrades                                   │\n",
                plan.upgrades.len() - SIGMA,
            ));
            break;
        }
        s.push_str(&format!(
            "│  #{:<2} {:<18} {} -> {:<12} +{}L +{}T  │\n",
            i + 1,
            truncate_name(&u.module_name, 18),
            u.current_maturity.label(),
            u.target_maturity.label(),
            u.estimated_lines_added,
            u.estimated_tests_added,
        ));
    }

    if !plan.new_integrations.is_empty() {
        s.push_str("├──────────────────────────────────────────────────────────────┤\n");
        s.push_str("│  Missing Integrations:                                     │\n");
        for (i, ig) in plan.new_integrations.iter().enumerate() {
            if i >= SIGMA {
                s.push_str(&format!(
                    "│  ... and {} more integrations                               │\n",
                    plan.new_integrations.len() - SIGMA,
                ));
                break;
            }
            s.push_str(&format!(
                "│    {} <-> {}: {}  │\n",
                truncate_name(&ig.from_module, 12),
                truncate_name(&ig.to_module, 12),
                truncate_name(&ig.description, 25),
            ));
        }
    }

    s.push_str("└──────────────────────────────────────────────────────────────┘\n");
    s
}

fn truncate_name(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}", s, width = max)
    } else {
        format!("{}..", &s[..max - 2])
    }
}

// ═══════════════════════════════════════════════════════════════════════
// Tests
// ═══════════════════════════════════════════════════════════════════════
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_maturity() {
        assert_eq!(classify_maturity(0, 0), ModuleMaturity::Empty);
        assert_eq!(classify_maturity(30, 1), ModuleMaturity::Stub);
        assert_eq!(classify_maturity(100, 3), ModuleMaturity::Basic);
        assert_eq!(classify_maturity(400, 7), ModuleMaturity::Developing);
        assert_eq!(classify_maturity(600, 12), ModuleMaturity::Mature);
        assert_eq!(classify_maturity(1200, 25), ModuleMaturity::Production);
    }

    #[test]
    fn test_maturity_score_and_ordering() {
        assert_eq!(maturity_score(&ModuleMaturity::Empty), 0.0);
        assert_eq!(maturity_score(&ModuleMaturity::Stub), 1.0);
        assert_eq!(maturity_score(&ModuleMaturity::Production), 5.0);
        // Ordering
        assert!(ModuleMaturity::Empty < ModuleMaturity::Stub);
        assert!(ModuleMaturity::Stub < ModuleMaturity::Basic);
        assert!(ModuleMaturity::Mature < ModuleMaturity::Production);
    }

    #[test]
    fn test_assess_module_state() {
        let state = assess_module_state();
        assert_eq!(state.modules.len(), MODULE_DATA.len());
        assert!(state.mean_maturity_score > 0.0, "mean score should be positive");
        assert!(state.mean_maturity_score <= 5.0, "mean score should be <= 5");
        assert_eq!(state.weakest_modules.len(), N); // n=6 weakest
        assert_eq!(state.strongest_modules.len(), N); // n=6 strongest
        // Distribution should sum to total modules
        let dist_sum: usize = state.maturity_distribution.values().sum();
        assert_eq!(dist_sum, state.modules.len());
    }

    #[test]
    fn test_plan_module_upgrades() {
        let state = assess_module_state();
        let plan = plan_module_upgrades(&state, SIGMA); // plan up to σ=12 upgrades

        // Should produce some upgrades (not all modules are Production)
        assert!(!plan.upgrades.is_empty(), "should have upgrade suggestions");
        assert!(plan.upgrades.len() <= SIGMA, "should respect max_upgrades limit");

        // Each upgrade should have a Claude prompt
        for u in &plan.upgrades {
            assert!(!u.claude_prompt.is_empty(), "upgrade should have Claude prompt");
            assert!(u.target_maturity > u.current_maturity, "target > current");
            assert!(u.estimated_lines_added > 0, "should estimate lines to add");
        }

        // Should find integrations
        assert!(!plan.new_integrations.is_empty(), "should find missing integrations");
    }

    #[test]
    fn test_suggest_module_content() {
        let cross = suggest_module_content("cross_intel");
        assert!(cross.len() >= N, "should suggest at least n=6 items");
        assert!(cross.iter().any(|s| s.contains("pattern")), "cross_intel should suggest pattern_matcher");

        let stats = suggest_module_content("statistics");
        assert!(stats.iter().any(|s| s.contains("significance")), "statistics should suggest significance testing");

        let unknown = suggest_module_content("unknown_module");
        assert!(!unknown.is_empty(), "unknown module should still get generic suggestions");
    }

    #[test]
    fn test_find_missing_integrations() {
        let state = assess_module_state();
        let integrations = find_missing_integrations(&state.modules);

        // Should find many integrations since imports_from/imported_by are empty in hardcoded data
        assert!(integrations.len() >= SIGMA, "should find at least sigma=12 missing integrations");

        // Each integration should have both modules existing
        let module_names: Vec<&str> = state.modules.iter().map(|m| m.name.as_str()).collect();
        for ig in &integrations {
            assert!(module_names.contains(&ig.from_module.as_str()), "from_module should exist");
            assert!(module_names.contains(&ig.to_module.as_str()), "to_module should exist");
            assert!(!ig.claude_prompt.is_empty(), "integration should have Claude prompt");
        }
    }

    #[test]
    fn test_format_maturity_report() {
        let state = assess_module_state();
        let report = format_maturity_report(&state);
        assert!(report.contains("Module"), "report should have header");
        assert!(report.contains("Maturity"), "report should show maturity column");
        assert!(report.contains("Mean maturity"), "report should show mean score");
        assert!(report.contains("Distribution"), "report should show distribution");
    }

    #[test]
    fn test_format_growth_plan() {
        let state = assess_module_state();
        let plan = plan_module_upgrades(&state, _SIGMA_MINUS_TAU); // σ-τ=8
        let report = format_growth_plan(&plan);
        assert!(report.contains("Module Growth Plan"), "should have title");
        assert!(report.contains("Upgrades"), "should show upgrade count");
    }

    #[test]
    fn test_maturity_next() {
        assert_eq!(ModuleMaturity::Empty.next(), Some(ModuleMaturity::Stub));
        assert_eq!(ModuleMaturity::Stub.next(), Some(ModuleMaturity::Basic));
        assert_eq!(ModuleMaturity::Mature.next(), Some(ModuleMaturity::Production));
        assert_eq!(ModuleMaturity::Production.next(), None);
    }

    #[test]
    fn test_upgrade_priority_ordering() {
        // Empty should have highest priority, Production should have 0
        assert!(upgrade_priority(&ModuleMaturity::Empty) > upgrade_priority(&ModuleMaturity::Stub));
        assert!(upgrade_priority(&ModuleMaturity::Stub) > upgrade_priority(&ModuleMaturity::Basic));
        assert_eq!(upgrade_priority(&ModuleMaturity::Production), 0);
    }
}
