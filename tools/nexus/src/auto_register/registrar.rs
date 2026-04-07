use serde::{Deserialize, Serialize};

use super::classifier::{ClassifiedDiscovery, DiscoveryType};

/// Result of registering a discovery to one or more backends.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RegistrationResult {
    pub discovery_id: String,
    pub registered_to: Vec<String>,
    pub success: bool,
    pub details: String,
}

/// Register a classified discovery to the appropriate storage backends.
///
/// This is a planning-only function: it determines *where* a discovery should
/// be stored and returns the list of target backends. Actual file I/O is
/// handled downstream (sync_trigger / CLI).
pub fn register(discovery: &ClassifiedDiscovery) -> RegistrationResult {
    match &discovery.discovery_type {
        DiscoveryType::Constant { .. } => register_to_atlas(discovery),
        DiscoveryType::Formula { .. } => register_to_atlas(discovery),
        DiscoveryType::Pattern { .. } => register_to_graph(discovery),
        DiscoveryType::Law { .. } => register_to_hypotheses(discovery),
        DiscoveryType::BtCandidate { .. } => register_to_bt(discovery),
        DiscoveryType::LensCandidate { .. } => register_to_lens_forge(discovery),
    }
}

fn register_to_atlas(discovery: &ClassifiedDiscovery) -> RegistrationResult {
    let mut targets = vec!["math_atlas".to_string()];
    // Constants and formulas also go into the graph
    targets.push("graph".to_string());

    let details = match &discovery.discovery_type {
        DiscoveryType::Constant { value, name, formula } => {
            format!(
                "Constant '{}' = {} (formula: {})",
                name,
                value,
                formula.as_deref().unwrap_or("none")
            )
        }
        DiscoveryType::Formula { expression, variables } => {
            format!("Formula: {} (vars: {})", expression, variables.join(", "))
        }
        _ => "atlas entry".to_string(),
    };

    RegistrationResult {
        discovery_id: discovery.id.clone(),
        registered_to: targets,
        success: true,
        details,
    }
}

fn register_to_graph(discovery: &ClassifiedDiscovery) -> RegistrationResult {
    let details = match &discovery.discovery_type {
        DiscoveryType::Pattern { pattern_type, description } => {
            format!("Pattern [{}]: {}", pattern_type, &description[..description.len().min(80)])
        }
        _ => "graph node".to_string(),
    };

    RegistrationResult {
        discovery_id: discovery.id.clone(),
        registered_to: vec!["graph".to_string()],
        success: true,
        details,
    }
}

fn register_to_hypotheses(discovery: &ClassifiedDiscovery) -> RegistrationResult {
    let mut targets = vec!["hypotheses".to_string(), "graph".to_string()];

    if let DiscoveryType::Law { domains, .. } = &discovery.discovery_type {
        if domains.len() >= 3 {
            // Multi-domain laws are also BT candidates
            targets.push("bt_candidates".to_string());
        }
    }

    let details = match &discovery.discovery_type {
        DiscoveryType::Law { statement, domains } => {
            format!(
                "Law across {} domains: {}",
                domains.len(),
                &statement[..statement.len().min(80)]
            )
        }
        _ => "hypothesis entry".to_string(),
    };

    RegistrationResult {
        discovery_id: discovery.id.clone(),
        registered_to: targets,
        success: true,
        details,
    }
}

fn register_to_bt(discovery: &ClassifiedDiscovery) -> RegistrationResult {
    let details = match &discovery.discovery_type {
        DiscoveryType::BtCandidate { title, domains, evidence } => {
            format!(
                "BT candidate: {} ({} domains, {} evidence)",
                &title[..title.len().min(60)],
                domains.len(),
                evidence.len()
            )
        }
        _ => "BT entry".to_string(),
    };

    RegistrationResult {
        discovery_id: discovery.id.clone(),
        registered_to: vec![
            "bt_list".to_string(),
            "graph".to_string(),
            "hypotheses".to_string(),
        ],
        success: true,
        details,
    }
}

fn register_to_lens_forge(discovery: &ClassifiedDiscovery) -> RegistrationResult {
    let details = match &discovery.discovery_type {
        DiscoveryType::LensCandidate { name, .. } => {
            format!("Lens candidate: {}", &name[..name.len().min(60)])
        }
        _ => "lens forge entry".to_string(),
    };

    RegistrationResult {
        discovery_id: discovery.id.clone(),
        registered_to: vec!["lens_forge".to_string(), "graph".to_string()],
        success: true,
        details,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::auto_register::classifier;

    #[test]
    fn test_register_constant() {
        let disc = classifier::classify("Found sigma = 12.0 in physics", 1.0, "test");
        let result = register(&disc);
        assert!(result.success);
        assert!(result.registered_to.contains(&"math_atlas".to_string()));
    }

    #[test]
    fn test_register_bt_candidate() {
        let disc = classifier::classify(
            "BT-999 cross-domain physics energy breakthrough",
            0.9,
            "test",
        );
        let result = register(&disc);
        assert!(result.success);
        assert!(result.registered_to.contains(&"bt_list".to_string()));
        assert!(result.registered_to.contains(&"graph".to_string()));
    }

    #[test]
    fn test_register_pattern() {
        let disc = classifier::classify("Some repeating cycle observation", 0.5, "test");
        let result = register(&disc);
        assert!(result.success);
        assert!(result.registered_to.contains(&"graph".to_string()));
    }
}
