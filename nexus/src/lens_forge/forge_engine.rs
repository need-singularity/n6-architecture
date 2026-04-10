use crate::history::recorder::ScanRecord;
use crate::telescope::registry::{LensCategory, LensEntry, LensRegistry};

use super::candidate_gen::{
    generate_from_analogy, generate_from_combination, generate_from_mutation, LensCandidate,
};
use super::gap_analyzer::{analyze_gaps, GapReport};
use super::validator::{validate, Recommendation, ValidationResult};

/// Configuration for the forge engine.
#[derive(Debug, Clone)]
pub struct ForgeConfig {
    /// Maximum candidates to consider per strategy.
    pub max_candidates: usize,
    /// Minimum confidence threshold for candidates.
    pub min_confidence: f64,
    /// Jaccard similarity threshold for uniqueness (reject if >= this).
    pub similarity_threshold: f64,
}

impl Default for ForgeConfig {
    fn default() -> Self {
        ForgeConfig {
            max_candidates: 20,
            min_confidence: 0.2,
            similarity_threshold: 0.8,
        }
    }
}

/// Result of a forge cycle.
#[derive(Debug, Clone)]
pub struct ForgeResult {
    /// Total candidates generated across all strategies.
    pub candidates_generated: usize,
    /// Candidates that passed validation.
    pub candidates_accepted: usize,
    /// New lenses ready for registration.
    pub new_lenses: Vec<LensEntry>,
    /// The gap report used for this cycle.
    pub gap_report: GapReport,
    /// All validation results for transparency.
    pub validations: Vec<ValidationResult>,
}

/// Convert an accepted candidate into a LensEntry.
fn candidate_to_entry(candidate: &LensCandidate) -> LensEntry {
    LensEntry {
        name: candidate.name.clone(),
        category: LensCategory::Custom,
        description: candidate.description.clone(),
        domain_affinity: candidate.domain_affinity.clone(),
        complementary: candidate.complementary.clone(),
    }
}

/// Run one complete forge cycle:
/// 1. Analyze gaps in current registry
/// 2. Generate candidates from three strategies
/// 3. Validate each candidate
/// 4. Return accepted lenses
pub fn forge_cycle(
    registry: &LensRegistry,
    history: &[ScanRecord],
    config: &ForgeConfig,
) -> ForgeResult {
    // 1. Gap analysis
    let gap_report = analyze_gaps(registry, history);

    // 2. Generate candidates from all strategies
    let mut all_candidates: Vec<LensCandidate> = Vec::new();

    let mut combo = generate_from_combination(registry, &gap_report);
    combo.truncate(config.max_candidates);
    all_candidates.extend(combo);

    let mut analogy = generate_from_analogy(registry, &gap_report);
    analogy.truncate(config.max_candidates);
    all_candidates.extend(analogy);

    let mut mutation = generate_from_mutation(registry);
    mutation.truncate(config.max_candidates);
    all_candidates.extend(mutation);

    // Filter by minimum confidence
    all_candidates.retain(|c| c.confidence >= config.min_confidence);

    let candidates_generated = all_candidates.len();

    // 3. Validate each candidate
    let mut validations = Vec::new();
    let mut new_lenses = Vec::new();

    for candidate in &all_candidates {
        let result = validate(
            candidate,
            registry,
            &gap_report,
            config.similarity_threshold,
        );

        if result.recommendation == Recommendation::Accept {
            new_lenses.push(candidate_to_entry(candidate));
        }

        validations.push(result);
    }

    let candidates_accepted = new_lenses.len();

    ForgeResult {
        candidates_generated,
        candidates_accepted,
        new_lenses,
        gap_report,
        validations,
    }
}
