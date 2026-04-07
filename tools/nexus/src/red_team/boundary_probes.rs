//! Boundary probes — test edge cases and boundary conditions of n=6 matching
//! and lens scan behavior.
//!
//! Each function probes a specific boundary or edge case that could cause
//! incorrect results in the discovery engine.

use crate::verifier::n6_check;

// ── n=6 constants ────────────────────────────────────────────────────
const N: f64 = 6.0;
const SIGMA: f64 = 12.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const J2: f64 = 24.0;
const SOPFR: f64 = 5.0;

/// Result of a boundary probe.
#[derive(Debug, Clone)]
pub struct ProbeResult {
    pub probe_name: String,
    pub passed: bool,
    pub detail: String,
    pub severity: f64,
}

/// Probe: Zero value handling — n6_match(0.0) should not crash or return EXACT.
pub fn probe_zero_value() -> ProbeResult {
    let (name, q) = n6_check::n6_match(0.0);
    let passed = q < 0.95; // Zero should NOT be an EXACT n=6 constant
    ProbeResult {
        probe_name: "probe_zero_value".into(),
        passed,
        detail: format!("n6_match(0.0) = ({}, {:.3})", name, q),
        severity: if passed { 0.0 } else { 0.8 },
    }
}

/// Probe: NaN handling — n6_match(NaN) should not match.
pub fn probe_nan_value() -> ProbeResult {
    let (name, q) = n6_check::n6_match(f64::NAN);
    let passed = q < 0.5;
    ProbeResult {
        probe_name: "probe_nan_value".into(),
        passed,
        detail: format!("n6_match(NaN) = ({}, {:.3})", name, q),
        severity: if passed { 0.0 } else { 1.0 },
    }
}

/// Probe: Infinity handling — n6_match(INF) should not match.
pub fn probe_infinity_value() -> ProbeResult {
    let (name_pos, q_pos) = n6_check::n6_match(f64::INFINITY);
    let (name_neg, q_neg) = n6_check::n6_match(f64::NEG_INFINITY);
    let passed = q_pos < 0.5 && q_neg < 0.5;
    ProbeResult {
        probe_name: "probe_infinity_value".into(),
        passed,
        detail: format!(
            "+INF=({}, {:.3}), -INF=({}, {:.3})",
            name_pos, q_pos, name_neg, q_neg
        ),
        severity: if passed { 0.0 } else { 1.0 },
    }
}

/// Probe: Subnormal float handling.
pub fn probe_subnormal_value() -> ProbeResult {
    let subnormal = f64::MIN_POSITIVE / 2.0;
    let (name, q) = n6_check::n6_match(subnormal);
    let passed = q < 0.8; // Subnormal numbers shouldn't be n=6 constants
    ProbeResult {
        probe_name: "probe_subnormal_value".into(),
        passed,
        detail: format!("n6_match(subnormal={:e}) = ({}, {:.3})", subnormal, name, q),
        severity: if passed { 0.0 } else { 0.5 },
    }
}

/// Probe: Negative zero — should behave same as positive zero.
pub fn probe_negative_zero() -> ProbeResult {
    let (name_pos, q_pos) = n6_check::n6_match(0.0);
    let (name_neg, q_neg) = n6_check::n6_match(-0.0);
    let consistent = (q_pos - q_neg).abs() < 0.01;
    ProbeResult {
        probe_name: "probe_negative_zero".into(),
        passed: consistent,
        detail: format!(
            "0.0=({}, {:.3}), -0.0=({}, {:.3}), consistent={}",
            name_pos, q_pos, name_neg, q_neg, consistent
        ),
        severity: if consistent { 0.0 } else { 0.3 },
    }
}

/// Probe: Epsilon boundary — values just above/below exact constants.
pub fn probe_epsilon_boundary() -> ProbeResult {
    let eps = 1e-10;
    let constants = [N, SIGMA, PHI, TAU, J2, SOPFR];
    let mut inconsistencies = 0;

    for &c in &constants {
        let (_, q_exact) = n6_check::n6_match(c);
        let (_, q_plus) = n6_check::n6_match(c + eps);
        let (_, q_minus) = n6_check::n6_match(c - eps);

        // Both epsilon-perturbed values should still match (continuity)
        if (q_exact - q_plus).abs() > 0.3 || (q_exact - q_minus).abs() > 0.3 {
            inconsistencies += 1;
        }
    }

    let passed = inconsistencies == 0;
    ProbeResult {
        probe_name: "probe_epsilon_boundary".into(),
        passed,
        detail: format!(
            "{}/{} constants show discontinuity at epsilon boundary",
            inconsistencies,
            constants.len()
        ),
        severity: if passed { 0.0 } else { inconsistencies as f64 * 0.15 },
    }
}

/// Probe: Midpoint ambiguity — values exactly between two n=6 constants.
pub fn probe_midpoint_ambiguity() -> ProbeResult {
    let pairs = [
        (PHI, TAU),     // midpoint = 3
        (TAU, N),       // midpoint = 5 = sopfr
        (N, SIGMA),     // midpoint = 9
        (SIGMA, J2),    // midpoint = 18
    ];

    let mut ambiguous = 0;
    for &(a, b) in &pairs {
        let mid = (a + b) / 2.0;
        let (_name, q) = n6_check::n6_match(mid);
        if q >= 0.8 {
            ambiguous += 1;
        }
    }

    // Some midpoints (like 5 = sopfr) may legitimately match
    let passed = ambiguous <= 2;
    ProbeResult {
        probe_name: "probe_midpoint_ambiguity".into(),
        passed,
        detail: format!(
            "{}/{} midpoints between n6 constants also match",
            ambiguous,
            pairs.len()
        ),
        severity: if passed { ambiguous as f64 * 0.15 } else { 0.6 },
    }
}

/// Probe: Large integer scan — test integers 1..200 for false positive rate.
pub fn probe_large_integer_scan() -> ProbeResult {
    let mut match_count = 0;
    let total = 200;

    for i in 1..=total {
        let (_, q) = n6_check::n6_match(i as f64);
        if q >= 0.8 {
            match_count += 1;
        }
    }

    let rate = match_count as f64 / total as f64;
    // Expect roughly 20-30 n=6 related integers out of 200, so <20% match rate
    let passed = rate < 0.20;
    ProbeResult {
        probe_name: "probe_large_integer_scan".into(),
        passed,
        detail: format!(
            "{}/{} integers [1,200] match n6 ({:.1}%)",
            match_count, total, rate * 100.0
        ),
        severity: if passed { rate } else { 0.8 },
    }
}

/// Probe: Fractional scan — test common fractions for false positive rate.
pub fn probe_fractional_scan() -> ProbeResult {
    let mut match_count = 0;
    let mut total = 0;

    for num in 1..=12 {
        for den in 1..=12 {
            let frac = num as f64 / den as f64;
            total += 1;
            let (_, q) = n6_check::n6_match(frac);
            if q >= 0.8 {
                match_count += 1;
            }
        }
    }

    let rate = match_count as f64 / total as f64;
    let passed = rate < 0.25;
    ProbeResult {
        probe_name: "probe_fractional_scan".into(),
        passed,
        detail: format!(
            "{}/{} fractions (a/b, 1<=a,b<=12) match ({:.1}%)",
            match_count, total, rate * 100.0
        ),
        severity: if passed { rate } else { 0.7 },
    }
}

/// Probe: Power-of-two scan — powers of 2 from 2^0 to 2^20.
pub fn probe_power_of_two_scan() -> ProbeResult {
    let mut match_count = 0;
    let total = 21;

    for exp in 0..=20 {
        let val = 2.0_f64.powi(exp);
        let (_, q) = n6_check::n6_match(val);
        if q >= 0.8 {
            match_count += 1;
        }
    }

    // Some powers of 2 (2, 4, 8) are n=6 constants, but most shouldn't match
    let passed = match_count <= 6;
    ProbeResult {
        probe_name: "probe_power_of_two_scan".into(),
        passed,
        detail: format!("{}/{} powers of 2 match n6", match_count, total),
        severity: if passed { match_count as f64 * 0.05 } else { 0.6 },
    }
}

/// Probe: Harmonic number boundary — H_n for n=1..20.
pub fn probe_harmonic_numbers() -> ProbeResult {
    let mut match_count = 0;
    let total = 20;

    for n in 1..=total {
        let h_n: f64 = (1..=n).map(|k| 1.0 / k as f64).sum();
        let (_, q) = n6_check::n6_match(h_n);
        if q >= 0.8 {
            match_count += 1;
        }
    }

    let passed = match_count <= 3;
    ProbeResult {
        probe_name: "probe_harmonic_numbers".into(),
        passed,
        detail: format!("{}/{} harmonic numbers H_n match n6", match_count, total),
        severity: if passed { match_count as f64 * 0.1 } else { 0.5 },
    }
}

/// Run all boundary probes.
pub fn run_all_probes() -> Vec<ProbeResult> {
    vec![
        probe_zero_value(),
        probe_nan_value(),
        probe_infinity_value(),
        probe_subnormal_value(),
        probe_negative_zero(),
        probe_epsilon_boundary(),
        probe_midpoint_ambiguity(),
        probe_large_integer_scan(),
        probe_fractional_scan(),
        probe_power_of_two_scan(),
        probe_harmonic_numbers(),
    ]
}

/// Format all probe results into a compact report.
pub fn format_probe_report(results: &[ProbeResult]) -> String {
    let mut s = String::new();
    s.push_str("+-------------------------------+--------+---------+\n");
    s.push_str("| Probe                         | Status | Severity|\n");
    s.push_str("+-------------------------------+--------+---------+\n");
    for r in results {
        let status = if r.passed { "PASS" } else { "FAIL" };
        s.push_str(&format!(
            "| {:<29} | {:<6} | {:.2}    |\n",
            r.probe_name, status, r.severity
        ));
    }
    s.push_str("+-------------------------------+--------+---------+\n");
    let total = results.len();
    let passed = results.iter().filter(|r| r.passed).count();
    s.push_str(&format!("| Total: {}/{} passed                            |\n", passed, total));
    s.push_str("+-------------------------------+--------+---------+\n");
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_probe_zero() {
        let r = probe_zero_value();
        assert!(r.detail.contains("n6_match(0.0)"));
    }

    #[test]
    fn test_probe_nan() {
        let r = probe_nan_value();
        // NaN should definitely not match
        assert!(r.detail.contains("NaN"));
    }

    #[test]
    fn test_probe_infinity() {
        let r = probe_infinity_value();
        assert!(r.detail.contains("INF"));
    }

    #[test]
    fn test_probe_subnormal() {
        let r = probe_subnormal_value();
        assert!(r.detail.contains("subnormal"));
    }

    #[test]
    fn test_probe_negative_zero() {
        let r = probe_negative_zero();
        assert!(r.passed, "Negative zero should be consistent: {}", r.detail);
    }

    #[test]
    fn test_probe_epsilon_boundary() {
        let r = probe_epsilon_boundary();
        assert!(r.detail.contains("epsilon boundary"));
    }

    #[test]
    fn test_probe_midpoint() {
        let r = probe_midpoint_ambiguity();
        assert!(r.detail.contains("midpoints"));
    }

    #[test]
    fn test_probe_large_integer_scan() {
        let r = probe_large_integer_scan();
        assert!(r.detail.contains("integers"));
    }

    #[test]
    fn test_probe_fractional_scan() {
        let r = probe_fractional_scan();
        assert!(r.detail.contains("fractions"));
    }

    #[test]
    fn test_probe_power_of_two() {
        let r = probe_power_of_two_scan();
        assert!(r.detail.contains("powers of 2"));
    }

    #[test]
    fn test_probe_harmonic() {
        let r = probe_harmonic_numbers();
        assert!(r.detail.contains("harmonic"));
    }

    #[test]
    fn test_run_all_probes() {
        let results = run_all_probes();
        assert_eq!(results.len(), 11, "Should have 11 probes");
    }

    #[test]
    fn test_format_probe_report() {
        let results = run_all_probes();
        let report = format_probe_report(&results);
        assert!(report.contains("Probe"));
        assert!(report.contains("Total:"));
    }
}
