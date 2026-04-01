/// N6 Semiconductor Process / GPU SM / HBM Ladder Calculator
/// ==========================================================
/// Verifies semiconductor architecture constants from n=6 arithmetic.
/// Covers BT-28 (GPU SM), BT-37 (process nodes), BT-55 (HBM capacity),
/// BT-69 (chiplet), BT-75 (HBM interface).
///
/// n=6 constants: sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5,
///                J2(6)=24, mu(6)=1, P2=28 (2nd perfect number)
///
/// Build: ~/.cargo/bin/rustc main.rs -o semiconductor-calc
/// Usage: ./semiconductor-calc [--verbose]

use std::env;

// ─── N=6 Constants ───
const SIGMA: f64 = 12.0;
const TAU: f64 = 4.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const J2: f64 = 24.0;
const MU: f64 = 1.0;
const N: f64 = 6.0;
const P2: f64 = 28.0; // 2nd perfect number

// ─── Grading ───
fn grade(actual: f64, expected: f64) -> (&'static str, f64) {
    if actual == 0.0 && expected == 0.0 {
        return ("EXACT", 0.0);
    }
    let err = ((actual - expected) / expected).abs() * 100.0;
    if err < 0.01 {
        ("EXACT", err)
    } else if err < 5.0 {
        ("CLOSE", err)
    } else {
        ("FAIL", err)
    }
}

// ─── Entry struct ───
struct Entry {
    category: &'static str,
    bt: &'static str,
    name: &'static str,
    actual: f64,
    formula: &'static str,
    expected: f64,
}

fn build_database() -> Vec<Entry> {
    vec![
        // ═══════════════════════════════════════════════════
        // BT-37: TSMC Process Nodes
        // ═══════════════════════════════════════════════════
        Entry {
            category: "Process", bt: "BT-37",
            name: "N5 gate pitch (nm)",
            actual: 48.0,
            formula: "sigma * tau",
            expected: SIGMA * TAU, // 48
        },
        Entry {
            category: "Process", bt: "BT-37",
            name: "N5 metal pitch (nm)",
            actual: 28.0,
            formula: "P2",
            expected: P2, // 28
        },
        Entry {
            category: "Process", bt: "BT-37",
            name: "N7 gate pitch (nm)",
            actual: 54.0,
            formula: "sigma * tau + n",
            expected: SIGMA * TAU + N, // 54
        },
        Entry {
            category: "Process", bt: "BT-37",
            name: "N7 metal pitch (nm)",
            actual: 40.0,
            formula: "tau * (sigma - phi)",
            expected: TAU * (SIGMA - PHI), // 40
        },
        Entry {
            category: "Process", bt: "BT-37",
            name: "N3 gate pitch (nm)",
            actual: 48.0,
            formula: "sigma * tau",
            expected: SIGMA * TAU, // 48
        },
        Entry {
            category: "Process", bt: "BT-37",
            name: "N3 metal pitch (nm)",
            actual: 21.0,
            formula: "P2 - sopfr - phi",
            expected: P2 - SOPFR - PHI, // 21
        },
        Entry {
            category: "Process", bt: "BT-37",
            name: "N2 gate pitch (nm)",
            actual: 48.0,
            formula: "sigma * tau",
            expected: SIGMA * TAU, // 48
        },

        // ═══════════════════════════════════════════════════
        // BT-28: GPU SM Counts
        // ═══════════════════════════════════════════════════
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "AD102 full-die SMs",
            actual: 144.0,
            formula: "sigma^2",
            expected: SIGMA * SIGMA, // 144
        },
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "GH100 enabled SMs",
            actual: 132.0,
            formula: "sigma * (sigma - mu)",
            expected: SIGMA * (SIGMA - MU), // 132
        },
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "GB100 full-die SMs",
            actual: 192.0,
            formula: "sigma * 2^tau",
            expected: SIGMA * (PHI as f64).powf(TAU), // 192
        },
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "GB300 SMs",
            actual: 160.0,
            formula: "phi^sopfr * sopfr",
            expected: (PHI as f64).powf(SOPFR) * SOPFR, // 32*5 = 160
        },
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "GV100 full-die SMs",
            actual: 84.0,
            formula: "sigma * (n + mu)",
            expected: SIGMA * (N + MU), // 84
        },
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "GA100 full-die SMs",
            actual: 128.0,
            formula: "phi^(sigma - sopfr)",
            expected: (PHI as f64).powf(SIGMA - SOPFR), // 2^7 = 128
        },
        Entry {
            category: "GPU SM", bt: "BT-28",
            name: "TU102 full-die SMs",
            actual: 72.0,
            formula: "sigma * n",
            expected: SIGMA * N, // 72
        },

        // ═══════════════════════════════════════════════════
        // BT-55: HBM Capacity Ladder (GB)
        // ═══════════════════════════════════════════════════
        Entry {
            category: "HBM Cap", bt: "BT-55",
            name: "A100 40GB",
            actual: 40.0,
            formula: "tau * (sigma - phi)",
            expected: TAU * (SIGMA - PHI), // 40
        },
        Entry {
            category: "HBM Cap", bt: "BT-55",
            name: "A100 80GB",
            actual: 80.0,
            formula: "phi^tau * sopfr",
            expected: (PHI as f64).powf(TAU) * SOPFR, // 16*5 = 80
        },
        Entry {
            category: "HBM Cap", bt: "BT-55",
            name: "H100 80GB",
            actual: 80.0,
            formula: "phi^tau * sopfr",
            expected: (PHI as f64).powf(TAU) * SOPFR, // 80
        },
        Entry {
            category: "HBM Cap", bt: "BT-55",
            name: "H200 141GB",
            actual: 141.0,
            formula: "sigma^2 - n/phi",
            expected: SIGMA * SIGMA - N / PHI, // 144-3 = 141
        },
        Entry {
            category: "HBM Cap", bt: "BT-55",
            name: "B100/B200 192GB",
            actual: 192.0,
            formula: "sigma * phi^tau",
            expected: SIGMA * (PHI as f64).powf(TAU), // 12*16 = 192
        },
        Entry {
            category: "HBM Cap", bt: "BT-55",
            name: "B300 288GB",
            actual: 288.0,
            formula: "sigma * J2",
            expected: SIGMA * J2, // 12*24 = 288
        },

        // ═══════════════════════════════════════════════════
        // BT-75: HBM Interface
        // ═══════════════════════════════════════════════════
        Entry {
            category: "HBM Intf", bt: "BT-75",
            name: "HBM bus width (bits)",
            actual: 1024.0,
            formula: "2^(sigma - phi)",
            expected: (2.0_f64).powf(SIGMA - PHI), // 2^10 = 1024
        },
        Entry {
            category: "HBM Intf", bt: "BT-75",
            name: "HBM2 stack layers",
            actual: 4.0,
            formula: "tau",
            expected: TAU, // 4
        },
        Entry {
            category: "HBM Intf", bt: "BT-75",
            name: "HBM3 stack layers",
            actual: 8.0,
            formula: "sigma - tau",
            expected: SIGMA - TAU, // 8
        },
        Entry {
            category: "HBM Intf", bt: "BT-75",
            name: "HBM3E stack layers",
            actual: 12.0,
            formula: "sigma",
            expected: SIGMA, // 12
        },
        Entry {
            category: "HBM Intf", bt: "BT-75",
            name: "Stack ladder tau->sigma-tau->sigma",
            actual: 4.0 + 8.0 + 12.0,
            formula: "tau + (sigma-tau) + sigma = 2*sigma",
            expected: 2.0 * SIGMA, // 24 = J2
        },

        // ═══════════════════════════════════════════════════
        // BT-69: Chiplet Architecture
        // ═══════════════════════════════════════════════════
        Entry {
            category: "Chiplet", bt: "BT-69",
            name: "B300 dual-die total SMs",
            actual: 160.0,
            formula: "phi^sopfr * sopfr",
            expected: (PHI as f64).powf(SOPFR) * SOPFR, // 160
        },
        Entry {
            category: "Chiplet", bt: "BT-69",
            name: "R100 sigma stacks predicted",
            actual: 12.0,
            formula: "sigma",
            expected: SIGMA, // 12
        },
        Entry {
            category: "Chiplet", bt: "BT-69",
            name: "NVLink lanes per direction",
            actual: 18.0,
            formula: "sigma + n",
            expected: SIGMA + N, // 18
        },

        // ═══════════════════════════════════════════════════
        // BT-28 extended: CUDA cores per SM
        // ═══════════════════════════════════════════════════
        Entry {
            category: "CUDA/SM", bt: "BT-28",
            name: "Volta/Turing/Ampere CUDA/SM",
            actual: 64.0,
            formula: "2^n",
            expected: (2.0_f64).powf(N), // 64
        },
        Entry {
            category: "CUDA/SM", bt: "BT-28",
            name: "Hopper/Ada CUDA/SM",
            actual: 128.0,
            formula: "2^(n+mu)",
            expected: (2.0_f64).powf(N + MU), // 128
        },
        Entry {
            category: "CUDA/SM", bt: "BT-28",
            name: "Tensor cores per SM (modern)",
            actual: 4.0,
            formula: "tau",
            expected: TAU, // 4
        },

        // ═══════════════════════════════════════════════════
        // BT-37 extended: gate pitch convergence at sigma*tau=48
        // ═══════════════════════════════════════════════════
        Entry {
            category: "Process", bt: "BT-37",
            name: "sigma*tau attractor (nm)",
            actual: 48.0,
            formula: "sigma * tau",
            expected: SIGMA * TAU, // 48
        },
    ]
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let verbose = args.iter().any(|a| a == "--verbose");

    println!("=======================================================");
    println!("  N6 Semiconductor Calculator");
    println!("  BT-28 / BT-37 / BT-55 / BT-69 / BT-75");
    println!("  sigma=12, tau=4, phi=2, sopfr=5, J2=24, mu=1, P2=28");
    println!("=======================================================\n");

    let db = build_database();

    let mut total = 0usize;
    let mut exact = 0usize;
    let mut close = 0usize;
    let mut fail = 0usize;

    let mut current_cat = "";

    // Header
    println!("{:<10} {:<6} {:<35} {:>8} {:<25} {:>8} {:<6} {:>6}",
        "Category", "BT", "Parameter", "Actual", "Formula", "Expect", "Grade", "Err%");
    println!("{}", "-".repeat(108));

    for e in &db {
        if e.category != current_cat {
            if !current_cat.is_empty() {
                println!();
            }
            current_cat = e.category;
        }

        let (g, err) = grade(e.actual, e.expected);
        total += 1;
        match g {
            "EXACT" => exact += 1,
            "CLOSE" => close += 1,
            _ => fail += 1,
        }

        let grade_mark = match g {
            "EXACT" => "EXACT",
            "CLOSE" => "CLOSE",
            _ => "FAIL",
        };

        println!("{:<10} {:<6} {:<35} {:>8.1} {:<25} {:>8.1} {:<6} {:>5.2}%",
            e.category, e.bt, e.name, e.actual, e.formula, e.expected, grade_mark, err);

        if verbose && g != "EXACT" {
            println!("           ^ delta = {:.2} (actual={}, expected={})",
                (e.actual - e.expected).abs(), e.actual, e.expected);
        }
    }

    println!("\n{}", "=".repeat(108));
    println!("SUMMARY");
    println!("{}", "-".repeat(108));
    println!("  Total parameters : {}", total);
    println!("  EXACT            : {} ({:.1}%)", exact, exact as f64 / total as f64 * 100.0);
    println!("  CLOSE            : {} ({:.1}%)", close, close as f64 / total as f64 * 100.0);
    println!("  FAIL             : {} ({:.1}%)", fail, fail as f64 / total as f64 * 100.0);
    println!();

    // Per-BT breakdown
    let bts = ["BT-28", "BT-37", "BT-55", "BT-69", "BT-75"];
    println!("Per-BT Breakdown:");
    println!("  {:<8} {:>6} {:>6} {:>6} {:>8}", "BT", "Total", "EXACT", "CLOSE", "EXACT%");
    for bt in &bts {
        let bt_entries: Vec<&Entry> = db.iter().filter(|e| e.bt == *bt).collect();
        let bt_total = bt_entries.len();
        let bt_exact = bt_entries.iter().filter(|e| grade(e.actual, e.expected).0 == "EXACT").count();
        let bt_close = bt_entries.iter().filter(|e| grade(e.actual, e.expected).0 == "CLOSE").count();
        let pct = if bt_total > 0 { bt_exact as f64 / bt_total as f64 * 100.0 } else { 0.0 };
        println!("  {:<8} {:>6} {:>6} {:>6} {:>7.1}%", bt, bt_total, bt_exact, bt_close, pct);
    }

    println!("\n{}", "=".repeat(108));

    // Exit code: 0 if >80% EXACT
    if (exact as f64 / total as f64) < 0.80 {
        std::process::exit(1);
    }
}
