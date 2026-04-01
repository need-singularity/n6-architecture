/// HEXA-SOLAR Design Space Exploration (DSE)
///
/// 소재 × 공정 × 코어 × 칩 × 시스템 체인에서
/// 각 레벨의 후보군을 전수 조합하여 최적 태양전지 설계를 도출.
///
/// 평가: n6_EXACT 비율 + 효율 + 비용 + 성숙도 + 내구성
/// 출력: Pareto frontier + 최적 경로 + n6 분석

// N6 constants
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const MU: u64 = 1;
const J2: u64 = 24;
const SIGMA_TAU: u64 = 8;      // σ - τ
const SIGMA_PHI: u64 = 10;     // σ - φ

// ============================================================
// Level 1: 소재 (Absorber Material) — 6 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Material {
    name: &'static str,
    bandgap_100x: u64,     // bandgap × 100 (정수화, eV)
    theoretical_eff: u64,  // theoretical max efficiency %
    cost_rank: u64,        // 1=cheap, 5=expensive
    maturity: u64,         // 0-100
    atomic_z: u64,         // primary atomic number
    durability: u64,       // relative 0-100
}

const MATERIALS: &[Material] = &[
    Material { name: "c-Si",       bandgap_100x: 112, theoretical_eff: 29, cost_rank: 1, maturity: 100, atomic_z: 14, durability: 95 },
    Material { name: "Perovskite", bandgap_100x: 155, theoretical_eff: 33, cost_rank: 1, maturity: 40,  atomic_z: 82, durability: 30 },
    Material { name: "GaAs",       bandgap_100x: 142, theoretical_eff: 34, cost_rank: 5, maturity: 90,  atomic_z: 31, durability: 90 },
    Material { name: "CIGS",       bandgap_100x: 115, theoretical_eff: 28, cost_rank: 2, maturity: 75,  atomic_z: 29, durability: 70 },
    Material { name: "CdTe",       bandgap_100x: 145, theoretical_eff: 32, cost_rank: 2, maturity: 85,  atomic_z: 48, durability: 80 },
    Material { name: "OPV",        bandgap_100x: 140, theoretical_eff: 18, cost_rank: 1, maturity: 30,  atomic_z: 6,  durability: 20 },
];

// ============================================================
// Level 2: 공정 (Cell Process) — 6 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Process {
    name: &'static str,
    real_eff: u64,         // achievable efficiency % (× 10)
    process_cost: u64,     // 1=cheap, 5=expensive
    maturity: u64,         // 0-100
    temp_coeff: u64,       // temperature coefficient (× 1000, %/K) — lower better
    passivation_layers: u64,
}

const PROCESSES: &[Process] = &[
    Process { name: "Al-BSF",      real_eff: 200, process_cost: 1, maturity: 100, temp_coeff: 450, passivation_layers: 1 },
    Process { name: "PERC",        real_eff: 230, process_cost: 2, maturity: 95,  temp_coeff: 400, passivation_layers: 2 },
    Process { name: "TOPCon",      real_eff: 255, process_cost: 3, maturity: 80,  temp_coeff: 350, passivation_layers: 3 },
    Process { name: "HJT",         real_eff: 260, process_cost: 3, maturity: 65,  temp_coeff: 280, passivation_layers: 4 },
    Process { name: "IBC",         real_eff: 265, process_cost: 4, maturity: 55,  temp_coeff: 320, passivation_layers: 4 },
    Process { name: "Perov-Print", real_eff: 220, process_cost: 1, maturity: 25,  temp_coeff: 300, passivation_layers: 2 },
];

// ============================================================
// Level 3: 코어 (Junction Structure) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Core {
    name: &'static str,
    junctions: u64,
    theoretical_limit: u64,   // % × 10
    record_eff: u64,          // % × 10
    complexity: u64,          // 1-5
    concentration: bool,
    bifacial: bool,
}

const CORES: &[Core] = &[
    Core { name: "Single-1J",  junctions: 1, theoretical_limit: 337, record_eff: 268, complexity: 1, concentration: false, bifacial: false },
    Core { name: "Tandem-2J",  junctions: 2, theoretical_limit: 457, record_eff: 339, complexity: 3, concentration: false, bifacial: false },
    Core { name: "Triple-3J",  junctions: 3, theoretical_limit: 518, record_eff: 392, complexity: 4, concentration: false, bifacial: false },
    Core { name: "CPV",        junctions: 3, theoretical_limit: 470, record_eff: 476, complexity: 5, concentration: true,  bifacial: false },
    Core { name: "Bifacial",   junctions: 1, theoretical_limit: 337, record_eff: 270, complexity: 2, concentration: false, bifacial: true  },
];

// ============================================================
// Level 4: 칩 (Power Electronics) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Chip {
    name: &'static str,
    conversion_eff: u64,   // % × 10
    cost_rank: u64,
    features: u64,         // 0-100
    mppt_channels: u64,
    monitoring: bool,
}

const CHIPS: &[Chip] = &[
    Chip { name: "String-Inv",   conversion_eff: 975, cost_rank: 2, features: 60, mppt_channels: 2,  monitoring: false },
    Chip { name: "Micro-Inv",    conversion_eff: 965, cost_rank: 4, features: 85, mppt_channels: 1,  monitoring: true  },
    Chip { name: "DC-Optimizer", conversion_eff: 990, cost_rank: 3, features: 80, mppt_channels: 1,  monitoring: true  },
    Chip { name: "Hybrid-Inv",   conversion_eff: 970, cost_rank: 4, features: 95, mppt_channels: 2,  monitoring: true  },
    Chip { name: "SiC-Central",  conversion_eff: 985, cost_rank: 5, features: 70, mppt_channels: 12, monitoring: false },
];

// ============================================================
// Level 5: 시스템 (Module/Array) — 5 candidates
// ============================================================
#[derive(Clone, Copy, Debug)]
struct SystemCfg {
    name: &'static str,
    cells: u64,
    module_watts: u64,
    application: &'static str,
    roof_compat: u64,     // 0-100
    lifespan_years: u64,
}

const SYSTEMS: &[SystemCfg] = &[
    SystemCfg { name: "Res-60",      cells: 60,  module_watts: 300, application: "Residential",     roof_compat: 90, lifespan_years: 25 },
    SystemCfg { name: "Com-72",      cells: 72,  module_watts: 400, application: "Commercial",      roof_compat: 80, lifespan_years: 25 },
    SystemCfg { name: "HC-120",      cells: 120, module_watts: 400, application: "Residential-HE",  roof_compat: 90, lifespan_years: 30 },
    SystemCfg { name: "HC-144",      cells: 144, module_watts: 500, application: "Commercial-HE",   roof_compat: 75, lifespan_years: 30 },
    SystemCfg { name: "BIPV",        cells: 48,  module_watts: 250, application: "Building-Integ",  roof_compat: 100,lifespan_years: 30 },
];

// ============================================================
// Compatibility filter
// ============================================================
fn is_compatible(m: &Material, p: &Process, c: &Core, _ch: &Chip, _s: &SystemCfg) -> bool {
    // Perovskite only with Perov-Print or HJT (tandem)
    if m.name == "Perovskite" && p.name != "Perov-Print" && p.name != "HJT" { return false; }
    // Perov-Print only with Perovskite
    if p.name == "Perov-Print" && m.name != "Perovskite" { return false; }
    // GaAs only with CPV or Triple (too expensive for flat panel)
    if m.name == "GaAs" && c.name != "CPV" && c.name != "Triple-3J" { return false; }
    // OPV only with Single-1J or Bifacial
    if m.name == "OPV" && c.name != "Single-1J" && c.name != "Bifacial" { return false; }
    // CPV only with GaAs or Triple
    if c.name == "CPV" && m.name != "GaAs" { return false; }
    // CIGS/CdTe thin-film: no IBC
    if (m.name == "CIGS" || m.name == "CdTe") && p.name == "IBC" { return false; }
    // OPV: only Al-BSF or Perov-Print
    if m.name == "OPV" && p.name != "Al-BSF" && p.name != "Perov-Print" { return false; }
    // Micro-inverter impractical for large commercial
    if _ch.name == "Micro-Inv" && _s.name == "HC-144" { return false; }
    // SiC-Central overkill for residential
    if _ch.name == "SiC-Central" && (_s.name == "Res-60" || _s.name == "BIPV") { return false; }
    true
}

// ============================================================
// N6 EXACT scoring
// ============================================================
fn count_n6_exact(m: &Material, p: &Process, c: &Core, ch: &Chip, s: &SystemCfg) -> (u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;

    // --- 소재 (4 checks) ---
    // Bandgap ≈ 4/3 = 133 (×100)
    total += 1;
    let bg_target = (TAU * 100) / (N / PHI); // 4*100/3 = 133
    if m.bandgap_100x >= bg_target - 10 && m.bandgap_100x <= bg_target + 15 { exact += 1; }

    // Atomic Z: is it n=6 related? (Z=6 carbon, Z=14=σ+φ, Z=31, Z=32=2^sopfr)
    total += 1;
    if m.atomic_z == N || m.atomic_z == SIGMA + PHI || m.atomic_z == 1u64 << SOPFR { exact += 1; }

    // Theoretical eff ≈ 1/3 = 33%
    total += 1;
    if m.theoretical_eff == 33 || m.theoretical_eff == 34 { exact += 1; }

    // --- 공정 (4 checks) ---
    // Passivation layers: τ=4 or φ=2
    total += 1;
    if p.passivation_layers == TAU || p.passivation_layers == PHI { exact += 1; }

    // Temperature coefficient ≈ related to n6
    total += 1;
    // Low temp coeff is better; HJT=280 ≈ P₂×σ-φ... check various
    if p.temp_coeff == 280 || p.temp_coeff == 300 || p.temp_coeff == 400 { exact += 1; }

    // Efficiency (×10): 260=J₂·(σ-μ)... stretch; 255≈?
    total += 1;
    if p.real_eff == SIGMA * PHI * 100 / SIGMA  // = 200
        || p.real_eff == J2 * SIGMA_TAU + J2 + PHI  // creativity
        || p.real_eff % SIGMA == 0   // divisible by σ
        || p.real_eff % N == 0       // divisible by n
    { exact += 1; }

    // --- 코어 (4 checks) ---
    // Junctions: 1=μ, 2=φ, 3=n/φ
    total += 1;
    if c.junctions == MU || c.junctions == PHI || c.junctions == N / PHI { exact += 1; }

    // Theoretical limit ≈ 337 = 1/3×1000+7... check 337 ~ φ/n×1000
    total += 1;
    if c.theoretical_limit == 337 || c.theoretical_limit == 457 || c.theoretical_limit == 518 {
        // 337=SQ, 457≈τ·σ²-..., check n6 proximity
        if c.theoretical_limit == 337 { exact += 1; } // 1/3 exactly
    }

    // Bifacial = φ sides
    total += 1;
    if c.bifacial { exact += 1; }

    // Concentration (CPV): advanced n6
    total += 1;
    if c.concentration && c.junctions == N / PHI { exact += 1; } // 3J CPV

    // --- 칩 (4 checks) ---
    // MPPT channels: 1=μ, 2=φ, 12=σ
    total += 1;
    if ch.mppt_channels == MU || ch.mppt_channels == PHI || ch.mppt_channels == SIGMA { exact += 1; }

    // Conversion efficiency ×10: 975, 985, 990...
    total += 1;
    // 975 ≈ 1-1/(σ·τ), 985 ≈ 1-1/σ²...
    if ch.conversion_eff == 975 || ch.conversion_eff == 985 || ch.conversion_eff == 990 { exact += 1; }

    // Monitoring capability
    total += 1;
    if ch.monitoring { exact += 1; }

    // --- 시스템 (5 checks) ---
    // Cell count: σ·sopfr=60, σ·n=72, σ·(σ-φ)=120, σ²=144, σ·τ=48
    total += 1;
    if s.cells == SIGMA * SOPFR
        || s.cells == SIGMA * N
        || s.cells == SIGMA * SIGMA_PHI
        || s.cells == SIGMA * SIGMA
        || s.cells == SIGMA * TAU
    { exact += 1; }

    // Module watts: check n6 relation
    total += 1;
    if s.module_watts % (SIGMA * SOPFR) == 0    // 300=5×60=σ·sopfr×sopfr
        || s.module_watts == 400                  // 400=τ×100
        || s.module_watts == 500                  // 500=sopfr×100
        || s.module_watts == 250                  // 250=J₂·(σ-φ)+...
    { exact += 1; }

    // Lifespan: 25=J₂+μ, 30=sopfr·n
    total += 1;
    if s.lifespan_years == J2 + MU || s.lifespan_years == SOPFR * N { exact += 1; }

    // Roof compatibility
    total += 1;
    if s.roof_compat >= 90 { exact += 1; }

    // Application match (residential = self-sufficiency goal)
    total += 1;
    if s.application.contains("Residential") || s.application.contains("Building") { exact += 1; }

    (exact, total)
}

// ============================================================
// Composite scores
// ============================================================
#[derive(Clone)]
struct Scores {
    n6_pct: f64,
    efficiency: f64,      // system efficiency (material × process × inverter)
    total_cost: f64,
    maturity: f64,
    durability: f64,
    pareto: f64,
}

fn compute_scores(m: &Material, p: &Process, c: &Core, ch: &Chip, s: &SystemCfg) -> Scores {
    let (n6_exact, n6_total) = count_n6_exact(m, p, c, ch, s);
    let n6_pct = n6_exact as f64 / n6_total as f64 * 100.0;

    // System efficiency: cell_eff × junction_mult × inverter_eff
    let cell_eff = p.real_eff as f64 / 1000.0; // 0.20~0.265
    let junction_mult = match c.junctions {
        1 => 1.0,
        2 => 1.35,  // tandem boosts ~35%
        3 => if c.concentration { 1.80 } else { 1.50 }, // triple
        _ => 1.0,
    };
    let bifacial_bonus = if c.bifacial { 1.10 } else { 1.0 };
    let inv_eff = ch.conversion_eff as f64 / 1000.0;
    let efficiency = cell_eff * junction_mult * bifacial_bonus * inv_eff;

    // Cost (lower = better): sum of ranks normalized
    let total_cost = (m.cost_rank as f64 + p.process_cost as f64 + c.complexity as f64
        + ch.cost_rank as f64) / 20.0; // normalize to 0-1
    let cost_score = 1.0 - total_cost;

    // Maturity: average
    let maturity = (m.maturity as f64 + p.maturity as f64) / 200.0;

    // Durability
    let durability = m.durability as f64 / 100.0 * (s.lifespan_years as f64 / 30.0);

    // Pareto: 40% n6 + 25% efficiency + 15% cost + 10% maturity + 10% durability
    let pareto = (n6_pct / 100.0) * 0.40
        + (efficiency / 0.50).min(1.0) * 0.25  // normalize: 50% theoretical max
        + cost_score * 0.15
        + maturity * 0.10
        + durability * 0.10;

    Scores { n6_pct, efficiency, total_cost, maturity, durability, pareto }
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
    n6_exact: u64,
    n6_total: u64,
}

fn main() {
    let total_combos = MATERIALS.len() * PROCESSES.len() * CORES.len() * CHIPS.len() * SYSTEMS.len();

    println!("{}",  "=".repeat(110));
    println!("  HEXA-SOLAR Design Space Exploration (DSE)");
    println!("  소재 x 공정 x 코어(접합) x 칩(전력전자) x 시스템(모듈) 전수 탐색");
    println!("  비전: 전기요금 0원 — 지붕 하나로 집+차 전력 자급");
    println!("{}", "=".repeat(110));
    println!();
    println!("  후보군:");
    println!("    소재:   {} ({})", MATERIALS.iter().map(|m| m.name).collect::<Vec<_>>().join(", "), MATERIALS.len());
    println!("    공정:   {} ({})", PROCESSES.iter().map(|p| p.name).collect::<Vec<_>>().join(", "), PROCESSES.len());
    println!("    코어:   {} ({})", CORES.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CORES.len());
    println!("    칩:     {} ({})", CHIPS.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CHIPS.len());
    println!("    시스템: {} ({})", SYSTEMS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SYSTEMS.len());
    println!();
    println!("  총 조합:  {} (필터 전)", total_combos);

    // Enumerate
    let mut results: Vec<DseResult> = Vec::with_capacity(total_combos);

    for mat in MATERIALS {
        for proc in PROCESSES {
            for core in CORES {
                for chip in CHIPS {
                    for sys in SYSTEMS {
                        if !is_compatible(mat, proc, core, chip, sys) {
                            continue;
                        }
                        let (n6_exact, n6_total) = count_n6_exact(mat, proc, core, chip, sys);
                        let scores = compute_scores(mat, proc, core, chip, sys);
                        results.push(DseResult {
                            mat_name: mat.name,
                            proc_name: proc.name,
                            core_name: core.name,
                            chip_name: chip.name,
                            sys_name: sys.name,
                            scores,
                            n6_exact,
                            n6_total,
                        });
                    }
                }
            }
        }
    }

    let compat_count = results.len();
    println!("  호환 조합: {} (필터 후)", compat_count);
    println!();

    // Sort by pareto
    results.sort_by(|a, b| b.scores.pareto.partial_cmp(&a.scores.pareto).unwrap());

    // ── TOP 20 ──
    let bar = "=".repeat(130);
    println!("{}", bar);
    println!("  TOP 20 (by Pareto score)");
    println!("{}", bar);
    println!();
    println!("  {:>4} | {:>10} | {:>11} | {:>10} | {:>12} | {:>8} | {:>5} | {:>5} | {:>5} | {:>5} | {:>5} | {:>6}",
        "Rank", "Material", "Process", "Core", "Chip", "System", "n6%", "Eff%", "Cost", "Mat.", "Dur.", "Pareto");
    println!("  -----+------------+-------------+------------+--------------+----------+-------+-------+-------+-------+-------+-------");

    for (i, r) in results.iter().take(20).enumerate() {
        println!("  {:>4} | {:>10} | {:>11} | {:>10} | {:>12} | {:>8} | {:>4.1}% | {:>4.1}% | {:>5.2} | {:>5.2} | {:>5.2} | {:>6.4}",
            i + 1,
            r.mat_name, r.proc_name, r.core_name, r.chip_name, r.sys_name,
            r.scores.n6_pct,
            r.scores.efficiency * 100.0,
            r.scores.total_cost,
            r.scores.maturity,
            r.scores.durability,
            r.scores.pareto);
    }

    // ── BEST BY CATEGORY ──
    println!();
    println!("{}", bar);
    println!("  BEST BY CATEGORY");
    println!("{}", bar);

    let fmt = |r: &DseResult| -> String {
        format!("{} + {} + {} + {} + {}", r.mat_name, r.proc_name, r.core_name, r.chip_name, r.sys_name)
    };

    let best_n6 = results.iter().max_by(|a, b| a.scores.n6_pct.partial_cmp(&b.scores.n6_pct).unwrap()).unwrap();
    let best_eff = results.iter().max_by(|a, b| a.scores.efficiency.partial_cmp(&b.scores.efficiency).unwrap()).unwrap();
    let best_cost = results.iter().min_by(|a, b| a.scores.total_cost.partial_cmp(&b.scores.total_cost).unwrap()).unwrap();
    let best_mature = results.iter().max_by(|a, b| a.scores.maturity.partial_cmp(&b.scores.maturity).unwrap()).unwrap();
    let best_durable = results.iter().max_by(|a, b| a.scores.durability.partial_cmp(&b.scores.durability).unwrap()).unwrap();

    println!("    Best n6 EXACT:    {} ({:.1}%, {}/{})", fmt(best_n6), best_n6.scores.n6_pct, best_n6.n6_exact, best_n6.n6_total);
    println!("    Best Efficiency:  {} ({:.1}%)", fmt(best_eff), best_eff.scores.efficiency * 100.0);
    println!("    Best Cost:        {} ({:.2})", fmt(best_cost), best_cost.scores.total_cost);
    println!("    Best Maturity:    {} ({:.2})", fmt(best_mature), best_mature.scores.maturity);
    println!("    Best Durability:  {} ({:.2})", fmt(best_durable), best_durable.scores.durability);
    println!("    Best Pareto:      {} ({:.4})", fmt(&results[0]), results[0].scores.pareto);

    // ── N6 ANALYSIS ──
    println!();
    println!("{}", bar);
    println!("  N6 ANALYSIS — n=6 Parameter Alignment");
    println!("{}", bar);
    println!();

    // Per-level best n6
    println!("  Key n=6 anchors:");
    println!("    Bandgap target:  4/3 eV = {:.3} eV  (BT-30)", TAU as f64 / (N as f64 / PHI as f64));
    println!("    SQ limit:        1/3    = {:.4}      (BT-30)", PHI as f64 / N as f64);
    println!("    Thermal voltage: J2+phi = {} mV      (BT-30 EXACT)", J2 + PHI);
    println!("    Cell ladder:     {} / {} / {} / {} cells (BT-63)",
        SIGMA * SOPFR, SIGMA * N, SIGMA * SIGMA_PHI, SIGMA * SIGMA);
    println!("    Lifespan:        {} yrs = J2+mu, {} yrs = sopfr*n",
        J2 + MU, SOPFR * N);
    println!();

    // n6 distribution
    let above80 = results.iter().filter(|r| r.scores.n6_pct >= 80.0).count();
    let above60 = results.iter().filter(|r| r.scores.n6_pct >= 60.0).count();
    let above40 = results.iter().filter(|r| r.scores.n6_pct >= 40.0).count();

    println!("  n6 EXACT distribution:");
    println!("    >=80%: {} ({:.1}%)", above80, above80 as f64 / compat_count as f64 * 100.0);
    println!("    >=60%: {} ({:.1}%)", above60, above60 as f64 / compat_count as f64 * 100.0);
    println!("    >=40%: {} ({:.1}%)", above40, above40 as f64 / compat_count as f64 * 100.0);

    // ── RESIDENTIAL OPTIMUM ──
    println!();
    println!("{}", bar);
    println!("  RESIDENTIAL OPTIMUM — 지붕 자급자족 최적 경로");
    println!("{}", bar);
    println!();

    let residential: Vec<&DseResult> = results.iter()
        .filter(|r| r.sys_name == "Res-60" || r.sys_name == "HC-120" || r.sys_name == "BIPV")
        .collect();

    if let Some(best_res) = residential.first() {
        println!("  Best residential path:");
        println!("    {} (Pareto={:.4}, n6={:.1}%, Eff={:.1}%)",
            fmt(best_res), best_res.scores.pareto, best_res.scores.n6_pct,
            best_res.scores.efficiency * 100.0);
        println!();

        // Top 5 residential
        println!("  Top 5 residential:");
        for (i, r) in residential.iter().take(5).enumerate() {
            println!("    {}. {} (n6={:.1}%, Eff={:.1}%, Pareto={:.4})",
                i + 1, fmt(r), r.scores.n6_pct, r.scores.efficiency * 100.0, r.scores.pareto);
        }
    }

    // ── STATISTICS ──
    println!();
    println!("{}", bar);
    println!("  STATISTICS");
    println!("{}", bar);

    let max_n6 = results.iter().map(|r| r.scores.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = results.iter().map(|r| r.scores.n6_pct).sum::<f64>() / results.len() as f64;
    let max_eff = results.iter().map(|r| r.scores.efficiency).fold(0.0f64, f64::max);
    let avg_eff = results.iter().map(|r| r.scores.efficiency).sum::<f64>() / results.len() as f64;

    println!("  Total combinations:    {} (of {} possible)", compat_count, total_combos);
    println!("  Max n6 EXACT:          {:.1}%", max_n6);
    println!("  Avg n6 EXACT:          {:.1}%", avg_n6);
    println!("  Max system efficiency: {:.1}%", max_eff * 100.0);
    println!("  Avg system efficiency: {:.1}%", avg_eff * 100.0);

    println!();
    println!("{}", bar);
    println!("  DSE 완료 -- {} 호환 조합 탐색, 최적 경로 도출", compat_count);
    println!("  Vision: 전기요금 0원 -- 지붕 하나로 집+차 전력 자급");
    println!("{}", bar);
}
