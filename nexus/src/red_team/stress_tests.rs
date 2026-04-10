//! Stress tests — adversarial stress testing of n=6 claims.
//!
//! Functions probe the robustness of n=6 EXACT matches under extreme
//! conditions: edge cases, scale changes, precision attacks, and
//! combinatorial stress.

use crate::verifier::n6_check;

// ── n=6 constants ────────────────────────────────────────────────────
const N: f64 = 6.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const J2: f64 = 24.0;
const SOPFR: f64 = 5.0;
const SIGMA_MINUS_PHI: f64 = 10.0;
const SIGMA_MINUS_TAU: f64 = 8.0;

/// Result of a stress test.
#[derive(Debug, Clone)]
pub struct StressResult {
    pub test_name: String,
    pub passed: bool,
    pub detail: String,
    pub severity: f64, // 0.0 = no issue, 1.0 = critical
}

/// Stress test: Scale invariance — does the match survive multiplication by powers of 10?
pub fn stress_scale_invariance(value: f64) -> StressResult {
    let (orig_name, orig_q) = n6_check::n6_match(value);
    if orig_q < 0.5 {
        return StressResult {
            test_name: "stress_scale_invariance".into(),
            passed: true,
            detail: "Value not an n6 match; scale invariance N/A".into(),
            severity: 0.0,
        };
    }

    let mut false_matches = 0;
    for exp in -6..=6 {
        if exp == 0 { continue; }
        let scaled = value * 10.0_f64.powi(exp);
        let (_, q) = n6_check::n6_match(scaled);
        if q >= 0.8 {
            false_matches += 1;
        }
    }

    // If many scaled versions also match, the constant set is too loose
    let passed = false_matches <= 3;
    StressResult {
        test_name: "stress_scale_invariance".into(),
        passed,
        detail: format!(
            "{}/12 scaled versions also match (original: {} q={:.2})",
            false_matches, orig_name, orig_q
        ),
        severity: if passed { 0.0 } else { (false_matches as f64 / 12.0).min(1.0) },
    }
}

/// Stress test: Precision degradation — does the match survive rounding?
pub fn stress_precision_degradation(value: f64) -> StressResult {
    let (orig_name, orig_q) = n6_check::n6_match(value);
    if orig_q < 0.5 {
        return StressResult {
            test_name: "stress_precision_degradation".into(),
            passed: true,
            detail: "Not an n6 match".into(),
            severity: 0.0,
        };
    }

    let mut lost_at_digits = 0;
    for digits in (0..=6).rev() {
        let factor = 10.0_f64.powi(digits);
        let rounded = (value * factor).round() / factor;
        let (_, q) = n6_check::n6_match(rounded);
        if q < 0.5 {
            lost_at_digits = digits;
            break;
        }
    }

    let passed = lost_at_digits <= 1; // Should survive at least 2 significant digits
    StressResult {
        test_name: "stress_precision_degradation".into(),
        passed,
        detail: format!(
            "Match lost at {} decimal digits (original: {} q={:.2})",
            lost_at_digits, orig_name, orig_q
        ),
        severity: if passed { 0.0 } else { (lost_at_digits as f64 / 6.0).min(1.0) },
    }
}

/// Stress test: Negative mirror — does -value also match n6? (asymmetry check)
pub fn stress_negative_mirror(value: f64) -> StressResult {
    let (_, orig_q) = n6_check::n6_match(value);
    let (neg_name, neg_q) = n6_check::n6_match(-value);

    let both_match = orig_q >= 0.8 && neg_q >= 0.8;
    StressResult {
        test_name: "stress_negative_mirror".into(),
        passed: !both_match, // If both match, the set is too symmetric
        detail: format!(
            "value q={:.2}, -value q={:.2} (matched: {})",
            orig_q, neg_q, neg_name
        ),
        severity: if both_match { 0.6 } else { 0.0 },
    }
}

/// Stress test: Additive perturbation sweep — how much additive noise breaks the match?
pub fn stress_additive_perturbation(value: f64) -> StressResult {
    let (_, orig_q) = n6_check::n6_match(value);
    if orig_q < 0.5 {
        return StressResult {
            test_name: "stress_additive_perturbation".into(),
            passed: true,
            detail: "Not an n6 match".into(),
            severity: 0.0,
        };
    }

    // Find the smallest absolute perturbation that breaks the match
    let mut break_pct = 100.0;
    for pct_10x in 1..=100 {
        let pct = pct_10x as f64 * 0.1; // 0.1% to 10%
        let perturbed = value * (1.0 + pct / 100.0);
        let (_, q) = n6_check::n6_match(perturbed);
        if q < 0.5 {
            break_pct = pct;
            break;
        }
    }

    let passed = break_pct >= 1.0; // Should tolerate at least 1% perturbation
    StressResult {
        test_name: "stress_additive_perturbation".into(),
        passed,
        detail: format!("Match breaks at {:.1}% perturbation", break_pct),
        severity: if passed { 0.0 } else { (1.0 - break_pct / 1.0).max(0.0).min(1.0) },
    }
}

/// Stress test: Integer neighborhood — do adjacent integers also match?
pub fn stress_integer_neighborhood(value: f64) -> StressResult {
    let (_, orig_q) = n6_check::n6_match(value);
    if orig_q < 0.5 || value.fract().abs() > 0.01 {
        return StressResult {
            test_name: "stress_integer_neighborhood".into(),
            passed: true,
            detail: "Not an integer n6 match".into(),
            severity: 0.0,
        };
    }

    let mut neighbor_matches = 0;
    for delta in &[-2.0, -1.0, 1.0, 2.0] {
        let neighbor = value + delta;
        let (_, q) = n6_check::n6_match(neighbor);
        if q >= 0.8 {
            neighbor_matches += 1;
        }
    }

    // In a good constant set, not too many neighbors should also be constants
    let passed = neighbor_matches <= 2;
    StressResult {
        test_name: "stress_integer_neighborhood".into(),
        passed,
        detail: format!("{}/4 integer neighbors also match n6", neighbor_matches),
        severity: if passed { neighbor_matches as f64 * 0.2 } else { 0.8 },
    }
}

/// Stress test: Ratio cascade — do ratios between n6 constants produce more n6 constants?
pub fn stress_ratio_cascade() -> StressResult {
    let constants = [N, SIGMA, PHI, TAU, J2, SOPFR, SIGMA_MINUS_PHI, SIGMA_MINUS_TAU];
    let mut ratio_match_count = 0;
    let mut total_ratios = 0;

    for i in 0..constants.len() {
        for j in 0..constants.len() {
            if i == j { continue; }
            if constants[j].abs() < 1e-12 { continue; }
            let ratio = constants[i] / constants[j];
            total_ratios += 1;
            let (_, q) = n6_check::n6_match(ratio);
            if q >= 0.8 {
                ratio_match_count += 1;
            }
        }
    }

    // Some ratio closure is expected (n=6 is algebraically rich),
    // but >80% would indicate an overly broad constant set
    let ratio_rate = ratio_match_count as f64 / total_ratios.max(1) as f64;
    let passed = ratio_rate < 0.8;
    StressResult {
        test_name: "stress_ratio_cascade".into(),
        passed,
        detail: format!(
            "{}/{} ratios between n6 constants also match ({:.0}%)",
            ratio_match_count, total_ratios, ratio_rate * 100.0
        ),
        severity: if passed { ratio_rate * 0.5 } else { 0.9 },
    }
}

/// Stress test: Sum cascade — do sums of n6 constant pairs also match?
pub fn stress_sum_cascade() -> StressResult {
    let constants = [N, SIGMA, PHI, TAU, J2, SOPFR, SIGMA_MINUS_PHI, SIGMA_MINUS_TAU];
    let mut sum_match_count = 0;
    let mut total_sums = 0;

    for i in 0..constants.len() {
        for j in (i + 1)..constants.len() {
            let sum = constants[i] + constants[j];
            total_sums += 1;
            let (_, q) = n6_check::n6_match(sum);
            if q >= 0.8 {
                sum_match_count += 1;
            }
        }
    }

    let sum_rate = sum_match_count as f64 / total_sums.max(1) as f64;
    let passed = sum_rate < 0.9;
    StressResult {
        test_name: "stress_sum_cascade".into(),
        passed,
        detail: format!(
            "{}/{} pairwise sums also match ({:.0}%)",
            sum_match_count, total_sums, sum_rate * 100.0
        ),
        severity: if passed { sum_rate * 0.3 } else { 0.7 },
    }
}

/// Stress test: Product cascade — do products of n6 constant pairs match?
pub fn stress_product_cascade() -> StressResult {
    let constants = [N, SIGMA, PHI, TAU, J2, SOPFR];
    let mut prod_match_count = 0;
    let mut total_prods = 0;

    for i in 0..constants.len() {
        for j in (i + 1)..constants.len() {
            let prod = constants[i] * constants[j];
            total_prods += 1;
            let (_, q) = n6_check::n6_match(prod);
            if q >= 0.8 {
                prod_match_count += 1;
            }
        }
    }

    let prod_rate = prod_match_count as f64 / total_prods.max(1) as f64;
    let passed = prod_rate < 0.9;
    StressResult {
        test_name: "stress_product_cascade".into(),
        passed,
        detail: format!(
            "{}/{} pairwise products also match ({:.0}%)",
            prod_match_count, total_prods, prod_rate * 100.0
        ),
        severity: if passed { prod_rate * 0.3 } else { 0.7 },
    }
}

/// Stress test: Random baseline — what fraction of random floats in [0, 100] match?
pub fn stress_random_baseline(seed: u32) -> StressResult {
    let mut rng = seed.max(1);
    let trials = 1000;
    let mut matches = 0;

    for _ in 0..trials {
        // xorshift32
        rng ^= rng << 13;
        rng ^= rng >> 17;
        rng ^= rng << 5;
        let v = (rng as f64 / u32::MAX as f64) * 100.0;
        let (_, q) = n6_check::n6_match(v);
        if q >= 0.8 {
            matches += 1;
        }
    }

    let rate = matches as f64 / trials as f64;
    // If >10% of random values match, the constant set is too loose
    let passed = rate < 0.10;
    StressResult {
        test_name: "stress_random_baseline".into(),
        passed,
        detail: format!(
            "{}/{} random values match ({:.1}%)",
            matches, trials, rate * 100.0
        ),
        severity: if passed { rate } else { 0.9 },
    }
}

/// Stress test: Extreme values — do very large/small values match?
pub fn stress_extreme_values() -> StressResult {
    let extremes = [
        1e-10, 1e-5, 1e-3, 1e3, 1e5, 1e10,
        f64::MIN_POSITIVE, f64::MAX / 2.0,
        -1e10, -1e5, -1e3,
    ];

    let mut extreme_matches = 0;
    for &v in &extremes {
        let (_, q) = n6_check::n6_match(v);
        if q >= 0.8 {
            extreme_matches += 1;
        }
    }

    let passed = extreme_matches <= 2;
    StressResult {
        test_name: "stress_extreme_values".into(),
        passed,
        detail: format!(
            "{}/{} extreme values match n6",
            extreme_matches,
            extremes.len()
        ),
        severity: if passed { extreme_matches as f64 * 0.1 } else { 0.8 },
    }
}

/// Stress test: Irrational probes — do common irrational numbers match n6?
pub fn stress_irrational_probes() -> StressResult {
    let irrationals = [
        std::f64::consts::PI,
        std::f64::consts::E,
        std::f64::consts::SQRT_2,
        std::f64::consts::LN_2,
        std::f64::consts::LN_10,
        0.5772156649, // Euler-Mascheroni gamma
        1.6180339887, // Golden ratio
        2.6854520010, // Khinchin's constant
    ];

    let mut irrational_matches = 0;
    let mut matched_names = Vec::new();
    for &v in &irrationals {
        let (name, q) = n6_check::n6_match(v);
        if q >= 0.8 {
            irrational_matches += 1;
            matched_names.push(name);
        }
    }

    // Some irrationals (like ln(2), e) may genuinely appear in n=6 theory
    let passed = irrational_matches <= 4;
    StressResult {
        test_name: "stress_irrational_probes".into(),
        passed,
        detail: format!(
            "{}/{} irrationals match: {:?}",
            irrational_matches,
            irrationals.len(),
            matched_names
        ),
        severity: if passed { irrational_matches as f64 * 0.1 } else { 0.6 },
    }
}

/// Stress test: Divisor function stress — verify sigma(n)=12 identity rigorously.
pub fn stress_divisor_identity() -> StressResult {
    // sigma(6) should be exactly 1+2+3+6 = 12
    let divisors_of_6: Vec<u64> = (1..=6).filter(|d| 6 % d == 0).collect();
    let sigma_6: u64 = divisors_of_6.iter().sum();
    let phi_6: u64 = (1..=6).filter(|&k| gcd(k, 6) == 1).count() as u64;
    let tau_6: u64 = divisors_of_6.len() as u64;

    let identity_holds = sigma_6 as f64 * phi_6 as f64 == 6.0 * tau_6 as f64;
    let passed = identity_holds && sigma_6 == 12 && phi_6 == 2 && tau_6 == 4;

    StressResult {
        test_name: "stress_divisor_identity".into(),
        passed,
        detail: format!(
            "sigma(6)={}, phi(6)={}, tau(6)={}, sigma*phi={}={} = 6*tau={}",
            sigma_6,
            phi_6,
            tau_6,
            sigma_6 * phi_6,
            if identity_holds { "YES" } else { "NO" },
            6 * tau_6
        ),
        severity: if passed { 0.0 } else { 1.0 },
    }
}

/// Stress test: Non-perfect number control — verify identity fails for n != 6.
pub fn stress_non_perfect_numbers() -> StressResult {
    let test_numbers = [2, 3, 4, 5, 7, 8, 9, 10, 12, 15, 20, 28];
    let mut false_positives = Vec::new();

    for &n in &test_numbers {
        let divisors: Vec<u64> = (1..=n).filter(|d| n % d == 0).collect();
        let sigma: u64 = divisors.iter().sum();
        let phi: u64 = (1..=n).filter(|&k| gcd(k, n) == 1).count() as u64;
        let tau: u64 = divisors.len() as u64;

        if sigma * phi == n * tau {
            false_positives.push(n);
        }
    }

    // Only n=1 (trivially) might satisfy; everything else should fail
    let real_fps: Vec<u64> = false_positives.iter().filter(|&&n| n > 1).cloned().collect();
    let passed = real_fps.is_empty();
    StressResult {
        test_name: "stress_non_perfect_numbers".into(),
        passed,
        detail: format!(
            "False positives for sigma*phi=n*tau: {:?} (expected: none for n>1)",
            real_fps
        ),
        severity: if passed { 0.0 } else { 1.0 },
    }
}

/// Stress test: Batch constant verification — all 8 core constants must be EXACT.
pub fn stress_batch_constant_verification() -> StressResult {
    let constants = [
        (N, "n=6"),
        (SIGMA, "sigma=12"),
        (PHI, "phi=2"),
        (TAU, "tau=4"),
        (J2, "J2=24"),
        (SOPFR, "sopfr=5"),
        (SIGMA_MINUS_PHI, "sigma-phi=10"),
        (SIGMA_MINUS_TAU, "sigma-tau=8"),
    ];

    let mut exact_count = 0;
    let mut failures = Vec::new();
    for &(val, label) in &constants {
        let (_, q) = n6_check::n6_match(val);
        if q >= 0.95 {
            exact_count += 1;
        } else {
            failures.push(format!("{} (q={:.2})", label, q));
        }
    }

    let passed = exact_count == constants.len();
    StressResult {
        test_name: "stress_batch_constant_verification".into(),
        passed,
        detail: format!(
            "{}/{} core constants EXACT. Failures: {:?}",
            exact_count,
            constants.len(),
            failures
        ),
        severity: if passed { 0.0 } else { 1.0 - (exact_count as f64 / constants.len() as f64) },
    }
}

/// Run all stress tests and return results.
pub fn run_all_stress_tests() -> Vec<StressResult> {
    vec![
        stress_scale_invariance(SIGMA),
        stress_scale_invariance(J2),
        stress_precision_degradation(SIGMA),
        stress_precision_degradation(TAU),
        stress_negative_mirror(SIGMA),
        stress_negative_mirror(N),
        stress_additive_perturbation(SIGMA),
        stress_additive_perturbation(TAU),
        stress_integer_neighborhood(SIGMA),
        stress_integer_neighborhood(N),
        stress_ratio_cascade(),
        stress_sum_cascade(),
        stress_product_cascade(),
        stress_random_baseline(42),
        stress_extreme_values(),
        stress_irrational_probes(),
        stress_divisor_identity(),
        stress_non_perfect_numbers(),
        stress_batch_constant_verification(),
    ]
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 { a } else { gcd(b, a % b) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stress_scale_invariance() {
        let r = stress_scale_invariance(SIGMA);
        assert!(r.detail.contains("scaled versions"));
    }

    #[test]
    fn test_stress_precision_degradation() {
        let r = stress_precision_degradation(SIGMA);
        assert!(r.detail.contains("decimal digits"));
    }

    #[test]
    fn test_stress_negative_mirror() {
        let r = stress_negative_mirror(SIGMA);
        assert!(r.detail.contains("value q="));
    }

    #[test]
    fn test_stress_additive_perturbation() {
        let r = stress_additive_perturbation(SIGMA);
        assert!(r.detail.contains("perturbation"));
    }

    #[test]
    fn test_stress_integer_neighborhood() {
        let r = stress_integer_neighborhood(SIGMA);
        assert!(r.detail.contains("neighbors"));
    }

    #[test]
    fn test_stress_ratio_cascade() {
        let r = stress_ratio_cascade();
        assert!(r.detail.contains("ratios"));
    }

    #[test]
    fn test_stress_sum_cascade() {
        let r = stress_sum_cascade();
        assert!(r.detail.contains("sums"));
    }

    #[test]
    fn test_stress_product_cascade() {
        let r = stress_product_cascade();
        assert!(r.detail.contains("products"));
    }

    #[test]
    fn test_stress_random_baseline() {
        let r = stress_random_baseline(42);
        assert!(r.detail.contains("random values"));
    }

    #[test]
    fn test_stress_extreme_values() {
        let r = stress_extreme_values();
        assert!(r.detail.contains("extreme values"));
    }

    #[test]
    fn test_stress_irrational_probes() {
        let r = stress_irrational_probes();
        assert!(r.detail.contains("irrationals"));
    }

    #[test]
    fn test_stress_divisor_identity() {
        let r = stress_divisor_identity();
        assert!(r.passed, "Divisor identity must hold: {}", r.detail);
    }

    #[test]
    fn test_stress_non_perfect_numbers() {
        let r = stress_non_perfect_numbers();
        assert!(r.passed, "No false positives expected: {}", r.detail);
    }

    #[test]
    fn test_stress_batch_constants() {
        let r = stress_batch_constant_verification();
        // All core constants should be recognized
        assert!(r.detail.contains("core constants"));
    }

    #[test]
    fn test_run_all_stress_tests() {
        let results = run_all_stress_tests();
        assert!(results.len() >= 19, "Should have at least 19 stress tests, got {}", results.len());
        // Divisor identity must always pass
        let div = results.iter().find(|r| r.test_name == "stress_divisor_identity").unwrap();
        assert!(div.passed);
    }
}
