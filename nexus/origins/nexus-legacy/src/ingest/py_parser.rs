/// Python constant parser — extract constant definitions from Python source files.
///
/// Lightweight regex-free parser for Python constant patterns:
///   - `NAME = number` (int, float, scientific notation)
///   - `NAME = "string"`
///   - `NAME = { "key": value, ... }` (flat and nested dicts)
///   - `# comment` preceding a constant → used as description
///   - Class-level constants (`class Foo:` scope tracking)

/// A single constant extracted from Python source.
#[derive(Debug, Clone)]
pub struct PyConstant {
    /// Constant name, e.g. "SIGMA" or "MyClass.SIGMA"
    pub name: String,
    /// Parsed value
    pub value: PyValue,
    /// Comment/description from preceding `#` line, if any
    pub comment: Option<String>,
    /// 1-based line number where the assignment appears
    pub line: usize,
}

/// Possible value types for a Python constant.
#[derive(Debug, Clone)]
pub enum PyValue {
    Int(i64),
    Float(f64),
    Str(String),
    Dict(Vec<(String, PyValue)>),
}

impl PyValue {
    /// Try to interpret this value as f64.
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            PyValue::Int(v) => Some(*v as f64),
            PyValue::Float(v) => Some(*v),
            _ => None,
        }
    }
}

/// Parse Python source and extract all top-level and class-level constant assignments.
pub fn parse_py_constants(source: &str) -> Vec<PyConstant> {
    let mut constants = Vec::new();
    let mut prev_comment: Option<String> = None;
    let mut class_name: Option<String> = None;
    let mut class_indent: usize = 0;

    let lines: Vec<&str> = source.lines().collect();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i];
        let trimmed = line.trim();

        // Track class scope
        if let Some(cls) = parse_class_def(trimmed) {
            class_name = Some(cls);
            class_indent = indent_level(line);
            prev_comment = None;
            i += 1;
            continue;
        }

        // Exit class scope when indentation returns to class level or less
        if class_name.is_some() && !trimmed.is_empty() && !trimmed.starts_with('#') {
            let cur_indent = indent_level(line);
            if cur_indent <= class_indent {
                class_name = None;
            }
        }

        // Collect comment lines
        if trimmed.starts_with('#') {
            let comment_text = trimmed.trim_start_matches('#').trim().to_string();
            if !comment_text.is_empty() {
                prev_comment = Some(comment_text);
            }
            i += 1;
            continue;
        }

        // Try to parse an assignment
        if let Some((name, value, consumed)) = try_parse_assignment(&lines, i) {
            let qualified_name = match &class_name {
                Some(cls) => format!("{}.{}", cls, name),
                None => name,
            };
            constants.push(PyConstant {
                name: qualified_name,
                value,
                comment: prev_comment.take(),
                line: i + 1,
            });
            i += consumed;
            continue;
        }

        // Empty or non-assignment line clears the pending comment
        if !trimmed.is_empty() {
            prev_comment = None;
        }

        i += 1;
    }

    constants
}

/// Extract only numeric constants as (name, f64) pairs.
///
/// Flattens dict values into "DICT_NAME.key" entries.
pub fn extract_numeric_constants(source: &str) -> Vec<(String, f64)> {
    let constants = parse_py_constants(source);
    let mut pairs = Vec::new();

    for c in &constants {
        match &c.value {
            PyValue::Int(v) => {
                pairs.push((c.name.clone(), *v as f64));
            }
            PyValue::Float(v) => {
                pairs.push((c.name.clone(), *v));
            }
            PyValue::Dict(entries) => {
                for (key, val) in entries {
                    if let Some(num) = val.as_f64() {
                        pairs.push((format!("{}.{}", c.name, key), num));
                    }
                }
            }
            PyValue::Str(_) => {}
        }
    }

    pairs
}

/// Read a Python file and extract numeric constants.
pub fn read_py_constants(path: &str) -> Result<Vec<(String, f64)>, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    Ok(extract_numeric_constants(&content))
}

// ─── Internal helpers ──────────────────────────────────────

/// Parse `class Foo:` or `class Foo(Base):` and return the class name.
fn parse_class_def(trimmed: &str) -> Option<String> {
    if !trimmed.starts_with("class ") {
        return None;
    }
    let rest = &trimmed[6..];
    let end = rest.find(|c: char| c == '(' || c == ':').unwrap_or(rest.len());
    let name = rest[..end].trim();
    if name.is_empty() || !name.chars().next().unwrap().is_alphabetic() {
        return None;
    }
    Some(name.to_string())
}

/// Count leading spaces (tabs count as 4).
fn indent_level(line: &str) -> usize {
    let mut level = 0;
    for ch in line.chars() {
        match ch {
            ' ' => level += 1,
            '\t' => level += 4,
            _ => break,
        }
    }
    level
}

/// Try to parse `NAME = value` starting at line index `start`.
///
/// Returns `(name, value, lines_consumed)` on success.
/// For dict values that span multiple lines, `lines_consumed` > 1.
fn try_parse_assignment(lines: &[&str], start: usize) -> Option<(String, PyValue, usize)> {
    let line = lines[start];
    let trimmed = line.trim();

    // Must match: IDENTIFIER = ...
    let eq_pos = trimmed.find('=')?;
    if eq_pos == 0 {
        return None;
    }

    // Reject ==, !=, <=, >= (comparison operators)
    let before_eq = trimmed.as_bytes().get(eq_pos.wrapping_sub(1));
    if matches!(before_eq, Some(b'!' | b'<' | b'>')) {
        return None;
    }
    if trimmed.as_bytes().get(eq_pos + 1) == Some(&b'=') {
        return None;
    }

    let name_part = trimmed[..eq_pos].trim();
    let val_part = trimmed[eq_pos + 1..].trim();

    // Name must be a valid Python identifier (allow dots for chained attrs)
    if !is_py_identifier(name_part) {
        return None;
    }

    // Strip inline comment from value part
    let val_clean = strip_inline_comment(val_part);

    // Try parsing the value
    if let Some(val) = parse_value(&val_clean) {
        return Some((name_part.to_string(), val, 1));
    }

    // Multi-line dict: value starts with `{` but doesn't close on same line
    if val_clean.starts_with('{') && !val_clean.contains('}') {
        let mut collected = val_clean.to_string();
        let mut consumed = 1;
        let mut j = start + 1;
        while j < lines.len() {
            let continuation = lines[j].trim();
            collected.push(' ');
            collected.push_str(continuation);
            consumed += 1;
            if continuation.contains('}') {
                break;
            }
            j += 1;
        }
        if let Some(val) = parse_value(&collected) {
            return Some((name_part.to_string(), val, consumed));
        }
    }

    None
}

/// Check if a string is a valid Python identifier (uppercase, lowercase, digits, underscores).
fn is_py_identifier(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let first = s.chars().next().unwrap();
    if !first.is_alphabetic() && first != '_' {
        return false;
    }
    s.chars().all(|c| c.is_alphanumeric() || c == '_')
}

/// Strip inline `# comment` from a value string, respecting string quotes.
fn strip_inline_comment(val: &str) -> String {
    let mut in_string = false;
    let mut quote_char = '"';
    let chars: Vec<char> = val.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        let ch = chars[i];
        if in_string {
            if ch == '\\' {
                i += 2;
                continue;
            }
            if ch == quote_char {
                in_string = false;
            }
        } else {
            if ch == '#' {
                return chars[..i].iter().collect::<String>().trim().to_string();
            }
            if ch == '"' || ch == '\'' {
                in_string = true;
                quote_char = ch;
            }
        }
        i += 1;
    }

    val.to_string()
}

/// Parse a Python value literal.
fn parse_value(s: &str) -> Option<PyValue> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // String literal
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        if s.len() >= 2 {
            return Some(PyValue::Str(s[1..s.len() - 1].to_string()));
        }
    }

    // Dict literal
    if s.starts_with('{') && s.ends_with('}') {
        return parse_dict(s);
    }

    // Try integer (before float to preserve type)
    if let Ok(v) = s.parse::<i64>() {
        return Some(PyValue::Int(v));
    }

    // Try float (including scientific notation)
    if let Ok(v) = s.parse::<f64>() {
        return Some(PyValue::Float(v));
    }

    // Negative number with spaces: - 273.15
    let no_space = s.replace(' ', "");
    if no_space.starts_with('-') {
        if let Ok(v) = no_space.parse::<i64>() {
            return Some(PyValue::Int(v));
        }
        if let Ok(v) = no_space.parse::<f64>() {
            return Some(PyValue::Float(v));
        }
    }

    None
}

/// Parse a Python dict literal `{ "key": value, ... }`.
///
/// Handles string keys with numeric, string, or nested dict values.
fn parse_dict(s: &str) -> Option<PyValue> {
    let inner = s[1..s.len() - 1].trim();
    if inner.is_empty() {
        return Some(PyValue::Dict(Vec::new()));
    }

    let mut entries = Vec::new();
    let tokens = split_dict_entries(inner);

    for token in &tokens {
        let token = token.trim();
        if token.is_empty() {
            continue;
        }

        // Find the colon separating key from value
        if let Some((key, val)) = split_kv_entry(token) {
            let key = key.trim();
            let val = val.trim();

            // Extract key from quotes
            let key_str = if (key.starts_with('"') && key.ends_with('"'))
                || (key.starts_with('\'') && key.ends_with('\''))
            {
                key[1..key.len() - 1].to_string()
            } else {
                key.to_string()
            };

            if let Some(parsed) = parse_value(val) {
                entries.push((key_str, parsed));
            }
        }
    }

    Some(PyValue::Dict(entries))
}

/// Split dict interior into entries, respecting nested braces and quotes.
fn split_dict_entries(s: &str) -> Vec<String> {
    let mut entries = Vec::new();
    let mut current = String::new();
    let mut depth = 0;
    let mut in_string = false;
    let mut quote_char = '"';

    for ch in s.chars() {
        if in_string {
            current.push(ch);
            if ch == '\\' {
                // next char is escaped, handled by pushing it naturally
            } else if ch == quote_char {
                in_string = false;
            }
            continue;
        }

        match ch {
            '"' | '\'' => {
                in_string = true;
                quote_char = ch;
                current.push(ch);
            }
            '{' => {
                depth += 1;
                current.push(ch);
            }
            '}' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                entries.push(current.clone());
                current.clear();
            }
            _ => {
                current.push(ch);
            }
        }
    }

    if !current.trim().is_empty() {
        entries.push(current);
    }

    entries
}

/// Split a single `key: value` entry at the first colon outside quotes.
fn split_kv_entry(s: &str) -> Option<(&str, &str)> {
    let mut in_string = false;
    let mut quote_char = b'"';
    let bytes = s.as_bytes();

    for (i, &b) in bytes.iter().enumerate() {
        if in_string {
            if b == b'\\' {
                // skip next
                continue;
            }
            if b == quote_char {
                in_string = false;
            }
            continue;
        }
        if b == b'"' || b == b'\'' {
            in_string = true;
            quote_char = b;
            continue;
        }
        if b == b':' {
            return Some((&s[..i], &s[i + 1..]));
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_int() {
        let src = "SIGMA = 12\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "SIGMA");
        assert!(matches!(consts[0].value, PyValue::Int(12)));
    }

    #[test]
    fn test_simple_float() {
        let src = "PSI_ALPHA = 0.014\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "PSI_ALPHA");
        match &consts[0].value {
            PyValue::Float(v) => assert!((v - 0.014).abs() < 1e-10),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_scientific_notation() {
        let src = "PLANCK = 6.626e-34\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            PyValue::Float(v) => assert!((v - 6.626e-34).abs() < 1e-44),
            _ => panic!("Expected Float"),
        }
    }

    #[test]
    fn test_string_value() {
        let src = r#"NAME = "hello world""#;
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            PyValue::Str(s) => assert_eq!(s, "hello world"),
            _ => panic!("Expected Str"),
        }
    }

    #[test]
    fn test_inline_comment() {
        let src = "SIGMA = 12          # sigma(6) = 1+2+3+6\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "SIGMA");
        assert!(matches!(consts[0].value, PyValue::Int(12)));
    }

    #[test]
    fn test_preceding_comment() {
        let src = "# sum of divisors\nSIGMA = 12\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].comment.as_deref(), Some("sum of divisors"));
    }

    #[test]
    fn test_flat_dict() {
        let src = r#"DERIVED = {'sigma': 12, 'tau': 4}"#;
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            PyValue::Dict(entries) => {
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].0, "sigma");
                assert!(matches!(entries[0].1, PyValue::Int(12)));
                assert_eq!(entries[1].0, "tau");
                assert!(matches!(entries[1].1, PyValue::Int(4)));
            }
            _ => panic!("Expected Dict"),
        }
    }

    #[test]
    fn test_class_constants() {
        let src = "class Physics:\n    SIGMA = 12\n    TAU = 4\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 2);
        assert_eq!(consts[0].name, "Physics.SIGMA");
        assert_eq!(consts[1].name, "Physics.TAU");
    }

    #[test]
    fn test_extract_numeric_constants() {
        let src = concat!(
            "SIGMA = 12\n",
            "TAU = 4\n",
            "NAME = \"test\"\n",
            "ELEMENTS = {'H': 1, 'He': 2}\n",
        );
        let pairs = extract_numeric_constants(src);
        // SIGMA=12, TAU=4, ELEMENTS.H=1, ELEMENTS.He=2
        assert_eq!(pairs.len(), 4);
        assert!(pairs.iter().any(|(n, v)| n == "SIGMA" && (*v - 12.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "ELEMENTS.H" && (*v - 1.0).abs() < 1e-10));
    }

    #[test]
    fn test_multiline_dict() {
        let src = "PHYSICS = {\n    'alpha': 137.036,\n    'mass': 1836.15,\n}\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        match &consts[0].value {
            PyValue::Dict(entries) => {
                assert_eq!(entries.len(), 2);
                assert_eq!(entries[0].0, "alpha");
            }
            _ => panic!("Expected Dict"),
        }
    }

    #[test]
    fn test_negative_number() {
        let src = "TEMP = -273\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert!(matches!(consts[0].value, PyValue::Int(-273)));
    }

    #[test]
    fn test_skips_expressions() {
        // Expressions like `math.factorial(TAU)` are not simple literals
        let src = "RESULT = math.factorial(TAU)\nSIGMA = 12\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "SIGMA");
    }

    #[test]
    fn test_skips_comparisons() {
        let src = "if x == 5:\n    pass\nSIGMA = 12\n";
        let consts = parse_py_constants(src);
        assert_eq!(consts.len(), 1);
        assert_eq!(consts[0].name, "SIGMA");
    }

    #[test]
    fn test_real_physics_file() {
        // Subset from physics_constant_engine.py
        let src = concat!(
            "# Base constants: derived from perfect number 6\n",
            "SIGMA = 12          # σ(6) = 1+2+3+6\n",
            "TAU = 4             # τ(6) = divisor count\n",
            "P1 = 6              # The perfect number itself\n",
            "M3 = 7              # Mersenne prime M3 = 2^3 - 1\n",
            "PERFECT_PAIR = 28   # Second perfect number\n",
        );
        let pairs = extract_numeric_constants(src);
        assert_eq!(pairs.len(), 5);
        assert!(pairs.iter().any(|(n, v)| n == "SIGMA" && (*v - 12.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "TAU" && (*v - 4.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "P1" && (*v - 6.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "M3" && (*v - 7.0).abs() < 1e-10));
        assert!(pairs.iter().any(|(n, v)| n == "PERFECT_PAIR" && (*v - 28.0).abs() < 1e-10));
    }

    #[test]
    fn test_empty_source() {
        assert!(parse_py_constants("").is_empty());
        assert!(extract_numeric_constants("").is_empty());
    }
}
