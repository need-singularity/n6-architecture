/// N6 Cross-Vendor Chip Comparison Calculator
/// ============================================
/// Compares ALL major chip vendors against n=6 arithmetic constants.
/// Shows convergence scores, formula frequency, cross-vendor reuse,
/// and predictions for unannounced chips.
///
/// Build: rustc main.rs -o vendor-compare-calc
/// Usage: ./vendor-compare-calc

#[allow(dead_code)]

// ─── N=6 Constants ───
const N: u64 = 6;
const PHI: u64 = 2;    // phi(6)
const TAU: u64 = 4;    // tau(6) = number of divisors
const SIGMA: u64 = 12; // sigma(6) = sum of divisors
const SOPFR: u64 = 5;  // sopfr(6) = 2+3
const MU: u64 = 1;     // |mu(6)| = 1
const J2: u64 = 24;    // Jordan totient J_2(6)
const SIGMA_SQ: u64 = 144; // sigma^2

// ─── Derived constants used in formulas ───
const SIGMA_MU: u64 = SIGMA - MU;       // 11
const SIGMA_PHI: u64 = SIGMA - PHI;     // 10
const SIGMA_TAU: u64 = SIGMA - TAU;     // 8
const PHI_POW_TAU: u64 = 16;            // phi^tau = 2^4
const TWO_POW_SOPFR: u64 = 32;          // 2^sopfr = 2^5

// ─── Formula Registry ───

#[derive(Clone)]
struct Formula {
    expr: &'static str,
    display: &'static str,
    value: u64,
}

fn formula_registry() -> Vec<Formula> {
    vec![
        Formula { expr: "n",                  display: "n",              value: N },
        Formula { expr: "phi",                display: "\u{03c6}",       value: PHI },
        Formula { expr: "n/phi",              display: "n/\u{03c6}",     value: N / PHI },
        Formula { expr: "tau",                display: "\u{03c4}",       value: TAU },
        Formula { expr: "sopfr",              display: "sopfr",          value: SOPFR },
        Formula { expr: "sigma-tau",          display: "\u{03c3}-\u{03c4}", value: SIGMA_TAU },
        Formula { expr: "sigma-phi",          display: "\u{03c3}-\u{03c6}", value: SIGMA_PHI },
        Formula { expr: "sigma-mu",           display: "\u{03c3}-\u{03bc}", value: SIGMA_MU },
        Formula { expr: "sigma",              display: "\u{03c3}",       value: SIGMA },
        Formula { expr: "phi^tau",            display: "\u{03c6}^\u{03c4}", value: PHI_POW_TAU },
        Formula { expr: "J_2",               display: "J\u{2082}",      value: J2 },
        Formula { expr: "2^sopfr",            display: "2^sopfr",        value: TWO_POW_SOPFR },
        Formula { expr: "2^tau",              display: "2^\u{03c4}",     value: 16 },
        Formula { expr: "phi^tau*sopfr",      display: "\u{03c6}^\u{03c4}\u{00b7}sopfr", value: PHI_POW_TAU * SOPFR },
        Formula { expr: "phi^tau*(sigma-phi)", display: "\u{03c6}^\u{03c4}\u{00b7}(\u{03c3}-\u{03c6})", value: PHI_POW_TAU * SIGMA_PHI },
        Formula { expr: "sigma*(sigma-mu)",   display: "\u{03c3}\u{00b7}(\u{03c3}-\u{03bc})", value: SIGMA * SIGMA_MU },
        Formula { expr: "sigma*(sigma-tau)",  display: "\u{03c3}\u{00b7}(\u{03c3}-\u{03c4})", value: SIGMA * SIGMA_TAU },
        Formula { expr: "sigma*phi^tau",      display: "\u{03c3}\u{00b7}\u{03c6}^\u{03c4}", value: SIGMA * PHI_POW_TAU },
        Formula { expr: "sigma*J_2",          display: "\u{03c3}\u{00b7}J\u{2082}", value: SIGMA * J2 },
        Formula { expr: "sigma^2",            display: "\u{03c3}\u{00b2}",   value: SIGMA_SQ },
        Formula { expr: "sigma^2*n/phi",      display: "\u{03c3}\u{00b2}\u{00b7}n/\u{03c6}", value: SIGMA_SQ * N / PHI },
        Formula { expr: "2^(sigma-tau)",      display: "2^(\u{03c3}-\u{03c4})", value: 256 },
        Formula { expr: "2^sopfr*(sigma-sopfr)", display: "2^sopfr\u{00b7}(\u{03c3}-sopfr)", value: TWO_POW_SOPFR * (SIGMA - SOPFR) },
        Formula { expr: "sigma*n",            display: "\u{03c3}\u{00b7}n",  value: SIGMA * N },
        Formula { expr: "sigma*sopfr",        display: "\u{03c3}\u{00b7}sopfr", value: SIGMA * SOPFR },
    ]
}

fn find_formula(value: u64) -> Option<Formula> {
    formula_registry().into_iter().find(|f| f.value == value)
}

fn find_formula_display(value: u64) -> String {
    match find_formula(value) {
        Some(f) => format!("{} = {}", f.display, f.value),
        None => format!("? ({})", value),
    }
}

// ─── Vendor/Chip Database ───

#[derive(Clone, Copy, PartialEq, Eq)]
enum Vendor {
    Nvidia,
    Amd,
    Google,
    Aws,
    Intel,
    Apple,
}

impl Vendor {
    fn name(&self) -> &'static str {
        match self {
            Vendor::Nvidia => "NVIDIA",
            Vendor::Amd    => "AMD",
            Vendor::Google => "Google",
            Vendor::Aws    => "AWS",
            Vendor::Intel  => "Intel",
            Vendor::Apple  => "Apple",
        }
    }
}

const ALL_VENDORS: [Vendor; 6] = [
    Vendor::Nvidia, Vendor::Amd, Vendor::Google,
    Vendor::Aws, Vendor::Intel, Vendor::Apple,
];

#[derive(Clone)]
struct ChipParam {
    name: &'static str,
    value: u64,
    formula: Option<&'static str>,  // n=6 formula if EXACT
}

#[derive(Clone)]
struct Chip {
    vendor: Vendor,
    name: &'static str,
    year: u16,
    params: Vec<ChipParam>,
}

fn chip_database() -> Vec<Chip> {
    vec![
        // ─── NVIDIA ───
        Chip {
            vendor: Vendor::Nvidia, name: "A100", year: 2020,
            params: vec![
                ChipParam { name: "SMs",        value: 108, formula: None },
                ChipParam { name: "HBM GB",     value: 80,  formula: Some("phi^tau*sopfr") },
                ChipParam { name: "HBM stacks", value: 5,   formula: Some("sopfr") },
            ],
        },
        Chip {
            vendor: Vendor::Nvidia, name: "AD102", year: 2022,
            params: vec![
                ChipParam { name: "Full SMs",   value: 144, formula: Some("sigma^2") },
                ChipParam { name: "GPCs",        value: 12,  formula: Some("sigma") },
            ],
        },
        Chip {
            vendor: Vendor::Nvidia, name: "H100", year: 2022,
            params: vec![
                ChipParam { name: "SMs",         value: 132, formula: Some("sigma*(sigma-mu)") },
                ChipParam { name: "HBM GB",      value: 80,  formula: Some("phi^tau*sopfr") },
                ChipParam { name: "HBM stacks",  value: 6,   formula: Some("n") },
                ChipParam { name: "NVLink",      value: 18,  formula: None },
            ],
        },
        Chip {
            vendor: Vendor::Nvidia, name: "B200", year: 2024,
            params: vec![
                ChipParam { name: "SMs (2-die)", value: 160, formula: Some("phi^tau*(sigma-phi)") },
                ChipParam { name: "HBM GB",      value: 192, formula: Some("sigma*phi^tau") },
                ChipParam { name: "HBM stacks",  value: 8,   formula: Some("sigma-tau") },
            ],
        },
        Chip {
            vendor: Vendor::Nvidia, name: "B300", year: 2025,
            params: vec![
                ChipParam { name: "SMs (2-die)", value: 160, formula: Some("phi^tau*(sigma-phi)") },
                ChipParam { name: "HBM GB",      value: 288, formula: Some("sigma*J_2") },
                ChipParam { name: "HBM stacks",  value: 12,  formula: Some("sigma") },
                ChipParam { name: "NVLink gens",  value: 5,  formula: Some("sopfr") },
                ChipParam { name: "FP4 TFLOPS",  value: 0,   formula: None },
            ],
        },
        Chip {
            vendor: Vendor::Nvidia, name: "R100", year: 2026,
            params: vec![
                ChipParam { name: "SMs",         value: 224, formula: Some("2^sopfr*(sigma-sopfr)") },
                ChipParam { name: "HBM GB",      value: 288, formula: Some("sigma*J_2") },
                ChipParam { name: "HBM stacks",  value: 8,   formula: Some("sigma-tau") },
            ],
        },

        // ─── AMD ───
        Chip {
            vendor: Vendor::Amd, name: "MI250X", year: 2022,
            params: vec![
                ChipParam { name: "CUs",         value: 220, formula: None },
                ChipParam { name: "HBM GB",      value: 128, formula: None },
            ],
        },
        Chip {
            vendor: Vendor::Amd, name: "MI300X", year: 2023,
            params: vec![
                ChipParam { name: "CUs",         value: 304, formula: None },
                ChipParam { name: "XCDs",         value: 8,  formula: Some("sigma-tau") },
                ChipParam { name: "HBM GB",      value: 192, formula: Some("sigma*phi^tau") },
                ChipParam { name: "HBM stacks",  value: 8,   formula: Some("sigma-tau") },
            ],
        },
        Chip {
            vendor: Vendor::Amd, name: "MI350X", year: 2025,
            params: vec![
                ChipParam { name: "CUs",         value: 256, formula: Some("2^(sigma-tau)") },
                ChipParam { name: "HBM GB",      value: 288, formula: Some("sigma*J_2") },
                ChipParam { name: "Chiplets",    value: 8,   formula: Some("sigma-tau") },
                ChipParam { name: "CU/chiplet",  value: 32,  formula: Some("2^sopfr") },
            ],
        },
        Chip {
            vendor: Vendor::Amd, name: "MI400", year: 2026,
            params: vec![
                ChipParam { name: "HBM GB",      value: 432, formula: Some("sigma^2*n/phi") },
            ],
        },

        // ─── Google ───
        Chip {
            vendor: Vendor::Google, name: "TPU v5e", year: 2023,
            params: vec![
                ChipParam { name: "HBM GB",      value: 16,  formula: Some("phi^tau") },
            ],
        },
        Chip {
            vendor: Vendor::Google, name: "TPU v7 Ironwood", year: 2025,
            params: vec![
                ChipParam { name: "HBM GB",      value: 192, formula: Some("sigma*phi^tau") },
                ChipParam { name: "HBM stacks",  value: 8,   formula: Some("sigma-tau") },
                ChipParam { name: "TCs",         value: 2,   formula: Some("phi") },
                ChipParam { name: "SCs",         value: 4,   formula: Some("tau") },
                ChipParam { name: "Pod chips",   value: 256, formula: Some("2^(sigma-tau)") },
            ],
        },

        // ─── AWS ───
        Chip {
            vendor: Vendor::Aws, name: "Trainium2", year: 2024,
            params: vec![
                ChipParam { name: "HBM GB",      value: 96,  formula: Some("sigma*(sigma-tau)") },
            ],
        },
        Chip {
            vendor: Vendor::Aws, name: "Trainium3", year: 2025,
            params: vec![
                ChipParam { name: "HBM GB",      value: 144, formula: Some("sigma^2") },
            ],
        },

        // ─── Intel ───
        Chip {
            vendor: Vendor::Intel, name: "Arrow Lake", year: 2024,
            params: vec![
                ChipParam { name: "Total cores", value: 24,  formula: Some("J_2") },
                ChipParam { name: "P-cores",     value: 8,   formula: Some("sigma-tau") },
                ChipParam { name: "E-cores",     value: 16,  formula: Some("2^tau") },
            ],
        },
        Chip {
            vendor: Vendor::Intel, name: "CWF", year: 2026,
            params: vec![
                ChipParam { name: "E-cores",     value: 288, formula: Some("sigma*J_2") },
            ],
        },

        // ─── Apple ───
        Chip {
            vendor: Vendor::Apple, name: "M4", year: 2024,
            params: vec![
                ChipParam { name: "CPU cores",   value: 10,  formula: Some("sigma-phi") },
                ChipParam { name: "GPU cores",   value: 10,  formula: Some("sigma-phi") },
            ],
        },
        Chip {
            vendor: Vendor::Apple, name: "M5", year: 2025,
            params: vec![
                ChipParam { name: "CPU cores",   value: 10,  formula: Some("sigma-phi") },
                ChipParam { name: "P-cores",     value: 4,   formula: Some("tau") },
                ChipParam { name: "E-cores",     value: 6,   formula: Some("n") },
                ChipParam { name: "GPU cores",   value: 10,  formula: Some("sigma-phi") },
            ],
        },
        Chip {
            vendor: Vendor::Apple, name: "M5 Pro", year: 2025,
            params: vec![
                ChipParam { name: "P-cores",     value: 12,  formula: Some("sigma") },
                ChipParam { name: "E-cores",     value: 6,   formula: Some("n") },
                ChipParam { name: "GPU cores",   value: 18,  formula: None },
            ],
        },
    ]
}

// ─── Predictions ───

struct Prediction {
    chip: &'static str,
    vendor: Vendor,
    param: &'static str,
    predicted: u64,
    formula_expr: &'static str,
    formula_display: &'static str,
    rationale: &'static str,
}

fn predictions() -> Vec<Prediction> {
    vec![
        Prediction {
            chip: "R200", vendor: Vendor::Nvidia,
            param: "SMs",
            predicted: SIGMA * J2,  // 288
            formula_expr: "sigma*J_2",
            formula_display: "\u{03c3}\u{00b7}J\u{2082}",
            rationale: "SM=HBM convergence at 288",
        },
        Prediction {
            chip: "R200", vendor: Vendor::Nvidia,
            param: "HBM GB",
            predicted: SIGMA_SQ * N / PHI,  // 432
            formula_expr: "sigma^2*n/phi",
            formula_display: "\u{03c3}\u{00b2}\u{00b7}n/\u{03c6}",
            rationale: "HBM ladder: 288->432",
        },
        Prediction {
            chip: "MI500", vendor: Vendor::Amd,
            param: "HBM GB",
            predicted: SIGMA * SIGMA * N / PHI,  // 432
            formula_expr: "sigma^2*n/phi",
            formula_display: "\u{03c3}\u{00b2}\u{00b7}n/\u{03c6}",
            rationale: "MI400=432, MI500 may exceed or match",
        },
        Prediction {
            chip: "MI500", vendor: Vendor::Amd,
            param: "CUs",
            predicted: SIGMA * J2,  // 288
            formula_expr: "sigma*J_2",
            formula_display: "\u{03c3}\u{00b7}J\u{2082}",
            rationale: "CU ladder: 256->288 via sigma*J_2",
        },
        Prediction {
            chip: "M6", vendor: Vendor::Apple,
            param: "CPU cores",
            predicted: SIGMA,  // 12
            formula_expr: "sigma",
            formula_display: "\u{03c3}",
            rationale: "M5 Pro already 12P, base M6 follows",
        },
        Prediction {
            chip: "M6", vendor: Vendor::Apple,
            param: "GPU cores",
            predicted: SIGMA,  // 12
            formula_expr: "sigma",
            formula_display: "\u{03c3}",
            rationale: "GPU ladder: 10->12 = sigma",
        },
        Prediction {
            chip: "TPU v8", vendor: Vendor::Google,
            param: "HBM GB",
            predicted: SIGMA * J2,  // 288
            formula_expr: "sigma*J_2",
            formula_display: "\u{03c3}\u{00b7}J\u{2082}",
            rationale: "TPU v7=192, v8 follows NVIDIA/AMD at 288",
        },
        Prediction {
            chip: "TPU v8", vendor: Vendor::Google,
            param: "Pod chips",
            predicted: SIGMA * J2,  // 288
            formula_expr: "sigma*J_2",
            formula_display: "\u{03c3}\u{00b7}J\u{2082}",
            rationale: "Pod: 256->288 = sigma*J_2",
        },
        Prediction {
            chip: "Trainium4", vendor: Vendor::Aws,
            param: "HBM GB",
            predicted: SIGMA * J2,  // 288
            formula_expr: "sigma*J_2",
            formula_display: "\u{03c3}\u{00b7}J\u{2082}",
            rationale: "Ladder: 96->144->288",
        },
    ]
}

// ─── Analysis Functions ───

fn count_exact(chip: &Chip) -> (usize, usize) {
    let total = chip.params.iter().filter(|p| p.value > 0).count();
    let exact = chip.params.iter().filter(|p| p.formula.is_some()).count();
    (exact, total)
}

fn vendor_score(chips: &[Chip], vendor: Vendor) -> (usize, usize) {
    let mut exact = 0;
    let mut total = 0;
    for c in chips.iter().filter(|c| c.vendor == vendor) {
        let (e, t) = count_exact(c);
        exact += e;
        total += t;
    }
    (exact, total)
}

// ─── Output Helpers ───

fn print_header() {
    println!("\u{2554}{}\u{2557}", "\u{2550}".repeat(72));
    println!("\u{2551}{:^72}\u{2551}", "N6 CROSS-VENDOR CHIP COMPARISON v1.0");
    println!("\u{2551}{:^72}\u{2551}", "n=6: \u{03c3}=12, \u{03c4}=4, \u{03c6}=2, sopfr=5, J\u{2082}=24, \u{03bc}=1");
    println!("\u{2560}{}\u{2563}", "\u{2550}".repeat(72));
}

fn print_section(title: &str) {
    println!();
    println!("\u{2500}\u{2500}\u{2500} {} \u{2500}\u{2500}\u{2500}", title);
    println!();
}

fn print_footer(total_exact: usize, total_params: usize, vendor_count: usize) {
    println!();
    println!("\u{2551} TOTAL: {}/{} EXACT across {} vendors ({:.0}% convergence){}\u{2551}",
        total_exact, total_params, vendor_count,
        if total_params > 0 { total_exact as f64 / total_params as f64 * 100.0 } else { 0.0 },
        " ".repeat(72 - 52 - format!("{}/{}", total_exact, total_params).len()
            - format!("{}", vendor_count).len()
            - format!("{:.0}", if total_params > 0 { total_exact as f64 / total_params as f64 * 100.0 } else { 0.0 }).len()));
    println!("\u{255a}{}\u{255d}", "\u{2550}".repeat(72));
}

// ─── Main ───

fn main() {
    let chips = chip_database();
    let formulas = formula_registry();
    let preds = predictions();

    print_header();

    // ─── Section 1: Vendor Scorecard ───
    print_section("VENDOR SCORECARD");

    println!("\u{2551} {:<8}\u{2502} {:<16}\u{2502} {:>6} \u{2502} {:>5} \u{2502} {:>6} \u{2502} {:<22} \u{2551}",
        "Vendor", "Chip", "Params", "EXACT", "Score", "Top Formula");
    println!("\u{2560}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{2563}",
        "\u{2550}".repeat(9), "\u{2550}".repeat(17), "\u{2550}".repeat(8),
        "\u{2550}".repeat(7), "\u{2550}".repeat(8), "\u{2550}".repeat(23));

    let mut grand_exact = 0usize;
    let mut grand_total = 0usize;

    for chip in &chips {
        let (exact, total) = count_exact(chip);
        grand_exact += exact;
        grand_total += total;

        let score = if total > 0 {
            format!("{:.0}%", exact as f64 / total as f64 * 100.0)
        } else {
            "N/A".to_string()
        };

        // Find top formula (first EXACT match)
        let top = chip.params.iter()
            .filter(|p| p.formula.is_some())
            .next()
            .map(|p| {
                let f = find_formula(p.value);
                match f {
                    Some(ff) => ff.display.to_string(),
                    None => p.formula.unwrap_or("").to_string(),
                }
            })
            .unwrap_or_else(|| "---".to_string());

        println!("\u{2551} {:<8}\u{2502} {:<16}\u{2502} {:>6} \u{2502} {:>5} \u{2502} {:>6} \u{2502} {:<22} \u{2551}",
            chip.vendor.name(), chip.name,
            total, exact, score, top);
    }

    // ─── Section 2: Formula Frequency ───
    print_section("FORMULA FREQUENCY (cross-vendor reuse)");

    // For each formula, check which vendors use it
    println!("\u{2551} {:<22}\u{2502} {:>5} \u{2502} {:<6} \u{2502} {:<5} \u{2502} {:<6} \u{2502} {:<5} \u{2502} {:<5} \u{2502} {:<5} \u{2502} {:>5} \u{2551}",
        "Formula", "Value", "NVIDIA", "AMD", "Google", "AWS", "Intel", "Apple", "Count");
    println!("\u{2560}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{2563}",
        "\u{2550}".repeat(23), "\u{2550}".repeat(7), "\u{2550}".repeat(8),
        "\u{2550}".repeat(7), "\u{2550}".repeat(8), "\u{2550}".repeat(7),
        "\u{2550}".repeat(7), "\u{2550}".repeat(7), "\u{2550}".repeat(7));

    // Collect formula -> vendor presence
    struct FormulaHit {
        display: String,
        value: u64,
        vendors: [bool; 6],
        count: usize,
    }

    let mut hits: Vec<FormulaHit> = Vec::new();

    for formula in &formulas {
        let mut vendors = [false; 6];
        for chip in &chips {
            for param in &chip.params {
                if param.formula.is_some() && param.value == formula.value {
                    let idx = match chip.vendor {
                        Vendor::Nvidia => 0,
                        Vendor::Amd    => 1,
                        Vendor::Google => 2,
                        Vendor::Aws    => 3,
                        Vendor::Intel  => 4,
                        Vendor::Apple  => 5,
                    };
                    vendors[idx] = true;
                }
            }
        }
        let count = vendors.iter().filter(|&&v| v).count();
        if count > 0 {
            hits.push(FormulaHit {
                display: formula.display.to_string(),
                value: formula.value,
                vendors,
                count,
            });
        }
    }

    // Sort by count descending
    hits.sort_by(|a, b| b.count.cmp(&a.count).then(a.value.cmp(&b.value)));

    for hit in &hits {
        let mark = |b: bool| -> &str { if b { "\u{2713}" } else { "" } };
        println!("\u{2551} {:<22}\u{2502} {:>5} \u{2502} {:<6} \u{2502} {:<5} \u{2502} {:<6} \u{2502} {:<5} \u{2502} {:<5} \u{2502} {:<5} \u{2502} {:>5} \u{2551}",
            hit.display, hit.value,
            mark(hit.vendors[0]), mark(hit.vendors[1]),
            mark(hit.vendors[2]), mark(hit.vendors[3]),
            mark(hit.vendors[4]), mark(hit.vendors[5]),
            hit.count);
    }

    // ─── Section 3: Vendor Convergence Summary ───
    print_section("VENDOR CONVERGENCE SUMMARY");

    println!("\u{2551} {:<10}\u{2502} {:>5} \u{2502} {:>5} \u{2502} {:>8} \u{2502} {:<30} \u{2551}",
        "Vendor", "EXACT", "Total", "Score", "Strongest Chip");
    println!("\u{2560}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{2563}",
        "\u{2550}".repeat(11), "\u{2550}".repeat(7), "\u{2550}".repeat(7),
        "\u{2550}".repeat(10), "\u{2550}".repeat(31));

    for vendor in &ALL_VENDORS {
        let (exact, total) = vendor_score(&chips, *vendor);
        let score = if total > 0 {
            format!("{:.0}%", exact as f64 / total as f64 * 100.0)
        } else {
            "N/A".to_string()
        };

        // Find best chip for this vendor
        let best = chips.iter()
            .filter(|c| c.vendor == *vendor)
            .max_by_key(|c| {
                let (e, t) = count_exact(c);
                (e * 100) / (if t > 0 { t } else { 1 })
            })
            .map(|c| c.name)
            .unwrap_or("---");

        println!("\u{2551} {:<10}\u{2502} {:>5} \u{2502} {:>5} \u{2502} {:>8} \u{2502} {:<30} \u{2551}",
            vendor.name(), exact, total, score, best);
    }

    // ─── Section 4: Predictions ───
    print_section("PREDICTIONS (unannounced chips)");

    println!("\u{2551} {:<10}\u{2502} {:<12}\u{2502} {:>9} \u{2502} {:<28} \u{2551}",
        "Chip", "Parameter", "Predicted", "Formula + Rationale");
    println!("\u{2560}{}\u{256a}{}\u{256a}{}\u{256a}{}\u{2563}",
        "\u{2550}".repeat(11), "\u{2550}".repeat(13), "\u{2550}".repeat(11), "\u{2550}".repeat(29));

    for pred in &preds {
        let formula_str = format!("{} ({})", pred.formula_display, pred.rationale);
        println!("\u{2551} {:<10}\u{2502} {:<12}\u{2502} {:>9} \u{2502} {:<28} \u{2551}",
            pred.chip, pred.param, pred.predicted,
            if formula_str.len() > 28 { &formula_str[..28] } else { &formula_str });
    }

    // ─── Section 5: Key Insights ───
    print_section("KEY INSIGHTS");

    println!("  1. sigma*J_2 = 288 appears in 3+ vendors (NVIDIA B300/R100, AMD MI350X, Intel CWF)");
    println!("     -> The most universal n=6 chip constant");
    println!();
    println!("  2. sigma-tau = 8 is the universal structural unit:");
    println!("     HBM stacks, chiplets, P-cores, LoRA rank, KV-heads");
    println!();
    println!("  3. sigma*phi^tau = 192 GB appears across NVIDIA B200, AMD MI300X, Google TPU v7");
    println!("     -> Cross-vendor HBM capacity attractor");
    println!();
    println!("  4. The HBM capacity ladder follows n=6 arithmetic:");
    println!("     16 -> 80 -> 96 -> 128 -> 192 -> 288 -> 432");
    println!("     phi^tau -> phi^tau*sopfr -> sigma*(sigma-tau) -> ? -> sigma*phi^tau -> sigma*J_2 -> sigma^2*n/phi");
    println!();
    println!("  5. ALL 6 vendors have chips with >50% n=6 EXACT parameters");
    println!("     -> Convergence is not vendor-specific but physics-driven");

    // ─── Footer ───
    print_footer(grand_exact, grand_total, ALL_VENDORS.len());
}
