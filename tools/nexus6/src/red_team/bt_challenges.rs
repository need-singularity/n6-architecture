//! BT Challenges — adversarial tests for 20 high-star breakthrough theorems.
//!
//! Applies the devil's advocate framework (RandomCoincidence, SmallSample,
//! PostHocRationalization) to each selected BT and computes a credibility score.
//!
//! # Key types
//!
//! - [`BTChallenge`] — a complete adversarial assessment for one BT.
//! - [`ChallengeResult`] — outcome of a single challenge check (pass/fail + detail).
//!
//! # Usage
//!
//! ```rust,ignore
//! let challenges = bt_challenges::bt_challenges();
//! for c in &challenges {
//!     println!("BT-{}: credibility={:.2}", c.bt_id, c.credibility_score);
//! }
//! ```

use super::devil_advocate::{self, ChallengeType};

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;               // the perfect number
const SIGMA: usize = 12;          // sigma(6) = sum of divisors
const PHI: usize = 2;             // phi(6) = Euler totient
const TAU: usize = 4;             // tau(6) = number of divisors
const J2: usize = 24;             // J_2(6) = Jordan totient
const SOPFR: usize = 5;           // sopfr(6) = 2+3
const SIGMA_MINUS_PHI: usize = 10; // sigma - phi = 10
const SIGMA_MINUS_TAU: usize = 8; // sigma - tau = 8

/// Result of a single adversarial check against a BT.
#[derive(Debug, Clone)]
pub struct ChallengeResult {
    /// Type of challenge applied.
    pub challenge_type: ChallengeType,
    /// Whether the BT passed this challenge.
    pub passed: bool,
    /// Severity of the concern (0.0 = none, 1.0 = critical).
    pub severity: f64,
    /// Human-readable explanation.
    pub detail: String,
}

/// Complete adversarial assessment for one breakthrough theorem.
#[derive(Debug, Clone)]
pub struct BTChallenge {
    /// BT number (e.g. 56 for BT-56).
    pub bt_id: usize,
    /// Short title of the BT.
    pub title: &'static str,
    /// Star rating (3 = highest).
    pub stars: usize,
    /// Number of claimed EXACT matches.
    pub exact_count: usize,
    /// Total data points checked.
    pub sample_size: usize,
    /// Number of independent domains confirming.
    pub domain_count: usize,
    /// Fraction of data points that are EXACT.
    pub exact_ratio: f64,
    /// Individual challenge results.
    pub checks: Vec<ChallengeResult>,
    /// Overall credibility score (0.0-1.0).
    pub credibility_score: f64,
}

/// Static descriptor for a BT to challenge.
struct BTSpec {
    bt_id: usize,
    title: &'static str,
    stars: usize,
    exact_count: usize,
    sample_size: usize,
    domain_count: usize,
}

/// The 20 three-star BTs selected for adversarial challenge.
const BT_SPECS: &[BTSpec] = &[
    BTSpec { bt_id: 43, title: "Battery cathode CN=6 universality", stars: 3, exact_count: 6, sample_size: 8, domain_count: 3 },
    BTSpec { bt_id: 54, title: "AdamW quintuplet", stars: 3, exact_count: 5, sample_size: 5, domain_count: 2 },
    BTSpec { bt_id: 56, title: "Complete n=6 LLM", stars: 3, exact_count: 15, sample_size: 15, domain_count: 4 },
    BTSpec { bt_id: 58, title: "sigma-tau=8 universal AI constant", stars: 3, exact_count: 16, sample_size: 16, domain_count: 5 },
    BTSpec { bt_id: 59, title: "8-layer AI stack", stars: 3, exact_count: 8, sample_size: 8, domain_count: 6 },
    BTSpec { bt_id: 61, title: "Diffusion n=6 universality", stars: 3, exact_count: 9, sample_size: 9, domain_count: 2 },
    BTSpec { bt_id: 64, title: "1/(sigma-phi)=0.1 universal regularization", stars: 3, exact_count: 8, sample_size: 8, domain_count: 7 },
    BTSpec { bt_id: 66, title: "Vision AI complete n=6", stars: 3, exact_count: 24, sample_size: 24, domain_count: 5 },
    BTSpec { bt_id: 67, title: "MoE activation fraction law", stars: 3, exact_count: 6, sample_size: 6, domain_count: 3 },
    BTSpec { bt_id: 69, title: "Chiplet architecture convergence", stars: 3, exact_count: 17, sample_size: 20, domain_count: 5 },
    BTSpec { bt_id: 74, title: "95/5 cross-domain resonance", stars: 3, exact_count: 5, sample_size: 5, domain_count: 5 },
    BTSpec { bt_id: 84, title: "96/192 energy-computing-AI triple convergence", stars: 3, exact_count: 5, sample_size: 5, domain_count: 3 },
    BTSpec { bt_id: 90, title: "SM = phi x K6 contact theorem", stars: 3, exact_count: 6, sample_size: 6, domain_count: 2 },
    BTSpec { bt_id: 93, title: "Carbon Z=6 chip material universality", stars: 3, exact_count: 8, sample_size: 10, domain_count: 4 },
    BTSpec { bt_id: 99, title: "Tokamak q=1 perfect number identity", stars: 3, exact_count: 3, sample_size: 3, domain_count: 2 },
    BTSpec { bt_id: 101, title: "Photosynthesis glucose 24 atoms=J2", stars: 3, exact_count: 9, sample_size: 9, domain_count: 3 },
    BTSpec { bt_id: 105, title: "SLE6 critical exponent universality", stars: 3, exact_count: 7, sample_size: 7, domain_count: 2 },
    BTSpec { bt_id: 113, title: "SW engineering constants stack", stars: 3, exact_count: 18, sample_size: 18, domain_count: 6 },
    BTSpec { bt_id: 117, title: "Software-physics isomorphism", stars: 3, exact_count: 18, sample_size: 18, domain_count: 6 },
    BTSpec { bt_id: 123, title: "SE(3) dim=n=6 robot universality", stars: 3, exact_count: 9, sample_size: 9, domain_count: 3 },
];

/// Generate adversarial challenges for all 20 selected BTs.
///
/// For each BT, applies three checks (RandomCoincidence, SmallSample,
/// PostHocRationalization) using the devil's advocate framework, then
/// computes an overall credibility score.
pub fn bt_challenges() -> Vec<BTChallenge> {
    BT_SPECS.iter().map(|spec| challenge_one_bt(spec)).collect()
}

/// Retrieve a challenge for a specific BT by id. Returns None if not
/// among the 20 selected BTs.
pub fn challenge_for_bt(bt_id: usize) -> Option<BTChallenge> {
    BT_SPECS
        .iter()
        .find(|s| s.bt_id == bt_id)
        .map(challenge_one_bt)
}

/// Summary statistics across all 20 challenges.
pub fn summary_stats() -> (f64, usize, usize) {
    let challenges = bt_challenges();
    let total = challenges.len();
    let credible = challenges.iter().filter(|c| c.credibility_score >= 0.7).count();
    let avg_cred = challenges.iter().map(|c| c.credibility_score).sum::<f64>() / total as f64;
    (avg_cred, credible, total)
}

/// Format a compact ASCII report of all BT challenges.
pub fn format_report() -> String {
    let challenges = bt_challenges();
    let mut s = String::new();

    s.push_str("+------+---+-----+-----+----+---------+-------+\n");
    s.push_str("|  BT  | * | EX  | N   | D  | Cred    | Grade |\n");
    s.push_str("+------+---+-----+-----+----+---------+-------+\n");

    for c in &challenges {
        let grade = if c.credibility_score >= 0.8 {
            "STRONG"
        } else if c.credibility_score >= 0.6 {
            "GOOD"
        } else if c.credibility_score >= 0.4 {
            "WEAK"
        } else {
            "SUSPECT"
        };

        s.push_str(&format!(
            "| {:>4} | {} | {:>3} | {:>3} | {:>2} | {:.3}   | {:<7}|\n",
            c.bt_id, c.stars, c.exact_count, c.sample_size, c.domain_count,
            c.credibility_score, grade,
        ));
    }

    s.push_str("+------+---+-----+-----+----+---------+-------+\n");

    let (avg, credible, total) = summary_stats();
    s.push_str(&format!(
        "| Avg credibility: {:.3}  |  Credible(>=0.7): {}/{}    |\n",
        avg, credible, total,
    ));
    s.push_str("+----------------------------------------------+\n");

    s
}

// ── Internal helpers ─────────────────────────────────────────────────

fn challenge_one_bt(spec: &BTSpec) -> BTChallenge {
    let exact_ratio = if spec.sample_size > 0 {
        spec.exact_count as f64 / spec.sample_size as f64
    } else {
        0.0
    };

    let mut checks = Vec::new();

    // Check 1: RandomCoincidence
    // z-score approximation: how unlikely is this exact_ratio by chance?
    // Baseline: ~5% of random values land near an n=6 constant.
    let baseline_rate = 0.05;
    let z_score = if spec.sample_size > 0 {
        let expected = baseline_rate * spec.sample_size as f64;
        let std_dev = (spec.sample_size as f64 * baseline_rate * (1.0 - baseline_rate)).sqrt();
        if std_dev > 0.0 {
            (spec.exact_count as f64 - expected) / std_dev
        } else {
            0.0
        }
    } else {
        0.0
    };

    let random_passed = z_score > 2.0; // p < 0.05
    let random_severity = if random_passed {
        0.0
    } else {
        (1.0 - (z_score / 2.0).min(1.0)).max(0.0)
    };

    checks.push(ChallengeResult {
        challenge_type: ChallengeType::RandomCoincidence,
        passed: random_passed,
        severity: random_severity,
        detail: format!(
            "z={:.2}, exact_ratio={:.0}%, baseline={:.0}%{}",
            z_score,
            exact_ratio * 100.0,
            baseline_rate * 100.0,
            if random_passed { " -- significantly above chance" } else { " -- could be chance" },
        ),
    });

    // Check 2: SmallSample
    // Need at least sigma-phi=10 data points for significance.
    let sample_passed = spec.sample_size >= SIGMA_MINUS_PHI;
    let sample_severity = if sample_passed {
        0.0
    } else {
        1.0 - (spec.sample_size as f64 / SIGMA_MINUS_PHI as f64).min(1.0)
    };

    checks.push(ChallengeResult {
        challenge_type: ChallengeType::SmallSample,
        passed: sample_passed,
        severity: sample_severity,
        detail: format!(
            "sample_size={}, minimum=sigma-phi={}{}",
            spec.sample_size,
            SIGMA_MINUS_PHI,
            if sample_passed { " -- sufficient" } else { " -- insufficient sample" },
        ),
    });

    // Check 3: PostHocRationalization
    // Risk is higher when: (a) perfect ratio on small sample, or
    // (b) the n=6 expression was chosen *after* seeing the data.
    // Heuristic: 100% EXACT on < n=6 samples is suspicious.
    let posthoc_suspicious = exact_ratio >= 1.0 && spec.sample_size < N;
    // Also flag if single domain with many EXACT (could be domain-specific fitting)
    let single_domain_risk = spec.domain_count < 2 && spec.exact_count > TAU;
    let posthoc_passed = !posthoc_suspicious && !single_domain_risk;
    let posthoc_severity = if posthoc_suspicious {
        0.8
    } else if single_domain_risk {
        0.5
    } else {
        0.0
    };

    checks.push(ChallengeResult {
        challenge_type: ChallengeType::PostHocRationalization,
        passed: posthoc_passed,
        severity: posthoc_severity,
        detail: format!(
            "perfect_small_sample={}, single_domain_risk={}{}",
            posthoc_suspicious,
            single_domain_risk,
            if posthoc_passed { " -- low post-hoc risk" } else { " -- post-hoc risk detected" },
        ),
    });

    // Compute overall credibility using the devil_advocate framework
    let da_challenges = devil_advocate::challenge_discovery(
        &format!("BT-{}", spec.bt_id),
        exact_ratio,
        spec.domain_count,
        spec.sample_size,
    );
    let base_credibility = devil_advocate::credibility_score(&da_challenges);

    // Adjust credibility with our additional checks
    let our_severity: f64 = checks.iter().map(|c| c.severity).sum::<f64>() / checks.len().max(1) as f64;
    let credibility = ((base_credibility + (1.0 - our_severity)) / PHI as f64).min(1.0).max(0.0);

    BTChallenge {
        bt_id: spec.bt_id,
        title: spec.title,
        stars: spec.stars,
        exact_count: spec.exact_count,
        sample_size: spec.sample_size,
        domain_count: spec.domain_count,
        exact_ratio,
        checks,
        credibility_score: credibility,
    }
}

// ── Tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bt_challenges_count() {
        let challenges = bt_challenges();
        // Should have exactly 20 challenges (one per selected BT)
        assert_eq!(challenges.len(), 20);
        // Each should have 3 checks (RandomCoincidence, SmallSample, PostHocRationalization)
        for c in &challenges {
            assert_eq!(c.checks.len(), 3, "BT-{} should have 3 checks", c.bt_id);
        }
    }

    #[test]
    fn test_strong_bt_high_credibility() {
        // BT-58: 16/16 EXACT across 5 domains -- should be strong
        let c = challenge_for_bt(58).expect("BT-58 should be in the list");
        assert_eq!(c.bt_id, 58);
        assert_eq!(c.stars, 3);
        assert!(
            c.credibility_score >= 0.6,
            "BT-58 should have high credibility, got {:.3}",
            c.credibility_score
        );
        // RandomCoincidence check should pass (16/16 is very unlikely by chance)
        let random_check = c.checks.iter().find(|ch| ch.challenge_type == ChallengeType::RandomCoincidence);
        assert!(random_check.is_some());
        assert!(random_check.unwrap().passed, "BT-58 should pass random coincidence check");
    }

    #[test]
    fn test_small_sample_bt_flagged() {
        // BT-99: only 3 data points -- should flag small sample
        let c = challenge_for_bt(99).expect("BT-99 should be in the list");
        let sample_check = c.checks.iter().find(|ch| ch.challenge_type == ChallengeType::SmallSample);
        assert!(sample_check.is_some());
        assert!(
            !sample_check.unwrap().passed,
            "BT-99 with 3 samples should fail small sample check"
        );
        assert!(
            sample_check.unwrap().severity > 0.5,
            "Severity should be high for 3 samples"
        );
    }

    #[test]
    fn test_credibility_scores_in_range() {
        let challenges = bt_challenges();
        for c in &challenges {
            assert!(
                c.credibility_score >= 0.0 && c.credibility_score <= 1.0,
                "BT-{} credibility {:.3} out of range",
                c.bt_id, c.credibility_score
            );
            assert!(
                c.exact_ratio >= 0.0 && c.exact_ratio <= 1.0,
                "BT-{} exact_ratio {:.3} out of range",
                c.bt_id, c.exact_ratio
            );
        }
    }

    #[test]
    fn test_summary_stats() {
        let (avg, credible, total) = summary_stats();
        assert_eq!(total, 20);
        assert!(avg >= 0.0 && avg <= 1.0, "avg credibility out of range: {}", avg);
        assert!(credible <= total);
    }

    #[test]
    fn test_format_report_contains_all_bts() {
        let report = format_report();
        // Should mention all 20 BT ids
        for spec in BT_SPECS {
            assert!(
                report.contains(&format!("{:>4}", spec.bt_id)),
                "Report should contain BT-{}",
                spec.bt_id,
            );
        }
        assert!(report.contains("Avg credibility"));
    }

    #[test]
    fn test_challenge_for_bt_missing() {
        // BT-1 is not in our 20 selected
        assert!(challenge_for_bt(1).is_none());
    }
}
