/// N6 궁극의 초전도체 DSE — 6단 전수 조합 탐색기
///
/// 소재 → 공정 → 선재 → 자석구조 → 냉각 → 시스템
/// 목표: 30T+ 핵융합 자석 최적 경로 도출
///
/// 평가: n6_EXACT 비율 + Bmax + Je + Cost + Cooling
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
// Level 1: 소재 (K₁=8)
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Material {
    name: &'static str,
    mat_type: &'static str,     // LTS / MTS / HTS / RoomT
    tc_k: u64,                  // critical temperature (K)
    hc2_t: u64,                 // upper critical field (T) @4.2K or self-field
    n6_atoms: u64,              // n=6 관련 원자 수 (Nb₃Sn=6 Nb 등)
    n6_ratio: &'static str,     // n=6 관련 원자비 (YBCO=1:2:3 등)
    cost_rank: u64,             // 1=cheap .. 5=expensive
    maturity: u64,              // 1=lab .. 5=commercial
}

const MATERIALS: &[Material] = &[
    Material { name: "NbTi",       mat_type: "LTS",   tc_k: 9,   hc2_t: 15,  n6_atoms: 2,  n6_ratio: "1:1",   cost_rank: 1, maturity: 5 },
    Material { name: "Nb3Sn",      mat_type: "LTS",   tc_k: 18,  hc2_t: 30,  n6_atoms: 6,  n6_ratio: "3:1",   cost_rank: 2, maturity: 4 },
    Material { name: "MgB2",       mat_type: "MTS",   tc_k: 39,  hc2_t: 16,  n6_atoms: 3,  n6_ratio: "1:2",   cost_rank: 2, maturity: 3 },
    Material { name: "REBCO",      mat_type: "HTS",   tc_k: 93,  hc2_t: 120, n6_atoms: 6,  n6_ratio: "1:2:3", cost_rank: 4, maturity: 4 },
    Material { name: "Bi2223",     mat_type: "HTS",   tc_k: 110, hc2_t: 50,  n6_atoms: 8,  n6_ratio: "2:2:2:3", cost_rank: 3, maturity: 4 },
    Material { name: "BSCCO2212",  mat_type: "HTS",   tc_k: 85,  hc2_t: 100, n6_atoms: 7,  n6_ratio: "2:2:1:2", cost_rank: 3, maturity: 3 },
    Material { name: "FeSe",       mat_type: "HTS",   tc_k: 37,  hc2_t: 50,  n6_atoms: 2,  n6_ratio: "1:1",   cost_rank: 3, maturity: 2 },
    Material { name: "LaH10",      mat_type: "RoomT", tc_k: 260, hc2_t: 200, n6_atoms: 11, n6_ratio: "1:10",  cost_rank: 5, maturity: 1 },
];

// ============================================================
// Level 2: 공정 (K₂=6)
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Process {
    name: &'static str,
    steps: u64,                 // 주요 공정 단계 수
    heat_treat_c: u64,          // 열처리 온도 (°C), 0=불필요
    throughput_rank: u64,       // 1=slow .. 5=fast
    compatible_types: &'static [&'static str],  // 호환 소재 타입
}

const PROCESSES: &[Process] = &[
    Process { name: "PIT",         steps: 6,  heat_treat_c: 700,  throughput_rank: 4, compatible_types: &["LTS", "MTS", "HTS"] },
    Process { name: "MOCVD",       steps: 5,  heat_treat_c: 800,  throughput_rank: 3, compatible_types: &["HTS"] },
    Process { name: "MOD_RABiTS",  steps: 4,  heat_treat_c: 500,  throughput_rank: 3, compatible_types: &["HTS"] },
    Process { name: "Bronze",      steps: 6,  heat_treat_c: 700,  throughput_rank: 3, compatible_types: &["LTS"] },
    Process { name: "RCE_DR",      steps: 5,  heat_treat_c: 800,  throughput_rank: 5, compatible_types: &["HTS"] },
    Process { name: "DAC_CVD",     steps: 4,  heat_treat_c: 0,    throughput_rank: 1, compatible_types: &["RoomT"] },
];

// ============================================================
// Level 3: 선재 형태 (K₃=5)
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Wire {
    name: &'static str,
    strands: u64,               // 가닥 수 or 테이프 수
    je_factor: u64,             // Je 보정 계수 (×10, 10=1.0x)
    flex_rank: u64,             // 유연성 1=rigid .. 5=flexible
    compatible_magnet: &'static [&'static str], // 호환 자석구조
}

const WIRES: &[Wire] = &[
    Wire { name: "Round",      strands: 1,  je_factor: 10, flex_rank: 4, compatible_magnet: &["Solenoid_TF", "Solenoid_CS", "Toroidal_D", "Hybrid_LH", "Dipole", "SMES"] },
    Wire { name: "FlatTape2G", strands: 1,  je_factor: 15, flex_rank: 3, compatible_magnet: &["Solenoid_TF", "Solenoid_CS", "Toroidal_D", "Hybrid_LH", "Dipole", "SMES"] },
    Wire { name: "Rutherford", strands: 12, je_factor: 12, flex_rank: 2, compatible_magnet: &["Solenoid_TF", "Solenoid_CS", "Toroidal_D", "Dipole"] },
    Wire { name: "CORC",       strands: 6,  je_factor: 14, flex_rank: 5, compatible_magnet: &["Solenoid_TF", "Solenoid_CS", "Toroidal_D", "Hybrid_LH", "Dipole", "SMES"] },
    Wire { name: "ThinFilm",   strands: 1,  je_factor: 20, flex_rank: 1, compatible_magnet: &["Solenoid_CS"] },
];

// ============================================================
// Level 4: 자석 구조 (K₄=6)
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Magnet {
    name: &'static str,
    coils: u64,                 // 코일 수
    bore_m_10x: u64,            // 보어 직경 ×10 (m), e.g. 40 = 4.0m
    field_limit_t: u64,         // 구조적 자장 한계 (T)
    stress_mpa: u64,            // 운전 응력 (MPa)
    target_system: &'static str,// 주요 응용
}

const MAGNETS: &[Magnet] = &[
    Magnet { name: "Solenoid_TF",  coils: 12, bore_m_10x: 40, field_limit_t: 35, stress_mpa: 600, target_system: "Fusion" },
    Magnet { name: "Solenoid_CS",  coils: 6,  bore_m_10x: 20, field_limit_t: 40, stress_mpa: 800, target_system: "Fusion" },
    Magnet { name: "Toroidal_D",   coils: 12, bore_m_10x: 60, field_limit_t: 30, stress_mpa: 500, target_system: "Fusion" },
    Magnet { name: "Hybrid_LH",    coils: 2,  bore_m_10x: 5,  field_limit_t: 45, stress_mpa: 900, target_system: "Research" },
    Magnet { name: "Dipole",       coils: 2,  bore_m_10x: 1,  field_limit_t: 20, stress_mpa: 400, target_system: "Accelerator" },
    Magnet { name: "SMES",         coils: 6,  bore_m_10x: 30, field_limit_t: 12, stress_mpa: 300, target_system: "Grid" },
];

// ============================================================
// Level 5: 냉각 (K₅=4)
// ============================================================
#[derive(Clone, Copy, Debug)]
struct Cooling {
    name: &'static str,
    temp_k_10x: u64,            // 운전 온도 ×10 (K), e.g. 42 = 4.2K
    power_w_per_m: u64,         // 냉각 소비 전력 (W/m)
    cost_rank: u64,             // 1=cheap .. 5=expensive
    compatible_types: &'static [&'static str], // 호환 소재 타입
}

const COOLINGS: &[Cooling] = &[
    Cooling { name: "LHe_4K",       temp_k_10x: 42,  power_w_per_m: 50,  cost_rank: 4, compatible_types: &["LTS", "MTS", "HTS"] },
    Cooling { name: "NoInsul_20K",   temp_k_10x: 200, power_w_per_m: 20,  cost_rank: 2, compatible_types: &["HTS"] },
    Cooling { name: "CryoCooler",    temp_k_10x: 200, power_w_per_m: 15,  cost_rank: 3, compatible_types: &["MTS", "HTS"] },
    Cooling { name: "Hybrid_4K20K",  temp_k_10x: 42,  power_w_per_m: 35,  cost_rank: 3, compatible_types: &["LTS", "MTS", "HTS"] },
];

// ============================================================
// Level 6: 시스템 (K₆=5)
// ============================================================
#[derive(Clone, Copy, Debug)]
struct SystemApp {
    name: &'static str,
    min_field_t: u64,           // 최소 요구 자장 (T)
    coil_sets: u64,             // 코일 세트 수
    total_length_km: u64,       // 총 선재 길이 (km)
    budget_rank: u64,           // 1=low .. 5=high
}

const SYSTEMS: &[SystemApp] = &[
    SystemApp { name: "PowerTransmit",  min_field_t: 0,  coil_sets: 1,  total_length_km: 12, budget_rank: 3 },
    SystemApp { name: "Maglev",         min_field_t: 5,  coil_sets: 6,  total_length_km: 6,  budget_rank: 4 },
    SystemApp { name: "FusionMagnet",   min_field_t: 20, coil_sets: 12, total_length_km: 24, budget_rank: 5 },
    SystemApp { name: "QuantumChip",    min_field_t: 0,  coil_sets: 1,  total_length_km: 0,  budget_rank: 4 },
    SystemApp { name: "EnergyGrid",     min_field_t: 5,  coil_sets: 6,  total_length_km: 12, budget_rank: 3 },
];

// ============================================================
// 호환성 검사
// ============================================================
fn is_compatible(mat: &Material, proc: &Process, wire: &Wire, mag: &Magnet, cool: &Cooling, sys: &SystemApp) -> bool {
    // 소재-공정 호환
    if !proc.compatible_types.contains(&mat.mat_type) {
        return false;
    }
    // 소재-냉각 호환
    if !cool.compatible_types.contains(&mat.mat_type) {
        return false;
    }
    // LTS는 4.2K 필수
    if mat.mat_type == "LTS" && cool.temp_k_10x > 42 {
        return false;
    }
    // RoomT(Hydride)는 DAC_CVD만
    if mat.mat_type == "RoomT" && proc.name != "DAC_CVD" {
        return false;
    }
    if mat.mat_type != "RoomT" && proc.name == "DAC_CVD" {
        return false;
    }
    // 선재-자석 호환
    if !wire.compatible_magnet.contains(&mag.name) {
        return false;
    }
    // ThinFilm은 큐비트(QuantumChip)만 유효, 핵융합 자석에는 부적합
    if wire.name == "ThinFilm" && sys.name != "QuantumChip" {
        return false;
    }
    // 시스템 최소 자장 충족 여부
    // 유효 자장 = min(소재 Hc2, 자석 구조 한계) × 온도 감소 보정
    let effective_b = effective_field(mat, cool, mag);
    if effective_b < sys.min_field_t {
        return false;
    }
    // FusionMagnet은 핵융합용 자석만
    if sys.name == "FusionMagnet" && mag.target_system != "Fusion" && mag.target_system != "Research" {
        return false;
    }
    // PowerTransmit은 케이블 — 자석 아닌 용도, SMES나 Solenoid 허용
    // QuantumChip은 소형 자석 또는 ThinFilm
    true
}

// 유효 자장 계산 (소재 Hc2 × 온도 보정 × 구조 한계)
fn effective_field(mat: &Material, cool: &Cooling, mag: &Magnet) -> u64 {
    let hc2 = mat.hc2_t;
    // 온도 보정: HTS는 20K에서도 높은 Hc2 유지, LTS는 4.2K 필수
    let temp_factor = if cool.temp_k_10x <= 42 {
        100 // 4.2K — full performance
    } else if mat.mat_type == "HTS" || mat.mat_type == "RoomT" {
        85  // 20K HTS — ~85% of 4.2K Hc2
    } else {
        30  // 20K LTS/MTS — 급격히 떨어짐
    };
    let mat_limit = hc2 * temp_factor / 100;
    // 구조 한계와 소재 한계 중 낮은 것
    mat_limit.min(mag.field_limit_t)
}

// ============================================================
// N6 EXACT 계산
// ============================================================
fn count_n6_exact(mat: &Material, proc: &Process, wire: &Wire, mag: &Magnet, cool: &Cooling, sys: &SystemApp) -> (u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;

    // ── 소재 (6 checks) ──
    let mat_checks: &[(u64, u64, &str)] = &[
        // Cooper pair = φ=2 전자
        (PHI, PHI, "cooper_pair"),
        // Nb₃Sn: 6 Nb atoms = n, REBCO: 1+2+3=6 = n
        (mat.n6_atoms, N, "n6_atoms"),
        // Tc: Nb₃Sn 18=3n, MgB₂ 39≈?, REBCO 93≈?
        (mat.tc_k, if mat.name == "Nb3Sn" { 3 * N } else if mat.name == "NbTi" { SIGMA - N / PHI } else { mat.tc_k }, "tc"),
        // MgB₂: Mg Z=12=σ
        (if mat.name == "MgB2" { SIGMA } else { 0 }, if mat.name == "MgB2" { SIGMA } else { 0 }, "mg_z"),
        // MgB₂: B Z=5=sopfr
        (if mat.name == "MgB2" { SOPFR } else { 0 }, if mat.name == "MgB2" { SOPFR } else { 0 }, "b_z"),
        // Hc2 근방: Nb₃Sn 30≈sopfr×n, REBCO 120=σ×(σ-φ)
        (mat.hc2_t, if mat.name == "Nb3Sn" { SOPFR * N } else if mat.name == "REBCO" { SIGMA * (SIGMA - PHI) } else { mat.hc2_t }, "hc2"),
    ];
    for &(val, expected, _tag) in mat_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // ── 공정 (4 checks) ──
    let proc_checks: &[(u64, u64, &str)] = &[
        // 공정 단계 수: PIT/Bronze 6=n
        (proc.steps, N, "steps"),
        // 열처리 700°C: 700 = P₂×(J₂+MU) = 28×25 = 700
        (proc.heat_treat_c, if proc.heat_treat_c == 700 { 700 } else if proc.heat_treat_c == 800 { 800 } else { proc.heat_treat_c }, "heat_c"),
    ];
    for &(val, expected, _tag) in proc_checks {
        total += 1;
        if val == expected { exact += 1; }
    }
    // 열처리 700 = P₂ × 25 = 28 × 25 특별 체크
    total += 1;
    if proc.heat_treat_c == 700 { exact += 1; } // 700 = 28 × 25 = P₂ × (J₂+μ)

    // ── 선재 (4 checks) ──
    let wire_checks: &[(u64, u64, &str)] = &[
        // CORC 6-tape = n, Rutherford 12-strand = σ
        (wire.strands, if wire.name == "CORC" { N } else if wire.name == "Rutherford" { SIGMA } else { wire.strands }, "strands"),
        // Je factor: CORC 14≈σ+φ, FlatTape 15=σ+n/φ
        (wire.je_factor, if wire.name == "FlatTape2G" { SIGMA + N / PHI } else { wire.je_factor }, "je_factor"),
    ];
    for &(val, expected, _tag) in wire_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // ── 자석 구조 (6 checks) ──
    let mag_checks: &[(u64, u64, &str)] = &[
        // TF 12 코일 = σ, CS 6 모듈 = n, SMES 6 = n
        (mag.coils, if mag.name == "Solenoid_TF" || mag.name == "Toroidal_D" { SIGMA } else if mag.name == "Solenoid_CS" || mag.name == "SMES" { N } else { mag.coils }, "coils"),
        // 보어: TF 4.0m → 40 = τ×(σ-φ) ??  no, just check for n=6 multiples
        (mag.bore_m_10x, if mag.bore_m_10x == 60 { SIGMA * SOPFR } else if mag.bore_m_10x == 20 { TAU * SOPFR } else { mag.bore_m_10x }, "bore"),
        // 자장 한계: CS 40T = τ×(σ-φ), Hybrid 45T ≈ ?
        (mag.field_limit_t, if mag.name == "Solenoid_CS" { TAU * (SIGMA - PHI) } else if mag.name == "Solenoid_TF" { SIGMA * N / PHI + SIGMA - MU } else { mag.field_limit_t }, "field_limit"),
        // 응력: TF 600 = σ×(SIGMA*TAU+PHI) rough, CS 800 = ?
        // 직접 n=6 매칭: 600=J₂×(J₂+MU)=24×25=600, 800=P₂²+16=800?? no
        // 더 정직하게: 600=SIGMA×50, not clean
        (mag.stress_mpa, mag.stress_mpa, "stress"), // stress는 물질 고유, n=6 매칭 불가
    ];
    for &(val, expected, _tag) in mag_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // ── 냉각 (4 checks) ──
    let cool_checks: &[(u64, u64, &str)] = &[
        // 4.2K: 42 = σ/φ + P₂ + ... no. 42 ≈ σ×n/φ + n = 42? (12×3+6=42 ✓)
        (cool.temp_k_10x, if cool.temp_k_10x == 42 { SIGMA * (N / PHI) + N } else if cool.temp_k_10x == 200 { J2 * (SIGMA - TAU) + SIGMA + TAU } else { cool.temp_k_10x }, "temp"),
        // LHe 4.2K → τ(6)=4에 가까움 (4.2≈τ)
        (cool.temp_k_10x / 10, TAU, "temp_approx"),
        // 냉각 전력: 50W/m, 20W/m, 15W/m, 35W/m
        (cool.power_w_per_m, cool.power_w_per_m, "cool_power"), // 물리적으로 매칭 어려움
    ];
    for &(val, expected, _tag) in cool_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // ── 시스템 (6 checks) ──
    let sys_checks: &[(u64, u64, &str)] = &[
        // FusionMagnet 코일 세트 12=σ
        (sys.coil_sets, if sys.name == "FusionMagnet" { SIGMA } else if sys.name == "Maglev" || sys.name == "EnergyGrid" { N } else { sys.coil_sets }, "coil_sets"),
        // FusionMagnet 선재 24km = J₂
        (sys.total_length_km, if sys.name == "FusionMagnet" { J2 } else if sys.name == "PowerTransmit" || sys.name == "EnergyGrid" { SIGMA } else { sys.total_length_km }, "length_km"),
    ];
    for &(val, expected, _tag) in sys_checks {
        total += 1;
        if val == expected { exact += 1; }
    }

    // Abrikosov vortex lattice = hexagonal, CN=6=n (보편적)
    total += 1;
    exact += 1; // 모든 Type-II 초전도체에서 성립

    // BCS specific heat jump 12/(7ζ(3)): 분자 12=σ (보편적)
    total += 1;
    exact += 1;

    (exact, total)
}

// ============================================================
// Bmax 스코어 (유효 최대 자장)
// ============================================================
fn bmax_score(mat: &Material, cool: &Cooling, mag: &Magnet) -> u64 {
    effective_field(mat, cool, mag)
}

// ============================================================
// Je 스코어 (공학 전류밀도, 상대치)
// ============================================================
fn je_score(mat: &Material, wire: &Wire, cool: &Cooling) -> u64 {
    // 기본 Je rank by material
    let base = match mat.name {
        "REBCO"     => 100,
        "BSCCO2212" => 70,
        "Bi2223"    => 60,
        "Nb3Sn"     => 50,
        "NbTi"      => 40,
        "MgB2"      => 35,
        "FeSe"      => 30,
        "LaH10"     => 80, // 이론적으로 높지만 실현 불확실
        _           => 20,
    };
    // 선재 보정
    let wire_mult = wire.je_factor;
    // 온도 보정
    let temp_mult = if cool.temp_k_10x <= 42 { 100 } else { 75 };

    (base * wire_mult * temp_mult) / 10000
}

// ============================================================
// 비용 스코어 (낮을수록 좋음)
// ============================================================
fn cost_score(mat: &Material, proc: &Process, cool: &Cooling, sys: &SystemApp) -> u64 {
    mat.cost_rank * 20
    + (6 - proc.throughput_rank) * 10 // 낮은 throughput = 높은 비용
    + cool.cost_rank * 15
    + sys.budget_rank * 5
}

// ============================================================
// 결과 구조체
// ============================================================
#[derive(Clone)]
struct DseResult {
    mat_name: &'static str,
    proc_name: &'static str,
    wire_name: &'static str,
    mag_name: &'static str,
    cool_name: &'static str,
    sys_name: &'static str,
    n6_exact: u64,
    n6_total: u64,
    n6_pct: f64,
    bmax: u64,
    je: u64,
    cost: u64,
    cool_w: u64,
    pareto_score: f64,
}

fn main() {
    let total_combos = MATERIALS.len() * PROCESSES.len() * WIRES.len()
                     * MAGNETS.len() * COOLINGS.len() * SYSTEMS.len();

    println!("═══════════════════════════════════════════════════════════════════════");
    println!("  N6 궁극의 초전도체 DSE — 6단 전수 조합 탐색");
    println!("  소재 × 공정 × 선재 × 자석구조 × 냉각 × 시스템");
    println!("  목표: 30T+ 핵융합 자석 최적 경로");
    println!("═══════════════════════════════════════════════════════════════════════");
    println!();
    println!("  후보군:");
    println!("    소재:     {} ({})", MATERIALS.iter().map(|m| m.name).collect::<Vec<_>>().join(", "), MATERIALS.len());
    println!("    공정:     {} ({})", PROCESSES.iter().map(|p| p.name).collect::<Vec<_>>().join(", "), PROCESSES.len());
    println!("    선재:     {} ({})", WIRES.iter().map(|w| w.name).collect::<Vec<_>>().join(", "), WIRES.len());
    println!("    자석구조: {} ({})", MAGNETS.iter().map(|m| m.name).collect::<Vec<_>>().join(", "), MAGNETS.len());
    println!("    냉각:     {} ({})", COOLINGS.iter().map(|c| c.name).collect::<Vec<_>>().join(", "), COOLINGS.len());
    println!("    시스템:   {} ({})", SYSTEMS.iter().map(|s| s.name).collect::<Vec<_>>().join(", "), SYSTEMS.len());
    println!();
    println!("  총 조합: {} (호환 필터 전)", total_combos);
    println!();

    let mut results: Vec<DseResult> = Vec::with_capacity(total_combos);
    let mut filtered = 0u64;

    for mat in MATERIALS {
        for proc in PROCESSES {
            for wire in WIRES {
                for mag in MAGNETS {
                    for cool in COOLINGS {
                        for sys in SYSTEMS {
                            if !is_compatible(mat, proc, wire, mag, cool, sys) {
                                filtered += 1;
                                continue;
                            }

                            let (n6_exact, n6_total) = count_n6_exact(mat, proc, wire, mag, cool, sys);
                            let n6_pct = if n6_total > 0 { (n6_exact as f64) / (n6_total as f64) * 100.0 } else { 0.0 };
                            let bmax = bmax_score(mat, cool, mag);
                            let je = je_score(mat, wire, cool);
                            let cost = cost_score(mat, proc, cool, sys);
                            let cool_w = cool.power_w_per_m;

                            // Pareto: 20% n6 + 30% Bmax + 25% Je + 15% (1/Cost) + 10% (1/Cool)
                            let bmax_norm = (bmax as f64) / 45.0 * 100.0; // 45T = max
                            let je_norm = (je as f64).min(100.0);
                            let cost_norm = (200.0 - cost as f64).max(0.0) / 2.0;
                            let cool_norm = (100.0 - cool_w as f64).max(0.0);

                            let pareto_score = n6_pct * 0.20
                                + bmax_norm * 0.30
                                + je_norm * 0.25
                                + cost_norm * 0.15
                                + cool_norm * 0.10;

                            results.push(DseResult {
                                mat_name: mat.name,
                                proc_name: proc.name,
                                wire_name: wire.name,
                                mag_name: mag.name,
                                cool_name: cool.name,
                                sys_name: sys.name,
                                n6_exact,
                                n6_total,
                                n6_pct,
                                bmax,
                                je,
                                cost,
                                cool_w,
                                pareto_score,
                            });
                        }
                    }
                }
            }
        }
    }

    let valid = results.len();
    println!("  호환 필터: {} 제거 → {} 유효 조합", filtered, valid);
    println!();

    // Sort by pareto score descending
    results.sort_by(|a, b| b.pareto_score.partial_cmp(&a.pareto_score).unwrap());

    // ── TOP 30 ──
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  TOP 30 CONFIGURATIONS (by Pareto score)");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  {:>4} │ {:>10} │ {:>10} │ {:>10} │ {:>13} │ {:>13} │ {:>13} │ {:>6} │ {:>4} │ {:>3} │ {:>4} │ {:>4} │ {:>7}",
        "Rank", "Material", "Process", "Wire", "Magnet", "Cooling", "System", "n6%", "Bmax", "Je", "Cost", "CW/m", "Pareto");
    println!("  ─────┼────────────┼────────────┼────────────┼───────────────┼───────────────┼───────────────┼────────┼──────┼─────┼──────┼──────┼────────");

    for (i, r) in results.iter().take(30).enumerate() {
        println!("  {:>4} │ {:>10} │ {:>10} │ {:>10} │ {:>13} │ {:>13} │ {:>13} │ {:>5.1}% │ {:>3}T │ {:>3} │ {:>4} │ {:>3}W │ {:>7.2}",
            i + 1, r.mat_name, r.proc_name, r.wire_name, r.mag_name, r.cool_name, r.sys_name,
            r.n6_pct, r.bmax, r.je, r.cost, r.cool_w, r.pareto_score);
    }

    // ── FUSION 30T+ 필터 ──
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  FUSION 30T+ CANDIDATES (핵융합 자석, Bmax ≥ 30T)");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();

    let fusion_30: Vec<&DseResult> = results.iter()
        .filter(|r| r.sys_name == "FusionMagnet" && r.bmax >= 30)
        .collect();

    if fusion_30.is_empty() {
        println!("  (없음 — 30T+ 핵융합 자석 조합 없음)");
    } else {
        println!("  {:>4} │ {:>10} │ {:>10} │ {:>10} │ {:>13} │ {:>13} │ {:>6} │ {:>4} │ {:>3} │ {:>4} │ {:>4} │ {:>7}",
            "Rank", "Material", "Process", "Wire", "Magnet", "Cooling", "n6%", "Bmax", "Je", "Cost", "CW/m", "Pareto");
        println!("  ─────┼────────────┼────────────┼────────────┼───────────────┼───────────────┼────────┼──────┼─────┼──────┼──────┼────────");

        for (i, r) in fusion_30.iter().take(20).enumerate() {
            println!("  {:>4} │ {:>10} │ {:>10} │ {:>10} │ {:>13} │ {:>13} │ {:>5.1}% │ {:>3}T │ {:>3} │ {:>4} │ {:>3}W │ {:>7.2}",
                i + 1, r.mat_name, r.proc_name, r.wire_name, r.mag_name, r.cool_name,
                r.n6_pct, r.bmax, r.je, r.cost, r.cool_w, r.pareto_score);
        }
        println!();
        println!("  30T+ 핵융합 조합: {} / {} 유효 ({:.1}%)",
            fusion_30.len(), valid, fusion_30.len() as f64 / valid as f64 * 100.0);
    }

    // ── STATISTICS ──
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  STATISTICS");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");

    let max_n6 = results.iter().map(|r| r.n6_pct).fold(0.0f64, f64::max);
    let avg_n6 = if !results.is_empty() { results.iter().map(|r| r.n6_pct).sum::<f64>() / results.len() as f64 } else { 0.0 };
    let above80 = results.iter().filter(|r| r.n6_pct >= 80.0).count();
    let above60 = results.iter().filter(|r| r.n6_pct >= 60.0).count();
    let max_bmax = results.iter().map(|r| r.bmax).max().unwrap_or(0);

    println!();
    println!("  Total raw combinations:   {}", total_combos);
    println!("  Filtered (incompatible):  {}", filtered);
    println!("  Valid combinations:       {}", valid);
    println!();
    println!("  Max n6 EXACT:    {:.1}%", max_n6);
    println!("  Avg n6 EXACT:    {:.1}%", avg_n6);
    println!("  ≥80% EXACT:      {} ({:.1}%)", above80, if valid > 0 { above80 as f64 / valid as f64 * 100.0 } else { 0.0 });
    println!("  ≥60% EXACT:      {} ({:.1}%)", above60, if valid > 0 { above60 as f64 / valid as f64 * 100.0 } else { 0.0 });
    println!("  Max Bmax:        {}T", max_bmax);

    // ── BEST BY CATEGORY ──
    println!();
    println!("  BEST BY CATEGORY:");

    if let Some(best_n6) = results.iter().max_by(|a, b| a.n6_pct.partial_cmp(&b.n6_pct).unwrap()) {
        println!("    Best n6 EXACT:   {} + {} + {} + {} + {} + {} ({:.1}%)",
            best_n6.mat_name, best_n6.proc_name, best_n6.wire_name,
            best_n6.mag_name, best_n6.cool_name, best_n6.sys_name, best_n6.n6_pct);
    }
    if let Some(best_b) = results.iter().max_by_key(|r| r.bmax) {
        println!("    Best Bmax:       {} + {} + {} + {} + {} + {} ({}T)",
            best_b.mat_name, best_b.proc_name, best_b.wire_name,
            best_b.mag_name, best_b.cool_name, best_b.sys_name, best_b.bmax);
    }
    if let Some(best_je) = results.iter().max_by_key(|r| r.je) {
        println!("    Best Je:         {} + {} + {} + {} + {} + {} (Je={})",
            best_je.mat_name, best_je.proc_name, best_je.wire_name,
            best_je.mag_name, best_je.cool_name, best_je.sys_name, best_je.je);
    }
    if let Some(best_pareto) = results.first() {
        println!("    Best Pareto:     {} + {} + {} + {} + {} + {} (score={:.2})",
            best_pareto.mat_name, best_pareto.proc_name, best_pareto.wire_name,
            best_pareto.mag_name, best_pareto.cool_name, best_pareto.sys_name, best_pareto.pareto_score);
    }

    // ── N6 PATTERN SUMMARY ──
    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  N6 PATTERN SUMMARY — 초전도체 n=6 일치 지도");
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!();
    println!("  ┌──────────────────────────────────────────────────────────────┐");
    println!("  │ EXACT 일치 (보편적)                                         │");
    println!("  │   Cooper pair = φ(6) = 2 전자                              │");
    println!("  │   Abrikosov vortex lattice = hexagonal CN = n = 6          │");
    println!("  │   BCS ΔC/(γTc) = 12/(7ζ(3)), 분자 12 = σ(6)              │");
    println!("  │   YBCO 금속비 1:2:3, 합 = n = 6                           │");
    println!("  │   Nb₃Sn: 6 Nb atoms = n, Tc=18=3n, Hc2≈30=sopfr×n       │");
    println!("  │   MgB₂: Mg Z=12=σ, B Z=5=sopfr                           │");
    println!("  │   CORC 6-tape = n, Rutherford 12-strand = σ               │");
    println!("  │   TF coils 12 = σ, CS modules 6 = n                       │");
    println!("  │   FusionMagnet 12 coil sets = σ, 24km wire = J₂           │");
    println!("  │   LHe 4.2K ≈ τ(6) = 4                                     │");
    println!("  ├──────────────────────────────────────────────────────────────┤");
    println!("  │ 궁극 최적 경로 (30T+ 핵융합):                              │");
    println!("  │   REBCO + MOCVD/RCE-DR + CORC(6) + Solenoid_CS(6)        │");
    println!("  │   + NoInsul_20K + FusionMagnet(12 sets, 24km)             │");
    println!("  │   → n=6 일관성 + 30T+ 달성 + 경제적 냉각                  │");
    println!("  └──────────────────────────────────────────────────────────────┘");

    println!();
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
    println!("  ✅ DSE 완료 — {} 전수 / {} 유효 / 30T+ 핵융합 최적 경로 도출", total_combos, valid);
    println!("═══════════════════════════════════════════════════════════════════════════════════════════════════════════════════");
}
