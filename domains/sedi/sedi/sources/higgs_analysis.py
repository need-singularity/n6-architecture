"""Comprehensive Higgs Boson Analysis through TECS-L n=6 Framework.

Higgs boson mass: 125.25 +/- 0.17 GeV
Higgs VEV: v = 246.22 GeV (electroweak symmetry breaking scale)

This module analyses the Higgs sector through five lenses:
  1. VEV = phi(P3) + P1 = 246  (0.09% from observed)
  2. Systematic Higgs mass formula search with Texas Sharpshooter tests
  3. Branching ratio matches to n=6 fractions
  4. Electroweak scale relations
  5. Resonance ladder extension to the Higgs

Data sources: PDG 2024, CMS/ATLAS Run-2 combined.
"""

from __future__ import annotations

import itertools
import math
import random
from collections import OrderedDict
from dataclasses import dataclass
from typing import Dict, List, Optional, Tuple

import numpy as np

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
    SIGMA_P2, TAU_P2, PHI_P2,
    TAU_P3, PHI_P3,
)


# =====================================================================
# Physical constants
# =====================================================================

HIGGS_MASS = 125.25       # GeV, PDG 2024
HIGGS_MASS_ERR = 0.17     # GeV
HIGGS_VEV = 246.22        # GeV
HIGGS_VEV_ERR = 0.01      # GeV (Fermi constant precision)
M_W = 80.377              # GeV
M_Z = 91.1876             # GeV
M_TOP = 172.76            # GeV
RHO_MASS = 0.77526        # GeV
JPSI_MASS = 3.09690       # GeV
UPSILON_MASS = 9.46040    # GeV

# PDG Higgs branching ratios (percent)
HIGGS_BR = OrderedDict([
    ('H->bb',    (58.2,  1.5)),
    ('H->WW*',   (21.4,  0.9)),
    ('H->gg',    (8.18,  0.5)),
    ('H->tautau',(6.27,  0.35)),
    ('H->cc',    (2.89,  0.12)),
    ('H->ZZ*',   (2.62,  0.11)),
    ('H->gamgam',(0.227, 0.007)),
    ('H->Zgam',  (0.154, 0.014)),
    ('H->mumu',  (0.022, 0.001)),
])

# Nuclear magic numbers
MAGIC_NUMBERS = [2, 8, 20, 28, 50, 82, 126]


# =====================================================================
# TECS-L constant pool for expression search
# =====================================================================

TECS_POOL = OrderedDict([
    ('sigma',    SIGMA_P1),      # 12
    ('tau',      TAU_P1),        # 4
    ('phi',      PHI_P1),        # 2
    ('sopfr',    SOPFR_P1),      # 5
    ('n',        P1),            # 6
    ('omega',    OMEGA_P1),      # 2
    ('P1',       P1),            # 6
    ('P2',       P2),            # 28
    ('P3',       P3),            # 496
    ('tau(P2)',   TAU_P2),       # 6
    ('tau(P3)',   TAU_P3),       # 10
    ('phi(P2)',   PHI_P2),       # 12
    ('phi(P3)',   PHI_P3),       # 240
    ('sigma(P2)', SIGMA_P2),    # 56
])

# Deduplicated values for MC null model
_POOL_VALUES = sorted(set(TECS_POOL.values()))


def _pct_err(predicted, observed):
    """Percentage error."""
    return abs(predicted - observed) / observed * 100


def _p_to_sigma(p):
    """Convert p-value to sigma significance."""
    from scipy.stats import norm
    if p <= 0:
        return float('inf')
    return norm.isf(p)


# =====================================================================
# PART 1: Higgs VEV = phi(P3) + P1 = 246
# =====================================================================

def vev_analysis():
    """Check VEV decompositions using TECS-L constants."""
    results = []

    # Primary: phi(496) + P1 = 240 + 6 = 246
    pred = PHI_P3 + P1
    results.append({
        'formula': 'phi(P3) + P1 = 240 + 6',
        'predicted': pred,
        'observed': HIGGS_VEV,
        'error_pct': _pct_err(pred, HIGGS_VEV),
        'note': 'phi(496) = 240 = number of E8 root vectors',
    })

    # Alternative: sigma*phi * tau(P3) + P1 = 24*10 + 6 = 246
    pred2 = SIGMA_P1 * PHI_P1 * TAU_P3 + P1
    results.append({
        'formula': 'sigma*phi * tau(P3) + P1 = 24*10 + 6',
        'predicted': pred2,
        'observed': HIGGS_VEV,
        'error_pct': _pct_err(pred2, HIGGS_VEV),
    })

    # 246 = 2 * 123; is 123 TECS-meaningful?
    note_123 = []
    if 123 == SIGMA_P1 * TAU_P3 + SIGMA_P1 / TAU_P1:
        note_123.append('sigma * tau(P3) + sigma/tau = 120 + 3 = 123')
    # Check: sigma^2 + sigma/tau = 144 + 3 = 147 (no)
    # sigma*tau(P3) + sigma/tau = 120+3 = 123
    pred3 = SIGMA_P1 * TAU_P3 + SIGMA_P1 // TAU_P1
    if pred3 == 123:
        note_123.append(f'sigma*tau(P3) + sigma/tau = {SIGMA_P1}*{TAU_P3} + {SIGMA_P1//TAU_P1} = 123')
    results.append({
        'formula': '246 = phi * 123',
        'predicted': PHI_P1 * 123,
        'observed': HIGGS_VEV,
        'error_pct': _pct_err(PHI_P1 * 123, HIGGS_VEV),
        'note': f'123 decomposition: {note_123 if note_123 else "no clean decomposition found"}',
    })

    # phi(P3) = sigma * (sigma*phi - tau) = 12 * 20 = 240
    inner = SIGMA_P1 * PHI_P1 - TAU_P1  # 24 - 4 = 20
    results.append({
        'formula': f'phi(P3) = sigma * (sigma*phi - tau) = {SIGMA_P1} * {inner}',
        'predicted': SIGMA_P1 * inner,
        'observed': PHI_P3,
        'error_pct': _pct_err(SIGMA_P1 * inner, PHI_P3),
        'note': 'Exact identity: sigma * 20 = 240 = phi(496)',
    })

    return results


def vev_mc_sharpshooter(n_trials=1000, tol_pct=0.5, seed=42):
    """Texas Sharpshooter test for VEV = 246.

    Generate random expressions from TECS pool using {+,-,*,/}
    with 2-3 operands. Count how many land within tol of 246.
    """
    rng = np.random.default_rng(seed)
    vals = list(_POOL_VALUES)
    ops = ['+', '-', '*']  # skip / to avoid division by zero issues

    target = 246
    hits = 0
    total = 0

    # Systematic: all pairs with all ops
    for a in vals:
        for b in vals:
            for op in ['+', '-', '*']:
                total += 1
                if op == '+':
                    r = a + b
                elif op == '-':
                    r = a - b
                else:
                    r = a * b
                if r > 0 and _pct_err(r, target) <= tol_pct:
                    hits += 1

            # Division (avoid div by zero)
            if b != 0:
                total += 1
                r = a / b
                if r > 0 and _pct_err(r, target) <= tol_pct:
                    hits += 1

    # Also triples: a op1 b op2 c
    for a in vals:
        for b in vals:
            for c in vals:
                for op1 in ops:
                    for op2 in ops:
                        total += 1
                        if op1 == '+':
                            ab = a + b
                        elif op1 == '-':
                            ab = a - b
                        else:
                            ab = a * b
                        if op2 == '+':
                            r = ab + c
                        elif op2 == '-':
                            r = ab - c
                        else:
                            r = ab * c
                        if r > 0 and _pct_err(r, target) <= tol_pct:
                            hits += 1

    p_value = hits / total if total > 0 else 1.0
    return {
        'target': target,
        'tolerance_pct': tol_pct,
        'total_expressions': total,
        'hits': hits,
        'p_value': p_value,
        'sigma': _p_to_sigma(p_value) if p_value > 0 else '> 5',
    }


# =====================================================================
# PART 2: Higgs Mass Formula Search
# =====================================================================

@dataclass
class FormulaMatch:
    formula: str
    predicted: float
    error_pct: float
    n_ops: int       # number of operations (simplicity metric)
    p_value: float = 0.0

    def __lt__(self, other):
        return self.error_pct < other.error_pct


def higgs_mass_search(target=HIGGS_MASS, tol_pct=1.0, max_ops=4):
    """Systematically search expressions using n=6 constants for Higgs mass.

    Strategy: build expressions with 2-4 operands from the basic TECS pool
    {sigma=12, tau=4, phi=2, sopfr=5, n=6, P2=28} using {+,-,*,/,**}.
    """
    # Use a smaller pool for the systematic search to keep combinatorics tractable
    basic = OrderedDict([
        ('sigma', SIGMA_P1),    # 12
        ('tau',   TAU_P1),      # 4
        ('phi',   PHI_P1),      # 2
        ('sopfr', SOPFR_P1),    # 5
        ('n',     P1),          # 6
        ('P2',    P2),          # 28
    ])

    # Also include sigma^2 = 144 as a derived value (very common in matches)
    extended = OrderedDict(list(basic.items()) + [
        ('sigma^2', SIGMA_P1**2),   # 144
        ('tau(P3)', TAU_P3),        # 10
        ('phi(P3)', PHI_P3),        # 240
        ('phi(P2)', PHI_P2),        # 12
        ('sigma(P2)', SIGMA_P2),    # 56
    ])

    matches = []
    seen = set()

    # --- 2-operand expressions: a OP b ---
    ops2 = {
        '+': lambda a, b: a + b,
        '-': lambda a, b: a - b,
        '*': lambda a, b: a * b,
    }

    for (na, va) in extended.items():
        for (nb, vb) in extended.items():
            for opname, opfn in ops2.items():
                r = opfn(va, vb)
                if r > 0 and _pct_err(r, target) <= tol_pct:
                    fstr = f'{na} {opname} {nb}'
                    fval = f'{va} {opname} {vb} = {r}'
                    if fval not in seen:
                        seen.add(fval)
                        matches.append(FormulaMatch(
                            formula=f'{fstr} = {fval}',
                            predicted=r, error_pct=_pct_err(r, target),
                            n_ops=1,
                        ))
            # Division
            if vb != 0:
                r = va / vb
                if r > 0 and _pct_err(r, target) <= tol_pct:
                    fstr = f'{na} / {nb}'
                    fval = f'{va} / {vb} = {r}'
                    if fval not in seen:
                        seen.add(fval)
                        matches.append(FormulaMatch(
                            formula=f'{fstr} = {fval}',
                            predicted=r, error_pct=_pct_err(r, target),
                            n_ops=1,
                        ))
            # Power (small exponents only)
            if 1 < vb <= 4 and va > 1:
                r = va ** vb
                if r > 0 and _pct_err(r, target) <= tol_pct:
                    fstr = f'{na}^{nb}'
                    fval = f'{va}^{vb} = {r}'
                    if fval not in seen:
                        seen.add(fval)
                        matches.append(FormulaMatch(
                            formula=f'{fstr} = {fval}',
                            predicted=r, error_pct=_pct_err(r, target),
                            n_ops=1,
                        ))

    # --- 3-operand expressions: a OP1 b OP2 c (left-associative) ---
    for (na, va) in basic.items():
        for (nb, vb) in basic.items():
            for (nc, vc) in basic.items():
                for op1n, op1f in ops2.items():
                    for op2n, op2f in ops2.items():
                        ab = op1f(va, vb)
                        r = op2f(ab, vc)
                        if r > 0 and _pct_err(r, target) <= tol_pct:
                            fstr = f'{na} {op1n} {nb} {op2n} {nc}'
                            fval = f'{va} {op1n} {vb} {op2n} {vc} = {r}'
                            if fval not in seen:
                                seen.add(fval)
                                matches.append(FormulaMatch(
                                    formula=f'{fstr} = {fval}',
                                    predicted=r,
                                    error_pct=_pct_err(r, target),
                                    n_ops=2,
                                ))

    # --- Special forms: a^2 - b - c - d ---
    for (na, va) in basic.items():
        for (nb, vb) in basic.items():
            for (nc, vc) in basic.items():
                for (nd, vd) in basic.items():
                    r = va**2 - vb - vc - vd
                    if r > 0 and _pct_err(r, target) <= tol_pct:
                        fstr = f'{na}^2 - {nb} - {nc} - {nd}'
                        fval = f'{va}^2 - {vb} - {vc} - {vd} = {r}'
                        if fval not in seen:
                            seen.add(fval)
                            matches.append(FormulaMatch(
                                formula=f'{fstr} = {fval}',
                                predicted=r,
                                error_pct=_pct_err(r, target),
                                n_ops=3,
                            ))

    # --- Special forms: a^2 - b - c + d ---
    for (na, va) in basic.items():
        for (nb, vb) in basic.items():
            for (nc, vc) in basic.items():
                for (nd, vd) in basic.items():
                    r = va**2 - vb - vc + vd
                    if r > 0 and _pct_err(r, target) <= tol_pct:
                        fstr = f'{na}^2 - {nb} - {nc} + {nd}'
                        fval = f'{va}^2 - {vb} - {vc} + {vd} = {r}'
                        if fval not in seen:
                            seen.add(fval)
                            matches.append(FormulaMatch(
                                formula=f'{fstr} = {fval}',
                                predicted=r,
                                error_pct=_pct_err(r, target),
                                n_ops=3,
                            ))

    # --- Special forms: a * b - c  or  a * b + c  (with extended pool) ---
    for (na, va) in extended.items():
        for (nb, vb) in extended.items():
            for (nc, vc) in extended.items():
                for sign, sn in [(1, '+'), (-1, '-')]:
                    r = va * vb + sign * vc
                    if r > 0 and _pct_err(r, target) <= tol_pct:
                        fstr = f'{na} * {nb} {sn} {nc}'
                        fval = f'{va} * {vb} {sn} {vc} = {r}'
                        if fval not in seen:
                            seen.add(fval)
                            matches.append(FormulaMatch(
                                formula=f'{fstr} = {fval}',
                                predicted=r,
                                error_pct=_pct_err(r, target),
                                n_ops=2,
                            ))

    # Sort by error, then by simplicity
    matches.sort(key=lambda m: (m.error_pct, m.n_ops))
    return matches


def higgs_mass_mc_pvalue(n_target_range=1.0, n_trials=10_000, seed=42):
    """For a generic integer target in [100, 150], how many expressions
    from the basic pool land within tol_pct?  Returns the fraction of
    integer targets that have at least one match, i.e. the a-priori
    probability of finding *some* match by chance."""
    rng = np.random.default_rng(seed)
    basic_vals = [SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, P1, P2]

    targets_checked = 0
    targets_with_match = 0

    for t in range(100, 151):
        targets_checked += 1
        found = False

        # 2-op: a OP b
        for a in basic_vals:
            if found:
                break
            for b in basic_vals:
                for r in [a + b, a - b, a * b]:
                    if r > 0 and _pct_err(r, t) <= n_target_range:
                        found = True
                        break
                if found:
                    break
                if b != 0 and a / b > 0 and _pct_err(a / b, t) <= n_target_range:
                    found = True
                    break

        # 3-op: a^2 - b - c + d / a^2 - b - c - d
        if not found:
            for a in basic_vals:
                if found:
                    break
                for b in basic_vals:
                    if found:
                        break
                    for c in basic_vals:
                        if found:
                            break
                        for d in basic_vals:
                            for r in [a**2 - b - c - d, a**2 - b - c + d]:
                                if r > 0 and _pct_err(r, t) <= n_target_range:
                                    found = True
                                    break
                            if found:
                                break

        if found:
            targets_with_match += 1

    return {
        'range': '100-150 GeV',
        'targets_checked': targets_checked,
        'targets_with_match': targets_with_match,
        'fraction': targets_with_match / targets_checked,
        'interpretation': (
            f'{targets_with_match}/{targets_checked} integers in [100,150] '
            f'have at least one TECS expression within {n_target_range}%'
        ),
    }


# =====================================================================
# PART 3: Higgs Branching Ratio Analysis
# =====================================================================

def _build_n6_fractions():
    """Build a catalog of n=6-derived fractions for matching."""
    s, t, p, sp, n = SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, P1
    fracs = OrderedDict()

    # Simple fractions from n=6 constants
    pool = {'sigma': s, 'tau': t, 'phi': p, 'sopfr': sp, 'n': n,
            'sigma+tau': s+t, 'sigma-tau': s-t, 'sigma*phi': s*p,
            'tau*phi': t*p, 'sigma*tau': s*t,
            'sigma^2': s**2, 'tau^2': t**2,
            'n^2': n**2, 'sopfr^2': sp**2,
            '1': 1}

    for na, va in pool.items():
        for nb, vb in pool.items():
            if vb == 0:
                continue
            frac = va / vb
            if 0 < frac < 1:
                label = f'{na}/{nb}'
                fracs[label] = frac

    # Some specific ones mentioned in the task
    fracs['7/12'] = 7 / 12
    fracs['1/16'] = 1 / 16
    fracs['5/24'] = 5 / 24

    return fracs


def branching_ratio_analysis():
    """Check each Higgs BR against n=6 fractions."""
    fracs = _build_n6_fractions()
    results = []

    for channel, (br_pct, br_err) in HIGGS_BR.items():
        br = br_pct / 100.0
        best_label = None
        best_frac = None
        best_err = float('inf')

        for label, fval in fracs.items():
            err = _pct_err(fval, br)
            if err < best_err:
                best_err = err
                best_label = label
                best_frac = fval

        results.append({
            'channel': channel,
            'br_pct': br_pct,
            'br_err': br_err,
            'best_n6_frac': best_label,
            'best_n6_val': best_frac,
            'best_n6_pct': best_frac * 100 if best_frac else 0,
            'match_err_pct': best_err,
            'within_exp_err': abs(best_frac - br) * 100 <= br_err if best_frac else False,
        })

    return results


def branching_ratio_mc(n_trials=100_000, seed=42):
    """Monte Carlo: generate random 9-channel branching ratios (Dirichlet)
    and check how often bb lands near 7/12 AND tautau near 1/16 simultaneously."""
    rng = np.random.default_rng(seed)

    target_bb = 7 / 12        # 0.58333
    target_tautau = 1 / 16    # 0.0625

    # Tolerances: same as observed match quality
    tol_bb = 0.003            # ~0.3% absolute
    tol_tautau = 0.001        # ~0.1% absolute

    # Dirichlet with uniform alpha (all channels equally likely a priori)
    alpha = np.ones(9)
    hits_bb = 0
    hits_tautau = 0
    hits_both = 0

    for _ in range(n_trials):
        br = rng.dirichlet(alpha)
        br_sorted = np.sort(br)[::-1]  # descending

        # bb is the largest channel
        if abs(br_sorted[0] - target_bb) < tol_bb:
            hits_bb += 1
            # tautau is the 4th largest
            if len(br_sorted) > 3 and abs(br_sorted[3] - target_tautau) < tol_tautau:
                hits_both += 1

        # Check tautau anywhere
        for val in br_sorted:
            if abs(val - target_tautau) < tol_tautau:
                hits_tautau += 1
                break

    p_bb = hits_bb / n_trials
    p_tautau = hits_tautau / n_trials
    p_both = hits_both / n_trials

    return {
        'n_trials': n_trials,
        'target_bb': f'7/12 = {target_bb:.5f}',
        'target_tautau': f'1/16 = {target_tautau:.5f}',
        'hits_bb': hits_bb,
        'hits_tautau': hits_tautau,
        'hits_both': hits_both,
        'p_bb': p_bb,
        'p_tautau': p_tautau,
        'p_both': p_both,
        'p_both_str': f'{p_both:.6f}' if p_both > 0 else f'< {1/n_trials:.1e}',
        'sigma_both': _p_to_sigma(p_both) if p_both > 0 else '> 4.3',
    }


# =====================================================================
# PART 4: Electroweak Scale Relations
# =====================================================================

def electroweak_relations():
    """Check ratios among EW masses against TECS-L targets."""
    s, t, p, sp, n = SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, P1
    v = HIGGS_VEV
    mH = HIGGS_MASS

    results = []

    # v / sigma = 20.52 vs magic number 20
    r = v / s
    tecs_pred = s * p - t  # 24 - 4 = 20
    results.append({
        'relation': f'v / sigma = {v}/{s}',
        'value': r,
        'tecs_target': f'sigma*phi - tau = {tecs_pred}',
        'tecs_value': tecs_pred,
        'error_pct': _pct_err(r, tecs_pred),
        'note': f'20 is a nuclear magic number; sigma*(sigma*phi-tau) = {s}*{tecs_pred} = {s*tecs_pred} = phi(P3)',
    })

    # v / (sigma*phi) = 10.26 vs tau(P3) = 10
    r2 = v / (s * p)
    results.append({
        'relation': f'v / (sigma*phi) = {v}/{s*p}',
        'value': r2,
        'tecs_target': f'tau(P3) = {TAU_P3}',
        'tecs_value': TAU_P3,
        'error_pct': _pct_err(r2, TAU_P3),
    })

    # v^2
    v2 = v ** 2
    # Check against TECS expressions
    # phi(P3) * sigma * P2 / ... try a few
    cand_v2 = PHI_P3 * P2 - P3  # 240*28 - 496 = 6720-496 = 6224 (no)
    # Actual v^2 = 60,624
    results.append({
        'relation': f'v^2 = {v}^2',
        'value': v2,
        'tecs_target': 'no clean decomposition found',
        'tecs_value': None,
        'error_pct': None,
        'note': f'v^2 = {v2:.1f}',
    })

    # mH / v = 0.5087 vs 1/2
    r3 = mH / v
    results.append({
        'relation': f'mH / v = {mH}/{v}',
        'value': r3,
        'tecs_target': f'phi/tau = 1/2',
        'tecs_value': 0.5,
        'error_pct': _pct_err(r3, 0.5),
    })

    # mH / mW
    r4 = mH / M_W
    results.append({
        'relation': f'mH / mW = {mH}/{M_W}',
        'value': r4,
        'tecs_target': 'no clean n=6 fraction',
        'tecs_value': None,
        'error_pct': None,
        'note': f'= {r4:.4f}',
    })

    # mH / mZ = 1.374 vs 4/3?
    r5 = mH / M_Z
    results.append({
        'relation': f'mH / mZ = {mH}/{M_Z}',
        'value': r5,
        'tecs_target': 'tau/sigma*tau = 4/3',
        'tecs_value': 4/3,
        'error_pct': _pct_err(r5, 4/3),
        'note': f'tau/(sigma/tau) = tau^2/sigma = {t**2}/{s} = {t**2/s:.4f} (exact 4/3)',
    })

    # mH * mZ / v^2
    r6 = mH * M_Z / v**2
    # Check against common fractions
    target_r6 = 3 / 16  # = 0.1875
    results.append({
        'relation': f'mH * mZ / v^2',
        'value': r6,
        'tecs_target': f'sigma/(tau*sigma^2) = 3/16 = {3/16}?',
        'tecs_value': target_r6,
        'error_pct': _pct_err(r6, target_r6),
        'note': f'= {r6:.4f}; 3/16 = 0.1875',
    })

    # Weinberg angle check: sin^2(theta_W) = 1 - mW^2/mZ^2
    sin2_w = 1 - (M_W / M_Z) ** 2
    results.append({
        'relation': 'sin^2(theta_W) = 1 - mW^2/mZ^2',
        'value': sin2_w,
        'tecs_target': f'sopfr/(sigma+sopfr+n+1) = 5/24 = {5/24:.4f}?',
        'tecs_value': 5 / 24,
        'error_pct': _pct_err(sin2_w, 5/24),
        'note': f'PDG sin^2(theta_W) = 0.2312; 5/24 = {5/24:.4f}',
    })

    return results


# =====================================================================
# PART 5: Resonance Ladder Extension to Higgs
# =====================================================================

def ladder_to_higgs():
    """Check if the QCD resonance ladder extends to the Higgs."""
    results = []

    # Upsilon * X = Higgs?
    x = HIGGS_MASS / UPSILON_MASS
    results.append({
        'relation': f'Higgs / Upsilon = {HIGGS_MASS} / {UPSILON_MASS}',
        'ratio': x,
        'nearest_tecs': f'sigma + 1 = 13',
        'tecs_value': 13,
        'error_pct': _pct_err(x, 13),
        'note': 'sigma + 1 = 13; not a clean n=6 expression (uses +1)',
    })

    # rho * sigma^2
    pred_rho_s2 = RHO_MASS * SIGMA_P1 ** 2
    results.append({
        'relation': f'rho * sigma^2 = {RHO_MASS} * {SIGMA_P1**2}',
        'predicted': pred_rho_s2,
        'target': HIGGS_MASS,
        'error_pct': _pct_err(pred_rho_s2, HIGGS_MASS),
        'note': f'= {pred_rho_s2:.2f} GeV (vs {HIGGS_MASS})',
    })

    # J/psi * sigma * tau / sopfr * phi / sigma... try various
    attempts = [
        ('J/psi * sigma*phi*tau/n',  JPSI_MASS * SIGMA_P1 * PHI_P1 * TAU_P1 / P1),
        ('J/psi * sigma*phi',        JPSI_MASS * SIGMA_P1 * PHI_P1),
        ('rho * sigma*phi*n + tau',  RHO_MASS * SIGMA_P1 * PHI_P1 * P1 + TAU_P1),
        ('rho * sigma * sigma + tau', RHO_MASS * SIGMA_P1 * SIGMA_P1 + TAU_P1),
        ('J/psi * sigma*tau/n*phi',  JPSI_MASS * SIGMA_P1 * TAU_P1 / P1 * PHI_P1),
        ('Upsilon * sigma + tau',    UPSILON_MASS * SIGMA_P1 + TAU_P1),
        ('Upsilon * sigma + n',      UPSILON_MASS * SIGMA_P1 + P1),
        ('Upsilon * (sigma+1)',      UPSILON_MASS * 13),
    ]

    for label, pred in attempts:
        if pred > 0:
            results.append({
                'relation': label,
                'predicted': pred,
                'target': HIGGS_MASS,
                'error_pct': _pct_err(pred, HIGGS_MASS),
            })

    # Sort by error
    results.sort(key=lambda r: r.get('error_pct', 999))
    return results


# =====================================================================
# Full Report
# =====================================================================

def run_analysis():
    """Run all Higgs analyses and print comprehensive report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  COMPREHENSIVE HIGGS ANALYSIS — TECS-L n=6 Framework')
    print(sep)
    print(f'  Higgs mass: {HIGGS_MASS} +/- {HIGGS_MASS_ERR} GeV')
    print(f'  Higgs VEV:  {HIGGS_VEV} +/- {HIGGS_VEV_ERR} GeV')
    print(f'  TECS-L P1=6: sigma={SIGMA_P1}, tau={TAU_P1}, phi={PHI_P1}, '
          f'sopfr={SOPFR_P1}, n={P1}')
    print(f'  P2={P2}: tau(P2)={TAU_P2}, phi(P2)={PHI_P2}, sigma(P2)={SIGMA_P2}')
    print(f'  P3={P3}: tau(P3)={TAU_P3}, phi(P3)={PHI_P3}')

    # ==================== PART 1 ====================
    print(f'\n{sep}')
    print('  PART 1: Higgs VEV Decomposition')
    print(sep)

    vev_res = vev_analysis()
    for r in vev_res:
        print(f'\n  {r["formula"]}')
        print(f'    Predicted: {r["predicted"]}')
        print(f'    Observed:  {r["observed"]}')
        print(f'    Error:     {r["error_pct"]:.3f}%')
        if 'note' in r:
            print(f'    Note:      {r["note"]}')

    print(f'\n  --- VEV Texas Sharpshooter Test ---')
    vev_mc = vev_mc_sharpshooter()
    print(f'    Total expressions tested:  {vev_mc["total_expressions"]:,}')
    print(f'    Hits within {vev_mc["tolerance_pct"]}% of 246: {vev_mc["hits"]}')
    print(f'    p-value: {vev_mc["p_value"]:.6f}')
    sig = vev_mc["sigma"]
    print(f'    Sigma:   {sig:.2f}' if isinstance(sig, float) else f'    Sigma:   {sig}')

    # ==================== PART 2 ====================
    print(f'\n{sep}')
    print('  PART 2: Higgs Mass Formula Search')
    print(sep)

    matches = higgs_mass_search()
    print(f'\n  Found {len(matches)} expressions within 1% of {HIGGS_MASS} GeV')
    print(f'\n  Top 30 matches (sorted by error):')
    print(f'  {"#":>3s}  {"Formula":<50s} {"Pred":>7s} {"Err%":>6s} {"Ops":>3s}')
    print(f'  {"-"*3}  {"-"*50} {"-"*7} {"-"*6} {"-"*3}')
    for i, m in enumerate(matches[:30], 1):
        print(f'  {i:>3d}  {m.formula:<50s} {m.predicted:>7.2f} {m.error_pct:>6.3f} {m.n_ops:>3d}')

    # Highlight the key formulae
    print(f'\n  --- Key Formulae ---')
    key_formulae = [
        ('sigma^2 - sigma - sopfr - tau + phi',
         SIGMA_P1**2 - SIGMA_P1 - SOPFR_P1 - TAU_P1 + PHI_P1),
        ('sigma^2 - sigma - n + 1 (= 144-12-6-1 = 125)',
         SIGMA_P1**2 - SIGMA_P1 - P1 - 1),
        ('sigma^2 - sigma - n = 126',
         SIGMA_P1**2 - SIGMA_P1 - P1),
    ]
    for label, val in key_formulae:
        err = _pct_err(val, HIGGS_MASS)
        print(f'    {label} = {val}  (error: {err:.3f}%)')

    # MC p-value
    print(f'\n  --- Texas Sharpshooter: Integer Coverage ---')
    mc_mass = higgs_mass_mc_pvalue()
    print(f'    Range: {mc_mass["range"]}')
    print(f'    Integers with >= 1 TECS match within 1%: '
          f'{mc_mass["targets_with_match"]}/{mc_mass["targets_checked"]} '
          f'= {mc_mass["fraction"]*100:.1f}%')
    print(f'    Interpretation: {mc_mass["interpretation"]}')

    # ==================== PART 3 ====================
    print(f'\n{sep}')
    print('  PART 3: Higgs Branching Ratios vs n=6 Fractions')
    print(sep)

    br_res = branching_ratio_analysis()
    print(f'\n  {"Channel":<12s} {"BR%":>6s} {"n=6 Frac":<20s} '
          f'{"n=6%":>7s} {"Err%":>6s} {"InExp":>5s}')
    print(f'  {"-"*12} {"-"*6} {"-"*20} {"-"*7} {"-"*6} {"-"*5}')
    for r in br_res:
        in_exp = 'YES' if r['within_exp_err'] else ''
        print(f'  {r["channel"]:<12s} {r["br_pct"]:>6.3f} {r["best_n6_frac"]:<20s} '
              f'{r["best_n6_pct"]:>7.3f} {r["match_err_pct"]:>6.2f} {in_exp:>5s}')

    # Highlight best matches
    print(f'\n  --- Best Matches ---')
    good = [r for r in br_res if r['match_err_pct'] < 3.0]
    for r in good:
        within = ' (within experimental error!)' if r['within_exp_err'] else ''
        print(f'    {r["channel"]}: {r["br_pct"]}% vs {r["best_n6_frac"]} = '
              f'{r["best_n6_pct"]:.3f}%  (err: {r["match_err_pct"]:.2f}%){within}')

    # MC
    print(f'\n  --- Branching Ratio Monte Carlo ---')
    print(f'  Drawing 100k random 9-channel Dirichlet branching ratios ...')
    br_mc = branching_ratio_mc()
    print(f'    Trials:        {br_mc["n_trials"]:,}')
    print(f'    Hits bb~7/12:  {br_mc["hits_bb"]}  (p = {br_mc["p_bb"]:.6f})')
    print(f'    Hits tt~1/16:  {br_mc["hits_tautau"]}  (p = {br_mc["p_tautau"]:.6f})')
    print(f'    Hits BOTH:     {br_mc["hits_both"]}  (p = {br_mc["p_both_str"]})')
    sig = br_mc["sigma_both"]
    print(f'    Sigma (both):  {sig:.2f}' if isinstance(sig, float) else
          f'    Sigma (both):  {sig}')

    # ==================== PART 4 ====================
    print(f'\n{sep}')
    print('  PART 4: Electroweak Scale Relations')
    print(sep)

    ew_res = electroweak_relations()
    for r in ew_res:
        print(f'\n  {r["relation"]}')
        print(f'    Value:       {r["value"]:.5f}')
        print(f'    TECS target: {r["tecs_target"]}')
        if r['tecs_value'] is not None:
            print(f'    TECS value:  {r["tecs_value"]:.5f}' if isinstance(r['tecs_value'], float)
                  else f'    TECS value:  {r["tecs_value"]}')
        if r['error_pct'] is not None:
            print(f'    Error:       {r["error_pct"]:.3f}%')
        if 'note' in r:
            print(f'    Note:        {r["note"]}')

    # ==================== PART 5 ====================
    print(f'\n{sep}')
    print('  PART 5: Resonance Ladder Extension to Higgs')
    print(sep)

    ladder_res = ladder_to_higgs()
    print(f'\n  {"Relation":<40s} {"Predicted":>9s} {"Target":>9s} {"Err%":>6s}')
    print(f'  {"-"*40} {"-"*9} {"-"*9} {"-"*6}')
    for r in ladder_res:
        pred = r.get('predicted', r.get('ratio', 0))
        tgt = r.get('target', r.get('tecs_value', HIGGS_MASS))
        print(f'  {r["relation"]:<40s} {pred:>9.3f} {tgt:>9.3f} {r["error_pct"]:>6.2f}')

    # ==================== SUMMARY ====================
    print(f'\n{sep}')
    print('  SUMMARY — All Findings Ranked by Significance')
    print(sep)

    all_findings = []

    # VEV
    all_findings.append(('VEV = phi(P3)+P1 = 240+6 = 246',
                         _pct_err(246, HIGGS_VEV), 'E8 roots + P1'))

    # Mass
    best_mass = matches[0] if matches else None
    if best_mass:
        all_findings.append((f'mH ~ {best_mass.formula}',
                             best_mass.error_pct, f'{best_mass.n_ops} ops'))

    # BRs
    for r in br_res:
        if r['match_err_pct'] < 3.0:
            all_findings.append((f'{r["channel"]} = {r["best_n6_frac"]}',
                                 r['match_err_pct'],
                                 'within exp err' if r['within_exp_err'] else ''))

    # EW
    for r in ew_res:
        if r['error_pct'] is not None and r['error_pct'] < 5.0:
            all_findings.append((f'{r["relation"]} ~ {r["tecs_target"]}',
                                 r['error_pct'], r.get('note', '')))

    # Ladder
    for r in ladder_res[:3]:
        all_findings.append((r['relation'],
                             r.get('error_pct', 99), r.get('note', '')))

    # Sort by error
    all_findings.sort(key=lambda x: x[1])

    print(f'\n  {"#":>3s}  {"Finding":<55s} {"Err%":>6s}  Note')
    print(f'  {"-"*3}  {"-"*55} {"-"*6}  {"-"*30}')
    for i, (desc, err, note) in enumerate(all_findings, 1):
        note_short = note[:30] if note else ''
        print(f'  {i:>3d}  {desc:<55s} {err:>6.3f}  {note_short}')

    print(f'\n  Total findings within 1%:  '
          f'{sum(1 for _, e, _ in all_findings if e < 1.0)}')
    print(f'  Total findings within 3%:  '
          f'{sum(1 for _, e, _ in all_findings if e < 3.0)}')
    print(f'  Total findings within 5%:  '
          f'{sum(1 for _, e, _ in all_findings if e < 5.0)}')

    print(f'\n  Texas Sharpshooter summary:')
    print(f'    VEV=246 p-value:      {vev_mc["p_value"]:.6f}')
    print(f'    Mass formula coverage: {mc_mass["fraction"]*100:.1f}% of integers have a match')
    print(f'    BR (bb+tt) p-value:   {br_mc["p_both_str"]}')

    print(f'\n{sep}')
    print('  END OF HIGGS ANALYSIS')
    print(f'{sep}\n')

    return {
        'vev': vev_res,
        'vev_mc': vev_mc,
        'mass_matches': matches,
        'mass_mc': mc_mass,
        'branching_ratios': br_res,
        'br_mc': br_mc,
        'electroweak': ew_res,
        'ladder': ladder_res,
        'all_findings': all_findings,
    }


if __name__ == '__main__':
    run_analysis()
