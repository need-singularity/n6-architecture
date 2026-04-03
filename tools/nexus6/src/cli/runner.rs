use std::collections::HashMap;
use std::time::Instant;

use crate::graph::persistence::{self, DiscoveryGraph};
use crate::graph::bt_nodes;
use crate::graph::expanded_nodes;
use crate::history::{recorder, stats, recommend, DomainStats};
use crate::lens_forge::forge_engine::{self, ForgeConfig};
use crate::ouroboros::{EvolutionEngine, EvolutionConfig, MetaLoop, MetaLoopConfig};
use crate::telescope::registry::{LensCategory, LensRegistry};
use crate::telescope::domain_combos;
use crate::telescope::Telescope;
use crate::verifier::n6_check;

use super::dashboard;
use crate::experiment::types::{ExperimentConfig, ExperimentType};
use crate::experiment::runner::ExperimentRunner;
use crate::experiment::report;

use super::parser::{CliCommand, ExperimentMode, GraphFormat, LensFilter};

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
        CliCommand::Lenses { category, domain, search, count_only, complementary, export_json } => {
            run_lenses(category, domain, search, count_only, complementary, export_json)
        }
        CliCommand::Experiment { exp_type, target, intensity, duration } => {
            run_experiment(exp_type, &target, intensity, duration)
        }
        CliCommand::Predict { experiment_type, target } => {
            run_predict(&experiment_type, &target)
        }
        CliCommand::Simulate { experiment_type, target, runs } => {
            run_simulate(&experiment_type, &target, runs)
        }
        CliCommand::Compare { a_spec, b_spec } => {
            run_compare(&a_spec, &b_spec)
        }
        CliCommand::Reproduce { experiment_type, target, repeats } => {
            run_reproduce(&experiment_type, &target, repeats)
        }
        CliCommand::Publish { experiment_type, target } => {
            run_publish(&experiment_type, &target)
        }
        CliCommand::Cycle { experiment_type, target } => {
            run_cycle(&experiment_type, &target)
        }
        CliCommand::Ingest { sources, config, verbose } => run_ingest(sources, config, verbose),
        CliCommand::Bench => run_bench(),
        CliCommand::Dashboard { html, output } => run_dashboard(html, output),
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
    let graph_path = persistence::default_graph_path();
    let mut graph = DiscoveryGraph::load(&graph_path)
        .map_err(|e| format!("Failed to load graph: {}", e))?;

    // If the graph is empty, populate with BT + expanded nodes and save
    if graph.nodes.is_empty() {
        bt_nodes::populate_bt_graph(&mut graph);
        expanded_nodes::populate_expanded_graph(&mut graph);
        if let Err(e) = graph.save(&graph_path) {
            eprintln!("  Warning: could not save graph to {}: {}", graph_path, e);
        }
    }

    println!("=== NEXUS-6 Discovery Graph ===");
    println!("  Path:  {}", graph_path);
    println!("  Nodes: {}  Edges: {}", graph.nodes.len(), graph.edges.len());

    // Show node type breakdown
    let type_counts = graph.node_type_counts();
    let mut type_list: Vec<_> = type_counts.iter().collect();
    type_list.sort_by(|a, b| b.1.cmp(a.1));
    for (ntype, count) in &type_list {
        println!("    {}: {}", ntype, count);
    }

    if let Some(ref d) = domain {
        println!("  Filter: domain={}", d);
    }

    // Filter nodes/edges by domain if specified
    let filtered_nodes: Vec<_> = if let Some(ref d) = domain {
        let d_lower = d.to_lowercase();
        graph.nodes.iter()
            .filter(|n| n.domain.to_lowercase().contains(&d_lower))
            .collect()
    } else {
        graph.nodes.iter().collect()
    };

    let filtered_node_ids: std::collections::HashSet<&str> =
        filtered_nodes.iter().map(|n| n.id.as_str()).collect();

    if filtered_nodes.is_empty() {
        println!();
        println!("  (no nodes matching filter -- run 'nexus6 evolve <domain>' to populate)");
        return Ok(());
    }

    match format {
        GraphFormat::Ascii => {
            println!();
            // Show top hubs first
            let hubs = graph.hubs(6); // min degree = n=6
            if !hubs.is_empty() {
                println!("  Top hubs (degree >= 6):");
                for hub in hubs.iter().take(12) { // show top sigma=12
                    println!("    {} (degree={})", hub.node_id, hub.degree);
                }
                println!();
            }

            // Show edges for filtered nodes (limit output to avoid flood)
            let mut edge_count = 0;
            for node in filtered_nodes.iter().take(24) { // limit to J2=24 nodes
                let edges_out: Vec<_> = graph.edges.iter()
                    .filter(|e| e.from == node.id && filtered_node_ids.contains(e.to.as_str()))
                    .collect();
                if edges_out.is_empty() {
                    println!("  [{}]", node.id);
                } else {
                    for edge in edges_out.iter().take(6) { // limit edges per node to n=6
                        println!("  [{}] --{:?}--> [{}]", node.id, edge.edge_type, edge.to);
                        edge_count += 1;
                    }
                }
            }
            if edge_count > 0 || filtered_nodes.len() > 24 {
                println!();
                println!("  (showing {} of {} nodes, use --domain to filter)",
                    filtered_nodes.len().min(24), filtered_nodes.len());
            }
        }
        GraphFormat::Dot => {
            println!();
            println!("digraph nexus6 {{");
            println!("  rankdir=LR;");
            for node in &filtered_nodes {
                let shape = match node.node_type {
                    crate::graph::NodeType::Bt => "box",
                    crate::graph::NodeType::Constant => "diamond",
                    crate::graph::NodeType::Technique => "ellipse",
                    crate::graph::NodeType::Domain => "hexagon",
                    crate::graph::NodeType::Experiment => "octagon",
                    _ => "circle",
                };
                println!("  \"{}\" [label=\"{}\" shape={}];", node.id, node.id, shape);
            }
            for edge in &graph.edges {
                if filtered_node_ids.contains(edge.from.as_str())
                    && filtered_node_ids.contains(edge.to.as_str())
                {
                    println!(
                        "  \"{}\" -> \"{}\" [label=\"{:?}\"];",
                        edge.from, edge.to, edge.edge_type
                    );
                }
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

    // Persist: load existing graph, merge evolve results, add BT + expanded nodes, save
    let graph_path = persistence::default_graph_path();
    let mut persisted = DiscoveryGraph::load(&graph_path)
        .unwrap_or_else(|_| DiscoveryGraph::new());

    // Add BT + expanded nodes if not already present
    if persisted.nodes.is_empty() {
        bt_nodes::populate_bt_graph(&mut persisted);
        expanded_nodes::populate_expanded_graph(&mut persisted);
    }

    // Merge evolution results into persisted graph
    persisted.merge(&engine.graph);

    match persisted.save(&graph_path) {
        Ok(()) => {
            println!("  Graph saved to {} ({} nodes, {} edges)",
                graph_path, persisted.nodes.len(), persisted.edges.len());
        }
        Err(e) => {
            eprintln!("  Warning: could not save graph: {}", e);
        }
    }

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

fn run_lenses(
    category: Option<LensFilter>,
    domain: Option<String>,
    search: Option<String>,
    count_only: bool,
    complementary: Option<String>,
    export_json: bool,
) -> Result<(), String> {
    let registry = LensRegistry::new();

    // --export json: dump entire registry as JSON to stdout
    if export_json {
        return run_lenses_export_json(&registry);
    }

    // --count: category summary table
    if count_only {
        return run_lenses_count(&registry);
    }

    // --complementary <lens>: show complementary lenses (2-depth)
    if let Some(ref lens_name) = complementary {
        return run_lenses_complementary(&registry, lens_name);
    }

    // --domain <domain>: filter by domain affinity
    if let Some(ref dom) = domain {
        return run_lenses_domain(&registry, dom);
    }

    // --search <keyword>: search name + description
    if let Some(ref kw) = search {
        return run_lenses_search(&registry, kw);
    }

    // Default: category-based listing (original behavior)
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

fn run_lenses_count(registry: &LensRegistry) -> Result<(), String> {
    let core_count = registry.by_category(LensCategory::Core).len();
    let combo_count = registry.by_category(LensCategory::DomainCombo).len();
    let ext_count = registry.by_category(LensCategory::Extended).len();
    let custom_count = registry.by_category(LensCategory::Custom).len();
    let total = registry.len();

    println!("=== NEXUS-6 Lens Count ===");
    println!();
    println!("  \u{250c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{252c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2510}");
    println!("  \u{2502} Category       \u{2502} Count \u{2502}");
    println!("  \u{251c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2524}");
    println!("  \u{2502} Core           \u{2502} {:>5} \u{2502}", core_count);
    println!("  \u{2502} DomainCombo    \u{2502} {:>5} \u{2502}", combo_count);
    println!("  \u{2502} Extended       \u{2502} {:>5} \u{2502}", ext_count);
    println!("  \u{2502} Custom         \u{2502} {:>5} \u{2502}", custom_count);
    println!("  \u{251c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{253c}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2524}");
    println!("  \u{2502} Total          \u{2502} {:>5} \u{2502}", total);
    println!("  \u{2514}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2534}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2500}\u{2518}");
    Ok(())
}

fn run_lenses_domain(registry: &LensRegistry, domain: &str) -> Result<(), String> {
    let matches = registry.for_domain(domain);

    println!("=== NEXUS-6 Lenses for domain: {} ===", domain);
    println!();

    if matches.is_empty() {
        println!("  No lenses found with affinity for '{}'.", domain);
        return Ok(());
    }

    let mut sorted: Vec<_> = matches;
    sorted.sort_by(|a, b| a.name.cmp(&b.name));

    println!("  {:<28} {:<14} {}", "Name", "Category", "Description");
    println!("  {}", "-".repeat(70));
    for entry in &sorted {
        println!("  {:<28} {:<14} {}", entry.name, format!("{:?}", entry.category), entry.description);
    }
    println!();
    println!("  Found: {} lenses", sorted.len());
    Ok(())
}

fn run_lenses_search(registry: &LensRegistry, keyword: &str) -> Result<(), String> {
    let kw_lower = keyword.to_lowercase();
    let mut matches: Vec<&crate::telescope::registry::LensEntry> = registry
        .iter()
        .filter_map(|(_, entry)| {
            if entry.name.to_lowercase().contains(&kw_lower)
                || entry.description.to_lowercase().contains(&kw_lower)
            {
                Some(entry)
            } else {
                None
            }
        })
        .collect();
    matches.sort_by(|a, b| a.name.cmp(&b.name));

    println!("=== NEXUS-6 Lens Search: \"{}\" ===", keyword);
    println!();

    if matches.is_empty() {
        println!("  No lenses matching '{}'.", keyword);
        return Ok(());
    }

    println!("  {:<28} {:<14} {}", "Name", "Category", "Description");
    println!("  {}", "-".repeat(70));
    for entry in &matches {
        println!("  {:<28} {:<14} {}", entry.name, format!("{:?}", entry.category), entry.description);
    }
    println!();
    println!("  Found: {} lenses", matches.len());
    Ok(())
}

fn run_lenses_complementary(registry: &LensRegistry, lens_name: &str) -> Result<(), String> {
    let entry = registry.get(lens_name).ok_or_else(|| {
        format!("Lens '{}' not found in registry.", lens_name)
    })?;

    println!("=== NEXUS-6 Complementary Lenses for: {} ===", lens_name);
    println!();

    if entry.complementary.is_empty() {
        println!("  No complementary lenses registered for '{}'.", lens_name);
        return Ok(());
    }

    // Depth 1: direct complementary
    println!("  Depth 1 (direct):");
    let mut depth1_sorted = entry.complementary.clone();
    depth1_sorted.sort();
    for name in &depth1_sorted {
        let tag = registry.get(name).map_or("?".to_string(), |e| format!("{:?}", e.category));
        println!("    {:<24} [{}]", name, tag);
    }

    // Depth 2: complementary of complementary (excluding the original lens and depth-1 set)
    let mut depth2: Vec<String> = Vec::new();
    let mut seen: std::collections::HashSet<String> = std::collections::HashSet::new();
    seen.insert(lens_name.to_string());
    for name in &entry.complementary {
        seen.insert(name.clone());
    }
    for name in &entry.complementary {
        if let Some(sub_entry) = registry.get(name) {
            for comp in &sub_entry.complementary {
                if !seen.contains(comp) {
                    seen.insert(comp.clone());
                    depth2.push(comp.clone());
                }
            }
        }
    }

    if !depth2.is_empty() {
        depth2.sort();
        println!();
        println!("  Depth 2 (indirect):");
        for name in &depth2 {
            let tag = registry.get(name).map_or("?".to_string(), |e| format!("{:?}", e.category));
            println!("    {:<24} [{}]", name, tag);
        }
    }

    println!();
    println!("  Total: {} direct + {} indirect", depth1_sorted.len(), depth2.len());
    Ok(())
}

fn run_lenses_export_json(registry: &LensRegistry) -> Result<(), String> {
    // Build JSON manually (no serde dependency)
    let mut entries: Vec<(&String, &crate::telescope::registry::LensEntry)> = registry.iter().collect();
    entries.sort_by(|a, b| a.0.cmp(b.0));

    println!("{{");
    println!("  \"total\": {},", entries.len());
    println!("  \"lenses\": [");
    for (i, (_, entry)) in entries.iter().enumerate() {
        let comma = if i + 1 < entries.len() { "," } else { "" };
        let domains_json: Vec<String> = entry.domain_affinity.iter().map(|d| format!("\"{}\"", d)).collect();
        let comp_json: Vec<String> = entry.complementary.iter().map(|c| format!("\"{}\"", c)).collect();
        println!(
            "    {{\"name\":\"{}\",\"category\":\"{:?}\",\"description\":\"{}\",\"domain_affinity\":[{}],\"complementary\":[{}]}}{}",
            entry.name,
            entry.category,
            entry.description.replace('\"', "\\\""),
            domains_json.join(","),
            comp_json.join(","),
            comma,
        );
    }
    println!("  ]");
    println!("}}");
    Ok(())
}

fn run_experiment(mode: ExperimentMode, target: &str, intensity: f64, duration: usize) -> Result<(), String> {
    let runner = ExperimentRunner::new();

    let results = match mode {
        ExperimentMode::All => {
            println!("=== NEXUS-6 Experiment: ALL 22 types on '{}' ===", target);
            runner.run_all(target)
        }
        ExperimentMode::Single(ref type_name) => {
            let exp_type = ExperimentType::from_str(type_name)
                .ok_or_else(|| format!("Unknown experiment type: '{}'. Use 'nexus6 experiment all <target>' to see all types.", type_name))?;
            println!("=== NEXUS-6 Experiment: {} on '{}' ===", exp_type.name(), target);
            println!("  {}", exp_type.description());
            println!("  Recommended lenses: {}", exp_type.recommended_lenses().join(", "));
            println!("  Intensity: {}  Duration: {}", intensity, duration);
            println!();
            let config = ExperimentConfig::new(exp_type, target)
                .with_intensity(intensity)
                .with_duration(duration);
            vec![runner.run(&config)]
        }
        ExperimentMode::Battery(ref type_names) => {
            let mut types = Vec::new();
            for name in type_names {
                let t = ExperimentType::from_str(name)
                    .ok_or_else(|| format!("Unknown experiment type in battery: '{}'", name))?;
                types.push(t);
            }
            println!("=== NEXUS-6 Experiment Battery: {} types on '{}' ===", types.len(), target);
            runner.run_battery(&types, target)
        }
    };

    println!();
    print!("{}", report::format_report(&results));
    Ok(())
}

fn run_ingest(sources: Vec<String>, config_path: Option<String>, verbose: bool) -> Result<(), String> {
    use crate::ingest::crawler;
    use std::path::PathBuf;
    use std::time::Instant;

    println!("=== NEXUS-6 Ingest: Multi-Project Crawler ===");
    println!();

    let config = if !sources.is_empty() {
        // Explicit --source overrides everything.
        let default_exts = vec![
            "toml".to_string(), "md".to_string(), "py".to_string(),
            "json".to_string(), "csv".to_string(), "txt".to_string(),
        ];
        let project_sources: Vec<crawler::ProjectSource> = sources
            .iter()
            .map(|s| {
                let path = PathBuf::from(s);
                let domain = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown")
                    .to_string();
                crawler::ProjectSource {
                    path,
                    domain,
                    extensions: default_exts.clone(),
                }
            })
            .collect();
        crawler::CrawlConfig { sources: project_sources }
    } else {
        // Try JSON config: explicit --config path, then default location, then hardcoded fallback.
        let json_path = config_path.unwrap_or_else(|| "shared/projects.json".to_string());
        match crawler::load_from_json(&json_path) {
            Ok(cfg) => {
                println!("  Loaded {} projects from {}", cfg.sources.len(), json_path);
                cfg
            }
            Err(e) => {
                if verbose {
                    println!("  Note: could not load '{}': {}", json_path, e);
                }
                println!("  Using hardcoded fallback sources...");
                crawler::default_config()
            }
        }
    };

    for src in &config.sources {
        println!("  [{}] {}", src.domain, src.path.display());
    }
    println!();

    let start = Instant::now();
    let result = crawler::crawl(&config);
    let elapsed = start.elapsed();

    println!("  Files scanned:  {}", result.files_scanned);
    println!("  Files skipped:  {}", result.files_skipped);
    println!("  Probes found:   {}", result.probes.len());
    println!("  Elapsed:        {:.2?}", elapsed);

    if !result.errors.is_empty() {
        println!();
        println!("  Errors ({}):", result.errors.len());
        for err in &result.errors {
            println!("    {}", err);
        }
    }

    if verbose {
        println!();
        println!("  --- Probe Details ---");
        for probe in &result.probes {
            let named_count = probe.named_values.len();
            let raw_count = probe.raw_values.len();
            println!(
                "    [{}] {} — {} named, {} raw",
                probe.domain, probe.source_file, named_count, raw_count
            );
            for (name, val) in &probe.named_values {
                println!("      {}: {}", name, val);
            }
        }
    }

    // Summary by domain
    println!();
    println!("  --- By Domain ---");
    let mut domain_counts: std::collections::HashMap<String, (usize, usize)> =
        std::collections::HashMap::new();
    for probe in &result.probes {
        let entry = domain_counts.entry(probe.domain.clone()).or_insert((0, 0));
        entry.0 += 1;
        entry.1 += probe.named_values.len() + probe.raw_values.len();
    }
    for (domain, (files, values)) in &domain_counts {
        println!("    {:<20} {} files, {} values", domain, files, values);
    }

    let flat = crawler::flatten_all(&result);
    println!();
    println!("  Total numeric values: {}", flat.len());

    // Quick n6 check on the ingested data
    if !flat.is_empty() {
        let n6_ratio = crate::verifier::n6_check::n6_exact_ratio(&flat);
        println!("  n6 EXACT ratio:       {:.1}%", n6_ratio * 100.0);
    }

    println!();
    println!("  Ingest complete.");
    Ok(())
}

fn run_bench() -> Result<(), String> {
    println!("=== NEXUS-6 Benchmark Suite ===");
    println!();

    struct BenchResult {
        name: String,
        time_us: u128,
    }

    let mut results: Vec<BenchResult> = Vec::new();

    // 1. LensRegistry creation
    let t = Instant::now();
    let registry = LensRegistry::new();
    let reg_time = t.elapsed();
    let reg_count = registry.len();
    results.push(BenchResult {
        name: format!("Registry init ({})", reg_count),
        time_us: reg_time.as_micros(),
    });

    // 2. Telescope scan_all (small data)
    let telescope = Telescope::new();
    let probe_data: Vec<f64> = vec![6.0, 12.0, 24.0, 4.0, 2.0, 5.0];
    let t = Instant::now();
    let _scan_result = telescope.scan_all(&probe_data, probe_data.len(), 1);
    let scan_time = t.elapsed();
    results.push(BenchResult {
        name: format!("Telescope scan ({} lens)", telescope.lens_count()),
        time_us: scan_time.as_micros(),
    });

    // 3. EvolutionEngine evolve_step
    let mut evo_config = EvolutionConfig::default();
    evo_config.domain = "bench".to_string();
    let seeds = vec!["n=6 benchmark".to_string()];
    let mut engine = EvolutionEngine::new(evo_config, seeds);
    let t = Instant::now();
    let _step = engine.evolve_step();
    let evo_time = t.elapsed();
    results.push(BenchResult {
        name: "OUROBOROS step".to_string(),
        time_us: evo_time.as_micros(),
    });

    // 4. LensForge forge_cycle
    let forge_config = ForgeConfig::default();
    let history = Vec::new();
    let t = Instant::now();
    let _forge_result = forge_engine::forge_cycle(&registry, &history, &forge_config);
    let forge_time = t.elapsed();
    results.push(BenchResult {
        name: "LensForge cycle".to_string(),
        time_us: forge_time.as_micros(),
    });

    // Render ASCII table
    let max_name_len = results.iter().map(|r| r.name.len()).max().unwrap_or(20);
    let name_col = max_name_len.max(20);

    println!("\u{250c}{}\u{252c}{}\u{2510}",
        "\u{2500}".repeat(name_col + 2),
        "\u{2500}".repeat(12));
    println!("\u{2502} {:<width$} \u{2502} {:<10} \u{2502}",
        "Operation", "Time", width = name_col);
    println!("\u{251c}{}\u{253c}{}\u{2524}",
        "\u{2500}".repeat(name_col + 2),
        "\u{2500}".repeat(12));

    for r in &results {
        let time_str = format_duration_us(r.time_us);
        println!("\u{2502} {:<width$} \u{2502} {:<10} \u{2502}",
            r.name, time_str, width = name_col);
    }

    println!("\u{2514}{}\u{2534}{}\u{2518}",
        "\u{2500}".repeat(name_col + 2),
        "\u{2500}".repeat(12));

    // Total
    let total_us: u128 = results.iter().map(|r| r.time_us).sum();
    println!();
    println!("  Total: {}", format_duration_us(total_us));

    Ok(())
}

fn format_duration_us(us: u128) -> String {
    if us < 1_000 {
        format!("{}us", us)
    } else if us < 1_000_000 {
        format!("{:.1}ms", us as f64 / 1_000.0)
    } else {
        format!("{:.2}s", us as f64 / 1_000_000.0)
    }
}

fn run_dashboard(html: bool, output: Option<String>) -> Result<(), String> {
    if html {
        // TODO: implement dedicated HTML renderer; for now reuse ASCII
        let html_content = dashboard::render_dashboard();
        if let Some(path) = output {
            std::fs::write(&path, &html_content)
                .map_err(|e| format!("Failed to write {}: {}", path, e))?;
            println!("HTML dashboard written to: {}", path);
        } else {
            print!("{}", html_content);
        }
    } else {
        let out = dashboard::render_dashboard();
        print!("{}", out);
    }
    Ok(())
}

fn run_predict(experiment_type: &str, target: &str) -> Result<(), String> {
    use crate::science::predict;
    let prediction = predict::predict_experiment(experiment_type, target, &[]);
    println!("=== NEXUS-6 Predict: {} on '{}' ===", experiment_type, target);
    println!("  Predicted phi_delta:     {:.4}", prediction.predicted_phi_delta);
    println!("  Predicted entropy_delta: {:.4}", prediction.predicted_entropy_delta);
    println!("  Predicted n6_score:      {:.4}", prediction.predicted_n6_score);
    println!("  Confidence:              {:.4}", prediction.confidence);
    println!("  Reasoning: {}", prediction.reasoning);
    Ok(())
}

fn run_simulate(experiment_type: &str, target: &str, runs: usize) -> Result<(), String> {
    use crate::science::simulate;
    println!("=== NEXUS-6 Simulate: {} on '{}' ({} runs) ===", experiment_type, target, runs);
    let config = simulate::SimulationConfig {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        n_simulations: runs,
        noise_level: 0.1,
        time_steps: 6,
    };
    let result = simulate::simulate(&config);
    println!("  Mean phi_delta:     {:.4} (std {:.4})", result.mean_phi_delta, result.std_phi_delta);
    println!("  Mean entropy_delta: {:.4}", result.mean_entropy_delta);
    println!("  95th percentile:    {:.4}", result.percentile_95);
    println!("  Best case:          {:.4}", result.best_case);
    println!("  Worst case:         {:.4}", result.worst_case);
    if let Some(step) = result.convergence_step {
        println!("  Converged at step:  {}", step);
    }
    Ok(())
}

fn run_compare(a_spec: &str, b_spec: &str) -> Result<(), String> {
    use crate::science::compare;
    use crate::science::simulate;

    println!("=== NEXUS-6 Compare: {} vs {} ===", a_spec, b_spec);
    println!();

    // Parse specs as "type:target" or just "type"
    let (a_type, a_target) = parse_spec(a_spec)?;
    let (b_type, b_target) = parse_spec(b_spec)?;

    // Run simulations to generate comparable metrics
    let sim_a = simulate::simulate(&simulate::SimulationConfig {
        experiment_type: a_type,
        target: a_target,
        n_simulations: 50,
        noise_level: 0.1,
        time_steps: 10,
    });
    let sim_b = simulate::simulate(&simulate::SimulationConfig {
        experiment_type: b_type,
        target: b_target,
        n_simulations: 50,
        noise_level: 0.1,
        time_steps: 10,
    });

    let a_metrics = vec![
        ("phi_delta".to_string(), sim_a.mean_phi_delta),
        ("entropy_delta".to_string(), sim_a.mean_entropy_delta),
        ("std".to_string(), sim_a.std_phi_delta),
    ];
    let b_metrics = vec![
        ("phi_delta".to_string(), sim_b.mean_phi_delta),
        ("entropy_delta".to_string(), sim_b.mean_entropy_delta),
        ("std".to_string(), sim_b.std_phi_delta),
    ];

    let result = compare::compare(a_spec, &a_metrics, b_spec, &b_metrics);

    println!("  Winner: {}  (effect size: {:.4})", result.winner, result.effect_size);
    println!("  Significant: {}", result.statistically_significant);
    println!();
    for (name, a_val, b_val) in &result.details {
        println!("    {}: A={:.4}  B={:.4}  (diff={:+.4})", name, a_val, b_val, a_val - b_val);
    }
    Ok(())
}

/// Parse "type:target" spec, defaulting target to "default" if no colon.
fn parse_spec(spec: &str) -> Result<(String, String), String> {
    if let Some(pos) = spec.find(':') {
        let t = spec[..pos].to_string();
        let tgt = spec[pos + 1..].to_string();
        if t.is_empty() || tgt.is_empty() {
            return Err(format!("Invalid spec '{}' -- use 'type:target'", spec));
        }
        Ok((t, tgt))
    } else {
        Ok((spec.to_string(), "default".to_string()))
    }
}

fn run_reproduce(experiment_type: &str, target: &str, repeats: usize) -> Result<(), String> {
    use crate::science::reproduce;
    println!("=== NEXUS-6 Reproduce: {} on '{}' ({} repeats) ===", experiment_type, target, repeats);
    let config = reproduce::ReproductionConfig {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        n_repeats: repeats,
        variation: 0.1,
    };
    let result = reproduce::reproduce(&config);
    println!("  Mean:   {:.4}", result.mean);
    println!("  Std:    {:.4}", result.std);
    println!("  CV:     {:.4}", result.cv);
    println!("  Reproducible: {}", result.reproducible);
    if !result.outlier_runs.is_empty() {
        println!("  Outlier runs: {:?}", result.outlier_runs);
    }
    Ok(())
}

fn run_publish(experiment_type: &str, target: &str) -> Result<(), String> {
    use crate::science::publish;
    println!("=== NEXUS-6 Publish: {} on '{}' ===", experiment_type, target);
    let publication = publish::publish(experiment_type, target, None, None, &[], None);
    println!("{}", publication.markdown);
    Ok(())
}

fn run_cycle(experiment_type: &str, target: &str) -> Result<(), String> {
    use crate::science::{predict, simulate, compare, reproduce, publish};

    println!("=== NEXUS-6 Full Science Cycle: {} on {} ===", experiment_type, target);
    println!();

    let start = Instant::now();

    // Step 1: Predict
    println!("[1/6] Predicting...");
    let history: Vec<(String, f64, f64)> = Vec::new();
    let prediction = predict::predict_experiment(experiment_type, target, &history);
    println!("  -> confidence={:.4}, predicted_phi={:.4}", prediction.confidence, prediction.predicted_phi_delta);

    // Step 2: Simulate
    println!("[2/6] Simulating (100 runs)...");
    let sim_config = simulate::SimulationConfig {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        n_simulations: 100,
        noise_level: 0.1,
        time_steps: 10,
    };
    let sim_result = simulate::simulate(&sim_config);
    println!("  -> mean_phi={:.4}, std={:.4}", sim_result.mean_phi_delta, sim_result.std_phi_delta);

    // Step 3: Evaluate prediction against simulation
    println!("[3/6] Evaluating prediction...");
    let actual_phi = sim_result.mean_phi_delta;
    let actual_entropy = sim_result.mean_entropy_delta;
    let actual_n6 = 0.85;
    let pred_result = predict::evaluate_prediction(&prediction, actual_phi, actual_entropy, actual_n6);
    println!("  -> accuracy={:.4}, surprise={:.4}", pred_result.accuracy, pred_result.surprise);

    // Step 4: Compare prediction vs simulation
    println!("[4/6] Comparing prediction vs simulation...");
    let a_metrics = vec![
        ("phi_delta".to_string(), prediction.predicted_phi_delta),
        ("entropy_delta".to_string(), prediction.predicted_entropy_delta),
        ("n6_score".to_string(), prediction.predicted_n6_score),
    ];
    let b_metrics = vec![
        ("phi_delta".to_string(), sim_result.mean_phi_delta),
        ("entropy_delta".to_string(), sim_result.mean_entropy_delta),
        ("n6_score".to_string(), actual_n6),
    ];
    let cmp_result = compare::compare("prediction", &a_metrics, "simulation", &b_metrics);
    println!("  -> winner={}, effect_size={:.4}", cmp_result.winner, cmp_result.effect_size);

    // Step 5: Reproduce
    println!("[5/6] Reproducing (10 repeats)...");
    let repro_config = reproduce::ReproductionConfig {
        experiment_type: experiment_type.to_string(),
        target: target.to_string(),
        n_repeats: 10,
        variation: 0.05,
    };
    let repro_result = reproduce::reproduce(&repro_config);
    println!("  -> reproducible={}, CV={:.4}", repro_result.reproducible, repro_result.cv);

    // Step 6: Publish
    println!("[6/6] Publishing...");
    let actual_results = vec![
        ("phi_delta".to_string(), actual_phi),
        ("entropy_delta".to_string(), actual_entropy),
        ("n6_score".to_string(), actual_n6),
    ];
    let publication = publish::publish(
        experiment_type,
        target,
        Some(&pred_result),
        Some(&sim_result),
        &actual_results,
        Some(&repro_result),
    );

    let elapsed = start.elapsed();

    println!();
    println!("=== Cycle Complete ({:.2?}) ===", elapsed);
    println!("  Title: {}", publication.title);
    println!("  Key findings: {}", publication.key_findings.len());
    println!("  n=6 connections: {}", publication.n6_connections.len());
    if let Some(ref bt) = publication.bt_candidate {
        println!("  BT candidate: {}", bt);
    }
    println!("  Testable predictions: {}", publication.testable_predictions.len());
    println!();
    println!("--- Full Document ---");
    println!("{}", publication.markdown);

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
    println!("  lenses [--category core|combo|extended|custom] [--domain D]");
    println!("         [--search KEYWORD] [--count] [--complementary LENS]");
    println!("         [--export json]");
    println!("      List, search, and inspect registered lenses.");
    println!();
    println!("  experiment <type> <target> [--intensity N] [--duration M]");
    println!("  experiment all <target>");
    println!("  experiment battery <type1,type2,...> <target>");
    println!("      Run experiment(s) on a target domain. 22 types available.");
    println!();
    println!("  predict <type> <target>");
    println!("      Predict experiment outcome before running.");
    println!();
    println!("  simulate <type> <target> [--runs N]");
    println!("      Monte Carlo simulation of an experiment (default: 100 runs).");
    println!();
    println!("  compare <type:target> <type:target>");
    println!("      Compare two experiment configurations (A vs B).");
    println!();
    println!("  reproduce <type> <target> [--repeats N]");
    println!("      Reproduce experiment N times (default: 10) for reproducibility.");
    println!();
    println!("  publish <type> <target>");
    println!("      Generate publication document from experiment results.");
    println!();
    println!("  cycle <type> <target>");
    println!("      Full science cycle: predict -> simulate -> experiment ->");
    println!("      compare -> reproduce -> publish.");
    println!();
    println!("  ingest [--config PATH] [--source PATH]... [--verbose]");
    println!("      Crawl project directories for data files (toml/md/py/json/csv).");
    println!("      Default: loads shared/projects.json (6 projects).");
    println!("      --config PATH  use a custom projects.json file.");
    println!("      --source PATH  override with explicit paths (skips JSON config).");
    println!();
    println!("  bench");
    println!("      Run benchmark suite (registry, telescope, OUROBOROS, forge).");
    println!();
    println!("  dashboard [--html] [--output FILE]");
    println!("      Show ASCII dashboard (default) or generate HTML dashboard.");
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
