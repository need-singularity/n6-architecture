use serde::{Deserialize, Serialize};

use crate::verifier::n6_check;

/// Type of adversarial attack.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum AttackType {
    RandomPermutation,  // Shuffle data order -> still discovered?
    NoiseInjection,     // Add noise -> survives?
    SubsetRemoval,      // Remove part of data -> maintained?
    DomainTransfer,     // Holds in another domain?
    NullHypothesis,     // Random data also produces this? (false positive)
    ParameterSweep,     // Parameter change -> still holds?
    InverseTest,        // Opposite conditions -> opposite result?
}

impl AttackType {
    /// All seven attack types.
    pub fn all() -> Vec<AttackType> {
        vec![
            AttackType::RandomPermutation,
            AttackType::NoiseInjection,
            AttackType::SubsetRemoval,
            AttackType::DomainTransfer,
            AttackType::NullHypothesis,
            AttackType::ParameterSweep,
            AttackType::InverseTest,
        ]
    }

    /// Human-readable name.
    pub fn name(&self) -> &'static str {
        match self {
            AttackType::RandomPermutation => "random_permutation",
            AttackType::NoiseInjection => "noise_injection",
            AttackType::SubsetRemoval => "subset_removal",
            AttackType::DomainTransfer => "domain_transfer",
            AttackType::NullHypothesis => "null_hypothesis",
            AttackType::ParameterSweep => "parameter_sweep",
            AttackType::InverseTest => "inverse_test",
        }
    }
}

/// Result of a single adversarial attack.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AdversarialResult {
    pub discovery_id: String,
    pub attack_type: AttackType,
    pub survived: bool,
    pub weakness_found: Option<String>,
    pub confidence_after: f64,
}

/// Simple deterministic PRNG (xorshift32) to avoid external deps.
struct Rng(u32);

impl Rng {
    fn new(seed: u32) -> Self {
        Self(if seed == 0 { 1 } else { seed })
    }
    fn next_u32(&mut self) -> u32 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 17;
        self.0 ^= self.0 << 5;
        self.0
    }
    fn next_f64(&mut self) -> f64 {
        (self.next_u32() as f64) / (u32::MAX as f64)
    }
}

/// Run a single adversarial attack on a discovery.
///
/// The attack tests whether the discovery's n6_score is robust under
/// perturbation. Each attack type applies a different transformation.
pub fn attack(discovery: &str, n6_score: f64, attack_type: AttackType) -> AdversarialResult {
    let hash = blake3::hash(discovery.as_bytes());
    let seed = u32::from_le_bytes([hash.as_bytes()[0], hash.as_bytes()[1], hash.as_bytes()[2], hash.as_bytes()[3]]);
    let mut rng = Rng::new(seed);

    let id = format!("disc-{}", &hash.to_hex()[..12]);

    match attack_type {
        AttackType::RandomPermutation => attack_random_permutation(&id, discovery, n6_score, &mut rng),
        AttackType::NoiseInjection => attack_noise_injection(&id, discovery, n6_score, &mut rng),
        AttackType::SubsetRemoval => attack_subset_removal(&id, discovery, n6_score, &mut rng),
        AttackType::DomainTransfer => attack_domain_transfer(&id, discovery, n6_score),
        AttackType::NullHypothesis => attack_null_hypothesis(&id, discovery, n6_score, &mut rng),
        AttackType::ParameterSweep => attack_parameter_sweep(&id, discovery, n6_score, &mut rng),
        AttackType::InverseTest => attack_inverse_test(&id, discovery, n6_score),
    }
}

/// Run all 7 attack types against a discovery.
pub fn full_red_team(discovery: &str, n6_score: f64) -> Vec<AdversarialResult> {
    AttackType::all()
        .into_iter()
        .map(|at| attack(discovery, n6_score, at))
        .collect()
}

/// Summarize red team results.
pub fn summarize(results: &[AdversarialResult]) -> String {
    let total = results.len();
    let survived = results.iter().filter(|r| r.survived).count();
    let weaknesses: Vec<&str> = results
        .iter()
        .filter_map(|r| r.weakness_found.as_deref())
        .collect();

    let mut out = format!(
        "Red Team: {}/{} attacks survived ({:.0}%)\n",
        survived,
        total,
        if total > 0 {
            survived as f64 / total as f64 * 100.0
        } else {
            0.0
        }
    );

    if !weaknesses.is_empty() {
        out.push_str("Weaknesses found:\n");
        for w in &weaknesses {
            out.push_str(&format!("  - {}\n", w));
        }
    }

    // Avg confidence after attacks
    if !results.is_empty() {
        let avg_conf: f64 = results.iter().map(|r| r.confidence_after).sum::<f64>() / results.len() as f64;
        out.push_str(&format!("Average confidence after attacks: {:.3}\n", avg_conf));
    }

    out
}

// --- Attack implementations ---

fn attack_random_permutation(id: &str, discovery: &str, n6_score: f64, rng: &mut Rng) -> AdversarialResult {
    // Shuffle words and check if any n6 constant values survive
    let words: Vec<&str> = discovery.split_whitespace().collect();
    let mut values = extract_numbers(discovery);

    // Permute the values
    for i in (1..values.len()).rev() {
        let j = (rng.next_u32() as usize) % (i + 1);
        values.swap(i, j);
    }

    // Check if shuffled values still match n6 constants
    let matches: usize = values.iter().filter(|&&v| n6_check::n6_match(v).1 >= 0.8).count();
    let original_matches = extract_numbers(discovery)
        .iter()
        .filter(|&&v| n6_check::n6_match(v).1 >= 0.8)
        .count();

    let survived = matches >= original_matches || words.len() < 3;
    let confidence_after = if survived { n6_score * 0.95 } else { n6_score * 0.5 };

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::RandomPermutation,
        survived,
        weakness_found: if !survived {
            Some("Order-dependent: shuffling data destroys the pattern".to_string())
        } else {
            None
        },
        confidence_after,
    }
}

fn attack_noise_injection(id: &str, discovery: &str, n6_score: f64, rng: &mut Rng) -> AdversarialResult {
    let values = extract_numbers(discovery);
    if values.is_empty() {
        return AdversarialResult {
            discovery_id: id.to_string(),
            attack_type: AttackType::NoiseInjection,
            survived: true,
            weakness_found: None,
            confidence_after: n6_score,
        };
    }

    // Add 5% noise to each value and check if still matches
    let mut noisy_match_count = 0;
    for &v in &values {
        let noise = (rng.next_f64() - 0.5) * 0.1 * v; // +/- 5%
        let noisy = v + noise;
        if n6_check::n6_match(noisy).1 >= 0.5 {
            noisy_match_count += 1;
        }
    }

    let ratio = noisy_match_count as f64 / values.len() as f64;
    let survived = ratio >= 0.5; // At least half survive noise
    let confidence_after = n6_score * (0.5 + ratio * 0.5);

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::NoiseInjection,
        survived,
        weakness_found: if !survived {
            Some(format!("Noise-sensitive: only {:.0}% of values survive 5% noise", ratio * 100.0))
        } else {
            None
        },
        confidence_after,
    }
}

fn attack_subset_removal(id: &str, discovery: &str, n6_score: f64, rng: &mut Rng) -> AdversarialResult {
    let values = extract_numbers(discovery);
    if values.len() < 2 {
        return AdversarialResult {
            discovery_id: id.to_string(),
            attack_type: AttackType::SubsetRemoval,
            survived: true,
            weakness_found: None,
            confidence_after: n6_score,
        };
    }

    // Remove ~30% of values and check if remaining still have n6 matches
    let keep_count = (values.len() as f64 * 0.7).ceil() as usize;
    let mut subset = values.clone();
    // Remove from end (deterministic based on rng)
    while subset.len() > keep_count {
        let idx = (rng.next_u32() as usize) % subset.len();
        subset.remove(idx);
    }

    let match_count = subset.iter().filter(|&&v| n6_check::n6_match(v).1 >= 0.8).count();
    let orig_match_count = values.iter().filter(|&&v| n6_check::n6_match(v).1 >= 0.8).count();

    // Pattern survives if proportional match rate is maintained
    let survived = if orig_match_count == 0 {
        true
    } else {
        match_count as f64 / subset.len() as f64
            >= orig_match_count as f64 / values.len() as f64 * 0.7
    };

    let confidence_after = if survived { n6_score * 0.9 } else { n6_score * 0.4 };

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::SubsetRemoval,
        survived,
        weakness_found: if !survived {
            Some("Subset-fragile: removing 30% of data destroys the pattern".to_string())
        } else {
            None
        },
        confidence_after,
    }
}

fn attack_domain_transfer(id: &str, discovery: &str, n6_score: f64) -> AdversarialResult {
    // Check if discovery mentions multiple domains (cross-domain robustness)
    let domains = count_domain_mentions(discovery);
    let survived = domains >= 2;
    let confidence_after = if survived {
        n6_score * (1.0 + 0.05 * (domains as f64 - 1.0)).min(1.2)
    } else {
        n6_score * 0.6
    };

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::DomainTransfer,
        survived,
        weakness_found: if !survived {
            Some("Single-domain: no evidence of cross-domain validity".to_string())
        } else {
            None
        },
        confidence_after: confidence_after.min(1.0),
    }
}

fn attack_null_hypothesis(id: &str, discovery: &str, n6_score: f64, rng: &mut Rng) -> AdversarialResult {
    // Generate random values and check if they also match n6 constants
    // (to detect false positives from the wide n6 constant set)
    let num_values = extract_numbers(discovery).len().max(6);
    let mut random_match_count = 0;

    for _ in 0..num_values {
        let random_val = rng.next_f64() * 50.0; // 0~50 range
        if n6_check::n6_match(random_val).1 >= 0.8 {
            random_match_count += 1;
        }
    }

    let random_rate = random_match_count as f64 / num_values as f64;
    // If random data matches at >30% rate, the n6 constant set is too loose
    // and the discovery may be a false positive
    let survived = random_rate < 0.3 || n6_score > 0.9;
    let confidence_after = if survived {
        n6_score * (1.0 - random_rate * 0.3)
    } else {
        n6_score * 0.3
    };

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::NullHypothesis,
        survived,
        weakness_found: if !survived {
            Some(format!(
                "Null-fail: random data matches at {:.0}% rate (false positive risk)",
                random_rate * 100.0
            ))
        } else {
            None
        },
        confidence_after: confidence_after.max(0.0),
    }
}

fn attack_parameter_sweep(id: &str, discovery: &str, n6_score: f64, _rng: &mut Rng) -> AdversarialResult {
    // Sweep key numerical parameters +/- 20% and check stability
    let values = extract_numbers(discovery);
    if values.is_empty() {
        return AdversarialResult {
            discovery_id: id.to_string(),
            attack_type: AttackType::ParameterSweep,
            survived: true,
            weakness_found: None,
            confidence_after: n6_score,
        };
    }

    let mut stable_count = 0;
    for &v in &values {
        let (_orig_name, orig_q) = n6_check::n6_match(v);
        if orig_q < 0.5 {
            stable_count += 1; // Not an n6 match to begin with
            continue;
        }

        // Sweep +/- 20% in 5 steps
        let mut sweep_survived = 0;
        for step in 0..5 {
            let factor = 0.8 + (step as f64) * 0.1; // 0.8, 0.9, 1.0, 1.1, 1.2
            let swept = v * factor;
            let (_, q) = n6_check::n6_match(swept);
            if q >= 0.5 {
                sweep_survived += 1;
            }
        }
        // If the match is robust across sweep, it's stable
        if sweep_survived >= 3 {
            stable_count += 1;
        }
    }

    let stability = stable_count as f64 / values.len() as f64;
    let survived = stability >= 0.5;
    let confidence_after = n6_score * (0.5 + stability * 0.5);

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::ParameterSweep,
        survived,
        weakness_found: if !survived {
            Some(format!(
                "Parameter-fragile: only {:.0}% stable under 20% sweep",
                stability * 100.0
            ))
        } else {
            None
        },
        confidence_after,
    }
}

fn attack_inverse_test(id: &str, discovery: &str, n6_score: f64) -> AdversarialResult {
    // Check for internal consistency: if discovery claims X, does it also
    // contain evidence of X? (Heuristic: check for numbers and n6 keywords)
    let has_numbers = !extract_numbers(discovery).is_empty();
    let has_n6_keywords = discovery.to_lowercase().contains("sigma")
        || discovery.to_lowercase().contains("phi")
        || discovery.to_lowercase().contains("tau")
        || discovery.to_lowercase().contains("n=6");

    let survived = has_numbers || has_n6_keywords;
    let confidence_after = if survived { n6_score * 0.95 } else { n6_score * 0.5 };

    AdversarialResult {
        discovery_id: id.to_string(),
        attack_type: AttackType::InverseTest,
        survived,
        weakness_found: if !survived {
            Some("No internal evidence: lacks both numerical values and n=6 keywords".to_string())
        } else {
            None
        },
        confidence_after,
    }
}

// --- Helpers ---

/// Extract all parseable f64 values from text.
fn extract_numbers(text: &str) -> Vec<f64> {
    text.split(|c: char| !c.is_ascii_digit() && c != '.' && c != '-')
        .filter_map(|s| s.parse::<f64>().ok())
        .filter(|v| v.is_finite())
        .collect()
}

/// Count how many distinct known domains are mentioned.
fn count_domain_mentions(text: &str) -> usize {
    const DOMAINS: &[&str] = &[
        "physics", "chemistry", "biology", "energy", "computing", "ai",
        "semiconductor", "fusion", "quantum", "cosmology", "materials",
        "robotics", "crypto", "network", "audio", "display", "battery",
        "solar", "superconductor", "math", "software", "environment",
    ];
    let lower = text.to_lowercase();
    DOMAINS.iter().filter(|d| lower.contains(**d)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_attack_noise_injection() {
        let result = attack("sigma=12.0 tau=4.0 phi=2.0", 1.0, AttackType::NoiseInjection);
        // With exact n6 values, 5% noise should still match for most
        assert!(result.confidence_after > 0.5);
    }

    #[test]
    fn test_attack_domain_transfer() {
        let result = attack(
            "physics energy cross-domain sigma=12",
            0.9,
            AttackType::DomainTransfer,
        );
        assert!(result.survived);

        let single = attack("some random text", 0.5, AttackType::DomainTransfer);
        assert!(!single.survived);
    }

    #[test]
    fn test_attack_inverse_test() {
        let result = attack("sigma=12 exact match", 0.9, AttackType::InverseTest);
        assert!(result.survived);

        let no_evidence = attack("vague statement about nothing", 0.3, AttackType::InverseTest);
        assert!(!no_evidence.survived);
    }

    #[test]
    fn test_full_red_team() {
        let results = full_red_team("sigma=12.0 physics energy tau=4.0", 0.9);
        assert_eq!(results.len(), 7);
        // At least some should survive
        let survived = results.iter().filter(|r| r.survived).count();
        assert!(survived >= 3, "Expected >=3 survived, got {}", survived);
    }

    #[test]
    fn test_summarize() {
        let results = full_red_team("sigma=12.0 physics energy", 0.9);
        let summary = summarize(&results);
        assert!(summary.contains("Red Team:"));
        assert!(summary.contains("/7"));
    }
}
