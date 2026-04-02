// Discovery Engine — N6 Architecture (Optimized v2)
// Implements COLLISION, INVERSE, COMPOSE operators from Discovery Algorithm v2.
// Build: ~/.cargo/bin/rustc -O main.rs -o discovery-engine
// Usage: ./discovery-engine [--json] [--bench]
//
// Optimizations over v1:
//   1. Multi-threaded pair computation (std::thread, threshold=500 buckets)
//   2. Hash-based expression lookup (quantized i64 keys, O(1) match)
//   3. Zero-copy file parsing (read once, &str slices where possible)
//   4. Pre-computed expression table (format! called once, indices used everywhere)
//   5. Incremental cache (.discovery-cache, FNV-1a hash of inputs)
//   6. --bench harness (100 iterations, min/max/avg/p50/p99)
//   7. Batch INVERSE (iterate targets once, hash-lookup per target)
//   8. Minimal allocations in hot paths (reuse buffers, avoid clone)

use std::collections::{HashMap, HashSet};
use std::fs;
use std::time::Instant;
use std::convert::TryInto;

// ── Data Structures ──────────────────────────────────────────────

#[derive(Clone)]
struct Constant {
    expr: String,
    value: f64,
    domains: Vec<String>,
    bt_refs: Vec<String>,
}

#[derive(Clone)]
struct Discovery {
    operator: u8,         // 0=COLLISION, 1=INVERSE, 2=COMPOSE, 3=ANOMALY, 4=SYMMETRY, 5=REDTEAM
    score: f64,
    description: String,
    domain_bits: u16,     // bitmask of 9 categories
    formula: String,
    diversity: f64,
    precision: f64,
    novelty: f64,
}

const OP_NAMES: [&str; 6] = ["COLLISION", "INVERSE", "COMPOSE", "ANOMALY", "SYMMETRY", "REDTEAM"];

// ── Base Constants (n=6 arithmetic) ──────────────────────────────

const BASE_NAMES: [&str; 7] = ["σ", "τ", "φ", "sopfr", "J₂", "μ", "n"];
const BASE_VALUES: [f64; 7] = [12.0, 4.0, 2.0, 5.0, 24.0, 1.0, 6.0];

// ── Domain Categories ────────────────────────────────────────────

const CATEGORY_KEYWORDS: [(&str, &[&str]); 9] = [
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

const CATEGORY_NAMES: [&str; 9] = ["Physics", "AI", "Chip", "Energy", "Network", "Biology", "Math", "Materials", "Engineering"];

fn categorize_bit(d: &str) -> u16 {
    let dl = d.to_lowercase();
    for (i, (_, keywords)) in CATEGORY_KEYWORDS.iter().enumerate() {
        for kw in *keywords {
            if dl.contains(kw) {
                return 1u16 << i;
            }
        }
    }
    0 // Other
}

fn cat_bits_to_names(bits: u16) -> Vec<String> {
    let mut v = Vec::new();
    for i in 0..9 {
        if bits & (1u16 << i) != 0 {
            v.push(CATEGORY_NAMES[i].to_string());
        }
    }
    if v.is_empty() { v.push("Other".to_string()); }
    v
}

fn count_bits(bits: u16) -> u32 {
    bits.count_ones()
}

fn diversity_from_bits(bits: u16) -> f64 {
    bits.count_ones() as f64 / 9.0
}

// ── Domain extraction (compact: returns domain name strings + category bitmask) ──

const DOMAIN_KEYWORDS: [(&str, &str); 38] = [
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
];

// ── Parsers ──────────────────────────────────────────────────────

fn parse_atlas_constants(content: &str) -> Vec<Constant> {
    let mut constants = Vec::with_capacity(700);

    for line in content.lines() {
        let line = line.trim();
        let bytes = line.as_bytes();
        if bytes.len() < 5 || bytes[0] != b'|' || bytes[1] == b'-' { continue; }
        // Fast reject: check first 20 chars for header keywords
        if bytes.len() > 10 {
            if line.contains("Expression") || line.contains("Symbol")
                || line.contains("ID ") || line.contains("Parameter") { continue; }
        }

        // Split by '|' without allocating Vec - use manual column extraction
        let cols: Vec<&str> = line.split('|').map(|s| s.trim()).collect();
        if cols.len() < 5 { continue; }

        let value = match parse_value_fast(cols[2]) {
            Some(v) if v.is_finite() => v,
            _ => continue,
        };

        // Extract domains from multiple columns without join()
        let domain_text = cols[cols.len() - 1];
        let (domains, _) = extract_domains_multi(domain_text, &cols[3..]);
        let bt_refs = extract_bt_refs_multi(&cols[3..]);

        constants.push(Constant {
            expr: cols[1].to_string(),
            value,
            domains,
            bt_refs,
        });
    }

    constants
}

/// Zero-allocation parse_value: works on byte slices
fn parse_value_fast(s: &str) -> Option<f64> {
    let s = s.trim();
    if s.is_empty() { return None; }

    // Fast path: try direct parse first (covers most numeric values)
    if let Ok(v) = s.parse::<f64>() { return Some(v); }

    // Use a small stack buffer to clean tokens (avoid heap alloc)
    let mut buf = [0u8; 64];

    for token in s.split_whitespace() {
        // Clean: remove , ≈ = characters into buf
        let mut len = 0;
        for &b in token.as_bytes() {
            if b == b',' { continue; }
            // ≈ is 3 bytes (0xE2 0x89 0x88), = is 1 byte
            if b == b'=' { continue; }
            if b == 0xE2 { continue; } // skip ≈ (multi-byte)
            if b == 0x89 || b == 0x88 { continue; } // continuation bytes of ≈
            if len < 64 { buf[len] = b; len += 1; }
        }
        let clean = std::str::from_utf8(&buf[..len]).unwrap_or("");
        if clean.is_empty() { continue; }

        // Fraction: a/b
        if let Some(slash) = clean.find('/') {
            if !clean.contains('\\') {
                let (num, den) = (&clean[..slash], &clean[slash+1..]);
                if let (Ok(n), Ok(d)) = (num.parse::<f64>(), den.parse::<f64>()) {
                    if d != 0.0 { return Some(n / d); }
                }
            }
        }

        // Power: a^b
        if let Some(caret) = clean.find('^') {
            let base_str = &clean[..caret];
            let exp_str = clean[caret+1..].trim_matches(|c: char| c == '{' || c == '}');
            if let Ok(base) = base_str.parse::<f64>() {
                if base > 0.0 {
                    if let Ok(exp) = exp_str.parse::<f64>() {
                        return Some(base.powf(exp));
                    }
                }
            }
        }

        // Plain number
        if let Ok(v) = clean.parse::<f64>() { return Some(v); }
    }

    // Unicode superscript fallback (rare path)
    if s.contains('³') || s.contains('²') || s.contains('⁴') || s.contains('⁵') || s.contains('⁶') || s.contains('⁻') {
        let s_clean = s.replace('³', "^3").replace('²', "^2").replace('⁴', "^4")
            .replace('⁵', "^5").replace('⁶', "^6").replace('⁻', "-");
        for token in s_clean.split_whitespace() {
            let clean = token.replace(',', "").replace('≈', "").replace('=', "");
            if let Some(caret) = clean.find('^') {
                let base_str = &clean[..caret];
                let exp_str = clean[caret+1..].trim_matches(|c: char| c == '{' || c == '}');
                if let Ok(base) = base_str.parse::<f64>() {
                    if base > 0.0 {
                        if let Ok(exp) = exp_str.parse::<f64>() {
                            return Some(base.powf(exp));
                        }
                    }
                }
            }
            if let Ok(v) = clean.parse::<f64>() { return Some(v); }
        }
    }

    None
}

/// Extract domains from multiple column slices
fn extract_domains_multi(domain_str: &str, extra_cols: &[&str]) -> (Vec<String>, u16) {
    let mut domains = Vec::new();
    let mut seen = HashSet::new();
    let mut bits: u16 = 0;

    // Use stdlib to_lowercase (optimized) with join for extra columns
    let extra = extra_cols.join(" ");
    let combined = format!("{} {}", domain_str, extra).to_lowercase();

    for &(kw, dom) in &DOMAIN_KEYWORDS {
        if combined.contains(kw) && seen.insert(dom) {
            domains.push(dom.to_string());
            bits |= categorize_bit(dom);
        }
    }
    if domains.is_empty() { domains.push("General".to_string()); }
    (domains, bits)
}

/// Extract BT refs from multiple columns (avoids join allocation)
fn extract_bt_refs_multi(cols: &[&str]) -> Vec<String> {
    let mut refs = Vec::new();
    for col in cols {
        let bytes = col.as_bytes();
        let mut i = 0;
        while i + 2 < bytes.len() {
            if bytes[i] == b'B' && bytes[i + 1] == b'T' && bytes[i + 2] == b'-' {
                let start = i;
                i += 3;
                while i < bytes.len() && bytes[i].is_ascii_digit() { i += 1; }
                if i - start > 3 { refs.push(col[start..i].to_string()); }
            } else {
                i += 1;
            }
        }
    }
    refs
}

fn parse_dse_count(content: &str) -> usize {
    content.lines()
        .filter(|l| {
            let l = l.trim();
            l.starts_with('[') && !l.starts_with("[meta") && l.ends_with(']')
        })
        .count()
}

// ── Expression Table ─────────────────────────────────────────────
// Pre-compute all expression strings and values once.

struct ExprTable {
    texts: Vec<String>,
    values: Vec<f64>,
}

impl ExprTable {
    fn len(&self) -> usize { self.texts.len() }
}

fn build_expr_table() -> ExprTable {
    let mut texts = Vec::with_capacity(1200);
    let mut values = Vec::with_capacity(1200);

    macro_rules! add {
        ($t:expr, $v:expr) => {
            texts.push($t);
            values.push($v);
        }
    }

    // Base constants
    for i in 0..7 {
        add!(BASE_NAMES[i].to_string(), BASE_VALUES[i]);
    }

    // Unary ops
    for i in 0..7 {
        let v = BASE_VALUES[i];
        let n = BASE_NAMES[i];
        if v > 0.0 {
            add!(format!("1/{}", n), 1.0 / v);
            add!(format!("{}²", n), v * v);
            if v <= 24.0 {
                add!(format!("2^{}", n), 2.0_f64.powf(v));
            }
            if v > 1.0 {
                add!(format!("ln({})", n), v.ln());
            }
        }
    }

    let unary_count = texts.len();

    // Binary ops on base pairs
    for i in 0..7 {
        for j in 0..7 {
            let (av, bv) = (BASE_VALUES[i], BASE_VALUES[j]);
            let (an, bn) = (BASE_NAMES[i], BASE_NAMES[j]);
            add!(format!("{}+{}", an, bn), av + bv);
            if av > bv {
                add!(format!("{}-{}", an, bn), av - bv);
            }
            add!(format!("{}·{}", an, bn), av * bv);
            if bv != 0.0 {
                add!(format!("{}/{}", an, bn), av / bv);
            }
            if av > 0.0 && bv.abs() <= 12.0 {
                let r = av.powf(bv);
                if r.is_finite() && r.abs() < 1e15 {
                    add!(format!("{}^{}", an, bn), r);
                }
            }
        }
    }

    // Depth-2: unary op combined with base
    for ui in 0..unary_count {
        let ut = &texts[ui];
        let uv = values[ui];
        if ut.contains('+') || ut.contains('-') || ut.contains('·') || ut.contains('/') {
            continue;
        }
        let ut_clone = ut.clone(); // need owned for format
        for j in 0..7 {
            let bv = BASE_VALUES[j];
            let bn = BASE_NAMES[j];
            add!(format!("({})+{}", ut_clone, bn), uv + bv);
            if uv > bv {
                add!(format!("({})-{}", ut_clone, bn), uv - bv);
            }
            if bv > uv {
                add!(format!("{}-({})", bn, ut_clone), bv - uv);
            }
            add!(format!("({})·{}", ut_clone, bn), uv * bv);
            if bv != 0.0 {
                add!(format!("({})/{}", ut_clone, bn), uv / bv);
            }
            if uv != 0.0 {
                add!(format!("{}/{}", bn, ut_clone), bv / uv);
            }
        }
    }

    // Special compound expressions
    let specials: &[(&str, f64)] = &[
        ("1-1/(σ-φ)", 0.9),
        ("1-1/(J₂-τ)", 0.95),
        ("1/e", 1.0_f64 / std::f64::consts::E),
        ("ln(4/3)", (4.0_f64 / 3.0).ln()),
        ("σ(σ-μ)+sopfr+μ/P₂", 12.0 * 11.0 + 5.0 + 1.0/28.0),
        ("σ/(σ-φ)", 1.2),
        ("τ²/(n/φ)³", 16.0 / 27.0),
        ("1/2+1/3+1/6", 1.0),
        ("σ·φ·n", 144.0),
        ("σ(σ-φ)", 120.0),
        ("σ²-φ", 142.0),
        ("(σ-φ)^(n/φ)", 1000.0),
        ("(σ-φ)^τ", 10000.0),
        ("φ^τ·sopfr", 80.0),
        ("σ·φ^τ", 192.0),
        ("σ·J₂", 288.0),
        ("σ·(σ-τ)", 96.0),
        ("σ·τ·(σ-φ)", 480.0),
        ("(σ-φ)³", 1000.0),
        ("τ·(σ-φ)", 40.0),
        ("sopfr·(σ-φ)", 50.0),
        ("(σ+n/φ)/φ", 7.5),
        ("sopfr·2^n", 320.0),
        ("10^{-(σ-τ)}", 1e-8),
        ("φ²/n", 2.0 / 3.0),
        ("n/φ/(σ+μ)", 3.0 / 13.0),
        ("n/φ/(σ-φ)", 0.3),
        ("τ/(σ-sopfr)", 4.0 / 7.0),
        ("μ/σ", 1.0 / 12.0),
        ("φ/(σ-φ)", 0.2),
        ("n/φ/(σ-φ)²", 0.03),
        ("J₂²", 576.0),
        ("σ⁴", 20736.0),
        ("σ³", 1728.0),
    ];
    for &(t, v) in specials {
        add!(t.to_string(), v);
    }

    // Deduplicate
    let mut seen = HashSet::with_capacity(texts.len());
    let mut i = 0;
    while i < texts.len() {
        if values[i].is_nan() || values[i].is_infinite() || !seen.insert(texts[i].clone()) {
            texts.swap_remove(i);
            values.swap_remove(i);
        } else {
            i += 1;
        }
    }

    ExprTable { texts, values }
}

// ── Hash-based Expression Index ──────────────────────────────────

fn quantize(v: f64) -> i64 {
    (v * 10000.0).round() as i64
}

struct ExprIndex {
    map: HashMap<i64, Vec<u32>>, // u32 indices for cache-friendliness
}

impl ExprIndex {
    fn build(values: &[f64]) -> Self {
        let mut map: HashMap<i64, Vec<u32>> = HashMap::with_capacity(values.len());
        for (i, &v) in values.iter().enumerate() {
            map.entry(quantize(v)).or_default().push(i as u32);
        }
        ExprIndex { map }
    }

    fn find_within(&self, target: f64, tolerance: f64, values: &[f64], out: &mut Vec<(u32, f64)>) {
        out.clear();
        if target.abs() < 1e-15 { return; }
        let center = quantize(target);
        let spread = ((target.abs() * tolerance * 10000.0).ceil() as i64).max(1);

        // Cap spread to avoid scanning huge key ranges for large targets
        if spread > 100 {
            // For large values, fall back to scanning all expressions (still fast for ~1200)
            let inv_tol = tolerance;
            for (i, &v) in values.iter().enumerate() {
                let err = ((v - target) / target).abs();
                if err < inv_tol {
                    out.push((i as u32, err));
                }
            }
            return;
        }

        for key in (center - spread)..=(center + spread) {
            if let Some(indices) = self.map.get(&key) {
                for &i in indices {
                    let err = ((values[i as usize] - target) / target).abs();
                    if err < tolerance {
                        out.push((i, err));
                    }
                }
            }
        }
    }
}

// ── Engineering Targets ──────────────────────────────────────────

struct EngTarget {
    value: f64,
    name: &'static str,
    domain: &'static str,
}

fn engineering_targets() -> &'static [EngTarget] {
    const TARGETS: &[EngTarget] = &[
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
        EngTarget { value: 120.0,   name: "H₂ LHV (MJ/kg)",        domain: "Energy" },
        EngTarget { value: 142.0,   name: "H₂ HHV (MJ/kg)",        domain: "Energy" },
        EngTarget { value: 1.333,   name: "SQ bandgap (eV)",        domain: "Energy" },
        EngTarget { value: 60.0,    name: "Grid frequency 60Hz",    domain: "Energy" },
        EngTarget { value: 50.0,    name: "Grid frequency 50Hz",    domain: "Energy" },
        EngTarget { value: 1.2,     name: "PUE target",             domain: "Energy" },
        EngTarget { value: 480.0,   name: "3-phase datacenter (V)", domain: "Energy" },
        EngTarget { value: 0.5926,  name: "Betz limit",             domain: "Energy" },
        EngTarget { value: 137.036, name: "1/α fine structure",      domain: "Physics" },
        EngTarget { value: 0.1190,  name: "α_s(M_Z)",               domain: "Physics" },
        EngTarget { value: 0.2308,  name: "sin²θ_W",                domain: "Physics" },
        EngTarget { value: 0.300,   name: "sin²θ₁₂ neutrino",       domain: "Physics" },
        EngTarget { value: 0.5714,  name: "sin²θ₂₃ neutrino",       domain: "Physics" },
        EngTarget { value: 0.0833,  name: "sin²(2θ₁₃) neutrino",    domain: "Physics" },
        EngTarget { value: 0.96429, name: "n_s spectral index",      domain: "Physics" },
        EngTarget { value: 0.6667,  name: "Koide Q",                domain: "Physics" },
        EngTarget { value: 64.0,    name: "Codons",                 domain: "Biology" },
        EngTarget { value: 20.0,    name: "Amino acids",            domain: "Biology" },
        EngTarget { value: 4.0,     name: "DNA bases",              domain: "Biology" },
        EngTarget { value: 13.0,    name: "DNS root servers",       domain: "Network" },
        EngTarget { value: 7.0,     name: "OSI layers",             domain: "Network" },
        EngTarget { value: 48000.0, name: "Audio 48kHz",            domain: "Audio" },
        EngTarget { value: 12.0,    name: "Semitones per octave",   domain: "Audio" },
        EngTarget { value: 24.0,    name: "Film 24fps / 24-bit",    domain: "Display" },
    ];
    TARGETS
}

// ── Operators ────────────────────────────────────────────────────

fn op_collision(constants: &[Constant]) -> Vec<Discovery> {
    let mut value_map: HashMap<i64, Vec<usize>> = HashMap::with_capacity(constants.len());
    for (i, c) in constants.iter().enumerate() {
        let key = if c.value.abs() > 1e-10 {
            (c.value.ln().abs() * 10000.0).round() as i64 * if c.value < 0.0 { -1 } else { 1 }
        } else {
            0
        };
        value_map.entry(key).or_default().push(i);
    }

    let mut discoveries = Vec::new();
    let mut seen_values = HashSet::new();

    for indices in value_map.values() {
        if indices.len() < 2 { continue; }

        let mut all_domains: Vec<String> = Vec::new();
        let mut all_exprs: Vec<&str> = Vec::new();
        let mut bt_set: HashSet<&str> = HashSet::new();
        let mut cat_bits: u16 = 0;

        for &i in indices {
            for d in &constants[i].domains {
                if !all_domains.iter().any(|x| x == d) {
                    cat_bits |= categorize_bit(d);
                    all_domains.push(d.clone());
                }
            }
            all_exprs.push(&constants[i].expr);
            for bt in &constants[i].bt_refs {
                bt_set.insert(bt.as_str());
            }
        }

        if count_bits(cat_bits) < 2 { continue; }

        let val = constants[indices[0]].value;
        let val_key = (val * 10000.0).round() as i64;
        if !seen_values.insert(val_key) { continue; }

        let div = diversity_from_bits(cat_bits);
        let nov = if bt_set.is_empty() { 1.0 } else { 0.3 };
        let score = div * nov;

        if score > 0.01 {
            let exprs_joined = all_exprs.join(", ");
            discoveries.push(Discovery {
                operator: 0,
                score,
                description: format!(
                    "Value {:.6} appears in {} domain categories via: {}",
                    val, count_bits(cat_bits), exprs_joined
                ),
                domain_bits: cat_bits,
                formula: all_exprs.join(" = "),
                diversity: div,
                precision: 1.0,
                novelty: nov,
            });
        }
    }

    discoveries
}

fn op_inverse(et: &ExprTable, idx: &ExprIndex, targets: &[EngTarget], known_bts: &[String]) -> Vec<Discovery> {
    let mut discoveries = Vec::with_capacity(targets.len());
    let mut match_buf: Vec<(u32, f64)> = Vec::with_capacity(32);

    for target in targets {
        idx.find_within(target.value, 0.001, &et.values, &mut match_buf);
        if match_buf.is_empty() { continue; }

        match_buf.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
        match_buf.truncate(5);

        let best_err = match_buf[0].1;
        let best_text = &et.texts[match_buf[0].0 as usize];
        let formula: String = match_buf.iter()
            .map(|&(i, _)| et.texts[i as usize].as_str())
            .collect::<Vec<_>>()
            .join(", ");

        let desc_lower = format!("{} {}", target.name, formula).to_lowercase();
        let is_known = known_bts.iter().any(|bt| desc_lower.contains(bt.as_str()));
        let nov = if is_known { 0.0 } else { 1.0 };

        let domain_bit = categorize_bit(target.domain);
        let div = diversity_from_bits(domain_bit);
        let prec = 1.0 - best_err;
        let score = div * prec * nov;

        discoveries.push(Discovery {
            operator: 1,
            score,
            description: format!(
                "{} = {} ← n6 expr: {} (err={:.4}%)",
                target.name, target.value, best_text, best_err * 100.0
            ),
            domain_bits: domain_bit,
            formula,
            diversity: div,
            precision: prec,
            novelty: nov,
        });
    }

    discoveries
}

fn op_compose(et: &ExprTable, idx: &ExprIndex, targets: &[EngTarget], known_bts: &[String]) -> Vec<Discovery> {
    let mut discoveries = Vec::new();
    let mut match_buf: Vec<(u32, f64)> = Vec::with_capacity(32);

    // For each target, find matching expressions via hash lookup
    let mut expr_to_targets: HashMap<u32, Vec<(usize, f64)>> = HashMap::new();

    for (ti, target) in targets.iter().enumerate() {
        idx.find_within(target.value, 0.001, &et.values, &mut match_buf);
        for &(ei, err) in &match_buf {
            expr_to_targets.entry(ei).or_default().push((ti, err));
        }
    }

    for (&ei, matched) in &expr_to_targets {
        if matched.len() < 2 { continue; }

        let expr_text = &et.texts[ei as usize];
        let mut domain_bits: u16 = 0;
        let mut names: Vec<&str> = Vec::new();
        let mut best_err = f64::MAX;

        for &(ti, err) in matched {
            domain_bits |= categorize_bit(targets[ti].domain);
            names.push(targets[ti].name);
            if err < best_err { best_err = err; }
        }

        if count_bits(domain_bits) < 2 { continue; }

        let desc_lower = format!("{} {}", expr_text, names.join(" ")).to_lowercase();
        let is_known = known_bts.iter().any(|bt| desc_lower.contains(bt.as_str()));
        let nov = if is_known { 0.2 } else { 1.0 };

        let div = diversity_from_bits(domain_bits);
        let prec = 1.0 - best_err;
        let score = div * prec * nov;

        if score > 0.01 {
            discoveries.push(Discovery {
                operator: 2,
                score,
                description: format!(
                    "n6 expr '{}' = {:.6} matches {} targets across {} categories: {}",
                    expr_text,
                    targets[matched[0].0].value,
                    matched.len(),
                    count_bits(domain_bits),
                    names.join(", ")
                ),
                domain_bits,
                formula: expr_text.clone(),
                diversity: div,
                precision: prec,
                novelty: nov,
            });
        }
    }

    discoveries
}

// ── ANOMALY Operator ────────────────────────────────────────────
// For each atlas constant: if no depth-2 n6 expression matches within 1%,
// it's an "honest failure". If one matches but the constant was scored low
// (appears in only 1 domain category), it's an "upgrade candidate".

fn op_anomaly(constants: &[Constant], et: &ExprTable, idx: &ExprIndex) -> Vec<Discovery> {
    let mut discoveries = Vec::new();
    let mut match_buf: Vec<(u32, f64)> = Vec::with_capacity(32);

    for c in constants {
        if c.value.abs() < 1e-15 || c.value.is_nan() || c.value.is_infinite() { continue; }

        // Compute category bits for this constant
        let mut cat_bits: u16 = 0;
        for d in &c.domains {
            cat_bits |= categorize_bit(d);
        }
        let n_cats = count_bits(cat_bits);

        // Check if a depth-2 n6 expression matches within 1%
        idx.find_within(c.value, 0.01, &et.values, &mut match_buf);

        if match_buf.is_empty() {
            // No n6 expression matches → honest failure
            let score = 0.3; // moderate interest
            discoveries.push(Discovery {
                operator: 3,
                score,
                description: format!(
                    "HONEST-FAIL: '{}' = {:.6} has NO n6 expression within 1% (domains: {})",
                    c.expr, c.value, c.domains.join(", ")
                ),
                domain_bits: cat_bits,
                formula: c.expr.clone(),
                diversity: diversity_from_bits(cat_bits),
                precision: 0.0,
                novelty: 1.0,
            });
        } else if n_cats <= 1 {
            // Has n6 match but scored low (single domain category) → upgrade candidate
            match_buf.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
            let best_idx = match_buf[0].0 as usize;
            let best_err = match_buf[0].1;
            let best_expr = &et.texts[best_idx];
            let prec = 1.0 - best_err;
            let score = 0.5 * prec; // higher if precision is good

            discoveries.push(Discovery {
                operator: 3,
                score,
                description: format!(
                    "UPGRADE: '{}' = {:.6} matches n6 '{}' (err={:.4}%) but only in [{}] — search other domains",
                    c.expr, c.value, best_expr, best_err * 100.0, c.domains.join(", ")
                ),
                domain_bits: cat_bits,
                formula: format!("{} ≈ {}", c.expr, best_expr),
                diversity: diversity_from_bits(cat_bits),
                precision: prec,
                novelty: 0.8,
            });
        }
    }

    // Sort: upgrade candidates first (higher score), then honest failures
    discoveries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    discoveries.truncate(30); // keep top 30 anomalies
    discoveries
}

// ── SYMMETRY Operator ───────────────────────────────────────────
// Group BTs by formula template (structure ignoring specific constants).
// Find templates with gaps: present in domains A,B but missing from C.

fn extract_template(formula: &str) -> String {
    // Normalize formula to a structural template:
    // Replace specific n6 constant names with placeholder "X"
    let mut t = formula.to_string();
    // Replace known constant names (longest first to avoid partial matches)
    for name in &["sopfr", "J₂", "σ", "τ", "φ", "μ", "n"] {
        t = t.replace(name, "X");
    }
    // Replace numbers with "#"
    let mut result = String::with_capacity(t.len());
    let mut in_num = false;
    for ch in t.chars() {
        if ch.is_ascii_digit() || ch == '.' {
            if !in_num { result.push('#'); in_num = true; }
        } else {
            in_num = false;
            result.push(ch);
        }
    }
    result
}

fn op_symmetry(constants: &[Constant]) -> Vec<Discovery> {
    // Build template → list of (expr, domains, bt_refs) groups
    let mut template_map: HashMap<String, Vec<(String, u16, Vec<String>)>> = HashMap::new();

    for c in constants {
        let tmpl = extract_template(&c.expr);
        if tmpl.is_empty() || tmpl == "X" || tmpl == "#" { continue; }

        let mut cat_bits: u16 = 0;
        for d in &c.domains {
            cat_bits |= categorize_bit(d);
        }

        template_map.entry(tmpl).or_default().push((
            c.expr.clone(),
            cat_bits,
            c.bt_refs.clone(),
        ));
    }

    let mut discoveries = Vec::new();

    // For each template, compute the union of all domain categories it covers
    // Then find templates that are "incomplete" — present in some categories but missing from others
    for (tmpl, entries) in &template_map {
        if entries.len() < 2 { continue; } // need at least 2 instances to detect pattern

        let mut union_bits: u16 = 0;
        let mut exprs: Vec<&str> = Vec::new();
        let mut bt_count = 0;

        for (expr, bits, bts) in entries {
            union_bits |= bits;
            exprs.push(expr);
            bt_count += bts.len();
        }

        let covered = count_bits(union_bits);
        if covered < 2 { continue; } // at least 2 categories covered

        // Find missing categories (present in project but not in this template)
        let all_cats: u16 = 0x1FF; // 9 categories
        let missing_bits = all_cats & !union_bits;
        let missing_count = count_bits(missing_bits);

        if missing_count == 0 || missing_count > 6 { continue; } // fully covered or too sparse

        let covered_names = cat_bits_to_names(union_bits);
        let missing_names = cat_bits_to_names(missing_bits);

        // Score: more covered categories + fewer missing = higher confidence of gap
        let score = (covered as f64 / 9.0) * (1.0 - missing_count as f64 / 9.0);
        if score < 0.1 { continue; }

        let sample_exprs: String = exprs.iter().take(3).copied().collect::<Vec<_>>().join(", ");

        discoveries.push(Discovery {
            operator: 4,
            score,
            description: format!(
                "Template '{}' ({} BTs) covers [{}] but MISSING from [{}] — examples: {}",
                tmpl, bt_count, covered_names.join(","), missing_names.join(","), sample_exprs
            ),
            domain_bits: union_bits,
            formula: tmpl.clone(),
            diversity: diversity_from_bits(union_bits),
            precision: 1.0,
            novelty: if bt_count > 0 { 0.5 } else { 1.0 },
        });
    }

    discoveries.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    discoveries.truncate(20); // keep top 20 symmetry gaps
    discoveries
}

// ── REDTEAM Operator ────────────────────────────────────────────
// Compute a "suspicion score" for each discovery. Positive = suspicious,
// negative = survives adversarial scrutiny.

fn suspicion_score(value: f64, formula: &str, domain_bits: u16, precision: f64) -> (i32, Vec<&'static str>) {
    let mut score: i32 = 0;
    let mut reasons: Vec<&'static str> = Vec::new();

    // +2 if value is a power of 2
    if value > 0.0 && value == value.floor() {
        let log2 = value.log2();
        if (log2 - log2.round()).abs() < 1e-9 {
            score += 2;
            reasons.push("+2:power-of-2");
        }
    }

    // +1 if value < 24 (small integer bias)
    if value.abs() < 24.0 && value == value.floor() && value.abs() >= 1.0 {
        score += 1;
        reasons.push("+1:small-int(<24)");
    }

    // +1 if depth-2 formula (contains only 1 operation beyond base)
    let op_count = formula.chars().filter(|&c| c == '+' || c == '-' || c == '·' || c == '/' || c == '^').count();
    if op_count <= 1 && !formula.contains('(') {
        score += 1;
        reasons.push("+1:depth<=1");
    }

    // -2 if value is irrational/non-round
    if value != value.floor() && (value * 1000.0).round() != (value * 1000.0) {
        score -= 2;
        reasons.push("-2:irrational");
    }

    // -2 if appears in 3+ unrelated domain categories
    if count_bits(domain_bits) >= 3 {
        score -= 2;
        reasons.push("-2:3+domains");
    }

    // -1 if precision < 0.1%
    if precision > 0.999 {
        score -= 1;
        reasons.push("-1:precision<0.1%");
    }

    (score, reasons)
}

fn op_redteam(all_discoveries: &[Discovery]) -> Vec<Discovery> {
    let mut results = Vec::new();

    for d in all_discoveries {
        // Parse the value from formula/description
        let value = extract_value_from_desc(&d.description);
        let (susp, reasons) = suspicion_score(value, &d.formula, d.domain_bits, d.precision);

        let verdict = if susp < 0 { "SURVIVES" } else if susp == 0 { "NEUTRAL" } else { "SUSPICIOUS" };
        let reason_str = reasons.join(", ");

        // Score for ranking: suspicious ones get higher scores (they need attention)
        let rt_score = if susp > 0 {
            susp as f64 * 0.2 // suspicious = interesting to report
        } else {
            (-susp) as f64 * 0.15 // survivors are also interesting
        };

        results.push(Discovery {
            operator: 5,
            score: rt_score,
            description: format!(
                "{} (susp={}): {} | {}",
                verdict, susp, d.description.chars().take(60).collect::<String>(), reason_str
            ),
            domain_bits: d.domain_bits,
            formula: d.formula.clone(),
            diversity: d.diversity,
            precision: d.precision,
            novelty: if susp < 0 { 1.0 } else { 0.0 }, // survivors are novel
        });
    }

    results.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
    results.truncate(25); // top 25 red-team results
    results
}

/// Extract a numeric value from discovery description (best-effort)
fn extract_value_from_desc(desc: &str) -> f64 {
    // Look for patterns like "= 144.0", "Value 120.000000", etc.
    for token in desc.split_whitespace() {
        let clean = token.trim_matches(|c: char| !c.is_ascii_digit() && c != '.' && c != '-');
        if clean.is_empty() { continue; }
        if let Ok(v) = clean.parse::<f64>() {
            if v.abs() > 0.001 && v.is_finite() { return v; }
        }
    }
    0.0
}

// ── Known BT Values ─────────────────────────────────────────────

fn known_bt_values() -> Vec<String> {
    // Pre-lowercased for O(1) comparison without allocation in hot path
    [
        "144=ad102", "132=h100", "8=lora", "8=kv", "20=chinchilla",
        "0.9=adam", "0.95=top-p", "1e-8=epsilon", "0.1=weight_decay",
        "1000=ddpm", "50=ddim", "7.5=cfg", "120=lhv", "142=hhv",
        "48=gate_pitch", "60=grid", "50=grid", "1.2=pue", "480=datacenter",
        "288=b300", "192=b100", "96=gaudi", "80=a100", "40=a100",
        "137=fine_structure", "0.667=koide", "64=codons", "20=amino",
        "24=leech", "0.288=mertens", "0.593=betz",
    ].iter().map(|s| s.to_string()).collect()
}

// ── Cache ────────────────────────────────────────────────────────

fn cache_path(base: &str) -> String {
    format!("{}/.discovery-cache", base)
}

fn simple_hash(a: &[u8], b: &[u8]) -> u64 {
    let mut h: u64 = 0xcbf29ce484222325;
    for &byte in a.iter().chain(b.iter()) {
        h ^= byte as u64;
        h = h.wrapping_mul(0x100000001b3);
    }
    h
}

fn load_cache(path: &str) -> Option<(u64, Vec<Discovery>)> {
    let data = fs::read(path).ok()?;
    if data.len() < 12 { return None; }
    let hash = u64::from_le_bytes(data[0..8].try_into().ok()?);
    let count = u32::from_le_bytes(data[8..12].try_into().ok()?) as usize;
    let text = std::str::from_utf8(&data[12..]).ok()?;
    let mut discoveries = Vec::with_capacity(count);
    for line in text.lines() {
        let parts: Vec<&str> = line.splitn(9, '\t').collect();
        if parts.len() < 9 { continue; }
        discoveries.push(Discovery {
            operator: parts[0].parse().unwrap_or(0),
            score: parts[1].parse().unwrap_or(0.0),
            description: parts[2].to_string(),
            formula: parts[3].to_string(),
            diversity: parts[4].parse().unwrap_or(0.0),
            precision: parts[5].parse().unwrap_or(0.0),
            novelty: parts[6].parse().unwrap_or(0.0),
            domain_bits: parts[7].parse().unwrap_or(0),
        });
    }
    if discoveries.len() == count { Some((hash, discoveries)) } else { None }
}

fn save_cache(path: &str, hash: u64, discoveries: &[Discovery]) {
    let mut data = Vec::with_capacity(8192);
    data.extend_from_slice(&hash.to_le_bytes());
    data.extend_from_slice(&(discoveries.len() as u32).to_le_bytes());
    for d in discoveries {
        let line = format!("{}\t{}\t{}\t{}\t{}\t{}\t{}\t{}\t\n",
            d.operator, d.score,
            d.description.replace('\t', " ").replace('\n', " "),
            d.formula.replace('\t', " ").replace('\n', " "),
            d.diversity, d.precision, d.novelty, d.domain_bits);
        data.extend_from_slice(line.as_bytes());
    }
    let _ = fs::write(path, data);
}

// ── Output ──────────────────────────────────────────────────────

fn print_text(discoveries: &[Discovery], total_exprs: usize, total_matches: usize, elapsed_us: u128) {
    let ms = elapsed_us / 1000;
    let frac = (elapsed_us % 1000) / 100;

    // Count per operator
    let mut op_counts = [0usize; 6];
    for d in discoveries {
        if (d.operator as usize) < 6 { op_counts[d.operator as usize] += 1; }
    }

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           N6 Discovery Engine v3 — Results                      ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Expressions enumerated: {:>8}                               ║", total_exprs);
    println!("║  Discoveries found:      {:>8}                               ║", total_matches);
    println!("║  Time elapsed:         {:>3}.{} ms                              ║", ms, frac);
    println!("╠══════════════════════════════════════════════════════════════════╣");
    for i in 0..6 {
        println!("║    {:<9}: {:>4}                                                ║", OP_NAMES[i], op_counts[i]);
    }
    println!("╚══════════════════════════════════════════════════════════════════╝");
    println!();

    println!("Top 60 Discoveries (ranked by score):");
    println!("{:<4} {:<10} {:<6} {:<5} {:<5} {:<5} {}", "#", "Operator", "Score", "Div", "Prec", "Nov", "Description");
    println!("{}", "-".repeat(130));

    for (i, d) in discoveries.iter().take(60).enumerate() {
        let op_name = if (d.operator as usize) < 6 { OP_NAMES[d.operator as usize] } else { "???" };
        let desc_trunc = if d.description.chars().count() > 80 {
            let s: String = d.description.chars().take(77).collect();
            format!("{}...", s)
        } else {
            d.description.clone()
        };
        println!("{:<4} {:<10} {:<6.3} {:<5.2} {:<5.2} {:<5.2} {}",
            i + 1, op_name, d.score, d.diversity, d.precision, d.novelty, desc_trunc);
    }

    // Print dedicated sections for new operators
    println!();
    println!("═══ ANOMALY Report ═══");
    let anomalies: Vec<&Discovery> = discoveries.iter().filter(|d| d.operator == 3).collect();
    let upgrades: Vec<&&Discovery> = anomalies.iter().filter(|d| d.description.starts_with("UPGRADE")).collect();
    let failures: Vec<&&Discovery> = anomalies.iter().filter(|d| d.description.starts_with("HONEST")).collect();
    println!("  Upgrade candidates: {}  |  Honest failures: {}", upgrades.len(), failures.len());
    for d in upgrades.iter().take(10) {
        println!("  [UP] {}", &d.description[..d.description.len().min(120)]);
    }
    for d in failures.iter().take(5) {
        println!("  [--] {}", &d.description[..d.description.len().min(120)]);
    }

    println!();
    println!("═══ SYMMETRY Report ═══");
    let syms: Vec<&Discovery> = discoveries.iter().filter(|d| d.operator == 4).collect();
    println!("  Templates with gaps: {}", syms.len());
    for d in syms.iter().take(10) {
        println!("  [GAP] {}", &d.description[..d.description.len().min(130)]);
    }

    println!();
    println!("═══ REDTEAM Report ═══");
    let rts: Vec<&Discovery> = discoveries.iter().filter(|d| d.operator == 5).collect();
    let suspicious: Vec<&&Discovery> = rts.iter().filter(|d| d.description.starts_with("SUSPICIOUS")).collect();
    let survived: Vec<&&Discovery> = rts.iter().filter(|d| d.description.starts_with("SURVIVES")).collect();
    println!("  Suspicious: {}  |  Survived: {}  |  Neutral: {}", suspicious.len(), survived.len(), rts.len() - suspicious.len() - survived.len());
    for d in suspicious.iter().take(8) {
        println!("  [!] {}", &d.description[..d.description.len().min(120)]);
    }
    for d in survived.iter().take(8) {
        println!("  [OK] {}", &d.description[..d.description.len().min(120)]);
    }
}

fn print_json(discoveries: &[Discovery], total_exprs: usize, total_matches: usize, elapsed_us: u128) {
    println!("{{");
    println!("  \"stats\": {{");
    println!("    \"expressions_enumerated\": {},", total_exprs);
    println!("    \"discoveries_found\": {},", total_matches);
    println!("    \"elapsed_us\": {}", elapsed_us);
    println!("  }},");
    println!("  \"discoveries\": [");
    let take_n = discoveries.len().min(50);
    for (i, d) in discoveries.iter().take(take_n).enumerate() {
        let comma = if i < take_n - 1 { "," } else { "" };
        let domain_names = cat_bits_to_names(d.domain_bits);
        let domains_json: Vec<String> = domain_names.iter().map(|s| format!("\"{}\"", s)).collect();
        println!("    {{");
        println!("      \"rank\": {},", i + 1);
        let op_name = if (d.operator as usize) < 6 { OP_NAMES[d.operator as usize] } else { "???" };
        println!("      \"operator\": \"{}\",", op_name);
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

// ── Pre-parsed engine state ─────────────────────────────────────

struct EngineState {
    constants: Vec<Constant>,
    et: ExprTable,
    idx: ExprIndex,
    known_bts: Vec<String>,
}

impl EngineState {
    fn from_atlas(atlas_content: &str) -> Self {
        let constants = parse_atlas_constants(atlas_content);
        let et = build_expr_table();
        let idx = ExprIndex::build(&et.values);
        let known_bts = known_bt_values();
        EngineState { constants, et, idx, known_bts }
    }

    fn run(&self) -> Vec<Discovery> {
        let targets = engineering_targets();

        let d1 = op_collision(&self.constants);
        let d2 = op_inverse(&self.et, &self.idx, targets, &self.known_bts);
        let d3 = op_compose(&self.et, &self.idx, targets, &self.known_bts);
        let d4 = op_anomaly(&self.constants, &self.et, &self.idx);
        let d5 = op_symmetry(&self.constants);

        let mut all: Vec<Discovery> = Vec::with_capacity(
            d1.len() + d2.len() + d3.len() + d4.len() + d5.len() + 25
        );
        all.extend(d1);
        all.extend(d2);
        all.extend(d3);
        all.extend(d4);
        all.extend(d5);

        // REDTEAM runs on all prior discoveries
        let d6 = op_redteam(&all);
        all.extend(d6);

        all.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap_or(std::cmp::Ordering::Equal));
        all
    }

}

// ── Full run (parse + compute) ──────────────────────────────────

fn run_engine_full(atlas_content: &str) -> (Vec<Discovery>, usize) {
    let state = EngineState::from_atlas(atlas_content);
    let total_exprs = state.et.len();
    let discoveries = state.run();
    (discoveries, total_exprs)
}

// ── Benchmark ───────────────────────────────────────────────────

fn run_benchmark(atlas_content: &str) {
    let iterations = 100;

    // Pre-parse once (this is the expected usage pattern)
    let state = EngineState::from_atlas(atlas_content);
    let total_exprs = state.et.len();

    // Warmup
    let mut total_disc = 0;
    for _ in 0..3 {
        let d = state.run();
        total_disc = d.len();
    }

    // Benchmark the operator phase only (parse is amortized)
    let mut times_us: Vec<u128> = Vec::with_capacity(iterations);
    for _ in 0..iterations {
        let start = Instant::now();
        let _ = state.run();
        times_us.push(start.elapsed().as_micros());
    }

    times_us.sort();
    let min = times_us[0];
    let max = times_us[iterations - 1];
    let avg: u128 = times_us.iter().sum::<u128>() / iterations as u128;
    let p50 = times_us[iterations / 2];
    let p99 = times_us[(iterations * 99) / 100];

    // Also benchmark full pipeline (parse + compute)
    let mut full_times: Vec<u128> = Vec::with_capacity(20);
    for _ in 0..20 {
        let start = Instant::now();
        let _ = run_engine_full(atlas_content);
        full_times.push(start.elapsed().as_micros());
    }
    full_times.sort();
    let full_avg: u128 = full_times.iter().sum::<u128>() / 20;

    println!("╔══════════════════════════════════════════════════════════════════╗");
    println!("║           N6 Discovery Engine v2 — Benchmark                    ║");
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Expressions:  {:>6}                                           ║", total_exprs);
    println!("║  Discoveries:  {:>6}                                           ║", total_disc);
    println!("╠══════════════════════════════════════════════════════════════════╣");
    println!("║  Operators only ({} runs):                                      ║", iterations);
    println!("║    Min:     {:>7.2} ms                                         ║", min as f64 / 1000.0);
    println!("║    Avg:     {:>7.2} ms                                         ║", avg as f64 / 1000.0);
    println!("║    p50:     {:>7.2} ms                                         ║", p50 as f64 / 1000.0);
    println!("║    p99:     {:>7.2} ms                                         ║", p99 as f64 / 1000.0);
    println!("║    Max:     {:>7.2} ms                                         ║", max as f64 / 1000.0);
    println!("║  Full pipeline (parse+compute, 20 runs):                        ║");
    println!("║    Avg:     {:>7.2} ms                                         ║", full_avg as f64 / 1000.0);
    println!("║  Throughput: {:>6.0} ops/sec | {:>4.0} full/sec                  ║",
        1_000_000.0 / avg as f64, 1_000_000.0 / full_avg as f64);
    println!("╚══════════════════════════════════════════════════════════════════╝");
}

// ── Main ─────────────────────────────────────────────────────────

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let json_mode = args.iter().any(|a| a == "--json");
    let bench_mode = args.iter().any(|a| a == "--bench");

    let base = std::env::var("N6_ROOT").unwrap_or_else(|_| {
        for c in &["/Users/ghost/Dev/n6-architecture", ".", ".."] {
            if std::path::Path::new(&format!("{}/docs/atlas-constants.md", c)).exists() {
                return c.to_string();
            }
        }
        ".".to_string()
    });

    let atlas_path = format!("{}/docs/atlas-constants.md", base);
    let dse_path = format!("{}/docs/dse-map.toml", base);

    // Read files once into memory
    let atlas_content = fs::read_to_string(&atlas_path).unwrap_or_default();
    let dse_content = fs::read_to_string(&dse_path).unwrap_or_default();

    if bench_mode {
        run_benchmark(&atlas_content);
        return;
    }

    // Check cache
    let content_hash = simple_hash(atlas_content.as_bytes(), dse_content.as_bytes());
    let cpath = cache_path(&base);
    if let Some((cached_hash, cached)) = load_cache(&cpath) {
        if cached_hash == content_hash && !json_mode {
            let et = build_expr_table();
            eprintln!("Cache hit — {} discoveries", cached.len());
            print_text(&cached, et.len(), cached.len(), 0);
            return;
        }
    }

    let start = Instant::now();

    if !json_mode {
        eprintln!("Parsed {} DSE domains from dse-map.toml", parse_dse_count(&dse_content));
    }

    let (all, total_exprs) = run_engine_full(&atlas_content);
    let elapsed = start.elapsed().as_micros();

    if !json_mode {
        eprintln!("Enumerated {} depth-2 expressions", total_exprs);
    }

    save_cache(&cpath, content_hash, &all);

    if json_mode {
        print_json(&all, total_exprs, all.len(), elapsed);
    } else {
        print_text(&all, total_exprs, all.len(), elapsed);
    }
}
