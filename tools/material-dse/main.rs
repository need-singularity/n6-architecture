/// N6 Material Synthesis DSE — 궁극의 물질합성 전수 조합 탐색기
///
/// 소재 × 공정 × 조립기 × 제어 × 시스템 체인에서
/// 각 레벨의 후보군을 전수 조합하여 최적 합성 설계를 도출.
///
/// 평가: n6_EXACT 비율 + 성능 + 전력 + 비용
/// 출력: Pareto frontier + 최적 경로

// N6 constants
const N: f64 = 6.0;
const PHI: f64 = 2.0;
const TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const SOPFR: f64 = 5.0;
const _MU: f64 = 1.0;
const _J2: f64 = 24.0;

// ============================================================
// Level 1: 소재 (Element/Molecule) — 5 candidates
// ============================================================
#[derive(Clone, Copy)]
struct Element {
    id: &'static str,
    name: &'static str,
    n6_score: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

const ELEMENTS: &[Element] = &[
    Element { id: "E1", name: "Carbon_Z6",       n6_score: 1.0, perf: 95.0, power: 70.0, cost: 60.0 },
    Element { id: "E2", name: "Silicon_Z14",      n6_score: 0.7, perf: 85.0, power: 75.0, cost: 90.0 },
    Element { id: "E3", name: "TransitionMetal",   n6_score: 0.8, perf: 80.0, power: 65.0, cost: 50.0 },
    Element { id: "E4", name: "NobleMetal",        n6_score: 0.6, perf: 90.0, power: 60.0, cost: 20.0 },
    Element { id: "E5", name: "Ceramic",           n6_score: 0.5, perf: 75.0, power: 80.0, cost: 70.0 },
];

// ============================================================
// Level 2: 공정 (Process) — 6 candidates
// ============================================================
#[derive(Clone, Copy)]
struct Process {
    id: &'static str,
    name: &'static str,
    n6_score: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

const PROCESSES: &[Process] = &[
    Process { id: "P1", name: "ALD",              n6_score: 0.9, perf: 90.0, power: 70.0, cost: 75.0 },
    Process { id: "P2", name: "MBE",              n6_score: 0.7, perf: 95.0, power: 50.0, cost: 40.0 },
    Process { id: "P3", name: "CVD",              n6_score: 0.6, perf: 80.0, power: 75.0, cost: 85.0 },
    Process { id: "P4", name: "Mechanosynthesis",  n6_score: 0.8, perf: 98.0, power: 60.0, cost: 30.0 },
    Process { id: "P5", name: "Electrochemistry",  n6_score: 0.7, perf: 75.0, power: 80.0, cost: 80.0 },
    Process { id: "P6", name: "SelfAssembly",      n6_score: 1.0, perf: 85.0, power: 90.0, cost: 70.0 },
];

// ============================================================
// Level 3: 조립기 (Assembler) — 6 candidates
// ============================================================
#[derive(Clone, Copy)]
struct Assembler {
    id: &'static str,
    name: &'static str,
    n6_score: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

const ASSEMBLERS: &[Assembler] = &[
    Assembler { id: "A1", name: "STM_tip",           n6_score: 0.8, perf: 95.0, power: 40.0, cost: 30.0 },
    Assembler { id: "A2", name: "MolecularAsm",      n6_score: 0.9, perf: 98.0, power: 70.0, cost: 25.0 },
    Assembler { id: "A3", name: "DNA_origami",        n6_score: 1.0, perf: 80.0, power: 90.0, cost: 60.0 },
    Assembler { id: "A4", name: "OpticalTweezer",     n6_score: 0.6, perf: 85.0, power: 65.0, cost: 50.0 },
    Assembler { id: "A5", name: "FocusedIonBeam",     n6_score: 0.5, perf: 90.0, power: 50.0, cost: 45.0 },
    Assembler { id: "A6", name: "NanobotSwarm",       n6_score: 0.7, perf: 92.0, power: 75.0, cost: 35.0 },
];

// ============================================================
// Level 4: 제어 (Control) — 4 candidates
// ============================================================
#[derive(Clone, Copy)]
struct Control {
    id: &'static str,
    name: &'static str,
    n6_score: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

const CONTROLS: &[Control] = &[
    Control { id: "C1", name: "ClassicalPID",         n6_score: 0.5, perf: 70.0,  power: 85.0, cost: 90.0 },
    Control { id: "C2", name: "QuantumSensing",        n6_score: 0.9, perf: 95.0,  power: 60.0, cost: 40.0 },
    Control { id: "C3", name: "AI_ML_Control",         n6_score: 0.7, perf: 90.0,  power: 75.0, cost: 70.0 },
    Control { id: "C4", name: "HybridQC",              n6_score: 0.8, perf: 98.0,  power: 70.0, cost: 50.0 },
];

// ============================================================
// Level 5: 시스템 (Factory) — 5 candidates
// ============================================================
#[derive(Clone, Copy)]
struct Factory {
    id: &'static str,
    name: &'static str,
    n6_score: f64,
    perf: f64,
    power: f64,
    cost: f64,
}

const FACTORIES: &[Factory] = &[
    Factory { id: "F1", name: "SingleStation",     n6_score: 0.4, perf: 50.0, power: 90.0, cost: 95.0 },
    Factory { id: "F2", name: "ParallelArray",     n6_score: 0.6, perf: 80.0, power: 75.0, cost: 70.0 },
    Factory { id: "F3", name: "ConvergentAsm",     n6_score: 0.8, perf: 90.0, power: 70.0, cost: 55.0 },
    Factory { id: "F4", name: "DistribSwarm",      n6_score: 0.7, perf: 85.0, power: 80.0, cost: 50.0 },
    Factory { id: "F5", name: "SelfReplicating",   n6_score: 1.0, perf: 99.0, power: 65.0, cost: 30.0 },
];

// ============================================================
// DSE Result
// ============================================================
#[derive(Clone)]
struct DseResult {
    elem_id: &'static str,
    elem_name: &'static str,
    proc_id: &'static str,
    proc_name: &'static str,
    asm_id: &'static str,
    asm_name: &'static str,
    ctrl_id: &'static str,
    ctrl_name: &'static str,
    fact_id: &'static str,
    fact_name: &'static str,
    n6_exact_pct: f64,
    perf: f64,
    power: f64,
    cost: f64,
    total_score: f64,
}

// ============================================================
// Pareto dominance check
// ============================================================
fn dominates(a: &DseResult, b: &DseResult) -> bool {
    // a dominates b if a is >= b in ALL objectives and > in at least one
    let a_vals = [a.n6_exact_pct, a.perf, a.power, a.cost];
    let b_vals = [b.n6_exact_pct, b.perf, b.power, b.cost];
    let mut all_geq = true;
    let mut any_gt = false;
    for i in 0..4 {
        if a_vals[i] < b_vals[i] { all_geq = false; break; }
        if a_vals[i] > b_vals[i] { any_gt = true; }
    }
    all_geq && any_gt
}

fn main() {
    let total_combos = ELEMENTS.len() * PROCESSES.len() * ASSEMBLERS.len()
        * CONTROLS.len() * FACTORIES.len();

    println!("================================================================");
    println!("  N6 Material Synthesis DSE");
    println!("  궁극의 물질합성 — 소재 x 공정 x 조립기 x 제어 x 시스템");
    println!("================================================================");
    println!();
    println!("  N6 Core: sigma({})·phi({}) = n({})·tau({})", SIGMA, PHI, N, TAU);
    println!("           {}·{} = {}·{} = {}", SIGMA, PHI, N, TAU, SIGMA * PHI);
    println!();
    println!("  후보군:");
    println!("    소재(Element):   {} candidates ({})",
        ELEMENTS.iter().map(|e| e.id).collect::<Vec<_>>().join(","),
        ELEMENTS.len());
    println!("    공정(Process):   {} candidates ({})",
        PROCESSES.iter().map(|p| p.id).collect::<Vec<_>>().join(","),
        PROCESSES.len());
    println!("    조립기(Assembler): {} candidates ({})",
        ASSEMBLERS.iter().map(|a| a.id).collect::<Vec<_>>().join(","),
        ASSEMBLERS.len());
    println!("    제어(Control):   {} candidates ({})",
        CONTROLS.iter().map(|c| c.id).collect::<Vec<_>>().join(","),
        CONTROLS.len());
    println!("    시스템(Factory): {} candidates ({})",
        FACTORIES.iter().map(|f| f.id).collect::<Vec<_>>().join(","),
        FACTORIES.len());
    println!();
    println!("  총 조합 수: {}x{}x{}x{}x{} = {}",
        ELEMENTS.len(), PROCESSES.len(), ASSEMBLERS.len(),
        CONTROLS.len(), FACTORIES.len(), total_combos);
    println!();

    // ============================================================
    // Enumerate all 3600 combinations
    // ============================================================
    let mut results: Vec<DseResult> = Vec::with_capacity(total_combos);

    for elem in ELEMENTS {
        for proc in PROCESSES {
            for asm in ASSEMBLERS {
                for ctrl in CONTROLS {
                    for fact in FACTORIES {
                        // n6_exact = product of all n6_scores, normalized to 0-100%
                        let n6_product = elem.n6_score
                            * proc.n6_score
                            * asm.n6_score
                            * ctrl.n6_score
                            * fact.n6_score;
                        let n6_exact_pct = n6_product * 100.0;

                        // Weighted averages (equal weight across 5 levels)
                        let perf = (elem.perf + proc.perf + asm.perf + ctrl.perf + fact.perf) / 5.0;
                        let power = (elem.power + proc.power + asm.power + ctrl.power + fact.power) / 5.0;
                        let cost = (elem.cost + proc.cost + asm.cost + ctrl.cost + fact.cost) / 5.0;

                        // total_score = 0.35*n6 + 0.25*perf + 0.20*power + 0.20*cost
                        let total_score = 0.35 * n6_exact_pct
                            + 0.25 * perf
                            + 0.20 * power
                            + 0.20 * cost;

                        results.push(DseResult {
                            elem_id: elem.id,
                            elem_name: elem.name,
                            proc_id: proc.id,
                            proc_name: proc.name,
                            asm_id: asm.id,
                            asm_name: asm.name,
                            ctrl_id: ctrl.id,
                            ctrl_name: ctrl.name,
                            fact_id: fact.id,
                            fact_name: fact.name,
                            n6_exact_pct,
                            perf,
                            power,
                            cost,
                            total_score,
                        });
                    }
                }
            }
        }
    }

    // Sort by total_score descending
    results.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

    // ============================================================
    // Find Pareto frontier
    // ============================================================
    let mut pareto: Vec<usize> = Vec::new();
    for i in 0..results.len() {
        let mut dominated = false;
        for j in 0..results.len() {
            if i != j && dominates(&results[j], &results[i]) {
                dominated = true;
                break;
            }
        }
        if !dominated {
            pareto.push(i);
        }
    }

    // ============================================================
    // TOP 20 TABLE
    // ============================================================
    println!("================================================================================================================================");
    println!("  TOP 20 CONFIGURATIONS (by total score)");
    println!("================================================================================================================================");
    println!();
    println!("  {:>4} | {:>14} | {:>14} | {:>14} | {:>14} | {:>14} | {:>7} | {:>5} | {:>5} | {:>5} | {:>6} | {:>3}",
        "Rank", "소재", "공정", "조립기", "제어", "시스템",
        "n6_EX%", "성능", "전력", "비용", "종합", "PF");
    println!("  -----+----------------+----------------+----------------+----------------+----------------+---------+-------+-------+-------+--------+----");

    for (i, r) in results.iter().take(20).enumerate() {
        let is_pareto = if pareto.contains(&i) { "*" } else { "" };
        println!("  {:>4} | {:>14} | {:>14} | {:>14} | {:>14} | {:>14} | {:>6.1}% | {:>5.1} | {:>5.1} | {:>5.1} | {:>6.2} | {:>3}",
            i + 1,
            r.elem_name, r.proc_name, r.asm_name, r.ctrl_name, r.fact_name,
            r.n6_exact_pct, r.perf, r.power, r.cost, r.total_score, is_pareto);
    }

    // ============================================================
    // PARETO FRONTIER
    // ============================================================
    println!();
    println!("================================================================");
    println!("  PARETO FRONTIER ({} non-dominated solutions)", pareto.len());
    println!("================================================================");
    println!();
    println!("  {:>4} | {:>14} | {:>14} | {:>14} | {:>14} | {:>14} | {:>7} | {:>5} | {:>5} | {:>5} | {:>6}",
        "#", "소재", "공정", "조립기", "제어", "시스템",
        "n6_EX%", "성능", "전력", "비용", "종합");
    println!("  -----+----------------+----------------+----------------+----------------+----------------+---------+-------+-------+-------+--------");

    // Sort pareto by total_score
    let mut pareto_sorted: Vec<usize> = pareto.clone();
    pareto_sorted.sort_by(|&a, &b| results[b].total_score.partial_cmp(&results[a].total_score).unwrap());

    for (i, &idx) in pareto_sorted.iter().enumerate() {
        let r = &results[idx];
        println!("  {:>4} | {:>14} | {:>14} | {:>14} | {:>14} | {:>14} | {:>6.1}% | {:>5.1} | {:>5.1} | {:>5.1} | {:>6.2}",
            i + 1,
            r.elem_name, r.proc_name, r.asm_name, r.ctrl_name, r.fact_name,
            r.n6_exact_pct, r.perf, r.power, r.cost, r.total_score);
    }

    // ============================================================
    // STATISTICS
    // ============================================================
    println!();
    println!("================================================================");
    println!("  STATISTICS");
    println!("================================================================");

    let max_n6 = results.iter().map(|r| r.n6_exact_pct).fold(0.0f64, f64::max);
    let min_n6 = results.iter().map(|r| r.n6_exact_pct).fold(100.0f64, f64::min);
    let avg_n6 = results.iter().map(|r| r.n6_exact_pct).sum::<f64>() / results.len() as f64;
    let above50 = results.iter().filter(|r| r.n6_exact_pct > 50.0).count();
    let above30 = results.iter().filter(|r| r.n6_exact_pct > 30.0).count();
    let above10 = results.iter().filter(|r| r.n6_exact_pct > 10.0).count();

    println!();
    println!("  Total combinations explored:  {}", total_combos);
    println!("  Pareto-optimal solutions:     {}", pareto.len());
    println!();
    println!("  n6 EXACT distribution:");
    println!("    Max n6_EXACT:    {:.1}%", max_n6);
    println!("    Min n6_EXACT:    {:.1}%", min_n6);
    println!("    Avg n6_EXACT:    {:.1}%", avg_n6);
    println!("    n6 > 50%:        {} ({:.1}% of total)", above50, above50 as f64 / total_combos as f64 * 100.0);
    println!("    n6 > 30%:        {} ({:.1}% of total)", above30, above30 as f64 / total_combos as f64 * 100.0);
    println!("    n6 > 10%:        {} ({:.1}% of total)", above10, above10 as f64 / total_combos as f64 * 100.0);

    // Best per objective
    println!();
    println!("  BEST BY OBJECTIVE:");
    let best_n6 = results.iter().max_by(|a, b| a.n6_exact_pct.partial_cmp(&b.n6_exact_pct).unwrap()).unwrap();
    let best_perf = results.iter().max_by(|a, b| a.perf.partial_cmp(&b.perf).unwrap()).unwrap();
    let best_power = results.iter().max_by(|a, b| a.power.partial_cmp(&b.power).unwrap()).unwrap();
    let best_cost = results.iter().max_by(|a, b| a.cost.partial_cmp(&b.cost).unwrap()).unwrap();
    let best_total = &results[0];

    println!("    Best n6_EXACT:  {}+{}+{}+{}+{} = {:.1}%",
        best_n6.elem_id, best_n6.proc_id, best_n6.asm_id, best_n6.ctrl_id, best_n6.fact_id,
        best_n6.n6_exact_pct);
    println!("    Best Perf:      {}+{}+{}+{}+{} = {:.1}",
        best_perf.elem_id, best_perf.proc_id, best_perf.asm_id, best_perf.ctrl_id, best_perf.fact_id,
        best_perf.perf);
    println!("    Best Power:     {}+{}+{}+{}+{} = {:.1}",
        best_power.elem_id, best_power.proc_id, best_power.asm_id, best_power.ctrl_id, best_power.fact_id,
        best_power.power);
    println!("    Best Cost:      {}+{}+{}+{}+{} = {:.1}",
        best_cost.elem_id, best_cost.proc_id, best_cost.asm_id, best_cost.ctrl_id, best_cost.fact_id,
        best_cost.cost);
    println!("    Best Total:     {}+{}+{}+{}+{} = {:.2}",
        best_total.elem_id, best_total.proc_id, best_total.asm_id, best_total.ctrl_id, best_total.fact_id,
        best_total.total_score);

    // ============================================================
    // BEST PATH ASCII DIAGRAM
    // ============================================================
    let best = &results[0];
    println!();
    println!("================================================================");
    println!("  OPTIMAL PATH — 궁극의 물질합성 최적 경로");
    println!("================================================================");
    println!();
    println!("  n6_EXACT = {:.1}% | 성능 = {:.1} | 전력 = {:.1} | 비용 = {:.1} | 종합 = {:.2}",
        best.n6_exact_pct, best.perf, best.power, best.cost, best.total_score);
    println!();
    println!("  ┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐     ┌──────────────┐");
    println!("  │   소재 (L1)  │     │  공정 (L2)   │     │ 조립기 (L3)  │     │  제어 (L4)   │     │ 시스템 (L5)  │");
    println!("  │              │     │              │     │              │     │              │     │              │");
    println!("  │ {:>12} │ ──> │ {:>12} │ ──> │ {:>12} │ ──> │ {:>12} │ ──> │ {:>12} │",
        best.elem_name, best.proc_name, best.asm_name, best.ctrl_name, best.fact_name);
    println!("  │  n6={:<4.0}%    │     │  n6={:<4.0}%    │     │  n6={:<4.0}%    │     │  n6={:<4.0}%    │     │  n6={:<4.0}%    │",
        ELEMENTS.iter().find(|e| e.id == best.elem_id).unwrap().n6_score * 100.0,
        PROCESSES.iter().find(|p| p.id == best.proc_id).unwrap().n6_score * 100.0,
        ASSEMBLERS.iter().find(|a| a.id == best.asm_id).unwrap().n6_score * 100.0,
        CONTROLS.iter().find(|c| c.id == best.ctrl_id).unwrap().n6_score * 100.0,
        FACTORIES.iter().find(|f| f.id == best.fact_id).unwrap().n6_score * 100.0);
    println!("  └──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘     └──────────────┘");
    println!();

    // N6 connection explanation
    println!("  N6 Connection:");
    println!("    L1 {} — {}", best.elem_id, match best.elem_id {
        "E1" => "Carbon Z=6 = n (diamond/graphene/CNT)",
        "E2" => "Silicon Z=14 = sigma+phi",
        "E3" => "Transition Metal CN=6 octahedral",
        "E4" => "Noble Metal Au(79~80=phi^tau*sopfr)",
        "E5" => "Ceramic oxide/nitride",
        _ => "",
    });
    println!("    L2 {} — {}", best.proc_id, match best.proc_id {
        "P1" => "ALD cycle ~ n steps",
        "P2" => "MBE molecular beam",
        "P3" => "CVD chemical vapor",
        "P4" => "Mechanosynthesis (tip)",
        "P5" => "Electrochemistry",
        "P6" => "SelfAssembly hexagonal = n=6 symmetry",
        _ => "",
    });
    println!("    L3 {} — {}", best.asm_id, match best.asm_id {
        "A1" => "STM tip 0.1nm = 1/(sigma-phi)",
        "A2" => "Molecular Assembler",
        "A3" => "DNA origami 6 H-bonds = n",
        "A4" => "Optical Tweezer",
        "A5" => "Focused Ion Beam",
        "A6" => "Nanobot Swarm",
        _ => "",
    });
    println!("    L4 {} — {}", best.ctrl_id, match best.ctrl_id {
        "C1" => "Classical PID",
        "C2" => "Quantum Sensing (NV diamond = C = Z6)",
        "C3" => "AI/ML Control",
        "C4" => "Hybrid Quantum-Classical",
        _ => "",
    });
    println!("    L5 {} — {}", best.fact_id, match best.fact_id {
        "F1" => "Single Station",
        "F2" => "Parallel Array",
        "F3" => "Convergent Assembly",
        "F4" => "Distributed Swarm",
        "F5" => "Self-Replicating (exponential)",
        _ => "",
    });

    // ============================================================
    // N6 EXACT RATIO
    // ============================================================
    println!();
    println!("================================================================");
    println!("  N6 EXACT RATIO (n6_exact > 50%)");
    println!("================================================================");
    println!();
    println!("  {} / {} combinations have n6_EXACT > 50%", above50, total_combos);
    println!("  Ratio: {:.1}%", above50 as f64 / total_combos as f64 * 100.0);
    println!();

    // Breakdown by level — which candidates appear most in top solutions
    println!("  Top-50 element distribution:");
    for elem in ELEMENTS {
        let count = results.iter().take(50).filter(|r| r.elem_id == elem.id).count();
        let bar: String = (0..count).map(|_| '#').collect();
        println!("    {} {:>14}: {:>2} {}", elem.id, elem.name, count, bar);
    }
    println!();
    println!("  Top-50 process distribution:");
    for proc in PROCESSES {
        let count = results.iter().take(50).filter(|r| r.proc_id == proc.id).count();
        let bar: String = (0..count).map(|_| '#').collect();
        println!("    {} {:>14}: {:>2} {}", proc.id, proc.name, count, bar);
    }
    println!();
    println!("  Top-50 assembler distribution:");
    for asm in ASSEMBLERS {
        let count = results.iter().take(50).filter(|r| r.asm_id == asm.id).count();
        let bar: String = (0..count).map(|_| '#').collect();
        println!("    {} {:>14}: {:>2} {}", asm.id, asm.name, count, bar);
    }
    println!();
    println!("  Top-50 control distribution:");
    for ctrl in CONTROLS {
        let count = results.iter().take(50).filter(|r| r.ctrl_id == ctrl.id).count();
        let bar: String = (0..count).map(|_| '#').collect();
        println!("    {} {:>14}: {:>2} {}", ctrl.id, ctrl.name, count, bar);
    }
    println!();
    println!("  Top-50 factory distribution:");
    for fact in FACTORIES {
        let count = results.iter().take(50).filter(|r| r.fact_id == fact.id).count();
        let bar: String = (0..count).map(|_| '#').collect();
        println!("    {} {:>14}: {:>2} {}", fact.id, fact.name, count, bar);
    }

    println!();
    println!("================================================================");
    println!("  DSE 완료 — {} 조합 전수 탐색, Pareto {} 최적 경로 도출", total_combos, pareto.len());
    println!("  sigma({})·phi({}) = n({})·tau({}) = {} -- n=6 is unique", SIGMA, PHI, N, TAU, SIGMA * PHI);
    println!("================================================================");
}
