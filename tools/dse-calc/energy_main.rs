// ═══════════════════════════════════════════════════════════════════════
// N6 Energy DSE — 궁극의 에너지 통합 설계공간 탐색
// ═══════════════════════════════════════════════════════════════════════
// 4 도메인 (Fusion, Solar, Battery, Grid) × 5 레벨 후보 전수탐색
// 빌드: ~/.cargo/bin/rustc tools/dse-calc/energy_main.rs -o tools/dse-calc/energy-dse
// 실행: ./tools/dse-calc/energy-dse
// ═══════════════════════════════════════════════════════════════════════

#![allow(dead_code)]

// ── N6 상수 ──
const N: u64 = 6;
const SIGMA: u64 = 12;   // σ(6) = 1+2+3+6
const TAU: u64 = 4;      // τ(6) = 4 divisors
const PHI: u64 = 2;      // φ(6) = 2
const SOPFR: u64 = 5;    // sopfr(6) = 2+3
const J2: u64 = 24;      // J₂(6) = Jordan totient
const MU: u64 = 1;       // μ(6) = 1 (squarefree)

// ── 가중치 ──
const W_N6: f64 = 0.40;
const W_PERF: f64 = 0.30;
const W_EFF: f64 = 0.20;
const W_COST: f64 = 0.10;

// ═══════════════════════════════════════════════════════════════════════
// 후보 구조체
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct Candidate {
    id: &'static str,
    name: &'static str,
    n6_exact: bool,
    // 도메인별 수치 파라미터 (범용)
    param_a: f64,  // 주 파라미터
    param_b: f64,  // 보조 파라미터
}

#[derive(Clone)]
struct DseResult {
    ids: [usize; 5],  // 각 레벨 후보 인덱스
    n6_exact_count: u32,
    n6_score: f64,
    performance: f64,
    efficiency: f64,
    cost: f64,
    total_score: f64,
}

// ═══════════════════════════════════════════════════════════════════════
// 도메인 1: FUSION (DSE-FU) — 6×5×5×4×4 = 2,400
// ═══════════════════════════════════════════════════════════════════════

fn fusion_materials() -> Vec<Candidate> {
    vec![
        Candidate { id: "FU-M1", name: "D-T Standard",      n6_exact: true,  param_a: 17.6, param_b: 5.0 },  // sopfr=2+3=5
        Candidate { id: "FU-M2", name: "D-D Proton",        n6_exact: true,  param_a: 3.27, param_b: 4.0 },  // φ+φ=τ
        Candidate { id: "FU-M3", name: "D-He3 Aneutronic",  n6_exact: false, param_a: 18.3, param_b: 17.0 }, // sopfr+σ=17 (non-n6)
        Candidate { id: "FU-M4", name: "p-B11 Clean",       n6_exact: false, param_a: 8.7,  param_b: 9.0 },  // σ-τ+μ=9
        Candidate { id: "FU-M5", name: "Li6-D Hybrid",      n6_exact: true,  param_a: 22.4, param_b: 22.0 }, // J₂-φ=22
        Candidate { id: "FU-M6", name: "Cat-DD Catalyzed",  n6_exact: false, param_a: 7.2,  param_b: 7.0 },  // σ-sopfr=7
    ]
}

fn fusion_confinement() -> Vec<Candidate> {
    vec![
        Candidate { id: "FU-P1", name: "Tokamak-12",       n6_exact: true,  param_a: 12.0, param_b: 1.5 },  // σ coils, ripple 1.5%
        Candidate { id: "FU-P2", name: "Tokamak-24",       n6_exact: true,  param_a: 24.0, param_b: 0.5 },  // J₂ coils
        Candidate { id: "FU-P3", name: "Stellarator-5",    n6_exact: true,  param_a: 5.0,  param_b: 0.8 },  // sopfr periods
        Candidate { id: "FU-P4", name: "Spherical Tokamak", n6_exact: true, param_a: 2.0,  param_b: 2.0 },  // aspect φ=2
        Candidate { id: "FU-P5", name: "ICF",              n6_exact: true,  param_a: 4.0,  param_b: 0.0 },  // τ beams
    ]
}

fn fusion_core() -> Vec<Candidate> {
    vec![
        Candidate { id: "FU-C1", name: "HTS High-Field",     n6_exact: true,  param_a: 12.0, param_b: 1.85 }, // σ=12T
        Candidate { id: "FU-C2", name: "Low-Field Large",    n6_exact: true,  param_a: 6.0,  param_b: 6.2 },  // n=6T
        Candidate { id: "FU-C3", name: "Compact ST",         n6_exact: true,  param_a: 3.0,  param_b: 1.5 },  // n/φ=3T
        Candidate { id: "FU-C4", name: "Laser ICF",          n6_exact: false, param_a: 0.0,  param_b: 5.0 },  // 0T (μ-1)
        Candidate { id: "FU-C5", name: "Magnetized Target",  n6_exact: true,  param_a: 4.0,  param_b: 3.0 },  // τ=4T
    ]
}

fn fusion_conversion() -> Vec<Candidate> {
    vec![
        Candidate { id: "FU-S1", name: "Steam Rankine",     n6_exact: true,  param_a: 0.333, param_b: 0.7 }, // 1/3, cost=0.7
        Candidate { id: "FU-S2", name: "Direct MHD",        n6_exact: true,  param_a: 0.500, param_b: 0.4 }, // 1/2
        Candidate { id: "FU-S3", name: "TEG Thermoelectric", n6_exact: true,  param_a: 0.167, param_b: 0.9 }, // 1/6
        Candidate { id: "FU-S4", name: "Egyptian Hybrid",   n6_exact: true,  param_a: 0.833, param_b: 0.2 }, // 1/2+1/3+1/6
    ]
}

fn fusion_grid() -> Vec<Candidate> {
    vec![
        Candidate { id: "FU-G1", name: "Central GW",         n6_exact: false, param_a: 1000.0, param_b: 0.3 }, // ~768 close
        Candidate { id: "FU-G2", name: "Distributed 100MW",  n6_exact: true,  param_a: 100.0,  param_b: 0.6 }, // (σ-φ)²=100
        Candidate { id: "FU-G3", name: "Micro-fusion 10MW",  n6_exact: true,  param_a: 10.0,   param_b: 0.8 }, // σ-φ=10
        Candidate { id: "FU-G4", name: "Fission-Fusion 500MW", n6_exact: true, param_a: 500.0, param_b: 0.5 }, // sopfr·(σ-φ)²
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// 도메인 2: SOLAR (DSE-SL) — 6×5×5×4×4 = 2,400
// ═══════════════════════════════════════════════════════════════════════

fn solar_materials() -> Vec<Candidate> {
    vec![
        Candidate { id: "SL-M1", name: "c-Si Mono",    n6_exact: false, param_a: 1.12, param_b: 0.22 },  // base eff 22%
        Candidate { id: "SL-M2", name: "mc-Si Poly",   n6_exact: false, param_a: 1.12, param_b: 0.19 },  // 19%
        Candidate { id: "SL-M3", name: "GaAs",         n6_exact: true,  param_a: 1.42, param_b: 0.29 },  // 4/3≈1.333 BT-30
        Candidate { id: "SL-M4", name: "CdTe",         n6_exact: true,  param_a: 1.50, param_b: 0.22 },  // n/τ=1.5
        Candidate { id: "SL-M5", name: "CIGS",         n6_exact: false, param_a: 1.15, param_b: 0.20 },  // ~σ/(σ-φ)
        Candidate { id: "SL-M6", name: "Perovskite",   n6_exact: false, param_a: 1.55, param_b: 0.26 },  // ~n/τ+
    ]
}

fn solar_process() -> Vec<Candidate> {
    vec![
        Candidate { id: "SL-P1", name: "PERC",                n6_exact: false, param_a: 0.0,  param_b: 0.9 }, // baseline, cost 0.9
        Candidate { id: "SL-P2", name: "TOPCon",              n6_exact: true,  param_a: 1.5,  param_b: 0.7 }, // n/τ=1.5
        Candidate { id: "SL-P3", name: "HJT Heterojunction",  n6_exact: true,  param_a: 2.0,  param_b: 0.5 }, // φ=2
        Candidate { id: "SL-P4", name: "IBC Back-contact",    n6_exact: true,  param_a: 2.5,  param_b: 0.4 }, // sopfr/φ=2.5
        Candidate { id: "SL-P5", name: "Tandem Perovskite/Si", n6_exact: true, param_a: 5.0,  param_b: 0.3 }, // sopfr=5
    ]
}

fn solar_module() -> Vec<Candidate> {
    vec![
        Candidate { id: "SL-C1", name: "60-cell",       n6_exact: true,  param_a: 60.0,  param_b: 0.8 },  // σ·sopfr BT-63
        Candidate { id: "SL-C2", name: "72-cell",       n6_exact: true,  param_a: 72.0,  param_b: 0.7 },  // σ·n BT-63
        Candidate { id: "SL-C3", name: "120-half",      n6_exact: true,  param_a: 120.0, param_b: 0.6 },  // σ(σ-φ) BT-63
        Candidate { id: "SL-C4", name: "144-half",      n6_exact: true,  param_a: 144.0, param_b: 0.5 },  // σ² BT-63
        Candidate { id: "SL-C5", name: "210mm Shingled", n6_exact: true, param_a: 66.0,  param_b: 0.6 },  // σ·sopfr+n
    ]
}

fn solar_inverter() -> Vec<Candidate> {
    vec![
        Candidate { id: "SL-S1", name: "String Inverter",    n6_exact: true,  param_a: 2.0, param_b: 0.8 },  // φ stages
        Candidate { id: "SL-S2", name: "Micro Inverter",     n6_exact: true,  param_a: 4.0, param_b: 0.4 },  // τ stages
        Candidate { id: "SL-S3", name: "Central Inverter",   n6_exact: true,  param_a: 3.0, param_b: 0.9 },  // n/φ stages
        Candidate { id: "SL-S4", name: "DC Optimizer Hybrid", n6_exact: true, param_a: 6.0, param_b: 0.5 },  // n stages
    ]
}

fn solar_plant() -> Vec<Candidate> {
    vec![
        Candidate { id: "SL-G1", name: "Fixed Tilt",      n6_exact: false, param_a: 0.0,  param_b: 0.9 },  // baseline
        Candidate { id: "SL-G2", name: "1-Axis",          n6_exact: true,  param_a: 10.0, param_b: 0.7 },  // σ-φ gain
        Candidate { id: "SL-G3", name: "2-Axis",          n6_exact: true,  param_a: 12.0, param_b: 0.5 },  // σ gain, φ axes
        Candidate { id: "SL-G4", name: "CPV Concentrated", n6_exact: true, param_a: 20.0, param_b: 0.3 },  // J₂-τ gain
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// 도메인 3: BATTERY (DSE-BT) — 6×5×5×4×4 = 2,400
// ═══════════════════════════════════════════════════════════════════════

fn battery_materials() -> Vec<Candidate> {
    vec![
        Candidate { id: "BT-M1", name: "LFP Cathode",      n6_exact: true,  param_a: 170.0,  param_b: 0.9 },  // CN=6
        Candidate { id: "BT-M2", name: "NMC811 Cathode",    n6_exact: true,  param_a: 200.0,  param_b: 0.6 },  // CN=6, 8:1:1
        Candidate { id: "BT-M3", name: "NCA Cathode",       n6_exact: true,  param_a: 210.0,  param_b: 0.5 },  // CN=6
        Candidate { id: "BT-M4", name: "Graphite Anode",    n6_exact: true,  param_a: 372.0,  param_b: 0.9 },  // LiC₆ n=6
        Candidate { id: "BT-M5", name: "Silicon Anode",     n6_exact: true,  param_a: 3579.0, param_b: 0.3 },  // σ-φ=10x BT-81
        Candidate { id: "BT-M6", name: "Li Metal Anode",    n6_exact: true,  param_a: 3860.0, param_b: 0.2 },  // σ-φ≈10x BT-81
    ]
}

fn battery_process() -> Vec<Candidate> {
    vec![
        Candidate { id: "BT-P1", name: "Pouch",            n6_exact: false, param_a: 1.0, param_b: 0.7 },
        Candidate { id: "BT-P2", name: "Cylindrical 21700", n6_exact: false, param_a: 1.0, param_b: 0.8 },
        Candidate { id: "BT-P3", name: "Cylindrical 4680", n6_exact: true,  param_a: 1.2, param_b: 0.6 },  // σ·τ≈48mm
        Candidate { id: "BT-P4", name: "Prismatic",        n6_exact: false, param_a: 1.0, param_b: 0.7 },
        Candidate { id: "BT-P5", name: "Solid-State",      n6_exact: true,  param_a: 1.5, param_b: 0.3 },  // CN=6 BT-80
    ]
}

fn battery_module() -> Vec<Candidate> {
    vec![
        Candidate { id: "BT-C1", name: "6S Module",       n6_exact: true,  param_a: 6.0,  param_b: 19.2 },  // n BT-57
        Candidate { id: "BT-C2", name: "12S Module",      n6_exact: true,  param_a: 12.0, param_b: 38.4 },  // σ BT-57
        Candidate { id: "BT-C3", name: "24S Module",      n6_exact: true,  param_a: 24.0, param_b: 76.8 },  // J₂ BT-57
        Candidate { id: "BT-C4", name: "CTP Cell-to-Pack", n6_exact: false, param_a: 16.0, param_b: 51.2 }, // skip module
        Candidate { id: "BT-C5", name: "CTC Cell-to-Chassis", n6_exact: false, param_a: 20.0, param_b: 64.0 },
    ]
}

fn battery_pack() -> Vec<Candidate> {
    vec![
        Candidate { id: "BT-S1", name: "96S 400V",          n6_exact: true,  param_a: 96.0,  param_b: 384.0 },  // σ(σ-τ) BT-57,84
        Candidate { id: "BT-S2", name: "192S 800V",         n6_exact: true,  param_a: 192.0, param_b: 768.0 },  // φ·σ(σ-τ) BT-57,84
        Candidate { id: "BT-S3", name: "Hybrid Egyptian",   n6_exact: true,  param_a: 72.0,  param_b: 288.0 },  // 1/2+1/3+1/6
        Candidate { id: "BT-S4", name: "Modular ESS 6kWh",  n6_exact: true,  param_a: 6.0,   param_b: 48.0 },   // n, σ·τ
    ]
}

fn battery_system() -> Vec<Candidate> {
    vec![
        Candidate { id: "BT-G1", name: "EV 400V",       n6_exact: true,  param_a: 96.0,  param_b: 0.5 },  // 96S=σ(σ-τ) BT-84
        Candidate { id: "BT-G2", name: "EV 800V",       n6_exact: true,  param_a: 192.0, param_b: 0.3 },  // 192S BT-84
        Candidate { id: "BT-G3", name: "Grid ESS MWh",  n6_exact: true,  param_a: 24.0,  param_b: 0.6 },  // J₂=24 clusters
        Candidate { id: "BT-G4", name: "Home ESS 10kWh", n6_exact: true, param_a: 12.0,  param_b: 0.8 },  // σ=12 modules
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// 도메인 4: GRID (DSE-GR) — 6×5×5×4×4 = 2,400
// ═══════════════════════════════════════════════════════════════════════

fn grid_materials() -> Vec<Candidate> {
    vec![
        Candidate { id: "GR-M1", name: "ACSR Aluminum",     n6_exact: false, param_a: 1.0, param_b: 0.9 },
        Candidate { id: "GR-M2", name: "HTLS High-Temp",    n6_exact: false, param_a: 1.5, param_b: 0.6 },
        Candidate { id: "GR-M3", name: "HTS YBCO",          n6_exact: true,  param_a: 3.0, param_b: 0.2 },  // loss=0=R(6)-1
        Candidate { id: "GR-M4", name: "XLPE Submarine",    n6_exact: false, param_a: 1.2, param_b: 0.5 },
        Candidate { id: "GR-M5", name: "GIL Gas-Insulated", n6_exact: true,  param_a: 2.0, param_b: 0.3 },  // n=6 fluorine
        Candidate { id: "GR-M6", name: "MgB2 Superconductor", n6_exact: false, param_a: 2.5, param_b: 0.3 },
    ]
}

fn grid_conversion() -> Vec<Candidate> {
    vec![
        Candidate { id: "GR-P1", name: "6-Pulse Rectifier",  n6_exact: true,  param_a: 6.0,  param_b: 25.0 },  // n pulses, 25% THD
        Candidate { id: "GR-P2", name: "12-Pulse Rectifier", n6_exact: true,  param_a: 12.0, param_b: 10.0 },  // σ pulses
        Candidate { id: "GR-P3", name: "MMC Multilevel",     n6_exact: true,  param_a: 6.0,  param_b: 2.0 },   // n levels
        Candidate { id: "GR-P4", name: "SiC MOSFET",         n6_exact: false, param_a: 1.0,  param_b: 1.0 },
        Candidate { id: "GR-P5", name: "GaN HEMT",           n6_exact: false, param_a: 1.0,  param_b: 0.5 },
    ]
}

fn grid_transformer() -> Vec<Candidate> {
    vec![
        Candidate { id: "GR-C1", name: "Oil-Immersed",    n6_exact: true,  param_a: 3.0, param_b: 0.8 },  // n/φ=3 phases
        Candidate { id: "GR-C2", name: "Dry-Type",        n6_exact: true,  param_a: 3.0, param_b: 0.7 },  // n/φ=3
        Candidate { id: "GR-C3", name: "SST Solid-State", n6_exact: true,  param_a: 3.0, param_b: 0.4 },  // n/φ=3
        Candidate { id: "GR-C4", name: "Superconducting", n6_exact: true,  param_a: 3.0, param_b: 0.2 },  // n/φ=3, loss=0
        Candidate { id: "GR-C5", name: "Multi-Winding",   n6_exact: true,  param_a: 4.0, param_b: 0.5 },  // τ=4 windings
    ]
}

fn grid_hvdc() -> Vec<Candidate> {
    vec![
        Candidate { id: "GR-S1", name: "HVDC ±500kV",   n6_exact: true,  param_a: 500.0,  param_b: 0.5 },  // sopfr·(σ-φ)²
        Candidate { id: "GR-S2", name: "UHVDC ±800kV",  n6_exact: true,  param_a: 800.0,  param_b: 0.3 },  // (σ-τ)·(σ-φ)²
        Candidate { id: "GR-S3", name: "UHVDC ±1100kV", n6_exact: true,  param_a: 1100.0, param_b: 0.2 },  // (σ-μ)·(σ-φ)²
        Candidate { id: "GR-S4", name: "MVDC 48V",      n6_exact: true,  param_a: 0.048,  param_b: 0.9 },  // σ·τ BT-60
    ]
}

fn grid_system() -> Vec<Candidate> {
    vec![
        Candidate { id: "GR-G1", name: "Central Radial",        n6_exact: false, param_a: 1.0, param_b: 0.9 },
        Candidate { id: "GR-G2", name: "Microgrid 24-node",     n6_exact: true,  param_a: 3.0, param_b: 0.5 },  // J₂ nodes
        Candidate { id: "GR-G3", name: "Supergrid Intercontinental", n6_exact: true, param_a: 4.0, param_b: 0.2 }, // σ links
        Candidate { id: "GR-G4", name: "Hybrid AC/DC Egyptian", n6_exact: true,  param_a: 3.5, param_b: 0.4 },  // Egyptian
    ]
}

// ═══════════════════════════════════════════════════════════════════════
// 성능 점수 계산 함수 (도메인별)
// ═══════════════════════════════════════════════════════════════════════

/// Fusion: energy_mev × efficiency × power_mw
fn fusion_performance(l1: &Candidate, _l2: &Candidate, _l3: &Candidate, l4: &Candidate, l5: &Candidate) -> (f64, f64, f64) {
    let raw_perf = l1.param_a * l4.param_a * l5.param_a;  // MeV × eff × MW
    let max_perf = 22.4 * 0.833 * 1000.0;  // 최대: Li6-D × Egyptian × Central GW
    let perf = raw_perf / max_perf;
    let eff = l4.param_a;  // 변환 효율 직접 사용
    let cost = 1.0 - (l1.param_b * 0.02 + l4.param_b * 0.5 + l5.param_b * 0.48);  // 비용 역전
    (perf.min(1.0), eff.min(1.0), cost.max(0.0).min(1.0))
}

/// Solar: (base_eff + process_boost/100) × cells × (1 + tracking_gain/100)
fn solar_performance(l1: &Candidate, l2: &Candidate, l3: &Candidate, _l4: &Candidate, l5: &Candidate) -> (f64, f64, f64) {
    let eff = l1.param_b + l2.param_a / 100.0;  // base_eff + boost
    let raw_perf = eff * l3.param_a * (1.0 + l5.param_a / 100.0);
    let max_perf = 0.34 * 144.0 * 1.20;  // 최대: GaAs+Tandem, 144-cell, 2-axis+CPV
    let perf = raw_perf / max_perf;
    let cost = (l1.param_b * 0.1 + l2.param_b * 0.3 + l3.param_b * 0.2 + l5.param_b * 0.4).max(0.0).min(1.0);
    (perf.min(1.0), eff.min(1.0), cost)
}

/// Battery: capacity × voltage × solid_state_bonus
fn battery_performance(l1: &Candidate, l2: &Candidate, _l3: &Candidate, l4: &Candidate, _l5: &Candidate) -> (f64, f64, f64) {
    let raw_perf = l1.param_a * l4.param_b * l2.param_a;  // mAh/g × V × form_bonus
    let max_perf = 3860.0 * 768.0 * 1.5;  // Li Metal × 192S × Solid-State
    let perf = raw_perf / max_perf;
    let eff = (l2.param_a - 1.0) * 0.5 + 0.7;  // 공정 효율
    let cost = (l1.param_b * 0.4 + l2.param_b * 0.3 + l4.param_b * 0.01 * 0.3).max(0.0).min(1.0);
    (perf.min(1.0), eff.max(0.0).min(1.0), cost)
}

/// Grid: voltage_kv × (1 - thd/100) × conductor_bonus
fn grid_performance(l1: &Candidate, l2: &Candidate, l3: &Candidate, l4: &Candidate, l5: &Candidate) -> (f64, f64, f64) {
    let thd_factor = 1.0 - l2.param_b / 100.0;
    let raw_perf = l4.param_a * thd_factor * l1.param_a * l5.param_a;
    let max_perf = 1100.0 * 1.0 * 3.0 * 4.0;  // 1100kV × 0% THD × HTS × Supergrid
    let perf = raw_perf / max_perf;
    let eff = thd_factor;
    let cost = (l1.param_b * 0.3 + l3.param_b * 0.2 + l4.param_b * 0.3 + l5.param_b * 0.2).max(0.0).min(1.0);
    (perf.min(1.0), eff.min(1.0), cost)
}

// ═══════════════════════════════════════════════════════════════════════
// DSE 엔진 — 전수 탐색
// ═══════════════════════════════════════════════════════════════════════

fn run_domain_dse(
    name: &str,
    code: &str,
    levels: [&Vec<Candidate>; 5],
    perf_fn: fn(&Candidate, &Candidate, &Candidate, &Candidate, &Candidate) -> (f64, f64, f64),
) -> Vec<DseResult> {
    let total = levels[0].len() * levels[1].len() * levels[2].len() * levels[3].len() * levels[4].len();

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Domain: {} ({})  ", name, code);
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Total combinations: {:>6}", total);
    println!();

    let mut results: Vec<DseResult> = Vec::with_capacity(total);

    for (i0, c0) in levels[0].iter().enumerate() {
        for (i1, c1) in levels[1].iter().enumerate() {
            for (i2, c2) in levels[2].iter().enumerate() {
                for (i3, c3) in levels[3].iter().enumerate() {
                    for (i4, c4) in levels[4].iter().enumerate() {
                        // n6 EXACT 카운트
                        let n6_count = [c0, c1, c2, c3, c4].iter()
                            .filter(|c| c.n6_exact).count() as u32;
                        let n6_score = n6_count as f64 / 5.0;

                        // 도메인별 성능/효율/비용
                        let (perf, eff, cost) = perf_fn(c0, c1, c2, c3, c4);

                        let total_score = W_N6 * n6_score
                            + W_PERF * perf
                            + W_EFF * eff
                            + W_COST * cost;

                        results.push(DseResult {
                            ids: [i0, i1, i2, i3, i4],
                            n6_exact_count: n6_count,
                            n6_score,
                            performance: perf,
                            efficiency: eff,
                            cost,
                            total_score,
                        });
                    }
                }
            }
        }
    }

    // 정렬: total_score 내림차순
    results.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

    // Top 10 출력
    println!("  Top 10 Pareto:");
    println!("  {}", "-".repeat(108));
    println!("  | {:>4} | {:^16} | {:^14} | {:^14} | {:^14} | {:^14} | {:>4} | {:>6} | {:>6} | {:>6} |",
        "Rank", "소재(L1)", "공정(L2)", "코어(L3)", "시스템(L4)", "그리드(L5)", "EXCT", "Score", "Perf", "Eff");
    println!("  {}", "-".repeat(108));

    for (rank, r) in results.iter().take(10).enumerate() {
        println!("  | {:>4} | {:^16} | {:^14} | {:^14} | {:^14} | {:^14} | {:>1}/5  | {:>5.3} | {:>5.3} | {:>5.3} |",
            rank + 1,
            levels[0][r.ids[0]].id,
            levels[1][r.ids[1]].id,
            levels[2][r.ids[2]].id,
            levels[3][r.ids[3]].id,
            levels[4][r.ids[4]].id,
            r.n6_exact_count,
            r.total_score,
            r.performance,
            r.efficiency,
        );
    }
    println!("  {}", "-".repeat(108));

    // 통계
    let total_f = results.len() as f64;
    let pct_100 = results.iter().filter(|r| r.n6_exact_count == 5).count() as f64 / total_f * 100.0;
    let pct_80  = results.iter().filter(|r| r.n6_exact_count >= 4).count() as f64 / total_f * 100.0;
    let pct_60  = results.iter().filter(|r| r.n6_exact_count >= 3).count() as f64 / total_f * 100.0;

    println!();
    println!("  Statistics:");
    println!("    100% EXACT (5/5): {:>6.1}% ({} combinations)", pct_100,
        results.iter().filter(|r| r.n6_exact_count == 5).count());
    println!("    ≥80% EXACT (4/5): {:>6.1}% ({} combinations)", pct_80,
        results.iter().filter(|r| r.n6_exact_count >= 4).count());
    println!("    ≥60% EXACT (3/5): {:>6.1}% ({} combinations)", pct_60,
        results.iter().filter(|r| r.n6_exact_count >= 3).count());

    results
}

// ═══════════════════════════════════════════════════════════════════════
// CROSS-DSE: 궁극의 에너지 통합
// ═══════════════════════════════════════════════════════════════════════

#[derive(Clone)]
struct CrossDseResult {
    domain_ranks: [usize; 4],  // 각 도메인 top-N 인덱스
    domain_scores: [f64; 4],
    total_n6_exact: u32,
    total_score: f64,
}

fn run_cross_dse(
    fusion_top: &[DseResult],
    solar_top: &[DseResult],
    battery_top: &[DseResult],
    grid_top: &[DseResult],
    // 후보 테이블 (이름 출력용)
    fu_levels: [&Vec<Candidate>; 5],
    sl_levels: [&Vec<Candidate>; 5],
    bt_levels: [&Vec<Candidate>; 5],
    gr_levels: [&Vec<Candidate>; 5],
) {
    let top_n = 5;
    let fu = &fusion_top[..top_n.min(fusion_top.len())];
    let sl = &solar_top[..top_n.min(solar_top.len())];
    let bt = &battery_top[..top_n.min(battery_top.len())];
    let gr = &grid_top[..top_n.min(grid_top.len())];

    let total_cross = fu.len() * sl.len() * bt.len() * gr.len();

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  CROSS-DSE: 궁극의 에너지 통합");
    println!("═══════════════════════════════════════════════════════════════");
    println!("  Domain top-{} winners × 4 domains = {} combinations", top_n, total_cross);
    println!();

    let mut cross_results: Vec<CrossDseResult> = Vec::with_capacity(total_cross);

    for (fi, fr) in fu.iter().enumerate() {
        for (si, sr) in sl.iter().enumerate() {
            for (bi, br) in bt.iter().enumerate() {
                for (gi, grd) in gr.iter().enumerate() {
                    let total_n6 = fr.n6_exact_count + sr.n6_exact_count
                        + br.n6_exact_count + grd.n6_exact_count;
                    let total_score = (fr.total_score + sr.total_score
                        + br.total_score + grd.total_score) / 4.0;

                    cross_results.push(CrossDseResult {
                        domain_ranks: [fi, si, bi, gi],
                        domain_scores: [fr.total_score, sr.total_score, br.total_score, grd.total_score],
                        total_n6_exact: total_n6,
                        total_score,
                    });
                }
            }
        }
    }

    cross_results.sort_by(|a, b| b.total_score.partial_cmp(&a.total_score).unwrap());

    // Top 10 Cross-DSE
    println!("  Top 10 Cross-DSE Pareto:");
    println!("  {}", "-".repeat(120));
    println!("  | {:>4} | {:^22} | {:^22} | {:^22} | {:^22} | {:>5} | {:>7} |",
        "Rank", "Fusion Path", "Solar Path", "Battery Path", "Grid Path", "EXACT", "Score");
    println!("  {}", "-".repeat(120));

    for (rank, cr) in cross_results.iter().take(10).enumerate() {
        let fu_r = &fu[cr.domain_ranks[0]];
        let sl_r = &sl[cr.domain_ranks[1]];
        let bt_r = &bt[cr.domain_ranks[2]];
        let gr_r = &gr[cr.domain_ranks[3]];

        let fu_path = format!("{}→{}", fu_levels[0][fu_r.ids[0]].id, fu_levels[3][fu_r.ids[3]].id);
        let sl_path = format!("{}→{}", sl_levels[0][sl_r.ids[0]].id, sl_levels[3][sl_r.ids[3]].id);
        let bt_path = format!("{}→{}", bt_levels[0][bt_r.ids[0]].id, bt_levels[3][bt_r.ids[3]].id);
        let gr_path = format!("{}→{}", gr_levels[0][gr_r.ids[0]].id, gr_levels[3][gr_r.ids[3]].id);

        println!("  | {:>4} | {:^22} | {:^22} | {:^22} | {:^22} | {:>2}/20 | {:>6.4} |",
            rank + 1, fu_path, sl_path, bt_path, gr_path,
            cr.total_n6_exact, cr.total_score);
    }
    println!("  {}", "-".repeat(120));

    // ═══ ULTIMATE WINNER ═══
    let winner = &cross_results[0];
    let fu_w = &fu[winner.domain_ranks[0]];
    let sl_w = &sl[winner.domain_ranks[1]];
    let bt_w = &bt[winner.domain_ranks[2]];
    let gr_w = &gr[winner.domain_ranks[3]];

    println!();
    println!("═══════════════════════════════════════════════════════════════");
    println!("  ULTIMATE ENERGY WINNER — 궁극의 에너지 아키텍처");
    println!("═══════════════════════════════════════════════════════════════");
    println!();
    println!("  Total Score: {:.4}  |  n6 EXACT: {}/20  |  n6 Coverage: {:.1}%",
        winner.total_score, winner.total_n6_exact, winner.total_n6_exact as f64 / 20.0 * 100.0);
    println!();

    // Fusion 상세
    println!("  ┌─── FUSION ─────────────────────────────────────────────┐");
    println!("  │  Score: {:.4}  n6: {}/5                                │", fu_w.total_score, fu_w.n6_exact_count);
    println!("  │  L1 소재:  {} ({})  {:>16}│", fu_levels[0][fu_w.ids[0]].id, fu_levels[0][fu_w.ids[0]].name, "");
    println!("  │  L2 공정:  {} ({})  {:>16}│", fu_levels[1][fu_w.ids[1]].id, fu_levels[1][fu_w.ids[1]].name, "");
    println!("  │  L3 코어:  {} ({})  {:>16}│", fu_levels[2][fu_w.ids[2]].id, fu_levels[2][fu_w.ids[2]].name, "");
    println!("  │  L4 변환:  {} ({})  {:>16}│", fu_levels[3][fu_w.ids[3]].id, fu_levels[3][fu_w.ids[3]].name, "");
    println!("  │  L5 계통:  {} ({})  {:>16}│", fu_levels[4][fu_w.ids[4]].id, fu_levels[4][fu_w.ids[4]].name, "");
    println!("  └────────────────────────────────────────────────────────┘");

    // Solar 상세
    println!("  ┌─── SOLAR ──────────────────────────────────────────────┐");
    println!("  │  Score: {:.4}  n6: {}/5                                │", sl_w.total_score, sl_w.n6_exact_count);
    println!("  │  L1 소재:  {} ({})  {:>16}│", sl_levels[0][sl_w.ids[0]].id, sl_levels[0][sl_w.ids[0]].name, "");
    println!("  │  L2 공정:  {} ({})  {:>16}│", sl_levels[1][sl_w.ids[1]].id, sl_levels[1][sl_w.ids[1]].name, "");
    println!("  │  L3 코어:  {} ({})  {:>16}│", sl_levels[2][sl_w.ids[2]].id, sl_levels[2][sl_w.ids[2]].name, "");
    println!("  │  L4 인버터: {} ({})  {:>16}│", sl_levels[3][sl_w.ids[3]].id, sl_levels[3][sl_w.ids[3]].name, "");
    println!("  │  L5 플랜트: {} ({})  {:>16}│", sl_levels[4][sl_w.ids[4]].id, sl_levels[4][sl_w.ids[4]].name, "");
    println!("  └────────────────────────────────────────────────────────┘");

    // Battery 상세
    println!("  ┌─── BATTERY ────────────────────────────────────────────┐");
    println!("  │  Score: {:.4}  n6: {}/5                                │", bt_w.total_score, bt_w.n6_exact_count);
    println!("  │  L1 소재:  {} ({})  {:>16}│", bt_levels[0][bt_w.ids[0]].id, bt_levels[0][bt_w.ids[0]].name, "");
    println!("  │  L2 공정:  {} ({})  {:>16}│", bt_levels[1][bt_w.ids[1]].id, bt_levels[1][bt_w.ids[1]].name, "");
    println!("  │  L3 코어:  {} ({})  {:>16}│", bt_levels[2][bt_w.ids[2]].id, bt_levels[2][bt_w.ids[2]].name, "");
    println!("  │  L4 팩:    {} ({})  {:>16}│", bt_levels[3][bt_w.ids[3]].id, bt_levels[3][bt_w.ids[3]].name, "");
    println!("  │  L5 시스템: {} ({})  {:>16}│", bt_levels[4][bt_w.ids[4]].id, bt_levels[4][bt_w.ids[4]].name, "");
    println!("  └────────────────────────────────────────────────────────┘");

    // Grid 상세
    println!("  ┌─── GRID ───────────────────────────────────────────────┐");
    println!("  │  Score: {:.4}  n6: {}/5                                │", gr_w.total_score, gr_w.n6_exact_count);
    println!("  │  L1 소재:  {} ({})  {:>16}│", gr_levels[0][gr_w.ids[0]].id, gr_levels[0][gr_w.ids[0]].name, "");
    println!("  │  L2 변환:  {} ({})  {:>16}│", gr_levels[1][gr_w.ids[1]].id, gr_levels[1][gr_w.ids[1]].name, "");
    println!("  │  L3 변압:  {} ({})  {:>16}│", gr_levels[2][gr_w.ids[2]].id, gr_levels[2][gr_w.ids[2]].name, "");
    println!("  │  L4 HVDC:  {} ({})  {:>16}│", gr_levels[3][gr_w.ids[3]].id, gr_levels[3][gr_w.ids[3]].name, "");
    println!("  │  L5 계통:  {} ({})  {:>16}│", gr_levels[4][gr_w.ids[4]].id, gr_levels[4][gr_w.ids[4]].name, "");
    println!("  └────────────────────────────────────────────────────────┘");

    // N6 상수 요약
    println!();
    println!("  ┌─── N6 상수 매핑 ───────────────────────────────────────┐");
    println!("  │  n=6  σ=12  τ=4  φ=2  sopfr=5  J₂=24  μ=1          │");
    println!("  │  σ·φ·τ = 12·2·4 = 96  → BT-84 triple convergence    │");
    println!("  │  σ(6)·φ(6) = n·τ(6) → 12·2 = 6·4 = 24 ✓            │");
    println!("  │  Egyptian: 1/2 + 1/3 + 1/6 = 1 ✓                    │");
    println!("  └────────────────────────────────────────────────────────┘");
    println!();
}

// ═══════════════════════════════════════════════════════════════════════
// main
// ═══════════════════════════════════════════════════════════════════════

fn main() {
    println!("╔═══════════════════════════════════════════════════════════════╗");
    println!("║  N6 Energy DSE — 궁극의 에너지 설계공간 전수탐색              ║");
    println!("║  σ(n)·φ(n) = n·τ(n) ⟺ n = 6                               ║");
    println!("║  4 Domains × 5 Levels = 9,600 total + 625 Cross-DSE         ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");
    println!();
    println!("  N6 Constants: n={} σ={} τ={} φ={} sopfr={} J₂={} μ={}",
        N, SIGMA, TAU, PHI, SOPFR, J2, MU);
    println!("  Weights: n6={:.0}% perf={:.0}% eff={:.0}% cost={:.0}%",
        W_N6 * 100.0, W_PERF * 100.0, W_EFF * 100.0, W_COST * 100.0);

    // ── 후보 데이터 생성 ──
    let fu_m = fusion_materials();
    let fu_p = fusion_confinement();
    let fu_c = fusion_core();
    let fu_s = fusion_conversion();
    let fu_g = fusion_grid();

    let sl_m = solar_materials();
    let sl_p = solar_process();
    let sl_c = solar_module();
    let sl_s = solar_inverter();
    let sl_g = solar_plant();

    let bt_m = battery_materials();
    let bt_p = battery_process();
    let bt_c = battery_module();
    let bt_s = battery_pack();
    let bt_g = battery_system();

    let gr_m = grid_materials();
    let gr_p = grid_conversion();
    let gr_c = grid_transformer();
    let gr_s = grid_hvdc();
    let gr_g = grid_system();

    // ── 4 도메인 DSE 실행 ──
    let fu_levels: [&Vec<Candidate>; 5] = [&fu_m, &fu_p, &fu_c, &fu_s, &fu_g];
    let sl_levels: [&Vec<Candidate>; 5] = [&sl_m, &sl_p, &sl_c, &sl_s, &sl_g];
    let bt_levels: [&Vec<Candidate>; 5] = [&bt_m, &bt_p, &bt_c, &bt_s, &bt_g];
    let gr_levels: [&Vec<Candidate>; 5] = [&gr_m, &gr_p, &gr_c, &gr_s, &gr_g];

    let fusion_results = run_domain_dse("FUSION", "DSE-FU", fu_levels, fusion_performance);
    let solar_results  = run_domain_dse("SOLAR",  "DSE-SL", sl_levels, solar_performance);
    let battery_results = run_domain_dse("BATTERY", "DSE-BT", bt_levels, battery_performance);
    let grid_results   = run_domain_dse("GRID",   "DSE-GR", gr_levels, grid_performance);

    // ── Cross-DSE ──
    let fu_levels2: [&Vec<Candidate>; 5] = [&fu_m, &fu_p, &fu_c, &fu_s, &fu_g];
    let sl_levels2: [&Vec<Candidate>; 5] = [&sl_m, &sl_p, &sl_c, &sl_s, &sl_g];
    let bt_levels2: [&Vec<Candidate>; 5] = [&bt_m, &bt_p, &bt_c, &bt_s, &bt_g];
    let gr_levels2: [&Vec<Candidate>; 5] = [&gr_m, &gr_p, &gr_c, &gr_s, &gr_g];

    run_cross_dse(
        &fusion_results, &solar_results, &battery_results, &grid_results,
        fu_levels2, sl_levels2, bt_levels2, gr_levels2,
    );

    // ── 총계 ──
    let grand_total = 2400 * 4 + 625;
    println!("  Grand Total: {} domain combinations + 625 cross-DSE = {} evaluated", 2400 * 4, grand_total);
    println!();
    println!("═══ DSE Complete ═══");
}
