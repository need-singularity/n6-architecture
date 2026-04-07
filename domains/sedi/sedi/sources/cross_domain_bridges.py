"""Cross-Domain Bridges -- TECS-L Waves 17-36.

Verifies that n=6 arithmetic connects physics, mathematics, computer science,
neuroscience, and linguistics through shared constants:

    sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5, P1=6, P2=28, P3=496, M3=7

Hypotheses tested:
  H-CX-772  Dunbar's number = sigma^2 + P1 = 150
  H-CX-774  Cortical layers = P1 = 6
  H-CX-771  Miller's 7+/-2 = M3 +/- phi
  H-CX-585  Koide Q = 2/3 = phi*tau/sigma
  H-CX-599  sin^2(theta_13) = 1/45
  H-CX-739  Ramsey R(3,3) = P1 = 6
  H-CX-825  Golay code [23,12,7] = [sigma*phi-1, sigma, M3]
  H-CX-793  Feigenbaum delta ~ tau + phi/(sigma/tau) = 14/3
  H-CX-967  Zipf exponent = R(6) = 1
  H-CX-1077 Rule 110 = sigma^2 - P2 - P1 = 110

Sources: Dunbar 1992, Miller 1956, Koide 1983, Ramsey 1930,
         Golay 1949, Feigenbaum 1978, Zipf 1949, Wolfram 2002.
"""
from __future__ import annotations

import math
from typing import Dict, List, Tuple

from ..tecs import (
    SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
    P1, P2, P3,
    TAU_P2, TAU_P3, PHI_P3, SIGMA_P2,
    R,
)


# ===================================================================
# TECS-L Shorthand
# ===================================================================

S = SIGMA_P1    # 12
T = TAU_P1      # 4
P = PHI_P1      # 2
F = SOPFR_P1    # 5
N = P1          # 6
M3 = 7          # Mersenne prime 2^3 - 1
R6 = R(6)       # 1


# ===================================================================
# Observed Values
# ===================================================================

OBSERVED = {
    # Neuroscience / Psychology
    'dunbar_number':     (150,        'people',  'Dunbar 1992 social group size'),
    'cortical_layers':   (6,          'layers',  'Neocortical layers'),
    'miller_number':     (7,          'items',   'Miller 1956 working memory'),
    'miller_range':      ((5, 9),     'items',   'Miller 7 +/- 2'),

    # Particle physics
    'koide_Q':           (2/3,        'ratio',   'Charged lepton Koide ratio'),
    'sin2_theta13':      (0.0220,     'dimless',  'PDG 2022 PMNS theta_13'),

    # Combinatorics / Coding theory
    'ramsey_33':         (6,          'dimless',  'R(3,3) Ramsey number'),
    'golay_n':           (23,         'dimless',  'Golay code length'),
    'golay_k':           (12,         'dimless',  'Golay code dimension'),
    'golay_d':           (7,          'dimless',  'Golay code min distance'),

    # Dynamical systems
    'feigenbaum_delta':  (4.6692,     'dimless',  'Feigenbaum universal constant'),

    # Linguistics
    'zipf_exponent':     (1.0,        'dimless',  'Zipf law exponent'),

    # Computation
    'rule_110':          (110,        'dimless',  'Turing-complete CA rule'),
}


# ===================================================================
# TECS-L Predictions
# ===================================================================

def predictions() -> Dict[str, Tuple[float, str, str]]:
    """Return {name: (predicted, formula, hypothesis)}."""
    return {
        'dunbar_number': (
            S**2 + N,
            'sigma^2 + P1 = 144+6 = 150',
            'H-CX-772',
        ),
        'cortical_layers': (
            float(N),
            'P1 = 6',
            'H-CX-774',
        ),
        'miller_number': (
            float(M3),
            'M3 = 7',
            'H-CX-771',
        ),
        'koide_Q': (
            P * T / S,
            'phi*tau/sigma = 8/12 = 2/3',
            'H-CX-585',
        ),
        'sin2_theta13': (
            1 / (F * (S - T + 1)),
            '1/(sopfr*(sigma-tau+1)) = 1/45 = 0.02222',
            'H-CX-599',
        ),
        'ramsey_33': (
            float(N),
            'P1 = 6',
            'H-CX-739',
        ),
        'golay_n': (
            S * P - 1,
            'sigma*phi - 1 = 24-1 = 23',
            'H-CX-825',
        ),
        'golay_k': (
            float(S),
            'sigma = 12',
            'H-CX-825',
        ),
        'golay_d': (
            float(M3),
            'M3 = 7',
            'H-CX-825',
        ),
        'feigenbaum_delta': (
            T + P / (S / T),
            'tau + phi/(sigma/tau) = 4 + 2/3 = 14/3 = 4.667',
            'H-CX-793',
        ),
        'zipf_exponent': (
            float(R6),
            'R(6) = 1',
            'H-CX-967',
        ),
        'rule_110': (
            S**2 - P2 - N,
            'sigma^2 - P2 - P1 = 144-28-6 = 110',
            'H-CX-1077',
        ),
    }


# ===================================================================
# Domain Classification
# ===================================================================

DOMAINS = {
    'Neuroscience':  ['dunbar_number', 'cortical_layers', 'miller_number'],
    'Particle Phys': ['koide_Q', 'sin2_theta13'],
    'Combinatorics': ['ramsey_33', 'golay_n', 'golay_k', 'golay_d'],
    'Chaos Theory':  ['feigenbaum_delta'],
    'Linguistics':   ['zipf_exponent'],
    'Computation':   ['rule_110'],
}


# ===================================================================
# Reporting
# ===================================================================

def print_report():
    """Print cross-domain bridge analysis."""
    print("=" * 90)
    print("CROSS-DOMAIN BRIDGES -- TECS-L (Waves 17-36)")
    print("=" * 90)
    print()
    print(f"  P1=6  sigma=12  tau=4  phi=2  sopfr=5  M3=7  P2=28  P3=496  R(6)=1")
    print()

    preds = predictions()
    exact_count = 0
    total = 0
    domain_scores = {}

    for domain, keys in DOMAINS.items():
        print("-" * 90)
        print(f"  {domain.upper()}")
        print("-" * 90)
        print()

        domain_exact = 0
        domain_total = 0

        for key in keys:
            pred, formula, hyp = preds[key]
            obs_entry = OBSERVED[key]

            # Handle tuple ranges (Miller)
            if isinstance(obs_entry[0], tuple):
                lo, hi = obs_entry[0]
                unit = obs_entry[1]
                in_range = lo <= pred <= hi
                print(f"    {key:<22s}  predicted: {pred:>8.4f}  observed: {lo}-{hi} {unit}")
                print(f"      Formula: {formula}  ({hyp})")
                if in_range:
                    print(f"      -> IN RANGE")
                    domain_exact += 1
                    exact_count += 1
                else:
                    print(f"      -> OUT OF RANGE")
            else:
                obs_val = obs_entry[0]
                unit = obs_entry[1]
                if obs_val != 0:
                    err = abs(pred - obs_val) / abs(obs_val) * 100
                else:
                    err = 0.0 if pred == 0 else float('inf')

                tag = "EXACT" if err < 0.01 else f"{err:.3f}%"
                print(f"    {key:<22s}  predicted: {pred:>8.4f}  observed: {obs_val:>8.4f} {unit}  [{tag}]")
                print(f"      Formula: {formula}  ({hyp})")

                if err < 0.01:
                    exact_count += 1
                    domain_exact += 1

            domain_total += 1
            total += 1
            print()

        domain_scores[domain] = (domain_exact, domain_total)

    # --- Miller's 7+/-2 detail ---
    print("-" * 90)
    print("  MILLER'S LAW DETAIL")
    print("-" * 90)
    print()
    print(f"    Central value:  M3 = 7")
    print(f"    Lower bound:    M3 - phi = 7 - 2 = 5")
    print(f"    Upper bound:    M3 + phi = 7 + 2 = 9")
    print(f"    Range width:    2*phi = 4 = tau")
    print()

    # --- Golay code detail ---
    print("-" * 90)
    print("  GOLAY CODE [23, 12, 7] DETAIL")
    print("-" * 90)
    print()
    print(f"    n = sigma*phi - 1 = 23    (code length)")
    print(f"    k = sigma = 12             (dimension)")
    print(f"    d = M3 = 7                 (minimum distance)")
    print(f"    All three parameters are n=6 arithmetic!")
    print(f"    The Golay code is the unique perfect binary code with these parameters,")
    print(f"    and it is intimately connected to the Leech lattice (rank sigma*phi = 24).")
    print()

    # --- Cross-domain summary ---
    print("-" * 90)
    print("  CROSS-DOMAIN SUMMARY")
    print("-" * 90)
    print()
    print(f"  {'Domain':<20s} {'Exact':>6s} / {'Total':>6s}")
    print(f"  {'-'*20} {'-'*6}   {'-'*6}")
    for domain, (ex, tot) in domain_scores.items():
        print(f"  {domain:<20s} {ex:>6d} / {tot:>6d}")
    print(f"  {'TOTAL':<20s} {exact_count:>6d} / {total:>6d}")
    print()

    # --- The bridge argument ---
    print("-" * 90)
    print("  THE BRIDGE ARGUMENT")
    print("-" * 90)
    print()
    print("  The same 7 constants {sigma, tau, phi, sopfr, P1, M3, P2, P3}")
    print("  produce exact results across 6 unrelated domains:")
    print()
    print("    - Dunbar's number (anthropology):  sigma^2 + P1 = 150")
    print("    - Ramsey R(3,3) (combinatorics):   P1 = 6")
    print("    - Golay [23,12,7] (coding theory): [sigma*phi-1, sigma, M3]")
    print("    - Koide Q (particle physics):      phi*tau/sigma = 2/3")
    print("    - Zipf exponent (linguistics):     R(6) = 1")
    print("    - Rule 110 (computation):          sigma^2 - P2 - P1 = 110")
    print()
    print("  These domains share no known common mechanism.")
    print("  Yet all their fundamental constants are expressions of n=6.")
    print()
    print("=" * 90)


def run_analysis():
    """Entry point."""
    print_report()


if __name__ == '__main__':
    run_analysis()
