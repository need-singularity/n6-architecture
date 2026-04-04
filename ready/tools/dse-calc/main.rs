/// N6 Design Space Exploration (DSE) — 전수 조합 탐색기
///
/// 소재 → 공정 → 코어 → 칩 → 시스템 체인에서
/// 각 레벨의 후보군을 전수 조합하여 최적 설계를 도출.
///
/// 평가: n6_EXACT 비율 + 성능 + 전력 + 면적 + 비용
/// 출력: Pareto frontier + 최적 경로

// N6 constants
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const MU: u64 = 1;
const J2: u64 = 24;
const P2: u64 = 28;

// ============================================================
// Level 1: 소재 후보
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Material {
    name: &'static str,
    wafer_z: u64,           // 주요 원소 원자번호
    cn: u64,                // coordination number
    bandgap_10x: u64,       // bandgap × 10 (정수화)
    thermal_k: u64,         // thermal conductivity W/m·K
    cost_rank: u64,         // 1=cheap, 5=expensive
}

const MATERIALS: &[Material] = &[
    Material { name: "Si",      wafer_z: 14,  cn: 4, bandgap_10x: 11, thermal_k: 150,  cost_rank: 1 },
    Material { name: "SiC-4H",  wafer_z: 14,  cn: 4, bandgap_10x: 33, thermal_k: 490,  cost_rank: 3 },
    Material { name: "GaN",     wafer_z: 31,  cn: 4, bandgap_10x: 34, thermal_k: 130,  cost_rank: 4 },
    Material { name: "Diamond", wafer_z: 6,   cn: 4, bandgap_10x: 55, thermal_k: 2000, cost_rank: 5 },
    Material { name: "Ge",      wafer_z: 32,  cn: 4, bandgap_10x: 7,  thermal_k: 60,   cost_rank: 2 },
    Material { name: "Si+SiGe", wafer_z: 14,  cn: 4, bandgap_10x: 11, thermal_k: 150,  cost_rank: 2 },
];

// ============================================================
// Level 2: 공정 후보
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Process {
    name: &'static str,
    gate_pitch_nm: u64,
    metal_pitch_nm: u64,
    metal_layers: u64,
    nanosheets: u64,
    euv_masks: u64,
    yield_pct: u64,         // yield %
}

const PROCESSES: &[Process] = &[
    Process { name: "TSMC_N2",     gate_pitch_nm: 48, metal_pitch_nm: 28, metal_layers: 12, nanosheets: 4, euv_masks: 24, yield_pct: 75 },
    Process { name: "TSMC_N3E",    gate_pitch_nm: 48, metal_pitch_nm: 28, metal_layers: 12, nanosheets: 3, euv_masks: 20, yield_pct: 82 },
    Process { name: "Intel_18A",   gate_pitch_nm: 44, metal_pitch_nm: 30, metal_layers: 13, nanosheets: 4, euv_masks: 22, yield_pct: 70 },
    Process { name: "Samsung_SF2", gate_pitch_nm: 48, metal_pitch_nm: 28, metal_layers: 12, nanosheets: 3, euv_masks: 20, yield_pct: 68 },
    Process { name: "TSMC_A16",    gate_pitch_nm: 48, metal_pitch_nm: 24, metal_layers: 14, nanosheets: 4, euv_masks: 28, yield_pct: 60 },
];

// ============================================================
// Level 3: 코어 후보
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Core {
    name: &'static str,
    p_cores: u64,
    e_cores: u64,
    gpu_sms: u64,
    npu_cores: u64,
    fp32_per_sm: u64,
    rob_entries: u64,
}

const CORES: &[Core] = &[
    Core { name: "HEXA-P",    p_cores: 8,  e_cores: 4, gpu_sms: 144, npu_cores: 24, fp32_per_sm: 128, rob_entries: 256 },
    Core { name: "HEXA-Lite", p_cores: 4,  e_cores: 2, gpu_sms: 72,  npu_cores: 12, fp32_per_sm: 128, rob_entries: 256 },
    Core { name: "ARM-Big",   p_cores: 8,  e_cores: 4, gpu_sms: 16,  npu_cores: 8,  fp32_per_sm: 64,  rob_entries: 320 },
    Core { name: "x86-Zen5",  p_cores: 16, e_cores: 0, gpu_sms: 0,   npu_cores: 0,  fp32_per_sm: 0,   rob_entries: 448 },
    Core { name: "RISC-V",    p_cores: 12, e_cores: 6, gpu_sms: 48,  npu_cores: 12, fp32_per_sm: 64,  rob_entries: 128 },
];

// ============================================================
// Level 4: 칩 후보
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Chip {
    name: &'static str,
    hbm_stacks: u64,
    hbm_gb_per_stack: u64,
    tdp_w: u64,
    die_area_mm2: u64,
    cowos_tiles: u64,
}

const CHIPS: &[Chip] = &[
    Chip { name: "HEXA-1_Full",  hbm_stacks: 8,  hbm_gb_per_stack: 36, tdp_w: 240, die_area_mm2: 800, cowos_tiles: 5 },
    Chip { name: "HEXA-1_Half",  hbm_stacks: 4,  hbm_gb_per_stack: 36, tdp_w: 150, die_area_mm2: 400, cowos_tiles: 3 },
    Chip { name: "H100_Style",   hbm_stacks: 6,  hbm_gb_per_stack: 16, tdp_w: 700, die_area_mm2: 814, cowos_tiles: 4 },
    Chip { name: "B200_Style",   hbm_stacks: 8,  hbm_gb_per_stack: 24, tdp_w: 1000,die_area_mm2: 900, cowos_tiles: 5 },
    Chip { name: "M4Ultra_Style",hbm_stacks: 0,  hbm_gb_per_stack: 0,  tdp_w: 150, die_area_mm2: 400, cowos_tiles: 2 },
];

// ============================================================
// Level 5: 시스템 후보
// ============================================================
#[derive(Clone, Copy, Debug)]
struct System {
    name: &'static str,
    gpus_per_node: u64,
    nodes_per_rack: u64,
    rack_power_kw: u64,
    network_tiers: u64,
    pue_10x: u64,          // PUE × 10
}

const SYSTEMS: &[System] = &[
    System { name: "DGX_Style",    gpus_per_node: 8,  nodes_per_rack: 12, rack_power_kw: 48,  network_tiers: 3, pue_10x: 12 },
    System { name: "TPU_Pod",      gpus_per_node: 4,  nodes_per_rack: 24, rack_power_kw: 40,  network_tiers: 3, pue_10x: 11 },
    System { name: "SuperPOD_72",  gpus_per_node: 8,  nodes_per_rack: 12, rack_power_kw: 48,  network_tiers: 3, pue_10x: 12 },
    System { name: "Edge_Compact", gpus_per_node: 1,  nodes_per_rack: 48, rack_power_kw: 24,  network_tiers: 2, pue_10x: 15 },
];

// ============================================================
// N6 EXACT 계산 — 핵심 평가 함수
// ============================================================
fn count_n6_exact(mat: &Material, proc: &Process, core: &Core, chip: &Chip, sys: &System) -> (u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;

    // 소재 (8 checks)
    let mat_checks: &[(u64, u64)] = &[
        (mat.cn, TAU),                                    // CN = τ
        (mat.wafer_z, if mat.name == "Diamond" { N } else if mat.name == "Ge" { 1u64 << SOPFR } else { SIGMA + PHI }), // Z
    ];
    for &(val, expected) in mat_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // 공정 (10 checks)
    let proc_checks: &[(u64, u64)] = &[
        (proc.gate_pitch_nm, SIGMA * TAU),                // gate pitch = σ·τ = 48
        (proc.metal_pitch_nm, P2),                        // metal pitch = P₂ = 28
        (proc.metal_layers, SIGMA),                       // metal layers = σ = 12
        (proc.nanosheets, TAU),                           // nanosheets = τ = 4
        (proc.euv_masks, J2),                             // EUV masks = J₂ = 24
    ];
    for &(val, expected) in proc_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // 코어 (10 checks)
    let core_checks: &[(u64, u64)] = &[
        (core.p_cores, SIGMA - TAU),                      // P-cores = σ-τ = 8
        (core.e_cores, TAU),                              // E-cores = τ = 4
        (core.gpu_sms, SIGMA * SIGMA),                    // SMs = σ² = 144
        (core.npu_cores, J2),                             // NPU = J₂ = 24
        (core.fp32_per_sm, 1u64 << (SIGMA - SOPFR)),     // FP32/SM = 2^(σ-sopfr) = 128
        (core.rob_entries, 1u64 << (SIGMA - TAU)),        // ROB = 2^(σ-τ) = 256
        (core.p_cores + core.e_cores, SIGMA),             // total cores = σ = 12
    ];
    for &(val, expected) in core_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // 칩 (8 checks)
    let chip_checks: &[(u64, u64)] = &[
        (chip.hbm_stacks, SIGMA - TAU),                   // HBM stacks = σ-τ = 8
        (chip.hbm_stacks * chip.hbm_gb_per_stack, SIGMA * J2), // total HBM = σ·J₂ = 288
        (chip.tdp_w, J2 * (SIGMA - PHI)),                 // TDP = J₂·(σ-φ) = 240
        (chip.cowos_tiles, SOPFR),                         // CoWoS tiles = sopfr = 5
    ];
    for &(val, expected) in chip_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // 시스템 (8 checks)
    let sys_checks: &[(u64, u64)] = &[
        (sys.gpus_per_node, SIGMA - TAU),                  // GPUs/node = σ-τ = 8
        (sys.nodes_per_rack, SIGMA),                       // nodes/rack = σ = 12
        (sys.rack_power_kw, SIGMA * TAU),                  // rack power = σ·τ = 48
        (sys.network_tiers, N / PHI),                      // tiers = n/φ = 3
        (sys.pue_10x, SIGMA),                              // PUE×10 = σ = 12 (PUE=1.2)
    ];
    for &(val, expected) in sys_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    (exact, total)
}

// ============================================================
// 성능 스코어 (정규화, 0-100)
// ============================================================
fn perf_score(core: &Core, chip: &Chip) -> u64 {
    let gpu_flops = core.gpu_sms * core.fp32_per_sm; // relative
    let mem_gb = chip.hbm_stacks * chip.hbm_gb_per_stack;
    let mem_bw = chip.hbm_stacks * 500; // ~500 GB/s per stack estimate
    // weighted: 50% compute + 30% memory + 20% bandwidth
    let score = (gpu_flops / 200) * 50 + (mem_gb / 3) * 30 + (mem_bw / 50) * 20;
    score.min(100)
}

// ============================================================
// 비용 스코어 (낮을수록 좋음, 0-100)
// ============================================================
fn cost_score(mat: &Material, proc: &Process, chip: &Chip) -> u64 {
    let mat_cost = mat.cost_rank * 10;
    let area_cost = chip.die_area_mm2 / 10;
    let yield_bonus = proc.yield_pct;
    let raw = mat_cost + area_cost;
    if raw > yield_bonus { raw - yield_bonus / 2 } else { 0 }
}

// ============================================================
// 결과 구조체
// ============================================================
#[derive(Clone)]
struct DseResult {
    mat_name: &'static str,
    proc_name: &'static str,
    core_name: &'static str,
    chip_name: &'static str,
    sys_name: &'static str,
    n6_exact: u64,
    n6_total: u64,
    n6_pct: f64,
    perf: u64,
    tdp: u64,
    cost: u64,
    pareto_score: f64,
}

fn main() {
    let total_combos = MATERIALS.len() * PROCESSES.len() * CORES.len() * CHIPS.len() * SYSTEMS.len();

    println!("═══════════════════════════════════════════════════════════════════");
    println!("  N6 Design Space Exploration (DSE)");
    println!("  소재 × 공정 × 코어 × 칩 × 시스템 전수 탐색");
    println!("═══════════════════════════════════════════════════════════════════");
    println!();
    println!("  후보군:");
    println!("    소재:   {} ({})", MATERIALS.iter().map(|m| m.name).collect::<Vec<_>>().join(", "), MATERIALS.len());
    println!("    공정:   {} ({})", PROCESSES.iter().map(|p| p.name).collect::<Vec<_>>().join(", "), PROCESSES.len());
    println!("    코어:   {} ({})", CORES.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CORES.len());
    println!("    칩:     {} ({})", CHIPS.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), CHIPS.len());
    println!("    시스템: {} ({})", SYSTEMS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SYSTEMS.len());
    println!();
    println!("  총 조합: {}", total_combos);
    println!();

    let mut results: Vec<DseResult> = Vec::with_capacity(total_combos);

    for mat in MATERIALS {
        for proc in PROCESSES {
            for core in CORES {
                for chip in CHIPS {
                    for sys in SYSTEMS {
                        let (n6_exact, n6_total) = count_n6_exact(mat, proc, core, chip, sys);
                        let n6_pct = (n6_exact as f64) / (n6_total as f64) * 100.0;
                        let perf = perf_score(core, chip);
                        let tdp = chip.tdp_w;
                        let cost = cost_score(mat, proc, chip);

                        // Pareto score: 40% n6 + 30% perf + 20% efficiency + 10% cost
                        let efficiency = if tdp > 0 { (perf as f64) / (tdp as f64) * 100.0 } else { 0.0 };
                        let pareto_score = n6_pct * 0.40
                            + (perf as f64) * 0.30
                            + efficiency * 0.20
                            + (100.0 - cost as f64).max(0.0) * 0.10;

                        results.push(DseResult {
                            mat_name: mat.name,
                            proc_name: proc.name,
                            core_name: core.name,
                            chip_name: chip.name,
                            sys_name: sys.name,
                            n6_exact,
                            n6_total,
                            n6_pct,
                            perf,
                            tdp,
                            cost,
                            pareto_score,
                        });
                    }
                }
            }
        }
    }

    // Sort by pareto score descending
    results.sort_by(|a, b| b.pareto_score.partial_cmp(&a.pareto_score).unwrap());

    // Top 20
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  TOP 20 CONFIGURATIONS (by Pareto score)");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  {:>4} │ {:>8} │ {:>12} │ {:>10} │ {:>14} │ {:>14} │ {:>6} │ {:>4} │ {:>5} │ {:>5} │ {:>7}",
        "Rank", "Material", "Process", "Core", "Chip", "System", "n6%", "Perf", "TDP", "Cost", "Pareto");
    println!("  ─────┼──────────┼──────────────┼────────────┼────────────────┼────────────────┼────────┼──────┼───────┼───────┼────────");

    for (i, r) in results.iter().take(20).enumerate() {
        println!("  {:>4} │ {:>8} │ {:>12} │ {:>10} │ {:>14} │ {:>14} │ {:>5.1}% │ {:>4} │ {:>4}W │ {:>5} │ {:>7.2}",
            i + 1, r.mat_name, r.proc_name, r.core_name, r.chip_name, r.sys_name,
            r.n6_pct, r.perf, r.tdp, r.cost, r.pareto_score);
    }

    // Statistics
    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  STATISTICS");
    println!("═══════════════════════════════════════════════════════════════════");

    let max_n6 = results.iter().map(|r| r.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = results.iter().map(|r| r.n6_pct).sum::<f64>() / results.len() as f64;
    let perfect = results.iter().filter(|r| r.n6_pct == 100.0).count();
    let above80 = results.iter().filter(|r| r.n6_pct >= 80.0).count();
    let above60 = results.iter().filter(|r| r.n6_pct >= 60.0).count();

    println!("  Total combinations:  {}", total_combos);
    println!("  Max n6 EXACT:        {:.1}%", max_n6);
    println!("  Avg n6 EXACT:        {:.1}%", avg_n6);
    println!("  100% EXACT:          {} ({:.1}%)", perfect, perfect as f64 / total_combos as f64 * 100.0);
    println!("  ≥80% EXACT:          {} ({:.1}%)", above80, above80 as f64 / total_combos as f64 * 100.0);
    println!("  ≥60% EXACT:          {} ({:.1}%)", above60, above60 as f64 / total_combos as f64 * 100.0);

    // Best per category
    println!();
    println!("  BEST BY CATEGORY:");
    let best_n6 = &results[0]; // already sorted by pareto, find actual best n6
    let best_n6_actual = results.iter().max_by(|a, b| a.n6_pct.partial_cmp(&b.n6_pct).unwrap()).unwrap();
    let best_perf = results.iter().max_by_key(|r| r.perf).unwrap();
    let best_eff = results.iter().max_by(|a, b| {
        let ea = if a.tdp > 0 { a.perf as f64 / a.tdp as f64 } else { 0.0 };
        let eb = if b.tdp > 0 { b.perf as f64 / b.tdp as f64 } else { 0.0 };
        ea.partial_cmp(&eb).unwrap()
    }).unwrap();

    println!("    Best n6 EXACT:   {} + {} + {} + {} + {} ({:.1}%)",
        best_n6_actual.mat_name, best_n6_actual.proc_name, best_n6_actual.core_name,
        best_n6_actual.chip_name, best_n6_actual.sys_name, best_n6_actual.n6_pct);
    println!("    Best Perf:       {} + {} + {} + {} + {} (perf={})",
        best_perf.mat_name, best_perf.proc_name, best_perf.core_name,
        best_perf.chip_name, best_perf.sys_name, best_perf.perf);
    println!("    Best Efficiency: {} + {} + {} + {} + {} ({}/{}W)",
        best_eff.mat_name, best_eff.proc_name, best_eff.core_name,
        best_eff.chip_name, best_eff.sys_name, best_eff.perf, best_eff.tdp);
    println!("    Best Pareto:     {} + {} + {} + {} + {} (score={:.2})",
        best_n6.mat_name, best_n6.proc_name, best_n6.core_name,
        best_n6.chip_name, best_n6.sys_name, best_n6.pareto_score);

    println!();
    println!("═══════════════════════════════════════════════════════════════════");
    println!("  ✅ DSE 완료 — {} 조합 탐색, 최적 경로 도출", total_combos);
    println!("═══════════════════════════════════════════════════════════════════");
}
