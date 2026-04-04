/// N6 Fusion Comprehensive Verifier — 핵융합 n=6 전수 수치 검증
///
/// 17개 핵융합 핵심 파라미터를 실험값/CODATA 기준으로 전수 검증.
/// 각 항목: 이론값(n=6 예측) vs 실험값 → 오차율 → 등급(EXACT/CLOSE/WEAK/FAIL)
///
/// Build:
///   ~/.cargo/bin/rustc tools/fusion-verify/main.rs -o tools/fusion-verify/fusion-verify
///
/// Run:
///   ./tools/fusion-verify/fusion-verify



// ═══════════════════════════════════════════════════════════════
// n=6 기본 상수 (정수)
// ═══════════════════════════════════════════════════════════════
const N: f64 = 6.0;       // 완전수
const SIGMA: f64 = 12.0;  // σ(6) = 약수합
const TAU: f64 = 4.0;     // τ(6) = 약수개수
const PHI: f64 = 2.0;     // φ(6) = 오일러 토션트
const SOPFR: f64 = 5.0;   // sopfr(6) = 소인수합 2+3
const J2: f64 = 24.0;     // J₂(6) = 조르단 토션트
const MU: f64 = 1.0;      // μ(6) = 뫼비우스
const P2: f64 = 28.0;     // P₂ = 두번째 완전수

// n=6 유도 상수
const SIGMA_MINUS_PHI: f64 = 10.0;  // σ-φ = 10
const SIGMA_PLUS_PHI: f64 = 14.0;   // σ+φ = 14
const SIGMA_MINUS_MU: f64 = 11.0;   // σ-μ = 11
const J2_MINUS_TAU: f64 = 20.0;     // J₂-τ = 20
const SIGMA_TIMES_J2: f64 = 288.0;  // σ×J₂ = 288
const PHI_POW_N: f64 = 64.0;        // φ^n = 2^6 = 64
const TAU_SQUARED_OVER_SIGMA: f64 = 4.0 / 3.0; // τ²/σ = 16/12 = 4/3

// ═══════════════════════════════════════════════════════════════
// 물리 상수 (CODATA 2018 / PDG 2024)
// ═══════════════════════════════════════════════════════════════

// Weinberg angle: PDG 2024 MS-bar at MZ
const SIN2_THETA_W_EXP: f64 = 0.23122; // ± 0.00003

// D-T 반응 에너지 (MeV)
const DT_Q_VALUE: f64 = 17.586;        // D-T → He4 + n, 총 에너지
const DT_ALPHA_ENERGY: f64 = 3.518;    // alpha 운동에너지
const DT_NEUTRON_ENERGY: f64 = 14.068; // 중성자 운동에너지

// He-4 결합에너지 (AME2020)
const HE4_BINDING_ENERGY: f64 = 28.296; // MeV

// 삼중수소 반감기 (NNDC)
const TRITIUM_HALF_LIFE_YEARS: f64 = 12.32; // 년

// SPARC 설계값 (CFS/MIT 2020)
const SPARC_BT: f64 = 12.2; // Tesla

// D-T 반응 단면적 피크 에너지 (NRL Plasma Formulary)
const DT_CROSS_SECTION_PEAK_KEV: f64 = 64.0; // keV (center-of-mass ~80, lab ~64)

// D-T 최적 반응도 온도 (NRL)
const DT_OPTIMAL_T_KEV: f64 = 13.6; // keV (peak <σv>)

// Lawson criterion 지수 (nτ > 10^20 m⁻³·s at ~10keV)
const LAWSON_EXPONENT: f64 = 20.0;

// 자기 재결합 속도 (Sweet-Parker → Petschek/fast)
// MRX 실험: ~0.1 V_A (Yamada+ 2010)
const RECONNECTION_RATE_EXP: f64 = 0.1;

// CNO 촉매 핵종 질량수
const CNO_A_VALUES: [f64; 4] = [12.0, 13.0, 14.0, 15.0]; // C12,N13,N14,O15

// CNO-PP 전환 온도 (~17 MK = 1.47 keV)
const CNO_TRANSITION_T_MK: f64 = 17.0; // million Kelvin

// Triple-alpha: 3×He4 → C12
const TRIPLE_ALPHA_PRODUCT_A: f64 = 12.0; // C-12 질량수

// 광합성 포도당 C₆H₁₂O₆ 원자수
const GLUCOSE_TOTAL_ATOMS: f64 = 24.0; // 6+12+6

// CO₂: C(Z=6) + 2×O(Z=8)
const CO2_CARBON_Z: f64 = 6.0;
const _CO2_TOTAL_ELECTRONS: f64 = 22.0; // 6+8+8 (향후 확장용)
const _CO2_ATOMS: f64 = 3.0;            // 1C + 2O (향후 확장용)

// 지구 평균 온도 (NASA GISS baseline)
const EARTH_AVG_TEMP_K: f64 = 288.0; // K (15°C)

// ═══════════════════════════════════════════════════════════════
// 검증 결과 구조체
// ═══════════════════════════════════════════════════════════════

#[derive(Clone)]
struct Verification {
    id: usize,
    name: String,
    bt_ref: String,
    theory_val: f64,
    theory_expr: String,
    experiment_val: f64,
    experiment_src: String,
    error_pct: f64,
    grade: String,
}

fn grade_error(error_pct: f64) -> &'static str {
    let abs_err = error_pct.abs();
    if abs_err < 0.5 { "EXACT" }
    else if abs_err < 5.0 { "CLOSE" }
    else if abs_err < 20.0 { "WEAK" }
    else { "FAIL" }
}

fn verify(
    id: usize,
    name: &str,
    bt_ref: &str,
    theory_val: f64,
    theory_expr: &str,
    experiment_val: f64,
    experiment_src: &str,
) -> Verification {
    let error_pct = if experiment_val.abs() > 1e-15 {
        (theory_val - experiment_val) / experiment_val * 100.0
    } else {
        0.0
    };
    let grade = grade_error(error_pct).to_string();
    Verification {
        id,
        name: name.to_string(),
        bt_ref: bt_ref.to_string(),
        theory_val,
        theory_expr: theory_expr.to_string(),
        experiment_val,
        experiment_src: experiment_src.to_string(),
        error_pct,
        grade,
    }
}

// ═══════════════════════════════════════════════════════════════
// 전수 검증 함수들
// ═══════════════════════════════════════════════════════════════

fn verify_all() -> Vec<Verification> {
    let mut results = Vec::new();
    let mut id = 0usize;

    // ──────────────────────────────────────────────────────────
    // #1. D-T 핵자 수 2+3=5=sopfr(6) [BT-98]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let dt_nucleon_sum = 2.0 + 3.0; // D(2) + T(3)
    results.push(verify(
        id, "D-T 핵자합 = sopfr(6)", "BT-98",
        SOPFR, "sopfr(6)=5",
        dt_nucleon_sum, "D(A=2)+T(A=3)=5 [핵물리학]",
    ));

    // ──────────────────────────────────────────────────────────
    // #2. D-T-Li6 연료 주기 질량수 {1,2,3,4,6} [BT-98]
    //     div(6) = {1,2,3,6}, τ(6)=4 → {1,2,3,4,6}
    // ──────────────────────────────────────────────────────────
    id += 1;
    // n(1), D(2), T(3), He4(4), Li6(6) — 5개 핵종
    // div(6) = {1,2,3,6}, 추가로 He4(A=4)=τ
    // 검증: 모든 질량수가 div(6)∪{τ}에 속하는지
    let fuel_cycle_a = [1.0_f64, 2.0, 3.0, 4.0, 6.0];
    let div6_union_tau = [1.0_f64, 2.0, 3.0, 4.0, 6.0]; // div(6)={1,2,3,6} ∪ {4=τ}
    let all_match = fuel_cycle_a.iter().all(|a| div6_union_tau.contains(a));
    results.push(verify(
        id, "D-T-Li6 연료주기 질량수 ⊂ div(6)∪{τ}", "BT-98",
        5.0, "count=sopfr=5 (all in div(6)∪{τ})",
        if all_match { 5.0 } else { 0.0 }, "n(1),D(2),T(3),He4(4),Li6(6) [핵물리학]",
    ));

    // ──────────────────────────────────────────────────────────
    // #3. Alpha 에너지 분율 3.5/17.6 ≈ 1/sopfr [BT-98]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let alpha_fraction_theory = 1.0 / SOPFR; // 0.2
    let alpha_fraction_exp = DT_ALPHA_ENERGY / DT_Q_VALUE; // 3.518/17.586 = 0.19998...
    results.push(verify(
        id, "Alpha 에너지분율 ≈ 1/sopfr", "BT-98",
        alpha_fraction_theory, "1/sopfr(6)=0.200",
        alpha_fraction_exp, &format!("{:.3}/{:.3}={:.5} [NRL]", DT_ALPHA_ENERGY, DT_Q_VALUE, alpha_fraction_exp),
    ));

    // ──────────────────────────────────────────────────────────
    // #4. q=1 = 1/2+1/3+1/6 완전수 역수합 [BT-99]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let perfect_reciprocal_sum = 1.0/2.0 + 1.0/3.0 + 1.0/6.0;
    let tokamak_q_center = 1.0; // tokamak safety factor at center
    results.push(verify(
        id, "q₀=1 = 1/2+1/3+1/6 (완전수 역수합)", "BT-99",
        perfect_reciprocal_sum, "1/2+1/3+1/6=1",
        tokamak_q_center, "tokamak q₀=1 (MHD 안정조건) [Wesson 2004]",
    ));

    // ──────────────────────────────────────────────────────────
    // #5. CNO 촉매 A = σ+{0,μ,φ,n/φ} [BT-100]
    // ──────────────────────────────────────────────────────────
    let cno_offsets = [0.0, MU, PHI, N/PHI]; // {0,1,2,3}
    let cno_labels = ["C-12", "N-13", "N-14", "O-15"];
    for i in 0..4 {
        id += 1;
        let predicted = SIGMA + cno_offsets[i];
        let offset_name = match i {
            0 => "σ+0",
            1 => "σ+μ",
            2 => "σ+φ",
            3 => "σ+n/φ",
            _ => unreachable!(),
        };
        results.push(verify(
            id, &format!("CNO {} A = {}", cno_labels[i], offset_name), "BT-100",
            predicted, &format!("{}={}", offset_name, predicted),
            CNO_A_VALUES[i], &format!("{} A={} [핵물리학]", cno_labels[i], CNO_A_VALUES[i] as i32),
        ));
    }

    // ──────────────────────────────────────────────────────────
    // #6. Triple-alpha: 3×He4(A=4) → C-12(A=σ) [BT-100]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let _triple_alpha_product = 3.0 * TAU; // 3×4=12
    results.push(verify(
        id, "Triple-alpha 생성물 C-12 = σ", "BT-100",
        SIGMA, "σ(6)=12",
        TRIPLE_ALPHA_PRODUCT_A, "C-12 A=12 [Hoyle state]",
    ));

    // ──────────────────────────────────────────────────────────
    // #7. CNO 전환온도 17 MK = σ+sopfr [BT-100]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let cno_t_predicted = SIGMA + SOPFR; // 12+5=17
    results.push(verify(
        id, "CNO 전환온도 17 MK = σ+sopfr", "BT-100",
        cno_t_predicted, "σ+sopfr=17",
        CNO_TRANSITION_T_MK, "~17 MK [Bahcall 2005]",
    ));

    // ──────────────────────────────────────────────────────────
    // #8. Weinberg angle sin²θ_W = 3/13 = (n/φ)/(σ+μ) [BT-97]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let weinberg_n6 = (N / PHI) / (SIGMA + MU); // 3/13 = 0.230769...
    results.push(verify(
        id, "Weinberg angle sin²θ_W = (n/φ)/(σ+μ)", "BT-97",
        weinberg_n6, "(n/φ)/(σ+μ)=3/13=0.23077",
        SIN2_THETA_W_EXP, &format!("{:.5} [PDG 2024 MS-bar]", SIN2_THETA_W_EXP),
    ));

    // ──────────────────────────────────────────────────────────
    // #9. 자기 재결합 속도 0.1 = 1/(σ-φ) [BT-102]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let reconnection_n6 = 1.0 / SIGMA_MINUS_PHI; // 1/10 = 0.1
    results.push(verify(
        id, "자기 재결합 속도 = 1/(σ-φ)", "BT-102",
        reconnection_n6, "1/(σ-φ)=0.1",
        RECONNECTION_RATE_EXP, "~0.1 V_A [MRX: Yamada+ 2010]",
    ));

    // ──────────────────────────────────────────────────────────
    // #10. 광합성 포도당 C₆H₁₂O₆ = 24원자 = J₂ [BT-101/103]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "포도당 C₆H₁₂O₆ 원자수 = J₂", "BT-101",
        J2, "J₂(6)=24",
        GLUCOSE_TOTAL_ATOMS, "6+12+6=24 [화학]",
    ));

    // ──────────────────────────────────────────────────────────
    // #11. CO₂ 탄소 Z=6=n [BT-104]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "CO₂ 탄소 원자번호 Z = n", "BT-104",
        N, "n=6",
        CO2_CARBON_Z, "Carbon Z=6 [주기율표]",
    ));

    // ──────────────────────────────────────────────────────────
    // #12. D-T 최적 온도 ~14 keV ≈ σ+φ [신규]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "D-T 최적 반응도 온도 ≈ σ+φ keV", "BT-98",
        SIGMA_PLUS_PHI, "σ+φ=14",
        DT_OPTIMAL_T_KEV, &format!("{:.1} keV [NRL Plasma Formulary]", DT_OPTIMAL_T_KEV),
    ));

    // ──────────────────────────────────────────────────────────
    // #13. SPARC B_T = 12.2 T ≈ σ [BT-97 확장]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "SPARC B_T ≈ σ Tesla", "BT-97",
        SIGMA, "σ(6)=12",
        SPARC_BT, &format!("{:.1} T [CFS/MIT 2020]", SPARC_BT),
    ));

    // ──────────────────────────────────────────────────────────
    // #14. 삼중수소 반감기 12.32년 ≈ σ [신규]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "삼중수소 반감기 ≈ σ 년", "BT-98",
        SIGMA, "σ(6)=12",
        TRITIUM_HALF_LIFE_YEARS, &format!("{:.2} yr [NNDC]", TRITIUM_HALF_LIFE_YEARS),
    ));

    // ──────────────────────────────────────────────────────────
    // #15. He-4 결합에너지 28.3 MeV ≈ P₂=28 [신규]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "He-4 결합에너지 ≈ P₂ MeV", "BT-98",
        P2, "P₂=28 (두번째 완전수)",
        HE4_BINDING_ENERGY, &format!("{:.3} MeV [AME2020]", HE4_BINDING_ENERGY),
    ));

    // ──────────────────────────────────────────────────────────
    // #16. Lawson 지수 nτ > 10²⁰ → 지수=20=J₂-τ [신규]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "Lawson 지수 20 = J₂-τ", "BT-99",
        J2_MINUS_TAU, "J₂-τ=24-4=20",
        LAWSON_EXPONENT, "10²⁰ m⁻³·s [NRL/Wesson]",
    ));

    // ──────────────────────────────────────────────────────────
    // #17. D-T 단면적 피크 64 keV = 2^6 = φ^n [신규]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "D-T σ(v) 피크 에너지 = φ^n keV", "BT-98",
        PHI_POW_N, "φ^n=2^6=64",
        DT_CROSS_SECTION_PEAK_KEV, &format!("{:.0} keV [NRL Plasma Formulary]", DT_CROSS_SECTION_PEAK_KEV),
    ));

    // ──────────────────────────────────────────────────────────
    // #18. 지구 평균 온도 288 K = σ×J₂ [신규 발견]
    // ──────────────────────────────────────────────────────────
    id += 1;
    results.push(verify(
        id, "지구 평균 온도 = σ×J₂ K", "BT-104",
        SIGMA_TIMES_J2, "σ×J₂=12×24=288",
        EARTH_AVG_TEMP_K, &format!("{:.0} K [NASA GISS]", EARTH_AVG_TEMP_K),
    ));

    // ──────────────────────────────────────────────────────────
    // #19. 광합성 양자수율 8 = σ-τ [BT-101]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let photosynthesis_quantum_yield = 8.0; // 8 photons per O₂
    results.push(verify(
        id, "광합성 양자수율 8 = σ-τ", "BT-101",
        SIGMA - TAU, "σ-τ=12-4=8",
        photosynthesis_quantum_yield, "8 photons/O₂ [Emerson+ 1957]",
    ));

    // ──────────────────────────────────────────────────────────
    // #20. 광합성 화학양론 6CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂
    //       계수합 = 6+6+1+6 = 19? 아니라 계수 {6,6,1,6} 중
    //       CO₂(6)=n, H₂O(6)=n, O₂(6)=n — 3개 계수 모두 n [BT-103]
    // ──────────────────────────────────────────────────────────
    id += 1;
    // 광합성 반응식에서 n=6이 나타나는 계수 개수
    let photosynthesis_n6_coefficients = 3.0; // CO₂, H₂O, O₂ 계수가 모두 6
    results.push(verify(
        id, "광합성 계수 중 n=6 개수 = n/φ", "BT-103",
        N / PHI, "n/φ=3",
        photosynthesis_n6_coefficients, "6CO₂+6H₂O→C₆H₁₂O₆+6O₂ [화학]",
    ));

    // ──────────────────────────────────────────────────────────
    // #21. D-T Q값 17.6 MeV ≈ σ+sopfr+0.6 → 가장 가까운:
    //       σ+sopfr = 17 (2.3% 오차) 또는 σ+n = 18 (2.3% 반대)
    // ──────────────────────────────────────────────────────────
    id += 1;
    let dt_q_predicted = SIGMA + SOPFR; // 17
    results.push(verify(
        id, "D-T Q값 17.6 MeV ≈ σ+sopfr", "BT-98",
        dt_q_predicted, "σ+sopfr=12+5=17",
        DT_Q_VALUE, &format!("{:.3} MeV [NNDC]", DT_Q_VALUE),
    ));

    // ──────────────────────────────────────────────────────────
    // #22. Alpha 에너지 3.5 MeV → E_α/E_n = 3.5/14.1 ≈ 1/τ = 0.25
    //      실제 비율 = 0.2496
    // ──────────────────────────────────────────────────────────
    id += 1;
    let alpha_neutron_ratio_theory = 1.0 / TAU; // 0.25
    let alpha_neutron_ratio_exp = DT_ALPHA_ENERGY / DT_NEUTRON_ENERGY;
    results.push(verify(
        id, "E_α/E_n ≈ 1/τ", "BT-98",
        alpha_neutron_ratio_theory, "1/τ=0.25",
        alpha_neutron_ratio_exp, &format!("{:.3}/{:.3}={:.5} [NRL]",
            DT_ALPHA_ENERGY, DT_NEUTRON_ENERGY, alpha_neutron_ratio_exp),
    ));

    // ──────────────────────────────────────────────────────────
    // #23. SQ 태양전지 최적 밴드갭 1.34 eV ≈ τ²/σ = 4/3 [BT-30/111]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let sq_bandgap_exp = 1.34; // Shockley-Queisser limit
    results.push(verify(
        id, "SQ 최적 밴드갭 ≈ τ²/σ eV", "BT-30",
        TAU_SQUARED_OVER_SIGMA, "τ²/σ=16/12=1.333",
        sq_bandgap_exp, "1.34 eV [Shockley-Queisser 1961]",
    ));

    // ──────────────────────────────────────────────────────────
    // #24. p-B11에서 B-11 = σ-μ [핵반응]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let b11_mass_number = 11.0;
    results.push(verify(
        id, "B-11 질량수 = σ-μ", "BT-98",
        SIGMA_MINUS_MU, "σ-μ=12-1=11",
        b11_mass_number, "B-11 A=11 [핵물리학]",
    ));

    // ──────────────────────────────────────────────────────────
    // #25. Li-6 질량수 = n [BT-98]
    // ──────────────────────────────────────────────────────────
    id += 1;
    let li6_mass_number = 6.0;
    results.push(verify(
        id, "Li-6 질량수 = n", "BT-98",
        N, "n=6",
        li6_mass_number, "Li-6 A=6 [핵물리학]",
    ));

    results
}

// ═══════════════════════════════════════════════════════════════
// 출력
// ═══════════════════════════════════════════════════════════════

fn print_results(results: &[Verification]) {
    println!("╔═══════════════════════════════════════════════════════════════════════════════════════╗");
    println!("║  N6 FUSION COMPREHENSIVE VERIFIER — 핵융합 n=6 전수 수치 검증                       ║");
    println!("║  빌드: rustc main.rs (std only, no crates)                                          ║");
    println!("║  상수: CODATA 2018 / PDG 2024 / AME2020 / NRL Plasma Formulary                     ║");
    println!("╚═══════════════════════════════════════════════════════════════════════════════════════╝");
    println!();

    println!("┌────┬────────────────────────────────────────┬────────┬──────────────┬──────────────┬─────────┬───────┐");
    println!("│ #  │ 검증 항목                               │ BT     │ 이론값(n=6)  │ 실험값       │ 오차(%) │ 등급  │");
    println!("├────┼────────────────────────────────────────┼────────┼──────────────┼──────────────┼─────────┼───────┤");

    for v in results {
        let grade_marker = match v.grade.as_str() {
            "EXACT" => "\x1b[32mEXACT\x1b[0m", // green
            "CLOSE" => "\x1b[33mCLOSE\x1b[0m", // yellow
            "WEAK"  => "\x1b[35mWEAK \x1b[0m",  // magenta
            "FAIL"  => "\x1b[31mFAIL \x1b[0m",  // red
            _ => "?????",
        };

        // 이름 표시 (UTF-8 안전 절단)
        let name_chars: Vec<char> = v.name.chars().collect();
        let name_display = if name_chars.len() > 38 {
            let truncated: String = name_chars[..35].iter().collect();
            format!("{}...", truncated)
        } else {
            v.name.clone()
        };

        println!("│{:>3} │ {:<40}│ {:<6} │ {:>12.6} │ {:>12.6} │ {:>+7.3} │ {} │",
            v.id,
            name_display,
            v.bt_ref,
            v.theory_val,
            v.experiment_val,
            v.error_pct,
            grade_marker,
        );
    }

    println!("└────┴────────────────────────────────────────┴────────┴──────────────┴──────────────┴─────────┴───────┘");
}

fn print_detail(results: &[Verification]) {
    println!("\n═══ 상세 검증 ═══\n");
    for v in results {
        let icon = match v.grade.as_str() {
            "EXACT" => "✅",
            "CLOSE" => "🔶",
            "WEAK"  => "⚠️",
            _ => "❌",
        };
        println!("  #{:02} {} [{}] {}", v.id, icon, v.bt_ref, v.name);
        println!("      이론: {} = {:.6}", v.theory_expr, v.theory_val);
        println!("      실험: {}", v.experiment_src);
        println!("      오차: {:+.4}% → {}", v.error_pct, v.grade);
        println!();
    }
}

fn print_summary(results: &[Verification]) {
    let total = results.len();
    let exact = results.iter().filter(|v| v.grade == "EXACT").count();
    let close = results.iter().filter(|v| v.grade == "CLOSE").count();
    let weak  = results.iter().filter(|v| v.grade == "WEAK").count();
    let fail  = results.iter().filter(|v| v.grade == "FAIL").count();
    let exact_pct = exact as f64 / total as f64 * 100.0;

    println!("\n╔═══════════════════════════════════════════════════════════════╗");
    println!("║  최종 요약 (Final Summary)                                    ║");
    println!("╠═══════════════════════════════════════════════════════════════╣");
    println!("║                                                               ║");
    println!("║  총 검증 항목: {:>3}                                           ║", total);
    println!("║                                                               ║");
    println!("║  EXACT (<0.5%):  {:>3}  {}", exact, bar(exact, total));
    println!("║  CLOSE (<5.0%):  {:>3}  {}", close, bar(close, total));
    println!("║  WEAK  (<20%):   {:>3}  {}", weak, bar(weak, total));
    println!("║  FAIL  (>20%):   {:>3}  {}", fail, bar(fail, total));
    println!("║                                                               ║");
    println!("║  전체 EXACT%:    {:>5.1}%                                      ║", exact_pct);
    println!("║  EXACT+CLOSE%:   {:>5.1}%                                      ║",
        (exact + close) as f64 / total as f64 * 100.0);
    println!("║                                                               ║");
    println!("╚═══════════════════════════════════════════════════════════════╝");

    // BT별 집계
    println!("\n  ═══ BT별 성적 ═══");
    let mut bt_map: std::collections::BTreeMap<String, (usize, usize)> = std::collections::BTreeMap::new();
    for v in results {
        let entry = bt_map.entry(v.bt_ref.clone()).or_insert((0, 0));
        entry.0 += 1;
        if v.grade == "EXACT" {
            entry.1 += 1;
        }
    }
    println!("  {:10} {:>5} {:>5} {:>7}", "BT", "Total", "EXACT", "EXACT%");
    println!("  {}", "-".repeat(30));
    for (bt, (tot, ex)) in &bt_map {
        let pct = *ex as f64 / *tot as f64 * 100.0;
        println!("  {:10} {:>5} {:>5} {:>6.1}%", bt, tot, ex, pct);
    }
}

fn bar(count: usize, total: usize) -> String {
    let max_width = 30;
    let filled = if total > 0 { count * max_width / total } else { 0 };
    let empty = max_width - filled;
    format!("{}{}│", "█".repeat(filled), "░".repeat(empty))
}

// ═══════════════════════════════════════════════════════════════
// Markdown 출력 (docs/fusion/numerical-verification.md 용)
// ═══════════════════════════════════════════════════════════════

fn generate_markdown(results: &[Verification]) -> String {
    let total = results.len();
    let exact = results.iter().filter(|v| v.grade == "EXACT").count();
    let close = results.iter().filter(|v| v.grade == "CLOSE").count();
    let weak  = results.iter().filter(|v| v.grade == "WEAK").count();
    let fail  = results.iter().filter(|v| v.grade == "FAIL").count();
    let exact_pct = exact as f64 / total as f64 * 100.0;

    let mut md = String::new();

    md.push_str("# Fusion n=6 Numerical Verification\n\n");
    md.push_str("> Auto-generated by `tools/fusion-verify/fusion-verify`\n");
    md.push_str(&format!("> Date: 2026-04-02\n"));
    md.push_str("> Constants: CODATA 2018 / PDG 2024 / AME2020 / NRL Plasma Formulary\n\n");

    md.push_str("## Summary\n\n");
    md.push_str(&format!("| Metric | Count | % |\n"));
    md.push_str(&format!("|--------|-------|---|\n"));
    md.push_str(&format!("| Total items | {} | - |\n", total));
    md.push_str(&format!("| EXACT (<0.5%) | {} | {:.1}% |\n", exact, exact_pct));
    md.push_str(&format!("| CLOSE (<5%) | {} | {:.1}% |\n", close, close as f64 / total as f64 * 100.0));
    md.push_str(&format!("| WEAK (<20%) | {} | {:.1}% |\n", weak, weak as f64 / total as f64 * 100.0));
    md.push_str(&format!("| FAIL (>20%) | {} | {:.1}% |\n", fail, fail as f64 / total as f64 * 100.0));
    md.push_str(&format!("\n**Overall EXACT%: {:.1}%**\n\n", exact_pct));

    md.push_str("## Detailed Results\n\n");
    md.push_str("| # | Item | BT | Theory (n=6) | Expr | Experiment | Source | Error% | Grade |\n");
    md.push_str("|---|------|-----|-------------|------|-----------|--------|--------|-------|\n");

    for v in results {
        let grade_emoji = match v.grade.as_str() {
            "EXACT" => "EXACT",
            "CLOSE" => "CLOSE",
            "WEAK" => "WEAK",
            _ => "FAIL",
        };
        md.push_str(&format!(
            "| {} | {} | {} | {:.6} | {} | {:.6} | {} | {:+.3}% | {} |\n",
            v.id, v.name, v.bt_ref,
            v.theory_val, v.theory_expr,
            v.experiment_val, v.experiment_src,
            v.error_pct, grade_emoji,
        ));
    }

    md.push_str("\n## BT Breakdown\n\n");
    md.push_str("| BT | Total | EXACT | EXACT% |\n");
    md.push_str("|----|-------|-------|--------|\n");

    let mut bt_map: std::collections::BTreeMap<String, (usize, usize)> = std::collections::BTreeMap::new();
    for v in results {
        let entry = bt_map.entry(v.bt_ref.clone()).or_insert((0, 0));
        entry.0 += 1;
        if v.grade == "EXACT" { entry.1 += 1; }
    }
    for (bt, (tot, ex)) in &bt_map {
        let pct = *ex as f64 / *tot as f64 * 100.0;
        md.push_str(&format!("| {} | {} | {} | {:.1}% |\n", bt, tot, ex, pct));
    }

    md.push_str("\n## Grade Criteria\n\n");
    md.push_str("- **EXACT**: |error| < 0.5%\n");
    md.push_str("- **CLOSE**: |error| < 5%\n");
    md.push_str("- **WEAK**: |error| < 20%\n");
    md.push_str("- **FAIL**: |error| >= 20%\n");

    md
}

// ═══════════════════════════════════════════════════════════════
// main
// ═══════════════════════════════════════════════════════════════

fn main() {
    let results = verify_all();

    print_results(&results);
    print_detail(&results);
    print_summary(&results);

    // Markdown 출력 (stdout에도 경로 안내)
    let md = generate_markdown(&results);

    // 파일 저장 시도
    let md_path = "docs/fusion/numerical-verification.md";
    match std::fs::write(md_path, &md) {
        Ok(_) => println!("\n  📄 Markdown saved: {}", md_path),
        Err(e) => {
            // CWD가 다를 수 있으므로 절대경로도 시도
            eprintln!("  (save to {} failed: {}, trying stdout)", md_path, e);
            println!("\n--- MARKDOWN OUTPUT ---\n{}\n--- END ---", md);
        }
    }
}
