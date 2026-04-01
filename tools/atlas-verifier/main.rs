// atlas-verifier — N6 Atlas Constants Auto-Verifier
// Build: ~/.cargo/bin/rustc main.rs -o atlas-verifier
// Usage: ./atlas-verifier <constants.md>
//
// Parses markdown tables from atlas-constants.md, evaluates known
// n=6 expressions against claimed values, and grades each as
// EXACT / CLOSE / WEAK / FAIL.

use std::collections::HashMap;
use std::env;
use std::fs;

// ── Base constants ──────────────────────────────────────────────
const N: f64 = 6.0;
const PHI: f64 = 2.0;   // φ(6)
const TAU: f64 = 4.0;   // τ(6)
const SIGMA: f64 = 12.0; // σ(6)
const J2: f64 = 24.0;    // J₂(6)
const SOPFR: f64 = 5.0;  // sopfr(6) = 2+3
const MU: f64 = 1.0;     // μ(6)

fn build_lookup() -> HashMap<&'static str, f64> {
    let mut m = HashMap::new();

    // Base
    m.insert("σ", SIGMA);
    m.insert("τ", TAU);
    m.insert("φ", PHI);
    m.insert("sopfr", SOPFR);
    m.insert("J₂", J2);
    m.insert("μ", MU);
    m.insert("n", N);

    // Simple arithmetic
    m.insert("σ-τ", SIGMA - TAU);               // 8
    m.insert("σ-φ", SIGMA - PHI);               // 10
    m.insert("σ-μ", SIGMA - MU);                // 11
    m.insert("σ+μ", SIGMA + MU);                // 13
    m.insert("σ-sopfr", SIGMA - SOPFR);         // 7
    m.insert("J₂-τ", J2 - TAU);                 // 20
    m.insert("J₂+φ", J2 + PHI);                 // 26
    m.insert("σ+n/φ", SIGMA + N / PHI);         // 15
    m.insert("n/φ", N / PHI);                    // 3

    // Products
    m.insert("σ·τ", SIGMA * TAU);               // 48
    m.insert("σ·n", SIGMA * N);                  // 72
    m.insert("σ·φ", SIGMA * PHI);               // 24
    m.insert("σ·sopfr", SIGMA * SOPFR);         // 60
    m.insert("σ·J₂", SIGMA * J2);               // 288
    m.insert("σ·(σ-φ)", SIGMA * (SIGMA - PHI)); // 120
    m.insert("σ(σ-φ)", SIGMA * (SIGMA - PHI));  // 120 alt
    m.insert("σ·(σ-τ)", SIGMA * (SIGMA - TAU)); // 96
    m.insert("σ(σ-τ)", SIGMA * (SIGMA - TAU));  // 96 alt
    m.insert("σ·(σ-1)/2", SIGMA * (SIGMA - 1.0) / 2.0); // 66
    m.insert("σ·(σ-τ)·2^n", SIGMA * (SIGMA - TAU) * 2.0_f64.powf(N)); // 6144
    m.insert("sopfr·φ", SOPFR * PHI);           // 10
    m.insert("sopfr·(σ-φ)", SOPFR * (SIGMA - PHI)); // 50
    m.insert("(σ-φ)·sopfr", (SIGMA - PHI) * SOPFR); // 50
    m.insert("σ·τ·(σ-φ)", SIGMA * TAU * (SIGMA - PHI)); // 480
    m.insert("σ·sopfr·τ", SIGMA * SOPFR * TAU); // 240
    m.insert("J₂·(σ-φ)", J2 * (SIGMA - PHI));  // 240
    m.insert("σ·n+φ", SIGMA * N + PHI);         // 74
    m.insert("σ×σ", SIGMA * SIGMA);             // 144
    m.insert("sopfr·2^n", SOPFR * 2.0_f64.powf(N)); // 320

    // Powers
    m.insert("σ²", SIGMA * SIGMA);              // 144
    m.insert("σ³", SIGMA.powf(3.0));            // 1728
    m.insert("σ⁴", SIGMA.powf(4.0));            // 20736
    m.insert("φ^τ", PHI.powf(TAU));             // 16
    m.insert("2^n", 2.0_f64.powf(N));           // 64
    m.insert("2^σ", 2.0_f64.powf(SIGMA));       // 4096
    m.insert("2^sopfr", 2.0_f64.powf(SOPFR));   // 32
    m.insert("2^(σ-τ)", 2.0_f64.powf(SIGMA - TAU)); // 256
    m.insert("2^(σ-sopfr)", 2.0_f64.powf(SIGMA - SOPFR)); // 128
    m.insert("φⁿ = τⁿ/φ", PHI.powf(N));        // 64  (φ^n alias)
    m.insert("φⁿ", PHI.powf(N));                // 64
    m.insert("J₂²", J2 * J2);                   // 576
    m.insert("σ²·τ", SIGMA * SIGMA * TAU);      // 576

    // Combined powers
    m.insert("σ²-φ", SIGMA * SIGMA - PHI);      // 142
    m.insert("σ²-n/φ", SIGMA * SIGMA - N / PHI); // 141
    m.insert("σ²+τ", SIGMA * SIGMA + TAU);      // 148
    m.insert("σ²-σ", SIGMA * SIGMA - SIGMA);    // 132
    m.insert("φ^τ·sopfr", PHI.powf(TAU) * SOPFR); // 80
    m.insert("σ·φ^τ", SIGMA * PHI.powf(TAU));   // 192
    m.insert("(σ-φ)^τ", (SIGMA - PHI).powf(TAU)); // 10000
    m.insert("(σ-φ)²·τ", (SIGMA - PHI).powi(2) * TAU); // 400
    m.insert("(σ-φ)³", (SIGMA - PHI).powi(3));  // 1000
    m.insert("(σ-φ)^(n/φ)", (SIGMA - PHI).powf(N / PHI)); // 1000
    m.insert("(σ-φ)^{-τ}", (SIGMA - PHI).powf(-TAU)); // 1e-4
    m.insert("τ·(σ-φ)", TAU * (SIGMA - PHI));   // 40

    // Ratios
    m.insert("τ²/σ", TAU * TAU / SIGMA);        // 4/3
    m.insert("φ/τ", PHI / TAU);                  // 0.5
    m.insert("τ/(n/φ)", TAU / (N / PHI));        // 4/3
    m.insert("τ/(σ-sopfr)", TAU / (SIGMA - SOPFR)); // 4/7
    m.insert("μ/σ", MU / SIGMA);                 // 1/12
    m.insert("μ/J₂", MU / J2);                   // 1/24
    m.insert("φ²/n", PHI * PHI / N);             // 2/3
    m.insert("σ/(σ-φ)", SIGMA / (SIGMA - PHI));  // 1.2
    m.insert("(σ-μ)/(σ-φ)", (SIGMA - MU) / (SIGMA - PHI)); // 1.1
    m.insert("1/(σ-φ)", 1.0 / (SIGMA - PHI));   // 0.1
    m.insert("φ/(σ-φ)", PHI / (SIGMA - PHI));   // 0.2
    m.insert("(n/φ)/(σ-φ)", (N / PHI) / (SIGMA - PHI)); // 0.3
    m.insert("(n/φ)/(σ+μ)", (N / PHI) / (SIGMA + MU)); // 3/13
    m.insert("(n/φ)/(σ-φ)^φ", (N / PHI) / (SIGMA - PHI).powf(PHI)); // 0.03
    m.insert("sopfr/((σ-sopfr)·n)", SOPFR / ((SIGMA - SOPFR) * N)); // 5/42
    m.insert("(σ+n/φ)/φ", (SIGMA + N / PHI) / PHI); // 7.5
    m.insert("(σ+n/φ)/(σ-sopfr)", (SIGMA + N / PHI) / (SIGMA - SOPFR)); // 15/7
    m.insert("1/n", 1.0 / N);                    // 1/6
    m.insert("1/e", 1.0_f64 / std::f64::consts::E); // 0.3679
    m.insert("φ/(σ-φ)^φ", PHI / (SIGMA - PHI).powf(PHI)); // 0.02

    // Adam / LLM
    m.insert("1-1/(J₂-τ)", 1.0 - 1.0 / (J2 - TAU)); // 0.95
    m.insert("1-1/(σ-φ)", 1.0 - 1.0 / (SIGMA - PHI)); // 0.9
    m.insert("10^{-(σ-τ)}", 10.0_f64.powf(-(SIGMA - TAU))); // 1e-8
    m.insert("R(6)=σφ/(nτ)", SIGMA * PHI / (N * TAU)); // 1.0
    m.insert("R(6)", 1.0);
    m.insert("ln(4/3)", (4.0_f64 / 3.0).ln());  // 0.2877
    m.insert("τ²/(n/φ)³", TAU * TAU / (N / PHI).powi(3)); // 16/27
    m.insert("1/(σ-φ)^(n/φ)", 1.0 / (SIGMA - PHI).powf(N / PHI)); // 0.001

    // Cosmology / particle
    m.insert("1-μ/P₂", 1.0 - MU / 28.0);       // 27/28 (P₂=28)
    m.insert("σ/σ(P₂)²", SIGMA / (56.0 * 56.0)); // 12/3136 (σ(28)=56)
    m.insert("(n/φ+μ/σ)·10⁻ˢᵒᵖᶠʳ", (N / PHI + MU / SIGMA) * 10.0_f64.powf(-SOPFR));

    // σ±μ twin primes
    m.insert("σ±μ", 0.0); // special pair, skip numeric

    // Egyptian
    m.insert("σ²·σ·J₂", SIGMA * SIGMA * SIGMA * J2); // 41472
    m.insert("σ²·τ/2", SIGMA * SIGMA * TAU / 2.0); // 288
    m.insert("σ²·240W", SIGMA * SIGMA * 240.0); // 34560

    // Cross-level fine-structure
    m.insert("σ(σ-μ)+sopfr+μ/P₂", SIGMA * (SIGMA - MU) + SOPFR + MU / 28.0); // ~137.036

    // J₂ expressions
    m.insert("J₂-τ = τ·sopfr", J2 - TAU); // 20 (identity)
    m.insert("τ·sopfr", TAU * SOPFR); // 20

    m
}

/// Try to parse a numeric value from a markdown cell.
/// Handles forms like "8 = 2³", "4/3 ≈ 1.333", "10³ = 1000", "1e-8", "0.1", "288 GB", etc.
fn parse_value(raw: &str) -> Option<f64> {
    let s = raw.trim();
    if s.is_empty() {
        return None;
    }

    // Remove markdown bold
    let s = s.replace("**", "");

    // Try direct float first
    if let Ok(v) = s.parse::<f64>() {
        return Some(v);
    }

    // Strip units / trailing text: take first token that looks numeric
    // Also handle "4/3 ≈ 1.333", "10³ = 1000", "1e-8"
    // Strategy: split on common delimiters and try each token

    // Handle fractions like "4/3", "15/2", "1/2", "5/42"
    fn try_fraction(token: &str) -> Option<f64> {
        if let Some(idx) = token.find('/') {
            let num = token[..idx].trim().parse::<f64>().ok()?;
            let den = token[idx + 1..].trim().parse::<f64>().ok()?;
            if den != 0.0 {
                return Some(num / den);
            }
        }
        None
    }

    // Handle superscript numbers: 10³→1000, 10⁴→10000, 10⁻⁴→0.0001 etc.
    fn try_power_notation(token: &str) -> Option<f64> {
        let superscripts: &[(&str, f64)] = &[
            ("⁻¹⁹", -19.0), ("⁻²¹", -21.0), ("⁻⁸", -8.0), ("⁻⁵", -5.0),
            ("⁻⁴", -4.0), ("⁻³", -3.0), ("⁻²", -2.0), ("⁻¹", -1.0),
            ("¹²", 12.0), ("¹¹", 11.0), ("¹⁰", 10.0),
            ("⁹", 9.0), ("⁸", 8.0), ("⁷", 7.0), ("⁶", 6.0),
            ("⁵", 5.0), ("⁴", 4.0), ("³", 3.0), ("²", 2.0), ("¹", 1.0),
        ];
        for &(suffix, exp) in superscripts {
            if token.ends_with(suffix) {
                let base_str = &token[..token.len() - suffix.len()];
                if let Ok(base) = base_str.parse::<f64>() {
                    return Some(base.powf(exp));
                }
            }
        }
        None
    }

    // Split on ≈, =, spaces and try each token
    let delims = ['≈', '='];
    let parts: Vec<&str> = s.split(|c: char| delims.contains(&c)).collect();

    for part in &parts {
        let part = part.trim();
        // Try each whitespace-separated token
        for token in part.split_whitespace() {
            // strip trailing units
            let token = token.trim_end_matches(|c: char| c.is_alphabetic() || c == '%' || c == '/' && false);
            let clean: String = token.chars()
                .filter(|c| !['~', ',', '≈'].contains(c))
                .collect();
            let clean = clean.trim_end_matches(|c: char| {
                c.is_alphabetic() && !['e', 'E'].contains(&c)
            });

            if let Ok(v) = clean.parse::<f64>() {
                return Some(v);
            }
            if let Some(v) = try_fraction(clean) {
                return Some(v);
            }
            if let Some(v) = try_power_notation(clean) {
                return Some(v);
            }
        }
    }

    // Last resort: try the whole string as a power notation
    let cleaned: String = s.chars().filter(|c| !['~', ',', ' '].contains(c)).collect();
    if let Some(v) = try_power_notation(&cleaned) {
        return Some(v);
    }

    None
}

/// Grade based on relative error
fn grade(expected: f64, actual: f64) -> &'static str {
    if expected == 0.0 && actual == 0.0 {
        return "EXACT";
    }
    let denom = if expected.abs() > 1e-30 { expected.abs() } else { 1e-30 };
    let rel_err = ((actual - expected) / denom).abs();

    if rel_err < 0.0001 {
        "EXACT"
    } else if rel_err < 0.05 {
        "CLOSE"
    } else if rel_err < 0.20 {
        "WEAK"
    } else {
        "FAIL"
    }
}

struct VerifyResult {
    expression: String,
    expected: f64,
    actual: f64,
    grade: &'static str,
    line_num: usize,
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <constants.md>", args[0]);
        eprintln!("  Verifies n=6 atlas constants from markdown tables.");
        std::process::exit(1);
    }

    let path = &args[1];
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading {}: {}", path, e);
            std::process::exit(1);
        }
    };

    let lookup = build_lookup();
    let mut results: Vec<VerifyResult> = Vec::new();
    let mut skipped = 0usize;
    let mut total_rows = 0usize;

    for (line_idx, line) in content.lines().enumerate() {
        let line = line.trim();

        // Must be a table row with at least 3 columns
        if !line.starts_with('|') || line.starts_with("|--") || line.starts_with("|-") {
            continue;
        }

        let cols: Vec<&str> = line.split('|')
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .collect();

        if cols.len() < 2 {
            continue;
        }

        // Skip header rows (contain "Expression", "Symbol", "ID", "Statement")
        let first_lower = cols[0].to_lowercase();
        if first_lower.contains("expression") || first_lower.contains("symbol")
            || first_lower.contains("id") || first_lower.contains("statement")
            || first_lower.contains("---")
        {
            continue;
        }

        total_rows += 1;

        let expr_raw = cols[0].trim();
        let value_raw = cols[1].trim();

        // Try to look up expression
        let expected = match lookup.get(expr_raw) {
            Some(&v) => v,
            None => {
                // Try stripping surrounding whitespace/backticks
                let cleaned = expr_raw.trim_matches('`').trim();
                match lookup.get(cleaned) {
                    Some(&v) => v,
                    None => {
                        skipped += 1;
                        continue;
                    }
                }
            }
        };

        // Special: skip expressions mapped to 0 (sentinel for non-numeric entries)
        if expected == 0.0 && !value_raw.contains('0') {
            skipped += 1;
            continue;
        }

        // Parse the claimed value
        let actual = match parse_value(value_raw) {
            Some(v) => v,
            None => {
                skipped += 1;
                continue;
            }
        };

        let g = grade(expected, actual);
        results.push(VerifyResult {
            expression: expr_raw.to_string(),
            expected,
            actual,
            grade: g,
            line_num: line_idx + 1,
        });
    }

    // ── Output ──────────────────────────────────────────────
    println!("=== N6 Atlas Verifier ===");
    println!("File: {}", path);
    println!("Total table rows: {}", total_rows);
    println!("Evaluated: {}", results.len());
    println!("Skipped (unknown expr or unparseable value): {}", skipped);
    println!();

    let exact_count = results.iter().filter(|r| r.grade == "EXACT").count();
    let close_count = results.iter().filter(|r| r.grade == "CLOSE").count();
    let weak_count = results.iter().filter(|r| r.grade == "WEAK").count();
    let fail_count = results.iter().filter(|r| r.grade == "FAIL").count();

    println!("--- Summary ---");
    println!("  EXACT : {} ({:.1}%)", exact_count, 100.0 * exact_count as f64 / results.len().max(1) as f64);
    println!("  CLOSE : {} ({:.1}%)", close_count, 100.0 * close_count as f64 / results.len().max(1) as f64);
    println!("  WEAK  : {} ({:.1}%)", weak_count, 100.0 * weak_count as f64 / results.len().max(1) as f64);
    println!("  FAIL  : {} ({:.1}%)", fail_count, 100.0 * fail_count as f64 / results.len().max(1) as f64);
    println!();

    // Show all non-EXACT results
    let non_exact: Vec<&VerifyResult> = results.iter()
        .filter(|r| r.grade != "EXACT")
        .collect();

    if non_exact.is_empty() {
        println!("All {} evaluated expressions are EXACT.", results.len());
    } else {
        println!("--- Non-EXACT entries ---");
        println!("{:<6} {:<40} {:>14} {:>14} {:<6}",
            "Line", "Expression", "Expected", "Actual", "Grade");
        println!("{:-<6} {:-<40} {:-<14} {:-<14} {:-<6}", "", "", "", "", "");
        for r in &non_exact {
            println!("{:<6} {:<40} {:>14.6} {:>14.6} {:<6}",
                r.line_num, r.expression, r.expected, r.actual, r.grade);
        }
    }

    // Show failures prominently
    let failures: Vec<&VerifyResult> = results.iter()
        .filter(|r| r.grade == "FAIL")
        .collect();

    if !failures.is_empty() {
        println!();
        println!("!!! {} FAILURES detected !!!", failures.len());
        for r in &failures {
            let rel_err = if r.expected.abs() > 1e-30 {
                ((r.actual - r.expected) / r.expected).abs() * 100.0
            } else {
                f64::INFINITY
            };
            println!("  L{}: {} — expected {}, got {} (err {:.2}%)",
                r.line_num, r.expression, r.expected, r.actual, rel_err);
        }
    }

    // Exit code: 0 if no FAIL, 1 if any FAIL
    if fail_count > 0 {
        std::process::exit(1);
    }
}
