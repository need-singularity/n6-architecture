use std::collections::HashMap;

/// Parse hypotheses from markdown text.
///
/// Expects `## H-XX-NN: Title` headers followed by `- Key = Value` lines.
/// Strips common units (K, GPa, nm, eV, mV, Hz, W, A, V, T, etc.) from values.
pub fn parse_hypotheses(md: &str) -> Vec<HashMap<String, String>> {
    let mut results: Vec<HashMap<String, String>> = Vec::new();
    let mut current: Option<HashMap<String, String>> = None;

    for line in md.lines() {
        let trimmed = line.trim();

        // Match hypothesis headers: ## H-XX-NN: Title
        if trimmed.starts_with("## H-") && trimmed.contains(':') {
            if let Some(entry) = current.take() {
                results.push(entry);
            }
            let mut entry = HashMap::new();
            // Extract ID and title from "## H-XX-NN: Title"
            let header = trimmed.trim_start_matches('#').trim();
            if let Some(colon_pos) = header.find(':') {
                let id = header[..colon_pos].trim().to_string();
                let title = header[colon_pos + 1..].trim().to_string();
                entry.insert("id".to_string(), id);
                entry.insert("title".to_string(), title);
            }
            current = Some(entry);
        }
        // Match key-value lines: - Key = Value
        else if trimmed.starts_with("- ") && trimmed.contains('=') {
            if let Some(ref mut entry) = current {
                let content = trimmed.trim_start_matches('-').trim();
                if let Some(eq_pos) = content.find('=') {
                    let key = content[..eq_pos].trim().to_string();
                    let raw_value = content[eq_pos + 1..].trim().to_string();
                    let clean_value = strip_units(&raw_value);
                    entry.insert(key, clean_value);
                }
            }
        }
    }

    // Don't forget the last entry
    if let Some(entry) = current {
        results.push(entry);
    }

    results
}

/// Strip common physical units from a value string.
/// Returns just the numeric portion if a unit suffix is found.
fn strip_units(value: &str) -> String {
    let units = [
        "GPa", "MPa", "kPa", "Pa",
        "GHz", "MHz", "kHz", "Hz",
        "MeV", "keV", "eV",
        "mV", "kV", "V",
        "mA", "kA", "A",
        "MW", "kW", "mW", "W",
        "nm", "mm", "cm", "km", "m",
        "kg", "mg", "g",
        "ns", "ms", "us", "s",
        "K", "T",
        "%",
    ];

    let trimmed = value.trim();
    for unit in &units {
        if trimmed.ends_with(unit) {
            let num_part = trimmed[..trimmed.len() - unit.len()].trim();
            // Only strip if what remains looks numeric
            if num_part.parse::<f64>().is_ok() {
                return num_part.to_string();
            }
        }
    }

    trimmed.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_units() {
        assert_eq!(strip_units("300 K"), "300");
        assert_eq!(strip_units("200 GPa"), "200");
        assert_eq!(strip_units("5 nm"), "5");
        assert_eq!(strip_units("1.34 eV"), "1.34");
        assert_eq!(strip_units("42"), "42");
        assert_eq!(strip_units("hello"), "hello");
    }

    #[test]
    fn test_parse_basic() {
        let md = "\
## H-SC-01: MgB2 superconductor
- Tc = 39 K
- Pressure = 0 GPa

## H-SC-02: YBCO cuprate
- Tc = 93 K
- Pressure = 0 GPa
";
        let entries = parse_hypotheses(md);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].get("id").unwrap(), "H-SC-01");
        assert_eq!(entries[0].get("Tc").unwrap(), "39");
        assert_eq!(entries[1].get("Tc").unwrap(), "93");
    }
}
