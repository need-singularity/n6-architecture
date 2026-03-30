/// Tokamak Shape Optimizer — n=6 매개변수 공간 탐색
///
/// 6개 형태 매개변수(R₀, a, κ, δ, ξ, q₉₅)의 최적 조합을 탐색하여
/// n=6 상수와의 일치도 + 물리적 성능 지표를 동시에 평가
///
/// Usage:
///   cargo run              # 전체 탐색
///   cargo run -- --scan    # R-score scan over parameter space
///   cargo run -- --bench   # 벤치마크: n=6 설계 vs ITER vs KSTAR

use std::env;

// n=6 constants
const SIGMA: f64 = 12.0;
const TAU: f64 = 4.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const N6: f64 = 6.0;

/// Tokamak geometry parameters
#[derive(Clone, Debug)]
struct TokamakShape {
    name: String,
    r0: f64,     // major radius [m]
    a: f64,      // minor radius [m]
    kappa: f64,  // elongation
    delta: f64,  // triangularity
    xi: f64,     // squareness
    q95: f64,    // safety factor at 95% flux
    bt: f64,     // toroidal field [T]
    ip: f64,     // plasma current [MA]
}

impl TokamakShape {
    fn aspect_ratio(&self) -> f64 { self.r0 / self.a }
    fn epsilon(&self) -> f64 { self.a / self.r0 }
    fn plasma_volume(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.r0
            * std::f64::consts::PI * self.a * self.a * self.kappa
    }
    fn plasma_surface(&self) -> f64 {
        // Approximate: 4π²R₀a√((1+κ²)/2)
        let avg_k = ((1.0 + self.kappa * self.kappa) / 2.0).sqrt();
        4.0 * std::f64::consts::PI * std::f64::consts::PI * self.r0 * self.a * avg_k
    }

    // ─── Physics performance metrics ───

    /// Troyon beta limit: β_N ≈ C / (a·B_T/I_p)
    fn beta_n_limit(&self) -> f64 {
        // Troyon: β_max = C_T × I_p / (a × B_T), C_T ≈ 2.8-3.5
        let c_troyon = 3.0;
        c_troyon * self.ip / (self.a * self.bt) // [% m T / MA]
    }

    /// Greenwald density limit [10²⁰ m⁻³]
    fn greenwald_density(&self) -> f64 {
        self.ip / (std::f64::consts::PI * self.a * self.a)
    }

    /// Estimated confinement time (IPB98(y,2) scaling)
    fn tau_e_estimate(&self) -> f64 {
        // τ_E ∝ I_p^0.93 × B_T^0.15 × n^0.41 × P^{-0.69} × R^1.97 × ε^0.58 × κ^0.78 × M^0.19
        // Simplified: τ_E ≈ 0.0562 × I_p^0.93 × B_T^0.15 × R^1.97 × (a/R)^0.58 × κ^0.78
        // Using P = 50 MW, n = 1.0 × 10²⁰, M = 2.5 (DT)
        let h = 0.0562_f64;
        h * self.ip.powf(0.93)
          * self.bt.powf(0.15)
          * self.r0.powf(1.97)
          * self.epsilon().powf(0.58)
          * self.kappa.powf(0.78)
          * 50.0_f64.powf(-0.69) // P = 50 MW
          * 1.0_f64.powf(0.41)   // n = 1.0
          * 2.5_f64.powf(0.19)   // M = 2.5
    }

    /// Estimated Q (fusion gain)
    fn q_estimate(&self) -> f64 {
        // Very rough: Q ∝ β_N² × B_T⁴ × R₀³ / P_heating
        // Using simplified Lawson-based estimate
        let tau_e = self.tau_e_estimate();
        let n_gw = self.greenwald_density();
        let t_kev = 10.0; // assume 10 keV
        let triple = n_gw * t_kev * tau_e; // [10²⁰ m⁻³ × keV × s]
        // Q ~ triple / 5 (ignition at triple ≈ 5 × 10²¹)
        triple / 5.0 * 10.0 // convert units
    }

    /// n=6 match score: how many parameters match n=6 constants?
    fn n6_score(&self) -> (f64, Vec<String>) {
        let mut matches = Vec::new();
        let mut score = 0.0;

        let checks: Vec<(&str, f64, f64, &str)> = vec![
            ("R₀", self.r0, N6, "n"),
            ("a", self.a, PHI, "φ"),
            ("κ", self.kappa, PHI, "φ"),
            ("δ", self.delta, 1.0/3.0, "1/3"),
            ("A", self.aspect_ratio(), N6/PHI, "n/φ"),
            ("q₉₅", self.q95, SOPFR, "sopfr"),
            ("B_T", self.bt, SIGMA, "σ"),
        ];

        for (name, actual, target, expr) in checks {
            let err = (actual - target).abs() / target;
            if err < 0.01 {
                score += 1.0;
                matches.push(format!("{} = {} (EXACT)", name, expr));
            } else if err < 0.1 {
                score += 0.5;
                matches.push(format!("{} ≈ {} ({:.0}%)", name, expr, err * 100.0));
            }
        }
        (score, matches)
    }
}

fn known_devices() -> Vec<TokamakShape> {
    vec![
        TokamakShape {
            name: "ITER".into(),
            r0: 6.2, a: 2.0, kappa: 1.7, delta: 0.33, xi: 0.0,
            q95: 3.0, bt: 5.3, ip: 15.0,
        },
        TokamakShape {
            name: "KSTAR".into(),
            r0: 1.8, a: 0.5, kappa: 2.0, delta: 0.5, xi: 0.0,
            q95: 5.0, bt: 3.5, ip: 2.0,
        },
        TokamakShape {
            name: "SPARC".into(),
            r0: 1.85, a: 0.57, kappa: 1.97, delta: 0.54, xi: 0.0,
            q95: 3.4, bt: 12.2, ip: 8.7,
        },
        TokamakShape {
            name: "ARC".into(),
            r0: 3.3, a: 1.1, kappa: 1.8, delta: 0.5, xi: 0.0,
            q95: 4.5, bt: 9.2, ip: 7.8,
        },
        // N6 optimal design
        TokamakShape {
            name: "N6-DESIGN".into(),
            r0: 6.0, a: 2.0, kappa: 2.0, delta: 0.333, xi: 0.0,
            q95: 5.0, bt: 12.0, ip: 10.0,
        },
    ]
}

fn benchmark(devices: &[TokamakShape]) {
    println!("\n  ═══════════════════════════════════════════════════════════════════════");
    println!("  {:10} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>5} {:>8} {:>8} {:>6} {:>6}",
             "Device", "R₀", "a", "κ", "δ", "A", "q₉₅", "B_T",
             "Vol(m³)", "τ_E(s)", "Q_est", "N6_sc");
    println!("  {}", "─".repeat(85));

    for d in devices {
        let (n6_sc, _) = d.n6_score();
        println!("  {:10} {:>5.1} {:>5.1} {:>5.2} {:>5.2} {:>5.1} {:>5.1} {:>5.1} {:>8.1} {:>8.3} {:>6.1} {:>6.1}",
                 d.name, d.r0, d.a, d.kappa, d.delta,
                 d.aspect_ratio(), d.q95, d.bt,
                 d.plasma_volume(), d.tau_e_estimate(),
                 d.q_estimate(), n6_sc);
    }
}

fn parameter_scan() {
    println!("\n  ═══ PARAMETER SCAN: n=6 score vs physics performance ═══\n");

    let mut results: Vec<(f64, f64, String)> = Vec::new(); // (n6_score, Q, name)

    // Scan over key parameters
    for &a in &[1.0, 1.5, 2.0, 2.5] {
        for &kappa in &[1.5, 1.7, 2.0, 2.2] {
            for &bt in &[5.0, 8.0, 10.0, 12.0] {
                for &q95 in &[3.0, 4.0, 5.0, 6.0] {
                    let r0 = 3.0 * a; // fix A = 3 = n/φ
                    let ip = 2.0 * std::f64::consts::PI * a * a * bt / (5.0 * q95 * r0) * 1000.0;
                    // Rough: I_p ≈ 5 × a² × B_T / (q × R₀)
                    let ip_simple = 5.0 * a * a * bt / (q95 * r0);

                    let d = TokamakShape {
                        name: format!("a{:.0}k{:.0}b{:.0}q{:.0}", a*10.0, kappa*10.0, bt, q95),
                        r0, a, kappa, delta: 0.333, xi: 0.0, q95, bt, ip: ip_simple,
                    };

                    let (n6_sc, _) = d.n6_score();
                    let q_est = d.q_estimate();
                    results.push((n6_sc, q_est, d.name.clone()));
                }
            }
        }
    }

    // Sort by n=6 score, then Q
    results.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap().then(b.1.partial_cmp(&a.1).unwrap()));

    println!("  {:>6} {:>8} {:30}", "N6_sc", "Q_est", "Config");
    println!("  {}", "─".repeat(48));
    for (n6, q, name) in results.iter().take(20) {
        let marker = if *n6 >= 4.0 { " ◄── HIGH N6" } else { "" };
        println!("  {:>6.1} {:>8.1} {:30}{}", n6, q, name, marker);
    }

    // Find Pareto front: best Q for each n6 score level
    println!("\n  ═══ PARETO FRONT: n=6 score vs Q ═══\n");
    let mut pareto: Vec<(f64, f64, String)> = Vec::new();
    let mut best_q_at_score: std::collections::HashMap<i32, (f64, String)> = std::collections::HashMap::new();

    for (n6, q, name) in &results {
        let key = (*n6 * 2.0) as i32; // bucket by 0.5
        let entry = best_q_at_score.entry(key).or_insert((0.0, String::new()));
        if *q > entry.0 {
            *entry = (*q, name.clone());
        }
    }

    let mut pareto_sorted: Vec<_> = best_q_at_score.into_iter().collect();
    pareto_sorted.sort_by_key(|(k, _)| *k);

    println!("  {:>6} {:>8} {:30}", "N6_sc", "Best_Q", "Config");
    println!("  {}", "─".repeat(48));
    for (key, (q, name)) in &pareto_sorted {
        let n6 = *key as f64 / 2.0;
        println!("  {:>6.1} {:>8.1} {:30}", n6, q, name);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("--all");

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  TOKAMAK SHAPE OPTIMIZER                            ║");
    println!("║  n=6 매개변수 공간 탐색 + 물리 성능 벤치마크           ║");
    println!("╚══════════════════════════════════════════════════════╝");

    let devices = known_devices();

    match mode {
        "--bench" => benchmark(&devices),
        "--scan" => parameter_scan(),
        _ => {
            benchmark(&devices);
            parameter_scan();

            // N6 design analysis
            println!("\n  ═══ N6 DESIGN ANALYSIS ═══\n");
            let n6 = &devices[4]; // N6-DESIGN
            let (score, matches) = n6.n6_score();
            println!("  N6 Design: R₀={}, a={}, κ={}, δ={}, q₉₅={}, B_T={}T",
                     n6.r0, n6.a, n6.kappa, n6.delta, n6.q95, n6.bt);
            println!("  Volume: {:.1} m³", n6.plasma_volume());
            println!("  τ_E estimate: {:.3} s", n6.tau_e_estimate());
            println!("  Q estimate: {:.1}", n6.q_estimate());
            println!("  N6 match score: {:.1}/7", score);
            println!("  Matches:");
            for m in &matches {
                println!("    ✅ {}", m);
            }

            println!("\n  ═══ KEY FINDING ═══");
            println!("  n=6 최적 설계 (A=3, κ=2, δ=1/3, B_T=12T)가");
            println!("  ITER/SPARC 수준의 Q를 달성할 수 있는지?");
            println!("  → τ_E와 Q 추정치로 판단");
        }
    }
}
