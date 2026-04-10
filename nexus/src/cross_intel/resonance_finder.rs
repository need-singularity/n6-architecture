/// Resonance finder — detects when the same n=6 constant appears across domains.

use std::collections::HashMap;

/// N=6 core constants for resonance detection.
const N6_CONSTANTS: &[(&str, f64)] = &[
    ("n", 6.0),
    ("sigma", 12.0),
    ("phi", 2.0),
    ("tau", 4.0),
    ("J2", 24.0),
    ("sopfr", 5.0),
    ("sigma-phi", 10.0),
    ("sigma-tau", 8.0),
    ("sigma*tau", 48.0),
    ("sigma^2", 144.0),
    ("J2-tau", 20.0),
];

/// A cross-domain resonance hit.
#[derive(Debug, Clone)]
pub struct ResonanceHit {
    pub constant_name: String,
    pub constant_value: f64,
    pub domains: Vec<String>,
    pub confidence: f64,
}

/// Find values that resonate (match the same n=6 constant) across domains.
///
/// `domain_values`: domain_name -> list of measured values
/// `tolerance`: relative error threshold (e.g., 0.001 for EXACT)
pub fn find_resonances(
    domain_values: &HashMap<String, Vec<f64>>,
    tolerance: f64,
) -> Vec<ResonanceHit> {
    let mut hits: HashMap<String, Vec<String>> = HashMap::new();

    for (domain, values) in domain_values {
        for &v in values {
            for &(name, constant) in N6_CONSTANTS {
                if constant == 0.0 {
                    continue;
                }
                let error = ((v - constant) / constant).abs();
                if error < tolerance {
                    hits.entry(name.to_string())
                        .or_default()
                        .push(domain.clone());
                }
            }
        }
    }

    let mut results: Vec<ResonanceHit> = hits
        .into_iter()
        .filter(|(_, domains)| domains.len() >= 2) // need 2+ domains
        .map(|(name, domains)| {
            let value = N6_CONSTANTS
                .iter()
                .find(|&&(n, _)| n == name)
                .map(|&(_, v)| v)
                .unwrap_or(0.0);
            let confidence = match domains.len() {
                2 => 0.5,
                3..=4 => 0.8,  // 3+ = high confidence per CLAUDE.md
                _ => 1.0,      // 5+ = very high
            };
            ResonanceHit {
                constant_name: name,
                constant_value: value,
                domains,
                confidence,
            }
        })
        .collect();

    results.sort_by(|a, b| {
        b.confidence
            .partial_cmp(&a.confidence)
            .unwrap_or(std::cmp::Ordering::Equal)
    });
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resonance_basic() {
        let mut domains = HashMap::new();
        domains.insert("AI".into(), vec![12.0, 24.0]);
        domains.insert("chip".into(), vec![12.0, 48.0]);
        domains.insert("energy".into(), vec![24.0, 60.0]);

        let hits = find_resonances(&domains, 0.001);
        // sigma=12 in AI+chip, J2=24 in AI+energy
        assert!(hits.len() >= 2);
        let sigma_hit = hits.iter().find(|h| h.constant_name == "sigma");
        assert!(sigma_hit.is_some());
        assert_eq!(sigma_hit.unwrap().domains.len(), 2);
    }

    #[test]
    fn test_resonance_three_domains() {
        let mut domains = HashMap::new();
        domains.insert("AI".into(), vec![20.0]);
        domains.insert("bio".into(), vec![20.0]);
        domains.insert("diffusion".into(), vec![20.0]);

        let hits = find_resonances(&domains, 0.001);
        let j2t = hits.iter().find(|h| h.constant_name == "J2-tau");
        assert!(j2t.is_some());
        assert_eq!(j2t.unwrap().domains.len(), 3);
        assert!(j2t.unwrap().confidence >= 0.8); // 3 domains = high
    }
}
