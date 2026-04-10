//! Red Team Grower — tracks challenge coverage and plans new challenges.
//!
//! Identifies BTs that haven't been adversarially tested, prioritizes
//! high-star unchallenged BTs, and generates red team challenge prompts.

use std::collections::HashSet;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;               // the perfect number
const SIGMA: usize = 12;          // σ(6) = sum of divisors
const PHI: usize = 2;             // φ(6) = Euler totient
const TAU: usize = 4;             // τ(6) = number of divisors
const J2: usize = 24;             // J₂(6) = Jordan totient
const _SOPFR: usize = 5;           // sopfr(6) = 2+3
const _SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10
const SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// Total known BTs.
const TOTAL_BTS: usize = 127;

/// Challenge types for adversarial testing.
const CHALLENGE_TYPES: &[&str] = &[
    "numerology_test",       // Is this just coincidence? Random baseline comparison
    "alternative_formula",   // Can a non-n=6 formula fit equally well?
    "boundary_stress",       // Does the pattern break at edge cases?
    "sample_size",           // Is the evidence base large enough?
    "cherry_picking",        // Were counter-examples excluded?
    "causal_mechanism",      // Is there an actual causal explanation?
];

/// BTs that have been formally challenged (red-teamed).
const CHALLENGED_BTS: &[usize] = &[
    // Core BTs challenged during initial red team pass
    26, 28, 33, 34, 36, 39, 42, 43, 44, 46,
    48, 49, 51, 53, 54, 56, 58, 59, 61, 62,
    64, 65, 66, 67, 69, 70, 74, 80, 84, 89,
    // Environmental + robotics batch
    90, 93, 97, 99, 101, 105, 113, 114, 118, 122,
    123, 127,
];

/// Star ratings for BTs (id, stars). Only listing 3-star BTs for prioritization.
const THREE_STAR_BTS: &[usize] = &[
    28, 36, 43, 49, 51, 54, 56, 58, 59, 61,
    64, 66, 67, 69, 74, 80, 84, 85, 86, 90,
    92, 93, 97, 98, 99, 100, 101, 102, 103, 104,
    105, 113, 114, 117, 118, 119, 120, 122, 123, 127,
];

/// Snapshot of red team coverage.
#[derive(Debug, Clone)]
pub struct RedTeamState {
    /// Total formal challenges completed
    pub total_challenges: usize,
    /// Number of BTs that have been challenged
    pub bts_challenged: usize,
    /// BT ids not yet challenged
    pub bts_unchallenged: Vec<String>,
    /// Average credibility score after challenges (0.0-1.0)
    pub avg_credibility: f64,
}

/// A challenge to create for a specific BT.
#[derive(Debug, Clone)]
pub struct ChallengeToCreate {
    /// Target BT identifier
    pub bt_id: String,
    /// Types of challenges to apply
    pub challenge_types: Vec<String>,
    /// Ready-to-use Claude CLI prompt
    pub claude_prompt: String,
}

/// Growth plan for red team expansion.
#[derive(Debug, Clone)]
pub struct RedTeamPlan {
    /// Challenges to create, ordered by priority
    pub new_challenges: Vec<ChallengeToCreate>,
}

/// Assess the current state of red team coverage.
pub fn assess_redteam_state() -> RedTeamState {
    let challenged_set: HashSet<usize> = CHALLENGED_BTS.iter().copied().collect();
    let unchallenged: Vec<String> = (1..=TOTAL_BTS)
        .filter(|id| !challenged_set.contains(id))
        .map(|id| format!("BT-{}", id))
        .collect();

    RedTeamState {
        total_challenges: CHALLENGED_BTS.len(),
        bts_challenged: CHALLENGED_BTS.len(),
        bts_unchallenged: unchallenged,
        // Average credibility after red team: ~0.74 (z=0.74 documented)
        avg_credibility: 0.74,
    }
}

/// Plan new red team challenges, prioritizing high-star unchallenged BTs.
///
/// `max` caps the number of challenges (capped to J₂=24).
pub fn plan_challenges(max: usize) -> RedTeamPlan {
    let max = max.min(J2); // cap at J₂=24
    let _state = assess_redteam_state();
    let challenged_set: HashSet<usize> = CHALLENGED_BTS.iter().copied().collect();

    // Priority 1: 3-star BTs not yet challenged (highest risk if wrong)
    let mut priority_targets: Vec<usize> = THREE_STAR_BTS
        .iter()
        .copied()
        .filter(|id| !challenged_set.contains(id))
        .collect();

    // Priority 2: All remaining unchallenged BTs
    let other_targets: Vec<usize> = (1..=TOTAL_BTS)
        .filter(|id| !challenged_set.contains(id) && !priority_targets.contains(id))
        .collect();

    priority_targets.extend(other_targets);

    let challenges: Vec<ChallengeToCreate> = priority_targets
        .into_iter()
        .take(max)
        .map(|id| create_challenge(id))
        .collect();

    RedTeamPlan {
        new_challenges: challenges,
    }
}

/// Generate a comprehensive red team prompt for a BT.
pub fn generate_challenge_prompt(bt_id: usize, star_rating: usize) -> String {
    let challenge_depth = match star_rating {
        3 => "DEEP (3-star: extraordinary claims require extraordinary evidence)",
        2 => "STANDARD (2-star: verify core claims thoroughly)",
        _ => "BASIC (1-star: quick sanity check)",
    };

    format!(
        r#"RED TEAM CHALLENGE for BT-{id}

Challenge depth: {depth}

Execute ALL {n} challenge types:

1. NUMEROLOGY TEST (z-score)
   - Generate {j2} random formulas using integers 1-{sigma}
   - For each, compute best-fit to the claimed value
   - Calculate z-score: how unlikely is the n=6 match vs random?
   - PASS if z > 2.0 (p < 0.05)

2. ALTERNATIVE FORMULA
   - Find the simplest non-n=6 formula that matches the known value
   - Compare complexity (Kolmogorov) of n=6 vs alternative
   - PASS only if n=6 formula is simpler or fits better

3. BOUNDARY STRESS
   - Test the claimed pattern at extreme values
   - If BT claims "X always equals Y", find edge cases
   - Check: does it hold for historical data? Future predictions?

4. SAMPLE SIZE
   - Count independent data points supporting the claim
   - Minimum {tau} independent sources required for EXACT grade
   - Minimum {sigma_minus_tau} for 3-star rating

5. CHERRY PICKING
   - List ALL known values in the domain, not just matching ones
   - Compute: what fraction of domain parameters match n=6?
   - PASS if matching rate > 50% (not just selected examples)

6. CAUSAL MECHANISM
   - Is there a physical/mathematical reason for the n=6 pattern?
   - Or is it purely empirical coincidence?
   - Rate: CAUSAL / EMPIRICAL / COINCIDENCE

OUTPUT FORMAT:
| Challenge | Result | Score | Notes |
|-----------|--------|-------|-------|
| Numerology | PASS/FAIL | z=X.XX | ... |
| Alternative | PASS/FAIL | complexity ratio | ... |
| Boundary | PASS/FAIL | N edge cases | ... |
| Sample size | PASS/FAIL | N sources | ... |
| Cherry picking | PASS/FAIL | X% match rate | ... |
| Causal | CAUSAL/EMPIRICAL/COINCIDENCE | - | ... |

OVERALL: [CREDIBLE / SUSPECT / DEBUNKED]
CREDIBILITY SCORE: 0.00-1.00"#,
        id = bt_id,
        depth = challenge_depth,
        n = N,
        j2 = J2,
        sigma = SIGMA,
        tau = TAU,
        sigma_minus_tau = SIGMA_MINUS_TAU,
    )
}

/// Format an ASCII red team coverage dashboard.
pub fn format_redteam_coverage() -> String {
    let state = assess_redteam_state();
    let coverage_pct = (state.bts_challenged as f64 / TOTAL_BTS as f64) * 100.0;
    let bar_filled = (coverage_pct / (TAU as f64)).round() as usize;
    let bar_empty = J2.saturating_sub(bar_filled);

    let mut lines = Vec::new();
    lines.push("┌───────────────────────────────────────────────────────────┐".to_string());
    lines.push("│           Red Team Coverage Dashboard                    │".to_string());
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push(format!(
        "│  Challenged: {:>3}/{:<3} BTs  [{}{}] {:.1}%       │",
        state.bts_challenged,
        TOTAL_BTS,
        "█".repeat(bar_filled),
        "░".repeat(bar_empty),
        coverage_pct,
    ));
    lines.push(format!(
        "│  Unchallenged: {:>3} BTs                                   │",
        state.bts_unchallenged.len(),
    ));
    lines.push(format!(
        "│  Avg credibility: {:.2}                                     │",
        state.avg_credibility,
    ));
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push(format!(
        "│  Challenge types: {} ({})                       │",
        CHALLENGE_TYPES.len(),
        CHALLENGE_TYPES.join(", "),
    ));
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push("│  Priority unchallenged (3-star):                         │".to_string());

    let challenged_set: HashSet<usize> = CHALLENGED_BTS.iter().copied().collect();
    let unchallenged_3star: Vec<usize> = THREE_STAR_BTS
        .iter()
        .copied()
        .filter(|id| !challenged_set.contains(id))
        .collect();

    for bt_id in unchallenged_3star.iter().take(SIGMA_MINUS_TAU) {
        lines.push(format!("│    BT-{:<4} ⭐⭐⭐  [NEEDS CHALLENGE]                     │", bt_id));
    }
    if unchallenged_3star.len() > SIGMA_MINUS_TAU {
        lines.push(format!(
            "│    ... and {} more 3-star BTs                             │",
            unchallenged_3star.len() - SIGMA_MINUS_TAU,
        ));
    }

    lines.push("└───────────────────────────────────────────────────────────┘".to_string());
    lines.join("\n")
}

// ── internal helpers ─────────────────────────────────────────────────

fn create_challenge(bt_id: usize) -> ChallengeToCreate {
    let is_3star = THREE_STAR_BTS.contains(&bt_id);
    let star_rating = if is_3star { 3 } else { PHI }; // 3 or φ=2

    // Select challenge types based on star rating
    let types: Vec<String> = if is_3star {
        // 3-star BTs get all 6 challenge types
        CHALLENGE_TYPES.iter().map(|s| s.to_string()).collect()
    } else {
        // Lower-star BTs get core τ=4 types
        CHALLENGE_TYPES.iter().take(TAU).map(|s| s.to_string()).collect()
    };

    let prompt = generate_challenge_prompt(bt_id, star_rating);

    ChallengeToCreate {
        bt_id: format!("BT-{}", bt_id),
        challenge_types: types,
        claude_prompt: prompt,
    }
}

// ── tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_redteam_state() {
        let state = assess_redteam_state();
        assert_eq!(state.bts_challenged, CHALLENGED_BTS.len());
        assert!(!state.bts_unchallenged.is_empty());
        assert_eq!(
            state.bts_challenged + state.bts_unchallenged.len(),
            TOTAL_BTS,
        );
        // Credibility should be in valid range
        assert!(state.avg_credibility >= 0.0 && state.avg_credibility <= 1.0);
    }

    #[test]
    fn test_plan_challenges_prioritizes_3star() {
        let plan = plan_challenges(SIGMA); // max σ=12
        assert!(plan.new_challenges.len() <= SIGMA);

        // First challenges should target 3-star unchallenged BTs
        if !plan.new_challenges.is_empty() {
            let first_id: usize = plan.new_challenges[0]
                .bt_id
                .trim_start_matches("BT-")
                .parse()
                .unwrap();
            let challenged_set: HashSet<usize> = CHALLENGED_BTS.iter().copied().collect();
            let unchallenged_3star: Vec<usize> = THREE_STAR_BTS
                .iter()
                .copied()
                .filter(|id| !challenged_set.contains(id))
                .collect();
            if !unchallenged_3star.is_empty() {
                assert!(
                    unchallenged_3star.contains(&first_id),
                    "First challenge should target a 3-star unchallenged BT",
                );
            }
        }
    }

    #[test]
    fn test_plan_challenges_respects_max() {
        let plan = plan_challenges(TAU); // max τ=4
        assert!(plan.new_challenges.len() <= TAU);
    }

    #[test]
    fn test_3star_bts_get_all_challenge_types() {
        let challenged_set: HashSet<usize> = CHALLENGED_BTS.iter().copied().collect();
        let unchallenged_3star: Vec<usize> = THREE_STAR_BTS
            .iter()
            .copied()
            .filter(|id| !challenged_set.contains(id))
            .collect();

        if !unchallenged_3star.is_empty() {
            let challenge = create_challenge(unchallenged_3star[0]);
            assert_eq!(
                challenge.challenge_types.len(),
                CHALLENGE_TYPES.len(),
                "3-star BTs should get all {} challenge types",
                CHALLENGE_TYPES.len(),
            );
        }
    }

    #[test]
    fn test_generate_challenge_prompt_contains_all_sections() {
        let prompt = generate_challenge_prompt(42, 3);
        assert!(prompt.contains("NUMEROLOGY TEST"));
        assert!(prompt.contains("ALTERNATIVE FORMULA"));
        assert!(prompt.contains("BOUNDARY STRESS"));
        assert!(prompt.contains("SAMPLE SIZE"));
        assert!(prompt.contains("CHERRY PICKING"));
        assert!(prompt.contains("CAUSAL MECHANISM"));
        assert!(prompt.contains("CREDIBILITY SCORE"));
    }

    #[test]
    fn test_format_redteam_coverage() {
        let coverage = format_redteam_coverage();
        assert!(coverage.contains("Red Team Coverage"));
        assert!(coverage.contains("Challenged"));
        assert!(coverage.contains("credibility"));
    }
}
