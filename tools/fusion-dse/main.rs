#![allow(unused)]

/// N6 Fusion Design Space Exploration (DSE) — 전수 조합 탐색기
///
/// 방식 → 소재(SC×블랭킷×구조재) → 코어(가열×가둠×연료) → 장치(TF×기하×자장×Q) → 시스템(발전×TBR×계통)
/// 14 파라미터 전수 조합으로 최적 핵융합 아키텍처 도출.
///
/// 평가: n6_EXACT(35%) + Q_gain(25%) + TRL(20%) + LCOE(12%) + T_comm(8%)
/// 출력: Pareto frontier + 최적 경로 + 방식별 분석 + ASCII 다이어그램

// N6 constants
const N: u64 = 6;
const PHI: u64 = 2;
const TAU: u64 = 4;
const SIGMA: u64 = 12;
const SOPFR: u64 = 5;
const MU: u64 = 1;
const J2: u64 = 24;

const N6_BASE: &[u64] = &[1, 2, 3, 4, 5, 6, 12, 24];

fn is_n6_base(v: u64) -> bool { N6_BASE.contains(&v) }

fn build_n6_derived_set() -> Vec<u64> {
    let mut set = Vec::new();
    for &a in N6_BASE {
        if !set.contains(&a) { set.push(a); }
        for &b in N6_BASE {
            for v in &[a + b, a * b] {
                if !set.contains(v) { set.push(*v); }
            }
            if a > b { let v = a - b; if !set.contains(&v) { set.push(v); } }
            if b > 0 && a % b == 0 { let v = a / b; if !set.contains(&v) { set.push(v); } }
        }
    }
    set.sort();
    set
}

static mut N6_DERIVED_SET: Option<Vec<u64>> = None;

fn init_n6_derived() {
    unsafe { N6_DERIVED_SET = Some(build_n6_derived_set()); }
}

fn is_n6_derived(v: u64) -> bool {
    unsafe { N6_DERIVED_SET.as_ref().unwrap().contains(&v) }
}

fn pad(s: &str, width: usize) -> String {
    if s.len() >= width { s.to_string() } else { format!("{}{}", s, " ".repeat(width - s.len())) }
}

// ============================================================
// Level 1: 방식 (Scheme) — 6 types
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Scheme {
    name: &'static str,
    q_upper: u64,       // Q 상한
    trl: u64,           // Technology Readiness Level
    lcoe_est: u64,      // LCOE $/MWh 추정
    t_comm: u64,        // 상용화 예상 연도
    pf_coils: u64,      // PF 코일 수
    cs_modules: u64,    // CS 모듈 수
    n6_bonus: u64,      // n6 보너스 점수
}

const SCHEMES: &[Scheme] = &[
    Scheme { name: "Tokamak",     q_upper: 10,  trl: 7, lcoe_est: 60,  t_comm: 2035, pf_coils: 6, cs_modules: 6, n6_bonus: 5 },
    Scheme { name: "Stellarator", q_upper: 5,   trl: 5, lcoe_est: 80,  t_comm: 2040, pf_coils: 0, cs_modules: 0, n6_bonus: 2 },
    Scheme { name: "ICF_Laser",   q_upper: 2,   trl: 4, lcoe_est: 200, t_comm: 2045, pf_coils: 0, cs_modules: 0, n6_bonus: 1 },
    Scheme { name: "FRC",         q_upper: 2,   trl: 3, lcoe_est: 100, t_comm: 2042, pf_coils: 0, cs_modules: 0, n6_bonus: 0 },
    Scheme { name: "Mirror",      q_upper: 1,   trl: 3, lcoe_est: 150, t_comm: 2045, pf_coils: 0, cs_modules: 0, n6_bonus: 0 },
    Scheme { name: "Z_Pinch",     q_upper: 1,   trl: 2, lcoe_est: 300, t_comm: 2050, pf_coils: 0, cs_modules: 0, n6_bonus: 0 },
];

// ============================================================
// Level 2: 소재 — 초전도체 × 블랭킷 × 구조재
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Superconductor {
    name: &'static str,
    tc_k: u64,          // 임계온도 K
    b_max_t: u64,       // 최대자장 T
    cost_rank: u64,
}

const SUPERCONDUCTORS: &[Superconductor] = &[
    Superconductor { name: "NbTi",  tc_k: 9,   b_max_t: 10,  cost_rank: 1 },
    Superconductor { name: "Nb3Sn", tc_k: 18,  b_max_t: 24,  cost_rank: 2 },
    Superconductor { name: "REBCO", tc_k: 92,  b_max_t: 45,  cost_rank: 4 },
    Superconductor { name: "BSCCO", tc_k: 108, b_max_t: 35,  cost_rank: 3 },
];

#[derive(Clone, Copy, Debug)]
struct Blanket {
    name: &'static str,
    tbr: u64,           // TBR x 100
    coolant_types: u64,
    max_temp_c: u64,
    cost_rank: u64,
}

const BLANKETS: &[Blanket] = &[
    Blanket { name: "Li_ceramic", tbr: 105, coolant_types: 1, max_temp_c: 550,  cost_rank: 2 },
    Blanket { name: "PbLi_eut",   tbr: 115, coolant_types: 2, max_temp_c: 700,  cost_rank: 3 },
    Blanket { name: "FLiBe_molt", tbr: 110, coolant_types: 1, max_temp_c: 800,  cost_rank: 4 },
    Blanket { name: "He_pebble",  tbr: 108, coolant_types: 1, max_temp_c: 500,  cost_rank: 2 },
];

#[derive(Clone, Copy, Debug)]
struct Structural {
    name: &'static str,
    dpa_limit: u64,
    max_temp_c: u64,
    cost_rank: u64,
}

const STRUCTURALS: &[Structural] = &[
    Structural { name: "RAFM",    dpa_limit: 80,  max_temp_c: 550,  cost_rank: 1 },
    Structural { name: "V_alloy", dpa_limit: 150, max_temp_c: 700,  cost_rank: 3 },
    Structural { name: "SiC_SiC", dpa_limit: 200, max_temp_c: 1000, cost_rank: 4 },
];

// ============================================================
// Level 3: 코어 — 가열 × 가둠 × 연료
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Heating {
    name: &'static str,
    freq_or_energy: u64,
    efficiency_pct: u64,
    typical_mw: u64,
}

const HEATINGS: &[Heating] = &[
    Heating { name: "NBI",  freq_or_energy: 120, efficiency_pct: 40, typical_mw: 8 },
    Heating { name: "ICRH", freq_or_energy: 60,  efficiency_pct: 60, typical_mw: 6 },
    Heating { name: "ECRH", freq_or_energy: 170, efficiency_pct: 70, typical_mw: 1 },
    Heating { name: "LHCD", freq_or_energy: 5,   efficiency_pct: 65, typical_mw: 3 },
];

#[derive(Clone, Copy, Debug)]
struct Confinement {
    name: &'static str,
    b_range_min: u64,
    b_range_max: u64,
    cost_rank: u64,
}

const CONFINEMENTS: &[Confinement] = &[
    Confinement { name: "SC_coil",     b_range_min: 5,  b_range_max: 20, cost_rank: 4 },
    Confinement { name: "Normal_Cu",   b_range_min: 2,  b_range_max: 8,  cost_rank: 1 },
    Confinement { name: "Perm_Hybrid", b_range_min: 3,  b_range_max: 12, cost_rank: 2 },
];

#[derive(Clone, Copy, Debug)]
struct Fuel {
    name: &'static str,
    q_value_10x: u64,
    cross_section_kev: u64,
    neutron_fraction_pct: u64,
    d_mass: u64,
}

const FUELS: &[Fuel] = &[
    Fuel { name: "D_T",   q_value_10x: 176, cross_section_kev: 10,  neutron_fraction_pct: 80, d_mass: 5 },
    Fuel { name: "D_D",   q_value_10x: 37,  cross_section_kev: 100, neutron_fraction_pct: 33, d_mass: 4 },
    Fuel { name: "D_He3", q_value_10x: 183, cross_section_kev: 200, neutron_fraction_pct: 5,  d_mass: 5 },
    Fuel { name: "p_B11", q_value_10x: 87,  cross_section_kev: 500, neutron_fraction_pct: 0,  d_mass: 12 },
];

// ============================================================
// Level 4: 장치 — TF 코일 × 기하 × 자장 × Q 목표
// ============================================================
#[derive(Clone, Copy, Debug)]
struct TfConfig {
    name: &'static str,
    coil_count: u64,
}

const TF_CONFIGS: &[TfConfig] = &[
    TfConfig { name: "TF6",  coil_count: 6 },
    TfConfig { name: "TF12", coil_count: 12 },
    TfConfig { name: "TF16", coil_count: 16 },
    TfConfig { name: "TF18", coil_count: 18 },
];

#[derive(Clone, Copy, Debug)]
struct Geometry {
    name: &'static str,
    aspect_ratio_10x: u64,
}

const GEOMETRIES: &[Geometry] = &[
    Geometry { name: "A2.5", aspect_ratio_10x: 25 },
    Geometry { name: "A3.0", aspect_ratio_10x: 30 },
    Geometry { name: "A3.1", aspect_ratio_10x: 31 },
    Geometry { name: "A4.0", aspect_ratio_10x: 40 },
    Geometry { name: "A5.0", aspect_ratio_10x: 50 },
];

#[derive(Clone, Copy, Debug)]
struct BField {
    name: &'static str,
    bt_t: u64,
}

const BFIELDS: &[BField] = &[
    BField { name: "B5",  bt_t: 5 },
    BField { name: "B12", bt_t: 12 },
    BField { name: "B20", bt_t: 20 },
];

#[derive(Clone, Copy, Debug)]
struct QTarget {
    name: &'static str,
    q_value: u64,
}

const QTARGETS: &[QTarget] = &[
    QTarget { name: "Q2",    q_value: 2 },
    QTarget { name: "Q10",   q_value: 10 },
    QTarget { name: "Q_ign", q_value: 1000 },
];

// ============================================================
// Level 5: 시스템 — 발전 × TBR × 계통
// ============================================================
#[derive(Clone, Copy, Debug)]
struct PowerConversion {
    name: &'static str,
    efficiency_pct: u64,
    maturity: u64,
}

const POWER_CONVERSIONS: &[PowerConversion] = &[
    PowerConversion { name: "Rankine",   efficiency_pct: 33, maturity: 5 },
    PowerConversion { name: "Brayton",   efficiency_pct: 45, maturity: 3 },
    PowerConversion { name: "Direct_CV", efficiency_pct: 60, maturity: 1 },
];

#[derive(Clone, Copy, Debug)]
struct TbrStrategy {
    name: &'static str,
    tbr_100x: u64,
    cost_rank: u64,
}

const TBR_STRATEGIES: &[TbrStrategy] = &[
    TbrStrategy { name: "Li6_ceramic", tbr_100x: 105, cost_rank: 2 },
    TbrStrategy { name: "PbLi_self",   tbr_100x: 115, cost_rank: 3 },
    TbrStrategy { name: "DCLL_dual",   tbr_100x: 120, cost_rank: 4 },
];

#[derive(Clone, Copy, Debug)]
struct Grid {
    name: &'static str,
    freq_hz: u64,
    voltage_kv: u64,
}

const GRIDS: &[Grid] = &[
    Grid { name: "AC_50Hz", freq_hz: 50,  voltage_kv: 500 },
    Grid { name: "AC_60Hz", freq_hz: 60,  voltage_kv: 500 },
    Grid { name: "HVDC",    freq_hz: 0,   voltage_kv: 800 },
];

// ============================================================
// 물리적 호환성 필터
// ============================================================
fn is_compatible(sc: &Superconductor, bf: &BField, conf: &Confinement, fuel: &Fuel) -> bool {
    // B_T > SC B_max: 초전도체 한계 초과
    if bf.bt_t > sc.b_max_t {
        return false;
    }
    // Normal_Cu 코일은 B > 8T 불가
    if conf.name == "Normal_Cu" && bf.bt_t > 8 {
        return false;
    }
    // p-B11 연료는 B < 12T에서 비실용적
    if fuel.name == "p_B11" && bf.bt_t < 12 {
        return false;
    }
    true
}

// ============================================================
// N6 EXACT 계산 — 14 파라미터 검증
// ============================================================
fn count_n6_exact(
    sch: &Scheme, sc: &Superconductor, bl: &Blanket, st: &Structural,
    ht: &Heating, conf: &Confinement, fuel: &Fuel,
    tf: &TfConfig, geo: &Geometry, bf: &BField, qt: &QTarget,
    pc: &PowerConversion, tbr: &TbrStrategy, gr: &Grid,
) -> (u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;

    // --- Level 1: 방식 ---
    // PF coils == N?
    total += 1;
    if sch.pf_coils == N { exact += 1; }
    // CS modules == N?
    total += 1;
    if sch.cs_modules == N { exact += 1; }

    // --- Level 2: 소재 ---
    // SC B_max: n6 derivable?
    total += 1;
    if is_n6_derived(sc.b_max_t) { exact += 1; }
    // Blanket coolant types: n6 base?
    total += 1;
    if is_n6_base(bl.coolant_types) { exact += 1; }

    // --- Level 3: 코어 ---
    // Heating typical MW: n6 base?
    total += 1;
    if is_n6_base(ht.typical_mw) { exact += 1; }
    // Fuel d_mass: n6 base?
    total += 1;
    if is_n6_base(fuel.d_mass) { exact += 1; }
    // Neutron fraction == 80? (tau*sopfr*tau = 4*5*4 = 80, or φ^τ·sopfr = 16*5 = 80)
    total += 1;
    if fuel.neutron_fraction_pct == PHI.pow(TAU as u32) * SOPFR { exact += 1; }

    // --- Level 4: 장치 ---
    // TF coil count: n6 base?
    total += 1;
    if is_n6_base(tf.coil_count) { exact += 1; }
    // Aspect ratio 10x == 30? (n/phi * 10 = 3*10 = 30)
    total += 1;
    if geo.aspect_ratio_10x == (N / PHI) * 10 { exact += 1; }
    // B_T == 12? (sigma)
    total += 1;
    if bf.bt_t == SIGMA { exact += 1; }
    // Q == 10? (sopfr * phi = 5*2 = 10)
    total += 1;
    if qt.q_value == SOPFR * PHI { exact += 1; }

    // --- Level 5: 시스템 ---
    // Efficiency == 33? (approx 1/3 * 100)
    total += 1;
    if pc.efficiency_pct == 100 / (N / PHI) { exact += 1; } // 33 ~ 100/3
    // TBR 100x == 120? (sigma / (sigma - phi) * 100 = 12/10 * 100 = 120)
    total += 1;
    if tbr.tbr_100x == SIGMA * 10 { exact += 1; }
    // Grid freq: 60 == sigma*sopfr? or 50 == sopfr*(sigma-phi)?
    total += 1;
    if gr.freq_hz == SIGMA * SOPFR || gr.freq_hz == SOPFR * (SIGMA - PHI) { exact += 1; }

    (exact, total)
}

// ============================================================
// 5축 스코어링
// ============================================================

/// Q_gain: 핵융합 이득 점수
fn q_gain_score(sch: &Scheme, qt: &QTarget, fuel: &Fuel) -> u64 {
    let q_eff = if sch.q_upper < qt.q_value { sch.q_upper } else { qt.q_value };
    let cs = if fuel.cross_section_kev > 0 { fuel.cross_section_kev } else { 1 };
    let raw = q_eff * 10 * 10 / cs; // scale up
    if raw > 100 { 100 } else { raw }
}

/// TRL: 기술 성숙도 점수
fn trl_score(sch: &Scheme, pc: &PowerConversion) -> u64 {
    let raw = sch.trl * 10 + pc.maturity * 4;
    if raw > 100 { 100 } else { raw }
}

/// LCOE: 발전단가 점수 (낮을수록 좋음 -> 높은 점수)
fn lcoe_score(sch: &Scheme, fuel: &Fuel, sc: &Superconductor, pc: &PowerConversion) -> u64 {
    let fuel_diff = fuel.cross_section_kev / 10; // difficulty
    let sc_cost = sc.cost_rank * 5;
    let eff_bonus = if pc.efficiency_pct > 40 { 10 } else { 0 };
    let lcoe_raw = sch.lcoe_est + fuel_diff + sc_cost;
    let lcoe_adj = if lcoe_raw > eff_bonus { lcoe_raw - eff_bonus } else { 0 };
    let score = if lcoe_adj < 500 { 100 - lcoe_adj / 5 } else { 0 };
    score
}

/// T_comm: 상용화 시점 점수 (빠를수록 높음)
fn t_comm_score(sch: &Scheme) -> u64 {
    let raw = if sch.t_comm < 2055 { (2055 - sch.t_comm) * 5 } else { 0 };
    if raw > 100 { 100 } else { raw }
}

// ============================================================
// 결과 구조체
// ============================================================
#[derive(Clone)]
struct DseResult {
    scheme: &'static str,
    sc: &'static str,
    blanket: &'static str,
    structural: &'static str,
    heating: &'static str,
    confinement: &'static str,
    fuel: &'static str,
    tf: &'static str,
    geometry: &'static str,
    bfield: &'static str,
    qtarget: &'static str,
    power_conv: &'static str,
    tbr_strat: &'static str,
    grid: &'static str,
    n6_exact: u64,
    n6_total: u64,
    n6_pct: f64,
    q_gain: u64,
    trl: u64,
    lcoe: u64,
    t_comm: u64,
    pareto_score: f64,
}

/// Insert into top-K sorted vec (descending by pareto_score), keeping at most `cap` entries.
fn topk_insert(results: &mut Vec<DseResult>, entry: DseResult, cap: usize) {
    // Find insertion point (binary search for descending order)
    let pos = results.partition_point(|r| r.pareto_score >= entry.pareto_score);
    if pos >= cap {
        return; // worse than all top-K
    }
    results.insert(pos, entry);
    if results.len() > cap {
        results.pop();
    }
}

const TOP_K: usize = 200; // keep top 200, display top 20

fn main() {
    init_n6_derived();

    let total_combos: usize =
        SCHEMES.len() * SUPERCONDUCTORS.len() * BLANKETS.len() * STRUCTURALS.len()
        * HEATINGS.len() * CONFINEMENTS.len() * FUELS.len()
        * TF_CONFIGS.len() * GEOMETRIES.len() * BFIELDS.len() * QTARGETS.len()
        * POWER_CONVERSIONS.len() * TBR_STRATEGIES.len() * GRIDS.len();

    println!("{}",  "=".repeat(90));
    println!("  N6 Fusion Design Space Exploration (DSE)");
    println!("  방식 x 소재(SC x 블랭킷 x 구조재) x 코어(가열 x 가둠 x 연료)");
    println!("  x 장치(TF x 기하 x 자장 x Q) x 시스템(발전 x TBR x 계통)");
    println!("{}", "=".repeat(90));
    println!();
    println!("  후보군:");
    println!("    방식:     {} ({})", SCHEMES.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SCHEMES.len());
    println!("    초전도체: {} ({})", SUPERCONDUCTORS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SUPERCONDUCTORS.len());
    println!("    블랭킷:   {} ({})", BLANKETS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), BLANKETS.len());
    println!("    구조재:   {} ({})", STRUCTURALS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), STRUCTURALS.len());
    println!("    가열:     {} ({})", HEATINGS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), HEATINGS.len());
    println!("    가둠:     {} ({})", CONFINEMENTS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), CONFINEMENTS.len());
    println!("    연료:     {} ({})", FUELS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), FUELS.len());
    println!("    TF코일:   {} ({})", TF_CONFIGS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), TF_CONFIGS.len());
    println!("    기하:     {} ({})", GEOMETRIES.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), GEOMETRIES.len());
    println!("    자장:     {} ({})", BFIELDS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), BFIELDS.len());
    println!("    Q목표:    {} ({})", QTARGETS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), QTARGETS.len());
    println!("    발전:     {} ({})", POWER_CONVERSIONS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), POWER_CONVERSIONS.len());
    println!("    TBR:      {} ({})", TBR_STRATEGIES.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), TBR_STRATEGIES.len());
    println!("    계통:     {} ({})", GRIDS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), GRIDS.len());
    println!();
    println!("  이론적 총 조합: {}", total_combos);
    println!("  탐색 시작...");
    println!();

    let mut results: Vec<DseResult> = Vec::with_capacity(TOP_K + 1);
    let mut skipped: u64 = 0;
    let mut evaluated: u64 = 0;

    // Inline statistics
    let mut sum_n6: f64 = 0.0;
    let mut max_n6: f64 = 0.0;
    let mut count_above80: u64 = 0;
    let mut count_above60: u64 = 0;

    // Best-by-axis trackers
    let mut best_n6_result: Option<DseResult> = None;
    let mut best_q_result: Option<DseResult> = None;
    let mut best_trl_result: Option<DseResult> = None;
    let mut best_lcoe_result: Option<DseResult> = None;

    // Per-scheme best
    let mut scheme_best: Vec<Option<DseResult>> = (0..SCHEMES.len()).map(|_| None).collect();

    // Precompute per-level n6 contributions to enable upper-bound pruning.
    // Max possible n6_exact from Level 5 inner loops (qt, pc, tbr, gr) = 4 out of 4 checks
    // Max Q_gain = 100, max TRL = 100, max LCOE = 100, max T_comm = 100
    // Upper-bound Pareto for pruning: assume all remaining axes score maximum.

    let max_q_gain_possible: f64 = 100.0;
    let max_trl_possible: f64 = 100.0;
    let max_lcoe_possible: f64 = 100.0;
    let max_t_comm_possible: f64 = 100.0;

    for (si, sch) in SCHEMES.iter().enumerate() {
        // Precompute Level 1 n6 partial
        let mut l1_exact: u64 = 0;
        if sch.pf_coils == N { l1_exact += 1; }
        if sch.cs_modules == N { l1_exact += 1; }
        let l1_t_comm = t_comm_score(sch);

        for sc in SUPERCONDUCTORS {
            // Level 2 partial: SC B_max
            let l2_sc_exact: u64 = if is_n6_derived(sc.b_max_t) { 1 } else { 0 };

            for bl in BLANKETS {
                let l2_bl_exact: u64 = if is_n6_base(bl.coolant_types) { 1 } else { 0 };

                for st in STRUCTURALS {
                    for ht in HEATINGS {
                        let l3_ht_exact: u64 = if is_n6_base(ht.typical_mw) { 1 } else { 0 };

                        for conf in CONFINEMENTS {
                            for fuel in FUELS {
                                let l3_fuel_exact: u64 = if is_n6_base(fuel.d_mass) { 1 } else { 0 };
                                let l3_neut_exact: u64 = if fuel.neutron_fraction_pct == PHI.pow(TAU as u32) * SOPFR { 1 } else { 0 };

                                // Partial n6 from levels 1-3 (7 checks out of 15 total)
                                let partial_exact_1_3 = l1_exact + l2_sc_exact + l2_bl_exact
                                    + l3_ht_exact + l3_fuel_exact + l3_neut_exact;

                                for tf in TF_CONFIGS {
                                    let l4_tf_exact: u64 = if is_n6_base(tf.coil_count) { 1 } else { 0 };

                                    for geo in GEOMETRIES {
                                        let l4_geo_exact: u64 = if geo.aspect_ratio_10x == (N / PHI) * 10 { 1 } else { 0 };

                                        for bf in BFIELDS {
                                            if !is_compatible(sc, bf, conf, fuel) {
                                                skipped += QTARGETS.len() as u64
                                                    * POWER_CONVERSIONS.len() as u64
                                                    * TBR_STRATEGIES.len() as u64
                                                    * GRIDS.len() as u64;
                                                continue;
                                            }

                                            let l4_bf_exact: u64 = if bf.bt_t == SIGMA { 1 } else { 0 };

                                            // Partial n6 from levels 1-4 (minus qt which is inner)
                                            let partial_exact_1_4_no_qt = partial_exact_1_3
                                                + l4_tf_exact + l4_geo_exact + l4_bf_exact;

                                            // Upper bound: remaining 4 checks (qt, pc eff, tbr, grid) all EXACT
                                            let max_remaining_exact: u64 = 4;
                                            let n6_total: u64 = 15;
                                            let upper_n6_pct = ((partial_exact_1_4_no_qt + 1 + max_remaining_exact) as f64)
                                                / (n6_total as f64) * 100.0;
                                            let upper_pareto = upper_n6_pct * 0.35
                                                + max_q_gain_possible * 0.25
                                                + max_trl_possible * 0.20
                                                + max_lcoe_possible * 0.12
                                                + max_t_comm_possible * 0.08;

                                            // Prune if upper bound can't beat current worst in top-K
                                            let min_topk = if results.len() >= TOP_K {
                                                results.last().unwrap().pareto_score
                                            } else {
                                                0.0
                                            };
                                            if results.len() >= TOP_K && upper_pareto < min_topk {
                                                let inner_count = QTARGETS.len() as u64
                                                    * POWER_CONVERSIONS.len() as u64
                                                    * TBR_STRATEGIES.len() as u64
                                                    * GRIDS.len() as u64;
                                                skipped += inner_count;
                                                continue;
                                            }

                                            for qt in QTARGETS {
                                                let l4_qt_exact: u64 = if qt.q_value == SOPFR * PHI { 1 } else { 0 };
                                                let q_gain = q_gain_score(sch, qt, fuel);

                                                for pc in POWER_CONVERSIONS {
                                                    let l5_pc_exact: u64 = if pc.efficiency_pct == 100 / (N / PHI) { 1 } else { 0 };
                                                    let trl = trl_score(sch, pc);
                                                    let lcoe = lcoe_score(sch, fuel, sc, pc);

                                                    for tbr in TBR_STRATEGIES {
                                                        let l5_tbr_exact: u64 = if tbr.tbr_100x == SIGMA * 10 { 1 } else { 0 };

                                                        for gr in GRIDS {
                                                            evaluated += 1;

                                                            let l5_gr_exact: u64 = if gr.freq_hz == SIGMA * SOPFR || gr.freq_hz == SOPFR * (SIGMA - PHI) { 1 } else { 0 };

                                                            let n6_exact = partial_exact_1_4_no_qt
                                                                + l4_qt_exact + l5_pc_exact + l5_tbr_exact + l5_gr_exact;
                                                            let n6_pct = (n6_exact as f64) / (n6_total as f64) * 100.0;
                                                            let t_comm = l1_t_comm;

                                                            let pareto_score =
                                                                n6_pct * 0.35
                                                                + (q_gain as f64) * 0.25
                                                                + (trl as f64) * 0.20
                                                                + (lcoe as f64) * 0.12
                                                                + (t_comm as f64) * 0.08;

                                                            // Stats
                                                            sum_n6 += n6_pct;
                                                            if n6_pct > max_n6 { max_n6 = n6_pct; }
                                                            if n6_pct >= 80.0 { count_above80 += 1; }
                                                            if n6_pct >= 60.0 { count_above60 += 1; }

                                                            let entry = DseResult {
                                                                scheme: sch.name,
                                                                sc: sc.name,
                                                                blanket: bl.name,
                                                                structural: st.name,
                                                                heating: ht.name,
                                                                confinement: conf.name,
                                                                fuel: fuel.name,
                                                                tf: tf.name,
                                                                geometry: geo.name,
                                                                bfield: bf.name,
                                                                qtarget: qt.name,
                                                                power_conv: pc.name,
                                                                tbr_strat: tbr.name,
                                                                grid: gr.name,
                                                                n6_exact,
                                                                n6_total,
                                                                n6_pct,
                                                                q_gain,
                                                                trl,
                                                                lcoe,
                                                                t_comm,
                                                                pareto_score,
                                                            };

                                                            // Best-by-axis
                                                            if best_n6_result.as_ref().map_or(true, |b| n6_pct > b.n6_pct) {
                                                                best_n6_result = Some(entry.clone());
                                                            }
                                                            if best_q_result.as_ref().map_or(true, |b| q_gain > b.q_gain) {
                                                                best_q_result = Some(entry.clone());
                                                            }
                                                            if best_trl_result.as_ref().map_or(true, |b| trl > b.trl) {
                                                                best_trl_result = Some(entry.clone());
                                                            }
                                                            if best_lcoe_result.as_ref().map_or(true, |b| lcoe > b.lcoe) {
                                                                best_lcoe_result = Some(entry.clone());
                                                            }

                                                            // Per-scheme best
                                                            if scheme_best[si].as_ref().map_or(true, |b| pareto_score > b.pareto_score) {
                                                                scheme_best[si] = Some(entry.clone());
                                                            }

                                                            // Top-K insert
                                                            topk_insert(&mut results, entry, TOP_K);
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    let avg_n6 = sum_n6 / evaluated as f64;

    // === TOP 20 ===
    println!("{}", "=".repeat(170));
    println!("  TOP 20 CONFIGURATIONS (by Pareto score)");
    println!("{}", "=".repeat(170));
    println!();
    println!("  {:>3} | {:>11} | {:>5} | {:>10} | {:>7} | {:>4} | {:>10} | {:>5} | {:>4} | {:>3} | {:>4} | {:>5} | {:>7} | {:>10} | {:>5} | {:>5} | {:>4} | {:>4} | {:>4} | {:>6}",
        "#", "Scheme", "SC", "Blanket", "Struct", "Heat", "Confine", "Fuel", "TF", "Geo", "B", "Q", "PwrConv", "TBR", "Grid", "n6%", "Q_g", "TRL", "LCOE", "Pareto");
    println!("  {}", "-".repeat(165));

    for (i, r) in results.iter().take(20).enumerate() {
        println!("  {:>3} | {:>11} | {:>5} | {:>10} | {:>7} | {:>4} | {:>10} | {:>5} | {:>4} | {:>3} | {:>4} | {:>5} | {:>7} | {:>10} | {:>5} | {:>4.0}% | {:>4} | {:>4} | {:>4} | {:>6.2}",
            i + 1, r.scheme, r.sc, r.blanket, r.structural,
            r.heating, r.confinement, r.fuel, r.tf, r.geometry,
            r.bfield, r.qtarget, r.power_conv, r.tbr_strat, r.grid,
            r.n6_pct, r.q_gain, r.trl, r.lcoe, r.pareto_score);
    }

    // === STATISTICS ===
    println!();
    println!("{}", "=".repeat(90));
    println!("  STATISTICS");
    println!("{}", "=".repeat(90));

    println!("  Theoretical combinations: {}", total_combos);
    println!("  Skipped (incompatible):   {}", skipped);
    println!("  Evaluated:                {}", evaluated);
    println!("  Max n6 EXACT:             {:.1}%", max_n6);
    println!("  Avg n6 EXACT:             {:.1}%", avg_n6);
    println!("  >=80% EXACT:              {} ({:.2}%)", count_above80, count_above80 as f64 / evaluated as f64 * 100.0);
    println!("  >=60% EXACT:              {} ({:.2}%)", count_above60, count_above60 as f64 / evaluated as f64 * 100.0);

    // === BEST BY AXIS ===
    println!();
    println!("  BEST BY AXIS:");

    if let Some(ref r) = best_n6_result {
        println!("    Best n6 EXACT:  {} + {} + {} + {} + {} + {} + {} + {} + {} + {} + {} + {} + {} + {} ({:.1}%)",
            r.scheme, r.sc, r.blanket, r.structural,
            r.heating, r.confinement, r.fuel, r.tf,
            r.geometry, r.bfield, r.qtarget, r.power_conv,
            r.tbr_strat, r.grid, r.n6_pct);
    }
    if let Some(ref r) = best_q_result {
        println!("    Best Q_gain:    {} + {} + {} (Q_gain={})",
            r.scheme, r.fuel, r.qtarget, r.q_gain);
    }
    if let Some(ref r) = best_trl_result {
        println!("    Best TRL:       {} + {} (TRL={})",
            r.scheme, r.power_conv, r.trl);
    }
    if let Some(ref r) = best_lcoe_result {
        println!("    Best LCOE:      {} + {} + {} (LCOE={})",
            r.scheme, r.fuel, r.sc, r.lcoe);
    }

    let best_pareto = &results[0];
    println!("    Best Pareto:    {} + {} + {} + {} + {} + {} + {} (score={:.2})",
        best_pareto.scheme, best_pareto.sc, best_pareto.fuel,
        best_pareto.tf, best_pareto.geometry, best_pareto.bfield,
        best_pareto.power_conv, best_pareto.pareto_score);

    // === SCHEME BREAKDOWN ===
    println!();
    println!("{}", "=".repeat(90));
    println!("  SCHEME BREAKDOWN (best config per scheme)");
    println!("{}", "=".repeat(90));
    println!();

    for (si, sch) in SCHEMES.iter().enumerate() {
        if let Some(ref r) = scheme_best[si] {
            println!("  [{}] Pareto={:.2}  n6={:.0}%  Q={}  TRL={}  LCOE={}",
                pad(sch.name, 11), r.pareto_score, r.n6_pct, r.q_gain, r.trl, r.lcoe);
            println!("    SC={} Blanket={} Struct={} Heat={} Conf={} Fuel={}",
                r.sc, r.blanket, r.structural, r.heating, r.confinement, r.fuel);
            println!("    TF={} Geo={} B={} Q={} PwrConv={} TBR={} Grid={}",
                r.tf, r.geometry, r.bfield, r.qtarget, r.power_conv, r.tbr_strat, r.grid);
            println!();
        }
    }

    // === OPTIMAL PATH ASCII DIAGRAM ===
    println!("{}", "=".repeat(90));
    println!("  OPTIMAL PATH (Pareto #1)");
    println!("{}", "=".repeat(90));
    println!();

    let r = &results[0];
    println!("  Level 1: 방식                Level 2: 소재                    Level 3: 코어");
    println!("  ┌─────────────┐             ┌──────────────────┐             ┌──────────────────┐");
    println!("  | {}|             | SC:  {}|             | Heat: {}|",
        pad(r.scheme, 12), pad(r.sc, 13), pad(r.heating, 11));
    println!("  | PF={} CS={} |  ────>    | Bl:  {}|  ────>    | Conf: {}|",
        if r.scheme == "Tokamak" { "6" } else { "0" },
        if r.scheme == "Tokamak" { "6" } else { "0" },
        pad(r.blanket, 13), pad(r.confinement, 11));
    println!("  | TRL={}       |             | St:  {}|             | Fuel: {}|",
        SCHEMES.iter().find(|s| s.name == r.scheme).unwrap().trl,
        pad(r.structural, 13), pad(r.fuel, 11));
    println!("  └─────────────┘             └──────────────────┘             └──────────────────┘");
    println!("                                                                        |");
    println!("                                                                        v");
    println!("  Level 5: 시스템               Level 4: 장치");
    println!("  ┌──────────────────┐             ┌──────────────────┐");
    println!("  | Pwr:  {}|             | TF:  {}|",
        pad(r.power_conv, 11), pad(r.tf, 13));
    println!("  | TBR:  {}|  <────    | Geo: {}|",
        pad(r.tbr_strat, 11), pad(r.geometry, 13));
    println!("  | Grid: {}|             | B:   {}|",
        pad(r.grid, 11), pad(r.bfield, 13));
    println!("  └──────────────────┘             | Q:   {}|",
        pad(r.qtarget, 13));
    println!("                                   └──────────────────┘");
    println!();
    println!("  Pareto Score: {:.2}", r.pareto_score);
    println!("  n6 EXACT:     {}/{} = {:.1}%", r.n6_exact, r.n6_total, r.n6_pct);
    println!("  Q_gain:       {}", r.q_gain);
    println!("  TRL:          {}", r.trl);
    println!("  LCOE:         {}", r.lcoe);
    println!("  T_comm:       {}", r.t_comm);

    // === N6 MATCH DETAIL ===
    println!();
    println!("  N6 MATCH DETAIL (optimal path):");
    println!("    Level 1: PF_coils={} (n=6? {}), CS_modules={} (n=6? {})",
        if r.scheme == "Tokamak" { 6 } else { 0 },
        if r.scheme == "Tokamak" { "YES" } else { "no" },
        if r.scheme == "Tokamak" { 6 } else { 0 },
        if r.scheme == "Tokamak" { "YES" } else { "no" });

    let sc_obj = SUPERCONDUCTORS.iter().find(|s| s.name == r.sc).unwrap();
    println!("    Level 2: SC_Bmax={} (n6_derived? {}), Coolant_types (n6_base? yes)",
        sc_obj.b_max_t, if is_n6_derived(sc_obj.b_max_t) { "YES" } else { "no" });

    let ht_obj = HEATINGS.iter().find(|s| s.name == r.heating).unwrap();
    let fuel_obj = FUELS.iter().find(|s| s.name == r.fuel).unwrap();
    println!("    Level 3: Heat_MW={} (n6_base? {}), Fuel_mass={} (n6_base? {}), Neutron={}%",
        ht_obj.typical_mw, if is_n6_base(ht_obj.typical_mw) { "YES" } else { "no" },
        fuel_obj.d_mass, if is_n6_base(fuel_obj.d_mass) { "YES" } else { "no" },
        fuel_obj.neutron_fraction_pct);

    let tf_obj = TF_CONFIGS.iter().find(|s| s.name == r.tf).unwrap();
    let geo_obj = GEOMETRIES.iter().find(|s| s.name == r.geometry).unwrap();
    let bf_obj = BFIELDS.iter().find(|s| s.name == r.bfield).unwrap();
    let qt_obj = QTARGETS.iter().find(|s| s.name == r.qtarget).unwrap();
    println!("    Level 4: TF_coils={} (n6_base? {}), A*10={} (==30? {}), B_T={} (==sigma? {}), Q={} (==10? {})",
        tf_obj.coil_count, if is_n6_base(tf_obj.coil_count) { "YES" } else { "no" },
        geo_obj.aspect_ratio_10x, if geo_obj.aspect_ratio_10x == 30 { "YES" } else { "no" },
        bf_obj.bt_t, if bf_obj.bt_t == SIGMA { "YES" } else { "no" },
        qt_obj.q_value, if qt_obj.q_value == 10 { "YES" } else { "no" });

    let pc_obj = POWER_CONVERSIONS.iter().find(|s| s.name == r.power_conv).unwrap();
    let tbr_obj = TBR_STRATEGIES.iter().find(|s| s.name == r.tbr_strat).unwrap();
    let gr_obj = GRIDS.iter().find(|s| s.name == r.grid).unwrap();
    println!("    Level 5: Eff={}% (==33? {}), TBR_100x={} (==120? {}), Freq={} (n6? {})",
        pc_obj.efficiency_pct, if pc_obj.efficiency_pct == 33 { "YES" } else { "no" },
        tbr_obj.tbr_100x, if tbr_obj.tbr_100x == 120 { "YES" } else { "no" },
        gr_obj.freq_hz,
        if gr_obj.freq_hz == SIGMA * SOPFR || gr_obj.freq_hz == SOPFR * (SIGMA - PHI) { "YES" } else { "no" });

    println!();
    println!("{}", "=".repeat(90));
    println!("  DSE 완료 -- {} 조합 중 {} 평가, {} 비호환 제외", total_combos, evaluated, skipped);
    println!("  최적 경로: {} + {} + {} + {} + {} + {} + {}",
        r.scheme, r.sc, r.fuel, r.tf, r.geometry, r.bfield, r.power_conv);
    println!("  Pareto Score: {:.2} | n6 EXACT: {:.1}%", r.pareto_score, r.n6_pct);
    println!("{}", "=".repeat(90));
}
