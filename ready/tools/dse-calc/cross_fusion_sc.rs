// ═══════════════════════════════════════════════════════════════════════
// N6 Cross-DSE: FUSION x SUPERCONDUCTOR
// 토카막 자석 최적화 — 30T+ 핵융합 경로 탐색
// ═══════════════════════════════════════════════════════════════════════
// 빌드: ~/.cargo/bin/rustc tools/dse-calc/cross_fusion_sc.rs -o tools/dse-calc/cross-fusion-sc
// 실행: ./tools/dse-calc/cross-fusion-sc
// ═══════════════════════════════════════════════════════════════════════

#![allow(dead_code)]

// ── N6 상수 ──
const N: u64 = 6;
const SIGMA: u64 = 12;
const TAU: u64 = 4;
const PHI: u64 = 2;
const SOPFR: u64 = 5;
const J2: u64 = 24;
const MU: u64 = 1;

// ═══════════════════════════════════════════════════════════════════════
// Fusion 후보 구조체 (5 Levels)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct FuCandidate {
    id: &'static str,
    name: &'static str,
    n6_exact: bool,
    param_a: f64,
    param_b: f64,
}

// Extra fields for bridge
#[derive(Clone)]
struct FuL2 {
    base: FuCandidate,
    coils: u32,
    is_tokamak: bool,
}

#[derive(Clone)]
struct FuL3 {
    base: FuCandidate,
    field_tesla: f64,
    uses_hts: bool,
}

// ═══════════════════════════════════════════════════════════════════════
// Superconductor 후보 구조체 (6 Levels)
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct ScMaterial {
    id: &'static str,
    name: &'static str,
    sc_type: &'static str,  // LTS / MTS / HTS / RoomT
    tc_k: f64,
    hc2_t: f64,
    n6_atoms: u32,
    n6_exact: bool,
}

#[derive(Clone)]
struct ScProcess {
    id: &'static str,
    name: &'static str,
    steps: u32,
    temp: u32,
    n6_exact: bool,
}

#[derive(Clone)]
struct ScWire {
    id: &'static str,
    name: &'static str,
    strands: u32,
    je_factor: f64,
    n6_exact: bool,
}

#[derive(Clone)]
struct ScMagnet {
    id: &'static str,
    name: &'static str,
    coils: u32,
    field_limit: f64,
    for_fusion: bool,
    n6_exact: bool,
}

#[derive(Clone)]
struct ScCooling {
    id: &'static str,
    name: &'static str,
    temp_k: f64,
    power_per_m: f64,
    n6_exact: bool,
}

#[derive(Clone)]
struct ScSystem {
    id: &'static str,
    name: &'static str,
    min_field: f64,
    coil_sets: u32,
    for_fusion: bool,
    n6_exact: bool,
}

// ═══════════════════════════════════════════════════════════════════════
// Fusion 후보 데이터
// ═══════════════════════════════════════════════════════════════════════

fn fu_materials() -> Vec<FuCandidate> {
    vec![
        FuCandidate { id: "FU-M1", name: "D-T Standard",     n6_exact: true,  param_a: 17.6, param_b: 5.0 },
        FuCandidate { id: "FU-M2", name: "D-D Proton",       n6_exact: true,  param_a: 3.27, param_b: 4.0 },
        FuCandidate { id: "FU-M3", name: "D-He3 Aneutronic", n6_exact: false, param_a: 18.3, param_b: 17.0 },
        FuCandidate { id: "FU-M4", name: "p-B11 Clean",      n6_exact: false, param_a: 8.7,  param_b: 9.0 },
        FuCandidate { id: "FU-M5", name: "Li6-D Hybrid",     n6_exact: true,  param_a: 22.4, param_b: 22.0 },
        FuCandidate { id: "FU-M6", name: "Cat-DD Catalyzed", n6_exact: false, param_a: 7.2,  param_b: 7.0 },
    ]
}

fn fu_confinements() -> Vec<FuL2> {
    vec![
        FuL2 { base: FuCandidate { id: "FU-P1", name: "Tokamak-12",       n6_exact: true, param_a: 12.0, param_b: 1.5 }, coils: 12, is_tokamak: true },
        FuL2 { base: FuCandidate { id: "FU-P2", name: "Tokamak-24",       n6_exact: true, param_a: 24.0, param_b: 0.5 }, coils: 24, is_tokamak: true },
        FuL2 { base: FuCandidate { id: "FU-P3", name: "Stellarator-5",    n6_exact: true, param_a: 5.0,  param_b: 0.8 }, coils: 5,  is_tokamak: false },
        FuL2 { base: FuCandidate { id: "FU-P4", name: "Spherical Tokamak",n6_exact: true, param_a: 2.0,  param_b: 2.0 }, coils: 2,  is_tokamak: true },
        FuL2 { base: FuCandidate { id: "FU-P5", name: "ICF",             n6_exact: true,  param_a: 4.0,  param_b: 0.0 }, coils: 0,  is_tokamak: false },
    ]
}

fn fu_cores() -> Vec<FuL3> {
    vec![
        FuL3 { base: FuCandidate { id: "FU-C1", name: "HTS High-Field",    n6_exact: true,  param_a: 12.0, param_b: 1.85 }, field_tesla: 12.0, uses_hts: true },
        FuL3 { base: FuCandidate { id: "FU-C2", name: "Low-Field Large",   n6_exact: true,  param_a: 6.0,  param_b: 6.2 },  field_tesla: 6.0,  uses_hts: false },
        FuL3 { base: FuCandidate { id: "FU-C3", name: "Compact ST",        n6_exact: true,  param_a: 3.0,  param_b: 1.5 },  field_tesla: 3.0,  uses_hts: true },
        FuL3 { base: FuCandidate { id: "FU-C4", name: "Laser ICF",         n6_exact: false, param_a: 0.0,  param_b: 5.0 },  field_tesla: 0.0,  uses_hts: false },
        FuL3 { base: FuCandidate { id: "FU-C5", name: "Magnetized Target", n6_exact: true,  param_a: 4.0,  param_b: 3.0 },  field_tesla: 4.0,  uses_hts: true },
    ]
}

fn fu_conversions() -> Vec<FuCandidate> {
    vec![
        FuCandidate { id: "FU-S1", name: "Steam Rankine",     n6_exact: true, param_a: 0.333, param_b: 0.7 },
        FuCandidate { id: "FU-S2", name: "Direct MHD",        n6_exact: true, param_a: 0.500, param_b: 0.4 },
        FuCandidate { id: "FU-S3", name: "TEG Thermoelectric", n6_exact: true, param_a: 0.167, param_b: 0.9 },
        FuCandidate { id: "FU-S4", name: "Egyptian Hybrid",   n6_exact: true, param_a: 0.833, param_b: 0.2 },
    ]
}

fn fu_grids() -> Vec<FuCandidate> {
    vec![
        FuCandidate { id: "FU-G1", name: "Central GW",         n6_exact: false, param_a: 1000.0, param_b: 0.3 },
        FuCandidate { id: "FU-G2", name: "Distributed 100MW",  n6_exact: true,  param_a: 100.0,  param_b: 0.6 },
        FuCandidate { id: "FU-G3", name: "Micro-fusion 10MW",  n6_exact: true,  param_a: 10.0,   param_b: 0.8 },
        FuCandidate { id: "FU-G4", name: "Fission-Fusion 500MW", n6_exact: true, param_a: 500.0, param_b: 0.5 },
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// Superconductor 후보 데이터
// ═══════════════════════════════════════════════════════════════════════

fn sc_materials() -> Vec<ScMaterial> {
    vec![
        ScMaterial { id: "SC-M1", name: "NbTi",     sc_type: "LTS",   tc_k: 9.0,   hc2_t: 15.0,  n6_atoms: 2,  n6_exact: false },
        ScMaterial { id: "SC-M2", name: "Nb3Sn",    sc_type: "LTS",   tc_k: 18.0,  hc2_t: 30.0,  n6_atoms: 6,  n6_exact: true },  // 6 atoms = n
        ScMaterial { id: "SC-M3", name: "MgB2",     sc_type: "MTS",   tc_k: 39.0,  hc2_t: 16.0,  n6_atoms: 3,  n6_exact: true },  // Mg Z=12=sigma
        ScMaterial { id: "SC-M4", name: "REBCO",    sc_type: "HTS",   tc_k: 93.0,  hc2_t: 120.0, n6_atoms: 6,  n6_exact: true },  // Y:Ba:Cu=1:2:3, sum=6
        ScMaterial { id: "SC-M5", name: "Bi2223",   sc_type: "HTS",   tc_k: 110.0, hc2_t: 50.0,  n6_atoms: 8,  n6_exact: false },
        ScMaterial { id: "SC-M6", name: "BSCCO2212",sc_type: "HTS",   tc_k: 85.0,  hc2_t: 100.0, n6_atoms: 7,  n6_exact: false },
        ScMaterial { id: "SC-M7", name: "FeSe",     sc_type: "HTS",   tc_k: 37.0,  hc2_t: 50.0,  n6_atoms: 2,  n6_exact: false },
        ScMaterial { id: "SC-M8", name: "LaH10",    sc_type: "RoomT", tc_k: 260.0, hc2_t: 200.0, n6_atoms: 11, n6_exact: false },
    ]
}

fn sc_processes() -> Vec<ScProcess> {
    vec![
        ScProcess { id: "SC-P1", name: "PIT",       steps: 6, temp: 700, n6_exact: true },  // n=6 steps
        ScProcess { id: "SC-P2", name: "MOCVD",     steps: 5, temp: 800, n6_exact: true },  // sopfr=5
        ScProcess { id: "SC-P3", name: "MOD_RABiTS",steps: 4, temp: 500, n6_exact: true },  // tau=4
        ScProcess { id: "SC-P4", name: "Bronze",    steps: 6, temp: 700, n6_exact: true },  // n=6
        ScProcess { id: "SC-P5", name: "RCE_DR",    steps: 5, temp: 800, n6_exact: true },  // sopfr=5
        ScProcess { id: "SC-P6", name: "DAC_CVD",   steps: 4, temp: 0,   n6_exact: true },  // tau=4
    ]
}

fn sc_wires() -> Vec<ScWire> {
    vec![
        ScWire { id: "SC-W1", name: "Round",       strands: 1,  je_factor: 1.0, n6_exact: false },
        ScWire { id: "SC-W2", name: "FlatTape2G",  strands: 1,  je_factor: 1.5, n6_exact: true },  // 15=sigma+n/phi
        ScWire { id: "SC-W3", name: "Rutherford",  strands: 12, je_factor: 1.2, n6_exact: true },  // sigma=12
        ScWire { id: "SC-W4", name: "CORC",        strands: 6,  je_factor: 1.4, n6_exact: true },  // n=6 tapes
        ScWire { id: "SC-W5", name: "ThinFilm",    strands: 1,  je_factor: 2.0, n6_exact: false },
    ]
}

fn sc_magnets() -> Vec<ScMagnet> {
    vec![
        ScMagnet { id: "SC-MG1", name: "Solenoid_TF", coils: 12, field_limit: 35.0, for_fusion: true,  n6_exact: true },  // sigma=12
        ScMagnet { id: "SC-MG2", name: "Solenoid_CS", coils: 6,  field_limit: 40.0, for_fusion: true,  n6_exact: true },  // n=6
        ScMagnet { id: "SC-MG3", name: "Toroidal_D",  coils: 12, field_limit: 30.0, for_fusion: true,  n6_exact: true },  // sigma=12
        ScMagnet { id: "SC-MG4", name: "Hybrid_LH",   coils: 2,  field_limit: 45.0, for_fusion: true,  n6_exact: true },  // phi=2
        ScMagnet { id: "SC-MG5", name: "Dipole",      coils: 2,  field_limit: 20.0, for_fusion: false, n6_exact: true },  // phi=2
        ScMagnet { id: "SC-MG6", name: "SMES",        coils: 6,  field_limit: 12.0, for_fusion: false, n6_exact: true },  // n=6
    ]
}

fn sc_coolings() -> Vec<ScCooling> {
    vec![
        ScCooling { id: "SC-CL1", name: "LHe_4K",       temp_k: 4.2,  power_per_m: 50.0, n6_exact: true },   // tau~4.2
        ScCooling { id: "SC-CL2", name: "NoInsul_20K",   temp_k: 20.0, power_per_m: 20.0, n6_exact: false },
        ScCooling { id: "SC-CL3", name: "CryoCooler",    temp_k: 20.0, power_per_m: 15.0, n6_exact: false },
        ScCooling { id: "SC-CL4", name: "Hybrid_4K20K",  temp_k: 4.2,  power_per_m: 35.0, n6_exact: true },   // dual tau
    ]
}

fn sc_systems() -> Vec<ScSystem> {
    vec![
        ScSystem { id: "SC-SY1", name: "PowerTransmit", min_field: 0.0,  coil_sets: 1,  for_fusion: false, n6_exact: false },
        ScSystem { id: "SC-SY2", name: "Maglev",        min_field: 5.0,  coil_sets: 6,  for_fusion: false, n6_exact: false },
        ScSystem { id: "SC-SY3", name: "FusionMagnet",  min_field: 20.0, coil_sets: 12, for_fusion: true,  n6_exact: true },  // sigma=12
        ScSystem { id: "SC-SY4", name: "QuantumChip",   min_field: 0.0,  coil_sets: 1,  for_fusion: false, n6_exact: false },
        ScSystem { id: "SC-SY5", name: "EnergyGrid",    min_field: 5.0,  coil_sets: 6,  for_fusion: false, n6_exact: false },
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// DSE 결과 구조체
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct FuResult {
    idx: [usize; 5],  // L1..L5 indices
    n6_count: u32,
    score: f64,
    energy_mev: f64,
    efficiency: f64,
    power_mw: f64,
    // bridge params
    coils: u32,
    is_tokamak: bool,
    field_tesla: f64,
    uses_hts: bool,
    has_egyptian: bool,
}

#[derive(Clone)]
struct ScResult {
    idx: [usize; 6],  // L1..L6 indices
    n6_count: u32,
    score: f64,
    field_achievable: f64,
    tc_k: f64,
    hc2_t: f64,
    for_fusion: bool,
    magnet_coils: u32,
    field_limit: f64,
    power_per_m: f64,
}

#[derive(Clone)]
struct CrossResult {
    fu_rank: usize,
    sc_rank: usize,
    fu_score: f64,
    sc_score: f64,
    bridge_bonus: f64,
    cross_score: f64,
    field_achieved: f64,
    n6_total: u32,
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 1a: Fusion DSE
// ═══════════════════════════════════════════════════════════════════════

fn run_fusion_dse() -> Vec<FuResult> {
    let l1 = fu_materials();
    let l2 = fu_confinements();
    let l3 = fu_cores();
    let l4 = fu_conversions();
    let l5 = fu_grids();

    let total = l1.len() * l2.len() * l3.len() * l4.len() * l5.len();
    let mut results: Vec<FuResult> = Vec::with_capacity(total);

    for (i0, m) in l1.iter().enumerate() {
        for (i1, p) in l2.iter().enumerate() {
            for (i2, c) in l3.iter().enumerate() {
                for (i3, s) in l4.iter().enumerate() {
                    for (i4, g) in l5.iter().enumerate() {
                        let n6_count = [m.n6_exact, p.base.n6_exact, c.base.n6_exact, s.n6_exact, g.n6_exact]
                            .iter().filter(|&&x| x).count() as u32;

                        let n6_score = n6_count as f64 / 5.0 * 0.40;

                        // perf: energy * eff * power
                        let raw_perf = m.param_a * s.param_a * g.param_a;
                        let max_perf = 22.4 * 0.833 * 1000.0;
                        let perf = (raw_perf / max_perf).min(1.0) * 0.30;

                        let eff = s.param_a.min(1.0) * 0.20;

                        let cost_raw = 1.0 - (m.param_b * 0.02 + s.param_b * 0.5 + g.param_b * 0.48);
                        let cost = cost_raw.max(0.0).min(1.0) * 0.10;

                        let score = n6_score + perf + eff + cost;

                        let has_egyptian = s.id == "FU-S4";  // Egyptian Hybrid 1/2+1/3+1/6

                        results.push(FuResult {
                            idx: [i0, i1, i2, i3, i4],
                            n6_count,
                            score,
                            energy_mev: m.param_a,
                            efficiency: s.param_a,
                            power_mw: g.param_a,
                            coils: p.coils,
                            is_tokamak: p.is_tokamak,
                            field_tesla: c.field_tesla,
                            uses_hts: c.uses_hts,
                            has_egyptian,
                        });
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 1b: Superconductor DSE (fusion-compatible filter)
// ═══════════════════════════════════════════════════════════════════════

fn run_sc_dse(fusion_only: bool) -> Vec<ScResult> {
    let l1 = sc_materials();
    let l2 = sc_processes();
    let l3 = sc_wires();
    let l4 = sc_magnets();
    let l5 = sc_coolings();
    let l6 = sc_systems();

    let mut results: Vec<ScResult> = Vec::new();

    for (i0, mat) in l1.iter().enumerate() {
        for (i1, proc) in l2.iter().enumerate() {
            for (i2, wire) in l3.iter().enumerate() {
                for (i3, mag) in l4.iter().enumerate() {
                    // fusion filter: only fusion magnets
                    if fusion_only && !mag.for_fusion { continue; }

                    for (i4, cool) in l5.iter().enumerate() {
                        for (i5, sys) in l6.iter().enumerate() {
                            // fusion filter: only FusionMagnet system
                            if fusion_only && !sys.for_fusion { continue; }

                            let n6_count = [mat.n6_exact, proc.n6_exact, wire.n6_exact,
                                           mag.n6_exact, cool.n6_exact, sys.n6_exact]
                                .iter().filter(|&&x| x).count() as u32;

                            // field_achievable = min(hc2 * je_factor * 0.3, magnet field_limit)
                            let field_achievable = (mat.hc2_t * wire.je_factor * 0.3).min(mag.field_limit);

                            let n6_score = n6_count as f64 / 6.0 * 0.35;
                            let field_score = (field_achievable / 45.0).min(1.0) * 0.30;
                            let eff_score = (1.0 - cool.power_per_m / 50.0).max(0.0) * 0.20;

                            // material cost heuristic: LTS=1, MTS=2, HTS=3, RoomT=5
                            let mat_cost = match mat.sc_type {
                                "LTS" => 1.0,
                                "MTS" => 2.0,
                                "HTS" => 3.0,
                                "RoomT" => 5.0,
                                _ => 3.0,
                            };
                            let cost_score = (1.0_f64 - mat_cost / 5.0).max(0.0) * 0.15;

                            let score = n6_score + field_score + eff_score + cost_score;

                            results.push(ScResult {
                                idx: [i0, i1, i2, i3, i4, i5],
                                n6_count,
                                score,
                                field_achievable,
                                tc_k: mat.tc_k,
                                hc2_t: mat.hc2_t,
                                for_fusion: mag.for_fusion && sys.for_fusion,
                                magnet_coils: mag.coils,
                                field_limit: mag.field_limit,
                                power_per_m: cool.power_per_m,
                            });
                        }
                    }
                }
            }
        }
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap());
    results
}

// ═══════════════════════════════════════════════════════════════════════
// Phase 2+3: Cross-DSE with compatibility filter
// ═══════════════════════════════════════════════════════════════════════

fn is_compatible(fu: &FuResult, sc: &ScResult, sc_mat: &ScMaterial) -> bool {
    // Rule 1: HTS fusion core requires HTS material (Tc >= 37K)
    if fu.uses_hts && sc.tc_k < 37.0 { return false; }

    // Rule 2+3: Tokamak requires fusion magnets
    if fu.is_tokamak && !sc.for_fusion { return false; }

    // Rule 4: SC must deliver what fusion needs
    if sc.field_limit < fu.field_tesla { return false; }

    // Rule 5: High-field fusion needs high Hc2
    if fu.field_tesla > 12.0 && sc_mat.hc2_t < fu.field_tesla * 2.0 { return false; }

    // Non-tokamak: allow any SC path
    true
}

fn bridge_bonus(fu: &FuResult, sc: &ScResult) -> f64 {
    let mut bonus: f64 = 0.0;

    // +0.2: coil match
    if fu.coils > 0 && fu.coils == sc.magnet_coils { bonus += 0.2; }

    // +0.2: next-gen tokamak threshold (30T+)
    if sc.field_achievable >= 30.0 { bonus += 0.2; }

    // +0.2: both domains >= 80% n6 EXACT
    let fu_pct = fu.n6_count as f64 / 5.0;
    let sc_pct = sc.n6_count as f64 / 6.0;
    if fu_pct >= 0.8 && sc_pct >= 0.8 { bonus += 0.2; }

    // +0.2: Egyptian fraction in conversion
    if fu.has_egyptian { bonus += 0.2; }

    // +0.2: total n6 >= 9/11
    let total_n6 = fu.n6_count + sc.n6_count;
    if total_n6 >= 9 { bonus += 0.2; }

    bonus.min(1.0)
}

fn run_cross_dse(fu_top: &[FuResult], sc_top: &[ScResult]) -> Vec<CrossResult> {
    let sc_mats = sc_materials();
    let mut results: Vec<CrossResult> = Vec::new();

    for (fi, fu) in fu_top.iter().enumerate() {
        for (si, sc) in sc_top.iter().enumerate() {
            let mat = &sc_mats[sc.idx[0]];
            if !is_compatible(fu, sc, mat) { continue; }

            let bb = bridge_bonus(fu, sc);
            let cross_score = fu.score * 0.45 + sc.score * 0.35 + bb * 0.20;

            results.push(CrossResult {
                fu_rank: fi,
                sc_rank: si,
                fu_score: fu.score,
                sc_score: sc.score,
                bridge_bonus: bb,
                cross_score,
                field_achieved: sc.field_achievable,
                n6_total: fu.n6_count + sc.n6_count,
            });
        }
    }

    results.sort_by(|a, b| b.cross_score.partial_cmp(&a.cross_score).unwrap());
    results
}

// ═══════════════════════════════════════════════════════════════════════
// 출력 헬퍼
// ═══════════════════════════════════════════════════════════════════════

fn fu_path_str(r: &FuResult) -> String {
    let l1 = fu_materials();
    let l2 = fu_confinements();
    let l3 = fu_cores();
    let l4 = fu_conversions();
    let l5 = fu_grids();
    format!("{} > {} > {} > {} > {}",
        l1[r.idx[0]].id, l2[r.idx[1]].base.id, l3[r.idx[2]].base.id,
        l4[r.idx[3]].id, l5[r.idx[4]].id)
}

fn sc_path_str(r: &ScResult) -> String {
    let l1 = sc_materials();
    let l2 = sc_processes();
    let l3 = sc_wires();
    let l4 = sc_magnets();
    let l5 = sc_coolings();
    let l6 = sc_systems();
    format!("{} > {} > {} > {} > {} > {}",
        l1[r.idx[0]].id, l2[r.idx[1]].id, l3[r.idx[2]].id,
        l4[r.idx[3]].id, l5[r.idx[4]].id, l6[r.idx[5]].id)
}

fn print_fu_table(results: &[FuResult], count: usize) {
    let l1 = fu_materials();
    let l2 = fu_confinements();
    let l3 = fu_cores();
    let l4 = fu_conversions();
    let l5 = fu_grids();

    println!("  | {:>4} | {:>14} | {:>14} | {:>14} | {:>14} | {:>16} | {:>3} | {:>6} |",
        "Rank", "L1 Material", "L2 Confine", "L3 Core", "L4 Convert", "L5 Grid", "n6", "Score");
    println!("  |------|----------------|----------------|----------------|----------------|------------------|-----|--------|");
    for (i, r) in results.iter().take(count).enumerate() {
        println!("  | {:>4} | {:>14} | {:>14} | {:>14} | {:>14} | {:>16} | {:>1}/{} | {:.4} |",
            i + 1,
            l1[r.idx[0]].name,
            l2[r.idx[1]].base.name,
            l3[r.idx[2]].base.name,
            l4[r.idx[3]].name,
            l5[r.idx[4]].name,
            r.n6_count, 5,
            r.score);
    }
}

fn print_sc_table(results: &[ScResult], count: usize) {
    let l1 = sc_materials();
    let l2 = sc_processes();
    let l3 = sc_wires();
    let l4 = sc_magnets();
    let l5 = sc_coolings();
    let l6 = sc_systems();

    println!("  | {:>4} | {:>8} | {:>9} | {:>10} | {:>11} | {:>12} | {:>12} | {:>3} | {:>6} | {:>7} |",
        "Rank", "L1 Mat", "L2 Proc", "L3 Wire", "L4 Magnet", "L5 Cooling", "L6 System", "n6", "Score", "Field");
    println!("  |------|----------|-----------|------------|-------------|--------------|--------------|-----|--------|---------|");
    for (i, r) in results.iter().take(count).enumerate() {
        println!("  | {:>4} | {:>8} | {:>9} | {:>10} | {:>11} | {:>12} | {:>12} | {:>1}/{} | {:.4} | {:>5.1}T |",
            i + 1,
            l1[r.idx[0]].name,
            l2[r.idx[1]].name,
            l3[r.idx[2]].name,
            l4[r.idx[3]].name,
            l5[r.idx[4]].name,
            l6[r.idx[5]].name,
            r.n6_count, 6,
            r.score,
            r.field_achievable);
    }
}

fn print_cross_table(cross: &[CrossResult], fu_top: &[FuResult], sc_top: &[ScResult], count: usize) {
    println!("  | {:>4} | {:>38} | {:>46} | {:>6} | {:>5} | {:>7} | {:>7} |",
        "Rank", "Fusion Path", "SC Path", "Bridge", "n6", "Score", "Field");
    println!("  |------|----------------------------------------|------------------------------------------------|--------|-------|---------|---------|");
    for (i, c) in cross.iter().take(count).enumerate() {
        let fu = &fu_top[c.fu_rank];
        let sc = &sc_top[c.sc_rank];
        println!("  | {:>4} | {:>38} | {:>46} | {:>5.2} | {:>2}/11 | {:.4} | {:>5.1}T |",
            i + 1,
            fu_path_str(fu),
            sc_path_str(sc),
            c.bridge_bonus,
            c.n6_total,
            c.cross_score,
            c.field_achieved);
    }
}

// ═══════════════════════════════════════════════════════════════════════
// main
// ═══════════════════════════════════════════════════════════════════════

fn main() {
    println!();
    println!("{}", "=".repeat(67));
    println!("  N6 Cross-DSE: FUSION x SUPERCONDUCTOR");
    println!("  sigma(n)*phi(n) = n*tau(n), n=6");
    println!("  30T+ Tokamak Magnet Optimization");
    println!("{}", "=".repeat(67));

    // ── Phase 1a: Fusion ──
    let fu_all = run_fusion_dse();
    let fu_total = fu_all.len();
    println!();
    println!("=== Phase 1a: Fusion Domain ({} combos) ===", fu_total);
    println!();
    println!("  Top 10:");
    print_fu_table(&fu_all, 10);

    // Stats
    let fu_5of5 = fu_all.iter().filter(|r| r.n6_count == 5).count();
    let fu_4of5 = fu_all.iter().filter(|r| r.n6_count >= 4).count();
    println!();
    println!("  n6 stats: 5/5 EXACT = {}, >=4/5 = {} ({:.1}%)",
        fu_5of5, fu_4of5, fu_4of5 as f64 / fu_total as f64 * 100.0);

    // ── Phase 1b: SC (fusion-compatible) ──
    let sc_filt = run_sc_dse(true);
    let sc_total = sc_filt.len();
    println!();
    println!("=== Phase 1b: Superconductor Domain (fusion-compatible: {} combos) ===", sc_total);
    println!();
    println!("  Top 10:");
    print_sc_table(&sc_filt, 10);

    let sc_6of6 = sc_filt.iter().filter(|r| r.n6_count == 6).count();
    let sc_5of6 = sc_filt.iter().filter(|r| r.n6_count >= 5).count();
    println!();
    println!("  n6 stats: 6/6 EXACT = {}, >=5/6 = {} ({:.1}%)",
        sc_6of6, sc_5of6, sc_5of6 as f64 / sc_total as f64 * 100.0);

    // Also run full SC for reference
    let sc_all = run_sc_dse(false);
    println!("  (Full SC domain: {} total combos, filtered to {} fusion-compatible)",
        sc_all.len(), sc_total);

    // ── Phase 2+3: Cross-DSE ──
    let fu_top50: Vec<FuResult> = fu_all.iter().take(50).cloned().collect();
    let sc_top50: Vec<ScResult> = sc_filt.iter().take(50).cloned().collect();

    let cross = run_cross_dse(&fu_top50, &sc_top50);
    let compatible = cross.len();
    println!();
    println!("=== Phase 2+3: Cross-DSE Bridge (top50 x top50 = 2,500 attempted) ===");
    println!("  Compatible pairs: {}", compatible);
    println!();
    println!("  Top 10 Cross-DSE:");
    print_cross_table(&cross, &fu_top50, &sc_top50, 10);

    // ── n6 distribution in cross results ──
    if !cross.is_empty() {
        let avg_n6: f64 = cross.iter().take(20).map(|c| c.n6_total as f64).sum::<f64>() / cross.iter().take(20).count() as f64;
        let max_n6 = cross.iter().take(20).map(|c| c.n6_total).max().unwrap_or(0);
        println!();
        println!("  Cross n6 stats (top-20): avg={:.1}/11, max={}/11", avg_n6, max_n6);
    }

    // ── Phase 4: ULTIMATE WINNER ──
    if let Some(winner) = cross.first() {
        let fu = &fu_top50[winner.fu_rank];
        let sc = &sc_top50[winner.sc_rank];

        let l1f = fu_materials();
        let l2f = fu_confinements();
        let l3f = fu_cores();
        let l4f = fu_conversions();
        let l5f = fu_grids();

        let l1s = sc_materials();
        let l2s = sc_processes();
        let l3s = sc_wires();
        let l4s = sc_magnets();
        let l5s = sc_coolings();
        let l6s = sc_systems();

        println!();
        println!("{}", "=".repeat(67));
        println!("  ULTIMATE FUSION-SC WINNER");
        println!("{}", "=".repeat(67));

        // Fusion detail
        println!();
        println!("  +--- FUSION (score={:.4}, n6={}/5) ------+", fu.score, fu.n6_count);
        println!("  | L1 Material:  {} ({})",
            l1f[fu.idx[0]].name, l1f[fu.idx[0]].id);
        println!("  |   energy = {:.1} MeV, n6_exact = {}",
            fu.energy_mev, l1f[fu.idx[0]].n6_exact);
        println!("  | L2 Confine:   {} ({})",
            l2f[fu.idx[1]].base.name, l2f[fu.idx[1]].base.id);
        println!("  |   coils = {}, tokamak = {}, n6_exact = {}",
            fu.coils, fu.is_tokamak, l2f[fu.idx[1]].base.n6_exact);
        println!("  | L3 Core:      {} ({})",
            l3f[fu.idx[2]].base.name, l3f[fu.idx[2]].base.id);
        println!("  |   field = {:.1}T, uses_hts = {}, n6_exact = {}",
            fu.field_tesla, fu.uses_hts, l3f[fu.idx[2]].base.n6_exact);
        println!("  | L4 Convert:   {} ({})",
            l4f[fu.idx[3]].name, l4f[fu.idx[3]].id);
        println!("  |   efficiency = {:.1}%, n6_exact = {}",
            fu.efficiency * 100.0, l4f[fu.idx[3]].n6_exact);
        println!("  | L5 Grid:      {} ({})",
            l5f[fu.idx[4]].name, l5f[fu.idx[4]].id);
        println!("  |   power = {:.0} MW, n6_exact = {}",
            fu.power_mw, l5f[fu.idx[4]].n6_exact);
        println!("  +-----------------------------------------+");

        // SC detail
        println!();
        println!("  +--- SUPERCONDUCTOR (score={:.4}, n6={}/6) ---+", sc.score, sc.n6_count);
        println!("  | L1 Material:  {} ({})",
            l1s[sc.idx[0]].name, l1s[sc.idx[0]].id);
        println!("  |   type = {}, Tc = {:.0}K, Hc2 = {:.0}T, atoms = {}, n6 = {}",
            l1s[sc.idx[0]].sc_type, sc.tc_k, sc.hc2_t, l1s[sc.idx[0]].n6_atoms, l1s[sc.idx[0]].n6_exact);
        println!("  | L2 Process:   {} ({})",
            l2s[sc.idx[1]].name, l2s[sc.idx[1]].id);
        println!("  |   steps = {}, temp = {}C, n6 = {}",
            l2s[sc.idx[1]].steps, l2s[sc.idx[1]].temp, l2s[sc.idx[1]].n6_exact);
        println!("  | L3 Wire:      {} ({})",
            l3s[sc.idx[2]].name, l3s[sc.idx[2]].id);
        println!("  |   strands = {}, je_factor = {:.1}, n6 = {}",
            l3s[sc.idx[2]].strands, l3s[sc.idx[2]].je_factor, l3s[sc.idx[2]].n6_exact);
        println!("  | L4 Magnet:    {} ({})",
            l4s[sc.idx[3]].name, l4s[sc.idx[3]].id);
        println!("  |   coils = {}, field_limit = {:.0}T, fusion = {}, n6 = {}",
            l4s[sc.idx[3]].coils, sc.field_limit, l4s[sc.idx[3]].for_fusion, l4s[sc.idx[3]].n6_exact);
        println!("  | L5 Cooling:   {} ({})",
            l5s[sc.idx[4]].name, l5s[sc.idx[4]].id);
        println!("  |   temp = {:.1}K, power = {:.0} W/m, n6 = {}",
            l5s[sc.idx[4]].temp_k, sc.power_per_m, l5s[sc.idx[4]].n6_exact);
        println!("  | L6 System:    {} ({})",
            l6s[sc.idx[5]].name, l6s[sc.idx[5]].id);
        println!("  |   coil_sets = {}, for_fusion = {}, n6 = {}",
            l6s[sc.idx[5]].coil_sets, l6s[sc.idx[5]].for_fusion, l6s[sc.idx[5]].n6_exact);
        println!("  +---------------------------------------------+");

        // Bridge
        println!();
        println!("  +--- BRIDGE PARAMETERS -------------------------+");
        let coil_match = fu.coils > 0 && fu.coils == sc.magnet_coils;
        println!("  | Coil match:   fusion {} == SC {} ? {}",
            fu.coils, sc.magnet_coils, if coil_match { "YES" } else { "NO" });
        println!("  | Field:        fusion needs {:.1}T, SC delivers {:.1}T",
            fu.field_tesla, sc.field_achievable);
        println!("  | 30T+ check:   {} (next-gen tokamak)",
            if sc.field_achievable >= 30.0 { "PASS" } else { "FAIL" });
        println!("  | Egyptian:     {} (1/2+1/3+1/6 conversion)",
            if fu.has_egyptian { "YES" } else { "NO" });
        println!("  | Bridge bonus: {:.2}", winner.bridge_bonus);
        println!("  | Cross score:  {:.4}", winner.cross_score);
        println!("  | n6 EXACT:     {}/11 ({:.1}%)",
            winner.n6_total, winner.n6_total as f64 / 11.0 * 100.0);
        println!("  +-------------------------------------------------+");

        // ASCII architecture diagram
        println!();
        println!("  +--- ARCHITECTURE DIAGRAM -----------------------------------------------+");
        println!("  |                                                                        |");
        println!("  |  FUSION                          SUPERCONDUCTOR                        |");
        println!("  |  ======                          ==============                        |");
        println!("  |  L1 {:>16}               L1 {:>10} (Tc={:.0}K)              |",
            l1f[fu.idx[0]].name, l1s[sc.idx[0]].name, sc.tc_k);
        println!("  |       |                                |                               |");
        println!("  |  L2 {:>16}               L2 {:>10}                         |",
            l2f[fu.idx[1]].base.name, l2s[sc.idx[1]].name);
        println!("  |       |                                |                               |");
        println!("  |  L3 {:>16} ---{:.0}T--> L3 {:>10} (je={:.1}x)             |",
            l3f[fu.idx[2]].base.name, fu.field_tesla, l3s[sc.idx[2]].name, l3s[sc.idx[2]].je_factor);
        println!("  |       |                ^               |                               |");
        println!("  |  L4 {:>16}  |          L4 {:>11} ({:.0}T limit)         |",
            l4f[fu.idx[3]].name, l4s[sc.idx[3]].name, sc.field_limit);
        println!("  |       |              BRIDGE            |                               |");
        println!("  |  L5 {:>16} ({:.0}MW)  L5 {:>12} ({:.1}K)              |",
            l5f[fu.idx[4]].name, fu.power_mw, l5s[sc.idx[4]].name, l5s[sc.idx[4]].temp_k);
        println!("  |                                        |                               |");
        println!("  |                                   L6 {:>12} (sigma=12)          |",
            l6s[sc.idx[5]].name);
        println!("  |                                                                        |");
        println!("  |  sigma(6)*phi(6) = 6*tau(6) = 24 = J_2(6)                             |");
        println!("  +------------------------------------------------------------------------+");
    }

    // ── Summary ──
    println!();
    println!("{}", "=".repeat(67));
    println!("  SUMMARY");
    println!("{}", "=".repeat(67));
    println!("  Fusion domain:     {} combinations explored", fu_total);
    println!("  SC domain (full):  {} combinations", sc_all.len());
    println!("  SC fusion-compat:  {} combinations", sc_total);
    println!("  Cross-DSE pairs:   {} compatible / 2,500 attempted", compatible);
    if let Some(w) = cross.first() {
        println!("  Best cross score:  {:.4}", w.cross_score);
        println!("  Best field:        {:.1}T", w.field_achieved);
        println!("  Best n6 EXACT:     {}/11 ({:.1}%)", w.n6_total, w.n6_total as f64 / 11.0 * 100.0);
    }

    // n=6 signature
    println!();
    println!("  n=6: sigma(6)*phi(6) = 12*2 = 24 = 6*4 = n*tau(6)");
    println!("  The unique solution. QED.");
    println!();
}
