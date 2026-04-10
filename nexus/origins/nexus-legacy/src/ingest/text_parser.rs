/// Text parser — extract numbers and key-value pairs from unstructured text.

/// Extract all floating-point numbers from text.
///
/// Scans whitespace-separated tokens and attempts to parse each as f64.
/// Handles negative numbers and decimals.
pub fn extract_numbers(text: &str) -> Vec<f64> {
    let mut numbers = Vec::new();

    for token in text.split(|c: char| c.is_whitespace() || c == ',' || c == ';' || c == '|') {
        let cleaned = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-');
        if cleaned.is_empty() {
            continue;
        }
        if let Ok(val) = cleaned.parse::<f64>() {
            numbers.push(val);
        }
    }

    numbers
}

/// Extract key-value pairs from text.
///
/// Recognized patterns:
///   "key = value"
///   "key: value"
///   "key=value"
///   "key:value"
pub fn extract_key_value_pairs(text: &str) -> Vec<(String, f64)> {
    let mut pairs = Vec::new();

    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        // Try "key = value" or "key: value"
        for sep in &["=", ":"] {
            if let Some(pos) = trimmed.find(sep) {
                let key = trimmed[..pos].trim().to_string();
                let val_str = trimmed[pos + sep.len()..].trim();

                // Take just the first token of the value part (in case of units/comments)
                let first_token = val_str.split_whitespace().next().unwrap_or("");
                let cleaned = first_token.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-');

                if !key.is_empty() && !cleaned.is_empty() {
                    if let Ok(val) = cleaned.parse::<f64>() {
                        pairs.push((key, val));
                        break; // found a valid pair on this line
                    }
                }
            }
        }
    }

    pairs
}

/// Extract numbers that appear near n=6 keywords.
///
/// Looks for lines containing n=6 related terms and extracts their numbers.
pub fn extract_n6_context(text: &str) -> Vec<(String, f64)> {
    let n6_terms = [
        "sigma", "phi", "tau", "n=6", "J2", "sopfr", "mu",
        "divisor", "euler", "perfect number",
    ];

    let mut results = Vec::new();

    for line in text.lines() {
        let lower = line.to_lowercase();
        for term in &n6_terms {
            if lower.contains(term) {
                let numbers = extract_numbers(line);
                for num in numbers {
                    results.push((term.to_string(), num));
                }
                break;
            }
        }
    }

    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers() {
        let text = "The sigma is 12.0 and phi is 2.0, tau=4.0";
        let nums = extract_numbers(text);
        assert!(nums.contains(&12.0));
        assert!(nums.contains(&2.0));
        assert!(nums.contains(&4.0));
    }

    #[test]
    fn test_extract_numbers_negative() {
        let text = "temperature -273.15 pressure 101.325";
        let nums = extract_numbers(text);
        assert!(nums.contains(&-273.15));
        assert!(nums.contains(&101.325));
    }

    #[test]
    fn test_extract_kv_equals() {
        let text = "sigma = 12.0\nphi = 2.0\ntau = 4.0";
        let pairs = extract_key_value_pairs(text);
        assert_eq!(pairs.len(), 3);
        assert_eq!(pairs[0].0, "sigma");
        assert!((pairs[0].1 - 12.0).abs() < 1e-10);
    }

    #[test]
    fn test_extract_kv_colon() {
        let text = "sigma: 12.0\nphi: 2.0";
        let pairs = extract_key_value_pairs(text);
        assert_eq!(pairs.len(), 2);
    }

    #[test]
    fn test_extract_kv_with_units() {
        let text = "energy = 24.0 eV\nbandgap = 1.333 eV";
        let pairs = extract_key_value_pairs(text);
        assert_eq!(pairs.len(), 2);
        assert!((pairs[0].1 - 24.0).abs() < 1e-10);
    }

    #[test]
    fn test_extract_n6_context() {
        let text = "The sigma value is 12.0\nRandom line 99\nphi equals 2.0";
        let ctx = extract_n6_context(text);
        assert!(!ctx.is_empty());
        assert!(ctx.iter().any(|(t, v)| t == "sigma" && (*v - 12.0).abs() < 1e-10));
    }

    #[test]
    fn test_empty_text() {
        assert!(extract_numbers("").is_empty());
        assert!(extract_key_value_pairs("").is_empty());
        assert!(extract_n6_context("").is_empty());
    }
}
