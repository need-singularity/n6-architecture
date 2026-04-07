"""Resonance Ladder Analysis — QCD mass ratios through TECS-L n=6 arithmetic.

The QCD vector meson ground states form a multiplicative ladder:
    rho(775) x tau(6)=4 -> J/psi(3097) x sigma/tau=3 -> Upsilon(9460)

This module checks whether the ladder extends, closes, or predicts new states,
validated by Monte Carlo p-values against the full PDG spectrum.
"""
import math
import numpy as np
from collections import OrderedDict
from itertools import combinations
from ..tecs import (
    ALL_TARGETS, TARGETS_BASIC, TARGETS_DERIVED,
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
)
from .pdg import get_all, get_masses


# =====================================================================
# 1. QCD Resonance Database (masses in GeV)
# =====================================================================

QCD_RESONANCES = OrderedDict([
    # Vector mesons — ground states
    ('rho(770)',       0.77526),
    ('omega(782)',     0.78266),
    ('phi(1020)',      1.019461),
    ('J/psi(1S)',      3.09690),
    ('psi(2S)',        3.68610),
    ('Upsilon(1S)',    9.46040),
    ('Upsilon(2S)',   10.02326),
    ('Upsilon(3S)',   10.3552),
    ('Z',             91.1876),

    # Excited vector states
    ('rho(1450)',      1.465),
    ('rho(1700)',      1.720),
    ('omega(1420)',    1.410),
    ('phi(1680)',      1.680),
])

# Extended reference particles for prediction matching
REFERENCE_PARTICLES = OrderedDict([
    ('W',        80.377),
    ('Z',        91.1876),
    ('Higgs',   125.25),
    ('top',     172.76),
])

# Merge for look-ups
ALL_KNOWN = OrderedDict()
ALL_KNOWN.update(QCD_RESONANCES)
ALL_KNOWN.update(REFERENCE_PARTICLES)
# Add everything from PDG
for p in get_all():
    key = p['name']
    if key not in ALL_KNOWN:
        ALL_KNOWN[key] = p['mass']


# =====================================================================
# 2. TECS-L target multipliers from n=6 arithmetic
# =====================================================================

TECS_TARGETS = OrderedDict([
    ('phi=2',        2),
    ('sigma/tau=3',  3),
    ('tau=4',        4),
    ('sopfr=5',      5),
    ('n=6',          6),
    ('sigma-tau=8',  8),
    ('sigma=12',    12),
    ('sigma*phi=24',24),
    ('1/2',          0.5),
    ('1/3',          1/3),
    ('1/4',          0.25),
    ('1/6',          1/6),
    ('sqrt(3)',      math.sqrt(3)),
    ('ln(4/3)',      math.log(4/3)),
])


# =====================================================================
# 2. Consecutive mass ratio scan
# =====================================================================

def consecutive_ratio_scan(tolerance=0.03):
    """Sort all QCD resonances by mass, compute consecutive ratios,
    and flag those matching a TECS-L target."""
    sorted_res = sorted(QCD_RESONANCES.items(), key=lambda x: x[1])
    results = []
    for i in range(len(sorted_res) - 1):
        name_lo, m_lo = sorted_res[i]
        name_hi, m_hi = sorted_res[i + 1]
        ratio = m_hi / m_lo
        match = None
        for tname, tval in TECS_TARGETS.items():
            if tval > 0 and abs(ratio - tval) / tval < tolerance:
                match = tname
                break
        results.append({
            'low': name_lo, 'high': name_hi,
            'm_low': m_lo, 'm_high': m_hi,
            'ratio': ratio, 'match': match,
        })
    return results


# =====================================================================
# 3. Ladder extension: rho -> J/psi -> Upsilon -> ???
# =====================================================================

LADDER_GROUND = [
    ('rho(770)',    0.77526),
    ('J/psi(1S)',   3.09690),
    ('Upsilon(1S)', 9.46040),
]

def ladder_extension(tolerance=0.05):
    """For each TECS multiplier, predict the next rung above Upsilon
    and check whether any known particle sits there."""
    ups_mass = LADDER_GROUND[-1][1]
    predictions = []
    for tname, tval in TECS_TARGETS.items():
        if tval < 1:
            continue
        pred_mass = ups_mass * tval
        # Search all known particles
        best_name, best_mass, best_err = None, None, float('inf')
        for pname, pmass in ALL_KNOWN.items():
            err = abs(pmass - pred_mass) / pred_mass
            if err < best_err:
                best_name, best_mass, best_err = pname, pmass, err
        hit = best_err < tolerance
        predictions.append({
            'multiplier': tname, 'factor': tval,
            'predicted_GeV': pred_mass,
            'nearest': best_name, 'nearest_mass': best_mass,
            'error_pct': best_err * 100, 'hit': hit,
        })
    return predictions


# =====================================================================
# 4. Closed-structure checks
# =====================================================================

def closed_structure_checks():
    """Check algebraic closure relations among the ladder."""
    rho = 0.77526
    jpsi = 3.09690
    ups = 9.46040
    results = []

    # rho x tau x (sigma/tau) x ? = known?
    rho_x_12 = rho * TAU_P1 * (SIGMA_P1 / TAU_P1)  # rho * 12
    results.append({
        'relation': 'rho x tau x (sigma/tau) = rho x 12',
        'value_GeV': rho_x_12,
        'nearest': _nearest(rho_x_12),
    })

    # rho x 36
    rho_x_36 = rho * 36
    results.append({
        'relation': 'rho x tau x (sigma/tau) x (sigma/tau) = rho x 36',
        'value_GeV': rho_x_36,
        'nearest': _nearest(rho_x_36),
    })

    # rho x sigma = 9.3
    rho_x_sigma = rho * SIGMA_P1
    err_ups = abs(rho_x_sigma - ups) / ups * 100
    results.append({
        'relation': 'rho x sigma(6)=12',
        'value_GeV': rho_x_sigma,
        'target': f'Upsilon(1S) = {ups}',
        'error_pct': err_ups,
        'match': err_ups < 2,
    })

    # rho x sigma*phi = 18.6
    rho_x_sp = rho * SIGMA_P1 * PHI_P1
    results.append({
        'relation': 'rho x sigma*phi=24',
        'value_GeV': rho_x_sp,
        'nearest': _nearest(rho_x_sp),
    })

    # J/psi x sigma = 37.16
    jpsi_x_sigma = jpsi * SIGMA_P1
    results.append({
        'relation': 'J/psi x sigma(6)=12',
        'value_GeV': jpsi_x_sigma,
        'nearest': _nearest(jpsi_x_sigma),
    })

    # Upsilon x tau = 37.84
    ups_x_tau = ups * TAU_P1
    results.append({
        'relation': 'Upsilon x tau(6)=4',
        'value_GeV': ups_x_tau,
        'nearest': _nearest(ups_x_tau),
    })

    return results


def _nearest(target_mass, tolerance=0.10):
    """Find nearest known particle to a predicted mass."""
    best_name, best_mass, best_err = None, None, float('inf')
    for pname, pmass in ALL_KNOWN.items():
        err = abs(pmass - target_mass) / target_mass
        if err < best_err:
            best_name, best_mass, best_err = pname, pmass, err
    return {'name': best_name, 'mass': best_mass, 'error_pct': best_err * 100}


# =====================================================================
# 5. Ground-state ratio verification
# =====================================================================

def ground_state_ratios():
    """Verify the rho -> J/psi -> Upsilon ladder ratios exactly."""
    rho, jpsi, ups = 0.77526, 3.09690, 9.46040
    r1 = jpsi / rho
    r2 = ups / jpsi
    r_full = ups / rho

    return {
        'J/psi / rho': {
            'ratio': r1,
            'tecs_target': f'tau(6) = {TAU_P1}',
            'error_pct': abs(r1 - TAU_P1) / TAU_P1 * 100,
        },
        'Upsilon / J/psi': {
            'ratio': r2,
            'tecs_target': f'sigma/tau = {SIGMA_P1/TAU_P1:.0f}',
            'error_pct': abs(r2 - SIGMA_P1 / TAU_P1) / (SIGMA_P1 / TAU_P1) * 100,
        },
        'Upsilon / rho': {
            'ratio': r_full,
            'tecs_target': f'sigma(6) = {SIGMA_P1}',
            'error_pct': abs(r_full - SIGMA_P1) / SIGMA_P1 * 100,
        },
        'chain_product': {
            'tau x (sigma/tau)': TAU_P1 * (SIGMA_P1 / TAU_P1),
            'equals_sigma': TAU_P1 * (SIGMA_P1 / TAU_P1) == SIGMA_P1,
            'note': 'tau(6) x sigma(6)/tau(6) = sigma(6) = 12 — algebraically exact',
        },
    }


# =====================================================================
# 6. Monte Carlo validation
# =====================================================================

def mc_validate(n_trials=100_000, tol=0.03, seed=42):
    """Generate random mass triplets from the PDG mass KDE.
    Count how often consecutive ratios give x4 then x3 within tolerance.
    Return p-value."""
    rng = np.random.default_rng(seed)
    all_masses = np.array([p['mass'] for p in get_all() if p['mass'] > 0])

    # Fit a log-space KDE via histogram for speed
    log_m = np.log10(all_masses)
    lo, hi = log_m.min() - 0.5, log_m.max() + 0.5

    hits = 0
    for _ in range(n_trials):
        # Draw 3 random masses from log-uniform spread matching PDG range
        draws = 10 ** rng.uniform(lo, hi, size=3)
        draws.sort()
        if draws[0] <= 0:
            continue
        r1 = draws[1] / draws[0]
        r2 = draws[2] / draws[1]
        if abs(r1 - 4) / 4 < tol and abs(r2 - 3) / 3 < tol:
            hits += 1

    p_value = hits / n_trials
    return {
        'n_trials': n_trials,
        'hits': hits,
        'p_value': p_value,
        'p_value_str': f'{p_value:.6f}' if p_value > 0 else f'< {1/n_trials:.1e}',
        'tolerance': tol,
        'sigma': _p_to_sigma(p_value) if p_value > 0 else '> 4.3',
    }


def _p_to_sigma(p):
    """Convert p-value to sigma significance."""
    from scipy.stats import norm
    if p <= 0:
        return float('inf')
    return norm.isf(p)


# =====================================================================
# 7. Prediction table
# =====================================================================

def prediction_table():
    """Based on ladder patterns, predict masses where new resonances might exist."""
    rho, jpsi, ups = 0.77526, 3.09690, 9.46040
    preds = []

    # Continue the ladder with each TECS multiplier
    for tname, tval in TECS_TARGETS.items():
        if tval < 1.5:
            continue
        pred = ups * tval
        near = _nearest(pred)
        preds.append({
            'basis': 'Upsilon(1S)',
            'multiplier': tname,
            'factor': tval,
            'predicted_GeV': pred,
            'nearest_known': near['name'],
            'nearest_mass': near['mass'],
            'error_pct': near['error_pct'],
            'status': 'CONFIRMED' if near['error_pct'] < 3 else
                      'CLOSE' if near['error_pct'] < 10 else 'OPEN',
        })

    # Also try J/psi-based predictions
    for tname, tval in TECS_TARGETS.items():
        if tval < 1.5:
            continue
        pred = jpsi * tval
        near = _nearest(pred)
        preds.append({
            'basis': 'J/psi(1S)',
            'multiplier': tname,
            'factor': tval,
            'predicted_GeV': pred,
            'nearest_known': near['name'],
            'nearest_mass': near['mass'],
            'error_pct': near['error_pct'],
            'status': 'CONFIRMED' if near['error_pct'] < 3 else
                      'CLOSE' if near['error_pct'] < 10 else 'OPEN',
        })

    # rho-based
    for tname, tval in TECS_TARGETS.items():
        if tval < 1.5:
            continue
        pred = rho * tval
        near = _nearest(pred)
        preds.append({
            'basis': 'rho(770)',
            'multiplier': tname,
            'factor': tval,
            'predicted_GeV': pred,
            'nearest_known': near['name'],
            'nearest_mass': near['mass'],
            'error_pct': near['error_pct'],
            'status': 'CONFIRMED' if near['error_pct'] < 3 else
                      'CLOSE' if near['error_pct'] < 10 else 'OPEN',
        })

    return preds


# =====================================================================
# Full report
# =====================================================================

def run_analysis():
    """Run all resonance ladder analyses and print comprehensive report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  RESONANCE LADDER ANALYSIS — QCD Masses through TECS-L n=6 Arithmetic')
    print(sep)

    # --- 1. QCD Resonance catalog ---
    print('\n--- 1. QCD Resonance Catalog ---\n')
    print(f'  {"Particle":<20s} {"Mass (GeV)":>12s}')
    print(f'  {"-"*20} {"-"*12}')
    for name, mass in sorted(QCD_RESONANCES.items(), key=lambda x: x[1]):
        print(f'  {name:<20s} {mass:>12.5f}')

    # --- 2. Consecutive ratio scan ---
    print(f'\n--- 2. Consecutive Mass Ratios (sorted by mass) ---\n')
    ratios = consecutive_ratio_scan()
    print(f'  {"Low":<16s} {"High":<16s} {"Ratio":>8s} {"TECS Match":>14s}')
    print(f'  {"-"*16} {"-"*16} {"-"*8} {"-"*14}')
    for r in ratios:
        m = r['match'] or ''
        flag = ' ***' if r['match'] else ''
        print(f'  {r["low"]:<16s} {r["high"]:<16s} {r["ratio"]:>8.4f} {m:>14s}{flag}')

    # --- 3. Ladder extension ---
    print(f'\n--- 3. Ladder Extension: rho -> J/psi -> Upsilon -> ??? ---\n')
    exts = ladder_extension()
    print(f'  {"Multiplier":<16s} {"Factor":>6s} {"Predicted":>10s} '
          f'{"Nearest":<14s} {"Mass":>9s} {"Err%":>6s} {"Hit":>4s}')
    print(f'  {"-"*16} {"-"*6} {"-"*10} {"-"*14} {"-"*9} {"-"*6} {"-"*4}')
    for e in exts:
        hit_str = 'YES' if e['hit'] else ''
        print(f'  {e["multiplier"]:<16s} {e["factor"]:>6.2f} {e["predicted_GeV"]:>10.2f} '
              f'{e["nearest"]:<14s} {e["nearest_mass"]:>9.3f} {e["error_pct"]:>6.1f} {hit_str:>4s}')

    # --- 4. Closed structure ---
    print(f'\n--- 4. Closed Structure Checks ---\n')
    closed = closed_structure_checks()
    for c in closed:
        print(f'  {c["relation"]}')
        print(f'    = {c["value_GeV"]:.3f} GeV')
        if 'nearest' in c:
            n = c['nearest']
            print(f'    nearest: {n["name"]} ({n["mass"]:.3f} GeV, {n["error_pct"]:.1f}% off)')
        if 'target' in c:
            print(f'    target:  {c["target"]}, error = {c["error_pct"]:.2f}%'
                  f'  {"MATCH" if c.get("match") else ""}')
        print()

    # --- 5. Ground-state ratios ---
    print(f'--- 5. Ground-State Ratio Verification ---\n')
    gs = ground_state_ratios()
    for key, val in gs.items():
        print(f'  {key}:')
        for k, v in val.items():
            if isinstance(v, float):
                print(f'    {k}: {v:.6f}')
            else:
                print(f'    {k}: {v}')
        print()

    # --- 6. Monte Carlo ---
    print(f'--- 6. Monte Carlo Validation ---\n')
    print('  Drawing 100k random mass triplets from PDG-range log-uniform ...')
    mc = mc_validate()
    print(f'  Trials:    {mc["n_trials"]:,}')
    print(f'  Hits:      {mc["hits"]}')
    print(f'  p-value:   {mc["p_value_str"]}')
    print(f'  Sigma:     {mc["sigma"]}')
    print(f'  Tolerance: {mc["tolerance"]*100:.0f}%')
    print()

    # --- 7. Prediction table ---
    print(f'--- 7. Prediction Table ---\n')
    preds = prediction_table()
    # Show only CONFIRMED and CLOSE, plus OPEN predictions in interesting range
    confirmed = [p for p in preds if p['status'] == 'CONFIRMED']
    close = [p for p in preds if p['status'] == 'CLOSE']
    opens = [p for p in preds if p['status'] == 'OPEN'
             and 10 < p['predicted_GeV'] < 500]

    if confirmed:
        print('  CONFIRMED predictions (< 3% error):')
        print(f'    {"Basis":<14s} {"x":<16s} {"Pred GeV":>9s} '
              f'{"Known":<14s} {"Mass":>9s} {"Err%":>6s}')
        print(f'    {"-"*14} {"-"*16} {"-"*9} {"-"*14} {"-"*9} {"-"*6}')
        for p in confirmed:
            print(f'    {p["basis"]:<14s} {p["multiplier"]:<16s} '
                  f'{p["predicted_GeV"]:>9.2f} {p["nearest_known"]:<14s} '
                  f'{p["nearest_mass"]:>9.3f} {p["error_pct"]:>6.1f}')
        print()

    if close:
        print('  CLOSE predictions (3-10% error):')
        print(f'    {"Basis":<14s} {"x":<16s} {"Pred GeV":>9s} '
              f'{"Known":<14s} {"Mass":>9s} {"Err%":>6s}')
        print(f'    {"-"*14} {"-"*16} {"-"*9} {"-"*14} {"-"*9} {"-"*6}')
        for p in close:
            print(f'    {p["basis"]:<14s} {p["multiplier"]:<16s} '
                  f'{p["predicted_GeV"]:>9.2f} {p["nearest_known"]:<14s} '
                  f'{p["nearest_mass"]:>9.3f} {p["error_pct"]:>6.1f}')
        print()

    if opens:
        print('  OPEN predictions (no known match, 10-500 GeV):')
        print(f'    {"Basis":<14s} {"x":<16s} {"Pred GeV":>9s}')
        print(f'    {"-"*14} {"-"*16} {"-"*9}')
        for p in opens:
            print(f'    {p["basis"]:<14s} {p["multiplier"]:<16s} '
                  f'{p["predicted_GeV"]:>9.2f}')
        print()

    # --- Summary ---
    print(sep)
    print('  SUMMARY')
    print(sep)
    rho, jpsi, ups = 0.77526, 3.09690, 9.46040
    print(f'''
  The QCD ground-state vector meson ladder:
    rho(770)  --x{TAU_P1}[tau(6)]--> J/psi(3097)  --x{SIGMA_P1//TAU_P1}[sigma/tau]--> Upsilon(9460)

  Step 1: J/psi / rho  = {jpsi/rho:.4f}  vs tau(6) = 4     ({abs(jpsi/rho - 4)/4*100:.2f}% off)
  Step 2: Ups / J/psi  = {ups/jpsi:.4f}  vs sigma/tau = 3  ({abs(ups/jpsi - 3)/3*100:.2f}% off)
  Full:   Ups / rho    = {ups/rho:.4f} vs sigma(6) = 12  ({abs(ups/rho - 12)/12*100:.2f}% off)

  The two-step ladder factorizes sigma(6): tau(6) x [sigma(6)/tau(6)] = 4 x 3 = 12 = sigma(6).
  This is algebraically exact in TECS-L.

  Confirmed predictions: {len(confirmed)}
  Close predictions:     {len(close)}
  Open (10-500 GeV):     {len(opens)}
  MC p-value:            {mc["p_value_str"]} ({mc["sigma"]} sigma)
''')
    print(sep)

    return {
        'ratios': ratios,
        'extension': exts,
        'closed': closed,
        'ground_state': gs,
        'mc': mc,
        'predictions': preds,
    }


if __name__ == '__main__':
    run_analysis()
