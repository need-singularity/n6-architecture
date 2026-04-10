/// TOML parser — extract domain info from DSE-map TOML files.
///
/// Parses n6-architecture TOML files (e.g. docs/dse-map.toml) and extracts
/// domain names, best_n6 values, combo counts, DSE status, and numeric fields.
/// Pure std implementation — no external crate required.

/// A single DSE domain entry extracted from TOML.
#[derive(Debug, Clone)]
pub struct DseDomain {
    pub name: String,
    pub dse: String,
    pub combos: Option<u64>,
    pub valid: Option<u64>,
    pub n6_max: Option<f64>,
    pub n6_avg: Option<f64>,
    pub best_n6: Option<String>,
    pub best_pareto: Option<String>,
    pub pareto_frontier: Option<u64>,
    pub levels: Vec<String>,
    pub cross_dse: Vec<String>,
    pub note: Option<String>,
}

/// Top-level metadata from the [meta] section.
#[derive(Debug, Clone)]
pub struct DseMeta {
    pub total_domains: Option<u64>,
    pub total_valid_paths: Option<u64>,
    pub domains_100pct_n6: Option<u64>,
    pub cross_dse_connected: Option<u64>,
}

/// Full parse result.
#[derive(Debug, Clone)]
pub struct DseMap {
    pub meta: DseMeta,
    pub domains: Vec<DseDomain>,
}

/// Read and parse a DSE-map TOML file.
pub fn read_toml(path: &str) -> Result<DseMap, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    parse_toml_content(&content)
}

/// Parse DSE-map TOML content from a string.
pub fn parse_toml_content(content: &str) -> Result<DseMap, String> {
    let sections = split_sections(content);

    let mut meta = DseMeta {
        total_domains: None,
        total_valid_paths: None,
        domains_100pct_n6: None,
        cross_dse_connected: None,
    };

    let mut domains = Vec::new();

    for (name, body) in &sections {
        if name == "meta" {
            meta.total_domains = parse_int_field(body, "total_domains");
            meta.total_valid_paths = parse_int_field(body, "total_valid_paths");
            meta.domains_100pct_n6 = parse_int_field(body, "domains_100pct_n6");
            meta.cross_dse_connected = parse_int_field(body, "cross_dse_connected");
        } else {
            domains.push(parse_domain(name, body));
        }
    }

    Ok(DseMap { meta, domains })
}

/// Extract all numeric values from the TOML (combos, n6_max, n6_avg, etc.).
///
/// Returns a flat list of f64 values found across all domains,
/// consistent with the csv_reader / json_reader pattern.
pub fn extract_all_numbers(dse_map: &DseMap) -> Vec<f64> {
    let mut values = Vec::new();

    if let Some(v) = dse_map.meta.total_domains {
        values.push(v as f64);
    }
    if let Some(v) = dse_map.meta.total_valid_paths {
        values.push(v as f64);
    }
    if let Some(v) = dse_map.meta.domains_100pct_n6 {
        values.push(v as f64);
    }

    for d in &dse_map.domains {
        if let Some(v) = d.combos {
            values.push(v as f64);
        }
        if let Some(v) = d.valid {
            values.push(v as f64);
        }
        if let Some(v) = d.n6_max {
            values.push(v);
        }
        if let Some(v) = d.n6_avg {
            values.push(v);
        }
        if let Some(v) = d.pareto_frontier {
            values.push(v as f64);
        }
    }

    values
}

/// Extract key-value pairs: domain name → n6_max score.
///
/// Useful for building a discovery graph where each domain is a node
/// weighted by its n6 convergence score.
pub fn extract_n6_scores(dse_map: &DseMap) -> Vec<(String, f64)> {
    dse_map
        .domains
        .iter()
        .filter_map(|d| d.n6_max.map(|v| (d.name.clone(), v)))
        .collect()
}

// ── Internal helpers ──────────────────────────────────────────

/// Split TOML content into named sections.
/// Returns (section_name, section_body) pairs.
fn split_sections(content: &str) -> Vec<(String, String)> {
    let mut sections: Vec<(String, String)> = Vec::new();
    let mut current_name = String::new();
    let mut current_body = String::new();

    for line in content.lines() {
        let trimmed = line.trim();

        // Skip comments and empty lines at top level
        if trimmed.starts_with('#') || trimmed.is_empty() {
            if !current_name.is_empty() {
                current_body.push_str(line);
                current_body.push('\n');
            }
            continue;
        }

        // Detect [section] header
        if trimmed.starts_with('[') && !trimmed.starts_with("[[") {
            // Save previous section
            if !current_name.is_empty() {
                sections.push((current_name.clone(), current_body.clone()));
            }
            // Parse section name: strip [ and ]
            current_name = trimmed
                .trim_start_matches('[')
                .trim_end_matches(']')
                .trim()
                .to_string();
            current_body = String::new();
        } else {
            current_body.push_str(line);
            current_body.push('\n');
        }
    }

    // Don't forget the last section
    if !current_name.is_empty() {
        sections.push((current_name, current_body));
    }

    sections
}

fn parse_domain(name: &str, body: &str) -> DseDomain {
    DseDomain {
        name: name.to_string(),
        dse: parse_string_field(body, "dse").unwrap_or_default(),
        combos: parse_int_field(body, "combos"),
        valid: parse_int_field(body, "valid"),
        n6_max: parse_float_field(body, "n6_max"),
        n6_avg: parse_float_field(body, "n6_avg"),
        best_n6: parse_string_field(body, "best_n6"),
        best_pareto: parse_string_field(body, "best_pareto"),
        pareto_frontier: parse_int_field(body, "pareto_frontier"),
        levels: parse_string_array(body, "levels"),
        cross_dse: parse_string_array(body, "cross_dse"),
        note: parse_string_field(body, "note"),
    }
}

/// Parse `key = "value"` from section body.
fn parse_string_field(body: &str, key: &str) -> Option<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        // Match key = "value" or key = 'value'
        if let Some(rest) = strip_key(trimmed, key) {
            let rest = rest.trim();
            if rest.starts_with('"') {
                // Find closing quote (handle simple cases)
                let inner = &rest[1..];
                if let Some(end) = inner.find('"') {
                    return Some(inner[..end].to_string());
                }
            }
        }
    }
    None
}

/// Parse `key = 123` from section body.
fn parse_int_field(body: &str, key: &str) -> Option<u64> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = strip_key(trimmed, key) {
            let val_str = rest.trim();
            if let Ok(v) = val_str.parse::<u64>() {
                return Some(v);
            }
        }
    }
    None
}

/// Parse `key = 72.4` from section body.
fn parse_float_field(body: &str, key: &str) -> Option<f64> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = strip_key(trimmed, key) {
            let val_str = rest.trim();
            if let Ok(v) = val_str.parse::<f64>() {
                return Some(v);
            }
        }
    }
    None
}

/// Parse `key = ["a", "b", "c"]` from section body.
fn parse_string_array(body: &str, key: &str) -> Vec<String> {
    for line in body.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            continue;
        }
        if let Some(rest) = strip_key(trimmed, key) {
            let rest = rest.trim();
            if rest.starts_with('[') {
                // Extract strings between quotes
                let mut items = Vec::new();
                let mut chars = rest.chars().peekable();
                while let Some(c) = chars.next() {
                    if c == '"' {
                        let mut s = String::new();
                        for c2 in chars.by_ref() {
                            if c2 == '"' {
                                break;
                            }
                            s.push(c2);
                        }
                        items.push(s);
                    }
                }
                return items;
            }
        }
    }
    Vec::new()
}

/// Strip `key =` prefix from a line, returning the value part.
fn strip_key<'a>(line: &'a str, key: &str) -> Option<&'a str> {
    let line = line.trim();
    if !line.starts_with(key) {
        return None;
    }
    let after_key = &line[key.len()..];
    let after_key = after_key.trim_start();
    if after_key.starts_with('=') {
        Some(&after_key[1..])
    } else {
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE_TOML: &str = r#"
# DSE Map sample

[meta]
updated = "2026-04-03T12"
total_domains = 3
total_valid_paths = 100000
domains_100pct_n6 = 2

[chip-architecture]
goal = true
dse = "done"
combos = 89250
valid = 89250
levels = ["Material", "Process", "Core", "Chip", "System"]
cross_dse = ["battery-architecture", "solar-architecture"]
best_pareto = "Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC (n6=100%, perf=0.950)"
best_n6 = "Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC (100%)"
pareto_frontier = 85
n6_max = 100.0
n6_avg = 72.4
note = "v3: Topo_DC best"

[battery-architecture]
dse = "done"
combos = 1908
best_n6 = "LCO + Si-SSB + Hex6_Prismatic + Wireless-12ch + 48V-ESS (100.0%)"
n6_max = 100.0
n6_avg = 76.3
levels = ["Cell", "Electrode", "Core", "Chip", "Pack+Grid"]
cross_dse = ["chip-architecture"]

[wip-domain]
dse = "wip"
combos = 500
n6_max = 50.0
levels = ["A", "B"]
cross_dse = []
"#;

    #[test]
    fn test_parse_meta() {
        let map = parse_toml_content(SAMPLE_TOML).unwrap();
        assert_eq!(map.meta.total_domains, Some(3));
        assert_eq!(map.meta.total_valid_paths, Some(100000));
        assert_eq!(map.meta.domains_100pct_n6, Some(2));
    }

    #[test]
    fn test_parse_domains() {
        let map = parse_toml_content(SAMPLE_TOML).unwrap();
        assert_eq!(map.domains.len(), 3);

        let chip = &map.domains[0];
        assert_eq!(chip.name, "chip-architecture");
        assert_eq!(chip.dse, "done");
        assert_eq!(chip.combos, Some(89250));
        assert_eq!(chip.valid, Some(89250));
        assert!((chip.n6_max.unwrap() - 100.0).abs() < 1e-10);
        assert!((chip.n6_avg.unwrap() - 72.4).abs() < 1e-10);
        assert_eq!(chip.pareto_frontier, Some(85));
        assert_eq!(chip.levels.len(), 5);
        assert_eq!(chip.levels[0], "Material");
        assert_eq!(chip.cross_dse.len(), 2);
        assert!(chip.best_n6.is_some());
        assert!(chip.note.is_some());
    }

    #[test]
    fn test_parse_wip_domain() {
        let map = parse_toml_content(SAMPLE_TOML).unwrap();
        let wip = &map.domains[2];
        assert_eq!(wip.name, "wip-domain");
        assert_eq!(wip.dse, "wip");
        assert_eq!(wip.combos, Some(500));
        assert!((wip.n6_max.unwrap() - 50.0).abs() < 1e-10);
        assert!(wip.cross_dse.is_empty());
    }

    #[test]
    fn test_extract_all_numbers() {
        let map = parse_toml_content(SAMPLE_TOML).unwrap();
        let nums = extract_all_numbers(&map);
        // meta: total_domains(3), total_valid_paths(100000), domains_100pct(2)
        // chip: combos(89250), valid(89250), n6_max(100), n6_avg(72.4), pareto(85)
        // battery: combos(1908), n6_max(100), n6_avg(76.3)
        // wip: combos(500), n6_max(50)
        assert!(nums.contains(&3.0));
        assert!(nums.contains(&89250.0));
        assert!(nums.contains(&100.0));
        assert!(nums.contains(&72.4));
        assert!(nums.len() >= 13);
    }

    #[test]
    fn test_extract_n6_scores() {
        let map = parse_toml_content(SAMPLE_TOML).unwrap();
        let scores = extract_n6_scores(&map);
        assert_eq!(scores.len(), 3);
        assert_eq!(scores[0].0, "chip-architecture");
        assert!((scores[0].1 - 100.0).abs() < 1e-10);
        assert_eq!(scores[2].0, "wip-domain");
        assert!((scores[2].1 - 50.0).abs() < 1e-10);
    }

    #[test]
    fn test_empty_content() {
        let map = parse_toml_content("").unwrap();
        assert!(map.domains.is_empty());
        assert!(map.meta.total_domains.is_none());
    }

    #[test]
    fn test_read_toml_file_not_found() {
        let result = read_toml("/nonexistent/path.toml");
        assert!(result.is_err());
    }
}
