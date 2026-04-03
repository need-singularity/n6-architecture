//! Lens Growth Engine — tracks which lenses need implementation and generates growth plans.
//!
//! NEXUS-6 has 693+ lenses registered as metadata (name/description/domain_affinity),
//! but only 24 have actual `Lens` trait implementations with real `scan()` logic.
//! This module assesses the gap and produces prioritized plans for auto-implementing more.

use std::collections::HashMap;

// ─── Data types ────────────────────────────────────────────────────────────────

/// Status of a lens.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LensStatus {
    /// Has `Lens` trait impl with real `scan()` logic.
    Implemented,
    /// Registered in registry but no impl.
    MetadataOnly,
    /// In growth queue (selected for next batch).
    Planned,
}

/// Snapshot of the current lens implementation state.
#[derive(Debug, Clone)]
pub struct LensGrowthState {
    pub implemented: Vec<String>,
    pub metadata_only: Vec<String>,
    pub total_registered: usize,
    pub implementation_rate: f64, // implemented / total
}

/// A batch plan describing which lenses to implement next.
#[derive(Debug, Clone)]
pub struct LensGrowthPlan {
    pub next_batch: Vec<LensToImplement>,
    pub batch_size: usize,
    pub estimated_tests_added: usize,
    pub priority_reason: String,
}

/// A single lens slated for implementation.
#[derive(Debug, Clone)]
pub struct LensToImplement {
    pub name: String,
    pub category: String,
    pub description: String,
    pub domain_affinity: Vec<String>,
    pub suggested_algorithm: String,
    pub claude_prompt: String,
    pub priority: f64,
}

// ─── Metadata entry (lightweight copy of registry LensEntry for internal use) ─

#[derive(Debug, Clone)]
pub struct MetaLens {
    name: String,
    category: String,
    description: String,
    domain_affinity: Vec<String>,
}

// ─── Constants ─────────────────────────────────────────────────────────────────

/// The 24 lenses that already have full `Lens` trait implementations.
const IMPLEMENTED_LENSES: &[&str] = &[
    "consciousness",
    "gravity",
    "topology",
    "thermo",
    "wave",
    "evolution",
    "info",
    "quantum",
    "em",
    "ruler",
    "triangle",
    "compass",
    "mirror",
    "scale",
    "causal",
    "void",
    "quantum_micro",
    "stability",
    "network",
    "memory",
    "recursion",
    "boundary",
    "multiscale",
    "barrier",
];

/// Source files that contain metadata-only lens definitions, in priority order.
/// Each tuple: (source_id, category_label, known_count).
const METADATA_SOURCES: &[(&str, &str, usize)] = &[
    ("n6_lenses", "n6-industry", 58),
    ("physics_deep_lenses", "physics-deep", 49),
    ("accel_lenses_a", "accel-ml", 58),
    ("cross_lenses", "cross-project", 40),
    ("accel_lenses_b", "accel-physics", 57),
    ("accel_lenses_c", "accel-engineering", 55),
    ("tecs_lenses", "tecs-math", 103),
    ("anima_lenses", "anima-consciousness", 88),
    ("sedi_lenses", "sedi-signal", 100),
    ("accel_lenses_d", "accel-humanities", 63),
    ("quantum_lenses", "quantum", 0), // count unknown
];

/// Priority weights per source (higher = implement sooner).
#[allow(dead_code)]
fn source_priority(source_id: &str) -> f64 {
    match source_id {
        "n6_lenses" => 1.0,
        "physics_deep_lenses" => 0.95,
        "accel_lenses_a" => 0.90,
        "cross_lenses" => 0.85,
        "accel_lenses_b" => 0.80,
        "accel_lenses_c" => 0.75,
        "tecs_lenses" => 0.70,
        "anima_lenses" => 0.65,
        "sedi_lenses" => 0.60,
        "accel_lenses_d" => 0.55,
        "quantum_lenses" => 0.50,
        _ => 0.30,
    }
}

/// Priority lens names within each source (implement these first).
const PRIORITY_LENSES: &[&str] = &[
    // n6_lenses — high impact industry
    "chip_architecture",
    "battery_chemistry",
    "solar_efficiency",
    "fusion_plasma",
    "transformer_anatomy",
    "moe_routing",
    "isomorphism",
    "emergence",
    "periodicity",
    "completeness",
    "surprise",
    "frustration",
    // physics_deep — scientific rigor
    "lattice_field",
    "renormalization",
    "conformal_bootstrap",
    "topological_insulator",
    "bose_einstein",
    "feynman_path",
    "electromagnetic_wave",
    "maxwell_equation",
    "superconductor_lens",
    "heat_conduction",
    // accel_a — practical ML
    "speculative_decode",
    "flash_attention_lens",
    "kernel_fusion",
    "batch_optimization",
    "gradient_checkpointing",
    "mixed_precision",
    // cross — cross-project synergy
    "cross_domain_resonance",
    "hypothesis_linker",
    "atlas_pattern",
];

// ─── Core functions ────────────────────────────────────────────────────────────

/// Assess current lens implementation state by comparing implemented set
/// against a full metadata registry extracted from known source files.
pub fn assess_lens_state(all_registered_names: &[String]) -> LensGrowthState {
    let impl_set: HashMap<&str, ()> = IMPLEMENTED_LENSES.iter().map(|&n| (n, ())).collect();

    let mut implemented = Vec::new();
    let mut metadata_only = Vec::new();

    for name in all_registered_names {
        if impl_set.contains_key(name.as_str()) {
            implemented.push(name.clone());
        } else {
            metadata_only.push(name.clone());
        }
    }

    // Include any implemented lenses not in the registry list (shouldn't happen, but safe).
    for &name in IMPLEMENTED_LENSES {
        let s = name.to_string();
        if !implemented.contains(&s) {
            implemented.push(s);
        }
    }

    let total = implemented.len() + metadata_only.len();
    let rate = if total > 0 {
        implemented.len() as f64 / total as f64
    } else {
        0.0
    };

    LensGrowthState {
        implemented,
        metadata_only,
        total_registered: total,
        implementation_rate: rate,
    }
}

/// Build a simple assessment when we don't have the full registry at hand.
/// Uses known counts from METADATA_SOURCES.
pub fn assess_lens_state_estimated() -> LensGrowthState {
    let implemented: Vec<String> = IMPLEMENTED_LENSES.iter().map(|s| s.to_string()).collect();
    let total_meta: usize = METADATA_SOURCES.iter().map(|(_, _, c)| c).sum();
    // Core lenses (22) are a subset of implemented. Metadata sources hold the rest.
    let total = implemented.len() + total_meta.saturating_sub(implemented.len());
    let metadata_only_count = total.saturating_sub(implemented.len());

    // We don't know all names without loading the registry, so use placeholders.
    let metadata_only: Vec<String> = (0..metadata_only_count)
        .map(|i| format!("unimplemented_{}", i))
        .collect();

    let rate = if total > 0 {
        implemented.len() as f64 / total as f64
    } else {
        0.0
    };

    LensGrowthState {
        implemented,
        metadata_only,
        total_registered: total,
        implementation_rate: rate,
    }
}

/// Pick the next batch of lenses to implement from the metadata-only set.
/// `batch_size` defaults to 6 (n=6).
pub fn plan_next_batch(
    state: &LensGrowthState,
    batch_size: usize,
    meta_entries: &[MetaLens],
) -> LensGrowthPlan {
    let bs = if batch_size == 0 { 6 } else { batch_size };
    let impl_set: HashMap<&str, ()> = state.implemented.iter().map(|s| (s.as_str(), ())).collect();

    // Score every unimplemented lens.
    let mut candidates: Vec<LensToImplement> = Vec::new();
    for entry in meta_entries {
        if impl_set.contains_key(entry.name.as_str()) {
            continue;
        }
        let base_priority = priority_for_name(&entry.name);
        let algo = suggest_algorithm(&entry.name, &entry.description);
        let prompt = generate_claude_prompt_inner(
            &entry.name,
            &entry.category,
            &entry.description,
            &entry.domain_affinity,
            &algo,
        );
        candidates.push(LensToImplement {
            name: entry.name.clone(),
            category: entry.category.clone(),
            description: entry.description.clone(),
            domain_affinity: entry.domain_affinity.clone(),
            suggested_algorithm: algo,
            claude_prompt: prompt,
            priority: base_priority,
        });
    }

    // Sort descending by priority.
    candidates.sort_by(|a, b| b.priority.partial_cmp(&a.priority).unwrap_or(std::cmp::Ordering::Equal));

    let selected: Vec<LensToImplement> = candidates.into_iter().take(bs).collect();
    let tests_est = selected.len() * 2; // at least 2 tests per lens

    LensGrowthPlan {
        batch_size: bs,
        estimated_tests_added: tests_est,
        priority_reason: format!(
            "Top {} by priority (n6-industry > physics-deep > accel-ml > cross > others)",
            bs
        ),
        next_batch: selected,
    }
}

/// Convenience: plan from the estimated state + priority lens list only.
pub fn plan_next_batch_simple(batch_size: usize) -> LensGrowthPlan {
    let state = assess_lens_state_estimated();
    // Build MetaLens entries from the priority list.
    let meta: Vec<MetaLens> = PRIORITY_LENSES
        .iter()
        .map(|&name| MetaLens {
            name: name.to_string(),
            category: guess_category(name).to_string(),
            description: guess_description(name),
            domain_affinity: guess_domains(name),
        })
        .collect();
    plan_next_batch(&state, batch_size, &meta)
}

// ─── Algorithm suggestion ──────────────────────────────────────────────────────

/// Based on the lens name and description, suggest what kind of mathematical
/// analysis the `scan()` function should perform.
pub fn suggest_algorithm(lens_name: &str, description: &str) -> String {
    let lower_name = lens_name.to_lowercase();
    let lower_desc = description.to_lowercase();
    let combined = format!("{} {}", lower_name, lower_desc);

    // Pattern matching — most specific first.
    let patterns: &[(&[&str], &str)] = &[
        (&["spectral", "fft", "fourier", "frequency", "harmonic"], "FFT / eigenvalue spectral analysis — compute power spectrum, detect dominant frequencies, match against n=6 harmonics (sigma=12, J2=24, tau=4)"),
        (&["cluster", "k-means", "dbscan", "grouping", "partition"], "Clustering analysis — k-means or density-based clustering, report cluster count, silhouette score, match cluster sizes against n=6 divisors"),
        (&["entropy", "information", "shannon", "renyi", "mutual"], "Shannon/Renyi entropy computation — bin data into histograms, compute H(X), compare to ln(6), ln(12), ln(24) reference values"),
        (&["topology", "persistent", "betti", "homology", "hole"], "Persistent homology / Betti numbers — build Vietoris-Rips filtration from distance matrix, track birth-death of topological features"),
        (&["correlation", "pearson", "spearman", "covariance", "corr"], "Correlation analysis — compute pairwise Pearson/Spearman correlations, detect significant positive/negative relationships, report r-values"),
        (&["fractal", "box-counting", "hausdorff", "self-similar"], "Fractal dimension estimation — box-counting or correlation dimension on embedded data, compare to known fractal dimensions"),
        (&["flow", "gradient", "flux", "divergence", "curl", "field"], "Gradient field analysis — estimate local gradients, compute divergence/curl proxies, detect sources/sinks"),
        (&["resonance", "oscillation", "vibration", "tuning"], "Frequency resonance analysis — identify peaks in power spectrum that align with n=6 constants (6, 12, 24, 48Hz harmonics)"),
        (&["stability", "lyapunov", "attractor", "basin", "equilib"], "Lyapunov exponent estimation — embed time-series, track divergence of nearby trajectories, classify stability regimes"),
        (&["symmetry", "group", "invariant", "parity", "rotation"], "Group-theoretic symmetry detection — test data invariance under permutations/reflections, quantify symmetry breaking"),
        (&["diffusion", "random walk", "brownian", "transport"], "Diffusion analysis — estimate mean-square displacement scaling, detect anomalous diffusion exponents"),
        (&["wave", "propagation", "interference", "phase", "oscillat"], "Wave analysis — detect propagating patterns, compute phase coherence, identify interference signatures"),
        (&["network", "graph", "adjacency", "degree", "connectiv"], "Network/graph analysis — build proximity graph, compute degree distribution, clustering coefficient, small-world metrics"),
        (&["scaling", "power law", "exponent", "zipf", "pareto"], "Scaling/power-law analysis — fit log-log slopes, test power-law hypothesis, extract scaling exponents"),
        (&["emergent", "self-organ", "spontaneous", "pattern form"], "Emergence detection — measure local vs global order parameters, detect phase transitions in complexity metrics"),
        (&["inverse", "reverse", "deconvol"], "Inverse analysis — estimate latent structure from observed effects using regularized inversion or maximum-entropy"),
        (&["combinat", "enumerate", "design space", "dse"], "Combinatorial enumeration — score discrete subsets, compute coverage of parameter space, rank by n=6 alignment"),
        (&["complet", "coverage", "exhaustive", "gap"], "Completeness analysis — measure what fraction of expected feature space is covered, identify gaps"),
        (&["periodi", "cycle", "recurrence", "repeat"], "Periodicity detection — autocorrelation analysis, detect dominant periods, compare to n=6 cycle lengths"),
        (&["chip", "semicond", "architect", "sm", "transistor"], "Chip architecture analysis — extract architectural parameters, compare against n=6 ladder (sigma=12, sigma^2=144, J2=24)"),
        (&["battery", "electroch", "cathode", "anode", "cell"], "Battery chemistry analysis — detect electrochemical signatures, coordination numbers, compare to CN=6 universality (BT-43)"),
        (&["solar", "photovolt", "bandgap", "absorb"], "Solar efficiency analysis — detect SQ-limit aligned bandgaps (4/3 eV), compute efficiency bounds"),
        (&["fusion", "plasma", "tokamak", "confine"], "Plasma/fusion analysis — detect confinement signatures, Lawson parameter estimation, n=6 tokamak metrics"),
        (&["transform", "attention", "head", "layer"], "Transformer anatomy analysis — detect attention head structure, layer depth scaling, sigma=12 atom matching (BT-33)"),
        (&["moe", "expert", "routing", "gating", "sparse"], "MoE routing analysis — detect expert activation fractions (1/2,1/3,1/6), gating sparsity patterns (BT-67)"),
        (&["superconduct", "meissner", "cooper"], "Superconductor analysis — detect phase transition signatures, critical temperature indicators"),
        (&["renormal", "rg flow", "beta function"], "Renormalization group analysis — detect scale-dependent coupling changes, RG flow direction"),
        (&["conformal", "bootstrap", "cft"], "Conformal bootstrap analysis — detect conformal dimension spectra, crossing symmetry signatures"),
        (&["bose", "einstein", "condensat"], "BEC analysis — detect macroscopic occupation signatures, coherence length estimation"),
        (&["feynman", "path integral", "propagator"], "Path integral analysis — sum-over-paths weighting, detect dominant path contributions"),
    ];

    for (keywords, algorithm) in patterns {
        for kw in *keywords {
            if combined.contains(kw) {
                return algorithm.to_string();
            }
        }
    }

    // Default: statistical profiling + n=6 constant matching.
    "Statistical profiling + n=6 constant matching — compute mean, std, skew, kurtosis, check ratios against sigma(6)=12, phi(6)=2, tau(6)=4, J2(6)=24, sopfr(6)=5".to_string()
}

// ─── Claude prompt generation ──────────────────────────────────────────────────

/// Generate a ready-to-use Claude CLI prompt for implementing a given lens.
pub fn generate_claude_prompt(lens: &LensToImplement) -> String {
    generate_claude_prompt_inner(
        &lens.name,
        &lens.category,
        &lens.description,
        &lens.domain_affinity,
        &lens.suggested_algorithm,
    )
}

fn generate_claude_prompt_inner(
    name: &str,
    category: &str,
    description: &str,
    domain_affinity: &[String],
    suggested_algorithm: &str,
) -> String {
    let snake = to_snake(name);
    let pascal = to_pascal(name);
    let domains = domain_affinity.join(", ");

    format!(
r#"In /Users/ghost/Dev/n6-architecture/tools/nexus6/src/telescope/lenses/, create {snake}_lens.rs implementing the Lens trait.

Lens name: "{name}"
Struct name: {pascal}Lens
Category: "{category}"
Description: "{description}"
Domain affinity: [{domains}]

The scan() function should:
- Take data as &[f64] with n points and d dimensions (row-major: data[i*d + dim])
- Use SharedData for pre-computed distance matrix (shared.dist(i, j)) if needed
- Perform: {suggested_algorithm}
- Return HashMap<String, Vec<f64>> with meaningful metric names as keys
- Include at least 2 #[cfg(test)] tests that verify non-empty output

Required imports:
```rust
use std::collections::HashMap;
use crate::telescope::lens_trait::{{Lens, LensResult}};
use crate::telescope::shared_data::SharedData;
```

Reference existing lens for style: src/telescope/lenses/consciousness_lens.rs

After creating the file:
1. Add `pub mod {snake}_lens;` and `pub use {snake}_lens::{pascal}Lens;` to src/telescope/lenses/mod.rs
2. Ensure it compiles: cargo check

IMPORTANT:
- Real analysis logic, not stubs or empty HashMaps
- No external crates beyond std
- shared.dist(i, j) panics if i==j, use 0.0 for self-distance
- Data is row-major: point i dimension d = data[i * d_count + d_idx]
"#)
}

// ─── Growth report ─────────────────────────────────────────────────────────────

/// Format an ASCII progress report showing implementation rate.
pub fn format_growth_report(state: &LensGrowthState) -> String {
    let _pct = (state.implementation_rate * 100.0) as usize;
    let bar_width: usize = 50;
    let filled = (state.implementation_rate * bar_width as f64) as usize;
    let empty = bar_width.saturating_sub(filled);

    let bar = format!(
        "[{}{}] {:.1}%",
        "█".repeat(filled),
        "░".repeat(empty),
        state.implementation_rate * 100.0,
    );

    let mut report = String::new();
    report.push_str("╔══════════════════════════════════════════════════════════════╗\n");
    report.push_str("║           NEXUS-6 Lens Growth Report                       ║\n");
    report.push_str("╠══════════════════════════════════════════════════════════════╣\n");
    report.push_str(&format!(
        "║  Total registered:   {:>4}                                   ║\n",
        state.total_registered
    ));
    report.push_str(&format!(
        "║  Implemented:        {:>4}                                   ║\n",
        state.implemented.len()
    ));
    report.push_str(&format!(
        "║  Metadata only:      {:>4}                                   ║\n",
        state.metadata_only.len()
    ));
    report.push_str("║                                                              ║\n");
    report.push_str(&format!(
        "║  {}  ║\n",
        bar
    ));
    report.push_str("║                                                              ║\n");

    // Source breakdown.
    report.push_str("║  Source breakdown:                                           ║\n");
    for &(_source, label, count) in METADATA_SOURCES {
        if count > 0 {
            report.push_str(&format!(
                "║    {:<28} {:>3} lenses                ║\n",
                label, count
            ));
        }
    }
    report.push_str("║                                                              ║\n");

    // Growth target.
    let remaining = state.metadata_only.len();
    let batches_needed = (remaining + 5) / 6; // ceil(remaining / 6)
    report.push_str(&format!(
        "║  Batches to 100% (n=6/batch): {:>3}                          ║\n",
        batches_needed
    ));
    report.push_str("╚══════════════════════════════════════════════════════════════╝\n");

    report
}

// ─── Helper functions ──────────────────────────────────────────────────────────

fn priority_for_name(name: &str) -> f64 {
    // Check if in PRIORITY_LENSES list.
    for (i, &pname) in PRIORITY_LENSES.iter().enumerate() {
        if pname == name {
            // Higher priority for earlier entries.
            return 1.0 - (i as f64 * 0.01);
        }
    }
    // Default priority based on source category heuristic.
    0.30
}

fn to_snake(name: &str) -> String {
    name.to_lowercase().replace('-', "_").replace(' ', "_")
}

fn to_pascal(name: &str) -> String {
    name.split(|c: char| c == '_' || c == '-' || c == ' ')
        .filter(|s| !s.is_empty())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(c) => {
                    let upper: String = c.to_uppercase().collect();
                    format!("{}{}", upper, chars.as_str().to_lowercase())
                }
                None => String::new(),
            }
        })
        .collect()
}

fn guess_category(name: &str) -> &str {
    let n = name.to_lowercase();
    if n.contains("chip") || n.contains("battery") || n.contains("solar")
        || n.contains("fusion") || n.contains("transformer") || n.contains("moe")
        || n.contains("isomorphism") || n.contains("emergence") || n.contains("periodicity")
        || n.contains("completeness") || n.contains("surprise") || n.contains("frustration")
    {
        "n6-industry"
    } else if n.contains("lattice") || n.contains("renormal") || n.contains("conformal")
        || n.contains("topological") || n.contains("bose") || n.contains("feynman")
        || n.contains("electromagnetic") || n.contains("maxwell") || n.contains("superconductor")
        || n.contains("heat")
    {
        "physics-deep"
    } else if n.contains("speculative") || n.contains("flash") || n.contains("kernel")
        || n.contains("batch") || n.contains("gradient") || n.contains("mixed")
    {
        "accel-ml"
    } else if n.contains("cross") || n.contains("hypothesis") || n.contains("atlas") {
        "cross-project"
    } else {
        "extended"
    }
}

fn guess_description(name: &str) -> String {
    let algo = suggest_algorithm(name, "");
    format!("Auto-detected lens for '{}' domain — {}", name, algo)
}

fn guess_domains(name: &str) -> Vec<String> {
    let n = name.to_lowercase();
    let mut domains = Vec::new();
    if n.contains("chip") || n.contains("semicond") { domains.push("semiconductor".into()); }
    if n.contains("battery") || n.contains("electro") { domains.push("energy".into()); }
    if n.contains("solar") || n.contains("photo") { domains.push("energy".into()); }
    if n.contains("fusion") || n.contains("plasma") { domains.push("fusion".into()); }
    if n.contains("transform") || n.contains("attention") || n.contains("moe") { domains.push("ai".into()); }
    if n.contains("lattice") || n.contains("field") { domains.push("physics".into()); }
    if n.contains("renormal") || n.contains("conformal") || n.contains("feynman") { domains.push("physics".into()); }
    if n.contains("topological") || n.contains("bose") { domains.push("physics".into()); }
    if n.contains("electromagnetic") || n.contains("maxwell") || n.contains("superconductor") { domains.push("physics".into()); }
    if n.contains("speculative") || n.contains("flash") || n.contains("kernel") || n.contains("gradient") { domains.push("ml".into()); }
    if n.contains("cross") || n.contains("hypothesis") || n.contains("atlas") { domains.push("meta".into()); }
    if domains.is_empty() {
        domains.push("general".into());
    }
    domains
}

// ─── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_state_estimated() {
        let state = assess_lens_state_estimated();
        assert_eq!(state.implemented.len(), IMPLEMENTED_LENSES.len());
        assert!(state.total_registered > state.implemented.len());
        assert!(state.implementation_rate > 0.0);
        assert!(state.implementation_rate < 1.0);
        // 24 implemented out of ~693+
        assert!(state.implementation_rate < 0.10, "Rate should be under 10%: {}", state.implementation_rate);
    }

    #[test]
    fn test_assess_state_with_names() {
        let names: Vec<String> = vec![
            "consciousness".into(),
            "gravity".into(),
            "isomorphism".into(),
            "extrapolation".into(),
        ];
        let state = assess_lens_state(&names);
        assert_eq!(state.implemented.len(), 24); // all 24 + from list
        assert!(state.metadata_only.contains(&"isomorphism".to_string()));
        assert!(state.metadata_only.contains(&"extrapolation".to_string()));
    }

    #[test]
    fn test_plan_next_batch_simple() {
        let plan = plan_next_batch_simple(6);
        assert_eq!(plan.batch_size, 6);
        assert!(plan.next_batch.len() <= 6);
        assert!(plan.estimated_tests_added >= plan.next_batch.len() * 2);
        // Check that prompts are non-empty.
        for lens in &plan.next_batch {
            assert!(!lens.claude_prompt.is_empty());
            assert!(!lens.suggested_algorithm.is_empty());
        }
    }

    #[test]
    fn test_suggest_algorithm_patterns() {
        let spectral = suggest_algorithm("spectral_decomposition", "FFT-based frequency analysis");
        assert!(spectral.contains("FFT") || spectral.contains("spectral"), "Should match spectral: {}", spectral);

        let cluster = suggest_algorithm("cluster_density", "clustering and grouping patterns");
        assert!(cluster.contains("luster"), "Should match clustering: {}", cluster);

        let entropy = suggest_algorithm("info_entropy", "Shannon entropy of data distribution");
        assert!(entropy.contains("ntropy"), "Should match entropy: {}", entropy);

        let chip = suggest_algorithm("chip_architecture", "semiconductor architecture analysis");
        assert!(chip.contains("hip") || chip.contains("rchitect"), "Should match chip: {}", chip);

        let default = suggest_algorithm("unknown_thing", "something very new and unusual");
        assert!(default.contains("tatistical"), "Should fall back to statistical: {}", default);
    }

    #[test]
    fn test_generate_claude_prompt() {
        let lens = LensToImplement {
            name: "chip_architecture".to_string(),
            category: "n6-industry".to_string(),
            description: "Analyze chip architecture parameters".to_string(),
            domain_affinity: vec!["semiconductor".to_string()],
            suggested_algorithm: "Chip architecture analysis".to_string(),
            claude_prompt: String::new(),
            priority: 1.0,
        };
        let prompt = generate_claude_prompt(&lens);
        assert!(prompt.contains("chip_architecture_lens.rs"));
        assert!(prompt.contains("ChipArchitectureLens"));
        assert!(prompt.contains("semiconductor"));
        assert!(prompt.contains("Lens trait"));
    }

    #[test]
    fn test_format_growth_report() {
        let state = assess_lens_state_estimated();
        let report = format_growth_report(&state);
        assert!(report.contains("NEXUS-6"));
        assert!(report.contains("Implemented"));
        assert!(report.contains("Metadata only"));
        // Should contain the bar.
        assert!(report.contains("["));
        assert!(report.contains("]"));
    }

    #[test]
    fn test_to_pascal() {
        assert_eq!(to_pascal("chip_architecture"), "ChipArchitecture");
        assert_eq!(to_pascal("bose_einstein"), "BoseEinstein");
        assert_eq!(to_pascal("moe_routing"), "MoeRouting");
        assert_eq!(to_pascal("em"), "Em");
    }

    #[test]
    fn test_priority_ordering() {
        // Priority lenses should have higher priority than unknown ones.
        let p_chip = priority_for_name("chip_architecture");
        let p_unknown = priority_for_name("totally_unknown_lens");
        assert!(p_chip > p_unknown, "chip={} should > unknown={}", p_chip, p_unknown);
    }
}
