/// Rust source parser — extract constant definitions and enum variant counts.
///
/// Lightweight line-based parser for Rust constant patterns:
///   - `const NAME: TYPE = VALUE;`
///   - `pub const NAME: TYPE = VALUE;`
///   - `pub(crate) const NAME: TYPE = VALUE;`
///   - `static NAME: TYPE = VALUE;`
///   - `pub static NAME: TYPE = VALUE;`
///   - `///` or `//` comment preceding a constant → used as description
///   - `pub enum Name { A, B, C }` → variant count as a numeric value

/// A single constant extracted from Rust source.
#[derive(Debug, Clone)]
pub struct RsConstant {
    /// Constant name, e.g. "MAX_RETRIES" or "DEFAULT_TIMEOUT".
    pub name: String,
    /// Parsed value.
    pub value: RsValue,
    /// Comment/description from preceding `///` or `//` lines, if any.
    pub comment: Option<String>,
    /// 1-based line number where the definition appears.
    pub line: usize,
}

/// Possible value types for a Rust constant.
#[derive(Debug, Clone)]
pub enum RsValue {
    Int(i64),
    Float(f64),
    Str(String),
    Bool(bool),
    /// Enum variant count: name of enum + number of variants.
    EnumVariants(String, usize),
}

impl RsValue {
    /// Try to interpret this value as f64.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            RsValue::Int(v) => Some(*v as f64),
            RsValue::Float(v) => Some(*v),
            RsValue::Bool(v) => Some(if *v { 1.0 } else { 0.0 }),
            RsValue::EnumVariants(_, count) => Some(*count as f64),
            RsValue::Str(_) => None,
        }
    }
}

/// Parse Rust source and extract all constant/static definitions and enum variant counts.
pub fn parse_rs_constants(source: &str) -> Vec<RsConstant> {
    let mut constants = Vec::new();
    let mut prev_comment: Option<String> = None;
    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Collect doc-comment or line-comment
        if trimmed.starts_with("///") {
            let text = trimmed.trim_start_matches('/').trim().to_string();
            prev_comment = merge_comment(prev_comment, &text);
            i += 1;
            continue;
        }
        if trimmed.starts_with("//") && !trimmed.starts_with("//!") {
            let text = trimmed.trim_start_matches('/').trim().to_string();
            if !text.is_empty() {
                prev_comment = merge_comment(prev_comment, &text);
            }
            i += 1;
            continue;
        }

        // Try const/static parse
        if let Some(c) = try_parse_const(trimmed, i + 1, &prev_comment) {
            constants.push(c);
            prev_comment = None;
            i += 1;
            continue;
        }

        // Try enum variant count (may span multiple lines)
        if let Some((c, consumed)) = try_parse_enum(&lines, i, &prev_comment) {
            constants.push(c);
            prev_comment = None;
            i += consumed;
            continue;
        }

        // Non-comment, non-match line clears pending comment
        if !trimmed.is_empty() {
            prev_comment = None;
        }

        i += 1;
    }

    constants
}

/// Extract only numeric constants as (name, f64) pairs.
///
/// Includes enum variant counts as "EnumName.variants".
pub fn extract_numeric_constants(source: &str) -> Vec<(String, f64)> {
    let constants = parse_rs_constants(source);
    let mut pairs = Vec::new();

    for c in &constants {
        match &c.value {
            RsValue::EnumVariants(name, count) => {
                pairs.push((format!("{}.variants", name), *count as f64));
            }
            _ => {
                if let Some(v) = c.value.as_f64() {
                    pairs.push((c.name.clone(), v));
                }
            }
        }
    }

    pairs
}

/// Read a Rust file and extract numeric constants.
pub fn read_rs_constants(path: &str) -> Result<Vec<(String, f64)>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    Ok(extract_numeric_constants(&content))
}

// ─── Internal helpers ──────────────────────────────────────

/// Merge a new comment line into an existing accumulated comment.
fn merge_comment(existing: Option<String>, new_line: &str) -> Option<String> {
    if new_line.is_empty() {
        return existing;
    }
    match existing {
        Some(prev) => Some(format!("{} {}", prev, new_line)),
        None => Some(new_line.to_string()),
    }
}

/// Strip visibility modifier and `const`/`static` keyword prefix.
///
/// Returns the remainder after `const`/`static` keyword, or None if not a const/static line.
fn strip_const_prefix(trimmed: &str) -> Option<&str> {
    let rest = strip_visibility(trimmed);

    if let Some(after) = rest.strip_prefix("const ") {
        Some(after)
    } else if let Some(after) = rest.strip_prefix("static ") {
        Some(after)
    } else {
        None
    }
}

/// Strip `pub`, `pub(crate)`, `pub(super)` prefix.
fn strip_visibility(s: &str) -> &str {
    if let Some(rest) = s.strip_prefix("pub") {
        let rest = rest.trim_start();
        if rest.starts_with('(') {
            // pub(crate), pub(super), etc.
            if let Some(close) = rest.find(')') {
                return rest[close + 1..].trim_start();
            }
        }
        rest
    } else {
        s
    }
}

/// Try to parse a `const` or `static` definition from a single line.
fn try_parse_const(trimmed: &str, line_num: usize, comment: &Option<String>) -> Option<RsConstant> {
    let rest = strip_const_prefix(trimmed)?;

    // rest should be: NAME: TYPE = VALUE;
    let colon_pos = rest.find(':')?;
    let name = rest[..colon_pos].trim();

    if name.is_empty() || !is_rs_identifier(name) {
        return None;
    }

    let after_colon = &rest[colon_pos + 1..];
    let eq_pos = after_colon.find('=')?;
    let val_part = after_colon[eq_pos + 1..].trim();

    // Strip trailing semicolon
    let val_str = val_part.trim_end_matches(';').trim();

    let value = parse_rs_value(val_str)?;

    Some(RsConstant {
        name: name.to_string(),
        value,
        comment: comment.clone(),
        line: line_num,
    })
}

/// Try to parse an enum definition and count its variants.
///
/// Handles both single-line `enum Foo { A, B, C }` and multi-line enums.
/// Returns (constant, lines_consumed).
fn try_parse_enum(lines: &[&str], start: usize, comment: &Option<String>) -> Option<(RsConstant, usize)> {
    let trimmed = lines[start].trim();
    let rest = strip_visibility(trimmed);

    if !rest.starts_with("enum ") {
        return None;
    }

    let after_enum = &rest[5..];
    // Extract enum name (stop at '{', '<', or whitespace)
    let name_end = after_enum
        .find(|c: char| c == '{' || c == '<' || c.is_whitespace())
        .unwrap_or(after_enum.len());
    let enum_name = after_enum[..name_end].trim();

    if enum_name.is_empty() {
        return None;
    }

    // Find the opening brace
    let full_first = lines[start];
    if let Some(brace_pos) = full_first.find('{') {
        // Check if closing brace is on same line
        let after_brace = &full_first[brace_pos + 1..];
        if let Some(close_pos) = after_brace.find('}') {
            let body = &after_brace[..close_pos];
            let count = count_enum_variants(body);
            if count > 0 {
                return Some((
                    RsConstant {
                        name: format!("{}.variants", enum_name),
                        value: RsValue::EnumVariants(enum_name.to_string(), count),
                        comment: comment.clone(),
                        line: start + 1,
                    },
                    1,
                ));
            }
        }

        // Multi-line: collect until closing brace
        let mut body = after_brace.to_string();
        let mut consumed = 1;
        let mut j = start + 1;
        while j < lines.len() {
            let line = lines[j];
            consumed += 1;
            if let Some(close) = line.find('}') {
                body.push(' ');
                body.push_str(&line[..close]);
                break;
            }
            body.push(' ');
            body.push_str(line);
            j += 1;
        }
        let count = count_enum_variants(&body);
        if count > 0 {
            return Some((
                RsConstant {
                    name: format!("{}.variants", enum_name),
                    value: RsValue::EnumVariants(enum_name.to_string(), count),
                    comment: comment.clone(),
                    line: start + 1,
                },
                consumed,
            ));
        }
    } else {
        // Opening brace on next line
        if start + 1 < lines.len() && lines[start + 1].trim() == "{" {
            let mut body = String::new();
            let mut consumed = 2;
            let mut j = start + 2;
            while j < lines.len() {
                let line = lines[j];
                consumed += 1;
                if let Some(close) = line.find('}') {
                    body.push(' ');
                    body.push_str(&line[..close]);
                    break;
                }
                body.push(' ');
                body.push_str(line);
                j += 1;
            }
            let count = count_enum_variants(&body);
            if count > 0 {
                return Some((
                    RsConstant {
                        name: format!("{}.variants", enum_name),
                        value: RsValue::EnumVariants(enum_name.to_string(), count),
                        comment: comment.clone(),
                        line: start + 1,
                    },
                    consumed,
                ));
            }
        }
    }

    None
}

/// Count non-empty variant entries in an enum body.
///
/// Splits on commas, filters out empty/comment-only entries.
/// Handles tuple variants `Foo(i32)` and struct variants `Foo { x: i32 }`.
fn count_enum_variants(body: &str) -> usize {
    let mut count = 0;
    let mut depth = 0;
    let mut current = String::new();

    for ch in body.chars() {
        match ch {
            '(' | '{' => {
                depth += 1;
                current.push(ch);
            }
            ')' | '}' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                if is_variant_entry(&current) {
                    count += 1;
                }
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    // Last entry (no trailing comma)
    if is_variant_entry(&current) {
        count += 1;
    }

    count
}

/// Check if a comma-separated entry looks like a real variant (not just whitespace/comments).
fn is_variant_entry(s: &str) -> bool {
    let trimmed = s.trim();
    if trimmed.is_empty() {
        return false;
    }
    // Skip attribute-only or comment-only lines
    let filtered: String = trimmed
        .lines()
        .map(|l| l.trim())
        .filter(|l| !l.starts_with("//") && !l.starts_with('#'))
        .collect::<Vec<_>>()
        .join(" ");
    let filtered = filtered.trim();
    !filtered.is_empty()
}

/// Check if a string is a valid Rust identifier.
fn is_rs_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let first = s.chars().next().unwrap();
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Parse a Rust value literal (numeric, string, bool).
fn parse_rs_value(s: &str) -> Option<RsValue> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Bool literals
    if s == "true" {
        return Some(RsValue::Bool(true));
    }
    if s == "false" {
        return Some(RsValue::Bool(false));
    }

    // String literal
    if s.starts_with('"') && s.ends_with('"') && s.len() >= 2 {
        return Some(RsValue::Str(s[1..s.len() - 1].to_string()));
    }

    // Strip type suffix for numeric literals (e.g. 42u32, 3.14f64, 1_000_i64)
    let cleaned = strip_numeric_suffix(s);
    // Remove underscores used as visual separators
    let no_underscores = cleaned.replace('_', "");

    // Try integer
    if let Ok(v) = no_underscores.parse::<i64>() {
        return Some(RsValue::Int(v));
    }

    // Try hex literal
    if no_underscores.starts_with("0x") || no_underscores.starts_with("0X") {
        if let Ok(v) = i64::from_str_radix(&no_underscores[2..], 16) {
            return Some(RsValue::Int(v));
        }
    }

    // Try binary literal
    if no_underscores.starts_with("0b") || no_underscores.starts_with("0B") {
        if let Ok(v) = i64::from_str_radix(&no_underscores[2..], 2) {
            return Some(RsValue::Int(v));
        }
    }

    // Try octal literal
    if no_underscores.starts_with("0o") || no_underscores.starts_with("0O") {
        if let Ok(v) = i64::from_str_radix(&no_underscores[2..], 8) {
            return Some(RsValue::Int(v));
        }
    }

    // Try float (including scientific notation)
    if let Ok(v) = no_underscores.parse::<f64>() {
        return Some(RsValue::Float(v));
    }

    None
}

/// Strip Rust numeric type suffixes: u8, u16, u32, u64, u128, usize,
/// i8, i16, i32, i64, i128, isize, f32, f64.
fn strip_numeric_suffix(s: &str) -> &str {
    let suffixes = [
        "u128", "i128", "usize", "isize",
        "u64", "i64", "f64", "u32", "i32", "f32",
        "u16", "i16", "u8", "i8",
    ];
    for suffix in &suffixes {
        if s.ends_with(suffix) {
            let prefix = &s[..s.len() - suffix.len()];
            // Make sure the character before the suffix is a digit or underscore
            if let Some(last) = prefix.chars().last() {
                if last.is_ascii_digit() || last == '_' {
                    return prefix;
                }
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_const() {
        let src = "const MAX_RETRIES: u32 = 5;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "MAX_RETRIES");
        assert!(matches!(consts[0].value, RsValue::Int(5)));
    }

    #[test]
    fn test_pub_const() {
        let src = "pub const TIMEOUT: f64 = 30.0;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "TIMEOUT");
        match &consts[0].value {
            RsValue::Float(v) => assert!((v - 30.0).abs() < 1e-10),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_pub_crate_const() {
        let src = "pub(crate) const LIMIT: i64 = -100;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "LIMIT");
        assert!(matches!(consts[0].value, RsValue::Int(-100)));
    }

    #[test]
    fn test_static_const() {
        let src = "static BUFFER_SIZE: usize = 4096;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "BUFFER_SIZE");
        assert!(matches!(consts[0].value, RsValue::Int(4096)));
    }

    #[test]
    fn test_pub_static() {
        let src = "pub static GLOBAL_SCALE: f32 = 1.5f32;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "GLOBAL_SCALE");
        match &consts[0].value {
            RsValue::Float(v) => assert!((v - 1.5).abs() < 1e-10),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_string_const() {
        let src = r#"pub const VERSION: &str = "1.0.0";"#;
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            RsValue::Str(s) => assert_eq!(s, "1.0.0"),
            _ => panic!("Expected Str"),
        }
    }

    #[test]
    fn test_bool_const() {
        let src = "const DEBUG: bool = true;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, RsValue::Bool(true)));
    }

    #[test]
    fn test_hex_literal() {
        let src = "const MASK: u32 = 0xFF00;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, RsValue::Int(0xFF00)));
    }

    #[test]
    fn test_binary_literal() {
        let src = "const FLAGS: u8 = 0b1010;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, RsValue::Int(0b1010)));
    }

    #[test]
    fn test_underscore_separator() {
        let src = "const MILLION: u64 = 1_000_000;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, RsValue::Int(1_000_000)));
    }

    #[test]
    fn test_typed_suffix() {
        let src = "const SIZE: usize = 256usize;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, RsValue::Int(256)));
    }

    #[test]
    fn test_scientific_notation() {
        let src = "const PLANCK: f64 = 6.626e-34;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            RsValue::Float(v) => assert!((v - 6.626e-34).abs() < 1e-44),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_doc_comment() {
        let src = "/// Maximum number of retries before giving up.\nconst MAX_RETRIES: u32 = 5;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(
            consts[0].comment.as_deref(),
            Some("Maximum number of retries before giving up.")
        );
    }

    #[test]
    fn test_line_comment() {
        let src = "// retry limit\nconst MAX_RETRIES: u32 = 5;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].comment.as_deref(), Some("retry limit"));
    }

    #[test]
    fn test_multi_line_doc_comment() {
        let src = "/// First line.\n/// Second line.\nconst X: i32 = 42;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(
            consts[0].comment.as_deref(),
            Some("First line. Second line.")
        );
    }

    #[test]
    fn test_enum_single_line() {
        let src = "pub enum NodeType { Alpha, Beta, Gamma }\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "NodeType.variants");
        match &consts[0].value {
            RsValue::EnumVariants(name, count) => {
                assert_eq!(name, "NodeType");
                assert_eq!(*count, 3);
            }
            _ => panic!("Expected EnumVariants"),
        }
    }

    #[test]
    fn test_enum_multi_line() {
        let src = "pub enum Color {\n    Red,\n    Green,\n    Blue,\n}\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            RsValue::EnumVariants(name, count) => {
                assert_eq!(name, "Color");
                assert_eq!(*count, 3);
            }
            _ => panic!("Expected EnumVariants"),
        }
    }

    #[test]
    fn test_enum_with_tuple_variants() {
        let src = "enum Expr {\n    Lit(i64),\n    Add(Box<Expr>, Box<Expr>),\n    Neg(Box<Expr>),\n}\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            RsValue::EnumVariants(_, count) => assert_eq!(*count, 3),
            _ => panic!("Expected EnumVariants"),
        }
    }

    #[test]
    fn test_enum_trailing_comma() {
        let src = "enum Dir { Up, Down, Left, Right, }\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            RsValue::EnumVariants(_, count) => assert_eq!(*count, 4),
            _ => panic!("Expected EnumVariants"),
        }
    }

    #[test]
    fn test_extract_numeric_constants() {
        let src = concat!(
            "pub const SIGMA: i32 = 12;\n",
            "pub const TAU: f64 = 4.0;\n",
            "const NAME: &str = \"test\";\n",
            "pub enum NodeType { A, B, C }\n",
        );
        let pairs = extract_numeric_constants(src);
        // SIGMA=12, TAU=4.0, NodeType.variants=3 (NAME is string, skipped)
        assert_eq!(pairs.len(), 3);
        assert!(pairs.iter().any(|(n, v)| n == "SIGMA" && (*v - 12.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "TAU" && (*v - 4.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "NodeType.variants" && (*v - 3.0).abs() < 1e-10));
    }

    #[test]
    fn test_multiple_constants() {
        let src = concat!(
            "/// The perfect number\n",
            "pub const P1: u32 = 6;\n",
            "/// Mersenne prime\n",
            "pub const M3: u32 = 7;\n",
            "pub const PAIR: u32 = 28;\n",
        );
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 3);
        assert_eq!(consts[0].name, "P1");
        assert_eq!(consts[0].comment.as_deref(), Some("The perfect number"));
        assert_eq!(consts[1].name, "M3");
        assert_eq!(consts[2].name, "PAIR");
        assert!(consts[2].comment.is_none());
    }

    #[test]
    fn test_skips_non_literal_values() {
        // Expressions and function calls are not simple literals
        let src = "const X: usize = Vec::new().len();\nconst Y: i32 = 42;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "Y");
    }

    #[test]
    fn test_negative_float() {
        let src = "const TEMP: f64 = -273.15;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            RsValue::Float(v) => assert!((v - (-273.15)).abs() < 1e-10),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_empty_source() {
        assert!(parse_rs_constants("").is_empty());
        assert!(extract_numeric_constants("").is_empty());
    }

    #[test]
    fn test_read_rs_constants_not_found() {
        let result = read_rs_constants("/nonexistent/path.rs");
        assert!(result.is_err());
    }

    #[test]
    fn test_octal_literal() {
        let src = "const PERMS: u32 = 0o755;\n";
        let consts = parse_rs_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, RsValue::Int(0o755)));
    }
}
