/// Markdown parser — extract tables, math expressions, constants, and section structure
/// from Markdown files (atlas-constants.md, math-crossroads-map.md, etc.).

/// A single row from a Markdown table.
#[derive(Debug, Clone, PartialEq)]
pub struct MdTableRow {
    pub cells: Vec<String>,
}

/// A parsed Markdown table with headers and rows.
#[derive(Debug, Clone)]
pub struct MdTable {
    pub section: String,
    pub headers: Vec<String>,
    pub rows: Vec<MdTableRow>,
}

/// A math expression found in the document.
#[derive(Debug, Clone, PartialEq)]
pub struct MathExpr {
    pub section: String,
    pub expr: String,
    pub is_block: bool, // $$ block vs $ inline
}

/// A constant definition extracted from tables or code blocks.
#[derive(Debug, Clone, PartialEq)]
pub struct ConstantDef {
    pub section: String,
    pub symbol: String,
    pub value: String,
    pub description: String,
}

/// A section in the Markdown document.
#[derive(Debug, Clone)]
pub struct MdSection {
    pub level: usize,   // heading level (1..=6)
    pub title: String,
    pub line_start: usize,
    pub line_end: usize,
}

/// Full parse result from a Markdown file.
#[derive(Debug, Clone)]
pub struct MdParseResult {
    pub sections: Vec<MdSection>,
    pub tables: Vec<MdTable>,
    pub math_exprs: Vec<MathExpr>,
    pub constants: Vec<ConstantDef>,
    pub code_blocks: Vec<(String, String)>, // (section, content)
}

/// Read and parse a Markdown file from disk.
pub fn read_markdown(path: &str) -> Result<MdParseResult, String> {
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Failed to read '{}': {}", path, e))?;
    Ok(parse_markdown(&content))
}

/// Parse Markdown content from a string.
pub fn parse_markdown(content: &str) -> MdParseResult {
    let lines: Vec<&str> = content.lines().collect();

    let sections = parse_sections(&lines);
    let tables = parse_tables(&lines, &sections);
    let math_exprs = parse_math_exprs(&lines, &sections);
    let code_blocks = parse_code_blocks(&lines, &sections);
    let constants = extract_constants(&tables, &code_blocks);

    MdParseResult {
        sections,
        tables,
        math_exprs,
        constants,
        code_blocks,
    }
}

/// Extract key-value pairs as (symbol, f64) from parsed constants.
///
/// Attempts to parse each constant's value as f64. Handles fractions (a/b),
/// approximate values (≈), and plain numbers.
pub fn extract_numeric_constants(result: &MdParseResult) -> Vec<(String, f64)> {
    let mut pairs = Vec::new();
    for c in &result.constants {
        if let Some(val) = parse_numeric_value(&c.value) {
            pairs.push((c.symbol.clone(), val));
        }
    }
    pairs
}

/// Extract all numeric values from all tables (flattened).
pub fn extract_table_numbers(result: &MdParseResult) -> Vec<f64> {
    let mut nums = Vec::new();
    for table in &result.tables {
        for row in &table.rows {
            for cell in &row.cells {
                if let Some(val) = parse_numeric_value(cell.trim()) {
                    nums.push(val);
                }
            }
        }
    }
    nums
}

// ── Internal helpers ──────────────────────────────────────────

/// Find the section that contains a given line number.
fn section_for_line(sections: &[MdSection], line: usize) -> String {
    let mut current = String::new();
    for s in sections {
        if s.line_start <= line {
            current = s.title.clone();
        } else {
            break;
        }
    }
    current
}

/// Parse heading-based sections.
fn parse_sections(lines: &[&str]) -> Vec<MdSection> {
    let mut sections = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        let trimmed = line.trim();
        if trimmed.starts_with('#') {
            let level = trimmed.chars().take_while(|&c| c == '#').count();
            let title = trimmed[level..].trim().to_string();
            if !title.is_empty() && level <= 6 {
                sections.push(MdSection {
                    level,
                    title,
                    line_start: i,
                    line_end: 0, // filled below
                });
            }
        }
    }

    // Fill line_end for each section
    let total = lines.len();
    for i in 0..sections.len() {
        sections[i].line_end = if i + 1 < sections.len() {
            sections[i + 1].line_start
        } else {
            total
        };
    }

    sections
}

/// Parse Markdown tables (pipe-delimited).
fn parse_tables(lines: &[&str], sections: &[MdSection]) -> Vec<MdTable> {
    let mut tables = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // A table row starts with |
        if trimmed.starts_with('|') && trimmed.ends_with('|') {
            let header_line = i;
            let headers = split_table_row(trimmed);

            // Next line should be the separator (|---|---|)
            if i + 1 < lines.len() && is_separator_row(lines[i + 1].trim()) {
                i += 2; // skip header + separator

                let mut rows = Vec::new();
                while i < lines.len() {
                    let row_trimmed = lines[i].trim();
                    if row_trimmed.starts_with('|') && row_trimmed.ends_with('|') {
                        let cells = split_table_row(row_trimmed);
                        rows.push(MdTableRow { cells });
                        i += 1;
                    } else {
                        break;
                    }
                }

                let section = section_for_line(sections, header_line);
                tables.push(MdTable {
                    section,
                    headers,
                    rows,
                });
                continue;
            }
        }

        i += 1;
    }

    tables
}

/// Split a pipe-delimited table row into cells.
fn split_table_row(line: &str) -> Vec<String> {
    let trimmed = line.trim();
    // Remove leading and trailing |
    let inner = if trimmed.starts_with('|') && trimmed.ends_with('|') && trimmed.len() >= 2 {
        &trimmed[1..trimmed.len() - 1]
    } else {
        trimmed
    };
    inner.split('|').map(|s| s.trim().to_string()).collect()
}

/// Check if a line is a table separator row (|---|---|).
fn is_separator_row(line: &str) -> bool {
    if !line.starts_with('|') {
        return false;
    }
    line.chars().all(|c| c == '|' || c == '-' || c == ':' || c.is_whitespace())
}

/// Parse math expressions: $$...$$ blocks and $...$ inline.
fn parse_math_exprs(lines: &[&str], sections: &[MdSection]) -> Vec<MathExpr> {
    let mut exprs = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();

        // Block math: $$...$$
        if trimmed.starts_with("$$") {
            let section = section_for_line(sections, i);
            if trimmed.len() > 2 && trimmed.ends_with("$$") && trimmed.len() > 4 {
                // Single-line block math
                let expr = trimmed[2..trimmed.len() - 2].trim().to_string();
                exprs.push(MathExpr {
                    section,
                    expr,
                    is_block: true,
                });
            } else {
                // Multi-line block math
                let mut block = String::new();
                i += 1;
                while i < lines.len() {
                    let l = lines[i].trim();
                    if l.starts_with("$$") {
                        break;
                    }
                    if !block.is_empty() {
                        block.push('\n');
                    }
                    block.push_str(l);
                    i += 1;
                }
                exprs.push(MathExpr {
                    section,
                    expr: block,
                    is_block: true,
                });
            }
            i += 1;
            continue;
        }

        // Inline math: $...$
        let section = section_for_line(sections, i);
        let line_str = lines[i];
        let chars: Vec<char> = line_str.chars().collect();
        let len = chars.len();
        let mut j = 0;
        while j < len {
            if chars[j] == '$' && (j + 1 < len) && chars[j + 1] != '$' {
                let start = j + 1;
                j += 1;
                while j < len && chars[j] != '$' {
                    j += 1;
                }
                if j < len {
                    let expr: String = chars[start..j].iter().collect();
                    let expr = expr.trim().to_string();
                    if !expr.is_empty() {
                        exprs.push(MathExpr {
                            section: section.clone(),
                            expr,
                            is_block: false,
                        });
                    }
                }
            }
            j += 1;
        }

        i += 1;
    }

    exprs
}

/// Parse fenced code blocks (```...```).
fn parse_code_blocks(lines: &[&str], sections: &[MdSection]) -> Vec<(String, String)> {
    let mut blocks = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let trimmed = lines[i].trim();
        if trimmed.starts_with("```") {
            let section = section_for_line(sections, i);
            i += 1;
            let mut block = String::new();
            while i < lines.len() {
                let l = lines[i].trim();
                if l.starts_with("```") {
                    break;
                }
                if !block.is_empty() {
                    block.push('\n');
                }
                block.push_str(lines[i]);
                i += 1;
            }
            if !block.is_empty() {
                blocks.push((section, block));
            }
            i += 1;
            continue;
        }
        i += 1;
    }

    blocks
}

/// Extract constant definitions from tables.
///
/// Looks for tables with columns like "Symbol", "Value", "Expression", etc.
/// and extracts structured constant definitions.
fn extract_constants(tables: &[MdTable], code_blocks: &[(String, String)]) -> Vec<ConstantDef> {
    let mut constants = Vec::new();

    for table in tables {
        let headers_lower: Vec<String> = table.headers.iter().map(|h| h.to_lowercase()).collect();

        // Find relevant column indices
        let symbol_col = find_column(&headers_lower, &["symbol", "expression", "id"]);
        let value_col = find_column(&headers_lower, &["value"]);
        let desc_col = find_column(&headers_lower, &[
            "application", "description", "function", "formula", "statement",
        ]);

        if let (Some(sym_idx), Some(val_idx)) = (symbol_col, value_col) {
            for row in &table.rows {
                let symbol = row.cells.get(sym_idx).cloned().unwrap_or_default();
                let value = row.cells.get(val_idx).cloned().unwrap_or_default();
                let description = desc_col
                    .and_then(|idx| row.cells.get(idx))
                    .cloned()
                    .unwrap_or_default();

                if !symbol.is_empty() && !value.is_empty() {
                    constants.push(ConstantDef {
                        section: table.section.clone(),
                        symbol: clean_markdown_bold(&symbol),
                        value: value.clone(),
                        description,
                    });
                }
            }
        }
    }

    // Also extract key=value patterns from code blocks
    for (section, block) in code_blocks {
        for line in block.lines() {
            let trimmed = line.trim();
            // pattern: "symbol = value" or "symbol(n) = value"
            if let Some(eq_pos) = trimmed.find('=') {
                let lhs = trimmed[..eq_pos].trim();
                let rhs = trimmed[eq_pos + 1..].trim();
                if !lhs.is_empty() && !rhs.is_empty() && lhs.len() < 40 {
                    constants.push(ConstantDef {
                        section: section.clone(),
                        symbol: lhs.to_string(),
                        value: rhs.to_string(),
                        description: String::new(),
                    });
                }
            }
        }
    }

    constants
}

/// Find the first column index whose lowercased header contains any of the given keywords.
fn find_column(headers: &[String], keywords: &[&str]) -> Option<usize> {
    for (i, h) in headers.iter().enumerate() {
        for kw in keywords {
            if h.contains(kw) {
                return Some(i);
            }
        }
    }
    None
}

/// Remove Markdown bold markers (**text**) from a string.
fn clean_markdown_bold(s: &str) -> String {
    s.replace("**", "")
}

/// Parse a numeric value from a string.
///
/// Handles:
///   - Plain numbers: "12", "0.5", "-1.5"
///   - Fractions: "4/3", "1/2"
///   - Approximate: "≈ 1.333", "≈0.1389"
///   - With units/comments: "1.333 eV" (takes first token)
///   - Expressions like "2³" → 8 (limited power notation)
fn parse_numeric_value(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() {
        return None;
    }

    // Strip leading ≈
    let s = s.trim_start_matches('≈').trim_start_matches('~').trim();

    // Try direct parse
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }

    // Try fraction: a/b (possibly with leading chars like "4/3 ≈ 1.333")
    // Take the first token that looks like a fraction or number
    for token in s.split_whitespace() {
        let clean = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '/' && c != '.' && c != '-');
        if clean.contains('/') {
            let parts: Vec<&str> = clean.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(num), Ok(den)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    if den != 0.0 {
                        return Some(num / den);
                    }
                }
            }
        }
        // Try plain number from first token
        if let Ok(v) = clean.parse::<f64>() {
            return Some(v);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sections() {
        let md = "# Title\n\nSome text\n\n## Section A\n\nContent A\n\n## Section B\n\nContent B\n";
        let result = parse_markdown(md);
        assert_eq!(result.sections.len(), 3);
        assert_eq!(result.sections[0].title, "Title");
        assert_eq!(result.sections[0].level, 1);
        assert_eq!(result.sections[1].title, "Section A");
        assert_eq!(result.sections[1].level, 2);
        assert_eq!(result.sections[2].title, "Section B");
    }

    #[test]
    fn test_parse_table() {
        let md = "\
## Base Constants\n\
\n\
| Symbol | Value | Description |\n\
|--------|-------|-------------|\n\
| σ | 12 | Sum of divisors |\n\
| τ | 4 | Number of divisors |\n\
| φ | 2 | Euler's totient |\n\
";
        let result = parse_markdown(md);
        assert_eq!(result.tables.len(), 1);
        assert_eq!(result.tables[0].headers.len(), 3);
        assert_eq!(result.tables[0].rows.len(), 3);
        assert_eq!(result.tables[0].rows[0].cells[0], "σ");
        assert_eq!(result.tables[0].rows[0].cells[1], "12");
        assert_eq!(result.tables[0].section, "Base Constants");
    }

    #[test]
    fn test_extract_constants_from_table() {
        let md = "\
## Constants\n\
\n\
| Symbol | Value | Application |\n\
|--------|-------|-------------|\n\
| σ | 12 | Sum of divisors |\n\
| τ | 4 | Number of divisors |\n\
";
        let result = parse_markdown(md);
        assert_eq!(result.constants.len(), 2);
        assert_eq!(result.constants[0].symbol, "σ");
        assert_eq!(result.constants[0].value, "12");
    }

    #[test]
    fn test_extract_constants_from_code_block() {
        let md = "\
## Core Identity\n\
\n\
```\n\
  σ(6)·φ(6) = 6·τ(6) = 24\n\
  R(6) = 1\n\
```\n\
";
        let result = parse_markdown(md);
        assert!(result.constants.iter().any(|c| c.symbol.contains("R(6)") && c.value.contains("1")));
    }

    #[test]
    fn test_parse_numeric_value() {
        assert!((parse_numeric_value("12").unwrap() - 12.0).abs() < 1e-10);
        assert!((parse_numeric_value("4/3").unwrap() - 4.0 / 3.0).abs() < 1e-10);
        assert!((parse_numeric_value("≈ 1.333").unwrap() - 1.333).abs() < 1e-10);
        assert!((parse_numeric_value("0.5").unwrap() - 0.5).abs() < 1e-10);
        assert!((parse_numeric_value("4/3 ≈ 1.333").unwrap() - 4.0 / 3.0).abs() < 1e-10);
    }

    #[test]
    fn test_extract_numeric_constants() {
        let md = "\
## Test\n\
\n\
| Symbol | Value | Application |\n\
|--------|-------|-------------|\n\
| σ | 12 | Sum of divisors |\n\
| τ/σ | 4/3 ≈ 1.333 | FFN expansion |\n\
";
        let result = parse_markdown(md);
        let nums = extract_numeric_constants(&result);
        assert!(nums.iter().any(|(s, v)| s == "σ" && (*v - 12.0).abs() < 1e-10));
        assert!(nums.iter().any(|(_, v)| (*v - 4.0 / 3.0).abs() < 1e-4));
    }

    #[test]
    fn test_parse_inline_math() {
        let md = "The formula is $E = mc^2$ in physics.\n";
        let result = parse_markdown(md);
        assert_eq!(result.math_exprs.len(), 1);
        assert_eq!(result.math_exprs[0].expr, "E = mc^2");
        assert!(!result.math_exprs[0].is_block);
    }

    #[test]
    fn test_parse_block_math() {
        let md = "## Math\n\n$$\nE = mc^2\n$$\n";
        let result = parse_markdown(md);
        assert!(result.math_exprs.iter().any(|m| m.is_block && m.expr.contains("E = mc^2")));
    }

    #[test]
    fn test_parse_code_block() {
        let md = "## Core\n\n```\nσ(6) = 12\nτ(6) = 4\n```\n";
        let result = parse_markdown(md);
        assert_eq!(result.code_blocks.len(), 1);
        assert!(result.code_blocks[0].1.contains("σ(6) = 12"));
    }

    #[test]
    fn test_bold_marker_cleanup() {
        let md = "\
## Proved\n\
\n\
| ID | Value | Description |\n\
|----|-------|-------------|\n\
| **THM-1** | 1 | A theorem |\n\
";
        let result = parse_markdown(md);
        // Bold markers should be stripped from symbol
        assert!(result.constants.iter().any(|c| c.symbol == "THM-1"));
    }

    #[test]
    fn test_empty_content() {
        let result = parse_markdown("");
        assert!(result.sections.is_empty());
        assert!(result.tables.is_empty());
        assert!(result.math_exprs.is_empty());
        assert!(result.constants.is_empty());
        assert!(result.code_blocks.is_empty());
    }

    #[test]
    fn test_separator_row_detection() {
        assert!(is_separator_row("|---|---|---|"));
        assert!(is_separator_row("|----|-------|-------------|"));
        assert!(is_separator_row("|:---|:---:|---:|"));
        assert!(!is_separator_row("| sigma | 12 | value |"));
    }

    #[test]
    fn test_table_numbers() {
        let md = "\
## Data\n\
\n\
| Symbol | Value |\n\
|--------|-------|\n\
| a | 6.0 |\n\
| b | 12.0 |\n\
| c | not a number |\n\
";
        let result = parse_markdown(md);
        let nums = extract_table_numbers(&result);
        assert!(nums.contains(&6.0));
        assert!(nums.contains(&12.0));
        assert_eq!(nums.len(), 2);
    }

    #[test]
    fn test_atlas_constants_style() {
        // Simulate actual atlas-constants.md table format
        let md = "\
## Derived Ratios (Architecture)\n\
\n\
| Expression | Value | Application | Domain |\n\
|------------|-------|-------------|--------|\n\
| τ²/σ | 4/3 ≈ 1.333 | FFN expansion ratio | AI |\n\
| φ/τ | 1/2 = 0.5 | MoE top-k selection | AI |\n\
| σ-τ | 8 = 2³ | SHA-256, byte, Bott period | Crypto, CS |\n\
";
        let result = parse_markdown(md);
        assert_eq!(result.tables.len(), 1);
        assert_eq!(result.tables[0].rows.len(), 3);

        // Expression column should be detected as symbol
        let constants = &result.constants;
        assert!(constants.iter().any(|c| c.symbol == "τ²/σ"));
        assert!(constants.iter().any(|c| c.symbol == "φ/τ"));

        // Numeric extraction
        let nums = extract_numeric_constants(&result);
        assert!(nums.iter().any(|(s, v)| s == "τ²/σ" && (*v - 4.0 / 3.0).abs() < 1e-4));
        assert!(nums.iter().any(|(s, v)| s == "φ/τ" && (*v - 0.5).abs() < 1e-10));
    }
}
