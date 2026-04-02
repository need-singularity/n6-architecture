use std::collections::HashSet;

use crate::telescope::registry::LensRegistry;

use super::candidate_gen::LensCandidate;
use super::gap_analyzer::GapReport;

/// Recommendation from the validator.
#[derive(Debug, Clone, PartialEq)]
pub enum Recommendation {
    /// Accept as a new lens.
    Accept,
    /// Needs modification before acceptance.
    Modify(String),
    /// Rejected — not useful or too similar.
    Reject(String),
}

/// Result of validating a candidate lens.
#[derive(Debug, Clone)]
pub struct ValidationResult {
    pub candidate: LensCandidate,
    pub is_unique: bool,
    pub is_useful: bool,
    pub similarity_to_existing: f64,
    pub recommendation: Recommendation,
}

/// Compute Jaccard similarity between two sets of domain affinities.
fn jaccard_similarity(a: &[String], b: &[String]) -> f64 {
    let set_a: HashSet<&str> = a.iter().map(|s| s.as_str()).collect();
    let set_b: HashSet<&str> = b.iter().map(|s| s.as_str()).collect();

    let intersection = set_a.intersection(&set_b).count();
    let union = set_a.union(&set_b).count();

    if union == 0 {
        return 0.0;
    }

    intersection as f64 / union as f64
}

/// Validate a candidate lens against the registry and gap report.
///
/// Checks:
/// 1. Name uniqueness
/// 2. Domain affinity similarity (Jaccard < 0.8 threshold)
/// 3. Usefulness: does it cover uncovered/weak domains?
pub fn validate(
    candidate: &LensCandidate,
    registry: &LensRegistry,
    gap: &GapReport,
    similarity_threshold: f64,
) -> ValidationResult {
    // 1. Name uniqueness check
    if registry.get(&candidate.name).is_some() {
        return ValidationResult {
            candidate: candidate.clone(),
            is_unique: false,
            is_useful: false,
            similarity_to_existing: 1.0,
            recommendation: Recommendation::Reject(format!(
                "Name '{}' already exists in registry",
                candidate.name
            )),
        };
    }

    // 2. Find maximum Jaccard similarity to any existing lens
    let mut max_similarity: f64 = 0.0;
    for (_name, entry) in registry.iter() {
        let sim = jaccard_similarity(&candidate.domain_affinity, &entry.domain_affinity);
        if sim > max_similarity {
            max_similarity = sim;
        }
    }

    let is_unique = max_similarity < similarity_threshold;

    // 3. Usefulness: does it cover any uncovered or weak domain?
    let uncovered: HashSet<&str> = gap
        .uncovered_domains
        .iter()
        .map(|s| s.as_str())
        .collect();
    let weak: HashSet<&str> = gap.weak_domains.iter().map(|(d, _)| d.as_str()).collect();

    let covers_uncovered = candidate.domain_affinity.iter().any(|d| {
        let dl = d.to_lowercase();
        uncovered.iter().any(|u| dl.contains(u))
    });
    let covers_weak = candidate.domain_affinity.iter().any(|d| {
        let dl = d.to_lowercase();
        weak.iter().any(|w| dl.contains(w))
    });

    let is_useful = covers_uncovered || covers_weak;

    // 4. Determine recommendation
    let recommendation = if !is_unique {
        Recommendation::Reject(format!(
            "Too similar to existing lens (Jaccard={:.2} >= {:.2})",
            max_similarity, similarity_threshold
        ))
    } else if !is_useful {
        Recommendation::Modify(
            "Candidate is unique but does not cover any gap domain; consider adjusting affinity"
                .to_string(),
        )
    } else {
        Recommendation::Accept
    };

    ValidationResult {
        candidate: candidate.clone(),
        is_unique,
        is_useful,
        similarity_to_existing: max_similarity,
        recommendation,
    }
}
