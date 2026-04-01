// EVOLVE — Genetic Formula Miner for n=6 Architecture
// Discovers new n=6 formulas matching physical/mathematical constants
// Build: ~/.cargo/bin/rustc main.rs -o formula-miner

use std::fmt;

// ── N=6 Base Constants ──────────────────────────────────────────────
const N: f64 = 6.0;
const PHI: f64 = 2.0;    // phi(6) = 2
const TAU: f64 = 4.0;    // tau(6) = 4  (number of divisors)
const SIGMA: f64 = 12.0; // sigma(6) = 12
const J2: f64 = 24.0;    // Jordan J_2(6) = 24
const SOPFR: f64 = 5.0;  // sopfr(6) = 2+3 = 5
const MU: f64 = 1.0;     // mu(6) = 1 (Mobius)

const CONST_NAMES: [&str; 7] = ["n", "phi", "tau", "sigma", "J2", "sopfr", "mu"];
const CONST_VALS: [f64; 7] = [N, PHI, TAU, SIGMA, J2, SOPFR, MU];

// ── Operations ──────────────────────────────────────────────────────
#[derive(Clone, Copy, PartialEq)]
enum Op { Add, Sub, Mul, Div, Pow }

impl Op {
    fn eval(self, a: f64, b: f64) -> Option<f64> {
        let r = match self {
            Op::Add => a + b,
            Op::Sub => a - b,
            Op::Mul => a * b,
            Op::Div => { if b.abs() < 1e-15 { return None; } a / b },
            Op::Pow => {
                if a.abs() < 1e-15 && b < 0.0 { return None; }
                if a.abs() > 1e6 || b.abs() > 50.0 { return None; }
                a.powf(b)
            }
        };
        if r.is_finite() && r.abs() < 1e30 { Some(r) } else { None }
    }
    fn sym(self) -> &'static str {
        match self { Op::Add=>"+", Op::Sub=>"-", Op::Mul=>"*", Op::Div=>"/", Op::Pow=>"^" }
    }
    fn from_idx(i: usize) -> Op {
        match i % 5 { 0=>Op::Add, 1=>Op::Sub, 2=>Op::Mul, 3=>Op::Div, _=>Op::Pow }
    }
}

// ── Expression Tree ─────────────────────────────────────────────────
#[derive(Clone)]
enum Expr {
    Leaf(usize),                       // index into CONST_VALS
    BinOp(Op, Box<Expr>, Box<Expr>),
}

impl Expr {
    fn eval(&self) -> Option<f64> {
        match self {
            Expr::Leaf(i) => Some(CONST_VALS[*i]),
            Expr::BinOp(op, l, r) => {
                let lv = l.eval()?;
                let rv = r.eval()?;
                op.eval(lv, rv)
            }
        }
    }

    fn depth(&self) -> usize {
        match self {
            Expr::Leaf(_) => 0,
            Expr::BinOp(_, l, r) => 1 + l.depth().max(r.depth()),
        }
    }

    fn node_count(&self) -> usize {
        match self {
            Expr::Leaf(_) => 1,
            Expr::BinOp(_, l, r) => 1 + l.node_count() + r.node_count(),
        }
    }
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Expr::Leaf(i) => write!(f, "{}", CONST_NAMES[*i]),
            Expr::BinOp(op, l, r) => {
                let need_parens_l = matches!(l.as_ref(), Expr::BinOp(..));
                let need_parens_r = matches!(r.as_ref(), Expr::BinOp(..));
                if need_parens_l { write!(f, "(")?; }
                write!(f, "{}", l)?;
                if need_parens_l { write!(f, ")")?; }
                write!(f, " {} ", op.sym())?;
                if need_parens_r { write!(f, "(")?; }
                write!(f, "{}", r)?;
                if need_parens_r { write!(f, ")")?; }
                Ok(())
            }
        }
    }
}

// ── Simple RNG (xorshift64) ────────────────────────────────────────
struct Rng(u64);

impl Rng {
    fn new(seed: u64) -> Self { Rng(seed ^ 0xdeadbeef12345678) }
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        self.0
    }
    fn usize(&mut self, max: usize) -> usize { (self.next() % max as u64) as usize }
    fn f64(&mut self) -> f64 { (self.next() & 0xFFFFFFFF) as f64 / 4294967296.0 }
}

// ── Random Tree Generation ──────────────────────────────────────────
fn random_tree(rng: &mut Rng, max_depth: usize) -> Expr {
    if max_depth == 0 || rng.f64() < 0.35 {
        Expr::Leaf(rng.usize(7))
    } else {
        let op = Op::from_idx(rng.usize(5));
        let l = random_tree(rng, max_depth - 1);
        let r = random_tree(rng, max_depth - 1);
        Expr::BinOp(op, Box::new(l), Box::new(r))
    }
}

// ── Mutation ────────────────────────────────────────────────────────
fn mutate(expr: &Expr, rng: &mut Rng) -> Expr {
    let choice = rng.f64();
    match expr {
        Expr::Leaf(_) => {
            if choice < 0.7 {
                // Replace with different constant
                Expr::Leaf(rng.usize(7))
            } else {
                // Grow into a small subtree
                let op = Op::from_idx(rng.usize(5));
                let other = Expr::Leaf(rng.usize(7));
                if rng.f64() < 0.5 {
                    Expr::BinOp(op, Box::new(expr.clone()), Box::new(other))
                } else {
                    Expr::BinOp(op, Box::new(other), Box::new(expr.clone()))
                }
            }
        }
        Expr::BinOp(op, l, r) => {
            if choice < 0.15 {
                // Replace entire subtree
                random_tree(rng, 2)
            } else if choice < 0.30 {
                // Change operation
                let new_op = Op::from_idx(rng.usize(5));
                Expr::BinOp(new_op, l.clone(), r.clone())
            } else if choice < 0.45 {
                // Shrink to one child
                if rng.f64() < 0.5 { *l.clone() } else { *r.clone() }
            } else if choice < 0.70 {
                // Mutate left child
                Expr::BinOp(*op, Box::new(mutate(l, rng)), r.clone())
            } else {
                // Mutate right child
                Expr::BinOp(*op, l.clone(), Box::new(mutate(r, rng)))
            }
        }
    }
}

// ── Crossover ───────────────────────────────────────────────────────
fn random_subtree<'a>(expr: &'a Expr, rng: &mut Rng) -> &'a Expr {
    match expr {
        Expr::Leaf(_) => expr,
        Expr::BinOp(_, l, r) => {
            let choice = rng.f64();
            if choice < 0.33 { expr }
            else if choice < 0.66 { random_subtree(l, rng) }
            else { random_subtree(r, rng) }
        }
    }
}

fn replace_random_subtree(expr: &Expr, replacement: &Expr, rng: &mut Rng) -> Expr {
    match expr {
        Expr::Leaf(_) => {
            if rng.f64() < 0.5 { replacement.clone() } else { expr.clone() }
        }
        Expr::BinOp(op, l, r) => {
            let choice = rng.f64();
            if choice < 0.25 {
                replacement.clone()
            } else if choice < 0.60 {
                Expr::BinOp(*op, Box::new(replace_random_subtree(l, replacement, rng)), r.clone())
            } else {
                Expr::BinOp(*op, l.clone(), Box::new(replace_random_subtree(r, replacement, rng)))
            }
        }
    }
}

fn crossover(a: &Expr, b: &Expr, rng: &mut Rng) -> Expr {
    let sub = random_subtree(b, rng).clone();
    let child = replace_random_subtree(a, &sub, rng);
    // Enforce max depth 3
    if child.depth() > 3 { a.clone() } else { child }
}

// ── Target Constants ────────────────────────────────────────────────
struct Target {
    name: &'static str,
    value: f64,
    category: &'static str,
}

fn targets() -> Vec<Target> {
    vec![
        // Fundamental physics
        Target { name: "c (speed of light)", value: 299792458.0, category: "physics" },
        Target { name: "h (Planck)", value: 6.626e-34, category: "physics" },
        Target { name: "k_B (Boltzmann)", value: 1.381e-23, category: "physics" },
        Target { name: "N_A (Avogadro)", value: 6.022e23, category: "physics" },
        Target { name: "G (gravitational)", value: 6.674e-11, category: "physics" },
        // Particle masses in MeV
        Target { name: "m_p (proton MeV)", value: 938.272, category: "particle" },
        Target { name: "m_e (electron MeV)", value: 0.511, category: "particle" },
        Target { name: "m_W (W boson GeV)", value: 80.377, category: "particle" },
        Target { name: "m_Z (Z boson GeV)", value: 91.188, category: "particle" },
        Target { name: "m_H (Higgs GeV)", value: 125.25, category: "particle" },
        // Mathematical constants
        Target { name: "pi", value: std::f64::consts::PI, category: "math" },
        Target { name: "e", value: std::f64::consts::E, category: "math" },
        Target { name: "golden ratio", value: 1.6180339887, category: "math" },
        Target { name: "sqrt(2)", value: std::f64::consts::SQRT_2, category: "math" },
        Target { name: "sqrt(3)", value: 1.7320508075688772, category: "math" },
        // Engineering constants already in atlas (verify + extend)
        Target { name: "ln(2)", value: 2.0_f64.ln(), category: "math" },
        Target { name: "ln(4/3)", value: (4.0/3.0_f64).ln(), category: "n6-known" },
        Target { name: "1/e (Boltzmann gate)", value: 1.0/std::f64::consts::E, category: "n6-known" },
        Target { name: "zeta(2)=pi^2/6", value: std::f64::consts::PI*std::f64::consts::PI/6.0, category: "math" },
        // Semiconductor / chip
        Target { name: "28nm (TSMC N5 pitch)", value: 28.0, category: "chip" },
        Target { name: "48nm (N3 gate)", value: 48.0, category: "chip" },
        Target { name: "132 (H100 SMs)", value: 132.0, category: "chip" },
        Target { name: "144 (AD102 SMs)", value: 144.0, category: "chip" },
        Target { name: "256 (typical width)", value: 256.0, category: "chip" },
        Target { name: "1024 (codebook size)", value: 1024.0, category: "chip" },
        // AI hyperparameters
        Target { name: "0.1 (universal reg)", value: 0.1, category: "ai" },
        Target { name: "0.95 (top-p/beta2)", value: 0.95, category: "ai" },
        Target { name: "0.9 (beta1)", value: 0.9, category: "ai" },
        Target { name: "128 (d_head)", value: 128.0, category: "ai" },
        Target { name: "8 (sigma-tau)", value: 8.0, category: "ai" },
        Target { name: "20 (Chinchilla ratio)", value: 20.0, category: "ai" },
        Target { name: "64 (codons)", value: 64.0, category: "bio" },
        Target { name: "96 (Tesla S / layers)", value: 96.0, category: "energy" },
        // Energy
        Target { name: "120 (H2 LHV)", value: 120.0, category: "energy" },
        Target { name: "60 (Hz / panel cells)", value: 60.0, category: "energy" },
        Target { name: "1.2 (PUE)", value: 1.2, category: "energy" },
        // Fine structure constant
        Target { name: "alpha (~1/137)", value: 1.0/137.036, category: "physics" },
        Target { name: "137 (1/alpha)", value: 137.036, category: "physics" },
        // Weinberg angle
        Target { name: "sin^2(theta_W)", value: 0.2312, category: "physics" },
        // Cosmology
        Target { name: "Omega_Lambda", value: 0.685, category: "cosmo" },
        Target { name: "Omega_matter", value: 0.315, category: "cosmo" },
        Target { name: "H0 (km/s/Mpc)", value: 67.4, category: "cosmo" },
    ]
}

// ── Fitness ─────────────────────────────────────────────────────────
#[derive(Clone)]
struct Match {
    target_idx: usize,
    rel_error: f64,
}

fn evaluate(expr: &Expr, tgts: &[Target]) -> (f64, Vec<Match>) {
    let val = match expr.eval() {
        Some(v) if v.is_finite() && v != 0.0 => v,
        _ => return (0.0, vec![]),
    };

    let mut score = 0.0;
    let mut matches = Vec::new();

    for (i, t) in tgts.iter().enumerate() {
        let rel_err = ((val - t.value) / t.value).abs();
        if rel_err < 0.05 {
            // Score: higher for closer matches
            let s = -rel_err.max(1e-15).log10();
            score += s;
            matches.push(Match { target_idx: i, rel_error: rel_err });
        }
    }

    // Penalize overly complex trees slightly
    let complexity_penalty = expr.node_count() as f64 * 0.05;
    // Bonus for simpler formulas that match
    let simplicity_bonus = if !matches.is_empty() {
        3.0 / (expr.node_count() as f64)
    } else {
        0.0
    };

    (score - complexity_penalty + simplicity_bonus, matches)
}

// ── Deduplication by value ──────────────────────────────────────────
fn val_key(v: f64) -> i64 {
    // Round to ~8 significant figures
    if v.abs() < 1e-30 { return 0; }
    let exp = v.abs().log10().floor();
    let mantissa = v / 10.0_f64.powf(exp);
    (mantissa * 1e8).round() as i64
}

// ── Main GA Loop ────────────────────────────────────────────────────
fn main() {
    let tgts = targets();
    let pop_size = 1000;
    let generations = 100;
    let elite_count = 100;
    let mut rng = Rng::new(42);

    println!("=== EVOLVE: n=6 Genetic Formula Miner ===");
    println!("Population: {}, Generations: {}", pop_size, generations);
    println!("Base constants: n=6, phi=2, tau=4, sigma=12, J2=24, sopfr=5, mu=1");
    println!("Targets: {} values across physics/math/chip/AI/energy", tgts.len());
    println!();

    // Initialize population
    let mut population: Vec<Expr> = (0..pop_size)
        .map(|_| random_tree(&mut rng, 3))
        .collect();

    // Also seed with known good formulas
    let seeds = vec![
        // sigma * n * phi = 144
        Expr::BinOp(Op::Mul, Box::new(Expr::BinOp(Op::Mul,
            Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(0)))),
            Box::new(Expr::Leaf(1))),
        // sigma * (sigma - mu) = 132
        Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(3)),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(6))))),
        // J2 - tau = 20
        Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(4)), Box::new(Expr::Leaf(2))),
        // sigma - tau = 8
        Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(2))),
        // sigma * sopfr = 60
        Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(5))),
        // phi ^ tau = 16
        Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(1)), Box::new(Expr::Leaf(2))),
        // sigma * (sigma - phi) = 120
        Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(3)),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1))))),
        // sigma / (sigma - phi) = 1.2
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(3)),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1))))),
        // mu / (sigma - phi) = 0.1
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(6)),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1))))),
        // phi ^ sigma = 4096
        Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(1)), Box::new(Expr::Leaf(3))),
        // n ^ sopfr = 7776
        Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(0)), Box::new(Expr::Leaf(5))),
    ];
    for (i, s) in seeds.into_iter().enumerate() {
        if i < pop_size { population[i] = s; }
    }

    let mut best_score = 0.0_f64;
    let mut best_gen = 0;

    // Collect all discoveries across generations
    let mut all_discoveries: Vec<(f64, Expr, Vec<Match>)> = Vec::new();

    for gen in 0..generations {
        // Evaluate
        let mut scored: Vec<(f64, usize)> = population.iter()
            .enumerate()
            .map(|(i, e)| {
                let (s, _) = evaluate(e, &tgts);
                (s, i)
            })
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let gen_best = scored[0].0;
        if gen_best > best_score {
            best_score = gen_best;
            best_gen = gen;
        }

        // Collect good matches this generation
        for &(sc, idx) in scored.iter().take(50) {
            if sc > 1.0 {
                let (_, matches) = evaluate(&population[idx], &tgts);
                if !matches.is_empty() {
                    all_discoveries.push((sc, population[idx].clone(), matches));
                }
            }
        }

        if gen % 20 == 0 {
            let best_expr = &population[scored[0].1];
            println!("Gen {:3}: best_score={:.2}, best_expr={}, val={:.6}",
                gen, gen_best, best_expr,
                best_expr.eval().unwrap_or(f64::NAN));
        }

        // Selection + reproduction
        let mut next_pop: Vec<Expr> = Vec::with_capacity(pop_size);

        // Elitism
        for i in 0..elite_count.min(scored.len()) {
            next_pop.push(population[scored[i].1].clone());
        }

        // Fill rest with crossover + mutation
        while next_pop.len() < pop_size {
            let p1_idx = scored[rng.usize(elite_count * 2)].1;
            let p2_idx = scored[rng.usize(elite_count * 2)].1;

            let mut child = if rng.f64() < 0.7 {
                crossover(&population[p1_idx], &population[p2_idx], &mut rng)
            } else {
                population[p1_idx].clone()
            };

            // Mutation
            if rng.f64() < 0.4 {
                child = mutate(&child, &mut rng);
            }

            // Enforce depth limit
            if child.depth() <= 3 {
                next_pop.push(child);
            } else {
                next_pop.push(random_tree(&mut rng, 3));
            }
        }

        // Inject some fresh random individuals
        for i in (pop_size - 50)..pop_size {
            next_pop[i] = random_tree(&mut rng, 3);
        }

        population = next_pop;
    }

    println!("\nBest score {:.2} found at generation {}", best_score, best_gen);

    // ── Deduplicate and rank all discoveries ────────────────────────
    // Sort by score descending
    all_discoveries.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // Deduplicate by (value, target_idx)
    let mut seen: std::collections::HashSet<(i64, usize)> = std::collections::HashSet::new();
    let mut unique: Vec<(f64, Expr, Vec<Match>)> = Vec::new();

    for (sc, expr, matches) in &all_discoveries {
        if let Some(v) = expr.eval() {
            let key = val_key(v);
            let tid = if matches.is_empty() { usize::MAX } else { matches[0].target_idx };
            if seen.insert((key, tid)) {
                unique.push((sc.clone(), expr.clone(), matches.clone()));
            }
        }
    }

    // ── Also do exhaustive enumeration of depth <= 2 ────────────────
    println!("\n=== Exhaustive Depth-2 Scan ===");
    let mut exhaustive_hits: Vec<(f64, Expr, Vec<Match>)> = Vec::new();

    // depth 0: just leaves
    for i in 0..7 {
        let e = Expr::Leaf(i);
        let (s, m) = evaluate(&e, &tgts);
        if s > 1.0 { exhaustive_hits.push((s, e, m)); }
    }

    // depth 1: op(leaf, leaf)
    for op_i in 0..5 {
        let op = Op::from_idx(op_i);
        for i in 0..7 {
            for j in 0..7 {
                let e = Expr::BinOp(op, Box::new(Expr::Leaf(i)), Box::new(Expr::Leaf(j)));
                let (s, m) = evaluate(&e, &tgts);
                if s > 1.0 { exhaustive_hits.push((s, e, m)); }
            }
        }
    }

    // depth 2: op(op(leaf,leaf), leaf) and op(leaf, op(leaf,leaf))
    for op1_i in 0..5 {
        let op1 = Op::from_idx(op1_i);
        for op2_i in 0..5 {
            let op2 = Op::from_idx(op2_i);
            for i in 0..7 {
                for j in 0..7 {
                    for k in 0..7 {
                        // op1(op2(i,j), k)
                        let e1 = Expr::BinOp(op1,
                            Box::new(Expr::BinOp(op2, Box::new(Expr::Leaf(i)), Box::new(Expr::Leaf(j)))),
                            Box::new(Expr::Leaf(k)));
                        let (s1, m1) = evaluate(&e1, &tgts);
                        if s1 > 1.0 { exhaustive_hits.push((s1, e1, m1)); }

                        // op1(k, op2(i,j))
                        let e2 = Expr::BinOp(op1,
                            Box::new(Expr::Leaf(k)),
                            Box::new(Expr::BinOp(op2, Box::new(Expr::Leaf(i)), Box::new(Expr::Leaf(j)))));
                        let (s2, m2) = evaluate(&e2, &tgts);
                        if s2 > 1.0 { exhaustive_hits.push((s2, e2, m2)); }
                    }
                }
            }
        }
    }
    println!("Exhaustive scan found {} raw hits", exhaustive_hits.len());

    // Merge exhaustive into unique
    for (sc, expr, matches) in exhaustive_hits {
        if let Some(v) = expr.eval() {
            let key = val_key(v);
            let tid = if matches.is_empty() { usize::MAX } else { matches[0].target_idx };
            if seen.insert((key, tid)) {
                unique.push((sc, expr, matches));
            }
        }
    }

    // Re-sort
    unique.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // ── Print Results ───────────────────────────────────────────────
    println!("\n{}", "=".repeat(70));
    println!("============================================================");
    println!("  TOP FORMULA DISCOVERIES (n=6 -> physical/engineering constants)");
    println!("============================================================\n");

    let display_count = unique.len().min(80);
    println!("{:<4} {:<40} {:>14} {:<28} {:>10}",
        "#", "Formula", "Value", "Target", "Rel Error");
    println!("{}", "-".repeat(100));

    for (rank, (score, expr, matches)) in unique.iter().take(display_count).enumerate() {
        let val = expr.eval().unwrap_or(f64::NAN);
        for m in matches {
            let t = &tgts[m.target_idx];
            let err_str = if m.rel_error < 1e-10 {
                "EXACT".to_string()
            } else if m.rel_error < 0.001 {
                format!("{:.6}%", m.rel_error * 100.0)
            } else {
                format!("{:.3}%", m.rel_error * 100.0)
            };
            println!("{:<4} {:<40} {:>14.6} {:<28} {:>10}",
                rank + 1, format!("{}", expr), val,
                format!("{} [{}]", t.name, t.category), err_str);
        }
    }

    // ── Summary by category ─────────────────────────────────────────
    println!("\n============================================================");
    println!("  SUMMARY BY CATEGORY");
    println!("============================================================\n");

    let categories = ["physics", "particle", "math", "n6-known", "chip", "ai", "bio", "energy", "cosmo"];
    for cat in &categories {
        let cat_matches: Vec<_> = unique.iter()
            .filter(|(_, _, ms)| ms.iter().any(|m| tgts[m.target_idx].category == *cat))
            .collect();
        if !cat_matches.is_empty() {
            println!("  {} ({} formulas found):", cat.to_uppercase(), cat_matches.len());
            for (_, expr, ms) in cat_matches.iter().take(5) {
                for m in ms.iter().filter(|m| tgts[m.target_idx].category == *cat) {
                    let t = &tgts[m.target_idx];
                    let err_pct = m.rel_error * 100.0;
                    println!("    {} = {:.6}  ~  {} ({:.4}%)",
                        expr, expr.eval().unwrap_or(f64::NAN), t.name, err_pct);
                }
            }
            println!();
        }
    }

    // ── Exact integer matches ───────────────────────────────────────
    println!("============================================================");
    println!("  EXACT INTEGER MATCHES (n=6 formulas = integer targets)");
    println!("============================================================\n");

    let mut int_matches: Vec<(Expr, f64, &str)> = Vec::new();
    for (_, expr, ms) in &unique {
        if let Some(v) = expr.eval() {
            for m in ms {
                if m.rel_error < 1e-10 {
                    int_matches.push((expr.clone(), v, tgts[m.target_idx].name));
                }
            }
        }
    }
    int_matches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    int_matches.dedup_by(|a, b| val_key(a.1) == val_key(b.1) && a.2 == b.2);

    for (expr, val, name) in &int_matches {
        println!("  {} = {} = {}", expr, val, name);
    }

    println!("\n============================================================");
    println!("  Total unique formulas discovered: {}", unique.len());
    println!("  Exact matches: {}", int_matches.len());
    println!("============================================================");
}
