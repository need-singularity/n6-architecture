/// Mutation strategies for hypothesis evolution.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutationStrategy {
    /// Shift a parameter value by an n=6 constant offset.
    ParameterShift,
    /// Transfer a hypothesis pattern to a different domain.
    DomainTransfer,
    /// Combine two hypothesis fragments into a new one.
    Combination,
    /// Invert the hypothesis (negate the core claim).
    Inversion,
}

/// n=6 constants used for parameter mutations.
const N6_SHIFTS: &[(&str, f64)] = &[
    ("sigma", 12.0),
    ("phi", 2.0),
    ("tau", 4.0),
    ("J2", 24.0),
    ("sopfr", 5.0),
    ("sigma-phi", 10.0),
    ("sigma-tau", 8.0),
    ("n", 6.0),
    ("ln(4/3)", 0.287_682_072),
    ("tau^2/sigma", 1.333_333_333),
];

/// Target domains for domain transfer mutations.
const TRANSFER_DOMAINS: &[&str] = &[
    "ai-efficiency",
    "chip-architecture",
    "energy-architecture",
    "battery-architecture",
    "quantum-computing",
    "fusion",
    "superconductor",
    "biology",
    "cosmology-particle",
    "robotics",
    "material-synthesis",
    "software-design",
];

/// Generate mutated hypotheses from a base hypothesis string.
///
/// Applies all four mutation strategies, returning a vec of mutated hypothesis strings.
/// Each strategy generates one or more variants.
pub fn mutate_hypothesis(base: &str) -> Vec<String> {
    let mut results = Vec::new();

    results.extend(parameter_shift(base));
    results.extend(domain_transfer(base));
    results.extend(combination(base));
    results.extend(inversion(base));

    results
}

/// Apply a single mutation strategy.
pub fn mutate_with_strategy(base: &str, strategy: MutationStrategy) -> Vec<String> {
    match strategy {
        MutationStrategy::ParameterShift => parameter_shift(base),
        MutationStrategy::DomainTransfer => domain_transfer(base),
        MutationStrategy::Combination => combination(base),
        MutationStrategy::Inversion => inversion(base),
    }
}

/// Parameter shift: replace numeric-like tokens with n=6 constant expressions.
fn parameter_shift(base: &str) -> Vec<String> {
    let mut results = Vec::new();

    // For each n=6 constant, generate a variant suggesting that constant
    for &(name, value) in N6_SHIFTS {
        results.push(format!(
            "[ParameterShift] {} → scaled by {}={:.4}: {}",
            base, name, value,
            apply_scale_hint(base, name, value),
        ));
    }

    results
}

/// Domain transfer: re-frame the hypothesis for different domains.
fn domain_transfer(base: &str) -> Vec<String> {
    let mut results = Vec::new();

    for domain in TRANSFER_DOMAINS {
        results.push(format!(
            "[DomainTransfer→{}] If '{}' holds, then analogous pattern may exist in {} domain",
            domain, truncate(base, 80), domain,
        ));
    }

    results
}

/// Combination: split hypothesis into halves and recombine with n=6 bridge.
fn combination(base: &str) -> Vec<String> {
    let words: Vec<&str> = base.split_whitespace().collect();
    if words.len() < 4 {
        return vec![format!(
            "[Combination] {} ∧ σ·φ=n·τ bridge → unified hypothesis",
            base,
        )];
    }

    let mid = words.len() / 2;
    let first_half = words[..mid].join(" ");
    let second_half = words[mid..].join(" ");

    vec![
        format!(
            "[Combination:σ·φ=n·τ] '{}' BRIDGED-BY n=6 identity WITH '{}'",
            first_half, second_half,
        ),
        format!(
            "[Combination:J₂=24] '{}' scaled-to J₂=24 dimension WITH '{}'",
            second_half, first_half,
        ),
    ]
}

/// Inversion: negate or flip the core claim.
fn inversion(base: &str) -> Vec<String> {
    vec![
        format!("[Inversion] NOT({}): what if the opposite holds?", base),
        format!(
            "[Inversion:reciprocal] Instead of X, consider 1/X where: {}",
            base,
        ),
    ]
}

/// Create a hint about how a value scales with an n=6 constant.
fn apply_scale_hint(_base: &str, const_name: &str, value: f64) -> String {
    format!(
        "key parameter ×{} ({}={:.4}) may reveal hidden n=6 alignment",
        const_name, const_name, value,
    )
}

/// Truncate a string to max_len characters, appending "..." if truncated.
/// UTF-8 safe: truncates at char boundary, not byte boundary.
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        let mut end = max_len;
        while end > 0 && !s.is_char_boundary(end) {
            end -= 1;
        }
        format!("{}...", &s[..end])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutate_produces_results() {
        let results = mutate_hypothesis("bandgap is 1.34 eV for optimal solar cells");
        assert!(!results.is_empty(), "Should produce at least one mutation");
    }

    #[test]
    fn test_all_strategies_produce_different_results() {
        let base = "transformer head count is 12 for optimal performance";
        let shift = mutate_with_strategy(base, MutationStrategy::ParameterShift);
        let transfer = mutate_with_strategy(base, MutationStrategy::DomainTransfer);
        let combo = mutate_with_strategy(base, MutationStrategy::Combination);
        let inv = mutate_with_strategy(base, MutationStrategy::Inversion);

        assert!(!shift.is_empty());
        assert!(!transfer.is_empty());
        assert!(!combo.is_empty());
        assert!(!inv.is_empty());

        // Each strategy should produce distinct outputs
        assert!(shift[0] != transfer[0]);
        assert!(combo[0] != inv[0]);
    }

    #[test]
    fn test_parameter_shift_count() {
        let results = mutate_with_strategy("test hypothesis", MutationStrategy::ParameterShift);
        // One per n=6 constant
        assert_eq!(results.len(), N6_SHIFTS.len());
    }

    #[test]
    fn test_domain_transfer_count() {
        let results = mutate_with_strategy("test hypothesis", MutationStrategy::DomainTransfer);
        assert_eq!(results.len(), TRANSFER_DOMAINS.len());
    }

    #[test]
    fn test_inversion_count() {
        let results = mutate_with_strategy("test hypothesis", MutationStrategy::Inversion);
        assert_eq!(results.len(), 2);
    }
}
