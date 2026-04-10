//! Improvement suggestions — structured recommendations for system enhancement.

/// A single improvement suggestion.
#[derive(Debug, Clone)]
pub struct ImprovementSuggestion {
    /// Category: "lens", "scan", "engine", "algorithm".
    pub category: String,
    /// Human-readable description.
    pub description: String,
    /// Expected improvement if applied.
    pub expected_improvement: String,
    /// Difficulty: "easy", "medium", "hard".
    pub difficulty: String,
}

impl ImprovementSuggestion {
    pub fn new(category: &str, description: &str, improvement: &str, difficulty: &str) -> Self {
        Self {
            category: category.to_string(),
            description: description.to_string(),
            expected_improvement: improvement.to_string(),
            difficulty: difficulty.to_string(),
        }
    }

    /// Priority score (higher = more impactful and easier).
    pub fn priority_score(&self) -> f64 {
        let difficulty_multiplier = match self.difficulty.as_str() {
            "easy" => 3.0,
            "medium" => 2.0,
            "hard" => 1.0,
            _ => 1.0,
        };
        let category_weight = match self.category.as_str() {
            "algorithm" => 1.5,
            "engine" => 1.3,
            "lens" => 1.1,
            "scan" => 1.0,
            _ => 1.0,
        };
        difficulty_multiplier * category_weight
    }
}

/// Sort suggestions by priority (highest first).
pub fn prioritize(suggestions: &mut [ImprovementSuggestion]) {
    suggestions.sort_by(|a, b| {
        b.priority_score()
            .partial_cmp(&a.priority_score())
            .unwrap_or(std::cmp::Ordering::Equal)
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suggestion_creation() {
        let s = ImprovementSuggestion::new("lens", "Add more lenses", "20% coverage", "easy");
        assert_eq!(s.category, "lens");
        assert_eq!(s.difficulty, "easy");
    }

    #[test]
    fn test_priority_score() {
        let easy = ImprovementSuggestion::new("algorithm", "desc", "improvement", "easy");
        let hard = ImprovementSuggestion::new("algorithm", "desc", "improvement", "hard");
        assert!(easy.priority_score() > hard.priority_score());
    }

    #[test]
    fn test_prioritize() {
        let mut suggestions = vec![
            ImprovementSuggestion::new("scan", "desc", "imp", "hard"),
            ImprovementSuggestion::new("algorithm", "desc", "imp", "easy"),
            ImprovementSuggestion::new("engine", "desc", "imp", "medium"),
        ];
        prioritize(&mut suggestions);
        assert_eq!(suggestions[0].category, "algorithm");
        assert_eq!(suggestions[0].difficulty, "easy");
    }

    #[test]
    fn test_category_weight() {
        let algo = ImprovementSuggestion::new("algorithm", "desc", "imp", "medium");
        let scan = ImprovementSuggestion::new("scan", "desc", "imp", "medium");
        assert!(algo.priority_score() > scan.priority_score());
    }

    #[test]
    fn test_prioritize_empty() {
        let mut suggestions: Vec<ImprovementSuggestion> = Vec::new();
        prioritize(&mut suggestions);
        assert!(suggestions.is_empty());
    }
}
