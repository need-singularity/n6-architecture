//! Reward signal computation for reinforcement-based growth.
/// Reward / Scoring System — score lens performance and discoveries
/// for prioritization within the NEXUS-6 discovery engine.

use std::collections::HashMap;

/// Types of reward signals emitted during discovery.
#[derive(Debug, Clone, PartialEq)]
pub enum RewardSignal {
    /// A known pattern was detected.
    PatternFound,
    /// Multiple lenses agree on a finding (consensus).
    ConsensusAchieved,
    /// A novel, previously unseen result was detected.
    NoveltyDetected,
    /// A value aligns with an n=6 constant.
    N6Alignment,
}

impl RewardSignal {
    /// Base score multiplier for each signal type.
    /// Aligned with n=6 divisor structure: 1, 2, 3, 6.
    pub fn base_multiplier(&self) -> f64 {
        match self {
            Self::PatternFound => 1.0,       // μ(6)=1
            Self::ConsensusAchieved => 2.0,  // φ(6)=2
            Self::NoveltyDetected => 3.0,    // n/φ = 6/2 = 3
            Self::N6Alignment => 6.0,        // n=6
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::PatternFound => "pattern_found",
            Self::ConsensusAchieved => "consensus",
            Self::NoveltyDetected => "novelty",
            Self::N6Alignment => "n6_alignment",
        }
    }
}

/// A single reward entry recording a scored event.
#[derive(Debug, Clone)]
pub struct RewardEntry {
    pub signal: RewardSignal,
    pub lens_name: String,
    /// Raw score before multiplier.
    pub score: f64,
    /// Unix timestamp (seconds).
    pub timestamp: u64,
}

impl RewardEntry {
    pub fn new(
        signal: RewardSignal,
        lens_name: impl Into<String>,
        score: f64,
        timestamp: u64,
    ) -> Self {
        Self {
            signal,
            lens_name: lens_name.into(),
            score,
            timestamp,
        }
    }

    /// Effective score = raw score * signal base multiplier.
    pub fn effective_score(&self) -> f64 {
        self.score * self.signal.base_multiplier()
    }
}

/// Tracker that accumulates reward scores per lens.
#[derive(Debug, Default)]
pub struct RewardTracker {
    entries: Vec<RewardEntry>,
    /// Accumulated effective score per lens name.
    scores: HashMap<String, f64>,
}

impl RewardTracker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Record a reward entry and update accumulated scores.
    pub fn record(&mut self, entry: RewardEntry) {
        let eff = entry.effective_score();
        *self.scores.entry(entry.lens_name.clone()).or_insert(0.0) += eff;
        self.entries.push(entry);
    }

    /// Get accumulated score for a specific lens.
    pub fn lens_score(&self, lens_name: &str) -> f64 {
        self.scores.get(lens_name).copied().unwrap_or(0.0)
    }

    /// Total number of recorded entries.
    pub fn entry_count(&self) -> usize {
        self.entries.len()
    }

    /// All entries (read-only).
    pub fn entries(&self) -> &[RewardEntry] {
        &self.entries
    }
}

/// Return the top `k` lenses by accumulated reward score.
/// Returns (lens_name, total_score) pairs sorted descending.
pub fn top_performers(tracker: &RewardTracker, k: usize) -> Vec<(String, f64)> {
    let mut pairs: Vec<(String, f64)> = tracker.scores.iter().map(|(k, v)| (k.clone(), *v)).collect();
    pairs.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
    pairs.truncate(k);
    pairs
}

/// Compute a novelty score for a result compared to historical results.
///
/// `result` is the new value to evaluate.
/// `history` is a slice of previous result values.
///
/// Returns a score in [0.0, 1.0] where 1.0 = maximally novel (far from
/// all history), 0.0 = already seen.
///
/// Method: minimum absolute distance to any historical value, normalized
/// by the range of history. Empty history → novelty = 1.0 (everything is new).
pub fn novelty_score(result: f64, history: &[f64]) -> f64 {
    if history.is_empty() {
        return 1.0;
    }

    let min_val = history.iter().cloned().fold(f64::INFINITY, f64::min);
    let max_val = history.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let range = max_val - min_val;

    // If all history values are the same, any different value is maximally novel
    if range < 1e-12 {
        return if (result - min_val).abs() < 1e-12 { 0.0 } else { 1.0 };
    }

    let min_dist = history
        .iter()
        .map(|&h| (result - h).abs())
        .fold(f64::INFINITY, f64::min);

    // Normalize by range, cap at 1.0
    (min_dist / range).min(1.0)
}

/// Reward proximity to n=6 constants.
///
/// Target constants: 6, 12(σ), 24(J₂), 48(σ·τ), 144(σ²).
/// For each value, find the closest n=6 constant and compute a reward
/// based on relative proximity. Returns average reward across all values.
///
/// Score formula per value: exp(-|v - nearest| / nearest)
/// Range: 1.0 = exact match, approaching 0.0 = far away.
pub fn n6_alignment_reward(values: &[f64]) -> f64 {
    if values.is_empty() {
        return 0.0;
    }

    // n=6 target constants: n, σ, J₂, σ·τ, σ²
    const N6_TARGETS: [f64; 5] = [6.0, 12.0, 24.0, 48.0, 144.0];

    let total: f64 = values
        .iter()
        .map(|&v| {
            let v_abs = v.abs();
            // Find closest target
            let (_, min_dist) = N6_TARGETS
                .iter()
                .map(|&t| (t, (v_abs - t).abs()))
                .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap();
            let nearest = N6_TARGETS
                .iter()
                .copied()
                .min_by(|a, b| {
                    (v_abs - a)
                        .abs()
                        .partial_cmp(&(v_abs - b).abs())
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .unwrap();
            // Exponential decay from nearest target
            (-min_dist / nearest).exp()
        })
        .sum();

    total / values.len() as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reward_signal_multipliers() {
        // Multipliers follow n=6 divisor structure: 1, 2, 3, 6
        assert!((RewardSignal::PatternFound.base_multiplier() - 1.0).abs() < 1e-12);
        assert!((RewardSignal::ConsensusAchieved.base_multiplier() - 2.0).abs() < 1e-12);
        assert!((RewardSignal::NoveltyDetected.base_multiplier() - 3.0).abs() < 1e-12);
        assert!((RewardSignal::N6Alignment.base_multiplier() - 6.0).abs() < 1e-12);
    }

    #[test]
    fn test_tracker_accumulation_and_top_performers() {
        let mut tracker = RewardTracker::new();
        tracker.record(RewardEntry::new(RewardSignal::PatternFound, "consciousness", 1.0, 100));
        tracker.record(RewardEntry::new(RewardSignal::N6Alignment, "topology", 2.0, 101));
        tracker.record(RewardEntry::new(RewardSignal::PatternFound, "consciousness", 0.5, 102));

        // consciousness: 1.0*1 + 0.5*1 = 1.5
        // topology: 2.0*6 = 12.0
        assert!((tracker.lens_score("consciousness") - 1.5).abs() < 1e-9);
        assert!((tracker.lens_score("topology") - 12.0).abs() < 1e-9);
        assert_eq!(tracker.entry_count(), 3);

        let top = top_performers(&tracker, 1);
        assert_eq!(top.len(), 1);
        assert_eq!(top[0].0, "topology");
        assert!((top[0].1 - 12.0).abs() < 1e-9);
    }

    #[test]
    fn test_novelty_score() {
        let history = vec![1.0, 2.0, 3.0, 4.0, 5.0];
        // Value already in history → novelty near 0
        assert!(novelty_score(3.0, &history) < 1e-9);
        // Value far outside → high novelty
        let far = novelty_score(10.0, &history);
        assert!(far > 0.9);
        // Empty history → everything is novel
        assert!((novelty_score(42.0, &[]) - 1.0).abs() < 1e-9);
    }

    #[test]
    fn test_n6_alignment_exact_matches() {
        // Exact n=6 constants should get reward = 1.0
        let exact = n6_alignment_reward(&[6.0, 12.0, 24.0, 48.0, 144.0]);
        assert!((exact - 1.0).abs() < 1e-9);

        // Values far from any target should get low reward
        let far = n6_alignment_reward(&[1000.0, 2000.0]);
        assert!(far < 0.1);
    }

    #[test]
    fn test_n6_alignment_close_values() {
        // Close to σ=12 should score high
        let close = n6_alignment_reward(&[12.1]);
        assert!(close > 0.99);

        // Midway values get moderate reward
        let mid = n6_alignment_reward(&[18.0]); // between 12 and 24
        let exact = n6_alignment_reward(&[12.0]);
        assert!(mid < exact);
    }

    #[test]
    fn test_effective_score() {
        let entry = RewardEntry::new(RewardSignal::N6Alignment, "test", 3.0, 0);
        // 3.0 * 6.0 = 18.0
        assert!((entry.effective_score() - 18.0).abs() < 1e-9);
    }

    #[test]
    fn test_top_performers_truncation() {
        let mut tracker = RewardTracker::new();
        for i in 0..10 {
            tracker.record(RewardEntry::new(
                RewardSignal::PatternFound,
                format!("lens_{}", i),
                (i + 1) as f64,
                i as u64,
            ));
        }
        let top3 = top_performers(&tracker, 3);
        assert_eq!(top3.len(), 3);
        // Highest score lens should be first
        assert_eq!(top3[0].0, "lens_9");
    }
}
