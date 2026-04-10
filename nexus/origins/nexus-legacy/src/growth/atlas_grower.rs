//! Atlas Grower — tracks Math Atlas coverage and plans expansion.
//!
//! Identifies constants from BTs not yet in the atlas, finds duplicate entries,
//! and generates plans to grow the atlas systematically.

// ── n=6 constants ────────────────────────────────────────────────────
#[allow(unused)]
const N: usize = 6;                 // the perfect number
const SIGMA: usize = 12;          // σ(6) = sum of divisors
const _PHI: usize = 2;             // φ(6) = Euler totient
const TAU: usize = 4;             // τ(6) = number of divisors
const _J2: usize = 24;             // J₂(6) = Jordan totient
const _SOPFR: usize = 5;           // sopfr(6) = 2+3
const _SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10
const _SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// Approximate total constants in math_atlas.json.
const KNOWN_ATLAS_CONSTANTS: usize = 1100;

/// Total domains covered in atlas.
const ATLAS_DOMAINS_COVERED: usize = 28;

/// Core n=6 expressions and their values (the fundamental vocabulary).
const CORE_N6_VALUES: &[(&str, f64)] = &[
    ("n", 6.0),
    ("φ", 2.0),
    ("τ", 4.0),
    ("sopfr", 5.0),
    ("n/φ", 3.0),
    ("σ-τ", 8.0),
    ("σ-φ", 10.0),
    ("σ-μ", 11.0),
    ("σ", 12.0),
    ("J₂", 24.0),
    ("σ²", 144.0),
    ("σ·J₂", 288.0),
    ("2^φ", 4.0),
    ("2^n", 64.0),
    ("2^σ", 4096.0),
    ("φ^τ", 16.0),
    ("τ²/σ", 1.333),     // 4/3
    ("φ²/n", 0.667),     // 2/3
    ("1/(σ-φ)", 0.1),
    ("σ/(σ-φ)", 1.2),    // PUE
    ("1-1/e", 0.632),    // Boltzmann gate
    ("ln(4/3)", 0.288),  // Mertens dropout
];

/// Snapshot of Math Atlas state.
#[derive(Debug, Clone)]
pub struct AtlasState {
    /// Total constants registered (~1100+)
    pub total_constants: usize,
    /// Number of domains covered
    pub domains_covered: usize,
    /// Constants from BTs that aren't in the atlas yet (estimated)
    pub constants_without_bt: usize,
    /// Groups of entries sharing the same numerical value
    pub duplicate_values: Vec<(f64, Vec<String>)>,
}

/// A new atlas entry to add.
#[derive(Debug, Clone)]
pub struct AtlasEntry {
    /// Human-readable name
    pub name: String,
    /// Numerical value
    pub value: f64,
    /// n=6 expression producing this value
    pub n6_expression: String,
    /// Primary domain
    pub domain: String,
    /// Source BT if applicable
    pub source_bt: Option<String>,
}

/// Growth plan for atlas expansion.
#[derive(Debug, Clone)]
pub struct AtlasGrowthPlan {
    /// New entries to add
    pub new_entries: Vec<AtlasEntry>,
    /// Pairs of entries that should be consolidated (name_a, name_b)
    pub consolidations: Vec<(String, String)>,
}

/// Assess the current state of the Math Atlas.
pub fn assess_atlas_state() -> AtlasState {
    let duplicates = find_duplicates();
    // Estimate: ~50 constants from BTs not yet registered
    let missing_count = estimate_missing_constants();

    AtlasState {
        total_constants: KNOWN_ATLAS_CONSTANTS,
        domains_covered: ATLAS_DOMAINS_COVERED,
        constants_without_bt: missing_count,
        duplicate_values: duplicates,
    }
}

/// Find constants from BTs that are missing from the atlas.
///
/// Returns atlas entries ready to be added.
pub fn find_missing_constants() -> Vec<AtlasEntry> {
    let mut missing = Vec::new();

    // BT-105~127 constants that may not be in atlas yet
    let recent_bt_constants: &[(&str, f64, &str, &str, &str)] = &[
        // (name, value, expression, domain, bt_id)
        ("SLE₆ kappa", 6.0, "n=6", "pure-mathematics", "BT-105"),
        ("S₃ order", 6.0, "n=6", "pure-mathematics", "BT-106"),
        ("Ramanujan eta exponent", 24.0, "J₂=24", "pure-mathematics", "BT-107"),
        ("Perfect consonance ratio count", 12.0, "σ=12", "display-audio", "BT-108"),
        ("ζ(2) denominator", 6.0, "n=6", "pure-mathematics", "BT-109"),
        ("M-theory dim", 11.0, "σ-μ=11", "cosmology-particle", "BT-110"),
        ("SQ bandgap", 1.333, "τ²/σ=4/3", "solar-architecture", "BT-111"),
        ("Koide Q", 0.667, "φ²/n=2/3", "cosmology-particle", "BT-112"),
        ("SOLID principles", 5.0, "sopfr=5", "software-design", "BT-113"),
        ("AES block bits", 128.0, "2^(σ-sopfr)=128", "cryptography", "BT-114"),
        ("OSI layers", 7.0, "σ-sopfr=7", "network-protocol", "BT-115"),
        ("ACID properties", 4.0, "τ=4", "software-design", "BT-116"),
        ("SE(3) dim", 6.0, "n=6", "robotics", "BT-123"),
        ("Bilateral symmetry", 2.0, "φ=2", "robotics", "BT-124"),
        ("Quadruped legs", 4.0, "τ=4", "robotics", "BT-125"),
        ("Human fingers", 5.0, "sopfr=5", "robotics", "BT-126"),
        ("3D kissing number", 12.0, "σ=12", "robotics", "BT-127"),
        ("Kyoto greenhouse gases", 6.0, "n=6", "environmental-protection", "BT-118"),
        ("Troposphere height km", 12.0, "σ=12", "environmental-protection", "BT-119"),
        ("Water treatment pH", 6.0, "n=6", "environmental-protection", "BT-120"),
        ("Major plastics count", 6.0, "n=6", "environmental-protection", "BT-121"),
        ("Honeycomb symmetry", 6.0, "n=6", "environmental-protection", "BT-122"),
    ];

    for (name, value, expr, domain, bt) in recent_bt_constants {
        missing.push(AtlasEntry {
            name: name.to_string(),
            value: *value,
            n6_expression: expr.to_string(),
            domain: domain.to_string(),
            source_bt: Some(bt.to_string()),
        });
    }

    missing
}

/// Find groups of atlas entries that share the same value (potential duplicates).
pub fn find_duplicates() -> Vec<(f64, Vec<String>)> {
    // Known value collisions in the atlas (same number, different contexts)
    vec![
        (6.0, vec![
            "n (perfect number)".to_string(),
            "carbon Z".to_string(),
            "DOF (SE3)".to_string(),
            "greenhouse gas count".to_string(),
            "major plastics".to_string(),
            "honeycomb symmetry".to_string(),
        ]),
        (12.0, vec![
            "σ (divisor sum)".to_string(),
            "semitones/octave".to_string(),
            "joints (robot)".to_string(),
            "HBM stacks".to_string(),
            "troposphere km".to_string(),
            "kissing number 3D".to_string(),
        ]),
        (24.0, vec![
            "J₂ (Jordan totient)".to_string(),
            "fps standard".to_string(),
            "bit depth audio".to_string(),
            "Leech lattice dim".to_string(),
            "glucose atoms".to_string(),
            "Ramanujan eta exp".to_string(),
        ]),
        (8.0, vec![
            "σ-τ".to_string(),
            "LoRA rank default".to_string(),
            "KV heads".to_string(),
            "EnCodec codebooks".to_string(),
            "S₈ polysulfide".to_string(),
        ]),
        (0.1, vec![
            "1/(σ-φ)".to_string(),
            "weight decay".to_string(),
            "reconnection rate".to_string(),
            "E-O loss".to_string(),
        ]),
    ]
}

/// Generate a Claude CLI prompt for atlas expansion.
pub fn generate_atlas_expansion_prompt(entries: &[AtlasEntry]) -> String {
    let mut entry_list = String::new();
    for (i, e) in entries.iter().enumerate() {
        entry_list.push_str(&format!(
            "  {}. {} = {} ({}) [{}] {}\n",
            i + 1,
            e.name,
            e.value,
            e.n6_expression,
            e.domain,
            e.source_bt.as_deref().unwrap_or("no BT"),
        ));
    }

    format!(
        r#"Expand the Math Atlas with these {count} new entries:

{entries}
Steps:
1. Read .shared/math_atlas.json
2. For each entry above, verify it doesn't already exist (check by name AND value)
3. Add missing entries in standard atlas format
4. Run: python3 .shared/scan_math_atlas.py --save --summary
5. Report: added N, skipped M (duplicates), total now X

Atlas entry format:
{{
  "name": "...",
  "value": ...,
  "n6_expr": "...",
  "domain": "...",
  "source": "BT-XXX",
  "grade": "EXACT"
}}"#,
        count = entries.len(),
        entries = entry_list,
    )
}

/// Format an ASCII atlas coverage summary.
pub fn format_atlas_coverage() -> String {
    let state = assess_atlas_state();
    let duplicates_total: usize = state.duplicate_values.iter().map(|(_, v)| v.len()).sum();

    let mut lines = Vec::new();
    lines.push("┌───────────────────────────────────────────────────────────┐".to_string());
    lines.push("│           Math Atlas Coverage Summary                    │".to_string());
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push(format!(
        "│  Total constants:  {:>5}                                 │",
        state.total_constants,
    ));
    lines.push(format!(
        "│  Domains covered:  {:>5}/{}                               │",
        state.domains_covered, 32,
    ));
    lines.push(format!(
        "│  Missing from BTs: {:>5}                                 │",
        state.constants_without_bt,
    ));
    lines.push(format!(
        "│  Duplicate values: {:>5} entries across {} groups          │",
        duplicates_total,
        state.duplicate_values.len(),
    ));
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push("│  Core n=6 vocabulary:                                    │".to_string());
    for (expr, val) in CORE_N6_VALUES.iter().take(SIGMA) {
        lines.push(format!("│    {:16} = {:>10.3}                         │", expr, val));
    }
    lines.push("└───────────────────────────────────────────────────────────┘".to_string());

    lines.join("\n")
}

// ── internal helpers ─────────────────────────────────────────────────

fn estimate_missing_constants() -> usize {
    // Each BT introduces ~3-5 verifiable constants
    // BTs 105-127 = 23 BTs, ~4 constants each = ~92
    // Subtract already-added ones: ~50 remaining
    let recent_bts = 23;               // BT-105 to BT-127
    let constants_per_bt = TAU;        // τ=4 on average
    let already_added_pct = 45;        // ~45% already in atlas
    (recent_bts * constants_per_bt * (100 - already_added_pct)) / 100
}

// ── tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_atlas_state() {
        let state = assess_atlas_state();
        assert_eq!(state.total_constants, KNOWN_ATLAS_CONSTANTS);
        assert!(state.domains_covered > 0);
        assert!(state.constants_without_bt > 0);
        assert!(!state.duplicate_values.is_empty());
    }

    #[test]
    fn test_find_missing_constants() {
        let missing = find_missing_constants();
        // Should have entries from recent BTs
        assert!(!missing.is_empty());
        // Each should have a source BT
        for entry in &missing {
            assert!(entry.source_bt.is_some());
            assert!(!entry.name.is_empty());
            assert!(!entry.n6_expression.is_empty());
        }
    }

    #[test]
    fn test_find_duplicates() {
        let dups = find_duplicates();
        // n=6 should be the most duplicated value
        let n6_group = dups.iter().find(|(v, _)| (*v - 6.0).abs() < 0.001);
        assert!(n6_group.is_some());
        let (_, names) = n6_group.unwrap();
        assert!(names.len() >= N); // at least n=6 entries share value 6
    }

    #[test]
    fn test_core_n6_values_consistent() {
        // Verify core identity: σ·φ = n·τ
        let sigma = CORE_N6_VALUES.iter().find(|(n, _)| *n == "σ").unwrap().1;
        let phi = CORE_N6_VALUES.iter().find(|(n, _)| *n == "φ").unwrap().1;
        let n = CORE_N6_VALUES.iter().find(|(n, _)| *n == "n").unwrap().1;
        let tau = CORE_N6_VALUES.iter().find(|(n, _)| *n == "τ").unwrap().1;
        assert!((sigma * phi - n * tau).abs() < 0.001, "σ·φ must equal n·τ");
    }

    #[test]
    fn test_generate_atlas_expansion_prompt() {
        let entries = vec![AtlasEntry {
            name: "test constant".to_string(),
            value: 6.0,
            n6_expression: "n=6".to_string(),
            domain: "test".to_string(),
            source_bt: Some("BT-999".to_string()),
        }];
        let prompt = generate_atlas_expansion_prompt(&entries);
        assert!(prompt.contains("test constant"));
        assert!(prompt.contains("BT-999"));
        assert!(prompt.contains("math_atlas.json"));
    }
}
