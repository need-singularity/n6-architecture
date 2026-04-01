// Discovery Engine — N6 Architecture
// Implements COLLISION, INVERSE, COMPOSE operators from Discovery Algorithm v2.
// Build: ~/.cargo/bin/rustc main.rs -o discovery-engine
// Usage: ./discovery-engine [--json]

use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;

// ── Data Structures ──────────────────────────────────────────────

#[derive(Clone, Debug)]
struct Constant {
    expr: String,
    value: f64,
    domains: Vec<String>,
    bt_refs: Vec<String>,
}

#[derive(Clone, Debug)]
struct DseDomain {
    name: String,
    n6_max: f64,
    status: String,
}

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
}

// ── Base Constants (n=6 arithmetic) ──────────────────────────────

struct BaseConst {
    name: &'static str,
    value: f64,
}

const BASE: [BaseConst; 7] = [
    BaseConst { name: "σ",     value: 12.0 },
    BaseConst { name: "τ",     value: 4.0  },
    BaseConst { name: "φ",     value: 2.0  },
    BaseConst { name: "sopfr", value: 5.0  },
    BaseConst { name: "J₂",    value: 24.0 },
    BaseConst { name: "μ",     value: 1.0  },
    BaseConst { name: "n",     value: 6.0  },
];

// ── Domain Categories (for diversity scoring) ────────────────────

const DOMAIN_CATS: [(&str, &[&str]); 9] = [
    ("Physics",      &["fusion", "superconductor", "superconducting-magnet", "tokamak", "plasma", "particle", "cosmology", "neutrino"]),
    ("AI",           &["ai", "llm", "diffusion", "ssm", "moe", "training", "transformer", "vision", "audio", "nerf", "codec", "tokenizer"]),
    ("Chip",         &["chip", "soc", "semiconductor", "gpu", "hbm", "pim", "3d", "photonic", "wafer", "sc", "fpga", "interconnect", "optical"]),
    ("Energy",       &["energy", "solar", "battery", "hydrogen", "grid", "power", "thermal", "nuclear", "hvdc"]),
    ("Network",      &["network", "crypto", "blockchain", "dns", "tcp", "ipv"]),
    ("Biology",      &["biology", "genetic", "codon", "amino"]),
    ("Math",         &["math", "kissing", "leech", "lattice"]),
    ("Materials",    &["material", "carbon", "diamond", "graphene"]),
    ("Engineering",  &["robotics", "autonomous", "3d-printing", "desalination", "earthquake", "pharmaceutical"]),
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
    cats.len() as f64 / 9.0
}

// ── Parsers ──────────────────────────────────────────────────────

fn parse_atlas_constants(path: &str) -> Vec<Constant> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Warning: cannot read {}: {}", path, e); return vec![]; }
    };
    let mut constants: Vec<Constant> = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        // Match table rows: | expr | value | application | domain |
        if !line.starts_with('|') || line.starts_with("|-") || line.contains("Expression") || line.contains("Symbol") || line.contains("ID ") || line.contains("Parameter") {
            continue;
        }
        let cols: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if cols.len() < 5 { continue; }

        let expr = cols[1].to_string();
        let val_str = cols[2];
        let app_or_domain = cols[3..].join(" ");

        // Try to parse value
        let value = parse_value(val_str);
        if value.is_none() { continue; }
        let value = value.unwrap();
        if value.is_nan() || value.is_infinite() { continue; }

        // Extract domains from the last column(s)
        let domain_text = if cols.len() >= 5 { cols[cols.len() - 1] } else { "" };
        let domains = extract_domains(domain_text, &app_or_domain);

        // Extract BT references
        let bt_refs = extract_bt_refs(&app_or_domain);

        constants.push(Constant { expr, value, domains, bt_refs });
    }
    constants
}

fn parse_value(s: &str) -> Option<f64> {
    let s = s.trim();
    // Handle expressions like "4/3 ≈ 1.333" or "1/2 = 0.5" or "10³ = 1000"
    // Try the first number-like token
    for token in s.split_whitespace() {
        let clean = token.replace(",", "").replace("≈", "").replace("=", "");
        // fraction
        if clean.contains('/') && !clean.contains("\\") {
            let parts: Vec<&str> = clean.split('/').collect();
            if parts.len() == 2 {
                if let (Ok(n), Ok(d)) = (parts[0].parse::<f64>(), parts[1].parse::<f64>()) {
                    if d != 0.0 { return Some(n / d); }
                }
            }
        }
        // superscript notation like 10³ or 10^{-4}
        if clean.contains('^') {
            let parts: Vec<&str> = clean.split('^').collect();
            if parts.len() == 2 {
                let base: f64 = parts[0].parse().unwrap_or(0.0);
                let exp_str = parts[1].trim_matches(|c| c == '{' || c == '}');
                let exp: f64 = exp_str.parse().unwrap_or(0.0);
                if base > 0.0 { return Some(base.powf(exp)); }
            }
        }
        // plain number
        if let Ok(v) = clean.parse::<f64>() {
            return Some(v);
        }
    }
    // Check for Unicode superscripts: ³ ² ⁴ etc.
    let s_clean = s.replace("³", "^3").replace("²", "^2").replace("⁴", "^4")
        .replace("⁵", "^5").replace("⁶", "^6").replace("⁻", "-");
    for token in s_clean.split_whitespace() {
        let clean = token.replace(",", "").replace("≈", "").replace("=", "");
        if clean.contains('^') {
            let parts: Vec<&str> = clean.split('^').collect();
            if parts.len() == 2 {
                let base: f64 = parts[0].parse().unwrap_or(0.0);
                let exp_str = parts[1].trim_matches(|c| c == '{' || c == '}');
                let exp: f64 = exp_str.parse().unwrap_or(0.0);
                if base > 0.0 { return Some(base.powf(exp)); }
            }
        }
        if let Ok(v) = clean.parse::<f64>() { return Some(v); }
    }
    None
}

fn extract_domains(domain_str: &str, full_text: &str) -> Vec<String> {
    let mut domains = Vec::new();
    let combined = format!("{} {}", domain_str, full_text).to_lowercase();
    let keywords = [
        ("ai", "AI"), ("llm", "AI"), ("diffusion", "AI (Diffusion)"),
        ("chip", "Chip"), ("soc", "SoC"), ("semiconductor", "Semiconductor"),
        ("gpu", "Chip"), ("hbm", "Chip"), ("solar", "Solar"), ("battery", "Battery"),
        ("energy", "Energy"), ("power", "Power"), ("grid", "Power"),
        ("fusion", "Fusion"), ("nuclear", "Nuclear"), ("hydrogen", "Hydrogen"),
        ("network", "Network"), ("crypto", "Crypto"), ("blockchain", "Blockchain"),
        ("biology", "Biology"), ("genetic", "Biology"), ("particle", "Particle"),
        ("cosmology", "Cosmology"), ("neutrino", "Neutrino"), ("math", "Math"),
        ("optical", "Optical"), ("photonic", "Photonic"), ("pim", "PIM"),
        ("3d", "3D"), ("wafer", "Wafer"), ("sc", "SC"), ("display", "Display"),
        ("audio", "Audio"), ("robotics", "Robotics"), ("thermal", "Thermal"),
        ("rl", "AI (RL)"), ("training", "AI (Training)"), ("ssm", "AI (SSM)"),
        ("moe", "AI (MoE)"), ("transformer", "AI"),
    ];
    let mut seen = HashSet::new();
    for (kw, dom) in &keywords {
        if combined.contains(kw) && seen.insert(*dom) {
            domains.push(dom.to_string());
        }
    }
    if domains.is_empty() {
        domains.push("General".to_string());
    }
    domains
}

fn extract_bt_refs(text: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let mut i = 0;
    let bytes = text.as_bytes();
    while i + 2 < bytes.len() {
        if i + 2 < bytes.len() && bytes[i] == b'B' && bytes[i + 1] == b'T' && bytes[i + 2] == b'-' {
            let start = i;
            i += 3;
            while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
            let bt = &text[start..i];
            if bt.len() > 3 { refs.push(bt.to_string()); }
        } else {
            i += 1;
        }
    }
    refs
}

fn parse_dse_map(path: &str) -> Vec<DseDomain> {
    let content = match fs::read_to_string(path) {
        Ok(c) => c,
        Err(e) => { eprintln!("Warning: cannot read {}: {}", path, e); return vec![]; }
    };
    let mut domains = Vec::new();
    let mut current_name = String::new();
    let mut current_n6: f64 = 0.0;
    let mut current_status = String::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with('[') && !line.starts_with("[meta") && line.ends_with(']') {
            // Flush previous
            if !current_name.is_empty() {
                domains.push(DseDomain {
                    name: current_name.clone(),
                    n6_max: current_n6,
                    status: current_status.clone(),
                });
            }
            current_name = line[1..line.len()-1].to_string();
            current_n6 = 0.0;
            current_status = String::new();
        }
        if line.starts_with("n6_max") {
            if let Some(v) = line.split('=').nth(1) {
                current_n6 = v.trim().parse().unwrap_or(0.0);
            }
        }
        if line.starts_with("dse") && line.contains('=') {
            if let Some(v) = line.split('=').nth(1) {
                current_status = v.trim().trim_matches('"').to_string();
            }
        }
    }
    // Flush last
    if !current_name.is_empty() {
        domains.push(DseDomain {
            name: current_name,
            n6_max: current_n6,
            status: current_status,
        });
    }
    domains
}

// ── Expression Enumeration ───────────────────────────────────────

#[derive(Clone, Debug)]
struct Expr {
    text: String,
    value: f64,
}

fn enumerate_depth1() -> Vec<Expr> {
    let mut exprs = Vec::new();
    // Base constants
    for b in &BASE {
        exprs.push(Expr { text: b.name.to_string(), value: b.value });
    }
    // Unary ops on base constants
    for b in &BASE {
        if b.value > 0.0 {
            // 1/x
            exprs.push(Expr { text: format!("1/{}", b.name), value: 1.0 / b.value });
            // x^2
            exprs.push(Expr { text: format!("{}²", b.name), value: b.value * b.value });
            // 2^x (only for small values)
            if b.value <= 24.0 {
                exprs.push(Expr { text: format!("2^{}", b.name), value: (2.0_f64).powf(b.value) });
            }
            // ln(x) for x > 1
            if b.value > 1.0 {
                exprs.push(Expr { text: format!("ln({})", b.name), value: b.value.ln() });
            }
        }
    }
    exprs
}

fn enumerate_depth2() -> Vec<Expr> {
    let mut exprs = enumerate_depth1();
    let n = BASE.len();

    // Binary ops on all pairs of base constants
    for i in 0..n {
        for j in 0..n {
            let a = &BASE[i];
            let b = &BASE[j];

            // a + b
            exprs.push(Expr {
                text: format!("{}+{}", a.name, b.name),
                value: a.value + b.value,
            });
            // a - b (if positive)
            if a.value > b.value {
                exprs.push(Expr {
                    text: format!("{}-{}", a.name, b.name),
                    value: a.value - b.value,
                });
            }
            // a * b
            exprs.push(Expr {
                text: format!("{}·{}", a.name, b.name),
                value: a.value * b.value,
            });
            // a / b
            if b.value != 0.0 {
                exprs.push(Expr {
                    text: format!("{}/{}", a.name, b.name),
                    value: a.value / b.value,
                });
            }
            // a ^ b (if result reasonable)
            if a.value > 0.0 && b.value.abs() <= 12.0 {
                let r = a.value.powf(b.value);
                if r.is_finite() && r.abs() < 1e15 {
                    exprs.push(Expr {
                        text: format!("{}^{}", a.name, b.name),
                        value: r,
                    });
                }
            }
        }
    }

    // Depth-2: binary op on (depth-1 unary) with base
    let unary_exprs = enumerate_depth1();
    for ue in &unary_exprs {
        if ue.text.contains('+') || ue.text.contains('-') || ue.text.contains('·') || ue.text.contains('/') {
            continue; // skip already-compound
        }
        for b in &BASE {
            // ue + b, ue - b, ue * b, ue / b
            exprs.push(Expr {
                text: format!("({})+{}", ue.text, b.name),
                value: ue.value + b.value,
            });
            if ue.value > b.value {
                exprs.push(Expr {
                    text: format!("({})-{}", ue.text, b.name),
                    value: ue.value - b.value,
                });
            }
            if b.value > ue.value {
                exprs.push(Expr {
                    text: format!("{}-({})", b.name, ue.text),
                    value: b.value - ue.value,
                });
            }
            exprs.push(Expr {
                text: format!("({})·{}", ue.text, b.name),
                value: ue.value * b.value,
            });
            if b.value != 0.0 {
                exprs.push(Expr {
                    text: format!("({})/{}", ue.text, b.name),
                    value: ue.value / b.value,
                });
            }
            if ue.value != 0.0 {
                exprs.push(Expr {
                    text: format!("{}/{}", b.name, ue.text),
                    value: b.value / ue.value,
                });
            }
        }
    }

    // Special compound expressions from n=6 theory
    let specials = vec![
        Expr { text: "1-1/(σ-φ)".into(), value: 0.9 },
        Expr { text: "1-1/(J₂-τ)".into(), value: 0.95 },
        Expr { text: "1/e".into(), value: 1.0_f64 / std::f64::consts::E },
        Expr { text: "ln(4/3)".into(), value: (4.0_f64 / 3.0).ln() },
        Expr { text: "σ(σ-μ)+sopfr+μ/P₂".into(), value: 12.0 * 11.0 + 5.0 + 1.0/28.0 },
        Expr { text: "σ/(σ-φ)".into(), value: 1.2 },
        Expr { text: "τ²/(n/φ)³".into(), value: 16.0 / 27.0 },
        Expr { text: "1/2+1/3+1/6".into(), value: 1.0 },
        Expr { text: "σ·φ·n".into(), value: 144.0 },
        Expr { text: "σ(σ-φ)".into(), value: 120.0 },
        Expr { text: "σ²-φ".into(), value: 142.0 },
        Expr { text: "(σ-φ)^(n/φ)".into(), value: 1000.0 },
        Expr { text: "(σ-φ)^τ".into(), value: 10000.0 },
        Expr { text: "φ^τ·sopfr".into(), value: 80.0 },
        Expr { text: "σ·φ^τ".into(), value: 192.0 },
        Expr { text: "σ·J₂".into(), value: 288.0 },
        Expr { text: "σ·(σ-τ)".into(), value: 96.0 },
        Expr { text: "σ·τ·(σ-φ)".into(), value: 480.0 },
        Expr { text: "(σ-φ)³".into(), value: 1000.0 },
        Expr { text: "τ·(σ-φ)".into(), value: 40.0 },
        Expr { text: "sopfr·(σ-φ)".into(), value: 50.0 },
        Expr { text: "(σ+n/φ)/φ".into(), value: 7.5 },
        Expr { text: "sopfr·2^n".into(), value: 320.0 },
        Expr { text: "10^{-(σ-τ)}".into(), value: 1e-8 },
        Expr { text: "φ²/n".into(), value: 2.0 / 3.0 },
        Expr { text: "n/φ/(σ+μ)".into(), value: 3.0 / 13.0 },
        Expr { text: "n/φ/(σ-φ)".into(), value: 0.3 },
        Expr { text: "τ/(σ-sopfr)".into(), value: 4.0 / 7.0 },
        Expr { text: "μ/σ".into(), value: 1.0 / 12.0 },
        Expr { text: "φ/(σ-φ)".into(), value: 0.2 },
        Expr { text: "n/φ/(σ-φ)²".into(), value: 0.03 },
        Expr { text: "J₂²".into(), value: 576.0 },
        Expr { text: "σ⁴".into(), value: 20736.0 },
        Expr { text: "σ³".into(), value: 1728.0 },
    ];
    exprs.extend(specials);

    // Deduplicate by (text)
    let mut seen = HashSet::new();
    exprs.retain(|e| {
        if e.value.is_nan() || e.value.is_infinite() { return false; }
        seen.insert(e.text.clone())
    });

    exprs
}

// ── Engineering Target Values ────────────────────────────────────

struct EngTarget {
    value: f64,
    name: &'static str,
    domain: &'static str,
}

fn engineering_targets() -> Vec<EngTarget> {
    vec![
        // AI / LLM
        EngTarget { value: 4096.0,  name: "GPT-3 d_model",          domain: "AI" },
        EngTarget { value: 2048.0,  name: "LLaMA-7B d_model",       domain: "AI" },
        EngTarget { value: 128.0,   name: "d_head universal",        domain: "AI" },
        EngTarget { value: 32.0,    name: "Attention heads (large)", domain: "AI" },
        EngTarget { value: 0.9,     name: "Adam β₁",                domain: "AI" },
        EngTarget { value: 0.95,    name: "Top-p sampling",          domain: "AI" },
        EngTarget { value: 0.999,   name: "Adam β₂ (BERT)",         domain: "AI" },
        EngTarget { value: 1e-8,    name: "Adam ε",                 domain: "AI" },
        EngTarget { value: 0.1,     name: "Weight decay",            domain: "AI" },
        EngTarget { value: 0.288,   name: "Mertens dropout",         domain: "AI" },
        EngTarget { value: 7.5,     name: "CFG guidance scale",      domain: "AI" },
        EngTarget { value: 1000.0,  name: "DDPM timesteps",          domain: "AI" },
        EngTarget { value: 50.0,    name: "DDIM steps",              domain: "AI" },
        EngTarget { value: 20.0,    name: "Chinchilla ratio",        domain: "AI" },
        EngTarget { value: 8.0,     name: "LoRA rank / KV heads",    domain: "AI" },
        EngTarget { value: 40.0,    name: "Top-k sampling",          domain: "AI" },
        // Chip
        EngTarget { value: 144.0,   name: "AD102 SMs",              domain: "Chip" },
        EngTarget { value: 132.0,   name: "H100 SMs",               domain: "Chip" },
        EngTarget { value: 80.0,    name: "A100-80GB HBM",          domain: "Chip" },
        EngTarget { value: 192.0,   name: "B100 HBM (GB)",          domain: "Chip" },
        EngTarget { value: 288.0,   name: "B300 HBM (GB)",          domain: "Chip" },
        EngTarget { value: 48.0,    name: "N3 gate pitch (nm)",      domain: "Chip" },
        EngTarget { value: 256.0,   name: "MI350X CUs",             domain: "Chip" },
        EngTarget { value: 96.0,    name: "Gaudi 2 HBM (GB)",       domain: "Chip" },
        EngTarget { value: 400.0,   name: "A100 TDP (W)",           domain: "Chip" },
        EngTarget { value: 1000.0,  name: "B200 TDP (W)",           domain: "Chip" },
        // Energy
        EngTarget { value: 120.0,   name: "H₂ LHV (MJ/kg)",        domain: "Energy" },
        EngTarget { value: 142.0,   name: "H₂ HHV (MJ/kg)",        domain: "Energy" },
        EngTarget { value: 1.333,   name: "SQ bandgap (eV)",        domain: "Energy" },
        EngTarget { value: 60.0,    name: "Grid frequency 60Hz",    domain: "Energy" },
        EngTarget { value: 50.0,    name: "Grid frequency 50Hz",    domain: "Energy" },
        EngTarget { value: 1.2,     name: "PUE target",             domain: "Energy" },
        EngTarget { value: 480.0,   name: "3-phase datacenter (V)", domain: "Energy" },
        EngTarget { value: 0.5926,  name: "Betz limit",             domain: "Energy" },
        // Physics
        EngTarget { value: 137.036, name: "1/α fine structure",      domain: "Physics" },
        EngTarget { value: 0.1190,  name: "α_s(M_Z)",               domain: "Physics" },
        EngTarget { value: 0.2308,  name: "sin²θ_W",                domain: "Physics" },
        EngTarget { value: 0.300,   name: "sin²θ₁₂ neutrino",       domain: "Physics" },
        EngTarget { value: 0.5714,  name: "sin²θ₂₃ neutrino",       domain: "Physics" },
        EngTarget { value: 0.0833,  name: "sin²(2θ₁₃) neutrino",    domain: "Physics" },
        EngTarget { value: 0.96429, name: "n_s spectral index",      domain: "Physics" },
        EngTarget { value: 0.6667,  name: "Koide Q",                domain: "Physics" },
        // Biology
        EngTarget { value: 64.0,    name: "Codons",                 domain: "Biology" },
        EngTarget { value: 20.0,    name: "Amino acids",            domain: "Biology" },
        EngTarget { value: 4.0,     name: "DNA bases",              domain: "Biology" },
        // Network
        EngTarget { value: 13.0,    name: "DNS root servers",       domain: "Network" },
        EngTarget { value: 7.0,     name: "OSI layers",             domain: "Network" },
        // Audio/Display
        EngTarget { value: 48000.0, name: "Audio 48kHz",            domain: "Audio" },
        EngTarget { value: 12.0,    name: "Semitones per octave",   domain: "Audio" },
        EngTarget { value: 24.0,    name: "Film 24fps / 24-bit",    domain: "Display" },
    ]
}

// ── Operators ────────────────────────────────────────────────────

/// COLLISION: Find constant values that appear in 3+ diverse domain categories
fn op_collision(constants: &[Constant]) -> Vec<Discovery> {
    // Group constants by approximate value (within 0.1%)
    let mut value_map: HashMap<i64, Vec<usize>> = HashMap::new();
    for (i, c) in constants.iter().enumerate() {
        // Quantize to 0.1% bucket
        let key = if c.value.abs() > 1e-10 {
            (c.value.ln().abs() * 10000.0).round() as i64 * if c.value < 0.0 { -1 } else { 1 }
        } else {
            0
        };
        value_map.entry(key).or_default().push(i);
    }

    let mut discoveries = Vec::new();
    let mut seen_values = HashSet::new();

    for (_key, indices) in &value_map {
        if indices.len() < 2 { continue; }

        // Merge all domains
        let mut all_domains: Vec<String> = Vec::new();
        let mut all_exprs: Vec<String> = Vec::new();
        let mut bt_set: HashSet<String> = HashSet::new();

        for &i in indices {
            for d in &constants[i].domains {
                if !all_domains.contains(d) { all_domains.push(d.clone()); }
            }
            all_exprs.push(constants[i].expr.clone());
            for bt in &constants[i].bt_refs { bt_set.insert(bt.clone()); }
        }

        let cats: HashSet<&str> = all_domains.iter().map(|d| categorize_domain(d)).collect();
        if cats.len() < 2 { continue; }

        let val = constants[indices[0]].value;
        let val_key = format!("{:.4}", val);
        if !seen_values.insert(val_key) { continue; }

        let div = diversity_score(&all_domains);
        let prec = 1.0; // exact match within bucket
        let nov = if bt_set.is_empty() { 1.0 } else { 0.3 };
        let score = div * prec * nov;

        if score > 0.01 {
            discoveries.push(Discovery {
                operator: "COLLISION".to_string(),
                score,
                description: format!(
                    "Value {:.6} appears in {} domain categories via: {}",
                    val, cats.len(), all_exprs.join(", ")
                ),
                domains: all_domains,
                formula: all_exprs.join(" = "),
                diversity: div,
                precision: prec,
                novelty: nov,
            });
        }
    }

    discoveries
}

/// INVERSE: Given engineering target values, find n=6 expressions within 0.1% tolerance
fn op_inverse(expressions: &[Expr], targets: &[EngTarget], known_bts: &HashSet<String>) -> Vec<Discovery> {
    let mut discoveries = Vec::new();

    for target in targets {
        let mut matches: Vec<(String, f64)> = Vec::new();

        for expr in expressions {
            if target.value.abs() < 1e-15 { continue; }
            let err = ((expr.value - target.value) / target.value).abs();
            if err < 0.001 {
                matches.push((expr.text.clone(), err));
            }
        }

        if matches.is_empty() { continue; }

        // Sort by precision
        matches.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        matches.truncate(5);

        let best_err = matches[0].1;
        let formula = matches.iter().map(|(e, _)| e.as_str()).collect::<Vec<_>>().join(", ");

        // Check if this is a known discovery
        let desc_lower = format!("{} {}", target.name, formula).to_lowercase();
        let is_known = known_bts.iter().any(|bt| desc_lower.contains(&bt.to_lowercase()));
        let nov = if is_known { 0.0 } else { 1.0 };

        let domains = vec![target.domain.to_string()];
        let div = diversity_score(&domains);
        let prec = 1.0 - best_err;

        let score = div * prec * nov;

        discoveries.push(Discovery {
            operator: "INVERSE".to_string(),
            score,
            description: format!(
                "{} = {} ← n6 expr: {} (err={:.4}%)",
                target.name, target.value,
                matches[0].0, best_err * 100.0
            ),
            domains,
            formula,
            diversity: div,
            precision: prec,
            novelty: nov,
        });
    }

    discoveries
}

/// COMPOSE: Cross-reference all depth-2 expressions against engineering targets
fn op_compose(expressions: &[Expr], targets: &[EngTarget], known_bts: &HashSet<String>) -> Vec<Discovery> {
    let mut discoveries = Vec::new();

    // For each expression, check if it matches any target
    // But also check multi-domain matches (expression matching targets in different domains)
    let mut expr_matches: HashMap<String, Vec<(&EngTarget, f64)>> = HashMap::new();

    for expr in expressions {
        for target in targets {
            if target.value.abs() < 1e-15 { continue; }
            let err = ((expr.value - target.value) / target.value).abs();
            if err < 0.001 {
                expr_matches.entry(expr.text.clone()).or_default().push((target, err));
            }
        }
    }

    for (expr_text, matched_targets) in &expr_matches {
        if matched_targets.len() < 2 { continue; }

        let mut domains: Vec<String> = Vec::new();
        let mut names: Vec<String> = Vec::new();
        let best_err = matched_targets.iter().map(|(_, e)| *e).fold(f64::MAX, f64::min);

        for (t, _) in matched_targets {
            if !domains.contains(&t.domain.to_string()) {
                domains.push(t.domain.to_string());
            }
            names.push(t.name.to_string());
        }

        let cats: HashSet<&str> = domains.iter().map(|d| categorize_domain(d)).collect();
        if cats.len() < 2 { continue; } // require cross-domain

        let desc_lower = format!("{} {}", expr_text, names.join(" ")).to_lowercase();
        let is_known = known_bts.iter().any(|bt| desc_lower.contains(&bt.to_lowercase()));
        let nov = if is_known { 0.2 } else { 1.0 };

        let div = diversity_score(&domains);
        let prec = 1.0 - best_err;
        let score = div * prec * nov;

        if score > 0.01 {
            discoveries.push(Discovery {
                operator: "COMPOSE".to_string(),
                score,
                description: format!(
                    "n6 expr '{}' = {:.6} matches {} targets across {} categories: {}",
                    expr_text,
                    matched_targets[0].0.value,
                    matched_targets.len(),
                    cats.len(),
                    names.join(", ")
                ),
                domains,
                formula: expr_text.clone(),
                diversity: div,
                precision: prec,
                novelty: nov,
            });
        }
    }

    discoveries
}

// ── Known BT Values (for novelty scoring) ────────────────────────

fn known_bt_values() -> HashSet<String> {
    let mut set = HashSet::new();
    // Key known mappings (value -> BT) to detect already-discovered connections
    let known = [
        "144=AD102", "132=H100", "8=LoRA", "8=KV", "20=Chinchilla",
        "0.9=Adam", "0.95=top-p", "1e-8=epsilon", "0.1=weight_decay",
        "1000=DDPM", "50=DDIM", "7.5=CFG", "120=LHV", "142=HHV",
        "48=gate_pitch", "60=grid", "50=grid", "1.2=PUE", "480=datacenter",
        "288=B300", "192=B100", "96=Gaudi", "80=A100", "40=A100",
        "137=fine_structure", "0.667=Koide", "64=codons", "20=amino",
        "24=Leech", "0.288=Mertens", "0.593=Betz",
    ];
    for k in &known { set.insert(k.to_string()); }
    set
}

// ── Output ───────────────────────────────────────────────────────

fn print_text(discoveries: &[Discovery], total_exprs: usize, total_matches: usize, elapsed_ms: u128) {
    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           N6 Discovery Engine — Results                         ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Expressions enumerated: {:>8}                               ║", total_exprs);
    println!("║  Discoveries found:      {:>8}                               ║", total_matches);
    println!("║  Time elapsed:           {:>5} ms                              ║", elapsed_ms);
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    println!("Top 50 Discoveries (ranked by score):");
    println!("{:<4} {:<9} {:<6} {:<5} {:<5} {:<5} {}", "#", "Operator", "Score", "Div", "Prec", "Nov", "Description");
    println!("{}", "-".repeat(120));

    for (i, d) in discoveries.iter().take(50).enumerate() {
        let desc_trunc = if d.description.chars().count() > 80 {
            let s: String = d.description.chars().take(77).collect();
            format!("{}...", s)
        } else {
            d.description.clone()
        };
        println!("{:<4} {:<9} {:<6.3} {:<5.2} {:<5.2} {:<5.2} {}",
            i + 1, d.operator, d.score, d.diversity, d.precision, d.novelty, desc_trunc);
    }
}

fn print_json(discoveries: &[Discovery], total_exprs: usize, total_matches: usize, elapsed_ms: u128) {
    println!("{{");
    println!("  \"stats\": {{");
    println!("    \"expressions_enumerated\": {},", total_exprs);
    println!("    \"discoveries_found\": {},", total_matches);
    println!("    \"elapsed_ms\": {}", elapsed_ms);
    println!("  }},");
    println!("  \"discoveries\": [");
    for (i, d) in discoveries.iter().take(50).enumerate() {
        let comma = if i < discoveries.len().min(50) - 1 { "," } else { "" };
        let domains_json: Vec<String> = d.domains.iter().map(|s| format!("\"{}\"", s.replace('"', "\\\""))).collect();
        println!("    {{");
        println!("      \"rank\": {},", i + 1);
        println!("      \"operator\": \"{}\",", d.operator);
        println!("      \"score\": {:.4},", d.score);
        println!("      \"diversity\": {:.4},", d.diversity);
        println!("      \"precision\": {:.4},", d.precision);
        println!("      \"novelty\": {:.4},", d.novelty);
        println!("      \"formula\": \"{}\",", d.formula.replace('"', "\\\""));
        println!("      \"description\": \"{}\",", d.description.replace('"', "\\\""));
        println!("      \"domains\": [{}]", domains_json.join(", "));
        println!("    }}{}", comma);
    }
    println!("  ]");
    println!("}}");
}

// ── Main ─────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.iter().any(|a| a == "--json");
    let start = Instant::now();

    // Resolve paths relative to the binary or use absolute
    let base = std::env::var("N6_ROOT").unwrap_or_else(|_| {
        // Try common locations
        let candidates = [
            "/Users/ghost/Dev/n6-architecture",
            ".",
            "..",
        ];
        for c in &candidates {
            let p = format!("{}/docs/atlas-constants.md", c);
            if std::path::Path::new(&p).exists() { return c.to_string(); }
        }
        ".".to_string()
    });

    let atlas_path = format!("{}/docs/atlas-constants.md", base);
    let dse_path = format!("{}/docs/dse-map.toml", base);

    // Parse inputs
    let constants = parse_atlas_constants(&atlas_path);
    let dse_domains = parse_dse_map(&dse_path);

    if !json_mode {
        eprintln!("Parsed {} constants from atlas-constants.md", constants.len());
        eprintln!("Parsed {} DSE domains from dse-map.toml", dse_domains.len());
    }

    // Enumerate expressions
    let expressions = enumerate_depth2();
    let total_exprs = expressions.len();

    if !json_mode {
        eprintln!("Enumerated {} depth-2 expressions", total_exprs);
    }

    // Load known BTs
    let known_bts = known_bt_values();

    // Run operators
    let targets = engineering_targets();
    let mut all_discoveries: Vec<Discovery> = Vec::new();

    let d1 = op_collision(&constants);
    let d2 = op_inverse(&expressions, &targets, &known_bts);
    let d3 = op_compose(&expressions, &targets, &known_bts);

    let counts = (d1.len(), d2.len(), d3.len());
    all_discoveries.extend(d1);
    all_discoveries.extend(d2);
    all_discoveries.extend(d3);

    // Sort by score descending
    all_discoveries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));

    let total_matches = all_discoveries.len();
    let elapsed = start.elapsed().as_millis();

    if !json_mode {
        eprintln!("COLLISION: {} | INVERSE: {} | COMPOSE: {}", counts.0, counts.1, counts.2);
        eprintln!();
    }

    // Output
    if json_mode {
        print_json(&all_discoveries, total_exprs, total_matches, elapsed);
    } else {
        print_text(&all_discoveries, total_exprs, total_matches, elapsed);
    }
}
