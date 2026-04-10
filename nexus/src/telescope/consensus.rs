use std::collections::HashMap;

use serde::{Deserialize, Serialize};

/// Consensus level based on number of agreeing lenses.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConsensusLevel {
    /// 3+ lenses agree
    Candidate,
    /// 7+ lenses agree
    High,
    /// 12+ lenses agree
    Confirmed,
}

/// A consensus result for a detected pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsensusResult {
    /// Identifier for the pattern (e.g. "void_at_3", "barrier_0_1")
    pub pattern_id: String,
    /// Names of agreeing lenses
    pub agreeing_lenses: Vec<String>,
    /// Weighted score (sum of hit_rate weights for agreeing lenses)
    pub weighted_score: f64,
    /// Consensus level
    pub level: ConsensusLevel,
}

use crate::telescope::lens_trait::LensResult;

/// Compute weighted consensus across multiple lens results.
///
/// `results`: lens_name -> LensResult (metric_name -> values)
/// `hit_rates`: lens_name -> historical accuracy weight (0.0..1.0)
///
/// Returns consensus results for patterns that at least 3 lenses agree on.
pub fn weighted_consensus(
    results: &HashMap<String, LensResult>,
    hit_rates: &HashMap<String, f64>,
) -> Vec<ConsensusResult> {
    // Collect all pattern IDs across all lenses
    // A lens "agrees" on a pattern if it has non-empty results for that metric
    let mut pattern_map: HashMap<String, Vec<(String, f64)>> = HashMap::new();

    for (lens_name, lr) in results {
        let weight = hit_rates.get(lens_name).copied().unwrap_or(1.0);
        for (metric_name, values) in lr {
            if !values.is_empty() {
                pattern_map
                    .entry(metric_name.clone())
                    .or_default()
                    .push((lens_name.clone(), weight));
            }
        }
    }

    let mut consensus_results = Vec::new();

    for (pattern_id, agreeing) in pattern_map {
        let count = agreeing.len();
        if count < 3 {
            continue;
        }

        let weighted_score: f64 = agreeing.iter().map(|(_, w)| w).sum();
        let agreeing_lenses: Vec<String> = agreeing.into_iter().map(|(name, _)| name).collect();

        let level = if count >= 12 {
            ConsensusLevel::Confirmed
        } else if count >= 7 {
            ConsensusLevel::High
        } else {
            ConsensusLevel::Candidate
        };

        consensus_results.push(ConsensusResult {
            pattern_id,
            agreeing_lenses,
            weighted_score,
            level,
        });
    }

    // Sort by weighted score descending
    consensus_results.sort_by(|a, b| {
        b.weighted_score
            .partial_cmp(&a.weighted_score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    consensus_results
}
