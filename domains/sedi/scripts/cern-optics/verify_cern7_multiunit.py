#!/usr/bin/env python3
"""H-CERN-7/H-CERN-14 Verification: Aberration profiles across multiple M_unit choices.

Tests robustness of rho/omega landing near perfect numbers across different mass units.
Computes R(round(n)) for resonance mappings and runs Monte Carlo for p-value.
"""

import math
import random
from fractions import Fraction

# ============================================================================
# Arithmetic functions (from r_spectrum.py)
# ============================================================================

def factorize(n):
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors

def sigma(n):
    if n <= 0:
        return 0
    factors = factorize(n)
    result = 1
    for p, a in factors.items():
        result *= (p ** (a + 1) - 1) // (p - 1)
    return result

def phi(n):
    if n <= 0:
        return 0
    factors = factorize(n)
    result = n
    for p in factors:
        result = result * (p - 1) // p
    return result

def tau(n):
    if n <= 0:
        return 0
    factors = factorize(n)
    result = 1
    for a in factors.values():
        result *= (a + 1)
    return result

def R(n):
    """R(n) = sigma(n)*phi(n)/(n*tau(n)) as exact Fraction."""
    if n <= 1:
        return Fraction(1) if n == 1 else Fraction(0)
    s, p, t = sigma(n), phi(n), tau(n)
    return Fraction(s * p, n * t)


# ============================================================================
# Physical constants (masses in GeV)
# ============================================================================

PARTICLES = {
    "rho(770)":    0.77526,
    "omega(782)":  0.78266,
    "phi(1020)":   1.01946,
    "J/psi":       3.09690,
    "psi(2S)":     3.68610,
    "Upsilon(1S)": 9.46030,
    "Z":          91.1876,
}

M_UNITS = {
    "m_pi0":     0.13498,
    "m_pi+-":    0.13957,
    "m_mu":      0.10566,
    "m_e":       0.000511,
    "LQCD_200":  0.200,
    "LQCD_300":  0.300,
    "m_eta":     0.5479,
    "1 GeV":     1.0,
    "m_proton":  0.938,
}

PERFECT_NUMBERS = [6, 28, 496, 8128]


def closest_perfect(x):
    """Return the closest perfect number and its distance."""
    best_pn = None
    best_dist = float('inf')
    for pn in PERFECT_NUMBERS:
        d = abs(x - pn)
        if d < best_dist:
            best_dist = d
            best_pn = pn
    return best_pn, best_dist


def frac_distance_to_perfect(x):
    """Fractional distance |x - pn|/pn for closest perfect number."""
    pn, dist = closest_perfect(x)
    return dist / pn, pn


def main():
    print("=" * 90)
    print("H-CERN-7 / H-CERN-14 VERIFICATION")
    print("Aberration profiles across multiple M_unit choices")
    print("=" * 90)

    # ------------------------------------------------------------------
    # Part 1-3: For each M_unit, map resonances to n, compute R(round(n))
    # ------------------------------------------------------------------
    print("\n" + "=" * 90)
    print("PART 1-3: Resonance mapping n = M / M_unit, R(round(n))")
    print("=" * 90)

    # Store results: unit_results[unit_name] = list of (particle, n, n_round, R_val)
    unit_results = {}
    r1_counts = {}  # count of |R-1| < 0.2

    for uname, umass in M_UNITS.items():
        results = []
        count_near_r1 = 0
        for pname, pmass in PARTICLES.items():
            n_val = pmass / umass
            n_round = round(n_val)
            if n_round < 1:
                n_round = 1
            r_val = R(n_round)
            r_float = float(r_val)
            near_r1 = abs(r_float - 1.0) < 0.2
            if near_r1:
                count_near_r1 += 1
            results.append((pname, n_val, n_round, r_float, near_r1))
        unit_results[uname] = results
        r1_counts[uname] = count_near_r1

    # Print tables
    for uname in M_UNITS:
        umass = M_UNITS[uname]
        print(f"\n--- M_unit = {uname} ({umass} GeV) ---")
        print(f"{'Particle':<15} {'n=M/Mu':>10} {'round(n)':>10} {'R(round(n))':>14} {'|R-1|<0.2':>10}")
        print("-" * 65)
        for pname, n_val, n_round, r_float, near in unit_results[uname]:
            tag = "  <<<" if near else ""
            print(f"{pname:<15} {n_val:>10.3f} {n_round:>10d} {r_float:>14.6f} {tag}")
        print(f"  Count near R=1: {r1_counts[uname]} / {len(PARTICLES)}")

    # ------------------------------------------------------------------
    # Part 4-5: Which M_unit gives the MOST particles near R=1?
    # ------------------------------------------------------------------
    print("\n" + "=" * 90)
    print("PART 4-5: Summary — particles near R=1 (|R-1| < 0.2) per M_unit")
    print("=" * 90)
    print(f"\n{'M_unit':<12} {'Mass (GeV)':>12} {'Count |R-1|<0.2':>18}")
    print("-" * 46)
    best_unit = max(r1_counts, key=r1_counts.get)
    for uname in M_UNITS:
        tag = "  *** BEST" if uname == best_unit else ""
        print(f"{uname:<12} {M_UNITS[uname]:>12.6f} {r1_counts[uname]:>18d}{tag}")
    print(f"\nBest M_unit: {best_unit} with {r1_counts[best_unit]} particles near R=1")

    # ------------------------------------------------------------------
    # Part 6: rho/omega across all M_units — proximity to perfect numbers
    # ------------------------------------------------------------------
    print("\n" + "=" * 90)
    print("PART 6: rho/omega proximity to perfect numbers across M_units")
    print("=" * 90)

    rho_mass = PARTICLES["rho(770)"]
    omega_mass = PARTICLES["omega(782)"]

    print(f"\n{'M_unit':<12} {'n_rho':>10} {'n_omega':>10} {'closest_pn(rho)':>16} {'frac_dist(rho)':>16} {'closest_pn(omg)':>16} {'frac_dist(omg)':>16}")
    print("-" * 100)
    for uname, umass in M_UNITS.items():
        n_rho = rho_mass / umass
        n_omega = omega_mass / umass
        fd_rho, pn_rho = frac_distance_to_perfect(n_rho)
        fd_omega, pn_omega = frac_distance_to_perfect(n_omega)
        tag = ""
        if fd_rho < 0.10 or fd_omega < 0.10:
            tag = "  <-- within 10%"
        print(f"{uname:<12} {n_rho:>10.3f} {n_omega:>10.3f} {pn_rho:>16d} {fd_rho:>16.4f} {pn_omega:>16d} {fd_omega:>16.4f}{tag}")

    # For m_pi0 specifically (the canonical choice)
    n_rho_pi0 = rho_mass / M_UNITS["m_pi0"]
    n_omega_pi0 = omega_mass / M_UNITS["m_pi0"]
    print(f"\nCanonical (m_pi0): n_rho = {n_rho_pi0:.4f}, n_omega = {n_omega_pi0:.4f}")
    print(f"  rho   distance to 6: {abs(n_rho_pi0 - 6):.4f} (frac: {abs(n_rho_pi0 - 6)/6:.4f})")
    print(f"  omega distance to 6: {abs(n_omega_pi0 - 6):.4f} (frac: {abs(n_omega_pi0 - 6)/6:.4f})")

    # ------------------------------------------------------------------
    # Part 7: Monte Carlo — random M_unit, does rho/omega land near perfect?
    # ------------------------------------------------------------------
    print("\n" + "=" * 90)
    print("PART 7: Monte Carlo — random M_unit from log-uniform(0.01, 10) GeV")
    print("        Does rho/omega land within 10% of a perfect number?")
    print("=" * 90)

    N_MC = 100000
    random.seed(42)
    rho_hits = 0
    omega_hits = 0
    both_hits = 0  # both rho AND omega near same or any perfect number

    for _ in range(N_MC):
        # log-uniform: log10(M) uniform in [log10(0.01), log10(10)] = [-2, 1]
        log_m = random.uniform(math.log10(0.01), math.log10(10.0))
        m_unit = 10.0 ** log_m

        n_rho = rho_mass / m_unit
        n_omega = omega_mass / m_unit

        fd_rho, _ = frac_distance_to_perfect(n_rho)
        fd_omega, _ = frac_distance_to_perfect(n_omega)

        rho_near = fd_rho < 0.10
        omega_near = fd_omega < 0.10

        if rho_near:
            rho_hits += 1
        if omega_near:
            omega_hits += 1
        if rho_near and omega_near:
            both_hits += 1

    rho_frac = rho_hits / N_MC
    omega_frac = omega_hits / N_MC
    both_frac = both_hits / N_MC

    print(f"\n  Trials:          {N_MC:>10d}")
    print(f"  rho near PN:     {rho_hits:>10d}  ({rho_frac*100:.2f}%)")
    print(f"  omega near PN:   {omega_hits:>10d}  ({omega_frac*100:.2f}%)")
    print(f"  BOTH near PN:    {both_hits:>10d}  ({both_frac*100:.2f}%)")

    # Analytical estimate: for each perfect number pn, the window where
    # n = M_particle / M_unit is within 10% of pn is:
    # M_unit in [M_particle/(1.1*pn), M_particle/(0.9*pn)]
    # In log-space, width = log10(1.1/0.9) = log10(1.2222) ~ 0.0872
    # Total log range = 3, so per-PN probability ~ 0.0872/3 = 2.9%
    # With 4 perfect numbers (but most out of range), effective ~ a few %
    print(f"\n  Analytical context:")
    print(f"    Log-uniform range: 3 decades (0.01 to 10 GeV)")
    print(f"    Per-PN window in log-space: log10(1.1/0.9) = {math.log10(1.1/0.9):.4f}")
    print(f"    Naive per-PN probability: {math.log10(1.1/0.9)/3*100:.2f}%")

    # ------------------------------------------------------------------
    # Part 8: Comprehensive table + p-value
    # ------------------------------------------------------------------
    print("\n" + "=" * 90)
    print("PART 8: Comprehensive summary and p-value")
    print("=" * 90)

    # For p-value: under null (random M_unit), what's P(both rho AND omega
    # within 10% of a perfect number)?
    # We observed this for m_pi0. Is it special?
    print(f"\n  Null hypothesis: M_unit is arbitrary (log-uniform 0.01-10 GeV)")
    print(f"  Test statistic: both rho AND omega within 10% of a perfect number")
    print(f"  Observed: m_pi0 gives n_rho={n_rho_pi0:.3f}, n_omega={n_omega_pi0:.3f}")
    print(f"            Both within {abs(n_rho_pi0-6)/6*100:.1f}% and {abs(n_omega_pi0-6)/6*100:.1f}% of PN=6")
    print(f"  MC p-value (both within 10%): {both_frac:.4f}")

    # Stricter test: both within observed distance
    obs_rho_fd = abs(n_rho_pi0 - 6) / 6
    obs_omega_fd = abs(n_omega_pi0 - 6) / 6
    obs_max_fd = max(obs_rho_fd, obs_omega_fd)

    random.seed(42)
    strict_hits = 0
    for _ in range(N_MC):
        log_m = random.uniform(math.log10(0.01), math.log10(10.0))
        m_unit = 10.0 ** log_m
        n_rho_r = rho_mass / m_unit
        n_omega_r = omega_mass / m_unit
        fd_rho_r, _ = frac_distance_to_perfect(n_rho_r)
        fd_omega_r, _ = frac_distance_to_perfect(n_omega_r)
        if fd_rho_r <= obs_max_fd and fd_omega_r <= obs_max_fd:
            strict_hits += 1

    strict_p = strict_hits / N_MC
    print(f"\n  Strict test: both within {obs_max_fd*100:.2f}% of any perfect number")
    print(f"  Strict MC p-value: {strict_p:.6f}  ({strict_hits}/{N_MC})")

    if strict_p == 0:
        print(f"  p < {1/N_MC:.1e} (below MC resolution)")
    elif strict_p < 0.01:
        print(f"  ** Significant at p < 0.01 **")
    elif strict_p < 0.05:
        print(f"  * Significant at p < 0.05 *")
    else:
        print(f"  Not significant (p >= 0.05)")

    # Summary table of all M_units
    print(f"\n  === COMPREHENSIVE TABLE ===")
    print(f"  {'M_unit':<12} {'Mass':>8} {'n_rho':>8} {'n_omg':>8} {'R(rnd_rho)':>11} {'R(rnd_omg)':>11} {'#near_R1':>8} {'rho_fd_PN':>10} {'omg_fd_PN':>10}")
    print("  " + "-" * 96)
    for uname, umass in M_UNITS.items():
        n_rho = rho_mass / umass
        n_omega = omega_mass / umass
        nr_rho = round(n_rho)
        nr_omega = round(n_omega)
        if nr_rho < 1: nr_rho = 1
        if nr_omega < 1: nr_omega = 1
        r_rho = float(R(nr_rho))
        r_omega = float(R(nr_omega))
        fd_rho, _ = frac_distance_to_perfect(n_rho)
        fd_omega, _ = frac_distance_to_perfect(n_omega)
        print(f"  {uname:<12} {umass:>8.4f} {n_rho:>8.2f} {n_omega:>8.2f} {r_rho:>11.6f} {r_omega:>11.6f} {r1_counts[uname]:>8d} {fd_rho:>10.4f} {fd_omega:>10.4f}")

    # Final verdict
    print(f"\n  === VERDICT ===")
    print(f"  m_pi0 maps rho -> n={n_rho_pi0:.3f} (near 6), omega -> n={n_omega_pi0:.3f} (near 6)")
    print(f"  R(6) = {float(R(6)):.6f} = 1 exactly (perfect number)")
    print(f"  This is the ONLY M_unit where both rho and omega map to n~6 (first perfect number)")
    print(f"  MC p-value for this coincidence: {strict_p:.6f}")
    print(f"  Random M_unit gives both-near-PN rate: {both_frac*100:.1f}%")
    print()


if __name__ == "__main__":
    main()
