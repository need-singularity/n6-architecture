/// N6 Optics Calculator — 렌즈/망원경 광학 파라미터와 n=6 검증
///
/// Usage:
///   cargo run                     # 전체 분석
///   cargo run -- --lens           # 렌즈 설계 매칭
///   cargo run -- --telescope      # 망원경 파라미터 매칭
///   cargo run -- --tokamak-optics # 토카막 광학 진단 장치

use std::env;
use std::f64::consts::PI;

// n=6 constants
const SIGMA: f64 = 12.0;
const TAU: f64 = 4.0;
const PHI: f64 = 2.0;
const SOPFR: f64 = 5.0;
const J2: f64 = 24.0;
const MU: f64 = 1.0;
const N6: f64 = 6.0;

fn check_n6(value: f64, tolerance: f64) -> Option<(&'static str, f64)> {
    let candidates: Vec<(&str, f64)> = vec![
        ("n", N6), ("σ", SIGMA), ("τ", TAU), ("φ", PHI),
        ("sopfr", SOPFR), ("J₂", J2), ("μ", MU),
        ("σ-τ", SIGMA - TAU), ("σ-sopfr", SIGMA - SOPFR),
        ("σ-μ", SIGMA - MU), ("n/φ", N6 / PHI),
        ("τ²/σ", TAU * TAU / SIGMA), ("sopfr×φ", SOPFR * PHI),
        ("σ×sopfr", SIGMA * SOPFR), ("J₂-τ", J2 - TAU),
        ("σ+n", SIGMA + N6), ("σ+τ", SIGMA + TAU),
        ("σ×τ", SIGMA * TAU),
    ];
    for (name, target) in candidates {
        if target == 0.0 { continue; }
        let err = (value - target).abs() / target;
        if err <= tolerance {
            return Some((name, target));
        }
    }
    None
}

fn lens_analysis() {
    println!("\n  ═══ LENS DESIGN — n=6 매칭 ═══\n");

    let params: Vec<(&str, f64, &str)> = vec![
        // Seidel aberrations
        ("Seidel aberration types", 5.0, "spherical, coma, astigmatism, field curvature, distortion"),
        ("3rd order + 5th order", 2.0, "aberration order groups"),
        // Lens design
        ("Achromat elements", 2.0, "crown + flint glass"),
        ("Apochromat elements", 3.0, "3-element color correction"),
        ("Cooke triplet elements", 3.0, "classic 3-element design"),
        ("Double Gauss elements", 6.0, "standard camera lens"),
        ("Tessar elements", 4.0, "4-element Zeiss design"),
        ("Petzval elements", 4.0, "4-element portrait lens"),
        // Glass properties
        ("Abbe number V_d typical range", 20.0, "flint glass ~20"),
        ("Abbe number V_d crown", 60.0, "crown glass ~60 = σ×sopfr"),
        // Optical coating
        ("Quarter-wave layers (AR)", 4.0, "typical broadband AR coating"),
        ("MgF2 refractive index", 1.38, "n ≈ 1.38"),
        // Fiber optics
        ("Single-mode fiber core", 6.0, "~6 μm core diameter for 1550nm"),
        ("Multimode fiber core", 50.0, "50 or 62.5 μm"),
        ("Fiber NA typical", 0.12, "σ/100 = 0.12"),
        // Camera
        ("Standard f-stops", 8.0, "f/1.4, 2, 2.8, 4, 5.6, 8, 11, 16 = σ-τ stops"),
        ("Full-frame sensor (mm)", 24.0, "24×36mm = J₂ × (σ×n/φ)"),
        ("APS-C crop factor", 1.5, "= n/τ"),
    ];

    println!("  {:40} {:>8} {:>8} {:>8} {}", "Parameter", "Value", "n=6", "Grade", "Note");
    println!("  {}", "─".repeat(90));

    let mut exact = 0; let mut close = 0; let mut miss = 0;
    for (name, value, note) in &params {
        if let Some((expr, _target)) = check_n6(*value, 0.05) {
            let err = (value - _target).abs() / _target;
            let grade = if err < 0.01 { exact += 1; "EXACT" } else { close += 1; "CLOSE" };
            println!("  {:40} {:>8.2} {:>8} {:>8} {}", name, value, expr, grade, note);
        } else {
            miss += 1;
            println!("  {:40} {:>8.2} {:>8} {:>8} {}", name, value, "-", "MISS", note);
        }
    }
    println!("\n  EXACT={}, CLOSE={}, MISS={}", exact, close, miss);
}

fn telescope_analysis() {
    println!("\n  ═══ TELESCOPE — n=6 매칭 ═══\n");

    let params: Vec<(&str, f64, &str)> = vec![
        // Telescope types
        ("Major telescope types", 3.0, "refractor, reflector, catadioptric"),
        ("Mirror shapes", 4.0, "spherical, parabolic, hyperbolic, elliptical"),
        // Famous telescopes
        ("HST primary diameter (m)", 2.4, "= J₂/10?"),
        ("HST instruments", 5.0, "WFC3, COS, STIS, ACS, FGS"),
        ("JWST mirror segments", 18.0, "= σ+n"),
        ("JWST wavelength bands", 3.0, "NIR, MIR, visible = n/φ"),
        ("JWST instruments", 4.0, "NIRCam, NIRSpec, MIRI, FGS/NIRISS"),
        ("JWST Lagrange point", 2.0, "L2 = φ"),
        ("ELT primary segments", 798.0, "not n=6 related"),
        ("ELT mirror diameter (m)", 39.3, "not n=6"),
        // Radio
        ("ALMA antennas (12m)", 50.0, "50 = σ×τ+φ? weak"),
        ("ALMA antennas (7m)", 12.0, "= σ!"),
        ("SKA-Mid dishes", 197.0, "not n=6"),
        // Optical design
        ("Cassegrain mirrors", 2.0, "primary + secondary = φ"),
        ("Nasmyth foci", 2.0, "= φ"),
        ("Ritchey-Chrétien mirrors", 2.0, "= φ"),
        // Adaptive optics
        ("AO Zernike modes (typical)", 12.0, "first 12 modes = σ? or more"),
        ("DM actuators across pupil", 12.0, "typical 12×12 = σ×σ?"),
        ("Wavefront sensor types", 3.0, "Shack-Hartmann, pyramid, curvature = n/φ"),
        // CCD
        ("CCD readout channels", 4.0, "typical quad-readout = τ"),
        ("Bayer filter pattern", 4.0, "RGGB = τ colors (2G+R+B)"),
        ("Color channels", 3.0, "RGB = n/φ"),
    ];

    println!("  {:40} {:>8} {:>8} {:>8} {}", "Parameter", "Value", "n=6", "Grade", "Note");
    println!("  {}", "─".repeat(90));

    let mut exact = 0; let mut close = 0; let mut miss = 0;
    for (name, value, note) in &params {
        if let Some((expr, _target)) = check_n6(*value, 0.05) {
            let err = (value - _target).abs() / _target;
            let grade = if err < 0.01 { exact += 1; "EXACT" } else { close += 1; "CLOSE" };
            println!("  {:40} {:>8.1} {:>8} {:>8} {}", name, value, expr, grade, note);
        } else {
            miss += 1;
            println!("  {:40} {:>8.1} {:>8} {:>8} {}", name, value, "-", "MISS", note);
        }
    }
    println!("\n  EXACT={}, CLOSE={}, MISS={}", exact, close, miss);
}

fn tokamak_optics() {
    println!("\n  ═══ TOKAMAK OPTICAL DIAGNOSTICS — n=6 매칭 ═══\n");

    let params: Vec<(&str, f64, &str)> = vec![
        // Tokamak diagnostic types
        ("Major diagnostic categories", 6.0, "magnetic, particle, spectroscopy, imaging, microwave, neutron"),
        ("Thomson scattering points", 12.0, "KSTAR: ~12 spatial points (σ)"),
        ("ECE channels", 24.0, "KSTAR: ~24 channels (J₂)"),
        ("Interferometer chords", 5.0, "KSTAR: ~5 chords (sopfr)"),
        ("Soft X-ray arrays", 2.0, "upper + lower = φ"),
        ("Visible cameras", 4.0, "KSTAR: ~4 fast cameras (τ)"),
        ("Bolometer channels", 24.0, "KSTAR: ~24 channels (J₂)"),
        ("Reflectometry frequencies", 6.0, "KSTAR: ~6 frequency bands (n)"),
        ("CXRS viewing lines", 12.0, "KSTAR: ~12 spatial channels (σ)"),
        ("MSE channels", 8.0, "KSTAR: ~8 spatial channels (σ-τ)"),
        // Optical components in tokamak
        ("Diagnostic port windows", 6.0, "KSTAR midplane: ~6 ports"),
        ("Mirror types for diagnostics", 3.0, "first/retro/relay = n/φ"),
        ("Spectrometer gratings (typical)", 3.0, "UV/visible/IR = n/φ"),
        // Laser diagnostics
        ("Thomson scattering lasers", 2.0, "KSTAR: Nd:YAG dual = φ"),
        ("Laser wavelength (Nd:YAG, nm)", 1064.0, "not n=6"),
    ];

    println!("  {:40} {:>8} {:>8} {:>8} {}", "Parameter", "Value", "n=6", "Grade", "Note");
    println!("  {}", "─".repeat(90));

    let mut exact = 0; let mut close = 0; let mut miss = 0;
    for (name, value, note) in &params {
        if let Some((expr, _target)) = check_n6(*value, 0.05) {
            let err = (value - _target).abs() / _target;
            let grade = if err < 0.01 { exact += 1; "EXACT" } else { close += 1; "CLOSE" };
            println!("  {:40} {:>8.1} {:>8} {:>8} {}", name, value, expr, grade, note);
        } else {
            miss += 1;
            println!("  {:40} {:>8.1} {:>8} {:>8} {}", name, value, "-", "MISS", note);
        }
    }

    println!("\n  EXACT={}, CLOSE={}, MISS={}", exact, close, miss);
    println!("\n  ═══ KEY FINDING ═══");
    println!("  KSTAR 진단 장치에서 n=6 상수가 자주 등장:");
    println!("    Thomson: σ=12 points, ECE: J₂=24 channels");
    println!("    Bolometer: J₂=24, CXRS: σ=12, MSE: σ-τ=8");
    println!("    Reflectometry: n=6 bands, Interferometer: sopfr=5");
    println!("    이것은 설계자가 n=6을 의도한 것이 아니라");
    println!("    '실용적 채널 수'가 n=6 상수와 일치하는 패턴");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("--all");

    println!("╔══════════════════════════════════════════════════════╗");
    println!("║  N6 OPTICS CALCULATOR                               ║");
    println!("║  렌즈/망원경/토카막 광학 진단 — n=6 검증              ║");
    println!("╚══════════════════════════════════════════════════════╝");

    match mode {
        "--lens" => lens_analysis(),
        "--telescope" => telescope_analysis(),
        "--tokamak-optics" => tokamak_optics(),
        _ => {
            lens_analysis();
            telescope_analysis();
            tokamak_optics();

            println!("\n  ═══ OVERALL SUMMARY ═══");
            println!("  Strongest matches:");
            println!("    ✅ Double Gauss lens: 6 elements = n");
            println!("    ✅ JWST: 18 segments = σ+n");
            println!("    ✅ Full-frame sensor: 24mm = J₂");
            println!("    ✅ Seidel aberrations: 5 types = sopfr");
            println!("    ✅ ALMA 7m antennas: 12 = σ");
            println!("    ✅ AO Zernike modes: 12 = σ");
            println!("    ✅ KSTAR Thomson: 12 pts, ECE: 24 ch, Bolometer: 24 ch");
            println!("    ✅ f-stop count: 8 = σ-τ");
        }
    }
}
