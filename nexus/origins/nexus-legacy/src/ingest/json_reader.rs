/// JSON data reader — lightweight, no serde_json dependency for reading simple numeric arrays.
///
/// Extracts numeric values from JSON content. Handles:
///   - Arrays of numbers: [1.0, 2.0, 3.0]
///   - Objects with numeric values: {"a": 1.0, "b": 2.0}
///   - Nested structures (flattened)

/// Read a JSON file and extract all numeric values found.
pub fn read_json_values(path: &str) -> Result<Vec<f64>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    parse_json_values(&content)
}

/// Parse numeric values from a JSON string.
///
/// Scans the JSON text for number literals and returns them all.
/// This is a lightweight approach that avoids a full JSON parser.
pub fn parse_json_values(content: &str) -> Result<Vec<f64>, String> {
    let mut values = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        let ch = chars[i];

        // Skip strings
        if ch == '"' {
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1; // skip escaped char
                }
                i += 1;
            }
            i += 1; // skip closing quote
            continue;
        }

        // Try to parse a number
        if ch == '-' || ch.is_ascii_digit() {
            let start = i;
            // Check we're not inside a key by looking at preceding non-whitespace
            if ch == '-' {
                i += 1;
            }
            while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                i += 1;
            }
            // Handle scientific notation
            if i < len && (chars[i] == 'e' || chars[i] == 'E') {
                i += 1;
                if i < len && (chars[i] == '+' || chars[i] == '-') {
                    i += 1;
                }
                while i < len && chars[i].is_ascii_digit() {
                    i += 1;
                }
            }
            let num_str: String = chars[start..i].iter().collect();
            if let Ok(val) = num_str.parse::<f64>() {
                values.push(val);
            }
            continue;
        }

        i += 1;
    }

    Ok(values)
}

/// Extract key-value pairs from a simple flat JSON object.
///
/// Handles: {"key1": 1.0, "key2": 2.0}
/// Ignores nested objects/arrays.
pub fn parse_json_kv(content: &str) -> Result<Vec<(String, f64)>, String> {
    let mut pairs = Vec::new();
    let chars: Vec<char> = content.chars().collect();
    let len = chars.len();
    let mut i = 0;

    while i < len {
        // Find a key (string in quotes followed by colon)
        if chars[i] == '"' {
            let key_start = i + 1;
            i += 1;
            while i < len && chars[i] != '"' {
                if chars[i] == '\\' {
                    i += 1;
                }
                i += 1;
            }
            let key_end = i;
            i += 1; // skip closing quote

            // Skip whitespace and colon
            while i < len && (chars[i].is_whitespace() || chars[i] == ':') {
                if chars[i] == ':' {
                    i += 1;
                    break;
                }
                i += 1;
            }
            while i < len && chars[i].is_whitespace() {
                i += 1;
            }

            // Try to read a number
            if i < len && (chars[i] == '-' || chars[i].is_ascii_digit()) {
                let num_start = i;
                if chars[i] == '-' {
                    i += 1;
                }
                while i < len && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                if i < len && (chars[i] == 'e' || chars[i] == 'E') {
                    i += 1;
                    if i < len && (chars[i] == '+' || chars[i] == '-') {
                        i += 1;
                    }
                    while i < len && chars[i].is_ascii_digit() {
                        i += 1;
                    }
                }
                let num_str: String = chars[num_start..i].iter().collect();
                if let Ok(val) = num_str.parse::<f64>() {
                    let key: String = chars[key_start..key_end].iter().collect();
                    pairs.push((key, val));
                }
            }
            continue;
        }
        i += 1;
    }

    Ok(pairs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_array() {
        let json = "[6.0, 12.0, 24.0, 4.0]";
        let vals = parse_json_values(json).unwrap();
        assert_eq!(vals, vec![6.0, 12.0, 24.0, 4.0]);
    }

    #[test]
    fn test_parse_object_values() {
        let json = r#"{"sigma": 12.0, "phi": 2.0, "tau": 4.0}"#;
        let vals = parse_json_values(json).unwrap();
        assert_eq!(vals, vec![12.0, 2.0, 4.0]);
    }

    #[test]
    fn test_parse_negative_and_scientific() {
        let json = "[-1.5, 2e3, 3.14E-2]";
        let vals = parse_json_values(json).unwrap();
        assert_eq!(vals.len(), 3);
        assert!((vals[0] - (-1.5)).abs() < 1e-10);
        assert!((vals[1] - 2000.0).abs() < 1e-10);
        assert!((vals[2] - 0.0314).abs() < 1e-10);
    }

    #[test]
    fn test_parse_json_kv() {
        let json = r#"{"sigma": 12.0, "phi": 2.0, "name": "test"}"#;
        let pairs = parse_json_kv(json).unwrap();
        assert_eq!(pairs.len(), 2);
        assert_eq!(pairs[0].0, "sigma");
        assert!((pairs[0].1 - 12.0).abs() < 1e-10);
        assert_eq!(pairs[1].0, "phi");
        assert!((pairs[1].1 - 2.0).abs() < 1e-10);
    }

    #[test]
    fn test_parse_empty() {
        let vals = parse_json_values("{}").unwrap();
        assert!(vals.is_empty());
    }
}
