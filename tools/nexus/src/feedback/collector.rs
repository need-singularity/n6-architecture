/// Feedback collector utilities — batch operations and filtering.

use super::{Feedback, FeedbackCollector};

impl FeedbackCollector {
    /// Record multiple feedbacks at once.
    pub fn record_batch(&mut self, feedbacks: Vec<Feedback>) {
        for fb in feedbacks {
            self.record(fb);
        }
    }

    /// Filter feedbacks by discovery ID.
    pub fn for_discovery(&self, discovery_id: &str) -> Vec<&Feedback> {
        self.feedbacks
            .iter()
            .filter(|(_, fb)| fb.discovery_id() == discovery_id)
            .map(|(_, fb)| fb)
            .collect()
    }

    /// Net score for a specific discovery (sum of all feedback scores).
    pub fn net_score(&self, discovery_id: &str) -> f64 {
        self.for_discovery(discovery_id)
            .iter()
            .map(|fb| fb.score())
            .sum()
    }

    /// Top-rated discoveries (sorted by net score, descending).
    pub fn top_discoveries(&self, n: usize) -> Vec<(String, f64)> {
        use std::collections::HashMap;
        let mut scores: HashMap<String, f64> = HashMap::new();
        for (_, fb) in &self.feedbacks {
            *scores.entry(fb.discovery_id().to_string()).or_insert(0.0) += fb.score();
        }
        let mut sorted: Vec<_> = scores.into_iter().collect();
        sorted.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
        sorted.truncate(n);
        sorted
    }
}
