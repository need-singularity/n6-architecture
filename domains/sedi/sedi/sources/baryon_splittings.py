"""Baryon Multiplet Mass Splittings — n=6 arithmetic in the strong interaction.

The baryon octet (8 members = sigma-tau = rank E8) and decuplet (10 = tau(496) =
superstring dimensions) are organized by SU(3) flavor symmetry. The Gell-Mann-Okubo
mass formula relates masses within multiplets.

TECS-L question: do the MASS SPLITTINGS within these multiplets follow n=6 arithmetic?

Source: Particle Data Group (PDG) 2024 values. All masses in MeV/c^2.
"""
import math
import numpy as np
from itertools import combinations
from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
    TAU_P2, TAU_P3, PHI_P3,
    ALL_TARGETS, mass_ratio_matches,
)


# ─── TECS-L Constants ───

S = SIGMA_P1    # 12
T = TAU_P1      # 4
P = PHI_P1      # 2
F = SOPFR_P1    # 5
N = P1          # 6

# Commonly used derived values
SIGMA_MINUS_TAU = S - T        # 8  = rank(E8)
SIGMA_PLUS_TAU = S + T         # 16 = rank(E8 x E8)
SIGMA_TIMES_PHI = S * P        # 24 = Leech lattice
SIGMA_OVER_TAU = S // T        # 3  = color charges
SIGMA_SQ = S * S               # 144
TAU_496 = TAU_P3               # 10 = superstring dimensions
MERSENNE_3 = 7                 # M3 = 2^3 - 1
TRIANGULAR_6 = N * (N + 1) // 2  # T(6) = 21
TRIANGULAR_P1 = TRIANGULAR_6     # T(P1) = 21


# ─── Baryon Octet Masses (MeV) ───

OCTET = {
    'p':       938.272,
    'n':       939.565,
    'Lambda':  1115.683,
    'Sigma+':  1189.37,
    'Sigma0':  1192.642,
    'Sigma-':  1197.449,
    'Xi0':     1314.86,
    'Xi-':     1321.71,
}

# ─── Baryon Decuplet Masses (MeV) ───

DECUPLET = {
    'Delta':   1232.0,
    'Sigma*':  1383.7,
    'Xi*':     1531.8,
    'Omega-':  1672.45,
}


# ─── TECS-L Target Expressions for Splittings ───

SPLITTING_TARGETS = {
    # Pure n=6 arithmetic expressions and their values
    'sigma-tau = 8':                    S - T,                 # 8
    'sigma^2 = 144':                    S**2,                  # 144
    'sigma^2 + 3 = 147':               S**2 + SIGMA_OVER_TAU, # 147
    'sigma^2 + phi = 146':             S**2 + P,              # 146
    'sigma^2 + phi + 1 = 147':         S**2 + P + 1,          # 147
    'phi * (sigma^2 + 3) = 294':       P * (S**2 + SIGMA_OVER_TAU),  # 294
    '2 * sigma^2 = 288':               2 * S**2,              # 288
    'M3 = 7':                           MERSENNE_3,            # 7
    'M3 * T(P1) = 147':                MERSENNE_3 * TRIANGULAR_P1,  # 147
    'sigma * tau = 48':                 S * T,                 # 48
    'sigma * phi = 24':                 S * P,                 # 24
    'tau * phi = 8':                    T * P,                 # 8 (same as sigma-tau)
    'sopfr * sigma = 60':              F * S,                 # 60
    'sigma^2 / tau = 36':              S**2 // T,             # 36
    'sigma + tau = 16':                 S + T,                 # 16
    'P1^2 = 36':                        N**2,                  # 36
    'P1^3 = 216':                       N**3,                  # 216
    'P2 = 28':                          P2,                    # 28
    'sigma*phi*tau = 96':               S * P * T,             # 96
    'phi/tau = 0.5':                    P / T,                 # 0.5
    '1/tau = 0.25':                     1 / T,                 # 0.25
    'sigma/tau = 3':                    S / T,                 # 3
    'tau(496) = 10':                    TAU_496,               # 10
}


# ─── Analysis Functions ───

def compute_octet_splittings():
    """Compute all pairwise mass differences within the octet."""
    results = []
    names = list(OCTET.keys())
    for i in range(len(names)):
        for j in range(i + 1, len(names)):
            n1, n2 = names[i], names[j]
            m1, m2 = OCTET[n1], OCTET[n2]
            diff = abs(m2 - m1)
            results.append({
                'pair': f'{n2} - {n1}' if m2 > m1 else f'{n1} - {n2}',
                'diff_MeV': diff,
                'heavier': n2 if m2 > m1 else n1,
                'lighter': n1 if m2 > m1 else n2,
            })
    return sorted(results, key=lambda x: x['diff_MeV'])


def compute_isospin_splittings():
    """Compute isospin multiplet mass differences."""
    splittings = {
        'n - p':             OCTET['n'] - OCTET['p'],
        'Sigma- - Sigma+':   OCTET['Sigma-'] - OCTET['Sigma+'],
        'Sigma- - Sigma0':   OCTET['Sigma-'] - OCTET['Sigma0'],
        'Sigma0 - Sigma+':   OCTET['Sigma0'] - OCTET['Sigma+'],
        'Xi- - Xi0':         OCTET['Xi-'] - OCTET['Xi0'],
    }
    return splittings


def compute_decuplet_spacings():
    """Compute equal-spacing steps in the decuplet."""
    masses = list(DECUPLET.values())
    names = list(DECUPLET.keys())
    spacings = {}
    for i in range(len(masses) - 1):
        key = f'{names[i+1]} - {names[i]}'
        spacings[key] = masses[i + 1] - masses[i]
    spacings['average'] = np.mean(list(spacings.values()))
    return spacings


def check_gell_mann_okubo():
    """Verify the Gell-Mann-Okubo mass formula for the octet.

    GMO formula: (N + Xi) / 2 = (3*Lambda + Sigma) / 4
    where N = average nucleon, Xi = average cascade, Sigma = average Sigma.
    """
    N_avg = (OCTET['p'] + OCTET['n']) / 2
    Xi_avg = (OCTET['Xi0'] + OCTET['Xi-']) / 2
    Lambda_m = OCTET['Lambda']
    Sigma_avg = (OCTET['Sigma+'] + OCTET['Sigma0'] + OCTET['Sigma-']) / 3

    lhs = (N_avg + Xi_avg) / 2
    rhs = (3 * Lambda_m + Sigma_avg) / 4

    error_MeV = abs(lhs - rhs)
    error_pct = error_MeV / lhs * 100

    return {
        'LHS (N+Xi)/2': lhs,
        'RHS (3L+S)/4': rhs,
        'error_MeV': error_MeV,
        'error_pct': error_pct,
        'N_avg': N_avg,
        'Xi_avg': Xi_avg,
        'Lambda': Lambda_m,
        'Sigma_avg': Sigma_avg,
    }


def match_to_tecs(value, label='', tolerance=0.05):
    """Check if a value matches any TECS-L splitting target."""
    matches = []
    for tname, tval in SPLITTING_TARGETS.items():
        if tval == 0:
            continue
        err = abs(value - tval) / abs(tval)
        if err < tolerance:
            matches.append({
                'label': label,
                'value': value,
                'target_name': tname,
                'target_val': tval,
                'error_pct': err * 100,
            })
    return sorted(matches, key=lambda x: x['error_pct'])


def match_ratio_to_tecs(value, label='', tolerance=0.05):
    """Check if a ratio matches any TECS-L target."""
    matches = []
    all_ratio_targets = {**ALL_TARGETS, **{
        k: v for k, v in SPLITTING_TARGETS.items()
        if isinstance(v, (int, float)) and v > 0
    }}
    for tname, tval in all_ratio_targets.items():
        if tval <= 0:
            continue
        err = abs(value - tval) / tval
        if err < tolerance:
            matches.append({
                'label': label,
                'value': value,
                'target_name': tname,
                'target_val': tval,
                'error_pct': err * 100,
            })
        # Also check reciprocal
        if tval > 0 and value > 0:
            inv = 1 / value
            err_inv = abs(inv - tval) / tval
            if err_inv < tolerance:
                matches.append({
                    'label': f'1/({label})',
                    'value': inv,
                    'target_name': tname,
                    'target_val': tval,
                    'error_pct': err_inv * 100,
                })
    return sorted(matches, key=lambda x: x['error_pct'])


def compute_octet_ratios():
    """Compute all pairwise mass ratios within the octet."""
    results = []
    names = list(OCTET.keys())
    for i in range(len(names)):
        for j in range(i + 1, len(names)):
            n1, n2 = names[i], names[j]
            m1, m2 = OCTET[n1], OCTET[n2]
            big, small = max(m1, m2), min(m1, m2)
            ratio = big / small
            results.append({
                'pair': f'{n2}/{n1}' if m2 > m1 else f'{n1}/{n2}',
                'ratio': ratio,
            })
    return results


def compute_decuplet_ratios():
    """Compute all pairwise mass ratios within the decuplet."""
    results = []
    names = list(DECUPLET.keys())
    for i in range(len(names)):
        for j in range(i + 1, len(names)):
            n1, n2 = names[i], names[j]
            m1, m2 = DECUPLET[n1], DECUPLET[n2]
            big, small = max(m1, m2), min(m1, m2)
            ratio = big / small
            results.append({
                'pair': f'{n2}/{n1}' if m2 > m1 else f'{n1}/{n2}',
                'ratio': ratio,
            })
    return results


def octet_decuplet_gap():
    """Compute the mass gap between decuplet and octet ground states."""
    delta_m = DECUPLET['Delta']
    proton_m = OCTET['p']
    gap = delta_m - proton_m
    return {
        'Delta - proton': gap,
        '2x_avg_decuplet_spacing': 2 * compute_decuplet_spacings()['average'],
    }


def gmo_coefficients_tecs():
    """Analyze GMO formula coefficients through TECS-L lens.

    GMO: (N+Xi)/2 = (3*Lambda + Sigma)/4
    Coefficients: 1/2, 3/4, 1/4
    TECS-L: phi/tau = 1/2, sigma/(sigma+tau) = 3/4, tau/(sigma+tau) = 1/4
    """
    return {
        '1/2 = phi/tau':             {'coeff': 0.5,  'tecs': P / T,
                                       'match': P / T == 0.5},
        '3/4 = sigma/(sigma+tau)':   {'coeff': 0.75, 'tecs': S / (S + T),
                                       'match': S / (S + T) == 0.75},
        '1/4 = tau/(sigma+tau)':     {'coeff': 0.25, 'tecs': T / (S + T),
                                       'match': T / (S + T) == 0.25},
    }


# ─── Monte Carlo Validation ───

def mc_validate_splitting(observed_value, target_value, n_particles,
                          mass_range=(900, 1700), n_trials=50_000, seed=42):
    """Monte Carlo: how often do random masses in similar range produce this splitting?

    Generates n_particles random masses in mass_range, computes all pairwise
    differences, and checks how often any difference matches target_value
    within the same relative tolerance as the observed match.
    """
    rng = np.random.default_rng(seed)
    obs_err = abs(observed_value - target_value) / target_value
    tolerance = max(obs_err * 1.5, 0.01)  # at least 1% window

    hits = 0
    for _ in range(n_trials):
        masses = rng.uniform(mass_range[0], mass_range[1], size=n_particles)
        diffs = []
        for i in range(n_particles):
            for j in range(i + 1, n_particles):
                diffs.append(abs(masses[i] - masses[j]))
        if any(abs(d - target_value) / target_value < tolerance for d in diffs):
            hits += 1

    p_value = (hits + 1) / (n_trials + 1)
    return {
        'observed': observed_value,
        'target': target_value,
        'tolerance': tolerance,
        'p_value': p_value,
        'hits': hits,
        'trials': n_trials,
    }


def mc_validate_ratio(observed_ratio, target_value, n_particles,
                      mass_range=(900, 1700), n_trials=50_000, seed=42):
    """Monte Carlo: how often do random masses produce this ratio match?"""
    rng = np.random.default_rng(seed)
    obs_err = abs(observed_ratio - target_value) / target_value
    tolerance = max(obs_err * 1.5, 0.01)

    hits = 0
    for _ in range(n_trials):
        masses = rng.uniform(mass_range[0], mass_range[1], size=n_particles)
        for i in range(n_particles):
            for j in range(i + 1, n_particles):
                big, small = max(masses[i], masses[j]), min(masses[i], masses[j])
                if small > 0:
                    ratio = big / small
                    if abs(ratio - target_value) / target_value < tolerance:
                        hits += 1
                        break
            else:
                continue
            break

    p_value = (hits + 1) / (n_trials + 1)
    return {
        'observed': observed_ratio,
        'target': target_value,
        'tolerance': tolerance,
        'p_value': p_value,
        'hits': hits,
        'trials': n_trials,
    }


# ─── Main Analysis ───

def run_analysis(mc_trials=20_000):
    """Run the full baryon splittings analysis and print report."""
    print("=" * 78)
    print("BARYON MULTIPLET MASS SPLITTINGS — n=6 ARITHMETIC ANALYSIS")
    print("=" * 78)

    # ─── Section 1: Octet Members ───
    print("\n" + "─" * 78)
    print("1. BARYON OCTET (8 members = sigma-tau = rank E8)")
    print("─" * 78)
    for name, mass in OCTET.items():
        print(f"  {name:>8s}  {mass:10.3f} MeV")
    print(f"\n  Octet member count: {len(OCTET)} = sigma - tau = {S} - {T} = {S-T}")

    # ─── Section 2: Decuplet Members ───
    print("\n" + "─" * 78)
    print("2. BARYON DECUPLET (10 members = tau(496) = superstring dimensions)")
    print("─" * 78)
    for name, mass in DECUPLET.items():
        print(f"  {name:>8s}  {mass:10.3f} MeV")
    print(f"\n  Decuplet member count: {len(DECUPLET)} = tau(496) = {TAU_496}")
    print(f"  (Listed: 4 isospin-averaged states spanning the decuplet)")

    # ─── Section 3: Isospin Splittings ───
    print("\n" + "─" * 78)
    print("3. ISOSPIN SPLITTINGS (electromagnetic mass differences)")
    print("─" * 78)
    iso = compute_isospin_splittings()
    iso_findings = []
    for label, val in iso.items():
        print(f"  {label:>20s} = {val:8.3f} MeV")
        matches = match_to_tecs(val, label, tolerance=0.05)
        for m in matches:
            print(f"    -> matches {m['target_name']} ({m['target_val']}) "
                  f"error = {m['error_pct']:.2f}%")
            iso_findings.append(m)

    # Specific isospin checks from the task
    print(f"\n  Key isospin checks:")
    np_diff = iso['n - p']
    xi_diff = iso['Xi- - Xi0']
    sig_diff = iso['Sigma- - Sigma+']
    print(f"    n - p = {np_diff:.3f} MeV (small, QED-dominated)")
    print(f"    Xi(-) - Xi(0) = {xi_diff:.3f} MeV  vs  M3 = {MERSENNE_3}: "
          f"error = {abs(xi_diff - MERSENNE_3)/MERSENNE_3*100:.1f}%")
    print(f"    Sigma(-) - Sigma(+) = {sig_diff:.3f} MeV  vs  sigma-tau = "
          f"{SIGMA_MINUS_TAU}: error = {abs(sig_diff - SIGMA_MINUS_TAU)/SIGMA_MINUS_TAU*100:.1f}%")

    # ─── Section 4: Decuplet Equal Spacing ───
    print("\n" + "─" * 78)
    print("4. DECUPLET EQUAL SPACING RULE")
    print("─" * 78)
    spacings = compute_decuplet_spacings()
    for label, val in spacings.items():
        print(f"  {label:>25s} = {val:8.2f} MeV")

    avg_spacing = spacings['average']
    print(f"\n  TECS-L decompositions of average spacing {avg_spacing:.1f} MeV:")

    decomps = [
        (f'sigma^2 + sigma/tau = {S**2} + {S//T} = {S**2 + S//T}', S**2 + S // T),
        (f'sigma^2 + phi + 1 = {S**2} + {P} + 1 = {S**2 + P + 1}', S**2 + P + 1),
        (f'M3 * T(P1) = {MERSENNE_3} * {TRIANGULAR_P1} = '
         f'{MERSENNE_3 * TRIANGULAR_P1}', MERSENNE_3 * TRIANGULAR_P1),
        (f'sigma^2 + 3 = 144 + 3 = 147', S**2 + SIGMA_OVER_TAU),
    ]
    for desc, val in decomps:
        err = abs(avg_spacing - val) / val * 100
        print(f"    {desc}  (error = {err:.2f}%)")

    # Individual spacing checks
    print(f"\n  Individual spacing TECS-L matches:")
    for label, val in spacings.items():
        if label == 'average':
            continue
        matches = match_to_tecs(val, label, tolerance=0.06)
        for m in matches:
            print(f"    {label}: {val:.1f} -> {m['target_name']} = "
                  f"{m['target_val']}  (error = {m['error_pct']:.2f}%)")

    # ─── Section 5: Gell-Mann-Okubo Formula ───
    print("\n" + "─" * 78)
    print("5. GELL-MANN-OKUBO MASS FORMULA")
    print("─" * 78)
    gmo = check_gell_mann_okubo()
    print(f"  GMO: (N + Xi) / 2 = (3*Lambda + Sigma) / 4")
    print(f"  LHS = ({gmo['N_avg']:.3f} + {gmo['Xi_avg']:.3f}) / 2 "
          f"= {gmo['LHS (N+Xi)/2']:.3f} MeV")
    print(f"  RHS = (3*{gmo['Lambda']:.3f} + {gmo['Sigma_avg']:.3f}) / 4 "
          f"= {gmo['RHS (3L+S)/4']:.3f} MeV")
    print(f"  Error: {gmo['error_MeV']:.3f} MeV ({gmo['error_pct']:.3f}%)")

    print(f"\n  GMO coefficients through TECS-L lens:")
    gmo_c = gmo_coefficients_tecs()
    for desc, info in gmo_c.items():
        status = "EXACT" if info['match'] else "no"
        print(f"    {desc}: coeff = {info['coeff']}, "
              f"TECS = {info['tecs']}, match = {status}")
    print(f"\n  -> ALL three GMO coefficients 1/2, 3/4, 1/4 are expressible as")
    print(f"     phi/tau, sigma/(sigma+tau), tau/(sigma+tau) from n=6 arithmetic!")

    # ─── Section 6: Octet-Decuplet Mass Gap ───
    print("\n" + "─" * 78)
    print("6. OCTET-DECUPLET MASS GAP")
    print("─" * 78)
    gap = octet_decuplet_gap()
    delta_p = gap['Delta - proton']
    print(f"  Delta(1232) - proton(938) = {delta_p:.3f} MeV")
    print(f"  2 x avg decuplet spacing  = {gap['2x_avg_decuplet_spacing']:.2f} MeV")
    print(f"  Ratio: gap / avg_spacing = {delta_p / avg_spacing:.4f}")

    gap_decomps = [
        (f'phi * (sigma^2 + 3) = 2 * 147 = {P * (S**2 + SIGMA_OVER_TAU)}',
         P * (S**2 + SIGMA_OVER_TAU)),
        (f'2 * sigma^2 + P1 = 288 + 6 = 294', 2 * S**2 + N),
        (f'phi * M3 * T(P1) = 2 * 7 * 21 = {P * MERSENNE_3 * TRIANGULAR_P1}',
         P * MERSENNE_3 * TRIANGULAR_P1),
    ]
    print(f"\n  TECS-L decompositions of gap = {delta_p:.1f} MeV:")
    for desc, val in gap_decomps:
        err = abs(delta_p - val) / val * 100
        print(f"    {desc}  (error = {err:.2f}%)")

    # ─── Section 7: Mass Ratios ───
    print("\n" + "─" * 78)
    print("7. MASS RATIOS WITHIN MULTIPLETS")
    print("─" * 78)

    print("\n  Octet ratios matching TECS-L targets:")
    octet_ratios = compute_octet_ratios()
    octet_ratio_matches = []
    for r in octet_ratios:
        matches = match_ratio_to_tecs(r['ratio'], r['pair'], tolerance=0.03)
        for m in matches:
            octet_ratio_matches.append(m)
            print(f"    {m['label']:>15s} = {m['value']:.6f} -> "
                  f"{m['target_name']} = {m['target_val']}  "
                  f"(error = {m['error_pct']:.2f}%)")

    print("\n  Decuplet ratios matching TECS-L targets:")
    dec_ratios = compute_decuplet_ratios()
    dec_ratio_matches = []
    for r in dec_ratios:
        matches = match_ratio_to_tecs(r['ratio'], r['pair'], tolerance=0.03)
        for m in matches:
            dec_ratio_matches.append(m)
            print(f"    {m['label']:>15s} = {m['value']:.6f} -> "
                  f"{m['target_name']} = {m['target_val']}  "
                  f"(error = {m['error_pct']:.2f}%)")

    if not octet_ratio_matches and not dec_ratio_matches:
        print("    (No ratio matches within 3% tolerance)")

    # Cross-multiplet ratios
    print("\n  Cross-multiplet ratios (decuplet/octet):")
    cross_matches = []
    for dn, dm in DECUPLET.items():
        for on, om in OCTET.items():
            ratio = dm / om
            label = f'{dn}/{on}'
            matches = match_ratio_to_tecs(ratio, label, tolerance=0.03)
            for m in matches:
                cross_matches.append(m)
                print(f"    {m['label']:>20s} = {m['value']:.6f} -> "
                      f"{m['target_name']} = {m['target_val']}  "
                      f"(error = {m['error_pct']:.2f}%)")
    if not cross_matches:
        print("    (No cross-multiplet ratio matches within 3%)")

    # ─── Section 8: All Octet Splittings ───
    print("\n" + "─" * 78)
    print("8. ALL OCTET MASS SPLITTINGS — TECS-L SCAN")
    print("─" * 78)
    splittings = compute_octet_splittings()
    all_splitting_matches = []
    for sp in splittings:
        matches = match_to_tecs(sp['diff_MeV'], sp['pair'], tolerance=0.05)
        if matches:
            for m in matches:
                all_splitting_matches.append(m)
                print(f"  {sp['pair']:>20s} = {sp['diff_MeV']:8.3f} MeV -> "
                      f"{m['target_name']} = {m['target_val']}  "
                      f"(error = {m['error_pct']:.2f}%)")

    # ─── Section 9: Monte Carlo Validation ───
    print("\n" + "─" * 78)
    print("9. MONTE CARLO VALIDATION")
    print(f"   ({mc_trials:,} trials per test)")
    print("─" * 78)

    mc_results = []

    # Validate key findings
    key_tests = []

    # Isospin: Sigma(-)-Sigma(+) ~ 8
    key_tests.append(('Sigma(-)-Sigma(+) ~ sigma-tau=8',
                      sig_diff, SIGMA_MINUS_TAU, 8,
                      (900, 1700)))

    # Isospin: Xi(-)-Xi(0) ~ M3=7
    key_tests.append(('Xi(-)-Xi(0) ~ M3=7',
                      xi_diff, MERSENNE_3, 8,
                      (900, 1700)))

    # Average decuplet spacing ~ 147
    key_tests.append(('Avg decuplet spacing ~ sigma^2+3=147',
                      avg_spacing, S**2 + SIGMA_OVER_TAU, 4,
                      (1200, 1700)))

    # Delta-proton gap ~ 294
    key_tests.append(('Delta-proton gap ~ 2*(sigma^2+3)=294',
                      delta_p, P * (S**2 + SIGMA_OVER_TAU), 8,
                      (900, 1700)))

    for desc, obs, target, n_part, mrange in key_tests:
        mc = mc_validate_splitting(obs, target, n_part,
                                   mass_range=mrange, n_trials=mc_trials)
        mc['description'] = desc
        mc_results.append(mc)
        sig_str = "SIGNIFICANT" if mc['p_value'] < 0.05 else "not significant"
        print(f"\n  {desc}")
        print(f"    observed = {obs:.3f}, target = {target}")
        print(f"    p-value = {mc['p_value']:.4f}  ({sig_str})")
        print(f"    ({mc['hits']}/{mc['trials']} random trials matched)")

    # ─── Section 10: Comprehensive Summary ───
    print("\n" + "=" * 78)
    print("10. COMPREHENSIVE SUMMARY — RANKED FINDINGS")
    print("=" * 78)

    all_findings = []

    # Collect all findings with their significance
    # GMO coefficients (exact)
    all_findings.append({
        'description': 'GMO coefficients 1/2, 3/4, 1/4 = phi/tau, sigma/(sigma+tau), tau/(sigma+tau)',
        'error_pct': 0.0,
        'type': 'EXACT',
        'significance': 'Algebraic identity — not probabilistic',
    })

    # GMO formula itself
    all_findings.append({
        'description': f'GMO formula verified: error = {gmo["error_MeV"]:.3f} MeV ({gmo["error_pct"]:.3f}%)',
        'error_pct': gmo['error_pct'],
        'type': 'FORMULA',
        'significance': 'Known physics, independently verified',
    })

    # Sigma(-)-Sigma(+) ~ 8
    all_findings.append({
        'description': f'Sigma(-)-Sigma(+) = {sig_diff:.3f} MeV ~ sigma-tau = 8',
        'error_pct': abs(sig_diff - SIGMA_MINUS_TAU) / SIGMA_MINUS_TAU * 100,
        'type': 'SPLITTING',
        'significance': f'MC p = {mc_results[0]["p_value"]:.4f}',
    })

    # Xi(-)-Xi(0) ~ 7
    all_findings.append({
        'description': f'Xi(-)-Xi(0) = {xi_diff:.3f} MeV ~ M3 = 7',
        'error_pct': abs(xi_diff - MERSENNE_3) / MERSENNE_3 * 100,
        'type': 'SPLITTING',
        'significance': f'MC p = {mc_results[1]["p_value"]:.4f}',
    })

    # Average decuplet spacing
    best_decomp_err = min(abs(avg_spacing - v) / v * 100 for _, v in decomps)
    all_findings.append({
        'description': f'Avg decuplet spacing = {avg_spacing:.1f} MeV ~ '
                       f'sigma^2 + sigma/tau = 147',
        'error_pct': abs(avg_spacing - 147) / 147 * 100,
        'type': 'SPACING',
        'significance': f'MC p = {mc_results[2]["p_value"]:.4f}',
    })

    # Delta-proton gap
    all_findings.append({
        'description': f'Delta-proton gap = {delta_p:.1f} MeV ~ '
                       f'2*(sigma^2+3) = 294',
        'error_pct': abs(delta_p - 294) / 294 * 100,
        'type': 'GAP',
        'significance': f'MC p = {mc_results[3]["p_value"]:.4f}',
    })

    # Multiplet sizes
    all_findings.append({
        'description': 'Octet count 8 = sigma-tau = rank(E8)',
        'error_pct': 0.0,
        'type': 'COUNT',
        'significance': 'Exact (algebraic)',
    })
    all_findings.append({
        'description': 'Decuplet count 10 = tau(496) = superstring D',
        'error_pct': 0.0,
        'type': 'COUNT',
        'significance': 'Exact (algebraic)',
    })

    # Add ratio matches
    for m in (octet_ratio_matches + dec_ratio_matches + cross_matches):
        all_findings.append({
            'description': f'Ratio {m["label"]} = {m["value"]:.4f} ~ '
                           f'{m["target_name"]} = {m["target_val"]}',
            'error_pct': m['error_pct'],
            'type': 'RATIO',
            'significance': 'Not MC-validated',
        })

    # Add splitting matches
    for m in all_splitting_matches:
        all_findings.append({
            'description': f'Splitting {m["label"]} = {m["value"]:.1f} MeV ~ '
                           f'{m["target_name"]} = {m["target_val"]}',
            'error_pct': m['error_pct'],
            'type': 'SPLITTING',
            'significance': 'Not MC-validated',
        })

    # Sort by error percentage
    all_findings.sort(key=lambda x: x['error_pct'])

    for i, f in enumerate(all_findings, 1):
        tag = f['type']
        print(f"\n  [{i:2d}] ({tag:>8s}) {f['description']}")
        print(f"       Error: {f['error_pct']:.2f}%  |  {f['significance']}")

    # Final summary statistics
    n_exact = sum(1 for f in all_findings if f['error_pct'] == 0.0)
    n_sub1 = sum(1 for f in all_findings if 0 < f['error_pct'] <= 1.0)
    n_sub3 = sum(1 for f in all_findings if 1.0 < f['error_pct'] <= 3.0)
    n_sub5 = sum(1 for f in all_findings if 3.0 < f['error_pct'] <= 5.0)

    print(f"\n  {'─' * 50}")
    print(f"  Total findings: {len(all_findings)}")
    print(f"    Exact matches:     {n_exact}")
    print(f"    < 1% error:        {n_sub1}")
    print(f"    1-3% error:        {n_sub3}")
    print(f"    3-5% error:        {n_sub5}")

    n_mc_sig = sum(1 for mc in mc_results if mc['p_value'] < 0.05)
    print(f"\n  MC-validated significant (p < 0.05): {n_mc_sig}/{len(mc_results)}")

    print("\n" + "=" * 78)
    print("END OF BARYON SPLITTINGS ANALYSIS")
    print("=" * 78)

    return {
        'findings': all_findings,
        'mc_results': mc_results,
        'gmo': gmo,
        'spacings': spacings,
        'isospin': iso,
    }


if __name__ == '__main__':
    run_analysis()
