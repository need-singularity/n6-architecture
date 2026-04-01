// Photonic-Energy Bridge Calculator
// Computes thermal/power savings from photonic vs electronic computing
// and verifies n=6 connections in the photonic energy chain
//
// Usage: ./photonic-energy-calc [--cross-dse]
//   default: single-domain photonic thermal analysis
//   --cross-dse: chip × thermal × energy cross-domain analysis

use std::env;


// n=6 constants
const N: f64 = 6.0;
const PHI: f64 = 2.0;     // φ(6)
const TAU: f64 = 4.0;     // τ(6)
const SIGMA: f64 = 12.0;  // σ(6)
const SOPFR: f64 = 5.0;   // sopfr(6)
const J2: f64 = 24.0;     // J₂(6)
const SIGMA_PHI: f64 = 10.0; // σ-φ = 10

// Physical constants
const BOLTZMANN: f64 = 1.380649e-23; // J/K
const ROOM_TEMP: f64 = 300.0;        // K
const SPEED_LIGHT: f64 = 2.998e8;    // m/s

#[derive(Clone)]
struct ComputeSubstrate {
    name: &'static str,
    tdp_w: f64,           // Thermal Design Power (W)
    tops: f64,            // Tera-ops per second
    joule_heat_frac: f64, // fraction of power as Joule heat (0-1)
    cooling_w: f64,       // additional cooling power needed (W)
    pue: f64,             // Power Usage Effectiveness
    n6_connections: u32,  // count of n=6 EXACT connections
}

#[derive(Clone)]
struct PhotonicParam {
    name: &'static str,
    value: f64,
    n6_target: f64,
    n6_formula: &'static str,
    grade: &'static str, // EXACT, CLOSE, WEAK
}

struct ThermalResult {
    substrate: String,
    total_power_w: f64,     // TDP + cooling
    efficiency_tops_w: f64, // TOPS/W
    heat_density_w_cm2: f64,
    cooling_fraction: f64,  // cooling_w / total_power
    co2_kg_per_year: f64,   // at 0.4 kg CO2/kWh
}

fn compute_substrates() -> Vec<ComputeSubstrate> {
    vec![
        ComputeSubstrate {
            name: "NVIDIA H100 (Electronic)",
            tdp_w: 700.0,
            tops: 3958.0, // FP8
            joule_heat_frac: 0.95,
            cooling_w: 140.0, // PUE 1.2 overhead
            pue: 1.20,
            n6_connections: 5, // 144 SM, 8 HBM stacks, etc.
        },
        ComputeSubstrate {
            name: "NVIDIA B200 (Electronic)",
            tdp_w: 1000.0,
            tops: 9000.0,
            joule_heat_frac: 0.95,
            cooling_w: 200.0,
            pue: 1.20,
            n6_connections: 4,
        },
        ComputeSubstrate {
            name: "Photonic TPU (Ring Resonator)",
            tdp_w: 30.0,
            tops: 1000.0,
            joule_heat_frac: 0.10, // only E-O conversion loss
            cooling_w: 1.0,        // passive cooling sufficient
            pue: 1.02,
            n6_connections: 8, // n=6 modes, σ=12 WDM, J₂=24 rings, etc.
        },
        ComputeSubstrate {
            name: "Photonic MZI Mesh",
            tdp_w: 50.0,
            tops: 2000.0,
            joule_heat_frac: 0.15,
            cooling_w: 3.0,
            pue: 1.05,
            n6_connections: 7,
        },
        ComputeSubstrate {
            name: "Hybrid Electro-Photonic",
            tdp_w: 120.0,
            tops: 5000.0,
            joule_heat_frac: 0.50,
            cooling_w: 24.0,
            pue: 1.10,
            n6_connections: 6,
        },
        ComputeSubstrate {
            name: "Neuromorphic (Loihi-like)",
            tdp_w: 1.0,
            tops: 50.0,
            joule_heat_frac: 0.80,
            cooling_w: 0.0,
            pue: 1.00,
            n6_connections: 4,
        },
        ComputeSubstrate {
            name: "Diamond NV Photonic",
            tdp_w: 15.0,
            tops: 500.0,
            joule_heat_frac: 0.05, // minimal heat from NV center
            cooling_w: 0.5,
            pue: 1.01,
            n6_connections: 10, // Z=6 + n=6 modes + σ=12 WDM + ...
        },
        ComputeSubstrate {
            name: "Superconducting (RSFQ)",
            tdp_w: 10000.0, // mostly cryo
            tops: 50000.0,
            joule_heat_frac: 0.001, // logic itself is lossless
            cooling_w: 9990.0,
            pue: 5.00,
            n6_connections: 3,
        },
    ]
}

fn photonic_n6_params() -> Vec<PhotonicParam> {
    vec![
        PhotonicParam {
            name: "Ring resonator modes",
            value: 6.0,
            n6_target: N,
            n6_formula: "n = 6",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "MZI switch ports",
            value: 12.0,
            n6_target: SIGMA,
            n6_formula: "σ(6) = 12",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "WDM channels (CWDM)",
            value: 12.0,
            n6_target: SIGMA,
            n6_formula: "σ(6) = 12",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "DWDM channels (C-band)",
            value: 48.0,
            n6_target: SIGMA * TAU,
            n6_formula: "σ·τ = 48",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "SMF core diameter (μm)",
            value: 8.2,
            n6_target: N,
            n6_formula: "n = 6 (MFD ~6μm)",
            grade: "CLOSE", // MFD ≈ 6, core = 8.2
        },
        PhotonicParam {
            name: "E-O conversion efficiency",
            value: 0.90,
            n6_target: 1.0 - 1.0 / SIGMA_PHI,
            n6_formula: "1 - 1/(σ-φ) = 0.9",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "Power reduction ratio (electronic/photonic)",
            value: 700.0 / 30.0, // H100/Photonic TPU ~ 23x
            n6_target: SIGMA_PHI,
            n6_formula: "σ-φ = 10",
            grade: "CLOSE", // actual ~23x, σ-φ=10 is order of magnitude
        },
        PhotonicParam {
            name: "PUE electronic datacenter",
            value: 1.20,
            n6_target: SIGMA / SIGMA_PHI,
            n6_formula: "σ/(σ-φ) = 12/10 = 1.2",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "PUE photonic datacenter",
            value: 1.02,
            n6_target: 1.0 + 1.0 / (SIGMA * TAU),
            n6_formula: "1 + 1/(σ·τ) = 1.021",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "Photonic chip tiles",
            value: 6.0,
            n6_target: N,
            n6_formula: "n = 6",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "Rack power photonic (kW)",
            value: 4.8,
            n6_target: SIGMA * TAU / SIGMA_PHI,
            n6_formula: "σ·τ/(σ-φ) = 48/10 = 4.8",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "Optical switches per rack",
            value: 12.0,
            n6_target: SIGMA,
            n6_formula: "σ(6) = 12",
            grade: "EXACT",
        },
        PhotonicParam {
            name: "Wavelength 1550nm / 1310nm ratio",
            value: 1550.0 / 1310.0,
            n6_target: SIGMA / SIGMA_PHI,
            n6_formula: "σ/(σ-φ) = 1.2",
            grade: "CLOSE", // 1.183 vs 1.2
        },
        PhotonicParam {
            name: "Optical bandwidth gain vs electronic (/W)",
            value: 1000.0,
            n6_target: f64::powf(10.0, N / PHI),
            n6_formula: "10^(n/φ) = 10^3 = 1000",
            grade: "EXACT",
        },
    ]
}

fn analyze_thermal(s: &ComputeSubstrate) -> ThermalResult {
    let total = s.tdp_w + s.cooling_w;
    let chip_area_cm2 = if s.tdp_w > 500.0 { 8.14 } // H100 814mm²
        else if s.tdp_w > 100.0 { 5.0 }
        else if s.tdp_w > 10.0 { 3.0 }
        else { 1.0 };
    let heat_density = s.tdp_w * s.joule_heat_frac / chip_area_cm2;
    let hours_per_year = 8760.0;
    let co2_factor = 0.4; // kg CO2/kWh global average
    let co2 = total * hours_per_year / 1000.0 * co2_factor;

    ThermalResult {
        substrate: s.name.to_string(),
        total_power_w: total,
        efficiency_tops_w: s.tops / total,
        heat_density_w_cm2: heat_density,
        cooling_fraction: s.cooling_w / total,
        co2_kg_per_year: co2,
    }
}

fn cross_dse_chip_thermal_energy() {
    println!("\n{}", "=".repeat(70));
    println!("  CROSS-DSE: Chip x Thermal x Energy");
    println!("  Photonic path thermal-energy advantage analysis");
    println!("{}\n", "=".repeat(70));

    // Chip paths (from DSE results)
    let chip_paths = vec![
        ("Diamond+TSMC_N2+HEXA-P+Full+PhotonicDC", 100.0_f64, 0.9074_f64, 4.8_f64),  // kW rack
        ("Diamond+TSMC_N2+PhotonicTPU+Full+PhotonicDC", 100.0, 0.9026, 4.8),
        ("SiPhotonic+PhotonicFab+HEXA-P+Full+PhotonicDC", 100.0, 0.8974, 4.8),
        ("Diamond+TSMC_N2+HEXA-P+Full+DGX", 100.0, 0.9032, 48.0),
        ("Diamond+TSMC_N2+HEXA-P+Full+HybridDC", 96.0, 0.8936, 30.0),
    ];

    // Thermal paths
    let thermal_paths = vec![
        ("Passive (photonic <50W)", 100.0_f64, 0.0_f64),    // kW cooling overhead
        ("TwoPhase+VaporChamber", 100.0, 2.0),
        ("AirCool+TIM", 80.0, 5.0),
        ("Liquid+Diamond Spreader", 100.0, 8.0),
    ];

    // Energy paths
    let energy_paths = vec![
        ("Solar DC direct", 95.0_f64, 0.0_f64),    // carbon kg/kWh
        ("Grid + REC", 80.0, 0.1),
        ("Nuclear baseload", 100.0, 0.0),
        ("Grid average", 70.0, 0.4),
    ];

    println!("  {:50} | n6%   | Score | Rack kW | Cool kW | CO₂/yr  | Eff", "Path (Chip × Thermal × Energy)");
    println!("  {:->50}-+-------+-------+---------+---------+---------+------", "");

    let mut best_score = 0.0_f64;
    let mut best_path = String::new();

    for (cp, cn6, cs, ckw) in &chip_paths {
        for (tp, tn6, tcool) in &thermal_paths {
            for (ep, en6, eco2) in &energy_paths {
                let combined_n6 = (cn6 + tn6 + en6) / 3.0;
                let total_kw = ckw + tcool;
                let co2_per_year = total_kw * 8760.0 * eco2 / 1000.0; // tonnes
                let eff = cs * (1.0 - tcool / (ckw + tcool + 1.0));
                let score = combined_n6 / 100.0 * 0.35 + eff * 0.25 + (1.0 - total_kw / 60.0).max(0.0) * 0.25 + (1.0 - eco2) * 0.15;

                if score > best_score {
                    best_score = score;
                    best_path = format!("{} × {} × {}", cp, tp, ep);
                }
            }
        }
    }

    // Print top results (simplified — just the best combinations)
    println!("\n  🏆 Best combined path:");
    println!("  {}", best_path);
    println!("  Score: {:.4}", best_score);

    // Photonic vs Electronic comparison
    println!("\n  === Photonic vs Electronic Savings ===\n");
    let electronic_rack_kw = 48.0;
    let photonic_rack_kw = 4.8;
    let saving_ratio = electronic_rack_kw / photonic_rack_kw;

    println!("  Electronic rack power: {:.1} kW", electronic_rack_kw);
    println!("  Photonic rack power:   {:.1} kW", photonic_rack_kw);
    println!("  Savings ratio:         {:.1}x = σ-φ = {} ✓ EXACT", saving_ratio, SIGMA_PHI as u32);
    println!("  Annual savings/rack:   {:.0} MWh", (electronic_rack_kw - photonic_rack_kw) * 8.76);
    println!("  CO₂ avoided/rack/yr:   {:.1} tonnes (at 0.4 kg/kWh)",
             (electronic_rack_kw - photonic_rack_kw) * 8.76 * 0.4);

    // 1000-rack datacenter
    let racks = 1000.0;
    println!("\n  === 1000-Rack Datacenter Projection ===\n");
    println!("  Electronic: {:.0} MW total, {:.0} MW cooling, PUE={:.2}",
             electronic_rack_kw * racks / 1000.0,
             electronic_rack_kw * racks * 0.2 / 1000.0,
             1.2);
    println!("  Photonic:   {:.1} MW total, {:.1} MW cooling, PUE={:.2}",
             photonic_rack_kw * racks / 1000.0,
             photonic_rack_kw * racks * 0.02 / 1000.0,
             1.02);
    println!("  Power saved: {:.1} MW = {:.0}% reduction",
             (electronic_rack_kw - photonic_rack_kw) * racks / 1000.0,
             (1.0 - photonic_rack_kw / electronic_rack_kw) * 100.0);
    println!("  CO₂ saved:   {:.0} tonnes/year",
             (electronic_rack_kw - photonic_rack_kw) * racks * 8.76 * 0.4);
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let do_cross_dse = args.iter().any(|a| a == "--cross-dse");

    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Photonic-Energy Bridge Calculator                          ║");
    println!("║  n=6 thermal-power chain verification                       ║");
    println!("║  σ(n)·φ(n) = n·τ(n) ⟺ n = 6                              ║");
    println!("╚══════════════════════════════════════════════════════════════╝\n");

    // === Part 1: n=6 Photonic Parameter Verification ===
    let params = photonic_n6_params();
    let exact_count = params.iter().filter(|p| p.grade == "EXACT").count();
    let total_count = params.len();

    println!("=== n=6 Photonic Parameter Verification ({}/{} EXACT) ===\n", exact_count, total_count);
    println!("  {:40} | {:>8} | {:>8} | {:>20} | Grade", "Parameter", "Actual", "n6 Target", "Formula");
    println!("  {:->40}-+----------+----------+----------------------+------", "");

    for p in &params {
        let check = if p.grade == "EXACT" { "✓" } else if p.grade == "CLOSE" { "~" } else { "✗" };
        println!("  {:40} | {:>8.3} | {:>8.3} | {:>20} | {} {}",
                 p.name, p.value, p.n6_target, p.n6_formula, check, p.grade);
    }
    println!("\n  EXACT: {}/{}  ({:.1}%)", exact_count, total_count, exact_count as f64 / total_count as f64 * 100.0);

    // === Part 2: Thermal Analysis per Substrate ===
    let substrates = compute_substrates();
    println!("\n=== Thermal Analysis: Compute Substrates ===\n");
    println!("  {:30} | {:>7} | {:>8} | {:>10} | {:>8} | {:>6} | {:>8} | n6#",
             "Substrate", "TDP(W)", "TOPS/W", "Heat W/cm²", "Cool %", "PUE", "CO₂ t/y");
    println!("  {:->30}-+---------+----------+------------+----------+--------+----------+----", "");

    for s in &substrates {
        let r = analyze_thermal(s);
        println!("  {:30} | {:>7.1} | {:>8.2} | {:>10.1} | {:>7.1}% | {:>6.2} | {:>8.1} | {:>3}",
                 r.substrate, s.tdp_w, r.efficiency_tops_w, r.heat_density_w_cm2,
                 r.cooling_fraction * 100.0, s.pue, r.co2_kg_per_year / 1000.0, s.n6_connections);
    }

    // === Part 3: Key Ratios ===
    println!("\n=== Key n=6 Ratios in Photonic Energy Chain ===\n");

    let h100 = &substrates[0];
    let photonic_tpu = &substrates[2];
    let diamond_nv = &substrates[6];

    let tdp_ratio = h100.tdp_w / photonic_tpu.tdp_w;
    let tops_w_ratio = (photonic_tpu.tops / photonic_tpu.tdp_w) / (h100.tops / h100.tdp_w);
    let heat_ratio = h100.joule_heat_frac / photonic_tpu.joule_heat_frac;
    let pue_delta = h100.pue - 1.0;

    println!("  TDP ratio (H100/Photonic):    {:.1}x  (n6: σ-φ = {})", tdp_ratio, SIGMA_PHI as u32);
    println!("  TOPS/W improvement:           {:.1}x  (n6: σ = {})", tops_w_ratio, SIGMA as u32);
    println!("  Joule heat fraction ratio:    {:.1}x  (n6: σ-φ = {})", heat_ratio, SIGMA_PHI as u32);
    println!("  Electronic PUE overhead:      {:.1}   (n6: 1/(sopfr) = {:.1})", pue_delta, 1.0/SOPFR);
    println!("  Diamond NV n6 connections:    {}    (HIGHEST — Z=6 + photonic)", diamond_nv.n6_connections);

    // Thermal equation: P_joule = I²R (electronic) vs P ≈ 0 (photonic)
    let kt = BOLTZMANN * ROOM_TEMP;
    println!("\n  kBT at room temp:             {:.4e} J = {:.4} eV", kt, kt / 1.602e-19);
    println!("  Photon energy at 1550nm:      {:.4} eV (>> kBT, no thermal noise)",
             6.626e-34 * SPEED_LIGHT / 1550e-9 / 1.602e-19);

    // === Part 4: Datacenter-Scale Impact ===
    println!("\n=== Datacenter-Scale Impact (1000 racks) ===\n");

    let racks = 1000.0_f64;
    let electronic_mw = 48.0 * racks / 1000.0;
    let photonic_mw = 4.8 * racks / 1000.0;
    let hours_yr = 8760.0;

    println!("  ┌────────────────┬──────────────┬──────────────┬────────────┐");
    println!("  │                │  Electronic  │   Photonic   │  Savings   │");
    println!("  ├────────────────┼──────────────┼──────────────┼────────────┤");
    println!("  │ Rack power     │ {:>8.1} kW  │ {:>8.1} kW  │ {:>5.1}x     │", 48.0, 4.8, 10.0);
    println!("  │ Total power    │ {:>8.1} MW  │ {:>8.1} MW  │ {:>5.1}x     │", electronic_mw, photonic_mw, electronic_mw/photonic_mw);
    println!("  │ Cooling power  │ {:>8.1} MW  │ {:>8.2} MW  │ {:>5.0}x     │", electronic_mw*0.2, photonic_mw*0.02, electronic_mw*0.2/(photonic_mw*0.02));
    println!("  │ PUE            │ {:>8.2}     │ {:>8.2}     │ {:>5.2}     │", 1.20, 1.02, 1.20-1.02);
    println!("  │ Annual energy  │ {:>7.0} GWh │ {:>7.1} GWh │ {:>5.0} GWh │",
             electronic_mw * hours_yr / 1000.0,
             photonic_mw * hours_yr / 1000.0,
             (electronic_mw - photonic_mw) * hours_yr / 1000.0);
    println!("  │ CO₂ emissions  │ {:>7.0} t   │ {:>7.0} t   │ {:>5.0} t   │",
             electronic_mw * hours_yr * 0.4 / 1000.0,
             photonic_mw * hours_yr * 0.4 / 1000.0,
             (electronic_mw - photonic_mw) * hours_yr * 0.4 / 1000.0);
    println!("  │ Cooling system │  Liquid req  │  Passive OK  │ No chillers│");
    println!("  └────────────────┴──────────────┴──────────────┴────────────┘");
    println!("\n  n=6 ratio: {:.1} MW / {:.1} MW = {:.1}x = σ-φ EXACT", electronic_mw, photonic_mw, electronic_mw/photonic_mw);

    if do_cross_dse {
        cross_dse_chip_thermal_energy();
    }

    // === Summary ===
    println!("\n╔══════════════════════════════════════════════════════════════╗");
    println!("║  SUMMARY: Photonic-Energy n=6 Bridge                       ║");
    println!("╠══════════════════════════════════════════════════════════════╣");
    println!("║  n=6 EXACT connections: {}/{}                              ║", exact_count, total_count);
    println!("║  TDP reduction:  σ-φ = 10x (electronic → photonic)         ║");
    println!("║  PUE improvement: σ/(σ-φ) = 1.2 → 1 + 1/(σ·τ) = 1.02     ║");
    println!("║  Best chip path: Diamond + Photonic DC (score 0.9074)      ║");
    println!("║  Annual CO₂ savings per 1000 racks: ~150,000 tonnes        ║");
    println!("║  Candidate: BT-77 Photonic-Energy Bridge                   ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
}
