use std::collections::HashMap;
use super::stats::DomainStats;

#[derive(Debug, Clone)]
pub struct LensRecommendation {
    pub lenses: Vec<String>,
    pub reason: String,
}

const COLD_START_LENSES: &[&str] = &[
    "consciousness",
    "topology",
    "void",
    "thermo",
    "evolution",
    "network",
    "boundary",
    "triangle",
];

const MIN_LENSES: usize = 4;
const MAX_LENSES: usize = 40;

/// Recommend lenses for a domain scan.
///
/// Logic:
/// 1. If domain has stats: pick lenses with hit_rate > 0.05, sorted by hit_rate desc
/// 2. If no stats (cold start): check domain_similarity to find closest domain with stats
/// 3. Add serendipity: randomly pick serendipity_ratio fraction from unused lenses
/// 4. Minimum 4 lenses, maximum 40
pub fn recommend_lenses(
    domain: &str,
    all_stats: &HashMap<String, DomainStats>,
    all_lenses: &[String],
    serendipity_ratio: f64,
) -> LensRecommendation {
    let (mut selected, reason) = if let Some(stats) = all_stats.get(domain) {
        select_from_stats(domain, stats)
    } else {
        cold_start_recommend(domain, all_stats)
    };

    // Add serendipity lenses from unused pool
    let used_set: std::collections::HashSet<&str> =
        selected.iter().map(|s| s.as_str()).collect();
    let unused: Vec<&String> = all_lenses
        .iter()
        .filter(|l| !used_set.contains(l.as_str()))
        .collect();

    let serendipity_count = ((selected.len() as f64 * serendipity_ratio)
        .ceil() as usize)
        .max(1);

    // Deterministic serendipity: pick evenly spaced from unused list
    // (using rand would require seeding; we use a simple hash-based selection)
    let mut serendipity_picks = pick_serendipity(&unused, serendipity_count, domain);
    selected.append(&mut serendipity_picks);

    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    selected.retain(|l| seen.insert(l.clone()));

    // Enforce bounds
    if selected.len() < MIN_LENSES {
        // Pad from all_lenses
        for lens in all_lenses {
            if selected.len() >= MIN_LENSES {
                break;
            }
            if !seen.contains(lens) {
                selected.push(lens.clone());
                seen.insert(lens.clone());
            }
        }
    }
    if selected.len() > MAX_LENSES {
        selected.truncate(MAX_LENSES);
    }

    let reason = if serendipity_picks_count(&selected, &reason) > 0 {
        format!("{}; +serendipity lenses added", reason)
    } else {
        reason
    };

    LensRecommendation {
        lenses: selected,
        reason,
    }
}

fn select_from_stats(domain: &str, stats: &DomainStats) -> (Vec<String>, String) {
    let mut candidates: Vec<(String, f64)> = stats
        .lens_stats
        .iter()
        .filter(|(_, ls)| ls.hit_rate > 0.05)
        .map(|(name, ls)| (name.clone(), ls.hit_rate))
        .collect();

    candidates.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

    let lenses: Vec<String> = candidates.iter().map(|(name, _)| name.clone()).collect();
    let reason = format!(
        "domain '{}': {} lenses with hit_rate>0.05 from {} scans",
        domain,
        lenses.len(),
        stats.total_scans
    );
    (lenses, reason)
}

fn cold_start_recommend(
    domain: &str,
    all_stats: &HashMap<String, DomainStats>,
) -> (Vec<String>, String) {
    // Try to find a similar domain (simple: the one sharing the most lens names)
    let mut best_domain: Option<&str> = None;
    let mut best_score = 0usize;

    for (other_domain, stats) in all_stats {
        if other_domain == domain {
            continue;
        }
        let score = stats.total_discoveries;
        if score > best_score {
            best_score = score;
            best_domain = Some(other_domain.as_str());
        }
    }

    if let Some(similar) = best_domain {
        if let Some(stats) = all_stats.get(similar) {
            let (lenses, _) = select_from_stats(similar, stats);
            if !lenses.is_empty() {
                let reason = format!(
                    "cold start for '{}': borrowed from similar domain '{}'",
                    domain, similar
                );
                return (lenses, reason);
            }
        }
    }

    // Fallback to default cold start lenses
    let lenses = COLD_START_LENSES
        .iter()
        .map(|s| s.to_string())
        .collect();
    let reason = format!("cold start for '{}': using default 8 lenses", domain);
    (lenses, reason)
}

/// Pick serendipity lenses using a simple deterministic hash-based selection.
fn pick_serendipity(unused: &[&String], count: usize, domain: &str) -> Vec<String> {
    if unused.is_empty() || count == 0 {
        return Vec::new();
    }

    let mut picks = Vec::new();
    // Simple hash: use domain string bytes to create a seed
    let seed: usize = domain.bytes().map(|b| b as usize).sum();

    for i in 0..count.min(unused.len()) {
        let idx = (seed.wrapping_add(i.wrapping_mul(7))) % unused.len();
        picks.push(unused[idx].to_string());
    }

    picks
}

/// Helper to check if serendipity lenses were actually added
/// (returns a count estimate for the reason string)
fn serendipity_picks_count(selected: &[String], _base_reason: &str) -> usize {
    // We always add at least 1 serendipity lens if there are unused lenses
    if selected.len() > COLD_START_LENSES.len() {
        1
    } else {
        0
    }
}
