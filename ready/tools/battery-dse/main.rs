#![allow(dead_code)]

/// HEXA-BATTERY Design Space Exploration (DSE)
///
/// 소재 × 공정 × 코어 × 칩 × 시스템 체인에서
/// 각 레벨의 후보군을 전수 조합하여 최적 배터리 설계를 도출.
///
/// 평가: n6_EXACT 비율 + 성능 + 비용 + 안전 + 수명 + 성숙도
/// 출력: Pareto frontier + 최적 경로 + n6 분석

// N6 constants
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const J2: u64 = 24;
const SIGMA_TAU: u64 = 8;          // sigma - tau
const SIGMA_PHI: u64 = 10;         // sigma - phi
const SIGMA_TIMES_TAU: u64 = 48;   // sigma * tau
const SIGMA_SIGMA_TAU: u64 = 96;   // sigma * (sigma - tau)

// ============================================================
// Level 1: 소재 (Material) — 6 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Material {
    name: &'static str,
    cn: u64,              // coordination number
    n6_matches: u64,      // count of n=6 parameter matches
    n6_total: u64,        // total n=6 parameters checked
    energy_density: f64,  // Wh/kg
    cycle_life: u64,      // cycles to 80%
    cost_relative: f64,   // 1.0 = NMC baseline
    safety: f64,          // 0-1
}

const MATERIALS: &[Material] = &[
    Material { name: "LFP",     cn: 6,  n6_matches: 4, n6_total: 4, energy_density: 170.0, cycle_life: 4000, cost_relative: 0.65, safety: 0.95 },
    Material { name: "NMC811",  cn: 6,  n6_matches: 3, n6_total: 4, energy_density: 280.0, cycle_life: 1000, cost_relative: 1.00, safety: 0.70 },
    Material { name: "NCA",     cn: 6,  n6_matches: 3, n6_total: 4, energy_density: 260.0, cycle_life: 1500, cost_relative: 0.95, safety: 0.72 },
    Material { name: "LCO",     cn: 6,  n6_matches: 4, n6_total: 4, energy_density: 200.0, cycle_life: 500,  cost_relative: 1.20, safety: 0.65 },
    Material { name: "Na-ion",  cn: 6,  n6_matches: 3, n6_total: 4, energy_density: 140.0, cycle_life: 3000, cost_relative: 0.45, safety: 0.92 },
    Material { name: "Li-S",    cn: 8,  n6_matches: 5, n6_total: 6, energy_density: 500.0, cycle_life: 300,  cost_relative: 0.55, safety: 0.50 },
];

// ============================================================
// Level 2: 공정 (Process/Electrode) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Process {
    name: &'static str,
    anode_type: &'static str,
    n6_matches: u64,
    n6_total: u64,
    capacity_gain: f64,   // multiplier vs graphite
    maturity: f64,        // 0-1 (1.0 = mass production)
    process_cost: f64,    // relative
}

const PROCESSES: &[Process] = &[
    Process { name: "Graphite-Wet",  anode_type: "Graphite",    n6_matches: 2, n6_total: 2, capacity_gain: 1.0, maturity: 1.00, process_cost: 0.50 },
    Process { name: "SiC-10%-Wet",   anode_type: "Si-Graphite", n6_matches: 1, n6_total: 2, capacity_gain: 1.5, maturity: 0.85, process_cost: 0.70 },
    Process { name: "SiC-20%-Dry",   anode_type: "Si-Graphite", n6_matches: 1, n6_total: 2, capacity_gain: 2.0, maturity: 0.55, process_cost: 0.80 },
    Process { name: "Si-SSB",        anode_type: "Si-Solid",    n6_matches: 2, n6_total: 2, capacity_gain: 3.0, maturity: 0.25, process_cost: 1.40 },
    Process { name: "Na-HardCarbon", anode_type: "HardCarbon",  n6_matches: 1, n6_total: 2, capacity_gain: 0.8, maturity: 0.70, process_cost: 0.40 },
];

// ============================================================
// Level 3: 코어 (Core/Cell Form) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Core {
    name: &'static str,
    form_factor: &'static str,
    n6_matches: u64,
    n6_total: u64,
    energy_mult: f64,     // cell-level energy density multiplier
    thermal: f64,         // thermal management ease 0-1
    mfg_cost: f64,        // manufacturing cost relative
}

const CORES: &[Core] = &[
    Core { name: "18650",     form_factor: "Cylindrical", n6_matches: 2, n6_total: 3, energy_mult: 1.00, thermal: 0.85, mfg_cost: 0.60 },
    Core { name: "21700",     form_factor: "Cylindrical", n6_matches: 1, n6_total: 3, energy_mult: 1.15, thermal: 0.82, mfg_cost: 0.65 },
    Core { name: "4680",      form_factor: "Cylindrical", n6_matches: 1, n6_total: 3, energy_mult: 1.30, thermal: 0.70, mfg_cost: 0.70 },
    Core { name: "Prismatic", form_factor: "Prismatic",   n6_matches: 1, n6_total: 3, energy_mult: 1.10, thermal: 0.75, mfg_cost: 0.80 },
    Core { name: "Pouch",     form_factor: "Pouch",       n6_matches: 1, n6_total: 3, energy_mult: 1.25, thermal: 0.65, mfg_cost: 0.85 },
];

// ============================================================
// Level 4: 칩 (Chip/BMS) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Chip {
    name: &'static str,
    channels: u64,
    adc_bits: u64,
    n6_matches: u64,
    n6_total: u64,
    accuracy: f64,        // SOC accuracy 0-1
    features: f64,        // feature richness 0-1
    chip_cost: f64,       // relative
}

const CHIPS: &[Chip] = &[
    Chip { name: "Discrete-6ch",    channels: 6,  adc_bits: 12, n6_matches: 3, n6_total: 4, accuracy: 0.85, features: 0.50, chip_cost: 0.30 },
    Chip { name: "Integrated-12ch", channels: 12, adc_bits: 12, n6_matches: 4, n6_total: 4, accuracy: 0.92, features: 0.80, chip_cost: 0.60 },
    Chip { name: "Wireless-12ch",   channels: 12, adc_bits: 12, n6_matches: 4, n6_total: 4, accuracy: 0.90, features: 0.90, chip_cost: 0.90 },
    Chip { name: "AI-BMS-12ch",     channels: 12, adc_bits: 16, n6_matches: 3, n6_total: 4, accuracy: 0.96, features: 1.00, chip_cost: 1.10 },
    Chip { name: "Minimal-4ch",     channels: 4,  adc_bits: 10, n6_matches: 2, n6_total: 4, accuracy: 0.78, features: 0.35, chip_cost: 0.20 },
];

// ============================================================
// Level 5: 시스템 (System) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct System {
    name: &'static str,
    cell_count: u64,
    voltage: u64,
    n6_matches: u64,
    n6_total: u64,
    scalability: f64,
    grid_compat: f64,
    system_cost: f64,
}

const SYSTEMS: &[System] = &[
    System { name: "48V-ESS",  cell_count: 24,   voltage: 48,   n6_matches: 4, n6_total: 4, scalability: 0.70, grid_compat: 0.85, system_cost: 0.50 },
    System { name: "400V-EV",  cell_count: 96,   voltage: 400,  n6_matches: 3, n6_total: 4, scalability: 0.80, grid_compat: 0.40, system_cost: 0.85 },
    System { name: "800V-EV",  cell_count: 192,  voltage: 800,  n6_matches: 3, n6_total: 4, scalability: 0.85, grid_compat: 0.40, system_cost: 1.10 },
    System { name: "Grid-MW",  cell_count: 3456, voltage: 1000, n6_matches: 3, n6_total: 4, scalability: 1.00, grid_compat: 1.00, system_cost: 2.00 },
    System { name: "DC-Micro", cell_count: 24,   voltage: 48,   n6_matches: 4, n6_total: 4, scalability: 0.55, grid_compat: 0.75, system_cost: 0.35 },
];

// ============================================================
// Compatibility filter
// ============================================================
fn is_compatible(m: &Material, p: &Process, c: &Core, _b: &Chip, s: &System) -> bool {
    // Na-ion only with Na-HardCarbon
    if m.name == "Na-ion" && p.name != "Na-HardCarbon" { return false; }
    if p.name == "Na-HardCarbon" && m.name != "Na-ion" { return false; }
    // Li-S only with Si-SSB
    if m.name == "Li-S" && p.name != "Si-SSB" { return false; }
    // Si-SSB not with Na-ion
    if p.name == "Si-SSB" && m.name == "Na-ion" { return false; }
    // Minimal BMS not for high voltage
    if _b.name == "Minimal-4ch" && (s.name == "400V-EV" || s.name == "800V-EV" || s.name == "Grid-MW") { return false; }
    // 18650 impractical for 800V
    if c.name == "18650" && s.name == "800V-EV" { return false; }
    true
}

// ============================================================
// Scoring
// ============================================================
#[derive(Clone)]
struct Scores {
    n6_pct: f64,
    perf: f64,
    total_cost: f64,
    cost_score: f64,
    safety: f64,
    life: f64,
    maturity: f64,
    pareto: f64,
}

fn score(m: &Material, p: &Process, c: &Core, b: &Chip, s: &System) -> Scores {
    // n6 score: total exact matches / total possible
    let n6_exact = m.n6_matches + p.n6_matches + c.n6_matches + b.n6_matches + s.n6_matches;
    let n6_total = m.n6_total + p.n6_total + c.n6_total + b.n6_total + s.n6_total;
    let n6_pct = n6_exact as f64 / n6_total as f64 * 100.0;

    // Performance: energy_density * capacity_gain * cell_multiplier (normalized to theoretical max)
    let perf = (m.energy_density * p.capacity_gain * c.energy_mult) / (500.0 * 3.0 * 1.30);

    // Cost: sum of all cost components (lower = better)
    let total_cost = m.cost_relative + p.process_cost + c.mfg_cost + b.chip_cost + s.system_cost;
    let cost_score = 1.0 / total_cost; // invert: lower cost -> higher score

    // Safety: material safety * thermal management
    let safety = m.safety * c.thermal;

    // Longevity: cycle life normalized to best (4000)
    let life = m.cycle_life as f64 / 4000.0;

    // Maturity: process maturity
    let maturity = p.maturity;

    // Pareto composite: weighted
    // n6=0.20, perf=0.25, cost=0.15, safety=0.15, life=0.15, maturity=0.10
    let pareto = n6_pct / 100.0 * 0.20
        + perf * 0.25
        + cost_score * 2.0 * 0.15
        + safety * 0.15
        + life * 0.15
        + maturity * 0.10;

    Scores { n6_pct, perf, total_cost, cost_score, safety, life, maturity, pareto }
}

// ============================================================
// Result
// ============================================================
#[derive(Clone)]
struct DseResult {
    mat_name: &'static str,
    proc_name: &'static str,
    core_name: &'static str,
    chip_name: &'static str,
    sys_name: &'static str,
    scores: Scores,
    // Per-level n6 for analysis
    mat_n6: (u64, u64),
    proc_n6: (u64, u64),
    core_n6: (u64, u64),
    chip_n6: (u64, u64),
    sys_n6: (u64, u64),
}

fn format_comma(n: u64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 { result.push(','); }
        result.push(c);
    }
    result.chars().rev().collect()
}

fn main() {
    let total_combos = MATERIALS.len() * PROCESSES.len() * CORES.len() * CHIPS.len() * SYSTEMS.len();

    println!("\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!("  HEXA-BATTERY Design Space Exploration (DSE)");
    println!("  소재 x 공정 x 코어 x 칩 x 시스템 전수 탐색");
    println!("\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}\u{2550}");
    println!();
    println!("  후보군:");
    println!("    소재:   {} ({})",
        MATERIALS.iter().map(|m| m.name).collect::<Vec<_>>().join(", "), MATERIALS.len());
    println!("    공정:   {} ({})",
        PROCESSES.iter().map(|p| p.name).collect::<Vec<_>>().join(", "), PROCESSES.len());
    println!("    코어:   {} ({})",
        CORES.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CORES.len());
    println!("    칩:     {} ({})",
        CHIPS.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CHIPS.len());
    println!("    시스템: {} ({})",
        SYSTEMS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SYSTEMS.len());
    println!();
    println!("  총 조합: {}", format_comma(total_combos as u64));

    // Enumerate all compatible combinations
    let mut results: Vec<DseResult> = Vec::with_capacity(total_combos);

    for mat in MATERIALS {
        for proc in PROCESSES {
            for core in CORES {
                for chip in CHIPS {
                    for sys in SYSTEMS {
                        if !is_compatible(mat, proc, core, chip, sys) {
                            continue;
                        }
                        let s = score(mat, proc, core, chip, sys);
                        results.push(DseResult {
                            mat_name: mat.name,
                            proc_name: proc.name,
                            core_name: core.name,
                            chip_name: chip.name,
                            sys_name: sys.name,
                            mat_n6: (mat.n6_matches, mat.n6_total),
                            proc_n6: (proc.n6_matches, proc.n6_total),
                            core_n6: (core.n6_matches, core.n6_total),
                            chip_n6: (chip.n6_matches, chip.n6_total),
                            sys_n6: (sys.n6_matches, sys.n6_total),
                            scores: s,
                        });
                    }
                }
            }
        }
    }

    let compat_count = results.len();
    println!("  호환 조합: {} (필터 후)", compat_count);
    println!();

    // Sort by pareto score descending
    results.sort_by(|a, b| b.scores.pareto.partial_cmp(&a.scores.pareto).unwrap());

    // ── TOP 20 ──
    let bar = "═".repeat(130);
    println!("{}", bar);
    println!("  TOP 20 (by Pareto score)");
    println!("{}", bar);
    println!();
    println!("  {:>4} \u{2502} {:>8} \u{2502} {:>14} \u{2502} {:>9} \u{2502} {:>15} \u{2502} {:>9} \u{2502} {:>5} \u{2502} {:>5} \u{2502} {:>5} \u{2502} {:>5} \u{2502} {:>5} \u{2502} {:>6}",
        "Rank", "Material", "Process", "Core", "Chip", "System", "n6%", "Perf", "Cost", "Safe", "Life", "Pareto");
    println!("  {}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}\u{253c}{}",
        "\u{2500}".repeat(5), "\u{2500}".repeat(10), "\u{2500}".repeat(16),
        "\u{2500}".repeat(11), "\u{2500}".repeat(17), "\u{2500}".repeat(11),
        "\u{2500}".repeat(7), "\u{2500}".repeat(7), "\u{2500}".repeat(7),
        "\u{2500}".repeat(7), "\u{2500}".repeat(7), "\u{2500}".repeat(7));

    for (i, r) in results.iter().take(20).enumerate() {
        println!("  {:>4} \u{2502} {:>8} \u{2502} {:>14} \u{2502} {:>9} \u{2502} {:>15} \u{2502} {:>9} \u{2502} {:>4.1}% \u{2502} {:>5.3} \u{2502} {:>5.2} \u{2502} {:>5.2} \u{2502} {:>5.2} \u{2502} {:>6.4}",
            i + 1,
            r.mat_name, r.proc_name, r.core_name, r.chip_name, r.sys_name,
            r.scores.n6_pct, r.scores.perf, r.scores.total_cost,
            r.scores.safety, r.scores.life, r.scores.pareto);
    }

    // ── BEST BY CATEGORY ──
    println!();
    println!("{}", bar);
    println!("  BEST BY CATEGORY");
    println!("{}", bar);

    let best_n6 = results.iter().max_by(|a, b|
        a.scores.n6_pct.partial_cmp(&b.scores.n6_pct).unwrap()).unwrap();
    let best_perf = results.iter().max_by(|a, b|
        a.scores.perf.partial_cmp(&b.scores.perf).unwrap()).unwrap();
    let best_cost = results.iter().min_by(|a, b|
        a.scores.total_cost.partial_cmp(&b.scores.total_cost).unwrap()).unwrap();
    let best_safe = results.iter().max_by(|a, b|
        a.scores.safety.partial_cmp(&b.scores.safety).unwrap()).unwrap();
    let best_life = results.iter().max_by(|a, b|
        a.scores.life.partial_cmp(&b.scores.life).unwrap()).unwrap();
    let best_mature = results.iter().max_by(|a, b|
        a.scores.maturity.partial_cmp(&b.scores.maturity).unwrap()).unwrap();

    let fmt_path = |r: &DseResult| -> String {
        format!("{} + {} + {} + {} + {}", r.mat_name, r.proc_name, r.core_name, r.chip_name, r.sys_name)
    };

    // Effective Wh/kg for best perf
    let eff_whkg = |r: &DseResult| -> f64 {
        r.scores.perf * 500.0 * 3.0 * 1.30 // undo normalization
    };

    println!("    Best n6 EXACT:    {} ({:.1}%)", fmt_path(best_n6), best_n6.scores.n6_pct);
    println!("    Best Performance: {} ({:.0} Wh/kg effective)", fmt_path(best_perf), eff_whkg(best_perf));
    println!("    Best Cost:        {} (${:.2} relative)", fmt_path(best_cost), best_cost.scores.total_cost);
    println!("    Best Safety:      {} ({:.2})", fmt_path(best_safe), best_safe.scores.safety);
    println!("    Best Longevity:   {} ({} cycles)", fmt_path(best_life), (best_life.scores.life * 4000.0) as u64);
    println!("    Best Maturity:    {} ({:.2} TRL)", fmt_path(best_mature), best_mature.scores.maturity);

    // ── N6 ANALYSIS ──
    println!();
    println!("{}", bar);
    println!("  N6 ANALYSIS");
    println!("{}", bar);
    println!();

    // Per-level best n6
    let best_mat_n6 = MATERIALS.iter().max_by_key(|m| m.n6_matches * 100 / m.n6_total).unwrap();
    let best_proc_n6 = PROCESSES.iter().max_by_key(|p| p.n6_matches * 100 / p.n6_total).unwrap();
    let best_core_n6 = CORES.iter().max_by_key(|c| c.n6_matches * 100 / c.n6_total).unwrap();
    let best_chip_n6 = CHIPS.iter().max_by_key(|c| c.n6_matches * 100 / c.n6_total).unwrap();
    let best_sys_n6 = SYSTEMS.iter().max_by_key(|s| s.n6_matches * 100 / s.n6_total).unwrap();

    println!("  Per-level best n6:");
    println!("    소재:   {} ({}/{})", best_mat_n6.name, best_mat_n6.n6_matches, best_mat_n6.n6_total);
    println!("    공정:   {} ({}/{})", best_proc_n6.name, best_proc_n6.n6_matches, best_proc_n6.n6_total);
    println!("    코어:   {} ({}/{})", best_core_n6.name, best_core_n6.n6_matches, best_core_n6.n6_total);
    println!("    칩:     {} ({}/{})", best_chip_n6.name, best_chip_n6.n6_matches, best_chip_n6.n6_total);
    println!("    시스템: {} ({}/{})", best_sys_n6.name, best_sys_n6.n6_matches, best_sys_n6.n6_total);
    println!();

    // n6 vs Performance trade-off
    // Find the combination with highest n6% and show its perf rank
    let _best_n6_idx = results.iter().position(|r| std::ptr::eq(r as *const _, best_n6 as *const _)).unwrap_or(0);
    // Find best perf and show its n6 rank
    let mut perf_sorted = results.clone();
    perf_sorted.sort_by(|a, b| b.scores.perf.partial_cmp(&a.scores.perf).unwrap());
    let best_perf_n6 = perf_sorted[0].scores.n6_pct;
    // Find perf rank of best n6 combo
    let best_n6_perf_rank = perf_sorted.iter().position(|r|
        r.mat_name == best_n6.mat_name && r.proc_name == best_n6.proc_name
        && r.core_name == best_n6.core_name && r.chip_name == best_n6.chip_name
        && r.sys_name == best_n6.sys_name
    ).map(|i| i + 1).unwrap_or(0);

    println!("  n6 vs Performance trade-off:");
    println!("    100% n6 path -> perf = {:.3} (perf rank #{})", best_n6.scores.perf, best_n6_perf_rank);
    println!("    Best perf    -> n6 = {:.1}% (pareto rank #{})",
        best_perf_n6,
        results.iter().position(|r|
            r.mat_name == perf_sorted[0].mat_name && r.proc_name == perf_sorted[0].proc_name
            && r.core_name == perf_sorted[0].core_name && r.chip_name == perf_sorted[0].chip_name
            && r.sys_name == perf_sorted[0].sys_name
        ).map(|i| i + 1).unwrap_or(0));

    // ── STATISTICS ──
    println!();
    println!("{}", bar);
    println!("  STATISTICS");
    println!("{}", bar);

    let max_n6 = results.iter().map(|r| r.scores.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = results.iter().map(|r| r.scores.n6_pct).sum::<f64>() / results.len() as f64;
    let above80 = results.iter().filter(|r| r.scores.n6_pct >= 80.0).count();
    let above60 = results.iter().filter(|r| r.scores.n6_pct >= 60.0).count();

    println!("  Compatible combinations: {}", compat_count);
    println!("  Max n6 EXACT:  {:.1}%", max_n6);
    println!("  Avg n6 EXACT:  {:.1}%", avg_n6);
    println!("  >=80% EXACT:   {} ({:.1}%)", above80, above80 as f64 / compat_count as f64 * 100.0);
    println!("  >=60% EXACT:   {} ({:.1}%)", above60, above60 as f64 / compat_count as f64 * 100.0);
    println!();
    println!("  {} DSE 완료", "\u{2705}");
    println!("{}", bar);
}
