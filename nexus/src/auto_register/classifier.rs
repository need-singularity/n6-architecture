use serde::{Deserialize, Serialize};

use crate::verifier::n6_check;

/// The type of a discovered entity.
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub enum DiscoveryType {
    Constant {
        value: f64,
        name: String,
        formula: Option<String>,
    },
    Formula {
        expression: String,
        variables: Vec<String>,
    },
    Pattern {
        pattern_type: String,
        description: String,
    },
    Law {
        statement: String,
        domains: Vec<String>,
    },
    BtCandidate {
        title: String,
        domains: Vec<String>,
        evidence: Vec<String>,
    },
    LensCandidate {
        name: String,
        description: String,
    },
}

/// A raw discovery after automatic classification.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ClassifiedDiscovery {
    pub id: String,
    pub discovery_type: DiscoveryType,
    pub confidence: f64,
    pub source_scan: String,
    pub timestamp: String,
    pub n6_score: f64,
}

/// Heuristic keywords that hint at a formula.
const FORMULA_HINTS: &[&str] = &["=", "+", "-", "*", "/", "^", "sigma", "phi", "tau"];

/// Heuristic keywords for patterns.
const PATTERN_HINTS: &[&str] = &["repeats", "periodic", "cycle", "ladder", "chain", "sequence"];

/// Heuristic keywords for laws / multi-domain statements.
const LAW_HINTS: &[&str] = &["universality", "law", "conservation", "invariant", "bridge"];

/// Heuristic keywords for BT candidates.
const BT_HINTS: &[&str] = &["BT-", "breakthrough", "cross-domain", "resonance", "convergence"];

/// Heuristic keywords for lens candidates.
const LENS_HINTS: &[&str] = &["lens", "perspective", "viewpoint", "gap", "blind spot"];

/// Generate a deterministic ID from source + content hash.
fn generate_id(source: &str, raw: &str) -> String {
    let hash = blake3::hash(format!("{}:{}", source, raw).as_bytes());
    let hex = hash.to_hex();
    format!("disc-{}", &hex[..12])
}

/// Get current timestamp as ISO-8601 string.
fn now_timestamp() -> String {
    // Simple monotonic stamp; no chrono dependency.
    let dur = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default();
    format!("{}s", dur.as_secs())
}

/// Try to parse a numeric value from the raw text.
fn try_parse_value(raw: &str) -> Option<f64> {
    // Scan tokens for the first parseable float.
    for token in raw.split_whitespace() {
        let cleaned = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-');
        if let Ok(v) = cleaned.parse::<f64>() {
            if v.is_finite() {
                return Some(v);
            }
        }
    }
    None
}

/// Count keyword hits in a lowercase string.
fn keyword_hits(text: &str, keywords: &[&str]) -> usize {
    let lower = text.to_lowercase();
    keywords.iter().filter(|k| lower.contains(&k.to_lowercase())).count()
}

/// Extract domain names from the raw text (simple heuristic).
fn extract_domains(raw: &str) -> Vec<String> {
    const KNOWN_DOMAINS: &[&str] = &[
        "physics", "chemistry", "biology", "energy", "computing", "ai",
        "semiconductor", "fusion", "quantum", "cosmology", "materials",
        "robotics", "crypto", "network", "audio", "display", "battery",
        "solar", "superconductor", "math", "software", "environment",
    ];
    let lower = raw.to_lowercase();
    KNOWN_DOMAINS
        .iter()
        .filter(|d| lower.contains(**d))
        .map(|d| d.to_string())
        .collect()
}

/// Extract variable names from a formula-like string.
fn extract_variables(raw: &str) -> Vec<String> {
    const VARS: &[&str] = &[
        "sigma", "phi", "tau", "n", "J2", "sopfr", "mu",
    ];
    let lower = raw.to_lowercase();
    VARS.iter()
        .filter(|v| lower.contains(&v.to_lowercase()))
        .map(|v| v.to_string())
        .collect()
}

/// Classify a raw discovery string into a structured `ClassifiedDiscovery`.
///
/// Heuristics:
/// 1. If the text looks like a numeric constant and matches an n=6 value -> `Constant`
/// 2. If it contains formula-like syntax (=, operators, variable names) -> `Formula`
/// 3. If it mentions BT / breakthrough / cross-domain -> `BtCandidate`
/// 4. If it mentions lens / gap / perspective -> `LensCandidate`
/// 5. If it mentions universality / law / conservation with multiple domains -> `Law`
/// 6. Otherwise -> `Pattern`
pub fn classify(raw_discovery: &str, n6_score: f64, source: &str) -> ClassifiedDiscovery {
    let id = generate_id(source, raw_discovery);
    let timestamp = now_timestamp();

    // 1. Try constant match
    if let Some(val) = try_parse_value(raw_discovery) {
        let (const_name, quality) = n6_check::n6_match(val);
        if quality >= 0.8 {
            return ClassifiedDiscovery {
                id,
                discovery_type: DiscoveryType::Constant {
                    value: val,
                    name: const_name.to_string(),
                    formula: None,
                },
                confidence: quality,
                source_scan: source.to_string(),
                timestamp,
                n6_score,
            };
        }
    }

    // 2. BT candidate
    let bt_hits = keyword_hits(raw_discovery, BT_HINTS);
    let domains = extract_domains(raw_discovery);
    if bt_hits >= 1 && domains.len() >= 2 {
        return ClassifiedDiscovery {
            id,
            discovery_type: DiscoveryType::BtCandidate {
                title: raw_discovery.chars().take(120).collect(),
                domains,
                evidence: vec![source.to_string()],
            },
            confidence: (bt_hits as f64 * 0.25).min(1.0),
            source_scan: source.to_string(),
            timestamp,
            n6_score,
        };
    }

    // 3. Lens candidate
    let lens_hits = keyword_hits(raw_discovery, LENS_HINTS);
    if lens_hits >= 1 {
        return ClassifiedDiscovery {
            id,
            discovery_type: DiscoveryType::LensCandidate {
                name: raw_discovery.chars().take(60).collect(),
                description: raw_discovery.to_string(),
            },
            confidence: (lens_hits as f64 * 0.3).min(1.0),
            source_scan: source.to_string(),
            timestamp,
            n6_score,
        };
    }

    // 4. Formula
    let formula_hits = keyword_hits(raw_discovery, FORMULA_HINTS);
    if formula_hits >= 2 {
        let vars = extract_variables(raw_discovery);
        return ClassifiedDiscovery {
            id,
            discovery_type: DiscoveryType::Formula {
                expression: raw_discovery.to_string(),
                variables: vars,
            },
            confidence: (formula_hits as f64 * 0.15).min(1.0),
            source_scan: source.to_string(),
            timestamp,
            n6_score,
        };
    }

    // 5. Law (multi-domain with law keywords)
    let law_hits = keyword_hits(raw_discovery, LAW_HINTS);
    if law_hits >= 1 && domains.len() >= 2 {
        return ClassifiedDiscovery {
            id,
            discovery_type: DiscoveryType::Law {
                statement: raw_discovery.to_string(),
                domains,
            },
            confidence: (law_hits as f64 * 0.2).min(1.0),
            source_scan: source.to_string(),
            timestamp,
            n6_score,
        };
    }

    // 6. Pattern (default)
    let pattern_hits = keyword_hits(raw_discovery, PATTERN_HINTS);
    let ptype = if pattern_hits > 0 {
        "structural"
    } else {
        "unknown"
    };

    ClassifiedDiscovery {
        id,
        discovery_type: DiscoveryType::Pattern {
            pattern_type: ptype.to_string(),
            description: raw_discovery.to_string(),
        },
        confidence: if pattern_hits > 0 {
            (pattern_hits as f64 * 0.2).min(1.0)
        } else {
            0.1
        },
        source_scan: source.to_string(),
        timestamp,
        n6_score,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_constant() {
        let result = classify("Found value 12.0 in physics scan", 1.0, "test");
        match &result.discovery_type {
            DiscoveryType::Constant { value, name, .. } => {
                assert!((value - 12.0).abs() < 1e-9);
                assert_eq!(name, "sigma");
            }
            other => panic!("Expected Constant, got {:?}", other),
        }
        assert!(result.confidence >= 0.8);
    }

    #[test]
    fn test_classify_bt_candidate() {
        let result = classify(
            "BT-128 cross-domain physics energy convergence pattern",
            0.9,
            "evolve",
        );
        match &result.discovery_type {
            DiscoveryType::BtCandidate { domains, .. } => {
                assert!(domains.contains(&"physics".to_string()));
                assert!(domains.contains(&"energy".to_string()));
            }
            other => panic!("Expected BtCandidate, got {:?}", other),
        }
    }

    #[test]
    fn test_classify_formula() {
        let result = classify("sigma * phi = n * tau identity", 1.0, "scan");
        match &result.discovery_type {
            DiscoveryType::Formula { variables, .. } => {
                assert!(variables.contains(&"sigma".to_string()));
                assert!(variables.contains(&"phi".to_string()));
            }
            other => panic!("Expected Formula, got {:?}", other),
        }
    }

    #[test]
    fn test_classify_pattern_default() {
        let result = classify("Some unknown observation about cats", 0.3, "test");
        match &result.discovery_type {
            DiscoveryType::Pattern { pattern_type, .. } => {
                assert_eq!(pattern_type, "unknown");
            }
            other => panic!("Expected Pattern, got {:?}", other),
        }
    }

    #[test]
    fn test_classify_lens_candidate() {
        let result = classify("New lens perspective on blind spot in topology", 0.5, "forge");
        match &result.discovery_type {
            DiscoveryType::LensCandidate { .. } => {}
            other => panic!("Expected LensCandidate, got {:?}", other),
        }
    }
}
