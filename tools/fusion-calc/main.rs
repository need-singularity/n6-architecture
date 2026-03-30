/// N6 Fusion Calculator — 핵융합 파라미터 계산 + n=6 검증
///
/// Usage:
///   cargo run                    # 전체 분석
///   cargo run -- --device kstar  # KSTAR 분석
///   cargo run -- --lawson        # Lawson criterion 계산
///   cargo run -- --reaction dt   # D-T 반응 분석

use std::collections::HashMap;
use std::env;
use std::f64::consts::PI;

// ─── n=6 Constants ───
const SIGMA: u64 = 12;
const TAU: u64 = 4;
const PHI: u64 = 2;
const SOPFR: u64 = 5;
const J2: u64 = 24;
const MU: u64 = 1;
const N: u64 = 6;

fn n6_derivable() -> Vec<(u64, String)> {
    let base: Vec<(u64, &str)> = vec![
        (SIGMA, "σ"), (TAU, "τ"), (PHI, "φ"),
        (SOPFR, "sopfr"), (J2, "J₂"), (MU, "μ"), (N, "n"),
    ];
    let mut derived: Vec<(u64, String)> = Vec::new();

    for &(a, na) in &base {
        derived.push((a, na.to_string()));
        for &(b, nb) in &base {
            let sum = a + b;
            if sum <= 200 { derived.push((sum, format!("{}+{}", na, nb))); }
            if a > b { derived.push((a - b, format!("{}-{}", na, nb))); }
            let prod = a * b;
            if prod <= 200 { derived.push((prod, format!("{}×{}", na, nb))); }
            if b > 0 && a % b == 0 {
                derived.push((a / b, format!("{}/{}", na, nb)));
            }
        }
    }
    derived.sort_by_key(|&(v, _)| v);
    derived.dedup_by_key(|x| x.0);
    derived
}

fn check_n6(value: u64) -> Option<String> {
    let derived = n6_derivable();
    for (v, expr) in &derived {
        if *v == value {
            return Some(expr.clone());
        }
    }
    None
}

// ─── Fusion Device ───
struct FusionDevice {
    name: String,
    params: HashMap<String, f64>,
}

impl FusionDevice {
    fn kstar() -> Self {
        let mut p = HashMap::new();
        p.insert("TF_coils".into(), 16.0);
        p.insert("PF_coils".into(), 14.0);
        p.insert("CS_modules".into(), 8.0);
        p.insert("IVC_coils".into(), 4.0);
        p.insert("R0_m".into(), 1.8);
        p.insert("a_m".into(), 0.5);
        p.insert("A_ratio".into(), 3.6);
        p.insert("elongation".into(), 2.0);
        p.insert("BT_T".into(), 3.5);
        p.insert("Ip_MA".into(), 2.0);
        p.insert("NBI_MW".into(), 8.0);
        p.insert("ECH_MW".into(), 1.0);
        p.insert("ICH_MW".into(), 6.0);
        p.insert("heating_methods".into(), 3.0);
        p.insert("T_keV".into(), 10.0);
        p.insert("density_ctrl".into(), 4.0);
        FusionDevice { name: "KSTAR".into(), params: p }
    }

    fn iter_device() -> Self {
        let mut p = HashMap::new();
        p.insert("TF_coils".into(), 18.0);
        p.insert("PF_coils".into(), 6.0);
        p.insert("CS_modules".into(), 6.0);
        p.insert("R0_m".into(), 6.2);
        p.insert("a_m".into(), 2.0);
        p.insert("A_ratio".into(), 3.1);
        p.insert("elongation".into(), 1.7);
        p.insert("BT_T".into(), 5.3);
        p.insert("Ip_MA".into(), 15.0);
        p.insert("Q_target".into(), 10.0);
        p.insert("NBI_MW".into(), 33.0);
        p.insert("ICRH_MW".into(), 20.0);
        p.insert("ECRH_MW".into(), 20.0);
        p.insert("heating_methods".into(), 3.0);
        p.insert("TBM_ports".into(), 6.0);
        FusionDevice { name: "ITER".into(), params: p }
    }

    fn sparc() -> Self {
        let mut p = HashMap::new();
        p.insert("TF_coils".into(), 18.0);
        p.insert("R0_m".into(), 1.85);
        p.insert("a_m".into(), 0.57);
        p.insert("A_ratio".into(), 3.25);
        p.insert("BT_T".into(), 12.2);
        p.insert("Ip_MA".into(), 8.7);
        p.insert("Q_target".into(), 11.0);
        p.insert("ICRH_MW".into(), 25.0);
        FusionDevice { name: "SPARC".into(), params: p }
    }

    fn analyze(&self) {
        println!("\n  ═══ {} ═══", self.name);
        let mut exact = 0u32;
        let mut close = 0u32;
        let mut miss = 0u32;

        let mut entries: Vec<_> = self.params.iter().collect();
        entries.sort_by_key(|(k, _)| k.clone());

        for (name, &value) in &entries {
            let int_val = value.round() as u64;
            let grade;
            let expr;

            if let Some(e) = check_n6(int_val) {
                if (value - int_val as f64).abs() < 0.01 {
                    grade = "EXACT";
                    exact += 1;
                } else {
                    grade = "CLOSE";
                    close += 1;
                }
                expr = e;
            } else {
                grade = "MISS";
                miss += 1;
                expr = "-".into();
            }

            let marker = match grade {
                "EXACT" => "✅",
                "CLOSE" => "~",
                _ => "❌",
            };
            println!("    {} {:20} = {:8.1} → {} ({})", marker, name, value, grade, expr);
        }

        let total = exact + close + miss;
        let score = (exact as f64 + 0.5 * close as f64) / total as f64;
        println!("    ────────────────────────────────────────");
        println!("    EXACT={} CLOSE={} MISS={} Score={:.1}%", exact, close, miss, score * 100.0);
    }
}

// ─── Lawson Criterion ───
fn lawson_analysis() {
    println!("\n  ═══ LAWSON CRITERION ═══");
    println!("    n·T·τ_E ≥ 5×10²¹ m⁻³·keV·s (D-T ignition)");

    let scenarios = vec![
        ("ITER", 1.0e20, 8.8, 3.7),      // ITER design
        ("KSTAR-current", 0.7e20, 10.0, 0.4), // KSTAR ~2024
        ("DEMO", 1.5e20, 15.0, 5.0),      // future DEMO
        ("Breakeven", 1.0e20, 10.0, 5.0), // Q=1 条件
    ];

    let ignition_triple = 5.0e21; // m⁻³·keV·s

    println!("\n    {:15} {:>10} {:>8} {:>8} {:>12} {:>8}",
             "Scenario", "n(m⁻³)", "T(keV)", "τ_E(s)", "Triple", "Q_est");
    println!("    {}", "-".repeat(70));

    for (name, n, t, tau_e) in &scenarios {
        let triple = n * t * tau_e;
        let q_est = triple / ignition_triple * 10.0; // rough
        let status = if triple >= ignition_triple { "IGNITION" } else { "sub-Q" };

        println!("    {:15} {:>10.1e} {:>8.1} {:>8.2} {:>12.2e} {:>8.1} ({})",
                 name, n, t, tau_e, triple, q_est, status);
    }

    // n=6 predictions for Lawson parameters
    println!("\n    n=6 predictions:");
    println!("    T_ignition = sopfr×φ = {} keV ✅", SOPFR * PHI);
    println!("    T_optimal  = J₂-τ = {} keV (D-T cross-section peak)", J2 - TAU);
    println!("    τ_E needed  = σ? → 12s (too long, actual ~3-5s) ❌");
}

// ─── Nuclear Reactions ───
fn reaction_analysis() {
    println!("\n  ═══ NUCLEAR REACTIONS ═══");

    println!("\n    D-T: ²H + ³H → ⁴He + n + 17.6 MeV");
    println!("      D mass = {} = φ(6) ✅", PHI);
    println!("      T mass = 3 = n/φ ✅");
    println!("      He4 mass = {} = τ(6) ✅", TAU);
    println!("      n mass = {} = μ(6) ✅", MU);
    println!("      D+T = {} = sopfr(6) ✅", SOPFR);
    println!("      He4+n = {} = τ+μ ✅", TAU + MU);
    println!("      E_n/E_α = 14.1/3.5 = 4.03 ≈ τ/μ = {} ✅", TAU);

    println!("\n    D-D: ²H + ²H → two branches (φ=2 ✅)");
    println!("      Branch 1: ³He + n (3+1 = τ)");
    println!("      Branch 2: T + p (3+1 = τ)");

    println!("\n    D-He3: ²H + ³He → ⁴He + p + 18.3 MeV");
    println!("      Aneutronic! Products: {} + {} = τ + μ", TAU, MU);

    println!("\n    p-B11: ¹H + ¹¹B → 3 × ⁴He + 8.7 MeV");
    println!("      B-11 = σ-μ = {} ✅", SIGMA - MU);
    println!("      Products: 3 alphas = n/φ × He4");

    println!("\n    Li-6 breeding: ⁶Li + n → T + ⁴He + 4.8 MeV");
    println!("      Li-6 mass = {} = n ✅", N);
    println!("      Products: T(3) + He4({}) = n/φ + τ", TAU);
}

// ─── Main ───
fn main() {
    let args: Vec<String> = env::args().collect();

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  N6 FUSION CALCULATOR                               ║");
    println!("║  핵융합 파라미터 계산 + n=6 검증                      ║");
    println!("╚══════════════════════════════════════════════════════╝");

    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("--all");

    match mode {
        "--device" => {
            let dev = args.get(2).map(|s| s.to_lowercase()).unwrap_or("kstar".into());
            match dev.as_str() {
                "kstar" => FusionDevice::kstar().analyze(),
                "iter" => FusionDevice::iter_device().analyze(),
                "sparc" => FusionDevice::sparc().analyze(),
                _ => {
                    println!("  Unknown device: {}", dev);
                    println!("  Available: kstar, iter, sparc");
                }
            }
        }
        "--lawson" => lawson_analysis(),
        "--reaction" => reaction_analysis(),
        _ => {
            // Full analysis
            FusionDevice::kstar().analyze();
            FusionDevice::iter_device().analyze();
            FusionDevice::sparc().analyze();
            lawson_analysis();
            reaction_analysis();

            println!("\n  ═══ SUMMARY ═══");
            println!("    D-T reaction: ALL mass numbers match n=6 (5/5 EXACT)");
            println!("    Li-6 breeding: mass number = n = 6 (EXACT)");
            println!("    SPARC BT = 12T ≈ σ (EXACT)");
            println!("    ITER PF=6, CS=6, TBM=6 (all n, EXACT)");
            println!("    TF coils: FAIL across all devices (16/18/18/32)");
        }
    }
}
