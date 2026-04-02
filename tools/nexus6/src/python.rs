//! PyO3 bindings for NEXUS-6 — `import nexus6` from Python.
//!
//! Build with: `maturin develop --features python`
//! Or: `cargo build --features python`

use pyo3::prelude::*;
use pyo3::types::PyDict;
use std::collections::HashMap;

use crate::history::recommend;
use crate::lens_forge::forge_engine::{self, ForgeConfig};
use crate::ouroboros::engine::{CycleResult, EvolutionConfig, EvolutionEngine};
use crate::ouroboros::meta_loop::{MetaLoop, MetaLoopConfig};
use crate::telescope::registry::{LensCategory, LensEntry, LensRegistry};
use crate::verifier::feasibility;
use crate::verifier::n6_check as n6_check_mod;

// ---------------------------------------------------------------------------
// Helper: LensCategory -> str
// ---------------------------------------------------------------------------
fn category_str(cat: LensCategory) -> &'static str {
    match cat {
        LensCategory::Core => "Core",
        LensCategory::DomainCombo => "DomainCombo",
        LensCategory::Extended => "Extended",
        LensCategory::Custom => "Custom",
    }
}

fn str_to_category(s: &str) -> LensCategory {
    match s {
        "Core" => LensCategory::Core,
        "DomainCombo" => LensCategory::DomainCombo,
        "Extended" => LensCategory::Extended,
        "Custom" => LensCategory::Custom,
        _ => LensCategory::Custom,
    }
}

// ---------------------------------------------------------------------------
// PyLensEntry — a single lens metadata record returned to Python as dict-like
// ---------------------------------------------------------------------------
#[pyclass(name = "LensEntry")]
#[derive(Clone)]
struct PyLensEntry {
    #[pyo3(get)]
    name: String,
    #[pyo3(get)]
    category: String,
    #[pyo3(get)]
    description: String,
    #[pyo3(get)]
    domain_affinity: Vec<String>,
    #[pyo3(get)]
    complementary: Vec<String>,
}

#[pymethods]
impl PyLensEntry {
    fn __repr__(&self) -> String {
        format!(
            "LensEntry(name='{}', category='{}', domains={})",
            self.name,
            self.category,
            self.domain_affinity.len()
        )
    }

    /// Convert to a plain Python dict.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("name", &self.name)?;
        dict.set_item("category", &self.category)?;
        dict.set_item("description", &self.description)?;
        dict.set_item("domain_affinity", &self.domain_affinity)?;
        dict.set_item("complementary", &self.complementary)?;
        Ok(dict)
    }
}

impl From<&LensEntry> for PyLensEntry {
    fn from(e: &LensEntry) -> Self {
        PyLensEntry {
            name: e.name.clone(),
            category: category_str(e.category).to_string(),
            description: e.description.clone(),
            domain_affinity: e.domain_affinity.clone(),
            complementary: e.complementary.clone(),
        }
    }
}

// ---------------------------------------------------------------------------
// PyLensRegistry — wraps the full lens registry
// ---------------------------------------------------------------------------
#[pyclass(name = "LensRegistry")]
struct PyLensRegistry {
    inner: LensRegistry,
}

#[pymethods]
impl PyLensRegistry {
    #[new]
    fn new() -> Self {
        PyLensRegistry {
            inner: LensRegistry::new(),
        }
    }

    /// Total number of registered lenses.
    fn __len__(&self) -> usize {
        self.inner.len()
    }

    /// Total number of registered lenses.
    fn len(&self) -> usize {
        self.inner.len()
    }

    /// Look up a lens by name. Returns LensEntry or None.
    fn get(&self, name: &str) -> Option<PyLensEntry> {
        self.inner.get(name).map(PyLensEntry::from)
    }

    /// Return all lenses belonging to the given category string.
    /// Valid categories: "Core", "DomainCombo", "Extended", "Custom".
    fn by_category(&self, category: &str) -> Vec<PyLensEntry> {
        let cat = str_to_category(category);
        self.inner
            .by_category(cat)
            .into_iter()
            .map(PyLensEntry::from)
            .collect()
    }

    /// Return lenses matching a domain string (case-insensitive substring).
    fn for_domain(&self, domain: &str) -> Vec<PyLensEntry> {
        self.inner
            .for_domain(domain)
            .into_iter()
            .map(PyLensEntry::from)
            .collect()
    }

    /// List all lens names.
    fn names(&self) -> Vec<String> {
        self.inner.iter().map(|(name, _)| name.clone()).collect()
    }

    fn __repr__(&self) -> String {
        format!("LensRegistry(lenses={})", self.inner.len())
    }
}

// ---------------------------------------------------------------------------
// PyN6Match — result of n6_check
// ---------------------------------------------------------------------------
#[pyclass(name = "N6Match")]
#[derive(Clone)]
struct PyN6Match {
    #[pyo3(get)]
    constant_name: String,
    #[pyo3(get)]
    quality: f64,
}

#[pymethods]
impl PyN6Match {
    fn __repr__(&self) -> String {
        let grade = if self.quality >= 1.0 {
            "EXACT"
        } else if self.quality >= 0.8 {
            "CLOSE"
        } else if self.quality >= 0.5 {
            "WEAK"
        } else {
            "NONE"
        };
        format!(
            "N6Match(constant='{}', quality={:.2}, grade='{}')",
            self.constant_name, self.quality, grade
        )
    }

    /// Convert to dict.
    fn to_dict<'py>(&self, py: Python<'py>) -> PyResult<Bound<'py, PyDict>> {
        let dict = PyDict::new_bound(py);
        dict.set_item("constant_name", &self.constant_name)?;
        dict.set_item("quality", self.quality)?;
        let grade = if self.quality >= 1.0 {
            "EXACT"
        } else if self.quality >= 0.8 {
            "CLOSE"
        } else if self.quality >= 0.5 {
            "WEAK"
        } else {
            "NONE"
        };
        dict.set_item("grade", grade)?;
        Ok(dict)
    }
}

// ---------------------------------------------------------------------------
// PyLensRecommendation
// ---------------------------------------------------------------------------
#[pyclass(name = "LensRecommendation")]
#[derive(Clone)]
struct PyLensRecommendation {
    #[pyo3(get)]
    lenses: Vec<String>,
    #[pyo3(get)]
    reason: String,
}

#[pymethods]
impl PyLensRecommendation {
    fn __repr__(&self) -> String {
        format!(
            "LensRecommendation(lenses={}, reason='{}')",
            self.lenses.len(),
            self.reason
        )
    }
}

// ---------------------------------------------------------------------------
// PyVerificationResult
// ---------------------------------------------------------------------------
#[pyclass(name = "VerificationResult")]
#[derive(Clone)]
struct PyVerificationResult {
    #[pyo3(get)]
    score: f64,
    #[pyo3(get)]
    grade: String,
    #[pyo3(get)]
    lens_consensus: f64,
    #[pyo3(get)]
    cross_validation: f64,
    #[pyo3(get)]
    physical_check: f64,
    #[pyo3(get)]
    graph_bonus: f64,
    #[pyo3(get)]
    novelty: f64,
    #[pyo3(get)]
    n6_exact: f64,
}

#[pymethods]
impl PyVerificationResult {
    fn __repr__(&self) -> String {
        format!(
            "VerificationResult(score={:.3}, grade='{}')",
            self.score, self.grade
        )
    }
}

// ---------------------------------------------------------------------------
// PyCycleResult — one OUROBOROS cycle
// ---------------------------------------------------------------------------
#[pyclass(name = "CycleResult")]
#[derive(Clone)]
struct PyCycleResult {
    #[pyo3(get)]
    cycle: usize,
    #[pyo3(get)]
    domain: String,
    #[pyo3(get)]
    lenses_used: Vec<String>,
    #[pyo3(get)]
    new_discoveries: usize,
    #[pyo3(get)]
    graph_nodes: usize,
    #[pyo3(get)]
    graph_edges: usize,
    #[pyo3(get)]
    verification_score: f64,
}

#[pymethods]
impl PyCycleResult {
    fn __repr__(&self) -> String {
        format!(
            "CycleResult(cycle={}, discoveries={}, score={:.3})",
            self.cycle, self.new_discoveries, self.verification_score
        )
    }
}

impl From<&CycleResult> for PyCycleResult {
    fn from(cr: &CycleResult) -> Self {
        PyCycleResult {
            cycle: cr.cycle,
            domain: cr.domain.clone(),
            lenses_used: cr.lenses_used.clone(),
            new_discoveries: cr.new_discoveries,
            graph_nodes: cr.graph_nodes,
            graph_edges: cr.graph_edges,
            verification_score: cr.verification_score,
        }
    }
}

// ---------------------------------------------------------------------------
// PyForgeResult
// ---------------------------------------------------------------------------
#[pyclass(name = "ForgeResult")]
#[derive(Clone)]
struct PyForgeResult {
    #[pyo3(get)]
    candidates_generated: usize,
    #[pyo3(get)]
    candidates_accepted: usize,
    #[pyo3(get)]
    new_lenses: Vec<String>,
}

#[pymethods]
impl PyForgeResult {
    fn __repr__(&self) -> String {
        format!(
            "ForgeResult(generated={}, accepted={}, lenses={:?})",
            self.candidates_generated, self.candidates_accepted, self.new_lenses
        )
    }
}

// ---------------------------------------------------------------------------
// PyMetaLoopResult
// ---------------------------------------------------------------------------
#[pyclass(name = "MetaLoopResult")]
#[derive(Clone)]
struct PyMetaLoopResult {
    #[pyo3(get)]
    total_discoveries: usize,
    #[pyo3(get)]
    meta_cycles_completed: usize,
    #[pyo3(get)]
    forged_lenses: Vec<String>,
    #[pyo3(get)]
    ouroboros_results: Vec<PyCycleResult>,
}

#[pymethods]
impl PyMetaLoopResult {
    fn __repr__(&self) -> String {
        format!(
            "MetaLoopResult(meta_cycles={}, discoveries={}, forged={})",
            self.meta_cycles_completed,
            self.total_discoveries,
            self.forged_lenses.len()
        )
    }
}

// ---------------------------------------------------------------------------
// Module-level functions
// ---------------------------------------------------------------------------

/// Match a single value against n=6 constants.
/// Returns N6Match with constant_name and quality (1.0=EXACT, 0.8=CLOSE, 0.5=WEAK, 0.0=NONE).
#[pyfunction]
#[pyo3(name = "n6_check")]
fn py_n6_check(value: f64) -> PyN6Match {
    let (name, quality) = n6_check_mod::n6_match(value);
    PyN6Match {
        constant_name: name.to_string(),
        quality,
    }
}

/// Compute the EXACT ratio: fraction of values matching an n=6 constant exactly.
#[pyfunction]
fn feasibility_score(values: Vec<f64>) -> f64 {
    n6_check_mod::n6_exact_ratio(&values)
}

/// Full verification with 6-dimension scoring.
#[pyfunction]
#[pyo3(signature = (lens_consensus, cross_validation, physical_check, graph_bonus, novelty, n6_exact))]
fn verify(
    lens_consensus: f64,
    cross_validation: f64,
    physical_check: f64,
    graph_bonus: f64,
    novelty: f64,
    n6_exact: f64,
) -> PyVerificationResult {
    let result = feasibility::verify(
        lens_consensus,
        cross_validation,
        physical_check,
        graph_bonus,
        novelty,
        n6_exact,
    );
    PyVerificationResult {
        score: result.score,
        grade: result.grade.label().to_string(),
        lens_consensus: result.breakdown.lens_consensus,
        cross_validation: result.breakdown.cross_validation,
        physical_check: result.breakdown.physical_check,
        graph_bonus: result.breakdown.graph_bonus,
        novelty: result.breakdown.novelty,
        n6_exact: result.breakdown.n6_exact,
    }
}

/// Recommend lenses for a domain.
/// Returns LensRecommendation with lenses list and reason string.
#[pyfunction]
#[pyo3(signature = (domain, serendipity_ratio=0.2))]
fn recommend_lenses(domain: &str, serendipity_ratio: f64) -> PyLensRecommendation {
    let registry = LensRegistry::new();
    let all_lenses: Vec<String> = registry.iter().map(|(name, _)| name.clone()).collect();

    // Cold start — no history
    let all_stats = HashMap::new();

    let rec = recommend::recommend_lenses(domain, &all_stats, &all_lenses, serendipity_ratio);
    PyLensRecommendation {
        lenses: rec.lenses,
        reason: rec.reason,
    }
}

/// Run OUROBOROS evolution cycles for a domain.
/// Returns a list of CycleResult.
#[pyfunction]
#[pyo3(signature = (domain, max_cycles=6, seeds=None))]
fn evolve(domain: &str, max_cycles: usize, seeds: Option<Vec<String>>) -> Vec<PyCycleResult> {
    let mut config = EvolutionConfig::default();
    config.domain = domain.to_string();

    let registry = LensRegistry::new();
    config.all_lenses = registry.iter().map(|(name, _)| name.clone()).collect();

    let seed_list = seeds.unwrap_or_else(|| {
        vec![format!(
            "n=6 pattern in {} domain: sigma*phi=n*tau identity",
            domain
        )]
    });

    let mut engine = EvolutionEngine::new(config, seed_list);
    let (_status, history) = engine.run_loop(max_cycles);

    history.iter().map(PyCycleResult::from).collect()
}

/// Run LensForge to generate new lens candidates.
/// Returns ForgeResult with generated/accepted counts and new lens names.
#[pyfunction]
#[pyo3(signature = (max_candidates=20, min_confidence=0.2))]
fn forge_lenses(max_candidates: usize, min_confidence: f64) -> PyForgeResult {
    let registry = LensRegistry::new();
    let history = Vec::new(); // no history — pure gap analysis

    let config = ForgeConfig {
        max_candidates,
        min_confidence,
        similarity_threshold: 0.8,
    };

    let result = forge_engine::forge_cycle(&registry, &history, &config);

    PyForgeResult {
        candidates_generated: result.candidates_generated,
        candidates_accepted: result.candidates_accepted,
        new_lenses: result.new_lenses.iter().map(|e| e.name.clone()).collect(),
    }
}

/// Run the full OUROBOROS + LensForge meta-loop.
/// Returns MetaLoopResult with all cycle results, forged lenses, and totals.
#[pyfunction]
#[pyo3(signature = (domain, meta_cycles=6, ouroboros_cycles=6, seeds=None))]
fn auto(
    domain: &str,
    meta_cycles: usize,
    ouroboros_cycles: usize,
    seeds: Option<Vec<String>>,
) -> PyMetaLoopResult {
    let seed_list = seeds.unwrap_or_else(|| {
        vec![format!(
            "n=6 pattern in {} domain: sigma*phi=n*tau identity",
            domain
        )]
    });

    let config = MetaLoopConfig {
        max_ouroboros_cycles: ouroboros_cycles,
        max_meta_cycles: meta_cycles,
        forge_after_n_cycles: 0,
        forge_config: ForgeConfig::default(),
    };

    let meta_loop = MetaLoop::new(domain.to_string(), seed_list, config);
    let result = meta_loop.run();

    PyMetaLoopResult {
        total_discoveries: result.total_discoveries,
        meta_cycles_completed: result.meta_cycles_completed,
        forged_lenses: result.forged_lenses,
        ouroboros_results: result.ouroboros_results.iter().map(PyCycleResult::from).collect(),
    }
}

// ---------------------------------------------------------------------------
// Module registration
// ---------------------------------------------------------------------------

/// NEXUS-6 Discovery Engine — Python bindings.
///
/// Usage:
///   import nexus6
///   reg = nexus6.LensRegistry()
///   print(reg.len())                         # 775
///   print(nexus6.n6_check(12.0))             # N6Match(constant='sigma', quality=1.00, grade='EXACT')
///   print(nexus6.feasibility_score([12.0, 6.0, 24.0]))  # 1.0
#[pymodule]
fn nexus6(m: &Bound<'_, PyModule>) -> PyResult<()> {
    // Classes
    m.add_class::<PyLensRegistry>()?;
    m.add_class::<PyLensEntry>()?;
    m.add_class::<PyN6Match>()?;
    m.add_class::<PyLensRecommendation>()?;
    m.add_class::<PyVerificationResult>()?;
    m.add_class::<PyCycleResult>()?;
    m.add_class::<PyForgeResult>()?;
    m.add_class::<PyMetaLoopResult>()?;

    // Functions
    m.add_function(wrap_pyfunction!(py_n6_check, m)?)?;
    m.add_function(wrap_pyfunction!(feasibility_score, m)?)?;
    m.add_function(wrap_pyfunction!(verify, m)?)?;
    m.add_function(wrap_pyfunction!(recommend_lenses, m)?)?;
    m.add_function(wrap_pyfunction!(evolve, m)?)?;
    m.add_function(wrap_pyfunction!(forge_lenses, m)?)?;
    m.add_function(wrap_pyfunction!(auto, m)?)?;

    // Constants for convenience
    m.add("__version__", "0.1.0")?;
    m.add("N", 6)?;
    m.add("SIGMA", 12)?;
    m.add("PHI", 2)?;
    m.add("TAU", 4)?;
    m.add("J2", 24)?;

    Ok(())
}
