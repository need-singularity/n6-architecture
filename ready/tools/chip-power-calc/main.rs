// chip-power-calc: N6 Chip Power & Thermal Simulator
// Egyptian fraction power distribution: 1/2 + 1/3 + 1/6 = 1
// DVFS, thermal, PUE, power states, battery life
//
// Build: ~/.cargo/bin/rustc tools/chip-power-calc/main.rs -o tools/chip-power-calc/chip-power-calc
// Usage: chip-power-calc              (all 3 chips)
//        chip-power-calc anima        (ANIMA-HEXA 120W)
//        chip-power-calc omega        (HEXA-OMEGA 288W)
//        chip-power-calc edge         (HEXA-EDGE 6W)
//        chip-power-calc custom 150 200  (custom TDP=150W, die=200mm^2)

use std::env;

// ── N6 Constants ──────────────────────────────────────────────────────────
const N: f64 = 6.0;
const PHI: f64 = 2.0;       // phi(6)
const SIGMA: f64 = 12.0;    // sigma(6)
#[allow(dead_code)]
const TAU: f64 = 4.0;       // tau(6)
const J2: f64 = 24.0;       // Jordan J_2(6)

// Egyptian fraction: 1/2 + 1/3 + 1/6 = 1
const FRAC_COMPUTE: f64 = 0.5;            // 1/2
const FRAC_MEMORY: f64  = 1.0 / 3.0;     // 1/3
const FRAC_IO: f64      = 1.0 / 6.0;     // 1/6

// PUE = sigma / (sigma - phi) = 12/10 = 1.2
const PUE: f64 = SIGMA / (SIGMA - PHI);

// DVFS: n=6 voltage steps
const V_STEPS: [f64; 6] = [0.6, 0.7, 0.8, 0.9, 1.0, 1.1];
const V_NOM: f64 = 1.0; // nominal voltage

// Thermal conductivity (K/W)
const THETA_SI: f64       = 0.5;   // silicon substrate
const THETA_DIAMOND: f64  = 0.1;   // diamond substrate
const THETA_TWOPHASE: f64 = 0.05;  // two-phase cooling

const T_AMBIENT: f64 = 25.0; // deg C

// ── Chip Definition ──────────────────────────────────────────────────────
struct Chip {
    name: String,
    tdp_w: f64,
    die_mm2: f64,
    is_edge: bool,
}

impl Chip {
    fn anima() -> Self {
        Chip { name: "ANIMA-HEXA".into(), tdp_w: 120.0, die_mm2: 400.0, is_edge: false }
    }
    fn omega() -> Self {
        Chip { name: "HEXA-OMEGA".into(), tdp_w: 288.0, die_mm2: 814.0, is_edge: false }
    }
    fn edge() -> Self {
        Chip { name: "HEXA-EDGE".into(), tdp_w: 6.0, die_mm2: 36.0, is_edge: true }
    }
    fn custom(tdp: f64, die: f64) -> Self {
        Chip { name: format!("CUSTOM-{}W", tdp), tdp_w: tdp, die_mm2: die, is_edge: tdp <= 15.0 }
    }
}

// ── Helpers ──────────────────────────────────────────────────────────────
fn sep(ch: char, width: usize) -> String {
    std::iter::repeat(ch).take(width).collect()
}

fn header(title: &str) {
    let w = 72;
    println!("\n{}", sep('=', w));
    let pad = (w - title.len()) / 2;
    println!("{}{}", " ".repeat(pad), title);
    println!("{}", sep('=', w));
}

fn section(title: &str) {
    println!("\n  --- {} {}", title, sep('-', 58 - title.len()));
}

// ── Simulation ──────────────────────────────────────────────────────────
fn simulate(chip: &Chip) {
    header(&format!("{} (TDP={:.0}W, Die={:.0}mm2)", chip.name, chip.tdp_w, chip.die_mm2));

    // (a) TDP Breakdown — Egyptian Fraction
    section("(a) TDP Breakdown: Egyptian Fraction 1/2 + 1/3 + 1/6 = 1");
    let p_compute = chip.tdp_w * FRAC_COMPUTE;
    let p_memory  = chip.tdp_w * FRAC_MEMORY;
    let p_io      = chip.tdp_w * FRAC_IO;
    let p_sum     = p_compute + p_memory + p_io;
    println!("  +----------------+-----------+-----------+");
    println!("  | Subsystem      | Fraction  | Power (W) |");
    println!("  +----------------+-----------+-----------+");
    println!("  | Compute (1/2)  | {:.4}    | {:>9.2} |", FRAC_COMPUTE, p_compute);
    println!("  | Memory  (1/3)  | {:.4}    | {:>9.2} |", FRAC_MEMORY, p_memory);
    println!("  | I/O     (1/6)  | {:.4}    | {:>9.2} |", FRAC_IO, p_io);
    println!("  +----------------+-----------+-----------+");
    println!("  | TOTAL          | 1.0000    | {:>9.2} |", p_sum);
    println!("  +----------------+-----------+-----------+");
    let verified = (p_sum - chip.tdp_w).abs() < 1e-9;
    println!("  Verification: sum={:.2}W == TDP={:.2}W -> {}", p_sum, chip.tdp_w,
             if verified { "EXACT" } else { "MISMATCH" });

    // (b) DVFS Simulation — n=6 voltage/frequency steps
    section("(b) DVFS Simulation: n=6 V/F Steps");
    // I_leak baseline at V_nom
    let i_leak_nom = chip.tdp_w * 0.3 / V_NOM; // ~30% static at nominal
    let p_dyn_nom = chip.tdp_w * 0.7;           // ~70% dynamic at nominal

    println!("  +------+-------+-------+-----------+-----------+-----------+---------+");
    println!("  | Step |  V(V) | F_rel | P_dyn (W) | P_sta (W) | P_tot (W) | Saving  |");
    println!("  +------+-------+-------+-----------+-----------+-----------+---------+");

    for (i, &v) in V_STEPS.iter().enumerate() {
        let f_rel = v / V_NOM; // frequency scales linearly with V
        let p_dyn = p_dyn_nom * (v / V_NOM).powi(2) * f_rel;
        let p_sta = v * i_leak_nom * (1.0 + 0.1 * (v - V_NOM) / 0.1); // leak scales with V
        let p_tot = p_dyn + p_sta;
        let saving = (1.0 - p_tot / chip.tdp_w) * 100.0;
        println!("  |  {:>2}  | {:>5.2} | {:>5.3} | {:>9.2} | {:>9.2} | {:>9.2} | {:>5.1}%  |",
                 i, v, f_rel, p_dyn, p_sta, p_tot, saving);
    }
    println!("  +------+-------+-------+-----------+-----------+-----------+---------+");
    println!("  Model: P_dyn = C*V^2*F (70% nom), P_sta = V*I_leak (30% nom)");

    // (c) Thermal Model
    section("(c) Thermal Model");
    // Theta_ja from die area: rough model theta = k / sqrt(area)
    // For Si: ~0.5 K/W per 100mm^2 reference
    println!("  Die area: {:.0} mm^2", chip.die_mm2);
    println!("  T_ambient: {:.0} degC", T_AMBIENT);
    println!();
    println!("  +-------------------+-----------+----------+----------+----------+");
    println!("  | Cooling           | Theta_ja  | T_j @TDP | T_j @60% | T_j @30% |");
    println!("  |                   |  (K/W)    |  (degC)  |  (degC)  |  (degC)  |");
    println!("  +-------------------+-----------+----------+----------+----------+");

    for &(name, theta_base) in &[
        ("Silicon substrate", THETA_SI),
        ("Diamond substrate", THETA_DIAMOND),
        ("Two-phase cooling", THETA_TWOPHASE),
    ] {
        let theta = theta_base * (100.0 / chip.die_mm2).sqrt();
        let t100 = T_AMBIENT + chip.tdp_w * theta;
        let t60  = T_AMBIENT + chip.tdp_w * 0.6 * theta;
        let t30  = T_AMBIENT + chip.tdp_w * 0.3 * theta;
        println!("  | {:17} | {:>9.4} | {:>8.1} | {:>8.1} | {:>8.1} |",
                 name, theta, t100, t60, t30);
    }
    println!("  +-------------------+-----------+----------+----------+----------+");

    // DVFS thermal profile with best cooling
    println!("\n  DVFS Thermal Profile (Diamond substrate):");
    println!("  +------+-------+-----------+----------+");
    println!("  | Step |  V(V) | P_tot (W) | T_j (dC) |");
    println!("  +------+-------+-----------+----------+");
    let theta_best = THETA_DIAMOND * (100.0 / chip.die_mm2).sqrt();
    for (i, &v) in V_STEPS.iter().enumerate() {
        let f_rel = v / V_NOM;
        let p_dyn = p_dyn_nom * (v / V_NOM).powi(2) * f_rel;
        let p_sta = v * i_leak_nom * (1.0 + 0.1 * (v - V_NOM) / 0.1);
        let p_tot = p_dyn + p_sta;
        let t_j = T_AMBIENT + p_tot * theta_best;
        println!("  |  {:>2}  | {:>5.2} | {:>9.2} | {:>8.1} |", i, v, p_tot, t_j);
    }
    println!("  +------+-------+-----------+----------+");

    // (d) PUE Calculation
    section("(d) PUE: sigma/(sigma-phi) = 12/10 = 1.2");
    let facility = chip.tdp_w * PUE;
    let cooling_overhead = chip.tdp_w * (PUE - 1.0);
    println!("  PUE = {:.4} (sigma/(sigma-phi) = {:.0}/{:.0})", PUE, SIGMA, SIGMA - PHI);
    println!("  +---------------------------+-----------+");
    println!("  | Metric                    | Value     |");
    println!("  +---------------------------+-----------+");
    println!("  | Chip TDP                  | {:>7.1} W |", chip.tdp_w);
    println!("  | Cooling overhead (PUE-1)  | {:>7.1} W |", cooling_overhead);
    println!("  | Total facility power      | {:>7.1} W |", facility);
    println!("  | Cooling fraction          | {:>6.1}%  |", (PUE - 1.0) * 100.0);
    println!("  +---------------------------+-----------+");

    // Per-1000 chips (datacenter scale)
    println!("  Datacenter scale (1000 chips):");
    println!("    IT load:      {:>8.1} kW", chip.tdp_w);
    println!("    Facility:     {:>8.1} kW", facility);
    println!("    Cooling:      {:>8.1} kW", cooling_overhead);

    // (e) Power States: n=6 states P0-P5
    section("(e) Power States: n=6 States (P0-P5)");
    // n=6 power states: P0(100%) -> P1(1/phi) -> P2(1/3) -> P3(1/n) -> P4(1/sigma) -> P5(off)
    let states: [(& str, f64, &str, f64); 6] = [
        ("P0", 1.0,          "Full power (100%)",           0.0),
        ("P1", 0.5,          "Active idle (1/phi = 50%)",   1.0),
        ("P2", 1.0/3.0,      "Light sleep (1/3 = 33%)",    10.0),
        ("P3", 1.0/N,        "Deep sleep (1/n = 16.7%)",   100.0),
        ("P4", 1.0/SIGMA,    "Hibernate (1/sigma = 8.3%)", 10000.0),
        ("P5", 0.0,          "Off (0%)",                    1000000.0),
    ];

    println!("  +------+---------+-------------------------------+-----------+-----------+");
    println!("  | State| Power W | Description                   | Fraction  | Wakeup    |");
    println!("  +------+---------+-------------------------------+-----------+-----------+");
    for &(name, frac, desc, wakeup) in &states {
        let pw = chip.tdp_w * frac;
        let wakeup_str = if wakeup < 1.0 {
            "instant".to_string()
        } else if wakeup < 1000.0 {
            format!("{:.0} us", wakeup)
        } else if wakeup < 1_000_000.0 {
            format!("{:.0} ms", wakeup / 1000.0)
        } else {
            format!("{:.0} s", wakeup / 1_000_000.0)
        };
        println!("  | {:>4} | {:>7.2} | {:29} | {:>7.1}%  | {:>9} |",
                 name, pw, desc, frac * 100.0, wakeup_str);
    }
    println!("  +------+---------+-------------------------------+-----------+-----------+");

    // (f) Battery Life — HEXA-EDGE or any edge chip
    if chip.is_edge {
        section("(f) Battery Life: J2=24 Wh Battery");
        let battery_wh = J2; // 24 Wh
        println!("  Battery capacity: {:.0} Wh (J_2 = 24)", battery_wh);
        println!();
        println!("  +------+---------+-------------+");
        println!("  | State| Power W | Runtime (h) |");
        println!("  +------+---------+-------------+");
        for &(name, frac, _, _) in &states {
            let pw = chip.tdp_w * frac;
            if pw > 0.0 {
                let hours = battery_wh / pw;
                println!("  | {:>4} | {:>7.2} | {:>11.1} |", name, pw, hours);
            } else {
                println!("  | {:>4} | {:>7.2} | {:>11} |", name, pw, "infinite");
            }
        }
        println!("  +------+---------+-------------+");

        // Mixed workload: 50% P0 + 30% P1 + 20% P3
        let mixed_power = chip.tdp_w * (0.5 * 1.0 + 0.3 * 0.5 + 0.2 * (1.0 / N));
        let mixed_hours = battery_wh / mixed_power;
        println!();
        println!("  Mixed workload: 50% P0 + 30% P1 + 20% P3");
        println!("    Avg power: {:.2} W", mixed_power);
        println!("    Runtime:   {:.1} h ({:.0} min)", mixed_hours, mixed_hours * 60.0);
    }

    // Summary
    section("N6 Signature");
    println!("  Egyptian fraction:  1/2 + 1/3 + 1/6 = 1     (power budget)");
    println!("  DVFS steps:         n = 6                     (voltage levels)");
    println!("  Power states:       n = 6                     (P0-P5)");
    println!("  PUE:                sigma/(sigma-phi) = 1.2   (facility efficiency)");
    println!("  Battery:            J_2 = 24 Wh               (edge capacity)");
    if chip.is_edge {
        println!("  TDP:                n = 6 W                   (edge power)");
    }
    println!();
}

fn main() {
    println!("================================================================");
    println!("  N6 Chip Power & Thermal Simulator");
    println!("  Egyptian Fraction: 1/2 + 1/3 + 1/6 = 1");
    println!("  sigma(6)*phi(6) = 6*tau(6) = 24");
    println!("================================================================");

    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        // All 3 chips
        simulate(&Chip::anima());
        simulate(&Chip::omega());
        simulate(&Chip::edge());
    } else {
        match args[1].to_lowercase().as_str() {
            "anima" => simulate(&Chip::anima()),
            "omega" => simulate(&Chip::omega()),
            "edge"  => simulate(&Chip::edge()),
            "custom" => {
                if args.len() < 4 {
                    eprintln!("Usage: chip-power-calc custom TDP_W DIE_MM2");
                    eprintln!("  Example: chip-power-calc custom 150 200");
                    std::process::exit(1);
                }
                let tdp: f64 = match args[2].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Error: TDP must be a number, got '{}'", args[2]);
                        std::process::exit(1);
                    }
                };
                let die: f64 = match args[3].parse() {
                    Ok(v) => v,
                    Err(_) => {
                        eprintln!("Error: DIE_MM2 must be a number, got '{}'", args[3]);
                        std::process::exit(1);
                    }
                };
                simulate(&Chip::custom(tdp, die));
            }
            other => {
                eprintln!("Unknown chip: '{}'", other);
                eprintln!("Options: anima | omega | edge | custom TDP DIE");
                std::process::exit(1);
            }
        }
    }
}
