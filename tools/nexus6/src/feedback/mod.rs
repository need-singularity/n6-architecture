//! Feedback loop for growth quality tracking and adjustment.
/// Feedback loop — collect user feedback on discoveries to improve lens weights.
pub mod collector;
pub mod learner;

/// User feedback on a discovery.
#[derive(Debug, Clone, PartialEq)]
pub enum Feedback {
    Good { discovery_id: String },
    Bad { discovery_id: String, reason: String },
    Interesting { discovery_id: String },
    Irrelevant { discovery_id: String },
}

impl Feedback {
    /// The discovery ID this feedback refers to.
    pub fn discovery_id(&self) -> &str {
        match self {
            Self::Good { discovery_id } => discovery_id,
            Self::Bad { discovery_id, .. } => discovery_id,
            Self::Interesting { discovery_id } => discovery_id,
            Self::Irrelevant { discovery_id } => discovery_id,
        }
    }

    /// Type tag for serialization/filtering.
    pub fn type_tag(&self) -> &'static str {
        match self {
            Self::Good { .. } => "good",
            Self::Bad { .. } => "bad",
            Self::Interesting { .. } => "interesting",
            Self::Irrelevant { .. } => "irrelevant",
        }
    }

    /// Numeric score: good=+1.0, interesting=+0.5, irrelevant=-0.5, bad=-1.0.
    pub fn score(&self) -> f64 {
        match self {
            Self::Good { .. } => 1.0,
            Self::Interesting { .. } => 0.5,
            Self::Irrelevant { .. } => -0.5,
            Self::Bad { .. } => -1.0,
        }
    }
}

/// Aggregated feedback statistics.
#[derive(Debug, Clone)]
pub struct FeedbackStats {
    pub total: usize,
    pub good: usize,
    pub bad: usize,
    pub interesting: usize,
    pub irrelevant: usize,
    pub good_rate: f64,
}

/// Collector that stores timestamped feedback and provides persistence.
pub struct FeedbackCollector {
    feedbacks: Vec<(String, Feedback)>, // (timestamp, feedback)
    path: String,
}

impl FeedbackCollector {
    /// Create a new collector with a file path for persistence.
    pub fn new(path: &str) -> Self {
        Self {
            feedbacks: Vec::new(),
            path: path.to_string(),
        }
    }

    /// Record a feedback item with current timestamp.
    pub fn record(&mut self, feedback: Feedback) {
        let ts = Self::now_timestamp();
        self.feedbacks.push((ts, feedback));
    }

    /// Load feedback from the backing file (one JSON line per entry).
    pub fn load(&mut self) {
        let content = match std::fs::read_to_string(&self.path) {
            Ok(c) => c,
            Err(_) => return,
        };

        for line in content.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            if let Some(fb) = Self::parse_feedback_line(line) {
                self.feedbacks.push(fb);
            }
        }
    }

    /// Save all feedback to the backing file.
    pub fn save(&self) {
        let mut content = String::new();
        for (ts, fb) in &self.feedbacks {
            content.push_str(&Self::serialize_feedback_line(ts, fb));
            content.push('\n');
        }
        let _ = std::fs::write(&self.path, &content);
    }

    /// Compute aggregate statistics.
    pub fn stats(&self) -> FeedbackStats {
        let total = self.feedbacks.len();
        let mut good = 0usize;
        let mut bad = 0usize;
        let mut interesting = 0usize;
        let mut irrelevant = 0usize;

        for (_, fb) in &self.feedbacks {
            match fb {
                Feedback::Good { .. } => good += 1,
                Feedback::Bad { .. } => bad += 1,
                Feedback::Interesting { .. } => interesting += 1,
                Feedback::Irrelevant { .. } => irrelevant += 1,
            }
        }

        let good_rate = if total > 0 {
            good as f64 / total as f64
        } else {
            0.0
        };

        FeedbackStats {
            total,
            good,
            bad,
            interesting,
            irrelevant,
            good_rate,
        }
    }

    /// Return all recorded feedbacks.
    pub fn all(&self) -> &[(String, Feedback)] {
        &self.feedbacks
    }

    /// Number of recorded feedbacks.
    pub fn len(&self) -> usize {
        self.feedbacks.len()
    }

    /// Whether empty.
    pub fn is_empty(&self) -> bool {
        self.feedbacks.is_empty()
    }

    fn now_timestamp() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let dur = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default();
        format!("{}.{:03}", dur.as_secs(), dur.subsec_millis())
    }

    /// Serialize a feedback entry to a simple tab-separated line.
    fn serialize_feedback_line(ts: &str, fb: &Feedback) -> String {
        match fb {
            Feedback::Good { discovery_id } => {
                format!("{}\tgood\t{}", ts, discovery_id)
            }
            Feedback::Bad { discovery_id, reason } => {
                format!("{}\tbad\t{}\t{}", ts, discovery_id, reason)
            }
            Feedback::Interesting { discovery_id } => {
                format!("{}\tinteresting\t{}", ts, discovery_id)
            }
            Feedback::Irrelevant { discovery_id } => {
                format!("{}\tirrelevant\t{}", ts, discovery_id)
            }
        }
    }

    /// Parse a serialized feedback line back into (timestamp, Feedback).
    fn parse_feedback_line(line: &str) -> Option<(String, Feedback)> {
        let parts: Vec<&str> = line.splitn(4, '\t').collect();
        if parts.len() < 3 {
            return None;
        }
        let ts = parts[0].to_string();
        let fb = match parts[1] {
            "good" => Feedback::Good {
                discovery_id: parts[2].to_string(),
            },
            "bad" => {
                let reason = if parts.len() >= 4 { parts[3] } else { "" };
                Feedback::Bad {
                    discovery_id: parts[2].to_string(),
                    reason: reason.to_string(),
                }
            }
            "interesting" => Feedback::Interesting {
                discovery_id: parts[2].to_string(),
            },
            "irrelevant" => Feedback::Irrelevant {
                discovery_id: parts[2].to_string(),
            },
            _ => return None,
        };
        Some((ts, fb))
    }
}

/// Update lens weights based on feedback.
///
/// Returns a list of (lens_name, weight_delta) adjustments.
/// Positive feedback on discoveries found by certain lenses boosts those lenses.
pub fn update_weights_from_feedback(feedbacks: &[Feedback]) -> Vec<(String, f64)> {
    learner::compute_weight_updates(feedbacks)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feedback_scores() {
        assert_eq!(Feedback::Good { discovery_id: "d1".into() }.score(), 1.0);
        assert_eq!(Feedback::Interesting { discovery_id: "d2".into() }.score(), 0.5);
        assert_eq!(Feedback::Irrelevant { discovery_id: "d3".into() }.score(), -0.5);
        assert_eq!(Feedback::Bad { discovery_id: "d4".into(), reason: "wrong".into() }.score(), -1.0);
    }

    #[test]
    fn test_feedback_type_tags() {
        let fb = Feedback::Good { discovery_id: "x".into() };
        assert_eq!(fb.type_tag(), "good");
        assert_eq!(fb.discovery_id(), "x");
    }

    #[test]
    fn test_collector_record_and_stats() {
        let mut collector = FeedbackCollector::new("/tmp/nexus6_test_feedback.jsonl");
        assert!(collector.is_empty());

        // Record 6 feedbacks (n=6)
        collector.record(Feedback::Good { discovery_id: "d1".into() });
        collector.record(Feedback::Good { discovery_id: "d2".into() });
        collector.record(Feedback::Interesting { discovery_id: "d3".into() });
        collector.record(Feedback::Bad { discovery_id: "d4".into(), reason: "noise".into() });
        collector.record(Feedback::Irrelevant { discovery_id: "d5".into() });
        collector.record(Feedback::Good { discovery_id: "d6".into() });

        assert_eq!(collector.len(), 6);

        let stats = collector.stats();
        assert_eq!(stats.total, 6);
        assert_eq!(stats.good, 3);
        assert_eq!(stats.bad, 1);
        assert_eq!(stats.interesting, 1);
        assert_eq!(stats.irrelevant, 1);
        assert!((stats.good_rate - 0.5).abs() < 1e-10);
    }

    #[test]
    fn test_collector_net_score_and_top_discoveries() {
        let mut collector = FeedbackCollector::new("/tmp/nexus6_test_fb2.jsonl");

        // Discovery d1 gets mixed feedback
        collector.record(Feedback::Good { discovery_id: "d1".into() });
        collector.record(Feedback::Good { discovery_id: "d1".into() });
        collector.record(Feedback::Bad { discovery_id: "d1".into(), reason: "x".into() });
        // d2 gets only good
        collector.record(Feedback::Good { discovery_id: "d2".into() });

        assert!((collector.net_score("d1") - 1.0).abs() < 1e-10); // +1+1-1 = 1.0
        assert!((collector.net_score("d2") - 1.0).abs() < 1e-10);
        assert!((collector.net_score("d_nonexistent") - 0.0).abs() < 1e-10);

        let top = collector.top_discoveries(6);
        assert!(!top.is_empty());
    }

    #[test]
    fn test_collector_serialize_parse_roundtrip() {
        let mut collector = FeedbackCollector::new("/tmp/nexus6_test_fb3.jsonl");
        collector.record(Feedback::Good { discovery_id: "sigma-lens-12".into() });
        collector.record(Feedback::Bad { discovery_id: "phi-lens-2".into(), reason: "too noisy".into() });

        // Save and reload
        collector.save();

        let mut collector2 = FeedbackCollector::new("/tmp/nexus6_test_fb3.jsonl");
        collector2.load();
        assert_eq!(collector2.len(), 2);

        let stats = collector2.stats();
        assert_eq!(stats.good, 1);
        assert_eq!(stats.bad, 1);

        // Cleanup
        let _ = std::fs::remove_file("/tmp/nexus6_test_fb3.jsonl");
    }

    #[test]
    fn test_learner_weight_updates() {
        let feedbacks = vec![
            Feedback::Good { discovery_id: "ai-lens-1".into() },
            Feedback::Good { discovery_id: "ai-lens-2".into() },
            Feedback::Bad { discovery_id: "chip-lens-1".into(), reason: "noise".into() },
        ];
        let updates = update_weights_from_feedback(&feedbacks);
        // "ai" should have positive delta, "chip" should have negative
        let ai_update = updates.iter().find(|(name, _)| name == "ai");
        let chip_update = updates.iter().find(|(name, _)| name == "chip");
        assert!(ai_update.is_some());
        assert!(chip_update.is_some());
        assert!(ai_update.unwrap().1 > 0.0);
        assert!(chip_update.unwrap().1 < 0.0);
    }

    #[test]
    fn test_learner_apply_updates_clamp() {
        let current = vec![
            ("consciousness".to_string(), 1.5),
            ("topology".to_string(), 0.1),
        ];
        let updates = vec![
            ("consciousness".to_string(), 0.8),  // 1.5+0.8=2.3 -> clamp to 2.0
            ("topology".to_string(), -0.5),       // 0.1-0.5=-0.4 -> clamp to 0.0
        ];
        let result = learner::apply_updates(&current, &updates);
        assert_eq!(result[0].1, 2.0); // clamped upper
        assert_eq!(result[1].1, 0.0); // clamped lower
    }
}
