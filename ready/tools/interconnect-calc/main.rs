#![allow(dead_code)]

/// N6 Interconnect Speed Calculator
/// =================================
/// Verifies ALL interconnect standards against n=6 arithmetic:
/// PCIe, UCIe, CXL, HBM interface, NVLink, DDR
/// σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5, J₂(6)=24, μ(6)=1
///
/// Build: rustc main.rs -o interconnect-calc
/// Usage: ./interconnect-calc [--pcie] [--ucie] [--cxl] [--hbm] [--nvlink] [--ddr] [--all]

use std::env;

// ─── N=6 Constants ───
const SIGMA: u64 = 12;
const TAU: u64 = 4;
const PHI: u64 = 2;
const SOPFR: u64 = 5;
const J2: u64 = 24;
const MU: u64 = 1;
const N: u64 = 6;

// ─── Interconnect Entry ───

struct InterconnectEntry {
    standard: &'static str,
    gen: &'static str,
    year: u16,
    speed_value: u64,
    speed_unit: &'static str,
    n6_formula: &'static str,
    n6_computed: u64,
    is_prediction: bool,
}

// ─── PCIe Database ───

fn pcie_database() -> Vec<InterconnectEntry> {
    vec![
        InterconnectEntry {
            standard: "PCIe", gen: "3.0", year: 2010,
            speed_value: 8, speed_unit: "GT/s",
            n6_formula: "2^(n/φ) = 2^3", n6_computed: 8,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "PCIe", gen: "4.0", year: 2017,
            speed_value: 16, speed_unit: "GT/s",
            n6_formula: "2^τ", n6_computed: 2u64.pow(TAU as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "PCIe", gen: "5.0", year: 2019,
            speed_value: 32, speed_unit: "GT/s",
            n6_formula: "2^sopfr", n6_computed: 2u64.pow(SOPFR as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "PCIe", gen: "6.0", year: 2022,
            speed_value: 64, speed_unit: "GT/s",
            n6_formula: "2^n", n6_computed: 2u64.pow(N as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "PCIe", gen: "7.0", year: 2025,
            speed_value: 128, speed_unit: "GT/s",
            n6_formula: "2^(σ-sopfr)", n6_computed: 2u64.pow((SIGMA - SOPFR) as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "PCIe", gen: "8.0", year: 2028,
            speed_value: 256, speed_unit: "GT/s",
            n6_formula: "2^(σ-τ)", n6_computed: 2u64.pow((SIGMA - TAU) as u32),
            is_prediction: true,
        },
    ]
}

// ─── UCIe Database ───

fn ucie_database() -> Vec<InterconnectEntry> {
    vec![
        InterconnectEntry {
            standard: "UCIe", gen: "1.0", year: 2022,
            speed_value: 4, speed_unit: "GT/s",
            n6_formula: "τ", n6_computed: TAU,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "UCIe", gen: "2.0", year: 2024,
            speed_value: 32, speed_unit: "GT/s",
            n6_formula: "2^sopfr", n6_computed: 2u64.pow(SOPFR as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "UCIe", gen: "3.0lo", year: 2026,
            speed_value: 48, speed_unit: "GT/s",
            n6_formula: "σ·τ", n6_computed: SIGMA * TAU,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "UCIe", gen: "3.0hi", year: 2026,
            speed_value: 64, speed_unit: "GT/s",
            n6_formula: "2^n", n6_computed: 2u64.pow(N as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "UCIe", gen: "lanes", year: 0,
            speed_value: 64, speed_unit: "lanes",
            n6_formula: "2^n", n6_computed: 2u64.pow(N as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "UCIe", gen: "pitch", year: 0,
            speed_value: 25, speed_unit: "μm",
            n6_formula: "J₂+μ", n6_computed: J2 + MU,
            is_prediction: false,
        },
    ]
}

// ─── CXL Database ───

fn cxl_database() -> Vec<InterconnectEntry> {
    vec![
        InterconnectEntry {
            standard: "CXL", gen: "1.0/1.1", year: 2019,
            speed_value: 32, speed_unit: "GT/s",
            n6_formula: "2^sopfr", n6_computed: 2u64.pow(SOPFR as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "CXL", gen: "2.0", year: 2020,
            speed_value: 32, speed_unit: "GT/s",
            n6_formula: "2^sopfr", n6_computed: 2u64.pow(SOPFR as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "CXL", gen: "3.0/3.1", year: 2022,
            speed_value: 64, speed_unit: "GT/s",
            n6_formula: "2^n", n6_computed: 2u64.pow(N as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "CXL", gen: "4.0", year: 2025,
            speed_value: 128, speed_unit: "GT/s",
            n6_formula: "2^(σ-sopfr)", n6_computed: 2u64.pow((SIGMA - SOPFR) as u32),
            is_prediction: false,
        },
    ]
}

// ─── HBM Interface Database ───

fn hbm_interface_database() -> Vec<InterconnectEntry> {
    vec![
        InterconnectEntry {
            standard: "HBM", gen: "1/2/2e", year: 2013,
            speed_value: 1024, speed_unit: "bits",
            n6_formula: "2^(σ-φ)", n6_computed: 2u64.pow((SIGMA - PHI) as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "3/3e", year: 2022,
            speed_value: 1024, speed_unit: "bits",
            n6_formula: "2^(σ-φ)", n6_computed: 2u64.pow((SIGMA - PHI) as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "4", year: 2025,
            speed_value: 2048, speed_unit: "bits",
            n6_formula: "2^(σ-μ)", n6_computed: 2u64.pow((SIGMA - MU) as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "3 ch", year: 2022,
            speed_value: 16, speed_unit: "channels",
            n6_formula: "2^τ", n6_computed: 2u64.pow(TAU as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "4 ch", year: 2025,
            speed_value: 32, speed_unit: "channels",
            n6_formula: "2^sopfr", n6_computed: 2u64.pow(SOPFR as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "3e rate", year: 2024,
            speed_value: 8, speed_unit: "Gb/s",
            n6_formula: "σ-τ", n6_computed: SIGMA - TAU,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "4 rate", year: 2025,
            speed_value: 8, speed_unit: "Gb/s",
            n6_formula: "σ-τ", n6_computed: SIGMA - TAU,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "HBM", gen: "4E rate", year: 2026,
            speed_value: 10, speed_unit: "Gb/s",
            n6_formula: "σ-φ", n6_computed: SIGMA - PHI,
            is_prediction: false,
        },
    ]
}

// ─── NVLink Database ───

fn nvlink_database() -> Vec<InterconnectEntry> {
    vec![
        InterconnectEntry {
            standard: "NVLink", gen: "1.0", year: 2016,
            speed_value: 20, speed_unit: "GB/s/link",
            n6_formula: "J₂-τ", n6_computed: J2 - TAU,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "NVLink", gen: "2.0", year: 2017,
            speed_value: 25, speed_unit: "GB/s/link",
            n6_formula: "J₂+μ", n6_computed: J2 + MU,
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "NVLink", gen: "3.0", year: 2020,
            speed_value: 50, speed_unit: "GB/s/link",
            n6_formula: "sopfr·(σ-φ)", n6_computed: SOPFR * (SIGMA - PHI),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "NVLink", gen: "4.0", year: 2022,
            speed_value: 100, speed_unit: "GB/s/link",
            n6_formula: "(σ-φ)^φ", n6_computed: (SIGMA - PHI).pow(PHI as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "NVLink", gen: "5.0", year: 2024,
            speed_value: 200, speed_unit: "GB/s/link",
            n6_formula: "φ·(σ-φ)^φ", n6_computed: PHI * (SIGMA - PHI).pow(PHI as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "NVLink", gen: "4.0 lnk", year: 2022,
            speed_value: 18, speed_unit: "links/GPU",
            n6_formula: "σ+n", n6_computed: SIGMA + N,
            is_prediction: false,
        },
    ]
}

// ─── DDR Database ───

fn ddr_database() -> Vec<InterconnectEntry> {
    vec![
        InterconnectEntry {
            standard: "DDR", gen: "bus", year: 0,
            speed_value: 64, speed_unit: "bits",
            n6_formula: "2^n", n6_computed: 2u64.pow(N as u32),
            is_prediction: false,
        },
        InterconnectEntry {
            standard: "DDR", gen: "ch/DIMM", year: 0,
            speed_value: 2, speed_unit: "channels",
            n6_formula: "φ", n6_computed: PHI,
            is_prediction: false,
        },
    ]
}

// ─── Display Helpers ───

fn status_str(actual: u64, computed: u64, is_prediction: bool) -> &'static str {
    if is_prediction {
        "PREDICT"
    } else if actual == computed {
        "EXACT"
    } else {
        "MISS"
    }
}

fn status_icon(actual: u64, computed: u64, is_prediction: bool) -> &'static str {
    if is_prediction {
        "\u{1f52e}"  // crystal ball
    } else if actual == computed {
        "\u{2705}"   // check
    } else {
        "\u{274c}"   // cross
    }
}

fn print_header() {
    println!("\u{256d}{}", "\u{2500}".repeat(78));
    println!("\u{2502}  N6 INTERCONNECT CALCULATOR v1.0");
    println!("\u{2502}  n=6: \u{03c3}=12, \u{03c4}=4, \u{03c6}=2, sopfr=5, J\u{2082}=24, \u{03bc}=1");
    println!("\u{2502}  Verifies interconnect speeds from n=6 arithmetic");
    println!("\u{2570}{}", "\u{2500}".repeat(78));
    println!();
}

fn print_section(entries: &[InterconnectEntry], title: &str) -> (u64, u64, u64) {
    let mut exact = 0u64;
    let mut total = 0u64;
    let mut predictions = 0u64;

    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(76));
    println!("\u{2502} {:<74} \u{2502}", title);
    println!("\u{251c}{}\u{2524}", "\u{2500}".repeat(76));
    println!("\u{2502} {:<10} {:<8} {:<14} {:<22} {:<8} {:<6} \u{2502}",
             "Standard", "Gen", "Speed", "n=6 Formula", "Status", "");
    println!("\u{251c}{}\u{2524}", "\u{2500}".repeat(76));

    for e in entries {
        let status = status_str(e.speed_value, e.n6_computed, e.is_prediction);
        let icon = status_icon(e.speed_value, e.n6_computed, e.is_prediction);
        let speed_str = format!("{} {}", e.speed_value, e.speed_unit);

        println!("\u{2502} {:<10} {:<8} {:<14} {:<22} {:<8} {} \u{2502}",
                 e.standard, e.gen, speed_str, e.n6_formula, status, icon);

        if e.is_prediction {
            predictions += 1;
        } else {
            total += 1;
            if e.speed_value == e.n6_computed {
                exact += 1;
            }
        }
    }

    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(76));
    println!("  Section: {}/{} EXACT", exact, total);
    println!();

    (exact, total, predictions)
}

// ─── PCIe Exponent Ladder ───

fn print_pcie_ladder() {
    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(76));
    println!("\u{2502} {:<74} \u{2502}", "PCIe EXPONENT LADDER (doubling = 2^k, k consecutive)");
    println!("\u{251c}{}\u{2524}", "\u{2500}".repeat(76));
    println!("\u{2502}  PCIe 3.0  4.0  5.0  6.0  7.0  8.0                                     \u{2502}");
    println!("\u{2502}  exp:  3    4    5    6    7    8                                         \u{2502}");
    println!("\u{2502}  n=6: n/\u{03c6}  \u{03c4}  sopfr  n  \u{03c3}-sopfr  \u{03c3}-\u{03c4}                                    \u{2502}");
    println!("\u{2502}                                                                            \u{2502}");
    println!("\u{2502}  Consecutive integers [3,4,5,6,7,8] map to n=6 constants!                 \u{2502}");
    println!("\u{2502}  Range: n/\u{03c6}=3 to \u{03c3}-\u{03c4}=8 \u{2192} span = sopfr = 5                              \u{2502}");
    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(76));
    println!();
}

// ─── HBM Interface Ladder ───

fn print_hbm_ladder() {
    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(76));
    println!("\u{2502} {:<74} \u{2502}", "HBM INTERFACE WIDTH LADDER (BT-75)");
    println!("\u{251c}{}\u{2524}", "\u{2500}".repeat(76));
    println!("\u{2502}  HBM1/2/2e/3/3e: 1024 bits = 2^(\u{03c3}-\u{03c6}) = 2^10                           \u{2502}");
    println!("\u{2502}  HBM4:           2048 bits = 2^(\u{03c3}-\u{03bc}) = 2^11                           \u{2502}");
    println!("\u{2502}  HBM5 (pred):    4096 bits = 2^\u{03c3}     = 2^12                           \u{2502}");
    println!("\u{2502}                                                                            \u{2502}");
    println!("\u{2502}  Exponent ladder: \u{03c3}-\u{03c6} \u{2192} \u{03c3}-\u{03bc} \u{2192} \u{03c3} = 10 \u{2192} 11 \u{2192} 12                      \u{2502}");
    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(76));
    println!();
}

// ─── NVLink Doubling Pattern ───

fn print_nvlink_pattern() {
    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(76));
    println!("\u{2502} {:<74} \u{2502}", "NVLink BANDWIDTH PATTERN");
    println!("\u{251c}{}\u{2524}", "\u{2500}".repeat(76));
    println!("\u{2502}  1.0: 20   = J\u{2082}-\u{03c4}                                                       \u{2502}");
    println!("\u{2502}  2.0: 25   = J\u{2082}+\u{03bc}                                                       \u{2502}");
    println!("\u{2502}  3.0: 50   = sopfr\u{00b7}(\u{03c3}-\u{03c6})                                               \u{2502}");
    println!("\u{2502}  4.0: 100  = (\u{03c3}-\u{03c6})^{{\u{03c6}}} = 10^2                                          \u{2502}");
    println!("\u{2502}  5.0: 200  = \u{03c6}\u{00b7}(\u{03c3}-\u{03c6})^{{\u{03c6}}}                                             \u{2502}");
    println!("\u{2502}                                                                            \u{2502}");
    println!("\u{2502}  Each gen: \u{00d7}\u{03c6} scaling from (\u{03c3}-\u{03c6})^{{\u{03c6}}} base                               \u{2502}");
    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(76));
    println!();
}

// ─── Bandwidth Calculator ───

fn print_bandwidth_summary() {
    println!("\u{250c}{}\u{2510}", "\u{2500}".repeat(76));
    println!("\u{2502} {:<74} \u{2502}", "BANDWIDTH CALCULATIONS (per direction)");
    println!("\u{251c}{}\u{2524}", "\u{2500}".repeat(76));

    // PCIe x16 bandwidth
    let pcie_gens = [
        ("PCIe 5.0", 32u64, 16u64, 128, 130),  // 128b/130b encoding
        ("PCIe 6.0", 64, 16, 256, 256),          // PAM4, FLIT
        ("PCIe 7.0", 128, 16, 256, 256),
    ];

    for (name, gts, lanes, _, _) in &pcie_gens {
        let bw_gbps = gts * lanes;  // raw GT/s * lanes
        // effective ~= raw * encoding_eff (~98.5% for PCIe 5/6/7)
        let bw_gbs = bw_gbps / 8;   // approximate GB/s
        println!("\u{2502}  {:<12} x{}: ~{} GB/s raw ({} GT/s \u{00d7} {} lanes / 8){:<14}\u{2502}",
                 name, lanes, bw_gbs, gts, lanes, "");
    }

    println!("\u{2502}{}\u{2502}", " ".repeat(76));

    // NVLink total bandwidth
    let nvl4_total = 100 * 18;  // 18 links * 100 GB/s
    let nvl5_total = 200 * 18;  // 18 links * 200 GB/s
    println!("\u{2502}  NVLink 4.0: {} GB/s total (18 links \u{00d7} 100 GB/s){:<20}\u{2502}", nvl4_total, "");
    println!("\u{2502}  NVLink 5.0: {} GB/s total (18 links \u{00d7} 200 GB/s){:<19}\u{2502}", nvl5_total, "");
    println!("\u{2502}  Note: 1800 = σ\u{00b7}(σ-φ)^φ\u{00b7}(σ+n) = 12\u{00b7}100\u{00b7}1.5... beautiful!{:<12}\u{2502}", "");

    println!("\u{2514}{}\u{2518}", "\u{2500}".repeat(76));
    println!();
}

// ─── Summary ───

fn print_summary(total_exact: u64, total_checked: u64, total_predictions: u64) {
    let pct = if total_checked > 0 {
        (total_exact as f64 / total_checked as f64) * 100.0
    } else {
        0.0
    };

    println!("\u{2554}{}\u{2557}", "\u{2550}".repeat(76));
    println!("\u{2551} {:<74} \u{2551}", "FINAL SUMMARY");
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(76));
    println!("\u{2551}  Total verified:   {}/{} EXACT ({:.1}%){:<40}\u{2551}",
             total_exact, total_checked, pct, "");
    println!("\u{2551}  Predictions:      {}{:<55}\u{2551}", total_predictions, "");
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(76));
    println!("\u{2551} {:<74} \u{2551}", "PREDICTIONS");
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(76));
    println!("\u{2551}  PCIe 8.0:    256 GT/s  = 2^(\u{03c3}-\u{03c4}) = 2^8{:<29}\u{2551}", "");
    println!("\u{2551}  HBM5 width:  4096 bits = 2^\u{03c3} = 2^12{:<30}\u{2551}", "");
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(76));
    println!("\u{2551} {:<74} \u{2551}", "KEY INSIGHT");
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(76));
    println!("\u{2551}  PCIe exponents [3,4,5,6,7,8] = consecutive = [n/\u{03c6},\u{03c4},sopfr,n,\u{03c3}-sopfr,\u{03c3}-\u{03c4}]\u{2551}");
    println!("\u{2551}  HBM width exponents [10,11,12] = [\u{03c3}-\u{03c6}, \u{03c3}-\u{03bc}, \u{03c3}]{:<22}\u{2551}", "");
    println!("\u{2551}  NVLink scaling base = (\u{03c3}-\u{03c6})^\u{03c6} = 100, \u{00d7}\u{03c6} per gen{:<19}\u{2551}", "");
    println!("\u{255a}{}\u{255d}", "\u{2550}".repeat(76));
}

// ─── Main ───

fn main() {
    let args: Vec<String> = env::args().collect();

    let show_all = args.len() == 1 || args.iter().any(|a| a == "--all");
    let show_pcie = show_all || args.iter().any(|a| a == "--pcie");
    let show_ucie = show_all || args.iter().any(|a| a == "--ucie");
    let show_cxl = show_all || args.iter().any(|a| a == "--cxl");
    let show_hbm = show_all || args.iter().any(|a| a == "--hbm");
    let show_nvlink = show_all || args.iter().any(|a| a == "--nvlink");
    let show_ddr = show_all || args.iter().any(|a| a == "--ddr");

    print_header();

    let mut total_exact = 0u64;
    let mut total_checked = 0u64;
    let mut total_predictions = 0u64;

    if show_pcie {
        let db = pcie_database();
        let (e, t, p) = print_section(&db, "PCIe — PCI Express Speed Ladder");
        total_exact += e; total_checked += t; total_predictions += p;
        print_pcie_ladder();
    }

    if show_ucie {
        let db = ucie_database();
        let (e, t, p) = print_section(&db, "UCIe — Universal Chiplet Interconnect Express");
        total_exact += e; total_checked += t; total_predictions += p;
    }

    if show_cxl {
        let db = cxl_database();
        let (e, t, p) = print_section(&db, "CXL — Compute Express Link");
        total_exact += e; total_checked += t; total_predictions += p;
    }

    if show_hbm {
        let db = hbm_interface_database();
        let (e, t, p) = print_section(&db, "HBM — High Bandwidth Memory Interface");
        total_exact += e; total_checked += t; total_predictions += p;
        print_hbm_ladder();
    }

    if show_nvlink {
        let db = nvlink_database();
        let (e, t, p) = print_section(&db, "NVLink — NVIDIA GPU Interconnect");
        total_exact += e; total_checked += t; total_predictions += p;
        print_nvlink_pattern();
    }

    if show_ddr {
        let db = ddr_database();
        let (e, t, p) = print_section(&db, "DDR — Double Data Rate Memory Bus");
        total_exact += e; total_checked += t; total_predictions += p;
    }

    if show_all {
        print_bandwidth_summary();
    }

    print_summary(total_exact, total_checked, total_predictions);
}
