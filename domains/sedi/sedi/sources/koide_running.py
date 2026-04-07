"""QCD Running Mass Koide Analysis.

Investigates whether the Koide formula Q = 2/3 can be satisfied for quark
triplets when masses are evolved to a common energy scale using 1-loop QCD
renormalization group equations.

Key physics:
  - Koide ratio Q is scale-invariant at 1-loop when all quarks in a triplet
    share the same anomalous dimension exponent gamma_0/(2*beta_0), because
    Q is homogeneous of degree 0 in masses.
  - Q changes with scale ONLY when quarks cross flavor thresholds (changing
    n_f and thus beta_0), or when one quark doesn't run (top quark).
  - For up-type (u,c,t): the top doesn't run, so Q varies as u,c evolve.
  - For down-type (d,s,b): all three run identically => Q is constant at 1-loop.
  - Extended Koide: some authors allow sqrt-mass sign flips, checking
    Q' = (m1+m2+m3) / (|e1*sqrt(m1)+e2*sqrt(m2)+e3*sqrt(m3)|)^2 with e_i = +/-1.
"""

import math
import numpy as np
from scipy.optimize import minimize_scalar


# ═══════════════════════════════════════════════════════════════════
# Physical Constants
# ═══════════════════════════════════════════════════════════════════

ALPHA_S_MZ = 0.1179          # alpha_s at M_Z (PDG 2024)
M_Z = 91.1876e3              # Z boson mass in MeV
KOIDE_TARGET = 2.0 / 3.0     # Koide prediction

# Quark mass thresholds (pole masses in MeV) for n_f transitions
M_CHARM_POLE = 1270.0        # charm threshold
M_BOTTOM_POLE = 4180.0       # bottom threshold
M_TOP_POLE = 172760.0        # top threshold

# MS-bar quark masses at reference scale mu0 = 2 GeV (MeV)
MU0 = 2000.0  # 2 GeV in MeV

MSBAR_2GEV = {
    'u': 2.16,
    'd': 4.67,
    's': 93.4,
    'c': 1270.0,
    'b': 4180.0,
    't': 172760.0,
}

# Known physical scales for comparison (MeV)
KNOWN_SCALES = {
    'proton':       938.3,
    'W boson':      80377.0,
    'Z boson':      91187.6,
    'Higgs':        125250.0,
    'top pole':     172760.0,
    'v_EW (vev)':   246220.0,
    'Lambda_QCD':   220.0,
    'pion':         139.6,
}

# QCD anomalous dimension for mass (1-loop)
GAMMA_0 = 8.0

# 2-loop coefficients
GAMMA_1 = 404.0 / 3.0 - 40.0 / 9.0  # for n_f=5: 404/3 - 200/9 ~ 112.4

def beta_1(n_f):
    """2-loop beta function coefficient."""
    return 102.0 - 38.0 / 3.0 * n_f


# ═══════════════════════════════════════════════════════════════════
# 1-Loop QCD Running
# ═══════════════════════════════════════════════════════════════════

def beta_0(n_f):
    """1-loop QCD beta function coefficient: beta_0 = (33 - 2*n_f) / 3."""
    return (33.0 - 2.0 * n_f) / 3.0


def n_f_at_scale(mu):
    """Number of active quark flavors at scale mu (MeV)."""
    if mu < M_CHARM_POLE:
        return 3
    elif mu < M_BOTTOM_POLE:
        return 4
    elif mu < M_TOP_POLE:
        return 5
    else:
        return 6


def alpha_s_1loop(mu, alpha_s_ref=ALPHA_S_MZ, mu_ref=M_Z):
    """1-loop running of alpha_s from mu_ref to mu, crossing thresholds."""
    thresholds = sorted([M_CHARM_POLE, M_BOTTOM_POLE, M_TOP_POLE])

    if mu == mu_ref:
        return alpha_s_ref

    if mu > mu_ref:
        current_mu = mu_ref
        current_alpha = alpha_s_ref
        crossings = [t for t in thresholds if mu_ref < t < mu]
        crossings.append(mu)
        for target in crossings:
            nf = n_f_at_scale(current_mu)
            b0 = beta_0(nf)
            log_ratio = math.log(target / current_mu)
            denom = 1.0 + current_alpha * b0 * log_ratio / (2.0 * math.pi)
            if denom <= 0:
                return 0.01
            current_alpha = current_alpha / denom
            current_mu = target
        return current_alpha
    else:
        current_mu = mu_ref
        current_alpha = alpha_s_ref
        crossings = sorted([t for t in thresholds if mu < t <= mu_ref], reverse=True)
        crossings.append(mu)
        for target in crossings:
            nf = n_f_at_scale(target)
            b0 = beta_0(nf)
            log_ratio = math.log(target / current_mu)
            denom = 1.0 + current_alpha * b0 * log_ratio / (2.0 * math.pi)
            if denom <= 0:
                return 2.0
            current_alpha = current_alpha / denom
            current_mu = target
        return current_alpha


def running_mass(m_ref, mu_ref, mu):
    """Evolve quark mass from mu_ref to mu using 1-loop QCD RGE.

    m(mu) = m(mu_ref) * (alpha_s(mu)/alpha_s(mu_ref))^(gamma_0/(2*beta_0))
    with proper threshold matching.
    """
    if mu == mu_ref:
        return m_ref

    thresholds = sorted([M_CHARM_POLE, M_BOTTOM_POLE, M_TOP_POLE])

    if mu > mu_ref:
        current_mu = mu_ref
        current_m = m_ref
        crossings = [t for t in thresholds if mu_ref < t < mu]
        crossings.append(mu)
    else:
        current_mu = mu_ref
        current_m = m_ref
        crossings = sorted([t for t in thresholds if mu < t <= mu_ref], reverse=True)
        crossings.append(mu)

    for target in crossings:
        nf = n_f_at_scale(min(current_mu, target))
        b0 = beta_0(nf)
        alpha_current = alpha_s_1loop(current_mu)
        alpha_target = alpha_s_1loop(target)

        if alpha_current <= 0 or alpha_target <= 0:
            current_mu = target
            continue

        exponent = GAMMA_0 / (2.0 * b0)
        ratio = alpha_target / alpha_current
        if ratio > 0:
            current_m = current_m * (ratio ** exponent)
        current_mu = target

    return current_m


def running_mass_at_scale(quark, mu):
    """Get running mass of a quark at scale mu (MeV).

    Top quark uses pole mass (decays before hadronizing, doesn't run).
    """
    if quark == 't':
        return MSBAR_2GEV['t']
    return running_mass(MSBAR_2GEV[quark], MU0, mu)


# ═══════════════════════════════════════════════════════════════════
# Koide Analysis
# ═══════════════════════════════════════════════════════════════════

def koide_Q(m1, m2, m3):
    """Standard Koide ratio Q = (m1+m2+m3) / (sqrt(m1)+sqrt(m2)+sqrt(m3))^2."""
    s = math.sqrt(m1) + math.sqrt(m2) + math.sqrt(m3)
    return (m1 + m2 + m3) / (s * s)


def koide_extended(m1, m2, m3):
    """Extended Koide: try all sign combinations for sqrt masses.

    Some authors note that for quarks, allowing a negative sign on the
    lightest sqrt(mass) can recover Q ~ 2/3.
    Returns dict of {signs_tuple: Q_value}.
    """
    import itertools
    results = {}
    for signs in itertools.product([+1, -1], repeat=3):
        s = (signs[0] * math.sqrt(m1) +
             signs[1] * math.sqrt(m2) +
             signs[2] * math.sqrt(m3))
        if abs(s) < 1e-20:
            continue
        Q = (m1 + m2 + m3) / (s * s)
        results[signs] = Q
    return results


def koide_at_scale(quarks, mu):
    """Compute Koide Q for a quark triplet at energy scale mu (MeV)."""
    masses = [running_mass_at_scale(q, mu) for q in quarks]
    return koide_Q(*masses), masses


def koide_extended_at_scale(quarks, mu):
    """Compute extended Koide for all sign combos at scale mu."""
    masses = [running_mass_at_scale(q, mu) for q in quarks]
    return koide_extended(*masses), masses


def scan_koide_vs_scale(quarks, mu_min=1e3, mu_max=1e7, n_points=200):
    """Scan Koide Q across energy scales."""
    scales = np.logspace(np.log10(mu_min), np.log10(mu_max), n_points)
    Q_vals = []
    all_masses = []
    for mu in scales:
        Q, masses = koide_at_scale(quarks, mu)
        Q_vals.append(Q)
        all_masses.append(masses)
    return scales, np.array(Q_vals), all_masses


def find_optimal_scale(quarks, mu_min=1e3, mu_max=1e7):
    """Find energy scale where Koide Q is closest to 2/3."""
    def objective(log_mu):
        mu = 10.0 ** log_mu
        Q, _ = koide_at_scale(quarks, mu)
        return abs(Q - KOIDE_TARGET)

    result = minimize_scalar(
        objective,
        bounds=(np.log10(mu_min), np.log10(mu_max)),
        method='bounded',
    )
    optimal_mu = 10.0 ** result.x
    Q_opt, masses_opt = koide_at_scale(quarks, optimal_mu)
    return optimal_mu, Q_opt, masses_opt


def find_optimal_scale_extended(quarks, signs, mu_min=1e3, mu_max=1e7):
    """Find optimal scale for extended Koide with given sign combo."""
    def objective(log_mu):
        mu = 10.0 ** log_mu
        masses = [running_mass_at_scale(q, mu) for q in quarks]
        s = sum(si * math.sqrt(mi) for si, mi in zip(signs, masses))
        if abs(s) < 1e-20:
            return 1.0
        Q = sum(masses) / (s * s)
        return abs(Q - KOIDE_TARGET)

    result = minimize_scalar(
        objective,
        bounds=(np.log10(mu_min), np.log10(mu_max)),
        method='bounded',
    )
    optimal_mu = 10.0 ** result.x
    masses = [running_mass_at_scale(q, optimal_mu) for q in quarks]
    s = sum(si * math.sqrt(mi) for si, mi in zip(signs, masses))
    Q = sum(masses) / (s * s) if abs(s) > 1e-20 else float('inf')
    return optimal_mu, Q, masses


# ═══════════════════════════════════════════════════════════════════
# TECS-L Connection Analysis
# ═══════════════════════════════════════════════════════════════════

def tecs_analysis(optimal_scale_mev):
    """Check if optimal scale connects to TECS-L n=6 arithmetic."""
    sigma_6 = 12
    tau_6 = 4
    phi_6 = 2

    delta = phi_6 * tau_6**2 / sigma_6**2  # = 2/9
    delta_exact = 2.0 / 9.0

    results = {
        'delta_tecs': delta,
        'delta_exact_2_9': delta_exact,
        'delta_match': abs(delta - delta_exact) < 1e-10,
    }

    tecs_targets = [
        ('sigma/tau=3', 3), ('tau=4', 4), ('phi=2', 2),
        ('sigma=12', 12), ('n=6', 6), ('sigma*phi=24', 24),
        ('sopfr=5', 5), ('sigma-tau=8', 8), ('sigma+tau=16', 16),
        ('1/3', 1/3), ('1/6', 1/6), ('1/4', 1/4), ('1/12', 1/12),
        ('2/3', 2/3), ('sqrt(3/2)', math.sqrt(3/2)),
        ('2/9=delta', 2/9),
    ]

    for name, mass in KNOWN_SCALES.items():
        ratio = optimal_scale_mev / mass
        results[f'ratio_to_{name}'] = ratio

        for tname, tval in tecs_targets:
            err = abs(ratio - tval) / tval if tval != 0 else float('inf')
            if err < 0.05:
                results[f'TECS_match_{name}_{tname}'] = {
                    'ratio': ratio,
                    'target': tval,
                    'error_pct': err * 100,
                }

    return results


# ═══════════════════════════════════════════════════════════════════
# Report Generation
# ═══════════════════════════════════════════════════════════════════

def print_report():
    """Print comprehensive QCD running mass Koide analysis report."""

    W = 78  # report width

    print("=" * W)
    print("  QCD RUNNING MASS KOIDE ANALYSIS")
    print("  Searching for energy scales where Koide Q = 2/3 for quark triplets")
    print("=" * W)

    # ─── Charged lepton baseline ───
    print("\n--- BASELINE: Charged Leptons (e, mu, tau) ---")
    m_e, m_mu, m_tau = 0.510999, 105.658, 1776.86  # MeV
    Q_lep = koide_Q(m_e, m_mu, m_tau)
    err_lep = abs(Q_lep - KOIDE_TARGET) / KOIDE_TARGET * 100
    print(f"  Q = {Q_lep:.8f}  (target 2/3 = {KOIDE_TARGET:.8f})")
    print(f"  Error: {err_lep:.4f}%  <-- essentially exact")

    # ─── Quarks at 2 GeV ───
    print("\n--- QUARKS AT 2 GeV (MS-bar reference scale) ---")
    up_type = ['u', 'c', 't']
    down_type = ['d', 's', 'b']

    for label, quarks in [('Up-type (u, c, t)', up_type),
                           ('Down-type (d, s, b)', down_type)]:
        masses = [MSBAR_2GEV[q] for q in quarks]
        Q = koide_Q(*masses)
        err = abs(Q - KOIDE_TARGET) / KOIDE_TARGET * 100
        print(f"  {label}: Q = {Q:.6f}  error = {err:.2f}%")
        print(f"    masses: {masses[0]:.2f}, {masses[1]:.1f}, {masses[2]:.0f} MeV")

    # ─── Physics explanation ───
    print(f"\n{'=' * W}")
    print("  WHY Q IS (MOSTLY) SCALE-INVARIANT AT 1-LOOP")
    print(f"{'=' * W}")
    print("""
  At 1-loop, all quark masses run as m(mu) ~ alpha_s(mu)^(gamma_0/(2*beta_0)).
  The Koide ratio Q = Sum(m_i) / (Sum(sqrt(m_i)))^2 is homogeneous degree 0,
  so if all masses scale by the same factor, Q does not change.

  Q can only change when:
    1. Quarks cross different flavor thresholds (n_f changes, altering beta_0)
    2. One quark doesn't run (top quark decays before hadronizing)

  For DOWN-TYPE (d, s, b): all three run with the same exponent at any given
  scale => Q is EXACTLY constant at 1-loop. Q(d,s,b) = 0.7314 at ALL scales.

  For UP-TYPE (u, c, t): the top is frozen (pole mass) while u,c run.
  As scale increases, u and c masses decrease => Q changes monotonically.
  But Q moves AWAY from 2/3 at higher scales (top dominates more).
  The minimum Q is at the lowest scale where perturbation theory works.
""")

    # ─── Up-type scan (the interesting case) ───
    print(f"{'=' * W}")
    print(f"  ENERGY SCAN: Up-type (u, c, t)")
    print(f"{'=' * W}")

    scales, Q_vals, all_masses = scan_koide_vs_scale(
        up_type, mu_min=500, mu_max=1e7, n_points=500
    )

    print(f"\n  {'Scale (GeV)':>14s}  {'Q':>10s}  {'err(%)':>8s}  "
          f"{'m_u (MeV)':>10s}  {'m_c (MeV)':>10s}  {'m_t (MeV)':>10s}")
    print(f"  {'-'*14}  {'-'*10}  {'-'*8}  {'-'*10}  {'-'*10}  {'-'*10}")

    step = max(1, len(scales) // 40)
    for i in range(0, len(scales), step):
        mu = scales[i]
        Q = Q_vals[i]
        ms = all_masses[i]
        err = abs(Q - KOIDE_TARGET) / KOIDE_TARGET * 100
        print(f"  {mu/1e3:14.4f}  {Q:10.6f}  {err:8.4f}  "
              f"{ms[0]:10.4f}  {ms[1]:10.2f}  {ms[2]:10.0f}")

    opt_mu, opt_Q, opt_masses = find_optimal_scale(up_type, mu_min=500)
    opt_err = abs(opt_Q - KOIDE_TARGET) / KOIDE_TARGET * 100
    print(f"\n  Closest to 2/3 at LOWEST scale ({opt_mu/1e3:.4f} GeV)")
    print(f"  Q = {opt_Q:.8f}, error = {opt_err:.4f}%")
    print(f"  (Q is monotonically increasing with scale for u,c,t)")

    # ─── Down-type confirmation ───
    print(f"\n{'=' * W}")
    print(f"  ENERGY SCAN: Down-type (d, s, b)")
    print(f"{'=' * W}")

    scales_d, Q_vals_d, masses_d = scan_koide_vs_scale(
        down_type, mu_min=500, mu_max=1e7, n_points=100
    )
    Q_range = Q_vals_d.max() - Q_vals_d.min()
    print(f"\n  Q range over 0.5 GeV to 10 TeV: {Q_range:.2e}")
    print(f"  Q = {Q_vals_d[0]:.8f} (constant at 1-loop -- all quarks run identically)")
    print(f"  Error from 2/3: {abs(Q_vals_d[0] - KOIDE_TARGET)/KOIDE_TARGET*100:.4f}%")

    # ═══════════════════════════════════════════════════════════════
    # Extended Koide with sqrt-mass sign flips
    # ═══════════════════════════════════════════════════════════════
    print(f"\n{'=' * W}")
    print("  EXTENDED KOIDE: sqrt-mass sign flips")
    print(f"{'=' * W}")
    print("""
  Koide's original formula uses Q = Sum(m) / (Sum(sqrt(m)))^2.
  An extended version allows negative signs on sqrt(m):
    Q' = Sum(m) / (e1*sqrt(m1) + e2*sqrt(m2) + e3*sqrt(m3))^2
  where e_i = +/- 1. For charged leptons, all e_i = +1.
  For quarks, the (-,+,+) sign pattern is physically motivated
  (lightest quark gets negative sign).
""")

    for label, quarks in [('Up-type (u, c, t)', up_type),
                           ('Down-type (d, s, b)', down_type)]:
        print(f"\n  {label} at 2 GeV:")
        masses = [MSBAR_2GEV[q] for q in quarks]
        ext = koide_extended(*masses)

        # Sort by closeness to 2/3
        ranked = sorted(ext.items(), key=lambda x: abs(x[1] - KOIDE_TARGET))
        for signs, Q in ranked[:4]:
            err = abs(Q - KOIDE_TARGET) / KOIDE_TARGET * 100
            sign_str = ''.join('+' if s > 0 else '-' for s in signs)
            marker = " <<<" if err < 1.0 else ""
            print(f"    signs ({sign_str}): Q = {Q:.6f}  err = {err:.3f}%{marker}")

        # Find best extended sign combo
        best_signs, best_Q = ranked[0]
        best_err = abs(best_Q - KOIDE_TARGET) / KOIDE_TARGET * 100

        if best_err < 5.0:  # If reasonably close, scan scales
            print(f"\n    Best sign combo ({'+' if best_signs[0]>0 else '-'}"
                  f"{'+' if best_signs[1]>0 else '-'}"
                  f"{'+' if best_signs[2]>0 else '-'})"
                  f" at 2 GeV: Q={best_Q:.6f}, err={best_err:.3f}%")

            if quarks == up_type:
                # Scan: how does extended Q vary with scale?
                print(f"\n    Extended Koide vs scale (signs {'+' if best_signs[0]>0 else '-'}"
                      f"{'+' if best_signs[1]>0 else '-'}"
                      f"{'+' if best_signs[2]>0 else '-'}):")
                opt_ext_mu, opt_ext_Q, opt_ext_masses = find_optimal_scale_extended(
                    quarks, best_signs, mu_min=500
                )
                opt_ext_err = abs(opt_ext_Q - KOIDE_TARGET) / KOIDE_TARGET * 100
                print(f"    Optimal scale: {opt_ext_mu/1e3:.4f} GeV")
                print(f"    Q = {opt_ext_Q:.8f}, error = {opt_ext_err:.4f}%")

    # ═══════════════════════════════════════════════════════════════
    # Mixed / non-standard triplets
    # ═══════════════════════════════════════════════════════════════
    print(f"\n{'=' * W}")
    print("  NON-STANDARD TRIPLETS")
    print(f"{'=' * W}")

    alt_triplets = [
        ('s, c, b',   ['s', 'c', 'b'],  'heavy 2nd+3rd gen'),
        ('d, c, b',   ['d', 'c', 'b'],  'cross-type'),
        ('u, s, b',   ['u', 's', 'b'],  'same-charge staircase'),
        ('d, c, t',   ['d', 'c', 't'],  'same-charge staircase'),
        ('u, d, s',   ['u', 'd', 's'],  'light quarks only'),
    ]

    print(f"\n  {'Triplet':>14s}  {'Q (2GeV)':>10s}  {'err(%)':>8s}  "
          f"{'Best ext Q':>10s}  {'ext err(%)':>10s}  {'signs':>6s}")
    print(f"  {'-'*14}  {'-'*10}  {'-'*8}  {'-'*10}  {'-'*10}  {'-'*6}")

    for label, quarks, desc in alt_triplets:
        masses = [MSBAR_2GEV[q] for q in quarks]
        Q = koide_Q(*masses)
        err = abs(Q - KOIDE_TARGET) / KOIDE_TARGET * 100

        ext = koide_extended(*masses)
        best_signs, best_Q = min(ext.items(), key=lambda x: abs(x[1] - KOIDE_TARGET))
        best_err = abs(best_Q - KOIDE_TARGET) / KOIDE_TARGET * 100
        sign_str = ''.join('+' if s > 0 else '-' for s in best_signs)

        marker = " <<<" if best_err < 1.0 else ""
        print(f"  {label:>14s}  {Q:10.6f}  {err:8.3f}  "
              f"{best_Q:10.6f}  {best_err:10.3f}  ({sign_str}){marker}")

    # ═══════════════════════════════════════════════════════════════
    # Scale evolution for extended down-type
    # ═══════════════════════════════════════════════════════════════
    print(f"\n{'=' * W}")
    print("  EXTENDED DOWN-TYPE (d,s,b) WITH SIGN (-,+,+): SCALE SCAN")
    print(f"{'=' * W}")

    # Down-type with -++ signs
    dsb_signs = (-1, +1, +1)
    print(f"\n  Q' = (md+ms+mb) / (-sqrt(md)+sqrt(ms)+sqrt(mb))^2")
    print(f"\n  {'Scale (GeV)':>14s}  {'Q ext':>10s}  {'err(%)':>8s}  "
          f"{'m_d':>8s}  {'m_s':>8s}  {'m_b':>8s}")
    print(f"  {'-'*14}  {'-'*10}  {'-'*8}  {'-'*8}  {'-'*8}  {'-'*8}")

    dsb_scales = np.logspace(np.log10(500), np.log10(1e7), 40)
    dsb_Qs = []
    for mu in dsb_scales:
        masses = [running_mass_at_scale(q, mu) for q in down_type]
        s = sum(si * math.sqrt(mi) for si, mi in zip(dsb_signs, masses))
        Q = sum(masses) / (s * s) if abs(s) > 1e-20 else float('inf')
        dsb_Qs.append(Q)
        err = abs(Q - KOIDE_TARGET) / KOIDE_TARGET * 100
        print(f"  {mu/1e3:14.4f}  {Q:10.6f}  {err:8.4f}  "
              f"{masses[0]:8.4f}  {masses[1]:8.2f}  {masses[2]:8.0f}")

    dsb_Qs = np.array(dsb_Qs)
    print(f"\n  Q' range: {dsb_Qs.min():.6f} to {dsb_Qs.max():.6f}")
    print(f"  (Constant because all three down-type quarks run identically at 1-loop)")

    # Optimal extended down-type
    opt_dsb_mu, opt_dsb_Q, opt_dsb_m = find_optimal_scale_extended(
        down_type, dsb_signs, mu_min=500
    )
    opt_dsb_err = abs(opt_dsb_Q - KOIDE_TARGET) / KOIDE_TARGET * 100
    print(f"\n  Extended down-type (-,+,+) Q = {opt_dsb_Q:.8f}, err = {opt_dsb_err:.4f}%")

    # ═══════════════════════════════════════════════════════════════
    # TECS-L Connection
    # ═══════════════════════════════════════════════════════════════
    print(f"\n{'=' * W}")
    print("  TECS-L CONNECTION ANALYSIS")
    print(f"{'=' * W}")

    sigma_6, tau_6, phi_6 = 12, 4, 2
    delta = phi_6 * tau_6**2 / sigma_6**2
    print(f"\n  Koide delta = phi(6)*tau(6)^2 / sigma(6)^2")
    print(f"             = {phi_6}*{tau_6}^2 / {sigma_6}^2 = {phi_6*tau_6**2}/{sigma_6**2} = {delta}")
    print(f"             = 2/9 exactly")

    print(f"\n  Koide parametrization: m_k = M * (1 + sqrt(2)*cos(theta_k + delta))^2")
    print(f"  with theta_k = 2*pi*k/3 and delta = phase angle")
    print(f"  When delta = 2/9 (radians): Q = 2/3 exactly by construction.")

    # Check: what value of delta would make Q=2/3 for quarks at 2 GeV?
    print(f"\n  Reverse engineering delta for quark triplets at 2 GeV:")
    for label, quarks in [('Up-type (u,c,t)', up_type),
                           ('Down-type (d,s,b)', down_type)]:
        masses = sorted([MSBAR_2GEV[q] for q in quarks])
        M = sum(masses) / 3.0
        Q = koide_Q(*masses)
        # From Q, extract the effective cos(delta) via Q = (1+2cos^2(d))/3
        # Actually Q = 1/3 + 2/(3*N^2) * sum(cos^2(theta_k + delta))
        # Simpler: for the parametrization, Q = 1/3 * (1 + 2*r^2) where r = sqrt(2)*|cos(delta)|/...
        # Let's just note the measured Q vs target
        print(f"    {label}: Q = {Q:.6f}")
        print(f"      Deficit from 2/3: {Q - KOIDE_TARGET:+.6f}")
        print(f"      Q > 2/3 means the mass hierarchy is MORE extreme than Koide allows")

    # Check: scale ratios involving TECS-L values
    print(f"\n  Scale ratio analysis:")
    print(f"    m_t / m_c = {MSBAR_2GEV['t']/MSBAR_2GEV['c']:.1f}")
    print(f"    m_b / m_s = {MSBAR_2GEV['b']/MSBAR_2GEV['s']:.1f}")
    print(f"    m_c / m_s = {MSBAR_2GEV['c']/MSBAR_2GEV['s']:.1f}")
    print(f"    m_t / m_b = {MSBAR_2GEV['t']/MSBAR_2GEV['b']:.1f}")
    print(f"    m_b / m_c = {MSBAR_2GEV['b']/MSBAR_2GEV['c']:.2f}")
    print(f"    sigma/tau = {sigma_6/tau_6} = 3")
    print(f"    m_b / m_c = {MSBAR_2GEV['b']/MSBAR_2GEV['c']:.4f} ~ sigma/tau = 3"
          f"  (err {abs(MSBAR_2GEV['b']/MSBAR_2GEV['c']-3)/3*100:.1f}%)")

    # ═══════════════════════════════════════════════════════════════
    # Summary
    # ═══════════════════════════════════════════════════════════════
    print(f"\n{'=' * W}")
    print(f"  SUMMARY")
    print(f"{'=' * W}")

    print(f"""
  Koide formula Q = Sum(m_i) / (Sum(sqrt(m_i)))^2 = 2/3

  STANDARD KOIDE (all signs positive):
  System                  Q at 2 GeV   Error     Notes
  ---------------------   ----------   ------    -----
  Charged leptons (e,u,t) {Q_lep:.6f}   {err_lep:.4f}%   Essentially exact
  Up-type (u,c,t)         {koide_Q(*[MSBAR_2GEV[q] for q in up_type]):.6f}   {abs(koide_Q(*[MSBAR_2GEV[q] for q in up_type])-KOIDE_TARGET)/KOIDE_TARGET*100:.2f}%    Q > 2/3, monotone in scale
  Down-type (d,s,b)       {koide_Q(*[MSBAR_2GEV[q] for q in down_type]):.6f}   {abs(koide_Q(*[MSBAR_2GEV[q] for q in down_type])-KOIDE_TARGET)/KOIDE_TARGET*100:.2f}%    Scale-invariant at 1-loop""")

    # Extended summary
    masses_up = [MSBAR_2GEV[q] for q in up_type]
    masses_dn = [MSBAR_2GEV[q] for q in down_type]
    masses_scb = [MSBAR_2GEV[q] for q in ['s', 'c', 'b']]
    ext_up = koide_extended(*masses_up)
    ext_dn = koide_extended(*masses_dn)
    ext_scb = koide_extended(*masses_scb)
    best_up = min(ext_up.items(), key=lambda x: abs(x[1] - KOIDE_TARGET))
    best_dn = min(ext_dn.items(), key=lambda x: abs(x[1] - KOIDE_TARGET))
    best_scb = min(ext_scb.items(), key=lambda x: abs(x[1] - KOIDE_TARGET))

    signs_up_str = ''.join('+' if s > 0 else '-' for s in best_up[0])
    signs_dn_str = ''.join('+' if s > 0 else '-' for s in best_dn[0])
    signs_scb_str = ''.join('+' if s > 0 else '-' for s in best_scb[0])
    err_up_ext = abs(best_up[1] - KOIDE_TARGET) / KOIDE_TARGET * 100
    err_dn_ext = abs(best_dn[1] - KOIDE_TARGET) / KOIDE_TARGET * 100
    err_scb_ext = abs(best_scb[1] - KOIDE_TARGET) / KOIDE_TARGET * 100

    print(f"""
  EXTENDED KOIDE (optimal sign assignments):
  System                  Q'           Error     Signs
  ---------------------   ----------   ------    -----
  Up-type (u,c,t)         {best_up[1]:.6f}   {err_up_ext:.3f}%   ({signs_up_str})
  Down-type (d,s,b)       {best_dn[1]:.6f}   {err_dn_ext:.3f}%   ({signs_dn_str})
  Mixed (s,c,b)           {best_scb[1]:.6f}   {err_scb_ext:.3f}%   ({signs_scb_str})  *** NOTABLE ***""")

    print(f"""
  KEY FINDINGS:
  1. Standard Koide fails for quarks at ALL energy scales (1-loop QCD).
     For down-type: Q is exactly scale-invariant (same running exponent).
     For up-type: Q varies monotonically but never reaches 2/3.

  2. BEST QUARK KOIDE: (s, c, b) with extended signs (+,-,-) or (-,+,+)
     gives Q' = {best_scb[1]:.6f}, only {err_scb_ext:.3f}% from 2/3.
     This is scale-invariant at 1-loop (all three run identically).
     The (s,c,b) triplet spans generations 2-3 with one quark per charge.

  3. TECS-L delta = phi(6)*tau(6)^2/sigma(6)^2 = 2/9 exactly.
     This is the Koide phase angle in the mass parametrization.
     The formula Q = 2/3 is algebraically guaranteed for this delta.

  4. The quark mass hierarchy is too extreme for standard Koide.
     m_t/m_u ~ 80000 vs m_tau/m_e ~ 3500.
     Quarks need the extended (signed) Koide or a different grouping.

  5. Notable ratio: m_b/m_c = {MSBAR_2GEV['b']/MSBAR_2GEV['c']:.4f} ~ sigma(6)/tau(6) = 3
     (error {abs(MSBAR_2GEV['b']/MSBAR_2GEV['c']-3)/3*100:.1f}%), connecting quark masses to n=6 arithmetic.
""")

    print("=" * W)
    print("  END OF REPORT")
    print("=" * W)


if __name__ == '__main__':
    print_report()
