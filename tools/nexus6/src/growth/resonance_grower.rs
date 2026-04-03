//! Resonance Grower — tracks and expands cross-domain resonance connections.
//!
//! Identifies domain pairs that share n=6 constants, finds unexplored pairs,
//! and generates search plans for new resonance discoveries.

use std::collections::HashSet;

// ── n=6 constants ────────────────────────────────────────────────────
const N: usize = 6;               // the perfect number
const SIGMA: usize = 12;          // σ(6) = sum of divisors
const PHI: usize = 2;             // φ(6) = Euler totient
const TAU: usize = 4;             // τ(6) = number of divisors
const J2: usize = 24;             // J₂(6) = Jordan totient
const _SOPFR: usize = 5;           // sopfr(6) = 2+3
const _SIGMA_MINUS_PHI: usize = 10; // σ-φ = 10
const _SIGMA_MINUS_TAU: usize = 8; // σ-τ = 8

/// Total documented domains.
const TOTAL_DOMAINS: usize = 32;

/// All 32 domains for pairwise analysis.
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

/// Constants that appear most frequently across domains.
const TOP_RESONANT_CONSTANTS: &[(&str, &str, usize)] = &[
    ("σ-τ=8",      "8",    16),  // LoRA, MoE, KV, FlashAttn, codebooks, qubits...
    ("σ=12",       "12",   14),  // semitones, joints, HBM, transformer heads...
    ("J₂=24",      "24",   12),  // fps, bits, atoms, Leech dim, expert capacity...
    ("1/(σ-φ)=0.1","0.1",  8),   // WD, DPO, reconnection rate, regularization...
    ("τ=4",        "4",    8),   // codons, quadrupeds, ACID, HBM stack...
    ("n=6",        "6",    10),  // DOF, carbon Z, greenhouse gases, plastics...
    ("σ²=144",     "144",  5),   // SMs, solar cells, GPU cores...
    ("sopfr=5",    "5",    6),   // fingers, SOLID, REST verbs, baryons...
    ("φ=2",        "2",    7),   // bilateral, FP precision ratio, Carmichael...
    ("σ·τ=48",     "48",   5),   // kHz, gate pitch, volts...
];

/// Known resonance connections (domain pairs already analyzed in BTs).
const KNOWN_RESONANCE_PAIRS: &[(&str, &str)] = &[
    ("ai-efficiency", "chip-architecture"),       // BT-59
    ("ai-efficiency", "energy-architecture"),      // BT-36
    ("chip-architecture", "energy-architecture"),  // BT-36
    ("battery-architecture", "chip-architecture"), // BT-84
    ("battery-architecture", "ai-efficiency"),     // BT-84
    ("display-audio", "pure-mathematics"),         // BT-48
    ("biology", "energy-architecture"),            // BT-27, BT-101
    ("fusion", "cosmology-particle"),              // BT-97-100
    ("software-design", "cryptography"),           // BT-113-114
    ("software-design", "network-protocol"),       // BT-115
    ("robotics", "ai-efficiency"),                 // BT-123
    ("environmental-protection", "biology"),       // BT-118
    ("environmental-protection", "energy-architecture"), // BT-118
    ("solar-architecture", "energy-architecture"), // BT-30, BT-63
    ("material-synthesis", "chip-architecture"),   // BT-93
    ("fusion", "biology"),                         // BT-101, BT-103
    ("blockchain", "cryptography"),                // BT-53
];

/// Snapshot of cross-domain resonance state.
#[derive(Debug, Clone)]
pub struct ResonanceState {
    /// Number of known resonance connections
    pub known_resonances: usize,
    /// Domains participating in at least one resonance
    pub domains_in_resonance_map: usize,
    /// Constants ranked by how many domains they appear in
    pub strongest_constants: Vec<(String, usize)>,
    /// Domain pairs not yet analyzed for resonance
    pub unexplored_pairs: Vec<(String, String)>,
}

/// A planned resonance search between two domains.
#[derive(Debug, Clone)]
pub struct ResonanceSearch {
    /// First domain
    pub domain_a: String,
    /// Second domain
    pub domain_b: String,
    /// n=6 constants to check in both domains
    pub constants_to_check: Vec<String>,
    /// Ready-to-use Claude CLI prompt
    pub claude_prompt: String,
}

/// Growth plan for resonance expansion.
#[derive(Debug, Clone)]
pub struct ResonancePlan {
    /// Prioritized resonance searches
    pub searches: Vec<ResonanceSearch>,
}

/// Assess the current state of cross-domain resonance coverage.
pub fn assess_resonance_state() -> ResonanceState {
    let unexplored = find_unexplored_pairs();

    // Count unique domains in resonance map
    let mut domains_seen = HashSet::new();
    for (a, b) in KNOWN_RESONANCE_PAIRS {
        domains_seen.insert(*a);
        domains_seen.insert(*b);
    }

    let strongest: Vec<(String, usize)> = TOP_RESONANT_CONSTANTS
        .iter()
        .map(|(name, _, count)| (name.to_string(), *count))
        .collect();

    ResonanceState {
        known_resonances: KNOWN_RESONANCE_PAIRS.len(),
        domains_in_resonance_map: domains_seen.len(),
        strongest_constants: strongest,
        unexplored_pairs: unexplored,
    }
}

/// Find all domain pairs that haven't been analyzed for resonance yet.
pub fn find_unexplored_pairs() -> Vec<(String, String)> {
    let known: HashSet<(&str, &str)> = KNOWN_RESONANCE_PAIRS
        .iter()
        .flat_map(|(a, b)| vec![(*a, *b), (*b, *a)])
        .collect();

    let mut pairs = Vec::new();
    for i in 0..ALL_DOMAINS.len() {
        for j in (i + 1)..ALL_DOMAINS.len() {
            let a = ALL_DOMAINS[i];
            let b = ALL_DOMAINS[j];
            if !known.contains(&(a, b)) && !known.contains(&(b, a)) {
                pairs.push((a.to_string(), b.to_string()));
            }
        }
    }
    pairs
}

/// Plan resonance searches, prioritizing high-value pairs.
///
/// `max` caps the number of searches (capped to J₂=24).
pub fn plan_resonance_search(max: usize) -> ResonancePlan {
    let max = max.min(J2); // cap at J₂=24
    let unexplored = find_unexplored_pairs();

    // Score pairs by how many high-resonance domains they contain
    let high_resonance_domains: HashSet<&str> = [
        "ai-efficiency", "chip-architecture", "energy-architecture",
        "battery-architecture", "fusion", "biology", "robotics",
        "software-design", "material-synthesis",
    ]
    .iter()
    .copied()
    .collect();

    let mut scored: Vec<(usize, String, String)> = unexplored
        .into_iter()
        .map(|(a, b)| {
            let score = (if high_resonance_domains.contains(a.as_str()) { SIGMA } else { 0 })
                + (if high_resonance_domains.contains(b.as_str()) { SIGMA } else { 0 });
            (score, a, b)
        })
        .collect();

    // Sort by score descending (highest value pairs first)
    scored.sort_by(|a, b| b.0.cmp(&a.0));

    let searches: Vec<ResonanceSearch> = scored
        .into_iter()
        .take(max)
        .map(|(_, a, b)| create_resonance_search(&a, &b))
        .collect();

    ResonancePlan { searches }
}

/// Format an ASCII resonance matrix showing explored vs unexplored pairs.
pub fn format_resonance_matrix() -> String {
    let state = assess_resonance_state();
    let total_pairs = TOTAL_DOMAINS * (TOTAL_DOMAINS - 1) / PHI; // C(32,2) = 496
    let explored_pct = (state.known_resonances as f64 / total_pairs as f64) * 100.0;

    let mut lines = Vec::new();
    lines.push("┌───────────────────────────────────────────────────────────┐".to_string());
    lines.push("│        Cross-Domain Resonance Coverage                   │".to_string());
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push(format!(
        "│  Known pairs:     {:>4}/{:<4}  ({:.1}% explored)            │",
        state.known_resonances, total_pairs, explored_pct,
    ));
    lines.push(format!(
        "│  Domains active:  {:>4}/{:<4}                               │",
        state.domains_in_resonance_map, TOTAL_DOMAINS,
    ));
    lines.push(format!(
        "│  Unexplored:      {:>4}                                    │",
        state.unexplored_pairs.len(),
    ));
    lines.push("├───────────────────────────────────────────────────────────┤".to_string());
    lines.push("│  Top resonant constants:                                 │".to_string());
    for (name, count) in state.strongest_constants.iter().take(N) {
        let bar: String = "█".repeat(*count);
        lines.push(format!("│    {:16} {:>2} domains  {}│", name, count, bar));
    }
    lines.push("└───────────────────────────────────────────────────────────┘".to_string());

    lines.join("\n")
}

// ── internal helpers ─────────────────────────────────────────────────

fn create_resonance_search(domain_a: &str, domain_b: &str) -> ResonanceSearch {
    // Select constants most likely to resonate based on domain types
    let constants = select_constants_for_pair(domain_a, domain_b);

    let claude_prompt = format!(
        r#"Search for n=6 cross-domain resonance between '{a}' and '{b}':

Constants to check: {constants}

Steps:
1. List the top {sigma} key parameters/constants in {a}
2. List the top {sigma} key parameters/constants in {b}
3. For each n=6 expression (σ,φ,τ,J₂,sopfr and combinations), check if it matches
   any parameter in BOTH domains simultaneously
4. Compute error % for each potential match
5. Grade: EXACT (<1%), CLOSE (1-5%), WEAK (5-20%)
6. If {tau}+ EXACT matches found, draft a cross-domain BT

Output: Table of matches + proposed BT (if qualifying)
| Parameter_A | Value_A | Parameter_B | Value_B | n=6 expr | Error% | Grade |"#,
        a = domain_a,
        b = domain_b,
        constants = constants.join(", "),
        sigma = SIGMA,
        tau = TAU,
    );

    ResonanceSearch {
        domain_a: domain_a.to_string(),
        domain_b: domain_b.to_string(),
        constants_to_check: constants,
        claude_prompt,
    }
}

fn select_constants_for_pair(domain_a: &str, domain_b: &str) -> Vec<String> {
    // Universal constants always checked
    let mut constants = vec![
        "σ-τ=8".to_string(),
        "σ=12".to_string(),
        "J₂=24".to_string(),
        "1/(σ-φ)=0.1".to_string(),
        "n=6".to_string(),
        "τ=4".to_string(),
    ];

    // Domain-specific additions
    let has_physics = ["fusion", "plasma-physics", "cosmology-particle", "superconductor"]
        .contains(&domain_a) || ["fusion", "plasma-physics", "cosmology-particle", "superconductor"]
        .contains(&domain_b);
    let has_computing = ["ai-efficiency", "chip-architecture", "quantum-computing"]
        .contains(&domain_a) || ["ai-efficiency", "chip-architecture", "quantum-computing"]
        .contains(&domain_b);
    let has_bio = ["biology", "environmental-protection"]
        .contains(&domain_a) || ["biology", "environmental-protection"]
        .contains(&domain_b);

    if has_physics {
        constants.push("sopfr=5 (baryons)".to_string());
        constants.push("σ+sopfr=17 (MK)".to_string());
    }
    if has_computing {
        constants.push("σ²=144 (SMs)".to_string());
        constants.push("2^σ=4096 (context)".to_string());
    }
    if has_bio {
        constants.push("Z=6 (carbon)".to_string());
        constants.push("64=2^n (codons)".to_string());
    }

    constants
}

// ── tests ────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assess_resonance_state() {
        let state = assess_resonance_state();
        assert_eq!(state.known_resonances, KNOWN_RESONANCE_PAIRS.len());
        assert!(state.domains_in_resonance_map > 0);
        assert!(!state.strongest_constants.is_empty());
        assert!(!state.unexplored_pairs.is_empty());
        // Should have many more unexplored than explored
        let total = TOTAL_DOMAINS * (TOTAL_DOMAINS - 1) / PHI;
        assert!(state.unexplored_pairs.len() > total / PHI);
    }

    #[test]
    fn test_find_unexplored_pairs_no_duplicates() {
        let pairs = find_unexplored_pairs();
        let mut seen = HashSet::new();
        for (a, b) in &pairs {
            // No pair should appear twice (in either order)
            let key = if a < b {
                (a.clone(), b.clone())
            } else {
                (b.clone(), a.clone())
            };
            assert!(seen.insert(key), "Duplicate pair found: {} - {}", a, b);
        }
    }

    #[test]
    fn test_plan_resonance_respects_max() {
        let plan = plan_resonance_search(TAU); // max τ=4
        assert!(plan.searches.len() <= TAU);
        for s in &plan.searches {
            assert!(!s.domain_a.is_empty());
            assert!(!s.domain_b.is_empty());
            assert!(!s.constants_to_check.is_empty());
            assert!(!s.claude_prompt.is_empty());
        }
    }

    #[test]
    fn test_plan_prioritizes_high_resonance_domains() {
        let plan = plan_resonance_search(SIGMA); // max σ=12
        // First search should involve at least one high-resonance domain
        if !plan.searches.is_empty() {
            let first = &plan.searches[0];
            let high = ["ai-efficiency", "chip-architecture", "energy-architecture",
                        "battery-architecture", "fusion", "biology", "robotics",
                        "software-design", "material-synthesis"];
            let has_high = high.contains(&first.domain_a.as_str())
                || high.contains(&first.domain_b.as_str());
            assert!(has_high, "First search should involve a high-resonance domain");
        }
    }

    #[test]
    fn test_format_resonance_matrix() {
        let matrix = format_resonance_matrix();
        assert!(matrix.contains("Cross-Domain Resonance"));
        assert!(matrix.contains("Known pairs"));
        assert!(matrix.contains("Top resonant constants"));
    }
}
