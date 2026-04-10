use std::collections::HashMap;

/// Convert parsed hypothesis entries into a flat f64 matrix.
///
/// Returns (flat_data, n_rows, n_cols) where flat_data is row-major.
/// Non-numeric or missing values become f64::NAN.
pub fn vectorize(
    entries: &[HashMap<String, String>],
    feature_keys: &[&str],
) -> (Vec<f64>, usize, usize) {
    let n_rows = entries.len();
    let n_cols = feature_keys.len();
    let mut data = Vec::with_capacity(n_rows * n_cols);

    for entry in entries {
        for &key in feature_keys {
            let val = entry
                .get(key)
                .and_then(|v| v.parse::<f64>().ok())
                .unwrap_or(f64::NAN);
            data.push(val);
        }
    }

    (data, n_rows, n_cols)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vectorize_basic() {
        let mut e1 = HashMap::new();
        e1.insert("Tc".to_string(), "39".to_string());
        e1.insert("P".to_string(), "0".to_string());

        let mut e2 = HashMap::new();
        e2.insert("Tc".to_string(), "93".to_string());
        e2.insert("P".to_string(), "0".to_string());

        let entries = vec![e1, e2];
        let (data, rows, cols) = vectorize(&entries, &["Tc", "P"]);

        assert_eq!(rows, 2);
        assert_eq!(cols, 2);
        assert_eq!(data.len(), 4);
        assert_eq!(data[0], 39.0);
        assert_eq!(data[1], 0.0);
        assert_eq!(data[2], 93.0);
    }

    #[test]
    fn test_vectorize_missing() {
        let mut e1 = HashMap::new();
        e1.insert("Tc".to_string(), "39".to_string());
        // Missing "P"

        let entries = vec![e1];
        let (data, _, _) = vectorize(&entries, &["Tc", "P"]);
        assert_eq!(data[0], 39.0);
        assert!(data[1].is_nan());
    }
}
