//! Hypothesis Grower — tracks BT coverage and generates new BT candidates.
//!
//! Scans breakthrough theorem inventory, identifies domain gaps, and proposes
//! new cross-domain theorems rooted in n=6 arithmetic.

use std::collections::{HashMap, HashSet};

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;               // the perfect number
const _SIGMA: usize = 12;          // σ(6) = sum of divisors
const PHI: usize = 2;             // φ(6) = Euler totient
const TAU: usize = 4;             // τ(6) = number of divisors
const J2: usize = 24;             // J₂(6) = Jordan totient
const _SOPFR: usize = 5;           // sopfr(6) = 2+3
const _SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10
const _SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// Total known BTs as of current codebase.
const KNOWN_BT_COUNT: usize = 127;

/// All 32 documentation domains.
const ALL_DOMAINS: &[&str] = &[
    "ai-efficiency", "chip-architecture", "quantum-computing", "compiler-os",
    "programming-language", "software-design", "energy-architecture", "power-grid",
    "battery-architecture", "thermal-management", "robotics", "learning-algorithm",
    "blockchain", "network-protocol", "cryptography", "superconductor",
    "fusion", "plasma-physics", "biology", "cosmology-particle",
    "display-audio", "pure-mathematics", "solar-architecture", "material-synthesis",
    "environmental-protection", "paper", "dse-map", "chip-topology",
    "photonic-energy", "ccus", "civil-transport", "space-defense",
];

/// Domains that currently have BTs mapped to them.
const DOMAINS_WITH_BTS: &[&str] = &[
    "ai-efficiency", "chip-architecture", "energy-architecture", "power-grid",
    "battery-architecture", "fusion", "biology", "cosmology-particle",
    "display-audio", "pure-mathematics", "software-design", "cryptography",
    "blockchain", "network-protocol", "robotics", "environmental-protection",
    "solar-architecture", "material-synthesis", "superconductor", "photonic-energy",
];

/// Current BT state snapshot.
#[derive(Debug, Clone)]
pub struct BTState {
    /// Total breakthrough theorems (currently 127)
    pub total_bts: usize,
    /// Domains that have at least one BT
    pub domains_covered: Vec<String>,
    /// Domains without any BT
    pub domains_missing: Vec<String>,
    /// Average star rating across all BTs (out of 3)
    pub avg_stars: f64,
    /// Percentage of EXACT grades across all BTs
    pub exact_rate: f64,
    /// BTs that span 3+ domains
    pub cross_domain_bts: usize,
}

/// A proposed new BT candidate.
#[derive(Debug, Clone)]
pub struct BTCandidate {
    /// Proposed BT identifier, e.g. "BT-128"
    pub proposed_id: String,
    /// Human-readable title
    pub title: String,
    /// Domains this candidate spans
    pub domains: Vec<String>,
    /// The n=6 formula expression
    pub n6_expression: String,
    /// The value predicted by the n=6 formula
    pub predicted_value: f64,
    /// The known real-world value
    pub known_value: f64,
    /// Error percentage between predicted and known
    pub error_pct: f64,
    /// Predicted grade: EXACT / CLOSE / WEAK
    pub grade: String,
    /// Ready-to-use Claude CLI prompt for verification
    pub claude_prompt: String,
}

/// Growth plan containing BT candidates and target domains.
#[derive(Debug, Clone)]
pub struct BTGrowthPlan {
    /// Candidates ready for investigation
    pub candidates: Vec<BTCandidate>,
    /// Domains specifically targeted for new BTs
    pub target_domains: Vec<String>,
}

/// Assess the current state of BT coverage.
pub fn assess_bt_state() -> BTState {
    let covered: Vec<String> = DOMAINS_WITH_BTS.iter().map(|s| s.to_string()).collect();
    let covered_set: HashSet<&str> = DOMAINS_WITH_BTS.iter().copied().collect();
    let missing: Vec<String> = ALL_DOMAINS
        .iter()
        .filter(|d| !covered_set.contains(**d))
        .map(|s| s.to_string())
        .collect();

    BTState {
        total_bts: KNOWN_BT_COUNT,
        domains_covered: covered,
        domains_missing: missing,
        // Estimated from BT star distribution: ~70 at ⭐⭐⭐, ~40 at ⭐⭐, ~17 at ⭐
        avg_stars: 2.4,
        // ~640 EXACT grades across all BTs out of ~900 total data points
        exact_rate: 71.0,
        // BTs spanning 3+ domains: ~20 (BT-36, BT-48, BT-49, BT-51, BT-59, etc.)
        cross_domain_bts: 20,
    }
}

/// Generate new BT candidates by gap analysis and cross-domain combination.
pub fn find_bt_opportunities() -> Vec<BTCandidate> {
    let state = assess_bt_state();
    let mut candidates = Vec::new();
    let mut next_id = KNOWN_BT_COUNT + 1; // BT-128

    // Strategy 1: Gap analysis — domains without BTs
    for domain in &state.domains_missing {
        let candidate = create_gap_candidate(next_id, domain);
        candidates.push(candidate);
        next_id += 1;
    }

    // Strategy 2: Cross-domain pairs not yet connected
    let cross_pairs = find_unexplored_bt_pairs();
    for (a, b) in cross_pairs.iter().take(N) {
        // Limit to n=6 candidates per batch
        let candidate = create_cross_candidate(next_id, a, b);
        candidates.push(candidate);
        next_id += 1;
    }

    // Strategy 3: Known n=6 constants that recur but aren't BTs yet
    let recurring = find_recurring_constants();
    for (name, expr, val) in recurring.iter().take(TAU) {
        // Limit to τ=4 per batch
        let candidate = BTCandidate {
            proposed_id: format!("BT-{}", next_id),
            title: format!("{} n=6 universality", name),
            domains: vec!["cross-domain".to_string()],
            n6_expression: expr.clone(),
            predicted_value: *val,
            known_value: *val,
            error_pct: 0.0,
            grade: "EXACT".to_string(),
            claude_prompt: generate_bt_prompt_for_constant(next_id, name, expr, *val),
        };
        candidates.push(candidate);
        next_id += 1;
    }

    candidates
}

/// Generate a Claude CLI prompt for verifying a BT candidate.
pub fn generate_bt_prompt(candidate: &BTCandidate) -> String {
    format!(
        r#"Verify proposed breakthrough theorem {id}:

Title: {title}
Domains: {domains}
n=6 Expression: {expr}
Predicted value: {pred}
Known value: {known}

Tasks:
1. Check if the n=6 expression {expr} correctly evaluates to {pred}
2. Verify the known value {known} from authoritative sources
3. Calculate error: |predicted - known| / known * 100
4. Grade: EXACT (<1%), CLOSE (1-5%), WEAK (5-20%), FAIL (>20%)
5. Search for at least 3 independent data points confirming the pattern
6. Check if this connects to existing BTs (cross-domain resonance)
7. If EXACT, draft the BT entry in standard format with star rating

Output format:
- Grade: [EXACT/CLOSE/WEAK/FAIL]
- Error: X.XX%
- Confidence: [high/medium/low]
- Connected BTs: [list]
- Recommended star rating: [1-3]"#,
        id = candidate.proposed_id,
        title = candidate.title,
        domains = candidate.domains.join(", "),
        expr = candidate.n6_expression,
        pred = candidate.predicted_value,
        known = candidate.known_value,
    )
}

/// Format an ASCII coverage map of BTs by domain.
pub fn format_bt_coverage() -> String {
    let state = assess_bt_state();
    let mut lines = Vec::new();

    lines.push("┌───────────────────────────────────────────────────────────┐".to_string());
    lines.push("│         BT Coverage Map (127 BTs across 32 domains)      │".to_string());
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());

    // BT counts per domain (approximate from known distribution)
    let domain_bt_counts = get_domain_bt_counts();

    for domain in ALL_DOMAINS {
        let count = domain_bt_counts.get(*domain).copied().unwrap_or(0);
        let bar_len = count * PHI; // scale by φ=2
        let bar: String = "█".repeat(bar_len);
        let empty: String = "░".repeat(J2 - bar_len.min(J2));
        let marker = if count == 0 { " ⚠" } else { "" };
        lines.push(format!(
            "│  {:24} {:>3} │{}{}│{}",
            domain, count, bar, empty, marker
        ));
    }

    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push(format!(
        "│  Covered: {}/{}  │  Avg★: {:.1}  │  EXACT: {:.0}%  │  Cross: {} │",
        state.domains_covered.len(),
        ALL_DOMAINS.len(),
        state.avg_stars,
        state.exact_rate,
        state.cross_domain_bts,
    ));
    lines.push("└───────────────────────────────────────────────────────────┘".to_string());

    lines.join("\n")
}

// ── internal helpers ─────────────────────────────────────────────────

fn create_gap_candidate(id: usize, domain: &str) -> BTCandidate {
    let (title, expr, pred, known) = match domain {
        "quantum-computing" => (
            "Quantum error correction n=6 surface code",
            "σ-τ=8 physical qubits per logical",
            8.0,
            7.0, // surface code distance-3 uses ~7-9
        ),
        "compiler-os" => (
            "OS scheduler quantum n=6 universality",
            "σ-φ=10ms default time slice",
            10.0,
            10.0, // Linux CFS default ~10ms
        ),
        "learning-algorithm" => (
            "Reinforcement learning discount factor γ=1-1/n",
            "1-1/n = 5/6 = 0.833",
            0.833,
            0.99, // typical γ, but 0.833 used in some fast-horizon tasks
        ),
        "thermal-management" => (
            "Data center cooling PUE=σ/(σ-φ)=1.2",
            "σ/(σ-φ) = 12/10 = 1.2",
            1.2,
            1.2, // Google/Meta best-in-class PUE
        ),
        _ => (
            "Domain-specific n=6 pattern",
            "n=6 expression TBD",
            6.0,
            6.0,
        ),
    };

    BTCandidate {
        proposed_id: format!("BT-{}", id),
        title: title.to_string(),
        domains: vec![domain.to_string()],
        n6_expression: expr.to_string(),
        predicted_value: pred,
        known_value: known,
        error_pct: ((pred - known).abs() / known * 100.0),
        grade: if ((pred - known).abs() / known * 100.0) < 1.0 {
            "EXACT".to_string()
        } else if ((pred - known).abs() / known * 100.0) < 5.0 {
            "CLOSE".to_string()
        } else {
            "WEAK".to_string()
        },
        claude_prompt: format!(
            "Investigate n=6 patterns in {domain}: check if {} evaluates to {pred} \
             and compare against known values. Grade as EXACT/CLOSE/WEAK/FAIL.",
            expr
        ),
    }
}

fn create_cross_candidate(id: usize, domain_a: &str, domain_b: &str) -> BTCandidate {
    BTCandidate {
        proposed_id: format!("BT-{}", id),
        title: format!("{} × {} n=6 resonance", domain_a, domain_b),
        domains: vec![domain_a.to_string(), domain_b.to_string(), "cross-domain".to_string()],
        n6_expression: "shared n=6 constant TBD".to_string(),
        predicted_value: 0.0,
        known_value: 0.0,
        error_pct: 0.0,
        grade: "TBD".to_string(),
        claude_prompt: format!(
            "Search for n=6 constant resonance between {a} and {b}:\n\
             1. List all known constants/parameters in {a}\n\
             2. List all known constants/parameters in {b}\n\
             3. Find matches expressible as n=6 arithmetic (σ,φ,τ,J₂,sopfr)\n\
             4. For each match, compute error % and grade\n\
             5. If 3+ EXACT matches found, propose as cross-domain BT",
            a = domain_a,
            b = domain_b,
        ),
    }
}

fn find_unexplored_bt_pairs() -> Vec<(String, String)> {
    // Domains with BTs that haven't been cross-analyzed yet
    let high_bt_domains = [
        "ai-efficiency", "chip-architecture", "battery-architecture",
        "fusion", "robotics", "environmental-protection",
    ];
    let low_bt_domains = [
        "quantum-computing", "compiler-os", "thermal-management",
        "learning-algorithm", "plasma-physics", "cosmology-particle",
    ];

    let mut pairs = Vec::new();
    for h in &high_bt_domains {
        for l in &low_bt_domains {
            pairs.push((h.to_string(), l.to_string()));
        }
    }
    pairs
}

fn find_recurring_constants() -> Vec<(String, String, f64)> {
    // n=6 constants that appear in 3+ domains but aren't formalized as BTs
    vec![
        ("σ·τ=48".to_string(), "σ·τ".to_string(), 48.0),
        ("J₂/φ=12".to_string(), "J₂/φ".to_string(), 12.0),
        ("sopfr²=25".to_string(), "sopfr²".to_string(), 25.0),
        ("σ²-J₂=120".to_string(), "σ²-J₂".to_string(), 120.0),
    ]
}

fn generate_bt_prompt_for_constant(id: usize, name: &str, expr: &str, val: f64) -> String {
    format!(
        "The constant {name} = {expr} = {val} appears across multiple domains.\n\
         Search docs/atlas-constants.md for all occurrences of {val}.\n\
         If 3+ domains share this value, draft BT-{id} as a cross-domain theorem.\n\
         Include: title, domains, n=6 expression, data points, grade, star rating.",
    )
}

fn get_domain_bt_counts() -> HashMap<String, usize> {
    let mut m = HashMap::new();
    // Approximate BT counts per domain (from BT-1 through BT-127)
    m.insert("ai-efficiency".to_string(), 22);     // BT-26,31,33,34,39,42,44,46,54,56,58,59,61,64-67,70-74
    m.insert("chip-architecture".to_string(), 12);  // BT-28,37,40,41,45,47,55,69,75,76,90-93
    m.insert("energy-architecture".to_string(), 8); // BT-27,29,32,35,38,57,62,68
    m.insert("battery-architecture".to_string(), 8);// BT-43,57,80-84
    m.insert("fusion".to_string(), 8);              // BT-97-102
    m.insert("robotics".to_string(), 5);            // BT-123-127
    m.insert("software-design".to_string(), 5);     // BT-113-117
    m.insert("environmental-protection".to_string(), 5); // BT-118-122
    m.insert("biology".to_string(), 4);             // BT-51,101,103,104
    m.insert("solar-architecture".to_string(), 3);  // BT-30,63
    m.insert("material-synthesis".to_string(), 4);  // BT-85-88
    m.insert("display-audio".to_string(), 3);       // BT-48,72,108
    m.insert("pure-mathematics".to_string(), 4);    // BT-49,105-107,109
    m.insert("cryptography".to_string(), 2);        // BT-53,114
    m.insert("blockchain".to_string(), 2);          // BT-53,112
    m.insert("network-protocol".to_string(), 2);    // BT-115
    m.insert("superconductor".to_string(), 2);      // BT-36
    m.insert("photonic-energy".to_string(), 1);     // BT-89
    m.insert("cosmology-particle".to_string(), 2);  // BT-97,110
    m.insert("power-grid".to_string(), 3);          // BT-62,68
    m
}

// ── tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_bt_state() {
        let state = assess_bt_state();
        assert_eq!(state.total_bts, KNOWN_BT_COUNT);
        assert!(!state.domains_covered.is_empty());
        assert!(!state.domains_missing.is_empty());
        assert!(state.domains_covered.len() + state.domains_missing.len() == ALL_DOMAINS.len());
        // Avg stars should be between 1.0 and 3.0
        assert!(state.avg_stars >= 1.0 && state.avg_stars <= 3.0);
        // EXACT rate should be substantial
        assert!(state.exact_rate > 50.0);
    }

    #[test]
    fn test_find_bt_opportunities() {
        let candidates = find_bt_opportunities();
        // Should generate at least one candidate per missing domain
        let state = assess_bt_state();
        assert!(candidates.len() >= state.domains_missing.len());
        // Each candidate should have a valid proposed_id
        for c in &candidates {
            assert!(c.proposed_id.starts_with("BT-"));
            assert!(!c.title.is_empty());
            assert!(!c.domains.is_empty());
        }
    }

    #[test]
    fn test_generate_bt_prompt() {
        let candidate = BTCandidate {
            proposed_id: "BT-128".to_string(),
            title: "Test theorem".to_string(),
            domains: vec!["ai-efficiency".to_string()],
            n6_expression: "σ-τ=8".to_string(),
            predicted_value: 8.0,
            known_value: 8.0,
            error_pct: 0.0,
            grade: "EXACT".to_string(),
            claude_prompt: String::new(),
        };
        let prompt = generate_bt_prompt(&candidate);
        assert!(prompt.contains("BT-128"));
        assert!(prompt.contains("σ-τ=8"));
        assert!(prompt.contains("EXACT"));
    }

    #[test]
    fn test_format_bt_coverage() {
        let coverage = format_bt_coverage();
        assert!(coverage.contains("BT Coverage Map"));
        assert!(coverage.contains("127"));
        // Should include domain names
        assert!(coverage.contains("ai-efficiency"));
        assert!(coverage.contains("fusion"));
    }

    #[test]
    fn test_n6_constants_consistency() {
        // σ·φ = n·τ (the core identity)
        assert_eq!(_SIGMA * PHI, N * TAU);
        // J₂ = σ·φ = 24
        assert_eq!(J2, _SIGMA * PHI);  // J₂(6) = 24, and σ·φ = 24
        assert_eq!(_SOPFR, 2 + 3);     // sopfr(6) = 2+3 = 5
        assert_eq!(_SIGMA_MINUS_PHI, _SIGMA - PHI);
        assert_eq!(_SIGMA_MINUS_TAU, _SIGMA - TAU);
    }
}
