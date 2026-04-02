// Deep Miner — Depth-4 targeted formula search for n=6 hard constants
// Focuses on 6 hardest targets: h, Lambda, G, top quark, tau lepton, W boson
// Strategy: enumerate all depth-2 expressions, then compose pairs via 5 ops
// For extreme-scale targets: also try base * 10^(n6_expr)
// Build: ~/.cargo/bin/rustc main.rs -o deep-miner



// ── N=6 Base Constants ──────────────────────────────────────────────
const N: f64 = 6.0;
const PHI: f64 = 2.0;      // phi(6) = 2
const TAU: f64 = 4.0;      // tau(6) = 4
const SIGMA: f64 = 12.0;   // sigma(6) = 12
const J2: f64 = 24.0;      // Jordan J_2(6) = 24
const SOPFR: f64 = 5.0;    // sopfr(6) = 2+3 = 5
const MU: f64 = 1.0;       // mu(6) = 1

const NC: usize = 7;
const CONST_NAMES: [&str; NC] = ["n", "phi", "tau", "sigma", "J2", "sopfr", "mu"];
const CONST_VALS: [f64; NC] = [N, PHI, TAU, SIGMA, J2, SOPFR, MU];

// ── Operations ──────────────────────────────────────────────────────
#[derive(Clone, Copy)]
enum Op { Add, Sub, Mul, Div, Pow }

const ALL_OPS: [Op; 5] = [Op::Add, Op::Sub, Op::Mul, Op::Div, Op::Pow];

impl Op {
    #[inline]
    fn eval(self, a: f64, b: f64) -> Option<f64> {
        let r = match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => { if b.abs() < 1e-15 { return None; } a / b },
            Op::Pow => {
                if a.abs() < 1e-15 && b < 0.0 { return None; }
                if a.abs() > 1e8 || b.abs() > 80.0 { return None; }
                a.powf(b)
            }
        };
        if r.is_finite() { Some(r) } else { None }
    }
    fn sym(self) -> &'static str {
        match self { Op::Add=>"+", Op::Sub=>"-", Op::Mul=>"*", Op::Div=>"/", Op::Pow=>"^" }
    }
}

// ── Depth-2 expression: (const op const) or just const ─────────────
// We store them as (value, formula_string) pairs
struct D2 {
    val: f64,
    formula: String,
}

fn gen_depth2() -> Vec<D2> {
    let mut out = Vec::with_capacity(300);
    // Depth 0: bare constants
    for i in 0..NC {
        out.push(D2 { val: CONST_VALS[i], formula: CONST_NAMES[i].to_string() });
    }
    // Depth 1: const op const (all pairs, all ops)
    for i in 0..NC {
        for j in 0..NC {
            for &op in &ALL_OPS {
                if let Some(v) = op.eval(CONST_VALS[i], CONST_VALS[j]) {
                    if v.is_finite() && v.abs() < 1e18 && v.abs() > 1e-18 {
                        let f = format!("({} {} {})", CONST_NAMES[i], op.sym(), CONST_NAMES[j]);
                        out.push(D2 { val: v, formula: f });
                    }
                }
            }
        }
    }
    // Depth 2: (const op const) op const  AND  const op (const op const)
    // This gives us richer building blocks
    let base_len = out.len();
    for idx in 0..base_len {
        for k in 0..NC {
            for &op in &ALL_OPS {
                // d2_expr op const
                if let Some(v) = op.eval(out[idx].val, CONST_VALS[k]) {
                    if v.is_finite() && v.abs() < 1e30 && v.abs() > 1e-30 {
                        let f = format!("({} {} {})", out[idx].formula, op.sym(), CONST_NAMES[k]);
                        out.push(D2 { val: v, formula: f });
                    }
                }
                // const op d2_expr
                if let Some(v) = op.eval(CONST_VALS[k], out[idx].val) {
                    if v.is_finite() && v.abs() < 1e30 && v.abs() > 1e-30 {
                        let f = format!("({} {} {})", CONST_NAMES[k], op.sym(), out[idx].formula);
                        out.push(D2 { val: v, formula: f });
                    }
                }
            }
        }
    }
    // Deduplicate by value (keep first/shortest formula)
    out.sort_by(|a, b| a.val.partial_cmp(&b.val).unwrap_or(std::cmp::Ordering::Equal));
    let mut deduped: Vec<D2> = Vec::with_capacity(out.len());
    for d in out {
        let dominated = deduped.last().map_or(false, |prev: &D2| {
            (prev.val - d.val).abs() / (prev.val.abs().max(1e-30)) < 1e-12
        });
        if !dominated {
            deduped.push(d);
        }
    }
    deduped
}

// ── Targets ─────────────────────────────────────────────────────────
struct Target {
    name: &'static str,
    value: f64,
    tolerance: f64,   // relative error threshold
    extreme: bool,    // needs 10^x scaling
}

fn targets() -> Vec<Target> {
    vec![
        // Extreme-scale fundamental constants
        Target { name: "h (Planck) = 6.626e-34", value: 6.62607015e-34, tolerance: 0.01, extreme: true },
        Target { name: "Lambda (cosmo) ~ 1.1056e-52", value: 1.1056e-52, tolerance: 0.05, extreme: true },
        Target { name: "G (gravitational) = 6.674e-11", value: 6.674e-11, tolerance: 0.01, extreme: true },
        // Near-miss particle masses
        Target { name: "m_top = 172.69 GeV", value: 172.69, tolerance: 0.0001, extreme: false },
        Target { name: "m_tau = 1776.86 MeV", value: 1776.86, tolerance: 0.0001, extreme: false },
        Target { name: "m_W = 80.377 GeV", value: 80.377, tolerance: 0.0001, extreme: false },
    ]
}

// ── Result tracking ─────────────────────────────────────────────────
struct Best {
    error: f64,
    formula: String,
    value: f64,
}

fn main() {
    println!("=== Deep Miner: Depth-4 Targeted Formula Search ===");
    println!("Building depth-2 expression table...");

    let d2 = gen_depth2();
    println!("  Depth-2 expressions: {} unique values", d2.len());

    let tgts = targets();
    let mut bests: Vec<Best> = tgts.iter().map(|t| Best {
        error: f64::MAX,
        formula: String::new(),
        value: 0.0,
    }).collect();

    // ── Phase 1: Direct depth-4 search (d2 op d2) for non-extreme targets ──
    println!("\nPhase 1: Direct depth-4 (d2 op d2) — {} x {} x 5 = {} candidates",
        d2.len(), d2.len(), d2.len() * d2.len() * 5);

    let total = d2.len();
    let mut checked: u64 = 0;

    for i in 0..total {
        if i % 500 == 0 && i > 0 {
            eprint!("\r  Progress: {:.1}%  ({} / {})", 100.0 * i as f64 / total as f64, i, total);
        }
        for j in 0..total {
            for &op in &ALL_OPS {
                if let Some(v) = op.eval(d2[i].val, d2[j].val) {
                    checked += 1;
                    if !v.is_finite() || v == 0.0 { continue; }

                    for (ti, t) in tgts.iter().enumerate() {
                        if t.extreme { continue; } // handled in phase 2
                        let rel = ((v - t.value) / t.value).abs();
                        if rel < bests[ti].error {
                            bests[ti].error = rel;
                            bests[ti].value = v;
                            bests[ti].formula = format!("{} {} {}",
                                d2[i].formula, op.sym(), d2[j].formula);
                        }
                    }
                }
            }
        }
    }
    eprintln!("\r  Phase 1 done: {} combinations checked                    ", checked);

    // ── Phase 2: Extreme-scale search: base * 10^(exponent_expr) ──────
    // For h, G, Lambda: find base_expr * 10^(exp_expr) close to target
    // Equivalently: log10(target) = log10(base_expr) + exp_expr
    println!("\nPhase 2: Extreme-scale search (base * 10^exp)");

    // Precompute log10 values of all d2 expressions that are positive
    let d2_positive: Vec<(f64, f64, &str)> = d2.iter()
        .filter(|d| d.val > 0.0)
        .map(|d| (d.val, d.val.log10(), d.formula.as_str()))
        .collect();

    println!("  Positive d2 values: {}", d2_positive.len());

    for (ti, t) in tgts.iter().enumerate() {
        if !t.extreme { continue; }

        let log_target = t.value.log10();
        println!("  Target: {} (log10 = {:.4})", t.name, log_target);

        // Strategy A: find d2_base and d2_exp such that
        //   log10(d2_base) + d2_exp ~= log10(target)
        //   i.e., d2_base * 10^d2_exp ~= target
        for (_, log_base, base_formula) in &d2_positive {
            // We need exp_val such that log_base + exp_val = log_target
            let needed_exp = log_target - log_base;

            // Search d2 for values close to needed_exp
            // Binary search since d2 is sorted by val
            let idx = d2.partition_point(|d| d.val < needed_exp - 0.5);
            let end = d2.len().min(idx + 100);
            for k in idx..end {
                let exp_val = d2[k].val;
                let diff = (exp_val - needed_exp).abs();
                if diff > 1.0 { continue; }

                // Compute actual value: base * 10^exp
                let actual_log = log_base + exp_val;
                let log_rel_error = (actual_log - log_target).abs();
                // Convert log error to relative error
                let rel = (10.0_f64.powf(log_rel_error) - 1.0).abs();

                if rel < bests[ti].error {
                    bests[ti].error = rel;
                    bests[ti].formula = format!("{} * 10^({})", base_formula, d2[k].formula);
                    // Compute actual value carefully
                    if actual_log.abs() < 300.0 {
                        bests[ti].value = 10.0_f64.powf(actual_log);
                    } else {
                        bests[ti].value = 0.0; // too extreme to represent
                    }
                }
            }
        }

        // Strategy B: find d2_a and d2_b such that d2_a * 10^(-d2_b) ~= target
        // (explicit negative exponent search)
        for (base_val, log_base, base_formula) in &d2_positive {
            for (_, _, _) in &[] as &[(f64, f64, &str)] {} // placeholder
            // For each d2 value as negative exponent
            for d in &d2 {
                if d.val < 1.0 || d.val > 200.0 { continue; }
                let actual_log = log_base - d.val;
                let log_rel_error = (actual_log - log_target).abs();
                if log_rel_error > 1.0 { continue; }
                let rel = (10.0_f64.powf(log_rel_error) - 1.0).abs();
                if rel < bests[ti].error {
                    bests[ti].error = rel;
                    bests[ti].formula = format!("{} * 10^(-({}))  [= 10^{:.4}]",
                        base_formula, d.formula, actual_log);
                    if actual_log.abs() < 300.0 {
                        bests[ti].value = 10.0_f64.powf(actual_log);
                    }
                }
            }
        }

        // Strategy C: exact exponent decomposition
        // Target = mantissa * 10^exponent
        // Find n6 formula for mantissa and n6 formula for exponent separately
        let exponent = log_target.floor();
        let mantissa = t.value / 10.0_f64.powf(exponent);
        println!("    Decomposed: {:.6} * 10^{}", mantissa, exponent);

        // Search d2 for mantissa match
        let mut best_mantissa_err = f64::MAX;
        let mut best_mantissa_formula = String::new();
        let mut best_mantissa_val = 0.0_f64;
        for d in &d2 {
            if d.val <= 0.0 { continue; }
            let rel = ((d.val - mantissa) / mantissa).abs();
            if rel < best_mantissa_err {
                best_mantissa_err = rel;
                best_mantissa_formula = d.formula.clone();
                best_mantissa_val = d.val;
            }
        }

        // Search d2 for exponent match
        let mut best_exp_err = f64::MAX;
        let mut best_exp_formula = String::new();
        let mut best_exp_val = 0.0_f64;
        for d in &d2 {
            let rel_exp = (d.val - exponent).abs();
            if rel_exp < best_exp_err {
                best_exp_err = rel_exp;
                best_exp_formula = d.formula.clone();
                best_exp_val = d.val;
            }
        }

        // Combined error
        let combined_log = best_mantissa_val.log10() + best_exp_val;
        let combined_rel = (10.0_f64.powf((combined_log - log_target).abs()) - 1.0).abs();
        if combined_rel < bests[ti].error {
            bests[ti].error = combined_rel;
            bests[ti].formula = format!("{} * 10^({})", best_mantissa_formula, best_exp_formula);
            if combined_log.abs() < 300.0 {
                bests[ti].value = 10.0_f64.powf(combined_log);
            }
        }

        println!("    Best mantissa: {} = {:.6} (err {:.4}%)",
            best_mantissa_formula, best_mantissa_val, best_mantissa_err * 100.0);
        println!("    Best exponent: {} = {:.2} (need {})",
            best_exp_formula, best_exp_val, exponent);
    }

    // ── Phase 3: For near-miss targets, also try depth-4 with scaling ──
    // d2 * d2 / d2 and d2^2 +/- d2 patterns
    println!("\nPhase 3: Enhanced patterns for near-miss targets");
    println!("  Pattern: (d2 * d2) +/- d2, d2^2 +/- d2, etc.");

    // Pre-select small pool of "interesting" d2 values near each target
    for (ti, t) in tgts.iter().enumerate() {
        if t.extreme { continue; }

        // Collect d2 values whose squares, products might land near target
        let sqrt_target = t.value.sqrt();
        let mut candidates_near_sqrt: Vec<usize> = Vec::new();
        for (idx, d) in d2.iter().enumerate() {
            if d.val > 0.0 && (d.val - sqrt_target).abs() / sqrt_target < 0.5 {
                candidates_near_sqrt.push(idx);
            }
        }

        // Try a*b +/- c patterns where a*b is close
        for &ai in &candidates_near_sqrt {
            for &bi in &candidates_near_sqrt {
                if let Some(prod) = Op::Mul.eval(d2[ai].val, d2[bi].val) {
                    let residual = t.value - prod;
                    // Find d2 closest to residual
                    let res_idx = d2.partition_point(|d| d.val < residual - 0.5);
                    let end = d2.len().min(res_idx + 20);
                    let start = if res_idx > 10 { res_idx - 10 } else { 0 };
                    for k in start..end {
                        let v = prod + d2[k].val;
                        let rel = ((v - t.value) / t.value).abs();
                        if rel < bests[ti].error {
                            bests[ti].error = rel;
                            bests[ti].value = v;
                            bests[ti].formula = format!("({} * {}) + {}",
                                d2[ai].formula, d2[bi].formula, d2[k].formula);
                        }
                        let v2 = prod - d2[k].val;
                        let rel2 = ((v2 - t.value) / t.value).abs();
                        if rel2 < bests[ti].error {
                            bests[ti].error = rel2;
                            bests[ti].value = v2;
                            bests[ti].formula = format!("({} * {}) - {}",
                                d2[ai].formula, d2[bi].formula, d2[k].formula);
                        }
                    }
                }
            }
        }
    }

    // ── Phase 4: Brute-force narrower depth-4 for near-miss ────────────
    // For each near-miss target, take only d2 values in a reasonable range
    // and do exhaustive search with tighter tolerance
    println!("\nPhase 4: Focused brute-force for near-miss targets");

    for (ti, t) in tgts.iter().enumerate() {
        if t.extreme { continue; }

        // Collect d2 indices where value could contribute to target
        let mut relevant: Vec<usize> = Vec::new();
        for (idx, d) in d2.iter().enumerate() {
            let v = d.val.abs();
            // Keep values that could combine to reach target
            if v < t.value * 100.0 && v > t.value * 0.001 {
                relevant.push(idx);
            }
            // Also keep small values for fine adjustment
            if v > 0.0 && v < 10.0 {
                relevant.push(idx);
            }
            // Keep values near target/small_int or target*small_int
            for div in &[2.0, 3.0, 4.0, 5.0, 6.0, 12.0, 24.0] {
                if ((d.val - t.value / div) / (t.value / div)).abs() < 0.1 {
                    relevant.push(idx);
                }
                if ((d.val - t.value * div) / (t.value * div)).abs() < 0.1 {
                    relevant.push(idx);
                }
            }
        }
        relevant.sort();
        relevant.dedup();

        println!("  {} — {} relevant d2 values", t.name, relevant.len());

        let mut phase4_count: u64 = 0;
        for &i in &relevant {
            for &j in &relevant {
                for &op in &ALL_OPS {
                    if let Some(v) = op.eval(d2[i].val, d2[j].val) {
                        phase4_count += 1;
                        if !v.is_finite() || v == 0.0 { continue; }
                        let rel = ((v - t.value) / t.value).abs();
                        if rel < bests[ti].error {
                            bests[ti].error = rel;
                            bests[ti].value = v;
                            bests[ti].formula = format!("{} {} {}",
                                d2[i].formula, op.sym(), d2[j].formula);
                        }
                    }
                }
            }
        }
        println!("    Checked {} combinations", phase4_count);
    }

    // ── Phase 5: Integer exponent ladders for extreme targets ──────────
    // Try expressions like: n6_int ^ n6_int * n6_small  (exact integer powers)
    println!("\nPhase 5: Integer power ladders for extreme-scale targets");

    for (ti, t) in tgts.iter().enumerate() {
        if !t.extreme { continue; }

        let log_target = t.value.log10();

        // Try: CONST_VAL[a]^CONST_VAL[b] * CONST_VAL[c]^CONST_VAL[d]
        // with negative exponents allowed
        for a in 0..NC {
            for b in 0..NC {
                let base1 = CONST_VALS[a];
                let exp1 = CONST_VALS[b];
                if base1.abs() < 1e-15 { continue; }
                for sign1 in &[1.0_f64, -1.0] {
                    let log1 = if base1 > 0.0 { sign1 * exp1 * base1.log10() } else { continue };

                    for c in 0..NC {
                        for d in 0..NC {
                            let base2 = CONST_VALS[c];
                            let exp2 = CONST_VALS[d];
                            if base2.abs() < 1e-15 { continue; }
                            for sign2 in &[1.0_f64, -1.0] {
                                let log2 = if base2 > 0.0 { sign2 * exp2 * base2.log10() } else { continue };

                                // total = base1^(s1*exp1) * base2^(s2*exp2)
                                let total_log = log1 + log2;
                                let log_err = (total_log - log_target).abs();
                                if log_err > 2.0 { continue; }
                                let rel = (10.0_f64.powf(log_err) - 1.0).abs();

                                if rel < bests[ti].error {
                                    bests[ti].error = rel;
                                    let s1 = if *sign1 < 0.0 { "-" } else { "" };
                                    let s2 = if *sign2 < 0.0 { "-" } else { "" };
                                    bests[ti].formula = format!(
                                        "{}^({}{}) * {}^({}{})",
                                        CONST_NAMES[a], s1, CONST_NAMES[b],
                                        CONST_NAMES[c], s2, CONST_NAMES[d]
                                    );
                                    if total_log.abs() < 300.0 {
                                        bests[ti].value = 10.0_f64.powf(total_log);
                                    }
                                }

                                // Also try with a d2 multiplier
                                for d2e in &d2 {
                                    if d2e.val <= 0.0 { continue; }
                                    let adj_log = total_log + d2e.val.log10();
                                    let adj_err = (adj_log - log_target).abs();
                                    if adj_err > 0.5 { continue; }
                                    let adj_rel = (10.0_f64.powf(adj_err) - 1.0).abs();
                                    if adj_rel < bests[ti].error {
                                        bests[ti].error = adj_rel;
                                        let s1 = if *sign1 < 0.0 { "-" } else { "" };
                                        let s2 = if *sign2 < 0.0 { "-" } else { "" };
                                        bests[ti].formula = format!(
                                            "{}^({}{}) * {}^({}{}) * {}",
                                            CONST_NAMES[a], s1, CONST_NAMES[b],
                                            CONST_NAMES[c], s2, CONST_NAMES[d],
                                            d2e.formula
                                        );
                                        if adj_log.abs() < 300.0 {
                                            bests[ti].value = 10.0_f64.powf(adj_log);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // ── Phase 6: Full log-space d2×d2 for extreme targets ──────────────
    // For each pair of d2 values, compute log10(|d2_i|) + log10(|d2_j|)
    // and also log10(|d2_i|) - log10(|d2_j|), matching against log10(target)
    println!("\nPhase 6: Full log-space d2 x d2 for extreme targets");

    // Precompute log10 of all d2 abs values
    struct LogD2 { log_abs: f64, sign: f64, idx: usize }
    let log_d2: Vec<LogD2> = d2.iter().enumerate()
        .filter(|(_, d)| d.val != 0.0 && d.val.is_finite())
        .map(|(idx, d)| LogD2 {
            log_abs: d.val.abs().log10(),
            sign: if d.val > 0.0 { 1.0 } else { -1.0 },
            idx,
        })
        .collect();

    println!("  Log-d2 entries: {}", log_d2.len());

    for (ti, t) in tgts.iter().enumerate() {
        if !t.extreme { continue; }
        // Target must be positive for this to work
        if t.value <= 0.0 { continue; }

        let log_target = t.value.log10();
        let mut phase6_count: u64 = 0;

        // d2_i * d2_j = target  =>  log(d2_i) + log(d2_j) = log(target)
        // d2_i / d2_j = target  =>  log(d2_i) - log(d2_j) = log(target)
        // d2_i ^ d2_j_val = target => d2_j_val * log(d2_i) = log(target)

        // Strategy: for each log_d2[i], compute needed = log_target - log_d2[i]
        // then binary search log_d2 for that needed value (for multiply)
        // Sort log_d2 by log_abs
        let mut sorted_logs: Vec<(f64, usize)> = log_d2.iter()
            .map(|l| (l.log_abs, l.idx))
            .collect();
        sorted_logs.sort_by(|a, b| a.0.partial_cmp(&b.0).unwrap());

        for li in &log_d2 {
            // For multiplication: need log_j such that log_i + log_j = log_target
            // (both positive values multiplied)
            let needed = log_target - li.log_abs;
            // Binary search
            let search_idx = sorted_logs.partition_point(|&(l, _)| l < needed - 0.001);
            let end = sorted_logs.len().min(search_idx + 5);
            let start = if search_idx > 5 { search_idx - 5 } else { 0 };
            for k in start..end {
                let (log_j, j_idx) = sorted_logs[k];
                let total_log = li.log_abs + log_j;
                let err = (total_log - log_target).abs();
                if err > 0.3 { continue; }
                phase6_count += 1;

                let rel = (10.0_f64.powf(err) - 1.0).abs();
                if rel < bests[ti].error {
                    bests[ti].error = rel;
                    bests[ti].formula = format!("{} * {}",
                        d2[li.idx].formula, d2[j_idx].formula);
                    if total_log.abs() < 300.0 {
                        bests[ti].value = 10.0_f64.powf(total_log);
                    }
                }

                // Also try with sign flip (negative * negative = positive)
            }

            // For division: need log_j such that log_i - log_j = log_target
            let needed_div = li.log_abs - log_target;
            let search_idx2 = sorted_logs.partition_point(|&(l, _)| l < needed_div - 0.001);
            let end2 = sorted_logs.len().min(search_idx2 + 5);
            let start2 = if search_idx2 > 5 { search_idx2 - 5 } else { 0 };
            for k in start2..end2 {
                let (log_j, j_idx) = sorted_logs[k];
                let total_log = li.log_abs - log_j;
                let err = (total_log - log_target).abs();
                if err > 0.3 { continue; }
                phase6_count += 1;

                let rel = (10.0_f64.powf(err) - 1.0).abs();
                if rel < bests[ti].error {
                    bests[ti].error = rel;
                    bests[ti].formula = format!("{} / {}",
                        d2[li.idx].formula, d2[j_idx].formula);
                    if total_log.abs() < 300.0 {
                        bests[ti].value = 10.0_f64.powf(total_log);
                    }
                }
            }

            // For power: d2_i ^ d2_j_val = target => d2_j_val * log10(d2_i) = log10(target)
            if li.log_abs.abs() > 0.01 {
                let needed_exp = log_target / li.log_abs;
                // Search d2 values near needed_exp
                let exp_idx = d2.partition_point(|d| d.val < needed_exp - 0.5);
                let exp_end = d2.len().min(exp_idx + 20);
                let exp_start = if exp_idx > 10 { exp_idx - 10 } else { 0 };
                for k in exp_start..exp_end {
                    let exp_log = d2[k].val * li.log_abs;
                    let err = (exp_log - log_target).abs();
                    if err > 0.3 { continue; }
                    phase6_count += 1;

                    let rel = (10.0_f64.powf(err) - 1.0).abs();
                    if rel < bests[ti].error {
                        bests[ti].error = rel;
                        bests[ti].formula = format!("{} ^ ({})",
                            d2[li.idx].formula, d2[k].formula);
                        if exp_log.abs() < 300.0 {
                            bests[ti].value = 10.0_f64.powf(exp_log);
                        }
                    }
                }
            }
        }

        println!("  {} — checked {} log-space combinations, best = {:.4}%",
            t.name, phase6_count, bests[ti].error * 100.0);
    }

    // ── Final Report ────────────────────────────────────────────────────
    println!("\n======================================================================");
    println!("============================================================");
    println!("  DEEP MINER RESULTS — Depth-4 Targeted Search");
    println!("============================================================\n");

    for (ti, t) in tgts.iter().enumerate() {
        let b = &bests[ti];
        let ppm = b.error * 1_000_000.0;
        let pct = b.error * 100.0;

        let grade = if pct < 0.01 { "EXACT" }
            else if pct < 0.1 { "CLOSE" }
            else if pct < 1.0 { "NEAR" }
            else if pct < 5.0 { "WEAK" }
            else { "MISS" };

        println!("--- {} ---", t.name);
        println!("  Target:  {:.6e}", t.value);
        if b.value != 0.0 {
            println!("  Found:   {:.6e}", b.value);
        }
        println!("  Error:   {:.4}% ({:.1} ppm)  [{}]", pct, ppm, grade);
        println!("  Formula: {}", b.formula);
        println!();
    }

    // Summary table
    println!("============================================================");
    println!("  SUMMARY");
    println!("============================================================");
    println!("{:<35} {:>10} {:>10} {:>6}", "Target", "Error%", "PPM", "Grade");
    println!("{:-<35} {:-<10} {:-<10} {:-<6}", "", "", "", "");
    for (ti, t) in tgts.iter().enumerate() {
        let b = &bests[ti];
        let ppm = b.error * 1_000_000.0;
        let pct = b.error * 100.0;
        let grade = if pct < 0.01 { "EXACT" }
            else if pct < 0.1 { "CLOSE" }
            else if pct < 1.0 { "NEAR" }
            else if pct < 5.0 { "WEAK" }
            else { "MISS" };
        println!("{:<35} {:>9.4}% {:>9.1} {:>6}", t.name, pct, ppm, grade);
    }
    println!();
}
