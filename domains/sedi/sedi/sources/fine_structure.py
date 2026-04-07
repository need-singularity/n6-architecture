"""Fine Structure Constant Analysis — TECS-L n=6 Framework.

The fine structure constant alpha is one of the most precisely measured
constants in physics:

    1/alpha = 137.035999084(21)    (CODATA 2018)

This module investigates the relationship between 1/alpha and n=6 arithmetic.

Integer part: 137
    - sigma^2 - n - 1 = 144 - 6 - 1 = 137       (exact)
    - sigma^2 - M3 = 144 - 7 = 137               (exact, M3 = 2^3-1 Mersenne prime)
    - (sigma - tau) * 17 + 1 = 8*17 + 1 = 137    (exact, but 17 is external)
    - 137 is itself prime

Fractional part: 0.035999084...
    - 1/P2 = 1/28 = 0.035714... (0.8% off)
    - 1/(P2-1) = 1/27 = 0.037037... (2.9% off)
    - Systematic search over TECS-L expressions

Analysis sections:
    1. Integer part derivations from n=6
    2. Systematic expression search for fractional part
    3. Full 1/alpha candidate formulas (integer + fraction)
    4. Wyler's formula comparison
    5. QED perturbative structure and TECS-L coefficients
    6. Texas Sharpshooter analysis
    7. Ranked summary of all findings

Data sources: CODATA 2018, PDG 2024.
"""

from __future__ import annotations

import itertools
import math
from collections import OrderedDict
from dataclasses import dataclass, field
from typing import Dict, List, Optional, Tuple

import numpy as np

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1, OMEGA_P1,
    P1, P2, P3,
    SIGMA_P2, TAU_P2, PHI_P2,
    TAU_P3, PHI_P3,
    R, S,
)


# =====================================================================
# Physical constants
# =====================================================================

ALPHA_INV_MEASURED = 137.035999084       # CODATA 2018
ALPHA_INV_UNCERT   = 0.000000021         # 1-sigma uncertainty
ALPHA_EM           = 1.0 / ALPHA_INV_MEASURED
ALPHA_INV_INTEGER  = 137
ALPHA_INV_FRAC     = ALPHA_INV_MEASURED - ALPHA_INV_INTEGER  # 0.035999084

# QED constants
ALPHA_OVER_PI      = ALPHA_EM / math.pi  # ~0.002323

# Wyler's formula (historical)
WYLER_INV_ALPHA    = (9.0 / (16.0 * math.pi**3)) * (math.pi / math.factorial(5))**(1.0/4.0)
WYLER_INV_ALPHA    = 1.0 / WYLER_INV_ALPHA  # ~137.036082


# =====================================================================
# TECS-L constants
# =====================================================================

N     = P1           # 6
SIGMA = SIGMA_P1     # 12
TAU   = TAU_P1       # 4
PHI   = PHI_P1       # 2
SOPFR = SOPFR_P1     # 5
OMEGA = OMEGA_P1     # 2
M3    = 7            # Mersenne prime 2^3 - 1

TECS_POOL = OrderedDict([
    ('sigma',      SIGMA),       # 12
    ('tau',        TAU),         # 4
    ('phi',        PHI),         # 2
    ('sopfr',      SOPFR),       # 5
    ('n',          N),           # 6
    ('omega',      OMEGA),       # 2
    ('P1',         P1),          # 6
    ('P2',         P2),          # 28
    ('P3',         P3),          # 496
    ('M3',         M3),          # 7
    ('tau(P2)',    TAU_P2),      # 6
    ('tau(P3)',    TAU_P3),      # 10
    ('phi(P2)',   PHI_P2),      # 12
    ('phi(P3)',   PHI_P3),      # 240
    ('sigma(P2)', SIGMA_P2),    # 56
])


# =====================================================================
# Helper functions
# =====================================================================

def _pct_err(predicted: float, observed: float) -> float:
    """Percentage error."""
    if observed == 0:
        return float('inf')
    return abs(predicted - observed) / abs(observed) * 100


@dataclass(order=True)
class Finding:
    """A single finding, sortable by error."""
    error_pct: float
    description: str = field(compare=False)
    predicted: float = field(compare=False)
    observed: float = field(compare=False)
    formula: str = field(compare=False, default='')
    category: str = field(compare=False, default='')
    note: str = field(compare=False, default='')


# =====================================================================
# PART 1: Integer part — 137 from n=6
# =====================================================================

def integer_part_analysis() -> List[Finding]:
    """Establish 137 from n=6 arithmetic identities."""
    findings = []
    target = ALPHA_INV_INTEGER  # 137

    # --- Direct identities ---
    formulas = [
        ('sigma^2 - n - 1',           SIGMA**2 - N - 1,
         'Primary TECS-L identity'),
        ('sigma^2 - M3',              SIGMA**2 - M3,
         'M3 = 2^3 - 1 = 7 (3rd Mersenne prime)'),
        ('(sigma-tau)*17 + 1',        (SIGMA - TAU) * 17 + 1,
         '17 is external — not purely n=6'),
        ('sigma^2 - (phi*sopfr-tau+1)', SIGMA**2 - (PHI*SOPFR - TAU + 1),
         'phi*sopfr - tau + 1 = 10-4+1 = 7 = M3'),
        ('sigma^2 - sigma/phi + 1',   SIGMA**2 - SIGMA // PHI + 1,
         'sigma/phi = 6, gives 144-6+1=139 — no'),
        ('sigma^2 - tau - phi - 1',   SIGMA**2 - TAU - PHI - 1,
         'tau+phi+1 = 7 = M3'),
    ]

    for label, value, note in formulas:
        err = _pct_err(value, target)
        findings.append(Finding(
            error_pct=err,
            description=f'137 = {label} = {value}',
            predicted=value,
            observed=target,
            formula=label,
            category='integer part',
            note=note,
        ))

    # --- 137 as prime: connections to n=6 ---
    # 137 is the 33rd prime. 33 = sigma + sigma*phi - 3 = 12 + 24 - 3?
    # Not compelling; just note primality.
    findings.append(Finding(
        error_pct=0.0 if target == 137 else 100.0,
        description='137 is prime (33rd prime)',
        predicted=137,
        observed=target,
        formula='isprime(137)',
        category='integer part',
        note='137 = prime; 33rd prime; 33 = sigma+sopfr*tau+1?',
    ))

    # --- 137 from sum identities ---
    # 137 = sum of first n=6 Catalan numbers? C(0..5) = 1+1+2+5+14+42 = 65 — no
    # 137 = 2^7 + 2^3 + 2^0 = 128+8+1 — binary 10001001
    # 137 = sigma^2 - tau - phi - 1 = 144-4-2-1 = 137 (same as M3 decomposition)

    return findings


# =====================================================================
# PART 2: Systematic expression search — fractional part
# =====================================================================

def _build_two_operand_expressions() -> List[Tuple[str, float]]:
    """Generate expressions from pairs of TECS-L values with arithmetic ops.

    Returns list of (label, value) tuples for the fractional part search.
    """
    # Named values for building expressions
    named = OrderedDict([
        ('sigma',      float(SIGMA)),       # 12
        ('tau',        float(TAU)),         # 4
        ('phi',        float(PHI)),         # 2
        ('sopfr',      float(SOPFR)),       # 5
        ('n',          float(N)),           # 6
        ('P2',         float(P2)),          # 28
        ('P3',         float(P3)),          # 496
        ('M3',         float(M3)),          # 7
        ('sigma(P2)',  float(SIGMA_P2)),    # 56
        ('tau(P2)',    float(TAU_P2)),      # 6
        ('tau(P3)',    float(TAU_P3)),      # 10
        ('phi(P2)',   float(PHI_P2)),      # 12
        ('phi(P3)',   float(PHI_P3)),      # 240
    ])

    # Also include transcendental constants
    transcendentals = OrderedDict([
        ('pi',         math.pi),
        ('e',          math.e),
        ('ln2',        math.log(2)),
        ('sqrt2',      math.sqrt(2)),
        ('sqrt3',      math.sqrt(3)),
    ])

    all_named = OrderedDict(list(named.items()) + list(transcendentals.items()))
    results = []

    # 1/x for each value
    for name, val in all_named.items():
        if val != 0:
            results.append((f'1/{name}', 1.0 / val))
        if val > 0:
            results.append((f'1/{name}^2', 1.0 / val**2))

    # a/b for all pairs
    names_list = list(all_named.keys())
    vals_list = list(all_named.values())
    for i in range(len(names_list)):
        for j in range(len(names_list)):
            if i == j:
                continue
            a_name, a_val = names_list[i], vals_list[i]
            b_name, b_val = names_list[j], vals_list[j]
            if b_val != 0:
                results.append((f'{a_name}/{b_name}', a_val / b_val))
            # a/(b^2)
            if b_val != 0:
                results.append((f'{a_name}/{b_name}^2', a_val / b_val**2))
            # a*b (only if result is useful range)
            prod = a_val * b_val
            if 0 < prod < 1000:
                results.append((f'{a_name}*{b_name}', prod))

    # Sums and differences of 1/x
    for i in range(len(names_list)):
        for j in range(i + 1, len(names_list)):
            a_name, a_val = names_list[i], vals_list[i]
            b_name, b_val = names_list[j], vals_list[j]
            if a_val != 0 and b_val != 0:
                results.append((f'1/{a_name}+1/{b_name}', 1.0/a_val + 1.0/b_val))
                results.append((f'1/{a_name}-1/{b_name}', 1.0/a_val - 1.0/b_val))

    # Three-operand: a/(b*c)
    tecs_names = list(named.keys())
    tecs_vals = list(named.values())
    for i in range(len(tecs_names)):
        for j in range(len(tecs_names)):
            for k in range(j + 1, len(tecs_names)):
                if i == j or i == k:
                    continue
                a, va = tecs_names[i], tecs_vals[i]
                b, vb = tecs_names[j], tecs_vals[j]
                c, vc = tecs_names[k], tecs_vals[k]
                denom = vb * vc
                if denom != 0:
                    results.append((f'{a}/({b}*{c})', va / denom))
                # Also (a*b)/c^2
                if vc != 0:
                    results.append((f'({a}*{b})/{c}^2', (va * vb) / vc**2))

    # Geometric series: 1/(P2-1) = 1/27, sum_{k=1}^{inf} 1/P2^k
    results.append(('1/(P2-1)', 1.0 / (P2 - 1)))  # 1/27
    results.append(('1/(P2+1)', 1.0 / (P2 + 1)))  # 1/29
    results.append(('1/P2 + 1/P2^2', 1.0/P2 + 1.0/P2**2))
    results.append(('1/P2 + 1/P2^2 + 1/P2^3', 1.0/P2 + 1.0/P2**2 + 1.0/P2**3))

    # Special combinations suggested in context
    results.append(('pi/(sigma*tau*phi - pi)', math.pi / (SIGMA * TAU * PHI - math.pi)))
    results.append(('phi/(sigma*sopfr-tau)', PHI / (SIGMA * SOPFR - TAU)))
    results.append(('tau/sigma^2', TAU / SIGMA**2))
    results.append(('1/sigma(P2)', 1.0 / SIGMA_P2))
    results.append(('pi/(sigma^2*sopfr)', math.pi / (SIGMA**2 * SOPFR)))
    results.append(('pi/(tau*sigma^2)', math.pi / (TAU * SIGMA**2)))
    results.append(('pi/(n*sigma^2)', math.pi / (N * SIGMA**2)))
    results.append(('e/(sigma^2*sopfr)', math.e / (SIGMA**2 * SOPFR)))
    results.append(('ln2/(sigma+M3)', math.log(2) / (SIGMA + M3)))

    # alpha_s(M_Z)/pi ~ 0.03753 (mentioned in context)
    ALPHA_S_MZ = 0.1179
    results.append(('alpha_s(M_Z)/pi', ALPHA_S_MZ / math.pi))

    return results


def fractional_part_search(threshold_pct: float = 0.1) -> List[Finding]:
    """Search for TECS-L expressions matching the fractional part 0.035999084.

    Args:
        threshold_pct: maximum percentage error to include (default 0.1%)

    Returns:
        List of findings sorted by error.
    """
    target = ALPHA_INV_FRAC  # 0.035999084
    expressions = _build_two_operand_expressions()
    findings = []

    seen = set()
    for label, value in expressions:
        if value <= 0 or not math.isfinite(value):
            continue
        err = _pct_err(value, target)
        if err <= threshold_pct:
            # Deduplicate by rounded value
            key = f'{label}:{value:.12f}'
            if key not in seen:
                seen.add(key)
                findings.append(Finding(
                    error_pct=err,
                    description=f'frac = {label} = {value:.8f}',
                    predicted=value,
                    observed=target,
                    formula=label,
                    category='fractional part',
                    note=f'target={target:.9f}',
                ))

    findings.sort()
    return findings


# =====================================================================
# PART 3: Full 1/alpha candidate formulas
# =====================================================================

def full_alpha_inv_search(threshold_pct: float = 0.001) -> List[Finding]:
    """Search for complete 1/alpha formulas: integer_expr + fractional_expr.

    Also tries direct formulas that don't decompose into integer+fraction.
    """
    target = ALPHA_INV_MEASURED  # 137.035999084
    findings = []

    # --- Integer bases ---
    int_formulas = [
        ('sigma^2-n-1',   SIGMA**2 - N - 1),          # 137
        ('sigma^2-M3',    SIGMA**2 - M3),              # 137
        ('sigma^2-tau-phi-1', SIGMA**2 - TAU - PHI - 1),  # 137
    ]

    # --- Fractional corrections ---
    frac_formulas = [
        ('1/P2',             1.0 / P2),                # 0.035714
        ('1/(P2-1)',         1.0 / (P2 - 1)),          # 0.037037
        ('1/P2+1/P2^2',     1.0/P2 + 1.0/P2**2),      # 0.036990
        ('pi/sigma^3',       math.pi / SIGMA**3),       # 0.001818
        ('pi/(sigma^2*sopfr)', math.pi / (SIGMA**2 * SOPFR)),  # 0.004363
        ('1/P2+1/P2^2+1/P2^3', 1.0/P2 + 1.0/P2**2 + 1.0/P2**3),
    ]

    # Combine integer + fractional
    for i_label, i_val in int_formulas:
        for f_label, f_val in frac_formulas:
            total = i_val + f_val
            err = _pct_err(total, target)
            if err <= threshold_pct:
                label = f'{i_label} + {f_label}'
                findings.append(Finding(
                    error_pct=err,
                    description=f'1/alpha = {label} = {total:.9f}',
                    predicted=total,
                    observed=target,
                    formula=label,
                    category='full 1/alpha',
                    note=f'residual={total-target:+.2e}',
                ))

    # --- Direct formulas ---
    direct = []

    # sigma^2 - n - 1 + pi/(sigma^2*n) = 137 + pi/864
    val = SIGMA**2 - N - 1 + math.pi / (SIGMA**2 * N)
    direct.append(('sigma^2-n-1 + pi/(sigma^2*n)', val))

    # sigma^2 - n - 1 + pi/(sigma*tau*sigma) = 137 + pi/576
    val = SIGMA**2 - N - 1 + math.pi / (SIGMA * TAU * SIGMA)
    direct.append(('sigma^2-n-1 + pi/(sigma^2*tau)', val))

    # Eddington-like: sigma^2 * (1 - n/sigma^2) - 1 + correction
    # = 144*(1 - 6/144) - 1 = 144 - 6 - 1 = 137 (same thing)

    # Try: sigma^2 - M3 + 1/P2 = 137.03571
    val = SIGMA**2 - M3 + 1.0 / P2
    direct.append(('sigma^2-M3 + 1/P2', val))

    # Try: sigma^2 - M3 + 1/(P2-1) = 137.03704
    val = SIGMA**2 - M3 + 1.0 / (P2 - 1)
    direct.append(('sigma^2-M3 + 1/(P2-1)', val))

    # Try: sigma^2 - M3 + pi/(sigma*P2/phi) = 137 + pi/168
    val = SIGMA**2 - M3 + math.pi / (SIGMA * P2 / PHI)
    direct.append(('sigma^2-M3 + phi*pi/(sigma*P2)', val))

    # Try: sigma^2 - M3 + 1/P2 + 1/P2^2
    val = SIGMA**2 - M3 + 1.0/P2 + 1.0/P2**2
    direct.append(('sigma^2-M3 + 1/P2+1/P2^2', val))

    # Try approaches with pi
    # pi/86.xxx?  137.036/pi = 43.614..., not clean
    val = math.pi * (SIGMA**2 + N) / (SIGMA - TAU + 1 + math.pi)
    direct.append(('pi*(sigma^2+n)/(sigma-tau+1+pi)', val))

    # sigma^2 - n - 1 + (n-sopfr)/(P2*sopfr-phi)
    denom = P2 * SOPFR - PHI
    if denom != 0:
        val = SIGMA**2 - N - 1 + (N - SOPFR) / denom
        direct.append(('sigma^2-n-1 + (n-sopfr)/(P2*sopfr-phi)', val))

    # sigma^2 - n - 1 + phi/sigma(P2) = 137 + 2/56 = 137.03571
    val = SIGMA**2 - N - 1 + PHI / SIGMA_P2
    direct.append(('sigma^2-n-1 + phi/sigma(P2)', val))

    # sigma^2 - n - 1 + pi/(sigma*M3) = 137 + pi/84
    val = SIGMA**2 - N - 1 + math.pi / (SIGMA * M3)
    direct.append(('sigma^2-n-1 + pi/(sigma*M3)', val))

    # sigma^2 - n - 1 + sopfr/(sigma*sigma-sopfr)
    val = SIGMA**2 - N - 1 + SOPFR / (SIGMA * SIGMA - SOPFR)
    direct.append(('sigma^2-n-1 + sopfr/(sigma^2-sopfr)', val))

    # Try: 137 + pi/(sigma*P2*tau/sigma)
    # = 137 + pi * sigma / (sigma * P2 * tau) = 137 + pi/(P2*tau)
    val = SIGMA**2 - N - 1 + math.pi / (P2 * TAU)
    direct.append(('sigma^2-n-1 + pi/(P2*tau)', val))

    # Try more with pi: 137 + pi/87.something?
    # 0.035999 * what = pi? -> pi/0.035999 = 87.267
    # 87 = 3*29. Not obvious. But 87.26 ~ sigma*M3 + tau/phi = 84+2=86 (no)
    # sigma*M3 = 84, sigma*M3 + sigma/tau = 84+3 = 87 -> pi/87 = 0.03612 (0.3% off)
    val = SIGMA**2 - N - 1 + math.pi / (SIGMA * M3 + SIGMA / TAU)
    direct.append(('sigma^2-n-1 + pi/(sigma*M3+sigma/tau)', val))

    # Try: sigma^2 - n - 1 + pi/(sigma*M3 + pi) = 137 + pi/87.14
    val = SIGMA**2 - N - 1 + math.pi / (SIGMA * M3 + math.pi)
    direct.append(('sigma^2-n-1 + pi/(sigma*M3+pi)', val))

    # Wyler's formula value for comparison
    direct.append(('Wyler formula', WYLER_INV_ALPHA))

    for label, val in direct:
        err = _pct_err(val, target)
        findings.append(Finding(
            error_pct=err,
            description=f'1/alpha = {label} = {val:.9f}',
            predicted=val,
            observed=target,
            formula=label,
            category='full 1/alpha',
            note=f'residual={val-target:+.2e}',
        ))

    findings.sort()
    return findings


# =====================================================================
# PART 4: Wyler's formula and comparison
# =====================================================================

def wyler_analysis() -> List[Finding]:
    """Analyze Wyler's famous near-miss formula and TECS-L variants."""
    findings = []
    target = ALPHA_INV_MEASURED

    # Wyler (1969): alpha = (9/(16*pi^3)) * (pi/120)^(1/4) * (1/pi)
    # Actually: alpha_W = (e^2/(4*pi)) where the geometric formula gives:
    # 1/alpha_W = (9/(8*pi^4)) * (pi^5/2^4*5!)^(1/4)  -- various forms exist
    # Standard form: 1/alpha_W ~ 137.03608...
    wyler_val = WYLER_INV_ALPHA
    err = _pct_err(wyler_val, target)
    findings.append(Finding(
        error_pct=err,
        description=f'Wyler: 1/alpha = {wyler_val:.6f}',
        predicted=wyler_val,
        observed=target,
        formula='Wyler (1969)',
        category='Wyler',
        note=f'Famous near-miss; residual={wyler_val-target:+.2e}',
    ))

    # Can TECS-L correct Wyler?
    # Wyler residual ~ +8.3e-5
    wyler_resid = wyler_val - target
    # Try subtracting small TECS-L terms
    corrections = [
        ('1/(sigma*P3)',        1.0 / (SIGMA * P3)),
        ('1/(sigma^2*P2)',      1.0 / (SIGMA**2 * P2)),
        ('1/(P3*tau)',          1.0 / (P3 * TAU)),
        ('phi/(sigma*P3)',      PHI / (SIGMA * P3)),
        ('1/(sigma^2*sigma)',   1.0 / SIGMA**3),
        ('1/(sigma*sigma(P2))', 1.0 / (SIGMA * SIGMA_P2)),
        ('pi/(sigma^3*P2)',     math.pi / (SIGMA**3 * P2)),
    ]

    for c_label, c_val in corrections:
        corrected = wyler_val - c_val
        err_c = _pct_err(corrected, target)
        if err_c < err:  # Only if correction improves
            findings.append(Finding(
                error_pct=err_c,
                description=f'Wyler - {c_label} = {corrected:.9f}',
                predicted=corrected,
                observed=target,
                formula=f'Wyler - {c_label}',
                category='Wyler+TECS-L',
                note=f'correction={c_val:.6e}; resid={corrected-target:+.2e}',
            ))

    findings.sort()
    return findings


# =====================================================================
# PART 5: QED perturbative coefficients and TECS-L
# =====================================================================

def qed_coefficients() -> List[Finding]:
    """Check QED perturbative expansion coefficients for TECS-L structure.

    The QED calculation:
        a_e = alpha/(2*pi) - 0.32848... * (alpha/pi)^2
              + 1.18124... * (alpha/pi)^3 - ...

    The Schwinger term alpha/(2*pi) has denominator 2*pi.
    The 2-loop coefficient involves 3/4 - pi^2/12 + ... terms.
    We check if these denominators or numerators encode n=6 arithmetic.
    """
    findings = []

    # Schwinger: coefficient of (alpha/pi) in a_e = 1/2
    # Denominator of the prefactor: 2*pi -> the "2" = phi(6)
    findings.append(Finding(
        error_pct=0.0,
        description='Schwinger 1/(2*pi): 2 = phi(6) (trivial)',
        predicted=2,
        observed=PHI,
        formula='phi(6) = 2',
        category='QED coefficients',
        note='Exact but trivially small number',
    ))

    # 2-loop: coefficient = 197/144 + ... where 144 = sigma(6)^2 = 12^2
    # Actually the full 2-loop Schwinger-type is:
    # a_e^(2) = -0.32847896... * (alpha/pi)^2
    # The rational part involves 197/144 among other terms
    # 144 = sigma^2 = 12^2  <-- this is exact
    findings.append(Finding(
        error_pct=0.0,
        description='2-loop QED denominator 144 = sigma(6)^2 (exact)',
        predicted=SIGMA**2,
        observed=144,
        formula='sigma^2 = 144',
        category='QED coefficients',
        note='Appears in a_e 2-loop rational coefficient 197/144',
    ))

    # The rational part: 197/144. Is 197 TECS-L?
    # 197 = sigma^2 + sopfr*sigma - 3 = 144 + 60 - 7 = 197 -- contrived
    # 197 is prime (45th prime)
    val_197 = SIGMA**2 + SOPFR * SIGMA - M3
    findings.append(Finding(
        error_pct=_pct_err(val_197, 197),
        description=f'197 = sigma^2 + sopfr*sigma - M3 = {val_197}',
        predicted=val_197,
        observed=197,
        formula='sigma^2 + sopfr*sigma - M3',
        category='QED coefficients',
        note='197 appears in 2-loop; 197 is 45th prime',
    ))

    # 3-loop coefficient: 1.181241... (alpha/pi)^3
    # Contains terms with zeta(3), pi^2*ln2, etc.
    # Not obviously TECS-L.

    # Running of alpha: alpha(q^2) = alpha(0) / (1 - Delta_alpha)
    # At M_Z: 1/alpha(M_Z) ~ 127.952
    alpha_inv_mz = 127.952
    # 128 = 2^7 = phi^7 = phi^(M3)
    findings.append(Finding(
        error_pct=_pct_err(128, alpha_inv_mz),
        description=f'1/alpha(M_Z) ~ 128 = phi^M3 = 2^7',
        predicted=128,
        observed=alpha_inv_mz,
        formula='phi^M3 = 2^7',
        category='QED running',
        note=f'alpha^-1(M_Z) = {alpha_inv_mz:.3f} vs 128 = {128}',
    ))

    # Difference: 1/alpha(0) - 1/alpha(M_Z) ~ 137.036 - 127.952 = 9.084
    delta_running = ALPHA_INV_MEASURED - alpha_inv_mz
    # 9 = sigma - sigma/tau = 12-3=9, or sigma*3/4 = 9
    findings.append(Finding(
        error_pct=_pct_err(9, delta_running),
        description=f'Delta(1/alpha) running ~ 9 = sigma*tau/sigma? = {delta_running:.3f}',
        predicted=9,
        observed=delta_running,
        formula='sigma - sigma/tau = 9',
        category='QED running',
        note='Running from 0 to M_Z changes 1/alpha by ~9',
    ))

    findings.sort()
    return findings


# =====================================================================
# PART 6: Texas Sharpshooter analysis
# =====================================================================

def texas_sharpshooter(findings: List[Finding], n_pool: int = 15,
                       n_ops: int = 6) -> Dict:
    """Estimate look-elsewhere effect for the best findings.

    With n_pool TECS-L parameters and n_ops operations, the number
    of candidate expressions grows combinatorially. We estimate
    how many independent expressions were tested.

    Args:
        findings: list of findings to evaluate
        n_pool: number of TECS-L parameters available
        n_ops: number of binary operations (+, -, *, /, ^, sqrt)

    Returns:
        Dictionary with trial-corrected p-values.
    """
    # Estimate number of distinct expressions tested
    # 2-operand: n_pool^2 * n_ops ~ 15^2*6 = 1350
    # 3-operand: n_pool^3 * n_ops^2 ~ 15^3*36 ~ 121,500
    # With transcendentals (5 more): (n_pool+5)^2 * n_ops = 2400 for 2-op
    n_2op = (n_pool + 5) ** 2 * n_ops
    n_3op = n_pool ** 3 * n_ops ** 2
    n_total = n_2op + n_3op

    # For each finding, estimate the probability of a random expression
    # landing within err_pct of the target
    # If target ~ 0.036, range of expressions ~ [0.001, 1.0]
    # probability of hitting within x% ~ 2 * (x/100) * target / range_width
    range_width = 1.0  # rough range for fractional expressions
    target = ALPHA_INV_FRAC

    results = {
        'n_2op_expressions': n_2op,
        'n_3op_expressions': n_3op,
        'n_total_expressions': n_total,
        'evaluations': [],
    }

    for f in findings[:20]:  # Top 20
        if f.error_pct == 0.0:
            # Exact match — likely an identity, not a coincidence
            p_trial = 0.0
            significance = float('inf')
        else:
            # Window width around target
            window = 2 * (f.error_pct / 100) * abs(f.observed)
            p_single = window / range_width
            p_trial = 1.0 - (1.0 - p_single) ** n_total
            if p_trial > 0:
                significance = abs(math.erfc(p_trial) if p_trial < 1 else 0)
            else:
                significance = float('inf')

        results['evaluations'].append({
            'description': f.description,
            'error_pct': f.error_pct,
            'p_single': window / range_width if f.error_pct > 0 else 0,
            'p_trial_corrected': min(p_trial, 1.0),
            'survives_correction': p_trial < 0.05,
        })

    return results


# =====================================================================
# PART 7: Full analysis — run all parts
# =====================================================================

def run_analysis() -> Dict:
    """Run the complete fine structure constant analysis."""
    sep = '=' * 80

    print(f'\n{sep}')
    print('  FINE STRUCTURE CONSTANT ANALYSIS — TECS-L n=6 Framework')
    print(f'{sep}')
    print(f'\n  Measured: 1/alpha = {ALPHA_INV_MEASURED} +/- {ALPHA_INV_UNCERT}')
    print(f'  Integer part:    {ALPHA_INV_INTEGER}')
    print(f'  Fractional part: {ALPHA_INV_FRAC:.9f}')
    print()

    all_findings = []

    # ==================== PART 1: Integer Part ====================
    print(f'\n{sep}')
    print('  PART 1: Integer Part — 137 from n=6')
    print(f'{sep}\n')

    int_findings = integer_part_analysis()
    all_findings.extend(int_findings)

    print(f'  {"Formula":<45s} {"Value":>6s}  {"Err%":>8s}  Note')
    print(f'  {"-"*45} {"-"*6}  {"-"*8}  {"-"*35}')
    for f in int_findings:
        marker = ' [EXACT]' if f.error_pct == 0 else ''
        print(f'  {f.formula:<45s} {f.predicted:>6.0f}  {f.error_pct:>8.4f}  '
              f'{f.note[:35]}{marker}')

    n_exact_int = sum(1 for f in int_findings if f.error_pct == 0)
    print(f'\n  Exact identities for 137: {n_exact_int}')
    print(f'  Primary: sigma(6)^2 - n - 1 = 12^2 - 6 - 1 = 137')
    print(f'  Note: tau+phi+1 = 4+2+1 = 7 = M3 (Mersenne prime 2^3-1)')
    print(f'        so sigma^2 - tau - phi - 1 = sigma^2 - M3')

    # ==================== PART 2: Fractional Part Search ====================
    print(f'\n{sep}')
    print('  PART 2: Systematic Expression Search — Fractional Part')
    print(f'{sep}\n')

    print(f'  Target: {ALPHA_INV_FRAC:.9f}')
    print()

    # Widen to 5% first to see the landscape
    frac_findings_wide = fractional_part_search(threshold_pct=5.0)
    frac_findings_narrow = [f for f in frac_findings_wide if f.error_pct <= 0.1]

    print(f'  Expressions within 0.1%: {len(frac_findings_narrow)}')
    print(f'  Expressions within 1.0%: {sum(1 for f in frac_findings_wide if f.error_pct <= 1.0)}')
    print(f'  Expressions within 5.0%: {len(frac_findings_wide)}')
    print()

    if frac_findings_narrow:
        print(f'  --- Within 0.1% ---')
        print(f'  {"#":>3s}  {"Formula":<50s} {"Value":>11s}  {"Err%":>8s}')
        print(f'  {"-"*3}  {"-"*50} {"-"*11}  {"-"*8}')
        for i, f in enumerate(frac_findings_narrow[:30], 1):
            print(f'  {i:>3d}  {f.formula:<50s} {f.predicted:>11.8f}  {f.error_pct:>8.5f}')

    # Show top within 1%
    frac_1pct = [f for f in frac_findings_wide if f.error_pct <= 1.0]
    if frac_1pct:
        print(f'\n  --- Within 1.0% (top 20) ---')
        print(f'  {"#":>3s}  {"Formula":<50s} {"Value":>11s}  {"Err%":>8s}')
        print(f'  {"-"*3}  {"-"*50} {"-"*11}  {"-"*8}')
        for i, f in enumerate(frac_1pct[:20], 1):
            print(f'  {i:>3d}  {f.formula:<50s} {f.predicted:>11.8f}  {f.error_pct:>8.5f}')

    all_findings.extend(frac_findings_wide)

    # ==================== PART 3: Full 1/alpha Formulas ====================
    print(f'\n{sep}')
    print('  PART 3: Full 1/alpha Candidate Formulas')
    print(f'{sep}\n')

    # Use wider threshold for interesting display
    full_findings = full_alpha_inv_search(threshold_pct=0.01)
    all_findings.extend(full_findings)

    print(f'  Target: 1/alpha = {ALPHA_INV_MEASURED}')
    print(f'  Formulas within 0.01%: {len(full_findings)}')
    print()

    if full_findings:
        print(f'  {"#":>3s}  {"Formula":<55s} {"Predicted":>14s}  {"Err%":>10s}  Residual')
        print(f'  {"-"*3}  {"-"*55} {"-"*14}  {"-"*10}  {"-"*12}')
        for i, f in enumerate(full_findings[:30], 1):
            resid = f.predicted - f.observed
            print(f'  {i:>3d}  {f.formula:<55s} {f.predicted:>14.9f}  '
                  f'{f.error_pct:>10.7f}  {resid:+.2e}')
    else:
        print('  No formulas within 0.01% found.')
        print('  Expanding to 0.1%...')
        full_findings = full_alpha_inv_search(threshold_pct=0.1)
        all_findings.extend(full_findings)
        if full_findings:
            print(f'  Formulas within 0.1%: {len(full_findings)}')
            print(f'  {"#":>3s}  {"Formula":<55s} {"Predicted":>14s}  {"Err%":>10s}  Residual')
            print(f'  {"-"*3}  {"-"*55} {"-"*14}  {"-"*10}  {"-"*12}')
            for i, f in enumerate(full_findings[:30], 1):
                resid = f.predicted - f.observed
                print(f'  {i:>3d}  {f.formula:<55s} {f.predicted:>14.9f}  '
                      f'{f.error_pct:>10.7f}  {resid:+.2e}')

    # ==================== PART 4: Wyler's Formula ====================
    print(f'\n{sep}')
    print('  PART 4: Wyler\'s Formula and TECS-L Corrections')
    print(f'{sep}\n')

    wyler_findings = wyler_analysis()
    all_findings.extend(wyler_findings)

    print(f'  Wyler (1969): 1/alpha = {WYLER_INV_ALPHA:.9f}')
    print(f'  Measured:     1/alpha = {ALPHA_INV_MEASURED:.9f}')
    print(f'  Wyler error:          = {_pct_err(WYLER_INV_ALPHA, ALPHA_INV_MEASURED):.6f}%')
    print(f'  Wyler residual:       = {WYLER_INV_ALPHA - ALPHA_INV_MEASURED:+.2e}')
    print()

    if wyler_findings:
        print(f'  {"#":>3s}  {"Formula":<55s} {"Predicted":>14s}  {"Err%":>10s}')
        print(f'  {"-"*3}  {"-"*55} {"-"*14}  {"-"*10}')
        for i, f in enumerate(wyler_findings[:15], 1):
            print(f'  {i:>3d}  {f.formula:<55s} {f.predicted:>14.9f}  {f.error_pct:>10.7f}')

    # ==================== PART 5: QED Coefficients ====================
    print(f'\n{sep}')
    print('  PART 5: QED Perturbative Coefficients')
    print(f'{sep}\n')

    qed_findings = qed_coefficients()
    all_findings.extend(qed_findings)

    for f in qed_findings:
        marker = ' ***' if f.error_pct < 0.1 else (' **' if f.error_pct < 1 else '')
        print(f'  {f.description:<60s}  {f.error_pct:>8.4f}%{marker}')
        if f.note:
            print(f'    {f.note}')

    # ==================== PART 6: Texas Sharpshooter ====================
    print(f'\n{sep}')
    print('  PART 6: Texas Sharpshooter Assessment')
    print(f'{sep}\n')

    # Run on fractional part findings
    frac_for_tss = frac_findings_narrow if frac_findings_narrow else frac_1pct[:10]
    if frac_for_tss:
        tss = texas_sharpshooter(frac_for_tss)
        print(f'  Expression space estimated:')
        print(f'    2-operand expressions: ~{tss["n_2op_expressions"]}')
        print(f'    3-operand expressions: ~{tss["n_3op_expressions"]}')
        print(f'    Total trial expressions: ~{tss["n_total_expressions"]}')
        print()

        print(f'  {"Finding":<50s} {"Err%":>8s}  {"p(single)":>10s}  {"p(corr)":>10s}  Survives?')
        print(f'  {"-"*50} {"-"*8}  {"-"*10}  {"-"*10}  {"-"*9}')
        for ev in tss['evaluations']:
            surv = 'YES' if ev['survives_correction'] else 'no'
            print(f'  {ev["description"][:50]:<50s} {ev["error_pct"]:>8.5f}  '
                  f'{ev["p_single"]:>10.6f}  {ev["p_trial_corrected"]:>10.6f}  {surv}')
    else:
        tss = {}
        print('  No fractional-part findings to assess.')

    # ==================== SUMMARY ====================
    print(f'\n{sep}')
    print('  SUMMARY — All Findings Ranked by Precision')
    print(f'{sep}\n')

    all_findings.sort()

    # Deduplicate
    seen_desc = set()
    unique_findings = []
    for f in all_findings:
        if f.description not in seen_desc:
            seen_desc.add(f.description)
            unique_findings.append(f)

    print(f'  {"#":>3s}  {"Finding":<55s} {"Err%":>8s}  {"Category":<20s}')
    print(f'  {"-"*3}  {"-"*55} {"-"*8}  {"-"*20}')
    for i, f in enumerate(unique_findings[:40], 1):
        marker = ' ***' if f.error_pct < 0.01 else (' **' if f.error_pct < 0.1 else (' *' if f.error_pct < 1 else ''))
        print(f'  {i:>3d}  {f.description[:55]:<55s} {f.error_pct:>8.5f}  '
              f'{f.category:<20s}{marker}')

    n_exact = sum(1 for f in unique_findings if f.error_pct < 0.01)
    n_01pct = sum(1 for f in unique_findings if f.error_pct < 0.1)
    n_1pct  = sum(1 for f in unique_findings if f.error_pct < 1.0)
    n_5pct  = sum(1 for f in unique_findings if f.error_pct < 5.0)

    print(f'\n  Exact (< 0.01%):   {n_exact}')
    print(f'  Within 0.1%:       {n_01pct}')
    print(f'  Within 1%:         {n_1pct}')
    print(f'  Within 5%:         {n_5pct}')
    print(f'  Total unique:      {len(unique_findings)}')

    # Key takeaways
    print(f'\n  --- KEY TAKEAWAYS ---')
    print(f'  1. INTEGER PART (exact):')
    print(f'     137 = sigma(6)^2 - n - 1 = 144 - 6 - 1 = 137')
    print(f'     Equivalently: sigma^2 - M3 where M3 = tau+phi+1 = 7')
    print(f'     This is an exact identity and the strongest result.')
    print(f'')
    print(f'  2. FRACTIONAL PART (0.035999084):')
    best_frac = frac_findings_wide[0] if frac_findings_wide else None
    if best_frac:
        print(f'     Best match: {best_frac.formula} = {best_frac.predicted:.8f} ({best_frac.error_pct:.4f}%)')
    print(f'     1/P2 = 1/28 = 0.035714 (0.79% off) — simplest near-miss')
    print(f'     No TECS-L expression matches the fractional part to < 0.01%')
    print(f'     without invoking pi or transcendental corrections.')
    print(f'')
    print(f'  3. QED STRUCTURE:')
    print(f'     2-loop denominator 144 = sigma(6)^2 (exact)')
    print(f'     1/alpha(M_Z) ~ 128 = phi^M3 = 2^7 (0.04% off)')
    print(f'')
    print(f'  4. WYLER COMPARISON:')
    print(f'     Wyler (1969): 1/alpha = {WYLER_INV_ALPHA:.6f} ({_pct_err(WYLER_INV_ALPHA, ALPHA_INV_MEASURED):.5f}%)')
    print(f'     TECS-L does not improve on Wyler for the fractional part.')
    print(f'')
    print(f'  5. TEXAS SHARPSHOOTER WARNING:')
    print(f'     With ~{tss.get("n_total_expressions", "125000")} trial expressions, matches within ~1%')
    print(f'     are expected by chance. Only the exact integer identity')
    print(f'     sigma^2 - n - 1 = 137 is statistically compelling.')

    print(f'\n  --- CAVEATS ---')
    print(f'  - 137 = sigma^2 - n - 1 is exact but uses three parameters')
    print(f'  - The fractional part 0.036 has no clean TECS-L formula')
    print(f'  - QED computes alpha perturbatively; it is not "derivable" from integers')
    print(f'  - Wyler\'s formula was debunked as numerology (no underlying physics)')
    print(f'  - Any improvement should predict alpha to > 10 digits to be compelling')

    print(f'\n{sep}')
    print('  END OF FINE STRUCTURE CONSTANT ANALYSIS')
    print(f'{sep}\n')

    return {
        'integer_part': int_findings,
        'fractional_part': frac_findings_wide,
        'full_formulas': full_findings,
        'wyler': wyler_findings,
        'qed': qed_findings,
        'texas_sharpshooter': tss,
        'all_findings': unique_findings,
    }


if __name__ == '__main__':
    run_analysis()
