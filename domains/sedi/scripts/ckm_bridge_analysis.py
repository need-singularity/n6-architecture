#!/usr/bin/env python3
"""CKM Jarlskog — Neutrino Mass Bridge Analysis.

The discovery engine found a BRIDGE between:
  - CKM Jarlskog invariant: J ~ 3.08e-5
  - Neutrino mass squared difference: dm^2_21 ~ 7.53e-5 eV^2

Deep analysis: what n=6 expression connects them?
"""
import math
from collections import OrderedDict

# ─── n=6 Constants ───
SIGMA = 12      # sigma(6) = sum of divisors
TAU = 4         # tau(6) = number of divisors
PHI = 2         # phi(6) = Euler totient
SOPFR = 5       # sopfr(6) = sum of prime factors with repetition
N = 6           # the perfect number itself
OMEGA = 2       # omega(6) = distinct prime factors

# ─── Physics Data (PDG 2024) ───
J_ARLSKOG = 3.08e-5      # Jarlskog invariant (range: 2.96-3.24 x 10^-5)
J_ARLSKOG_UNC = 0.14e-5  # uncertainty

DM21_SQ = 7.53e-5        # eV^2, solar neutrino mass squared difference
DM21_SQ_UNC = 0.18e-5

DM32_SQ = 2.453e-3       # eV^2, atmospheric (normal ordering)
DM32_SQ_UNC = 0.034e-3

# CKM matrix elements (PDG 2024)
CKM = OrderedDict([
    ('|V_ud|', 0.97373),
    ('|V_us|', 0.2243),
    ('|V_ub|', 0.00394),
    ('|V_cd|', 0.221),
    ('|V_cs|', 0.975),
    ('|V_cb|', 0.0422),
    ('|V_td|', 0.0080),
    ('|V_ts|', 0.0388),
    ('|V_tb|', 1.013),
])

# Wolfenstein parameters
WOLFENSTEIN = {
    'lambda': 0.22650,
    'A': 0.790,
    'rho_bar': 0.141,
    'eta_bar': 0.357,
}


def build_n6_expressions():
    """Build dictionary of n=6 arithmetic expressions and their values."""
    s, t, p, f, n = SIGMA, TAU, PHI, SOPFR, N
    exprs = OrderedDict()

    # Simple ratios
    exprs['phi/sigma'] = p / s                    # 1/6
    exprs['phi/sopfr'] = p / f                    # 2/5 = 0.4
    exprs['tau/sigma'] = t / s                    # 1/3
    exprs['sopfr/sigma'] = f / s                  # 5/12
    exprs['phi/tau'] = p / t                      # 1/2
    exprs['tau/sopfr'] = t / f                    # 4/5
    exprs['sopfr/tau'] = f / t                    # 5/4
    exprs['n/sigma'] = n / s                      # 1/2
    exprs['sigma/n'] = s / n                      # 2
    exprs['sigma/tau'] = s / t                    # 3
    exprs['sigma/sopfr'] = s / f                  # 12/5
    exprs['sopfr/n'] = f / n                      # 5/6
    exprs['n/sopfr'] = n / f                      # 6/5

    # Products / compounds
    exprs['1/(sigma*phi)'] = 1 / (s * p)          # 1/24
    exprs['1/(sigma*tau)'] = 1 / (s * t)          # 1/48
    exprs['1/(sigma*sopfr)'] = 1 / (s * f)        # 1/60
    exprs['phi/(sigma*tau)'] = p / (s * t)        # 1/24
    exprs['sopfr/(sigma*phi)'] = f / (s * p)      # 5/24
    exprs['tau/(sigma*sopfr)'] = t / (s * f)      # 1/15
    exprs['phi*tau/sigma^2'] = p * t / s**2       # 8/144 = 1/18

    # Powers
    exprs['1/sigma^2'] = 1 / s**2                 # 1/144
    exprs['1/sigma^3'] = 1 / s**3                 # 1/1728
    exprs['1/(sigma^2*phi)'] = 1 / (s**2 * p)    # 1/288
    exprs['1/(sigma^2*tau)'] = 1 / (s**2 * t)    # 1/576
    exprs['1/(sigma^2*tau*phi)'] = 1 / (s**2 * t * p)  # 1/1152

    # Trig
    exprs['sin(pi/sigma)'] = math.sin(math.pi / s)        # sin(15 deg)
    exprs['sin(pi/(sigma+phi))'] = math.sin(math.pi / (s + p))  # sin(pi/14)

    # Special Jarlskog candidates
    exprs['sopfr/(sigma^3*tau)'] = f / (s**3 * t)  # 5/6912
    exprs['1/(n^4)'] = 1 / n**4                     # 1/1296
    exprs['phi/sigma^3'] = p / s**3                  # 1/864
    exprs['1/(sigma^2*n)'] = 1 / (s**2 * n)        # 1/864
    exprs['1/(sigma*tau*n*phi)'] = 1 / (s * t * n * p)  # 1/576

    return exprs


def bridge_analysis():
    """Analyze the J -- dm^2_21 bridge through n=6 lens."""
    print("=" * 80)
    print("  CKM JARLSKOG — NEUTRINO MASS BRIDGE ANALYSIS")
    print("  J_arlskog = {:.3e}   |   dm^2_21 = {:.3e} eV^2".format(J_ARLSKOG, DM21_SQ))
    print("=" * 80)

    # ── 1. Direct ratio analysis ──
    print("\n" + "-" * 80)
    print("  SECTION 1: Direct Ratio J / dm^2_21")
    print("-" * 80)

    ratio = J_ARLSKOG / DM21_SQ
    print(f"\n  J / dm^2_21 = {J_ARLSKOG:.3e} / {DM21_SQ:.3e} = {ratio:.6f}")

    # What n=6 expressions match this ratio?
    exprs = build_n6_expressions()
    matches = []
    for name, val in exprs.items():
        if val == 0:
            continue
        err = abs(ratio - val) / val * 100
        if err < 10:
            matches.append((name, val, err))
    matches.sort(key=lambda x: x[2])

    print(f"\n  Candidate n=6 matches for ratio = {ratio:.6f}:")
    print(f"  {'Expression':<35s} {'Value':>12s} {'Error':>8s}")
    print(f"  {'─'*35} {'─'*12} {'─'*8}")
    for name, val, err in matches[:10]:
        flag = " ***" if err < 3 else ""
        print(f"  {name:<35s} {val:>12.6f} {err:>7.2f}%{flag}")

    # Key check: phi/sopfr = 0.4
    phi_over_sopfr = PHI / SOPFR
    err_phi_sopfr = abs(ratio - phi_over_sopfr) / phi_over_sopfr * 100
    print(f"\n  KEY CHECK: phi/sopfr = {PHI}/{SOPFR} = {phi_over_sopfr:.4f}")
    print(f"  J/dm^2_21 = {ratio:.6f}")
    print(f"  Error: {err_phi_sopfr:.2f}%")
    if err_phi_sopfr < 3:
        print(f"  ==> STRONG MATCH: J/dm^2_21 ~ phi/sopfr = 2/5 = 0.4")
    else:
        print(f"  ==> Weak match ({err_phi_sopfr:.1f}%)")

    # ── 2. Product analysis ──
    print("\n" + "-" * 80)
    print("  SECTION 2: Product J x dm^2_21")
    print("-" * 80)

    product = J_ARLSKOG * DM21_SQ
    print(f"\n  J x dm^2_21 = {product:.6e}")

    # Check against n=6 expressions scaled by powers of 10
    print(f"\n  Product in scientific notation: {product:.6e}")
    mantissa = product / 10**(math.floor(math.log10(product)))
    exponent = math.floor(math.log10(product))
    print(f"  Mantissa: {mantissa:.6f}, Exponent: 10^{exponent}")

    product_matches = []
    for name, val in exprs.items():
        if val <= 0:
            continue
        err = abs(mantissa - val) / val * 100
        if err < 10:
            product_matches.append((name, val, err))
    product_matches.sort(key=lambda x: x[2])
    if product_matches:
        print(f"\n  Mantissa matches:")
        for name, val, err in product_matches[:5]:
            print(f"    {name:<35s} = {val:.6f}  ({err:.2f}%)")

    # ── 3. Difference analysis ──
    print("\n" + "-" * 80)
    print("  SECTION 3: Difference dm^2_21 - J")
    print("-" * 80)

    diff = DM21_SQ - J_ARLSKOG
    print(f"\n  dm^2_21 - J = {diff:.6e}")
    ratio_diff = diff / J_ARLSKOG
    print(f"  (dm^2_21 - J) / J = {ratio_diff:.6f}")

    diff_matches = []
    for name, val in exprs.items():
        if val <= 0:
            continue
        err = abs(ratio_diff - val) / val * 100
        if err < 10:
            diff_matches.append((name, val, err))
    diff_matches.sort(key=lambda x: x[2])
    if diff_matches:
        for name, val, err in diff_matches[:5]:
            print(f"    {name:<35s} = {val:.6f}  ({err:.2f}%)")

    # Also check dm^2_21 / J
    inv_ratio = DM21_SQ / J_ARLSKOG
    print(f"\n  dm^2_21 / J = {inv_ratio:.6f}")
    inv_matches = []
    for name, val in exprs.items():
        if val <= 0:
            continue
        err = abs(inv_ratio - val) / val * 100
        if err < 10:
            inv_matches.append((name, val, err))
    inv_matches.sort(key=lambda x: x[2])
    if inv_matches:
        print(f"  Matches:")
        for name, val, err in inv_matches[:5]:
            flag = " ***" if err < 3 else ""
            print(f"    {name:<35s} = {val:.6f}  ({err:.2f}%){flag}")

    # ── 4. CKM matrix n=6 decomposition ──
    print("\n" + "-" * 80)
    print("  SECTION 4: CKM Matrix n=6 Decomposition")
    print("-" * 80)

    print(f"\n  {'Element':<12s} {'PDG Value':>12s} {'Best n=6 Expression':<40s} {'n=6 Value':>12s} {'Error':>8s}")
    print(f"  {'─'*12} {'─'*12} {'─'*40} {'─'*12} {'─'*8}")

    s, t, p, f, n = SIGMA, TAU, PHI, SOPFR, N

    # Pre-defined motivated matches for CKM elements
    ckm_guesses = OrderedDict([
        ('|V_ud|', [
            ('1 - 1/(sigma*phi) - 1/sigma^3', 1 - 1/(s*p) - 1/s**3),
            ('1 - sopfr/(sigma*phi*tau)', 1 - f/(s*p*t)),
            ('cos(pi/(sigma+phi))', math.cos(math.pi / (s + p))),
        ]),
        ('|V_us|', [
            ('sin(pi/(sigma+phi))', math.sin(math.pi / (s + p))),
            ('sopfr/(sigma*phi)', f / (s * p)),
            ('(tau-1)/(sigma+1)', (t - 1) / (s + 1)),
        ]),
        ('|V_ub|', [
            ('1/(sigma^2*phi)', 1 / (s**2 * p)),
            ('tau/(sigma^3)', t / s**3),
            ('sopfr/(sigma^3*tau)', f / (s**3 * t)),
        ]),
        ('|V_cd|', [
            ('sin(pi/(sigma+phi))', math.sin(math.pi / (s + p))),
            ('sopfr/(sigma*phi)', f / (s * p)),
        ]),
        ('|V_cs|', [
            ('1 - 1/(sigma*phi)', 1 - 1/(s*p)),
            ('cos(pi/(sigma+phi))', math.cos(math.pi / (s + p))),
        ]),
        ('|V_cb|', [
            ('1/(sigma*phi)', 1 / (s * p)),
            ('tau/(sigma*n)', t / (s * n)),
        ]),
        ('|V_td|', [
            ('1/(sigma*tau*phi)', 1 / (s * t * p)),
            ('phi/(sigma^2)', p / s**2),
        ]),
        ('|V_ts|', [
            ('1/(sigma*phi) - 1/sigma^2', 1/(s*p) - 1/s**2),
            ('tau/(sigma*n) - 1/sigma^3', t/(s*n) - 1/s**3),
        ]),
        ('|V_tb|', [
            ('1 + 1/sigma^3', 1 + 1/s**3),
            ('1 + 1/(sigma*tau*n)', 1 + 1/(s*t*n)),
        ]),
    ])

    for elem, pdg_val in CKM.items():
        guesses = ckm_guesses.get(elem, [])
        best_name, best_val, best_err = None, None, 100
        for gname, gval in guesses:
            err = abs(gval - pdg_val) / pdg_val * 100
            if err < best_err:
                best_name, best_val, best_err = gname, gval, err

        # Also search all expressions
        for ename, eval_ in exprs.items():
            err = abs(eval_ - pdg_val) / pdg_val * 100
            if err < best_err:
                best_name, best_val, best_err = ename, eval_, err

        if best_name:
            flag = " ***" if best_err < 2 else ""
            print(f"  {elem:<12s} {pdg_val:>12.6f} {best_name:<40s} {best_val:>12.6f} {best_err:>7.2f}%{flag}")
        else:
            print(f"  {elem:<12s} {pdg_val:>12.6f} {'(no match)':<40s}")

    # ── 5. Jarlskog invariant deep dive ──
    print("\n" + "-" * 80)
    print("  SECTION 5: Jarlskog Invariant n=6 Expressions")
    print("-" * 80)

    j_candidates = OrderedDict([
        ('1/(sigma^2*tau*phi) = 1/1152', 1 / (s**2 * t * p)),
        ('sopfr/(sigma^3*tau) = 5/6912', f / (s**3 * t)),
        ('1/(n^4) = 1/1296', 1 / n**4),
        ('phi/sigma^3 = 1/864', p / s**3),
        ('pi/(sigma^4*tau) = pi/82944', math.pi / (s**4 * t)),
        ('1/(sigma^2*n) = 1/864', 1 / (s**2 * n)),
        ('sin(pi/n)/(sigma^2*phi)', math.sin(math.pi/n) / (s**2 * p)),
        ('tau/(sigma^3*sopfr) = 4/8640', t / (s**3 * f)),
    ])

    print(f"\n  J_arlskog = {J_ARLSKOG:.4e} +/- {J_ARLSKOG_UNC:.2e}")
    print(f"\n  {'Expression':<40s} {'Value':>12s} {'Error':>8s} {'Sigma':>8s}")
    print(f"  {'─'*40} {'─'*12} {'─'*8} {'─'*8}")
    for name, val in j_candidates.items():
        err_pct = abs(val - J_ARLSKOG) / J_ARLSKOG * 100
        tension = abs(val - J_ARLSKOG) / J_ARLSKOG_UNC
        flag = " ***" if err_pct < 5 else ""
        print(f"  {name:<40s} {val:>12.4e} {err_pct:>7.2f}% {tension:>7.2f}s{flag}")

    # ── 6. Bridge interpretation ──
    print("\n" + "-" * 80)
    print("  SECTION 6: Bridge Interpretation")
    print("-" * 80)

    print(f"""
  The J -- dm^2_21 bridge through n=6:

  J_arlskog   = {J_ARLSKOG:.3e}
  dm^2_21     = {DM21_SQ:.3e} eV^2
  J / dm^2_21 = {ratio:.6f}

  If J/dm^2_21 ~ phi/sopfr = 2/5 = 0.4:
    J = (phi/sopfr) * dm^2_21
    J = (2/5) * 7.53e-5 = {0.4 * DM21_SQ:.3e}  (vs observed {J_ARLSKOG:.3e})
    Error: {err_phi_sopfr:.2f}%

  This would mean:
    CP violation (J) = (2/5) x solar neutrino mass splitting
    The Euler totient phi(6) over the sum of prime factors sopfr(6)
    connects quarks (CKM, CP) to leptons (neutrino oscillation).

  Cross-check with dm^2_32 (atmospheric):
    J / dm^2_32 = {J_ARLSKOG / DM32_SQ:.6f}
    dm^2_32 / dm^2_21 = {DM32_SQ / DM21_SQ:.4f}""")

    # Check dm32/dm21 ratio
    atm_sol_ratio = DM32_SQ / DM21_SQ
    print(f"    dm^2_32 / dm^2_21 = {atm_sol_ratio:.4f}")
    for name, val in exprs.items():
        err = abs(atm_sol_ratio - val) / val * 100
        if err < 5:
            print(f"      ~ {name} = {val:.4f} ({err:.2f}%)")

    # Check specific: is it related to sigma^2/tau or similar?
    for label, val in [('sigma^2/tau', s**2/t), ('n*sopfr', n*f), ('sigma*phi+sopfr+n', s*p+f+n),
                       ('sigma*tau/phi', s*t/p), ('n^2-tau-phi', n**2-t-p)]:
        err = abs(atm_sol_ratio - val) / val * 100
        if err < 5:
            print(f"      ~ {label} = {val:.4f} ({err:.2f}%)")

    # ── 7. Predictions ──
    print("\n" + "-" * 80)
    print("  SECTION 7: Predictions from Bridge")
    print("-" * 80)

    print(f"""
  If J = (phi/sopfr) * dm^2_21, then:

  1. Improved J measurement should converge to:
     J_predicted = (2/5) * dm^2_21 = {0.4 * DM21_SQ:.4e}
     Current:  J = ({J_ARLSKOG:.4e} +/- {J_ARLSKOG_UNC:.2e})
     Tension:  {abs(0.4 * DM21_SQ - J_ARLSKOG) / J_ARLSKOG_UNC:.2f} sigma

  2. Improved dm^2_21 should converge to:
     dm^2_21_predicted = (sopfr/phi) * J = (5/2) * J = {2.5 * J_ARLSKOG:.4e} eV^2
     Current:  dm^2_21 = ({DM21_SQ:.4e} +/- {DM21_SQ_UNC:.2e})
     Tension:  {abs(2.5 * J_ARLSKOG - DM21_SQ) / DM21_SQ_UNC:.2f} sigma

  3. If bridge holds, then CP violation and neutrino oscillation
     share a common n=6 origin: both are controlled by the
     arithmetic of the first perfect number.
""")

    print("=" * 80)
    print("  END OF BRIDGE ANALYSIS")
    print("=" * 80)


if __name__ == '__main__':
    bridge_analysis()
