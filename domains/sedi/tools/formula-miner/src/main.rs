// SEDI Formula Miner v3 — Genetic Algorithm for SETI/Physics Constants
// Discovers n=6 formulas matching SEDI-domain targets:
//   Drake equation, SETI frequencies, particle physics, cosmology, exoplanet resonances
//
// v3 upgrades (on top of v2):
//   - Physics formula templates: A * 10^B patterns for extreme-scale constants
//   - GA seeded with engine v3 best discoveries (6pi^5, sigma^2-7, etc.)
//   - --json flag for structured JSON output
//
// v2 upgrades:
//   - Unary operations (Sqrt, Ln, Exp, Inv, Floor, Sin, Cos)
//   - Transcendental constants (pi, e)
//   - Depth-3 exhaustive scan
//   - Multi-objective fitness with cross-category bonus
//   - FALSIFY filter (Texas sharpshooter correction)
//   - Increased GA parameters (pop=2000, gen=200, 3 independent runs)
//   - 9 additional targets (Koide, Cabibbo, CP violation, etc.)
//   - Bayesian evidence & tier classification

use std::collections::HashSet;
use std::fmt;
use std::time::Instant;

// -- N=6 Base Constants + Transcendentals -----------------------------------
const N: f64 = 6.0;
const PHI: f64 = 2.0;       // phi(6) = 2
const TAU: f64 = 4.0;       // tau(6) = 4  (number of divisors)
const SIGMA: f64 = 12.0;    // sigma(6) = 12
const J2: f64 = 24.0;       // Jordan J_2(6) = 24
const SOPFR: f64 = 5.0;     // sopfr(6) = 2+3 = 5
const MU: f64 = 1.0;        // mu(6) = 1 (Mobius)
const PI_CONST: f64 = std::f64::consts::PI;
const E_CONST: f64 = std::f64::consts::E;

const NUM_CONSTS: usize = 9;
const CONST_NAMES: [&str; NUM_CONSTS] = ["n", "phi", "tau", "sigma", "J2", "sopfr", "mu", "pi", "e"];
const CONST_VALS: [f64; NUM_CONSTS] = [N, PHI, TAU, SIGMA, J2, SOPFR, MU, PI_CONST, E_CONST];

// -- Operations --------------------------------------------------------------
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
        if r.is_finite() && r.abs() < 1e15 { Some(r) } else { None }
    }
    fn sym(self) -> &'static str {
        match self { Op::Add=>"+", Op::Sub=>"-", Op::Mul=>"*", Op::Div=>"/", Op::Pow=>"^" }
    }
    fn from_idx(i: usize) -> Op {
        match i % 5 { 0=>Op::Add, 1=>Op::Sub, 2=>Op::Mul, 3=>Op::Div, _=>Op::Pow }
    }
}

// -- Unary Operations --------------------------------------------------------
#[derive(Clone, Copy, PartialEq)]
enum UnOp { Sqrt, Ln, Exp, Inv, Floor, Sin, Cos }

const NUM_UNOPS: usize = 7;

impl UnOp {
    fn eval(self, x: f64) -> Option<f64> {
        let r = match self {
            UnOp::Sqrt => { if x < 0.0 { return None; } x.sqrt() },
            UnOp::Ln   => { if x <= 0.0 { return None; } x.ln() },
            UnOp::Exp  => { if x.abs() > 35.0 { return None; } x.exp() },
            UnOp::Inv  => { if x.abs() < 1e-15 { return None; } 1.0 / x },
            UnOp::Floor => x.floor(),
            UnOp::Sin  => x.sin(),
            UnOp::Cos  => x.cos(),
        };
        if r.is_finite() && r.abs() < 1e15 { Some(r) } else { None }
    }
    fn sym(self) -> &'static str {
        match self {
            UnOp::Sqrt => "sqrt", UnOp::Ln => "ln", UnOp::Exp => "exp",
            UnOp::Inv => "inv", UnOp::Floor => "floor", UnOp::Sin => "sin", UnOp::Cos => "cos",
        }
    }
    fn from_idx(i: usize) -> UnOp {
        match i % NUM_UNOPS {
            0 => UnOp::Sqrt, 1 => UnOp::Ln, 2 => UnOp::Exp, 3 => UnOp::Inv,
            4 => UnOp::Floor, 5 => UnOp::Sin, _ => UnOp::Cos,
        }
    }
}

// -- Expression Tree ---------------------------------------------------------
#[derive(Clone)]
enum Expr {
    Leaf(usize),                           // index into CONST_VALS
    BinOp(Op, Box<Expr>, Box<Expr>),
    UnaryOp(UnOp, Box<Expr>),              // NEW in v2
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
            Expr::UnaryOp(op, child) => {
                let cv = child.eval()?;
                op.eval(cv)
            }
        }
    }

    fn depth(&self) -> usize {
        match self {
            Expr::Leaf(_) => 0,
            Expr::BinOp(_, l, r) => 1 + l.depth().max(r.depth()),
            Expr::UnaryOp(_, c) => 1 + c.depth(),
        }
    }

    fn node_count(&self) -> usize {
        match self {
            Expr::Leaf(_) => 1,
            Expr::BinOp(_, l, r) => 1 + l.node_count() + r.node_count(),
            Expr::UnaryOp(_, c) => 1 + c.node_count(),
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
            Expr::UnaryOp(op, c) => {
                write!(f, "{}({})", op.sym(), c)
            }
        }
    }
}

// -- Simple RNG (xorshift64) ------------------------------------------------
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

// -- Random Tree Generation --------------------------------------------------
fn random_tree(rng: &mut Rng, max_depth: usize) -> Expr {
    if max_depth == 0 || rng.f64() < 0.30 {
        Expr::Leaf(rng.usize(NUM_CONSTS))
    } else if rng.f64() < 0.20 {
        // Unary op
        let uop = UnOp::from_idx(rng.usize(NUM_UNOPS));
        let child = random_tree(rng, max_depth - 1);
        Expr::UnaryOp(uop, Box::new(child))
    } else {
        let op = Op::from_idx(rng.usize(5));
        let l = random_tree(rng, max_depth - 1);
        let r = random_tree(rng, max_depth - 1);
        Expr::BinOp(op, Box::new(l), Box::new(r))
    }
}

// -- Mutation ----------------------------------------------------------------
fn mutate(expr: &Expr, rng: &mut Rng) -> Expr {
    let choice = rng.f64();
    match expr {
        Expr::Leaf(_) => {
            if choice < 0.50 {
                Expr::Leaf(rng.usize(NUM_CONSTS))
            } else if choice < 0.75 {
                let op = Op::from_idx(rng.usize(5));
                let other = Expr::Leaf(rng.usize(NUM_CONSTS));
                if rng.f64() < 0.5 {
                    Expr::BinOp(op, Box::new(expr.clone()), Box::new(other))
                } else {
                    Expr::BinOp(op, Box::new(other), Box::new(expr.clone()))
                }
            } else {
                // Wrap in unary op
                let uop = UnOp::from_idx(rng.usize(NUM_UNOPS));
                Expr::UnaryOp(uop, Box::new(expr.clone()))
            }
        }
        Expr::UnaryOp(uop, c) => {
            if choice < 0.20 {
                random_tree(rng, 2)
            } else if choice < 0.40 {
                // Change unary op
                let new_uop = UnOp::from_idx(rng.usize(NUM_UNOPS));
                Expr::UnaryOp(new_uop, c.clone())
            } else if choice < 0.55 {
                // Unwrap
                *c.clone()
            } else {
                Expr::UnaryOp(*uop, Box::new(mutate(c, rng)))
            }
        }
        Expr::BinOp(op, l, r) => {
            if choice < 0.12 {
                random_tree(rng, 2)
            } else if choice < 0.24 {
                let new_op = Op::from_idx(rng.usize(5));
                Expr::BinOp(new_op, l.clone(), r.clone())
            } else if choice < 0.36 {
                if rng.f64() < 0.5 { *l.clone() } else { *r.clone() }
            } else if choice < 0.48 {
                // Wrap in unary
                let uop = UnOp::from_idx(rng.usize(NUM_UNOPS));
                Expr::UnaryOp(uop, Box::new(expr.clone()))
            } else if choice < 0.74 {
                Expr::BinOp(*op, Box::new(mutate(l, rng)), r.clone())
            } else {
                Expr::BinOp(*op, l.clone(), Box::new(mutate(r, rng)))
            }
        }
    }
}

// -- Crossover ---------------------------------------------------------------
fn random_subtree<'a>(expr: &'a Expr, rng: &mut Rng) -> &'a Expr {
    match expr {
        Expr::Leaf(_) => expr,
        Expr::UnaryOp(_, c) => {
            if rng.f64() < 0.4 { expr } else { random_subtree(c, rng) }
        }
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
        Expr::UnaryOp(uop, c) => {
            if rng.f64() < 0.3 {
                replacement.clone()
            } else {
                Expr::UnaryOp(*uop, Box::new(replace_random_subtree(c, replacement, rng)))
            }
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
    if child.depth() > 4 { a.clone() } else { child }
}

// -- SEDI Target Constants ---------------------------------------------------
struct Target {
    name: &'static str,
    value: f64,
    category: &'static str,
}

fn targets() -> Vec<Target> {
    vec![
        // === Drake Equation Parameters ===
        Target { name: "R* (star formation)", value: 1.5, category: "drake" },
        Target { name: "R* (upper)", value: 3.0, category: "drake" },
        Target { name: "f_p (planets)", value: 1.0, category: "drake" },
        Target { name: "n_e (habitable)", value: 0.4, category: "drake" },
        Target { name: "f_l (life)", value: 0.1, category: "drake" },
        Target { name: "f_i (intelligent)", value: 0.01, category: "drake" },
        Target { name: "f_c (communicating)", value: 0.1, category: "drake" },
        Target { name: "f_c (upper)", value: 0.2, category: "drake" },
        Target { name: "L (civ lifetime)", value: 1000.0, category: "drake" },
        Target { name: "L (upper)", value: 10000.0, category: "drake" },

        // === SETI Frequencies ===
        Target { name: "H 21cm (MHz)", value: 1420.405, category: "seti" },
        Target { name: "OH radical (MHz)", value: 1665.0, category: "seti" },
        Target { name: "Water hole center (MHz)", value: 1545.0, category: "seti" },
        Target { name: "Pi*GHz (MHz)", value: 3141.59, category: "seti" },
        Target { name: "H-alpha (nm)", value: 656.3, category: "seti" },

        // === Particle Physics (SEDI verified) ===
        Target { name: "m_e (MeV)", value: 0.511, category: "particle" },
        Target { name: "m_mu (MeV)", value: 105.658, category: "particle" },
        Target { name: "m_tau (MeV)", value: 1776.86, category: "particle" },
        Target { name: "m_p/m_e", value: 1836.15, category: "particle" },
        Target { name: "alpha^-1", value: 137.036, category: "particle" },
        Target { name: "sin2_theta_W", value: 0.2312, category: "particle" },
        Target { name: "Koide Q=2/3", value: 0.6667, category: "particle" },
        Target { name: "sin(theta_C)", value: 0.2253, category: "particle" },
        Target { name: "CP violation delta", value: 1.196, category: "particle" },
        Target { name: "neutron lifetime (s)", value: 879.4, category: "particle" },
        Target { name: "muon (g-2)/2", value: 1.16592e-3, category: "particle" },

        // === Cosmological ===
        Target { name: "H0 (km/s/Mpc)", value: 67.4, category: "cosmology" },
        Target { name: "Omega_m", value: 0.315, category: "cosmology" },
        Target { name: "Omega_Lambda", value: 0.685, category: "cosmology" },
        Target { name: "T_CMB (K)", value: 2.7255, category: "cosmology" },
        Target { name: "eta (baryon asym)", value: 6.14e-10, category: "cosmology" },
        Target { name: "Omega_b h^2", value: 0.0224, category: "cosmology" },
        Target { name: "CMB quadrupole C2 (uK^2)", value: 1090.0, category: "cosmology" },
        Target { name: "reionization z_re", value: 7.7, category: "cosmology" },
        Target { name: "age of universe (Gyr)", value: 13.8, category: "cosmology" },
        Target { name: "baryon/photon eta", value: 6.14e-10, category: "cosmology" },

        // === Exoplanet Resonances (SEDI n6_tracker) ===
        Target { name: "HD110067 b/g", value: 6.0096, category: "exoplanet" },
        Target { name: "TRAPPIST-1 r1", value: 1.603, category: "exoplanet" },
        Target { name: "TRAPPIST-1 r2", value: 1.672, category: "exoplanet" },
        Target { name: "TRAPPIST-1 r3", value: 1.506, category: "exoplanet" },
        Target { name: "TOI-1136 b/d", value: 3.0004, category: "exoplanet" },
    ]
}

// -- FALSIFY Filter ----------------------------------------------------------
fn is_common_number(v: f64) -> bool {
    let common = [
        1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 8.0, 10.0, 12.0, 16.0, 24.0,
        32.0, 64.0, 128.0, 256.0, 0.5, 0.25, 0.1, 0.01,
    ];
    common.iter().any(|&c| (v - c).abs() < 1e-10)
}

fn texas_sharpshooter_correction(target: f64, all_values: &[f64], tolerance: f64) -> f64 {
    let hits = all_values.iter()
        .filter(|&&v| {
            if target.abs() < 1e-15 { return false; }
            ((v - target) / target).abs() < tolerance
        })
        .count();
    hits as f64 / all_values.len().max(1) as f64
}

/// FALSIFY verdict: how likely is this match by chance?
fn falsify_verdict(
    rel_error: f64,
    expr_val: f64,
    target_val: f64,
    node_count: usize,
    all_values: &[f64],
) -> &'static str {
    // Reject common numbers matched by trivial expressions
    if is_common_number(expr_val) && node_count <= 2 {
        return "REJECT";
    }

    // Texas sharpshooter: how many of our evaluated values hit near this target?
    let tss = texas_sharpshooter_correction(target_val, all_values, 0.05);
    if tss > 0.10 {
        return "REJECT"; // >10% of all values hit near this target
    }

    if rel_error < 1e-6 && node_count <= 5 && tss < 0.01 {
        "STRONG"
    } else if rel_error < 0.005 && node_count <= 7 {
        "WEAK"
    } else {
        "REJECT"
    }
}

// -- Multi-Objective Fitness (v2) --------------------------------------------
#[derive(Clone)]
struct Match {
    target_idx: usize,
    rel_error: f64,
}

fn evaluate_v2(expr: &Expr, tgts: &[Target]) -> (f64, Vec<Match>) {
    let val = match expr.eval() {
        Some(v) if v.is_finite() && v != 0.0 && v.abs() < 1e15 => v,
        _ => return (0.0, vec![]),
    };

    let mut score = 0.0;
    let mut matches = Vec::new();
    let mut categories_hit: HashSet<&str> = HashSet::new();

    for (i, t) in tgts.iter().enumerate() {
        let rel_err = ((val - t.value) / t.value).abs();
        if rel_err < 0.05 {
            let s = -rel_err.max(1e-15).log10(); // precision score
            score += s;
            matches.push(Match { target_idx: i, rel_error: rel_err });
            categories_hit.insert(t.category);
        }
    }

    if matches.is_empty() {
        return (0.0, vec![]);
    }

    // Cross-category bonus
    if categories_hit.len() >= 2 { score *= 1.5; }
    if categories_hit.len() >= 3 { score *= 2.0; }

    // Simplicity bonus
    let simplicity = 5.0 / (expr.node_count() as f64);
    score += simplicity;

    // Complexity penalty
    score -= expr.node_count() as f64 * 0.03;

    (score, matches)
}

// -- Bayesian evidence (bits) ------------------------------------------------
fn bayesian_bits(rel_error: f64, node_count: usize, num_targets: usize) -> f64 {
    // Prior: chance of random formula hitting within rel_error of any target
    // Approximate: 2 * rel_error * num_targets (fraction of value space covered)
    let prior = (2.0 * rel_error.max(1e-15) * num_targets as f64).min(1.0);
    // Complexity prior: simpler formulas are more likely a priori
    let complexity_prior = 0.5_f64.powi(node_count as i32);
    // Posterior odds ~ 1 / (prior * complexity_prior), but capped
    let evidence = (1.0 / (prior * complexity_prior)).log2();
    evidence.max(0.0).min(100.0)
}

// -- Tier classification -----------------------------------------------------
fn tier_label(bits: f64, rel_error: f64, verdict: &str) -> &'static str {
    if verdict == "REJECT" { return "E"; }
    if bits > 30.0 && rel_error < 1e-6  { return "A"; }
    if bits > 20.0 && rel_error < 1e-4  { return "B"; }
    if bits > 10.0 && rel_error < 0.01  { return "C"; }
    if bits > 5.0                       { return "D"; }
    "E"
}

// -- Deduplication by value --------------------------------------------------
fn val_key(v: f64) -> i64 {
    if v.abs() < 1e-30 { return 0; }
    let exp = v.abs().log10().floor();
    let mantissa = v / 10.0_f64.powf(exp);
    (mantissa * 1e8).round() as i64
}

// -- Physics formula templates (v3) ------------------------------------------
/// Generate extreme-scale expressions: A * 10^B patterns.
/// These cover physics constants that span many orders of magnitude
/// (e.g. baryon asymmetry ~6e-10, cosmological constants, etc.)
fn physics_templates() -> Vec<Expr> {
    let mut out = Vec::new();

    // For each pair of base constants (A, B), generate A * 10^B
    // where 10^B is represented as (sigma - phi)^B  (since sigma-phi = 10)
    // and also literal power-of-10 via nested ops
    let ten_expr = Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1)));
    // sigma(3) - phi(1) = 12 - 2 = 10

    for a_idx in 0..NUM_CONSTS {
        for b_idx in 0..NUM_CONSTS {
            // A * (10 ^ B)
            let power = Expr::BinOp(Op::Pow, Box::new(ten_expr.clone()), Box::new(Expr::Leaf(b_idx)));
            let template = Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(a_idx)), Box::new(power.clone()));
            if let Some(v) = template.eval() {
                if v.is_finite() && v.abs() < 1e15 && v.abs() > 1e-15 {
                    out.push(template);
                }
            }

            // A * (10 ^ -B)  =  A / (10 ^ B)
            let neg_power = Expr::BinOp(Op::Div, Box::new(Expr::Leaf(a_idx)), Box::new(power));
            if let Some(v) = neg_power.eval() {
                if v.is_finite() && v.abs() < 1e15 && v.abs() > 1e-15 {
                    out.push(neg_power);
                }
            }
        }
    }

    // Also: (A op B) * 10^C for depth-1 A op B
    for op_i in 0..5 {
        let op = Op::from_idx(op_i);
        for a_idx in 0..NUM_CONSTS {
            for b_idx in 0..NUM_CONSTS {
                let ab = Expr::BinOp(op, Box::new(Expr::Leaf(a_idx)), Box::new(Expr::Leaf(b_idx)));
                if let Some(ab_val) = ab.eval() {
                    if !ab_val.is_finite() || ab_val.abs() >= 1e15 { continue; }
                    for c_idx in 0..NUM_CONSTS {
                        // (A op B) * 10^C
                        let power = Expr::BinOp(Op::Pow, Box::new(ten_expr.clone()), Box::new(Expr::Leaf(c_idx)));
                        let template = Expr::BinOp(Op::Mul, Box::new(ab.clone()), Box::new(power.clone()));
                        if let Some(v) = template.eval() {
                            if v.is_finite() && v.abs() < 1e15 && v.abs() > 1e-15 {
                                out.push(template);
                            }
                        }
                        // (A op B) / 10^C
                        let neg = Expr::BinOp(Op::Div, Box::new(ab.clone()), Box::new(power));
                        if let Some(v) = neg.eval() {
                            if v.is_finite() && v.abs() < 1e15 && v.abs() > 1e-15 {
                                out.push(neg);
                            }
                        }
                    }
                }
            }
        }
    }

    out
}

// -- Exhaustive enumeration helpers ------------------------------------------
/// Generate all depth-0 expressions (leaves only)
fn gen_depth0() -> Vec<Expr> {
    (0..NUM_CONSTS).map(|i| Expr::Leaf(i)).collect()
}

/// Generate all depth-1 expressions (one op applied to depth-0 children)
fn gen_depth1(depth0: &[Expr]) -> Vec<Expr> {
    let mut out = Vec::new();
    // BinOp(op, leaf, leaf)
    for op_i in 0..5 {
        let op = Op::from_idx(op_i);
        for l in depth0 {
            for r in depth0 {
                out.push(Expr::BinOp(op, Box::new(l.clone()), Box::new(r.clone())));
            }
        }
    }
    // UnaryOp(uop, leaf)
    for uop_i in 0..NUM_UNOPS {
        let uop = UnOp::from_idx(uop_i);
        for c in depth0 {
            out.push(Expr::UnaryOp(uop, Box::new(c.clone())));
        }
    }
    out
}

/// Evaluate and collect hits, deduplicating by (val_key, target_idx).
fn collect_exhaustive_hits(
    exprs: &[Expr],
    tgts: &[Target],
    seen: &mut HashSet<(i64, usize)>,
    out: &mut Vec<(f64, Expr, Vec<Match>)>,
) -> usize {
    let mut raw_count = 0;
    for e in exprs {
        let (s, m) = evaluate_v2(e, tgts);
        if s > 1.0 && !m.is_empty() {
            raw_count += 1;
            if let Some(v) = e.eval() {
                let key = val_key(v);
                let tid = m[0].target_idx;
                if seen.insert((key, tid)) {
                    out.push((s, e.clone(), m));
                }
            }
        }
    }
    raw_count
}

// -- Main GA Loop ------------------------------------------------------------
fn run_ga(
    tgts: &[Target],
    seed: u64,
    pop_size: usize,
    generations: usize,
    elite_count: usize,
    fresh_inject: usize,
    run_label: &str,
) -> Vec<(f64, Expr, Vec<Match>)> {
    let mut rng = Rng::new(seed);
    let mut population: Vec<Expr> = (0..pop_size)
        .map(|_| random_tree(&mut rng, 4))
        .collect();

    // Seed with known SEDI-relevant formulas (indices: n=0, phi=1, tau=2, sigma=3, J2=4, sopfr=5, mu=6, pi=7, e=8)
    let seeds: Vec<Expr> = vec![
        // === Engine v3 best discoveries ===
        // 6 * pi^5 = 6 * 306.02 = 1836.12 ~ m_p/m_e  (KEY SEDI FORMULA!)
        Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(0)),
            Box::new(Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(7)), Box::new(Expr::Leaf(5))))),
        // sigma^2 - 7 = 144 - 7 = 137 ~ alpha^-1
        // 7 = sopfr + phi = 5 + 2
        Expr::BinOp(Op::Sub,
            Box::new(Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1)))),
            Box::new(Expr::BinOp(Op::Add, Box::new(Expr::Leaf(5)), Box::new(Expr::Leaf(1))))),
        // sigma^3 - sigma*J2 - sopfr*tau = 1728 - 288 - 20 = 1420 ~ H 21cm
        // Simplified: (sigma^3 - sigma*J2) - sopfr*tau
        Expr::BinOp(Op::Sub,
            Box::new(Expr::BinOp(Op::Sub,
                Box::new(Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(3)),
                    Box::new(Expr::BinOp(Op::Add, Box::new(Expr::Leaf(1)), Box::new(Expr::Leaf(6)))))),
                Box::new(Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(4)))))),
            Box::new(Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(5)), Box::new(Expr::Leaf(2))))),
        // (sigma - mu) / (sigma - phi) = 11/10 = 1.1 ~ Lambda scaling
        Expr::BinOp(Op::Div,
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(6)))),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1))))),

        // === Original v2 seeds ===
        // n = 6.0 ~ HD110067 b/g ratio 6.0096
        Expr::Leaf(0),
        // tau - mu = 3 ~ TOI-1136 b/d 3.0004
        Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(2)), Box::new(Expr::Leaf(6))),
        // sigma / tau = 3
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(2))),
        // n / tau = 1.5 ~ R*
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(0)), Box::new(Expr::Leaf(2))),
        // mu / sigma = 0.0833
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(6)), Box::new(Expr::Leaf(3))),
        // mu / (sigma - phi) = 0.1 ~ f_l, f_c
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(6)),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1))))),
        // phi / (sigma - phi) = 0.2 ~ f_c upper
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(1)),
            Box::new(Expr::BinOp(Op::Sub, Box::new(Expr::Leaf(3)), Box::new(Expr::Leaf(1))))),
        // n * pi^5 ~ 1836.12 ~ m_p/m_e  (alternate form with sopfr exponent)
        Expr::BinOp(Op::Mul, Box::new(Expr::Leaf(0)),
            Box::new(Expr::BinOp(Op::Pow, Box::new(Expr::Leaf(7)), Box::new(Expr::Leaf(4))))),
        // phi / n = 1/3 ~ Koide-related
        Expr::BinOp(Op::Div, Box::new(Expr::Leaf(1)), Box::new(Expr::Leaf(0))),
        // sin(pi / n) = sin(pi/6) = 0.5
        Expr::UnaryOp(UnOp::Sin, Box::new(Expr::BinOp(Op::Div,
            Box::new(Expr::Leaf(7)), Box::new(Expr::Leaf(0))))),
        // sqrt(n) = 2.449
        Expr::UnaryOp(UnOp::Sqrt, Box::new(Expr::Leaf(0))),
        // ln(J2) = 3.178
        Expr::UnaryOp(UnOp::Ln, Box::new(Expr::Leaf(4))),
    ];
    for (i, s) in seeds.into_iter().enumerate() {
        if i < pop_size { population[i] = s; }
    }

    let mut best_score = 0.0_f64;
    let mut best_gen = 0;
    let mut all_discoveries: Vec<(f64, Expr, Vec<Match>)> = Vec::new();

    for gen in 0..generations {
        let mut scored: Vec<(f64, usize)> = population.iter()
            .enumerate()
            .map(|(i, e)| {
                let (s, _) = evaluate_v2(e, tgts);
                (s, i)
            })
            .collect();
        scored.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

        let gen_best = scored[0].0;
        if gen_best > best_score {
            best_score = gen_best;
            best_gen = gen;
        }

        // Collect good matches
        for &(sc, idx) in scored.iter().take(100) {
            if sc > 1.0 {
                let (_, matches) = evaluate_v2(&population[idx], tgts);
                if !matches.is_empty() {
                    all_discoveries.push((sc, population[idx].clone(), matches));
                }
            }
        }

        if gen % 40 == 0 {
            let best_expr = &population[scored[0].1];
            println!("  {} Gen {:3}: best_score={:.2}, expr={}, val={:.6}",
                run_label, gen, gen_best, best_expr,
                best_expr.eval().unwrap_or(f64::NAN));
        }

        // Selection + reproduction
        let mut next_pop: Vec<Expr> = Vec::with_capacity(pop_size);

        // Elitism
        for i in 0..elite_count.min(scored.len()) {
            next_pop.push(population[scored[i].1].clone());
        }

        // Fill rest with crossover + mutation
        while next_pop.len() < pop_size - fresh_inject {
            let p1_idx = scored[rng.usize(elite_count * 2)].1;
            let p2_idx = scored[rng.usize(elite_count * 2)].1;

            let mut child = if rng.f64() < 0.7 {
                crossover(&population[p1_idx], &population[p2_idx], &mut rng)
            } else {
                population[p1_idx].clone()
            };

            if rng.f64() < 0.4 {
                child = mutate(&child, &mut rng);
            }

            if child.depth() <= 4 {
                next_pop.push(child);
            } else {
                next_pop.push(random_tree(&mut rng, 3));
            }
        }

        // Inject fresh random individuals
        while next_pop.len() < pop_size {
            next_pop.push(random_tree(&mut rng, 4));
        }

        population = next_pop;
    }

    println!("  {} finished: best_score={:.2} at gen {}", run_label, best_score, best_gen);
    all_discoveries
}

fn main() {
    let start_time = Instant::now();
    let tgts = targets();

    // Parse --json flag
    let args: Vec<String> = std::env::args().collect();
    let json_output = args.iter().any(|a| a == "--json");

    let pop_size = 2000;
    let generations = 200;
    let elite_count = 200;
    let fresh_inject = 100;
    let num_runs = 3;

    println!("=== SEDI Formula Miner v3: n=6 Genetic Search ===");
    println!("Population: {}, Generations: {}, Runs: {}", pop_size, generations, num_runs);
    println!("Constants: n=6, phi=2, tau=4, sigma=12, J2=24, sopfr=5, mu=1, pi, e");
    println!("Ops: +, -, *, /, ^  |  Unary: sqrt, ln, exp, inv, floor, sin, cos");
    println!("Targets: {} values across drake/seti/particle/cosmology/exoplanet", tgts.len());
    println!();

    // === Multiple independent GA runs ===
    let mut all_discoveries: Vec<(f64, Expr, Vec<Match>)> = Vec::new();

    for run in 0..num_runs {
        let label = format!("[Run {}]", run + 1);
        let seed = 42 + run as u64 * 7919; // different primes for diversity
        let discoveries = run_ga(&tgts, seed, pop_size, generations, elite_count, fresh_inject, &label);
        all_discoveries.extend(discoveries);
        println!();
    }

    // === Deduplicate GA results ===
    all_discoveries.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    let mut seen: HashSet<(i64, usize)> = HashSet::new();
    let mut unique: Vec<(f64, Expr, Vec<Match>)> = Vec::new();

    for (sc, expr, matches) in &all_discoveries {
        if let Some(v) = expr.eval() {
            if !v.is_finite() || v.abs() > 1e15 { continue; }
            let key = val_key(v);
            let tid = if matches.is_empty() { usize::MAX } else { matches[0].target_idx };
            if seen.insert((key, tid)) {
                unique.push((*sc, expr.clone(), matches.clone()));
            }
        }
    }

    // === Physics templates (v3) ===
    println!("=== Physics Templates (A * 10^B patterns) ===");
    let phys_templates = physics_templates();
    println!("  Generated {} physics template expressions", phys_templates.len());
    let phys_raw = collect_exhaustive_hits(&phys_templates, &tgts, &mut seen, &mut unique);
    println!("  Physics template hits: {} raw, {} unique added", phys_raw, unique.len());
    println!();

    // === Exhaustive enumeration depth <= 3 ===
    println!("=== Exhaustive Depth-3 Scan ===");

    let d0 = gen_depth0();
    println!("  Depth 0: {} expressions", d0.len());

    let d1 = gen_depth1(&d0);
    println!("  Depth 1: {} expressions", d1.len());

    // Collect depth 0+1 hits
    collect_exhaustive_hits(&d0, &tgts, &mut seen, &mut unique);
    collect_exhaustive_hits(&d1, &tgts, &mut seen, &mut unique);

    // Depth 2: BinOp(d1, d0) + BinOp(d0, d1) + UnaryOp(d1) + BinOp(d1_left, d1_right) partial
    let mut d2_count = 0_usize;
    let mut d2_hits = Vec::new();

    // BinOp(depth1_expr, leaf) and BinOp(leaf, depth1_expr)
    for op_i in 0..5 {
        let op = Op::from_idx(op_i);
        for d1e in &d1 {
            for d0e in &d0 {
                let e1 = Expr::BinOp(op, Box::new(d1e.clone()), Box::new(d0e.clone()));
                let e2 = Expr::BinOp(op, Box::new(d0e.clone()), Box::new(d1e.clone()));
                d2_count += 2;
                let (s1, m1) = evaluate_v2(&e1, &tgts);
                if s1 > 1.0 && !m1.is_empty() { d2_hits.push((s1, e1, m1)); }
                let (s2, m2) = evaluate_v2(&e2, &tgts);
                if s2 > 1.0 && !m2.is_empty() { d2_hits.push((s2, e2, m2)); }
            }
        }
    }
    // UnaryOp(depth1_expr)
    for uop_i in 0..NUM_UNOPS {
        let uop = UnOp::from_idx(uop_i);
        for d1e in &d1 {
            let e = Expr::UnaryOp(uop, Box::new(d1e.clone()));
            d2_count += 1;
            let (s, m) = evaluate_v2(&e, &tgts);
            if s > 1.0 && !m.is_empty() { d2_hits.push((s, e, m)); }
        }
    }
    println!("  Depth 2: {} expressions, {} raw hits", d2_count, d2_hits.len());

    for (sc, expr, matches) in d2_hits {
        if let Some(v) = expr.eval() {
            let key = val_key(v);
            let tid = if matches.is_empty() { usize::MAX } else { matches[0].target_idx };
            if seen.insert((key, tid)) {
                unique.push((sc, expr, matches));
            }
        }
    }

    // Depth 3: BinOp(depth2_expr_type, leaf) and UnaryOp(depth2_expr) — sampled
    // We cannot enumerate all depth-2 x depth-2, so we do depth-2 x depth-0
    // and UnaryOp(depth-2). This covers the most productive patterns.
    let mut d3_count = 0_usize;
    let mut d3_hits = Vec::new();

    // For depth 3 we need actual depth-2 expressions. We'll generate a subset:
    // BinOp(d1, d0) for all ops — these are the most useful depth-2 forms
    let mut d2_exprs: Vec<Expr> = Vec::new();
    for op_i in 0..5 {
        let op = Op::from_idx(op_i);
        for d1e in &d1 {
            for d0e in &d0 {
                let e = Expr::BinOp(op, Box::new(d1e.clone()), Box::new(d0e.clone()));
                if let Some(v) = e.eval() {
                    if v.is_finite() && v.abs() < 1e15 {
                        d2_exprs.push(e);
                    }
                }
            }
        }
    }
    // Also add UnaryOp(d1)
    for uop_i in 0..NUM_UNOPS {
        let uop = UnOp::from_idx(uop_i);
        for d1e in &d1 {
            let e = Expr::UnaryOp(uop, Box::new(d1e.clone()));
            if let Some(v) = e.eval() {
                if v.is_finite() && v.abs() < 1e15 {
                    d2_exprs.push(e);
                }
            }
        }
    }

    // Deduplicate d2_exprs by value to keep manageable
    let mut d2_val_seen: HashSet<i64> = HashSet::new();
    let mut d2_unique: Vec<Expr> = Vec::new();
    for e in &d2_exprs {
        if let Some(v) = e.eval() {
            let k = val_key(v);
            if d2_val_seen.insert(k) {
                d2_unique.push(e.clone());
            }
        }
    }
    println!("  Depth 2 unique values for depth-3 base: {}", d2_unique.len());

    // Depth 3 = BinOp(d2_unique, d0) + BinOp(d0, d2_unique) + UnaryOp(d2_unique)
    for op_i in 0..5 {
        let op = Op::from_idx(op_i);
        for d2e in &d2_unique {
            for d0e in &d0 {
                d3_count += 2;
                let e1 = Expr::BinOp(op, Box::new(d2e.clone()), Box::new(d0e.clone()));
                let (s1, m1) = evaluate_v2(&e1, &tgts);
                if s1 > 1.0 && !m1.is_empty() { d3_hits.push((s1, e1, m1)); }

                let e2 = Expr::BinOp(op, Box::new(d0e.clone()), Box::new(d2e.clone()));
                let (s2, m2) = evaluate_v2(&e2, &tgts);
                if s2 > 1.0 && !m2.is_empty() { d3_hits.push((s2, e2, m2)); }
            }
        }
    }
    for uop_i in 0..NUM_UNOPS {
        let uop = UnOp::from_idx(uop_i);
        for d2e in &d2_unique {
            d3_count += 1;
            let e = Expr::UnaryOp(uop, Box::new(d2e.clone()));
            let (s, m) = evaluate_v2(&e, &tgts);
            if s > 1.0 && !m.is_empty() { d3_hits.push((s, e, m)); }
        }
    }
    println!("  Depth 3: {} expressions, {} raw hits", d3_count, d3_hits.len());

    for (sc, expr, matches) in d3_hits {
        if let Some(v) = expr.eval() {
            let key = val_key(v);
            let tid = if matches.is_empty() { usize::MAX } else { matches[0].target_idx };
            if seen.insert((key, tid)) {
                unique.push((sc, expr, matches));
            }
        }
    }

    // Re-sort all
    unique.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());

    // === Collect all evaluated values for FALSIFY filter ===
    let all_values: Vec<f64> = unique.iter()
        .filter_map(|(_, e, _)| e.eval())
        .collect();

    // === Print Results ======================================================
    println!("\n{}", "=".repeat(120));
    println!("  TOP FORMULA DISCOVERIES (n=6 -> SEDI targets)  [v2]");
    println!("{}\n", "=".repeat(120));

    let display_count = unique.len().min(100);
    println!("{:<4} {:<42} {:>14} {:<28} {:>10} {:>6} {:>4} {:>8}",
        "#", "Formula", "Value", "Target", "Rel Error", "Bits", "Tier", "FALSIFY");
    println!("{}", "-".repeat(120));

    for (rank, (_score, expr, matches)) in unique.iter().take(display_count).enumerate() {
        let val = expr.eval().unwrap_or(f64::NAN);
        let nc = expr.node_count();
        for m in matches {
            let t = &tgts[m.target_idx];
            let bits = bayesian_bits(m.rel_error, nc, tgts.len());
            let verdict = falsify_verdict(m.rel_error, val, t.value, nc, &all_values);
            let tier = tier_label(bits, m.rel_error, verdict);

            let err_str = if m.rel_error < 1e-10 {
                "EXACT".to_string()
            } else if m.rel_error < 0.001 {
                format!("{:.6}%", m.rel_error * 100.0)
            } else {
                format!("{:.3}%", m.rel_error * 100.0)
            };

            let verdict_sym = match verdict {
                "STRONG" => "STRONG",
                "WEAK"   => "WEAK",
                _        => "REJECT",
            };

            println!("{:<4} {:<42} {:>14.6} {:<28} {:>10} {:>6.1} {:>4} {:>8}",
                rank + 1, format!("{}", expr), val,
                format!("{} [{}]", t.name, t.category), err_str,
                bits, tier, verdict_sym);
        }
    }

    // === Summary by category ================================================
    println!("\n{}", "=".repeat(120));
    println!("  SUMMARY BY CATEGORY (best discoveries)");
    println!("{}\n", "=".repeat(120));

    let categories = ["particle", "cosmology", "seti", "exoplanet", "drake"];
    for cat in &categories {
        let cat_matches: Vec<_> = unique.iter()
            .filter(|(_, _, ms)| ms.iter().any(|m| tgts[m.target_idx].category == *cat))
            .collect();
        if cat_matches.is_empty() { continue; }

        // Count by verdict
        let mut strong = 0;
        let mut weak = 0;
        let mut reject = 0;
        for (_, expr, ms) in &cat_matches {
            let val = expr.eval().unwrap_or(f64::NAN);
            let nc = expr.node_count();
            for m in ms.iter().filter(|m| tgts[m.target_idx].category == *cat) {
                match falsify_verdict(m.rel_error, val, tgts[m.target_idx].value, nc, &all_values) {
                    "STRONG" => strong += 1,
                    "WEAK" => weak += 1,
                    _ => reject += 1,
                }
            }
        }

        println!("  {} ({} formulas: {} STRONG, {} WEAK, {} REJECT):",
            cat.to_uppercase(), cat_matches.len(), strong, weak, reject);

        for (_, expr, ms) in cat_matches.iter().take(5) {
            let val = expr.eval().unwrap_or(f64::NAN);
            let nc = expr.node_count();
            for m in ms.iter().filter(|m| tgts[m.target_idx].category == *cat) {
                let t = &tgts[m.target_idx];
                let err_pct = m.rel_error * 100.0;
                let verdict = falsify_verdict(m.rel_error, val, t.value, nc, &all_values);
                let bits = bayesian_bits(m.rel_error, nc, tgts.len());
                println!("    {} = {:.6}  ~  {} ({:.4}%) [{} {:.0}bits]",
                    expr, val, t.name, err_pct, verdict, bits);
            }
        }
        println!();
    }

    // === Exact integer matches ==============================================
    println!("{}", "=".repeat(120));
    println!("  EXACT INTEGER MATCHES (n=6 formulas = integer targets)");
    println!("{}\n", "=".repeat(120));

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

    // === Final stats ========================================================
    let elapsed = start_time.elapsed();

    // Count verdicts globally
    let mut g_strong = 0_usize;
    let mut g_weak = 0_usize;
    let mut g_reject = 0_usize;
    for (_, expr, ms) in &unique {
        let val = expr.eval().unwrap_or(f64::NAN);
        let nc = expr.node_count();
        for m in ms {
            match falsify_verdict(m.rel_error, val, tgts[m.target_idx].value, nc, &all_values) {
                "STRONG" => g_strong += 1,
                "WEAK" => g_weak += 1,
                _ => g_reject += 1,
            }
        }
    }

    println!("\n{}", "=".repeat(120));
    println!("  SEDI Formula Miner v3 — Final Report");
    println!("{}", "=".repeat(120));
    println!("  Total unique formulas discovered: {}", unique.len());
    println!("  Exact matches: {}", int_matches.len());
    println!("  FALSIFY verdicts: {} STRONG, {} WEAK, {} REJECT", g_strong, g_weak, g_reject);
    println!("  GA runs: {} x (pop={}, gen={})", num_runs, pop_size, generations);
    println!("  Exhaustive: depth 0-3 scan + physics templates");
    println!("  Total runtime: {:.2}s", elapsed.as_secs_f64());
    println!("{}", "=".repeat(120));

    // === JSON output (v3) ===
    if json_output {
        let mut json_entries: Vec<String> = Vec::new();
        for (_score, expr, matches) in unique.iter().take(display_count) {
            let val = expr.eval().unwrap_or(f64::NAN);
            let nc = expr.node_count();
            for m in matches {
                let t = &tgts[m.target_idx];
                let bits = bayesian_bits(m.rel_error, nc, tgts.len());
                let verdict = falsify_verdict(m.rel_error, val, t.value, nc, &all_values);
                let tier = tier_label(bits, m.rel_error, verdict);
                let formula_str = format!("{}", expr).replace('"', "\\\"");
                let target_name = t.name.replace('"', "\\\"");
                json_entries.push(format!(
                    concat!(
                        "{{",
                        "\"formula\":\"{}\",",
                        "\"value\":{},",
                        "\"target\":\"{}\",",
                        "\"category\":\"{}\",",
                        "\"rel_error\":{},",
                        "\"bits\":{:.1},",
                        "\"tier\":\"{}\",",
                        "\"verdict\":\"{}\"",
                        "}}"
                    ),
                    formula_str, val, target_name, t.category,
                    m.rel_error, bits, tier, verdict
                ));
            }
        }
        eprintln!("\n--- JSON OUTPUT ---");
        println!("[{}]", json_entries.join(",\n"));
    }
}
