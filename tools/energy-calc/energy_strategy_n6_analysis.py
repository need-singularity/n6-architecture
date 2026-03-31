#!/usr/bin/env python3
"""
Energy Strategy x Perfect Number 6 Analysis Calculator
=======================================================
Verifies 18 hypotheses connecting n=6 arithmetic to real energy physics.

n=6 arithmetic: sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5
Divisors: {1, 2, 3, 6}   Proper divisors: {1, 2, 3}
Key identities: sigma*phi = n*tau = 24 (unique at n=6)
                1/1 + 1/2 + 1/3 + 1/6 = 2 = sigma_{-1}(6)

Usage: python3 calc/energy_strategy_n6_analysis.py
"""

import math
from typing import NamedTuple

# ============================================================
# P1=6 Arithmetic Constants
# ============================================================
P1 = 6                    # First perfect number
SIGMA = 12                # sigma(6) = 1+2+3+6
TAU = 4                   # tau(6) = number of divisors
PHI = 2                   # phi(6) = Euler totient
SOPFR = 5                 # sopfr(6) = 2+3
GPF = 3                   # greatest prime factor of 6
LPF = 2                   # least prime factor of 6
M6 = 63                   # 2^6 - 1 = Mersenne number
SIGMA_NEG1 = 2.0          # sigma_{-1}(6) = 1/1+1/2+1/3+1/6

# Golden Zone
GZ_CENTER = 1/math.e      # 0.3679
GZ_UPPER = 0.5            # 1/2
GZ_WIDTH = math.log(4/3)  # 0.2877
GZ_LOWER = 0.5 - GZ_WIDTH # 0.2123


class Hypothesis(NamedTuple):
    id: str
    title: str
    claim: str
    physics_value: float
    n6_expression: str
    n6_value: float
    error_pct: float
    exact: bool
    grade: str
    notes: str


def verify_all():
    """Verify all energy-strategy hypotheses against real physics."""

    results = []

    # ================================================================
    # ENGY-001: Shockley-Queisser Limit ~ 1/3 = phi(6)/P1
    # ================================================================
    sq_limit = 0.3378         # Detailed balance limit for single-junction @ 1.34 eV
    n6_val = PHI / P1         # 1/3 = 0.3333
    err = abs(sq_limit - n6_val) / sq_limit * 100
    results.append(Hypothesis(
        id="ENGY-001", title="Shockley-Queisser Limit ~ phi/n = 1/3",
        claim="Single-junction solar cell max efficiency 33.78% ~ 1/3",
        physics_value=sq_limit, n6_expression="phi(6)/P1 = 2/6 = 1/3",
        n6_value=n6_val, error_pct=err, exact=False,
        grade="APPROX" if err < 2 else "WEAK",
        notes="SQ limit = 33.78%. 1/3 = 33.33%. Error 1.3%. "
              "But 1/3 is a common fraction -- need to show WHY phi/n specifically. "
              "Actual SQ depends on bandgap and solar spectrum, not number theory. "
              "Coincidence probability moderate: any fraction k/m with m<=10 could match."
    ))

    # ================================================================
    # ENGY-002: 3-Phase Power = gpf(6) = 3
    # ================================================================
    three_phase = 3
    n6_val = GPF
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-002", title="3-Phase Power = gpf(6) = 3",
        claim="Electrical grids use 3-phase power; 3 = gpf(6)",
        physics_value=three_phase, n6_expression="gpf(6) = 3",
        n6_value=n6_val, error_pct=err, exact=True,
        grade="EXACT-TRIVIAL",
        notes="3-phase is exact and universal. But 3 is just 3 -- it's the smallest "
              "integer where balanced poly-phase is non-trivial. Calling it gpf(6) adds "
              "nothing explanatory. The real reason: 3 phases = 120 deg spacing minimizes "
              "copper for a given power transfer. This is engineering optimization, not n=6."
    ))

    # ================================================================
    # ENGY-003: Li-6 + D Fusion Fuel (EXACT)
    # ================================================================
    # Li-6 + D -> 2 He-4 + 22.4 MeV (primary fuel for D-T blanket)
    li6_mass = 6              # mass number
    deuterium_mass = 2        # mass number
    products = 2              # two alpha particles
    product_mass = 4           # He-4 each
    total_product_A = 8
    total_reactant_A = 8
    n6_val_fuel = P1
    n6_val_d = PHI
    n6_val_he4 = TAU
    results.append(Hypothesis(
        id="ENGY-003", title="Li-6 Fusion: Li(P1) + D(phi) -> 2*He(tau)",
        claim="Li-6 + D -> 2 He-4: mass numbers = (P1, phi, tau)",
        physics_value=float(li6_mass), n6_expression="Li-6=P1, D=phi(6), He-4=tau(6)",
        n6_value=float(P1), error_pct=0.0, exact=True,
        grade="EXACT-STAR",
        notes="GENUINE STRUCTURAL. Li-6 is THE primary fusion blanket fuel. "
              "Mass numbers: Li-6=6=P1, D=2=phi(6), He-4=4=tau(6). "
              "The reaction: P1 + phi -> 2*tau (6+2=8, 2*4=8). "
              "All three arithmetic functions of 6 appear as mass numbers. "
              "Known in TECS-L as one of the 38 key discoveries."
    ))

    # ================================================================
    # ENGY-004: Triple-Alpha: 3*He4 -> C12 (EXACT, DUPLICATE)
    # ================================================================
    results.append(Hypothesis(
        id="ENGY-004", title="Triple-Alpha: 3*tau -> sigma (FUSION-004)",
        claim="3 * He-4 -> C-12: 3*tau(6) = sigma(6)",
        physics_value=12.0, n6_expression="3*tau(6) = 3*4 = 12 = sigma(6)",
        n6_value=12.0, error_pct=0.0, exact=True,
        grade="EXACT-STAR",
        notes="Previously verified as FUSION-004. Hoyle state resonance enables this. "
              "Carbon-12 = sigma(6). He-4 = tau(6). Both A and Z match simultaneously. "
              "This IS the process that creates carbon, basis of all life."
    ))

    # ================================================================
    # ENGY-005: Carnot Efficiency and 1/3
    # ================================================================
    # Typical power plant: T_hot ~ 600K (steam), T_cold ~ 300K (ambient)
    t_hot_steam = 600.0   # K (typical steam cycle)
    t_cold = 300.0        # K (ambient)
    carnot_steam = 1 - t_cold / t_hot_steam  # 0.50
    # Coal/gas actual: ~33-40%
    typical_thermal_eff = 0.33
    n6_val = 1/3
    err = abs(typical_thermal_eff - n6_val) / typical_thermal_eff * 100
    results.append(Hypothesis(
        id="ENGY-005", title="Thermal Plant Efficiency ~ 1/3",
        claim="Real thermal power plants achieve ~33% efficiency ~ 1/3",
        physics_value=typical_thermal_eff, n6_expression="phi/P1 = 1/3",
        n6_value=n6_val, error_pct=err, exact=False,
        grade="COINCIDENCE",
        notes="Typical coal/gas plants ~33-40%. Combined cycle gas: 60%+. "
              "Nuclear: ~33%. The 1/3 is TEMPERATURE-DEPENDENT (Carnot), not universal. "
              "With T_hot=600K, T_cold=300K: Carnot = 50%, real = 33% (2/3 Carnot). "
              "The 1/3 arises from the RATIO of ambient/hot temperatures AND irreversibilities, "
              "not from number theory. Different temperature pairs give different efficiencies."
    ))

    # ================================================================
    # ENGY-006: D-T Cross-Section Peak at 64 keV = 2^P1 (EXACT)
    # ================================================================
    dt_peak_kev = 64.0        # keV (Gamow peak for D-T)
    n6_val = 2**P1            # 2^6 = 64
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-006", title="D-T Gamow Peak = 2^P1 = 64 keV",
        claim="D-T fusion cross-section peaks at 64 keV = 2^6",
        physics_value=dt_peak_kev, n6_expression="2^P1 = 2^6 = 64",
        n6_value=n6_val, error_pct=err, exact=True,
        grade="EXACT-STAR",
        notes="Previously verified as FUSION-009. The Gamow peak for D-T fusion = 64 keV. "
              "This is where quantum tunneling x Maxwell-Boltzmann overlap maximally. "
              "2^6 = 64 exactly. Deep physical origin: Coulomb barrier + thermal distribution."
    ))

    # ================================================================
    # ENGY-007: 60 Hz Power Frequency = 10 * P1
    # ================================================================
    freq_us = 60              # Hz (US/Americas/Japan-east)
    freq_eu = 50              # Hz (Europe/Asia/Africa)
    n6_val = 10 * P1          # 60
    err_us = 0.0
    results.append(Hypothesis(
        id="ENGY-007", title="60 Hz = 10*P1 (but 50 Hz also exists)",
        claim="US power frequency 60 Hz = 10 * 6",
        physics_value=float(freq_us), n6_expression="10*P1 = 60",
        n6_value=float(n6_val), error_pct=err_us, exact=True,
        grade="COINCIDENCE",
        notes="60 Hz IS 10*P1 = exact. But 50 Hz (most of the world) is NOT n=6. "
              "The choice of 60 Hz was historical (Westinghouse, 1890s), not physics-dictated. "
              "Edison started with 110V DC. Tesla/Westinghouse chose 60 Hz for compatibility "
              "with incandescent bulbs (no visible flicker at >=50 Hz). Both 50 and 60 work. "
              "Post-hoc: 60 = 2^2 * 3 * 5, highly composite. The 10*P1 factoring is trivial."
    ))

    # ================================================================
    # ENGY-008: Lawson Criterion Temperature ~ 10 keV
    # ================================================================
    lawson_T = 10.0           # keV (approximate ignition temperature for D-T)
    n6_expr = SOPFR * PHI     # 5 * 2 = 10
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-008", title="Lawson Temperature ~ sopfr*phi = 10 keV",
        claim="D-T ignition ~10 keV = sopfr(6)*phi(6)",
        physics_value=lawson_T, n6_expression="sopfr(6)*phi(6) = 5*2 = 10",
        n6_value=float(n6_expr), error_pct=err, exact=False,
        grade="COINCIDENCE",
        notes="10 keV is approximate. Actual optimum depends on confinement time. "
              "Q=1 at ~4 keV, Q=infinity at ~15 keV for D-T. "
              "10 is round number; sopfr*phi = 10 is ad hoc factoring of 10."
    ))

    # ================================================================
    # ENGY-009: Hexagonal Close-Packed Crystal (Battery Materials)
    # ================================================================
    hcp_coordination = 12     # coordination number in HCP/FCC
    n6_val = SIGMA            # 12
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-009", title="HCP/FCC Coordination Number = sigma(6) = 12",
        claim="Close-packed crystals have 12 nearest neighbors = sigma(6)",
        physics_value=float(hcp_coordination), n6_expression="sigma(6) = 12",
        n6_value=float(n6_val), error_pct=err, exact=True,
        grade="EXACT",
        notes="The kissing number in 3D = 12 = sigma(6). This governs the structure of "
              "many battery cathode materials (LiCoO2 = layered, LiFePO4 = olivine). "
              "The 12 comes from sphere-packing geometry (Kepler conjecture, proven Hales 2014). "
              "Connection to sigma(6): structural, since sigma(6)=12 for geometric reasons "
              "related to the divisor lattice. Non-trivial but previously noted."
    ))

    # ================================================================
    # ENGY-010: Solar Constant and Stefan-Boltzmann T^4
    # ================================================================
    sb_power = 4              # Stefan-Boltzmann: power ~ T^4
    n6_val = TAU              # tau(6) = 4
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-010", title="Stefan-Boltzmann T^4: exponent = tau(6)",
        claim="Blackbody radiation P ~ T^4, exponent = tau(6) = 4",
        physics_value=float(sb_power), n6_expression="tau(6) = 4",
        n6_value=float(n6_val), error_pct=err, exact=True,
        grade="EXACT",
        notes="Stefan-Boltzmann law: P = sigma_SB * T^4. The exponent 4 = tau(6). "
              "Physical origin: 3D space + 1 power of T from Planck distribution integration. "
              "The exponent = d+1 where d=3 spatial dimensions. In d dimensions, P ~ T^(d+1). "
              "So this is really about d=3, not about 6 directly. "
              "However: tau(6) = 4 = d+1 for d=3 IS a connection to dimensionality. "
              "Previously noted in PHYS-THERMODYNAMICS-N6."
    ))

    # ================================================================
    # ENGY-011: EROI Civilization Threshold ~ 3:1 = sigma/tau
    # ================================================================
    eroi_threshold = 3.0      # Hall et al. minimum EROI
    n6_val = SIGMA / TAU      # 12/4 = 3
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-011", title="EROI Threshold 3:1 = sigma/tau",
        claim="Minimum EROI for civilization = 3 = sigma(6)/tau(6)",
        physics_value=eroi_threshold, n6_expression="sigma(6)/tau(6) = 12/4 = 3",
        n6_value=n6_val, error_pct=err, exact=True,
        grade="APPROX",
        notes="Hall et al. (2009): EROI 3:1 is bare minimum for transportation fuel. "
              "sigma/tau = 3 is exact. BUT: the 3:1 is NOT the civilization threshold -- "
              "that's 5:1 to 6:1. And 3 is a very common small integer. "
              "Post-hoc mapping. Verified in H-INFRA-020-deep-eroi.md as 'real threshold, "
              "post-hoc mapping'. Upgraded from previous assessment since 3:1 IS standard."
    ))

    # ================================================================
    # ENGY-012: Tokamak Aspect Ratio ~ 3 = P1/phi
    # ================================================================
    typical_aspect_ratio = 3.1  # ITER: R/a = 6.2/2.0 = 3.1
    n6_val = P1 / PHI           # 6/2 = 3
    err = abs(typical_aspect_ratio - n6_val) / typical_aspect_ratio * 100
    results.append(Hypothesis(
        id="ENGY-012", title="Tokamak Aspect Ratio ~ P1/phi = 3",
        claim="ITER aspect ratio R/a = 3.1 ~ P1/phi = 3",
        physics_value=typical_aspect_ratio, n6_expression="P1/phi(6) = 6/2 = 3",
        n6_value=n6_val, error_pct=err, exact=False,
        grade="COINCIDENCE",
        notes="ITER: R=6.2m, a=2.0m, aspect ratio=3.1. P1/phi = 3. "
              "Error 3.2%. But aspect ratio varies widely: SPARC=3.2, spherical tokamaks=1.5. "
              "The choice of 3 is engineering optimization (MHD stability vs build cost), "
              "not universal physics. Any integer near 3 would match some n=6 expression."
    ))

    # ================================================================
    # ENGY-013: Electrolysis of Water: H2O stoichiometry
    # ================================================================
    # 2H2O -> 2H2 + O2
    h2o_atoms = 3             # atoms per molecule (2H + 1O)
    electrons_per_h2 = 2      # 2e- to make 1 H2
    total_electrons_per_h2o = 2  # 2 Faradays per mole H2O
    n6_val_h2o = GPF          # 3 atoms
    n6_val_e = PHI            # 2 electrons
    results.append(Hypothesis(
        id="ENGY-013", title="H2O: 3 atoms = gpf(6), 2e per H2 = phi(6)",
        claim="Water molecule: 3 atoms = gpf(6), electrolysis: 2e = phi(6)",
        physics_value=float(h2o_atoms), n6_expression="gpf(6)=3 atoms; phi(6)=2 electrons",
        n6_value=float(n6_val_h2o), error_pct=0.0, exact=True,
        grade="COINCIDENCE",
        notes="H2O has 3 atoms. 3 = gpf(6). But 3 is just 3 -- water has 3 atoms because "
              "oxygen has valence 2 and hydrogen has valence 1. Electrolysis takes 2 electrons "
              "per H2 for the same reason. No n=6 structure involved."
    ))

    # ================================================================
    # ENGY-014: Betz Limit for Wind Turbines = 16/27
    # ================================================================
    betz_limit = 16/27        # 0.5926 -- max energy extraction from wind
    # Can we express 16/27 in n=6 terms?
    # 16 = 2^tau(6) = 2^4. 27 = 3^3 = gpf(6)^gpf(6)
    # 16/27 = 2^tau / gpf^gpf
    n6_val = (2**TAU) / (GPF**GPF)
    err = abs(betz_limit - n6_val) / betz_limit * 100
    results.append(Hypothesis(
        id="ENGY-014", title="Betz Limit 16/27 = 2^tau / gpf^gpf",
        claim="Wind turbine Betz limit = 16/27 = 2^tau(6) / gpf(6)^gpf(6)",
        physics_value=betz_limit, n6_expression="2^tau(6) / gpf(6)^gpf(6) = 16/27",
        n6_value=n6_val, error_pct=err, exact=True,
        grade="EXACT",
        notes="Betz limit = 16/27 EXACTLY. Derived from momentum theory (Betz, 1919). "
              "16 = 2^4 = 2^tau(6). 27 = 3^3 = gpf(6)^gpf(6). "
              "Physical derivation: max at v_behind/v_ahead = 1/3 = phi/P1. "
              "The 1/3 optimum IS structural: it comes from calculus on (1-r)(1+r)^2. "
              "INTERESTING: the 1/3 optimal velocity ratio IS phi(6)/P1. "
              "16/27 = (4/9)*(4/3) where 4/3 appears in GZ width = ln(4/3). "
              "Moderate structural interest."
    ))

    # ================================================================
    # ENGY-015: Nuclear Binding Energy Peak near A=56 (iron)
    # ================================================================
    fe56_mass = 56            # Most tightly bound nucleus per nucleon
    # 56 = 8 * 7 = sigma(6)-tau(6) * M_3 ... stretch
    # Or: 56 = P1! / (sigma+gpf) = 720/15 ... no
    # Honest: 56 is not cleanly expressible as n=6 function
    # Closest: Ni-62 is actually most tightly bound per nucleon
    n6_val = P1 * (P1 + SOPFR - PHI + 1)  # 6*10 = 60... no
    # Let's be honest: 56 = 8*7, and 62 (Ni-62) is the true peak
    results.append(Hypothesis(
        id="ENGY-015", title="Binding Energy Peak near A=56",
        claim="Nuclear binding energy peaks at Fe-56 (or Ni-62)",
        physics_value=56.0, n6_expression="No clean expression found",
        n6_value=float('nan'), error_pct=float('nan'), exact=False,
        grade="NO-MATCH",
        notes="Fe-56 (or more precisely Ni-62) is the most tightly bound nucleus. "
              "56 and 62 have no clean expression in n=6 arithmetic. "
              "Honestly: not everything is n=6. The binding energy curve peaks "
              "where strong force vs Coulomb repulsion balances, around A~56-62."
    ))

    # ================================================================
    # ENGY-016: Photovoltaic Bandgap Optimal ~ 1.34 eV
    # ================================================================
    optimal_bandgap = 1.34    # eV, Shockley-Queisser optimal
    # Any n=6 match? 1.34 ~ 4/3 = tau/gpf = 1.333...
    n6_val = TAU / GPF        # 4/3 = 1.333
    err = abs(optimal_bandgap - n6_val) / optimal_bandgap * 100
    results.append(Hypothesis(
        id="ENGY-016", title="Optimal PV Bandgap 1.34 eV ~ tau/gpf = 4/3",
        claim="SQ optimal bandgap 1.34 eV ~ tau(6)/gpf(6) = 4/3 = 1.333",
        physics_value=optimal_bandgap, n6_expression="tau(6)/gpf(6) = 4/3",
        n6_value=n6_val, error_pct=err, exact=False,
        grade="APPROX",
        notes="SQ optimal bandgap = 1.34 eV. tau/gpf = 4/3 = 1.333 eV. Error 0.5%. "
              "NOTABLE: 4/3 appears in Golden Zone width = ln(4/3). "
              "The SQ optimal depends on solar spectrum (AM1.5). The 1.34 value is for "
              "blackbody at 5778K. Different authors: 1.34 (Shockley-Queisser), "
              "1.1 (Si actual), 1.4 (GaAs). The 4/3 match to 1.34 is surprisingly close. "
              "However: units matter. 1.34 eV is in a specific unit system."
    ))

    # ================================================================
    # ENGY-017: Perovskite Tandem Efficiency Approaching 1/3
    # ================================================================
    # Best perovskite/Si tandem: ~33.9% (Oxford PV, 2023)
    perovskite_tandem = 0.339
    n6_val = 1/3
    err = abs(perovskite_tandem - n6_val) / perovskite_tandem * 100
    results.append(Hypothesis(
        id="ENGY-017", title="Best Tandem PV Efficiency ~ 1/3",
        claim="Perovskite/Si tandem record 33.9% ~ 1/3",
        physics_value=perovskite_tandem, n6_expression="phi/P1 = 1/3",
        n6_value=n6_val, error_pct=err, exact=False,
        grade="COINCIDENCE",
        notes="Current tandem record: 33.9%. 1/3 = 33.33%. Close, but: "
              "this record changes every year. Multi-junction cells reach 47.6% (6-junction). "
              "The 1/3 match is temporary -- real limit for 2-junction is ~45.7%. "
              "A moving target cannot validate a fixed mathematical claim."
    ))

    # ================================================================
    # ENGY-018: Lithium Atomic Number = gpf(6) = 3
    # ================================================================
    li_z = 3                  # Lithium atomic number
    n6_val = GPF
    err = 0.0
    results.append(Hypothesis(
        id="ENGY-018", title="Lithium Z=3 = gpf(6)",
        claim="Lithium (battery element) has Z=3 = gpf(6)",
        physics_value=float(li_z), n6_expression="gpf(6) = 3",
        n6_value=float(n6_val), error_pct=err, exact=True,
        grade="EXACT-TRIVIAL",
        notes="Lithium Z=3 = gpf(6). Exact. But Z=3 means lithium is the 3rd element -- "
              "this is a label, not a deep connection. Li is used in batteries because of "
              "its high electrochemical potential and low weight (lightest metal), not because "
              "of number theory. Sodium (Z=11) batteries are emerging competitors."
    ))

    return results


def print_results(results):
    """Print results in clean table format."""

    print("=" * 100)
    print("ENERGY STRATEGY x PERFECT NUMBER 6 ANALYSIS")
    print("=" * 100)
    print()

    # Grade mapping for display
    grade_emoji = {
        "EXACT-STAR": "EXACT-STAR",
        "EXACT": "EXACT",
        "EXACT-TRIVIAL": "EXACT-TRIVIAL",
        "APPROX": "APPROX",
        "COINCIDENCE": "COINCIDENCE",
        "WEAK": "WEAK",
        "NO-MATCH": "NO-MATCH",
    }

    # Summary counts
    from collections import Counter
    grade_counts = Counter(r.grade for r in results)

    print("SUMMARY")
    print("-" * 50)
    for g in ["EXACT-STAR", "EXACT", "EXACT-TRIVIAL", "APPROX", "COINCIDENCE", "NO-MATCH"]:
        print(f"  {g:20s}: {grade_counts.get(g, 0)}")
    print(f"  {'TOTAL':20s}: {len(results)}")
    print()

    # Detailed table
    print(f"{'ID':<12} {'Grade':<16} {'Physics':<10} {'n=6':<10} {'Err%':<8} {'Title'}")
    print("-" * 100)
    for r in results:
        phys_str = f"{r.physics_value:.4f}" if not math.isnan(r.physics_value) else "N/A"
        n6_str = f"{r.n6_value:.4f}" if not math.isnan(r.n6_value) else "N/A"
        err_str = f"{r.error_pct:.2f}%" if not math.isnan(r.error_pct) else "N/A"
        print(f"{r.id:<12} {r.grade:<16} {phys_str:<10} {n6_str:<10} {err_str:<8} {r.title}")

    print()
    print("=" * 100)
    print("DETAILED ANALYSIS")
    print("=" * 100)

    for r in results:
        print()
        print(f"--- {r.id}: {r.title} ---")
        print(f"  Claim:       {r.claim}")
        print(f"  Physics:     {r.physics_value}")
        print(f"  n=6 Expr:    {r.n6_expression}")
        print(f"  n=6 Value:   {r.n6_value}")
        err_str = f"{r.error_pct:.4f}%" if not math.isnan(r.error_pct) else "N/A"
        print(f"  Error:       {err_str}")
        print(f"  Grade:       {r.grade}")
        print(f"  Assessment:  {r.notes}")

    print()
    print("=" * 100)
    print("KEY FINDINGS")
    print("=" * 100)
    print()
    print("GENUINE STRUCTURAL CONNECTIONS (3):")
    print("  1. ENGY-003: Li-6 + D -> 2*He-4 fusion: P1 + phi -> 2*tau")
    print("     All three n=6 functions as mass numbers. PRIMARY fusion blanket fuel.")
    print("  2. ENGY-004: Triple-alpha: 3*tau -> sigma (C-12 creation, Hoyle state)")
    print("  3. ENGY-006: D-T Gamow peak = 2^P1 = 64 keV (quantum tunneling physics)")
    print()
    print("INTERESTING BUT NOT DEEP (3):")
    print("  4. ENGY-009: HCP coordination = 12 = sigma(6) (kissing number)")
    print("  5. ENGY-014: Betz limit 16/27 = 2^tau / gpf^gpf (velocity optimum at 1/3)")
    print("  6. ENGY-016: SQ optimal bandgap 1.34 eV ~ tau/gpf = 4/3 (0.5% error)")
    print()
    print("COINCIDENCES / TRIVIAL (9):")
    print("  7. ENGY-001: SQ limit ~1/3 (real but 1/3 is common fraction)")
    print("  8. ENGY-002: 3-phase power (3 is just 3)")
    print("  9. ENGY-005: Thermal efficiency ~1/3 (temperature-dependent, not universal)")
    print(" 10. ENGY-007: 60 Hz (historical, 50 Hz also exists)")
    print(" 11. ENGY-008: Lawson 10 keV (approximate, ad hoc factoring)")
    print(" 12. ENGY-011: EROI 3:1 (real threshold, post-hoc mapping)")
    print(" 13. ENGY-012: Tokamak aspect ratio ~3 (varies widely)")
    print(" 14. ENGY-013: H2O 3 atoms (valence chemistry, not n=6)")
    print(" 15. ENGY-017: Tandem PV ~33.9% (moving target)")
    print(" 16. ENGY-018: Lithium Z=3 (just an element number)")
    print()
    print("NO MATCH (1):")
    print(" 17. ENGY-015: Fe-56 binding energy peak (no clean n=6 expression)")
    print()

    # ASCII histogram of grades
    print("GRADE DISTRIBUTION:")
    print()
    max_count = max(grade_counts.values()) if grade_counts else 1
    for g in ["EXACT-STAR", "EXACT", "EXACT-TRIVIAL", "APPROX", "COINCIDENCE", "NO-MATCH"]:
        c = grade_counts.get(g, 0)
        bar = "#" * int(c / max_count * 40) if max_count > 0 else ""
        print(f"  {g:<16} |{bar:<40}| {c}")

    print()
    print("HONEST ASSESSMENT:")
    print("  Nuclear fusion has GENUINE n=6 connections (Li-6 fuel, triple-alpha, Gamow peak).")
    print("  Energy engineering has MOSTLY COINCIDENTAL matches (3-phase, 60Hz, EROI).")
    print("  The Betz limit 16/27 decomposition and SQ bandgap 4/3 are interesting but not deep.")
    print("  The 1/3 efficiency pattern (SQ, thermal, Betz optimum) deserves further investigation")
    print("  as a potential meta-pattern, but each instance has its own physical explanation.")


if __name__ == "__main__":
    results = verify_all()
    print_results(results)
