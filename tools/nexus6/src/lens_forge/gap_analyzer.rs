use std::collections::{HashMap, HashSet};

use crate::history::recorder::ScanRecord;
use crate::telescope::registry::LensRegistry;

/// Known domains in the NEXUS-6 universe.
/// Used as the ground truth for gap analysis.
const KNOWN_DOMAINS: &[&str] = &[
    "ai", "chip", "energy", "battery", "solar", "fusion",
    "superconductor", "quantum", "biology", "cosmology",
    "robotics", "materials", "blockchain", "network",
    "cryptography", "display", "audio", "environment",
    "mathematics", "software", "plasma", "compiler",
    "consciousness", "thermodynamics",
];

/// Report identifying gaps in the current lens coverage.
#[derive(Debug, Clone)]
pub struct GapReport {
    /// Domains with zero lens coverage.
    pub uncovered_domains: Vec<String>,
    /// Domains with weak lens coverage (domain, coverage_score).
    pub weak_domains: Vec<(String, f64)>,
    /// Suggested new lens categories to fill gaps.
    pub suggested_categories: Vec<String>,
}

/// Analyze gaps in the current lens registry given scan history.
///
/// For each known domain, check how many lenses have affinity for it
/// and how effective scans have been (from history). Domains with zero
/// lenses = uncovered; domains with low hit rates = weak.
pub fn analyze_gaps(
    registry: &LensRegistry,
    scan_history: &[ScanRecord],
) -> GapReport {
    // 1. Build domain -> lens count from registry
    let mut domain_lens_count: HashMap<String, usize> = HashMap::new();
    for domain in KNOWN_DOMAINS {
        domain_lens_count.insert(domain.to_string(), 0);
    }
    for (_name, entry) in registry.iter() {
        for affinity in &entry.domain_affinity {
            let aff_lower = affinity.to_lowercase();
            for domain in KNOWN_DOMAINS {
                if aff_lower.contains(domain) {
                    *domain_lens_count.entry(domain.to_string()).or_insert(0) += 1;
                }
            }
        }
    }

    // 2. Build domain -> hit rate from scan history
    let mut domain_scans: HashMap<String, usize> = HashMap::new();
    let mut domain_discoveries: HashMap<String, usize> = HashMap::new();
    for record in scan_history {
        let d = record.domain.to_lowercase();
        *domain_scans.entry(d.clone()).or_insert(0) += 1;
        *domain_discoveries.entry(d).or_insert(0) += record.discoveries.len();
    }

    // 3. Classify domains
    let mut uncovered_domains = Vec::new();
    let mut weak_domains = Vec::new();
    let mut suggested_categories = Vec::new();

    for domain in KNOWN_DOMAINS {
        let domain_str = domain.to_string();
        let lens_count = domain_lens_count.get(&domain_str).copied().unwrap_or(0);
        let scans = domain_scans.get(&domain_str).copied().unwrap_or(0);
        let discoveries = domain_discoveries.get(&domain_str).copied().unwrap_or(0);

        if lens_count == 0 {
            uncovered_domains.push(domain_str.clone());
            suggested_categories.push(format!("{}_specialist", domain));
            continue;
        }

        // Compute coverage score: weighted combination of lens density and hit rate
        let lens_density = (lens_count as f64).min(20.0) / 20.0; // normalize to 0..1
        let hit_rate = if scans > 0 {
            discoveries as f64 / scans as f64
        } else {
            0.0 // no scans = unknown effectiveness, treat as low
        };

        let coverage = 0.5 * lens_density + 0.5 * hit_rate;
        if coverage < 0.3 {
            weak_domains.push((domain_str.clone(), coverage));
        }
    }

    // Sort for determinism
    uncovered_domains.sort();
    weak_domains.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    suggested_categories.sort();

    // Deduplicate suggested categories
    let seen: HashSet<String> = suggested_categories.iter().cloned().collect();
    suggested_categories = seen.into_iter().collect();
    suggested_categories.sort();

    GapReport {
        uncovered_domains,
        weak_domains,
        suggested_categories,
    }
}
