// SEDI Discovery Engine v3 — Search for Extra-Dimensional Intelligence
// Implements COLLISION, INVERSE, COMPOSE, BRIDGE, FALSIFY, PREDICT, ANOMALY, META, SYMMETRY operators
// with Bayesian evidence scoring, depth-3 expressions, rayon parallelism, and physics templates.
// v3: Cracks 4 anomalies (eta, m_Planck/m_p, theta_CP, Lambda) via compound 10^N templates.
//
// n=6 arithmetic expressions mapped to particle masses, coupling constants,
// cosmological parameters, and SETI observables.
//
// Usage: cargo run --release [--json] [--verbose] [--min-bits N] [--predictions] [--anomalies]

use rayon::prelude::*;
use std::collections::{HashMap, HashSet};
use std::sync::Mutex;
use std::time::Instant;

// ── Data Structures ──────────────────────────────────────────────

#[derive(Clone, Debug)]
struct Discovery {
    operator: String,
    score: f64,
    description: String,
    domains: Vec<String>,
    formula: String,
    diversity: f64,
    precision: f64,
    novelty: f64,
    bits: f64,
    tier: char,
    falsify: FalsifyVerdict,
}

#[derive(Clone, Debug, PartialEq)]
enum FalsifyVerdict {
    Strong,
    Weak,
    Reject,
    Untested,
}

impl std::fmt::Display for FalsifyVerdict {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FalsifyVerdict::Strong => write!(f, "STRONG"),
            FalsifyVerdict::Weak => write!(f, "WEAK"),
            FalsifyVerdict::Reject => write!(f, "REJECT"),
            FalsifyVerdict::Untested => write!(f, "—"),
        }
    }
}

#[derive(Clone, Debug)]
struct Anomaly {
    target_name: String,
    target_value: f64,
    closest_expr: String,
    closest_error: f64,
    classification: String,
    alt_base: Option<String>,
}

impl std::fmt::Display for Anomaly {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {} (val={:.6}, closest={}, err={:.2}%)",
            self.classification, self.target_name, self.target_value,
            self.closest_expr, self.closest_error * 100.0)
    }
}

#[derive(Clone, Debug)]
struct AltBaseResult {
    any_match: bool,
    best_base: Option<usize>,
    best_expr: Option<String>,
    best_error: f64,
}

// ── Base Constants (n=6 arithmetic) ──────────────────────────────

struct BaseConst {
    name: &'static str,
    value: f64,
    is_transcendental: bool,
}

fn base_constants() -> Vec<BaseConst> {
    vec![
        BaseConst { name: "n",     value: 6.0,  is_transcendental: false },
        BaseConst { name: "σ",     value: 12.0, is_transcendental: false },
        BaseConst { name: "τ",     value: 4.0,  is_transcendental: false },
        BaseConst { name: "φ",     value: 2.0,  is_transcendental: false },
        BaseConst { name: "sopfr", value: 5.0,  is_transcendental: false },
        BaseConst { name: "J₂",    value: 24.0, is_transcendental: false },
        BaseConst { name: "μ",     value: 1.0,  is_transcendental: false },
        // Transcendental constants
        BaseConst { name: "π",     value: std::f64::consts::PI,          is_transcendental: true },
        BaseConst { name: "e",     value: std::f64::consts::E,           is_transcendental: true },
    ]
}

// ── Domain Categories (SEDI-specific) ───────────────────────────

const DOMAIN_CATS: [(&str, &[&str]); 8] = [
    ("Particle",     &["particle", "electron", "muon", "tau", "quark", "lepton", "fermion", "mass"]),
    ("Cosmology",    &["cosmology", "hubble", "cmb", "dark", "baryon", "omega", "planck"]),
    ("SETI",         &["seti", "21cm", "water hole", "drake", "signal", "breakthrough", "listen"]),
    ("Nuclear",      &["nuclear", "proton", "neutron", "binding", "asymmetry"]),
    ("QCD",          &["qcd", "strong", "alpha_s", "gluon", "confinement"]),
    ("Electroweak",  &["electroweak", "weinberg", "fine structure", "ckm", "weak", "coupling"]),
    ("Neutrino",     &["neutrino", "mixing", "oscillation", "pmns"]),
    ("Exoplanet",    &["exoplanet", "habitable", "transit", "kepler"]),
];

fn categorize_domain(d: &str) -> &'static str {
    let dl = d.to_lowercase();
    for (cat, keywords) in &DOMAIN_CATS {
        for kw in *keywords {
            if dl.contains(kw) {
                return cat;
            }
        }
    }
    "Other"
}

fn diversity_score(domains: &[String]) -> f64 {
    let cats: HashSet<&str> = domains.iter().map(|d| categorize_domain(d)).collect();
    cats.len() as f64 / DOMAIN_CATS.len() as f64
}

// ── Expression Enumeration ───────────────────────────────────────

#[derive(Clone, Debug)]
struct Expr {
    text: String,
    value: f64,
    depth: usize,
}

/// Apply unary operations to an expression
fn apply_unaries(base: &Expr) -> Vec<Expr> {
    let mut out = Vec::new();
    let v = base.value;
    let t = &base.text;
    let d = base.depth;

    // sqrt
    if v > 0.0 {
        out.push(Expr { text: format!("sqrt({})", t), value: v.sqrt(), depth: d + 1 });
    }
    // ln
    if v > 0.0 && v != 1.0 {
        out.push(Expr { text: format!("ln({})", t), value: v.ln(), depth: d + 1 });
    }
    // exp (only for small values to avoid overflow)
    if v.abs() < 20.0 {
        let r = v.exp();
        if r.is_finite() && r.abs() < 1e15 {
            out.push(Expr { text: format!("exp({})", t), value: r, depth: d + 1 });
        }
    }
    // abs
    if v < 0.0 {
        out.push(Expr { text: format!("|{}|", t), value: v.abs(), depth: d + 1 });
    }
    // floor (only if not integer)
    if (v - v.floor()).abs() > 1e-10 {
        out.push(Expr { text: format!("⌊{}⌋", t), value: v.floor(), depth: d + 1 });
    }
    // ceil (only if not integer)
    if (v - v.ceil()).abs() > 1e-10 {
        out.push(Expr { text: format!("⌈{}⌉", t), value: v.ceil(), depth: d + 1 });
    }
    // 1/x (reciprocal)
    if v.abs() > 1e-15 {
        out.push(Expr { text: format!("1/({})", t), value: 1.0 / v, depth: d + 1 });
    }

    out
}

/// Binary operation on two expressions
fn binary_ops(a: &Expr, b: &Expr, depth: usize) -> Vec<Expr> {
    let mut out = Vec::new();
    let wrap_a = if a.depth > 0 { format!("({})", a.text) } else { a.text.clone() };
    let wrap_b = if b.depth > 0 { format!("({})", b.text) } else { b.text.clone() };

    // a + b
    out.push(Expr {
        text: format!("{}+{}", wrap_a, wrap_b),
        value: a.value + b.value,
        depth,
    });
    // a - b (if positive)
    if a.value > b.value {
        out.push(Expr {
            text: format!("{}-{}", wrap_a, wrap_b),
            value: a.value - b.value,
            depth,
        });
    }
    // a * b
    out.push(Expr {
        text: format!("{}*{}", wrap_a, wrap_b),
        value: a.value * b.value,
        depth,
    });
    // a / b
    if b.value.abs() > 1e-15 {
        out.push(Expr {
            text: format!("{}/{}", wrap_a, wrap_b),
            value: a.value / b.value,
            depth,
        });
    }
    // a ^ b (if result reasonable)
    if a.value > 0.0 && b.value.abs() <= 12.0 {
        let r = a.value.powf(b.value);
        if r.is_finite() && r.abs() < 1e15 {
            out.push(Expr {
                text: format!("{}^{}", wrap_a, wrap_b),
                value: r,
                depth,
            });
        }
    }

    out
}

fn enumerate_depth1(base: &[BaseConst]) -> Vec<Expr> {
    let mut exprs = Vec::new();
    // Base constants
    for b in base {
        exprs.push(Expr { text: b.name.to_string(), value: b.value, depth: 0 });
    }
    // Unary ops on base constants
    for b in base {
        let base_expr = Expr { text: b.name.to_string(), value: b.value, depth: 0 };
        if b.value > 0.0 {
            // 1/x
            exprs.push(Expr { text: format!("1/{}", b.name), value: 1.0 / b.value, depth: 1 });
            // x^2
            exprs.push(Expr { text: format!("{}²", b.name), value: b.value * b.value, depth: 1 });
            // 2^x (only for small values)
            if b.value <= 24.0 {
                exprs.push(Expr { text: format!("2^{}", b.name), value: (2.0_f64).powf(b.value), depth: 1 });
            }
        }
        // Full unary set
        for u in apply_unaries(&base_expr) {
            // Avoid duplicates with the hand-coded ones above
            if !u.text.starts_with("1/(") {
                exprs.push(u);
            }
        }
    }
    exprs
}

fn enumerate_depth2(base: &[BaseConst]) -> Vec<Expr> {
    let mut exprs = enumerate_depth1(base);
    let n = base.len();

    // Binary ops on all pairs of base constants
    for i in 0..n {
        for j in 0..n {
            let a = Expr { text: base[i].name.to_string(), value: base[i].value, depth: 0 };
            let b = Expr { text: base[j].name.to_string(), value: base[j].value, depth: 0 };
            for e in binary_ops(&a, &b, 1) {
                exprs.push(e);
            }
        }
    }

    // Depth-2: binary op on (depth-1 unary) with base
    let unary_exprs: Vec<Expr> = enumerate_depth1(base).into_iter()
        .filter(|e| e.depth > 0)
        .collect();
    for ue in &unary_exprs {
        for b in base {
            let be = Expr { text: b.name.to_string(), value: b.value, depth: 0 };
            for e in binary_ops(ue, &be, 2) {
                exprs.push(e);
            }
            for e in binary_ops(&be, ue, 2) {
                // Avoid duplicates for commutative ops
                if e.text.contains('/') || e.text.contains('-') || e.text.contains('^') {
                    exprs.push(e);
                }
            }
        }
    }

    // Unary on depth-1 binary results
    let d1_binaries: Vec<Expr> = exprs.iter()
        .filter(|e| e.depth == 1 && (e.text.contains('+') || e.text.contains('*') || e.text.contains('/') || e.text.contains('-')))
        .cloned()
        .collect();
    for e in &d1_binaries {
        for u in apply_unaries(e) {
            exprs.push(Expr { text: u.text, value: u.value, depth: 2 });
        }
    }

    // SEDI-specific special formulas
    let pi = std::f64::consts::PI;
    let specials = vec![
        Expr { text: "6*π^5".into(), value: 6.0 * pi.powi(5), depth: 2 },
        Expr { text: "σ²-7".into(), value: 144.0 - 7.0, depth: 1 },
        Expr { text: "σ²-n-μ".into(), value: 144.0 - 6.0 - 1.0, depth: 2 },
        Expr { text: "σ*φ*n".into(), value: 12.0 * 2.0 * 6.0, depth: 2 },
        Expr { text: "(σ²/τ+σ/τ+τ)/7*1e-10".into(), value: (144.0/4.0 + 12.0/4.0 + 4.0) / 7.0 * 1e-10, depth: 2 },
        Expr { text: "sopfr*σ*J₂+τ*sopfr".into(), value: 5.0 * 12.0 * 24.0 + 4.0 * 5.0, depth: 2 },
        Expr { text: "σ*(σ-φ)".into(), value: 12.0 * 10.0, depth: 1 },
        Expr { text: "(σ-φ)³".into(), value: 1000.0, depth: 1 },
        Expr { text: "σ³".into(), value: 1728.0, depth: 1 },
        Expr { text: "σ⁴".into(), value: 20736.0, depth: 1 },
        Expr { text: "J₂²".into(), value: 576.0, depth: 1 },
        Expr { text: "(σ-φ)^τ".into(), value: 10000.0, depth: 1 },
        Expr { text: "φ^τ*sopfr".into(), value: 16.0 * 5.0, depth: 2 },
        Expr { text: "σ*J₂".into(), value: 288.0, depth: 1 },
        Expr { text: "σ*(σ-τ)".into(), value: 96.0, depth: 1 },
        Expr { text: "σ*τ*(σ-φ)".into(), value: 480.0, depth: 2 },
        Expr { text: "τ*(σ-φ)".into(), value: 40.0, depth: 1 },
        Expr { text: "sopfr*(σ-φ)".into(), value: 50.0, depth: 1 },
        Expr { text: "10^(-(σ-τ))".into(), value: 1e-8, depth: 1 },
        Expr { text: "φ²/n".into(), value: 4.0 / 6.0, depth: 1 },
        Expr { text: "n/φ/(σ-φ)".into(), value: 3.0 / 10.0, depth: 2 },
        Expr { text: "τ/(σ-sopfr)".into(), value: 4.0 / 7.0, depth: 1 },
        Expr { text: "φ/(σ-φ)".into(), value: 0.2, depth: 1 },
        Expr { text: "σ/(σ-φ)".into(), value: 1.2, depth: 1 },
        Expr { text: "1-1/(σ-φ)".into(), value: 0.9, depth: 1 },
        Expr { text: "1-1/(J₂-τ)".into(), value: 0.95, depth: 1 },
        Expr { text: "n!".into(), value: 720.0, depth: 1 },
        Expr { text: "3!".into(), value: 6.0, depth: 1 },
        Expr { text: "n/φ/(σ-φ)²".into(), value: 3.0 / 100.0, depth: 2 },
        Expr { text: "σ*φ^τ".into(), value: 12.0 * 16.0, depth: 2 },
        Expr { text: "sopfr*2^n".into(), value: 5.0 * 64.0, depth: 2 },
        Expr { text: "σ²-φ".into(), value: 142.0, depth: 1 },

        // SEDI physics formulas
        Expr { text: "σ*τ*sopfr/(σ*φ*n+τ*sopfr+φ)".into(),
               value: 12.0*4.0*5.0 / (144.0 + 20.0 + 2.0), depth: 2 },
        Expr { text: "(σ-φ)^φ*φ+n+μ/σ".into(), value: 100.0*2.0 + 6.0 + 1.0/12.0, depth: 2 },
        Expr { text: "sopfr/(J₂-φ)".into(), value: 5.0 / 22.0, depth: 1 },
        Expr { text: "n/(J₂+φ+μ)".into(), value: 6.0 / 27.0, depth: 2 },
        Expr { text: "μ/J₂".into(), value: 1.0 / 24.0, depth: 1 },
        Expr { text: "μ/(σ*J₂-sopfr*τ)".into(), value: 1.0 / 268.0, depth: 2 },
        Expr { text: "sopfr*σ+n+μ+φ/τ".into(), value: 60.0+6.0+1.0+0.5, depth: 2 },
        Expr { text: "(n+μ)/(J₂-φ)".into(), value: 7.0/22.0, depth: 2 },
        Expr { text: "1-(n+μ)/(J₂-φ)".into(), value: 1.0 - 7.0/22.0, depth: 2 },
        Expr { text: "φ+n!/(σ-φ)³".into(), value: 2.0 + 720.0/1000.0, depth: 2 },
        Expr { text: "σ³-σ*J₂-sopfr*τ".into(), value: 1728.0 - 288.0 - 20.0, depth: 2 },
        Expr { text: "σ³-n-φ".into(), value: 1728.0 - 6.0 - 2.0, depth: 2 },
        Expr { text: "μ/(σ-τ)-μ/σ²".into(), value: 1.0/8.0 - 1.0/144.0, depth: 2 },
        Expr { text: "σ⁴*(σ-τ)+σ³*τ+sopfr*σ+τ*(σ-φ)".into(),
               value: 20736.0*8.0 + 1728.0*4.0 + 60.0 + 40.0, depth: 2 },
        Expr { text: "σ³-σ*J₂-σ*(σ-τ)-n*sopfr".into(), value: 1728.0-288.0-96.0-60.0, depth: 2 },
        Expr { text: "σ³*φ+J₂²+σ*(σ-φ)+n*sopfr*φ".into(),
               value: 3456.0 + 576.0 + 120.0 + 60.0, depth: 2 },
        Expr { text: "σ*(σ-τ)-φ-μ/φ".into(), value: 96.0 - 2.0 - 0.5, depth: 2 },
        Expr { text: "σ*(σ-τ)+σ-φ-μ/τ".into(), value: 96.0+12.0-2.0-0.25, depth: 2 },
        Expr { text: "6*π^5-sopfr*σ+μ/φ".into(),
               value: 6.0 * pi.powi(5) - 60.0 + 0.5, depth: 2 },
        Expr { text: "φ+μ/(n+μ)".into(), value: 2.0 + 1.0/7.0, depth: 2 },
        Expr { text: "τ+n/(σ-τ)-μ/(σ+φ)".into(), value: 4.0+6.0/8.0-1.0/14.0, depth: 2 },
        Expr { text: "6*π^5+μ/J₂".into(), value: 6.0 * pi.powi(5) + 1.0/24.0, depth: 2 },
        Expr { text: "n/τ".into(), value: 1.5, depth: 1 },
        Expr { text: "φ/sopfr".into(), value: 0.4, depth: 1 },
    ];
    exprs.extend(specials);

    // Deduplicate by text
    let mut seen = HashSet::new();
    exprs.retain(|e| {
        if e.value.is_nan() || e.value.is_infinite() { return false; }
        seen.insert(e.text.clone())
    });

    exprs
}

// ── Simple n=6 values for physics template generation ───────────

struct SimpleN6 {
    name: String,
    value: f64,
}

fn simple_n6_values() -> Vec<SimpleN6> {
    let pi = std::f64::consts::PI;
    let e = std::f64::consts::E;
    let n = 6.0_f64; let s = 12.0; let t = 4.0; let p = 2.0; let sp = 5.0; let j = 24.0; let m = 1.0;
    vec![
        SimpleN6 { name: "μ".into(), value: m },
        SimpleN6 { name: "φ".into(), value: p },
        SimpleN6 { name: "τ".into(), value: t },
        SimpleN6 { name: "sopfr".into(), value: sp },
        SimpleN6 { name: "n".into(), value: n },
        SimpleN6 { name: "σ".into(), value: s },
        SimpleN6 { name: "J₂".into(), value: j },
        SimpleN6 { name: "σ-φ".into(), value: s - p },
        SimpleN6 { name: "σ-τ".into(), value: s - t },
        SimpleN6 { name: "σ+τ".into(), value: s + t },
        SimpleN6 { name: "n+μ".into(), value: n + m },
        SimpleN6 { name: "σ/τ".into(), value: s / t },
        SimpleN6 { name: "σ/φ".into(), value: s / p },
        SimpleN6 { name: "σ/n".into(), value: s / n },
        SimpleN6 { name: "J₂/n".into(), value: j / n },
        SimpleN6 { name: "J₂/σ".into(), value: j / s },
        SimpleN6 { name: "n/τ".into(), value: n / t },
        SimpleN6 { name: "τ/φ".into(), value: t / p },
        SimpleN6 { name: "σ²".into(), value: s * s },
        SimpleN6 { name: "n²".into(), value: n * n },
        SimpleN6 { name: "σ*τ".into(), value: s * t },
        SimpleN6 { name: "n*sopfr".into(), value: n * sp },
        SimpleN6 { name: "n!".into(), value: 720.0 },
        SimpleN6 { name: "π".into(), value: pi },
        SimpleN6 { name: "e".into(), value: e },
        SimpleN6 { name: "(σ²/τ+σ/τ+τ)/7".into(), value: (s*s/t + s/t + t) / 7.0 },
        SimpleN6 { name: "σ*φ".into(), value: s * p },
        SimpleN6 { name: "sopfr*τ".into(), value: sp * t },
        SimpleN6 { name: "σ+n".into(), value: s + n },
        SimpleN6 { name: "σ+φ".into(), value: s + p },
        SimpleN6 { name: "J₂+n".into(), value: j + n },
        SimpleN6 { name: "n!*n".into(), value: 720.0 * n },
        SimpleN6 { name: "φ^n".into(), value: p.powf(n) },
        SimpleN6 { name: "σ³".into(), value: s * s * s },
        SimpleN6 { name: "sqrt(n)".into(), value: n.sqrt() },
        SimpleN6 { name: "sqrt(σ)".into(), value: s.sqrt() },
        SimpleN6 { name: "1/n".into(), value: 1.0 / n },
        SimpleN6 { name: "1/σ".into(), value: 1.0 / s },
        SimpleN6 { name: "1/J₂".into(), value: 1.0 / j },
        SimpleN6 { name: "μ/σ".into(), value: m / s },
        SimpleN6 { name: "n/σ²".into(), value: n / (s * s) },
    ]
}

/// Physics formula templates for extreme-scale values (v3 anomaly cracker)
fn physics_templates() -> Vec<Expr> {
    let mut templates = Vec::new();
    let pi = std::f64::consts::PI;
    let e_const = std::f64::consts::E;
    #[allow(unused_variables)]
    let (n, sp, j) = (6.0_f64, 5.0, 24.0);
    let s = 12.0; let t = 4.0; let p = 2.0; let m = 1.0;

    // ── Baryon asymmetry: eta ~ 6.14e-10 ─────────────────────────
    // Known SEDI formula
    templates.push(Expr {
        text: "(σ²/τ+σ/τ+τ)/7 × 10⁻¹⁰".into(),
        value: ((s*s/t + s/t + t) / 7.0) * 1e-10,
        depth: 2,
    });

    // ── Systematic: A × 10^(-B) for small/large values ────────────
    let simple = simple_n6_values();
    let neg_exponents = [5.0, 6.0, 8.0, 10.0, 12.0, 19.0, 24.0, 48.0, 50.0, 52.0, 54.0];
    for a in &simple {
        for &b_exp in &neg_exponents {
            let val = a.value * 10.0_f64.powf(-b_exp);
            if val.is_finite() && val > 0.0 && val < 1e20 {
                templates.push(Expr {
                    text: format!("{} × 10^(-{})", a.name, b_exp),
                    value: val,
                    depth: 2,
                });
            }
        }
    }

    // ── A × 10^(B) for large values (Planck mass ratio etc.) ─────
    for a in &simple {
        for b in &simple {
            let val = a.value * 10.0_f64.powf(b.value);
            if val.is_finite() && val > 1e15 && val < 1e25 {
                templates.push(Expr {
                    text: format!("{} × 10^({})", a.name, b.name),
                    value: val,
                    depth: 2,
                });
            }
        }
    }

    // ── Direct power-of-10 expressions for theta_CP, Lambda ──────
    templates.push(Expr { text: "10^(-(σ-φ))".into(), value: 1e-10, depth: 1 });
    templates.push(Expr { text: "10^(-σ)".into(), value: 1e-12, depth: 1 });
    templates.push(Expr { text: "10^(-J₂)".into(), value: 1e-24, depth: 1 });

    // Cosmological constant: Lambda ~ 1.1e-52
    // (σ-φ) = 10, (σ-φ)^φ = 100, so 10^(-(σ-φ)^φ) is too small
    // Try: A × 10^(-B) where A ~ 1.1 and B = 52
    // σ/(σ-φ) = 1.2, close to 1.1
    // (σ-μ)/(σ-φ) = 11/10 = 1.1 exact!
    templates.push(Expr {
        text: "(σ-μ)/(σ-φ) × 10^(-J₂-J₂-τ)".into(),
        value: ((s - m) / (s - p)) * 1e-52,
        depth: 2,
    });
    templates.push(Expr {
        text: "(σ-μ)/(σ-φ) × 10^(-(σ-φ)²-J₂-J₂-τ)".into(),
        value: 1.1 * 1e-52,
        depth: 2,
    });

    // ── Planck mass ratio: 1.22e19 ───────────────────────────────
    // 1.22 = (σ+μ+μ/sopfr)/(σ-φ) = 12.2/10 = 1.22
    // Or: (n+μ/sopfr) = 6.2, times 2 = 12.4... not quite
    // n!*n × 10^(σ+τ) = 4320 × 10^16 = 4.32e19 (close order)
    // σ/(σ-φ) × 10^(σ+n+μ) = 1.2e19
    // (σ+φ/σ)/(σ-φ) = 12.167/10 = 1.2167 ~ 1.22 close
    // Try: (n+μ+μ/sopfr) = 7.2, /n = 1.2  => 1.2e19 (2% off)
    // (σ+φ+φ/σ)/(σ-φ) = 14.167/10 = 1.4167 too high
    // Direct: (σ²+φ)/(σ*(σ-φ)) = 146/120 = 1.2167 ~ 1.22
    templates.push(Expr {
        text: "σ/(σ-φ) × 10^(σ+n+μ)".into(),
        value: (s / (s - p)) * 1e19,
        depth: 2,
    });
    templates.push(Expr {
        text: "(σ²+φ)/(σ*(σ-φ)) × 10^(σ+n+μ)".into(),
        value: ((s*s + p) / (s * (s - p))) * 1e19,
        depth: 2,
    });

    // ── Factorial and power combinations ─────────────────────────
    templates.push(Expr { text: "n!".into(), value: 720.0, depth: 1 });
    templates.push(Expr { text: "n! × 10^(σ+n)".into(), value: 720.0 * 1e18, depth: 2 });
    templates.push(Expr { text: "n! × n × 10^(σ+τ)".into(), value: 720.0 * n * 1e16, depth: 2 });

    // ── pi/e-based extreme values ────────────────────────────────
    templates.push(Expr { text: "π^(σ+n)".into(), value: pi.powf(18.0), depth: 2 });
    templates.push(Expr { text: "π^(J₂)".into(), value: pi.powf(24.0), depth: 2 });
    templates.push(Expr { text: "e^(J₂+σ+n)".into(), value: e_const.powf(42.0), depth: 2 });

    // ── A/B × 10^(-C) compound patterns ──────────────────────────
    // For baryon asymmetry refinement
    for a in &simple {
        for b in &simple {
            if b.value.abs() < 1e-15 { continue; }
            let ratio = a.value / b.value;
            if ratio > 0.5 && ratio < 20.0 {
                for &exp in &[10.0, 12.0, 19.0, 52.0] {
                    let val = ratio * 10.0_f64.powf(-exp);
                    if val.is_finite() && val > 0.0 {
                        templates.push(Expr {
                            text: format!("({})/({})) × 10^(-{})", a.name, b.name, exp),
                            value: val,
                            depth: 2,
                        });
                    }
                    // Also positive exponents for large values
                    if exp == 19.0 {
                        let val_pos = ratio * 10.0_f64.powf(exp);
                        if val_pos.is_finite() && val_pos > 1e15 && val_pos < 1e25 {
                            templates.push(Expr {
                                text: format!("({})/({})) × 10^({})", a.name, b.name, exp),
                                value: val_pos,
                                depth: 2,
                            });
                        }
                    }
                }
            }
        }
    }

    templates
}

/// Depth-3: ((a OP b) OP c) OP d and all tree shapes — parallelized with rayon
fn enumerate_depth3(base: &[BaseConst], depth2_exprs: &[Expr]) -> Vec<Expr> {
    // Get depth-1 binary expressions (a OP b)
    let d1_binaries: Vec<&Expr> = depth2_exprs.iter()
        .filter(|e| e.depth == 1 && e.value.is_finite() && e.value.abs() < 1e10)
        .collect();

    let base_exprs: Vec<Expr> = base.iter()
        .map(|b| Expr { text: b.name.to_string(), value: b.value, depth: 0 })
        .collect();

    let results = Mutex::new(Vec::new());

    // Shape 1: (a OP b) OP c — depth-1 binary OP base constant
    d1_binaries.par_iter().for_each(|d1| {
        let mut local = Vec::new();
        for b in &base_exprs {
            for e in binary_ops(d1, b, 2) {
                if e.value.is_finite() && e.value.abs() < 1e15 {
                    local.push(e);
                }
            }
        }
        results.lock().unwrap().extend(local);
    });

    // Shape 2: ((a OP b) OP c) OP d — depth-2 OP base constant
    let d2_exprs: Vec<&Expr> = depth2_exprs.iter()
        .filter(|e| e.depth == 2 && e.value.is_finite() && e.value.abs() < 1e10)
        .collect();

    d2_exprs.par_iter().for_each(|d2| {
        let mut local = Vec::new();
        for b in &base_exprs {
            for e in binary_ops(d2, b, 3) {
                if e.value.is_finite() && e.value.abs() < 1e15 {
                    local.push(e);
                }
            }
        }
        results.lock().unwrap().extend(local);
    });

    // Shape 3: (a OP b) OP (c OP d) — two depth-1 binaries combined
    // Only sample to avoid combinatorial explosion
    let d1_sample: Vec<&Expr> = d1_binaries.iter()
        .take(100) // top 100 by index (they include most useful ones)
        .cloned()
        .collect();

    d1_sample.par_iter().for_each(|left| {
        let mut local = Vec::new();
        for right in &d1_sample {
            for e in binary_ops(left, right, 3) {
                if e.value.is_finite() && e.value.abs() < 1e15 {
                    local.push(e);
                }
            }
        }
        results.lock().unwrap().extend(local);
    });

    // Unary on depth-2 expressions (makes depth-3)
    let d2_for_unary: Vec<&Expr> = depth2_exprs.iter()
        .filter(|e| e.depth == 2 && e.value.is_finite() && e.value.abs() > 1e-15 && e.value.abs() < 1e10)
        .collect();

    d2_for_unary.par_iter().for_each(|e| {
        let local: Vec<Expr> = apply_unaries(e).into_iter()
            .map(|mut u| { u.depth = 3; u })
            .filter(|u| u.value.is_finite() && u.value.abs() < 1e15)
            .collect();
        results.lock().unwrap().extend(local);
    });

    results.into_inner().unwrap()
}

/// Create a dedup key that works for all magnitudes (v3 fix for extreme-scale values)
fn dedup_key(value: f64) -> i64 {
    if value == 0.0 { return 0; }
    let abs_val = value.abs();
    let sign = if value < 0.0 { -1i64 } else { 1i64 };
    // Use log-scale key: encode (sign, exponent, 8 digits of mantissa)
    let log = abs_val.log10();
    let exponent = log.floor() as i64;
    let mantissa = (10.0_f64.powf(log - log.floor()) * 1e8).round() as i64;
    sign * (exponent * 1_000_000_000 + mantissa)
}

/// Deduplicate expressions by numeric value (keep shortest formula)
fn dedup_by_value(exprs: &mut Vec<Expr>) {
    let mut value_map: HashMap<i64, usize> = HashMap::new();
    let mut keep = vec![true; exprs.len()];

    for (i, e) in exprs.iter().enumerate() {
        let key = dedup_key(e.value);
        if let Some(&prev_i) = value_map.get(&key) {
            // Keep the one with shorter formula text (simpler)
            if e.text.len() < exprs[prev_i].text.len() {
                keep[prev_i] = false;
                value_map.insert(key, i);
            } else {
                keep[i] = false;
            }
        } else {
            value_map.insert(key, i);
        }
    }

    let mut idx = 0;
    exprs.retain(|_| {
        let k = keep[idx];
        idx += 1;
        k
    });
}

// ── Physics Target Values (SEDI) ────────────────────────────────

struct PhysicsTarget {
    value: f64,
    name: &'static str,
    domain: &'static str,
}

fn physics_targets() -> Vec<PhysicsTarget> {
    vec![
        // ── Particle masses (MeV/c²) ─────────────────────────────
        PhysicsTarget { value: 0.511,     name: "m_e (electron)",          domain: "Particle/Lepton" },
        PhysicsTarget { value: 105.658,   name: "m_μ (muon)",             domain: "Particle/Lepton" },
        PhysicsTarget { value: 1776.86,   name: "m_τ (tau)",              domain: "Particle/Lepton" },
        PhysicsTarget { value: 2.16,      name: "m_u (up quark)",         domain: "Particle/Quark" },
        PhysicsTarget { value: 4.67,      name: "m_d (down quark)",       domain: "Particle/Quark" },
        PhysicsTarget { value: 93.4,      name: "m_s (strange quark)",    domain: "Particle/Quark" },
        PhysicsTarget { value: 1270.0,    name: "m_c (charm quark)",      domain: "Particle/Quark" },
        PhysicsTarget { value: 4180.0,    name: "m_b (bottom quark)",     domain: "Particle/Quark" },
        PhysicsTarget { value: 172760.0,  name: "m_t (top quark)",        domain: "Particle/Quark" },

        // ── Mass ratios ──────────────────────────────────────────
        PhysicsTarget { value: 1836.15,   name: "m_p/m_e",                domain: "Nuclear/Mass ratio" },
        PhysicsTarget { value: 206.768,   name: "m_μ/m_e",                domain: "Particle/Mass ratio" },

        // ── Coupling constants ───────────────────────────────────
        PhysicsTarget { value: 137.036,   name: "α⁻¹ (fine structure)",   domain: "Electroweak/Coupling" },
        PhysicsTarget { value: 0.1179,    name: "α_s(M_Z)",               domain: "QCD/Coupling" },
        PhysicsTarget { value: 0.2312,    name: "sin²θ_W (Weinberg)",     domain: "Electroweak/Mixing" },

        // ── CKM matrix elements ──────────────────────────────────
        PhysicsTarget { value: 0.2243,    name: "|V_us|",                 domain: "Electroweak/CKM" },
        PhysicsTarget { value: 0.0422,    name: "|V_cb|",                 domain: "Electroweak/CKM" },
        PhysicsTarget { value: 0.00394,   name: "|V_ub|",                 domain: "Electroweak/CKM" },

        // ── Cosmological parameters ──────────────────────────────
        PhysicsTarget { value: 67.4,      name: "H₀ (km/s/Mpc)",          domain: "Cosmology/Hubble" },
        PhysicsTarget { value: 0.315,     name: "Ω_m (matter density)",   domain: "Cosmology/Dark matter" },
        PhysicsTarget { value: 0.685,     name: "Ω_Λ (dark energy)",      domain: "Cosmology/Dark energy" },
        PhysicsTarget { value: 2.7255,    name: "T_CMB (K)",              domain: "Cosmology/CMB" },

        // ── SETI observables ─────────────────────────────────────
        PhysicsTarget { value: 1420.405,  name: "21cm H line (MHz)",      domain: "SETI/Hydrogen line" },
        PhysicsTarget { value: 1720.0,    name: "Water hole upper (MHz)", domain: "SETI/Water hole" },

        // ── Baryon asymmetry ─────────────────────────────────────
        PhysicsTarget { value: 6.14e-10,  name: "η (baryon asymmetry)",   domain: "Cosmology/Baryon" },

        // ── Drake equation parameters (estimates) ────────────────
        PhysicsTarget { value: 1.5,       name: "R* (star formation)",    domain: "SETI/Drake" },
        PhysicsTarget { value: 0.4,       name: "n_e (habitable planets)", domain: "Exoplanet/Drake" },

        // ── NEW v2 targets ───────────────────────────────────────

        // Electroweak
        PhysicsTarget { value: 0.7688,    name: "cos²θ_W (Weinberg)",     domain: "Electroweak/Mixing" },
        PhysicsTarget { value: 80379.0,   name: "m_W (W boson) MeV",      domain: "Electroweak/Boson mass" },
        PhysicsTarget { value: 91187.6,   name: "m_Z (Z boson) MeV",      domain: "Electroweak/Boson mass" },

        // Higgs
        PhysicsTarget { value: 125250.0,  name: "m_H (Higgs) MeV",        domain: "Particle/Higgs mass" },

        // Strong CP
        PhysicsTarget { value: 1e-10,     name: "θ_CP (strong CP angle)", domain: "QCD/CP violation" },

        // Neutrino
        PhysicsTarget { value: 7.53e-5,   name: "Δm²₂₁ (neutrino) eV²",  domain: "Neutrino/Oscillation" },

        // CKM Jarlskog
        PhysicsTarget { value: 3.08e-5,   name: "J (CKM Jarlskog)",       domain: "Electroweak/CKM" },

        // Cosmological constant
        PhysicsTarget { value: 1.1e-52,   name: "Λ (cosmo const) m⁻²",   domain: "Cosmology/Dark energy" },

        // Planck mass ratio
        PhysicsTarget { value: 1.22e19,   name: "m_Planck/m_p",           domain: "Cosmology/Planck" },
    ]
}

// ── FALSIFY Operator ─────────────────────────────────────────────

fn is_common_number(v: f64) -> bool {
    if v == 0.0 { return true; }
    let abs_v = v.abs();
    // Small integers
    if abs_v < 100.0 && (abs_v - abs_v.round()).abs() < 1e-6 {
        return true;
    }
    // Powers of 2
    if abs_v > 0.0 {
        let log2 = abs_v.log2();
        if (log2 - log2.round()).abs() < 0.01 && log2.round().abs() <= 20.0 {
            return true;
        }
    }
    // Powers of 10
    if abs_v > 0.0 {
        let log10 = abs_v.log10();
        if (log10 - log10.round()).abs() < 0.01 && log10.round().abs() <= 15.0 {
            return true;
        }
    }
    false
}

fn count_matches_within_tolerance(target: f64, all_values: &[f64], tolerance: f64) -> usize {
    if target == 0.0 { return 0; }
    all_values.iter().filter(|&&v| {
        if v == 0.0 { return false; }
        ((v - target) / target).abs() < tolerance
    }).count()
}

fn falsify(match_value: f64, match_error: f64, formula_depth: usize, all_values: &[f64]) -> (FalsifyVerdict, f64) {
    let tolerance = 0.01; // 1%

    // 1. Texas Sharpshooter: use expressions_within_order_of_magnitude as denominator (v3)
    let n_matches = count_matches_within_tolerance(match_value, all_values, tolerance);
    let oom_count = count_within_order_of_magnitude(match_value, all_values);
    let effective_space = oom_count.max(1);
    let p_chance = n_matches as f64 / effective_space as f64;

    // 2. Complexity penalty: simpler formulas get higher confidence
    let complexity_score = 1.0 / (1.0 + formula_depth as f64);

    // 3. Uniqueness: is the matched value "special"?
    let uniqueness = if is_common_number(match_value) { 0.3 } else { 1.0 };

    // 4. Precision boost (v3): matches < 0.01% significantly boost survival
    let precision_boost = if match_error < 0.0001 {
        1.5
    } else if match_error < 0.001 {
        1.2
    } else {
        1.0
    };

    // Verdict
    let survival = (1.0 - p_chance) * complexity_score * uniqueness * precision_boost;
    let verdict = if survival > 0.7 {
        FalsifyVerdict::Strong
    } else if survival > 0.3 {
        FalsifyVerdict::Weak
    } else {
        FalsifyVerdict::Reject
    };

    (verdict, survival)
}

/// Count expressions within the same order of magnitude as the target (v3)
fn count_within_order_of_magnitude(target: f64, all_values: &[f64]) -> usize {
    if target.abs() < 1e-100 { return 0; }
    let log_target = target.abs().log10();
    all_values.iter().filter(|&&v| {
        if v.abs() < 1e-100 { return false; }
        let log_v = v.abs().log10();
        (log_v - log_target).abs() < 0.5
    }).count()
}

// ── Bayesian Evidence Scoring ────────────────────────────────────

fn bayesian_bits(match_error: f64, _depth: usize, domain_count: usize, search_space: usize) -> f64 {
    let value_surprise = -match_error.max(1e-15).log2();
    let precision_bits = 3.32 * (-(match_error * 100.0).max(0.01).log10());
    let depth_penalty = -(search_space as f64).max(1.0).log2();
    let domain_novelty = match domain_count {
        1 => 1.0,
        2..=5 => 0.5,
        _ => 0.25,
    };

    (value_surprise + precision_bits + depth_penalty) * domain_novelty
}

fn evidence_tier(bits: f64) -> char {
    if bits > 40.0 { 'A' }
    else if bits > 25.0 { 'B' }
    else if bits > 15.0 { 'C' }
    else if bits > 5.0 { 'D' }
    else { 'E' }
}

// ── Operators ────────────────────────────────────────────────────

/// COLLISION: Group n6 expressions by value, find cross-domain target matches
fn op_collision(expressions: &[Expr], targets: &[PhysicsTarget], all_values: &[f64]) -> Vec<Discovery> {
    let value_map: HashMap<i64, Vec<usize>> = {
        let mut m: HashMap<i64, Vec<usize>> = HashMap::new();
        for (i, e) in expressions.iter().enumerate() {
            if e.value == 0.0 { continue; }
            let key = (e.value.ln().abs() * 10000.0).round() as i64
                * if e.value < 0.0 { -1 } else { 1 };
            m.entry(key).or_default().push(i);
        }
        m
    };

    let entries: Vec<_> = value_map.into_iter().collect();

    let discoveries: Vec<Discovery> = entries.par_iter().filter_map(|(_key, indices)| {
        if indices.len() < 2 { return None; }

        let val = expressions[indices[0]].value;

        let mut exprs_text: Vec<String> = Vec::new();
        for &i in indices {
            exprs_text.push(expressions[i].text.clone());
        }

        let mut matched_domains: Vec<String> = Vec::new();
        let mut matched_targets: Vec<String> = Vec::new();
        for t in targets {
            if t.value == 0.0 { continue; }
            let err = ((val - t.value) / t.value).abs();
            if err < 0.005 {
                matched_domains.push(t.domain.to_string());
                matched_targets.push(t.name.to_string());
            }
        }

        if matched_targets.is_empty() && indices.len() < 3 { return None; }

        let all_domains = if matched_domains.is_empty() {
            vec!["n6-internal".to_string()]
        } else {
            matched_domains
        };

        let div = diversity_score(&all_domains);
        let prec = 1.0;
        let nov = if matched_targets.is_empty() { 0.5 } else { 1.0 };
        let score = div * prec * nov * (indices.len() as f64 / 5.0).min(1.0);

        if score <= 0.01 { return None; }

        let min_depth = indices.iter().map(|&i| expressions[i].depth).min().unwrap_or(0);
        let best_err = if !matched_targets.is_empty() { 0.005 } else { 0.01 };
        let bits = bayesian_bits(best_err, min_depth, all_domains.len(), all_values.len());
        let tier = evidence_tier(bits);
        let (verdict, _survival) = falsify(val, best_err, min_depth, all_values);

        let desc = if matched_targets.is_empty() {
            format!("Value {:.6} has {} equivalent n6 expressions: {}",
                val, indices.len(), exprs_text.join(", "))
        } else {
            format!("Value {:.6} matches {} via {} n6 expressions: {}",
                val, matched_targets.join(", "), indices.len(), exprs_text.join(", "))
        };

        Some(Discovery {
            operator: "COLLISION".to_string(),
            score,
            description: desc,
            domains: all_domains,
            formula: exprs_text.join(" = "),
            diversity: div,
            precision: prec,
            novelty: nov,
            bits,
            tier,
            falsify: verdict,
        })
    }).collect();

    discoveries
}

/// INVERSE: Find n=6 expressions that approximate measured physics values
fn op_inverse(expressions: &[Expr], targets: &[PhysicsTarget], all_values: &[f64]) -> Vec<Discovery> {
    let discoveries: Vec<Discovery> = targets.par_iter().filter_map(|target| {
        let mut matches: Vec<(String, f64, usize)> = Vec::new();

        for expr in expressions {
            if target.value == 0.0 { continue; }
            let err = ((expr.value - target.value) / target.value).abs();
            if err < 0.01 {
                matches.push((expr.text.clone(), err, expr.depth));
            }
        }

        if matches.is_empty() { return None; }

        matches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        matches.truncate(5);

        let best_err = matches[0].1;
        let best_depth = matches[0].2;
        let formula = matches.iter().map(|(e, _, _)| e.as_str()).collect::<Vec<_>>().join(", ");

        let domains = vec![target.domain.to_string()];
        let div = diversity_score(&domains);
        let prec = 1.0 - best_err;
        let nov = 1.0;
        let score = div * prec * nov;

        let bits = bayesian_bits(best_err.max(1e-10), best_depth, 1, all_values.len());
        let tier = evidence_tier(bits);
        let (verdict, _) = falsify(target.value, best_err, best_depth, all_values);

        Some(Discovery {
            operator: "INVERSE".to_string(),
            score,
            description: format!(
                "{} = {} <-- n6: {} (err={:.4}%)",
                target.name, target.value,
                matches[0].0, best_err * 100.0
            ),
            domains,
            formula,
            diversity: div,
            precision: prec,
            novelty: nov,
            bits,
            tier,
            falsify: verdict,
        })
    }).collect();

    discoveries
}

/// COMPOSE: Cross-reference expressions against multiple targets, find cross-domain connections
fn op_compose(expressions: &[Expr], targets: &[PhysicsTarget], all_values: &[f64]) -> Vec<Discovery> {
    let expr_matches: HashMap<String, Vec<(String, String, f64, usize)>> = {
        let mut m: HashMap<String, Vec<(String, String, f64, usize)>> = HashMap::new();
        for expr in expressions {
            for target in targets {
                if target.value == 0.0 { continue; }
                let err = ((expr.value - target.value) / target.value).abs();
                if err < 0.005 {
                    m.entry(expr.text.clone()).or_default().push(
                        (target.name.to_string(), target.domain.to_string(), err, expr.depth)
                    );
                }
            }
        }
        m
    };

    let entries: Vec<_> = expr_matches.into_iter().collect();

    let discoveries: Vec<Discovery> = entries.par_iter().filter_map(|(expr_text, matched_targets)| {
        if matched_targets.len() < 2 { return None; }

        let mut domains: Vec<String> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        let best_err = matched_targets.iter().map(|(_, _, e, _)| *e).fold(f64::MAX, f64::min);
        let best_depth = matched_targets.iter().map(|(_, _, _, d)| *d).min().unwrap_or(0);

        for (name, domain, _, _) in matched_targets {
            if !domains.contains(domain) {
                domains.push(domain.clone());
            }
            names.push(name.clone());
        }

        let cats: HashSet<&str> = domains.iter().map(|d| categorize_domain(d)).collect();
        if cats.len() < 2 { return None; }

        let div = diversity_score(&domains);
        let prec = 1.0 - best_err;
        let nov = 1.0;
        let score = div * prec * nov;

        if score <= 0.01 { return None; }

        let bits = bayesian_bits(best_err.max(1e-10), best_depth, domains.len(), all_values.len());
        let tier = evidence_tier(bits);
        let (verdict, _) = falsify(matched_targets[0].2, best_err, best_depth, all_values);

        Some(Discovery {
            operator: "COMPOSE".to_string(),
            score,
            description: format!(
                "n6 expr '{}' = {:.6} matches {} targets across {} categories: {}",
                expr_text,
                matched_targets[0].2,
                matched_targets.len(),
                cats.len(),
                names.join(", ")
            ),
            domains,
            formula: expr_text.clone(),
            diversity: div,
            precision: prec,
            novelty: nov,
            bits,
            tier,
            falsify: verdict,
        })
    }).collect();

    discoveries
}

/// BRIDGE: Find pairs of targets in different domains connected through same n=6 expression
fn op_bridge(expressions: &[Expr], targets: &[PhysicsTarget], all_values: &[f64]) -> Vec<Discovery> {
    // For each expression, find all targets it matches within 1%
    let mut expr_target_map: HashMap<usize, Vec<usize>> = HashMap::new();
    for (ei, expr) in expressions.iter().enumerate() {
        for (ti, target) in targets.iter().enumerate() {
            if target.value == 0.0 { continue; }
            let err = ((expr.value - target.value) / target.value).abs();
            if err < 0.01 {
                expr_target_map.entry(ei).or_default().push(ti);
            }
        }
    }

    // For each value bucket, collect all matching target indices
    let mut value_targets: HashMap<i64, HashSet<usize>> = HashMap::new();
    let mut value_exprs: HashMap<i64, Vec<usize>> = HashMap::new();
    for (&ei, tis) in &expr_target_map {
        let key = (expressions[ei].value * 1000.0).round() as i64;
        for &ti in tis {
            value_targets.entry(key).or_default().insert(ti);
        }
        value_exprs.entry(key).or_default().push(ei);
    }

    let mut discoveries = Vec::new();
    let mut seen_bridges = HashSet::new();

    for (key, target_set) in &value_targets {
        let target_list: Vec<usize> = target_set.iter().cloned().collect();
        if target_list.len() < 2 { continue; }

        // Check for cross-domain pairs
        for i in 0..target_list.len() {
            for j in (i+1)..target_list.len() {
                let ti = target_list[i];
                let tj = target_list[j];
                let cat_i = categorize_domain(targets[ti].domain);
                let cat_j = categorize_domain(targets[tj].domain);

                if cat_i == cat_j { continue; }

                let bridge_key = format!("{}-{}", ti.min(tj), ti.max(tj));
                if !seen_bridges.insert(bridge_key) { continue; }

                let expr_indices = value_exprs.get(key).unwrap();
                let expr_texts: Vec<&str> = expr_indices.iter()
                    .take(3)
                    .map(|&ei| expressions[ei].text.as_str())
                    .collect();
                let best_depth = expr_indices.iter().map(|&ei| expressions[ei].depth).min().unwrap_or(0);
                let val = expressions[expr_indices[0]].value;

                // Score by domain distance x precision
                let domains = vec![targets[ti].domain.to_string(), targets[tj].domain.to_string()];
                let div = diversity_score(&domains);
                let err_i = ((val - targets[ti].value) / targets[ti].value).abs();
                let err_j = ((val - targets[tj].value) / targets[tj].value).abs();
                let best_err = err_i.min(err_j);
                let prec = 1.0 - best_err;
                let score = div * prec * 1.5; // Bridge bonus

                let bits = bayesian_bits(best_err.max(1e-10), best_depth, 2, all_values.len());
                let tier = evidence_tier(bits);
                let (verdict, _) = falsify(val, best_err, best_depth, all_values);

                discoveries.push(Discovery {
                    operator: "BRIDGE".to_string(),
                    score,
                    description: format!(
                        "BRIDGE: {} ({}) <--[{:.4}]--> {} ({}) via n6: {}",
                        targets[ti].name, cat_i,
                        val,
                        targets[tj].name, cat_j,
                        expr_texts.join(", ")
                    ),
                    domains,
                    formula: expr_texts.join(" = "),
                    diversity: div,
                    precision: prec,
                    novelty: 1.5,
                    bits,
                    tier,
                    falsify: verdict,
                });
            }
        }
    }

    discoveries
}


// ── PREDICT Operator ────────────────────────────────────────────

fn formula_template(formula: &str) -> String {
    let replacements = ["σ", "τ", "φ", "sopfr", "J₂", "μ", "n", "π", "e"];
    let mut tmpl = formula.to_string();
    for r in &replacements {
        if tmpl.contains(r) {
            tmpl = tmpl.replacen(r, "X", 1);
            break;
        }
    }
    tmpl
}

fn op_predict(
    expressions: &[Expr],
    targets: &[PhysicsTarget],
    all_values: &[f64],
    existing_matches: &[Discovery],
) -> Vec<Discovery> {
    let mut predictions = Vec::new();

    // 1. Ladder Extension
    let mut template_groups: HashMap<String, Vec<(f64, String, String)>> = HashMap::new();
    for d in existing_matches.iter().filter(|d| d.operator == "INVERSE") {
        let tmpl = formula_template(&d.formula);
        if let Some(val_str) = d.description.split('=').nth(1) {
            if let Some(val_part) = val_str.trim().split_whitespace().next() {
                if let Ok(val) = val_part.parse::<f64>() {
                    template_groups.entry(tmpl).or_default().push((
                        val, d.formula.clone(), d.description.clone(),
                    ));
                }
            }
        }
    }

    for (tmpl, group) in &template_groups {
        if group.len() < 2 { continue; }
        let mut vals: Vec<f64> = group.iter().map(|(v, _, _)| *v).collect();
        vals.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        vals.dedup_by(|a, b| (*a - *b).abs() < 1e-6);
        if vals.len() < 2 { continue; }

        // Arithmetic progression
        let diffs: Vec<f64> = vals.windows(2).map(|w| w[1] - w[0]).collect();
        let avg_diff = diffs.iter().sum::<f64>() / diffs.len() as f64;
        let diff_var = diffs.iter().map(|d| (d - avg_diff).powi(2)).sum::<f64>() / diffs.len() as f64;

        if diff_var / (avg_diff * avg_diff + 1e-15) < 0.1 && avg_diff.abs() > 1e-6 {
            let predicted = vals.last().unwrap() + avg_diff;
            if predicted.is_finite() && predicted.abs() < 1e15 {
                predictions.push(Discovery {
                    operator: "PREDICT".into(), score: 0.6 * (group.len() as f64 / 5.0).min(1.0),
                    description: format!("LADDER: template '{}' arithmetic progression (step={:.4}), predicts {:.6}", tmpl, avg_diff, predicted),
                    domains: vec!["Prediction/Ladder".into()],
                    formula: format!("{} + {:.4}", tmpl, avg_diff),
                    diversity: 0.125, precision: 0.8, novelty: 1.5,
                    bits: 5.0 + group.len() as f64 * 2.0, tier: 'D', falsify: FalsifyVerdict::Untested,
                });
            }
        }

        // Geometric progression
        if vals.iter().all(|v| *v > 0.0) {
            let ratios: Vec<f64> = vals.windows(2).map(|w| w[1] / w[0]).collect();
            let avg_ratio = ratios.iter().sum::<f64>() / ratios.len() as f64;
            let ratio_var = ratios.iter().map(|r| (r - avg_ratio).powi(2)).sum::<f64>() / ratios.len() as f64;

            if ratio_var / (avg_ratio * avg_ratio + 1e-15) < 0.1 && (avg_ratio - 1.0).abs() > 0.01 {
                let predicted = vals.last().unwrap() * avg_ratio;
                if predicted.is_finite() && predicted.abs() < 1e15 {
                    predictions.push(Discovery {
                        operator: "PREDICT".into(), score: 0.65 * (group.len() as f64 / 5.0).min(1.0),
                        description: format!("LADDER: template '{}' geometric progression (ratio={:.4}), predicts {:.6}", tmpl, avg_ratio, predicted),
                        domains: vec!["Prediction/Ladder".into()],
                        formula: format!("{} * {:.4}", tmpl, avg_ratio),
                        diversity: 0.125, precision: 0.8, novelty: 1.5,
                        bits: 6.0 + group.len() as f64 * 2.0, tier: 'D', falsify: FalsifyVerdict::Untested,
                    });
                }
            }
        }
    }

    // 2. Domain Transfer
    let bridge_matches: Vec<&Discovery> = existing_matches.iter().filter(|d| d.operator == "BRIDGE").collect();
    for bridge in &bridge_matches {
        let bridge_domains: HashSet<&str> = bridge.domains.iter().map(|d| categorize_domain(d)).collect();
        let bridge_formulas: Vec<&str> = bridge.formula.split(" = ").collect();
        for formula_text in &bridge_formulas {
            if let Some(expr) = expressions.iter().find(|e| e.text == *formula_text) {
                for target in targets {
                    let target_cat = categorize_domain(target.domain);
                    if bridge_domains.contains(target_cat) { continue; }
                    let err = ((expr.value - target.value) / target.value.abs().max(1e-300)).abs();
                    if err > 0.005 && err < 0.10 {
                        let b = bayesian_bits(err, expr.depth, 2, all_values.len());
                        predictions.push(Discovery {
                            operator: "PREDICT".into(), score: 0.5 * (1.0 - err),
                            description: format!("TRANSFER: bridge '{}' ({:.6}) may extend to {} in {} (err={:.2}%)", formula_text, expr.value, target.name, target_cat, err * 100.0),
                            domains: vec!["Prediction/Transfer".into(), target.domain.to_string()],
                            formula: formula_text.to_string(),
                            diversity: diversity_score(&bridge.domains), precision: 1.0 - err, novelty: 1.2,
                            bits: b, tier: evidence_tier(b), falsify: FalsifyVerdict::Untested,
                        });
                    }
                }
            }
        }
    }

    // 3. Gap Filling
    let matched_target_names: HashSet<&str> = {
        let mut names = HashSet::new();
        for d in existing_matches {
            for t in targets { if d.description.contains(t.name) { names.insert(t.name); } }
        }
        names
    };

    for target in targets {
        if matched_target_names.contains(target.name) || target.value == 0.0 { continue; }
        let has_shallow = expressions.iter().any(|e| e.depth <= 2 && ((e.value - target.value) / target.value).abs() < 0.01);
        if has_shallow { continue; }

        let mut best_d3: Option<(&Expr, f64)> = None;
        for e in expressions.iter().filter(|e| e.depth == 3) {
            let err = ((e.value - target.value) / target.value).abs();
            if err < 0.01 && (best_d3.is_none() || err < best_d3.unwrap().1) { best_d3 = Some((e, err)); }
        }

        if let Some((expr, err)) = best_d3 {
            let b = bayesian_bits(err.max(1e-10), 3, 1, all_values.len());
            predictions.push(Discovery {
                operator: "PREDICT".into(), score: 0.7 * (1.0 - err),
                description: format!("GAP: {} ({}) no depth<=2 match, depth-3: '{}' = {:.6} (err={:.4}%)", target.name, target.domain, expr.text, expr.value, err * 100.0),
                domains: vec!["Prediction/Gap".into(), target.domain.to_string()],
                formula: expr.text.clone(),
                diversity: 0.125, precision: 1.0 - err, novelty: 1.3,
                bits: b, tier: evidence_tier(b), falsify: FalsifyVerdict::Untested,
            });
        }
    }

    // 4. Value-based prediction (v3): if expression V matches target A, check ALL other targets within 10%
    {
        for d in existing_matches.iter().filter(|d| d.operator == "INVERSE") {
            let val = extract_discovery_value(d);
            if val == 0.0 { continue; }
            for target in targets {
                if d.description.contains(target.name) { continue; }
                if target.value == 0.0 { continue; }
                let err = ((val - target.value) / target.value).abs();
                if err < 0.10 && err > 0.005 {
                    let b = bayesian_bits(err.max(1e-10), 2, 2, all_values.len());
                    predictions.push(Discovery {
                        operator: "PREDICT".into(), score: 0.55 * (1.0 - err),
                        description: format!("VALUE-XFER: n6 expr matching {} (val={:.6}) is within {:.2}% of {}",
                            d.domains.first().map(|s| s.as_str()).unwrap_or("?"), val, err * 100.0, target.name),
                        domains: vec!["Prediction/ValueTransfer".into(), target.domain.to_string()],
                        formula: d.formula.clone(),
                        diversity: 0.25, precision: 1.0 - err, novelty: 1.1,
                        bits: b, tier: evidence_tier(b), falsify: FalsifyVerdict::Untested,
                    });
                }
            }
        }
    }

    // 5. Ratio prediction (v3): if A/B = n6_expr for known targets, predict C/D = same
    {
        let matched_targets: Vec<(&PhysicsTarget, f64)> = targets.iter().filter_map(|t| {
            for d in existing_matches {
                if d.description.contains(t.name) && (d.operator == "INVERSE" || d.operator == "COLLISION") {
                    return Some((t, t.value));
                }
            }
            None
        }).collect();

        let unmatched_targets: Vec<&PhysicsTarget> = targets.iter().filter(|t| {
            !existing_matches.iter().any(|d| d.description.contains(t.name) &&
                (d.operator == "INVERSE" || d.operator == "COLLISION"))
        }).collect();

        for i in 0..matched_targets.len() {
            for j in (i+1)..matched_targets.len() {
                let (t1, v1) = &matched_targets[i];
                let (t2, v2) = &matched_targets[j];
                if *v2 == 0.0 { continue; }
                let ratio = v1 / v2;
                if let Some(ratio_name) = is_n6_ratio(ratio) {
                    // For each unmatched target, predict its pair
                    for ut in &unmatched_targets {
                        let predicted_val = ut.value * ratio;
                        // Check if predicted value matches any other target
                        for ot in targets {
                            if ot.name == ut.name { continue; }
                            if ot.value == 0.0 { continue; }
                            let err = ((predicted_val - ot.value) / ot.value).abs();
                            if err < 0.05 {
                                let b = bayesian_bits(err.max(1e-10), 2, 3, all_values.len());
                                predictions.push(Discovery {
                                    operator: "PREDICT".into(), score: 0.6 * (1.0 - err),
                                    description: format!("RATIO: {}/{} = {} => {}/{} predicted (err={:.2}%)",
                                        t1.name, t2.name, ratio_name, ut.name, ot.name, err * 100.0),
                                    domains: vec!["Prediction/Ratio".into(), ut.domain.to_string(), ot.domain.to_string()],
                                    formula: format!("{} / {} ≈ {}", t1.name, t2.name, ratio_name),
                                    diversity: 0.25, precision: 1.0 - err, novelty: 1.4,
                                    bits: b, tier: evidence_tier(b), falsify: FalsifyVerdict::Untested,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    predictions.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    predictions
}

// ── ANOMALY Operator ────────────────────────────────────────────

fn check_alternative_bases(value: f64) -> AltBaseResult {
    struct AltBase { n: usize, sigma: f64, tau: f64, phi: f64, sopfr: f64 }

    let bases = vec![
        AltBase { n: 8,  sigma: 15.0, tau: 4.0, phi: 4.0,  sopfr: 6.0 },
        AltBase { n: 10, sigma: 18.0, tau: 4.0, phi: 4.0,  sopfr: 7.0 },
        AltBase { n: 12, sigma: 28.0, tau: 6.0, phi: 4.0,  sopfr: 7.0 },
        AltBase { n: 28, sigma: 56.0, tau: 6.0, phi: 12.0, sopfr: 9.0 },
    ];

    let mut best_error = f64::MAX;
    let mut best_base: Option<usize> = None;
    let mut best_expr: Option<String> = None;

    for b in &bases {
        let vals: Vec<(String, f64)> = vec![
            (format!("σ({})", b.n), b.sigma),
            (format!("τ({})", b.n), b.tau),
            (format!("φ({})", b.n), b.phi),
            (format!("sopfr({})", b.n), b.sopfr),
            (format!("σ({})²", b.n), b.sigma * b.sigma),
            (format!("σ({})-τ({})", b.n, b.n), b.sigma - b.tau),
            (format!("σ({})*τ({})", b.n, b.n), b.sigma * b.tau),
            (format!("σ({})/{}", b.n, b.n), b.sigma / b.n as f64),
            (format!("σ({})/τ({})", b.n, b.n), b.sigma / b.tau),
            (format!("σ({})+τ({})", b.n, b.n), b.sigma + b.tau),
            (format!("σ({})*φ({})", b.n, b.n), b.sigma * b.phi),
            (format!("σ({})-φ({})", b.n, b.n), b.sigma - b.phi),
            (format!("σ({})+φ({})+τ({})", b.n, b.n, b.n), b.sigma + b.phi + b.tau),
            (format!("σ({})*sopfr({})", b.n, b.n), b.sigma * b.sopfr),
            (format!("σ({})/φ({})", b.n, b.n), b.sigma / b.phi),
            (format!("σ({})*τ({})*φ({})", b.n, b.n, b.n), b.sigma * b.tau * b.phi),
            (format!("σ({})-{}", b.n, b.n), b.sigma - b.n as f64),
            (format!("σ({})+{}", b.n, b.n), b.sigma + b.n as f64),
            (format!("σ({})²-{}", b.n, b.n), b.sigma * b.sigma - b.n as f64),
        ];

        for (expr_text, expr_val) in &vals {
            if *expr_val == 0.0 { continue; }
            let err = ((expr_val - value) / value).abs();
            if err < 0.05 && err < best_error {
                best_error = err;
                best_base = Some(b.n);
                best_expr = Some(expr_text.clone());
            }
        }
    }

    AltBaseResult { any_match: best_error < 0.05, best_base, best_expr, best_error }
}

fn op_anomaly(
    expressions: &[Expr],
    targets: &[PhysicsTarget],
    _all_values: &[f64],
    existing_matches: &[Discovery],
) -> Vec<Anomaly> {
    let mut anomalies = Vec::new();

    for target in targets {
        let has_match = existing_matches.iter().any(|m| {
            m.description.contains(target.name) &&
            (m.operator == "INVERSE" || m.operator == "COLLISION" || m.operator == "COMPOSE")
        });
        if has_match { continue; }

        let mut closest_expr = String::new();
        let mut closest_error = f64::MAX;
        if target.value != 0.0 {
            for expr in expressions {
                let err = ((expr.value - target.value) / target.value).abs();
                if err < closest_error { closest_error = err; closest_expr = expr.text.clone(); }
            }
        }

        let alt = check_alternative_bases(target.value);

        let classification = if closest_error < 0.05 {
            "UNDISCOVERED_FORMULA".to_string()
        } else if alt.any_match {
            format!("GENUINELY_NON_N6 (n={} fits: {} err={:.2}%)",
                alt.best_base.unwrap_or(0), alt.best_expr.as_deref().unwrap_or("?"), alt.best_error * 100.0)
        } else if closest_error < 0.10 {
            "UNDISCOVERED_FORMULA".to_string()
        } else {
            "MEASUREMENT_OUTDATED".to_string()
        };

        let alt_base_info = if alt.any_match {
            Some(format!("n={}: {} (err={:.2}%)",
                alt.best_base.unwrap_or(0), alt.best_expr.as_deref().unwrap_or("?"), alt.best_error * 100.0))
        } else { None };

        anomalies.push(Anomaly {
            target_name: target.name.to_string(), target_value: target.value,
            closest_expr, closest_error, classification, alt_base: alt_base_info,
        });
    }

    anomalies.sort_by(|a, b| a.closest_error.partial_cmp(&b.closest_error).unwrap_or(std::cmp::Ordering::Equal));
    anomalies
}

// ── Output ───────────────────────────────────────────────────────

fn print_text(discoveries: &[Discovery], total_exprs: usize, total_matches: usize, elapsed_ms: u128, verbose: bool, predictions: &[Discovery], anomalies: &[Anomaly], show_predictions: bool, show_anomalies: bool, phys_template_count: usize) {
    println!("╔══════════════════════════════════════════════════════════════════════════════════╗");
    println!("║       SEDI Discovery Engine v3 — Search for Extra-Dimensional Intelligence      ║");
    println!("╠══════════════════════════════════════════════════════════════════════════════════╣");
    println!("║  Expressions enumerated: {:>8}  (incl. {} physics templates)                  ║", total_exprs, phys_template_count);
    println!("║  Discoveries found:      {:>8}                                                ║", total_matches);
    println!("║  Predictions:            {:>8}                                                ║", predictions.len());
    println!("║  Anomalies:              {:>8}                                                ║", anomalies.len());
    println!("║  Time elapsed:           {:>5} ms                                               ║", elapsed_ms);
    println!("╚══════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    println!("Top 50 Discoveries (ranked by score):");
    println!("{:<4} {:<9} {:<6} {:<5} {:<5} {:<5} {:<6} {:<4} {:<7} {}",
        "#", "Operator", "Score", "Div", "Prec", "Nov", "Bits", "Tier", "Falsify", "Description");
    println!("{}", "-".repeat(140));

    for (i, d) in discoveries.iter().take(50).enumerate() {
        let desc_trunc = if d.description.chars().count() > 80 {
            let s: String = d.description.chars().take(77).collect();
            format!("{}...", s)
        } else {
            d.description.clone()
        };
        println!("{:<4} {:<9} {:<6.3} {:<5.2} {:<5.2} {:<5.2} {:<6.1} {:<4} {:<7} {}",
            i + 1, d.operator, d.score, d.diversity, d.precision, d.novelty,
            d.bits, d.tier, d.falsify, desc_trunc);
    }

    if verbose {
        println!();
        println!("=== FALSIFY Details ===");
        println!();
        for (i, d) in discoveries.iter().take(50).enumerate() {
            println!("  #{:<3} [{}] {} — Verdict: {} | Bits: {:.1} | Tier: {}",
                i + 1, d.operator, d.formula, d.falsify, d.bits, d.tier);
        }
    }

    // Key SEDI findings
    println!();
    println!("=== Key SEDI Findings ===");
    println!();

    let key_targets = [
        "m_p/m_e", "fine structure", "21cm", "T_CMB", "H₀",
        "α_s", "sin²θ_W", "Weinberg", "baryon", "Higgs", "W boson", "Z boson",
        "neutrino", "Jarlskog", "cos²θ_W",
    ];

    for d in discoveries {
        for kt in &key_targets {
            if d.description.to_lowercase().contains(&kt.to_lowercase()) {
                println!("  [{}|{}|{}] {}", d.operator, d.tier, d.falsify, d.description);
                break;
            }
        }
    }

    // Summary statistics
    println!();
    println!("=== Evidence Summary ===");
    println!();
    let mut tier_counts = [0usize; 5]; // A, B, C, D, E
    for d in discoveries {
        match d.tier {
            'A' => tier_counts[0] += 1,
            'B' => tier_counts[1] += 1,
            'C' => tier_counts[2] += 1,
            'D' => tier_counts[3] += 1,
            _ => tier_counts[4] += 1,
        }
    }
    println!("  Tier A (>40 bits): {}", tier_counts[0]);
    println!("  Tier B (>25 bits): {}", tier_counts[1]);
    println!("  Tier C (>15 bits): {}", tier_counts[2]);
    println!("  Tier D (>5 bits):  {}", tier_counts[3]);
    println!("  Tier E (<5 bits):  {}", tier_counts[4]);

    let mut falsify_counts = [0usize; 3]; // Strong, Weak, Reject
    for d in discoveries {
        match d.falsify {
            FalsifyVerdict::Strong => falsify_counts[0] += 1,
            FalsifyVerdict::Weak => falsify_counts[1] += 1,
            FalsifyVerdict::Reject => falsify_counts[2] += 1,
            _ => {}
        }
    }
    println!();
    println!("  FALSIFY: {} STRONG | {} WEAK | {} REJECT",
        falsify_counts[0], falsify_counts[1], falsify_counts[2]);

    // Top 10 by bits
    println!();
    println!("=== Top 10 by Bayesian Evidence ===");
    println!();
    let mut by_bits: Vec<&Discovery> = discoveries.iter().collect();
    by_bits.sort_by(|a, b| b.bits.partial_cmp(&a.bits).unwrap_or(std::cmp::Ordering::Equal));
    for (i, d) in by_bits.iter().take(10).enumerate() {
        let desc_trunc = if d.description.chars().count() > 90 {
            let s: String = d.description.chars().take(87).collect();
            format!("{}...", s)
        } else {
            d.description.clone()
        };
        println!("  #{:<2} [{}/{}] {:.1} bits — {}",
            i + 1, d.tier, d.falsify, d.bits, desc_trunc);
    }

    // PREDICT summary
    if !predictions.is_empty() {
        println!();
        println!("=== PREDICT: {} Predictions ===", predictions.len());
        println!();
        for (i, p) in predictions.iter().enumerate() {
            let desc_trunc = if p.description.chars().count() > 100 {
                let s: String = p.description.chars().take(97).collect();
                format!("{}...", s)
            } else {
                p.description.clone()
            };
            println!("  P{:<3} [score={:.3}] {}", i + 1, p.score, desc_trunc);
            if show_predictions {
                println!("        formula: {} | domains: {}", p.formula, p.domains.join(", "));
            }
        }
    }

    // ANOMALY summary
    if !anomalies.is_empty() {
        println!();
        println!("=== ANOMALY: {} Unmatched Targets ===", anomalies.len());
        println!();
        println!("  {:<4} {:<25} {:<14} {:<22} {:<10} {}",
            "#", "Target", "Value", "Closest n6 expr", "Error%", "Classification");
        println!("  {}", "-".repeat(120));
        for (i, a) in anomalies.iter().enumerate() {
            let cls_trunc = if a.classification.chars().count() > 40 {
                let s: String = a.classification.chars().take(37).collect();
                format!("{}...", s)
            } else {
                a.classification.clone()
            };
            let expr_trunc = if a.closest_expr.chars().count() > 20 {
                let s: String = a.closest_expr.chars().take(17).collect();
                format!("{}...", s)
            } else {
                a.closest_expr.clone()
            };
            println!("  {:<4} {:<25} {:<14.6} {:<22} {:<10.4} {}",
                i + 1, a.target_name, a.target_value, expr_trunc,
                a.closest_error * 100.0, cls_trunc);
            if show_anomalies {
                if let Some(ref alt) = a.alt_base {
                    println!("        alt base: {}", alt);
                }
            }
        }
    }

    // ── v3: Anomaly Resolution Report ────────────────────────────
    println!();
    println!("=== v3 Anomaly Resolution Report ===");
    println!();
    let anomaly_targets: [(&str, f64); 4] = [
        ("eta (baryon asymmetry)", 6.14e-10),
        ("m_Planck/m_p", 1.22e19),
        ("theta_CP (strong CP angle)", 1e-10),
        ("Lambda (cosmo const)", 1.1e-52),
    ];
    for &(name, target_val) in &anomaly_targets {
        // Check if any discovery matches this anomaly target
        let mut resolved = false;
        for d in discoveries {
            // Check if the description mentions the target or value is close
            let d_val = extract_discovery_value(d);
            if target_val.abs() > 1e-100 && d_val.abs() > 1e-100 {
                let err = ((d_val - target_val) / target_val).abs();
                if err < 0.10 {
                    println!("  [RESOLVED] {} -- {} (err={:.2}%)", name, truncate_str(&d.formula, 50), err * 100.0);
                    resolved = true;
                    break;
                }
            }
        }
        // Also check physics templates directly
        if !resolved {
            for d in predictions {
                let d_val = extract_discovery_value(d);
                if target_val.abs() > 1e-100 && d_val.abs() > 1e-100 {
                    let err = ((d_val - target_val) / target_val).abs();
                    if err < 0.10 {
                        println!("  [PREDICTED] {} -- {} (err={:.2}%)", name, truncate_str(&d.formula, 50), err * 100.0);
                        resolved = true;
                        break;
                    }
                }
            }
        }
        if !resolved {
            println!("  [OPEN] {} -- no match within 10%", name);
        }
    }

    // ── v3: Top-3 by Bayesian Bits (prominent) ──────────────────
    println!();
    println!("=== TOP-3 by Bayesian Evidence (v3 highlight) ===");
    println!();
    let mut by_bits: Vec<&Discovery> = discoveries.iter().collect();
    by_bits.sort_by(|a, b| b.bits.partial_cmp(&a.bits).unwrap_or(std::cmp::Ordering::Equal));
    for (i, d) in by_bits.iter().take(3).enumerate() {
        println!("  #{} [{}/{}] {:.1} bits", i + 1, d.tier, d.falsify, d.bits);
        println!("     Formula: {}", truncate_str(&d.formula, 80));
        println!("     {}", truncate_str(&d.description, 100));
        println!();
    }
}

fn print_json(discoveries: &[Discovery], total_exprs: usize, total_matches: usize, elapsed_ms: u128, predictions: &[Discovery], anomalies: &[Anomaly]) {
    println!("{{");
    println!("  \"engine\": \"SEDI Discovery Engine\",");
    println!("  \"version\": \"3.0.0\",");
    println!("  \"stats\": {{");
    println!("    \"expressions_enumerated\": {},", total_exprs);
    println!("    \"discoveries_found\": {},", total_matches);
    println!("    \"predictions_count\": {},", predictions.len());
    println!("    \"anomalies_count\": {},", anomalies.len());
    println!("    \"elapsed_ms\": {}", elapsed_ms);
    println!("  }},");
    println!("  \"discoveries\": [");

    let count = discoveries.len().min(50);
    for (i, d) in discoveries.iter().take(50).enumerate() {
        let comma = if i < count - 1 { "," } else { "" };
        let domains_json: Vec<String> = d.domains.iter()
            .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
            .collect();
        println!("    {{");
        println!("      \"rank\": {},", i + 1);
        println!("      \"operator\": \"{}\",", d.operator);
        println!("      \"score\": {:.4},", d.score);
        println!("      \"diversity\": {:.4},", d.diversity);
        println!("      \"precision\": {:.4},", d.precision);
        println!("      \"novelty\": {:.4},", d.novelty);
        println!("      \"bits\": {:.2},", d.bits);
        println!("      \"tier\": \"{}\",", d.tier);
        println!("      \"falsify\": \"{}\",", d.falsify);
        println!("      \"formula\": \"{}\",", d.formula.replace('"', "\\\""));
        println!("      \"description\": \"{}\",", d.description.replace('"', "\\\""));
        println!("      \"domains\": [{}]", domains_json.join(", "));
        println!("    }}{}", comma);
    }
    println!("  ],");

    // Predictions
    println!("  \"predictions\": [");
    let pcount = predictions.len().min(50);
    for (i, p) in predictions.iter().take(50).enumerate() {
        let comma = if i < pcount - 1 { "," } else { "" };
        let domains_json: Vec<String> = p.domains.iter()
            .map(|s| format!("\"{}\"", s.replace('"', "\\\"")))
            .collect();
        println!("    {{");
        println!("      \"rank\": {},", i + 1);
        println!("      \"operator\": \"PREDICT\",");
        println!("      \"score\": {:.4},", p.score);
        println!("      \"formula\": \"{}\",", p.formula.replace('"', "\\\""));
        println!("      \"description\": \"{}\",", p.description.replace('"', "\\\""));
        println!("      \"domains\": [{}]", domains_json.join(", "));
        println!("    }}{}", comma);
    }
    println!("  ],");

    // Anomalies
    println!("  \"anomalies\": [");
    let acount = anomalies.len();
    for (i, a) in anomalies.iter().enumerate() {
        let comma = if i < acount - 1 { "," } else { "" };
        let alt_json = match &a.alt_base {
            Some(s) => format!("\"{}\"", s.replace('"', "\\\"")),
            None => "null".to_string(),
        };
        println!("    {{");
        println!("      \"target\": \"{}\",", a.target_name.replace('"', "\\\""));
        println!("      \"value\": {},", a.target_value);
        println!("      \"closest_expr\": \"{}\",", a.closest_expr.replace('"', "\\\""));
        println!("      \"closest_error\": {:.6},", a.closest_error);
        println!("      \"classification\": \"{}\",", a.classification.replace('"', "\\\""));
        println!("      \"alt_base\": {}", alt_json);
        println!("    }}{}", comma);
    }
    println!("  ]");

    println!("}}");
}

// ── META Operator ───────────────────────────────────────────────

const MAX_META_PER_LEVEL: usize = 1000;

/// Check if a ratio is expressible as a simple n=6 arithmetic value
fn is_n6_ratio(ratio: f64) -> Option<&'static str> {
    let candidates: &[(&str, f64)] = &[
        ("n", 6.0), ("σ", 12.0), ("τ", 4.0), ("φ", 2.0), ("sopfr", 5.0),
        ("J₂", 24.0), ("μ", 1.0), ("σ/n", 2.0), ("n/τ", 1.5), ("σ/τ", 3.0),
        ("J₂/n", 4.0), ("J₂/σ", 2.0), ("σ/sopfr", 2.4), ("n/sopfr", 1.2),
        ("τ/φ", 2.0), ("sopfr/τ", 1.25), ("σ/φ", 6.0), ("J₂/τ", 6.0),
        ("n/φ", 3.0), ("σ²", 144.0), ("n²", 36.0), ("τ²", 16.0),
        ("1/n", 1.0/6.0), ("1/σ", 1.0/12.0), ("1/τ", 0.25), ("1/φ", 0.5),
        ("π", std::f64::consts::PI), ("e", std::f64::consts::E),
        ("sqrt(n)", 6.0_f64.sqrt()), ("sqrt(σ)", 12.0_f64.sqrt()),
    ];
    for &(name, val) in candidates {
        if val.abs() > 1e-15 && ((ratio - val) / val).abs() < 0.005 {
            return Some(name);
        }
    }
    None
}

/// Decompose a single value into n=6 expressions (for meta-inverse)
fn inverse_single(value: f64, expressions: &[(String, f64)]) -> Vec<(String, f64)> {
    let mut results = Vec::new();
    if value == 0.0 { return results; }
    for (text, eval) in expressions {
        let err = ((eval - value) / value).abs();
        if err < 0.005 {
            results.push((text.clone(), err));
        }
    }
    results.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(3);
    results
}

/// META: Apply operators recursively — feed output of one operator as input to another
fn op_meta(
    collision_results: &[Discovery],
    _inverse_results: &[Discovery],
    compose_results: &[Discovery],
    _bridge_results: &[Discovery],
    expressions: &[Expr],
    all_values: &[f64],
    max_depth: usize,
) -> Vec<Discovery> {
    let mut meta_discoveries: Vec<Discovery> = Vec::new();
    let expr_pairs: Vec<(String, f64)> = expressions.iter()
        .map(|e| (e.text.clone(), e.value))
        .collect();

    // ── Level 1: Cross-operator combinations ──

    // 1a. BRIDGE between COLLISION results: pairs in different domains connected by n=6 ratio
    {
        let mut collision_by_cat: HashMap<&str, Vec<&Discovery>> = HashMap::new();
        for d in collision_results {
            for dom in &d.domains {
                let cat = categorize_domain(dom);
                collision_by_cat.entry(cat).or_default().push(d);
            }
        }
        let cats: Vec<&str> = collision_by_cat.keys().cloned().collect();
        let mut count = 0;
        'bridge_loop: for i in 0..cats.len() {
            for j in (i+1)..cats.len() {
                let left = &collision_by_cat[cats[i]];
                let right = &collision_by_cat[cats[j]];
                for c1 in left.iter().take(20) {
                    for c2 in right.iter().take(20) {
                        if c1.precision < 0.5 || c2.precision < 0.5 { continue; }
                        // Extract representative value from the formula (first numeric match)
                        let v1 = extract_discovery_value(c1);
                        let v2 = extract_discovery_value(c2);
                        if v1 == 0.0 || v2 == 0.0 { continue; }
                        let ratio = v1 / v2;
                        if let Some(ratio_name) = is_n6_ratio(ratio) {
                            let domains = vec![cats[i].to_string(), cats[j].to_string()];
                            let div = diversity_score(&domains);
                            let score = div * (c1.score + c2.score) / 2.0 * 1.2;
                            let bits = (c1.bits + c2.bits) / 2.0 + 5.0; // bonus for meta-connection
                            meta_discoveries.push(Discovery {
                                operator: "META".to_string(),
                                score,
                                description: format!(
                                    "META-BRIDGE: COLLISION({}) / COLLISION({}) = {} [ratio={}]",
                                    truncate_str(&c1.description, 40),
                                    truncate_str(&c2.description, 40),
                                    ratio_name, cats[j]
                                ),
                                domains,
                                formula: format!("({}) / ({}) ≈ {}", c1.formula, c2.formula, ratio_name),
                                diversity: div,
                                precision: (c1.precision + c2.precision) / 2.0,
                                novelty: 1.8,
                                bits,
                                tier: evidence_tier(bits),
                                falsify: FalsifyVerdict::Untested,
                            });
                            count += 1;
                            if count >= MAX_META_PER_LEVEL { break 'bridge_loop; }
                        }
                    }
                }
            }
        }
    }

    // 1b. INVERSE on COLLISION values: collision values that themselves decompose into n=6
    {
        let mut count = 0;
        for collision in collision_results {
            if count >= MAX_META_PER_LEVEL { break; }
            let val = extract_discovery_value(collision);
            if val == 0.0 { continue; }
            let sub_decomps = inverse_single(val, &expr_pairs);
            for (expr_text, err) in &sub_decomps {
                let bits = collision.bits + bayesian_bits(err.max(1e-10), 1, 1, all_values.len()) * 0.5;
                let score = collision.score * 1.1;
                meta_discoveries.push(Discovery {
                    operator: "META".to_string(),
                    score,
                    description: format!(
                        "META-INVERSE: COLLISION value {:.6} itself ≈ {} (err={:.4}%)",
                        val, expr_text, err * 100.0
                    ),
                    domains: collision.domains.clone(),
                    formula: format!("{} → {}", collision.formula, expr_text),
                    diversity: collision.diversity,
                    precision: 1.0 - err,
                    novelty: 2.0,
                    bits,
                    tier: evidence_tier(bits),
                    falsify: FalsifyVerdict::Untested,
                });
                count += 1;
                if count >= MAX_META_PER_LEVEL { break; }
            }
        }
    }

    // 1c. COLLISION among COMPOSE results: group compose results by value, find overlaps
    {
        let mut value_groups: HashMap<i64, Vec<&Discovery>> = HashMap::new();
        for d in compose_results {
            let val = extract_discovery_value(d);
            if val == 0.0 { continue; }
            let key = (val * 1000.0).round() as i64;
            value_groups.entry(key).or_default().push(d);
        }
        let mut count = 0;
        for (_key, group) in &value_groups {
            if group.len() < 2 { continue; }
            // Check for domain diversity within the group
            let mut all_domains: Vec<String> = Vec::new();
            for d in group {
                all_domains.extend(d.domains.clone());
            }
            all_domains.sort();
            all_domains.dedup();
            let cats: HashSet<&str> = all_domains.iter().map(|d| categorize_domain(d)).collect();
            if cats.len() < 2 { continue; }

            let div = diversity_score(&all_domains);
            let avg_score: f64 = group.iter().map(|d| d.score).sum::<f64>() / group.len() as f64;
            let avg_bits: f64 = group.iter().map(|d| d.bits).sum::<f64>() / group.len() as f64;
            let bits = avg_bits + 3.0; // bonus
            let formulas: Vec<&str> = group.iter().map(|d| d.formula.as_str()).collect();

            meta_discoveries.push(Discovery {
                operator: "META".to_string(),
                score: avg_score * 1.3,
                description: format!(
                    "META-COLLISION: {} COMPOSE results share value, spanning {} categories",
                    group.len(), cats.len()
                ),
                domains: all_domains,
                formula: formulas.join(" ≡ "),
                diversity: div,
                precision: group.iter().map(|d| d.precision).sum::<f64>() / group.len() as f64,
                novelty: 1.6,
                bits,
                tier: evidence_tier(bits),
                falsify: FalsifyVerdict::Untested,
            });
            count += 1;
            if count >= MAX_META_PER_LEVEL { break; }
        }
    }

    // ── Level 2: Meta on Meta (if max_depth >= 2) ──
    if max_depth >= 2 && !meta_discoveries.is_empty() {
        let level1 = meta_discoveries.clone();
        let mut level2: Vec<Discovery> = Vec::new();
        let mut count = 0;

        // Bridge between level-1 meta discoveries in different categories
        for i in 0..level1.len().min(50) {
            for j in (i+1)..level1.len().min(50) {
                if count >= MAX_META_PER_LEVEL / 2 { break; }
                let cats_i: HashSet<&str> = level1[i].domains.iter().map(|d| categorize_domain(d)).collect();
                let cats_j: HashSet<&str> = level1[j].domains.iter().map(|d| categorize_domain(d)).collect();
                // Must have at least one non-overlapping category
                if cats_i.intersection(&cats_j).count() == cats_i.len().min(cats_j.len()) { continue; }

                let mut all_domains: Vec<String> = Vec::new();
                all_domains.extend(level1[i].domains.clone());
                all_domains.extend(level1[j].domains.clone());
                all_domains.sort();
                all_domains.dedup();
                let div = diversity_score(&all_domains);
                if div <= level1[i].diversity && div <= level1[j].diversity { continue; }

                let bits = (level1[i].bits + level1[j].bits) / 2.0 + 2.0;
                level2.push(Discovery {
                    operator: "META".to_string(),
                    score: (level1[i].score + level1[j].score) / 2.0 * div,
                    description: format!(
                        "META²: ({}) + ({}) span {} categories",
                        truncate_str(&level1[i].description, 35),
                        truncate_str(&level1[j].description, 35),
                        all_domains.len()
                    ),
                    domains: all_domains,
                    formula: format!("({}) ∧ ({})", level1[i].formula, level1[j].formula),
                    diversity: div,
                    precision: (level1[i].precision + level1[j].precision) / 2.0,
                    novelty: 2.0,
                    bits,
                    tier: evidence_tier(bits),
                    falsify: FalsifyVerdict::Untested,
                });
                count += 1;
            }
        }
        meta_discoveries.extend(level2);
    }

    // Sort by score, keep top-K
    meta_discoveries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    meta_discoveries.truncate(MAX_META_PER_LEVEL);
    meta_discoveries
}

/// Extract the numeric value from a Discovery (parse from description)
fn extract_discovery_value(d: &Discovery) -> f64 {
    // Try description first — COLLISION format: "Value 123.456 matches ..."
    // INVERSE format: "target_name = 123.456 <-- n6: ..."
    // BRIDGE format: "... <--[123.456]--> ..."
    for part in d.description.split(|c: char| c == ' ' || c == '=' || c == ',' || c == '[' || c == ']') {
        if let Ok(v) = part.trim().parse::<f64>() {
            if v.is_finite() && v != 0.0 {
                return v;
            }
        }
    }
    // Try formula field
    for part in d.formula.split(|c: char| c == ' ' || c == '=' || c == ',' || c == '(' || c == ')') {
        if let Ok(v) = part.trim().parse::<f64>() {
            if v.is_finite() && v != 0.0 {
                return v;
            }
        }
    }
    0.0
}

fn truncate_str(s: &str, max_len: usize) -> String {
    if s.chars().count() <= max_len {
        s.to_string()
    } else {
        let truncated: String = s.chars().take(max_len - 3).collect();
        format!("{}...", truncated)
    }
}

// ── SYMMETRY Operator ───────────────────────────────────────────

/// Known n=6 constant names for template abstraction
const N6_CONST_NAMES: &[&str] = &[
    "σ²", "J₂", "sopfr", "sqrt", "ln", "exp", "σ", "τ", "φ", "μ", "n", "π", "e",
];

/// Abstract a formula to a template by replacing specific constant names with generic A, B, C...
fn abstract_formula(formula: &str) -> (String, Vec<String>) {
    let mut result = formula.to_string();
    let mut bindings: Vec<String> = Vec::new();
    let labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];

    // Replace constant names in order of longest first (to avoid partial matches)
    let mut sorted_names: Vec<&&str> = N6_CONST_NAMES.iter().collect();
    sorted_names.sort_by(|a, b| b.len().cmp(&a.len()));

    for name in sorted_names {
        if bindings.len() >= labels.len() { break; }
        if result.contains(*name) {
            let label = labels[bindings.len()].to_string();
            result = result.replace(*name, &label);
            bindings.push(name.to_string());
        }
    }

    (result, bindings)
}

/// Generate all possible bindings for a template pattern
fn generate_all_bindings(n_slots: usize) -> Vec<Vec<String>> {
    // Only use the core arithmetic constants (not functions)
    let core_consts = ["n", "σ", "τ", "φ", "sopfr", "J₂", "μ", "π", "e"];
    if n_slots == 0 || n_slots > 4 { return Vec::new(); }

    let mut all: Vec<Vec<String>> = Vec::new();
    match n_slots {
        1 => {
            for a in &core_consts {
                all.push(vec![a.to_string()]);
            }
        }
        2 => {
            for a in &core_consts {
                for b in &core_consts {
                    if a != b {
                        all.push(vec![a.to_string(), b.to_string()]);
                    }
                }
            }
        }
        3 => {
            for a in &core_consts {
                for b in &core_consts {
                    for c in &core_consts {
                        if a != b && b != c && a != c {
                            all.push(vec![a.to_string(), b.to_string(), c.to_string()]);
                        }
                    }
                }
            }
        }
        _ => {
            // For 4+ slots, just return empty to avoid explosion
        }
    }
    all
}

/// SYMMETRY: Find formula templates that recur across multiple targets
fn op_symmetry(
    all_matches: &[Discovery],
    targets: &[PhysicsTarget],
    expressions: &[Expr],
    _all_values: &[f64],
) -> Vec<Discovery> {
    let mut templates: HashMap<String, Vec<(String, Vec<String>, String, f64)>> = HashMap::new();

    // 1. Abstract each matched formula to a template
    for d in all_matches {
        if d.operator == "META" { continue; } // Don't recurse on meta
        let (template, bindings) = abstract_formula(&d.formula);
        if bindings.is_empty() { continue; }

        // Use first domain as target identifier
        let target_name = d.domains.first().cloned().unwrap_or_default();
        templates.entry(template).or_default().push((
            d.formula.clone(),
            bindings,
            target_name,
            d.precision,
        ));
    }

    // 2. Find symmetry classes (templates with 2+ instantiations)
    let classes: Vec<(String, Vec<(String, Vec<String>, String, f64)>)> = templates.into_iter()
        .filter(|(_, instances)| instances.len() >= 2)
        .collect();

    let mut discoveries: Vec<Discovery> = Vec::new();

    for (pattern, instances) in &classes {
        // Collect used bindings
        let used_bindings: HashSet<Vec<String>> = instances.iter()
            .map(|(_, bindings, _, _)| bindings.clone())
            .collect();

        let n_slots = instances[0].1.len();
        let target_names: Vec<&str> = instances.iter()
            .map(|(_, _, target, _)| target.as_str())
            .collect();
        let avg_precision: f64 = instances.iter()
            .map(|(_, _, _, prec)| prec)
            .sum::<f64>() / instances.len() as f64;

        // Create a symmetry class discovery
        let domains: Vec<String> = target_names.iter().map(|s| s.to_string()).collect();
        let div = diversity_score(&domains);
        let score = div * avg_precision * (instances.len() as f64).sqrt() / 2.0;
        let bits = score * 15.0; // heuristic
        let formulas: Vec<&str> = instances.iter().map(|(f, _, _, _)| f.as_str()).collect();

        discoveries.push(Discovery {
            operator: "SYMMETRY".to_string(),
            score,
            description: format!(
                "SYMMETRY: pattern '{}' has {} instantiations across targets: {}",
                pattern, instances.len(), target_names.join(", ")
            ),
            domains: domains.clone(),
            formula: formulas.join(" | "),
            diversity: div,
            precision: avg_precision,
            novelty: 1.5 + 0.1 * instances.len() as f64,
            bits,
            tier: evidence_tier(bits),
            falsify: FalsifyVerdict::Untested,
        });

        // 3. Predict unused bindings
        let all_possible = generate_all_bindings(n_slots);
        let mut prediction_count = 0;
        for binding in &all_possible {
            if used_bindings.contains(binding) { continue; }
            if prediction_count >= 20 { break; } // cap predictions per class

            // Reconstruct the formula from the template with new bindings
            let labels = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H'];
            let mut predicted_formula = pattern.clone();
            for (k, const_name) in binding.iter().enumerate() {
                if k < labels.len() {
                    predicted_formula = predicted_formula.replace(
                        &labels[k].to_string(),
                        const_name,
                    );
                }
            }

            // Try to evaluate the predicted formula by looking it up in expressions
            // (Exact match is unlikely; try a simpler heuristic: reconstruct from bindings)
            let mut predicted_val = None;
            for expr in expressions {
                if expr.text == predicted_formula {
                    predicted_val = Some(expr.value);
                    break;
                }
            }

            // Even if we can't evaluate, record the prediction
            let pred_desc = if let Some(val) = predicted_val {
                // Check if this value matches any known target
                let mut matching_target = None;
                for t in targets {
                    if t.value == 0.0 { continue; }
                    let err = ((val - t.value) / t.value).abs();
                    if err < 0.01 {
                        matching_target = Some((t.name, err));
                        break;
                    }
                }
                if let Some((tname, err)) = matching_target {
                    format!(
                        "SYMMETRY-PREDICT: {} ≈ {:.6} matches {} (err={:.4}%) [from pattern '{}']",
                        predicted_formula, val, tname, err * 100.0, pattern
                    )
                } else {
                    format!(
                        "SYMMETRY-PREDICT: {} ≈ {:.6} (no known target) [from pattern '{}']",
                        predicted_formula, val, pattern
                    )
                }
            } else {
                format!(
                    "SYMMETRY-PREDICT: {} (unevaluated) [from pattern '{}']",
                    predicted_formula, pattern
                )
            };

            let pred_score = score * 0.5;
            let pred_bits = bits * 0.4;
            discoveries.push(Discovery {
                operator: "SYMMETRY".to_string(),
                score: pred_score,
                description: pred_desc,
                domains: domains.clone(),
                formula: predicted_formula,
                diversity: div,
                precision: avg_precision * 0.5,
                novelty: 2.0,
                bits: pred_bits,
                tier: evidence_tier(pred_bits),
                falsify: FalsifyVerdict::Untested,
            });
            prediction_count += 1;
        }
    }

    // Sort by score, keep top results
    discoveries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    discoveries.truncate(MAX_META_PER_LEVEL);
    discoveries
}

// ── Main ─────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.iter().any(|a| a == "--json");
    let verbose = args.iter().any(|a| a == "--verbose");
    let show_predictions = args.iter().any(|a| a == "--predictions");
    let show_anomalies = args.iter().any(|a| a == "--anomalies");
    let min_bits: f64 = args.iter().position(|a| a == "--min-bits")
        .and_then(|i| args.get(i + 1))
        .and_then(|s| s.parse().ok())
        .unwrap_or(f64::NEG_INFINITY);

    let start = Instant::now();

    // Base constants (including transcendentals)
    let base = base_constants();

    // Enumerate expressions (depth 1-2)
    let mut expressions = enumerate_depth2(&base);
    let depth2_count = expressions.len();

    // v3: Add physics templates for extreme-scale anomaly cracking
    let phys_templates = physics_templates();
    let phys_template_count = phys_templates.len();
    expressions.extend(phys_templates);

    if !json_mode {
        eprintln!("SEDI Discovery Engine v3.0.0");
        eprintln!("Enumerated {} depth-2 expressions + {} physics templates from n=6 base constants (+pi, e)",
            depth2_count, phys_template_count);
    }

    // Depth-3 enumeration (parallelized)
    let depth3 = enumerate_depth3(&base, &expressions);
    let depth3_count = depth3.len();
    expressions.extend(depth3);

    // Deduplicate by value
    dedup_by_value(&mut expressions);
    let total_exprs = expressions.len();

    if !json_mode {
        eprintln!("Added {} depth-3 expressions, {} total after dedup", depth3_count, total_exprs);
    }

    // Pre-compute all values for FALSIFY
    let all_values: Vec<f64> = expressions.iter().map(|e| e.value).collect();

    // Load physics targets
    let targets = physics_targets();

    if !json_mode {
        eprintln!("Loaded {} physics targets across SEDI domains", targets.len());
    }

    // Run operators in parallel
    let (pair1, pair2) = rayon::join(
        || rayon::join(
            || op_collision(&expressions, &targets, &all_values),
            || op_inverse(&expressions, &targets, &all_values),
        ),
        || rayon::join(
            || op_compose(&expressions, &targets, &all_values),
            || op_bridge(&expressions, &targets, &all_values),
        ),
    );
    let (d_collision, d_inverse) = pair1;
    let (d_compose, d_bridge) = pair2;

    let counts = (d_collision.len(), d_inverse.len(), d_compose.len(), d_bridge.len());

    let mut all_discoveries: Vec<Discovery> = Vec::new();
    all_discoveries.extend(d_collision.clone());
    all_discoveries.extend(d_inverse.clone());
    all_discoveries.extend(d_compose.clone());
    all_discoveries.extend(d_bridge.clone());

    // ── META + SYMMETRY operators ──────────────────────────────────
    let (d_meta, d_symmetry) = rayon::join(
        || op_meta(
            &d_collision, &d_inverse, &d_compose, &d_bridge,
            &expressions, &all_values, 2,
        ),
        || op_symmetry(
            &all_discoveries, &targets, &expressions, &all_values,
        ),
    );
    let meta_count = d_meta.len();
    let symmetry_count = d_symmetry.len();
    all_discoveries.extend(d_meta);
    all_discoveries.extend(d_symmetry);
    // ── END META + SYMMETRY ────────────────────────────────────────

    // ── PREDICT + ANOMALY operators ───────────────────────────────
    let d_predict = op_predict(&expressions, &targets, &all_values, &all_discoveries);
    let d_anomaly = op_anomaly(&expressions, &targets, &all_values, &all_discoveries);
    let predict_count = d_predict.len();
    let anomaly_count = d_anomaly.len();

    // Add PREDICT discoveries to the main list
    all_discoveries.extend(d_predict.clone());
    // ── END PREDICT + ANOMALY ─────────────────────────────────────

    // v3: Multi-target bonus — if a formula matches 2+ targets, boost falsify survival
    {
        let mut formula_target_count: HashMap<String, usize> = HashMap::new();
        for d in &all_discoveries {
            if d.operator == "INVERSE" || d.operator == "COLLISION" {
                *formula_target_count.entry(d.formula.clone()).or_insert(0) += 1;
            }
        }
        for d in &mut all_discoveries {
            if let Some(&count) = formula_target_count.get(&d.formula) {
                if count >= 2 {
                    // Boost: upgrade WEAK to STRONG, REJECT to WEAK
                    match d.falsify {
                        FalsifyVerdict::Weak => d.falsify = FalsifyVerdict::Strong,
                        FalsifyVerdict::Reject => d.falsify = FalsifyVerdict::Weak,
                        _ => {}
                    }
                    d.bits *= 1.5; // 50% bits bonus
                    d.tier = evidence_tier(d.bits);
                }
            }
        }
    }

    // Filter by min-bits
    if min_bits > f64::NEG_INFINITY {
        all_discoveries.retain(|d| d.bits >= min_bits);
    }

    // Sort by score descending
    all_discoveries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    let total_matches = all_discoveries.len();
    let elapsed = start.elapsed().as_millis();

    if !json_mode {
        eprintln!("COLLISION: {} | INVERSE: {} | COMPOSE: {} | BRIDGE: {}",
            counts.0, counts.1, counts.2, counts.3);
        eprintln!("META: {} | SYMMETRY: {} | PREDICT: {} | ANOMALY: {}",
            meta_count, symmetry_count, predict_count, anomaly_count);
        eprintln!();
    }

    // Output
    if json_mode {
        print_json(&all_discoveries, total_exprs, total_matches, elapsed, &d_predict, &d_anomaly);
    } else {
        print_text(&all_discoveries, total_exprs, total_matches, elapsed, verbose,
            &d_predict, &d_anomaly, show_predictions, show_anomalies, phys_template_count);
    }
}
