// Cross-DSE Pair Analysis Calculator
// Reads all .toml domain files, computes cross-compatibility scores for every pair.
// Build: ~/.cargo/bin/rustc main.rs -o cross-dse-calc
// Usage: ./cross-dse-calc <domains_dir> [--top N] [--csv output.csv]

use std::collections::HashMap;
use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

// ---------- Data structures ----------

#[derive(Debug, Clone)]
struct Domain {
    name: String,
    levels: Vec<Level>,
}

#[derive(Debug, Clone)]
struct Level {
    name: String,
    candidates: Vec<Candidate>,
}

#[derive(Debug, Clone)]
struct Candidate {
    id: String,
    n6: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

#[derive(Debug, Clone)]
struct PairResult {
    a: String,
    b: String,
    cross_score: f64,
    shared_levels: usize,
    shared_n6_constants: Vec<String>,
}

// ---------- TOML parser (manual, no crates) ----------

fn parse_toml(content: &str) -> Domain {
    let mut name = String::new();
    let mut levels: Vec<Level> = Vec::new();
    let mut current_level: Option<Level> = None;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // [meta] section — look for name
        if line == "[meta]" {
            continue;
        }
        if line == "[scoring]" {
            continue;
        }

        // [[level]] starts a new level
        if line == "[[level]]" {
            if let Some(lv) = current_level.take() {
                levels.push(lv);
            }
            current_level = Some(Level {
                name: String::new(),
                candidates: Vec::new(),
            });
            continue;
        }

        // [[candidate]] starts a new candidate in current level
        if line == "[[candidate]]" {
            if let Some(ref mut lv) = current_level {
                lv.candidates.push(Candidate {
                    id: String::new(),
                    n6: 0.0,
                    perf: 0.0,
                    power: 0.0,
                    cost: 0.0,
                });
            }
            continue;
        }

        // [[rule]] — skip
        if line == "[[rule]]" || line.starts_with("[[rule") {
            continue;
        }

        // Key = value parsing
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let val = line[eq_pos + 1..].trim();

            // Are we inside a level (after [[level]] and possibly after [[candidate]])?
            if let Some(ref mut lv) = current_level {
                if lv.candidates.is_empty() {
                    // Level-level keys
                    if key == "name" {
                        lv.name = strip_quotes(val);
                    }
                } else {
                    // Candidate-level keys
                    let cand = lv.candidates.last_mut().unwrap();
                    match key {
                        "id" => cand.id = strip_quotes(val),
                        "n6" => cand.n6 = parse_f64(val),
                        "perf" => cand.perf = parse_f64(val),
                        "power" => cand.power = parse_f64(val),
                        "cost" => cand.cost = parse_f64(val),
                        _ => {}
                    }
                }
            } else {
                // Top-level keys (meta section)
                if key == "name" {
                    name = strip_quotes(val);
                }
            }
        }
    }

    // Push last level
    if let Some(lv) = current_level {
        levels.push(lv);
    }

    // Fallback name from empty
    if name.is_empty() {
        name = "unknown".to_string();
    }

    Domain { name, levels }
}

fn strip_quotes(s: &str) -> String {
    let s = s.trim();
    if (s.starts_with('"') && s.ends_with('"')) || (s.starts_with('\'') && s.ends_with('\'')) {
        s[1..s.len() - 1].to_string()
    } else {
        s.to_string()
    }
}

fn parse_f64(s: &str) -> f64 {
    let s = s.trim();
    // Strip trailing comments
    let s = if let Some(pos) = s.find('#') {
        s[..pos].trim()
    } else {
        s
    };
    s.parse::<f64>().unwrap_or(0.0)
}

// ---------- Analysis ----------

fn best_n6_per_level(domain: &Domain) -> Vec<f64> {
    domain
        .levels
        .iter()
        .map(|lv| {
            lv.candidates
                .iter()
                .map(|c| c.n6)
                .fold(0.0_f64, f64::max)
        })
        .collect()
}

fn best_candidates_per_level(domain: &Domain) -> Vec<&Candidate> {
    domain
        .levels
        .iter()
        .filter_map(|lv| lv.candidates.iter().max_by(|a, b| a.n6.partial_cmp(&b.n6).unwrap()))
        .collect()
}

/// Cross-score = geometric mean of (best_n6_A[i] * best_n6_B[i]) for min(levelsA, levelsB) levels
fn compute_cross_score(a: &Domain, b: &Domain) -> PairResult {
    let n6_a = best_n6_per_level(a);
    let n6_b = best_n6_per_level(b);
    let shared = n6_a.len().min(n6_b.len());

    let cross_score = if shared == 0 {
        0.0
    } else {
        // Geometric mean of products
        let log_sum: f64 = (0..shared)
            .map(|i| {
                let product = n6_a[i] * n6_b[i];
                if product > 0.0 {
                    product.ln()
                } else {
                    -100.0 // effectively zero
                }
            })
            .sum();
        (log_sum / shared as f64).exp()
    };

    // Find shared n6 constants (candidates with n6 == 1.0 in both)
    let best_a = best_candidates_per_level(a);
    let best_b = best_candidates_per_level(b);
    let mut shared_constants = Vec::new();
    for i in 0..shared {
        if i < best_a.len() && i < best_b.len() {
            if best_a[i].n6 >= 0.99 && best_b[i].n6 >= 0.99 {
                shared_constants.push(format!("L{}:{}+{}", i, best_a[i].id, best_b[i].id));
            }
        }
    }

    PairResult {
        a: a.name.clone(),
        b: b.name.clone(),
        cross_score,
        shared_levels: shared,
        shared_n6_constants: shared_constants,
    }
}

// ---------- Stats ----------

fn median(vals: &mut Vec<f64>) -> f64 {
    if vals.is_empty() {
        return 0.0;
    }
    vals.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mid = vals.len() / 2;
    if vals.len() % 2 == 0 {
        (vals[mid - 1] + vals[mid]) / 2.0
    } else {
        vals[mid]
    }
}

// ---------- Main ----------

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: {} <domains_dir> [--top N] [--csv output.csv]", args[0]);
        eprintln!("  Reads all .toml files from domains_dir, computes all pair cross-scores.");
        std::process::exit(1);
    }

    let domains_dir = &args[1];
    let mut top_n: usize = 20;
    let mut csv_path: Option<String> = None;

    // Parse optional args
    let mut i = 2;
    while i < args.len() {
        match args[i].as_str() {
            "--top" => {
                if i + 1 < args.len() {
                    top_n = args[i + 1].parse().unwrap_or(20);
                    i += 1;
                }
            }
            "--csv" => {
                if i + 1 < args.len() {
                    csv_path = Some(args[i + 1].clone());
                    i += 1;
                }
            }
            _ => {}
        }
        i += 1;
    }

    // Read all .toml files
    let dir = Path::new(domains_dir);
    if !dir.is_dir() {
        eprintln!("Error: '{}' is not a directory", domains_dir);
        std::process::exit(1);
    }

    let mut entries: Vec<_> = fs::read_dir(dir)
        .expect("Cannot read directory")
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.path()
                .extension()
                .map_or(false, |ext| ext == "toml")
        })
        .collect();
    entries.sort_by_key(|e| e.file_name());

    // Parse all domains
    let mut domains: Vec<Domain> = Vec::new();
    let mut parse_errors = 0;

    for entry in &entries {
        let path = entry.path();
        match fs::read_to_string(&path) {
            Ok(content) => {
                let mut domain = parse_toml(&content);
                // If name is still "unknown", use filename stem
                if domain.name == "unknown" {
                    if let Some(stem) = path.file_stem() {
                        domain.name = stem.to_string_lossy().to_string();
                    }
                }
                // Only include domains with at least 1 level with candidates
                if domain.levels.iter().any(|l| !l.candidates.is_empty()) {
                    domains.push(domain);
                } else {
                    parse_errors += 1;
                }
            }
            Err(_) => {
                parse_errors += 1;
            }
        }
    }

    let n = domains.len();
    let total_pairs = n * (n - 1) / 2;

    println!("Cross-DSE Analysis: {} domains, {} pairs", n, total_pairs);
    if parse_errors > 0 {
        println!("  ({} files skipped due to parse errors or empty levels)", parse_errors);
    }
    println!();

    // Compute all pairs
    let mut results: Vec<PairResult> = Vec::with_capacity(total_pairs);

    for i in 0..n {
        for j in (i + 1)..n {
            results.push(compute_cross_score(&domains[i], &domains[j]));
        }
    }

    // Sort by cross_score descending
    results.sort_by(|a, b| b.cross_score.partial_cmp(&a.cross_score).unwrap());

    // Print top N
    let show_top = top_n.min(results.len());
    println!("Top {} pairs:", show_top);
    for r in results.iter().take(show_top) {
        let constants_str = if r.shared_n6_constants.is_empty() {
            String::new()
        } else {
            format!(" [{}]", r.shared_n6_constants.join(", "))
        };
        println!(
            "  {} x {}: {:.4} ({} shared levels){}",
            r.a, r.b, r.cross_score, r.shared_levels, constants_str
        );
    }

    // Print bottom N
    println!();
    let show_bottom = top_n.min(results.len());
    println!("Bottom {} pairs:", show_bottom);
    let start = if results.len() > show_bottom {
        results.len() - show_bottom
    } else {
        0
    };
    for r in results[start..].iter() {
        let constants_str = if r.shared_n6_constants.is_empty() {
            String::new()
        } else {
            format!(" [{}]", r.shared_n6_constants.join(", "))
        };
        println!(
            "  {} x {}: {:.4} ({} shared levels){}",
            r.a, r.b, r.cross_score, r.shared_levels, constants_str
        );
    }

    // Stats
    let mut scores: Vec<f64> = results.iter().map(|r| r.cross_score).collect();
    let avg = if scores.is_empty() {
        0.0
    } else {
        scores.iter().sum::<f64>() / scores.len() as f64
    };
    let med = median(&mut scores);
    let above_95 = scores.iter().filter(|&&s| s > 0.95).count();
    let perfect = scores.iter().filter(|&&s| s >= 0.9999).count();

    println!();
    println!("Stats:");
    println!("  avg={:.4}, median={:.4}", avg, med);
    println!(
        "  >0.95: {} pairs ({:.1}%)",
        above_95,
        if total_pairs > 0 {
            100.0 * above_95 as f64 / total_pairs as f64
        } else {
            0.0
        }
    );
    println!(
        "  =1.000: {} pairs ({:.1}%)",
        perfect,
        if total_pairs > 0 {
            100.0 * perfect as f64 / total_pairs as f64
        } else {
            0.0
        }
    );

    // Per-domain summary: average cross-score
    println!();
    println!("Per-domain average cross-score (top 20):");
    let mut domain_avgs: HashMap<String, (f64, usize)> = HashMap::new();
    for r in &results {
        {
            let e = domain_avgs.entry(r.a.clone()).or_insert((0.0, 0));
            e.0 += r.cross_score;
            e.1 += 1;
        }
        {
            let e = domain_avgs.entry(r.b.clone()).or_insert((0.0, 0));
            e.0 += r.cross_score;
            e.1 += 1;
        }
    }
    let mut domain_avg_vec: Vec<(String, f64)> = domain_avgs
        .iter()
        .map(|(k, (sum, cnt))| (k.clone(), sum / *cnt as f64))
        .collect();
    domain_avg_vec.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    for (name, avg_score) in domain_avg_vec.iter().take(20) {
        println!("  {}: {:.4}", name, avg_score);
    }

    // CSV output
    if let Some(ref path) = csv_path {
        match fs::File::create(path) {
            Ok(mut f) => {
                writeln!(f, "domain_a,domain_b,cross_score,shared_levels,shared_n6_constants")
                    .ok();
                for r in &results {
                    writeln!(
                        f,
                        "{},{},{:.6},{},\"{}\"",
                        r.a,
                        r.b,
                        r.cross_score,
                        r.shared_levels,
                        r.shared_n6_constants.join("; ")
                    )
                    .ok();
                }
                println!();
                println!("CSV written to: {}", path);
            }
            Err(e) => {
                eprintln!("Error writing CSV: {}", e);
            }
        }
    }
}
