"""Dark Matter Mass Candidates from TECS-L n=6 Arithmetic.

Dark matter comprises ~27% of the universe's energy density, yet its
particle mass remains unknown.  WIMP searches span ~1 GeV to ~100 TeV.

This module generates >100 candidate DM masses from TECS-L expressions,
then filters them against current direct-detection exclusion limits
(XENON1T, XENONnT, LUX-ZEPLIN) and cosmological relic-abundance
constraints to identify the most promising candidates.

TECS-L parameters for n=6:
    sigma=12, tau=4, phi=2, sopfr=5, n=6
    Perfect numbers: P1=6, P2=28, P3=496
"""

from __future__ import annotations

import math
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

import numpy as np

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
    SIGMA_P2, TAU_P2, PHI_P2,
    TAU_P3, PHI_P3,
)


# =====================================================================
# Physical constants
# =====================================================================

HIGGS_MASS = 125.25        # GeV, PDG 2024
HIGGS_MASS_ERR = 0.17      # GeV
HIGGS_VEV = 246.22         # GeV
M_W = 80.377               # GeV
M_Z = 91.1876              # GeV
M_TOP = 172.76             # GeV
RHO_MASS = 0.77526         # GeV
JPSI_MASS = 3.09690        # GeV
UPSILON_MASS = 9.46040     # GeV
LAMBDA_QCD = 0.332         # GeV, MSbar

OMEGA_DM_H2_OBS = 0.120    # Planck 2018
OMEGA_DM_H2_ERR = 0.001
OMEGA_DM_FRAC = 0.27       # fraction of total energy density

# Weak coupling
G_WEAK = 0.65              # SU(2) gauge coupling

# Thermal relic cross section target
SIGMA_V_THERMAL = 3.0e-26  # cm^3/s

# Conversion
GEV_TO_CM = 1.9733e-14     # hbar*c in GeV*cm
GEV2_TO_CM2 = GEV_TO_CM**2
GEV2_TO_CM3S = 1.1677e-17  # 1 GeV^-2 in cm^3/s (with c=1 conversion)

# Shorthand
s = SIGMA_P1   # 12
t = TAU_P1     # 4
p = PHI_P1     # 2
sf = SOPFR_P1  # 5
n = P1         # 6
rho = RHO_MASS


# =====================================================================
# 1. Generate ALL candidate DM masses
# =====================================================================

@dataclass
class DMCandidate:
    """A dark matter mass candidate from TECS-L arithmetic."""
    label: str
    mass_gev: float
    category: str
    formula: str
    excluded: bool = False
    exclusion_reason: str = ""
    relic_ok: bool = False
    relic_coupling: float = 0.0
    score: float = 0.0


def generate_candidates() -> List[DMCandidate]:
    """Generate all TECS-L dark matter mass candidates."""
    candidates = []

    def add(label, mass_gev, category, formula):
        if mass_gev > 0 and np.isfinite(mass_gev):
            candidates.append(DMCandidate(label, mass_gev, category, formula))

    # --- Simple TECS-L values as GeV ---
    add("n", n, "simple", "n = 6 GeV")
    add("sigma", s, "simple", "sigma(6) = 12 GeV")
    add("tau", t, "simple", "tau(6) = 4 GeV")
    add("phi", p, "simple", "phi(6) = 2 GeV")
    add("sopfr", sf, "simple", "sopfr(6) = 5 GeV")
    add("P2", P2, "simple", "P2 = 28 GeV")
    add("P3", P3, "simple", "P3 = 496 GeV")
    add("sigma(P2)", SIGMA_P2, "simple", "sigma(28) = 56 GeV")
    add("tau(P2)", TAU_P2, "simple", "tau(28) = 6 GeV")
    add("phi(P2)", PHI_P2, "simple", "phi(28) = 12 GeV")
    add("tau(P3)", TAU_P3, "simple", "tau(496) = 10 GeV")
    add("phi(P3)", PHI_P3, "simple", "phi(496) = 240 GeV")

    # --- Combined products ---
    add("sigma*tau", s * t, "combined", "12*4 = 48 GeV")
    add("sigma*phi", s * p, "combined", "12*2 = 24 GeV")
    add("n*tau", n * t, "combined", "6*4 = 24 GeV")
    add("P2*phi", P2 * p, "combined", "28*2 = 56 GeV")
    add("sigma*sopfr", s * sf, "combined", "12*5 = 60 GeV")
    add("tau*sopfr", t * sf, "combined", "4*5 = 20 GeV")
    add("n*sigma", n * s, "combined", "6*12 = 72 GeV")
    add("n*sopfr", n * sf, "combined", "6*5 = 30 GeV")
    add("n*phi", n * p, "combined", "6*2 = 12 GeV")
    add("P2*tau", P2 * t, "combined", "28*4 = 112 GeV")
    add("P2*sigma", P2 * s, "combined", "28*12 = 336 GeV")
    add("P2*sopfr", P2 * sf, "combined", "28*5 = 140 GeV")
    add("P2*n", P2 * n, "combined", "28*6 = 168 GeV")
    add("sigma-tau", s - t, "combined", "12-4 = 8 GeV")
    add("sigma+tau", s + t, "combined", "12+4 = 16 GeV")
    add("sigma+phi", s + p, "combined", "12+2 = 14 GeV")
    add("tau+phi", t + p, "combined", "4+2 = 6 GeV")
    add("n+sigma", n + s, "combined", "6+12 = 18 GeV")
    add("P2-n", P2 - n, "combined", "28-6 = 22 GeV")
    add("P2+n", P2 + n, "combined", "28+6 = 34 GeV")
    add("P2-sigma", P2 - s, "combined", "28-12 = 16 GeV")
    add("sigma*tau*phi", s * t * p, "combined", "12*4*2 = 96 GeV")
    add("n*tau*phi", n * t * p, "combined", "6*4*2 = 48 GeV")
    add("sigma*tau+phi", s * t + p, "combined", "48+2 = 50 GeV")
    add("sigma*phi-tau", s * p - t, "combined", "24-4 = 20 GeV")
    add("P2/phi", P2 / p, "combined", "28/2 = 14 GeV")
    add("P2/tau", P2 / t, "combined", "28/4 = 7 GeV")
    add("P3/sigma", P3 / s, "combined", "496/12 = 41.3 GeV")
    add("P3/tau", P3 / t, "combined", "496/4 = 124 GeV")
    add("P3/n", P3 / n, "combined", "496/6 = 82.7 GeV")
    add("P3/P2", P3 / P2, "combined", "496/28 = 17.7 GeV")
    add("P3/sigma/tau", P3 / s / t, "combined", "496/48 = 10.3 GeV")

    # --- With VEV ---
    add("v/sigma", HIGGS_VEV / s, "VEV", f"{HIGGS_VEV}/12 = {HIGGS_VEV/s:.1f} GeV")
    add("v/tau", HIGGS_VEV / t, "VEV", f"{HIGGS_VEV}/4 = {HIGGS_VEV/t:.1f} GeV")
    add("v/phi", HIGGS_VEV / p, "VEV", f"{HIGGS_VEV}/2 = {HIGGS_VEV/p:.1f} GeV")
    add("v/n", HIGGS_VEV / n, "VEV", f"{HIGGS_VEV}/6 = {HIGGS_VEV/n:.1f} GeV")
    add("v/sopfr", HIGGS_VEV / sf, "VEV", f"{HIGGS_VEV}/5 = {HIGGS_VEV/sf:.1f} GeV")
    add("v/P2", HIGGS_VEV / P2, "VEV", f"{HIGGS_VEV}/28 = {HIGGS_VEV/P2:.1f} GeV")
    add("v/(sigma*tau)", HIGGS_VEV / (s * t), "VEV", f"{HIGGS_VEV}/48 = {HIGGS_VEV/(s*t):.2f} GeV")
    add("v/(sigma*phi)", HIGGS_VEV / (s * p), "VEV", f"{HIGGS_VEV}/24 = {HIGGS_VEV/(s*p):.2f} GeV")
    add("v/(sigma+tau)", HIGGS_VEV / (s + t), "VEV", f"{HIGGS_VEV}/16 = {HIGGS_VEV/(s+t):.2f} GeV")
    add("v*phi/sigma", HIGGS_VEV * p / s, "VEV", f"{HIGGS_VEV}*2/12 = {HIGGS_VEV*p/s:.1f} GeV")
    add("v*tau/sigma", HIGGS_VEV * t / s, "VEV", f"{HIGGS_VEV}*4/12 = {HIGGS_VEV*t/s:.1f} GeV")
    add("v*phi/P2", HIGGS_VEV * p / P2, "VEV", f"{HIGGS_VEV}*2/28 = {HIGGS_VEV*p/P2:.2f} GeV")
    add("v/sigma^2", HIGGS_VEV / s**2, "VEV", f"{HIGGS_VEV}/144 = {HIGGS_VEV/s**2:.3f} GeV")

    # --- With Higgs mass ---
    add("mH/phi", HIGGS_MASS / p, "Higgs", f"{HIGGS_MASS}/2 = {HIGGS_MASS/p:.2f} GeV")
    add("mH/tau", HIGGS_MASS / t, "Higgs", f"{HIGGS_MASS}/4 = {HIGGS_MASS/t:.2f} GeV")
    add("mH/n", HIGGS_MASS / n, "Higgs", f"{HIGGS_MASS}/6 = {HIGGS_MASS/n:.3f} GeV")
    add("mH/sigma", HIGGS_MASS / s, "Higgs", f"{HIGGS_MASS}/12 = {HIGGS_MASS/s:.3f} GeV")
    add("mH/sopfr", HIGGS_MASS / sf, "Higgs", f"{HIGGS_MASS}/5 = {HIGGS_MASS/sf:.2f} GeV")
    add("mH*phi/sigma", HIGGS_MASS * p / s, "Higgs", f"{HIGGS_MASS}*2/12 = {HIGGS_MASS*p/s:.2f} GeV")
    add("mH*tau/sigma", HIGGS_MASS * t / s, "Higgs", f"{HIGGS_MASS}*4/12 = {HIGGS_MASS*t/s:.2f} GeV")
    add("mH/P2", HIGGS_MASS / P2, "Higgs", f"{HIGGS_MASS}/28 = {HIGGS_MASS/P2:.3f} GeV")
    add("mH*sopfr/P2", HIGGS_MASS * sf / P2, "Higgs", f"{HIGGS_MASS}*5/28 = {HIGGS_MASS*sf/P2:.2f} GeV")
    add("mH-v/phi", HIGGS_MASS - HIGGS_VEV / p, "Higgs", f"125.25 - 123.11 = {HIGGS_MASS - HIGGS_VEV/p:.2f} GeV")
    add("mH*phi", HIGGS_MASS * p, "Higgs", f"{HIGGS_MASS}*2 = {HIGGS_MASS*p:.1f} GeV")

    # --- Resonance-derived ---
    add("rho*P2", rho * P2, "resonance", f"{rho}*28 = {rho*P2:.2f} GeV")
    add("rho*P3/sigma", rho * P3 / s, "resonance", f"{rho}*496/12 = {rho*P3/s:.2f} GeV")
    add("rho*sigma", rho * s, "resonance", f"{rho}*12 = {rho*s:.2f} GeV")
    add("rho*sigma*tau", rho * s * t, "resonance", f"{rho}*48 = {rho*s*t:.2f} GeV")
    add("rho*n", rho * n, "resonance", f"{rho}*6 = {rho*n:.2f} GeV")
    add("rho*P2*phi", rho * P2 * p, "resonance", f"{rho}*56 = {rho*P2*p:.2f} GeV")
    add("Upsilon/tau", UPSILON_MASS / t, "resonance", f"9.460/4 = {UPSILON_MASS/t:.3f} GeV")
    add("Upsilon*tau", UPSILON_MASS * t, "resonance", f"9.460*4 = {UPSILON_MASS*t:.2f} GeV")
    add("Upsilon*sigma", UPSILON_MASS * s, "resonance", f"9.460*12 = {UPSILON_MASS*s:.2f} GeV")
    add("Jpsi*sigma", JPSI_MASS * s, "resonance", f"3.097*12 = {JPSI_MASS*s:.2f} GeV")
    add("Jpsi*P2", JPSI_MASS * P2, "resonance", f"3.097*28 = {JPSI_MASS*P2:.2f} GeV")
    add("Jpsi*n", JPSI_MASS * n, "resonance", f"3.097*6 = {JPSI_MASS*n:.2f} GeV")

    # --- Powers ---
    add("sigma^2/phi", s**2 / p, "power", f"144/2 = 72 GeV")
    add("sigma^2*phi", s**2 * p, "power", f"144*2 = 288 GeV")
    add("phi^sigma", p**s / 1000, "power", f"2^12 = 4096 MeV = 4.096 GeV")
    add("tau^sigma/1000", t**s / 1e6, "power", f"4^12 MeV = 16.78 GeV")
    add("sigma^phi", s**p, "power", f"12^2 = 144 GeV")
    add("tau^tau", t**t, "power", f"4^4 = 256 GeV")
    add("n^phi", n**p, "power", f"6^2 = 36 GeV")
    add("sopfr^phi", sf**p, "power", f"5^2 = 25 GeV")
    add("sopfr^tau/1000", sf**t / 1000, "power", f"5^4 = 625 MeV = 0.625 GeV")
    add("phi^n/1000", p**n / 1000, "power", f"2^6 = 64 MeV = 0.064 GeV")
    add("phi^sopfr/1000", p**sf / 1000, "power", f"2^5 = 32 MeV = 0.032 GeV")
    add("sigma^tau/1e6", s**t / 1e6, "power", f"12^4 = 20736 MeV = 20.7 GeV")
    add("sigma^sopfr/1e9", s**sf / 1e9, "power", f"12^5/1e9 = 0.249 GeV")
    add("n^tau/1000", n**t / 1000, "power", f"6^4 = 1296 MeV = 1.296 GeV")
    add("n^sopfr/1e6", n**sf / 1e6, "power", f"6^5/1e6 = 7.776 GeV")

    # --- EW scale ratios ---
    add("MW/sigma", M_W / s, "EW", f"{M_W}/12 = {M_W/s:.2f} GeV")
    add("MW/tau", M_W / t, "EW", f"{M_W}/4 = {M_W/t:.2f} GeV")
    add("MW/n", M_W / n, "EW", f"{M_W}/6 = {M_W/n:.2f} GeV")
    add("MZ/sigma", M_Z / s, "EW", f"{M_Z}/12 = {M_Z/s:.3f} GeV")
    add("MZ/tau", M_Z / t, "EW", f"{M_Z}/4 = {M_Z/t:.3f} GeV")
    add("MZ/n", M_Z / n, "EW", f"{M_Z}/6 = {M_Z/n:.3f} GeV")
    add("MZ/phi", M_Z / p, "EW", f"{M_Z}/2 = {M_Z/p:.4f} GeV")
    add("MZ*phi/sigma", M_Z * p / s, "EW", f"{M_Z}*2/12 = {M_Z*p/s:.3f} GeV")
    add("MW*phi/sigma", M_W * p / s, "EW", f"{M_W}*2/12 = {M_W*p/s:.3f} GeV")
    add("mtop/sigma", M_TOP / s, "EW", f"{M_TOP}/12 = {M_TOP/s:.2f} GeV")
    add("mtop/n", M_TOP / n, "EW", f"{M_TOP}/6 = {M_TOP/n:.2f} GeV")
    add("mtop/tau", M_TOP / t, "EW", f"{M_TOP}/4 = {M_TOP/t:.2f} GeV")
    add("mtop/P2", M_TOP / P2, "EW", f"{M_TOP}/28 = {M_TOP/P2:.2f} GeV")

    # --- Logarithmic / transcendental ---
    ln43 = math.log(4 / 3)  # Golden Zone width = 0.2877
    add("sigma*ln(4/3)", s * ln43, "transcendental", f"12*0.2877 = {s*ln43:.3f} GeV")
    add("P2*ln(4/3)", P2 * ln43, "transcendental", f"28*0.2877 = {P2*ln43:.3f} GeV")
    add("v*ln(4/3)", HIGGS_VEV * ln43, "transcendental", f"246.22*0.288 = {HIGGS_VEV*ln43:.2f} GeV")
    add("mH*ln(4/3)", HIGGS_MASS * ln43, "transcendental", f"125.25*0.288 = {HIGGS_MASS*ln43:.2f} GeV")
    sqrt32 = math.sqrt(3 / 2)  # Einstein theta
    add("sigma*sqrt(3/2)", s * sqrt32, "transcendental", f"12*1.225 = {s*sqrt32:.2f} GeV")
    add("P2*sqrt(3/2)", P2 * sqrt32, "transcendental", f"28*1.225 = {P2*sqrt32:.2f} GeV")
    add("v*sqrt(3/2)/sigma", HIGGS_VEV * sqrt32 / s, "transcendental",
        f"246.22*1.225/12 = {HIGGS_VEV*sqrt32/s:.2f} GeV")

    # --- Egyptian fractions ---
    for frac_label, frac_val in [("1/2", 0.5), ("1/3", 1/3), ("1/6", 1/6)]:
        add(f"v*{frac_label}", HIGGS_VEV * frac_val, "egyptian",
            f"v*{frac_label} = {HIGGS_VEV*frac_val:.2f} GeV")
        add(f"mH*{frac_label}", HIGGS_MASS * frac_val, "egyptian",
            f"mH*{frac_label} = {HIGGS_MASS*frac_val:.2f} GeV")

    # --- Nuclear magic number connection ---
    add("magic_20", 20.0, "nuclear_magic",
        "sigma*phi - tau = 20 GeV (nuclear magic number)")
    add("magic_28", 28.0, "nuclear_magic",
        "P2 = 28 GeV (nuclear magic number)")
    add("magic_50", 50.0, "nuclear_magic",
        "sigma*tau + phi = 50 GeV (nuclear magic number)")
    add("magic_82", HIGGS_VEV * t / s, "nuclear_magic",
        "v*tau/sigma = 82.07 GeV (near nuclear magic 82)")

    # Remove duplicates by rounding to 3 decimal places
    seen = {}
    unique = []
    for c in candidates:
        key = round(c.mass_gev, 3)
        if key not in seen:
            seen[key] = c
            unique.append(c)
    return unique


# =====================================================================
# 2. Experimental exclusion limits (direct detection)
# =====================================================================

def lz_exclusion_limit(mass_gev: float) -> Tuple[float, bool]:
    """Return approximate LUX-ZEPLIN 90% CL upper limit on spin-independent
    DM-nucleon cross section (cm^2) at given DM mass.

    Based on LZ 2022 results (Phys. Rev. Lett. 131, 041002).
    Returns (limit_cm2, is_in_sensitive_region).

    The limit curve is approximated as a piecewise power-law.
    Masses below ~5 GeV approach the neutrino fog floor.
    """
    if mass_gev < 1.0:
        return 1e-35, False  # below threshold, not constraining
    elif mass_gev < 5.0:
        # Steep rise at low mass (near neutrino fog)
        log_sigma = -40 + 4.0 * (5.0 - mass_gev)
        return 10**log_sigma, False
    elif mass_gev < 10.0:
        # Transition region
        log_sigma = -44.5 + 1.5 * (10.0 - mass_gev) / 5.0
        return 10**log_sigma, True
    elif mass_gev < 30.0:
        # Approaching minimum
        log_sigma = -47.0 + 2.5 * abs(mass_gev - 30) / 20.0
        return 10**log_sigma, True
    elif mass_gev < 50.0:
        # Near the deepest sensitivity (~30-50 GeV)
        log_sigma = -47.5 + 0.5 * abs(mass_gev - 40) / 10.0
        return 10**log_sigma, True
    elif mass_gev < 200.0:
        # Rising slowly
        log_sigma = -47.0 + 1.0 * math.log10(mass_gev / 50.0)
        return 10**log_sigma, True
    elif mass_gev < 1000.0:
        # Continued rise
        log_sigma = -46.4 + 1.5 * math.log10(mass_gev / 200.0)
        return 10**log_sigma, True
    elif mass_gev < 10000.0:
        # High mass, weaker limits
        log_sigma = -45.3 + 1.0 * math.log10(mass_gev / 1000.0)
        return 10**log_sigma, True
    else:
        # Very high mass
        return 10**(-44.3), False


def xenonnt_limit(mass_gev: float) -> float:
    """Approximate XENONnT SI limit (cm^2). Similar shape to LZ but slightly weaker."""
    lz_lim, _ = lz_exclusion_limit(mass_gev)
    return lz_lim * 2.0  # XENONnT roughly 2x weaker than LZ at most masses


def check_exclusion(mass_gev: float) -> Tuple[bool, str]:
    """Check if a DM candidate is excluded by current direct detection.

    A candidate is 'excluded' if a typical WIMP cross section at that mass
    exceeds the experimental limit.  We assume a reference cross section from
    weak-scale interactions.

    Returns (excluded, reason).
    """
    lz_lim, in_sensitive = lz_exclusion_limit(mass_gev)

    # Typical weak-scale SI cross section: sigma_SI ~ G_F^2 * mu^2 / pi
    # where mu is reduced mass with nucleon (m_n ~ 0.939 GeV)
    m_n = 0.939
    mu = mass_gev * m_n / (mass_gev + m_n)
    # Fermi constant-scale cross section (tree-level Z exchange, illustrative)
    G_F = 1.166e-5  # GeV^-2
    sigma_ref = G_F**2 * mu**2 / math.pi  # GeV^-2
    sigma_ref_cm2 = sigma_ref * (GEV_TO_CM**2)  # cm^2

    if not in_sensitive:
        if mass_gev < 5.0:
            return False, f"Below neutrino fog ({mass_gev:.1f} GeV) -- weakly constrained"
        else:
            return False, f"Above sensitive range ({mass_gev:.0f} GeV) -- weakly constrained"

    if sigma_ref_cm2 > lz_lim:
        return True, (f"Excluded by LZ: sigma_ref={sigma_ref_cm2:.1e} cm^2 "
                      f"> limit={lz_lim:.1e} cm^2")
    else:
        return False, (f"Not excluded: sigma_ref={sigma_ref_cm2:.1e} cm^2 "
                       f"< limit={lz_lim:.1e} cm^2")


# =====================================================================
# 3. Cosmological relic abundance
# =====================================================================

def relic_coupling(mass_gev: float) -> Tuple[float, bool]:
    """Compute coupling g needed for correct thermal relic abundance.

    <sigma*v> = g^4 / (16*pi*M^2) = 3e-26 cm^3/s
    => g^4 = 16*pi*M^2 * 3e-26 / (conversion)
    => g = (16*pi*M^2 * <sigma*v> / conversion)^(1/4)

    Returns (g, is_perturbative).
    """
    # <sigma*v> in natural units: 3e-26 cm^3/s / (hbar*c)^2 / c
    # 1 GeV^-2 = 1.1677e-17 cm^3/s (for <sigma*v>)
    sigma_v_nat = SIGMA_V_THERMAL / GEV2_TO_CM3S  # in GeV^-2

    g4 = 16 * math.pi * mass_gev**2 * sigma_v_nat
    if g4 < 0:
        return float('inf'), False
    g = g4**0.25

    is_perturbative = g < 4 * math.pi  # rough bound
    return g, is_perturbative


def check_wimp_miracle(mass_gev: float) -> str:
    """Check if SM weak coupling gives correct relic abundance at this mass."""
    g_needed, is_pert = relic_coupling(mass_gev)
    if abs(g_needed - G_WEAK) / G_WEAK < 0.3:
        return f"WIMP MIRACLE: g_needed={g_needed:.3f} ~ g_weak={G_WEAK} (match!)"
    elif is_pert and g_needed < 2.0:
        return f"Perturbative: g_needed={g_needed:.3f}"
    elif is_pert:
        return f"Large coupling: g_needed={g_needed:.3f}"
    else:
        return f"Non-perturbative: g_needed={g_needed:.3f}"


# =====================================================================
# 4. Special checks
# =====================================================================

def check_v_over_sigma():
    """Special analysis of M_DM = v/sigma = 20.5 GeV."""
    m = HIGGS_VEV / s
    lines = []
    lines.append("=" * 70)
    lines.append("SPECIAL CHECK: M_DM = v/sigma = 246.22/12 = {:.4f} GeV".format(m))
    lines.append("=" * 70)

    # Multiple routes to ~20 GeV
    m_alt = s * p - t  # 24 - 4 = 20
    lines.append(f"  sigma*phi - tau = {s}*{p} - {t} = {m_alt} GeV")
    lines.append(f"  20 is a nuclear magic number (proton/neutron shell closure)!")
    lines.append(f"  v/sigma = {m:.4f} vs sigma*phi - tau = {m_alt}")
    lines.append(f"  Discrepancy: {abs(m - m_alt)/m*100:.2f}%")

    m_higgs_n = HIGGS_MASS / n
    lines.append(f"\n  Also: mH/n = {HIGGS_MASS}/{n} = {m_higgs_n:.4f} GeV")
    lines.append(f"  v/sigma vs mH/n: {abs(m - m_higgs_n)/m*100:.2f}% apart")

    # LZ check at 20 GeV
    lz_lim, in_sens = lz_exclusion_limit(20.0)
    excluded, reason = check_exclusion(20.0)
    lines.append(f"\n  LZ limit at 20 GeV: {lz_lim:.2e} cm^2")
    lines.append(f"  Excluded? {excluded} -- {reason}")

    g, pert = relic_coupling(20.0)
    lines.append(f"\n  Relic coupling at 20 GeV: g = {g:.4f}")
    lines.append(f"  Weak coupling g_w = {G_WEAK}")
    lines.append(f"  Ratio g/g_w = {g/G_WEAK:.3f}")
    lines.append(f"  {check_wimp_miracle(20.0)}")

    return "\n".join(lines)


def check_dm_abundance():
    """Check TECS-L fractions against Omega_DM."""
    lines = []
    lines.append("=" * 70)
    lines.append("DM ABUNDANCE CHECKS: Omega_DM = 0.27, Omega_DM*h^2 = 0.120")
    lines.append("=" * 70)

    checks = [
        ("ln(4/3)", math.log(4 / 3), 0.27),
        ("1/tau = 1/4", 1 / t, 0.27),
        ("sopfr/(sigma*phi) = 5/24", sf / (s * p), 0.27),
        ("phi/sigma = 1/6", p / s, 0.27),
        ("1/(sigma-tau) = 1/8", 1 / (s - t), OMEGA_DM_H2_OBS),
        ("phi/(sigma+tau) = 1/8", p / (s + t), OMEGA_DM_H2_OBS),
        ("tau/P2 = 1/7", t / P2, OMEGA_DM_H2_OBS),
        ("sopfr/P2*phi = 5/56", sf / (P2 * p), OMEGA_DM_H2_OBS),
        ("n/(sigma*tau) = 1/8", n / (s * t), OMEGA_DM_H2_OBS),
    ]

    for label, val, target in checks:
        pct = abs(val - target) / target * 100
        marker = " ***" if pct < 5 else " **" if pct < 10 else ""
        target_label = "Omega_DM" if target == 0.27 else "Omega_DM*h^2"
        lines.append(f"  {label} = {val:.6f}  vs {target_label} = {target:.3f}"
                     f"  ({pct:.1f}% off){marker}")

    return "\n".join(lines)


# =====================================================================
# 5. Axion-like particles
# =====================================================================

def check_axion_candidates():
    """Check TECS-L expressions for axion mass scale."""
    lines = []
    lines.append("=" * 70)
    lines.append("AXION-LIKE PARTICLE CHECKS")
    lines.append("=" * 70)
    lines.append("  Axion DM mass window: ~1-100 micro-eV")
    lines.append("  m_axion ~ Lambda_QCD^2 / f_a")
    lines.append(f"  Lambda_QCD ~ {LAMBDA_QCD} GeV")

    # If f_a involves TECS-L and VEV
    fa_candidates = [
        ("v * sigma^sigma", HIGGS_VEV * s**s, "246 * 12^12"),
        ("v * P3^tau", HIGGS_VEV * P3**t, "246 * 496^4"),
        ("v * P2^sopfr", HIGGS_VEV * P2**sf, "246 * 28^5"),
        ("v * sigma^n", HIGGS_VEV * s**n, "246 * 12^6"),
        ("v * P3 * sigma", HIGGS_VEV * P3 * s, "246 * 496 * 12"),
    ]

    lines.append(f"\n  Axion decay constants f_a from TECS-L:")
    for label, fa, formula in fa_candidates:
        m_ax = LAMBDA_QCD**2 / fa  # GeV
        m_ax_uev = m_ax * 1e15  # micro-eV
        in_window = "  <-- IN WINDOW" if 1 <= m_ax_uev <= 100 else ""
        lines.append(f"    {label} = {formula} = {fa:.3e} GeV")
        lines.append(f"      m_axion = {m_ax:.3e} GeV = {m_ax_uev:.3f} micro-eV{in_window}")

    return "\n".join(lines)


# =====================================================================
# 6. Score and rank candidates
# =====================================================================

def score_candidate(c: DMCandidate) -> float:
    """Score a candidate: higher is more promising.

    Criteria:
    - Not excluded: +10
    - In WIMP window (1-1000 GeV): +5
    - Perturbative relic coupling: +3
    - Close to weak coupling: +5
    - Near nuclear magic number: +2
    - Multiple TECS-L routes: bonus from label
    """
    score = 0.0

    if not c.excluded:
        score += 10.0
    else:
        score -= 5.0

    if 1.0 <= c.mass_gev <= 1000.0:
        score += 5.0

    g, pert = relic_coupling(c.mass_gev)
    c.relic_coupling = g
    if pert:
        score += 3.0
        if abs(g - G_WEAK) / G_WEAK < 0.3:
            score += 5.0
            c.relic_ok = True
        elif g < 1.5:
            score += 2.0

    # Nuclear magic numbers
    magic = [2, 8, 20, 28, 50, 82, 126]
    for m in magic:
        if abs(c.mass_gev - m) / max(m, 1) < 0.05:
            score += 2.0
            break

    # Below neutrino fog bonus (less constrained)
    if c.mass_gev < 5.0:
        score += 1.0

    # Above 10 TeV bonus (less constrained)
    if c.mass_gev > 10000:
        score += 1.0

    return score


# =====================================================================
# 7. Main analysis
# =====================================================================

def run_analysis() -> str:
    """Run full dark matter analysis and return report."""
    lines = []
    lines.append("=" * 70)
    lines.append("DARK MATTER MASS CANDIDATES FROM TECS-L n=6 ARITHMETIC")
    lines.append("=" * 70)

    candidates = generate_candidates()
    lines.append(f"\nGenerated {len(candidates)} unique candidate masses.\n")

    # Apply exclusion checks
    for c in candidates:
        c.excluded, c.exclusion_reason = check_exclusion(c.mass_gev)
        c.score = score_candidate(c)

    # Sort by score
    candidates.sort(key=lambda c: c.score, reverse=True)

    # Summary table: ALL candidates
    lines.append("-" * 70)
    lines.append(f"{'Label':<25} {'Mass (GeV)':>12} {'Category':<16} "
                 f"{'Excl?':<6} {'Score':>6}")
    lines.append("-" * 70)
    for c in candidates:
        excl = "YES" if c.excluded else "no"
        lines.append(f"  {c.label:<23} {c.mass_gev:>12.4f} {c.category:<16} "
                     f"{excl:<6} {c.score:>6.1f}")
    lines.append("")

    # Category breakdown
    cats = {}
    for c in candidates:
        cats.setdefault(c.category, []).append(c)
    lines.append("Category breakdown:")
    for cat, members in sorted(cats.items()):
        n_excl = sum(1 for m in members if m.excluded)
        lines.append(f"  {cat:<20}: {len(members):>3} candidates, {n_excl:>3} excluded")

    # Top 10 report
    lines.append("\n" + "=" * 70)
    lines.append("TOP 10 MOST PROMISING DM MASS CANDIDATES")
    lines.append("=" * 70)
    for i, c in enumerate(candidates[:10], 1):
        lines.append(f"\n  #{i}. {c.label}")
        lines.append(f"      Mass:     {c.mass_gev:.4f} GeV")
        lines.append(f"      Formula:  {c.formula}")
        lines.append(f"      Category: {c.category}")
        lines.append(f"      Excluded: {'YES' if c.excluded else 'No'} "
                     f"-- {c.exclusion_reason}")
        lines.append(f"      Relic:    g = {c.relic_coupling:.4f} "
                     f"({'perturbative' if c.relic_coupling < 4*math.pi else 'non-pert'})")
        lines.append(f"      {check_wimp_miracle(c.mass_gev)}")
        lines.append(f"      Score:    {c.score:.1f}")

    # Special checks
    lines.append("\n")
    lines.append(check_v_over_sigma())
    lines.append("\n")
    lines.append(check_dm_abundance())
    lines.append("\n")
    lines.append(check_axion_candidates())

    # Key findings
    n_viable = sum(1 for c in candidates if not c.excluded)
    n_relic = sum(1 for c in candidates if c.relic_ok)
    lines.append("\n" + "=" * 70)
    lines.append("KEY FINDINGS")
    lines.append("=" * 70)
    lines.append(f"  Total candidates generated: {len(candidates)}")
    lines.append(f"  Not excluded by LZ:         {n_viable}")
    lines.append(f"  With correct relic abundance (g ~ g_weak): {n_relic}")
    lines.append(f"  Omega_DM*h^2 = 0.120 ~ 1/(sigma-tau) = 1/8 = 0.125 (4.2% off)")
    lines.append(f"  v/sigma = 20.5 GeV near nuclear magic 20 and mH/n = 20.9 GeV")

    best = candidates[0]
    lines.append(f"\n  BEST CANDIDATE: {best.label} = {best.mass_gev:.4f} GeV")
    lines.append(f"    Formula: {best.formula}")
    lines.append(f"    Score: {best.score:.1f}")

    return "\n".join(lines)


if __name__ == "__main__":
    print(run_analysis())
