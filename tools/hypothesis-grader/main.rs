// hypothesis-grader: High-speed n=6 hypothesis grading for TECS-L .md files
// Build: ~/.cargo/bin/rustc main.rs -o hypothesis-grader
// Usage: ./hypothesis-grader <dir> [--min-score N] [--csv output.csv]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

// ── n=6 constant table ──────────────────────────────────────────────────────

struct N6Constant {
    name: &'static str,
    value: f64,
    is_float: bool,
    tolerance: f64, // 0.0 for exact integer match
}

fn n6_constants() -> Vec<N6Constant> {
    vec![
        // Integers (standalone numbers to search for)
        N6Constant { name: "phi=2",              value: 2.0,    is_float: false, tolerance: 0.0 },
        N6Constant { name: "n/phi=3",            value: 3.0,    is_float: false, tolerance: 0.0 },
        N6Constant { name: "tau=4",              value: 4.0,    is_float: false, tolerance: 0.0 },
        N6Constant { name: "sopfr=5",            value: 5.0,    is_float: false, tolerance: 0.0 },
        N6Constant { name: "n=6",                value: 6.0,    is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma-tau=8",        value: 8.0,    is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma-phi=10",       value: 10.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma-mu=11",        value: 11.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma=12",           value: 12.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma+mu=13",        value: 13.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "J2=24",              value: 24.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma*tau=48",        value: 48.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "phi^n=64",           value: 64.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma*n=72",         value: 72.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "96=sigma*(sigma-tau)",value: 96.0,   is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma*(sigma-phi)=120",value:120.0,  is_float: false, tolerance: 0.0 },
        N6Constant { name: "2^(sigma-sopfr)=128", value: 128.0,  is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma^2=144",        value: 144.0,  is_float: false, tolerance: 0.0 },
        N6Constant { name: "192=sigma*2^tau",    value: 192.0,  is_float: false, tolerance: 0.0 },
        N6Constant { name: "2^(sigma-tau)=256",  value: 256.0,  is_float: false, tolerance: 0.0 },
        N6Constant { name: "sigma*J2=288",       value: 288.0,  is_float: false, tolerance: 0.0 },
        N6Constant { name: "2^sigma=4096",       value: 4096.0, is_float: false, tolerance: 0.0 },
        // Floats (approximate matching)
        N6Constant { name: "1/(sigma-phi)=0.1",  value: 0.1,    is_float: true, tolerance: 0.005 },
        N6Constant { name: "phi/sigma=0.167",    value: 0.167,  is_float: true, tolerance: 0.005 },
        N6Constant { name: "1/n=0.167",          value: 0.1667, is_float: true, tolerance: 0.005 },
        N6Constant { name: "1/sopfr=0.2",        value: 0.2,    is_float: true, tolerance: 0.005 },
        N6Constant { name: "ln(4/3)=0.288",      value: 0.288,  is_float: true, tolerance: 0.02  },
        N6Constant { name: "1/n/phi=0.333",      value: 0.333,  is_float: true, tolerance: 0.005 },
        N6Constant { name: "1/e=0.368",          value: 0.368,  is_float: true, tolerance: 0.03  },
        N6Constant { name: "phi/tau=0.5",        value: 0.5,    is_float: true, tolerance: 0.0   },
        N6Constant { name: "Betz=0.593",         value: 0.593,  is_float: true, tolerance: 0.01  },
        N6Constant { name: "1-1/e=0.625",        value: 0.625,  is_float: true, tolerance: 0.01  },
        N6Constant { name: "phi/n/phi=0.667",    value: 0.667,  is_float: true, tolerance: 0.005 },
        N6Constant { name: "sigma/(sigma-phi)=1.2", value: 1.2, is_float: true, tolerance: 0.01  },
        N6Constant { name: "tau/n/phi=1.333",    value: 1.333,  is_float: true, tolerance: 0.01  },
    ]
}

// Small integers that are too common to count as standalone matches
fn is_small_int_skip(val: f64) -> bool {
    let iv = val as i64;
    iv >= 2 && iv <= 4 && (val - iv as f64).abs() < 1e-9
}

// ── n=6 keywords ────────────────────────────────────────────────────────────

const N6_KEYWORDS: &[&str] = &[
    "sigma",
    "tau",
    "divisor",
    "totient",
    "sopfr",
    "egyptian",
    "perfect number",
    "n=6",
    "n = 6",
    "leech",
    "jordan",
    "carmichael",
    "dedekind",
    "mobius",
    "möbius",
    "j_2",
    "j₂",
    "1/2+1/3+1/6",
    "egyptian fraction",
];

// ── Number extraction ───────────────────────────────────────────────────────

/// Check if byte is a word-boundary character (not alphanumeric, not '_', '/', '.', '\\')
fn is_boundary(b: u8) -> bool {
    !b.is_ascii_alphanumeric() && b != b'_' && b != b'/' && b != b'\\' && b != b'%'
}

/// Check left boundary: either start of string or a boundary char (also not '.')
fn is_left_boundary(text: &[u8], pos: usize) -> bool {
    if pos == 0 {
        return true;
    }
    let b = text[pos - 1];
    is_boundary(b) && b != b'.'
}

/// Check right boundary: either end of string or a boundary char
fn is_right_boundary(text: &[u8], pos: usize) -> bool {
    if pos >= text.len() {
        return true;
    }
    let b = text[pos];
    is_boundary(b) && b != b'.'
}

/// Extract all standalone numbers from text.
fn extract_numbers(text: &str) -> Vec<f64> {
    let bytes = text.as_bytes();
    let len = bytes.len();
    let mut numbers = Vec::new();
    let mut i = 0;

    while i < len {
        // Skip non-digit characters
        if !bytes[i].is_ascii_digit() {
            i += 1;
            continue;
        }

        // Check left boundary
        if !is_left_boundary(bytes, i) {
            // Skip past digits
            while i < len && (bytes[i].is_ascii_digit() || bytes[i] == b'.') {
                i += 1;
            }
            continue;
        }

        // Collect the number substring
        let start = i;
        let mut has_dot = false;
        while i < len && (bytes[i].is_ascii_digit() || (bytes[i] == b'.' && !has_dot)) {
            if bytes[i] == b'.' {
                has_dot = true;
            }
            i += 1;
        }

        // Trailing dot is not part of number
        if has_dot && i > start && bytes[i - 1] == b'.' {
            i -= 1;
            // Reconsider: single digit followed by dot → keep digit only
        }

        // Check right boundary
        if !is_right_boundary(bytes, i) {
            continue;
        }

        let num_str = &text[start..i];
        if let Ok(val) = num_str.parse::<f64>() {
            numbers.push(val);
        }
    }

    numbers
}

// ── Scoring ─────────────────────────────────────────────────────────────────

struct FileResult {
    path: PathBuf,
    filename: String,
    score: u32,
    unique_constants: u32,
    keyword_count: u32,
    matches: Vec<(String, u32)>, // (constant_name, count)
    grade: &'static str,
}

fn count_keywords(text_lower: &str) -> u32 {
    let mut count: u32 = 0;
    for kw in N6_KEYWORDS {
        // Count non-overlapping occurrences
        let mut start = 0;
        while let Some(pos) = text_lower[start..].find(kw) {
            count += 1;
            start += pos + kw.len();
        }
    }
    count
}

fn grade_file(text: &str, constants: &[N6Constant]) -> FileResult {
    let numbers = extract_numbers(text);
    let text_lower = text.to_lowercase();
    let keyword_count = count_keywords(&text_lower);

    // Match constants
    let mut matched: HashMap<&str, u32> = HashMap::new();

    for c in constants {
        if !c.is_float && is_small_int_skip(c.value) {
            // Skip small integers (2, 3, 4) — too many false positives
            continue;
        }

        let mut hit_count: u32 = 0;

        if c.tolerance == 0.0 {
            // Exact match
            for &n in &numbers {
                if (n - c.value).abs() < 1e-9 {
                    hit_count += 1;
                }
            }
        } else {
            // Approximate match
            for &n in &numbers {
                if (n - c.value).abs() <= c.tolerance {
                    hit_count += 1;
                }
            }
        }

        if hit_count > 0 {
            matched.insert(c.name, hit_count);
        }
    }

    let unique_constants = matched.len() as u32;
    // Score formula: unique_constants * 10 + keyword_count
    let score = unique_constants * 10 + keyword_count;

    let grade = if score >= 50 {
        "STRONG"
    } else if score >= 20 {
        "MODERATE"
    } else if score >= 1 {
        "WEAK"
    } else {
        "NONE"
    };

    let mut match_vec: Vec<(String, u32)> = matched
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    match_vec.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.cmp(&b.0)));

    FileResult {
        path: PathBuf::new(),
        filename: String::new(),
        score,
        unique_constants,
        keyword_count,
        matches: match_vec,
        grade,
    }
}

// ── Directory scanning ──────────────────────────────────────────────────────

fn scan_md_files(dir: &Path) -> Vec<PathBuf> {
    let mut files = Vec::new();
    if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                // Recurse into subdirectories
                files.extend(scan_md_files(&path));
            } else if let Some(ext) = path.extension() {
                if ext == "md" {
                    files.push(path);
                }
            }
        }
    }
    files.sort();
    files
}

// ── CSV output ──────────────────────────────────────────────────────────────

fn write_csv(results: &[FileResult], csv_path: &Path) -> std::io::Result<()> {
    let mut f = fs::File::create(csv_path)?;
    writeln!(f, "file,score,unique_constants,keyword_hits,grade,top_matches")?;
    for r in results {
        let top: Vec<String> = r.matches.iter().take(8).map(|(k, v)| format!("{}({})", k, v)).collect();
        let top_str = top.join("; ");
        // Escape commas in match string
        writeln!(
            f,
            "{},{},{},{},{},\"{}\"",
            r.filename, r.score, r.unique_constants, r.keyword_count, r.grade, top_str
        )?;
    }
    Ok(())
}

// ── Main ────────────────────────────────────────────────────────────────────

fn print_usage() {
    eprintln!("hypothesis-grader: High-speed n=6 hypothesis grading");
    eprintln!();
    eprintln!("Usage:");
    eprintln!("  hypothesis-grader <hypothesis_dir> [--min-score N] [--csv output.csv]");
    eprintln!();
    eprintln!("Examples:");
    eprintln!("  hypothesis-grader ~/Dev/TECS-L/docs/hypotheses/");
    eprintln!("  hypothesis-grader ./docs/ --min-score 20 --csv results.csv");
    eprintln!();
    eprintln!("Scoring:");
    eprintln!("  score = (unique_n6_constants * 10) + keyword_count");
    eprintln!("  STRONG >= 50 | MODERATE 20-49 | WEAK 1-19 | NONE 0");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        print_usage();
        std::process::exit(1);
    }

    // Parse arguments
    let mut dir_path: Option<PathBuf> = None;
    let mut min_score: u32 = 0;
    let mut csv_path: Option<PathBuf> = None;

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--min-score" => {
                i += 1;
                if i < args.len() {
                    min_score = args[i].parse().unwrap_or(0);
                }
            }
            "--csv" => {
                i += 1;
                if i < args.len() {
                    csv_path = Some(PathBuf::from(&args[i]));
                }
            }
            "--help" | "-h" => {
                print_usage();
                std::process::exit(0);
            }
            _ => {
                if dir_path.is_none() {
                    dir_path = Some(PathBuf::from(&args[i]));
                } else {
                    eprintln!("Error: unexpected argument '{}'", args[i]);
                    std::process::exit(1);
                }
            }
        }
        i += 1;
    }

    let dir = match dir_path {
        Some(d) => d,
        None => {
            eprintln!("Error: no directory specified");
            print_usage();
            std::process::exit(1);
        }
    };

    if !dir.is_dir() {
        eprintln!("Error: '{}' is not a directory", dir.display());
        std::process::exit(1);
    }

    // Scan files
    let files = scan_md_files(&dir);
    println!("Scanning: {}", dir.display());
    println!("Found {} .md files", files.len());
    println!();

    if files.is_empty() {
        println!("No .md files found.");
        return;
    }

    // Grade each file
    let constants = n6_constants();
    let mut results: Vec<FileResult> = Vec::with_capacity(files.len());

    for path in &files {
        let text = match fs::read_to_string(path) {
            Ok(t) => t,
            Err(_) => {
                // Try reading as lossy UTF-8
                match fs::read(path) {
                    Ok(bytes) => String::from_utf8_lossy(&bytes).to_string(),
                    Err(e) => {
                        eprintln!("  skip {}: {}", path.display(), e);
                        continue;
                    }
                }
            }
        };

        let mut result = grade_file(&text, &constants);
        result.filename = path
            .file_name()
            .map(|n| n.to_string_lossy().to_string())
            .unwrap_or_default();
        result.path = path.clone();
        results.push(result);
    }

    // Sort by score descending, then by unique constants, then filename
    results.sort_by(|a, b| {
        b.score
            .cmp(&a.score)
            .then(b.unique_constants.cmp(&a.unique_constants))
            .then(a.filename.cmp(&b.filename))
    });

    // ── Summary statistics ──────────────────────────────────────────────

    let total = results.len();
    let mut grade_counts: HashMap<&str, usize> = HashMap::new();
    for r in &results {
        *grade_counts.entry(r.grade).or_insert(0) += 1;
    }

    println!("{}", "=".repeat(90));
    println!("RESULTS: {} files graded", total);
    println!(
        "  STRONG  (>=50): {:>5}",
        grade_counts.get("STRONG").unwrap_or(&0)
    );
    println!(
        "  MODERATE(20-49): {:>4}",
        grade_counts.get("MODERATE").unwrap_or(&0)
    );
    println!(
        "  WEAK    (1-19): {:>5}",
        grade_counts.get("WEAK").unwrap_or(&0)
    );
    println!(
        "  NONE    (0):    {:>5}",
        grade_counts.get("NONE").unwrap_or(&0)
    );
    println!("{}", "=".repeat(90));

    // ── Score distribution histogram ────────────────────────────────────

    let mut buckets = [0u32; 6]; // 0, 1-9, 10-19, 20-49, 50-99, 100+
    for r in &results {
        match r.score {
            0 => buckets[0] += 1,
            1..=9 => buckets[1] += 1,
            10..=19 => buckets[2] += 1,
            20..=49 => buckets[3] += 1,
            50..=99 => buckets[4] += 1,
            _ => buckets[5] += 1,
        }
    }
    println!();
    println!("Score Distribution:");
    let labels = ["    0", "  1-9", "10-19", "20-49", "50-99", " 100+"];
    let max_bar = 60u32;
    let max_count = *buckets.iter().max().unwrap_or(&1);
    for (label, &count) in labels.iter().zip(buckets.iter()) {
        let bar_len = if max_count > 0 {
            (count as u64 * max_bar as u64 / max_count as u64) as usize
        } else {
            0
        };
        let bar: String = "#".repeat(bar_len);
        println!("  {}: {:5} {}", label, count, bar);
    }

    // ── Most common constants across all files ──────────────────────────

    let mut const_freq: HashMap<String, u32> = HashMap::new();
    for r in &results {
        for (name, count) in &r.matches {
            *const_freq.entry(name.clone()).or_insert(0) += count;
        }
    }
    let mut freq_vec: Vec<(String, u32)> = const_freq.into_iter().collect();
    freq_vec.sort_by(|a, b| b.1.cmp(&a.1));

    println!();
    println!("Most Common n=6 Constants:");
    for (name, count) in freq_vec.iter().take(20) {
        println!("  {:<35} {:>6} hits", name, count);
    }

    // ── Filtered results table ──────────────────────────────────────────

    let filtered: Vec<&FileResult> = results.iter().filter(|r| r.score >= min_score).collect();

    println!();
    println!("{}", "=".repeat(90));
    if min_score > 0 {
        println!(
            "FILES WITH SCORE >= {} ({}/{})",
            min_score,
            filtered.len(),
            total
        );
    } else {
        println!("ALL FILES RANKED BY n=6 SCORE ({} total)", total);
    }
    println!("{}", "=".repeat(90));
    println!(
        "{:>4} {:>5} {:>4} {:>4} {:<8} {:<45} {}",
        "#", "Score", "Uniq", "KW", "Grade", "File", "Top Matches"
    );
    println!("{}", "-".repeat(130));

    // Show up to 100 entries
    let show_count = filtered.len().min(100);
    for (idx, r) in filtered.iter().take(show_count).enumerate() {
        let top: Vec<String> = r
            .matches
            .iter()
            .take(5)
            .map(|(k, v)| format!("{}({})", k, v))
            .collect();
        let top_str = top.join(", ");
        println!(
            "{:4} {:5} {:4} {:4} {:<8} {:<45} {}",
            idx + 1,
            r.score,
            r.unique_constants,
            r.keyword_count,
            r.grade,
            if r.filename.len() > 44 {
                format!("{}...", &r.filename[..41])
            } else {
                r.filename.clone()
            },
            top_str
        );
    }

    if filtered.len() > show_count {
        println!("  ... and {} more files", filtered.len() - show_count);
    }

    // ── CSV output ──────────────────────────────────────────────────────

    if let Some(ref csv) = csv_path {
        match write_csv(&results, csv) {
            Ok(()) => println!("\nCSV written to: {}", csv.display()),
            Err(e) => eprintln!("\nError writing CSV: {}", e),
        }
    }

    // ── Final summary line ──────────────────────────────────────────────

    println!();
    println!(
        "Done. {} files | STRONG {} | MODERATE {} | WEAK {} | NONE {}",
        total,
        grade_counts.get("STRONG").unwrap_or(&0),
        grade_counts.get("MODERATE").unwrap_or(&0),
        grade_counts.get("WEAK").unwrap_or(&0),
        grade_counts.get("NONE").unwrap_or(&0),
    );
}

// ── Tests ───────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_numbers_basic() {
        let nums = extract_numbers("The value is 12 and 0.288 here");
        assert!(nums.contains(&12.0));
        assert!(nums.contains(&0.288));
    }

    #[test]
    fn test_extract_numbers_boundary() {
        // Should NOT match numbers inside paths or identifiers
        let nums = extract_numbers("file_v12 path/12/foo 12px");
        // "file_v12" — preceded by _, skip
        // "path/12/foo" — preceded by /, skip
        // "12px" — followed by letter, skip
        assert!(nums.is_empty(), "got: {:?}", nums);
    }

    #[test]
    fn test_extract_numbers_standalone() {
        let nums = extract_numbers("sigma=12, tau=4, value (144) end");
        assert!(nums.contains(&12.0));
        assert!(nums.contains(&4.0));
        assert!(nums.contains(&144.0));
    }

    #[test]
    fn test_grade_strong() {
        let text = "sigma=12, J2=24, tau=4, sopfr=5, n=6, phi=2, 144 SM, 48kHz, 72 cells, 120 factor, 64 codebook. Keywords: sigma sigma divisor totient perfect number n=6 egyptian leech jordan carmichael";
        let constants = n6_constants();
        let result = grade_file(text, &constants);
        assert_eq!(result.grade, "STRONG", "score={}", result.score);
    }

    #[test]
    fn test_grade_none() {
        let text = "This is a plain text with no special numbers or keywords.";
        let constants = n6_constants();
        let result = grade_file(text, &constants);
        assert_eq!(result.grade, "NONE");
        assert_eq!(result.score, 0);
    }

    #[test]
    fn test_keyword_counting() {
        let text = "sigma and tau and sigma again and perfect number here";
        let lower = text.to_lowercase();
        let count = count_keywords(&lower);
        // "sigma" x2, "tau" x1, "perfect number" x1 = 4
        assert_eq!(count, 4);
    }

    #[test]
    fn test_float_matching() {
        let text = "dropout rate 0.288 and efficiency 1.2 and sparsity 0.368";
        let constants = n6_constants();
        let result = grade_file(text, &constants);
        assert!(result.unique_constants >= 3, "unique={}", result.unique_constants);
    }
}
