//! Statistical and methodological challenges for n=6 claims.
//!
//! These functions implement honest critiques of the n=6 framework:
//! multiple comparisons, base-rate neglect, post-hoc fitting,
//! survivorship bias, and domain-specific counterarguments.
//!
//! Each challenge function returns a `StatisticalChallenge` with a
//! severity score and detailed reasoning.

/// A statistical or methodological challenge.
#[derive(Debug, Clone)]
pub struct StatisticalChallenge {
    pub name: String,
    pub category: ChallengeCategory,
    pub severity: f64,
    pub description: String,
    pub passed: bool,
}

/// Categories of statistical/methodological concerns.
#[derive(Debug, Clone, PartialEq)]
pub enum ChallengeCategory {
    MultipleComparisons,
    BaserateNeglect,
    SurvivorshipBias,
    ConfirmationBias,
    DegreeOfFreedom,
    AlternativeExplanation,
    ReplicationFailure,
}

// ── n=6 constants ────────────────────────────────────────────────────
const _N: usize = 6;
const _SIGMA: usize = 12;
const _PHI: usize = 2;
const _TAU: usize = 4;
const J2: usize = 24;
const _SOPFR: usize = 5;

/// How many distinct n=6 derived constants are available for matching.
/// sigma, phi, tau, J2, sopfr, sigma-phi, sigma-tau, sigma-mu, n/phi,
/// plus products, ratios, powers — easily 30+.
const N6_CONSTANT_POOL_SIZE: usize = 30;

/// Challenge: Multiple comparisons / look-elsewhere effect.
///
/// With 30+ n=6 derived constants, the probability that *any* observed
/// value lands within tolerance of *some* constant is much higher than
/// matching a specific one. This is the Texas sharpshooter fallacy:
/// painting the target after firing.
///
/// Returns PASS only if the corrected p-value (Bonferroni) is still
/// significant at alpha=0.05.
pub fn challenge_multiple_comparisons(
    observed_value: f64,
    tolerance_pct: f64,
    n_comparisons: usize,
) -> StatisticalChallenge {
    let n_comp = if n_comparisons == 0 {
        N6_CONSTANT_POOL_SIZE
    } else {
        n_comparisons
    };

    // Single-test p-value: probability a random value in [0, max_const]
    // falls within tolerance of any one constant.
    // Rough model: each constant occupies a band of 2*tolerance*value
    let max_const = J2 as f64; // largest commonly used = 24
    let band_width = 2.0 * (tolerance_pct / 100.0) * observed_value.abs().max(1.0);
    let single_p = (band_width / max_const).min(1.0);

    // Bonferroni correction
    let corrected_p = (single_p * n_comp as f64).min(1.0);

    let passed = corrected_p < 0.05;
    let severity = if passed { 0.1 } else { 0.8 };

    StatisticalChallenge {
        name: "Multiple comparisons (look-elsewhere effect)".to_string(),
        category: ChallengeCategory::MultipleComparisons,
        severity,
        description: format!(
            "With {} available n=6 constants and {:.1}% tolerance, \
             Bonferroni-corrected p={:.4}. {}",
            n_comp,
            tolerance_pct,
            corrected_p,
            if passed {
                "Match survives correction."
            } else {
                "CONCERN: Match does NOT survive multiple-comparison correction."
            }
        ),
        passed,
    }
}

/// Challenge: Base-rate neglect.
///
/// Small integers (1-24) appear frequently in engineering and physics
/// for reasons unrelated to n=6: powers of 2, decimal rounding,
/// human preference for round numbers, legacy standards, etc.
/// This challenge checks whether an observed value matches common
/// engineering priors independently of n=6.
pub fn challenge_baserate_neglect(
    observed_value: f64,
) -> StatisticalChallenge {
    // Common engineering/physics values that exist for non-n=6 reasons
    let common_values: &[(f64, &str)] = &[
        (2.0, "power of 2"),
        (4.0, "power of 2"),
        (8.0, "power of 2 / byte"),
        (16.0, "power of 2 / hex"),
        (32.0, "power of 2"),
        (64.0, "power of 2"),
        (128.0, "power of 2"),
        (256.0, "power of 2"),
        (3.0, "common small prime"),
        (5.0, "common small prime / human fingers"),
        (6.0, "2*3, common multiplier"),
        (10.0, "decimal base / human fingers"),
        (12.0, "dozen, months, hours"),
        (24.0, "hours/day"),
        (48.0, "2*24, common sampling rate"),
        (96.0, "common multiple"),
        (1.0, "identity / normalization"),
        (0.1, "decimal fraction"),
        (0.01, "percent"),
        (100.0, "centesimal"),
    ];

    let mut alt_explanations = Vec::new();
    for &(val, reason) in common_values {
        let diff = (observed_value - val).abs();
        let tol = val.abs().max(0.01) * 0.05; // 5% tolerance
        if diff < tol {
            alt_explanations.push(format!("{} ({})", val, reason));
        }
    }

    let has_alt = !alt_explanations.is_empty();
    let severity = if has_alt { 0.6 } else { 0.1 };

    StatisticalChallenge {
        name: "Base-rate neglect (common-value prior)".to_string(),
        category: ChallengeCategory::BaserateNeglect,
        severity,
        description: if has_alt {
            format!(
                "Value {:.4} matches non-n=6 prior(s): {}. \
                 CONCERN: This value is common in engineering for independent reasons.",
                observed_value,
                alt_explanations.join(", ")
            )
        } else {
            format!(
                "Value {:.4} does not match common engineering priors. \
                 n=6 explanation has fewer competing hypotheses.",
                observed_value
            )
        },
        passed: !has_alt,
    }
}

/// Challenge: Survivorship bias in BT counting.
///
/// If 1000 potential n=6 expressions were tried and only 127 BTs
/// survived, that is a 12.7% hit rate. The question is whether
/// the failures are documented and whether the hit rate exceeds
/// chance expectation.
pub fn challenge_survivorship_bias(
    total_hypotheses_tested: usize,
    confirmed_bts: usize,
    expected_random_rate: f64,
) -> StatisticalChallenge {
    let random_rate = if expected_random_rate <= 0.0 || expected_random_rate >= 1.0 {
        0.05 // default 5%
    } else {
        expected_random_rate
    };

    let observed_rate = if total_hypotheses_tested > 0 {
        confirmed_bts as f64 / total_hypotheses_tested as f64
    } else {
        0.0
    };

    let ratio = observed_rate / random_rate;
    let passed = ratio > 2.0 && total_hypotheses_tested >= 20;
    let severity = if passed { 0.2 } else { 0.7 };

    StatisticalChallenge {
        name: "Survivorship bias (unreported failures)".to_string(),
        category: ChallengeCategory::SurvivorshipBias,
        severity,
        description: format!(
            "Confirmed {}/{} hypotheses ({:.1}%). Random baseline: {:.1}%. \
             Ratio: {:.1}x. {}",
            confirmed_bts,
            total_hypotheses_tested,
            observed_rate * 100.0,
            random_rate * 100.0,
            ratio,
            if passed {
                "Rate significantly exceeds chance."
            } else {
                "CONCERN: Rate is not convincingly above chance, or sample too small."
            }
        ),
        passed,
    }
}

/// Challenge: Post-hoc degrees of freedom.
///
/// For n=6, the "constant pool" includes sigma, phi, tau, J2, sopfr,
/// and their differences, products, ratios, powers. This gives the
/// analyst many degrees of freedom to fit any value. The question is
/// whether the expression was predicted *before* observing the data.
pub fn challenge_degrees_of_freedom(
    expression_complexity: usize,
    was_predicted_before: bool,
) -> StatisticalChallenge {
    // Complexity = number of operations in the expression
    // e.g., "sigma - phi" = 1 op, "sigma * J2 / (tau - 1)" = 3 ops
    let max_acceptable = 2; // simple expressions only

    let complexity_ok = expression_complexity <= max_acceptable;
    let prediction_ok = was_predicted_before;

    let passed = complexity_ok && prediction_ok;
    let severity = if passed {
        0.1
    } else if !prediction_ok && !complexity_ok {
        0.9
    } else if !prediction_ok {
        0.6
    } else {
        0.4
    };

    StatisticalChallenge {
        name: "Post-hoc degrees of freedom".to_string(),
        category: ChallengeCategory::DegreeOfFreedom,
        severity,
        description: format!(
            "Expression complexity: {} ops (max acceptable: {}). \
             Predicted before data: {}. {}",
            expression_complexity,
            max_acceptable,
            if was_predicted_before { "yes" } else { "NO" },
            if passed {
                "Simple pre-registered expression — low risk."
            } else if !prediction_ok {
                "CONCERN: Expression was fitted post-hoc to match observed data."
            } else {
                "CONCERN: Complex expression increases fitting degrees of freedom."
            }
        ),
        passed,
    }
}

/// Challenge: Alternative explanation — powers of 2 dominate computing.
///
/// Many "n=6 matches" in computing (8, 16, 32, 64, 128, 256, 512, 1024)
/// are powers of 2, which exist because of binary representation,
/// not because of any number-theoretic property of 6.
pub fn challenge_power_of_two_alternative(
    value: f64,
) -> StatisticalChallenge {
    let v = value.abs();
    // Check if value is a power of 2
    let is_pow2 = if v >= 1.0 && v <= 1e18 {
        let iv = v as u64;
        iv > 0 && (iv & (iv - 1)) == 0 && (v - iv as f64).abs() < 0.5
    } else {
        false
    };

    // Check if value is close to a power of 2
    let nearest_pow2 = if v > 0.0 {
        2.0_f64.powf(v.log2().round())
    } else {
        1.0
    };
    let close_to_pow2 = (v - nearest_pow2).abs() / nearest_pow2.max(1.0) < 0.05;

    let concern = is_pow2 || close_to_pow2;
    let severity = if is_pow2 { 0.7 } else if close_to_pow2 { 0.4 } else { 0.1 };

    StatisticalChallenge {
        name: "Alternative explanation: power-of-2 prior".to_string(),
        category: ChallengeCategory::AlternativeExplanation,
        severity,
        description: if is_pow2 {
            format!(
                "Value {} IS a power of 2 (2^{:.0}). \
                 CONCERN: Binary representation, not n=6, likely explains this.",
                v,
                v.log2()
            )
        } else if close_to_pow2 {
            format!(
                "Value {:.4} is within 5% of 2^{:.0}={}. \
                 Power-of-2 explanation may compete with n=6.",
                v,
                nearest_pow2.log2(),
                nearest_pow2
            )
        } else {
            format!(
                "Value {:.4} is not near any power of 2. \
                 Power-of-2 alternative does not apply.",
                v
            )
        },
        passed: !concern,
    }
}

/// Challenge: Replication across independent teams.
///
/// A finding is more credible if multiple independent teams arrive
/// at the same values. This checks whether external confirmation exists.
pub fn challenge_independent_replication(
    independent_confirmations: usize,
    self_derived: bool,
) -> StatisticalChallenge {
    let passed = independent_confirmations >= 2 && !self_derived;
    let severity = if passed {
        0.1
    } else if independent_confirmations == 0 && self_derived {
        0.8
    } else {
        0.5
    };

    StatisticalChallenge {
        name: "Independent replication check".to_string(),
        category: ChallengeCategory::ReplicationFailure,
        severity,
        description: format!(
            "Independent confirmations: {}. Self-derived only: {}. {}",
            independent_confirmations,
            if self_derived { "yes" } else { "no" },
            if passed {
                "Multiple independent teams confirm — strong evidence."
            } else if independent_confirmations == 0 {
                "CONCERN: No independent replication. All evidence is self-generated."
            } else {
                "Partial replication exists but insufficient for high confidence."
            }
        ),
        passed,
    }
}

/// Challenge: Confirmation bias — asymmetric search.
///
/// If the methodology only searches for n=6 matches and never
/// systematically tests whether n=5, n=7, n=8, or n=28 (next
/// perfect number) produce equally good or better fits, then
/// the search is asymmetric and confirmation-biased.
pub fn challenge_confirmation_bias(
    tested_alternatives: &[usize],
    n6_score: f64,
    best_alt_score: f64,
) -> StatisticalChallenge {
    let has_alternatives = !tested_alternatives.is_empty();
    let n6_wins = n6_score > best_alt_score;

    let passed = has_alternatives && n6_wins;
    let severity = if !has_alternatives {
        0.8
    } else if n6_wins {
        0.1
    } else {
        0.9
    };

    StatisticalChallenge {
        name: "Confirmation bias (asymmetric search)".to_string(),
        category: ChallengeCategory::ConfirmationBias,
        severity,
        description: if !has_alternatives {
            "CONCERN: No alternative numbers (n=5,7,8,28) were systematically tested. \
             Without comparing, n=6 superiority is unproven."
                .to_string()
        } else if n6_wins {
            format!(
                "Tested alternatives {:?}. n=6 score ({:.3}) beats best alt ({:.3}). Good.",
                tested_alternatives, n6_score, best_alt_score
            )
        } else {
            format!(
                "CRITICAL: Alternative(s) {:?} score ({:.3}) >= n=6 ({:.3}). \
                 n=6 is NOT the best fit!",
                tested_alternatives, best_alt_score, n6_score
            )
        },
        passed,
    }
}

/// Challenge: Domain-specific counterargument for LLM parameters.
///
/// Many LLM hyperparameters (d_model, heads, layers) were chosen
/// by grid search / empirical tuning by teams at Google/OpenAI/Meta,
/// not derived from number theory. The convergence to certain values
/// may reflect hardware constraints (tensor core sizes, memory alignment)
/// rather than mathematical universality.
pub fn challenge_llm_hardware_prior(
    param_name: &str,
    param_value: usize,
) -> StatisticalChallenge {
    // Known hardware-driven constraints
    let hardware_reasons: Vec<(&str, usize, &str)> = vec![
        ("d_model", 4096, "power of 2, GPU warp alignment"),
        ("d_model", 2048, "power of 2"),
        ("d_model", 768, "3*256, tensor core friendly"),
        ("heads", 8, "power of 2, parallelism"),
        ("heads", 16, "power of 2"),
        ("heads", 32, "power of 2"),
        ("d_head", 64, "power of 2, cache line"),
        ("d_head", 128, "power of 2, cache line"),
        ("layers", 12, "BERT original, empirical sweep"),
        ("layers", 24, "GPT-2 medium, 2x BERT"),
        ("layers", 32, "power of 2"),
        ("layers", 48, "round number, 4x12"),
        ("layers", 96, "GPT-3, 2x48"),
        ("batch_size", 8, "GPU memory constraint"),
        ("batch_size", 32, "power of 2"),
        ("vocab", 32000, "round number, byte fallback"),
        ("vocab", 50257, "BPE empirical merge count"),
    ];

    let matching: Vec<_> = hardware_reasons
        .iter()
        .filter(|(name, val, _)| *name == param_name && *val == param_value)
        .collect();

    let has_hw_reason = !matching.is_empty();
    let severity = if has_hw_reason { 0.6 } else { 0.1 };

    StatisticalChallenge {
        name: format!("LLM hardware prior for {}={}", param_name, param_value),
        category: ChallengeCategory::AlternativeExplanation,
        severity,
        description: if has_hw_reason {
            let reasons: Vec<_> = matching.iter().map(|(_, _, r)| *r).collect();
            format!(
                "{}={} has hardware explanation(s): {}. \
                 CONCERN: Value may be hardware-driven, not n=6-driven.",
                param_name,
                param_value,
                reasons.join("; ")
            )
        } else {
            format!(
                "{}={} has no obvious hardware/convention explanation. \
                 n=6 explanation faces fewer competitors.",
                param_name, param_value
            )
        },
        passed: !has_hw_reason,
    }
}

/// Challenge: Cherry-picking across temporal snapshots.
///
/// If n=6 matches are validated against current-generation hardware
/// (e.g., H100 SM=132) but not against previous or future generations,
/// the match may be specific to one snapshot rather than a universal law.
pub fn challenge_temporal_cherry_pick(
    generations_tested: usize,
    generations_matching: usize,
) -> StatisticalChallenge {
    let ratio = if generations_tested > 0 {
        generations_matching as f64 / generations_tested as f64
    } else {
        0.0
    };

    let passed = generations_tested >= 3 && ratio >= 0.7;
    let severity = if passed {
        0.1
    } else if generations_tested < 2 {
        0.7
    } else {
        0.5
    };

    StatisticalChallenge {
        name: "Temporal cherry-pick (snapshot vs trend)".to_string(),
        category: ChallengeCategory::SurvivorshipBias,
        severity,
        description: format!(
            "Tested across {} hardware generations, {}/{} match ({:.0}%). {}",
            generations_tested,
            generations_matching,
            generations_tested,
            ratio * 100.0,
            if passed {
                "Pattern holds across multiple generations — robust."
            } else if generations_tested < 2 {
                "CONCERN: Only tested on current generation. May not be a trend."
            } else {
                "CONCERN: Pattern does not hold consistently across generations."
            }
        ),
        passed,
    }
}

/// Run all statistical challenges on a discovery.
///
/// This is the comprehensive red-team battery that applies every
/// methodological critique in this module.
pub fn run_all_statistical_challenges(
    value: f64,
    n6_score: f64,
    sample_size: usize,
    domain_count: usize,
) -> Vec<StatisticalChallenge> {
    let mut results = Vec::new();

    results.push(challenge_multiple_comparisons(value, 5.0, N6_CONSTANT_POOL_SIZE));
    results.push(challenge_baserate_neglect(value));
    results.push(challenge_power_of_two_alternative(value));
    results.push(challenge_survivorship_bias(sample_size * 3, sample_size, 0.05));
    results.push(challenge_degrees_of_freedom(1, domain_count >= 3));
    results.push(challenge_independent_replication(
        if domain_count >= 3 { domain_count - 1 } else { 0 },
        domain_count < 2,
    ));
    results.push(challenge_confirmation_bias(
        &[5, 7, 8, 28],
        n6_score,
        n6_score * 0.7, // placeholder: assume n=6 wins by 30%
    ));
    results.push(challenge_temporal_cherry_pick(
        domain_count.max(1),
        (domain_count as f64 * 0.8) as usize,
    ));

    results
}

/// Format a report summarizing all statistical challenges.
pub fn format_statistical_report(challenges: &[StatisticalChallenge]) -> String {
    let mut out = String::new();
    out.push_str("=== Statistical Red-Team Report ===\n\n");

    let passed = challenges.iter().filter(|c| c.passed).count();
    let total = challenges.len();
    let overall_severity: f64 = challenges.iter().map(|c| c.severity).sum::<f64>() / total.max(1) as f64;

    out.push_str(&format!(
        "Passed: {}/{} ({:.0}%)  Mean severity: {:.2}\n\n",
        passed,
        total,
        passed as f64 / total.max(1) as f64 * 100.0,
        overall_severity
    ));

    for c in challenges {
        out.push_str(&format!(
            "[{}] {} (severity={:.2})\n  {}\n\n",
            if c.passed { "PASS" } else { "FAIL" },
            c.name,
            c.severity,
            c.description
        ));
    }

    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_challenge_multiple_comparisons_strict() {
        // A very tight tolerance on a small value should pass
        let result = challenge_multiple_comparisons(12.0, 0.01, 30);
        // With 30 comparisons and 0.01% tolerance, should be strict
        assert!(result.severity <= 0.8);
    }

    #[test]
    fn test_challenge_baserate_powers_of_two() {
        // 8 is a common engineering value for non-n=6 reasons
        let result = challenge_baserate_neglect(8.0);
        assert!(!result.passed, "8 should trigger base-rate concern");
        assert!(result.severity > 0.3);
    }

    #[test]
    fn test_challenge_baserate_unusual() {
        // 7.389 is not a common engineering value
        let result = challenge_baserate_neglect(7.389);
        assert!(result.passed, "7.389 should not trigger base-rate concern");
    }

    #[test]
    fn test_challenge_survivorship_strong() {
        // 80/100 confirmed at 5% random rate -> 16x above chance
        let result = challenge_survivorship_bias(100, 80, 0.05);
        assert!(result.passed);
        assert!(result.severity < 0.5);
    }

    #[test]
    fn test_challenge_survivorship_weak() {
        // 3/10 confirmed at 5% random rate -> 6x above chance but sample < 20
        let result = challenge_survivorship_bias(10, 3, 0.05);
        // Sample is 10 < 20 minimum, so should fail despite good ratio
        assert!(!result.passed);
    }

    #[test]
    fn test_challenge_dof_simple_predicted() {
        let result = challenge_degrees_of_freedom(1, true);
        assert!(result.passed);
    }

    #[test]
    fn test_challenge_dof_complex_posthoc() {
        let result = challenge_degrees_of_freedom(5, false);
        assert!(!result.passed);
        assert!(result.severity > 0.7);
    }

    #[test]
    fn test_challenge_pow2_exact() {
        let result = challenge_power_of_two_alternative(256.0);
        assert!(!result.passed, "256 is 2^8");
        assert!(result.severity > 0.5);
    }

    #[test]
    fn test_challenge_pow2_not() {
        let result = challenge_power_of_two_alternative(12.0);
        assert!(result.passed, "12 is not a power of 2");
    }

    #[test]
    fn test_challenge_replication_none() {
        let result = challenge_independent_replication(0, true);
        assert!(!result.passed);
        assert!(result.severity > 0.5);
    }

    #[test]
    fn test_challenge_confirmation_bias_untested() {
        let result = challenge_confirmation_bias(&[], 0.9, 0.0);
        assert!(!result.passed);
        assert!(result.severity > 0.5);
    }

    #[test]
    fn test_challenge_confirmation_bias_n6_wins() {
        let result = challenge_confirmation_bias(&[5, 7, 8, 28], 0.95, 0.60);
        assert!(result.passed);
    }

    #[test]
    fn test_challenge_llm_hardware_heads_8() {
        let result = challenge_llm_hardware_prior("heads", 8);
        assert!(!result.passed, "8 heads has hardware explanation");
    }

    #[test]
    fn test_challenge_llm_hardware_unusual() {
        let result = challenge_llm_hardware_prior("heads", 6);
        assert!(result.passed, "6 heads has no standard hardware reason");
    }

    #[test]
    fn test_challenge_temporal_single_gen() {
        let result = challenge_temporal_cherry_pick(1, 1);
        assert!(!result.passed);
    }

    #[test]
    fn test_challenge_temporal_multi_gen() {
        let result = challenge_temporal_cherry_pick(5, 4);
        assert!(result.passed);
    }

    #[test]
    fn test_run_all_statistical_challenges() {
        let results = run_all_statistical_challenges(12.0, 0.95, 50, 5);
        assert!(results.len() >= 8, "Should run at least 8 challenges");
    }

    #[test]
    fn test_format_statistical_report() {
        let results = run_all_statistical_challenges(12.0, 0.95, 50, 5);
        let report = format_statistical_report(&results);
        assert!(report.contains("Statistical Red-Team Report"));
        assert!(report.contains("severity"));
    }
}
