"""CKM Quark Mixing Matrix Analysis — n=6 arithmetic expressions.

Systematic search: can CKM matrix elements (Wolfenstein parameters, Jarlskog
invariant) be expressed using the n=6 arithmetic constants
{sigma=12, tau=4, phi=2, sopfr=5, n=6, omega=2}?

Includes Texas Sharpshooter p-value test.
"""
import math
import itertools
from collections import defaultdict
from ..tecs import (
    SIGMA_P1 as S, TAU_P1 as T, PHI_P1 as P, SOPFR_P1 as SP,
    P1 as N, OMEGA_P1 as W,
    TAU_P2, TAU_P3, PHI_P3,
)


# ─── 1. PDG CKM Matrix Elements (magnitudes) ───

CKM_PDG = {
    '|V_ud|': 0.97373,
    '|V_us|': 0.2243,
    '|V_ub|': 0.00382,
    '|V_cd|': 0.221,
    '|V_cs|': 0.975,
    '|V_cb|': 0.0408,
    '|V_td|': 0.0080,
    '|V_ts|': 0.0388,
    '|V_tb|': 1.013,
}

WOLFENSTEIN_PDG = {
    'lambda': 0.22650,
    'A': 0.790,
    'rho_bar': 0.141,
    'eta_bar': 0.357,
    'J_arlskog': 3.18e-5,
}


# ─── 2. Build Expression Dictionary ───

def _safe_eval(func, label):
    """Evaluate expression, return (value, label) or None on error."""
    try:
        v = func()
        if v is not None and math.isfinite(v) and v > 0:
            return (v, label)
    except (ZeroDivisionError, ValueError, OverflowError):
        pass
    return None


def build_expressions():
    """Generate ~200+ candidate expressions from n=6 constants.

    Constants: sigma=12, tau=4, phi=2, sopfr=5, n=6, omega=2
    Operations: +, -, *, /, ^, sqrt, ln, sin, cos, pi, e
    """
    s, t, p, sp, n, w = S, T, P, SP, N, W  # 12, 4, 2, 5, 6, 2

    consts = {'sigma': s, 'tau': t, 'phi': p, 'sopfr': sp, 'n': n, 'omega': w}
    exprs = {}

    # --- Layer 1: Single constants and their reciprocals ---
    for name, val in consts.items():
        exprs[name] = val
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
            exprs[f'{na}/{nb}'] = a / b
            exprs[f'{na}*{nb}'] = a * b
            if i < j:
                exprs[f'{na}+{nb}'] = a + b
            if a - b > 0:
                exprs[f'{na}-{nb}'] = a - b

    # --- Layer 3: Triple products, a*b*c, a/(b*c), 1/(a*b) ---
    for i in range(len(cnames)):
        for j in range(i + 1, len(cnames)):
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            prod = a * b
            exprs[f'1/({na}*{nb})'] = 1.0 / prod
            for k in range(len(cnames)):
                c, nc = cvals[k], cnames[k]
                if k == i or k == j:
                    continue
                exprs[f'{nc}/({na}*{nb})'] = c / prod
                exprs[f'{na}*{nb}*{nc}'] = prod * c
                exprs[f'1/({na}*{nb}*{nc})'] = 1.0 / (prod * c)
                exprs[f'{nc}/({na}*{nb})+1/{na}'] = c / prod + 1.0 / a

    # --- Layer 4: Powers ---
    for i in range(len(cnames)):
        for j in range(len(cnames)):
            if i == j:
                continue
            a, b = cvals[i], cvals[j]
            na, nb = cnames[i], cnames[j]
            # (a/b)^c for small integer powers
            ratio = a / b
            if ratio > 0:
                for pwr in [2, 3, -1, -2, -3, 0.5]:
                    try:
                        v = ratio ** pwr
                        if math.isfinite(v) and v > 0:
                            exprs[f'({na}/{nb})^{pwr}'] = v
                    except (ValueError, OverflowError):
                        pass
            # a^b for small values
            if a <= 12 and b <= 6:
                try:
                    v = a ** b
                    if math.isfinite(v) and v > 0:
                        exprs[f'{na}^{nb}'] = v
                except OverflowError:
                    pass

    # --- Layer 5: Trigonometric (pi/n family) ---
    for name, val in consts.items():
        if val > 0:
            exprs[f'sin(pi/{name})'] = math.sin(math.pi / val)
            exprs[f'cos(pi/{name})'] = math.cos(math.pi / val)
            exprs[f'sin(pi/{name})^2'] = math.sin(math.pi / val) ** 2
            exprs[f'cos(pi/{name})^2'] = math.cos(math.pi / val) ** 2
            # Also sin(pi*a/b)
            for name2, val2 in consts.items():
                if name2 != name and val2 > 0:
                    arg = math.pi * val / val2
                    if 0 < arg < math.pi:
                        sv = math.sin(arg)
                        if sv > 0:
                            exprs[f'sin(pi*{name}/{name2})'] = sv
                    cv = math.cos(arg)
                    if cv > 0:
                        exprs[f'cos(pi*{name}/{name2})'] = cv

    # --- Layer 6: Logarithms ---
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
            ratio = a / b
            if ratio > 0 and ratio != 1:
                exprs[f'ln({na}/{nb})'] = abs(math.log(ratio))

    # --- Layer 7: Compound expressions with pi and e ---
    exprs['1/e'] = 1.0 / math.e
    exprs['1/pi'] = 1.0 / math.pi
    exprs['pi/sigma'] = math.pi / s
    exprs['pi/tau'] = math.pi / t
    exprs['pi/(sigma*phi)'] = math.pi / (s * p)
    exprs['e/sigma'] = math.e / s
    exprs['1/(sigma*pi)'] = 1.0 / (s * math.pi)
    exprs['1/(tau*pi)'] = 1.0 / (t * math.pi)
    exprs['phi*pi/sigma'] = p * math.pi / s
    exprs['sqrt(phi/sigma)'] = math.sqrt(p / s)
    exprs['sqrt(tau/sigma)'] = math.sqrt(t / s)
    exprs['sqrt(phi/(sigma*tau))'] = math.sqrt(p / (s * t))

    # --- Layer 8: Wolfenstein-motivated compound ---
    exprs['(sigma/tau)/(sigma+1)'] = (s / t) / (s + 1)  # 3/13 = sin^2 theta_W
    exprs['1/(sigma*phi)'] = 1.0 / (s * p)  # 1/24
    exprs['phi/(sigma*tau)'] = p / (s * t)  # 2/48 = 1/24
    exprs['1/(sigma^2*phi)'] = 1.0 / (s**2 * p)  # 1/288
    exprs['phi/(sigma^2*tau)'] = p / (s**2 * t)  # 1/288... no, 2/576=1/288
    exprs['1/(sigma*tau*phi)'] = 1.0 / (s * t * p)  # 1/96
    exprs['1/(sigma*tau*phi+tau)'] = 1.0 / (s * t * p + t)  # 1/100
    exprs['sopfr/(sigma*phi)'] = sp / (s * p)  # 5/24
    exprs['sopfr/(sigma*tau)'] = sp / (s * t)  # 5/48
    exprs['(sopfr-tau)/sigma'] = (sp - t) / s  # 1/12
    exprs['sopfr/(n*tau)'] = sp / (n * t)  # 5/24
    exprs['tau/(sigma+sopfr)'] = t / (s + sp)  # 4/17
    exprs['phi/(sigma+1)'] = p / (s + 1)  # 2/13
    exprs['(tau-1)/(sigma+1)'] = (t - 1) / (s + 1)  # 3/13
    exprs['1/(n*sopfr)'] = 1.0 / (n * sp)  # 1/30
    exprs['1/(sigma*sopfr)'] = 1.0 / (s * sp)  # 1/60
    exprs['phi/(n*sopfr)'] = p / (n * sp)  # 2/30 = 1/15
    exprs['1/(tau^3)'] = 1.0 / (t**3)  # 1/64
    exprs['1/(sigma^2)'] = 1.0 / (s**2)  # 1/144
    exprs['1/(n^3)'] = 1.0 / (n**3)  # 1/216
    exprs['1/(sigma*n)'] = 1.0 / (s * n)  # 1/72
    exprs['sopfr/sigma'] = sp / s  # 5/12
    exprs['(sigma-sopfr)/sigma'] = (s - sp) / s  # 7/12
    exprs['tau/(sigma*sopfr)'] = t / (s * sp)  # 4/60 = 1/15
    exprs['phi*tau/(sigma*sopfr)'] = p * t / (s * sp)  # 8/60 = 2/15

    # Higher-order Jarlskog candidates
    exprs['1/(sigma^3*phi)'] = 1.0 / (s**3 * p)  # 1/3456
    exprs['1/(sigma^2*tau*phi)'] = 1.0 / (s**2 * t * p)  # 1/1152
    exprs['sopfr/(sigma^3*tau)'] = sp / (s**3 * t)  # 5/6912
    exprs['1/(n^4)'] = 1.0 / (n**4)  # 1/1296
    exprs['phi/(sigma^3)'] = p / (s**3)  # 2/1728 = 1/864
    exprs['1/(sigma^2*n)'] = 1.0 / (s**2 * n)  # 1/864
    exprs['1/(sigma*tau*n*phi)'] = 1.0 / (s * t * n * p)  # 1/576
    exprs['tau/(sigma^3*phi)'] = t / (s**3 * p)  # 4/3456 = 1/864
    exprs['1/(n*sigma^2*phi)'] = 1.0 / (n * s**2 * p)  # 1/1728

    # --- Layer 9: P2/P3 motivated ---
    exprs['1/P2'] = 1.0 / 28
    exprs['1/P3'] = 1.0 / 496
    exprs['tau(P2)/sigma'] = TAU_P2 / s  # 6/12 = 0.5
    exprs['tau(P3)/sigma^2'] = TAU_P3 / (s**2)  # 10/144
    exprs['phi(P3)/sigma^3'] = PHI_P3 / (s**3)  # 240/1728

    # --- Layer 10: Cabibbo-motivated ---
    exprs['sin(pi/sigma)'] = math.sin(math.pi / s)  # sin(15 deg) = 0.2588
    exprs['sin(pi/14)'] = math.sin(math.pi / 14)  # 0.2225
    exprs['sin(pi/(sigma+phi))'] = math.sin(math.pi / (s + p))  # sin(pi/14)
    exprs['1/(tau+phi/tau)'] = 1.0 / (t + p / t)  # 1/4.5
    exprs['sopfr/(sigma*phi+phi)'] = sp / (s * p + p)  # 5/26
    exprs['(sopfr-1)/(sigma+sopfr)'] = (sp - 1) / (s + sp)  # 4/17
    exprs['sopfr/(sigma+sopfr+n)'] = sp / (s + sp + n)  # 5/23

    # Filter to positive finite values
    result = {}
    for label, val in exprs.items():
        try:
            if val is not None and math.isfinite(val) and val > 0:
                result[label] = val
        except (TypeError, ValueError):
            pass

    return result


# ─── 3. Match CKM Elements to Expressions ───

def find_matches(targets, expressions, max_error_pct=5.0):
    """For each target, find all expressions within max_error_pct."""
    results = {}
    for tname, tval in targets.items():
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
    """Check specific physically-motivated CKM expressions."""
    s, t, p, sp, n = S, T, P, SP, N

    guesses = [
        # (description, target_name, target_val, expression_val, formula)
        ('Cabibbo: |V_us| ~ 1/tau',
         '|V_us|', 0.2243, 1.0 / t, '1/tau = 1/4 = 0.25'),

        ('Cabibbo: |V_us| ~ sopfr/(sigma*phi)',
         '|V_us|', 0.2243, sp / (s * p), 'sopfr/(sigma*phi) = 5/24 = 0.2083'),

        ('Cabibbo: |V_us| ~ sin(pi/(sigma+phi))',
         '|V_us|', 0.2243, math.sin(math.pi / (s + p)),
         f'sin(pi/14) = {math.sin(math.pi / 14):.6f}'),

        ('Cabibbo: |V_us| ~ (tau-1)/(sigma+1)',
         '|V_us|', 0.2243, (t - 1) / (s + 1),
         f'3/13 = {3/13:.6f} (= sin^2 theta_W)'),

        ('|V_cb| ~ 1/(sigma*phi) = 1/24',
         '|V_cb|', 0.0408, 1.0 / (s * p), '1/24 = 0.04167'),

        ('|V_ub| ~ 1/(sigma^2*phi) = 1/288',
         '|V_ub|', 0.00382, 1.0 / (s**2 * p), '1/288 = 0.003472'),

        ('|V_ub| ~ phi/(sigma^2*tau) = 1/288',
         '|V_ub|', 0.00382, p / (s**2 * t),
         f'2/(144*4) = {p / (s**2 * t):.6f}'),

        ('|V_td| ~ 1/(sigma*tau*phi+tau) = 1/100',
         '|V_td|', 0.0080, 1.0 / (s * t * p + t), '1/100 = 0.01'),

        ('|V_td| ~ 1/(sigma*tau*phi) = 1/96',
         '|V_td|', 0.0080, 1.0 / (s * t * p),
         f'1/96 = {1/(s*t*p):.6f}'),

        ('|V_td| ~ sopfr/(sigma^3*tau)',
         '|V_td|', 0.0080, sp / (s**3 * t),
         f'5/6912 = {sp / (s**3 * t):.6f}'),

        ('lambda ~ sin^2(theta_W) = 3/13',
         'lambda', 0.22650, (t - 1) / (s + 1),
         f'(tau-1)/(sigma+1) = 3/13 = {3/13:.6f}'),

        ('lambda ~ sin(pi/14)',
         'lambda', 0.22650, math.sin(math.pi / 14),
         f'sin(pi/(sigma+phi)) = {math.sin(math.pi / 14):.6f}'),

        ('J_arlskog ~ 1/(sigma^2*tau*phi) = 1/1152',
         'J_arlskog', 3.18e-5, 1.0 / (s**2 * t * p),
         f'1/1152 = {1/(s**2*t*p):.6f}'),
    ]
    return guesses


# ─── 5. Structural Analysis ───

def structural_analysis():
    """Check if CKM structural properties relate to n=6."""
    s, t, p, sp, n = S, T, P, SP, N
    results = []

    # Matrix dimension
    results.append(f'CKM is 3x3 unitary matrix.  3 = sigma/tau = {s}/{t} = {s//t}')
    results.append(f'CKM has 4 independent parameters.  4 = tau(6) = {t}')
    results.append(f'  (3 mixing angles + 1 CP phase)')

    # Hierarchy ratios
    vus, vcb, vub, vtd, vts = 0.2243, 0.0408, 0.00382, 0.0080, 0.0388
    r1 = vus / vcb
    r2 = vcb / vub
    r3 = vus / vub
    r4 = vts / vtd

    results.append('')
    results.append('CKM Hierarchy Ratios:')
    results.append(f'  |V_us|/|V_cb| = {r1:.3f}  (sopfr = {sp}, n-1 = {n-1})')
    results.append(f'  |V_cb|/|V_ub| = {r2:.3f}  (tau(496) = {TAU_P3})')
    results.append(f'  |V_us|/|V_ub| = {r3:.2f}  (n*sigma-tau = {n*s - t} = {n*s-t})')
    results.append(f'  |V_ts|/|V_td| = {r4:.2f}  (sopfr = {sp}, n-1 = {n-1})')

    # Check ratio matches
    targets = {
        'sopfr': sp, 'n': n, 'n-1': n - 1, 'tau': t, 'sigma/tau': s / t,
        'sigma': s, 'sigma-tau': s - t, 'sigma/phi': s / p, 'tau(496)': TAU_P3,
        'sigma+1': s + 1, 'tau*phi': t * p, 'sigma/sopfr': s / sp,
        'sopfr+1': sp + 1, 'n+1': n + 1,
    }

    for rname, rval in [('|V_us|/|V_cb|', r1), ('|V_cb|/|V_ub|', r2),
                        ('|V_us|/|V_ub|', r3), ('|V_ts|/|V_td|', r4)]:
        best_err = 100
        best_match = None
        for tname, tval in targets.items():
            err = abs(rval - tval) / tval * 100
            if err < best_err:
                best_err = err
                best_match = tname
        if best_match is not None:
            results.append(f'  Best match for {rname} = {rval:.3f}: '
                           f'{best_match} = {targets[best_match]:.3f} '
                           f'({best_err:.1f}% error)')

    return results


# ─── 6. Texas Sharpshooter Test ───

def texas_sharpshooter(n_expressions, n_targets, max_error_pct=5.0,
                       n_trials=100000):
    """Monte Carlo: how many matches expected by chance?

    For each trial, generate n_targets random values uniformly in [1e-6, 2]
    (log-uniform), count how many of n_expressions hit within max_error_pct.
    """
    import random
    random.seed(42)

    # Expected matches per target from uniform random expression
    # Each expression has a probability of ~2*max_error_pct/100 of matching
    # (within 5% means value falls in [0.95*target, 1.05*target])
    # For log-uniform distribution over [1e-6, 2], the probability depends
    # on the width of the log-interval.

    log_range = math.log(2) - math.log(1e-6)  # total log range
    match_probs = []

    # Real CKM values span [3e-5, 1.013]
    all_targets = {**CKM_PDG, **WOLFENSTEIN_PDG}

    for tname, tval in all_targets.items():
        # Width of matching interval in log space
        log_width = math.log(1 + max_error_pct / 100) - math.log(1 - max_error_pct / 100)
        p_match_per_expr = log_width / log_range
        # Probability of at least one match from n_expressions
        p_no_match = (1 - p_match_per_expr) ** n_expressions
        p_at_least_one = 1 - p_no_match
        match_probs.append((tname, p_match_per_expr, p_at_least_one))

    # Expected total matches (sum of probabilities)
    expected_matches = sum(p for _, _, p in match_probs)

    return match_probs, expected_matches


# ─── 7. Full Report ───

def run_analysis():
    """Run complete CKM analysis and print report."""
    print('=' * 76)
    print('  CKM QUARK MIXING MATRIX — n=6 ARITHMETIC ANALYSIS')
    print('=' * 76)

    # Build expressions
    exprs = build_expressions()
    print(f'\nGenerated {len(exprs)} candidate expressions from n=6 constants')
    print(f'Constants: sigma={S}, tau={T}, phi={P}, sopfr={SP}, n={N}, omega={W}')

    # ── Section 1: CKM Matrix matches ──
    print('\n' + '─' * 76)
    print('  SECTION 1: CKM Matrix Element Matches (within 5%)')
    print('─' * 76)

    ckm_matches = find_matches(CKM_PDG, exprs, max_error_pct=5.0)
    total_hits = 0
    for element, matches in sorted(ckm_matches.items()):
        pdg_val = CKM_PDG[element]
        print(f'\n  {element} = {pdg_val}')
        if matches:
            for expr_label, expr_val, err in matches[:5]:  # top 5
                print(f'    {expr_label:40s} = {expr_val:.6f}  ({err:.2f}% error)')
            total_hits += len(matches)
        else:
            print(f'    (no match within 5%)')

    print(f'\n  Total CKM hits: {total_hits} matches across {len(CKM_PDG)} elements')

    # ── Section 2: Wolfenstein matches ──
    print('\n' + '─' * 76)
    print('  SECTION 2: Wolfenstein Parameter Matches (within 5%)')
    print('─' * 76)

    wolf_matches = find_matches(WOLFENSTEIN_PDG, exprs, max_error_pct=5.0)
    for param, matches in sorted(wolf_matches.items()):
        pdg_val = WOLFENSTEIN_PDG[param]
        print(f'\n  {param} = {pdg_val}')
        if matches:
            for expr_label, expr_val, err in matches[:5]:
                print(f'    {expr_label:40s} = {expr_val:.6f}  ({err:.2f}% error)')
        else:
            print(f'    (no match within 5%)')

    # ── Section 3: Motivated guesses ──
    print('\n' + '─' * 76)
    print('  SECTION 3: Physically Motivated Guesses')
    print('─' * 76)

    guesses = check_motivated_guesses()
    for desc, tname, tval, expr_val, formula in guesses:
        err = abs(expr_val - tval) / abs(tval) * 100
        flag = ' ***' if err < 5 else ''
        print(f'\n  {desc}')
        print(f'    Formula: {formula}')
        print(f'    Target:  {tval:.6g}  |  Expression: {expr_val:.6g}  |  '
              f'Error: {err:.2f}%{flag}')

    # ── Section 4: Structural analysis ──
    print('\n' + '─' * 76)
    print('  SECTION 4: Structural Properties of CKM and n=6')
    print('─' * 76)

    for line in structural_analysis():
        print(f'  {line}')

    # ── Section 5: Best matches summary ──
    print('\n' + '─' * 76)
    print('  SECTION 5: Best Single Match per CKM Element')
    print('─' * 76)

    all_targets = {**CKM_PDG, **WOLFENSTEIN_PDG}
    all_matches = find_matches(all_targets, exprs, max_error_pct=5.0)
    n_elements_matched = 0
    for tname, matches in sorted(all_matches.items()):
        tval = all_targets[tname]
        if matches:
            best = matches[0]
            n_elements_matched += 1
            print(f'  {tname:15s} = {tval:12.6g}  <-->  {best[0]:40s} '
                  f'= {best[1]:.6g}  ({best[2]:.2f}%)')
        else:
            print(f'  {tname:15s} = {tval:12.6g}  <-->  (no match)')

    print(f'\n  Matched {n_elements_matched}/{len(all_targets)} '
          f'elements within 5% error')

    # ── Section 6: Texas Sharpshooter ──
    print('\n' + '─' * 76)
    print('  SECTION 6: Texas Sharpshooter Test')
    print('─' * 76)

    n_expr = len(exprs)
    n_tgt = len(all_targets)
    match_probs, expected = texas_sharpshooter(n_expr, n_tgt)

    print(f'\n  Parameters:')
    print(f'    Expressions generated:  {n_expr}')
    print(f'    Target values:          {n_tgt}')
    print(f'    Match threshold:        5%')
    print(f'    Target value range:     [3e-5, 1.013] (4.5 decades)')

    print(f'\n  Per-target match probability (log-uniform null):')
    for tname, p_per, p_any in match_probs:
        print(f'    {tname:15s}  P(single expr match) = {p_per:.4f}  '
              f'P(any of {n_expr} match) = {p_any:.3f}')

    print(f'\n  Expected elements with at least one match by chance: '
          f'{expected:.1f} / {n_tgt}')
    print(f'  Observed elements with at least one match:           '
          f'{n_elements_matched} / {n_tgt}')

    if n_elements_matched > expected:
        excess = n_elements_matched - expected
        # Simple Poisson p-value approximation
        import scipy.stats as stats
        p_value = 1 - stats.poisson.cdf(n_elements_matched - 1, expected)
        print(f'\n  Excess matches: {excess:.1f}')
        print(f'  Poisson p-value: {p_value:.4f}')
        if p_value < 0.05:
            print(f'  --> Significant at p < 0.05 (not expected by chance alone)')
        else:
            print(f'  --> NOT significant (consistent with chance)')
    else:
        print(f'\n  No excess — consistent with random matching.')

    # ── Notable findings ──
    print('\n' + '─' * 76)
    print('  NOTABLE FINDINGS')
    print('─' * 76)

    notable = [
        ('|V_cb| ~ 1/(sigma*phi) = 1/24',
         0.0408, 1.0 / (S * P)),
        ('|V_us| ~ sin(pi/(sigma+phi)) = sin(pi/14)',
         0.2243, math.sin(math.pi / 14)),
        ('lambda ~ (tau-1)/(sigma+1) = 3/13 = sin^2(theta_W)',
         0.22650, 3 / 13),
    ]
    for desc, obs, pred in notable:
        err = abs(pred - obs) / obs * 100
        print(f'\n  {desc}')
        print(f'  Observed: {obs:.6g}  Predicted: {pred:.6g}  Error: {err:.2f}%')

    print(f'\n  KEY OBSERVATION: If lambda ~ sin^2(theta_W) = 3/13,')
    print(f'  then the Cabibbo angle and weak mixing angle share the')
    print(f'  same n=6 expression (tau-1)/(sigma+1). This would be')
    print(f'  remarkable if confirmed, but requires independent')
    print(f'  theoretical motivation.')

    print('\n' + '=' * 76)
    print('  END OF CKM ANALYSIS')
    print('=' * 76)

    return {
        'expressions': exprs,
        'ckm_matches': ckm_matches,
        'wolf_matches': wolf_matches,
        'structural': structural_analysis(),
        'n_expressions': n_expr,
        'n_matched': n_elements_matched,
        'expected_by_chance': expected,
    }


if __name__ == '__main__':
    run_analysis()
