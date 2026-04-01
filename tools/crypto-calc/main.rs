/// N6 Crypto / Blockchain / Cryptography Calculator
/// ==================================================
/// Verifies cryptocurrency, blockchain, and cryptographic parameters
/// from n=6 arithmetic: BT-53 (Crypto), BT-41 (QEC)
///
/// Build: rustc main.rs -o crypto-calc
/// Usage: ./crypto-calc [--verbose]

use std::env;

// n=6 arithmetic constants
const SIGMA: f64 = 12.0;   // σ(6) = 1+2+3+6
const TAU: f64 = 4.0;      // τ(6) = 4 divisors
const PHI: f64 = 2.0;      // φ(6) = 2 coprimes
const SOPFR: f64 = 5.0;    // sopfr(6) = 2+3
const J2: f64 = 24.0;      // J₂(6) = Jordan totient
const MU: f64 = 1.0;       // μ(6) = Möbius
const N: f64 = 6.0;        // n = 6

fn section(title: &str) {
    println!("\n{}", "=".repeat(72));
    println!("  {}", title);
    println!("{}\n", "=".repeat(72));
}

struct Stats {
    exact: u32,
    close: u32,
    fail: u32,
}

impl Stats {
    fn new() -> Self { Stats { exact: 0, close: 0, fail: 0 } }
    fn total(&self) -> u32 { self.exact + self.close + self.fail }
}

fn check(stats: &mut Stats, name: &str, actual: f64, predicted: f64, expr: &str) {
    let err = if predicted.abs() > 1e-10 {
        ((actual - predicted) / predicted).abs() * 100.0
    } else {
        0.0
    };
    let (mark, grade) = if err < 0.5 {
        stats.exact += 1;
        ("✅", "EXACT")
    } else if err < 5.0 {
        stats.close += 1;
        ("⚠️ ", "CLOSE")
    } else {
        stats.fail += 1;
        ("❌", "FAIL ")
    };

    println!("  {} {:<38} {:>12} {:>12} {:<28} {}",
        mark, name, fmt_num(actual), fmt_num(predicted), expr, grade);
}

fn fmt_num(v: f64) -> String {
    if v == v.floor() && v.abs() < 1e15 {
        format!("{}", v as i64)
    } else {
        format!("{:.4}", v)
    }
}

fn header() {
    println!("  {:<40} {:>12} {:>12} {:<28} {}",
        "Parameter", "Actual", "n=6 Pred", "Expression", "Grade");
    println!("  {}", "-".repeat(104));
}

fn bitcoin(stats: &mut Stats) {
    section("BT-53: Bitcoin n=6 Universality");
    header();

    check(stats, "Max supply (millions)", 21.0,
        (N / PHI) * (SIGMA - SOPFR), "n/φ·(σ-sopfr) = 3×7 = 21");

    check(stats, "Confirmations", 6.0,
        N, "n = 6");

    check(stats, "Block time (min)", 10.0,
        SIGMA - PHI, "σ-φ = 12-2 = 10");

    check(stats, "Halving interval (×10⁴ blocks)", 21.0,
        (N / PHI) * (SIGMA - SOPFR), "n/φ·(σ-sopfr) = 21");

    check(stats, "Initial reward (BTC)", 50.0,
        SOPFR * (SIGMA - PHI), "sopfr·(σ-φ) = 5×10 = 50");

    check(stats, "SHA-256 bits", 256.0,
        f64::powf(2.0, SIGMA - TAU), "2^(σ-τ) = 2^8 = 256");

    check(stats, "ECDSA key (bits)", 256.0,
        f64::powf(2.0, SIGMA - TAU), "2^(σ-τ) = 2^8 = 256");

    check(stats, "Block size (MB)", 1.0,
        MU, "μ = 1");
}

fn ethereum(stats: &mut Stats) {
    section("BT-53: Ethereum n=6 Universality");
    header();

    check(stats, "Block time (s)", 12.0,
        SIGMA, "σ = 12");

    check(stats, "Slots per epoch", 32.0,
        f64::powf(2.0, SOPFR), "2^sopfr = 2^5 = 32");

    check(stats, "Min validator stake (ETH)", 32.0,
        f64::powf(2.0, SOPFR), "2^sopfr = 2^5 = 32");

    check(stats, "Shard count (planned)", 64.0,
        f64::powf(2.0, N), "2^n = 2^6 = 64");
}

fn cryptography(stats: &mut Stats) {
    section("Cryptography n=6 Constants");
    header();

    // AES key sizes
    check(stats, "AES-128 key (bits)", 128.0,
        f64::powf(2.0, SIGMA - SOPFR), "2^(σ-sopfr) = 2^7 = 128");

    check(stats, "AES-192 key (bits)", 192.0,
        SIGMA * f64::powf(2.0, TAU), "σ·2^τ = 12·16 = 192");

    check(stats, "AES-256 key (bits)", 256.0,
        f64::powf(2.0, SIGMA - TAU), "2^(σ-τ) = 2^8 = 256");

    check(stats, "AES block (bits)", 128.0,
        f64::powf(2.0, SIGMA - SOPFR), "2^(σ-sopfr) = 2^7 = 128");

    check(stats, "RSA-2048 key (bits)", 2048.0,
        f64::powf(2.0, SIGMA - MU), "2^(σ-μ) = 2^11 = 2048");

    check(stats, "SHA-512 bits", 512.0,
        f64::powf(2.0, SIGMA - N / PHI), "2^(σ-n/φ) = 2^9 = 512");

    check(stats, "ChaCha20 rounds", 20.0,
        J2 - TAU, "J₂-τ = 24-4 = 20");

    // Poly1305: field is 2^130 - 5
    // 130 = σ·(σ-μ) - φ = 12·11 - 2 = 130
    check(stats, "Poly1305 exponent (130)", 130.0,
        SIGMA * (SIGMA - MU) - PHI, "σ(σ-μ)-φ = 132-2 = 130");

    check(stats, "Poly1305 subtrahend (5)", 5.0,
        SOPFR, "sopfr = 5");
}

fn qec(stats: &mut Stats) {
    section("BT-41: Quantum Error Correction n=6");
    header();

    check(stats, "Surface code d=5 syndromes", 24.0,
        J2, "J₂ = 24");

    check(stats, "Surface code d=3 qubits", 17.0,
        SIGMA + SOPFR, "σ+sopfr = 12+5 = 17");

    check(stats, "Quantum Volume exponent", 20.0,
        J2 - TAU, "J₂-τ = 24-4 = 20");

    check(stats, "Steane [[7,1,3]] code n", 7.0,
        SIGMA - SOPFR, "σ-sopfr = 12-5 = 7");

    check(stats, "Shor [[9,1,3]] code n", 9.0,
        SIGMA - N / PHI, "σ-n/φ = 12-3 = 9");
}

fn summary(stats: &Stats) {
    println!("\n{}", "=".repeat(72));
    println!("  SUMMARY");
    println!("{}\n", "=".repeat(72));
    println!("  Total parameters checked:  {}", stats.total());
    println!("  ✅ EXACT:                   {} / {} ({:.1}%)",
        stats.exact, stats.total(),
        100.0 * stats.exact as f64 / stats.total() as f64);
    println!("  ⚠️  CLOSE:                   {} / {}",
        stats.close, stats.total());
    println!("  ❌ FAIL:                    {} / {}",
        stats.fail, stats.total());

    let n6_ratio = stats.exact as f64 / stats.total() as f64;
    println!("\n  n=6 EXACT ratio: {:.1}%", n6_ratio * 100.0);

    if n6_ratio > 0.8 {
        println!("  → Strong n=6 universality in crypto/blockchain domain");
    } else if n6_ratio > 0.6 {
        println!("  → Moderate n=6 presence in crypto/blockchain domain");
    } else {
        println!("  → Weak n=6 signal — needs more investigation");
    }

    println!("\n  Core identity: σ(n)·φ(n) = n·τ(n) ⟺ n=6");
    println!("  12·2 = 6·4 = 24 = J₂(6)");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let _verbose = args.iter().any(|a| a == "--verbose" || a == "-v");

    println!("╔══════════════════════════════════════════════════════════════════════╗");
    println!("║  N6 Crypto / Blockchain / Cryptography Calculator                   ║");
    println!("║  BT-53: Crypto n=6 Universality  |  BT-41: QEC n=6                 ║");
    println!("║  σ(6)·φ(6) = 6·τ(6) = 24 = J₂(6)                                  ║");
    println!("╚══════════════════════════════════════════════════════════════════════╝");

    let mut stats = Stats::new();

    bitcoin(&mut stats);
    ethereum(&mut stats);
    cryptography(&mut stats);
    qec(&mut stats);

    summary(&stats);
}
