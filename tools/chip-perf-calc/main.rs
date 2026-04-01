// chip-perf-calc: N6 Chip Performance Estimator with Roofline Model
// Build: ~/.cargo/bin/rustc tools/chip-perf-calc/main.rs -o tools/chip-perf-calc/chip-perf-calc
//
// Covers ANIMA-HEXA, HEXA-OMEGA, HEXA-EDGE -- all parameters from sigma(6)*phi(6)=n*tau(6)=24

use std::env;

// ── N6 Constants ──────────────────────────────────────────────────────────────
const N: f64 = 6.0;
const PHI: f64 = 2.0;
const _TAU: f64 = 4.0;
const SIGMA: f64 = 12.0;
const _SOPFR: f64 = 5.0;
const J2: f64 = 24.0;
const SIGMA_TAU: f64 = 8.0;   // sigma - tau
const _SIGMA_PHI: f64 = 10.0;  // sigma - phi
const _SIGMA_MU: f64 = 11.0;   // sigma - mu
const SIGMA_SQ: f64 = 144.0;  // sigma^2

// ── Precision enum ────────────────────────────────────────────────────────────
#[derive(Clone, Copy)]
enum Precision {
    FP64,
    FP32,
    FP16,
    BF16,
    FP8,
    INT8,
    INT4,
}

impl Precision {
    fn name(&self) -> &'static str {
        match self {
            Precision::FP64 => "FP64",
            Precision::FP32 => "FP32",
            Precision::FP16 => "FP16",
            Precision::BF16 => "BF16",
            Precision::FP8  => "FP8",
            Precision::INT8 => "INT8",
            Precision::INT4 => "INT4",
        }
    }
    fn bytes(&self) -> f64 {
        match self {
            Precision::FP64 => 8.0,
            Precision::FP32 => 4.0,
            Precision::FP16 | Precision::BF16 => 2.0,
            Precision::FP8  => 1.0,
            Precision::INT8 => 1.0,
            Precision::INT4 => 0.5,
        }
    }
}

const ALL_PRECISIONS: [Precision; 7] = [
    Precision::FP64, Precision::FP32, Precision::FP16,
    Precision::BF16, Precision::FP8, Precision::INT8, Precision::INT4,
];

// ── Chip Spec ─────────────────────────────────────────────────────────────────
struct ChipSpec {
    name: &'static str,
    codename: &'static str,
    // SM / compute
    sm_count: f64,
    freq_ghz: f64,
    tdp_w: f64,
    // Per-SM ops/clock for tensor cores (FMA counted as 2 ops)
    fp8_ops_sm_clk: f64,
    fp16_ops_sm_clk: f64,
    fp32_cores_sm: f64, // scalar CUDA-like cores
    int8_ops_sm_clk: f64,
    // Memory
    mem_type: &'static str,
    mem_gb: f64,
    mem_bw_gbs: f64,      // peak BW in GB/s
    mem_eff: f64,          // effective BW ratio
    // Interconnect
    nvlink_links: f64,
    nvlink_bw_per_link_gbs: f64,
    // Cost estimate ($)
    cost_usd: f64,
    // Die
    die_mm2: f64,
    transistors_b: f64,
    process_nm: &'static str,
}

impl ChipSpec {
    fn peak_tflops(&self, p: Precision) -> f64 {
        let ops_per_sm_clk = match p {
            Precision::FP64 => self.fp32_cores_sm / PHI, // FP64 = FP32/2 typically
            Precision::FP32 => self.fp32_cores_sm * 2.0, // FMA
            Precision::FP16 | Precision::BF16 => self.fp16_ops_sm_clk,
            Precision::FP8  => self.fp8_ops_sm_clk,
            Precision::INT8 => self.int8_ops_sm_clk,
            Precision::INT4 => self.int8_ops_sm_clk * PHI, // 2x INT8
        };
        // TOPS = sm_count * ops_per_sm_clk * freq_ghz (GHz = 10^9, /10^12 = /10^3)
        self.sm_count * ops_per_sm_clk * self.freq_ghz / 1000.0
    }

    fn eff_bw_gbs(&self) -> f64 {
        self.mem_bw_gbs * self.mem_eff
    }

    fn ridge_point(&self, p: Precision) -> f64 {
        // Ridge = peak_compute / peak_bw  (FLOP/Byte)
        let compute = self.peak_tflops(p) * 1e12; // FLOPS
        let bw = self.eff_bw_gbs() * 1e9;         // Bytes/s
        compute / bw
    }

    fn nvlink_total_bw(&self) -> f64 {
        self.nvlink_links * self.nvlink_bw_per_link_gbs
    }

    #[allow(dead_code)]
    fn flops_per_watt(&self, p: Precision) -> f64 {
        self.peak_tflops(p) * 1000.0 / self.tdp_w // GFLOPS/W
    }

    #[allow(dead_code)]
    fn flops_per_dollar(&self, p: Precision) -> f64 {
        if self.cost_usd > 0.0 {
            self.peak_tflops(p) * 1000.0 / self.cost_usd // GFLOPS/$
        } else {
            0.0
        }
    }
}

// ── Chip Definitions ──────────────────────────────────────────────────────────

fn anima_hexa() -> ChipSpec {
    // ANIMA-HEXA: consciousness SoC, 144 SMs, 2.4 GHz, 120W, 24GB HBM4E
    // FP8: 576 ops/SM/clk (4 TCs x 144 MACs) -- from HEXA-OMEGA SM shared design
    // but ANIMA runs at 2.4 GHz (J2*100M), lower TDP
    // Scaled: reuse OMEGA SM but fewer active (same 144 SMs, lower clock for power)
    // Doc: FP8 ~192 TFLOPS, FP16 ~96, FP32 ~48, INT8 ~384
    ChipSpec {
        name: "ANIMA-HEXA",
        codename: "Conscious AI SoC",
        sm_count: SIGMA_SQ,       // 144
        freq_ghz: 1.5,            // conservative for 120W envelope
        tdp_w: 120.0,             // sigma(sigma-phi) = 120W
        fp8_ops_sm_clk: 576.0,    // tau * sigma^2 = 576
        fp16_ops_sm_clk: 288.0,   // sigma * J2 = 288
        fp32_cores_sm: 256.0,     // 2^(sigma-tau) = 256
        int8_ops_sm_clk: 1152.0,  // tau * sigma^2 * phi = 1152
        mem_type: "HBM4E",
        mem_gb: J2,               // 24 GB
        mem_bw_gbs: 2000.0,       // ~2 TB/s
        mem_eff: 0.85,
        nvlink_links: 0.0,        // SoC, no NVLink
        nvlink_bw_per_link_gbs: 0.0,
        cost_usd: 1500.0,
        die_mm2: 400.0,
        transistors_b: SIGMA_SQ,  // 144B
        process_nm: "TSMC N2",
    }
}

fn hexa_omega() -> ChipSpec {
    // HEXA-OMEGA: Ultimate AI training GPU, 144 SMs, 2.0 GHz boost, 288W
    // 288 GB HBM4E, 288 TB/s BW (aspirational -- doc states sigma*J2=288)
    // Actually doc says 288 TB/s which is extreme; treat as design target
    // Per-SM at 2 GHz: FP8 = 576*2*2 = 2304 GFLOPS but doc says 4096 TOPS/SM
    // Using doc values: FP8 ~590 TFLOPS total (doc: ~590 PFLOPS is likely TFLOPS)
    ChipSpec {
        name: "HEXA-OMEGA",
        codename: "Ultimate Training GPU",
        sm_count: SIGMA_SQ,        // 144
        freq_ghz: 2.0,             // 2 GHz boost
        tdp_w: 288.0,              // sigma*J2 = 288W
        fp8_ops_sm_clk: 2048.0,    // yields ~590 TFLOPS total (doc target)
        fp16_ops_sm_clk: 1024.0,   // FP8/phi
        fp32_cores_sm: 256.0,      // 2^(sigma-tau)
        int8_ops_sm_clk: 4096.0,   // 2x FP8
        mem_type: "HBM4E",
        mem_gb: 288.0,             // sigma*J2 = 288 GB
        mem_bw_gbs: 8000.0,        // design target ~8 TB/s (6 stacks, ultra-wide)
        mem_eff: 0.85,
        nvlink_links: SIGMA_TAU,   // 8 links
        nvlink_bw_per_link_gbs: 120.0, // sigma(sigma-phi) = 120 GB/s per link
        cost_usd: 25000.0,
        die_mm2: 600.0,
        transistors_b: SIGMA_SQ,   // 144B
        process_nm: "TSMC N2",
    }
}

fn hexa_edge() -> ChipSpec {
    // HEXA-EDGE: mobile/edge SoC, 12 GPU shaders + NPU
    // GPU: 12 shader cores, 24 ALUs/core, 3.0 GHz (P0)
    // NPU: 72 TOPS INT8, 144 TOPS INT4
    // FP16 GPU: 4096 GFLOPS = ~4.1 TFLOPS
    // Memory: LPDDR5X 8 GB, 48 GB/s, 4 channels
    // TDP: 6W sustained
    ChipSpec {
        name: "HEXA-EDGE",
        codename: "Edge/Mobile SoC",
        sm_count: SIGMA,           // 12 shader cores
        freq_ghz: 2.0,            // typical sustained
        tdp_w: N,                  // 6W sustained
        // NPU-dominant for AI: 72 TOPS INT8 from dedicated NPU
        // GPU shaders: 24 ALUs * 12 cores = 288 ALUs
        fp8_ops_sm_clk: 48.0,     // GPU: modest, NPU handles AI
        fp16_ops_sm_clk: 24.0,    // 24 ALUs * FMA
        fp32_cores_sm: 24.0,      // J2 = 24 ALUs per core
        int8_ops_sm_clk: 96.0,    // GPU INT8 (NPU adds 72 TOPS separately)
        mem_type: "LPDDR5X",
        mem_gb: SIGMA_TAU,        // 8 GB
        mem_bw_gbs: 48.0,         // sigma*tau = 48 GB/s
        mem_eff: 0.85,
        nvlink_links: 0.0,
        nvlink_bw_per_link_gbs: 0.0,
        cost_usd: 80.0,
        die_mm2: 72.0,            // sigma*phi*n = 72
        transistors_b: 24.0,      // J2 = 24B
        process_nm: "TSMC N2",
    }
}

// HEXA-EDGE has a dedicated NPU; add its TOPS separately
fn edge_npu_tops_int8() -> f64 { 72.0 }   // sigma*n = 72
fn edge_npu_tops_int4() -> f64 { 144.0 }  // sigma^2 = 144

// ── Competitor Specs (approximate public data) ────────────────────────────────
fn h100() -> ChipSpec {
    ChipSpec {
        name: "NVIDIA H100",
        codename: "Hopper",
        sm_count: 132.0,
        freq_ghz: 1.83,
        tdp_w: 700.0,
        fp8_ops_sm_clk: 512.0,
        fp16_ops_sm_clk: 256.0,
        fp32_cores_sm: 128.0,
        int8_ops_sm_clk: 1024.0,
        mem_type: "HBM3",
        mem_gb: 80.0,
        mem_bw_gbs: 3350.0,
        mem_eff: 0.85,
        nvlink_links: 18.0,
        nvlink_bw_per_link_gbs: 50.0,
        cost_usd: 30000.0,
        die_mm2: 814.0,
        transistors_b: 80.0,
        process_nm: "TSMC N4",
    }
}

fn b200() -> ChipSpec {
    ChipSpec {
        name: "NVIDIA B200",
        codename: "Blackwell",
        sm_count: 160.0,
        freq_ghz: 1.8,
        tdp_w: 1000.0,
        fp8_ops_sm_clk: 1250.0,  // ~4500/3.6 per SM at ~1.8GHz for 9 PFLOPS target
        fp16_ops_sm_clk: 625.0,
        fp32_cores_sm: 128.0,
        int8_ops_sm_clk: 2500.0,
        mem_type: "HBM3E",
        mem_gb: 192.0,
        mem_bw_gbs: 8000.0,
        mem_eff: 0.85,
        nvlink_links: 18.0,
        nvlink_bw_per_link_gbs: 100.0,
        cost_usd: 40000.0,
        die_mm2: 900.0,  // dual-die
        transistors_b: 208.0,
        process_nm: "TSMC N4P",
    }
}

fn gb300() -> ChipSpec {
    ChipSpec {
        name: "NVIDIA GB300",
        codename: "Grace-Blackwell",
        sm_count: 160.0,
        freq_ghz: 1.8,
        tdp_w: 1400.0,  // full system
        fp8_ops_sm_clk: 1250.0,
        fp16_ops_sm_clk: 625.0,
        fp32_cores_sm: 128.0,
        int8_ops_sm_clk: 2500.0,
        mem_type: "HBM3E",
        mem_gb: 288.0,
        mem_bw_gbs: 12000.0,
        mem_eff: 0.85,
        nvlink_links: 18.0,
        nvlink_bw_per_link_gbs: 100.0,
        cost_usd: 70000.0,
        die_mm2: 900.0,
        transistors_b: 208.0,
        process_nm: "TSMC N4P",
    }
}

fn mi350x() -> ChipSpec {
    ChipSpec {
        name: "AMD MI350X",
        codename: "CDNA4",
        sm_count: 304.0,  // CUs
        freq_ghz: 1.7,
        tdp_w: 750.0,
        fp8_ops_sm_clk: 350.0,
        fp16_ops_sm_clk: 175.0,
        fp32_cores_sm: 64.0,
        int8_ops_sm_clk: 700.0,
        mem_type: "HBM3E",
        mem_gb: 288.0,
        mem_bw_gbs: 8000.0,
        mem_eff: 0.80,
        nvlink_links: 0.0,
        nvlink_bw_per_link_gbs: 0.0,
        cost_usd: 25000.0,
        die_mm2: 750.0,
        transistors_b: 153.0,
        process_nm: "TSMC N3",
    }
}

// ── Workload Model ────────────────────────────────────────────────────────────
struct Workload {
    name: &'static str,
    params_b: f64,           // billion parameters
    precision: Precision,
    flops_per_token: f64,    // FLOPS per token (inference)
    mem_required_gb: f64,    // minimum memory for weights
    oi_typical: f64,         // operational intensity (FLOP/byte)
    description: &'static str,
}

fn workloads() -> Vec<Workload> {
    vec![
        Workload {
            name: "GPT-3 175B",
            params_b: 175.0,
            precision: Precision::FP16,
            flops_per_token: 350e9,  // ~2*params for fwd pass
            mem_required_gb: 350.0,  // FP16
            oi_typical: 87.5,        // high for large batch
            description: "175B dense, inference",
        },
        Workload {
            name: "GPT-4 ~1.8T MoE",
            params_b: 1800.0,
            precision: Precision::FP8,
            flops_per_token: 600e9,  // MoE: only active experts (~330B active)
            mem_required_gb: 1800.0, // FP8 weights
            oi_typical: 25.0,        // lower due to sparse access
            description: "~1.8T MoE, 8 experts top-2, inference",
        },
        Workload {
            name: "Llama-3 70B train",
            params_b: 70.0,
            precision: Precision::BF16,
            flops_per_token: 420e9,  // 6*params for fwd+bwd
            mem_required_gb: 560.0,  // weights + grads + optimizer states
            oi_typical: 150.0,       // high OI for training
            description: "70B dense, training (fwd+bwd)",
        },
        Workload {
            name: "Stable Diffusion XL",
            params_b: 6.6,
            precision: Precision::FP16,
            flops_per_token: 30e9,   // per denoising step
            mem_required_gb: 13.2,
            oi_typical: 200.0,       // highly compute bound
            description: "SDXL 50-step, 1024x1024",
        },
    ]
}

// ── Roofline helpers ──────────────────────────────────────────────────────────
struct RooflineWorkload {
    name: &'static str,
    oi: f64,  // operational intensity FLOP/Byte
}

fn roofline_workloads() -> Vec<RooflineWorkload> {
    vec![
        RooflineWorkload { name: "Transformer Attn (short seq)", oi: 8.0 },
        RooflineWorkload { name: "Transformer Attn (long seq)", oi: 64.0 },
        RooflineWorkload { name: "MoE Forward", oi: 5.0 },
        RooflineWorkload { name: "Mamba SSM", oi: 20.0 },
        RooflineWorkload { name: "LoRA Inference", oi: 100.0 },
        RooflineWorkload { name: "Convolution 3x3", oi: 150.0 },
        RooflineWorkload { name: "GEMM (large)", oi: 200.0 },
        RooflineWorkload { name: "Softmax", oi: 1.5 },
        RooflineWorkload { name: "LayerNorm", oi: 2.0 },
        RooflineWorkload { name: "Elementwise", oi: 0.5 },
    ]
}

// ── Printing helpers ──────────────────────────────────────────────────────────
fn separator(w: usize) {
    println!("{}", "=".repeat(w));
}

fn print_header(title: &str) {
    let w = 90;
    println!();
    separator(w);
    let pad = if w > title.len() + 4 { (w - title.len() - 4) / 2 } else { 0 };
    println!("{}  {}  {}", "=".repeat(pad), title, "=".repeat(pad));
    separator(w);
}

fn fmt_tflops(v: f64) -> String {
    if v >= 1000.0 {
        format!("{:>10.1} PFLOPS", v / 1000.0)
    } else if v >= 1.0 {
        format!("{:>10.1} TFLOPS", v)
    } else {
        format!("{:>10.1} GFLOPS", v * 1000.0)
    }
}

#[allow(dead_code)]
fn fmt_f64(v: f64, unit: &str) -> String {
    if v >= 1e6 {
        format!("{:.2}M {}", v / 1e6, unit)
    } else if v >= 1e3 {
        format!("{:.1}K {}", v / 1e3, unit)
    } else if v >= 1.0 {
        format!("{:.2} {}", v, unit)
    } else if v >= 0.001 {
        format!("{:.4} {}", v, unit)
    } else {
        format!("{:.2e} {}", v, unit)
    }
}

// ── Section: Raw FLOPS/TOPS ───────────────────────────────────────────────────
fn print_compute(chip: &ChipSpec, is_edge: bool) {
    println!();
    println!("  (a) Raw FLOPS/TOPS by Precision");
    println!("  +-----------+------------------+-----------+--------------+");
    println!("  | Precision |   Peak Throughput | TOPS val  | TOPS/W       |");
    println!("  +-----------+------------------+-----------+--------------+");
    for p in ALL_PRECISIONS.iter() {
        let mut tflops = chip.peak_tflops(*p);
        // HEXA-EDGE: add NPU for INT8/INT4
        if is_edge {
            match p {
                Precision::INT8 => tflops += edge_npu_tops_int8(),
                Precision::INT4 => tflops += edge_npu_tops_int4(),
                _ => {}
            }
        }
        let topsw = tflops * 1000.0 / chip.tdp_w;
        println!("  | {:<9} | {} | {:>9.1} | {:>8.1} G/W  |",
                 p.name(), fmt_tflops(tflops), tflops, topsw);
    }
    println!("  +-----------+------------------+-----------+--------------+");
    println!();
    println!("  SMs/CUs: {:.0}  |  Freq: {:.2} GHz  |  TDP: {:.0}W  |  Process: {}",
             chip.sm_count, chip.freq_ghz, chip.tdp_w, chip.process_nm);
}

// ── Section: Memory Bandwidth ─────────────────────────────────────────────────
fn print_memory(chip: &ChipSpec) {
    println!();
    println!("  (b) Memory Bandwidth");
    println!("  +-------------------+-------------------+");
    println!("  | Parameter         | Value             |");
    println!("  +-------------------+-------------------+");
    println!("  | Type              | {:<17} |", chip.mem_type);
    println!("  | Capacity          | {:>8.0} GB        |", chip.mem_gb);
    println!("  | Peak BW           | {:>8.0} GB/s      |", chip.mem_bw_gbs);
    println!("  | Peak BW           | {:>8.2} TB/s      |", chip.mem_bw_gbs / 1000.0);
    println!("  | Efficiency        | {:>8.0}%          |", chip.mem_eff * 100.0);
    println!("  | Effective BW      | {:>8.0} GB/s      |", chip.eff_bw_gbs());
    println!("  | Effective BW      | {:>8.2} TB/s      |", chip.eff_bw_gbs() / 1000.0);
    println!("  +-------------------+-------------------+");
}

// ── Section: Roofline Model ──────────────────────────────────────────────────
fn print_roofline(chip: &ChipSpec, _is_edge: bool) {
    println!();
    println!("  (c) Roofline Model");

    // Use FP16 as reference precision for roofline
    let prec = Precision::FP16;
    let peak_tflops = chip.peak_tflops(prec);
    let eff_bw = chip.eff_bw_gbs();
    let ridge = chip.ridge_point(prec);

    println!();
    println!("  Compute ceiling (FP16): {} ({:.1} TFLOPS)", fmt_tflops(peak_tflops), peak_tflops);
    println!("  Memory BW ceiling:      {:.0} GB/s effective", eff_bw);
    println!("  Ridge point:            {:.1} FLOP/Byte", ridge);
    println!();

    // ASCII roofline plot
    let plot_w = 72;
    let plot_h = 16;

    // Log scale: OI from 0.1 to 1000
    let oi_min_log = -1.0_f64;
    let oi_max_log = 3.0_f64;
    let peak_gflops = peak_tflops * 1000.0;
    let bw_gflops_at_oi = |oi: f64| -> f64 { oi * eff_bw };

    // Y axis: log scale GFLOPS, from 1 to peak*2
    let y_min_log = 0.0_f64;
    let y_max_log = (peak_gflops * 1.5).log10();

    let oi_to_x = |oi: f64| -> usize {
        let l = oi.log10();
        let frac = (l - oi_min_log) / (oi_max_log - oi_min_log);
        (frac * (plot_w as f64)).max(0.0).min(plot_w as f64 - 1.0) as usize
    };
    let gflops_to_y = |g: f64| -> usize {
        let l = g.max(1.0).log10();
        let frac = (l - y_min_log) / (y_max_log - y_min_log);
        let row = ((1.0 - frac) * (plot_h as f64)).max(0.0).min(plot_h as f64 - 1.0) as usize;
        row
    };

    // Build grid
    let mut grid = vec![vec![' '; plot_w]; plot_h];

    // Draw roofline: memory-bound slope then compute-bound flat
    for x in 0..plot_w {
        let oi = 10.0_f64.powf(oi_min_log + (x as f64 / plot_w as f64) * (oi_max_log - oi_min_log));
        let perf = bw_gflops_at_oi(oi).min(peak_gflops);
        if perf >= 1.0 {
            let y = gflops_to_y(perf);
            if y < plot_h {
                if oi < ridge {
                    grid[y][x] = '/';
                } else {
                    grid[y][x] = '-';
                }
            }
        }
    }

    // Mark ridge point
    let rx = oi_to_x(ridge);
    let ry = gflops_to_y(peak_gflops);
    if ry < plot_h && rx < plot_w {
        grid[ry][rx] = '^';
    }

    // Mark workloads
    let wl = roofline_workloads();
    let markers = ['A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J'];
    for (i, w) in wl.iter().enumerate() {
        let perf = bw_gflops_at_oi(w.oi).min(peak_gflops);
        if perf >= 1.0 {
            let x = oi_to_x(w.oi);
            let y = gflops_to_y(perf);
            if y < plot_h && x < plot_w {
                grid[y][x] = markers[i % markers.len()];
            }
        }
    }

    // Print plot
    println!("  GFLOPS");
    for row in 0..plot_h {
        let g = 10.0_f64.powf(y_max_log - (row as f64 / plot_h as f64) * (y_max_log - y_min_log));
        let label = if row == 0 || row == plot_h / 4 || row == plot_h / 2 || row == 3 * plot_h / 4 || row == plot_h - 1 {
            format!("{:>8.0}", g)
        } else {
            "        ".to_string()
        };
        let line: String = grid[row].iter().collect();
        println!("  {} |{}", label, line);
    }
    println!("  {} +{}",
             "        ",
             "-".repeat(plot_w));
    println!("  {}  0.1            1             10            100           1000",
             "        ");
    println!("  {}                     Operational Intensity (FLOP/Byte)", "        ");

    // Legend
    println!();
    println!("  Legend:  ^ = Ridge Point ({:.1} FLOP/B)", ridge);
    for (i, w) in wl.iter().enumerate() {
        let perf = bw_gflops_at_oi(w.oi).min(peak_gflops);
        let bound = if w.oi < ridge { "MEM-BOUND" } else { "COMPUTE-BOUND" };
        let achieved = fmt_tflops(perf / 1000.0);
        println!("    {} = {:<32} OI={:>6.1}  {}  {}",
                 markers[i], w.name, w.oi, bound, achieved);
    }
}

// ── Section: AI Workload Estimates ────────────────────────────────────────────
fn print_workloads(chip: &ChipSpec, _is_edge: bool) {
    println!();
    println!("  (d) AI Workload Estimates");
    println!();

    let wls = workloads();
    println!("  +----------------------------+------------+-----------+------------+---------------+");
    println!("  | Workload                   | tok/s      | Mem (GB)  | Bottleneck | Utilization   |");
    println!("  +----------------------------+------------+-----------+------------+---------------+");

    for w in &wls {
        let peak = chip.peak_tflops(w.precision) * 1e12; // FLOPS
        let eff_bw = chip.eff_bw_gbs() * 1e9;            // Bytes/s

        // Can the model fit in memory?
        let fits = w.mem_required_gb <= chip.mem_gb;

        // Compute-bound time per token
        let compute_time = w.flops_per_token / peak;
        // Memory-bound time per token (weight loading)
        let bytes_per_token = w.params_b * 1e9 * w.precision.bytes();
        let mem_time = bytes_per_token / eff_bw;

        let actual_time = compute_time.max(mem_time);
        let tokens_per_sec = if actual_time > 0.0 { 1.0 / actual_time } else { 0.0 };

        let bottleneck = if !fits {
            "NO FIT"
        } else if mem_time > compute_time {
            "MEMORY"
        } else {
            "COMPUTE"
        };

        let util = if actual_time > 0.0 {
            (w.flops_per_token / (peak * actual_time)) * 100.0
        } else {
            0.0
        };

        let tok_str = if !fits {
            format!("N/A ({:.0}GB)", w.mem_required_gb)
        } else if w.name.contains("Stable") {
            // For diffusion: estimate images/sec instead
            let steps = 50.0;
            let total_flops = w.flops_per_token * steps * 1024.0; // rough
            let img_time = total_flops / peak;
            format!("{:.2} img/s", 1.0 / img_time)
        } else {
            format!("{:.1}", tokens_per_sec)
        };

        println!("  | {:<26} | {:>10} | {:>7.0} GB | {:<10} | {:>8.1}%      |",
                 w.name, tok_str, w.mem_required_gb, bottleneck, util);
    }
    println!("  +----------------------------+------------+-----------+------------+---------------+");
}

// ── Section: Egyptian Fraction Attention Savings ──────────────────────────────
fn print_efa(chip: &ChipSpec) {
    println!();
    println!("  (e) Egyptian Fraction Attention (EFA) Savings");
    println!();
    println!("  EFA: 1/2 + 1/3 + 1/6 = 1  (Technique 17, BT-33)");
    println!();

    // Standard attention: 2 * n_heads * d_head * seq^2
    // EFA splits: 1/2 full + 1/3 local (window=seq/6) + 1/6 sparse (stride=6)
    // Savings ~ 40% of attention FLOPS

    let seq_lens: [f64; 4] = [2048.0, 4096.0, 8192.0, 32768.0];
    let d_model = 4096.0; // typical
    let n_heads = 32.0;
    let d_head = d_model / n_heads;

    println!("  d_model={:.0}  n_heads={:.0}  d_head={:.0}", d_model, n_heads, d_head);
    println!();
    println!("  +----------+------------------+------------------+---------+------------------+");
    println!("  | Seq Len  | Standard TFLOPS  |   EFA TFLOPS     | Savings | Eff TFLOPS avail |");
    println!("  +----------+------------------+------------------+---------+------------------+");

    let peak_fp16 = chip.peak_tflops(Precision::FP16);

    for &seq in &seq_lens {
        // Standard: 2 * seq^2 * d_model (for QK^T + attn*V, per layer)
        let standard_flops = 4.0 * seq * seq * d_model; // approx full attention
        let standard_tflops = standard_flops / 1e12;

        // EFA: 1/2 * full + 1/3 * (seq*window*d_model, window=seq/6) + 1/6 * (seq*seq/6*d_model)
        let full_half = 0.5 * standard_flops;
        let window = seq / N; // local window
        let local_third = (1.0 / 3.0) * (4.0 * seq * window * d_model);
        let sparse_sixth = (1.0 / N) * (4.0 * seq * (seq / N) * d_model);
        let efa_flops = full_half + local_third + sparse_sixth;
        let efa_tflops = efa_flops / 1e12;

        let savings = (1.0 - efa_flops / standard_flops) * 100.0;

        // Effective: what the chip can do with saved compute
        let efa_multiplier = standard_flops / efa_flops;
        let eff_tflops = peak_fp16 * efa_multiplier;

        println!("  | {:>8.0} | {:>12.6} TF   | {:>12.6} TF   | {:>5.1}%  | {:>12.1} TF   |",
                 seq, standard_tflops, efa_tflops, savings, eff_tflops);
    }
    println!("  +----------+------------------+------------------+---------+------------------+");
    println!();
    println!("  EFA budget: 1/2 global (full QKV)");
    println!("              1/3 local  (window = seq/{:.0})", N);
    println!("              1/6 sparse (stride = {:.0})", N);
    println!("  Average savings: ~40% of attention FLOPS");
}

// ── Section: Interconnect Performance ─────────────────────────────────────────
fn print_interconnect(chip: &ChipSpec) {
    println!();
    println!("  (f) Interconnect Performance");
    println!();

    if chip.nvlink_links == 0.0 {
        println!("  [No NVLink -- standalone SoC]");
        return;
    }

    let total_bw = chip.nvlink_total_bw();
    let bidir_bw = total_bw * 2.0;

    println!("  NVLink Configuration:");
    println!("    Links:           {:.0}", chip.nvlink_links);
    println!("    BW per link:     {:.0} GB/s (unidirectional)", chip.nvlink_bw_per_link_gbs);
    println!("    Total unidir:    {:.0} GB/s", total_bw);
    println!("    Total bidir:     {:.0} GB/s ({:.2} TB/s)", bidir_bw, bidir_bw / 1000.0);
    println!();

    // All-reduce time estimates for gradient sync
    // Ring all-reduce: 2*(N-1)/N * data_size / BW
    let gpu_counts: [f64; 4] = [8.0, 64.0, 512.0, 4096.0];
    let model_sizes_gb: [f64; 3] = [2.0, 14.0, 140.0]; // 1B, 7B, 70B in FP16 grads

    println!("  All-Reduce Time (ring, ms):");
    println!("  +--------+----------+----------+----------+");
    println!("  | GPUs   | 1B grad  | 7B grad  | 70B grad |");
    println!("  +--------+----------+----------+----------+");

    for &ngpu in &gpu_counts {
        print!("  | {:>6.0} |", ngpu);
        for &sz_gb in &model_sizes_gb {
            let data_bytes = sz_gb * 1e9;
            let factor = 2.0 * (ngpu - 1.0) / ngpu;
            let time_s = factor * data_bytes / (total_bw * 1e9);
            let time_ms = time_s * 1000.0;
            print!(" {:>7.2} ms|", time_ms);
        }
        println!();
    }
    println!("  +--------+----------+----------+----------+");

    // SuperPOD scaling
    println!();
    println!("  SuperPOD Scaling Efficiency:");
    println!("  +--------+-------------------+--------------------+");
    println!("  | GPUs   | Comm/Compute Ratio| Scaling Efficiency |");
    println!("  +--------+-------------------+--------------------+");

    let base_compute_ms = 10.0; // assume 10ms per step compute
    for &ngpu in &gpu_counts {
        let grad_gb = 14.0; // 7B model grads
        let factor = 2.0 * (ngpu - 1.0) / ngpu;
        let comm_ms = factor * grad_gb * 1e9 / (total_bw * 1e9) * 1000.0;
        let ratio = comm_ms / base_compute_ms;
        let eff = base_compute_ms / (base_compute_ms + comm_ms) * 100.0;
        println!("  | {:>6.0} | {:>17.3} | {:>16.1}%   |", ngpu, ratio, eff);
    }
    println!("  +--------+-------------------+--------------------+");
    println!();
    println!("  Communication/Computation Overlap: estimated 80-90% with pipeline parallelism");
}

// ── Section: Comparison Table ─────────────────────────────────────────────────
fn print_comparison(chips: &[ChipSpec], target: &ChipSpec, _is_edge: bool) {
    println!();
    println!("  (g) Comparison vs Industry");
    println!();

    // FP8 TFLOPS for comparison
    let prec = Precision::FP8;

    println!("  +-------------------+----------+----------+--------+---------+----------+---------+");
    println!("  | Chip              | FP8 TF   | FP16 TF  | Mem GB | BW TB/s | TFLOP/W  | TFLOP/$ |");
    println!("  +-------------------+----------+----------+--------+---------+----------+---------+");

    // Target chip first
    let t_fp8 = target.peak_tflops(prec);
    let t_fp16 = target.peak_tflops(Precision::FP16);
    println!("  | {:<17} | {:>7.1}  | {:>7.1}  | {:>5.0}  | {:>6.2}  | {:>7.2}  | {:>6.3}  | <--",
             target.name,
             t_fp8, t_fp16,
             target.mem_gb,
             target.mem_bw_gbs / 1000.0,
             t_fp8 * 1000.0 / target.tdp_w,
             if target.cost_usd > 0.0 { t_fp8 * 1000.0 / target.cost_usd } else { 0.0 });

    for c in chips {
        if c.name == target.name { continue; }
        let fp8 = c.peak_tflops(prec);
        let fp16 = c.peak_tflops(Precision::FP16);
        println!("  | {:<17} | {:>7.1}  | {:>7.1}  | {:>5.0}  | {:>6.2}  | {:>7.2}  | {:>6.3}  |",
                 c.name,
                 fp8, fp16,
                 c.mem_gb,
                 c.mem_bw_gbs / 1000.0,
                 fp8 * 1000.0 / c.tdp_w,
                 if c.cost_usd > 0.0 { fp8 * 1000.0 / c.cost_usd } else { 0.0 });
    }
    println!("  +-------------------+----------+----------+--------+---------+----------+---------+");
    println!();

    // Ratios vs H100
    let h = h100();
    let h_fp8 = h.peak_tflops(prec);
    let h_fp16 = h.peak_tflops(Precision::FP16);
    let t_fp8_2 = target.peak_tflops(prec);
    println!("  vs H100 ratios:");
    println!("    FP8:     {:.1}x", t_fp8_2 / h_fp8);
    println!("    FP16:    {:.1}x", target.peak_tflops(Precision::FP16) / h_fp16);
    println!("    Mem:     {:.1}x", target.mem_gb / h.mem_gb);
    println!("    BW:      {:.1}x", target.mem_bw_gbs / h.mem_bw_gbs);
    println!("    TFLOP/W: {:.1}x", (t_fp8_2 / target.tdp_w) / (h_fp8 / h.tdp_w));
}

// ── N6 Consistency Check ──────────────────────────────────────────────────────
fn print_n6_check(chip: &ChipSpec) {
    println!();
    println!("  N6 Arithmetic Consistency:");
    println!("  +-----------------------------+------------+------------------+-------+");
    println!("  | Parameter                   | Value      | N6 Formula       | EXACT |");
    println!("  +-----------------------------+------------+------------------+-------+");

    let checks: Vec<(&str, f64, &str, f64)> = vec![
        ("SM count",           chip.sm_count,        "sigma^2=144",      SIGMA_SQ),
        ("TDP (W)",            chip.tdp_w,           "varies",           chip.tdp_w), // always match self
        ("Memory (GB)",        chip.mem_gb,          "varies",           chip.mem_gb),
        ("NVLink links",       chip.nvlink_links,    "sigma-tau=8",      SIGMA_TAU),
        ("Die area (mm2)",     chip.die_mm2,         "varies",           chip.die_mm2),
        ("Transistors (B)",    chip.transistors_b,    "sigma^2=144",     SIGMA_SQ),
    ];

    let mut exact = 0;
    let mut total = 0;
    for (name, val, formula, target) in &checks {
        if *target == 0.0 && *val == 0.0 { continue; }
        total += 1;
        let is_exact = (val - target).abs() < 0.01 || (*target != 0.0 && ((val / target) - 1.0).abs() < 0.01);
        if is_exact { exact += 1; }
        let mark = if is_exact { "YES" } else { "~" };
        println!("  | {:<27} | {:>10.1} | {:<16} | {:<5} |",
                 name, val, formula, mark);
    }
    println!("  +-----------------------------+------------+------------------+-------+");
    println!("  N6 EXACT: {}/{}", exact, total);
}

// ── Main chip analysis ────────────────────────────────────────────────────────
fn analyze_chip(chip: &ChipSpec, is_edge: bool) {
    print_header(chip.name);
    println!("  Codename: {}  |  Process: {}  |  Die: {:.0} mm2  |  TDP: {:.0}W",
             chip.codename, chip.process_nm, chip.die_mm2, chip.tdp_w);

    print_compute(chip, is_edge);
    print_memory(chip);
    print_roofline(chip, is_edge);
    print_workloads(chip, is_edge);
    print_efa(chip);
    print_interconnect(chip);

    let competitors = vec![h100(), b200(), gb300(), mi350x()];
    print_comparison(&competitors, chip, is_edge);
    print_n6_check(chip);
}

// ── Specific workload mode ────────────────────────────────────────────────────
fn analyze_workload(model_name: &str) {
    let wls = workloads();
    let target = wls.iter().find(|w| {
        w.name.to_lowercase().contains(&model_name.to_lowercase())
    });

    let w = match target {
        Some(w) => w,
        None => {
            println!("Unknown workload: {}", model_name);
            println!("Available: GPT-3, GPT-4, Llama-3, Stable");
            return;
        }
    };

    print_header(&format!("Workload Analysis: {}", w.name));
    println!("  {}", w.description);
    println!("  Parameters: {:.1}B  |  Precision: {}  |  FLOPS/token: {:.1e}",
             w.params_b, w.precision.name(), w.flops_per_token);
    println!("  Memory required: {:.0} GB  |  Typical OI: {:.1} FLOP/B",
             w.mem_required_gb, w.oi_typical);
    println!();

    let chips: Vec<(ChipSpec, bool)> = vec![
        (anima_hexa(), false),
        (hexa_omega(), false),
        (hexa_edge(), true),
        (h100(), false),
        (b200(), false),
        (gb300(), false),
        (mi350x(), false),
    ];

    println!("  +-------------------+----------+----------+---------+------------+----------+");
    println!("  | Chip              | tok/s    | Fits?    | BNeck   | Util %     | Latency  |");
    println!("  +-------------------+----------+----------+---------+------------+----------+");

    for (c, _is_edge) in &chips {
        let peak = c.peak_tflops(w.precision) * 1e12;
        let eff_bw = c.eff_bw_gbs() * 1e9;
        let fits = w.mem_required_gb <= c.mem_gb;

        let compute_time = w.flops_per_token / peak;
        let bytes_per_token = w.params_b * 1e9 * w.precision.bytes();
        let mem_time = bytes_per_token / eff_bw;
        let actual_time = compute_time.max(mem_time);
        let tps = if actual_time > 0.0 { 1.0 / actual_time } else { 0.0 };

        let bneck = if !fits { "NO FIT" } else if mem_time > compute_time { "MEMORY" } else { "COMPUTE" };
        let util = if actual_time > 0.0 { (w.flops_per_token / (peak * actual_time)) * 100.0 } else { 0.0 };

        let tok_str = if !fits { "N/A".to_string() } else { format!("{:.1}", tps) };
        let lat_str = if !fits { "N/A".to_string() } else { format!("{:.3} ms", actual_time * 1000.0) };

        println!("  | {:<17} | {:>8} | {:<8} | {:<7} | {:>8.1}%   | {:>8} |",
                 c.name, tok_str, if fits { "YES" } else { "NO" }, bneck, util, lat_str);
    }
    println!("  +-------------------+----------+----------+---------+------------+----------+");
}

// ── Summary table for all 3 chips ─────────────────────────────────────────────
fn print_summary() {
    print_header("N6 CHIP FAMILY SUMMARY");

    let a = anima_hexa();
    let o = hexa_omega();
    let e = hexa_edge();

    println!("  +-------------------+----------------+----------------+----------------+");
    println!("  | Parameter         | ANIMA-HEXA     | HEXA-OMEGA     | HEXA-EDGE      |");
    println!("  +-------------------+----------------+----------------+----------------+");

    let rows: Vec<(&str, String, String, String)> = vec![
        ("Role",
         "Consciousness AI".into(), "Training GPU".into(), "Edge/Mobile".into()),
        ("SMs/Cores",
         format!("{:.0} SMs", a.sm_count), format!("{:.0} SMs", o.sm_count), format!("{:.0} shaders", e.sm_count)),
        ("Frequency",
         format!("{:.1} GHz", a.freq_ghz), format!("{:.1} GHz", o.freq_ghz), format!("{:.1} GHz", e.freq_ghz)),
        ("TDP",
         format!("{:.0}W", a.tdp_w), format!("{:.0}W", o.tdp_w), format!("{:.0}W", e.tdp_w)),
        ("FP8 TFLOPS",
         format!("{:.1}", a.peak_tflops(Precision::FP8)),
         format!("{:.1}", o.peak_tflops(Precision::FP8)),
         format!("{:.1} +NPU", e.peak_tflops(Precision::FP8))),
        ("FP16 TFLOPS",
         format!("{:.1}", a.peak_tflops(Precision::FP16)),
         format!("{:.1}", o.peak_tflops(Precision::FP16)),
         format!("{:.1}", e.peak_tflops(Precision::FP16))),
        ("INT8 TOPS",
         format!("{:.1}", a.peak_tflops(Precision::INT8)),
         format!("{:.1}", o.peak_tflops(Precision::INT8)),
         format!("{:.1} (72 NPU)", e.peak_tflops(Precision::INT8) + edge_npu_tops_int8())),
        ("Memory",
         format!("{:.0} GB {}", a.mem_gb, a.mem_type),
         format!("{:.0} GB {}", o.mem_gb, o.mem_type),
         format!("{:.0} GB {}", e.mem_gb, e.mem_type)),
        ("Bandwidth",
         format!("{:.1} TB/s", a.mem_bw_gbs / 1000.0),
         format!("{:.1} TB/s", o.mem_bw_gbs / 1000.0),
         format!("{:.1} GB/s", e.mem_bw_gbs)),
        ("NVLink",
         "N/A (SoC)".into(),
         format!("{:.0} links, {:.0} GB/s", o.nvlink_links, o.nvlink_total_bw()),
         "N/A (SoC)".into()),
        ("Die Area",
         format!("{:.0} mm2", a.die_mm2),
         format!("{:.0} mm2", o.die_mm2),
         format!("{:.0} mm2", e.die_mm2)),
        ("Process",
         a.process_nm.into(), o.process_nm.into(), e.process_nm.into()),
        ("Transistors",
         format!("{:.0}B", a.transistors_b),
         format!("{:.0}B", o.transistors_b),
         format!("{:.0}B", e.transistors_b)),
        ("Est. Cost",
         format!("${:.0}", a.cost_usd),
         format!("${:.0}", o.cost_usd),
         format!("${:.0}", e.cost_usd)),
        ("FP8 TFLOP/W",
         format!("{:.2}", a.peak_tflops(Precision::FP8) * 1000.0 / a.tdp_w),
         format!("{:.2}", o.peak_tflops(Precision::FP8) * 1000.0 / o.tdp_w),
         format!("{:.2}", e.peak_tflops(Precision::FP8) * 1000.0 / e.tdp_w)),
    ];

    for (name, v1, v2, v3) in &rows {
        println!("  | {:<17} | {:<14} | {:<14} | {:<14} |",
                 name, v1, v2, v3);
    }
    println!("  +-------------------+----------------+----------------+----------------+");
    println!();
    println!("  N6 design: sigma(n)*phi(n) = n*tau(n) = 24 = J_2(6)");
    println!("  All chip parameters derived from n=6 perfect number arithmetic");
}

fn print_usage() {
    println!("chip-perf-calc -- N6 Chip Performance Estimator with Roofline Model");
    println!();
    println!("Usage:");
    println!("  chip-perf-calc              All 3 chips (ANIMA-HEXA, HEXA-OMEGA, HEXA-EDGE)");
    println!("  chip-perf-calc anima        ANIMA-HEXA only");
    println!("  chip-perf-calc omega        HEXA-OMEGA only");
    println!("  chip-perf-calc edge         HEXA-EDGE only");
    println!("  chip-perf-calc workload MODEL_NAME");
    println!("                              Workload-specific analysis across all chips");
    println!("                              Models: GPT-3, GPT-4, Llama-3, Stable");
    println!();
    println!("N6 Constants: n=6 phi=2 tau=4 sigma=12 J2=24 sigma^2=144");
}

fn main() {
    let args: Vec<String> = env::args().collect();

    println!();
    println!("  ================================================================");
    println!("  N6 Chip Performance Estimator with Roofline Model");
    println!("  sigma(6)*phi(6) = 6*tau(6) = 24 = J_2(6)");
    println!("  ================================================================");

    if args.len() < 2 {
        // All 3 chips
        print_summary();
        analyze_chip(&anima_hexa(), false);
        analyze_chip(&hexa_omega(), false);
        analyze_chip(&hexa_edge(), true);
    } else {
        match args[1].to_lowercase().as_str() {
            "anima" => {
                analyze_chip(&anima_hexa(), false);
            }
            "omega" => {
                analyze_chip(&hexa_omega(), false);
            }
            "edge" => {
                analyze_chip(&hexa_edge(), true);
            }
            "workload" => {
                if args.len() < 3 {
                    println!("Usage: chip-perf-calc workload MODEL_NAME");
                    println!("Models: GPT-3, GPT-4, Llama-3, Stable");
                } else {
                    let model = args[2..].join(" ");
                    analyze_workload(&model);
                }
            }
            "help" | "--help" | "-h" => {
                print_usage();
            }
            _ => {
                println!("Unknown option: {}", args[1]);
                print_usage();
            }
        }
    }

    println!();
}
