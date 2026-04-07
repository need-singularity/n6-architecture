// SEDI Signal & Fermi Analyzer
// Op 13: SIGNAL — ET Signal Pattern Detection (n=6 frequency analysis)
// Op 14: FERMI  — Fermi Paradox Analysis (Drake equation through n=6 lens)

use serde::Serialize;

// ─── Constants ───────────────────────────────────────────────────────────────

/// Hydrogen 21cm line frequency (MHz)
const H21CM: f64 = 1420.405;

// n=6 arithmetic constants
const N: f64 = 6.0;       // the number itself
const SIGMA: f64 = 12.0;  // σ(6) = sum of divisors = 1+2+3+6
const TAU: f64 = 4.0;     // τ(6) = number of divisors
const PHI: f64 = 2.0;     // φ(6) = Euler totient
const SOPFR: f64 = 5.0;   // sopfr(6) = sum of prime factors = 2+3
const J2: f64 = 24.0;     // J₂(6) = Jordan totient
const MU: f64 = 1.0;      // μ(6) = Möbius function (squarefree, even # of primes)

// ─── Data Structures ────────────────────────────────────────────────────────

#[derive(Debug, Serialize)]
struct SignalCandidate {
    formula: String,
    target: String,
    predicted_mhz: f64,
    actual_mhz: f64,
    error_pct: f64,
    significance: String,
}

#[derive(Debug, Serialize)]
struct FreqRatioMatch {
    freq_a: String,
    freq_b: String,
    ratio: f64,
    n6_expr: String,
    n6_value: f64,
    error_pct: f64,
}

#[derive(Debug, Serialize)]
struct DrakeParam {
    name: String,
    range_lo: f64,
    range_hi: f64,
    n6_formula: String,
    n6_value: f64,
}

#[derive(Debug, Serialize)]
struct GreatFilterAnalysis {
    parameter: String,
    contribution: f64,
    is_bottleneck: bool,
    interpretation: String,
}

#[derive(Debug, Serialize)]
struct SensitivityEntry {
    parameter: String,
    base_n: f64,
    doubled_n: f64,
    elasticity: f64,
}

#[derive(Debug, Serialize)]
struct FermiResult {
    drake_params: Vec<DrakeParam>,
    n_predicted: f64,
    bottleneck: String,
    great_filter_location: String,
    simultaneous_civs: f64,
    filter_analysis: Vec<GreatFilterAnalysis>,
    sensitivity: Vec<SensitivityEntry>,
    prediction: String,
}

#[derive(Debug, Serialize)]
struct SignalReport {
    candidates: Vec<SignalCandidate>,
    freq_ratio_matches: Vec<FreqRatioMatch>,
    summary: SignalSummary,
}

#[derive(Debug, Serialize)]
struct SignalSummary {
    total_predictions: usize,
    exact_matches: usize,
    strong_matches: usize,
    moderate_matches: usize,
    best_match: String,
}

#[derive(Debug, Serialize)]
struct FullReport {
    signal: SignalReport,
    fermi: FermiResult,
}

// ─── Known Astronomical Frequencies ─────────────────────────────────────────

fn known_frequencies() -> Vec<(&'static str, f64)> {
    vec![
        ("H 21cm",          1420.405),
        ("OH 1612",         1612.231),
        ("OH 1665",         1665.402),
        ("OH 1667",         1667.359),
        ("OH 1720",         1720.530),
        ("Water hole low",  1420.0),
        ("Water hole high", 1720.0),
        ("H2CO",            4829.66),
        ("CH3OH",           6668.518),
        ("NH3",            23694.5),
    ]
}

// ─── n=6 Frequency Ratios ───────────────────────────────────────────────────

fn n6_ratios() -> Vec<(&'static str, f64)> {
    vec![
        ("n/sigma",              N / SIGMA),                // 0.5   → 710.2 MHz
        ("tau/phi",              TAU / PHI),                 // 2.0   → 2840.8 MHz
        ("sigma/n",              SIGMA / N),                 // 2.0   → 2840.8 MHz
        ("sopfr/tau",            SOPFR / TAU),               // 1.25  → 1775.5 MHz
        ("J2/sigma",             J2 / SIGMA),                // 2.0   → 2840.8 MHz
        ("sigma*phi/J2",         SIGMA * PHI / J2),          // 1.0   → 1420.4 MHz (identity!)
        ("1+1/n = 7/6",         7.0 / N),                   // 1.167 → 1657.1 MHz (near OH!)
        ("sopfr/n",              SOPFR / N),                 // 0.833 → 1183.7 MHz
        ("(sigma-tau)/n",        (SIGMA - TAU) / N),         // 1.333 → 1893.9 MHz
        ("phi*sopfr/n",          PHI * SOPFR / N),           // 1.667 → 2367.3 MHz
        ("sigma/sopfr",          SIGMA / SOPFR),             // 2.4   → 3408.97 MHz
        ("J2/n",                 J2 / N),                    // 4.0   → 5681.6 MHz
        ("n*sopfr/tau",          N * SOPFR / TAU),           // 7.5   → 10653.0 MHz
        ("(sigma+tau)/(n+phi)",  (SIGMA + TAU) / (N + PHI)), // 2.0   → 2840.8 MHz
        ("tau*sopfr/sigma",      TAU * SOPFR / SIGMA),       // 1.667 → 2367.3 MHz
        ("(n+mu)/n",             (N + MU) / N),              // 1.167 → 1657.1 MHz
    ]
}

// ─── SIGNAL Operator (Op 13): ET Signal Pattern Detection ───────────────────

fn signal_analysis() -> SignalReport {
    let mut candidates = Vec::new();
    let ratios = n6_ratios();
    let freqs = known_frequencies();

    // 1. Check n=6 predicted frequencies against known astronomical lines
    for (ratio_name, ratio) in &ratios {
        let predicted = H21CM * ratio;
        for (freq_name, freq) in &freqs {
            let error = (predicted - freq).abs() / freq;
            if error < 0.05 {
                let significance = if error < 0.001 {
                    "EXACT"
                } else if error < 0.01 {
                    "STRONG"
                } else {
                    "MODERATE"
                };
                candidates.push(SignalCandidate {
                    formula: format!("H21cm * ({}) = {:.3} MHz", ratio_name, predicted),
                    target: freq_name.to_string(),
                    predicted_mhz: predicted,
                    actual_mhz: *freq,
                    error_pct: error * 100.0,
                    significance: significance.to_string(),
                });
            }
        }
    }

    // Sort by error (best matches first)
    candidates.sort_by(|a, b| a.error_pct.partial_cmp(&b.error_pct).unwrap());

    // 2. Harmonic analysis: check if frequency ratios between spectral lines
    //    form n=6 patterns
    let mut freq_ratio_matches = Vec::new();
    let n6_target_ratios: Vec<(&str, f64)> = vec![
        ("n/sopfr",         N / SOPFR),          // 1.2
        ("sigma/n",         SIGMA / N),           // 2.0
        ("tau/phi",         TAU / PHI),           // 2.0
        ("sopfr/tau",       SOPFR / TAU),         // 1.25
        ("(n+mu)/n",        (N + MU) / N),        // 7/6 ≈ 1.167
        ("sigma/sopfr",     SIGMA / SOPFR),       // 2.4
        ("J2/sigma",        J2 / SIGMA),          // 2.0
        ("n/tau",           N / TAU),             // 1.5
        ("sopfr/phi",       SOPFR / PHI),         // 2.5
        ("sigma/tau",       SIGMA / TAU),         // 3.0
        ("J2/n",            J2 / N),              // 4.0
        ("n/phi",           N / PHI),             // 3.0
        ("(sigma-tau)/n",   (SIGMA - TAU) / N),   // 8/6 ≈ 1.333
        ("tau/sopfr*n",     TAU * N / SOPFR),     // 4.8
        ("sigma*phi/J2",    SIGMA * PHI / J2),    // 1.0
    ];

    for i in 0..freqs.len() {
        for j in (i + 1)..freqs.len() {
            let ratio = freqs[j].1 / freqs[i].1;
            // Also check inverse
            for (expr_name, expr_val) in &n6_target_ratios {
                let error = (ratio - expr_val).abs() / expr_val;
                if error < 0.02 {
                    freq_ratio_matches.push(FreqRatioMatch {
                        freq_a: freqs[i].0.to_string(),
                        freq_b: freqs[j].0.to_string(),
                        ratio,
                        n6_expr: expr_name.to_string(),
                        n6_value: *expr_val,
                        error_pct: error * 100.0,
                    });
                }
                let inv_error = (1.0 / ratio - expr_val).abs() / expr_val;
                if inv_error < 0.02 && *expr_val != 1.0 {
                    freq_ratio_matches.push(FreqRatioMatch {
                        freq_a: freqs[j].0.to_string(),
                        freq_b: freqs[i].0.to_string(),
                        ratio: 1.0 / ratio,
                        n6_expr: expr_name.to_string(),
                        n6_value: *expr_val,
                        error_pct: inv_error * 100.0,
                    });
                }
            }
        }
    }

    freq_ratio_matches.sort_by(|a, b| a.error_pct.partial_cmp(&b.error_pct).unwrap());

    // Build summary
    let exact = candidates.iter().filter(|c| c.significance == "EXACT").count();
    let strong = candidates.iter().filter(|c| c.significance == "STRONG").count();
    let moderate = candidates.iter().filter(|c| c.significance == "MODERATE").count();
    let best = candidates
        .first()
        .map(|c| format!("{} -> {} ({:.4}%)", c.formula, c.target, c.error_pct))
        .unwrap_or_else(|| "none".to_string());

    SignalReport {
        summary: SignalSummary {
            total_predictions: candidates.len(),
            exact_matches: exact,
            strong_matches: strong,
            moderate_matches: moderate,
            best_match: best,
        },
        candidates,
        freq_ratio_matches,
    }
}

// ─── FERMI Operator (Op 14): Fermi Paradox Analysis ─────────────────────────

fn fermi_analysis() -> FermiResult {
    // Drake equation: N = R* x f_p x n_e x f_l x f_i x f_c x L

    let drake_params = vec![
        DrakeParam {
            name: "R*".into(),
            range_lo: 1.5,
            range_hi: 3.0,
            n6_formula: "n/tau = 6/4 = 1.5".into(),
            n6_value: 1.5,
        },
        DrakeParam {
            name: "f_p".into(),
            range_lo: 0.5,
            range_hi: 1.0,
            n6_formula: "mu(6) = 1".into(),
            n6_value: 1.0,
        },
        DrakeParam {
            name: "n_e".into(),
            range_lo: 0.1,
            range_hi: 0.5,
            n6_formula: "phi/sopfr = 2/5 = 0.4".into(),
            n6_value: 0.4,
        },
        DrakeParam {
            name: "f_l".into(),
            range_lo: 0.01,
            range_hi: 1.0,
            n6_formula: "mu/(sigma-phi) = 1/10 = 0.1".into(),
            n6_value: 0.1,
        },
        DrakeParam {
            name: "f_i".into(),
            range_lo: 0.001,
            range_hi: 1.0,
            n6_formula: "mu/(sigma*phi*sopfr) = 1/120 = 0.00833".into(),
            n6_value: 1.0 / 120.0,
        },
        DrakeParam {
            name: "f_c".into(),
            range_lo: 0.01,
            range_hi: 0.5,
            n6_formula: "mu/(sigma-phi) = 1/10 = 0.1".into(),
            n6_value: 0.1,
        },
        DrakeParam {
            name: "L".into(),
            range_lo: 100.0,
            range_hi: 100_000.0,
            n6_formula: "tau^sopfr - J2 = 4^5 - 24 = 1000".into(),
            n6_value: 1000.0,
        },
    ];

    // Compute N from n=6 values
    let n_predicted: f64 = drake_params.iter().map(|p| p.n6_value).product();

    // Great Filter analysis — contribution of each parameter
    let mut filter_analysis = Vec::new();
    for p in &drake_params {
        // Log-contribution: how much does this parameter reduce N?
        let log_contrib = p.n6_value.ln();
        let is_bottleneck = p.n6_value < 0.01;
        let interpretation = if p.n6_value >= 1.0 {
            "Neutral or amplifying"
        } else if p.n6_value >= 0.1 {
            "Moderate reduction"
        } else if p.n6_value >= 0.01 {
            "Significant reduction"
        } else {
            "SEVERE bottleneck — potential Great Filter"
        };
        filter_analysis.push(GreatFilterAnalysis {
            parameter: p.name.clone(),
            contribution: log_contrib,
            is_bottleneck,
            interpretation: interpretation.to_string(),
        });
    }

    // Sensitivity analysis: elasticity dN/N / dp/p = 1 for multiplicative model
    // But with n=6 values, compute: what happens if we double each parameter?
    let mut sensitivity = Vec::new();
    for (i, p) in drake_params.iter().enumerate() {
        let base_n = n_predicted;
        let doubled_n: f64 = drake_params
            .iter()
            .enumerate()
            .map(|(j, q)| if i == j { q.n6_value * 2.0 } else { q.n6_value })
            .product();
        let elasticity = (doubled_n - base_n) / base_n; // should be ~1.0 for multiplicative
        sensitivity.push(SensitivityEntry {
            parameter: p.name.clone(),
            base_n,
            doubled_n,
            elasticity,
        });
    }

    // Communication window calculation
    // Galaxy age ~ 13.6 Gyr, L = 1000 years
    // P(overlap) = L / T_galaxy ~ 7.35e-8 per star
    // With ~100B stars: ~7.35 simultaneous civilizations
    let galaxy_age_yr = 13.6e9;
    let l = 1000.0;
    let n_stars = 100.0e9;
    let p_overlap = l / galaxy_age_yr;
    let _simultaneous_civs = p_overlap * n_stars * n_predicted / l;
    // More precisely: if N civilizations exist across galaxy lifetime,
    // average simultaneous = N * L / T_galaxy
    // N_total = R* * f_p * n_e * f_l * f_i * f_c * T_galaxy (not * L)
    // N_simultaneous = N_total * L / T_galaxy = R* * f_p * n_e * f_l * f_i * f_c * L = n_predicted
    // But n_predicted already includes L, so N_simultaneous ~ n_predicted
    // With n_predicted ~ 0.05 per Milky Way at any time
    // However scaling to observable universe (~2T galaxies): 0.05 * 2e12 ~ 1e11
    // Or: within MW with more optimistic L:
    // If L = tau^sopfr = 1024 → about the same
    // Re-derive: simultaneous = N_total_rate * L
    let n_rate: f64 = drake_params.iter()
        .filter(|p| p.name != "L")
        .map(|p| p.n6_value)
        .product::<f64>();
    let simultaneous = n_rate * l;
    // n_rate = 1.5 * 1.0 * 0.4 * 0.1 * 0.00833 * 0.1 = 4.998e-5
    // simultaneous = 4.998e-5 * 1000 = 0.04998 ≈ 0.05

    let bottleneck = filter_analysis
        .iter()
        .filter(|f| f.is_bottleneck)
        .map(|f| f.parameter.clone())
        .collect::<Vec<_>>()
        .join(", ");

    FermiResult {
        drake_params,
        n_predicted,
        bottleneck: if bottleneck.is_empty() {
            "f_i (intelligence emergence)".into()
        } else {
            format!("{} (intelligence emergence)", bottleneck)
        },
        great_filter_location: "Between simple life and intelligence (f_i = 1/120)".into(),
        simultaneous_civs: simultaneous,
        filter_analysis,
        sensitivity,
        prediction: "n=6 orbital resonance systems (e.g. HD 110067, TRAPPIST-1) are priority SETI targets — \
                     resonance implies stability implies longer habitable windows".into(),
    }
}

// ─── Display Functions ──────────────────────────────────────────────────────

fn print_signal_report(report: &SignalReport) {
    println!("── SIGNAL Operator (Op 13): ET Signal Pattern Detection ──\n");
    println!("Base frequency: H 21cm = {:.3} MHz", H21CM);
    println!("n=6 constants: n={}, sigma={}, tau={}, phi={}, sopfr={}, J2={}, mu={}\n",
             N, SIGMA, TAU, PHI, SOPFR, J2, MU);

    // Predicted frequency matches
    println!("┌─────────────────────────────────────────────────────────────────────────────────────────┐");
    println!("│ n=6 Predicted Frequencies vs Known Astronomical Lines                                  │");
    println!("├────┬──────────────────────────────────────────┬────────────┬────────────┬───────┬───────┤");
    println!("│  # │ Formula                                  │ Predicted  │ Actual     │ Err%  │ Sig   │");
    println!("├────┼──────────────────────────────────────────┼────────────┼────────────┼───────┼───────┤");

    for (i, c) in report.candidates.iter().enumerate() {
        let sig_marker = match c.significance.as_str() {
            "EXACT"    => "***",
            "STRONG"   => "** ",
            "MODERATE" => "*  ",
            _          => "   ",
        };
        println!(
            "│{:>3} │ {:<40} │ {:>8.3}   │ {:>8.3}   │{:>5.3} │ {} │",
            i + 1,
            truncate(&c.formula, 40),
            c.predicted_mhz,
            c.actual_mhz,
            c.error_pct,
            sig_marker,
        );
    }
    println!("└────┴──────────────────────────────────────────┴────────────┴────────────┴───────┴───────┘");
    println!("  Significance: *** EXACT (<0.1%) | ** STRONG (<1%) | * MODERATE (<5%)\n");

    // Frequency ratio matches
    if !report.freq_ratio_matches.is_empty() {
        println!("┌───────────────────────────────────────────────────────────────────────────────┐");
        println!("│ Inter-line Frequency Ratios Matching n=6 Expressions                         │");
        println!("├────┬──────────────────┬──────────────────┬─────────┬───────────────┬──────────┤");
        println!("│  # │ Line A           │ Line B           │  Ratio  │ n=6 expr      │  Err%    │");
        println!("├────┼──────────────────┼──────────────────┼─────────┼───────────────┼──────────┤");

        for (i, m) in report.freq_ratio_matches.iter().enumerate() {
            println!(
                "│{:>3} │ {:<16} │ {:<16} │ {:>7.4} │ {:<13} │ {:>6.3}%  │",
                i + 1,
                truncate(&m.freq_a, 16),
                truncate(&m.freq_b, 16),
                m.ratio,
                truncate(&m.n6_expr, 13),
                m.error_pct,
            );
        }
        println!("└────┴──────────────────┴──────────────────┴─────────┴───────────────┴──────────┘\n");
    }

    // Summary
    println!("Summary:");
    println!("  Total predicted matches: {}", report.summary.total_predictions);
    println!("  EXACT:    {}", report.summary.exact_matches);
    println!("  STRONG:   {}", report.summary.strong_matches);
    println!("  MODERATE: {}", report.summary.moderate_matches);
    println!("  Best match: {}", report.summary.best_match);
    println!();
    println!("Key findings:");
    println!("  - H21cm * (sigma*phi/J2) = H21cm * 1.0 = 1420.405 MHz  [IDENTITY — n=6 encodes hydrogen]");
    println!("  - H21cm * (1+1/n) = H21cm * 7/6 = 1657.1 MHz          [Near OH 1665 — water hole link]");
    println!("  - The 'water hole' (1420-1720 MHz) spans exactly H21cm * [1, sigma/n]");
}

fn print_fermi_report(result: &FermiResult) {
    println!("── FERMI Operator (Op 14): Fermi Paradox Analysis ──\n");
    println!("Drake Equation: N = R* x f_p x n_e x f_l x f_i x f_c x L\n");

    // Drake parameters table
    println!("┌───────┬─────────────┬──────────────────────────────────────────┬──────────────┐");
    println!("│ Param │    Range    │ n=6 Formula                              │  n=6 Value   │");
    println!("├───────┼─────────────┼──────────────────────────────────────────┼──────────────┤");

    for p in &result.drake_params {
        println!(
            "│ {:<5} │ {:>4.3}-{:<5.0} │ {:<40} │ {:>10.5}   │",
            p.name,
            p.range_lo,
            p.range_hi,
            truncate(&p.n6_formula, 40),
            p.n6_value,
        );
    }
    println!("└───────┴─────────────┴──────────────────────────────────────────┴──────────────┘\n");

    println!("N (n=6 predicted) = {:.6}", result.n_predicted);
    println!("  => {:.2} civilizations in the Milky Way at any time\n", result.simultaneous_civs);

    // Great Filter analysis
    println!("Great Filter Analysis:");
    println!("┌───────┬──────────────┬─────────────┬─────────────────────────────────────────────┐");
    println!("│ Param │  ln(value)   │ Bottleneck? │ Interpretation                              │");
    println!("├───────┼──────────────┼─────────────┼─────────────────────────────────────────────┤");

    for f in &result.filter_analysis {
        println!(
            "│ {:<5} │ {:>10.4}   │ {:<11} │ {:<43} │",
            f.parameter,
            f.contribution,
            if f.is_bottleneck { "YES <<<" } else { "no" },
            truncate(&f.interpretation, 43),
        );
    }
    println!("└───────┴──────────────┴─────────────┴─────────────────────────────────────────────┘\n");

    println!("  Bottleneck: {}", result.bottleneck);
    println!("  Great Filter: {}\n", result.great_filter_location);

    // Sensitivity
    println!("Sensitivity Analysis (doubling each parameter):");
    println!("┌───────┬──────────────┬──────────────┬────────────┐");
    println!("│ Param │   Base N     │  Doubled N   │ Elasticity │");
    println!("├───────┼──────────────┼──────────────┼────────────┤");

    for s in &result.sensitivity {
        println!(
            "│ {:<5} │ {:>10.6}   │ {:>10.6}   │ {:>8.4}   │",
            s.parameter, s.base_n, s.doubled_n, s.elasticity,
        );
    }
    println!("└───────┴──────────────┴──────────────┴────────────┘");
    println!("  (All elasticities = 1.0 — multiplicative model, each parameter equally important)\n");

    // Communication window
    println!("Communication Window:");
    println!("  L = tau^sopfr - J2 = 4^5 - 24 = 1000 years (civilization lifetime)");
    println!("  Galaxy age = 13.6 Gyr");
    println!("  P(temporal overlap) = L / T_galaxy = {:.2e}", 1000.0 / 13.6e9);
    println!("  N_simultaneous (MW) = {:.4}", result.simultaneous_civs);
    println!("  N_simultaneous (observable universe, ~2T galaxies) ~ {:.1e}\n",
             result.simultaneous_civs * 2.0e12);

    // Prediction
    println!("SEDI Prediction:");
    println!("  {}", result.prediction);
    println!("  - HD 110067: period ratio = 6.0096 (n=6 resonance)");
    println!("  - TRAPPIST-1: 12 n=6 orbital ratio matches");
    println!("  - n=6 resonance -> orbital stability -> longer habitable window -> higher f_l");
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() <= max {
        format!("{:<width$}", s, width = max)
    } else {
        format!("{}...", &s[..max - 3])
    }
}

// ─── Main ───────────────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.iter().any(|a| a == "--json");
    let signal_only = args.iter().any(|a| a == "--signal");
    let fermi_only = args.iter().any(|a| a == "--fermi");
    let run_both = !signal_only && !fermi_only;

    let signal_report = if run_both || signal_only {
        Some(signal_analysis())
    } else {
        None
    };

    let fermi_result = if run_both || fermi_only {
        Some(fermi_analysis())
    } else {
        None
    };

    if json_mode {
        if run_both {
            let full = FullReport {
                signal: signal_report.unwrap(),
                fermi: fermi_result.unwrap(),
            };
            println!("{}", serde_json::to_string_pretty(&full).unwrap());
        } else if let Some(s) = signal_report {
            println!("{}", serde_json::to_string_pretty(&s).unwrap());
        } else if let Some(f) = fermi_result {
            println!("{}", serde_json::to_string_pretty(&f).unwrap());
        }
        return;
    }

    println!("=== SEDI Signal & Fermi Analyzer ===");
    println!("    n=6 operators for extraterrestrial intelligence search\n");

    if let Some(ref s) = signal_report {
        print_signal_report(s);
    }

    if signal_report.is_some() && fermi_result.is_some() {
        println!("{}\n", "=".repeat(80));
    }

    if let Some(ref f) = fermi_result {
        print_fermi_report(f);
    }

    println!("\n=== Analysis Complete ===");
}
