//! NEXUS-6 Foundation Tests — n=6 core identity & BT constant verification
//!
//! Covers:
//!   1. n6_check: all 6 primary constants (n, sigma, phi, tau, sopfr, J2)
//!   2. n6_check: derived constants and edge cases
//!   3. scan: simple data scan completes without panic
//!   4. consensus: 3+/7+/12+ lens agreement levels
//!   5. BT identity: sigma(6)*phi(6) = 6*tau(6) and related identities

use std::collections::HashMap;

use nexus6::telescope::consensus::{weighted_consensus, ConsensusLevel};
use nexus6::telescope::lens_trait::LensResult;
use nexus6::telescope::Telescope;
use nexus6::verifier::n6_check::{n6_exact_ratio, n6_match};
use nexus6::verifier::feasibility;

// ═══════════════════════════════════════════════════════════════
// 1. n6_check — Primary constants (n=6, sigma=12, phi=2, tau=4, sopfr=5, J2=24)
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_n6_primary_n() {
    let (name, quality) = n6_match(6.0);
    assert_eq!(name, "n");
    assert_eq!(quality, 1.0, "n=6 must be EXACT");
}

#[test]
fn test_n6_primary_sigma() {
    let (name, quality) = n6_match(12.0);
    assert_eq!(name, "sigma");
    assert_eq!(quality, 1.0, "sigma=12 must be EXACT");
}

#[test]
fn test_n6_primary_phi() {
    let (name, quality) = n6_match(2.0);
    // phi=2 or sigma/n=2 — both valid names for the same value
    assert_eq!(quality, 1.0, "phi=2 must be EXACT");
    assert!(
        name == "phi" || name == "sigma/n" || name == "J2/sigma",
        "2.0 should match phi (or equivalent), got '{}'",
        name
    );
}

#[test]
fn test_n6_primary_tau() {
    let (name, quality) = n6_match(4.0);
    assert_eq!(name, "tau");
    assert_eq!(quality, 1.0, "tau=4 must be EXACT");
}

#[test]
fn test_n6_primary_sopfr() {
    let (name, quality) = n6_match(5.0);
    assert_eq!(name, "sopfr");
    assert_eq!(quality, 1.0, "sopfr=5 must be EXACT");
}

#[test]
fn test_n6_primary_j2() {
    let (name, quality) = n6_match(24.0);
    assert_eq!(quality, 1.0, "J2=24 must be EXACT");
    assert!(
        name == "J2" || name == "sigma*phi",
        "24.0 should match J2 or sigma*phi, got '{}'",
        name
    );
}

#[test]
fn test_n6_primary_mu() {
    let (name, quality) = n6_match(1.0);
    assert_eq!(name, "mu");
    assert_eq!(quality, 1.0, "mu=1 must be EXACT");
}

// ═══════════════════════════════════════════════════════════════
// 2. n6_check — Derived constants
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_n6_derived_sigma_minus_phi() {
    let (name, quality) = n6_match(10.0);
    assert_eq!(name, "sigma-phi");
    assert_eq!(quality, 1.0, "sigma-phi=10 must be EXACT");
}

#[test]
fn test_n6_derived_sigma_minus_tau() {
    let (name, quality) = n6_match(8.0);
    assert_eq!(name, "sigma-tau");
    assert_eq!(quality, 1.0, "sigma-tau=8 must be EXACT");
}

#[test]
fn test_n6_derived_sigma_minus_mu() {
    let (name, quality) = n6_match(11.0);
    assert_eq!(name, "sigma-mu");
    assert_eq!(quality, 1.0, "sigma-mu=11 must be EXACT");
}

#[test]
fn test_n6_derived_sigma_times_tau() {
    let (name, quality) = n6_match(48.0);
    assert_eq!(name, "sigma*tau");
    assert_eq!(quality, 1.0, "sigma*tau=48 must be EXACT");
}

#[test]
fn test_n6_derived_sigma_squared() {
    let (name, quality) = n6_match(144.0);
    assert_eq!(name, "sigma^2");
    assert_eq!(quality, 1.0, "sigma^2=144 must be EXACT");
}

#[test]
fn test_n6_derived_j2_minus_tau() {
    let (name, quality) = n6_match(20.0);
    assert_eq!(name, "J2-tau");
    assert_eq!(quality, 1.0, "J2-tau=20 must be EXACT");
}

#[test]
fn test_n6_derived_n_over_phi() {
    let (name, quality) = n6_match(3.0);
    assert_eq!(name, "n/phi");
    assert_eq!(quality, 1.0, "n/phi=3 must be EXACT");
}

#[test]
fn test_n6_derived_phi_to_tau() {
    let (name, quality) = n6_match(16.0);
    assert_eq!(name, "phi^tau");
    assert_eq!(quality, 1.0, "phi^tau=16 must be EXACT");
}

#[test]
fn test_n6_derived_ln_4_3() {
    // ln(4/3) = 0.28768...
    let (name, quality) = n6_match(0.2877);
    assert_eq!(name, "ln(4/3)");
    assert_eq!(quality, 1.0, "ln(4/3) must be EXACT");
}

#[test]
fn test_n6_derived_tau_sq_over_sigma() {
    // tau^2/sigma = 16/12 = 4/3 = 1.333...
    let (name, quality) = n6_match(1.333);
    assert_eq!(name, "tau^2/sigma");
    assert_eq!(quality, 1.0, "tau^2/sigma=4/3 must be EXACT");
}

// ═══════════════════════════════════════════════════════════════
// 3. n6_check — Edge cases
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_n6_zero_returns_none() {
    let (name, quality) = n6_match(0.0);
    assert_eq!(name, "none");
    assert_eq!(quality, 0.0);
}

#[test]
fn test_n6_far_value_returns_none() {
    // 7.0 is >10% from any n=6 constant
    let (name, quality) = n6_match(7.0);
    assert_eq!(name, "none");
    assert_eq!(quality, 0.0);
}

#[test]
fn test_n6_negative_returns_none() {
    let (name, quality) = n6_match(-6.0);
    assert_eq!(name, "none");
    assert_eq!(quality, 0.0);
}

#[test]
fn test_n6_close_match() {
    // 12.3 is ~2.5% off from sigma=12 -> CLOSE (quality 0.8)
    let (name, quality) = n6_match(12.3);
    assert_eq!(name, "sigma");
    assert_eq!(quality, 0.8, "2.5% off should be CLOSE");
}

#[test]
fn test_n6_weak_match() {
    // 6.5 is ~8.3% off from n=6 -> WEAK (quality 0.5)
    let (name, quality) = n6_match(6.5);
    assert_eq!(name, "n");
    assert_eq!(quality, 0.5, "8.3% off should be WEAK");
}

// ═══════════════════════════════════════════════════════════════
// 4. n6_exact_ratio — Batch verification
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_exact_ratio_all_primary() {
    // All 6 primary constants
    let values = vec![6.0, 12.0, 2.0, 4.0, 5.0, 24.0];
    assert_eq!(n6_exact_ratio(&values), 1.0, "All primary constants should be EXACT");
}

#[test]
fn test_exact_ratio_mixed() {
    // 3 EXACT + 3 non-matching = 50%
    let values = vec![6.0, 12.0, 24.0, 7.0, 99.0, 42.0];
    let ratio = n6_exact_ratio(&values);
    assert!((ratio - 0.5).abs() < 1e-9, "Expected 0.5, got {}", ratio);
}

#[test]
fn test_exact_ratio_empty() {
    assert_eq!(n6_exact_ratio(&[]), 0.0);
}

#[test]
fn test_exact_ratio_none_match() {
    let values = vec![7.0, 13.0, 99.0, 42.0];
    assert_eq!(n6_exact_ratio(&values), 0.0);
}

// ═══════════════════════════════════════════════════════════════
// 5. BT Identity Verification — sigma(6)*phi(6) = 6*tau(6)
// ═══════════════════════════════════════════════════════════════

/// The fundamental identity: sigma(n)*phi(n) = n*tau(n) holds iff n=6
#[test]
fn test_bt_core_identity_sigma_phi_eq_n_tau() {
    let sigma: f64 = 12.0; // sigma(6) = 1+2+3+6 = 12
    let phi: f64 = 2.0;    // phi(6) = |{1,5}| = 2
    let n: f64 = 6.0;
    let tau: f64 = 4.0;    // tau(6) = |{1,2,3,6}| = 4

    let lhs = sigma * phi; // 12 * 2 = 24
    let rhs = n * tau;     // 6 * 4 = 24

    assert_eq!(lhs, rhs, "sigma(6)*phi(6) must equal 6*tau(6)");
    assert_eq!(lhs, 24.0, "The identity value is 24 = J2(6)");

    // Verify both sides match J2
    let (name, quality) = n6_match(lhs);
    assert_eq!(quality, 1.0);
    assert!(name == "J2" || name == "sigma*phi");
}

/// Verify the identity FAILS for n != 6 (small cases)
#[test]
fn test_bt_identity_fails_for_non_6() {
    // Arithmetic function values for small n
    // sigma(n) = sum of divisors, phi(n) = Euler totient, tau(n) = number of divisors
    let cases: Vec<(u64, f64, f64, f64)> = vec![
        // (n, sigma(n), phi(n), tau(n))
        (2,  3.0,  1.0, 2.0),   // 3*1=3 vs 2*2=4
        (3,  4.0,  2.0, 2.0),   // 4*2=8 vs 3*2=6
        (4,  7.0,  2.0, 3.0),   // 7*2=14 vs 4*3=12
        (5,  6.0,  4.0, 2.0),   // 6*4=24 vs 5*2=10
        (6,  12.0, 2.0, 4.0),   // 12*2=24 vs 6*4=24  <-- ONLY match
        (7,  8.0,  6.0, 2.0),   // 8*6=48 vs 7*2=14
        (8,  15.0, 4.0, 4.0),   // 15*4=60 vs 8*4=32
        (9,  13.0, 6.0, 3.0),   // 13*6=78 vs 9*3=27
        (10, 18.0, 4.0, 4.0),   // 18*4=72 vs 10*4=40
        (12, 28.0, 4.0, 6.0),   // 28*4=112 vs 12*6=72
        (24, 60.0, 8.0, 8.0),   // 60*8=480 vs 24*8=192
        (28, 56.0, 12.0, 6.0),  // 56*12=672 vs 28*6=168 (next perfect number)
    ];

    for (n, sigma, phi, tau) in cases {
        let lhs = sigma * phi;
        let rhs = (n as f64) * tau;
        if n == 6 {
            assert_eq!(lhs, rhs, "Identity MUST hold for n=6");
        } else {
            assert_ne!(lhs, rhs, "Identity must NOT hold for n={} (lhs={}, rhs={})", n, lhs, rhs);
        }
    }
}

/// BT-33: Transformer sigma=12 atom
#[test]
fn test_bt33_transformer_sigma_12() {
    // BERT base: 12 attention heads, 12 layers
    // GPT-3: 12288 = 12 * 1024 hidden dim
    let heads = 12.0;
    let (name, quality) = n6_match(heads);
    assert_eq!(name, "sigma");
    assert_eq!(quality, 1.0, "12 attention heads = sigma(6)");
}

/// BT-54: AdamW quintuplet — all 5 hyperparameters from n=6
#[test]
fn test_bt54_adamw_quintuplet() {
    // beta1 = 1 - 1/(sigma-phi) = 1 - 1/10 = 0.9
    let beta1: f64 = 1.0 - 1.0 / 10.0;
    assert!((beta1 - 0.9).abs() < 1e-10);

    // beta2 = 1 - 1/(J2-tau) = 1 - 1/20 = 0.95
    let beta2: f64 = 1.0 - 1.0 / 20.0;
    assert!((beta2 - 0.95).abs() < 1e-10);

    // epsilon = 10^-(sigma-tau) = 10^-8
    let eps = 10.0_f64.powi(-(8_i32)); // sigma-tau = 8
    assert!((eps - 1e-8).abs() < 1e-15);

    // weight_decay = 1/(sigma-phi) = 0.1
    let wd: f64 = 1.0 / 10.0;
    assert!((wd - 0.1).abs() < 1e-10);
    let (name, quality) = n6_match(10.0);
    assert_eq!(name, "sigma-phi");
    assert_eq!(quality, 1.0);

    // gradient_clip = R(6) = 1
    let clip = 1.0;
    let (name_clip, quality_clip) = n6_match(clip);
    assert_eq!(name_clip, "mu");
    assert_eq!(quality_clip, 1.0);
}

/// BT-56: Complete n=6 LLM architecture
#[test]
fn test_bt56_complete_llm() {
    // d_model = 2^sigma = 2^12 = 4096
    let d_model = 2.0_f64.powi(12);
    assert_eq!(d_model, 4096.0);
    let (_, q) = n6_match(12.0); // sigma
    assert_eq!(q, 1.0);

    // num_layers = 2^sopfr = 2^5 = 32
    let layers = 2.0_f64.powi(5);
    assert_eq!(layers, 32.0);
    let (_, q) = n6_match(5.0); // sopfr
    assert_eq!(q, 1.0);

    // d_head = 2^(sigma-sopfr) = 2^7 = 128
    let d_head = 2.0_f64.powi(12 - 5);
    assert_eq!(d_head, 128.0);
}

/// BT-58: sigma-tau=8 universal AI constant
#[test]
fn test_bt58_sigma_tau_8_universal() {
    let sigma_tau = 12.0 - 4.0;
    assert_eq!(sigma_tau, 8.0);
    let (name, quality) = n6_match(8.0);
    assert_eq!(name, "sigma-tau");
    assert_eq!(quality, 1.0);

    // Appears in: LoRA rank=8, MoE experts=8, KV heads=8, batch=8*N, etc.
}

/// BT-64: 1/(sigma-phi)=0.1 universal regularization
#[test]
fn test_bt64_universal_regularization() {
    let sigma_phi: f64 = 12.0 - 2.0; // = 10
    let reg_value: f64 = 1.0 / sigma_phi;
    assert!((reg_value - 0.1).abs() < 1e-10);
    let (name, quality) = n6_match(sigma_phi);
    assert_eq!(name, "sigma-phi");
    assert_eq!(quality, 1.0);
}

/// BT-90: SM = phi * K6 contact number
#[test]
fn test_bt90_sm_phi_k6() {
    // sigma^2 = 144 = phi * K6_contact = 2 * 72
    let sigma_sq = 12.0 * 12.0;
    assert_eq!(sigma_sq, 144.0);
    let (name, quality) = n6_match(144.0);
    assert_eq!(name, "sigma^2");
    assert_eq!(quality, 1.0);
}

// ═══════════════════════════════════════════════════════════════
// 6. Scan — Simple data completes without panic
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_scan_n6_constants_no_panic() {
    let telescope = Telescope::new();
    // All 6 primary n=6 constants as data
    let data = vec![6.0, 12.0, 2.0, 4.0, 5.0, 24.0];
    let results = telescope.scan_all(&data, 6, 1);
    assert!(!results.is_empty(), "Scan of n=6 constants should produce results");
}

#[test]
fn test_scan_empty_data_no_panic() {
    let telescope = Telescope::new();
    // Edge case: minimal data
    let data = vec![1.0, 2.0];
    let results = telescope.scan_all(&data, 2, 1);
    // Should not panic, results can be empty or minimal
    let _ = results;
}

#[test]
fn test_scan_large_values_no_panic() {
    let telescope = Telescope::new();
    let data = vec![1e10, 1e-10, 144.0, 48.0, 0.2877, 1.333];
    let results = telescope.scan_all(&data, 6, 1);
    assert!(!results.is_empty());
}

#[test]
fn test_scan_all_derived_constants() {
    let telescope = Telescope::new();
    // All derived constants
    let data = vec![
        10.0, 8.0, 11.0, 48.0, 144.0, 16.0,
        1.333, 3.0, 20.0, 0.2877,
    ];
    let results = telescope.scan_all(&data, 10, 1);
    assert!(!results.is_empty());
}

// ═══════════════════════════════════════════════════════════════
// 7. Consensus — Level verification
// ═══════════════════════════════════════════════════════════════

fn make_consensus_input(n_lenses: usize, pattern: &str) -> (HashMap<String, LensResult>, HashMap<String, f64>) {
    let mut results: HashMap<String, LensResult> = HashMap::new();
    let mut hit_rates: HashMap<String, f64> = HashMap::new();

    for i in 0..n_lenses {
        let name = format!("Lens_{}", i);
        let mut lr = HashMap::new();
        lr.insert(pattern.to_string(), vec![1.0]);
        results.insert(name.clone(), lr);
        hit_rates.insert(name, 0.8);
    }

    (results, hit_rates)
}

#[test]
fn test_consensus_below_3_no_result() {
    let (results, hit_rates) = make_consensus_input(2, "test_pattern");
    let consensus = weighted_consensus(&results, &hit_rates);
    // Less than 3 lenses = no consensus
    assert!(consensus.is_empty(), "2 lenses should not produce consensus");
}

#[test]
fn test_consensus_3_lenses_candidate() {
    let (results, hit_rates) = make_consensus_input(3, "test_pattern");
    let consensus = weighted_consensus(&results, &hit_rates);
    assert_eq!(consensus.len(), 1);
    assert_eq!(consensus[0].level, ConsensusLevel::Candidate);
    assert_eq!(consensus[0].agreeing_lenses.len(), 3);
}

#[test]
fn test_consensus_7_lenses_high() {
    let (results, hit_rates) = make_consensus_input(7, "test_pattern");
    let consensus = weighted_consensus(&results, &hit_rates);
    assert_eq!(consensus.len(), 1);
    assert_eq!(consensus[0].level, ConsensusLevel::High);
    assert_eq!(consensus[0].agreeing_lenses.len(), 7);
}

#[test]
fn test_consensus_12_lenses_confirmed() {
    let (results, hit_rates) = make_consensus_input(12, "test_pattern");
    let consensus = weighted_consensus(&results, &hit_rates);
    assert_eq!(consensus.len(), 1);
    assert_eq!(consensus[0].level, ConsensusLevel::Confirmed);
    assert_eq!(consensus[0].agreeing_lenses.len(), 12);
}

#[test]
fn test_consensus_weighted_score_sum() {
    let (results, hit_rates) = make_consensus_input(5, "test_pattern");
    let consensus = weighted_consensus(&results, &hit_rates);
    assert_eq!(consensus.len(), 1);
    // 5 lenses * 0.8 weight each = 4.0
    assert!((consensus[0].weighted_score - 4.0).abs() < 1e-10);
}

#[test]
fn test_consensus_multiple_patterns() {
    let mut results: HashMap<String, LensResult> = HashMap::new();
    let mut hit_rates: HashMap<String, f64> = HashMap::new();

    // 4 lenses agree on pattern_a, 3 agree on pattern_b
    for i in 0..4 {
        let name = format!("L{}", i);
        let mut lr = HashMap::new();
        lr.insert("pattern_a".to_string(), vec![1.0]);
        if i < 3 {
            lr.insert("pattern_b".to_string(), vec![2.0]);
        }
        results.insert(name.clone(), lr);
        hit_rates.insert(name, 1.0);
    }

    let consensus = weighted_consensus(&results, &hit_rates);
    assert_eq!(consensus.len(), 2, "Should find 2 consensus patterns");

    let pattern_a = consensus.iter().find(|c| c.pattern_id == "pattern_a").unwrap();
    assert_eq!(pattern_a.agreeing_lenses.len(), 4);
    assert_eq!(pattern_a.level, ConsensusLevel::Candidate);

    let pattern_b = consensus.iter().find(|c| c.pattern_id == "pattern_b").unwrap();
    assert_eq!(pattern_b.agreeing_lenses.len(), 3);
    assert_eq!(pattern_b.level, ConsensusLevel::Candidate);
}

// ═══════════════════════════════════════════════════════════════
// 8. Feasibility — Integration with n6_check
// ═══════════════════════════════════════════════════════════════

#[test]
fn test_feasibility_with_full_n6_data() {
    // All primary constants -> 100% EXACT ratio
    let values = vec![6.0, 12.0, 2.0, 4.0, 5.0, 24.0];
    let ratio = n6_exact_ratio(&values);
    assert_eq!(ratio, 1.0);

    let result = feasibility::verify(0.9, 0.8, 0.9, 0.5, 0.7, ratio);
    // High n6_exact should contribute to a high score
    assert!(result.score > 0.8, "Full n6 data + high lens scores -> high feasibility, got {}", result.score);
}

#[test]
fn test_feasibility_with_zero_n6() {
    // No n6 matches -> n6_exact = 0.0
    let values = vec![7.0, 13.0, 99.0];
    let ratio = n6_exact_ratio(&values);
    assert_eq!(ratio, 0.0);

    let result = feasibility::verify(0.5, 0.5, 0.5, 0.0, 0.5, ratio);
    // Score should still be positive from other components
    assert!(result.score > 0.0);
    assert!(result.score < 0.8, "Zero n6 should lower score, got {}", result.score);
}

// ═══════════════════════════════════════════════════════════════
// 9. Cross-verification: n6 constants are self-consistent
// ═══════════════════════════════════════════════════════════════

/// Verify all n=6 arithmetic function relationships hold
#[test]
fn test_n6_arithmetic_consistency() {
    let n: f64 = 6.0;
    let sigma: f64 = 12.0;     // sum of divisors
    let phi: f64 = 2.0;        // Euler totient
    let tau: f64 = 4.0;        // number of divisors
    let sopfr: f64 = 5.0;      // sum of prime factors (2+3)
    let j2: f64 = 24.0;        // Jordan function J_2(6)
    let mu: f64 = 1.0;         // Mobius function |mu(6)|

    // Core identity
    assert_eq!(sigma * phi, n * tau, "sigma*phi = n*tau");

    // J2 relationships
    assert_eq!(j2, sigma * phi, "J2 = sigma*phi");
    assert_eq!(j2, n * tau, "J2 = n*tau");

    // Derived values
    assert_eq!(sigma - phi, 10.0, "sigma-phi = 10");
    assert_eq!(sigma - tau, 8.0, "sigma-tau = 8");
    assert_eq!(sigma - mu, 11.0, "sigma-mu = 11");
    assert_eq!(sigma * tau, 48.0, "sigma*tau = 48");
    assert_eq!(sigma * sigma, 144.0, "sigma^2 = 144");
    assert_eq!(n / phi, 3.0, "n/phi = 3");
    assert_eq!(j2 - tau, 20.0, "J2-tau = 20");
    assert_eq!(phi.powf(tau), 16.0, "phi^tau = 16");

    // Proper divisors of 6: {1, 2, 3}
    // Sum of reciprocals: 1/1 + 1/2 + 1/3 + 1/6 = 1 + 0.5 + 0.333... + 0.1666... = 2
    // This is the definition of a perfect number!
    let reciprocal_sum: f64 = 1.0 / 1.0 + 1.0 / 2.0 + 1.0 / 3.0 + 1.0 / 6.0;
    assert!((reciprocal_sum - 2.0).abs() < 1e-10, "6 is perfect: sum of d|6 of 1/d = sigma(6)/6 = 2");

    // Egyptian fraction: proper divisors give 1/2 + 1/3 + 1/6 = 1
    let egyptian: f64 = 1.0 / 2.0 + 1.0 / 3.0 + 1.0 / 6.0;
    assert!((egyptian - 1.0).abs() < 1e-10, "Egyptian fraction identity: 1/2 + 1/3 + 1/6 = 1");

    // sopfr = 2 + 3 (prime factorization 6 = 2*3)
    assert_eq!(sopfr, 2.0 + 3.0, "sopfr(6) = 2+3");
}

/// Verify that 6 is the ONLY solution for sigma*phi = n*tau in [2..100]
#[test]
fn test_n6_uniqueness_up_to_100() {
    fn sigma(n: u64) -> u64 {
        (1..=n).filter(|d| n % d == 0).sum()
    }
    fn phi(n: u64) -> u64 {
        (1..=n).filter(|k| gcd(*k, n) == 1).count() as u64
    }
    fn tau(n: u64) -> u64 {
        (1..=n).filter(|d| n % d == 0).count() as u64
    }
    fn gcd(a: u64, b: u64) -> u64 {
        if b == 0 { a } else { gcd(b, a % b) }
    }

    let mut solutions = Vec::new();
    for n in 2..=100 {
        if sigma(n) * phi(n) == n * tau(n) {
            solutions.push(n);
        }
    }

    assert_eq!(solutions, vec![6], "n=6 must be the ONLY solution in [2,100]");
}
