/// CSV data reader — no external crates, pure std.

/// Read a CSV file and extract numeric values.
///
/// Returns a 2D vector where each inner vec is a row of f64 values.
/// Non-numeric cells are skipped within each row.
/// The header row (if detected) is skipped.
pub fn read_csv(path: &str) -> Result<Vec<Vec<f64>>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;

    parse_csv_content(&content)
}

/// Parse CSV content from a string (for testability).
pub fn parse_csv_content(content: &str) -> Result<Vec<Vec<f64>>, String> {
    let mut rows: Vec<Vec<f64>> = Vec::new();
    let mut lines = content.lines().peekable();

    // Detect and skip header: if first line has no purely numeric cells, skip it
    if let Some(first_line) = lines.peek() {
        let is_header = first_line
            .split(',')
            .all(|cell| cell.trim().parse::<f64>().is_err());
        if is_header {
            lines.next(); // consume header
        }
    }

    for line in lines {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            continue;
        }

        let row: Vec<f64> = trimmed
            .split(',')
            .filter_map(|cell| cell.trim().parse::<f64>().ok())
            .collect();

        if !row.is_empty() {
            rows.push(row);
        }
    }

    Ok(rows)
}

/// Flatten all CSV rows into a single vector of f64 values.
pub fn flatten(rows: &[Vec<f64>]) -> Vec<f64> {
    rows.iter().flat_map(|r| r.iter().copied()).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_csv_basic() {
        let csv = "a,b,c\n1.0,2.0,3.0\n4.0,5.0,6.0\n";
        let rows = parse_csv_content(csv).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec![1.0, 2.0, 3.0]);
        assert_eq!(rows[1], vec![4.0, 5.0, 6.0]);
    }

    #[test]
    fn test_parse_csv_no_header() {
        let csv = "6.0,12.0,24.0\n4.0,2.0,5.0\n";
        let rows = parse_csv_content(csv).unwrap();
        assert_eq!(rows.len(), 2);
        assert_eq!(rows[0], vec![6.0, 12.0, 24.0]);
    }

    #[test]
    fn test_parse_csv_mixed_cells() {
        let csv = "name,value\nalpha,6.0\nbeta,12.0\n";
        let rows = parse_csv_content(csv).unwrap();
        assert_eq!(rows.len(), 2);
        // Non-numeric "alpha" and "beta" are filtered out
        assert_eq!(rows[0], vec![6.0]);
        assert_eq!(rows[1], vec![12.0]);
    }

    #[test]
    fn test_parse_csv_empty() {
        let csv = "";
        let rows = parse_csv_content(csv).unwrap();
        assert!(rows.is_empty());
    }

    #[test]
    fn test_flatten() {
        let rows = vec![vec![1.0, 2.0], vec![3.0, 4.0, 5.0]];
        assert_eq!(flatten(&rows), vec![1.0, 2.0, 3.0, 4.0, 5.0]);
    }
}
