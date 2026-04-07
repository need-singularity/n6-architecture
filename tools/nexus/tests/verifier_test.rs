use nexus::verifier::n6_check::{n6_match, n6_exact_ratio};
use nexus::verifier::feasibility::{verify, Grade};

// ── n6_match tests ──

#[test]
fn test_n6_match_exact() {
    let (name, quality) = n6_match(12.0);
    assert_eq!(name, "sigma");
    assert_eq!(quality, 1.0);
}

#[test]
fn test_n6_match_close() {
    // 11.8 is ~1.7% off from sigma=12 → CLOSE (quality 0.8)
    let (name, quality) = n6_match(11.8);
    assert_eq!(name, "sigma");
    assert!(quality < 1.0, "should not be EXACT");
    assert!(quality > 0.0, "should match something");
}

#[test]
fn test_n6_match_none() {
    // 7.77 is ~3% from sigma-tau=8, so it matches CLOSE.
    // Use 7.0 which is >10% away from all n=6 constants:
    //   nearest are n=6 (16.7% off) and sigma-tau=8 (12.5% off)
    let (name, quality) = n6_match(7.0);
    assert_eq!(name, "none");
    assert_eq!(quality, 0.0);
}

#[test]
fn test_n6_match_j2() {
    let (name, quality) = n6_match(24.0);
    // Could match J2 or sigma*phi — both are 24.0
    assert!(name == "J2" || name == "sigma*phi");
    assert_eq!(quality, 1.0);
}

#[test]
fn test_n6_match_ln43() {
    let (name, quality) = n6_match(0.2877);
    assert_eq!(name, "ln(4/3)");
    assert_eq!(quality, 1.0);
}

// ── n6_exact_ratio tests ──

#[test]
fn test_n6_exact_ratio() {
    // 12.0=sigma EXACT, 24.0=J2 EXACT, 7.0=none → 2/3
    let ratio = n6_exact_ratio(&[12.0, 24.0, 7.0]);
    assert!((ratio - 2.0 / 3.0).abs() < 1e-9, "expected 0.667, got {}", ratio);
}

#[test]
fn test_n6_exact_ratio_empty() {
    assert_eq!(n6_exact_ratio(&[]), 0.0);
}

#[test]
fn test_n6_exact_ratio_all_exact() {
    assert_eq!(n6_exact_ratio(&[6.0, 12.0, 24.0, 4.0]), 1.0);
}

// ── feasibility tests ──

#[test]
fn test_feasibility_grade_s() {
    let result = verify(1.0, 1.0, 1.0, 1.0, 1.0, 1.0);
    assert_eq!(result.grade, Grade::S);
    assert!((result.score - 1.0).abs() < 1e-9);
}

#[test]
fn test_feasibility_grade_d() {
    let result = verify(0.0, 0.0, 0.0, 0.0, 0.0, 0.0);
    assert_eq!(result.grade, Grade::D);
    assert_eq!(result.score, 0.0);
}

#[test]
fn test_feasibility_grade_a() {
    // 0.8*0.25 + 0.7*0.20 + 0.9*0.25 + 0.6*0.15 + 0.5*0.05 + 0.8*0.10
    // = 0.20 + 0.14 + 0.225 + 0.09 + 0.025 + 0.08 = 0.76
    let result = verify(0.8, 0.7, 0.9, 0.6, 0.5, 0.8);
    assert!((result.score - 0.76).abs() < 1e-9, "expected 0.76, got {}", result.score);
    assert_eq!(result.grade, Grade::A);
}

#[test]
fn test_feasibility_grade_b() {
    let result = verify(0.5, 0.5, 0.5, 0.5, 0.5, 0.5);
    assert_eq!(result.score, 0.5);
    assert_eq!(result.grade, Grade::B);
}

#[test]
fn test_feasibility_breakdown_preserved() {
    let result = verify(0.1, 0.2, 0.3, 0.4, 0.5, 0.6);
    assert_eq!(result.breakdown.lens_consensus, 0.1);
    assert_eq!(result.breakdown.cross_validation, 0.2);
    assert_eq!(result.breakdown.physical_check, 0.3);
    assert_eq!(result.breakdown.graph_bonus, 0.4);
    assert_eq!(result.breakdown.novelty, 0.5);
    assert_eq!(result.breakdown.n6_exact, 0.6);
}
