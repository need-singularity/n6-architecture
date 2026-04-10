use std::collections::HashSet;

use crate::telescope::registry::{LensEntry, LensRegistry};

use super::gap_analyzer::GapReport;

/// Source of a candidate lens.
#[derive(Debug, Clone, PartialEq)]
pub enum CandidateSource {
    /// Combination of two existing lenses.
    Combination(String, String),
    /// Analogy: transfer from a strong domain to a weak one.
    Analogy(String),
    /// Gap-fill: created to cover an uncovered domain.
    GapFill(String),
    /// Mutation: variant of an existing lens with shifted affinity.
    Mutation(String),
}

/// A candidate lens proposed by the forge.
#[derive(Debug, Clone)]
pub struct LensCandidate {
    pub name: String,
    pub description: String,
    pub source: CandidateSource,
    pub domain_affinity: Vec<String>,
    pub complementary: Vec<String>,
    pub confidence: f64,
}

/// Generate candidates by combining pairs of existing lenses.
///
/// For each pair of lenses whose combined domain_affinity covers an
/// uncovered domain, produce a combination candidate.
pub fn generate_from_combination(
    registry: &LensRegistry,
    gap: &GapReport,
) -> Vec<LensCandidate> {
    let uncovered: HashSet<&str> = gap
        .uncovered_domains
        .iter()
        .map(|s| s.as_str())
        .collect();

    if uncovered.is_empty() {
        return Vec::new();
    }

    // Collect lens entries as a vector for pair iteration
    let entries: Vec<&LensEntry> = registry.iter().map(|(_, e)| e).collect();
    let mut candidates = Vec::new();
    let mut seen_names: HashSet<String> = HashSet::new();

    // Limit iteration for performance — take up to 100 entries
    let limit = entries.len().min(100);

    for i in 0..limit {
        for j in (i + 1)..limit {
            let a = &entries[i];
            let b = &entries[j];

            // Merge domain affinities
            let mut merged: HashSet<String> = HashSet::new();
            for d in &a.domain_affinity {
                merged.insert(d.to_lowercase());
            }
            for d in &b.domain_affinity {
                merged.insert(d.to_lowercase());
            }

            // Check if merged covers any uncovered domain
            for uc in &uncovered {
                if merged.iter().any(|m| m.contains(uc)) {
                    let name = format!("{}_{}_combo", a.name, b.name);
                    if seen_names.contains(&name) {
                        continue;
                    }
                    seen_names.insert(name.clone());

                    let mut affinity: Vec<String> = merged.iter().cloned().collect();
                    affinity.sort();

                    candidates.push(LensCandidate {
                        name,
                        description: format!(
                            "Combination of {} and {} to cover {}",
                            a.name, b.name, uc
                        ),
                        source: CandidateSource::Combination(
                            a.name.clone(),
                            b.name.clone(),
                        ),
                        domain_affinity: affinity,
                        complementary: vec![a.name.clone(), b.name.clone()],
                        confidence: 0.6,
                    });
                    break; // one candidate per pair
                }
            }
        }
        // Cap total candidates
        if candidates.len() >= 20 {
            break;
        }
    }

    candidates
}

/// Generate candidates by analogy: transfer strong-domain lenses to weak domains.
///
/// For each weak domain, find lenses from strong domains and create
/// analogical variants targeting the weak domain.
pub fn generate_from_analogy(
    registry: &LensRegistry,
    gap: &GapReport,
) -> Vec<LensCandidate> {
    let weak_domains: Vec<&str> = gap
        .weak_domains
        .iter()
        .map(|(d, _)| d.as_str())
        .collect();

    if weak_domains.is_empty() {
        return Vec::new();
    }

    let mut candidates = Vec::new();

    // Find lenses that have high domain affinity count (strong lenses)
    let mut strong_lenses: Vec<&LensEntry> = registry
        .iter()
        .map(|(_, e)| e)
        .filter(|e| e.domain_affinity.len() >= 2)
        .collect();
    strong_lenses.sort_by(|a, b| b.domain_affinity.len().cmp(&a.domain_affinity.len()));
    strong_lenses.truncate(50);

    for weak_domain in &weak_domains {
        for lens in &strong_lenses {
            let name = format!("{}_to_{}", lens.name, weak_domain);
            let mut affinity = vec![weak_domain.to_string()];
            for d in &lens.domain_affinity {
                affinity.push(d.clone());
            }
            affinity.sort();
            affinity.dedup();

            candidates.push(LensCandidate {
                name,
                description: format!(
                    "Analogical transfer of {} to {} domain",
                    lens.name, weak_domain
                ),
                source: CandidateSource::Analogy(lens.name.clone()),
                domain_affinity: affinity,
                complementary: vec![lens.name.clone()],
                confidence: 0.4,
            });

            if candidates.len() >= 20 {
                return candidates;
            }
        }
    }

    candidates
}

/// Generate candidates by mutating existing lenses' domain affinities.
///
/// Take existing lenses and produce variants where one affinity is
/// shifted to a different related domain.
pub fn generate_from_mutation(registry: &LensRegistry) -> Vec<LensCandidate> {
    let mut candidates = Vec::new();
    let entries: Vec<&LensEntry> = registry.iter().map(|(_, e)| e).collect();

    // Domain shift pairs — related domains that could benefit from cross-pollination
    let shifts: &[(&str, &str)] = &[
        ("ai", "robotics"),
        ("chip", "quantum"),
        ("energy", "fusion"),
        ("biology", "materials"),
        ("network", "blockchain"),
        ("software", "compiler"),
    ];

    for entry in entries.iter().take(50) {
        for (from, to) in shifts {
            let has_from = entry
                .domain_affinity
                .iter()
                .any(|d| d.to_lowercase().contains(from));
            if !has_from {
                continue;
            }

            let has_to = entry
                .domain_affinity
                .iter()
                .any(|d| d.to_lowercase().contains(to));
            if has_to {
                continue; // already covers target
            }

            let name = format!("{}_mut_{}", entry.name, to);
            let mut affinity = entry.domain_affinity.clone();
            affinity.push(to.to_string());
            affinity.sort();

            candidates.push(LensCandidate {
                name,
                description: format!(
                    "Mutation of {} shifting {} -> {}",
                    entry.name, from, to
                ),
                source: CandidateSource::Mutation(entry.name.clone()),
                domain_affinity: affinity,
                complementary: vec![entry.name.clone()],
                confidence: 0.3,
            });

            if candidates.len() >= 20 {
                return candidates;
            }
        }
    }

    candidates
}
