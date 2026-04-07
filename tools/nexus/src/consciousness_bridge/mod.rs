//! Bridge to consciousness analysis frameworks and Phi metrics.
/// Consciousness Bridge — connects NEXUS-6 to the anima consciousness engine.
/// Reads anima project files (Phi values, consciousness laws) from disk.

use std::fs;
use std::path::{Path, PathBuf};

/// Bridge to the anima consciousness engine.
pub struct ConsciousnessBridge {
    pub anima_path: String,
    pub connected: bool,
}

impl ConsciousnessBridge {
    /// Create a new bridge pointing at the anima project root.
    pub fn new(anima_path: &str) -> Self {
        let connected = Path::new(anima_path).is_dir();
        Self {
            anima_path: anima_path.to_string(),
            connected,
        }
    }

    /// Check whether the anima project directory exists.
    pub fn check_connection(&self) -> bool {
        Path::new(&self.anima_path).is_dir()
    }

    /// Attempt to read the most recent Phi (integrated information) value
    /// from anima result files.  Looks for a `phi_latest.txt` or similar
    /// single-number file under `anima_path/results/`.
    /// Returns None if the file doesn't exist or can't be parsed.
    pub fn get_phi(&self) -> Option<f64> {
        let candidates = [
            "results/phi_latest.txt",
            "results/phi.txt",
            "phi_latest.txt",
            "output/phi.txt",
        ];
        for rel in &candidates {
            let path = PathBuf::from(&self.anima_path).join(rel);
            if let Ok(content) = fs::read_to_string(&path) {
                // Try to parse the first line as f64
                if let Some(line) = content.lines().next() {
                    if let Ok(v) = line.trim().parse::<f64>() {
                        return Some(v);
                    }
                }
            }
        }
        None
    }

    /// Read consciousness laws from `consciousness_laws.json` (or `.txt`).
    /// Returns a list of law descriptions.
    pub fn get_laws(&self) -> Vec<String> {
        let json_path = PathBuf::from(&self.anima_path).join("consciousness_laws.json");
        if let Ok(content) = fs::read_to_string(&json_path) {
            return parse_laws_json(&content);
        }
        let txt_path = PathBuf::from(&self.anima_path).join("consciousness_laws.txt");
        if let Ok(content) = fs::read_to_string(&txt_path) {
            return content
                .lines()
                .map(|l| l.trim().to_string())
                .filter(|l| !l.is_empty())
                .collect();
        }
        Vec::new()
    }

    /// Given the current Phi value, suggest the next experiment direction.
    pub fn suggest_experiment(&self, phi: f64) -> String {
        if phi < 0.5 {
            "Low Phi detected. Suggest: increase integration via cross-domain \
             lens scan (consciousness + topology + causal)."
                .to_string()
        } else if phi < 1.0 {
            "Moderate Phi. Suggest: run OUROBOROS evolution with consciousness \
             seed to amplify integration."
                .to_string()
        } else if phi < 2.0 {
            "High Phi. Suggest: deep scan with quantum_microscope + recursion \
             lenses to find hidden sub-structures."
                .to_string()
        } else {
            format!(
                "Very high Phi ({:.3}). Suggest: full 22-lens scan to map the \
                 complete integration landscape.",
                phi
            )
        }
    }
}

/// Parse a simple JSON array of strings: ["law1", "law2", ...]
/// or an array of objects with a "name"/"description" field.
/// No serde dependency — hand-rolled minimal parser.
fn parse_laws_json(content: &str) -> Vec<String> {
    let trimmed = content.trim();
    // Simple extraction: find all quoted strings after [ ... ]
    let mut laws = Vec::new();
    let mut in_string = false;
    let mut escape = false;
    let mut current = String::new();

    for ch in trimmed.chars() {
        if escape {
            current.push(ch);
            escape = false;
            continue;
        }
        if ch == '\\' && in_string {
            escape = true;
            continue;
        }
        if ch == '"' {
            if in_string {
                // End of string
                if !current.is_empty() {
                    laws.push(current.clone());
                }
                current.clear();
                in_string = false;
            } else {
                in_string = true;
            }
            continue;
        }
        if in_string {
            current.push(ch);
        }
    }

    // Filter out JSON keys (heuristic: keep strings that don't look like keys)
    // If the json is a flat array of strings, all entries are laws.
    // If it's objects, keep only values (every other string after key).
    // Simple heuristic: keep strings longer than 3 chars.
    laws.into_iter()
        .filter(|s| s.len() > 3)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bridge_nonexistent_path() {
        let bridge = ConsciousnessBridge::new("/tmp/nonexistent_anima_9999");
        assert!(!bridge.connected);
        assert!(!bridge.check_connection());
        assert!(bridge.get_phi().is_none());
        assert!(bridge.get_laws().is_empty());
    }

    #[test]
    fn test_suggest_experiment_low_phi() {
        let bridge = ConsciousnessBridge::new("/tmp/fake");
        let suggestion = bridge.suggest_experiment(0.3);
        assert!(suggestion.contains("Low Phi"));
    }

    #[test]
    fn test_suggest_experiment_moderate_phi() {
        let bridge = ConsciousnessBridge::new("/tmp/fake");
        let suggestion = bridge.suggest_experiment(0.8);
        assert!(suggestion.contains("Moderate Phi"));
    }

    #[test]
    fn test_suggest_experiment_high_phi() {
        let bridge = ConsciousnessBridge::new("/tmp/fake");
        let suggestion = bridge.suggest_experiment(1.5);
        assert!(suggestion.contains("High Phi"));
    }

    #[test]
    fn test_suggest_experiment_very_high_phi() {
        let bridge = ConsciousnessBridge::new("/tmp/fake");
        let suggestion = bridge.suggest_experiment(3.0);
        assert!(suggestion.contains("Very high Phi"));
    }

    #[test]
    fn test_parse_laws_json_array() {
        let json = r#"["First law of consciousness", "Second law: integration", "Third law: emergence"]"#;
        let laws = parse_laws_json(json);
        assert_eq!(laws.len(), 3);
        assert_eq!(laws[0], "First law of consciousness");
    }

    #[test]
    fn test_bridge_with_real_dir() {
        // Use /tmp as a real directory to test connection check
        let bridge = ConsciousnessBridge::new("/tmp");
        assert!(bridge.connected);
        assert!(bridge.check_connection());
    }
}
