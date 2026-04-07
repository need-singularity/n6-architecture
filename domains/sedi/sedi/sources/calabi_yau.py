"""Calabi-Yau Hodge Number Analysis — CY threefolds through TECS-L n=6 arithmetic.

H-CX-504: Analyse Calabi-Yau manifold Hodge numbers (h11, h21) and check
whether the Tian-Yau manifold and other three-generation CY threefolds
exhibit n=6 arithmetic structure.

Key result: the Tian-Yau manifold (h11=6, h21=9) saturates n=6 relations,
yielding exactly 3 = sigma/tau fermion generations.
"""
import math
from collections import OrderedDict
from ..tecs import (
    P1, P2, SIGMA_P1, TAU_P1, PHI_P1, SOPFR_P1,
)


# =====================================================================
# 1. n=6 Arithmetic Constants
# =====================================================================

N = P1                          # 6
SIGMA = SIGMA_P1                # 12
TAU = TAU_P1                    # 4
PHI = PHI_P1                    # 2
SOPFR = SOPFR_P1                # 5
P2_VAL = P2                     # 28

# Derived
SIGMA_OVER_TAU = SIGMA // TAU   # 3
SIGMA_PLUS_TAU = SIGMA + TAU    # 16
SIGMA_MINUS_TAU = SIGMA - TAU   # 8


# =====================================================================
# 2. Calabi-Yau Threefold Utilities
# =====================================================================

def euler_number(h11, h21):
    """Euler characteristic of a CY threefold: chi = 2(h11 - h21)."""
    return 2 * (h11 - h21)


def generations(h11, h21):
    """Number of fermion generations from heterotic string: |chi/2| = |h11 - h21|."""
    return abs(h11 - h21)


def total_moduli(h11, h21):
    """Total moduli count: h11 + h21."""
    return h11 + h21


def betti_3(h21):
    """Third Betti number of a CY threefold: b3 = 2 + 2*h21."""
    return 2 + 2 * h21


# =====================================================================
# 3. TECS-L Expressibility Scoring
# =====================================================================

# Dictionary of n=6 arithmetic expressions and their values
TECS_EXPRESSIONS = OrderedDict([
    ('n',                   N),                         # 6
    ('sigma',               SIGMA),                     # 12
    ('tau',                 TAU),                        # 4
    ('phi',                 PHI),                        # 2
    ('sopfr',               SOPFR),                      # 5
    ('P2',                  P2_VAL),                     # 28
    ('sigma/tau',           SIGMA_OVER_TAU),             # 3
    ('sigma-tau',           SIGMA_MINUS_TAU),            # 8
    ('sigma+tau',           SIGMA_PLUS_TAU),             # 16
    ('sigma*phi',           SIGMA * PHI),                # 24
    ('tau!',                math.factorial(TAU)),         # 24
    ('sigma+sigma/tau',     SIGMA + SIGMA_OVER_TAU),     # 15
    ('sigma+2*tau',         SIGMA + 2 * TAU),            # 20
    ('2*(sigma/tau)**3',    2 * SIGMA_OVER_TAU ** 3),    # 54
    ('(sigma/tau)**2',      SIGMA_OVER_TAU ** 2),        # 9
    ('2*sigma',             2 * SIGMA),                  # 24
    ('n*(n-1)/2',           N * (N - 1) // 2),           # 15
    ('C(6,3)',              math.comb(N, 3)),             # 20
    ('n-1',                 N - 1),                      # 5
    ('n+1',                 N + 1),                      # 7
    ('2*n',                 2 * N),                      # 12
    ('3*n',                 3 * N),                      # 18
    ('n**2',                N ** 2),                     # 36
    ('sigma**2',            SIGMA ** 2),                 # 144
    ('tau**2',              TAU ** 2),                   # 16
    ('-n',                  -N),                         # -6
    ('-sigma',              -SIGMA),                     # -12
    ('2*sigma/tau',         2 * SIGMA_OVER_TAU),         # 6
    ('phi*tau',             PHI * TAU),                  # 8
    ('d_M-theory=11',       11),
])


def _tecs_score_value(value):
    """Return list of TECS-L expression names that produce the given value."""
    matches = []
    for name, expr_val in TECS_EXPRESSIONS.items():
        if expr_val == value:
            matches.append(name)
    return matches


def tecs_score(h11, h21):
    """Compute how many CY-derived quantities are expressible in n=6 arithmetic.

    Returns (score, details) where details lists each property and its matches.
    """
    chi = euler_number(h11, h21)
    gens = generations(h11, h21)
    mods = total_moduli(h11, h21)
    b3 = betti_3(h21)
    product = h11 * h21
    diff = h21 - h11

    properties = OrderedDict([
        ('h11',         h11),
        ('h21',         h21),
        ('chi',         chi),
        ('|chi/2|',     gens),
        ('h21-h11',     diff),
        ('h11+h21',     mods),
        ('h11*h21',     product),
        ('b3',          b3),
    ])

    score = 0
    details = []
    for prop_name, prop_val in properties.items():
        matches = _tecs_score_value(prop_val)
        if matches:
            score += 1
            details.append((prop_name, prop_val, matches))

    return score, details


# =====================================================================
# 4. Tian-Yau Manifold Analysis
# =====================================================================

TIAN_YAU = (6, 9)  # (h11, h21)


def analyze_tian_yau():
    """Detailed analysis of the Tian-Yau manifold (h11=6, h21=9)."""
    h11, h21 = TIAN_YAU
    chi = euler_number(h11, h21)
    gens = generations(h11, h21)
    mods = total_moduli(h11, h21)
    b3 = betti_3(h21)
    product = h11 * h21
    diff = h21 - h11

    results = OrderedDict([
        ('h11',     (h11,    f'{N} = P1')),
        ('h21',     (h21,    f'{SIGMA_OVER_TAU}^2 = (sigma/tau)^2')),
        ('chi',     (chi,    f'-{N} = -P1')),
        ('|chi/2|', (gens,   f'{SIGMA_OVER_TAU} = sigma/tau = fermion generations')),
        ('h21-h11', (diff,   f'{SIGMA_OVER_TAU} = generations')),
        ('h11+h21', (mods,   f'{SIGMA} + {SIGMA_OVER_TAU} = sigma + sigma/tau')),
        ('h11*h21', (product, f'2 * ({SIGMA_OVER_TAU})^3 = 2*(sigma/tau)^3')),
        ('b3',      (b3,     f'{SIGMA} + 2*{TAU} = sigma + 2*tau = C(6,3)')),
    ])
    return results


# =====================================================================
# 5. Three-Generation Search
# =====================================================================

def three_generation_search(max_h=50):
    """Find all (h11, h21) pairs with |h11 - h21| = 3 and h11, h21 <= max_h.

    Returns list of dicts sorted by TECS-L score descending.
    """
    candidates = []
    for h11 in range(1, max_h + 1):
        for h21 in range(1, max_h + 1):
            if abs(h11 - h21) != 3:
                continue
            sc, det = tecs_score(h11, h21)
            candidates.append({
                'h11': h11,
                'h21': h21,
                'chi': euler_number(h11, h21),
                'moduli': total_moduli(h11, h21),
                'b3': betti_3(h21),
                'product': h11 * h21,
                'tecs_score': sc,
                'tecs_details': det,
            })
    candidates.sort(key=lambda c: (-c['tecs_score'], c['moduli']))
    return candidates


# =====================================================================
# 6. (6, 17) Hypothesis Comparison
# =====================================================================

def analyze_6_17():
    """Analyze the (h11=6, h21=17) hypothesis."""
    h11, h21 = 6, 17
    chi = euler_number(h11, h21)
    gens = generations(h11, h21)
    mods = total_moduli(h11, h21)
    b3 = betti_3(h21)

    return OrderedDict([
        ('h11',     (h11,  f'{N} = P1')),
        ('h21',     (h21,  '17 = prime, no simple n=6 expression')),
        ('chi',     (chi,  f'{chi}')),
        ('|chi/2|', (gens, f'{gens} = d_M-theory (M-theory spacetime dimension)')),
        ('note',    (None, 'Gives M-theory dimension 11, NOT SM generations 3')),
        ('h11+h21', (mods, f'{mods}')),
        ('b3',      (b3,   f'{b3} = 2 + 2*17 = 36 = n^2')),
    ])


# =====================================================================
# Full Report
# =====================================================================

def run_analysis():
    """Run all Calabi-Yau Hodge number analyses and print comprehensive report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  CALABI-YAU HODGE NUMBER ANALYSIS — CY Threefolds & TECS-L n=6 Arithmetic')
    print(f'  H-CX-504')
    print(sep)

    # --- 1. n=6 Arithmetic definitions ---
    print('\n--- 1. n=6 Arithmetic Definitions ---\n')
    print(f'  sigma(6) = {SIGMA:>3d}    (sum of divisors)')
    print(f'  tau(6)   = {TAU:>3d}    (number of divisors)')
    print(f'  phi(6)   = {PHI:>3d}    (Euler totient)')
    print(f'  sopfr(6) = {SOPFR:>3d}    (sum of prime factors with repetition)')
    print(f'  n        = {N:>3d}')
    print(f'  P2       = {P2_VAL:>3d}    (second perfect number)')

    # --- 2. CY threefold formulae ---
    print('\n--- 2. CY Threefold Formulae ---\n')
    print('  For a CY threefold with Hodge numbers (h11, h21):')
    print('    Euler number:        chi = 2(h11 - h21)')
    print('    Generations:         |chi/2| = |h11 - h21|  (heterotic string)')
    print('    Total moduli:        h11 + h21')
    print('    Third Betti number:  b3 = 2 + 2*h21')

    # --- 3. Tian-Yau manifold ---
    print(f'\n--- 3. Tian-Yau Manifold (h11=6, h21=9) ---\n')
    ty = analyze_tian_yau()
    for prop, (val, expr) in ty.items():
        if val is not None:
            check = 'OK' if _tecs_score_value(val) else '--'
            print(f'  {prop:<10s} = {val:>6}    {expr:<45s} [{check}]')
        else:
            print(f'  {prop:<10s}   {expr}')

    print()
    print('  Verification:')
    h11, h21 = TIAN_YAU
    print(f'    h11 = {h11} = P1 = n                              : {h11 == N}')
    print(f'    h21 = {h21} = (sigma/tau)^2 = 3^2                 : {h21 == SIGMA_OVER_TAU**2}')
    print(f'    chi = {euler_number(h11,h21)} = -P1                               : {euler_number(h11,h21) == -N}')
    print(f'    |chi/2| = {generations(h11,h21)} = sigma/tau = fermion generations  : {generations(h11,h21) == SIGMA_OVER_TAU}')
    print(f'    h21-h11 = {h21-h11} = generations                       : {h21-h11 == SIGMA_OVER_TAU}')
    print(f'    h11+h21 = {total_moduli(h11,h21)} = sigma + sigma/tau = 12 + 3       : {total_moduli(h11,h21) == SIGMA + SIGMA_OVER_TAU}')
    print(f'    h11*h21 = {h11*h21} = 2*(sigma/tau)^3 = 2*27            : {h11*h21 == 2*SIGMA_OVER_TAU**3}')
    print(f'    b3      = {betti_3(h21)} = sigma + 2*tau = C(6,3) = 20       : {betti_3(h21) == SIGMA + 2*TAU}')

    # --- 4. Three-generation search ---
    print(f'\n--- 4. Three-Generation CY Manifolds (|chi/2|=3, h11,h21 <= 50) ---\n')
    candidates = three_generation_search(max_h=50)
    print(f'  Total three-generation pairs found: {len(candidates)}\n')

    print(f'  {"h11":>4s} {"h21":>4s} {"chi":>5s} {"mod":>4s} {"b3":>4s} {"prod":>5s} '
          f'{"score":>5s}  TECS-L matches')
    print(f'  {"-"*4} {"-"*4} {"-"*5} {"-"*4} {"-"*4} {"-"*5} '
          f'{"-"*5}  {"-"*30}')
    for c in candidates:
        detail_str = '; '.join(
            f'{pn}={pv}[{",".join(ms)}]'
            for pn, pv, ms in c['tecs_details']
        )
        print(f'  {c["h11"]:>4d} {c["h21"]:>4d} {c["chi"]:>5d} '
              f'{c["moduli"]:>4d} {c["b3"]:>4d} {c["product"]:>5d} '
              f'{c["tecs_score"]:>5d}  {detail_str}')

    # --- 5. (6, 17) hypothesis ---
    print(f'\n--- 5. Comparison: (h11=6, h21=17) Hypothesis ---\n')
    hyp = analyze_6_17()
    for prop, (val, expr) in hyp.items():
        if val is not None:
            print(f'  {prop:<10s} = {val:>6}    {expr}')
        else:
            print(f'  {prop:<10s}   {expr}')
    print()
    print('  Conclusion: (6, 17) yields |chi/2| = 11 = d_M-theory, not 3 SM generations.')
    print('  It encodes M-theory dimensionality rather than the Standard Model spectrum.')

    # --- 6. Top 10 ranking ---
    print(f'\n--- 6. Top 10 CY Manifolds by TECS-L Score ---\n')
    top10 = candidates[:10]
    print(f'  {"Rank":>4s} {"h11":>4s} {"h21":>4s} {"score":>5s}  '
          f'{"Properties in n=6 arithmetic"}')
    print(f'  {"-"*4} {"-"*4} {"-"*4} {"-"*5}  {"-"*45}')
    for i, c in enumerate(top10, 1):
        detail_str = '; '.join(
            f'{pn}={pv}[{",".join(ms)}]'
            for pn, pv, ms in c['tecs_details']
        )
        print(f'  {i:>4d} {c["h11"]:>4d} {c["h21"]:>4d} {c["tecs_score"]:>5d}  {detail_str}')

    # --- Summary ---
    print(f'\n{sep}')
    print('  SUMMARY')
    print(sep)

    ty_score, _ = tecs_score(*TIAN_YAU)
    hyp_score, _ = tecs_score(6, 17)
    best = top10[0] if top10 else None

    print(f'''
  The Tian-Yau manifold (h11=6, h21=9) achieves TECS-L score {ty_score}:
    - h11 = 6 = n = P1
    - h21 = 9 = (sigma/tau)^2
    - Exactly 3 = sigma/tau fermion generations
    - b3 = 20 = C(6,3) = sigma + 2*tau
    - h11 * h21 = 54 = 2*(sigma/tau)^3

  Among all {len(candidates)} three-generation CY pairs (h11,h21 <= 50):
    - Best TECS-L score: {best['tecs_score'] if best else 'N/A'} at ({best['h11']},{best['h21']}) {'= Tian-Yau' if best and (best['h11'],best['h21'])==TIAN_YAU else '' if best else ''}
    - The Tian-Yau manifold is {'the top-ranked' if best and (best['h11'],best['h21'])==TIAN_YAU else 'ranked'} by TECS-L score

  Comparison with (6, 17): score {hyp_score}, gives |chi/2| = 11 = d_M-theory
    (M-theory dimension, not SM generations)
''')
    print(sep)

    return {
        'tian_yau': ty,
        'three_gen_candidates': candidates,
        'hypothesis_6_17': hyp,
        'top10': top10,
    }


if __name__ == '__main__':
    run_analysis()
