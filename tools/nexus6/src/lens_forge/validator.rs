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

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::candidate_gen::CandidateSource;
    use super::super::gap_analyzer::GapReport;

    fn make_candidate(name: &str, domains: &[&str], confidence: f64) -> LensCandidate {
        LensCandidate {
            name: name.into(),
            description: format!("Test lens {}", name),
            source: CandidateSource::GapFill("test".into()),
            domain_affinity: domains.iter().map(|s| s.to_string()).collect(),
            complementary: vec![],
            confidence,
        }
    }

    fn make_gap(uncovered: &[&str], weak: &[(&str, f64)]) -> GapReport {
        GapReport {
            uncovered_domains: uncovered.iter().map(|s| s.to_string()).collect(),
            weak_domains: weak.iter().map(|(d, s)| (d.to_string(), *s)).collect(),
            suggested_categories: vec![],
        }
    }

    #[test]
    fn test_jaccard_similarity_identical() {
        let a = vec!["ai".to_string(), "chip".to_string()];
        assert!((jaccard_similarity(&a, &a) - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_jaccard_similarity_disjoint() {
        let a = vec!["ai".to_string(), "chip".to_string()];
        let b = vec!["energy".to_string(), "fusion".to_string()];
        assert_eq!(jaccard_similarity(&a, &b), 0.0);
    }

    #[test]
    fn test_jaccard_similarity_partial() {
        let a = vec!["ai".to_string(), "chip".to_string(), "energy".to_string()];
        let b = vec!["ai".to_string(), "fusion".to_string(), "energy".to_string()];
        // intersection=2 (ai,energy), union=4 => 0.5
        assert!((jaccard_similarity(&a, &b) - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_validate_unique_and_useful() {
        let registry = LensRegistry::new();
        let gap = make_gap(&["consciousness"], &[]);
        let candidate = make_candidate("n6_test_unique_lens_xyz", &["consciousness"], 0.8);

        let result = validate(&candidate, &registry, &gap, 0.8);
        assert!(result.is_unique);
        assert!(result.is_useful);
        assert_eq!(result.recommendation, Recommendation::Accept);
    }

    #[test]
    fn test_validate_not_useful_no_gap_coverage() {
        let registry = LensRegistry::new();
        let gap = make_gap(&["consciousness"], &[]); // only consciousness is uncovered
        // Candidate covers "zzz_nonexistent" which is not an uncovered/weak domain
        let candidate = make_candidate("n6_test_niche_lens", &["zzz_nonexistent"], 0.5);

        let result = validate(&candidate, &registry, &gap, 0.8);
        assert!(result.is_unique);
        assert!(!result.is_useful);
        assert!(matches!(result.recommendation, Recommendation::Modify(_)));
    }

    #[test]
    fn test_jaccard_empty_sets() {
        let empty: Vec<String> = vec![];
        assert_eq!(jaccard_similarity(&empty, &empty), 0.0);
    }
}
