"""Gravitational Wave TECS-L Analysis — GWTC-3 catalog deep scan.

Comprehensive analysis of LIGO/Virgo/KAGRA gravitational wave events:
  - Mass ratio distribution vs TECS-L fractions
  - Chirp mass clustering and R-filter
  - ISCO = 6 r_g connection to P1
  - Peak frequency analysis
  - Monte Carlo validation

Data: GWTC-1, GWTC-2, GWTC-3 catalogs (~90 events)
"""
import math
import numpy as np
from collections import defaultdict
from ..tecs import (
    ALL_TARGETS, TARGETS_BASIC, TARGETS_DERIVED,
    SIGMA_P1, TAU_P1, PHI_P1,
    P1, P2, P3,
    EGYPTIAN_FRACTIONS,
)
from ..constants import N, SIGMA, TAU, PHI, RATIOS
from ..filter import r_filter


# ============================================================
# GWTC-3 EVENT DATABASE
# ============================================================
# Each event: m1, m2 (solar masses), M_chirp, q=m2/m1,
#             f_peak (Hz, where available), classification
# Sources: GWTC-1 (arXiv:1811.12907), GWTC-2 (arXiv:2010.14527),
#          GWTC-3 (arXiv:2111.03606)

GWTC_EVENTS = [
    # ── GWTC-1 (O1+O2) ──
    {"name": "GW150914", "m1": 35.6, "m2": 30.6, "Mc": 28.6, "q": 0.86,
     "f_peak": 150, "cls": "BBH", "note": "First detection"},
    {"name": "GW151012", "m1": 23.2, "m2": 13.6, "Mc": 15.2, "q": 0.59,
     "f_peak": 180, "cls": "BBH"},
    {"name": "GW151226", "m1": 13.7, "m2": 7.7, "Mc": 8.9, "q": 0.56,
     "f_peak": 450, "cls": "BBH"},
    {"name": "GW170104", "m1": 30.8, "m2": 20.0, "Mc": 21.4, "q": 0.65,
     "f_peak": 160, "cls": "BBH"},
    {"name": "GW170608", "m1": 11.0, "m2": 7.6, "Mc": 7.9, "q": 0.69,
     "f_peak": 420, "cls": "BBH"},
    {"name": "GW170729", "m1": 50.2, "m2": 34.0, "Mc": 35.4, "q": 0.68,
     "f_peak": 110, "cls": "BBH"},
    {"name": "GW170809", "m1": 35.0, "m2": 23.8, "Mc": 24.9, "q": 0.68,
     "f_peak": 150, "cls": "BBH"},
    {"name": "GW170814", "m1": 30.6, "m2": 25.2, "Mc": 24.1, "q": 0.82,
     "f_peak": 160, "cls": "BBH"},
    {"name": "GW170817", "m1": 1.46, "m2": 1.27, "Mc": 1.186, "q": 0.87,
     "f_peak": 1600, "cls": "BNS", "note": "First neutron star merger"},
    {"name": "GW170818", "m1": 35.4, "m2": 26.7, "Mc": 26.5, "q": 0.75,
     "f_peak": 145, "cls": "BBH"},
    {"name": "GW170823", "m1": 39.5, "m2": 29.0, "Mc": 29.2, "q": 0.73,
     "f_peak": 135, "cls": "BBH"},

    # ── GWTC-2 (O3a) ──
    {"name": "GW190408", "m1": 24.6, "m2": 18.4, "Mc": 18.3, "q": 0.75,
     "f_peak": 185, "cls": "BBH"},
    {"name": "GW190412", "m1": 29.7, "m2": 8.4, "Mc": 13.3, "q": 0.28,
     "f_peak": 250, "cls": "BBH", "note": "Asymmetric masses, higher harmonics"},
    {"name": "GW190421", "m1": 40.6, "m2": 32.8, "Mc": 31.7, "q": 0.81,
     "f_peak": 125, "cls": "BBH"},
    {"name": "GW190425", "m1": 1.7, "m2": 1.5, "Mc": 1.44, "q": 0.88,
     "f_peak": 1400, "cls": "BNS", "note": "Heaviest NS-NS system"},
    {"name": "GW190503", "m1": 41.1, "m2": 30.3, "Mc": 30.4, "q": 0.74,
     "f_peak": 130, "cls": "BBH"},
    {"name": "GW190512", "m1": 23.3, "m2": 12.5, "Mc": 14.6, "q": 0.54,
     "f_peak": 200, "cls": "BBH"},
    {"name": "GW190513", "m1": 35.7, "m2": 18.0, "Mc": 21.4, "q": 0.50,
     "f_peak": 165, "cls": "BBH"},
    {"name": "GW190514", "m1": 39.2, "m2": 28.0, "Mc": 28.7, "q": 0.71,
     "f_peak": 135, "cls": "BBH"},
    {"name": "GW190517", "m1": 37.4, "m2": 25.3, "Mc": 26.5, "q": 0.68,
     "f_peak": 145, "cls": "BBH"},
    {"name": "GW190519", "m1": 63.3, "m2": 40.2, "Mc": 42.6, "q": 0.64,
     "f_peak": 90, "cls": "BBH"},
    {"name": "GW190521", "m1": 85.0, "m2": 66.0, "Mc": 63.3, "q": 0.78,
     "f_peak": 60, "cls": "BBH", "note": "IMBH! Most massive merger"},
    {"name": "GW190521_074359", "m1": 42.2, "m2": 32.8, "Mc": 32.2, "q": 0.78,
     "f_peak": 120, "cls": "BBH"},
    {"name": "GW190527", "m1": 36.5, "m2": 22.0, "Mc": 24.0, "q": 0.60,
     "f_peak": 150, "cls": "BBH"},
    {"name": "GW190602", "m1": 67.6, "m2": 47.8, "Mc": 48.7, "q": 0.71,
     "f_peak": 80, "cls": "BBH"},
    {"name": "GW190620", "m1": 56.9, "m2": 36.3, "Mc": 38.5, "q": 0.64,
     "f_peak": 95, "cls": "BBH"},
    {"name": "GW190630", "m1": 35.1, "m2": 23.6, "Mc": 24.8, "q": 0.67,
     "f_peak": 150, "cls": "BBH"},
    {"name": "GW190701", "m1": 53.6, "m2": 40.5, "Mc": 40.3, "q": 0.76,
     "f_peak": 100, "cls": "BBH"},
    {"name": "GW190706", "m1": 67.0, "m2": 38.2, "Mc": 42.3, "q": 0.57,
     "f_peak": 85, "cls": "BBH"},
    {"name": "GW190707", "m1": 11.6, "m2": 8.4, "Mc": 8.5, "q": 0.72,
     "f_peak": 400, "cls": "BBH"},
    {"name": "GW190708", "m1": 17.6, "m2": 12.5, "Mc": 12.7, "q": 0.71,
     "f_peak": 240, "cls": "BBH"},
    {"name": "GW190720", "m1": 13.4, "m2": 7.8, "Mc": 8.8, "q": 0.58,
     "f_peak": 350, "cls": "BBH"},
    {"name": "GW190727", "m1": 38.2, "m2": 28.9, "Mc": 28.9, "q": 0.76,
     "f_peak": 135, "cls": "BBH"},
    {"name": "GW190728", "m1": 12.3, "m2": 8.1, "Mc": 8.6, "q": 0.66,
     "f_peak": 380, "cls": "BBH"},
    {"name": "GW190814", "m1": 23.2, "m2": 2.59, "Mc": 6.09, "q": 0.11,
     "f_peak": 300, "cls": "BBH?", "note": "Mystery: 2.59 Msun companion"},
    {"name": "GW190828_063405", "m1": 32.1, "m2": 26.6, "Mc": 25.6, "q": 0.83,
     "f_peak": 155, "cls": "BBH"},
    {"name": "GW190828_065509", "m1": 24.1, "m2": 10.2, "Mc": 13.2, "q": 0.42,
     "f_peak": 220, "cls": "BBH"},
    {"name": "GW190910", "m1": 44.5, "m2": 33.2, "Mc": 33.3, "q": 0.75,
     "f_peak": 120, "cls": "BBH"},
    {"name": "GW190915", "m1": 35.3, "m2": 24.4, "Mc": 25.2, "q": 0.69,
     "f_peak": 145, "cls": "BBH"},
    {"name": "GW190924", "m1": 8.9, "m2": 5.8, "Mc": 6.1, "q": 0.65,
     "f_peak": 490, "cls": "BBH"},
    {"name": "GW190929", "m1": 80.8, "m2": 24.1, "Mc": 34.7, "q": 0.30,
     "f_peak": 75, "cls": "BBH"},
    {"name": "GW190930", "m1": 12.3, "m2": 7.8, "Mc": 8.4, "q": 0.63,
     "f_peak": 370, "cls": "BBH"},

    # ── GWTC-3 (O3b) ──
    {"name": "GW191109", "m1": 65.0, "m2": 47.0, "Mc": 47.5, "q": 0.72,
     "f_peak": 80, "cls": "BBH"},
    {"name": "GW191129", "m1": 10.7, "m2": 6.7, "Mc": 7.2, "q": 0.63,
     "f_peak": 420, "cls": "BBH"},
    {"name": "GW191204", "m1": 11.9, "m2": 8.2, "Mc": 8.5, "q": 0.69,
     "f_peak": 390, "cls": "BBH"},
    {"name": "GW191215", "m1": 24.4, "m2": 18.1, "Mc": 18.0, "q": 0.74,
     "f_peak": 185, "cls": "BBH"},
    {"name": "GW191216", "m1": 12.1, "m2": 7.7, "Mc": 8.3, "q": 0.64,
     "f_peak": 380, "cls": "BBH"},
    {"name": "GW191222", "m1": 46.0, "m2": 22.2, "Mc": 26.5, "q": 0.48,
     "f_peak": 125, "cls": "BBH"},
    {"name": "GW200105", "m1": 8.9, "m2": 1.9, "Mc": 3.41, "q": 0.21,
     "f_peak": 500, "cls": "NSBH", "note": "NS-BH merger confirmed"},
    {"name": "GW200112", "m1": 33.6, "m2": 25.0, "Mc": 25.1, "q": 0.74,
     "f_peak": 150, "cls": "BBH"},
    {"name": "GW200115", "m1": 5.7, "m2": 1.5, "Mc": 2.43, "q": 0.26,
     "f_peak": 600, "cls": "NSBH", "note": "NS-BH merger confirmed"},
    {"name": "GW200129", "m1": 34.5, "m2": 29.0, "Mc": 27.4, "q": 0.84,
     "f_peak": 148, "cls": "BBH"},
    {"name": "GW200202", "m1": 10.1, "m2": 7.3, "Mc": 7.4, "q": 0.72,
     "f_peak": 410, "cls": "BBH"},
    {"name": "GW200208", "m1": 37.8, "m2": 28.8, "Mc": 28.6, "q": 0.76,
     "f_peak": 133, "cls": "BBH"},
    {"name": "GW200219", "m1": 37.5, "m2": 27.9, "Mc": 28.0, "q": 0.74,
     "f_peak": 135, "cls": "BBH"},
    {"name": "GW200225", "m1": 19.3, "m2": 14.0, "Mc": 14.1, "q": 0.73,
     "f_peak": 210, "cls": "BBH"},
    {"name": "GW200311", "m1": 34.2, "m2": 27.7, "Mc": 26.7, "q": 0.81,
     "f_peak": 148, "cls": "BBH"},
    {"name": "GW200316", "m1": 13.1, "m2": 7.8, "Mc": 8.7, "q": 0.60,
     "f_peak": 360, "cls": "BBH"},
]


def get_events(cls_filter=None):
    """Return events, optionally filtered by classification."""
    if cls_filter is None:
        return GWTC_EVENTS
    return [e for e in GWTC_EVENTS if e["cls"] == cls_filter]


def get_all_masses():
    """Return dict of all component masses keyed by event+component."""
    masses = {}
    for e in GWTC_EVENTS:
        masses[f"{e['name']}_m1"] = e["m1"]
        masses[f"{e['name']}_m2"] = e["m2"]
    return masses


def get_chirp_masses():
    """Return sorted chirp masses."""
    return sorted(e["Mc"] for e in GWTC_EVENTS)


def get_mass_ratios():
    """Return all q values."""
    return [e["q"] for e in GWTC_EVENTS]


def get_total_masses():
    """Return total masses m1+m2 for all events."""
    return {e["name"]: e["m1"] + e["m2"] for e in GWTC_EVENTS}


# ============================================================
# 1. MASS RATIO DISTRIBUTION vs TECS-L FRACTIONS
# ============================================================

# TECS-L fractions to test against mass ratio distribution
TECS_FRACTIONS = {
    "phi/tau = 1/2":       1/2,
    "tau/sigma = 1/3":     1/3,
    "1/tau = 1/4":         1/4,
    "1/n = 1/6":           1/6,
    "sopfr/n = 5/6":       5/6,
    "phi/n = 1/3":         1/3,   # same as tau/sigma
    "sigma/tau-2 = 1":     1.0,   # equal mass
    "(sigma-tau)/sigma = 2/3": 2/3,
}
# Deduplicate by value
_seen = {}
for k, v in TECS_FRACTIONS.items():
    rv = round(v, 6)
    if rv not in _seen:
        _seen[rv] = k
TECS_FRACTIONS_UNIQUE = {_seen[rv]: rv for rv in sorted(_seen)}


def analyze_mass_ratio_distribution(bin_width=0.05):
    """Histogram q values and check clustering at TECS-L fractions."""
    q_vals = np.array(get_mass_ratios())
    n_events = len(q_vals)

    print(f"\n  Mass ratio (q = m2/m1) distribution for {n_events} events:")
    print(f"    Range: [{q_vals.min():.3f}, {q_vals.max():.3f}]")
    print(f"    Mean: {q_vals.mean():.3f}, Median: {np.median(q_vals):.3f}")
    print(f"    Std: {q_vals.std():.3f}")

    # Histogram
    bins = np.arange(0, 1.0 + bin_width, bin_width)
    counts, edges = np.histogram(q_vals, bins=bins)

    print(f"\n  Histogram (bin width = {bin_width}):")
    for i in range(len(counts)):
        lo, hi = edges[i], edges[i+1]
        bar = "#" * counts[i]
        if counts[i] > 0:
            print(f"    [{lo:.2f}, {hi:.2f}): {counts[i]:2d}  {bar}")

    # Check proximity to TECS-L fractions
    print(f"\n  Events near TECS-L fractions (tolerance = {bin_width}):")
    tecs_hits = {}
    for label, frac in TECS_FRACTIONS_UNIQUE.items():
        if frac > 1.0:
            continue
        near = [(e["name"], e["q"]) for e in GWTC_EVENTS
                if abs(e["q"] - frac) < bin_width]
        tecs_hits[label] = near
        if near:
            names = ", ".join(f"{n}(q={q:.2f})" for n, q in near)
            print(f"    {label:30s} -> {len(near)} events: {names}")
        else:
            print(f"    {label:30s} -> 0 events")

    # Expected count per bin under uniform
    uniform_expect = n_events * bin_width
    print(f"\n  Uniform expectation per bin: {uniform_expect:.1f}")

    return {
        "q_values": q_vals,
        "histogram": (counts, edges),
        "tecs_hits": tecs_hits,
        "n_events": n_events,
    }


# ============================================================
# 2. CHIRP MASS ANALYSIS + R-FILTER
# ============================================================

def analyze_chirp_masses():
    """Analyze chirp mass distribution for TECS-L patterns."""
    Mc_sorted = np.array(get_chirp_masses())
    n = len(Mc_sorted)

    print(f"\n  Chirp mass distribution ({n} events):")
    print(f"    Range: [{Mc_sorted[0]:.2f}, {Mc_sorted[-1]:.2f}] Msun")
    print(f"    Mean: {Mc_sorted.mean():.2f}, Median: {np.median(Mc_sorted):.2f}")

    # Notable chirp masses
    tecs_mass_targets = {
        "P2=28":        28,
        "sigma=12":     12,
        "n=6":          6,
        "tau=4":        4,
        "sigma*phi=24": 24,
        "n^2=36":       36,
        "sigma-tau=8":  8,
    }

    print(f"\n  Chirp masses near TECS-L targets (5% tolerance):")
    mc_hits = []
    for label, target in sorted(tecs_mass_targets.items(), key=lambda x: x[1]):
        near = [(e["name"], e["Mc"]) for e in GWTC_EVENTS
                if abs(e["Mc"] - target) / target < 0.05]
        if near:
            for name, mc in near:
                err_pct = abs(mc - target) / target * 100
                print(f"    {name:20s} Mc = {mc:6.2f}  ~  {label:15s} ({target})  err = {err_pct:.1f}%")
                mc_hits.append({"event": name, "Mc": mc, "target": label,
                                "target_val": target, "err_pct": err_pct})

    # Special cases
    print(f"\n  Notable chirp mass values:")
    gw150914_mc = 28.6
    print(f"    GW150914: Mc = {gw150914_mc} ~ P2 = 28  (err = {abs(gw150914_mc-28)/28*100:.1f}%)")
    gw170817_mc = 1.186
    lambda_mass = 1.116  # Lambda baryon in GeV (but this is solar masses)
    print(f"    GW170817: Mc = {gw170817_mc} Msun  (note: 1.186 Msun, not GeV)")
    print(f"      Lambda baryon = {lambda_mass} GeV -- different units, no connection")

    # R-filter on sorted chirp masses
    print(f"\n  R-filter on sorted chirp masses:")
    rf = r_filter(Mc_sorted)
    if rf["ratio_hits"]:
        for hit in rf["ratio_hits"]:
            if hit["z_score"] > 1.5:
                print(f"    Pattern '{hit['pattern']}' (target={hit['target']:.4f}): "
                      f"count={hit['count']}, expected={hit['expected']:.1f}, "
                      f"z={hit['z_score']:.1f}")
    else:
        print(f"    No significant ratio patterns in consecutive chirp masses")

    # Consecutive ratios
    ratios = Mc_sorted[1:] / Mc_sorted[:-1]
    print(f"\n  Consecutive chirp mass ratios (sorted):")
    print(f"    Mean ratio: {ratios.mean():.4f}")
    print(f"    Std: {ratios.std():.4f}")

    return {
        "chirp_masses": Mc_sorted,
        "hits": mc_hits,
        "r_filter": rf,
        "consecutive_ratios": ratios,
    }


# ============================================================
# 3. TOTAL MASS ANALYSIS
# ============================================================

def analyze_total_masses():
    """Check total masses (m1+m2) against TECS-L multiples."""
    totals = get_total_masses()
    tecs_multiples = {
        "1x n=6":       6,
        "2x n=12":      12,
        "4x n=24":      24,
        "P2=28":        28,
        "6x n=36":      36,
        "8x n=48":      48,
        "10x n=60":     60,
        "sigma^2=144":  144,
        "P2*2=56":      56,
    }

    print(f"\n  Total masses (m1+m2) near TECS-L multiples (5% tolerance):")
    hits = []
    for event, mtot in sorted(totals.items(), key=lambda x: x[1]):
        for label, target in tecs_multiples.items():
            if abs(mtot - target) / target < 0.05:
                err = abs(mtot - target) / target * 100
                print(f"    {event:20s} M_tot = {mtot:6.1f}  ~  {label:15s} ({target})  err = {err:.1f}%")
                hits.append({"event": event, "M_tot": mtot, "target": label,
                             "target_val": target, "err_pct": err})

    return hits


# ============================================================
# 4. ISCO = 6 r_g: THE PERFECT NUMBER IN GR
# ============================================================

# Physical constants
G_SI = 6.674e-11       # m^3 kg^-1 s^-2
c_SI = 2.998e8          # m/s
M_sun = 1.989e30        # kg
pi = math.pi


def isco_frequency(M_total_solar, a_spin=0.0):
    """Compute ISCO orbital frequency for a Kerr black hole.

    For Schwarzschild (a=0): r_ISCO = 6 * GM/c^2 = P1 * r_g
    f_ISCO = c^3 / (6^(3/2) * pi * G * M)  [for a=0]

    For Kerr: r_ISCO ranges from 1 (prograde maximal) to 9 (retrograde maximal)
    gravitational radii.

    Returns f_ISCO in Hz.
    """
    M_kg = M_total_solar * M_sun

    if abs(a_spin) < 1e-10:
        # Schwarzschild: exact formula with 6^(3/2)
        f = c_SI**3 / (6**1.5 * pi * G_SI * M_kg)
        return f
    else:
        # Kerr: approximate ISCO radius
        # r_ISCO/M depends on spin parameter a
        # For prograde orbits (simplified):
        if a_spin >= 0:
            r_isco_rg = 6.0 - 4.0 * a_spin  # rough linear approx
        else:
            r_isco_rg = 6.0 - 4.0 * a_spin  # retrograde goes up to ~9
        r_isco_rg = max(1.0, min(9.0, r_isco_rg))
        f = c_SI**3 / (r_isco_rg**1.5 * pi * G_SI * M_kg)
        return f


def analyze_isco_connection():
    """Deep analysis of ISCO = 6 r_g and P1 = 6.

    The innermost stable circular orbit (ISCO) of a Schwarzschild black hole
    is at EXACTLY 6 gravitational radii. This 6 is P1, the first perfect number.

    Key formulas:
      r_ISCO = 6 * GM/c^2 = P1 * r_g
      f_ISCO = c^3 / (P1^(3/2) * pi * G * M)
      E_bind,ISCO = 1 - sqrt(8/9) = 1 - 2*sqrt(2)/3
      => Radiative efficiency eta = 1 - sqrt(8/9) ~ 5.72%
    """
    print("\n" + "=" * 70)
    print("  ISCO = 6 r_g : THE PERFECT NUMBER IN GENERAL RELATIVITY")
    print("=" * 70)

    print(f"""
  The Schwarzschild ISCO radius:
    r_ISCO = 6 * GM/c^2 = P1 * r_g

  Why 6? From the effective potential V_eff(r) for a test mass:
    V_eff = -GM/r + L^2/(2r^2) - GML^2/(c^2 * r^3)
    The stability condition d^2V/dr^2 = 0 gives r = 6M (geometrized units).

  The number 6 arises from solving a cubic equation in GR.
  It is NOT a free parameter -- it is fixed by the Einstein field equations.

  TECS-L connection:
    P1 = 6 = first perfect number (1 + 2 + 3 = 6 = 1 * 2 * 3)
    The ISCO sits at exactly P1 gravitational radii.
""")

    # ISCO binding energy and radiative efficiency
    eta_schwarz = 1 - math.sqrt(8/9)
    print(f"  Schwarzschild radiative efficiency:")
    print(f"    eta = 1 - sqrt(8/9) = {eta_schwarz:.6f}")
    print(f"    8/9 = (sigma - tau) / (sigma - sigma/tau) = 8/9")
    print(f"    Note: 8 = sigma - tau, 9 = sigma - sigma/tau")
    err_89 = abs((SIGMA_P1 - TAU_P1) / (SIGMA_P1 - SIGMA_P1/TAU_P1) - 8/9)
    print(f"    (sigma-tau)/(sigma-sigma/tau) = {(SIGMA_P1-TAU_P1)}/{(SIGMA_P1-SIGMA_P1/TAU_P1):.0f} = {(SIGMA_P1-TAU_P1)/(SIGMA_P1-SIGMA_P1/TAU_P1):.6f}  (err={err_89:.2e})")

    # Kerr ISCO range
    print(f"\n  Kerr black hole ISCO range:")
    print(f"    a = 0 (Schwarzschild): r_ISCO = 6 r_g = P1 * r_g")
    print(f"    a = 1 (max prograde):  r_ISCO = 1 r_g")
    print(f"    a = -1 (max retrograde): r_ISCO = 9 r_g")
    print(f"    Range: [1, 9]")
    print(f"    Note: 9 = sigma - sigma/tau = 12 - 3")
    print(f"    Range span: 9 - 1 = 8 = sigma - tau")
    print(f"    Midpoint: (1 + 9)/2 = 5 = sopfr(6)")
    print(f"    Geometric mean: sqrt(1*9) = 3 = sigma/tau")

    # ISCO frequency for all events
    print(f"\n  ISCO frequencies for GWTC events:")
    print(f"    {'Event':20s} {'M_tot':>8s} {'f_ISCO':>10s} {'f_peak':>8s} {'f_peak/f_ISCO':>14s}")
    print(f"    {'-'*20} {'-'*8} {'-'*10} {'-'*8} {'-'*14}")

    freq_ratios = []
    for e in GWTC_EVENTS:
        M_tot = e["m1"] + e["m2"]
        f_isco = isco_frequency(M_tot)
        f_pk = e.get("f_peak", None)
        if f_pk:
            ratio = f_pk / f_isco
            freq_ratios.append(ratio)
            print(f"    {e['name']:20s} {M_tot:8.1f} {f_isco:10.1f} {f_pk:8.0f} {ratio:14.3f}")

    freq_ratios = np.array(freq_ratios)
    print(f"\n  f_peak / f_ISCO statistics:")
    print(f"    Mean: {freq_ratios.mean():.3f}")
    print(f"    Std:  {freq_ratios.std():.3f}")
    print(f"    Note: f_peak > f_ISCO expected (merger/ringdown > ISCO)")

    # Check if ratio clusters near TECS-L values
    for label, target in TECS_FRACTIONS_UNIQUE.items():
        near = freq_ratios[np.abs(freq_ratios - target) / max(target, 0.01) < 0.1]
        if len(near) > 2:
            print(f"    {len(near)} events have f_peak/f_ISCO near {label}")

    return {
        "eta_schwarzschild": eta_schwarz,
        "isco_is_P1": True,
        "kerr_range": (1, 9),
        "freq_ratios": freq_ratios,
    }


# ============================================================
# 5. PEAK FREQUENCY ANALYSIS
# ============================================================

def analyze_frequencies():
    """Analyze GW peak frequencies for n=6 patterns."""
    events_with_freq = [e for e in GWTC_EVENTS if "f_peak" in e]
    freqs = np.array([e["f_peak"] for e in events_with_freq])
    bbh_freqs = np.array([e["f_peak"] for e in events_with_freq if e["cls"] == "BBH"])

    print(f"\n  Peak frequency analysis ({len(freqs)} events with f_peak):")
    print(f"    All events:  [{freqs.min():.0f}, {freqs.max():.0f}] Hz")
    print(f"    BBH only:    [{bbh_freqs.min():.0f}, {bbh_freqs.max():.0f}] Hz")

    # f_ISCO contains 6^(3/2) = P1^(3/2) in the denominator
    print(f"\n  The ISCO frequency formula:")
    print(f"    f_ISCO = c^3 / (6^(3/2) * pi * G * M)")
    print(f"    6^(3/2) = {6**1.5:.4f} = sqrt(216) = sqrt(6^3)")
    print(f"    Note: 216 = 6^3 = P1^3")
    print(f"    Also: 6^(3/2) = 6 * sqrt(6) = P1 * sqrt(P1)")

    # R-filter on BBH frequencies
    print(f"\n  R-filter on BBH peak frequencies:")
    rf = r_filter(np.sort(bbh_freqs))
    if rf["ratio_hits"]:
        for hit in rf["ratio_hits"]:
            if abs(hit["z_score"]) > 1.0:
                print(f"    Pattern '{hit['pattern']}': count={hit['count']}, "
                      f"z={hit['z_score']:.1f}")
    else:
        print(f"    No significant patterns in frequency distribution")

    return {"frequencies": freqs, "bbh_frequencies": bbh_freqs, "r_filter": rf}


# ============================================================
# 6. INTER-EVENT MASS RATIOS
# ============================================================

def analyze_cross_event_ratios():
    """Check ratios between different events' masses."""
    print(f"\n  Cross-event mass ratios at TECS-L values:")

    # Compare chirp masses between events
    events = GWTC_EVENTS
    hits = []
    tecs_targets = {
        "n=6":          6.0,
        "tau=4":        4.0,
        "sigma/tau=3":  3.0,
        "phi=2":        2.0,
        "sigma=12":     12.0,
        "sigma-tau=8":  8.0,
    }

    for i in range(len(events)):
        for j in range(i+1, len(events)):
            if events[j]["Mc"] == 0:
                continue
            ratio = events[i]["Mc"] / events[j]["Mc"]
            if ratio < 1:
                ratio = 1 / ratio

            for label, target in tecs_targets.items():
                if abs(ratio - target) / target < 0.02:
                    err = abs(ratio - target) / target * 100
                    hits.append({
                        "e1": events[i]["name"], "e2": events[j]["name"],
                        "Mc1": events[i]["Mc"], "Mc2": events[j]["Mc"],
                        "ratio": ratio, "target": label, "err_pct": err,
                    })

    hits.sort(key=lambda h: h["err_pct"])
    if hits:
        print(f"    Found {len(hits)} cross-event chirp mass ratios near TECS-L values")
        for h in hits[:15]:
            print(f"    {h['e1']:15s} / {h['e2']:15s} : "
                  f"Mc={h['Mc1']:.1f}/{h['Mc2']:.1f} = {h['ratio']:.4f} "
                  f"~ {h['target']} (err={h['err_pct']:.1f}%)")
    else:
        print(f"    No significant cross-event ratio matches")

    return hits


# ============================================================
# 7. MONTE CARLO VALIDATION
# ============================================================

def mc_validate_mass_ratios(n_trials=50_000, seed=42):
    """Monte Carlo: are mass ratios clustered at TECS-L fractions?

    Null model: draw q from the astrophysical prior (power law or uniform).
    """
    rng = np.random.default_rng(seed)
    observed_q = np.array(get_mass_ratios())
    n_events = len(observed_q)

    # TECS-L target fractions
    targets = {
        "1/6": 1/6, "1/4": 1/4, "1/3": 1/3, "1/2": 1/2,
        "2/3": 2/3, "5/6": 5/6,
    }
    tol = 0.05  # window around each fraction

    # Count observed hits
    obs_counts = {}
    for label, frac in targets.items():
        obs_counts[label] = np.sum(np.abs(observed_q - frac) < tol)

    print(f"\n  Monte Carlo validation: mass ratio clustering")
    print(f"    {n_trials:,} trials, {n_events} events, tolerance = {tol}")

    # Null model 1: Uniform in [0,1]
    print(f"\n    Null model: uniform q in [0, 1]")
    mc_counts_uniform = {label: [] for label in targets}
    for _ in range(n_trials):
        q_sim = rng.uniform(0, 1, size=n_events)
        for label, frac in targets.items():
            mc_counts_uniform[label].append(np.sum(np.abs(q_sim - frac) < tol))

    # Null model 2: Power-law q^beta (beta ~ 1.4 from LIGO population fits)
    beta = 1.4
    print(f"    Null model: power-law q^{beta} (astrophysical prior)")
    mc_counts_powerlaw = {label: [] for label in targets}
    for _ in range(n_trials):
        # Sample from p(q) ~ q^beta using inverse CDF
        u = rng.uniform(0, 1, size=n_events)
        q_sim = u ** (1 / (beta + 1))
        for label, frac in targets.items():
            mc_counts_powerlaw[label].append(np.sum(np.abs(q_sim - frac) < tol))

    # Compute p-values
    results = {}
    print(f"\n    {'Fraction':>10s} {'Obs':>5s} {'E[unif]':>8s} {'p(unif)':>10s} "
          f"{'E[plaw]':>8s} {'p(plaw)':>10s}")
    print(f"    {'-'*10} {'-'*5} {'-'*8} {'-'*10} {'-'*8} {'-'*10}")

    for label, frac in sorted(targets.items(), key=lambda x: x[1]):
        obs = obs_counts[label]
        mc_u = np.array(mc_counts_uniform[label])
        mc_p = np.array(mc_counts_powerlaw[label])

        p_uniform = (np.sum(mc_u >= obs) + 1) / (n_trials + 1)
        p_powerlaw = (np.sum(mc_p >= obs) + 1) / (n_trials + 1)

        # Use more conservative p-value
        p_val = max(p_uniform, p_powerlaw)

        results[label] = {
            "observed": int(obs), "frac": frac,
            "uniform_mean": float(mc_u.mean()),
            "powerlaw_mean": float(mc_p.mean()),
            "p_uniform": float(p_uniform),
            "p_powerlaw": float(p_powerlaw),
            "p_conservative": float(p_val),
        }

        flag = "***" if p_val < 0.001 else "**" if p_val < 0.01 else "*" if p_val < 0.05 else ""
        print(f"    {label:>10s} {obs:5d} {mc_u.mean():8.1f} {p_uniform:10.4f} "
              f"{mc_p.mean():8.1f} {p_powerlaw:10.4f}  {flag}")

    # Bonferroni correction
    raw_ps = [results[l]["p_conservative"] for l in targets]
    n_tests = len(targets)
    bonf_ps = [min(p * n_tests, 1.0) for p in raw_ps]
    print(f"\n    Bonferroni correction ({n_tests} tests):")
    for (label, _), bp in zip(sorted(targets.items(), key=lambda x: x[1]), bonf_ps):
        flag = " *" if bp < 0.05 else ""
        print(f"      {label:>10s}: p_corrected = {bp:.4f}{flag}")

    return results


def mc_validate_chirp_masses(n_trials=10_000, seed=42):
    """Monte Carlo: are chirp masses clustered at TECS-L values?"""
    rng = np.random.default_rng(seed)
    Mc_obs = np.array(get_chirp_masses())
    n = len(Mc_obs)

    targets = {"P2=28": 28, "sigma=12": 12, "n=6": 6, "sigma*phi=24": 24,
               "n^2=36": 36, "sigma-tau=8": 8, "tau=4": 4}
    tol_frac = 0.05  # 5% tolerance

    # Count observed hits
    obs_counts = {}
    for label, target in targets.items():
        obs_counts[label] = sum(1 for mc in Mc_obs if abs(mc - target) / target < tol_frac)

    # Null: KDE of log10(chirp masses)
    from scipy.stats import gaussian_kde
    log_mc = np.log10(Mc_obs[Mc_obs > 0])
    kde = gaussian_kde(log_mc)

    print(f"\n  MC validation: chirp mass clustering ({n_trials:,} trials)")
    mc_counts = {label: [] for label in targets}
    for _ in range(n_trials):
        log_samples = kde.resample(n, seed=rng.integers(0, 2**31)).flatten()
        mc_sim = 10 ** log_samples
        for label, target in targets.items():
            mc_counts[label].append(sum(1 for m in mc_sim if abs(m - target) / target < tol_frac))

    print(f"\n    {'Target':>15s} {'Obs':>5s} {'E[KDE]':>8s} {'p-value':>10s}")
    print(f"    {'-'*15} {'-'*5} {'-'*8} {'-'*10}")

    for label, target in sorted(targets.items(), key=lambda x: x[1]):
        obs = obs_counts[label]
        mc_arr = np.array(mc_counts[label])
        p_val = (np.sum(mc_arr >= obs) + 1) / (n_trials + 1)
        flag = "***" if p_val < 0.001 else "**" if p_val < 0.01 else "*" if p_val < 0.05 else ""
        print(f"    {label:>15s} {obs:5d} {mc_arr.mean():8.2f} {p_val:10.4f}  {flag}")

    return mc_counts


# ============================================================
# 8. FULL REPORT
# ============================================================

def run_analysis():
    """Run complete GWTC-3 gravitational wave TECS-L analysis."""
    print("=" * 72)
    print("  GRAVITATIONAL WAVE ANALYSIS: GWTC-3 CATALOG x TECS-L")
    print("  LIGO/Virgo/KAGRA events vs n=6 arithmetic")
    print("=" * 72)

    n_events = len(GWTC_EVENTS)
    n_bbh = len([e for e in GWTC_EVENTS if e["cls"] == "BBH"])
    n_bns = len([e for e in GWTC_EVENTS if e["cls"] == "BNS"])
    n_nsbh = len([e for e in GWTC_EVENTS if e["cls"] == "NSBH"])
    print(f"\n  Database: {n_events} events ({n_bbh} BBH, {n_bns} BNS, "
          f"{n_nsbh} NSBH, {n_events - n_bbh - n_bns - n_nsbh} other)")

    # 1. Mass ratio distribution
    print("\n" + "=" * 72)
    print("  1. MASS RATIO DISTRIBUTION vs TECS-L FRACTIONS")
    print("=" * 72)
    q_result = analyze_mass_ratio_distribution()

    # 2. Chirp mass analysis
    print("\n" + "=" * 72)
    print("  2. CHIRP MASS DISTRIBUTION + R-FILTER")
    print("=" * 72)
    mc_result = analyze_chirp_masses()

    # 3. Total mass analysis
    print("\n" + "=" * 72)
    print("  3. TOTAL MASSES vs TECS-L MULTIPLES")
    print("=" * 72)
    tot_result = analyze_total_masses()

    # 4. ISCO = P1 analysis
    print("\n" + "=" * 72)
    print("  4. ISCO = 6 r_g = P1 GRAVITATIONAL RADII")
    print("=" * 72)
    isco_result = analyze_isco_connection()

    # 5. Frequency analysis
    print("\n" + "=" * 72)
    print("  5. PEAK FREQUENCY ANALYSIS")
    print("=" * 72)
    freq_result = analyze_frequencies()

    # 6. Cross-event ratios
    print("\n" + "=" * 72)
    print("  6. CROSS-EVENT CHIRP MASS RATIOS")
    print("=" * 72)
    cross_result = analyze_cross_event_ratios()

    # 7. Monte Carlo validation
    print("\n" + "=" * 72)
    print("  7. MONTE CARLO VALIDATION")
    print("=" * 72)
    mc_q = mc_validate_mass_ratios(n_trials=50_000)
    mc_mc = mc_validate_chirp_masses(n_trials=10_000)

    # 8. Summary
    print("\n" + "=" * 72)
    print("  8. SUMMARY AND ASSESSMENT")
    print("=" * 72)

    print(f"""
  KEY FINDING: r_ISCO = 6 r_g = P1 * r_g
  ──────────────────────────────────────────
  The innermost stable circular orbit of a Schwarzschild black hole
  is at EXACTLY 6 gravitational radii, where 6 = P1 (first perfect number).

  This is NOT a free parameter. It emerges from the Einstein field equations:
    - The effective potential V_eff has an inflection point at r = 6M
    - This is a mathematical consequence of the 1/r^3 GR correction term
    - The cubic equation yields 6 as the stability boundary

  The ISCO radius of 6 r_g leads to:
    - Binding energy: E/mc^2 = 1 - sqrt(8/9) ~ 5.72%
    - ISCO frequency: f = c^3 / (6^(3/2) * pi * G * M)
    - 6^(3/2) = 6*sqrt(6) ~ 14.70

  Kerr ISCO range: [1, 9] gravitational radii
    - Span = 8 = sigma - tau
    - Non-spinning value = 6 = P1
    - Midpoint = 5 = sopfr(6)
    - Geometric mean = 3 = sigma/tau

  ASSESSMENT:
  The r_ISCO = 6 is a genuine mathematical fact of GR, not numerology.
  However, the question is whether this 6 has deeper significance.
  In TECS-L, 6 is the smallest perfect number and generates all key constants.
  That the ISCO happens to sit at P1 radii is striking but may be coincidence.

  MASS RATIO / CHIRP MASS RESULTS:
  The mass ratio and chirp mass distributions are shaped by astrophysical
  processes (stellar evolution, pair-instability supernovae, mass-gap physics).
  Any apparent clustering at TECS-L values must survive Monte Carlo testing
  against realistic astrophysical population models.

  Statistical verdict: see MC validation above for per-test p-values.
""")

    return {
        "n_events": n_events,
        "q_result": q_result,
        "mc_result": mc_result,
        "tot_result": tot_result,
        "isco_result": isco_result,
        "freq_result": freq_result,
        "cross_result": cross_result,
        "mc_q_validation": mc_q,
    }


if __name__ == "__main__":
    run_analysis()
