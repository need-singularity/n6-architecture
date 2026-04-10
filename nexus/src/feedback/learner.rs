/// Feedback learner — derive lens weight adjustments from user feedback patterns.

use super::Feedback;
use std::collections::HashMap;

/// Compute weight updates for lenses based on feedback.
///
/// The algorithm:
/// 1. Extract discovery IDs and their associated lens names (from ID format: "domain-lens-N").
/// 2. Sum feedback scores per lens.
/// 3. Normalize to [-1.0, +1.0] range.
/// 4. Return as (lens_name, weight_delta).
pub fn compute_weight_updates(feedbacks: &[Feedback]) -> Vec<(String, f64)> {
    let mut lens_scores: HashMap<String, f64> = HashMap::new();
    let mut lens_counts: HashMap<String, usize> = HashMap::new();

    for fb in feedbacks {
        // Extract lens hint from discovery ID (format: "domain-tier-N" or "domain-lens-N")
        let id = fb.discovery_id();
        let parts: Vec<&str> = id.split('-').collect();
        let lens_name = if parts.len() >= 2 {
            parts[0].to_string()
        } else {
            "unknown".to_string()
        };

        *lens_scores.entry(lens_name.clone()).or_insert(0.0) += fb.score();
        *lens_counts.entry(lens_name).or_insert(0) += 1;
    }

    // Normalize: average score per lens, then scale by learning rate (1/n=1/6)
    let learning_rate = 1.0 / 6.0; // n=6

    let mut updates: Vec<(String, f64)> = lens_scores
        .iter()
        .map(|(name, total_score)| {
            let count = *lens_counts.get(name).unwrap_or(&1) as f64;
            let avg_score = total_score / count;
            let delta = avg_score * learning_rate;
            (name.clone(), delta)
        })
        .collect();

    updates.sort_by(|a, b| a.0.cmp(&b.0));
    updates
}

/// Given weight updates and current weights, produce new weights (clamped to [0.0, 2.0]).
pub fn apply_updates(
    current: &[(String, f64)],
    updates: &[(String, f64)],
) -> Vec<(String, f64)> {
    let update_map: HashMap<&str, f64> = updates.iter().map(|(k, v)| (k.as_str(), *v)).collect();

    current
        .iter()
        .map(|(name, weight)| {
            let delta = update_map.get(name.as_str()).copied().unwrap_or(0.0);
            let new_weight = (weight + delta).clamp(0.0, 2.0);
            (name.clone(), new_weight)
        })
        .collect()
}
