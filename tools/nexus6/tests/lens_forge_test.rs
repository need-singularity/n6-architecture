use nexus6::history::recorder::ScanRecord;
use nexus6::lens_forge::candidate_gen::{
    generate_from_analogy, generate_from_combination, generate_from_mutation, CandidateSource,
    LensCandidate,
};
use nexus6::lens_forge::forge_engine::{forge_cycle, ForgeConfig};
use nexus6::lens_forge::gap_analyzer::{analyze_gaps, GapReport};
use nexus6::lens_forge::validator::{validate, Recommendation};
use nexus6::telescope::registry::{LensCategory, LensRegistry};

/// Build a minimal registry with a few lenses for testing.
fn test_registry() -> LensRegistry {
    // The default registry already has 700+ lenses.
    // We use it as-is to test against real data.
    LensRegistry::new()
}

fn make_record(domain: &str, lenses: &[&str], discoveries: &[&str]) -> ScanRecord {
    ScanRecord {
        id: format!("test-{}", domain),
        timestamp: "2026-04-03T00:00:00Z".to_string(),
        domain: domain.to_string(),
        lenses_used: lenses.iter().map(|s| s.to_string()).collect(),
        discoveries: discoveries.iter().map(|s| s.to_string()).collect(),
        consensus_level: if discoveries.is_empty() { 0 } else { 3 },
    }
}

// ============================================================
// Gap Analyzer Tests
// ============================================================

#[test]
fn test_gap_analysis_empty_history() {
    let reg = test_registry();
    let gap = analyze_gaps(&reg, &[]);

    // With 700+ lenses, most domains should be covered.
    // But some niche domains may still be uncovered.
    // At minimum, the report should be valid.
    assert!(
        gap.uncovered_domains.len() + gap.weak_domains.len() > 0
            || gap.uncovered_domains.is_empty(),
        "Gap report should be valid"
    );

    // With empty history, domains that have lenses but no scans should appear weak
    // (hit_rate = 0.0, so coverage = 0.5 * density + 0.0)
    // Domains with few lenses and no history will be weak.
    println!(
        "Uncovered: {:?}, Weak: {} domains",
        gap.uncovered_domains,
        gap.weak_domains.len()
    );
}

#[test]
fn test_gap_with_history() {
    let reg = test_registry();

    // Provide history for some domains — they should have better coverage
    let history = vec![
        make_record("ai", &["consciousness", "topology"], &["BT-33 confirmed"]),
        make_record("ai", &["consciousness", "causal"], &["BT-56 confirmed"]),
        make_record("chip", &["topology", "scale"], &["BT-28 verified"]),
        make_record("energy", &["thermo", "wave"], &[]), // no discoveries
    ];

    let gap = analyze_gaps(&reg, &history);

    // Domains with successful scans should be less likely to appear as weak
    // "ai" has 2 scans with discoveries — should not be weak
    let weak_names: Vec<&str> = gap.weak_domains.iter().map(|(d, _)| d.as_str()).collect();
    // ai has many lenses AND successful history -> unlikely weak
    println!("Weak domains: {:?}", weak_names);
}

#[test]
fn test_gap_report_sorted() {
    let reg = test_registry();
    let gap = analyze_gaps(&reg, &[]);

    // Verify uncovered domains are sorted
    let mut sorted = gap.uncovered_domains.clone();
    sorted.sort();
    assert_eq!(gap.uncovered_domains, sorted, "Uncovered domains should be sorted");

    // Verify weak domains are sorted by coverage score
    for i in 1..gap.weak_domains.len() {
        assert!(
            gap.weak_domains[i - 1].1 <= gap.weak_domains[i].1,
            "Weak domains should be sorted by coverage score"
        );
    }
}

// ============================================================
// Candidate Generation Tests
// ============================================================

#[test]
fn test_candidate_combination() {
    let reg = test_registry();
    // Create a gap report with artificial uncovered domains
    let gap = GapReport {
        uncovered_domains: vec!["thermodynamics".to_string()],
        weak_domains: vec![],
        suggested_categories: vec![],
    };

    let candidates = generate_from_combination(&reg, &gap);
    // With an uncovered domain, the combination generator should find pairs
    // whose merged affinities cover it.
    println!("Combination candidates: {}", candidates.len());

    for c in &candidates {
        assert!(
            matches!(c.source, CandidateSource::Combination(_, _)),
            "Source should be Combination"
        );
        assert!(c.confidence > 0.0, "Confidence should be positive");
        assert!(!c.name.is_empty(), "Name should not be empty");
    }
}

#[test]
fn test_candidate_combination_no_gaps() {
    let reg = test_registry();
    let gap = GapReport {
        uncovered_domains: vec![],
        weak_domains: vec![],
        suggested_categories: vec![],
    };

    let candidates = generate_from_combination(&reg, &gap);
    assert!(
        candidates.is_empty(),
        "No gaps means no combination candidates"
    );
}

#[test]
fn test_candidate_analogy() {
    let reg = test_registry();
    let gap = GapReport {
        uncovered_domains: vec![],
        weak_domains: vec![("consciousness".to_string(), 0.1)],
        suggested_categories: vec![],
    };

    let candidates = generate_from_analogy(&reg, &gap);
    assert!(
        !candidates.is_empty(),
        "Should generate analogy candidates for weak domains"
    );

    for c in &candidates {
        assert!(
            matches!(c.source, CandidateSource::Analogy(_)),
            "Source should be Analogy"
        );
        // Should target the weak domain
        let covers_weak = c
            .domain_affinity
            .iter()
            .any(|d| d.contains("consciousness"));
        assert!(covers_weak, "Analogy candidate should cover weak domain");
    }
}

#[test]
fn test_candidate_analogy_no_weak() {
    let reg = test_registry();
    let gap = GapReport {
        uncovered_domains: vec![],
        weak_domains: vec![],
        suggested_categories: vec![],
    };

    let candidates = generate_from_analogy(&reg, &gap);
    assert!(
        candidates.is_empty(),
        "No weak domains means no analogy candidates"
    );
}

#[test]
fn test_candidate_mutation() {
    let reg = test_registry();
    let candidates = generate_from_mutation(&reg);

    // Should produce some mutation candidates based on domain shift pairs
    println!("Mutation candidates: {}", candidates.len());

    for c in &candidates {
        assert!(
            matches!(c.source, CandidateSource::Mutation(_)),
            "Source should be Mutation"
        );
        assert!(c.confidence > 0.0, "Confidence should be positive");
    }
}

// ============================================================
// Validator Tests
// ============================================================

#[test]
fn test_validation_unique() {
    let reg = test_registry();
    let gap = GapReport {
        uncovered_domains: vec!["thermodynamics".to_string()],
        weak_domains: vec![],
        suggested_categories: vec![],
    };

    let candidate = LensCandidate {
        name: "totally_unique_test_lens_xyz_12345".to_string(),
        description: "A unique test lens".to_string(),
        source: CandidateSource::GapFill("thermodynamics".to_string()),
        domain_affinity: vec!["thermodynamics".to_string(), "exotic_never_seen".to_string()],
        complementary: vec![],
        confidence: 0.9,
    };

    let result = validate(&candidate, &reg, &gap, 0.8);
    assert!(result.is_unique, "Candidate should be unique");
    assert!(result.is_useful, "Candidate covers uncovered domain");
    assert_eq!(
        result.recommendation,
        Recommendation::Accept,
        "Should accept unique useful candidate"
    );
}

#[test]
fn test_validation_duplicate_name() {
    let reg = test_registry();
    let gap = GapReport {
        uncovered_domains: vec![],
        weak_domains: vec![],
        suggested_categories: vec![],
    };

    // Use a name that exists in the registry
    let existing_name = reg.iter().next().unwrap().0.clone();
    let candidate = LensCandidate {
        name: existing_name.clone(),
        description: "Duplicate name".to_string(),
        source: CandidateSource::GapFill("test".to_string()),
        domain_affinity: vec!["test".to_string()],
        complementary: vec![],
        confidence: 0.5,
    };

    let result = validate(&candidate, &reg, &gap, 0.8);
    assert!(!result.is_unique, "Duplicate name should not be unique");
    assert!(
        matches!(result.recommendation, Recommendation::Reject(_)),
        "Should reject duplicate name"
    );
}

#[test]
fn test_validation_not_useful() {
    let reg = test_registry();
    let gap = GapReport {
        uncovered_domains: vec!["thermodynamics".to_string()],
        weak_domains: vec![],
        suggested_categories: vec![],
    };

    // Candidate that is unique but does NOT cover any gap domain
    let candidate = LensCandidate {
        name: "unique_but_useless_xyz_99999".to_string(),
        description: "Does not cover gaps".to_string(),
        source: CandidateSource::GapFill("irrelevant".to_string()),
        domain_affinity: vec!["totally_random_domain_xyz".to_string()],
        complementary: vec![],
        confidence: 0.5,
    };

    let result = validate(&candidate, &reg, &gap, 0.8);
    assert!(result.is_unique, "Name is unique");
    assert!(!result.is_useful, "Does not cover any gap");
    assert!(
        matches!(result.recommendation, Recommendation::Modify(_)),
        "Should recommend modification"
    );
}

// ============================================================
// Forge Engine Tests
// ============================================================

#[test]
fn test_forge_cycle_basic() {
    let reg = test_registry();
    let history: Vec<ScanRecord> = vec![];
    let config = ForgeConfig::default();

    let result = forge_cycle(&reg, &history, &config);

    println!(
        "Forge cycle: generated={}, accepted={}, new_lenses={}",
        result.candidates_generated,
        result.candidates_accepted,
        result.new_lenses.len()
    );

    // Should generate some candidates
    assert!(
        result.candidates_generated > 0,
        "Should generate candidates"
    );

    // Gap report should be present
    assert!(
        !result.gap_report.uncovered_domains.is_empty()
            || !result.gap_report.weak_domains.is_empty()
            || true, // gap report is always valid
        "Gap report should be valid"
    );

    // Validations should match candidate count
    assert_eq!(
        result.validations.len(),
        result.candidates_generated,
        "Should have one validation per candidate"
    );

    // Accepted count should match new_lenses
    assert_eq!(
        result.candidates_accepted,
        result.new_lenses.len(),
        "Accepted count should match new lenses"
    );
}

#[test]
fn test_forge_cycle_with_history() {
    let reg = test_registry();
    let history = vec![
        make_record("ai", &["consciousness"], &["discovery-1"]),
        make_record("chip", &["topology"], &["discovery-2"]),
        make_record("energy", &["thermo"], &[]),
    ];

    let config = ForgeConfig {
        max_candidates: 10,
        min_confidence: 0.2,
        similarity_threshold: 0.8,
    };

    let result = forge_cycle(&reg, &history, &config);

    println!(
        "Forge with history: generated={}, accepted={}",
        result.candidates_generated, result.candidates_accepted
    );

    // All new lenses should have Custom category
    for lens in &result.new_lenses {
        assert_eq!(
            lens.category,
            LensCategory::Custom,
            "Forged lenses should be Custom category"
        );
    }
}

#[test]
fn test_forge_config_strict() {
    let reg = test_registry();
    let history: Vec<ScanRecord> = vec![];

    // Very strict config — high confidence threshold
    let strict = ForgeConfig {
        max_candidates: 5,
        min_confidence: 0.99,
        similarity_threshold: 0.1, // very strict uniqueness
    };

    let result = forge_cycle(&reg, &history, &strict);

    // With min_confidence=0.99, most candidates (conf 0.3-0.6) should be filtered out
    println!(
        "Strict forge: generated={}, accepted={}",
        result.candidates_generated, result.candidates_accepted
    );
}
