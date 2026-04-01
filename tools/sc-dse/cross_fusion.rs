/// Cross-DSE: Fusion × Superconductor — 토카막 자석 최적화
///
/// Fusion DSE (5단): 연료 × 가둠 × 자석 × 변환 × 계통
/// SC DSE (6단):     소재 × 공정 × 선재 × 자석구조 × 냉각 × 시스템(=FusionMagnet 고정)
///
/// 교차 방식:
///   Fusion의 자석 레벨(L3)을 SC DSE의 (소재×공정×선재×자석구조×냉각) 5차원으로 확장
///   → Fusion 4단(L1,L2,L4,L5) × SC 5단(소재,공정,선재,자석,냉각) = 9차원 DSE
///
/// 평가: 통합 n6_EXACT + Q-gain(자장 향상) + 비용 + 냉각

// N6 constants
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const MU: u64 = 1;
const J2: u64 = 24;

// ============================================================
// Fusion side (4 levels, excluding magnet)
// ============================================================
#[derive(Clone, Copy)]
struct FusionFuel {
    name: &'static str,
    energy_mev_10x: u64,   // ×10
    n6_exact: u64,         // n6 매칭 수
    n6_total: u64,
}

const FUELS: &[FusionFuel] = &[
    FusionFuel { name: "D-T",    energy_mev_10x: 176, n6_exact: 1, n6_total: 2 },
    FusionFuel { name: "D-D",    energy_mev_10x: 33,  n6_exact: 1, n6_total: 2 },
    FusionFuel { name: "D-He3",  energy_mev_10x: 183, n6_exact: 0, n6_total: 2 },
    FusionFuel { name: "p-B11",  energy_mev_10x: 87,  n6_exact: 0, n6_total: 2 },
    FusionFuel { name: "Li6-D",  energy_mev_10x: 224, n6_exact: 2, n6_total: 2 },
    FusionFuel { name: "Cat-DD", energy_mev_10x: 72,  n6_exact: 0, n6_total: 2 },
];

#[derive(Clone, Copy)]
struct FusionGeometry {
    name: &'static str,
    tf_coils: u64,
    n6_exact: u64,
    n6_total: u64,
}

const GEOMETRIES: &[FusionGeometry] = &[
    FusionGeometry { name: "Tokamak-12",    tf_coils: 12, n6_exact: 1, n6_total: 1 },  // σ
    FusionGeometry { name: "Tokamak-24",    tf_coils: 24, n6_exact: 1, n6_total: 1 },  // J₂
    FusionGeometry { name: "Stellarator-5", tf_coils: 5,  n6_exact: 1, n6_total: 1 },  // sopfr
    FusionGeometry { name: "ST-Compact",    tf_coils: 12, n6_exact: 1, n6_total: 1 },  // σ (compact)
    FusionGeometry { name: "ICF",           tf_coils: 0,  n6_exact: 0, n6_total: 1 },  // 자석 불필요
];

#[derive(Clone, Copy)]
struct FusionConversion {
    name: &'static str,
    efficiency_pct: u64,   // %
    n6_exact: u64,
    n6_total: u64,
}

const CONVERSIONS: &[FusionConversion] = &[
    FusionConversion { name: "Rankine-33",      efficiency_pct: 33,  n6_exact: 1, n6_total: 1 },  // 1/(n/φ)
    FusionConversion { name: "MHD-50",          efficiency_pct: 50,  n6_exact: 1, n6_total: 1 },  // 1/φ
    FusionConversion { name: "TEG-17",          efficiency_pct: 17,  n6_exact: 1, n6_total: 1 },  // 1/n
    FusionConversion { name: "Egyptian-Hybrid", efficiency_pct: 83,  n6_exact: 1, n6_total: 1 },  // 1/2+1/3+1/6
];

#[derive(Clone, Copy)]
struct FusionGrid {
    name: &'static str,
    power_mw: u64,
    n6_exact: u64,
    n6_total: u64,
}

const GRIDS: &[FusionGrid] = &[
    FusionGrid { name: "Central-GW",      power_mw: 1000, n6_exact: 0, n6_total: 1 },
    FusionGrid { name: "Distributed-100",  power_mw: 100,  n6_exact: 1, n6_total: 1 },  // (σ-φ)²
    FusionGrid { name: "Micro-10",         power_mw: 10,   n6_exact: 1, n6_total: 1 },  // σ-φ
    FusionGrid { name: "Fission-Fusion",   power_mw: 500,  n6_exact: 1, n6_total: 1 },  // sopfr·(σ-φ)²
];

// ============================================================
// Superconductor side (5 levels, system=FusionMagnet 고정)
// ============================================================
#[derive(Clone, Copy)]
struct ScMaterial {
    name: &'static str,
    mat_type: &'static str,
    tc_k: u64,
    hc2_t: u64,
    n6_exact: u64,
    n6_total: u64,
    cost_rank: u64,
}

const SC_MATERIALS: &[ScMaterial] = &[
    ScMaterial { name: "NbTi",      mat_type: "LTS", tc_k: 9,   hc2_t: 15,  n6_exact: 2, n6_total: 6, cost_rank: 1 },
    ScMaterial { name: "Nb3Sn",     mat_type: "LTS", tc_k: 18,  hc2_t: 30,  n6_exact: 5, n6_total: 6, cost_rank: 2 },
    ScMaterial { name: "MgB2",      mat_type: "MTS", tc_k: 39,  hc2_t: 16,  n6_exact: 4, n6_total: 6, cost_rank: 2 },
    ScMaterial { name: "REBCO",     mat_type: "HTS", tc_k: 93,  hc2_t: 120, n6_exact: 5, n6_total: 6, cost_rank: 4 },
    ScMaterial { name: "Bi2223",    mat_type: "HTS", tc_k: 110, hc2_t: 50,  n6_exact: 2, n6_total: 6, cost_rank: 3 },
    ScMaterial { name: "BSCCO2212", mat_type: "HTS", tc_k: 85,  hc2_t: 100, n6_exact: 2, n6_total: 6, cost_rank: 3 },
    ScMaterial { name: "FeSe",      mat_type: "HTS", tc_k: 37,  hc2_t: 50,  n6_exact: 1, n6_total: 6, cost_rank: 3 },
];
// LaH10 제외: 핵융합 자석에 DAC 고압 비현실적

#[derive(Clone, Copy)]
struct ScProcess {
    name: &'static str,
    steps: u64,
    n6_exact: u64,
    n6_total: u64,
    throughput: u64,
    compatible: &'static [&'static str],
}

const SC_PROCESSES: &[ScProcess] = &[
    ScProcess { name: "PIT",       steps: 6, n6_exact: 2, n6_total: 3, throughput: 4, compatible: &["LTS", "MTS", "HTS"] },
    ScProcess { name: "MOCVD",     steps: 5, n6_exact: 1, n6_total: 3, throughput: 3, compatible: &["HTS"] },
    ScProcess { name: "MOD_RABiTS",steps: 4, n6_exact: 1, n6_total: 3, throughput: 3, compatible: &["HTS"] },
    ScProcess { name: "Bronze",    steps: 6, n6_exact: 2, n6_total: 3, throughput: 3, compatible: &["LTS"] },
    ScProcess { name: "RCE_DR",    steps: 5, n6_exact: 1, n6_total: 3, throughput: 5, compatible: &["HTS"] },
];

#[derive(Clone, Copy)]
struct ScWire {
    name: &'static str,
    strands: u64,
    je_factor: u64,   // ×10
    n6_exact: u64,
    n6_total: u64,
}

const SC_WIRES: &[ScWire] = &[
    ScWire { name: "Round",      strands: 1,  je_factor: 10, n6_exact: 0, n6_total: 2 },
    ScWire { name: "FlatTape2G", strands: 1,  je_factor: 15, n6_exact: 1, n6_total: 2 },  // 15=σ+n/φ
    ScWire { name: "Rutherford", strands: 12, je_factor: 12, n6_exact: 2, n6_total: 2 },  // 12=σ
    ScWire { name: "CORC",       strands: 6,  je_factor: 14, n6_exact: 1, n6_total: 2 },  // 6=n
];
// ThinFilm 제외: 핵융합 자석에 부적합

#[derive(Clone, Copy)]
struct ScMagnet {
    name: &'static str,
    coils: u64,
    field_limit_t: u64,
    n6_exact: u64,
    n6_total: u64,
}

const SC_MAGNETS: &[ScMagnet] = &[
    ScMagnet { name: "TF-12",     coils: 12, field_limit_t: 35, n6_exact: 2, n6_total: 4 },  // σ coils
    ScMagnet { name: "CS-6",      coils: 6,  field_limit_t: 40, n6_exact: 3, n6_total: 4 },  // n coils, τ(σ-φ)
    ScMagnet { name: "Toroidal-D",coils: 12, field_limit_t: 30, n6_exact: 1, n6_total: 4 },  // σ coils
    ScMagnet { name: "Hybrid-LH", coils: 2,  field_limit_t: 45, n6_exact: 1, n6_total: 4 },  // φ coils, 극한자장
];

#[derive(Clone, Copy)]
struct ScCooling {
    name: &'static str,
    temp_k_10x: u64,
    power_w_per_m: u64,
    n6_exact: u64,
    n6_total: u64,
    cost_rank: u64,
    compatible: &'static [&'static str],
}

const SC_COOLINGS: &[ScCooling] = &[
    ScCooling { name: "LHe_4K",      temp_k_10x: 42,  power_w_per_m: 50, n6_exact: 2, n6_total: 3, cost_rank: 4, compatible: &["LTS", "MTS", "HTS"] },
    ScCooling { name: "NoInsul_20K",  temp_k_10x: 200, power_w_per_m: 20, n6_exact: 1, n6_total: 3, cost_rank: 2, compatible: &["HTS"] },
    ScCooling { name: "CryoCooler",   temp_k_10x: 200, power_w_per_m: 15, n6_exact: 0, n6_total: 3, cost_rank: 3, compatible: &["MTS", "HTS"] },
    ScCooling { name: "Hybrid_4K20K", temp_k_10x: 42,  power_w_per_m: 35, n6_exact: 2, n6_total: 3, cost_rank: 3, compatible: &["LTS", "MTS", "HTS"] },
];

// ============================================================
// 호환성 검사
// ============================================================
fn sc_compatible(mat: &ScMaterial, proc: &ScProcess, cool: &ScCooling) -> bool {
    if !proc.compatible.contains(&mat.mat_type) { return false; }
    if !cool.compatible.contains(&mat.mat_type) { return false; }
    if mat.mat_type == "LTS" && cool.temp_k_10x > 42 { return false; }
    true
}

fn fusion_sc_compatible(geo: &FusionGeometry, mag: &ScMagnet) -> bool {
    // ICF는 자석 불필요 — 제외
    if geo.name == "ICF" { return false; }
    // Tokamak-12 → TF-12 또는 Hybrid-LH 가능
    // Tokamak-24 → 모든 자석 가능 (24코일 중 자석 모듈 선택)
    // Stellarator → 모든 가능
    // ST-Compact → TF-12, CS-6, Hybrid-LH
    true
}

// ============================================================
// 유효 자장 계산
// ============================================================
fn effective_field(mat: &ScMaterial, cool: &ScCooling, mag: &ScMagnet) -> u64 {
    let temp_factor = if cool.temp_k_10x <= 42 {
        100
    } else if mat.mat_type == "HTS" {
        85
    } else {
        30
    };
    let mat_limit = mat.hc2_t * temp_factor / 100;
    mat_limit.min(mag.field_limit_t)
}

// ============================================================
// Q-gain 추정: 자장이 높을수록 Q가 높아짐 (B^4 스케일링)
// Q ∝ β_N^2 · B^4 · a^2 / (n_bar·R) — 간략화
// ============================================================
fn q_gain_factor(field_t: u64) -> f64 {
    // ITER 기준 12T → Q=10, 자장 비례 B^2 (간략)
    let ratio = field_t as f64 / 12.0;
    ratio * ratio  // B² 스케일링
}

// ============================================================
// 결과 구조체
// ============================================================
#[derive(Clone)]
struct CrossResult {
    fuel: &'static str,
    geometry: &'static str,
    conversion: &'static str,
    grid: &'static str,
    sc_mat: &'static str,
    sc_proc: &'static str,
    sc_wire: &'static str,
    sc_mag: &'static str,
    sc_cool: &'static str,
    n6_exact: u64,
    n6_total: u64,
    n6_pct: f64,
    field_t: u64,
    q_gain: f64,
    efficiency: u64,
    cost: u64,
    cool_w: u64,
    score: f64,
}

fn main() {
    let total_raw = FUELS.len() * GEOMETRIES.len() * CONVERSIONS.len() * GRIDS.len()
                  * SC_MATERIALS.len() * SC_PROCESSES.len() * SC_WIRES.len()
                  * SC_MAGNETS.len() * SC_COOLINGS.len();

    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("  Cross-DSE: Fusion × Superconductor — 토카막 자석 최적화");
    println!("  9차원 전수 탐색: Fusion(4) × SC(5)");
    println!("  목표: 30T+ 핵융합 자석, n=6 일관성 극대화");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  Fusion: 연료({}) × 가둠({}) × 변환({}) × 계통({})",
        FUELS.len(), GEOMETRIES.len(), CONVERSIONS.len(), GRIDS.len());
    println!("  SC:     소재({}) × 공정({}) × 선재({}) × 자석({}) × 냉각({})",
        SC_MATERIALS.len(), SC_PROCESSES.len(), SC_WIRES.len(), SC_MAGNETS.len(), SC_COOLINGS.len());
    println!("  총 조합: {} (호환 필터 전)", total_raw);
    println!();

    let mut results: Vec<CrossResult> = Vec::with_capacity(8000);
    let mut filtered = 0u64;

    for fuel in FUELS {
        for geo in GEOMETRIES {
            for conv in CONVERSIONS {
                for grid in GRIDS {
                    for mat in SC_MATERIALS {
                        for proc in SC_PROCESSES {
                            for wire in SC_WIRES {
                                for mag in SC_MAGNETS {
                                    for cool in SC_COOLINGS {
                                        // 호환성 체크
                                        if !sc_compatible(mat, proc, cool) {
                                            filtered += 1;
                                            continue;
                                        }
                                        if !fusion_sc_compatible(geo, mag) {
                                            filtered += 1;
                                            continue;
                                        }

                                        // 유효 자장
                                        let field = effective_field(mat, cool, mag);

                                        // 핵융합 최소 자장: 12T (ITER급 이상)
                                        if field < 12 {
                                            filtered += 1;
                                            continue;
                                        }

                                        // n6 EXACT 통합 계산
                                        let n6_exact = fuel.n6_exact + geo.n6_exact
                                            + conv.n6_exact + grid.n6_exact
                                            + mat.n6_exact + proc.n6_exact
                                            + wire.n6_exact + mag.n6_exact + cool.n6_exact
                                            + 2; // 보편: Abrikosov(CN=6) + BCS(12)
                                        let n6_total = fuel.n6_total + geo.n6_total
                                            + conv.n6_total + grid.n6_total
                                            + mat.n6_total + proc.n6_total
                                            + wire.n6_total + mag.n6_total + cool.n6_total
                                            + 2;
                                        let n6_pct = (n6_exact as f64) / (n6_total as f64) * 100.0;

                                        // Q-gain
                                        let q_gain = q_gain_factor(field);

                                        // 비용
                                        let cost = mat.cost_rank * 20
                                            + (5 - proc.throughput.min(5)) * 10
                                            + cool.cost_rank * 15;

                                        let cool_w = cool.power_w_per_m;
                                        let eff = conv.efficiency_pct;

                                        // 통합 스코어
                                        // 20% n6 + 25% Q-gain + 20% field + 15% efficiency + 10% cost + 10% cooling
                                        let field_norm = (field as f64) / 45.0 * 100.0;
                                        let q_norm = (q_gain / 14.0 * 100.0).min(100.0); // max ~14× at 45T
                                        let eff_norm = eff as f64;
                                        let cost_norm = (150.0 - cost as f64).max(0.0) / 1.5;
                                        let cool_norm = (100.0 - cool_w as f64).max(0.0);

                                        let score = n6_pct * 0.20
                                            + q_norm * 0.25
                                            + field_norm * 0.20
                                            + eff_norm * 0.15
                                            + cost_norm * 0.10
                                            + cool_norm * 0.10;

                                        results.push(CrossResult {
                                            fuel: fuel.name,
                                            geometry: geo.name,
                                            conversion: conv.name,
                                            grid: grid.name,
                                            sc_mat: mat.name,
                                            sc_proc: proc.name,
                                            sc_wire: wire.name,
                                            sc_mag: mag.name,
                                            sc_cool: cool.name,
                                            n6_exact,
                                            n6_total,
                                            n6_pct,
                                            field_t: field,
                                            q_gain,
                                            efficiency: eff,
                                            cost,
                                            cool_w,
                                            score,
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let valid = results.len();
    println!("  호환 필터: {} 제거 → {} 유효 조합", filtered, valid);
    println!();

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());

    // ── TOP 20 ──
    println!("═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  TOP 20 CROSS-DSE (Fusion × Superconductor)");
    println!("═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  {:>3} │ {:>6} │ {:>12} │ {:>15} │ {:>8} │ {:>10} │ {:>7} │ {:>10} │ {:>8} │ {:>12} │ {:>5} │ {:>3} │ {:>4} │ {:>3} │ {:>3} │ {:>6}",
        "#", "Fuel", "Geometry", "Conversion", "Grid", "SC-Mat", "SC-Proc", "SC-Wire", "SC-Mag", "SC-Cool", "n6%", "B(T)", "Q×", "Eff", "C/W", "Score");
    println!("  ────┼────────┼──────────────┼─────────────────┼──────────┼────────────┼─────────┼────────────┼──────────┼──────────────┼───────┼─────┼──────┼─────┼─────┼───────");

    for (i, r) in results.iter().take(20).enumerate() {
        println!("  {:>3} │ {:>6} │ {:>12} │ {:>15} │ {:>8} │ {:>10} │ {:>7} │ {:>10} │ {:>8} │ {:>12} │{:>4.0}% │ {:>3}T │{:>4.1}× │ {:>2}% │ {:>2}W │ {:>6.2}",
            i + 1, r.fuel, r.geometry, r.conversion, r.grid,
            r.sc_mat, r.sc_proc, r.sc_wire, r.sc_mag, r.sc_cool,
            r.n6_pct, r.field_t, r.q_gain, r.efficiency, r.cool_w, r.score);
    }

    // ── 30T+ 필터 ──
    println!();
    println!("═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  30T+ FUSION MAGNETS — 궁극의 핵융합 자석 경로");
    println!("═════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();

    let fusion_30: Vec<&CrossResult> = results.iter()
        .filter(|r| r.field_t >= 30)
        .collect();

    if fusion_30.is_empty() {
        println!("  (30T+ 조합 없음)");
    } else {
        println!("  {:>3} │ {:>6} │ {:>12} │ {:>15} │ {:>10} │ {:>7} │ {:>10} │ {:>8} │ {:>12} │ {:>5} │ {:>3} │ {:>4} │ {:>6}",
            "#", "Fuel", "Geometry", "Conversion", "SC-Mat", "SC-Proc", "SC-Wire", "SC-Mag", "SC-Cool", "n6%", "B(T)", "Q×", "Score");
        println!("  ────┼────────┼──────────────┼─────────────────┼────────────┼─────────┼────────────┼──────────┼──────────────┼───────┼─────┼──────┼───────");

        for (i, r) in fusion_30.iter().take(20).enumerate() {
            println!("  {:>3} │ {:>6} │ {:>12} │ {:>15} │ {:>10} │ {:>7} │ {:>10} │ {:>8} │ {:>12} │{:>4.0}% │ {:>3}T │{:>4.1}× │ {:>6.2}",
                i + 1, r.fuel, r.geometry, r.conversion,
                r.sc_mat, r.sc_proc, r.sc_wire, r.sc_mag, r.sc_cool,
                r.n6_pct, r.field_t, r.q_gain, r.score);
        }
        println!();
        println!("  30T+ 조합: {} / {} 유효 ({:.1}%)",
            fusion_30.len(), valid, fusion_30.len() as f64 / valid as f64 * 100.0);
    }

    // ── STATISTICS ──
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("  CROSS-DSE STATISTICS");
    println!("═══════════════════════════════════════════════════════════════════════════");

    let max_n6 = results.iter().map(|r| r.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = if !results.is_empty() { results.iter().map(|r| r.n6_pct).sum::<f64>() / valid as f64 } else { 0.0 };
    let above90 = results.iter().filter(|r| r.n6_pct >= 90.0).count();
    let above80 = results.iter().filter(|r| r.n6_pct >= 80.0).count();
    let max_field = results.iter().map(|r| r.field_t).max().unwrap_or(0);
    let max_q = results.iter().map(|r| r.q_gain).fold(0.0f64, f64::max);
    let f30_count = fusion_30.len();

    println!();
    println!("  Total raw:     {}", total_raw);
    println!("  Filtered:      {}", filtered);
    println!("  Valid:         {}", valid);
    println!("  30T+ fusion:   {} ({:.1}%)", f30_count, if valid > 0 { f30_count as f64 / valid as f64 * 100.0 } else { 0.0 });
    println!();
    println!("  n6 EXACT max:  {:.1}%", max_n6);
    println!("  n6 EXACT avg:  {:.1}%", avg_n6);
    println!("  n6 ≥90%:       {} ({:.1}%)", above90, if valid > 0 { above90 as f64 / valid as f64 * 100.0 } else { 0.0 });
    println!("  n6 ≥80%:       {} ({:.1}%)", above80, if valid > 0 { above80 as f64 / valid as f64 * 100.0 } else { 0.0 });
    println!("  Max field:     {}T", max_field);
    println!("  Max Q-gain:    {:.1}× (vs ITER 12T baseline)", max_q);

    // ── BEST BY CATEGORY ──
    println!();
    println!("  BEST BY CATEGORY:");

    if let Some(r) = results.iter().max_by(|a, b| a.n6_pct.partial_cmp(&b.n6_pct).unwrap()) {
        println!("    Best n6:   {} + {} + {} | {} + {} + {} + {} + {} ({:.1}%)",
            r.fuel, r.geometry, r.conversion, r.sc_mat, r.sc_proc, r.sc_wire, r.sc_mag, r.sc_cool, r.n6_pct);
    }
    if let Some(r) = results.iter().max_by_key(|r| r.field_t) {
        println!("    Best B:    {} + {} | {} + {} + {} + {} ({}T, Q={:.1}×)",
            r.fuel, r.geometry, r.sc_mat, r.sc_proc, r.sc_wire, r.sc_mag, r.field_t, r.q_gain);
    }
    if let Some(r) = results.first() {
        println!("    Best Pareto: {} + {} + {} + {} | {} + {} + {} + {} + {} (score={:.2})",
            r.fuel, r.geometry, r.conversion, r.grid,
            r.sc_mat, r.sc_proc, r.sc_wire, r.sc_mag, r.sc_cool, r.score);
    }

    // ── N6 CONVERGENCE MAP ──
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("  FUSION × SC n=6 수렴 지도");
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  ┌───────────────────────────────────────────────────────────────┐");
    println!("  │ Fusion 측 n=6                                                │");
    println!("  │   Li6-D: 에너지 22.4 MeV ≈ J₂-φ = 22                       │");
    println!("  │   Tokamak-12: σ=12 TF 코일                                  │");
    println!("  │   Egyptian-Hybrid: 1/2+1/3+1/6 = 1 효율 ≈ 83%              │");
    println!("  │   Distributed-100MW: (σ-φ)² = 100                           │");
    println!("  ├───────────────────────────────────────────────────────────────┤");
    println!("  │ SC 측 n=6                                                    │");
    println!("  │   REBCO: 1:2:3 sum=n=6, Hc2=120=σ(σ-φ)                    │");
    println!("  │   Nb₃Sn: 6 Nb=n, Tc=18=3n, Hc2=30=5n                     │");
    println!("  │   PIT/Bronze: 6 공정 단계 = n                               │");
    println!("  │   CORC: 6 tape = n, Rutherford: 12 strand = σ              │");
    println!("  │   CS-6: 6 모듈 = n, field=40=τ(σ-φ)                       │");
    println!("  │   LHe 4.2K ≈ τ=4                                           │");
    println!("  ├───────────────────────────────────────────────────────────────┤");
    println!("  │ 교차 수렴                                                    │");
    println!("  │   Tokamak σ=12 코일 × SC σ=12T 자장 → σ² = 144 복합일치    │");
    println!("  │   Egyptian 효율 × REBCO 1:2:3 → 같은 Egyptian 구조           │");
    println!("  │   CS-6 모듈 × Fusion n=6 연료 → n 이중 수렴                 │");
    println!("  │   J₂=24km 선재 × Tokamak-24 코일 → J₂ 이중 수렴            │");
    println!("  └───────────────────────────────────────────────────────────────┘");

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════");
    println!("  ✅ Cross-DSE 완료 — {} 전수 / {} 유효 / 30T+ 경로 {} 조합",
        total_raw, valid, f30_count);
    println!("═══════════════════════════════════════════════════════════════════════════");
}
