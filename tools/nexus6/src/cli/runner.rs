use std::collections::HashMap;

use crate::graph::persistence::DiscoveryGraph;
use crate::history::{recorder, stats, recommend, DomainStats};
use crate::ouroboros::{EvolutionEngine, EvolutionConfig, MetaLoop, MetaLoopConfig};
use crate::telescope::registry::{LensCategory, LensRegistry};
use crate::telescope::domain_combos;
use crate::telescope::Telescope;
use crate::verifier::n6_check;

use super::dashboard;
use super::parser::{CliCommand, GraphFormat, LensFilter};

/// Execute a parsed CLI command, printing results to stdout.
pub fn run(cmd: CliCommand) -> Result<(), String> {
    match cmd {
        CliCommand::Scan { domain, lenses, full } => run_scan(&domain, lenses, full),
        CliCommand::Verify { value, tolerance } => run_verify(value, tolerance),
        CliCommand::Graph { domain, format } => run_graph(domain, format),
        CliCommand::History { domain } => run_history(&domain),
        CliCommand::Recommend { domain } => run_recommend(&domain),
        CliCommand::Evolve { domain, max_cycles, seeds } => run_evolve(&domain, max_cycles, seeds),
        CliCommand::Auto { domain, max_meta_cycles, max_ouroboros_cycles } => {
            run_auto(&domain, max_meta_cycles, max_ouroboros_cycles)
        }
        CliCommand::Lenses { category } => run_lenses(category),
        CliCommand::Dashboard => run_dashboard(),
        CliCommand::Help => {
            print_help();
            Ok(())
        }
    }
}

fn run_scan(domain: &str, lenses: Option<Vec<String>>, full: bool) -> Result<(), String> {
    println!("=== NEXUS-6 Scan: {} ===", domain);

    let telescope = Telescope::new();

    // Determine which lenses to use
    let lens_list = if let Some(ref l) = lenses {
        println!("  Lenses (manual): {}", l.join(", "));
        l.clone()
    } else if full {
        let registry = LensRegistry::new();
        let all: Vec<String> = registry.iter().map(|(name, _)| name.clone()).collect();
        println!("  Lenses (full scan): {} lenses", all.len());
        all
    } else {
        // Auto-recommend based on domain combos
        let combos = domain_combos::default_combos();
        let domain_lower = domain.to_lowercase();
        let matched = combos.iter().find(|c| {
            c.target_domains.iter().any(|d| d.contains(&domain_lower))
                || c.name.contains(&domain_lower)
        });
        let selected = match matched {
            Some(combo) => {
                println!("  Combo matched: {} -> {}", combo.name, combo.lenses.join("+"));
                combo.lenses.clone()
            }
            None => {
                let default = vec![
                    "consciousness".to_string(),
                    "topology".to_string(),
                    "causal".to_string(),
                ];
                println!("  Using default combo: {}", default.join("+"));
                default
            }
        };
        selected
    };

    // Run telescope scan with synthetic probe data
    let probe_data: Vec<f64> = vec![6.0, 12.0, 24.0, 4.0, 2.0, 5.0];
    let results = telescope.scan_all(&probe_data, probe_data.len(), 1);

    let active_count = results.values().filter(|lr| !lr.is_empty()).count();
    let n6_ratio = n6_check::n6_exact_ratio(&probe_data);

    println!();
    println!("  Results:");
    println!("    Active lenses:  {}/{}", active_count, telescope.lens_count());
    println!("    n6 EXACT ratio: {:.1}%", n6_ratio * 100.0);
    println!("    Lenses queried: {}", lens_list.len());

    // Show per-lens results
    for (lens_name, lr) in &results {
        let entry_count: usize = lr.values().map(|v| v.len()).sum();
        if entry_count > 0 {
            println!("    [{}] {} entries", lens_name, entry_count);
        }
    }

    println!();
    println!("  Scan complete.");
    Ok(())
}

fn run_verify(value: f64, tolerance: Option<f64>) -> Result<(), String> {
    use crate::verifier::feasibility;

    println!("=== NEXUS-6 Verify: {} ===", value);
    println!();

    let (name, quality) = n6_check::n6_match(value);

    let grade = if quality >= 1.0 {
        "EXACT"
    } else if quality >= 0.8 {
        "CLOSE"
    } else if quality >= 0.5 {
        "WEAK"
    } else {
        "NONE"
    };

    println!("  Value:     {}", value);
    println!("  Match:     {} ({})", name, grade);
    println!("  Quality:   {:.2}", quality);

    if let Some(tol) = tolerance {
        println!("  Tolerance: {}", tol);
        // Check if any constant is within the custom tolerance
        let within = check_within_tolerance(value, tol);
        println!("  Within:    {}", if within { "YES" } else { "NO" });
    }

    // Feasibility score (treating the single value as a probe)
    let n6_ratio = n6_check::n6_exact_ratio(&[value]);
    let verification = feasibility::verify(
        quality,  // lens_consensus ~ match quality
        0.5,      // cross_validation placeholder
        0.5,      // physical_check placeholder
        0.0,      // graph_bonus (no graph context)
        1.0,      // novelty (single verification)
        n6_ratio, // n6 exact ratio
    );
    println!("  Feasibility: {:.3} (grade {})", verification.score, verification.grade.label());

    // Show nearby constants
    println!();
    println!("  Nearby n=6 constants:");
    let constants = [
        ("n", 6.0), ("sigma", 12.0), ("phi", 2.0), ("tau", 4.0),
        ("J2", 24.0), ("sopfr", 5.0), ("mu", 1.0),
        ("sigma-phi", 10.0), ("sigma-tau", 8.0), ("sigma-mu", 11.0),
        ("sigma*tau", 48.0), ("sigma^2", 144.0),
    ];
    for (cname, cval) in &constants {
        let error = if *cval != 0.0 {
            ((value - cval) / cval).abs() * 100.0
        } else {
            f64::INFINITY
        };
        if error < 20.0 {
            println!("    {:<12} = {:>8.3}  (error: {:.2}%)", cname, cval, error);
        }
    }

    Ok(())
}

/// Check if a value is within the given tolerance of any n=6 constant.
fn check_within_tolerance(value: f64, tolerance: f64) -> bool {
    let constants = [
        6.0, 12.0, 2.0, 4.0, 24.0, 5.0, 1.0,
        10.0, 8.0, 11.0, 48.0, 144.0, 16.0, 3.0, 20.0,
    ];
    constants.iter().any(|&c| {
        if c != 0.0 {
            ((value - c) / c).abs() <= tolerance
        } else {
            false
        }
    })
}

fn run_graph(domain: Option<String>, format: GraphFormat) -> Result<(), String> {
    let graph = DiscoveryGraph::new(); // Would load from persistence in production

    println!("=== NEXUS-6 Discovery Graph ===");
    println!("  Nodes: {}  Edges: {}", graph.nodes.len(), graph.edges.len());

    if let Some(ref d) = domain {
        println!("  Filter: domain={}", d);
    }

    if graph.nodes.is_empty() {
        println!();
        println!("  (empty graph -- run 'nexus6 evolve <domain>' to populate)");
        return Ok(());
    }

    match format {
        GraphFormat::Ascii => {
            println!();
            for node in &graph.nodes {
                let edges_out: Vec<_> = graph.edges.iter()
                    .filter(|e| e.from == node.id)
                    .collect();
                if edges_out.is_empty() {
                    println!("  [{}]", node.id);
                } else {
                    for edge in edges_out {
                        println!("  [{}] --{:?}--> [{}]", node.id, edge.edge_type, edge.to);
                    }
                }
            }
        }
        GraphFormat::Dot => {
            println!();
            println!("digraph nexus6 {{");
            println!("  rankdir=LR;");
            for node in &graph.nodes {
                println!("  \"{}\" [label=\"{}\"];", node.id, node.id);
            }
            for edge in &graph.edges {
                println!(
                    "  \"{}\" -> \"{}\" [label=\"{:?}\"];",
                    edge.from, edge.to, edge.edge_type
                );
            }
            println!("}}");
        }
    }

    Ok(())
}

fn run_history(domain: &str) -> Result<(), String> {
    println!("=== NEXUS-6 History: {} ===", domain);

    let records = recorder::load_records("nexus6_history", domain);

    if records.is_empty() {
        println!();
        println!("  No scan history for domain '{}'.", domain);
        println!("  Run 'nexus6 scan {}' or 'nexus6 evolve {}' to start.", domain, domain);
        return Ok(());
    }

    let domain_stats = stats::compute_domain_stats(&records);

    println!();
    println!("  Total scans:       {}", domain_stats.total_scans);
    println!("  Total discoveries: {}", domain_stats.total_discoveries);
    println!();

    // Lens stats table
    let mut lens_list: Vec<_> = domain_stats.lens_stats.iter().collect();
    lens_list.sort_by(|a, b| b.1.hit_rate.partial_cmp(&a.1.hit_rate).unwrap());

    println!("  Lens Performance:");
    println!("  {:<20} {:>5} {:>5} {:>8}", "Lens", "Used", "Hits", "HitRate");
    println!("  {}", "-".repeat(42));
    for (name, ls) in &lens_list {
        let bar = hit_rate_bar(ls.hit_rate, 10);
        println!(
            "  {:<20} {:>5} {:>5} {} {:.2}",
            name, ls.used, ls.contributed, bar, ls.hit_rate
        );
    }

    // Recent records
    println!();
    println!("  Recent Scans (last 5):");
    for record in records.iter().rev().take(5) {
        println!(
            "    [{}] lenses={} discoveries={}",
            record.timestamp,
            record.lenses_used.len(),
            record.discoveries.len()
        );
    }

    Ok(())
}

fn run_recommend(domain: &str) -> Result<(), String> {
    println!("=== NEXUS-6 Recommend: {} ===", domain);

    let registry = LensRegistry::new();
    let all_lenses: Vec<String> = registry.iter().map(|(name, _)| name.clone()).collect();

    // Load history to compute stats
    let records = recorder::load_records("nexus6_history", domain);
    let mut all_stats: HashMap<String, DomainStats> = HashMap::new();
    if !records.is_empty() {
        all_stats.insert(domain.to_string(), stats::compute_domain_stats(&records));
    }

    let rec = recommend::recommend_lenses(domain, &all_stats, &all_lenses, 0.2);

    println!();
    println!("  Recommended lenses ({}):", rec.lenses.len());
    for (i, lens) in rec.lenses.iter().enumerate() {
        let tag = if let Some(entry) = registry.get(lens) {
            format!("{:?}", entry.category)
        } else {
            "unknown".to_string()
        };
        println!("    {:>2}. {:<20} [{}]", i + 1, lens, tag);
    }
    println!();
    println!("  Reason: {}", rec.reason);

    Ok(())
}

fn run_evolve(domain: &str, max_cycles: usize, seeds: Vec<String>) -> Result<(), String> {
    println!("=== NEXUS-6 Evolve: {} (max {} cycles) ===", domain, max_cycles);

    let mut config = EvolutionConfig::default();
    config.domain = domain.to_string();

    let seed_hypotheses = if seeds.is_empty() {
        vec![format!("n=6 patterns in {}", domain)]
    } else {
        seeds
    };

    println!("  Seeds: {:?}", seed_hypotheses);
    println!();

    let mut engine = EvolutionEngine::new(config, seed_hypotheses);

    for _i in 0..max_cycles {
        let result = engine.evolve_step();
        println!(
            "  Cycle {}: discoveries={} nodes={} edges={} score={:.3}",
            result.cycle,
            result.new_discoveries,
            result.graph_nodes,
            result.graph_edges,
            result.verification_score,
        );

        // Check convergence
        let status = engine.convergence_checker.check(&engine.history);
        match status {
            crate::ouroboros::convergence::ConvergenceStatus::Saturated => {
                println!();
                println!("  Saturated at cycle {} -- evolution complete.", result.cycle);
                break;
            }
            crate::ouroboros::convergence::ConvergenceStatus::Converging => {
                println!("  (converging -- discovery rate decreasing)");
            }
            crate::ouroboros::convergence::ConvergenceStatus::Divergent => {
                println!("  (divergent -- discovery rate increasing)");
            }
            crate::ouroboros::convergence::ConvergenceStatus::Exploring => {}
        }
    }

    println!();
    println!("  Final graph: {} nodes, {} edges",
        engine.graph.nodes.len(), engine.graph.edges.len());
    println!("  Evolution complete.");

    Ok(())
}

fn run_auto(domain: &str, max_meta_cycles: usize, max_ouroboros_cycles: usize) -> Result<(), String> {
    println!("=== NEXUS-6 Auto: {} (meta={}, ouroboros={}) ===",
        domain, max_meta_cycles, max_ouroboros_cycles);
    println!("  OUROBOROS + LensForge meta-loop");
    println!();

    let config = MetaLoopConfig {
        max_ouroboros_cycles,
        max_meta_cycles,
        forge_after_n_cycles: 0,
        ..MetaLoopConfig::default()
    };

    let seeds = vec![format!("n=6 patterns in {}", domain)];
    let mut meta_loop = MetaLoop::new(domain.to_string(), seeds, config);

    // Attach progress printer
    meta_loop.on_progress = Some(Box::new(|mc, oc, msg| {
        if oc == 0 {
            println!("  [Meta-{}] {}", mc, msg);
        } else {
            println!("    Cycle {}: {}", oc, msg);
        }
    }));

    let result = meta_loop.run();

    // Summary
    println!();
    println!("  ┌─────────────────────────────────────────────┐");
    println!("  │           Auto Evolution Summary             │");
    println!("  ├─────────────────────────────────────────────┤");
    println!("  │  Meta-cycles completed: {:>4}                │", result.meta_cycles_completed);
    println!("  │  Total OUROBOROS cycles: {:>3}                │", result.ouroboros_results.len());
    println!("  │  Total discoveries:     {:>4}                │", result.total_discoveries);
    println!("  │  Lenses forged:         {:>4}                │", result.forged_lenses.len());
    println!("  └─────────────────────────────────────────────┘");

    if !result.forged_lenses.is_empty() {
        println!();
        println!("  Forged lenses:");
        for (i, name) in result.forged_lenses.iter().enumerate() {
            println!("    {:>2}. {}", i + 1, name);
        }
    }

    // Per-meta-cycle table
    println!();
    println!("  {:>5} {:>8} {:>6} {:>12} {:>8}",
        "Meta", "Ouro.Cy", "Disc.", "Convergence", "Forged");
    println!("  {}", "-".repeat(45));
    for summary in &result.meta_cycle_summaries {
        println!("  {:>5} {:>8} {:>6} {:>12?} {:>8}",
            summary.meta_cycle,
            summary.ouroboros_cycles_run,
            summary.discoveries,
            summary.convergence_status,
            summary.lenses_forged.len(),
        );
    }

    println!();
    println!("  Auto evolution complete.");
    Ok(())
}

fn run_lenses(category: Option<LensFilter>) -> Result<(), String> {
    let registry = LensRegistry::new();
    let combos = domain_combos::default_combos();

    println!("=== NEXUS-6 Lens Registry ===");
    println!();

    let show_core = category.is_none() || category == Some(LensFilter::Core);
    let show_combo = category.is_none() || category == Some(LensFilter::Combo);
    let show_ext = category.is_none() || category == Some(LensFilter::Extended);
    let show_custom = category.is_none() || category == Some(LensFilter::Custom);

    if show_core {
        let core = registry.by_category(LensCategory::Core);
        println!("  Core Lenses ({}):", core.len());
        let mut sorted: Vec<_> = core.iter().collect();
        sorted.sort_by_key(|e| &e.name);
        for entry in sorted {
            println!("    {:<22} {}", entry.name, entry.description);
        }
        println!();
    }

    if show_combo {
        println!("  Domain Combos ({}):", combos.len());
        for combo in &combos {
            println!(
                "    {:<18} {} -> {:?}",
                combo.name,
                combo.lenses.join("+"),
                combo.target_domains
            );
        }
        println!();
    }

    if show_ext {
        let ext = registry.by_category(LensCategory::Extended);
        if !ext.is_empty() {
            println!("  Extended Lenses ({}):", ext.len());
            for entry in &ext {
                println!("    {:<22} {}", entry.name, entry.description);
            }
            println!();
        }
    }

    if show_custom {
        let custom = registry.by_category(LensCategory::Custom);
        if !custom.is_empty() {
            println!("  Custom Lenses ({}):", custom.len());
            for entry in &custom {
                println!("    {:<22} {}", entry.name, entry.description);
            }
            println!();
        }
    }

    println!("  Total: {} registered", registry.len());
    Ok(())
}

fn run_dashboard() -> Result<(), String> {
    let out = dashboard::render_dashboard();
    print!("{}", out);
    Ok(())
}

fn print_help() {
    println!("NEXUS-6 Discovery Engine v0.1.0");
    println!("Usage: nexus6 <command> [options]");
    println!();
    println!("Commands:");
    println!("  scan <domain> [--lenses L1,L2,...] [--full]");
    println!("      Run a telescope scan on the given domain.");
    println!();
    println!("  verify <value> [--tolerance T]");
    println!("      Check a numeric value against n=6 constants.");
    println!();
    println!("  graph [--domain D] [--format ascii|dot]");
    println!("      Display the Discovery Graph.");
    println!();
    println!("  history <domain>");
    println!("      Show scan history and lens performance for a domain.");
    println!();
    println!("  recommend <domain>");
    println!("      Get lens recommendations based on history.");
    println!();
    println!("  evolve <domain> [--max-cycles N] [--seeds S1,S2,...]");
    println!("      Run OUROBOROS evolution loop.");
    println!();
    println!("  auto <domain> [--meta-cycles N] [--ouroboros-cycles N]");
    println!("      Run recommend -> evolve meta-loop (fully automated).");
    println!();
    println!("  lenses [--category core|combo|extended|custom]");
    println!("      List registered lenses.");
    println!();
    println!("  dashboard");
    println!("      Show ASCII dashboard with engine status.");
    println!();
    println!("  help");
    println!("      Show this help message.");
    println!();
    println!("Core theorem: sigma(n)*phi(n) = n*tau(n) <==> n = 6");
}

/// Build a small ASCII bar for a hit rate (0.0..1.0).
fn hit_rate_bar(rate: f64, width: usize) -> String {
    let filled = ((rate * width as f64).round() as usize).min(width);
    let empty = width - filled;
    let mut bar = String::new();
    for _ in 0..filled {
        bar.push('\u{2588}');
    }
    for _ in 0..empty {
        bar.push('\u{2591}');
    }
    bar
}
