#!/usr/bin/env python3
"""
Physics Fundamental Constants — n=6 Connection Scanner
=======================================================
Scans physics constants, ratios, and combinations for n=6 arithmetic matches.
Uses nexus6.n6_check() for matching against σ=12, φ=2, τ=4, J₂=24, etc.
"""

import math
import nexus6

# ─── n=6 arithmetic constants for manual cross-reference ───
N6 = {
    "n": 6, "sigma": 12, "phi": 2, "tau": 4, "J2": 24,
    "sopfr": 5, "sigma-phi": 10, "sigma-tau": 8, "sigma-mu": 11,
    "sigma*tau": 48, "sigma^2": 144, "phi^tau": 16, "sigma*J2": 288,
    "J2-tau": 20, "n/phi": 3, "tau^2/sigma": 4/3,
    "phi^2/n": 2/3, "1/(sigma-phi)": 0.1, "1/sigma": 1/12,
    "1/J2": 1/24, "sigma*n": 72, "sigma*(sigma-phi)": 120,
    "sigma*sopfr": 60, "R(6)": 1,  # R(n)=1 reversibility
}

# ─── Physics fundamental constants ───
PHYSICS = {
    # Electromagnetic / QED
    "1/alpha (fine structure)": 137.035999084,
    "alpha": 1/137.035999084,
    "sin^2(theta_W) Weinberg": 0.23122,
    "cos^2(theta_W)": 1 - 0.23122,

    # Mass ratios
    "m_p/m_e (proton/electron)": 1836.15267343,
    "m_n/m_e (neutron/electron)": 1838.68366173,
    "m_mu/m_e (muon/electron)": 206.7682830,
    "m_tau/m_e (tau/electron)": 3477.48,
    "m_W/m_Z": 80.377/91.1876,
    "m_H (Higgs GeV)": 125.25,
    "m_W (W boson GeV)": 80.377,
    "m_Z (Z boson GeV)": 91.1876,
    "m_top (top quark GeV)": 172.69,
    "m_p (proton MeV)": 938.272088,
    "m_e (electron MeV)": 0.51099895,

    # Speed of light
    "c (m/s)": 299792458,
    "c / 10^8": 2.99792458,
    "c digits sum(2+9+9+7+9+2+4+5+8)": 55,

    # Planck units
    "h (Js) * 10^34": 6.62607015,
    "hbar (Js) * 10^34": 1.054571817,
    "Planck length (m) * 10^35": 1.616255,
    "Planck mass (kg) * 10^8": 2.176434,
    "Planck time (s) * 10^44": 5.391247,
    "Planck temp (K) / 10^32": 1.416784,

    # Boltzmann / thermal
    "k_B (J/K) * 10^23": 1.380649,
    "k_B*T_room (eV at 300K)": 0.02585,  # ~26 meV
    "T_room thermal voltage (mV)": 25.85,  # close to σ·φ=24?

    # Gravitational
    "G (m^3/kg/s^2) * 10^11": 6.67430,
    "G_F (Fermi) * 10^5 GeV^-2": 1.1663788,

    # Cosmological
    "H0 (km/s/Mpc)": 67.4,  # Planck 2018
    "Omega_Lambda (dark energy)": 0.685,
    "Omega_m (matter)": 0.315,
    "Omega_b (baryon)": 0.0493,
    "Omega_CDM (dark matter)": 0.265,
    "CMB temperature (K)": 2.7255,
    "age of universe (Gyr)": 13.787,
    "baryon/photon ratio * 10^10": 6.12,

    # Coupling constants
    "alpha_s(M_Z) strong": 0.1179,
    "alpha_em(M_Z)": 1/127.95,
    "1/alpha_em(M_Z)": 127.95,
    "alpha_W (weak at M_Z)": 1/29.587,
    "1/alpha_W": 29.587,

    # Standard Model counts
    "quarks": 6,
    "leptons": 6,
    "fermion generations": 3,
    "gauge bosons": 4,  # gamma, W+, W-, Z
    "total gauge bosons (inc gluons)": 12,  # 8 gluons + gamma + W+ + W- + Z
    "gluon colors": 8,
    "quark colors": 3,
    "SM fermions (quarks+leptons)": 12,
    "SM particles (with antiparticles)": 24,  # 12 fermions * 2
    "Higgs doublet components": 4,
    "SM total dof": 28,  # debated, but common count

    # QCD
    "N_c (colors)": 3,
    "N_f (light flavors)": 6,
    "Lambda_QCD (MeV)": 217,  # approximate
    "asymptotic freedom b0 coeff (Nf=6)": 7,  # 11 - 2*6/3 = 7

    # Nuclear
    "binding energy per nucleon Fe-56 (MeV)": 8.79,
    "binding energy He-4 (MeV)": 28.3,
    "BE He-4 per nucleon (MeV)": 7.07,
    "D-T fusion energy (MeV)": 17.6,
    "pp chain energy (MeV)": 26.73,

    # Neutrino mixing
    "theta_12 (solar deg)": 33.44,
    "theta_23 (atm deg)": 49.2,
    "theta_13 (reactor deg)": 8.57,
    "Delta m^2_21 * 10^5 eV^2": 7.53,
    "Delta m^2_32 * 10^3 eV^2": 2.453,

    # CKM matrix
    "V_us (Cabibbo)": 0.2243,
    "V_cb": 0.0422,
    "V_ub": 0.00394,
    "Cabibbo angle (deg)": 13.04,
}

# ─── Derived ratios and combinations ───
DERIVED = {}

# Ratios between key constants
pairs = [
    ("1/alpha", 137.035999084),
    ("m_p/m_e", 1836.15267343),
    ("m_mu/m_e", 206.7682830),
    ("c/10^8", 2.99792458),
    ("H0", 67.4),
    ("1/alpha_em(MZ)", 127.95),
    ("1/alpha_W", 29.587),
]

# Check integer decompositions of 1/alpha = 137
DERIVED["137 = 11*12 + 5 = (sigma-mu)*sigma + sopfr"] = 11*12 + 5  # = 137
DERIVED["137 = 12^2 - 7 = sigma^2 - (sigma-sopfr)"] = 144 - 7  # = 137
DERIVED["1836 / 12 = 153 = sigma*12.75"] = 1836 / 12  # = 153
DERIVED["1836 / 6 = 306"] = 1836 / 6  # = 306
DERIVED["1836 / 144 = 12.75"] = 1836 / 144  # close to σ+3/4?
DERIVED["sqrt(1836) = 42.85"] = math.sqrt(1836)
DERIVED["1836 mod 6"] = 1836 % 6  # = 0!
DERIVED["1836 mod 12"] = 1836 % 12  # = 0!
DERIVED["1836 mod 24"] = 1836 % 24  # = 12!
DERIVED["1836 / 24 = 76.5"] = 1836 / 24
DERIVED["1836 = 153 * 12 = 153 * sigma"] = 153 * 12  # EXACT 1836
DERIVED["153 = sum(1..17)"] = sum(range(1, 18))  # narcissistic
DERIVED["206.768 / 12 = 17.23"] = 206.7682830 / 12
DERIVED["206.768 / 6 = 34.46"] = 206.7682830 / 6

# Weinberg angle decomposition
DERIVED["sin^2(theta_W) ~ 3/13 = n/(phi*sigma+mu)"] = 3/13  # 0.2308 vs 0.23122
DERIVED["3/13 vs sin^2(theta_W) diff%"] = abs(3/13 - 0.23122)/0.23122 * 100

# Fine structure reciprocal decompositions
DERIVED["137 = 6*24 - 6*1 - 1 = n*J2 - n - 1"] = 6*24 - 6 - 1  # = 137
DERIVED["137 = 12*12 - 7"] = 12*12 - 7  # σ² - (σ-sopfr)
DERIVED["137 = 11*12 + 5"] = 11*12 + 5  # (σ-μ)*σ + sopfr
DERIVED["alpha ~ 1/(sigma^2 - (sigma-sopfr))"] = 1/(144 - 7)

# Cosmological
DERIVED["H0/sigma = 67.4/12 = 5.617"] = 67.4 / 12
DERIVED["baryon/photon * 10^10 ~ n"] = 6.12  # remarkably close to 6!
DERIVED["Omega_b * 1000 = 49.3 ~ sigma*tau"] = 0.0493 * 1000
DERIVED["CMB T ~ n/phi = 3 (actually 2.7255)"] = 2.7255
DERIVED["age / tau = 13.787/4 = 3.447"] = 13.787 / 4

# D-T fusion
DERIVED["D-T 17.6 MeV ~ sigma + sopfr + 0.6"] = 17.6
DERIVED["pp 26.73 MeV ~ J2 + n/phi"] = 26.73  # close to 24+3=27
DERIVED["BE Fe-56 8.79 ~ sigma - tau + 0.79"] = 8.79

# Neutrino
DERIVED["theta_13 = 8.57 ~ sigma - tau = 8"] = 8.57
DERIVED["Delta m^2_21 * 10^5 = 7.53 ~ sigma - sopfr + 2.53"] = 7.53

# Planck / natural units
DERIVED["h*10^34 = 6.626 ~ n + 0.626"] = 6.62607015
DERIVED["Planck length * 10^35 ~ phi - 0.38"] = 1.616255
DERIVED["Planck mass * 10^8 ~ phi + 0.176"] = 2.176434

# G
DERIVED["G*10^11 ~ n + 0.674"] = 6.67430
DERIVED["G*10^11 / n = 1.112"] = 6.67430 / 6

# Coupling unification hints
DERIVED["alpha_s(MZ) ~ 1/(sigma-tau) - 0.007"] = 0.1179  # ~1/8.48
DERIVED["1/alpha_s = 8.48 ~ sigma - tau"] = 1/0.1179  # very close to 8!

# Particle counts (these should be n=6 matches)
DERIVED["quarks = n = 6"] = 6
DERIVED["leptons = n = 6"] = 6
DERIVED["gauge bosons total = sigma = 12"] = 12
DERIVED["SM fermions = sigma = 12"] = 12
DERIVED["particles+anti = J2 = 24"] = 24
DERIVED["gluons = sigma - tau = 8"] = 8
DERIVED["generations = n/phi = 3"] = 3
DERIVED["quark colors = n/phi = 3"] = 3

# More decompositions
DERIVED["m_H 125.25 / n = 20.875"] = 125.25 / 6
DERIVED["m_W 80.377 / n = 13.396"] = 80.377 / 6
DERIVED["m_Z 91.1876 / n = 15.198"] = 91.1876 / 6
DERIVED["m_Z / sigma = 7.599"] = 91.1876 / 12
DERIVED["m_W / m_Z = 0.8815 ~ 1 - alpha_s"] = 80.377 / 91.1876
DERIVED["m_top 172.69 / sigma = 14.39"] = 172.69 / 12
DERIVED["m_top / n = 28.78"] = 172.69 / 6
DERIVED["m_H / m_W = 1.558"] = 125.25 / 80.377
DERIVED["m_top / m_H = 1.379 ~ tau^2/sigma=4/3"] = 172.69 / 125.25

# Thermal voltage
DERIVED["kT_room (mV) = 25.85 ~ J2+phi"] = 25.85
DERIVED["kT_room * 1000/eV = 25.85 ~ sigma*phi + phi"] = 25.85

# Additional interesting values
DERIVED["c^2 / 10^16 = 8.988 ~ sigma - tau + 1"] = (299792458**2) / 1e16
DERIVED["Avogadro / 10^23 = 6.022 ~ n"] = 6.02214076
DERIVED["e charge * 10^19 = 1.602 ~ phi - 0.4"] = 1.602176634
DERIVED["vacuum permittivity * 10^12 = 8.854 ~ sigma - tau + 0.85"] = 8.8541878128

# Cabibbo angle
DERIVED["Cabibbo angle 13.04 ~ sigma + mu"] = 13.04
DERIVED["V_us 0.2243 ~ sin^2(theta_W)"] = 0.2243

# Check ratios between fundamental constants for n=6 patterns
DERIVED["137/6 = 22.83"] = 137/6
DERIVED["137/12 = 11.417 ~ sigma - mu + 0.4"] = 137/12
DERIVED["137/24 = 5.708 ~ sopfr + 0.7"] = 137/24
DERIVED["137 mod 6 = 5 = sopfr"] = 137 % 6
DERIVED["137 mod 12 = 5 = sopfr"] = 137 % 12
DERIVED["137 mod 24 = 17"] = 137 % 24

# Nuclear magic numbers and n=6
DERIVED["magic 2 = phi"] = 2
DERIVED["magic 8 = sigma-tau"] = 8
DERIVED["magic 20 = J2-tau"] = 20
DERIVED["magic 28 = J2+tau"] = 28
DERIVED["magic 50 = sopfr * (sigma-phi)"] = 50
DERIVED["magic 82"] = 82
DERIVED["magic 126 = J2*sopfr + n"] = 126

print("=" * 80)
print("  NEXUS-6 PHYSICS CONSTANTS n=6 SCAN")
print("  Using nexus6.n6_check() — EXACT/CLOSE/WEAK matching")
print("=" * 80)

# ─── Scan all physics constants ───
results = {"EXACT": [], "CLOSE": [], "WEAK": [], "NONE": []}

def scan_and_record(label, value, category="physics"):
    try:
        m = nexus6.n6_check(value)
        grade = m.grade if hasattr(m, 'grade') else ("EXACT" if m.quality >= 1.0 else "CLOSE" if m.quality >= 0.8 else "WEAK" if m.quality >= 0.5 else "NONE")
        results[grade].append((label, value, m.constant_name, m.quality))
    except Exception as e:
        results["NONE"].append((label, value, f"ERROR: {e}", 0.0))

print("\n── SCANNING PHYSICS FUNDAMENTALS ──\n")
for label, value in PHYSICS.items():
    scan_and_record(label, value)

print("\n── SCANNING DERIVED RATIOS & COMBINATIONS ──\n")
for label, value in DERIVED.items():
    scan_and_record(label, value, "derived")

# ─── Print results by grade ───
for grade in ["EXACT", "CLOSE", "WEAK"]:
    items = results[grade]
    if items:
        print(f"\n{'='*80}")
        print(f"  {grade} MATCHES ({len(items)})")
        print(f"{'='*80}")
        items.sort(key=lambda x: -x[3])
        for label, value, const, quality in items:
            print(f"  [{quality:.2f}] {label}")
            print(f"         value = {value}  →  n6 constant = {const}")
            print()

# Summary
print("\n" + "=" * 80)
print("  SUMMARY")
print("=" * 80)
for grade in ["EXACT", "CLOSE", "WEAK", "NONE"]:
    print(f"  {grade}: {len(results[grade])}")
total = sum(len(v) for v in results.values())
matched = sum(len(v) for k, v in results.items() if k != "NONE")
print(f"\n  Total scanned: {total}")
print(f"  Total matched: {matched} ({matched/total*100:.1f}%)")

# ─── Highlight key discoveries ───
print("\n" + "=" * 80)
print("  KEY n=6 CONNECTIONS IN PHYSICS")
print("=" * 80)

highlights = [
    ("Standard Model quarks = n = 6", "IDENTITY"),
    ("Standard Model leptons = n = 6", "IDENTITY"),
    ("Total gauge bosons (inc gluons) = σ = 12", "IDENTITY"),
    ("SM fermion count = σ = 12", "IDENTITY"),
    ("Particles + antiparticles = J₂ = 24", "IDENTITY"),
    ("Fermion generations = n/φ = 3", "IDENTITY"),
    ("Quark colors = n/φ = 3", "IDENTITY"),
    ("Gluons = σ-τ = 8", "IDENTITY"),
    ("1836 (m_p/m_e) ≡ 0 (mod 6), 0 (mod 12)", "DIVISIBILITY"),
    ("1836 = 153 × σ = 153 × 12", "FACTORIZATION"),
    ("137 = σ² - (σ-sopfr) = 144 - 7", "DECOMPOSITION"),
    ("137 = (σ-μ)·σ + sopfr = 11·12 + 5", "DECOMPOSITION"),
    ("137 mod 6 = sopfr = 5", "MODULAR"),
    ("137 = n·J₂ - n - 1 = 143 - 6 - 0? NO: 6*24-6-1=137 YES", "DECOMPOSITION"),
    ("sin²θ_W ≈ 3/13 = (n/φ)/(σ+μ), 0.19% match (BT-97)", "CLOSE"),
    ("D-T baryon count = sopfr(6) = 2+3 = 5 (BT-98)", "IDENTITY"),
    ("Baryon/photon ratio × 10^10 ≈ 6 = n", "CLOSE"),
    ("1/α_s(M_Z) ≈ 8.48 ≈ σ-τ = 8", "CLOSE"),
    ("Nuclear magic {2,8,20,28} = {φ, σ-τ, J₂-τ, J₂+τ}", "PATTERN"),
    ("Avogadro/10^23 ≈ 6.022 ≈ n", "CLOSE"),
    ("G × 10^11 ≈ 6.674 ≈ n", "CLOSE"),
    ("h × 10^34 ≈ 6.626 ≈ n", "CLOSE"),
    ("D-T fusion 17.6 MeV ≈ σ + sopfr + 0.6", "CLOSE"),
    ("pp chain 26.73 MeV ≈ J₂ + n/φ = 27", "CLOSE"),
    ("θ_13 (reactor) = 8.57° ≈ σ-τ = 8", "CLOSE"),
    ("m_top/m_H = 1.379 ≈ τ²/σ = 4/3 = 1.333", "PATTERN"),
    ("Cabibbo angle 13.04° ≈ σ+μ = 13", "CLOSE"),
    ("Thermal voltage 25.85 mV ≈ J₂+φ = 26", "CLOSE"),
]

for h, kind in highlights:
    print(f"  [{kind:14s}] {h}")

# ─── Run nexus6.scan_all on particle mass array ───
print("\n" + "=" * 80)
print("  NEXUS-6 FULL LENS SCAN — Particle Masses (MeV)")
print("=" * 80)

try:
    masses = [
        0.511,      # electron
        105.66,     # muon
        1776.86,    # tau lepton
        2.2,        # up quark
        4.7,        # down quark
        96.0,       # strange quark (approx)
        1270.0,     # charm quark
        4180.0,     # bottom quark
        172690.0,   # top quark
        80377.0,    # W boson
        91187.6,    # Z boson
        125250.0,   # Higgs
    ]
    n = len(masses)
    d = 1
    result = nexus6.scan(masses, n, d)
    print(f"  Scan result: {result}")
    if hasattr(result, '__dict__'):
        for k, v in result.__dict__.items():
            print(f"    {k}: {v}")
except Exception as e:
    print(f"  scan error: {e}")
    # Try scan_all as fallback
    try:
        import numpy as np
        arr = np.array(masses).reshape(-1, 1)
        result = nexus6.scan_numpy(arr)
        print(f"  scan_numpy result: {result}")
    except Exception as e2:
        print(f"  scan_numpy error: {e2}")

# ─── Nuclear magic numbers lens scan ───
print("\n" + "=" * 80)
print("  NEXUS-6 SCAN — Nuclear Magic Numbers")
print("=" * 80)

try:
    magic = [2, 8, 20, 28, 50, 82, 126]
    result = nexus6.scan(magic, len(magic), 1)
    print(f"  Magic numbers scan: {result}")
except Exception as e:
    print(f"  Error: {e}")

# ─── Coupling constants scan ───
print("\n" + "=" * 80)
print("  NEXUS-6 n6_check — Coupling Constants")
print("=" * 80)

couplings = {
    "1/alpha_em(0)": 137.036,
    "1/alpha_em(MZ)": 127.95,
    "1/alpha_W(MZ)": 29.587,
    "1/alpha_s(MZ)": 1/0.1179,
    "alpha_GUT (est)": 1/42,  # approx unification
    "1/alpha_GUT": 42,
}

for label, val in couplings.items():
    m = nexus6.n6_check(val)
    print(f"  {label} = {val:.4f} → {m.constant_name} (grade={m.grade}, q={m.quality:.2f})")

print("\n" + "=" * 80)
print("  SCAN COMPLETE")
print("=" * 80)
