/// Devil's advocate — challenges discoveries by generating counter-arguments.

/// A challenge posed against a discovery or hypothesis.
#[derive(Debug, Clone)]
pub struct Challenge {
    pub discovery_id: String,
    pub challenge_type: ChallengeType,
    pub description: String,
    pub severity: f64, // 0.0..1.0
}

/// Types of adversarial challenges.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeType {
    /// Could this be random coincidence?
    RandomCoincidence,
    /// Is the sample size sufficient?
    SmallSample,
    /// Could confounding variables explain this?
    Confounding,
    /// Is this just overfitting / cherry-picking?
    Overfitting,
    /// Does this contradict established physics?
    PhysicsViolation,
    /// Is the n=6 expression post-hoc?
    PostHocRationalization,
}

/// Generate devil's advocate challenges for a discovery.
///
/// `n6_exact_ratio`: fraction of EXACT matches (0.0..1.0)
/// `domain_count`: how many independent domains confirm this
/// `sample_size`: number of data points
pub fn challenge_discovery(
    discovery_id: &str,
    n6_exact_ratio: f64,
    domain_count: usize,
    sample_size: usize,
) -> Vec<Challenge> {
    let mut challenges = Vec::new();

    // Always challenge random coincidence
    // P(k exact out of N by chance) ~ binomial, rough threshold
    let expected_random = 0.05; // ~5% chance any value lands near a constant
    if n6_exact_ratio < 0.5 {
        challenges.push(Challenge {
            discovery_id: discovery_id.to_string(),
            challenge_type: ChallengeType::RandomCoincidence,
            description: format!(
                "EXACT ratio {:.1}% is below 50% — could be noise (random baseline ~{:.0}%)",
                n6_exact_ratio * 100.0,
                expected_random * 100.0
            ),
            severity: 1.0 - n6_exact_ratio,
        });
    }

    // Small sample challenge
    // n=6 minimum: need at least σ-φ=10 data points for significance
    let min_sample = 10; // sigma - phi = 10
    if sample_size < min_sample {
        challenges.push(Challenge {
            discovery_id: discovery_id.to_string(),
            challenge_type: ChallengeType::SmallSample,
            description: format!(
                "Sample size {} < σ-φ={} minimum for significance",
                sample_size, min_sample
            ),
            severity: 1.0 - (sample_size as f64 / min_sample as f64).min(1.0),
        });
    }

    // Single-domain challenge
    if domain_count < 3 {
        challenges.push(Challenge {
            discovery_id: discovery_id.to_string(),
            challenge_type: ChallengeType::Confounding,
            description: format!(
                "Only {} domain(s) confirm — need 3+ for candidate consensus",
                domain_count
            ),
            severity: 1.0 - (domain_count as f64 / 3.0).min(1.0),
        });
    }

    // Post-hoc rationalization check: if ratio is suspiciously perfect (100%)
    // on very small samples, might be cherry-picked
    if n6_exact_ratio >= 1.0 && sample_size < 6 {
        challenges.push(Challenge {
            discovery_id: discovery_id.to_string(),
            challenge_type: ChallengeType::PostHocRationalization,
            description: format!(
                "100% EXACT on only {} samples — high cherry-picking risk",
                sample_size
            ),
            severity: 0.8,
        });
    }

    // Sort by severity descending
    challenges.sort_by(|a, b| {
        b.severity
            .partial_cmp(&a.severity)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    challenges
}

/// Overall credibility score after devil's advocate review.
/// 1.0 = bulletproof, 0.0 = completely debunked.
pub fn credibility_score(challenges: &[Challenge]) -> f64 {
    if challenges.is_empty() {
        return 1.0;
    }
    let max_severity = challenges
        .iter()
        .map(|c| c.severity)
        .fold(0.0_f64, f64::max);
    let mean_severity: f64 =
        challenges.iter().map(|c| c.severity).sum::<f64>() / challenges.len() as f64;

    // Credibility drops with max severity and mean severity
    (1.0 - 0.6 * max_severity - 0.4 * mean_severity).max(0.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strong_discovery() {
        // High EXACT ratio, many domains, large sample
        let challenges = challenge_discovery("BT-56", 0.95, 5, 100);
        // Should have few/no challenges
        let score = credibility_score(&challenges);
        assert!(score > 0.8, "Strong discovery should score >0.8, got {}", score);
    }

    #[test]
    fn test_weak_discovery() {
        // Low EXACT ratio, single domain, tiny sample
        let challenges = challenge_discovery("H-TEST-1", 0.2, 1, 3);
        assert!(challenges.len() >= 3, "Weak discovery should have 3+ challenges");
        let score = credibility_score(&challenges);
        assert!(score < 0.5, "Weak discovery should score <0.5, got {}", score);
    }

    #[test]
    fn test_cherry_pick_detection() {
        let challenges = challenge_discovery("H-TEST-2", 1.0, 1, 2);
        let has_posthoc = challenges
            .iter()
            .any(|c| c.challenge_type == ChallengeType::PostHocRationalization);
        assert!(has_posthoc, "Should detect post-hoc rationalization");
    }

    #[test]
    fn test_credibility_no_challenges() {
        assert!((credibility_score(&[]) - 1.0).abs() < 1e-12);
    }
}
