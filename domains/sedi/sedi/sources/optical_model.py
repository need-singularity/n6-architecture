"""Optical Model Analysis — TECS-L lens/optics analogies applied to particle masses.

The R-spectrum has built-in optical structure:
  - f = 1/(sigma*phi) = 1/24 is the "focal length"
  - R(6) = 1 uniquely: achromatic fixed point (no chromatic aberration)
  - delta+ = 1/n = 1/6, delta- = 1/tau = 1/4 are R-spectrum gaps
  - ln(4/3) = 0.2877 = Golden Zone bandwidth

This module applies thin-lens, lensmaker, Snell, interference, and chromatic
aberration formulas to PDG particle masses, searching for patterns that
align with TECS-L n=6 arithmetic.
"""

import math
import numpy as np
from itertools import combinations
from collections import OrderedDict
from scipy.stats import norm, gaussian_kde

from ..tecs import (
    R, sigma, tau, phi,
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, P1,
    ALL_TARGETS,
)
from .pdg import get_all, get_masses, PARTICLES


# =====================================================================
# Constants
# =====================================================================

N = P1           # 6
SIGMA = SIGMA_P1 # 12
TAU_ = TAU_P1    # 4
PHI_ = PHI_P1    # 2

# Optical constants from TECS-L
FOCAL_LENGTHS = OrderedDict([
    ('f=1/24 (1/sigma*phi)', 1/24),
    ('f=1/12 (1/sigma)',     1/12),
    ('f=1/4  (1/tau)',       1/4),
    ('f=1/6  (1/n)',         1/6),
])

# R-spectrum values for small integers
R_VALUES = {}
for _n in range(2, 31):
    R_VALUES[_n] = R(_n)

# All particle masses
ALL_PARTICLES = get_all()
MASS_DICT = get_masses()
MASS_LIST = [(p['name'], p['mass']) for p in ALL_PARTICLES]
MASS_ARRAY = np.array([p['mass'] for p in ALL_PARTICLES])


# =====================================================================
# Helper: p-value to sigma
# =====================================================================

def _p_to_sigma(p):
    if p <= 0:
        return float('inf')
    return norm.isf(p)


def _nearest_particle(target_mass, exclude=None):
    """Find nearest known particle to a target mass."""
    best_name, best_mass, best_err = None, None, float('inf')
    for name, mass in MASS_LIST:
        if exclude and name in exclude:
            continue
        err = abs(mass - target_mass) / target_mass if target_mass != 0 else float('inf')
        if err < best_err:
            best_name, best_mass, best_err = name, mass, err
    return best_name, best_mass, best_err


# =====================================================================
# 1. THIN LENS EQUATION: 1/f = 1/u + 1/v
# =====================================================================

def thin_lens_scan(tolerance=0.03):
    """For each focal length f, find particle pairs where |1/m1 + 1/m2 - 1/f| < tol.

    The thin lens equation 1/f = 1/u + 1/v is tested with particle masses
    as object/image distances.
    """
    results = {}
    pairs = list(combinations(MASS_LIST, 2))

    for f_name, f_val in FOCAL_LENGTHS.items():
        target = 1.0 / f_val  # lens power
        matches = []
        for (n1, m1), (n2, m2) in pairs:
            if m1 == 0 or m2 == 0:
                continue
            inv_sum = 1.0/m1 + 1.0/m2
            rel_err = abs(inv_sum - target) / target
            if rel_err < tolerance:
                matches.append({
                    'p1': n1, 'm1': m1,
                    'p2': n2, 'm2': m2,
                    'inv_sum': inv_sum,
                    'target': target,
                    'f': f_val,
                    'error_pct': rel_err * 100,
                })
        matches.sort(key=lambda x: x['error_pct'])
        results[f_name] = matches
    return results


# =====================================================================
# 2. LENSMAKER'S EQUATION: 1/f = (n_refr - 1)(1/R1 - 1/R2)
# =====================================================================

def lensmaker_scan(tolerance=0.03):
    """Use R(n) as refractive index, particle masses as radii of curvature.

    For each n and particle pair, compute f from the lensmaker equation
    and check if f equals any known particle mass.
    """
    n_values = [2, 3, 4, 5, 7, 8, 9, 10, 12, 15, 20]
    results = []
    pairs = list(combinations(MASS_LIST, 2))

    for n_refr in n_values:
        r_val = R(n_refr)
        if r_val == 1:  # R(6) = 1 gives infinite focal length
            continue
        prefactor = r_val - 1

        for (n1, m1), (n2, m2) in pairs:
            if m1 == 0 or m2 == 0:
                continue
            curvature_diff = 1.0/m1 - 1.0/m2
            if curvature_diff == 0:
                continue
            inv_f = prefactor * curvature_diff
            if inv_f <= 0:
                # Also try swapped
                inv_f = prefactor * (1.0/m2 - 1.0/m1)
                if inv_f <= 0:
                    continue
            f_computed = 1.0 / inv_f

            # Check against all particle masses
            pname, pmass, perr = _nearest_particle(f_computed, exclude={n1, n2})
            if perr < tolerance:
                results.append({
                    'n': n_refr, 'R_n': r_val,
                    'p1': n1, 'm1': m1,
                    'p2': n2, 'm2': m2,
                    'f_computed': f_computed,
                    'matched_particle': pname,
                    'matched_mass': pmass,
                    'error_pct': perr * 100,
                })

    results.sort(key=lambda x: x['error_pct'])
    return results


# =====================================================================
# 3. TELESCOPE RESOLVING POWER
# =====================================================================

def telescope_resolving_power():
    """Compute theta = 1.22 * delta_plus / sigma and compare to constants."""
    delta_plus = 1.0 / N          # 1/6
    delta_minus = 1.0 / TAU_      # 1/4

    theta = 1.22 * delta_plus / SIGMA  # 1.22 / (6*12) = 1.22/72
    theta_val = theta

    # Compare to known coupling constants
    alpha_em = 1.0 / 137.036      # fine structure
    alpha_s_mz = 0.1179           # strong coupling at M_Z
    alpha_w = 1.0 / 29.0          # weak coupling approx
    g_fermi_gev2 = 1.1664e-5      # Fermi constant

    comparisons = {
        'theta = 1.22*delta+/sigma': theta_val,
        'alpha_em (1/137)': alpha_em,
        'alpha_s(M_Z)': alpha_s_mz,
        'alpha_weak (~1/29)': alpha_w,
        'Fermi G_F (GeV^-2)': g_fermi_gev2,
        'theta/alpha_em': theta_val / alpha_em,
        'theta/alpha_s': theta_val / alpha_s_mz,
        'theta*72': theta_val * 72,  # should be 1.22
    }

    # Also: theta with delta_minus
    theta2 = 1.22 * delta_minus / SIGMA  # 1.22 / 48
    comparisons['theta2 = 1.22*delta-/sigma'] = theta2
    comparisons['theta2/alpha_w'] = theta2 / alpha_w

    return comparisons


# =====================================================================
# 4. INTERFERENCE / DIFFRACTION
# =====================================================================

def interference_scan(tolerance=0.03):
    """For mass differences d between particles, check if d/sigma = known mass.

    From d*sin(pi/6) = 6*lambda => d = 12*lambda = sigma(6)*lambda.
    So lambda = d/sigma. Check if lambda is a known particle mass.
    """
    results = []
    pairs = list(combinations(MASS_LIST, 2))

    for (n1, m1), (n2, m2) in pairs:
        d = abs(m2 - m1)
        if d == 0:
            continue
        lam = d / SIGMA  # d / 12

        pname, pmass, perr = _nearest_particle(lam, exclude={n1, n2})
        if perr < tolerance:
            results.append({
                'p1': n1, 'm1': m1,
                'p2': n2, 'm2': m2,
                'diff': d,
                'lambda': lam,
                'matched_particle': pname,
                'matched_mass': pmass,
                'error_pct': perr * 100,
            })

    results.sort(key=lambda x: x['error_pct'])
    return results


# =====================================================================
# 5. SNELL'S LAW: n1*sin(theta1) = n2*sin(theta2)
# =====================================================================

def snell_law_analysis():
    """Apply Snell's law with R(n) as refractive indices.

    Compute refraction angles and check for physically meaningful results.
    """
    results = []
    n_pairs = [(2, 3), (3, 4), (4, 5), (2, 5), (3, 5), (2, 4),
               (3, 7), (4, 7), (5, 7), (2, 7)]

    incident_angles = [math.pi/6, math.pi/4, math.pi/3]
    angle_names = ['pi/6', 'pi/4', 'pi/3']

    for n1, n2 in n_pairs:
        r1 = R(n1)
        r2 = R(n2)
        if r2 == 0:
            continue
        for theta_in, aname in zip(incident_angles, angle_names):
            sin_out = r1 * math.sin(theta_in) / r2
            if abs(sin_out) > 1:
                # Total internal reflection
                results.append({
                    'n1': n1, 'n2': n2,
                    'R1': r1, 'R2': r2,
                    'theta_in': aname,
                    'result': 'TOTAL INTERNAL REFLECTION',
                    'sin_out': sin_out,
                    'theta_out': None,
                    'significance': None,
                })
                continue

            theta_out = math.asin(sin_out)
            theta_out_deg = math.degrees(theta_out)

            # Check if output angle is a "nice" fraction of pi
            significance = None
            for denom in [2, 3, 4, 5, 6, 8, 10, 12]:
                target = math.pi / denom
                if abs(theta_out - target) / target < 0.03:
                    significance = f'pi/{denom}'
                    break

            # Check if sin_out matches a TECS ratio
            for tname, tval in ALL_TARGETS.items():
                if 0 < tval < 1 and abs(sin_out - tval) / tval < 0.03:
                    significance = f'sin(out) = {tname}'
                    break

            results.append({
                'n1': n1, 'n2': n2,
                'R1': r1, 'R2': r2,
                'theta_in': aname,
                'sin_out': sin_out,
                'theta_out_deg': theta_out_deg,
                'significance': significance,
            })

    return results


# =====================================================================
# 6. OPTICAL PATH LENGTH: OPL = n * d
# =====================================================================

def optical_path_length():
    """Compute OPL = R(n) * mass for each particle.

    Check if R(round(m1/m2)) * m1 gives constants for related particles,
    and if summing OPLs along the resonance ladder gives meaningful totals.
    """
    # Resonance ladder
    ladder = [
        ('rho_770', 0.77526),
        ('J_psi', 3.09690),
        ('Upsilon_1S', 9.46040),
    ]

    # OPL for each particle at different R-values
    opl_table = []
    for name, mass in MASS_LIST:
        for n_val in [2, 3, 4, 5, 6, 7, 8, 12]:
            r_val = R(n_val)
            opl = r_val * mass
            opl_table.append({
                'particle': name, 'mass': mass,
                'n': n_val, 'R_n': r_val, 'OPL': opl,
            })

    # Sum OPLs along the ladder
    ladder_opls = {}
    for n_val in [2, 3, 4, 5, 6, 7, 8, 12]:
        r_val = R(n_val)
        total = sum(r_val * m for _, m in ladder)
        ladder_opls[f'n={n_val}, R={r_val:.4f}'] = {
            'total_OPL': total,
            'nearest': _nearest_particle(total),
        }

    # Check mass-ratio-based R
    ratio_opl_results = []
    pairs = list(combinations(MASS_LIST, 2))
    for (n1, m1), (n2, m2) in pairs:
        if m2 == 0 or m1 == 0:
            continue
        ratio = max(m1, m2) / min(m1, m2)
        n_approx = round(ratio)
        if n_approx < 2 or n_approx > 30:
            continue
        r_val = R(n_approx)
        opl = r_val * max(m1, m2)
        # Check if this OPL equals another particle mass
        pname, pmass, perr = _nearest_particle(opl, exclude={n1, n2})
        if perr < 0.03:
            ratio_opl_results.append({
                'p1': n1 if m1 > m2 else n2,
                'p2': n2 if m1 > m2 else n1,
                'mass_ratio': ratio,
                'n_approx': n_approx,
                'R_n': r_val,
                'OPL': opl,
                'matched': pname,
                'matched_mass': pmass,
                'error_pct': perr * 100,
            })
    ratio_opl_results.sort(key=lambda x: x['error_pct'])

    return {
        'ladder_opls': ladder_opls,
        'ratio_opl_matches': ratio_opl_results[:30],
    }


# =====================================================================
# 7. CHROMATIC ABERRATION
# =====================================================================

def chromatic_aberration(tolerance=0.03):
    """Particles with mass ratio ~ 6 are "in focus" (achromatic).

    R(6) = 1 means n=6 is achromatic. Aberration = |R(ratio) - 1|.
    Pairs with low aberration are "in focus" optically.
    """
    results = []
    pairs = list(combinations(MASS_LIST, 2))

    for (n1, m1), (n2, m2) in pairs:
        if m1 == 0 or m2 == 0:
            continue
        ratio = max(m1, m2) / min(m1, m2)
        n_approx = round(ratio)
        if n_approx < 2 or n_approx > 30:
            continue
        r_val = R(n_approx)
        aberration = abs(r_val - 1)

        results.append({
            'p_heavy': n1 if m1 >= m2 else n2,
            'p_light': n2 if m1 >= m2 else n1,
            'm_heavy': max(m1, m2),
            'm_light': min(m1, m2),
            'ratio': ratio,
            'n_approx': n_approx,
            'R_n': r_val,
            'aberration': aberration,
            'ratio_error_pct': abs(ratio - n_approx) / n_approx * 100,
        })

    # Sort by aberration (lowest = most achromatic)
    results.sort(key=lambda x: (x['aberration'], x['ratio_error_pct']))

    # Also compute dR/dn near n=6 (dispersion)
    # Numerical derivative
    r5, r6, r7 = R(5), R(6), R(7)
    dispersion_left = r6 - r5     # = 1 - 6/5 = -1/5
    dispersion_right = r7 - r6    # R(7) - 1
    dispersion_avg = (r7 - r5) / 2

    dispersion_info = {
        'R(5)': r5, 'R(6)': r6, 'R(7)': r7,
        'dR/dn (left)': dispersion_left,
        'dR/dn (right)': dispersion_right,
        'dR/dn (avg)': dispersion_avg,
    }

    return results, dispersion_info


# =====================================================================
# 8. MONTE CARLO VALIDATION
# =====================================================================

def mc_validate_thin_lens(real_matches, f_val, n_trials=10000, seed=42):
    """MC validation for thin lens matches — vectorized.

    Draw random mass sets from PDG KDE and count how many produce
    at least as many pairs satisfying |1/m1 + 1/m2 - 1/f| / (1/f) < 3%.
    """
    rng = np.random.default_rng(seed)
    log_m = np.log10(MASS_ARRAY[MASS_ARRAY > 0])
    lo, hi = log_m.min() - 0.3, log_m.max() + 0.3

    n_real = len(real_matches)
    n_p = len(MASS_LIST)
    target = 1.0 / f_val
    tol = 0.03

    count_ge = 0
    for _ in range(n_trials):
        masses = 10 ** rng.uniform(lo, hi, size=n_p)
        inv_m = 1.0 / masses
        # Upper triangle pairwise sums via broadcasting
        inv_sums = inv_m[:, None] + inv_m[None, :]
        # Take upper triangle only
        iu = np.triu_indices(n_p, k=1)
        vals = inv_sums[iu]
        hits = np.sum(np.abs(vals - target) / target < tol)
        if hits >= n_real:
            count_ge += 1

    return count_ge / n_trials


def mc_validate_interference(n_real_matches, n_trials=10000, seed=42):
    """MC validation for interference matches — vectorized."""
    rng = np.random.default_rng(seed)
    log_m = np.log10(MASS_ARRAY[MASS_ARRAY > 0])
    lo, hi = log_m.min() - 0.3, log_m.max() + 0.3
    n_p = len(MASS_LIST)
    tol = 0.03

    count_ge = 0
    for _ in range(n_trials):
        masses = np.sort(10 ** rng.uniform(lo, hi, size=n_p))
        # All pairwise diffs / sigma
        diffs = np.abs(masses[:, None] - masses[None, :])
        lam_matrix = diffs / SIGMA
        iu = np.triu_indices(n_p, k=1)
        lambdas = lam_matrix[iu]
        # Check each lambda against all masses
        hits = 0
        for lam in lambdas:
            if lam <= 0:
                continue
            rel_err = np.abs(masses - lam) / lam
            # Exclude self (approximate: just check if any mass matches)
            if np.any(rel_err < tol):
                hits += 1
                if hits >= n_real_matches:
                    break
        if hits >= n_real_matches:
            count_ge += 1

    return count_ge / n_trials


def mc_validate_lensmaker(n_real_matches, n_trials=10000, seed=42):
    """MC validation for lensmaker matches — early exit."""
    rng = np.random.default_rng(seed)
    log_m = np.log10(MASS_ARRAY[MASS_ARRAY > 0])
    lo, hi = log_m.min() - 0.3, log_m.max() + 0.3
    n_p = len(MASS_LIST)
    tol = 0.03

    n_values = [2, 3, 4, 5, 7, 8, 9, 10, 12, 15, 20]
    prefactors = np.array([R(nv) - 1 for nv in n_values if R(nv) != 1])

    count_ge = 0
    for _ in range(n_trials):
        masses = np.sort(10 ** rng.uniform(lo, hi, size=n_p))
        inv_m = 1.0 / masses
        hits = 0
        found = False
        for pf in prefactors:
            if found:
                break
            # Pairwise curvature diffs
            cdiff = inv_m[:, None] - inv_m[None, :]
            iu = np.triu_indices(n_p, k=1)
            cvals = cdiff[iu]
            # Both signs
            for sign in [1, -1]:
                inv_f_arr = pf * sign * cvals
                valid = inv_f_arr > 0
                f_arr = np.zeros_like(inv_f_arr)
                f_arr[valid] = 1.0 / inv_f_arr[valid]
                for fv in f_arr[valid]:
                    rel = np.abs(masses - fv) / fv
                    if np.any(rel < tol):
                        hits += 1
                        if hits >= n_real_matches:
                            found = True
                            break
                if found:
                    break
        if hits >= n_real_matches:
            count_ge += 1

    return count_ge / n_trials


def mc_validate_chromatic(n_achromatic_pairs, ratio_tol=0.05, n_trials=10000, seed=42):
    """MC validation for achromatic pairs (ratio ~ 6) — vectorized."""
    rng = np.random.default_rng(seed)
    log_m = np.log10(MASS_ARRAY[MASS_ARRAY > 0])
    lo, hi = log_m.min() - 0.3, log_m.max() + 0.3
    n_p = len(MASS_LIST)

    count_ge = 0
    for _ in range(n_trials):
        masses = np.sort(10 ** rng.uniform(lo, hi, size=n_p))
        # Ratio matrix
        ratios = masses[None, :] / masses[:, None]
        iu = np.triu_indices(n_p, k=1)
        ratio_vals = ratios[iu]
        hits = np.sum(np.abs(ratio_vals - 6) / 6 < ratio_tol)
        if hits >= n_achromatic_pairs:
            count_ge += 1

    return count_ge / n_trials


# =====================================================================
# COMPREHENSIVE REPORT
# =====================================================================

def run_analysis():
    """Run all optical model analyses and print comprehensive report."""
    width = 78
    sep = '=' * width
    findings = []  # (significance_score, description)

    print(f'\n{sep}')
    print('  OPTICAL MODEL ANALYSIS — TECS-L Lens Analogies Applied to Particle Masses')
    print(sep)

    # --- R-spectrum reference ---
    print('\n--- R-Spectrum Reference (Refractive Indices) ---\n')
    print(f'  {"n":>4s}  {"R(n)":>10s}  {"Note"}')
    print(f'  {"----":>4s}  {"----------":>10s}  {"----"}')
    for n_val in range(2, 16):
        r_val = R(n_val)
        note = ''
        if n_val == 6:
            note = '<-- ACHROMATIC FIXED POINT (R=1)'
        elif abs(r_val - 1) < 0.1:
            note = '<-- near-achromatic'
        print(f'  {n_val:>4d}  {r_val:>10.6f}  {note}')

    # ─────────────────────────────────────────────────────────────────
    # 1. THIN LENS
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  1. THIN LENS EQUATION: 1/f = 1/m1 + 1/m2')
    print(sep)

    tl_results = thin_lens_scan(tolerance=0.03)
    for f_name, matches in tl_results.items():
        f_val = FOCAL_LENGTHS[f_name]
        print(f'\n  Focal length {f_name}  (lens power 1/f = {1/f_val:.0f})')
        if not matches:
            print('    No matches within 3% tolerance.')
        else:
            print(f'    {len(matches)} match(es):')
            print(f'    {"Particle 1":<18s} {"Particle 2":<18s} {"1/m1+1/m2":>12s} {"Target":>8s} {"Err%":>6s}')
            print(f'    {"-"*18} {"-"*18} {"-"*12} {"-"*8} {"-"*6}')
            for m in matches[:20]:
                print(f'    {m["p1"]:<18s} {m["p2"]:<18s} {m["inv_sum"]:>12.4f} {m["target"]:>8.1f} {m["error_pct"]:>6.2f}')
            if len(matches) > 0:
                findings.append((len(matches), f'Thin lens f={f_name}: {len(matches)} pairs'))

    # ─────────────────────────────────────────────────────────────────
    # 2. LENSMAKER'S EQUATION
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  2. LENSMAKER EQUATION: 1/f = (R(n)-1)(1/m1 - 1/m2)')
    print(sep)

    lm_results = lensmaker_scan(tolerance=0.03)
    if not lm_results:
        print('\n  No matches within 3% tolerance.')
    else:
        print(f'\n  {len(lm_results)} matches found (showing top 40):')
        print(f'  {"n":>3s} {"R(n)":>7s} {"P1":<14s} {"P2":<14s} '
              f'{"f_comp":>9s} {"Matched":<14s} {"Mass":>9s} {"Err%":>6s}')
        print(f'  {"---":>3s} {"-------":>7s} {"-"*14} {"-"*14} '
              f'{"-"*9} {"-"*14} {"-"*9} {"-"*6}')
        for r in lm_results[:40]:
            print(f'  {r["n"]:>3d} {r["R_n"]:>7.4f} {r["p1"]:<14s} {r["p2"]:<14s} '
                  f'{r["f_computed"]:>9.4f} {r["matched_particle"]:<14s} '
                  f'{r["matched_mass"]:>9.4f} {r["error_pct"]:>6.2f}')
        findings.append((len(lm_results), f'Lensmaker: {len(lm_results)} triplet matches'))

    # ─────────────────────────────────────────────────────────────────
    # 3. TELESCOPE RESOLVING POWER
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  3. TELESCOPE RESOLVING POWER')
    print(sep)

    tr = telescope_resolving_power()
    print()
    for key, val in tr.items():
        print(f'  {key:<40s} = {val:.8f}')

    # ─────────────────────────────────────────────────────────────────
    # 4. INTERFERENCE / DIFFRACTION
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  4. INTERFERENCE: (m2-m1)/sigma(6) = known mass?')
    print(sep)

    int_results = interference_scan(tolerance=0.03)
    if not int_results:
        print('\n  No matches within 3% tolerance.')
    else:
        print(f'\n  {len(int_results)} matches found (showing top 40):')
        print(f'  {"P1":<14s} {"P2":<14s} {"Diff":>9s} {"lambda":>9s} '
              f'{"Matched":<14s} {"Mass":>9s} {"Err%":>6s}')
        print(f'  {"-"*14} {"-"*14} {"-"*9} {"-"*9} {"-"*14} {"-"*9} {"-"*6}')
        for r in int_results[:40]:
            print(f'  {r["p1"]:<14s} {r["p2"]:<14s} {r["diff"]:>9.4f} {r["lambda"]:>9.4f} '
                  f'{r["matched_particle"]:<14s} {r["matched_mass"]:>9.4f} {r["error_pct"]:>6.2f}')
        findings.append((len(int_results), f'Interference d/sigma: {len(int_results)} matches'))

    # ─────────────────────────────────────────────────────────────────
    # 5. SNELL'S LAW
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  5. SNELL\'S LAW: R(n1)*sin(theta_in) = R(n2)*sin(theta_out)')
    print(sep)

    snell = snell_law_analysis()
    significant_snell = [s for s in snell if s.get('significance')]
    print(f'\n  {len(snell)} angle computations, {len(significant_snell)} significant:')
    if significant_snell:
        print(f'  {"n1->n2":>8s} {"R1":>7s} {"R2":>7s} {"theta_in":>10s} '
              f'{"sin_out":>8s} {"theta_out":>10s} {"Significance"}')
        print(f'  {"-"*8} {"-"*7} {"-"*7} {"-"*10} {"-"*8} {"-"*10} {"-"*20}')
        for s in significant_snell:
            tout = f'{s["theta_out_deg"]:.2f} deg' if s.get('theta_out_deg') is not None else 'TIR'
            print(f'  {s["n1"]:>3d}->{s["n2"]:<3d} {s["R1"]:>7.4f} {s["R2"]:>7.4f} '
                  f'{s["theta_in"]:>10s} {s["sin_out"]:>8.4f} {tout:>10s} {s["significance"]}')
        findings.append((len(significant_snell), f'Snell: {len(significant_snell)} significant angles'))

    # Full table
    print(f'\n  Full Snell\'s law table:')
    print(f'  {"n1->n2":>8s} {"R1":>7s} {"R2":>7s} {"theta_in":>10s} '
          f'{"sin_out":>8s} {"theta_out":>10s}')
    print(f'  {"-"*8} {"-"*7} {"-"*7} {"-"*10} {"-"*8} {"-"*10}')
    for s in snell:
        if s.get('theta_out_deg') is not None:
            tout = f'{s["theta_out_deg"]:.2f} deg'
        else:
            tout = 'TIR'
        print(f'  {s["n1"]:>3d}->{s["n2"]:<3d} {s["R1"]:>7.4f} {s["R2"]:>7.4f} '
              f'{s["theta_in"]:>10s} {s["sin_out"]:>8.4f} {tout:>10s}')

    # ─────────────────────────────────────────────────────────────────
    # 6. OPTICAL PATH LENGTH
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  6. OPTICAL PATH LENGTH: OPL = R(n) * mass')
    print(sep)

    opl = optical_path_length()

    print('\n  Resonance ladder OPL sums:')
    print(f'  {"R-index":<25s} {"Total OPL":>10s} {"Nearest":<14s} {"Mass":>9s} {"Err%":>6s}')
    print(f'  {"-"*25} {"-"*10} {"-"*14} {"-"*9} {"-"*6}')
    for label, info in opl['ladder_opls'].items():
        pname, pmass, perr = info['nearest']
        print(f'  {label:<25s} {info["total_OPL"]:>10.4f} {pname:<14s} {pmass:>9.4f} {perr*100:>6.2f}')

    if opl['ratio_opl_matches']:
        print(f'\n  R(ratio) * mass = known particle ({len(opl["ratio_opl_matches"])} matches, top 25):')
        print(f'  {"Heavy":<14s} {"Light":<14s} {"Ratio":>6s} {"n":>3s} {"R(n)":>7s} '
              f'{"OPL":>9s} {"Matched":<14s} {"Mass":>9s} {"Err%":>6s}')
        print(f'  {"-"*14} {"-"*14} {"-"*6} {"-"*3} {"-"*7} '
              f'{"-"*9} {"-"*14} {"-"*9} {"-"*6}')
        for r in opl['ratio_opl_matches'][:25]:
            print(f'  {r["p1"]:<14s} {r["p2"]:<14s} {r["mass_ratio"]:>6.2f} {r["n_approx"]:>3d} '
                  f'{r["R_n"]:>7.4f} {r["OPL"]:>9.4f} {r["matched"]:<14s} '
                  f'{r["matched_mass"]:>9.4f} {r["error_pct"]:>6.2f}')
        findings.append((len(opl['ratio_opl_matches']),
                        f'OPL: {len(opl["ratio_opl_matches"])} matches'))

    # ─────────────────────────────────────────────────────────────────
    # 7. CHROMATIC ABERRATION
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  7. CHROMATIC ABERRATION: R(6)=1 is achromatic')
    print(sep)

    chrom_results, disp_info = chromatic_aberration()

    print('\n  R-spectrum dispersion near n=6:')
    for k, v in disp_info.items():
        print(f'    {k}: {v:.6f}')

    # Filter to low-aberration pairs (aberration < 0.1) with good ratio match
    low_aberr = [r for r in chrom_results if r['aberration'] < 0.15
                 and r['ratio_error_pct'] < 5]
    achromatic_exact = [r for r in chrom_results if r['n_approx'] == 6
                        and r['ratio_error_pct'] < 5]

    print(f'\n  Achromatic pairs (ratio ~ 6, R(6)=1, aberration=0):')
    if achromatic_exact:
        print(f'  {"Heavy":<14s} {"Light":<14s} {"Ratio":>8s} {"Err%":>6s}')
        print(f'  {"-"*14} {"-"*14} {"-"*8} {"-"*6}')
        for r in achromatic_exact:
            print(f'  {r["p_heavy"]:<14s} {r["p_light"]:<14s} '
                  f'{r["ratio"]:>8.4f} {r["ratio_error_pct"]:>6.2f}')
        findings.append((len(achromatic_exact),
                        f'Achromatic pairs (ratio~6): {len(achromatic_exact)}'))

    if low_aberr:
        print(f'\n  Low-aberration pairs (|R(n)-1| < 0.15, ratio error < 5%):')
        print(f'  {"Heavy":<14s} {"Light":<14s} {"Ratio":>8s} {"n":>3s} '
              f'{"R(n)":>7s} {"Aberr":>7s}')
        print(f'  {"-"*14} {"-"*14} {"-"*8} {"-"*3} {"-"*7} {"-"*7}')
        for r in low_aberr[:20]:
            print(f'  {r["p_heavy"]:<14s} {r["p_light"]:<14s} {r["ratio"]:>8.4f} '
                  f'{r["n_approx"]:>3d} {r["R_n"]:>7.4f} {r["aberration"]:>7.4f}')

    # ─────────────────────────────────────────────────────────────────
    # 8. MONTE CARLO VALIDATION
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  8. MONTE CARLO VALIDATION (10,000 trials each)')
    print(sep)

    mc_results = {}

    # Thin lens MC (for the focal length with the most matches)
    best_f_name = max(tl_results.keys(), key=lambda k: len(tl_results[k])) if any(tl_results.values()) else None
    if best_f_name and tl_results[best_f_name]:
        n_matches = len(tl_results[best_f_name])
        f_val = dict(FOCAL_LENGTHS.items())[best_f_name]
        print(f'\n  Thin lens (f={best_f_name}, {n_matches} real matches)...')
        p_tl = mc_validate_thin_lens(tl_results[best_f_name], f_val, n_trials=10000)
        sig_tl = _p_to_sigma(p_tl) if p_tl > 0 else float('inf')
        mc_results['thin_lens'] = {'p_value': p_tl, 'sigma': sig_tl, 'n_matches': n_matches}
        print(f'    p-value: {p_tl:.4f}  ({sig_tl:.1f} sigma)')

    # Interference MC
    if int_results:
        n_int = len(int_results)
        # Use fewer trials if match count is high (each trial is O(n^2) work)
        mc_trials_int = 10000 if n_int < 50 else 2000
        print(f'\n  Interference ({n_int} real matches, {mc_trials_int} MC trials)...')
        p_int = mc_validate_interference(n_int, n_trials=mc_trials_int)
        sig_int = _p_to_sigma(p_int) if p_int > 0 else float('inf')
        mc_results['interference'] = {'p_value': p_int, 'sigma': sig_int, 'n_matches': n_int}
        print(f'    p-value: {p_int:.4f}  ({sig_int:.1f} sigma)')

    # Lensmaker MC — skip if too many matches (combinatorially expected)
    if lm_results and len(lm_results) < 500:
        n_lm = len(lm_results)
        print(f'\n  Lensmaker ({n_lm} real matches)...')
        p_lm = mc_validate_lensmaker(n_lm, n_trials=10000)
        sig_lm = _p_to_sigma(p_lm) if p_lm > 0 else float('inf')
        mc_results['lensmaker'] = {'p_value': p_lm, 'sigma': sig_lm, 'n_matches': n_lm}
        print(f'    p-value: {p_lm:.4f}  ({sig_lm:.1f} sigma)')
    elif lm_results:
        n_lm = len(lm_results)
        print(f'\n  Lensmaker ({n_lm} real matches) — too many for MC (combinatorially expected)')
        mc_results['lensmaker'] = {'p_value': 1.0, 'sigma': 0.0, 'n_matches': n_lm}

    # Chromatic (achromatic pairs ratio~6) MC
    if achromatic_exact:
        n_ach = len(achromatic_exact)
        print(f'\n  Achromatic pairs ratio~6 ({n_ach} real matches)...')
        p_ch = mc_validate_chromatic(n_ach, n_trials=10000)
        sig_ch = _p_to_sigma(p_ch) if p_ch > 0 else float('inf')
        mc_results['chromatic'] = {'p_value': p_ch, 'sigma': sig_ch, 'n_matches': n_ach}
        print(f'    p-value: {p_ch:.4f}  ({sig_ch:.1f} sigma)')

    # ─────────────────────────────────────────────────────────────────
    # SUMMARY
    # ─────────────────────────────────────────────────────────────────
    print(f'\n{sep}')
    print('  SUMMARY — Findings Ranked by Count')
    print(sep)

    findings.sort(key=lambda x: -x[0])
    print()
    for i, (count, desc) in enumerate(findings, 1):
        print(f'  {i}. [{count:>4d} matches] {desc}')

    print(f'\n  Monte Carlo p-values:')
    for name, mc in mc_results.items():
        p_str = f'{mc["p_value"]:.4f}' if mc['p_value'] > 0 else '< 0.0001'
        sig_str = f'{mc["sigma"]:.1f}' if mc['sigma'] != float('inf') else '> 4.3'
        print(f'    {name:<20s}: p = {p_str}  ({sig_str} sigma)  [{mc["n_matches"]} matches]')

    print(f'\n  Key optical identities from TECS-L n=6:')
    print(f'    f = 1/(sigma*phi) = 1/24           focal length')
    print(f'    1/f = sigma*phi = n*tau = 24       lens power')
    print(f'    R(6) = 1                            achromatic fixed point')
    print(f'    delta+ = 1/n = 1/6                 R-spectrum gap')
    print(f'    delta- = 1/tau = 1/4               R-spectrum gap')
    print(f'    ln(4/3) = {math.log(4/3):.4f}                  Golden Zone bandwidth')
    print(f'    sigma*phi*f = 1                     focal identity')
    print()
    print(sep)

    return {
        'thin_lens': tl_results,
        'lensmaker': lm_results,
        'telescope': telescope_resolving_power(),
        'interference': int_results,
        'snell': snell,
        'opl': optical_path_length(),
        'chromatic': (chrom_results, disp_info),
        'mc': mc_results,
        'findings': findings,
    }


if __name__ == '__main__':
    run_analysis()
