// Universal DSE Explorer — n6-architecture
// Rust single-file, no crate dependencies
// Usage: universal-dse <domain.toml> [domain2.toml ...] [--top N]
//   Single domain:  universal-dse domains/chip.toml
//   Cross-DSE:      universal-dse domains/chip.toml domains/battery.toml

use std::env;
use std::fs;
use std::fmt;

// ── N6 Constants ──
const N: f64 = 6.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const SOPFR: f64 = 5.0;
const MU: f64 = 1.0;
const J2: f64 = 24.0;

// ── Data Structures ──

#[derive(Clone, Debug)]
struct Candidate {
    id: String,
    label: String,
    n6: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

#[derive(Clone, Debug)]
struct Level {
    name: String,
    candidates: Vec<Candidate>,
}

#[derive(Clone, Debug)]
struct Rule {
    rule_type: String, // "require" or "exclude"
    if_level: usize,
    if_id: String,
    then_level: usize,
    then_ids: Vec<String>,
}

#[derive(Clone, Debug)]
struct Weights {
    n6: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

#[derive(Clone, Debug)]
struct Domain {
    name: String,
    desc: String,
    weights: Weights,
    levels: Vec<Level>,
    rules: Vec<Rule>,
}

#[derive(Clone)]
struct Combo {
    indices: Vec<usize>,
    n6_avg: f64,
    perf_avg: f64,
    power_avg: f64,
    cost_avg: f64,
    pareto_score: f64,
}

// ── TOML Subset Parser ──
// Supports: [section], [[array]], key = "str", key = 0.5, # comments
// [[level]] starts a new level; [[candidate]] adds to last level; [[rule]] adds a rule

fn parse_toml(content: &str) -> Domain {
    let mut name = String::new();
    let mut desc = String::new();
    let mut weights = Weights { n6: 0.40, perf: 0.30, power: 0.20, cost: 0.10 };
    let mut levels: Vec<Level> = Vec::new();
    let mut rules: Vec<Rule> = Vec::new();

    #[derive(PartialEq)]
    enum Section { Meta, Scoring, Level, Candidate, Rule, None }
    let mut section = Section::None;

    // Temp buffers for building candidates and rules
    let mut cur_cand = Candidate {
        id: String::new(), label: String::new(),
        n6: 0.0, perf: 0.0, power: 0.0, cost: 0.0,
    };
    let mut cur_rule = Rule {
        rule_type: String::new(), if_level: 0, if_id: String::new(),
        then_level: 0, then_ids: Vec::new(),
    };
    let mut has_cand = false;
    let mut has_rule = false;

    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        // Section headers
        if line == "[[level]]" {
            // Flush pending candidate
            if has_cand && !levels.is_empty() {
                levels.last_mut().unwrap().candidates.push(cur_cand.clone());
                has_cand = false;
            }
            if has_rule {
                rules.push(cur_rule.clone());
                has_rule = false;
            }
            levels.push(Level { name: String::new(), candidates: Vec::new() });
            section = Section::Level;
            continue;
        }
        if line == "[[candidate]]" {
            if has_cand && !levels.is_empty() {
                levels.last_mut().unwrap().candidates.push(cur_cand.clone());
            }
            cur_cand = Candidate {
                id: String::new(), label: String::new(),
                n6: 0.0, perf: 0.0, power: 0.0, cost: 0.0,
            };
            has_cand = true;
            section = Section::Candidate;
            continue;
        }
        if line == "[[rule]]" {
            if has_cand && !levels.is_empty() {
                levels.last_mut().unwrap().candidates.push(cur_cand.clone());
                has_cand = false;
            }
            if has_rule {
                rules.push(cur_rule.clone());
            }
            cur_rule = Rule {
                rule_type: String::new(), if_level: 0, if_id: String::new(),
                then_level: 0, then_ids: Vec::new(),
            };
            has_rule = true;
            section = Section::Rule;
            continue;
        }
        if line == "[meta]" { section = Section::Meta; continue; }
        if line == "[scoring]" { section = Section::Scoring; continue; }

        // Key = Value
        if let Some(eq_pos) = line.find('=') {
            let key = line[..eq_pos].trim();
            let val = line[eq_pos + 1..].trim();
            let val_str = val.trim_matches('"');

            match section {
                Section::Meta => match key {
                    "name" => name = val_str.to_string(),
                    "desc" => desc = val_str.to_string(),
                    _ => {}
                },
                Section::Scoring => {
                    let v: f64 = val.parse().unwrap_or(0.0);
                    match key {
                        "n6" => weights.n6 = v,
                        "perf" => weights.perf = v,
                        "power" => weights.power = v,
                        "cost" => weights.cost = v,
                        _ => {}
                    }
                },
                Section::Level => {
                    if key == "name" {
                        if let Some(l) = levels.last_mut() {
                            l.name = val_str.to_string();
                        }
                    }
                },
                Section::Candidate => match key {
                    "id" => cur_cand.id = val_str.to_string(),
                    "label" => cur_cand.label = val_str.to_string(),
                    "n6" => cur_cand.n6 = val.parse().unwrap_or(0.0),
                    "perf" => cur_cand.perf = val.parse().unwrap_or(0.0),
                    "power" => cur_cand.power = val.parse().unwrap_or(0.0),
                    "cost" => cur_cand.cost = val.parse().unwrap_or(0.0),
                    _ => {}
                },
                Section::Rule => match key {
                    "type" => cur_rule.rule_type = val_str.to_string(),
                    "if_level" => cur_rule.if_level = val.parse().unwrap_or(0),
                    "if_id" => cur_rule.if_id = val_str.to_string(),
                    "then_level" => cur_rule.then_level = val.parse().unwrap_or(0),
                    "then_ids" => {
                        cur_rule.then_ids = val_str.split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                    }
                    _ => {}
                },
                Section::None => {}
            }
        }
    }

    // Flush remaining
    if has_cand && !levels.is_empty() {
        levels.last_mut().unwrap().candidates.push(cur_cand);
    }
    if has_rule {
        rules.push(cur_rule);
    }

    Domain { name, desc, weights, levels, rules }
}

// ── Compatibility Check ──

fn is_compatible(rules: &[Rule], levels: &[Level], indices: &[usize]) -> bool {
    for rule in rules {
        if rule.if_level >= levels.len() || rule.then_level >= levels.len() {
            continue;
        }
        let if_cand = &levels[rule.if_level].candidates[indices[rule.if_level]];
        if if_cand.id == rule.if_id {
            let then_cand = &levels[rule.then_level].candidates[indices[rule.then_level]];
            match rule.rule_type.as_str() {
                "require" => {
                    if !rule.then_ids.iter().any(|id| id == &then_cand.id) {
                        return false;
                    }
                }
                "exclude" => {
                    if rule.then_ids.iter().any(|id| id == &then_cand.id) {
                        return false;
                    }
                }
                _ => {}
            }
        }
    }
    true
}

// ── Scoring ──

fn score(domain: &Domain, indices: &[usize]) -> Combo {
    let n = domain.levels.len() as f64;
    let mut n6_sum = 0.0;
    let mut perf_sum = 0.0;
    let mut power_sum = 0.0;
    let mut cost_sum = 0.0;

    for (i, level) in domain.levels.iter().enumerate() {
        let c = &level.candidates[indices[i]];
        n6_sum += c.n6;
        perf_sum += c.perf;
        power_sum += c.power;
        cost_sum += c.cost;
    }

    let n6_avg = n6_sum / n;
    let perf_avg = perf_sum / n;
    let power_avg = power_sum / n;
    let cost_avg = cost_sum / n;

    let w = &domain.weights;
    let pareto_score = w.n6 * n6_avg + w.perf * perf_avg
                     + w.power * power_avg + w.cost * cost_avg;

    Combo {
        indices: indices.to_vec(),
        n6_avg, perf_avg, power_avg, cost_avg, pareto_score,
    }
}

// ── Enumeration (variable number of levels) ──

fn enumerate(domain: &Domain) -> Vec<Combo> {
    let num_levels = domain.levels.len();
    if num_levels == 0 { return Vec::new(); }

    let sizes: Vec<usize> = domain.levels.iter().map(|l| l.candidates.len()).collect();
    let total: usize = sizes.iter().product();

    let mut results = Vec::with_capacity(total);
    let mut indices = vec![0usize; num_levels];

    for _ in 0..total {
        if is_compatible(&domain.rules, &domain.levels, &indices) {
            results.push(score(domain, &indices));
        }
        // Odometer increment
        let mut carry = true;
        for i in (0..num_levels).rev() {
            if carry {
                indices[i] += 1;
                if indices[i] >= sizes[i] {
                    indices[i] = 0;
                } else {
                    carry = false;
                }
            }
        }
    }

    results.sort_by(|a, b| b.pareto_score.partial_cmp(&a.pareto_score).unwrap());
    results
}

// ── Pareto Frontier (dominance-based) ──

fn pareto_frontier(combos: &[Combo]) -> Vec<usize> {
    let mut frontier: Vec<usize> = Vec::new();
    for i in 0..combos.len() {
        let mut dominated = false;
        for j in 0..combos.len() {
            if i == j { continue; }
            if dominates(&combos[j], &combos[i]) {
                dominated = true;
                break;
            }
        }
        if !dominated {
            frontier.push(i);
        }
    }
    frontier
}

fn dominates(a: &Combo, b: &Combo) -> bool {
    let a_vals = [a.n6_avg, a.perf_avg, a.power_avg, a.cost_avg];
    let b_vals = [b.n6_avg, b.perf_avg, b.power_avg, b.cost_avg];
    let all_geq = a_vals.iter().zip(b_vals.iter()).all(|(av, bv)| av >= bv);
    let any_gt = a_vals.iter().zip(b_vals.iter()).any(|(av, bv)| av > bv);
    all_geq && any_gt
}

// ── Output ──

fn combo_path(domain: &Domain, combo: &Combo) -> String {
    combo.indices.iter().enumerate()
        .map(|(i, &idx)| domain.levels[i].candidates[idx].id.clone())
        .collect::<Vec<_>>()
        .join(" + ")
}

fn combo_labels(domain: &Domain, combo: &Combo) -> String {
    combo.indices.iter().enumerate()
        .map(|(i, &idx)| domain.levels[i].candidates[idx].label.clone())
        .collect::<Vec<_>>()
        .join(" -> ")
}

fn print_header(domain: &Domain, total: usize, compatible: usize) {
    let bar = "=".repeat(66);
    println!("\n{}", bar);
    println!("  Universal DSE -- {}", domain.name);
    if !domain.desc.is_empty() {
        println!("  {}", domain.desc);
    }
    println!("  {} total combinations -> {} compatible", total, compatible);
    println!("  Weights: n6={:.0}% perf={:.0}% power={:.0}% cost={:.0}%",
        domain.weights.n6 * 100.0, domain.weights.perf * 100.0,
        domain.weights.power * 100.0, domain.weights.cost * 100.0);
    println!("{}\n", bar);
}

fn print_candidates(domain: &Domain) {
    println!("=== CANDIDATES ===\n");
    for (i, level) in domain.levels.iter().enumerate() {
        print!("  L{}: {} ({})  ", i + 1, level.name, level.candidates.len());
        let ids: Vec<&str> = level.candidates.iter().map(|c| c.id.as_str()).collect();
        println!("{}", ids.join(", "));
    }
    println!();
}

fn print_top(domain: &Domain, combos: &[Combo], top_n: usize) {
    let show = std::cmp::min(top_n, combos.len());
    println!("=== TOP {} ===\n", show);

    // Header
    let level_names: Vec<String> = domain.levels.iter()
        .map(|l| {
            let n = &l.name;
            if n.len() > 10 { n[..10].to_string() } else { n.clone() }
        })
        .collect();

    print!("  {:>4} |", "Rank");
    for n in &level_names {
        print!(" {:>10} |", n);
    }
    println!("  n6%  | Perf  | Power | Cost  | Pareto");

    let sep_w = 7 + (level_names.len() * 13) + 45;
    println!("  {}", "-".repeat(sep_w));

    for (rank, combo) in combos[..show].iter().enumerate() {
        print!("  {:>4} |", rank + 1);
        for (i, &idx) in combo.indices.iter().enumerate() {
            let id = &domain.levels[i].candidates[idx].id;
            let short = if id.len() > 10 { &id[..10] } else { id.as_str() };
            print!(" {:>10} |", short);
        }
        println!(" {:5.1} | {:.3} | {:.3} | {:.3} | {:.4}",
            combo.n6_avg * 100.0, combo.perf_avg, combo.power_avg,
            combo.cost_avg, combo.pareto_score);
    }
    println!();
}

fn print_best_by_category(domain: &Domain, combos: &[Combo]) {
    if combos.is_empty() { return; }
    println!("=== BEST BY CATEGORY ===\n");

    // Best n6
    let best_n6 = combos.iter().max_by(|a, b|
        a.n6_avg.partial_cmp(&b.n6_avg).unwrap()).unwrap();
    println!("  Best n6:    {} ({:.1}%)", combo_path(domain, best_n6), best_n6.n6_avg * 100.0);

    // Best perf
    let best_perf = combos.iter().max_by(|a, b|
        a.perf_avg.partial_cmp(&b.perf_avg).unwrap()).unwrap();
    println!("  Best Perf:  {} (perf={:.3})", combo_path(domain, best_perf), best_perf.perf_avg);

    // Best power
    let best_pow = combos.iter().max_by(|a, b|
        a.power_avg.partial_cmp(&b.power_avg).unwrap()).unwrap();
    println!("  Best Power: {} (power={:.3})", combo_path(domain, best_pow), best_pow.power_avg);

    // Best cost
    let best_cost = combos.iter().max_by(|a, b|
        a.cost_avg.partial_cmp(&b.cost_avg).unwrap()).unwrap();
    println!("  Best Cost:  {} (cost={:.3})", combo_path(domain, best_cost), best_cost.cost_avg);

    println!();
}

fn print_pareto(domain: &Domain, combos: &[Combo], frontier: &[usize]) {
    println!("=== PARETO FRONTIER ({} non-dominated solutions) ===\n", frontier.len());

    let show = std::cmp::min(10, frontier.len());
    for (i, &idx) in frontier[..show].iter().enumerate() {
        let c = &combos[idx];
        println!("  {:>2}. {} | n6={:.1}% perf={:.3} pow={:.3} cost={:.3}",
            i + 1, combo_path(domain, c),
            c.n6_avg * 100.0, c.perf_avg, c.power_avg, c.cost_avg);
    }
    if frontier.len() > show {
        println!("  ... +{} more", frontier.len() - show);
    }
    println!();
}

fn print_stats(combos: &[Combo]) {
    if combos.is_empty() { return; }
    println!("=== STATISTICS ===\n");

    let n6_vals: Vec<f64> = combos.iter().map(|c| c.n6_avg * 100.0).collect();
    let perf_vals: Vec<f64> = combos.iter().map(|c| c.perf_avg).collect();

    let n6_max = n6_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let n6_min = n6_vals.iter().cloned().fold(f64::INFINITY, f64::min);
    let n6_avg = n6_vals.iter().sum::<f64>() / n6_vals.len() as f64;

    let perf_max = perf_vals.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let perf_avg = perf_vals.iter().sum::<f64>() / perf_vals.len() as f64;

    // Percentiles
    let mut sorted_n6 = n6_vals.clone();
    sorted_n6.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let p50 = sorted_n6[sorted_n6.len() / 2];
    let p75 = sorted_n6[sorted_n6.len() * 3 / 4];
    let p90 = sorted_n6[sorted_n6.len() * 9 / 10];

    println!("  n6%:  max={:.1}  min={:.1}  avg={:.1}  p50={:.1}  p75={:.1}  p90={:.1}",
        n6_max, n6_min, n6_avg, p50, p75, p90);
    println!("  perf: max={:.3}  avg={:.3}", perf_max, perf_avg);
    println!("  combos: {}", combos.len());
    println!();
}

fn print_ascii_path(domain: &Domain, combo: &Combo) {
    println!("=== OPTIMAL PATH ===\n");
    let n = domain.levels.len();
    for (i, &idx) in combo.indices.iter().enumerate() {
        let c = &domain.levels[i].candidates[idx];
        let bar_len = (c.n6 * 20.0) as usize;
        let bar: String = "█".repeat(bar_len);
        let empty: String = "░".repeat(20 - bar_len);
        println!("  L{} {:>10}: [{}{}] n6={:.0}%  {}",
            i + 1, domain.levels[i].name,
            bar, empty, c.n6 * 100.0, c.label);
        if i < n - 1 {
            println!("        |");
            println!("        v");
        }
    }
    println!();
}

// ── Cross-DSE ──

fn cross_dse(domains: &[Domain], top_k: usize) {
    println!("\n{}", "=".repeat(66));
    println!("  Cross-DSE: {} domains", domains.len());
    for d in domains {
        println!("    - {}", d.name);
    }
    println!("{}\n", "=".repeat(66));

    // Run each domain, take top-K
    let mut domain_tops: Vec<(String, Vec<Combo>)> = Vec::new();
    for domain in domains {
        let combos = enumerate(domain);
        let take = std::cmp::min(top_k, combos.len());
        domain_tops.push((domain.name.clone(), combos[..take].to_vec()));
    }

    // Pairwise cross-combination
    if domain_tops.len() >= 2 {
        for i in 0..domain_tops.len() {
            for j in (i + 1)..domain_tops.len() {
                let (ref name_a, ref tops_a) = domain_tops[i];
                let (ref name_b, ref tops_b) = domain_tops[j];

                println!("--- Cross: {} x {} ---\n", name_a, name_b);

                let mut cross_results: Vec<(usize, usize, f64, f64, f64, f64, f64)> = Vec::new();

                for (ai, a) in tops_a.iter().enumerate() {
                    for (bi, b) in tops_b.iter().enumerate() {
                        let cn6 = (a.n6_avg + b.n6_avg) / 2.0;
                        let cperf = (a.perf_avg + b.perf_avg) / 2.0;
                        let cpow = (a.power_avg + b.power_avg) / 2.0;
                        let ccost = (a.cost_avg + b.cost_avg) / 2.0;
                        let cscore = 0.40 * cn6 + 0.30 * cperf + 0.20 * cpow + 0.10 * ccost;
                        cross_results.push((ai, bi, cn6, cperf, cpow, ccost, cscore));
                    }
                }

                cross_results.sort_by(|a, b| b.6.partial_cmp(&a.6).unwrap());

                let show = std::cmp::min(10, cross_results.len());
                println!("  {:>4} | {:>6} | {:>6} | {:>5} | {:>5} | {:>5} | {:>5} | {:>6}",
                    "Rank", name_a, name_b, "n6%", "Perf", "Power", "Cost", "Score");
                let sep = "-".repeat(70);
                println!("  {}", sep);

                for (r, &(ai, bi, cn6, cp, cpw, cc, cs)) in cross_results[..show].iter().enumerate() {
                    let pa = combo_short(&domains[i], &tops_a[ai]);
                    let pb = combo_short(&domains[j], &tops_b[bi]);
                    println!("  {:>4} | {:>6} | {:>6} | {:4.1} | {:.3} | {:.3} | {:.3} | {:.4}",
                        r + 1, pa, pb, cn6 * 100.0, cp, cpw, cc, cs);
                }
                println!();
            }
        }
    }
}

fn combo_short(domain: &Domain, combo: &Combo) -> String {
    if combo.indices.is_empty() { return String::from("-"); }
    let first = &domain.levels[0].candidates[combo.indices[0]].id;
    if first.len() > 6 { first[..6].to_string() } else { first.clone() }
}

// ── Main ──

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Universal DSE Explorer — n6-architecture");
        eprintln!();
        eprintln!("Usage:");
        eprintln!("  {} <domain.toml>                 Single domain DSE", args[0]);
        eprintln!("  {} <d1.toml> <d2.toml> [...]     Cross-DSE", args[0]);
        eprintln!("  {} <domain.toml> --top 30         Custom top-N", args[0]);
        eprintln!();
        eprintln!("TOML Format:");
        eprintln!("  [meta]           name, desc");
        eprintln!("  [scoring]        n6, perf, power, cost weights (sum=1.0)");
        eprintln!("  [[level]]        name of each level");
        eprintln!("  [[candidate]]    id, label, n6, perf, power, cost (0.0-1.0)");
        eprintln!("  [[rule]]         type=require|exclude, if_level, if_id, then_level, then_ids");
        std::process::exit(1);
    }

    // Parse args
    let mut toml_files: Vec<String> = Vec::new();
    let mut top_n: usize = 20;
    let mut i = 1;
    while i < args.len() {
        if args[i] == "--top" && i + 1 < args.len() {
            top_n = args[i + 1].parse().unwrap_or(20);
            i += 2;
        } else {
            toml_files.push(args[i].clone());
            i += 1;
        }
    }

    if toml_files.is_empty() {
        eprintln!("Error: no TOML files specified");
        std::process::exit(1);
    }

    // Load domains
    let mut domains: Vec<Domain> = Vec::new();
    for path in &toml_files {
        let content = match fs::read_to_string(path) {
            Ok(c) => c,
            Err(e) => {
                eprintln!("Error reading {}: {}", path, e);
                std::process::exit(1);
            }
        };
        let domain = parse_toml(&content);
        if domain.levels.is_empty() {
            eprintln!("Warning: {} has no levels defined", path);
            continue;
        }
        domains.push(domain);
    }

    if domains.is_empty() {
        eprintln!("Error: no valid domains loaded");
        std::process::exit(1);
    }

    // Cross-DSE mode
    if domains.len() > 1 {
        // First run each individually
        for domain in &domains {
            run_single(domain, top_n);
        }
        // Then cross-DSE
        cross_dse(&domains, 5);
        return;
    }

    // Single domain mode
    run_single(&domains[0], top_n);
}

fn run_single(domain: &Domain, top_n: usize) {
    let sizes: Vec<usize> = domain.levels.iter().map(|l| l.candidates.len()).collect();
    let total: usize = sizes.iter().product();

    let combos = enumerate(domain);
    let compatible = combos.len();

    print_header(domain, total, compatible);
    print_candidates(domain);
    print_top(domain, &combos, top_n);
    print_best_by_category(domain, &combos);

    // Pareto frontier (limit to top 200 for O(n^2) dominance check)
    let pareto_set = if combos.len() <= 500 {
        pareto_frontier(&combos)
    } else {
        pareto_frontier(&combos[..500])
    };
    print_pareto(domain, &combos, &pareto_set);

    print_stats(&combos);

    if !combos.is_empty() {
        print_ascii_path(domain, &combos[0]);
    }
}
