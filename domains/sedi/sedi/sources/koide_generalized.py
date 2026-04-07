"""Generalized Koide Formula with TECS-L Color Charge Correction.

Standard Koide: Q = (m1+m2+m3) / (sqrt(m1)+sqrt(m2)+sqrt(m3))^2 = 2/3 for leptons.
Quarks deviate: Q_up ~ 0.85, Q_down ~ 0.73 — the color charge breaks the symmetry.

TECS-L provides the arithmetic of n=6:
    sigma(6) = 12, tau(6) = 4, phi(6) = 2, sopfr(6) = 5
    Koide angle delta = phi*tau^2/sigma^2 = 2/9
    Color charges N_c = 3 = sigma/tau

This module systematically searches for a TECS-L color correction that unifies
the Koide formula across leptons and quarks.
"""
import math
import numpy as np
from itertools import combinations
from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
    koide_ratio, koide_check,
)
from .pdg import get_quarks, get_leptons, COUNTS


# ─── Constants from TECS-L n=6 arithmetic ───

S = SIGMA_P1    # 12  (sum of divisors)
T = TAU_P1      # 4   (number of divisors)
P = PHI_P1      # 2   (Euler totient)
F = SOPFR_P1    # 5   (sum of prime factors with multiplicity)
N_c = S // T    # 3   (color charges = sigma/tau)
DELTA = P * T**2 / S**2  # 2/9, Koide angle from TECS-L


# ─── Mass triplets ───

def _get_triplets():
    """Return the three standard mass triplets in GeV."""
    leptons = get_leptons()
    lep = sorted([p['mass'] for p in leptons])

    quarks = get_quarks()
    qm = {q['name']: q['mass'] for q in quarks}
    up_type = sorted([qm['up'], qm['charm'], qm['top']])
    down_type = sorted([qm['down'], qm['strange'], qm['bottom']])

    return {
        'leptons (e,mu,tau)': lep,
        'up-quarks (u,c,t)': up_type,
        'down-quarks (d,s,b)': down_type,
    }


# ─── Generalized Koide functions ───

def standard_koide(masses):
    """Q = (sum m) / (sum sqrt(m))^2."""
    m = np.array(masses)
    return np.sum(m) / np.sum(np.sqrt(m))**2


def generalized_koide_alpha(masses, alpha):
    """Q_alpha = (sum m) / (sum m^alpha)^(1/alpha).
    Standard Koide is alpha=1/2 with an extra square, but we test the
    generalized form: Q = (sum m_i) / (sum m_i^alpha)^(1/alpha).
    """
    m = np.array(masses, dtype=float)
    s_alpha = np.sum(m**alpha)
    if s_alpha <= 0:
        return float('inf')
    return np.sum(m) / s_alpha**(1.0/alpha)


def koide_with_root(masses, root):
    """Q_root = (sum m) / (sum m^(1/root))^root.
    Standard Koide has root=2 (square root). Test root=3 for quarks (cube root).
    """
    m = np.array(masses, dtype=float)
    s = np.sum(m**(1.0/root))
    return np.sum(m) / s**root


def koide_color_power(masses, n_c):
    """Q = (sum m)^(1/N_c) / (sum sqrt(m))^(2/N_c).
    Takes N_c-th root of both numerator and denominator-power.
    """
    m = np.array(masses, dtype=float)
    num = np.sum(m)**(1.0/n_c)
    den = np.sum(np.sqrt(m))**(2.0/n_c)
    return num / den


def koide_mixed_exponent(masses, alpha_num, alpha_den):
    """Q = (sum m^alpha_num) / (sum m^alpha_den)^(alpha_num/alpha_den)."""
    m = np.array(masses, dtype=float)
    num = np.sum(m**alpha_num)
    den = np.sum(m**alpha_den)
    if den <= 0:
        return float('inf')
    return num / den**(alpha_num/alpha_den)


def koide_rescaled(masses, n_c):
    """Standard Koide but rescale result: Q_eff = 1 - (1 - Q)*N_c/3.
    For leptons (colorless, N_c=1): Q_eff = Q.
    For quarks (N_c=3): Q_eff = 1 - (1-Q)*1 = Q. (This just tests the algebra.)
    Actually: Q_eff = Q * (N_c * delta + offset). We search for offset.
    """
    q = standard_koide(masses)
    return q


def koide_log(masses):
    """Log-Koide: Q_log = (sum ln(m)) / (sum sqrt(ln(m)))^2.
    Uses log-masses instead of masses. May linearize hierarchy.
    """
    m = np.array(masses, dtype=float)
    # Shift so all log-masses are positive
    lnm = np.log(m)
    shift = -np.min(lnm) + 1.0  # ensure all positive
    lnm_s = lnm + shift
    return np.sum(lnm_s) / np.sum(np.sqrt(lnm_s))**2


# ─── Attempt catalog ───

def run_all_attempts():
    """Systematically test TECS-L-motivated Koide generalizations."""
    triplets = _get_triplets()
    attempts = []

    # --- Attempt 1: Standard Koide (baseline) ---
    results = {}
    for name, masses in triplets.items():
        results[name] = standard_koide(masses)
    attempts.append({
        'name': 'Standard Koide: Q = sum(m) / (sum sqrt(m))^2',
        'formula': 'sum(m) / (sum m^(1/2))^2',
        'alpha_lepton': '1/2',
        'alpha_quark': '1/2',
        'tecs_params': 'none (baseline)',
        'results': results,
        'target': 2/3,
    })

    # --- Attempt 2: Cube-root Koide for quarks (root=3=N_c=sigma/tau) ---
    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = koide_with_root(masses, 2)  # standard
        else:
            results[name] = koide_with_root(masses, N_c)  # cube root
    attempts.append({
        'name': 'Split Koide: root=2 leptons, root=3=N_c quarks',
        'formula': 'sum(m)/(sum m^(1/N_c))^N_c for quarks',
        'alpha_lepton': '1/2',
        'alpha_quark': '1/3 = tau/sigma',
        'tecs_params': f'N_c={N_c}=sigma/tau',
        'results': results,
        'target': None,  # unknown
    })

    # --- Attempt 3: Uniform root scan — find root where all three agree ---
    best_root = None
    best_spread = float('inf')
    root_scan = np.linspace(1.5, 6.0, 2000)
    for r in root_scan:
        vals = [koide_with_root(m, r) for m in triplets.values()]
        spread = max(vals) - min(vals)
        if spread < best_spread:
            best_spread = spread
            best_root = r

    results = {}
    for name, masses in triplets.items():
        results[name] = koide_with_root(masses, best_root)
    attempts.append({
        'name': f'Uniform root scan: best root={best_root:.4f}',
        'formula': f'sum(m) / (sum m^(1/{best_root:.4f}))^{best_root:.4f}',
        'alpha_lepton': f'1/{best_root:.4f}',
        'alpha_quark': f'1/{best_root:.4f}',
        'tecs_params': f'best-fit root={best_root:.4f}',
        'results': results,
        'target': np.mean(list(results.values())),
    })

    # --- Attempt 4: Color-power Koide ---
    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = koide_color_power(masses, 1)  # no color
        else:
            results[name] = koide_color_power(masses, N_c)
    attempts.append({
        'name': 'Color-power: (sum m)^(1/N_c) / (sum sqrt(m))^(2/N_c)',
        'formula': 'standard^(1/N_c) for quarks, standard for leptons',
        'alpha_lepton': 'standard',
        'alpha_quark': f'N_c={N_c}',
        'tecs_params': f'N_c={N_c}=sigma/tau',
        'results': results,
        'target': 2/3,
    })

    # --- Attempt 5: Generalized alpha = phi/tau for leptons, tau/sigma for quarks ---
    alpha_lep = P / T  # 2/4 = 1/2 (standard!)
    alpha_q = T / S    # 4/12 = 1/3
    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = generalized_koide_alpha(masses, alpha_lep)
        else:
            results[name] = generalized_koide_alpha(masses, alpha_q)
    attempts.append({
        'name': f'TECS-L alpha: phi/tau={alpha_lep} (lep), tau/sigma={alpha_q:.4f} (quark)',
        'formula': 'sum(m) / (sum m^alpha)^(1/alpha)',
        'alpha_lepton': f'phi/tau = {P}/{T} = {alpha_lep}',
        'alpha_quark': f'tau/sigma = {T}/{S} = {alpha_q:.4f}',
        'tecs_params': f'sigma={S}, tau={T}, phi={P}',
        'results': results,
        'target': None,
    })

    # --- Attempt 6: Mixed-exponent: sum(m^a)/sum(m^b)^(a/b) ---
    # Leptons: a=1, b=1/2 (standard). Quarks: a=1, b=1/3
    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = koide_mixed_exponent(masses, 1.0, 0.5)
        else:
            results[name] = koide_mixed_exponent(masses, 1.0, 1.0/N_c)
    attempts.append({
        'name': 'Mixed exponent: sum(m)/sum(m^b)^(1/b), b=1/2 lep, 1/3 quark',
        'formula': 'sum(m^1) / sum(m^(1/N_c))^N_c for quarks',
        'alpha_lepton': 'b=1/2',
        'alpha_quark': f'b=1/{N_c}',
        'tecs_params': f'N_c={N_c}',
        'results': results,
        'target': None,
    })

    # --- Attempt 7: Scan alpha for quarks to match lepton Q=2/3 ---
    lep_masses = list(triplets.values())[0]
    lepton_Q = standard_koide(lep_masses)

    best_alpha_up = None
    best_alpha_down = None
    for name, masses in triplets.items():
        if 'up-quark' in name:
            # Find alpha such that koide_with_root(masses, 1/alpha) ~ 2/3
            alpha_scan = np.linspace(0.05, 0.95, 5000)
            best_err = float('inf')
            for a in alpha_scan:
                m = np.array(masses, dtype=float)
                s = np.sum(m**a)
                q = np.sum(m) / s**(1.0/a)
                err = abs(q - lepton_Q)
                if err < best_err:
                    best_err = err
                    best_alpha_up = a
        elif 'down-quark' in name:
            alpha_scan = np.linspace(0.05, 0.95, 5000)
            best_err = float('inf')
            for a in alpha_scan:
                m = np.array(masses, dtype=float)
                s = np.sum(m**a)
                q = np.sum(m) / s**(1.0/a)
                err = abs(q - lepton_Q)
                if err < best_err:
                    best_err = err
                    best_alpha_down = a

    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = generalized_koide_alpha(masses, 0.5)
        elif 'up-quark' in name:
            results[name] = generalized_koide_alpha(masses, best_alpha_up)
        else:
            results[name] = generalized_koide_alpha(masses, best_alpha_down)
    attempts.append({
        'name': f'Fitted alpha: up={best_alpha_up:.4f}, down={best_alpha_down:.4f} to match 2/3',
        'formula': 'sum(m)/(sum m^alpha)^(1/alpha), alpha fitted per sector',
        'alpha_lepton': '0.5 (standard)',
        'alpha_quark': f'up={best_alpha_up:.4f}, down={best_alpha_down:.4f}',
        'tecs_params': 'fitted (post-hoc)',
        'results': results,
        'target': 2/3,
    })

    # Check if fitted alphas match TECS-L ratios
    tecs_ratios = {
        'tau/sigma': T/S,           # 1/3 = 0.3333
        'phi/sigma': P/S,           # 1/6 = 0.1667
        'phi/tau': P/T,             # 1/2 = 0.5
        'sopfr/sigma': F/S,         # 5/12 = 0.4167
        'phi/sopfr': P/F,           # 2/5 = 0.4
        '(tau-phi)/tau': (T-P)/T,   # 1/2 = 0.5
        'tau/(sigma-tau)': T/(S-T), # 1/2 = 0.5
        'sopfr/tau^2': F/T**2,      # 5/16 = 0.3125
        'N_c/sigma': N_c/S,         # 1/4 = 0.25
        'phi*tau/sigma': P*T/S,     # 2/3 = 0.6667
    }

    # --- Attempt 8: Root = sigma/sopfr = 12/5 = 2.4 ---
    r_tecs = S / F  # 12/5 = 2.4
    results = {}
    for name, masses in triplets.items():
        results[name] = koide_with_root(masses, r_tecs)
    attempts.append({
        'name': f'Uniform root = sigma/sopfr = {S}/{F} = {r_tecs}',
        'formula': f'sum(m) / (sum m^(1/{r_tecs}))^{r_tecs}',
        'alpha_lepton': f'1/{r_tecs} = {1/r_tecs:.4f}',
        'alpha_quark': f'1/{r_tecs} = {1/r_tecs:.4f}',
        'tecs_params': f'sigma/sopfr = {S}/{F} = {r_tecs}',
        'results': results,
        'target': None,
    })

    # --- Attempt 9: Root = sopfr/phi = 5/2 = 2.5 ---
    r_tecs2 = F / P  # 5/2 = 2.5
    results = {}
    for name, masses in triplets.items():
        results[name] = koide_with_root(masses, r_tecs2)
    attempts.append({
        'name': f'Uniform root = sopfr/phi = {F}/{P} = {r_tecs2}',
        'formula': f'sum(m) / (sum m^(1/{r_tecs2}))^{r_tecs2}',
        'alpha_lepton': f'1/{r_tecs2} = {1/r_tecs2:.4f}',
        'alpha_quark': f'1/{r_tecs2} = {1/r_tecs2:.4f}',
        'tecs_params': f'sopfr/phi = {F}/{P} = {r_tecs2}',
        'results': results,
        'target': None,
    })

    # --- Attempt 10: Log-Koide ---
    results = {}
    for name, masses in triplets.items():
        results[name] = koide_log(masses)
    attempts.append({
        'name': 'Log-Koide: Q = sum(ln m) / (sum sqrt(ln m))^2 (shifted)',
        'formula': 'Standard Koide on log-masses (shifted positive)',
        'alpha_lepton': 'log-space',
        'alpha_quark': 'log-space',
        'tecs_params': 'none (log transform)',
        'results': results,
        'target': 2/3,
    })

    # --- Attempt 11: Split root — root=2 for leptons, root=sigma/tau=3 for quarks,
    #     but with a multiplicative correction factor delta = 2/9 ---
    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = standard_koide(masses)
        else:
            q3 = koide_with_root(masses, N_c)
            results[name] = q3 * (1 - DELTA)  # correct by delta=2/9
    attempts.append({
        'name': f'Cube-root + delta correction: Q3*(1-delta), delta={DELTA:.4f}=2/9',
        'formula': 'sum(m)/(sum m^(1/3))^3 * (1 - 2/9) for quarks',
        'alpha_lepton': 'standard',
        'alpha_quark': f'root={N_c}, correction=1-{DELTA:.4f}',
        'tecs_params': f'N_c={N_c}, delta={DELTA}=phi*tau^2/sigma^2',
        'results': results,
        'target': 2/3,
    })

    # --- Attempt 12: Additive correction — Q + (1-Q)*color_factor ---
    # If Q_lepton = 2/3 exactly, then (1-Q)=1/3 for leptons
    # For quarks: Q_quark + (1-Q_quark)*f = 2/3 => f = (2/3 - Q)/(1-Q)
    # Check if f has a TECS-L form
    results = {}
    correction_factors = {}
    for name, masses in triplets.items():
        q = standard_koide(masses)
        if 'lepton' in name:
            results[name] = q
            correction_factors[name] = 0.0
        else:
            f_needed = (2/3 - q) / (1 - q) if abs(1 - q) > 1e-12 else float('inf')
            results[name] = q + (1 - q) * f_needed  # should be 2/3 by construction
            correction_factors[name] = f_needed
    attempts.append({
        'name': 'Additive: Q + (1-Q)*f = 2/3; solve for f',
        'formula': 'Q + (1-Q)*f where f = (2/3 - Q)/(1-Q)',
        'alpha_lepton': 'standard, f=0',
        'alpha_quark': f'f_up={correction_factors.get("up-quarks (u,c,t)", "?"):.6f}, '
                       f'f_down={correction_factors.get("down-quarks (d,s,b)", "?"):.6f}',
        'tecs_params': 'reverse-engineered f — checking if TECS-L',
        'results': results,
        'target': 2/3,
        'extra': correction_factors,
    })

    # --- Attempt 13: Full 2D scan — different root for up-type and down-type ---
    # Find roots r_u, r_d such that koide_with_root gives the SAME value as lepton Q
    best_ru = None
    best_rd = None
    for name, masses in triplets.items():
        if 'up-quark' in name:
            rscan = np.linspace(1.01, 10.0, 5000)
            best_err = float('inf')
            for r in rscan:
                q = koide_with_root(masses, r)
                err = abs(q - lepton_Q)
                if err < best_err:
                    best_err = err
                    best_ru = r
        elif 'down-quark' in name:
            rscan = np.linspace(1.01, 10.0, 5000)
            best_err = float('inf')
            for r in rscan:
                q = koide_with_root(masses, r)
                err = abs(q - lepton_Q)
                if err < best_err:
                    best_err = err
                    best_rd = r

    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            results[name] = koide_with_root(masses, 2)
        elif 'up-quark' in name:
            results[name] = koide_with_root(masses, best_ru)
        else:
            results[name] = koide_with_root(masses, best_rd)
    attempts.append({
        'name': f'Fitted roots: lep=2, up={best_ru:.4f}, down={best_rd:.4f}',
        'formula': 'sum(m)/(sum m^(1/r))^r, r fitted per sector',
        'alpha_lepton': 'r=2 (standard)',
        'alpha_quark': f'r_up={best_ru:.4f}, r_down={best_rd:.4f}',
        'tecs_params': 'fitted — checking TECS-L match',
        'results': results,
        'target': 2/3,
    })

    # --- Attempt 14: Universal formula with color degeneracy ---
    # Q_univ = (sum m_i^(1/d)) / (N * sum m_i^(1/d) / N)  ... trivial
    # Better: Q = (sum m) / (sum m^(1/(1+c)))^(1+c) where c=0 for leptons, c=1/(N_c-1)=1/2 quarks
    results = {}
    for name, masses in triplets.items():
        if 'lepton' in name:
            c = 0
        else:
            c = 1.0 / (N_c - 1)  # 1/2
        root = 2 + c  # 2 for leptons, 2.5 for quarks
        results[name] = koide_with_root(masses, root)
    attempts.append({
        'name': f'Color-shifted root: r=2+1/(N_c-1)=2.5 for quarks',
        'formula': 'sum(m)/(sum m^(1/r))^r, r=2 lep, r=2.5 quark',
        'alpha_lepton': 'r=2',
        'alpha_quark': f'r=2+1/(N_c-1) = 2.5',
        'tecs_params': f'N_c={N_c}, 1/(N_c-1)=0.5',
        'results': results,
        'target': None,
    })

    # --- Attempt 15: Weighted Koide with mass hierarchy factor ---
    # Q_w = (sum m_i * w_i) / (sum sqrt(m_i * w_i))^2
    # where w_i = (m_i / m_max)^(1/N_c - 1/2) for quarks, w_i=1 for leptons
    results = {}
    for name, masses in triplets.items():
        m = np.array(masses, dtype=float)
        if 'lepton' in name:
            results[name] = standard_koide(masses)
        else:
            exp = 1.0/N_c - 0.5  # 1/3 - 1/2 = -1/6
            w = (m / m[-1])**exp
            mw = m * w
            results[name] = np.sum(mw) / np.sum(np.sqrt(mw))**2
    attempts.append({
        'name': 'Weighted Koide: w_i = (m_i/m_max)^(1/N_c - 1/2)',
        'formula': 'sum(m*w)/(sum sqrt(m*w))^2, w=(m/mmax)^(-1/6)',
        'alpha_lepton': 'standard (w=1)',
        'alpha_quark': 'w = (m/mmax)^(1/3-1/2) = (m/mmax)^(-1/6)',
        'tecs_params': f'exponent = 1/N_c - 1/2 = 1/{N_c}-1/2 = -1/6',
        'results': results,
        'target': 2/3,
    })

    return attempts, triplets, tecs_ratios, correction_factors, {
        'best_alpha_up': best_alpha_up,
        'best_alpha_down': best_alpha_down,
        'best_root_uniform': best_root,
        'best_root_up': best_ru,
        'best_root_down': best_rd,
    }


# ─── Monte Carlo validation ───

def monte_carlo_test(formula_fn, target, n_trials=1000, mass_range=(1e-4, 200)):
    """Test how often random mass triples hit the target value.

    Returns (n_hits, p_value, hit_fraction).
    """
    rng = np.random.default_rng(42)
    tolerance = 0.01  # 1% match
    hits = 0
    for _ in range(n_trials):
        # Log-uniform masses spanning particle physics range
        log_masses = rng.uniform(np.log(mass_range[0]), np.log(mass_range[1]), 3)
        masses = sorted(np.exp(log_masses))
        try:
            q = formula_fn(masses)
            if abs(q - target) / abs(target) < tolerance:
                hits += 1
        except Exception:
            pass
    return hits, hits / n_trials


def texas_sharpshooter_test(n_attempts, best_p_value):
    """Bonferroni-corrected p-value for look-elsewhere effect."""
    return min(1.0, best_p_value * n_attempts)


# ─── Neutrino mass prediction ───

def predict_neutrino_masses(target_Q=2/3):
    """If generalized Koide holds for neutrinos, predict mass ratios.

    From oscillation data: dm21^2 ~ 7.5e-5 eV^2, dm31^2 ~ 2.5e-3 eV^2.
    Assume normal ordering: m1 < m2 < m3.
    Lightest mass m1 is unknown. Scan m1 and check Koide.
    """
    dm21_sq = 7.53e-5  # eV^2
    dm31_sq = 2.453e-3  # eV^2 (normal ordering)

    results = []
    m1_scan = np.logspace(-4, -1, 10000)  # eV
    for m1 in m1_scan:
        m2 = math.sqrt(m1**2 + dm21_sq)
        m3 = math.sqrt(m1**2 + dm31_sq)
        q = standard_koide([m1, m2, m3])
        results.append((m1, m2, m3, q))

    # Find m1 that gives Q closest to target
    best = min(results, key=lambda x: abs(x[3] - target_Q))
    return {
        'm1_eV': best[0],
        'm2_eV': best[1],
        'm3_eV': best[2],
        'Q': best[3],
        'sum_eV': best[0] + best[1] + best[2],
        'error_from_target': abs(best[3] - target_Q) / target_Q * 100,
    }


# ─── Report ───

def run_report():
    """Run full analysis and print comprehensive report."""
    print("=" * 80)
    print("  GENERALIZED KOIDE FORMULA WITH TECS-L COLOR CHARGE CORRECTION")
    print("=" * 80)
    print()

    # TECS-L constants
    print("TECS-L Arithmetic (n=6):")
    print(f"  sigma(6) = {S}")
    print(f"  tau(6)   = {T}")
    print(f"  phi(6)   = {P}")
    print(f"  sopfr(6) = {F}")
    print(f"  N_c = sigma/tau = {N_c}")
    print(f"  delta = phi*tau^2/sigma^2 = {DELTA:.6f} = 2/9")
    print()

    # Mass triplets
    triplets = _get_triplets()
    print("Mass Triplets (GeV):")
    for name, masses in triplets.items():
        print(f"  {name}: {[f'{m:.6f}' for m in masses]}")
    print()

    # Standard Koide baseline
    print("-" * 80)
    print("BASELINE: Standard Koide Q = sum(m) / (sum sqrt(m))^2")
    print("-" * 80)
    for name, masses in triplets.items():
        q = standard_koide(masses)
        err = abs(q - 2/3) / (2/3) * 100
        print(f"  {name:30s}: Q = {q:.6f}  (target 2/3 = {2/3:.6f}, error = {err:.2f}%)")
    print()

    # Run all attempts
    attempts, _, tecs_ratios, correction_factors, fitted = run_all_attempts()

    # Score each attempt
    scored = []
    for att in attempts:
        vals = list(att['results'].values())
        spread = max(vals) - min(vals)
        mean_val = np.mean(vals)
        target = att.get('target', mean_val)
        if target is None:
            target = mean_val
        mean_err = np.mean([abs(v - target) for v in vals])
        # Penalize if target is not 2/3 (ad-hoc)
        target_penalty = abs(target - 2/3) if target != 2/3 else 0
        score = spread + mean_err + target_penalty * 0.5
        scored.append((score, spread, mean_err, att))

    scored.sort(key=lambda x: x[0])

    print("=" * 80)
    print("  ALL ATTEMPTS — RANKED BY UNIFICATION QUALITY")
    print("  (lower score = better: spread + mean_error + target_penalty)")
    print("=" * 80)
    print()

    for rank, (score, spread, mean_err, att) in enumerate(scored, 1):
        print(f"{'─' * 78}")
        print(f"  #{rank}  {att['name']}")
        print(f"  Formula: {att['formula']}")
        print(f"  TECS-L params: {att['tecs_params']}")
        target = att.get('target')
        if target is not None:
            print(f"  Target: {target:.6f}")
        print(f"  Results:")
        for name, val in att['results'].items():
            t = target if target else np.mean(list(att['results'].values()))
            err = abs(val - t) / abs(t) * 100 if t != 0 else float('inf')
            print(f"    {name:30s}: {val:.6f}  (err from target: {err:.2f}%)")
        print(f"  Spread (max-min): {spread:.6f}")
        print(f"  Score: {score:.6f}")
        print()

    # Analyze the additive correction factors
    print("=" * 80)
    print("  CORRECTION FACTOR ANALYSIS")
    print("=" * 80)
    print()
    print("  Additive correction Q + (1-Q)*f = 2/3:")
    for name, f in correction_factors.items():
        if f != 0:
            print(f"    {name}: f = {f:.6f}")
            # Check against TECS-L ratios
            print(f"    TECS-L ratio matches:")
            for rname, rval in tecs_ratios.items():
                if abs(f - rval) / abs(rval) < 0.05:
                    print(f"      ** MATCH: {rname} = {rval:.6f} (err {abs(f-rval)/abs(rval)*100:.2f}%)")
                elif abs(f - rval) / abs(rval) < 0.15:
                    print(f"      ~  NEAR:  {rname} = {rval:.6f} (err {abs(f-rval)/abs(rval)*100:.2f}%)")
    print()

    # Analyze fitted parameters
    print("  Fitted exponents that achieve Q=2/3:")
    print(f"    alpha_up   = {fitted['best_alpha_up']:.6f}")
    print(f"    alpha_down = {fitted['best_alpha_down']:.6f}")
    print(f"    root_up    = {fitted['best_root_up']:.4f}")
    print(f"    root_down  = {fitted['best_root_down']:.4f}")
    print()
    print("  TECS-L ratio comparison:")
    for rname, rval in sorted(tecs_ratios.items(), key=lambda x: x[1]):
        up_match = abs(fitted['best_alpha_up'] - rval) / abs(rval) * 100
        dn_match = abs(fitted['best_alpha_down'] - rval) / abs(rval) * 100
        ru_match = abs(fitted['best_root_up'] - 1/rval) / abs(1/rval) * 100 if rval != 0 else float('inf')
        rd_match = abs(fitted['best_root_down'] - 1/rval) / abs(1/rval) * 100 if rval != 0 else float('inf')
        markers = []
        if up_match < 5:
            markers.append(f"alpha_up matches ({up_match:.1f}%)")
        if dn_match < 5:
            markers.append(f"alpha_down matches ({dn_match:.1f}%)")
        if ru_match < 5:
            markers.append(f"1/root_up matches ({ru_match:.1f}%)")
        if rd_match < 5:
            markers.append(f"1/root_down matches ({rd_match:.1f}%)")
        marker_str = " << " + ", ".join(markers) if markers else ""
        print(f"    {rname:25s} = {rval:.6f}{marker_str}")
    print()

    # Monte Carlo on the best non-trivial formula
    print("=" * 80)
    print("  MONTE CARLO VALIDATION")
    print("=" * 80)
    print()

    # Test standard Koide
    hits_std, p_std = monte_carlo_test(standard_koide, 2/3, n_trials=1000)
    print(f"  Standard Koide (target=2/3): {hits_std}/1000 random triples match")
    print(f"    => p-value (single test): {p_std:.4f}")

    # Test cube-root Koide
    def cube_root_koide(m):
        return koide_with_root(m, N_c)

    # Use the mean of the cube-root results as target
    cr_vals = [koide_with_root(m, N_c) for m in triplets.values()]
    cr_target = np.mean(cr_vals)
    hits_cr, p_cr = monte_carlo_test(cube_root_koide, cr_target, n_trials=1000)
    print(f"  Cube-root Koide (target={cr_target:.4f}): {hits_cr}/1000 match")
    print(f"    => p-value (single test): {p_cr:.4f}")

    # Test best uniform root
    def best_root_koide(m):
        return koide_with_root(m, fitted['best_root_uniform'])

    br_vals = [koide_with_root(m, fitted['best_root_uniform']) for m in triplets.values()]
    br_target = np.mean(br_vals)
    hits_br, p_br = monte_carlo_test(best_root_koide, br_target, n_trials=1000)
    print(f"  Best uniform root={fitted['best_root_uniform']:.4f} (target={br_target:.4f}): {hits_br}/1000 match")
    print(f"    => p-value (single test): {p_br:.4f}")

    # Texas sharpshooter
    n_total = len(attempts)
    best_p = min(p_std, p_cr, p_br) if min(p_std, p_cr, p_br) > 0 else 1.0/1000
    ts_p = texas_sharpshooter_test(n_total, best_p)
    print()
    print(f"  Texas Sharpshooter correction ({n_total} attempts):")
    print(f"    Best raw p-value: {best_p:.4f}")
    print(f"    Bonferroni-corrected: {ts_p:.4f}")
    print()

    # Neutrino prediction
    print("=" * 80)
    print("  NEUTRINO MASS PREDICTION")
    print("=" * 80)
    print()
    print("  If Koide holds for neutrinos (normal ordering):")
    print(f"  dm21^2 = 7.53e-5 eV^2, dm31^2 = 2.453e-3 eV^2")
    print()

    nu = predict_neutrino_masses(target_Q=2/3)
    print(f"  Target Q = 2/3:")
    print(f"    m1 = {nu['m1_eV']:.6f} eV")
    print(f"    m2 = {nu['m2_eV']:.6f} eV")
    print(f"    m3 = {nu['m3_eV']:.6f} eV")
    print(f"    Q  = {nu['Q']:.6f} (error: {nu['error_from_target']:.2f}%)")
    print(f"    sum = {nu['sum_eV']:.6f} eV")
    print(f"    Cosmological bound: sum < 0.12 eV {'PASS' if nu['sum_eV'] < 0.12 else 'FAIL'}")
    print()

    # Also try Q = delta = 2/9 as neutrino target (lower bound might work better)
    nu_delta = predict_neutrino_masses(target_Q=DELTA)
    print(f"  Target Q = delta = 2/9 = {DELTA:.6f}:")
    print(f"    m1 = {nu_delta['m1_eV']:.6f} eV")
    print(f"    m2 = {nu_delta['m2_eV']:.6f} eV")
    print(f"    m3 = {nu_delta['m3_eV']:.6f} eV")
    print(f"    Q  = {nu_delta['Q']:.6f} (error: {nu_delta['error_from_target']:.2f}%)")
    print(f"    sum = {nu_delta['sum_eV']:.6f} eV")
    print(f"    Cosmological bound: sum < 0.12 eV {'PASS' if nu_delta['sum_eV'] < 0.12 else 'FAIL'}")
    print()

    # Summary
    print("=" * 80)
    print("  SUMMARY")
    print("=" * 80)
    print()
    top = scored[0]
    print(f"  Best unification attempt: #{1}")
    print(f"    {top[3]['name']}")
    print(f"    Score: {top[0]:.6f}")
    print(f"    Spread: {top[1]:.6f}")
    print()
    print("  Key findings:")
    print(f"    1. Standard Koide is perfect for leptons (Q={standard_koide(list(triplets.values())[0]):.6f})")
    print(f"    2. Quarks break Koide: up-type Q={standard_koide(list(triplets.values())[1]):.6f}, "
          f"down-type Q={standard_koide(list(triplets.values())[2]):.6f}")
    print(f"    3. Color correction factor f needed:")
    for name, f in correction_factors.items():
        if f != 0:
            print(f"       {name}: f = {f:.6f}")
    print(f"    4. Fitted alpha for quarks: up={fitted['best_alpha_up']:.4f}, down={fitted['best_alpha_down']:.4f}")
    print(f"       (leptons use alpha=0.5 = phi/tau)")

    # Check if any TECS-L ratio matches
    any_match = False
    for rname, rval in tecs_ratios.items():
        for pname, pval in [('alpha_up', fitted['best_alpha_up']),
                            ('alpha_down', fitted['best_alpha_down'])]:
            if abs(pval - rval) / abs(rval) < 0.03:
                print(f"    5. ** {pname} = {pval:.4f} matches TECS-L {rname} = {rval:.4f} ({abs(pval-rval)/abs(rval)*100:.1f}% error)")
                any_match = True

    if not any_match:
        print(f"    5. No exact TECS-L ratio match found for fitted quark exponents.")
        print(f"       Closest TECS-L ratios to alpha_up={fitted['best_alpha_up']:.4f}:")
        diffs = [(rname, rval, abs(fitted['best_alpha_up'] - rval))
                 for rname, rval in tecs_ratios.items()]
        diffs.sort(key=lambda x: x[2])
        for rname, rval, d in diffs[:3]:
            print(f"         {rname} = {rval:.4f} (diff = {d:.4f})")

    print()
    print("=" * 80)


if __name__ == '__main__':
    run_report()
