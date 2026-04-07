"""PMNS Neutrino Mixing Matrix Analysis — n=6 arithmetic expressions.

Systematic search: can PMNS mixing parameters (sin^2 theta_ij, delta_CP,
mass-squared differences, Jarlskog invariant) be expressed using the n=6
arithmetic constants {sigma=12, tau=4, phi=2, sopfr=5, n=6, omega=2}?

Includes tribimaximal deviation analysis, CP phase decomposition,
and Texas Sharpshooter p-value test with Monte Carlo validation.
"""
import math
import random
from collections import defaultdict
from ..tecs import (
    SIGMA_P1 as S, TAU_P1 as T, PHI_P1 as P, SOPFR_P1 as SP,
    P1 as N, OMEGA_P1 as W,
    TAU_P2, TAU_P3, PHI_P3, P2, P3,
)

M3 = 7  # Mersenne prime 2^3 - 1


# ─── 1. PDG Neutrino Mixing Parameters ───

PMNS_PDG = {
    'sin2_theta12': (0.307, 0.013),   # solar angle
    'sin2_theta23': (0.545, 0.020),   # atmospheric angle
    'sin2_theta13': (0.0220, 0.0007), # reactor angle
    'delta_CP_deg': (197.0, 25.0),    # CP violation phase (degrees)
}

MASS_SQUARED = {
    'Dm2_21': (7.53e-5, 0.18e-5),     # solar (eV^2)
    'abs_Dm2_32': (2.453e-3, 0.034e-3),  # atmospheric (eV^2)
}

# Derived quantities
MASS_RATIO = MASS_SQUARED['abs_Dm2_32'][0] / MASS_SQUARED['Dm2_21'][0]  # ~32.6

# Tribimaximal reference values
TRIBIMAXIMAL = {
    'sin2_theta12': 1.0 / 3,
    'sin2_theta23': 1.0 / 2,
    'sin2_theta13': 0.0,
}


# ─── 2. Build Expression Dictionary ───

def build_expressions():
    """Generate candidate expressions from n=6 constants.

    Constants: sigma=12, tau=4, phi=2, sopfr=5, n=6, omega=2, M3=7
    """
    s, t, p, sp, n, w, m3 = S, T, P, SP, N, W, M3

    consts = {'sigma': s, 'tau': t, 'phi': p, 'sopfr': sp, 'n': n,
              'omega': w, 'M3': m3}
    exprs = {}

    # --- Layer 1: Single constants and reciprocals ---
    for name, val in consts.items():
        if val != 0:
            exprs[name] = float(val)
            exprs[f'1/{name}'] = 1.0 / val

    # --- Layer 2: Pairwise a/b, a*b, a+b, a-b ---
    cnames = list(consts.keys())
    cvals = list(consts.values())
    for i in range(len(cnames)):
        for j in range(len(cnames)):
            if i == j:
                continue
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            if b != 0:
                exprs[f'{na}/{nb}'] = a / b
            exprs[f'{na}*{nb}'] = float(a * b)
            if i < j:
                exprs[f'{na}+{nb}'] = float(a + b)
            if a - b > 0:
                exprs[f'{na}-{nb}'] = float(a - b)

    # --- Layer 3: a*b/c, a/(b*c), 1/(a*b), (a+b)/c, a/(b+c), a*b/(c+d) ---
    for i in range(len(cnames)):
        for j in range(i + 1, len(cnames)):
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            prod = a * b
            if prod != 0:
                exprs[f'1/({na}*{nb})'] = 1.0 / prod
            for k in range(len(cnames)):
                c, nc = cvals[k], cnames[k]
                if k == i or k == j:
                    continue
                if prod != 0:
                    exprs[f'{nc}/({na}*{nb})'] = c / prod
                if c != 0:
                    exprs[f'{na}*{nb}/{nc}'] = prod / c
                denom_sum = a + b
                if denom_sum != 0:
                    exprs[f'{nc}/({na}+{nb})'] = c / denom_sum
                if c != 0:
                    exprs[f'({na}+{nb})/{nc}'] = denom_sum / c
                if a + c != 0:
                    exprs[f'{nb}/({na}+{nc})'] = b / (a + c)
                if b + c != 0:
                    exprs[f'{na}/({nb}+{nc})'] = a / (b + c)
                # a*b/(c+d) for fourth constant
                for m in range(k + 1, len(cnames)):
                    if m == i or m == j:
                        continue
                    d, nd = cvals[m], cnames[m]
                    denom = c + d
                    if denom != 0:
                        exprs[f'{na}*{nb}/({nc}+{nd})'] = prod / denom
                    denom2 = c + d + a
                    if denom2 != 0:
                        exprs[f'{na}*{nb}/({na}+{nc}+{nd})'] = prod / denom2

    # --- Layer 4: (a*b+c)/d, (a-b)/c, a/(b*c-d), 1/(a*b-c) ---
    for i in range(len(cnames)):
        for j in range(len(cnames)):
            if i == j:
                continue
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            for k in range(len(cnames)):
                if k == i or k == j:
                    continue
                c, nc = cvals[k], cnames[k]
                # (a-b)/c
                if c != 0 and a - b > 0:
                    exprs[f'({na}-{nb})/{nc}'] = (a - b) / c
                # a/(b*c-d+1), a/(b*c+d)
                for m in range(len(cnames)):
                    if m in (i, j, k):
                        continue
                    d, nd = cvals[m], cnames[m]
                    denom = b * c - d
                    if denom > 0:
                        exprs[f'{na}/({nb}*{nc}-{nd})'] = a / denom
                        exprs[f'1/({nb}*{nc}-{nd})'] = 1.0 / denom
                    denom2 = b * c + d
                    if denom2 > 0:
                        exprs[f'1/({nb}*{nc}+{nd})'] = 1.0 / denom2
                    denom3 = b * c - d + 1
                    if denom3 > 0:
                        exprs[f'1/({nb}*{nc}-{nd}+1)'] = 1.0 / denom3
                    denom4 = b * c - d - 1
                    if denom4 > 0:
                        exprs[f'1/({nb}*{nc}-{nd}-1)'] = 1.0 / denom4

    # --- Layer 5: Powers ---
    for i in range(len(cnames)):
        for j in range(len(cnames)):
            if i == j:
                continue
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            if a > 0 and b <= 6:
                try:
                    v = a ** b
                    if math.isfinite(v) and v > 0:
                        exprs[f'{na}^{nb}'] = float(v)
                        if v != 0:
                            exprs[f'1/{na}^{nb}'] = 1.0 / v
                except (ValueError, OverflowError):
                    pass
            ratio = a / b if b != 0 else 0
            if ratio > 0:
                for pwr in [2, 3, -1, -2, 0.5]:
                    try:
                        v = ratio ** pwr
                        if math.isfinite(v) and v > 0:
                            exprs[f'({na}/{nb})^{pwr}'] = v
                    except (ValueError, OverflowError):
                        pass

    # --- Layer 6: Quadratic combos a^2 + b*c, a^2 - b + c, etc. ---
    for i in range(len(cnames)):
        a, na = cvals[i], cnames[i]
        for j in range(len(cnames)):
            if j == i:
                continue
            b, nb = cvals[j], cnames[j]
            for k in range(len(cnames)):
                if k == i or k == j:
                    continue
                c, nc = cvals[k], cnames[k]
                v1 = a**2 + b * c
                exprs[f'{na}^2+{nb}*{nc}'] = float(v1)
                v2 = a**2 - b * c
                if v2 > 0:
                    exprs[f'{na}^2-{nb}*{nc}'] = float(v2)
                v3 = a**2 + b * a - c
                exprs[f'{na}^2+{nb}*{na}-{nc}'] = float(v3)

    # --- Layer 7: Trigonometric (pi/n family) ---
    for name, val in consts.items():
        if val > 0:
            exprs[f'sin(pi/{name})'] = math.sin(math.pi / val)
            exprs[f'cos(pi/{name})'] = abs(math.cos(math.pi / val))
            exprs[f'sin(pi/{name})^2'] = math.sin(math.pi / val) ** 2
            exprs[f'cos(pi/{name})^2'] = math.cos(math.pi / val) ** 2
            for name2, val2 in consts.items():
                if name2 != name and val2 > 0:
                    arg = math.pi * val / val2
                    sv = abs(math.sin(arg))
                    if 0 < sv:
                        exprs[f'|sin(pi*{name}/{name2})|'] = sv
                    cv = abs(math.cos(arg))
                    if 0 < cv:
                        exprs[f'|cos(pi*{name}/{name2})|'] = cv

    # --- Layer 8: Logarithms ---
    for name, val in consts.items():
        if val > 1:
            exprs[f'ln({name})'] = math.log(val)
            exprs[f'1/ln({name})'] = 1.0 / math.log(val)
    for i in range(len(cnames)):
        for j in range(len(cnames)):
            if i == j:
                continue
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            ratio = a / b if b != 0 else 0
            if ratio > 0 and ratio != 1:
                exprs[f'ln({na}/{nb})'] = abs(math.log(ratio))

    # --- Layer 9: Compound with pi and e ---
    exprs['1/e'] = 1.0 / math.e
    exprs['1/pi'] = 1.0 / math.pi
    exprs['pi/sigma'] = math.pi / s
    exprs['pi/(sigma*phi)'] = math.pi / (s * p)
    exprs['sqrt(phi/sigma)'] = math.sqrt(p / s)
    exprs['sqrt(tau/sigma)'] = math.sqrt(t / s)
    exprs['sqrt(phi/(sigma*tau))'] = math.sqrt(p / (s * t))

    # --- Layer 10: P2/P3 motivated ---
    exprs['1/P2'] = 1.0 / P2
    exprs['1/P3'] = 1.0 / P3
    exprs['tau(P2)/sigma'] = TAU_P2 / s       # 6/12 = 0.5
    exprs['tau(P3)/sigma^2'] = TAU_P3 / s**2  # 10/144
    exprs['phi(P3)/sigma^3'] = PHI_P3 / s**3  # 240/1728

    # --- Layer 11: Neutrino-motivated compound expressions ---
    # Tribimaximal deviations
    exprs['tau/sigma'] = t / s                          # 1/3
    exprs['phi/tau'] = p / t                            # 1/2
    exprs['sopfr/(sigma+tau)'] = sp / (s + t)           # 5/16 = 0.3125
    exprs['tau*phi/(sigma+sigma+phi)'] = t * p / (s + s + p)  # 8/26
    exprs['n/(sigma-1)'] = n / (s - 1)                  # 6/11
    exprs['1/(sigma*tau-phi-1)'] = 1.0 / (s * t - p - 1)  # 1/45
    exprs['1/(sigma*tau-sopfr+1)'] = 1.0 / (s * t - sp + 1)  # 1/44
    exprs['sopfr/(sigma*phi+phi)'] = sp / (s * p + p)   # 5/26

    # CP phase: 197 = sigma^2 + sopfr*sigma - M3
    exprs['sigma^2+sopfr*sigma-M3'] = float(s**2 + sp * s - M3)  # 197

    # Mass ratio motivated
    exprs['phi^sopfr'] = float(p ** sp)                  # 32
    exprs['sigma^2/tau-sopfr+1'] = float(s**2 / t - sp + 1)  # 32

    # Filter to positive finite values
    result = {}
    for label, val in exprs.items():
        try:
            if val is not None and math.isfinite(val) and val > 0:
                result[label] = val
        except (TypeError, ValueError):
            pass

    return result


# ─── 3. Match Parameters to Expressions ───

def find_matches(targets, expressions, max_error_pct=5.0):
    """For each target, find all expressions within max_error_pct."""
    results = {}
    for tname, tval in targets.items():
        if isinstance(tval, tuple):
            tval = tval[0]  # central value
        matches = []
        for elabel, eval_ in expressions.items():
            if tval == 0:
                continue
            err = abs(eval_ - tval) / abs(tval) * 100
            if err <= max_error_pct:
                matches.append((elabel, eval_, err))
        matches.sort(key=lambda x: x[2])
        results[tname] = matches
    return results


# ─── 4. Motivated Guesses (explicit checks) ───

def check_motivated_guesses():
    """Check specific physically-motivated PMNS expressions."""
    s, t, p, sp, n, m3 = S, T, P, SP, N, M3

    guesses = [
        # sin^2 theta_12 = 0.307
        ('sin2_theta12 ~ sopfr/(sigma+tau) = 5/16',
         'sin2_theta12', 0.307, sp / (s + t), 'sopfr/(sigma+tau) = 5/16 = 0.3125'),

        ('sin2_theta12 ~ tau*phi/(2*sigma+phi) = 8/26',
         'sin2_theta12', 0.307, t * p / (2 * s + p),
         'tau*phi/(2*sigma+phi) = 8/26 = 0.30769'),

        ('sin2_theta12 ~ tau/sigma = 1/3 (tribimaximal)',
         'sin2_theta12', 0.307, t / s, 'tau/sigma = 1/3 = 0.3333'),

        # sin^2 theta_23 = 0.545
        ('sin2_theta23 ~ n/(sigma-1) = 6/11',
         'sin2_theta23', 0.545, n / (s - 1), 'n/(sigma-1) = 6/11 = 0.54545'),

        ('sin2_theta23 ~ phi/tau = 1/2 (tribimaximal)',
         'sin2_theta23', 0.545, p / t, 'phi/tau = 1/2 = 0.5'),

        # sin^2 theta_13 = 0.0220
        ('sin2_theta13 ~ 1/(sigma*tau-phi-1) = 1/45',
         'sin2_theta13', 0.0220, 1.0 / (s * t - p - 1),
         '1/(sigma*tau-phi-1) = 1/45 = 0.02222'),

        ('sin2_theta13 ~ 1/(sigma*tau-sopfr+1) = 1/44',
         'sin2_theta13', 0.0220, 1.0 / (s * t - sp + 1),
         '1/(sigma*tau-sopfr+1) = 1/44 = 0.02273'),

        # CP phase delta_CP = 197 degrees
        ('delta_CP ~ sigma^2 + sopfr*sigma - M3 = 197 degrees',
         'delta_CP_deg', 197.0, float(s**2 + sp * s - m3),
         f'sigma^2+sopfr*sigma-M3 = {s**2}+{sp*s}-{m3} = {s**2+sp*s-m3}'),

        # Mass ratio |Dm2_32|/Dm2_21 ~ 32.6
        ('mass_ratio ~ phi^sopfr = 2^5 = 32',
         'mass_ratio', MASS_RATIO, float(p ** sp), 'phi^sopfr = 2^5 = 32'),

        ('mass_ratio ~ sigma^2/tau - sopfr + 1 = 32',
         'mass_ratio', MASS_RATIO, float(s**2 / t - sp + 1),
         'sigma^2/tau - sopfr + 1 = 36-5+1 = 32'),
    ]
    return guesses


# ─── 5. Tribimaximal Deviation Analysis ───

def tribimaximal_analysis():
    """Analyze deviations from tribimaximal mixing in TECS-L terms."""
    s, t, p, sp, n, m3 = S, T, P, SP, N, M3
    results = []

    results.append('Tribimaximal mixing (Harrison-Perkins-Scott):')
    results.append(f'  sin2_theta12 = 1/3 = tau/sigma = {t}/{s} = {t/s:.6f}')
    results.append(f'  sin2_theta23 = 1/2 = phi/tau   = {p}/{t} = {p/t:.6f}')
    results.append(f'  sin2_theta13 = 0   (exact)')
    results.append('')
    results.append('Observed deviations from tribimaximal:')

    # theta_12 deviation
    d12 = 0.307 - 1.0 / 3
    cand12 = -1.0 / (s * t - sp + 1)  # -1/44
    err12 = abs(cand12 - d12) / abs(d12) * 100
    results.append(f'  Delta(sin2_theta12) = 0.307 - 1/3 = {d12:.4f}')
    results.append(f'    Candidate: -1/(sigma*tau-sopfr+1) = -1/44 = {cand12:.4f} '
                   f'({err12:.1f}% error)')

    # theta_23 deviation
    d23 = 0.545 - 0.5
    cand23 = 1.0 / (s * p)  # 1/24
    err23 = abs(cand23 - d23) / abs(d23) * 100
    cand23b = 1.0 / (s * p + p)  # 1/26
    err23b = abs(cand23b - d23) / abs(d23) * 100
    results.append(f'  Delta(sin2_theta23) = 0.545 - 1/2 = {d23:.4f}')
    results.append(f'    Candidate: 1/(sigma*phi) = 1/24 = {cand23:.4f} '
                   f'({err23:.1f}% error)')
    results.append(f'    Candidate: 1/(sigma*phi+phi) = 1/26 = {cand23b:.4f} '
                   f'({err23b:.1f}% error)')

    # theta_13 (pure deviation, no tribimaximal baseline)
    results.append(f'  sin2_theta13 = 0.0220 (no tribimaximal baseline)')
    cand13 = 1.0 / (s * t - p - 1)  # 1/45
    err13 = abs(cand13 - 0.0220) / 0.0220 * 100
    results.append(f'    Best: 1/(sigma*tau-phi-1) = 1/45 = {cand13:.6f} '
                   f'({err13:.1f}% error)')

    results.append('')
    results.append('Self-referential observation:')
    results.append(f'  delta_CP / 360 = 197/360 = {197/360:.4f}')
    results.append(f'  sin2_theta23            = 0.545')
    results.append(f'  Difference: {abs(197/360 - 0.545):.4f} '
                   f'({abs(197/360 - 0.545)/0.545*100:.2f}% off)')

    return results


# ─── 6. CP Phase Analysis ───

def cp_phase_analysis():
    """Decompose the CP phase delta_CP = 197 degrees."""
    s, t, p, sp, n, m3 = S, T, P, SP, N, M3
    results = []

    results.append('CP phase delta_CP = 197 degrees:')
    val = s**2 + sp * s - m3
    results.append(f'  sigma^2 + sopfr*sigma - M3 = {s**2} + {sp*s} - {m3} = {val}')
    if val == 197:
        results.append(f'  EXACT MATCH: delta_CP = sigma^2 + sopfr*sigma - M3 = 197')
    results.append('')

    # QED connection
    results.append('  197 is also the QED 2-loop numerator:')
    results.append(f'    a_mu(2-loop) ~ (alpha/pi)^2 * 197/144')
    results.append(f'    Note: 144 = sigma^2 = {s}^2')
    results.append(f'    So 197/144 = (sigma^2+sopfr*sigma-M3)/sigma^2')

    return results


# ─── 7. Jarlskog Invariant Analysis ───

def jarlskog_analysis():
    """Compute and analyze the leptonic Jarlskog invariant."""
    results = []

    # PDG values
    s12_sq = 0.307
    s23_sq = 0.545
    s13_sq = 0.0220
    dcp_rad = math.radians(197.0)

    # Extract sin and cos of angles
    s12 = math.sqrt(s12_sq)
    c12 = math.sqrt(1 - s12_sq)
    s23 = math.sqrt(s23_sq)
    c23 = math.sqrt(1 - s23_sq)
    s13 = math.sqrt(s13_sq)
    c13 = math.sqrt(1 - s13_sq)

    # J_CP = sin(delta) * cos(theta13) * sin(2*theta12) * sin(2*theta23) * sin(2*theta13) / 8
    # Using sin(2*theta) = 2*sin(theta)*cos(theta):
    # J_CP = sin(delta) * c13^2 * s13 * s12 * c12 * s23 * c23
    # (the c13^2 comes from one c13 in the formula and one from cos(theta13))
    j_cp = math.sin(dcp_rad) * c13**2 * s13 * s12 * c12 * s23 * c23

    results.append(f'Leptonic Jarlskog invariant:')
    results.append(f'  J_CP = sin(delta) * c13^2 * s13 * s12 * c12 * s23 * c23')
    results.append(f'  sin(delta_CP) = sin(197 deg) = {math.sin(dcp_rad):.6f}')
    results.append(f'  J_CP = {j_cp:.6e}')
    results.append('')

    # Compare with quark Jarlskog
    j_quark = 3.18e-5
    ratio = j_cp / j_quark
    results.append(f'  Quark Jarlskog: J_quark = {j_quark:.2e}')
    results.append(f'  Ratio: J_lepton / J_quark = {ratio:.2f}')

    # TECS-L search for the ratio
    s_c, t_c, p_c, sp_c, n_c, m3_c = S, T, P, SP, N, M3
    candidates = {
        'sigma*tau*phi': s_c * t_c * p_c,
        'sigma^2-sigma': s_c**2 - s_c,
        'n*sigma*phi': n_c * s_c * p_c,
        'sigma*sopfr*phi': s_c * sp_c * p_c,
        'P2*sopfr': P2 * sp_c,
        'sigma^3/sigma': s_c**2,
        'sigma*n': s_c * n_c,
        'sigma^2/phi': s_c**2 / p_c,
        '-sigma*phi*sopfr': s_c * p_c * sp_c,  # just checking magnitude
    }
    best_err = 100
    best_name = None
    for cname, cval in candidates.items():
        if cval > 0:
            err = abs(cval - abs(ratio)) / abs(ratio) * 100
            if err < best_err:
                best_err = err
                best_name = cname
    if best_name:
        results.append(f'  Ratio closest match: {best_name} = {candidates[best_name]:.2f} '
                       f'({best_err:.1f}% error)')

    # Absolute value of J_CP match
    results.append('')
    results.append('  Searching TECS-L expression for |J_CP|:')
    abs_j = abs(j_cp)
    j_candidates = {
        '1/(sigma^2*tau*phi)': 1.0 / (s_c**2 * t_c * p_c),  # 1/1152
        'sin(pi/n)/(sigma^3)': math.sin(math.pi / n_c) / s_c**3,
        'sopfr/(sigma^3*tau)': sp_c / (s_c**3 * t_c),
        '1/(sigma*tau*n*phi)': 1.0 / (s_c * t_c * n_c * p_c),
        'M3/(sigma^3*tau)': m3_c / (s_c**3 * t_c),
    }
    for jname, jval in j_candidates.items():
        jerr = abs(jval - abs_j) / abs_j * 100
        results.append(f'    {jname:35s} = {jval:.6e}  ({jerr:.1f}% off)')

    return results, j_cp


# ─── 8. Systematic Expression Search ───

def systematic_search(max_error_pct=5.0):
    """Generate all expressions and find best matches for each parameter."""
    exprs = build_expressions()

    # Targets (central values only)
    targets = {
        'sin2_theta12': 0.307,
        'sin2_theta23': 0.545,
        'sin2_theta13': 0.0220,
        'delta_CP_deg': 197.0,
        'mass_ratio': MASS_RATIO,
    }

    matches = find_matches(targets, exprs, max_error_pct)
    return exprs, matches


# ─── 9. Texas Sharpshooter Test ───

def texas_sharpshooter(n_expressions, n_targets, target_values,
                       max_error_pct=5.0, n_trials=100000):
    """Monte Carlo: how many matches expected by chance?

    For each trial, generate n_targets random values (log-uniform),
    count how many of n_expressions hit within max_error_pct.
    """
    rng = random.Random(42)

    # Determine log range from actual target values
    tvals = [v for v in target_values if v > 0]
    if not tvals:
        return [], 0.0

    log_min = math.log(min(tvals)) - 2
    log_max = math.log(max(tvals)) + 2
    log_range = log_max - log_min

    match_probs = []
    for tval in tvals:
        # Width of matching interval in log space
        lo = max(1e-10, 1 - max_error_pct / 100)
        hi = 1 + max_error_pct / 100
        log_width = math.log(hi) - math.log(lo)
        p_match_per_expr = log_width / log_range
        p_no_match = (1 - p_match_per_expr) ** n_expressions
        p_at_least_one = 1 - p_no_match
        match_probs.append((tval, p_match_per_expr, p_at_least_one))

    expected_matches = sum(p for _, _, p in match_probs)
    return match_probs, expected_matches


# ─── 10. Monte Carlo Validation (simultaneous match) ───

def mc_simultaneous_validation(n_trials=200000):
    """Monte Carlo: probability of simultaneously matching all 3 angles + phase.

    For each trial:
    1. Generate a random pool of ~N_expr expression values (log-uniform).
    2. Check if all 4 PMNS parameters can be matched within tolerance.
    """
    rng = random.Random(137)

    targets = {
        'sin2_theta12': (0.307, 0.05),   # (value, fractional_tolerance)
        'sin2_theta23': (0.545, 0.05),
        'sin2_theta13': (0.0220, 0.05),
        'delta_CP_deg': (197.0, 0.05),
    }

    # Build actual expression set to get count and value range
    exprs = build_expressions()
    n_expr = len(exprs)

    # Log-uniform range covering our target space
    log_min = math.log(0.01)
    log_max = math.log(300)

    n_all_match = 0

    for _ in range(n_trials):
        # Generate random expression values
        rand_exprs = [math.exp(rng.uniform(log_min, log_max))
                      for _ in range(n_expr)]

        all_matched = True
        for tname, (tval, tol) in targets.items():
            lo = tval * (1 - tol)
            hi = tval * (1 + tol)
            found = any(lo <= v <= hi for v in rand_exprs)
            if not found:
                all_matched = False
                break

        if all_matched:
            n_all_match += 1

    p_chance = n_all_match / n_trials
    return n_trials, n_all_match, p_chance, n_expr


# ─── 11. Full Report ───

def run_analysis():
    """Run complete PMNS neutrino mixing analysis and print report."""
    print('=' * 76)
    print('  PMNS NEUTRINO MIXING MATRIX — n=6 ARITHMETIC ANALYSIS')
    print('=' * 76)

    # Build expressions
    exprs = build_expressions()
    print(f'\nGenerated {len(exprs)} candidate expressions from n=6 constants')
    print(f'Constants: sigma={S}, tau={T}, phi={P}, sopfr={SP}, '
          f'n={N}, omega={W}, M3={M3}')

    # ── Section 1: Motivated guesses ──
    print('\n' + '-' * 76)
    print('  SECTION 1: Physically Motivated PMNS Guesses')
    print('-' * 76)

    guesses = check_motivated_guesses()
    for desc, tname, tval, expr_val, formula in guesses:
        err = abs(expr_val - tval) / abs(tval) * 100
        flag = ' ***' if err < 2 else (' **' if err < 5 else '')
        print(f'\n  {desc}')
        print(f'    Formula: {formula}')
        print(f'    Target:  {tval:.6g}  |  Expression: {expr_val:.6g}  |  '
              f'Error: {err:.2f}%{flag}')

    # ── Section 2: Systematic search ──
    print('\n' + '-' * 76)
    print('  SECTION 2: Systematic Expression Search (within 5%)')
    print('-' * 76)

    targets_flat = {
        'sin2_theta12': 0.307,
        'sin2_theta23': 0.545,
        'sin2_theta13': 0.0220,
        'delta_CP_deg': 197.0,
        'mass_ratio': MASS_RATIO,
    }

    all_matches = find_matches(targets_flat, exprs, max_error_pct=5.0)
    total_hits = 0
    n_matched = 0
    for tname in ['sin2_theta12', 'sin2_theta23', 'sin2_theta13',
                   'delta_CP_deg', 'mass_ratio']:
        tval = targets_flat[tname]
        matches = all_matches.get(tname, [])
        print(f'\n  {tname} = {tval}')
        if matches:
            n_matched += 1
            for expr_label, expr_val, err in matches[:8]:
                flag = ' ***' if err < 1 else (' **' if err < 3 else '')
                print(f'    {expr_label:45s} = {expr_val:.6f}  '
                      f'({err:.2f}% error){flag}')
            total_hits += len(matches)
        else:
            print(f'    (no match within 5%)')

    print(f'\n  Total hits: {total_hits} matches across {len(targets_flat)} parameters')
    print(f'  Matched: {n_matched}/{len(targets_flat)} parameters')

    # ── Section 3: Tribimaximal deviations ──
    print('\n' + '-' * 76)
    print('  SECTION 3: Tribimaximal Deviation Analysis')
    print('-' * 76)

    for line in tribimaximal_analysis():
        print(f'  {line}')

    # ── Section 4: CP Phase ──
    print('\n' + '-' * 76)
    print('  SECTION 4: CP Phase Decomposition')
    print('-' * 76)

    for line in cp_phase_analysis():
        print(f'  {line}')

    # ── Section 5: Jarlskog invariant ──
    print('\n' + '-' * 76)
    print('  SECTION 5: Jarlskog Invariant (Leptons vs Quarks)')
    print('-' * 76)

    j_lines, j_cp = jarlskog_analysis()
    for line in j_lines:
        print(f'  {line}')

    # ── Section 6: Mass ratio ──
    print('\n' + '-' * 76)
    print('  SECTION 6: Mass-Squared Difference Ratio')
    print('-' * 76)

    print(f'\n  |Dm2_32| / Dm2_21 = {MASS_RATIO:.2f}')
    print(f'  phi^sopfr = 2^5 = 32  ({abs(32 - MASS_RATIO)/MASS_RATIO*100:.1f}% off)')
    print(f'  sigma^2/tau - sopfr + 1 = 36-5+1 = 32  '
          f'({abs(32 - MASS_RATIO)/MASS_RATIO*100:.1f}% off)')

    # ── Section 7: Best matches summary ──
    print('\n' + '-' * 76)
    print('  SECTION 7: Best Single Match per Parameter')
    print('-' * 76)

    best_formulas = {}
    for tname, matches in sorted(all_matches.items()):
        tval = targets_flat[tname]
        if matches:
            best = matches[0]
            best_formulas[tname] = best
            print(f'  {tname:20s} = {tval:12.6g}  <-->  {best[0]:45s} '
                  f'= {best[1]:.6g}  ({best[2]:.2f}%)')
        else:
            print(f'  {tname:20s} = {tval:12.6g}  <-->  (no match)')

    # ── Section 8: Texas Sharpshooter ──
    print('\n' + '-' * 76)
    print('  SECTION 8: Texas Sharpshooter Test')
    print('-' * 76)

    n_expr = len(exprs)
    n_tgt = len(targets_flat)
    tvals_list = list(targets_flat.values())
    match_probs, expected = texas_sharpshooter(
        n_expr, n_tgt, tvals_list, max_error_pct=5.0)

    print(f'\n  Parameters:')
    print(f'    Expressions generated:  {n_expr}')
    print(f'    Target values:          {n_tgt}')
    print(f'    Match threshold:        5%')
    print(f'    Target value range:     [{min(tvals_list):.4g}, {max(tvals_list):.1f}]')

    print(f'\n  Per-target match probability (log-uniform null):')
    for tval, p_per, p_any in match_probs:
        print(f'    target={tval:10.4g}  P(single)={p_per:.4f}  '
              f'P(any of {n_expr})={p_any:.3f}')

    print(f'\n  Expected parameters with >= 1 match by chance: '
          f'{expected:.2f} / {n_tgt}')
    print(f'  Observed parameters with >= 1 match:           '
          f'{n_matched} / {n_tgt}')

    if n_matched > expected:
        excess = n_matched - expected
        try:
            from scipy.stats import poisson
            p_value = 1 - poisson.cdf(n_matched - 1, expected)
            print(f'\n  Excess matches: {excess:.2f}')
            print(f'  Poisson p-value: {p_value:.4f}')
            if p_value < 0.05:
                print(f'  --> Significant at p < 0.05')
            else:
                print(f'  --> NOT significant (consistent with chance)')
        except ImportError:
            print(f'\n  Excess matches: {excess:.2f}')
            print(f'  (scipy not available for Poisson p-value)')
    else:
        print(f'\n  No excess — consistent with random matching.')

    # ── Section 9: Monte Carlo simultaneous match ──
    print('\n' + '-' * 76)
    print('  SECTION 9: Monte Carlo Simultaneous Validation')
    print('-' * 76)

    print(f'\n  Testing: can {n_expr} random expressions simultaneously match')
    print(f'  all 3 mixing angles + CP phase within 5%?')
    print(f'  Running 200,000 trials ...')

    n_trials, n_all_match, p_chance, n_pool = mc_simultaneous_validation(
        n_trials=200000)

    print(f'\n  Trials:          {n_trials:,}')
    print(f'  Pool size:       {n_pool} expressions per trial')
    print(f'  All-4 matches:   {n_all_match:,}')
    print(f'  P(chance):       {p_chance:.6f}')

    if p_chance < 0.05:
        print(f'  --> Simultaneous match is UNLIKELY by chance (p = {p_chance:.4f})')
    elif p_chance > 0:
        print(f'  --> Simultaneous match is plausible by chance (p = {p_chance:.4f})')
    else:
        print(f'  --> No random trial matched all 4 simultaneously '
              f'(p < {1/n_trials:.2e})')

    # ── Notable findings ──
    print('\n' + '-' * 76)
    print('  NOTABLE FINDINGS')
    print('-' * 76)

    notable = [
        ('sin2_theta12 ~ tau*phi/(2*sigma+phi) = 8/26 = 4/13',
         0.307, T * P / (2 * S + P)),
        ('sin2_theta23 ~ n/(sigma-1) = 6/11',
         0.545, N / (S - 1)),
        ('sin2_theta13 ~ 1/(sigma*tau-phi-1) = 1/45',
         0.0220, 1.0 / (S * T - P - 1)),
        ('delta_CP = sigma^2 + sopfr*sigma - M3 = 197 degrees (EXACT)',
         197.0, float(S**2 + SP * S - M3)),
        ('mass ratio |Dm2_32|/Dm2_21 ~ phi^sopfr = 2^5 = 32',
         MASS_RATIO, float(P ** SP)),
    ]

    for desc, obs, pred in notable:
        err = abs(pred - obs) / obs * 100
        flag = ' ***' if err < 1 else (' **' if err < 3 else '')
        print(f'\n  {desc}')
        print(f'  Observed: {obs:.6g}  Predicted: {pred:.6g}  Error: {err:.2f}%{flag}')

    print('\n' + '-' * 76)
    print('  STRUCTURAL OBSERVATIONS')
    print('-' * 76)

    print(f'\n  PMNS is 3x3 unitary: 3 = sigma/tau = {S}/{T}')
    print(f'  4 parameters (3 angles + 1 phase): 4 = tau(6) = {T}')
    print(f'  3 neutrino flavors = sigma/tau = fermion generations')
    print(f'')
    print(f'  Best simultaneous TECS-L representation:')
    print(f'    sin2_theta12 = tau*phi/(2*sigma+phi)   = 4/13  = 0.30769')
    print(f'    sin2_theta23 = n/(sigma-1)             = 6/11  = 0.54545')
    print(f'    sin2_theta13 = 1/(sigma*tau-phi-1)     = 1/45  = 0.02222')
    print(f'    delta_CP     = sigma^2+sopfr*sigma-M3  = 197 degrees (exact)')
    print(f'')
    print(f'  Tribimaximal corrections:')
    print(f'    theta_12: 1/3 -> 4/13  (shift = -1/39 = -0.0256)')
    print(f'    theta_23: 1/2 -> 6/11  (shift = +1/22 = +0.0455)')
    print(f'    theta_13: 0   -> 1/45  (emergence from zero)')

    print('\n' + '=' * 76)
    print('  END OF PMNS NEUTRINO MIXING ANALYSIS')
    print('=' * 76)

    return {
        'expressions': exprs,
        'matches': all_matches,
        'n_expressions': n_expr,
        'n_matched': n_matched,
        'expected_by_chance': expected,
        'jarlskog_lepton': j_cp,
        'mass_ratio': MASS_RATIO,
        'best_formulas': best_formulas,
    }


if __name__ == '__main__':
    run_analysis()
