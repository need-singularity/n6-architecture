/// N6 GPU/HBM Architecture Calculator
/// ====================================
/// Verifies and predicts AI accelerator architecture constants
/// from n=6 arithmetic: σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5, J₂(6)=24, μ(6)=1
///
/// Build: rustc main.rs -o gpu-arch-calc
/// Usage: ./gpu-arch-calc [--predict] [--verify] [--hbm] [--all]

use std::env;

// ─── N=6 Constants ───
const SIGMA: u64 = 12;
const TAU: u64 = 4;
const PHI: u64 = 2;
const SOPFR: u64 = 5;
const J2: u64 = 24;
const MU: i64 = 1;
const N: u64 = 6;
const P2: u64 = 28; // second perfect number

// ─── GPU Architecture Database ───

#[derive(Clone)]
struct GpuArch {
    name: &'static str,
    gen: &'static str,
    year: u16,
    full_die_sms: u64,
    enabled_sms: u64,
    gpcs: u64,
    tpcs_per_gpc: u64,
    sms_per_tpc: u64,
    cuda_per_sm: u64,
    tc_per_sm: u64,
    hbm_stacks: u64,
    vram_gb: u64,
    nvlink_links: u64,
    tdp_w: u64,
}

fn gpu_database() -> Vec<GpuArch> {
    vec![
        GpuArch {
            name: "GV100", gen: "Volta", year: 2017,
            full_die_sms: 84, enabled_sms: 80,
            gpcs: 6, tpcs_per_gpc: 7, sms_per_tpc: 2,
            cuda_per_sm: 64, tc_per_sm: 8,
            hbm_stacks: 4, vram_gb: 32, nvlink_links: 6, tdp_w: 300,
        },
        GpuArch {
            name: "TU102", gen: "Turing", year: 2018,
            full_die_sms: 72, enabled_sms: 68,
            gpcs: 6, tpcs_per_gpc: 6, sms_per_tpc: 2,
            cuda_per_sm: 64, tc_per_sm: 8,
            hbm_stacks: 0, vram_gb: 24, nvlink_links: 0, tdp_w: 250,
        },
        GpuArch {
            name: "GA100", gen: "Ampere", year: 2020,
            full_die_sms: 128, enabled_sms: 108,
            gpcs: 8, tpcs_per_gpc: 8, sms_per_tpc: 2,
            cuda_per_sm: 64, tc_per_sm: 4,
            hbm_stacks: 5, vram_gb: 80, nvlink_links: 12, tdp_w: 400,
        },
        GpuArch {
            name: "AD102", gen: "Ada Lovelace", year: 2022,
            full_die_sms: 144, enabled_sms: 128,
            gpcs: 12, tpcs_per_gpc: 6, sms_per_tpc: 2,
            cuda_per_sm: 128, tc_per_sm: 4,
            hbm_stacks: 0, vram_gb: 24, nvlink_links: 0, tdp_w: 450,
        },
        GpuArch {
            name: "GH100", gen: "Hopper", year: 2022,
            full_die_sms: 144, enabled_sms: 132,
            gpcs: 8, tpcs_per_gpc: 9, sms_per_tpc: 2,
            cuda_per_sm: 128, tc_per_sm: 4,
            hbm_stacks: 5, vram_gb: 80, nvlink_links: 18, tdp_w: 700,
        },
        GpuArch {
            name: "GB202", gen: "Blackwell", year: 2024,
            full_die_sms: 192, enabled_sms: 192,
            gpcs: 12, tpcs_per_gpc: 8, sms_per_tpc: 2,
            cuda_per_sm: 128, tc_per_sm: 4,
            hbm_stacks: 6, vram_gb: 192, nvlink_links: 18, tdp_w: 1000,
        },
    ]
}

// ─── HBM Database ───

struct HbmGen {
    name: &'static str,
    year: u16,
    stack_height: u64,
    channels: u64,
    bits_per_channel: u64,
    pin_gbps: f64,
}

fn hbm_database() -> Vec<HbmGen> {
    vec![
        HbmGen { name: "HBM1", year: 2013, stack_height: 4, channels: 8, bits_per_channel: 128, pin_gbps: 1.0 },
        HbmGen { name: "HBM2", year: 2016, stack_height: 8, channels: 8, bits_per_channel: 128, pin_gbps: 2.0 },
        HbmGen { name: "HBM2e", year: 2020, stack_height: 8, channels: 8, bits_per_channel: 128, pin_gbps: 3.6 },
        HbmGen { name: "HBM3", year: 2022, stack_height: 12, channels: 16, bits_per_channel: 64, pin_gbps: 6.4 },
        HbmGen { name: "HBM3e", year: 2024, stack_height: 12, channels: 16, bits_per_channel: 64, pin_gbps: 9.6 },
        HbmGen { name: "HBM4", year: 2025, stack_height: 16, channels: 16, bits_per_channel: 128, pin_gbps: 12.0 },
    ]
}

// ─── Multi-Vendor Chip Database (BT-69 extension) ───

struct ChipArch {
    name: &'static str,
    vendor: &'static str,
    year: u16,
    params: Vec<(&'static str, u64, &'static str)>, // (name, value, n6_formula)
}

fn multivendor_database() -> Vec<ChipArch> {
    vec![
        ChipArch {
            name: "R100 (predicted)", vendor: "NVIDIA", year: 2027,
            params: vec![
                ("SMs", 224, "2^sopfr × (σ-sopfr) = 32×7"),
                ("VRAM (GB)", 288, "σ×J₂ = 12×24"),
                ("HBM stacks", 8, "σ-τ"),
            ],
        },
        ChipArch {
            name: "MI400 (predicted)", vendor: "AMD", year: 2026,
            params: vec![
                ("VRAM (GB)", 432, "σ²×(n/φ) = 144×3"),
            ],
        },
        ChipArch {
            name: "TPU v7 Ironwood", vendor: "Google", year: 2025,
            params: vec![
                ("VRAM (GB)", 192, "σ×φ^τ = 12×16"),
                ("HBM stacks", 8, "σ-τ"),
                ("TensorCores", 2, "φ"),
                ("SparseCores", 4, "τ"),
                ("Pod chips", 256, "2^(σ-τ)"),
            ],
        },
        ChipArch {
            name: "Trainium3 (predicted)", vendor: "AWS", year: 2026,
            params: vec![
                ("VRAM (GB)", 144, "σ²"),
            ],
        },
        ChipArch {
            name: "Clearwater Forest", vendor: "Intel", year: 2025,
            params: vec![
                ("E-cores", 288, "σ×J₂ = 12×24"),
            ],
        },
        ChipArch {
            name: "M5 (predicted)", vendor: "Apple", year: 2026,
            params: vec![
                ("CPU cores", 10, "σ-φ"),
                ("P-cores", 4, "τ"),
                ("E-cores", 6, "n"),
                ("GPU cores", 10, "σ-φ"),
            ],
        },
    ]
}

// ─── Interconnect / Memory Standard Database ───

struct InterconnectStd {
    name: &'static str,
    year: u16,
    params: Vec<(&'static str, u64, &'static str)>,
}

fn interconnect_database() -> Vec<InterconnectStd> {
    vec![
        InterconnectStd {
            name: "HBM4 JEDEC", year: 2025,
            params: vec![
                ("Interface bits", 2048, "2^(σ-μ) = 2^11"),
                ("Channels", 32, "2^sopfr = 2^5"),
                ("Pin Gb/s", 8, "σ-τ"),
            ],
        },
        InterconnectStd {
            name: "UCIe 3.0 (predicted)", year: 2026,
            params: vec![
                ("GT/s (standard)", 48, "σ×τ = 12×4"),
                ("GT/s (advanced)", 64, "2^n = 2^6"),
            ],
        },
        InterconnectStd {
            name: "CXL 4.0 (predicted)", year: 2026,
            params: vec![
                ("GT/s", 128, "2^(σ-sopfr) = 2^7"),
            ],
        },
    ]
}

// ─── N6 Expression Finder ───

struct N6Match {
    value: u64,
    expr: String,
    exact: bool,
}

fn find_n6_expression(target: u64) -> Option<N6Match> {
    // Direct constants
    let constants: Vec<(u64, &str)> = vec![
        (1, "μ"), (2, "φ"), (3, "n/φ"), (4, "τ"), (5, "sopfr"),
        (6, "n"), (7, "σ-sopfr"), (8, "σ-τ"), (10, "σ-φ"),
        (11, "σ-μ"), (12, "σ"), (13, "σ+μ"), (16, "2^τ"),
        (20, "J₂-τ"), (24, "J₂"), (28, "P₂"), (32, "2^sopfr"),
        (48, "σ·τ"), (64, "2^n"), (72, "σ·n"), (128, "2^(σ-sopfr)"),
        (132, "σ(σ-μ)"), (144, "σ²"), (192, "σ·2^τ"),
        (224, "2^sopfr·(σ-sopfr)"), (240, "σ·(J₂-τ)"),
        (256, "2^(σ-τ)"), (288, "σ·J₂"),
        (384, "σ·2^sopfr"), (432, "σ²·(n/φ)"),
        (768, "σ·2^n"), (784, "P₂²"),
        (1024, "2^(σ-φ)"), (2048, "2^(σ-μ)"), (4096, "2^σ"),
        (12288, "σ·2^(σ-φ)"), (28672, "P₂·2^(σ-φ)"),
    ];

    for (val, expr) in &constants {
        if *val == target {
            return Some(N6Match { value: target, expr: expr.to_string(), exact: true });
        }
    }

    // Try σ * k
    if target % SIGMA == 0 {
        let k = target / SIGMA;
        if let Some(inner) = find_simple(k) {
            return Some(N6Match { value: target, expr: format!("σ·{} = σ·{}", inner.expr, k), exact: true });
        }
    }

    // Try J₂ * k
    if target % J2 == 0 {
        let k = target / J2;
        if let Some(inner) = find_simple(k) {
            return Some(N6Match { value: target, expr: format!("J₂·{} = J₂·{}", inner.expr, k), exact: true });
        }
    }

    // Try 2^k
    if target > 0 && (target & (target - 1)) == 0 {
        let exp = (target as f64).log2() as u64;
        if let Some(inner) = find_simple(exp) {
            return Some(N6Match { value: target, expr: format!("2^{} = 2^{}", inner.expr, exp), exact: true });
        }
    }

    None
}

fn find_simple(target: u64) -> Option<N6Match> {
    let simple: Vec<(u64, &str)> = vec![
        (1, "μ"), (2, "φ"), (3, "n/φ"), (4, "τ"), (5, "sopfr"),
        (6, "n"), (7, "σ-sopfr"), (8, "σ-τ"), (10, "σ-φ"),
        (11, "σ-μ"), (12, "σ"), (13, "σ+μ"), (16, "2^τ"),
        (20, "J₂-τ"), (24, "J₂"), (28, "P₂"), (32, "2^sopfr"),
    ];
    for (val, expr) in &simple {
        if *val == target {
            return Some(N6Match { value: target, expr: expr.to_string(), exact: true });
        }
    }
    None
}

// ─── Verification Engine ───

fn verify_gpu(gpu: &GpuArch) {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  {} ({}, {})                          ", gpu.name, gpu.gen, gpu.year);
    println!("╠══════════════════════════════════════════════════════════════╣");

    let checks: Vec<(&str, u64)> = vec![
        ("Full die SMs", gpu.full_die_sms),
        ("Enabled SMs", gpu.enabled_sms),
        ("GPCs", gpu.gpcs),
        ("TPCs/GPC", gpu.tpcs_per_gpc),
        ("SMs/TPC", gpu.sms_per_tpc),
        ("CUDA/SM", gpu.cuda_per_sm),
        ("TC/SM", gpu.tc_per_sm),
        ("HBM stacks", gpu.hbm_stacks),
        ("VRAM (GB)", gpu.vram_gb),
        ("NVLink links", gpu.nvlink_links),
        ("TDP (W)", gpu.tdp_w),
    ];

    let mut exact_count = 0u32;
    let mut total = 0u32;

    for (name, value) in &checks {
        if *value == 0 { continue; }
        total += 1;
        if let Some(m) = find_n6_expression(*value) {
            println!("║  ✅ {:15} = {:>5}  =  {:<30} ║", name, value, m.expr);
            exact_count += 1;
        } else {
            println!("║  ❌ {:15} = {:>5}  =  (no n=6 match)              ║", name, value);
        }
    }

    // Hierarchy check: GPCs × TPCs × SMs
    let computed = gpu.gpcs * gpu.tpcs_per_gpc * gpu.sms_per_tpc;
    let hierarchy_ok = computed == gpu.full_die_sms;

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  Hierarchy: {} GPCs × {} TPCs × {} SMs = {} {}           ",
        gpu.gpcs, gpu.tpcs_per_gpc, gpu.sms_per_tpc, computed,
        if hierarchy_ok { "✅" } else { "❌" });
    println!("║  N6 match: {}/{} parameters ({:.0}%)                        ",
        exact_count, total, (exact_count as f64 / total as f64) * 100.0);

    // Special: check if enabled SMs = σ(σ-μ) = 132
    if gpu.enabled_sms == SIGMA * (SIGMA - MU as u64) {
        println!("║  ⭐ Enabled SMs = σ(σ-μ) = 132 = 1/α leading term!       ║");
    }

    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

// ─── HBM Verification ───

fn verify_hbm() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  HBM Stack Height Ladder: τ → (σ-τ) → σ → 2^τ → J₂?      ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    let hbm = hbm_database();
    let ladder: Vec<(u64, &str)> = vec![
        (4, "τ"), (8, "σ-τ"), (12, "σ"), (16, "2^τ"), (24, "J₂"),
    ];

    for gen in &hbm {
        let n6 = find_n6_expression(gen.stack_height);
        let bus_width = gen.channels * gen.bits_per_channel;
        let bw_per_stack = (bus_width as f64) * gen.pin_gbps / 8.0;

        let n6_str = match &n6 {
            Some(m) => format!("{}", m.expr),
            None => "?".to_string(),
        };

        println!("║  {:<6} {}  {:>2}-hi = {:<8}  {}ch × {}b = {}b  {:.0} GB/s/stack",
            gen.name, gen.year, gen.stack_height, n6_str,
            gen.channels, gen.bits_per_channel, bus_width, bw_per_stack);
    }

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  Ladder: 4(τ) → 8(σ-τ) → 12(σ) → 16(2^τ) → 24(J₂)?      ║");
    println!("║  Channel evolution: 8(σ-τ) → 16(2^τ)                       ║");
    println!("║  Bus width: 1024b = (σ-τ)·2^(σ-sopfr) = 8·128             ║");
    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  PREDICTION: HBM5 (~2027) = 24-hi = J₂(6)                  ║");
    println!("║  PREDICTION: HBM5 bus = 2048b = 2^(σ-μ)                    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

// ─── Prediction Engine ───

fn predict_rubin() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  NVIDIA Rubin (~2026) — N6 Predictions                     ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    let candidates: Vec<(u64, &str, &str)> = vec![
        (240, "σ·(J₂-τ) = 12·20", "Chinchilla multiplier"),
        (256, "2^(σ-τ) = 2^8", "Bott periodicity power"),
        (288, "σ·J₂ = 12·24", "Leech lattice multiplier"),
    ];

    println!("║  SM count candidates (σ-multiples):                        ║");
    for (sms, expr, note) in &candidates {
        let cuda_total = sms * 128; // assuming 128 CUDA/SM
        let tflops_fp16 = (cuda_total as f64) * 2.0 * 2.0 / 1000.0; // rough est
        println!("║    {} SMs = {}  ({})        ", sms, expr, note);
        println!("║      → {} CUDA cores, ~{:.0} TFLOPS FP16 est.           ", cuda_total, tflops_fp16);
    }

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  Other predictions:                                         ║");
    println!("║    GPCs: 12(σ) or 16(2^τ) or 20(J₂-τ)                     ║");
    println!("║    HBM: HBM4 16-hi (2^τ), 6(n) stacks → 192 GB            ║");
    println!("║    NVLink: 24(J₂) links = 1:1 with Golay code length      ║");
    println!("║    TDP: ~1200W = σ²·(σ-τ+φ/τ) or σ³/√φ                   ║");
    println!("║    TC/SM: τ=4 (maintained since Ampere)                     ║");
    println!("║    CUDA/SM: 128=2^(σ-sopfr) or 192=σ·2^τ (Kepler回帰)    ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

// ─── Cross-generation Analysis ───

fn analyze_sm_sequence() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  GPU SM Count Sequence — σ(6)=12 as Architectural Atom     ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    let sequence: Vec<(u16, &str, u64, &str)> = vec![
        (2017, "Volta GV100", 84, "σ·(σ-sopfr) = 12·7"),
        (2018, "Turing TU102", 72, "σ·n = 12·6"),
        (2020, "Ampere GA100", 128, "2^(σ-sopfr) = 2^7"),
        (2022, "Ada AD102", 144, "σ² = 12·12"),
        (2022, "Hopper GH100", 144, "σ² = 12·12"),
        (2024, "Blackwell GB202", 192, "σ·2^τ = 12·16"),
        (2026, "Rubin (pred A)", 240, "σ·(J₂-τ) = 12·20"),
        (2026, "Rubin (pred B)", 288, "σ·J₂ = 12·24"),
    ];

    for (year, name, sms, expr) in &sequence {
        let is_sigma_mult = sms % SIGMA == 0;
        let marker = if *year >= 2026 { "🔮" } else if is_sigma_mult { "✅" } else { "⚠️" };
        let factor = if is_sigma_mult { sms / SIGMA } else { 0 };
        println!("║  {} {} {:20} {:>3} SMs = {:<25} (σ×{})",
            marker, year, name, sms, expr, factor);
    }

    // Check: how many are σ multiples?
    let total_real = 6u32;
    let sigma_mult: u32 = [84, 72, 128, 144, 144, 192].iter()
        .filter(|&&x| x % SIGMA == 0)
        .count() as u32;

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  σ-multiples: {}/{}  ({:.0}%)                                ",
        sigma_mult, total_real, (sigma_mult as f64 / total_real as f64) * 100.0);
    println!("║  128 = 2^(σ-sopfr) — Ampere is the only non-σ-multiple     ║");
    println!("║  Factor sequence: 7→6→?→12→12→16→20?→24?                  ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

// ─── Exponent Ladder ───

fn show_exponent_ladder() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  BT-28 Exponent Ladder: 2^k where k ∈ n=6 constants       ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    let ladder: Vec<(u64, &str, &str)> = vec![
        (TAU, "τ = 4", "Tensor Core dim = 16"),
        (SOPFR, "sopfr = 5", "CUDA warp = 32"),
        (N, "n = 6", "Cache line = 64"),
        (SIGMA - SOPFR, "σ-sopfr = 7", "SSE/NEON/TPU = 128"),
        (SIGMA - TAU, "σ-τ = 8", "AVX-256 / SHA-256"),
        (SIGMA - MU as u64, "σ-μ = 11", "L2 TLB = 2048"),
        (SIGMA, "σ = 12", "Page size = 4096"),
    ];

    for (exp, n6_expr, hw_example) in &ladder {
        let val = 1u64 << exp;
        println!("║  2^({:<12}) = 2^{:<2} = {:>5}   ← {}",
            n6_expr, exp, val, hw_example);
    }

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  Next: 2^(σ+μ)=2^13=8192 (GPU shared mem per SM?)         ║");
    println!("║        2^(2^τ)=2^16=65536 (max thread block size?)        ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

// ─── Industry Cross-Check ───

// ─── Multi-Vendor Verification ───

fn verify_multivendor() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Multi-Vendor Chip Verification (BT-69 extension)          ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    let chips = multivendor_database();
    let mut total_exact = 0u32;
    let mut total_params = 0u32;

    for chip in &chips {
        println!("║                                                              ║");
        println!("║  {} — {} ({})                   ", chip.vendor, chip.name, chip.year);
        println!("║  ────────────────────────────────────────────                 ║");
        for (name, value, formula) in &chip.params {
            total_params += 1;
            let matched = find_n6_expression(*value).is_some();
            if matched { total_exact += 1; }
            let icon = if matched { "✅" } else { "⚠️" };
            println!("║    {} {:18} = {:>5}  =  {:<25}",
                icon, name, value, formula);
        }
    }

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  Multi-vendor N6 match: {}/{} ({:.0}%)                       ",
        total_exact, total_params,
        (total_exact as f64 / total_params as f64) * 100.0);
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

// ─── Interconnect Standard Verification ───

fn verify_interconnect() {
    println!("╔══════════════════════════════════════════════════════════════╗");
    println!("║  Interconnect / Memory Standards — N6 Verification         ║");
    println!("╠══════════════════════════════════════════════════════════════╣");

    let stds = interconnect_database();
    let mut total_exact = 0u32;
    let mut total_params = 0u32;

    for std in &stds {
        println!("║                                                              ║");
        println!("║  {} ({})                                          ", std.name, std.year);
        println!("║  ────────────────────────────────────────────                 ║");
        for (name, value, formula) in &std.params {
            total_params += 1;
            let matched = find_n6_expression(*value).is_some();
            if matched { total_exact += 1; }
            let icon = if matched { "✅" } else { "⚠️" };
            println!("║    {} {:18} = {:>5}  =  {:<25}",
                icon, name, value, formula);
        }
    }

    println!("╠──────────────────────────────────────────────────────────────╣");
    println!("║  Interconnect N6 match: {}/{} ({:.0}%)                       ",
        total_exact, total_params,
        (total_exact as f64 / total_params as f64) * 100.0);
    println!("║  PREDICTION: UCIe 3.0 = σ·τ = 48 GT/s base rate            ║");
    println!("║  PREDICTION: CXL 4.0 = 2^(σ-sopfr) = 128 GT/s             ║");
    println!("╚══════════════════════════════════════════════════════════════╝");
    println!();
}

fn industry_check(value: u64) {
    println!("Checking {} against n=6 architecture constants...", value);
    match find_n6_expression(value) {
        Some(m) => println!("  ✅ {} = {} (EXACT)", value, m.expr),
        None => {
            // Try nearby values
            println!("  ❌ {} has no direct n=6 expression", value);
            println!("  Nearby n=6 values:");
            for delta in 1..=5u64 {
                if let Some(m) = find_n6_expression(value - delta) {
                    println!("    {} - {} = {} = {}", value, delta, value - delta, m.expr);
                }
                if let Some(m) = find_n6_expression(value + delta) {
                    println!("    {} + {} = {} = {}", value, delta, value + delta, m.expr);
                }
            }
        }
    }
}

// ─── Main ───

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args.contains(&"--all".to_string()) {
        println!("╔══════════════════════════════════════════════════════════════╗");
        println!("║     N6 GPU/HBM Architecture Calculator                     ║");
        println!("║     σ(6)=12  τ(6)=4  φ(6)=2  sopfr(6)=5  J₂(6)=24       ║");
        println!("╚══════════════════════════════════════════════════════════════╝");
        println!();

        // Verify all GPUs
        for gpu in gpu_database() {
            verify_gpu(&gpu);
        }

        // HBM analysis
        verify_hbm();

        // Exponent ladder
        show_exponent_ladder();

        // SM sequence
        analyze_sm_sequence();

        // Multi-vendor chips
        verify_multivendor();

        // Interconnect standards
        verify_interconnect();

        // Predictions
        predict_rubin();

        return;
    }

    if args.contains(&"--verify".to_string()) {
        for gpu in gpu_database() {
            verify_gpu(&gpu);
        }
    }

    if args.contains(&"--hbm".to_string()) {
        verify_hbm();
    }

    if args.contains(&"--predict".to_string()) {
        predict_rubin();
        analyze_sm_sequence();
    }

    if args.contains(&"--ladder".to_string()) {
        show_exponent_ladder();
    }

    if args.contains(&"--multivendor".to_string()) {
        verify_multivendor();
    }

    if args.contains(&"--interconnect".to_string()) {
        verify_interconnect();
    }

    // --check <value>
    if let Some(pos) = args.iter().position(|a| a == "--check") {
        if let Some(val_str) = args.get(pos + 1) {
            if let Ok(val) = val_str.parse::<u64>() {
                industry_check(val);
            }
        }
    }
}
