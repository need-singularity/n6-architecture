//! Speculative hypothesis generation via stochastic exploration.
/// Dream Engine — recombines past discoveries into novel hypotheses.
/// Uses a simple deterministic hash-based "randomness" (no external crates).

/// A single dream: fragments recombined with novelty/plausibility scores.
#[derive(Debug, Clone)]
pub struct Dream {
    pub fragments: Vec<String>,
    pub recombination: String,
    pub novelty_score: f64,
    pub plausibility: f64,
}

/// The dream engine accumulates past discoveries and recombines them.
pub struct DreamEngine {
    past_discoveries: Vec<String>,
}

impl DreamEngine {
    pub fn new() -> Self {
        Self {
            past_discoveries: Vec::new(),
        }
    }

    /// Add a discovery to memory.
    pub fn add_memory(&mut self, discovery: &str) {
        self.past_discoveries.push(discovery.to_string());
    }

    /// Number of memories stored.
    pub fn memory_count(&self) -> usize {
        self.past_discoveries.len()
    }

    /// Generate `n_dreams` dreams by recombining past discoveries.
    /// Each dream picks a different subset of fragments using deterministic rotation.
    /// Returns empty vec if fewer than 2 memories exist.
    pub fn dream(&self, n_dreams: usize) -> Vec<Dream> {
        if self.past_discoveries.len() < 2 {
            return Vec::new();
        }

        let n_mem = self.past_discoveries.len();
        let mut dreams = Vec::with_capacity(n_dreams);

        for i in 0..n_dreams {
            // Pick 2-3 fragments per dream using rotation
            let frag_count = 2 + (i % 2); // alternates 2 and 3
            let mut fragments = Vec::with_capacity(frag_count);
            for j in 0..frag_count {
                let idx = (i * 7 + j * 3 + 1) % n_mem; // deterministic pseudo-random
                fragments.push(self.past_discoveries[idx].clone());
            }

            let recombination = recombine_fragments(&fragments);
            let novelty_score = compute_novelty(&fragments, n_mem, i);
            let plausibility = compute_plausibility(&fragments);

            dreams.push(Dream {
                fragments,
                recombination,
                novelty_score,
                plausibility,
            });
        }

        dreams
    }

    /// Lucid dream: intentional recombination seeded by a keyword.
    /// Picks fragments whose text contains the seed (case-insensitive),
    /// then recombines them.  Falls back to first 2 memories if no match.
    pub fn lucid_dream(&self, seed: &str) -> Dream {
        let seed_lower = seed.to_lowercase();
        let mut fragments: Vec<String> = self
            .past_discoveries
            .iter()
            .filter(|d| d.to_lowercase().contains(&seed_lower))
            .cloned()
            .collect();

        if fragments.len() < 2 {
            // fallback: take first available memories
            fragments = self
                .past_discoveries
                .iter()
                .take(2.min(self.past_discoveries.len()))
                .cloned()
                .collect();
        }

        // Cap at 4 fragments
        fragments.truncate(4);

        let recombination = recombine_fragments(&fragments);
        let novelty_score = compute_novelty(&fragments, self.past_discoveries.len(), 42);
        let plausibility = compute_plausibility(&fragments);

        Dream {
            fragments,
            recombination,
            novelty_score,
            plausibility,
        }
    }
}

impl Default for DreamEngine {
    fn default() -> Self {
        Self::new()
    }
}

/// Recombine fragments into a hypothesis string.
fn recombine_fragments(fragments: &[String]) -> String {
    if fragments.is_empty() {
        return "empty dream".to_string();
    }
    // Extract domain-like tokens from each fragment and cross-pollinate
    let tokens: Vec<&str> = fragments
        .iter()
        .flat_map(|f| f.split_whitespace())
        .collect();

    // Build a recombination by interleaving tokens from different fragments
    let half = tokens.len() / 2;
    let front = &tokens[..half.min(tokens.len())];
    let back = &tokens[half.min(tokens.len())..];
    format!(
        "Cross-domain hypothesis: [{}] <-> [{}]",
        front.join(" "),
        back.join(" ")
    )
}

/// Novelty: higher when fragments are distant in the memory list.
fn compute_novelty(fragments: &[String], total_memories: usize, seed: usize) -> f64 {
    if total_memories == 0 || fragments.is_empty() {
        return 0.0;
    }
    // Simple heuristic: average fragment length diversity + seed jitter
    let avg_len: f64 =
        fragments.iter().map(|f| f.len() as f64).sum::<f64>() / fragments.len() as f64;
    let variance: f64 = fragments
        .iter()
        .map(|f| {
            let diff = f.len() as f64 - avg_len;
            diff * diff
        })
        .sum::<f64>()
        / fragments.len() as f64;
    let jitter = ((seed as f64 * 0.1).sin().abs()) * 0.2;
    (variance.sqrt() / (avg_len + 1.0) + jitter).min(1.0)
}

/// Plausibility: higher when fragments share common tokens.
fn compute_plausibility(fragments: &[String]) -> f64 {
    if fragments.len() < 2 {
        return 0.5;
    }
    // Count shared words between first and rest
    let first_words: Vec<&str> = fragments[0].split_whitespace().collect();
    let mut shared = 0_usize;
    let mut total = 0_usize;
    for frag in &fragments[1..] {
        for word in frag.split_whitespace() {
            total += 1;
            if first_words.contains(&word) {
                shared += 1;
            }
        }
    }
    if total == 0 {
        return 0.5;
    }
    // Normalize to 0..1
    (shared as f64 / total as f64).min(1.0).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dream_requires_minimum_memories() {
        let engine = DreamEngine::new();
        assert!(engine.dream(5).is_empty());

        let mut engine2 = DreamEngine::new();
        engine2.add_memory("only one");
        assert!(engine2.dream(3).is_empty());
    }

    #[test]
    fn test_dream_produces_output() {
        let mut engine = DreamEngine::new();
        engine.add_memory("BT-33 Transformer sigma=12 atom");
        engine.add_memory("BT-43 Battery cathode CN=6 universality");
        engine.add_memory("BT-97 Weinberg angle sin2theta = 3/13");

        let dreams = engine.dream(4);
        assert_eq!(dreams.len(), 4);
        for d in &dreams {
            assert!(!d.fragments.is_empty());
            assert!(!d.recombination.is_empty());
            assert!(d.novelty_score >= 0.0 && d.novelty_score <= 1.0);
            assert!(d.plausibility >= 0.0 && d.plausibility <= 1.0);
        }
    }

    #[test]
    fn test_lucid_dream_with_seed() {
        let mut engine = DreamEngine::new();
        engine.add_memory("BT-33 Transformer sigma=12");
        engine.add_memory("BT-43 Battery cathode CN=6");
        engine.add_memory("BT-97 Transformer angle");

        let d = engine.lucid_dream("Transformer");
        // Should pick fragments containing "Transformer"
        assert!(d.fragments.len() >= 2);
        assert!(d
            .fragments
            .iter()
            .any(|f| f.to_lowercase().contains("transformer")));
    }

    #[test]
    fn test_lucid_dream_fallback() {
        let mut engine = DreamEngine::new();
        engine.add_memory("alpha");
        engine.add_memory("beta");

        // No match for "zzz" -> fallback to first 2
        let d = engine.lucid_dream("zzz");
        assert_eq!(d.fragments.len(), 2);
    }
}
