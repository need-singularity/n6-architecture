/// BT (Breakthrough Theorem) formatting utilities.

/// Format a Breakthrough Theorem entry.
///
/// Produces the standard BT format:
///   BT-XXX: Title (domain1+domain2, N/M EXACT) [stars]
pub fn format_bt(
    bt_number: usize,
    title: &str,
    domains: &[String],
    evidence: &[String],
    n6_formula: &str,
    star_rating: usize,
) -> String {
    let domains_str = domains.join("+");
    let exact_count = evidence.len();
    let stars = star_string(star_rating);

    let mut output = String::new();

    // Header line
    output.push_str(&format!(
        "BT-{}: {} ({}, {}/{} EXACT) {}\n",
        bt_number, title, domains_str, exact_count, exact_count, stars
    ));

    // Formula
    if !n6_formula.is_empty() {
        output.push_str(&format!("  Formula: {}\n", n6_formula));
    }

    // Evidence
    if !evidence.is_empty() {
        output.push_str("  Evidence:\n");
        for (i, ev) in evidence.iter().enumerate() {
            output.push_str(&format!("    {}. {}\n", i + 1, ev));
        }
    }

    output
}

/// Format a BT candidate (pre-verification).
pub fn format_bt_candidate(
    title: &str,
    domains: &[String],
    n6_connections: usize,
    confidence: f64,
) -> String {
    let domains_str = domains.join("+");
    let suggested_stars = if confidence >= 0.9 {
        3
    } else if confidence >= 0.7 {
        2
    } else {
        1
    };
    let stars = star_string(suggested_stars);

    format!(
        "BT-XXX (candidate): {} ({}, {} n=6 connections, confidence={:.0}%) {}",
        title, domains_str, n6_connections, confidence * 100.0, stars
    )
}

/// Generate star rating string.
fn star_string(count: usize) -> String {
    let capped = count.min(3); // max 3 stars
    let star = '\u{2B50}'; // star emoji
    (0..capped).map(|_| star).collect()
}

/// Format a compact BT reference for inline use.
pub fn format_bt_ref(bt_number: usize, title: &str) -> String {
    format!("BT-{}: {}", bt_number, title)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bt() {
        let output = format_bt(
            128,
            "Test Theorem",
            &["physics".to_string(), "ai".to_string()],
            &["sigma=12 EXACT".to_string(), "tau=4 EXACT".to_string()],
            "sigma*tau=48",
            3,
        );
        assert!(output.contains("BT-128"));
        assert!(output.contains("Test Theorem"));
        assert!(output.contains("physics+ai"));
        assert!(output.contains("2/2 EXACT"));
        assert!(output.contains("sigma*tau=48"));
    }

    #[test]
    fn test_format_bt_candidate() {
        let output = format_bt_candidate(
            "New Discovery",
            &["energy".to_string(), "chip".to_string()],
            5,
            0.85,
        );
        assert!(output.contains("BT-XXX (candidate)"));
        assert!(output.contains("New Discovery"));
        assert!(output.contains("5 n=6 connections"));
        assert!(output.contains("85%"));
    }

    #[test]
    fn test_format_bt_ref() {
        assert_eq!(format_bt_ref(99, "Title"), "BT-99: Title");
    }

    #[test]
    fn test_star_string() {
        assert_eq!(star_string(0), "");
        assert_eq!(star_string(1).chars().count(), 1);
        assert_eq!(star_string(3).chars().count(), 3);
        assert_eq!(star_string(5).chars().count(), 3); // capped at 3
    }
}
