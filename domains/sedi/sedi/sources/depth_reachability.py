"""Depth Reachability Analysis — H-CX-475/489.

Computes depth-1 and depth-2 reachability matrices (8 domains x 9 targets)
and their matrix ranks.  Depth-1 checks whether any single constant in a
domain matches a target within 0.1 %.  Depth-2 checks whether any binary
arithmetic combination (a op b, op in {+,-,*,/}) of two domain constants
reaches the target.

The rank of each binary reachability matrix is compared to the fermion
generation count (3).
"""
import math
from collections import OrderedDict
from itertools import product as iprod

import numpy as np

# =====================================================================
# 1. Eight Mathematical Domains (N, A, G, T, C, Q, I, S)
#    matching the original H-CX-453 convergence engine domains
# =====================================================================

DOMAINS = OrderedDict([
    ('N (Number Theory)',  [12, 4, 2, 5, 6, 6, 28, 496, 7, 24, 21, 1, 3, 8]),
    ('A (Analysis)',       [math.e, math.pi, 0.5772156649, 1.2020569031,
                           math.log(2), math.log(3), math.log(4/3),
                           math.sqrt(2), math.sqrt(3), 1.6180339887]),
    ('G (Algebra)',        [3, 8, 248, 10, 26]),
    ('T (Topology)',       [12, 10, 11, 26, 2, 4]),
    ('C (Combinatorics)',  [8, 20, 4.6692, 2.5029, 1, 2, 3, 5, 13]),
    ('Q (Quantum Mech)',   [137.036, 0.1185, 0.231, 80.379, 91.188,
                           125.25, 172.76, 0.000511, 0.1057, 1.777]),
    ('I (Quantum Info)',   [1.4427, 1.3863, math.log(2), 1.0, 0.6309]),
    ('S (Stat Mech)',      [0.27, 2.269, 0.6300, 0.3265, 1.2372, 4.789]),
])

DOMAIN_NAMES = list(DOMAINS.keys())


# =====================================================================
# 2. Nine Targets
# =====================================================================

TARGETS = OrderedDict([
    ('sqrt2',   math.sqrt(2)),       # 1.41421
    ('sqrt3',   math.sqrt(3)),       # 1.73205
    ('5/6',     5 / 6),             # 0.83333
    ('e',       math.e),             # 2.71828
    ('zeta3',   1.20206),            # Apery's constant
    ('GZ',      0.28768),            # Glaisher-Kinkelin analog / Gauss-Zagier
    ('ln2',     math.log(2)),        # 0.69315
    ('gamma',   0.57722),            # Euler-Mascheroni
    ('1/2',     0.5),                # phi(6)/tau(6)
])

TARGET_NAMES = list(TARGETS.keys())
TARGET_VALS = np.array(list(TARGETS.values()))


# =====================================================================
# 3. Matching Predicate
# =====================================================================

TOLERANCE = 0.001  # 0.1 %


def _matches(value, target):
    """Return True if value is within 0.1 % of target (relative)."""
    if target == 0:
        return abs(value) < 1e-12
    return abs(value - target) / abs(target) < TOLERANCE


# =====================================================================
# 4. Depth-1 Reachability
# =====================================================================

def depth1_matrix():
    """Build 8 x 9 binary matrix: domain reaches target via single constant."""
    n_dom = len(DOMAINS)
    n_tgt = len(TARGETS)
    mat = np.zeros((n_dom, n_tgt), dtype=int)
    for i, constants in enumerate(DOMAINS.values()):
        for j, tval in enumerate(TARGET_VALS):
            for c in constants:
                if _matches(c, tval):
                    mat[i, j] = 1
                    break
    return mat


# =====================================================================
# 5. Depth-2 Reachability
# =====================================================================

_OPS = [
    ('+', lambda a, b: a + b),
    ('-', lambda a, b: a - b),
    ('*', lambda a, b: a * b),
    ('/', lambda a, b: a / b if b != 0 else None),
]


def depth2_matrix():
    """Build 8 x 9 binary matrix: domain reaches target via (a op b)."""
    n_dom = len(DOMAINS)
    n_tgt = len(TARGETS)
    mat = np.zeros((n_dom, n_tgt), dtype=int)
    for i, constants in enumerate(DOMAINS.values()):
        for j, tval in enumerate(TARGET_VALS):
            if _depth2_hit(constants, tval):
                mat[i, j] = 1
    return mat


def _depth2_hit(constants, target):
    """Check whether any (a op b) for a, b in constants hits target."""
    for a, b in iprod(constants, repeat=2):
        for _, fn in _OPS:
            result = fn(a, b)
            if result is not None and math.isfinite(result):
                if _matches(result, target):
                    return True
    return False


# =====================================================================
# 6. Pretty Printing
# =====================================================================

def _print_matrix(mat, title):
    """Print a reachability matrix with check-mark formatting."""
    rank = int(np.linalg.matrix_rank(mat))
    print(f'\n  {title}')
    print(f'  Rank = {rank}\n')

    # Header
    hdr = '  ' + f'{"Domain":<16s}'
    for tn in TARGET_NAMES:
        hdr += f'{tn:>8s}'
    hdr += '  hits'
    print(hdr)
    print('  ' + '-' * len(hdr.strip()))

    for i, dn in enumerate(DOMAIN_NAMES):
        row = f'  {dn:<16s}'
        hits = 0
        for j in range(len(TARGET_NAMES)):
            sym = '\u2713' if mat[i, j] else '\u00b7'
            row += f'{sym:>8s}'
            hits += mat[i, j]
        row += f'{hits:>5d}'
        print(row)

    # Column totals
    col_totals = mat.sum(axis=0)
    footer = '  ' + f'{"(total)":<16s}'
    for ct in col_totals:
        footer += f'{int(ct):>8d}'
    footer += f'{int(mat.sum()):>5d}'
    print(footer)

    return rank


# =====================================================================
# 7. Dark-Domain Detection
# =====================================================================

def _dark_domains(mat, depth_label):
    """List domains with zero reachability at given depth."""
    dark = []
    for i, dn in enumerate(DOMAIN_NAMES):
        if mat[i].sum() == 0:
            dark.append(dn)
    if dark:
        print(f'\n  Dark domains at {depth_label}: {", ".join(dark)}')
    else:
        print(f'\n  No dark domains at {depth_label} (all domains reach >= 1 target).')
    return dark


# =====================================================================
# 8. Full Analysis
# =====================================================================

def run_analysis():
    """Run depth-reachability analysis and print comprehensive report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  DEPTH REACHABILITY ANALYSIS — H-CX-475/489')
    print(sep)

    # --- Domain catalog ---
    print('\n--- 1. Domain Catalog (8 domains) ---\n')
    for dn, consts in DOMAINS.items():
        vals = ', '.join(f'{c:.5g}' for c in consts)
        print(f'  {dn:<16s}  [{vals}]')

    # --- Target catalog ---
    print('\n--- 2. Target Vector (9 targets) ---\n')
    for tn, tv in TARGETS.items():
        print(f'  {tn:<10s}  {tv:.5f}')

    # --- Depth-1 ---
    print(f'\n--- 3. Depth-1 Reachability (single constant match, tol={TOLERANCE*100:.1f}%) ---')
    m1 = depth1_matrix()
    rank1 = _print_matrix(m1, 'Depth-1 Matrix')
    dark1 = _dark_domains(m1, 'depth-1')

    # --- Depth-2 ---
    print(f'\n--- 4. Depth-2 Reachability (a op b, tol={TOLERANCE*100:.1f}%) ---')
    m2 = depth2_matrix()
    rank2 = _print_matrix(m2, 'Depth-2 Matrix')
    dark2 = _dark_domains(m2, 'depth-2')

    # --- Rank comparison ---
    fermion_generations = 3
    print(f'\n--- 5. Rank vs Fermion Generation Count ---\n')
    print(f'  Depth-1 rank:          {rank1}')
    print(f'  Depth-2 rank:          {rank2}')
    print(f'  Fermion generations:   {fermion_generations}')
    print(f'  rank_1 == 3?           {"YES" if rank1 == fermion_generations else "NO"}'
          f'  (rank_1 = {rank1})')
    print(f'  rank_2 == 3?           {"YES" if rank2 == fermion_generations else "NO"}'
          f'  (rank_2 = {rank2})')
    delta = rank2 - rank1
    print(f'  rank_2 - rank_1:       {delta}')

    # --- Summary ---
    print(f'\n{sep}')
    print('  SUMMARY')
    print(sep)
    total_1 = int(m1.sum())
    total_2 = int(m2.sum())
    print(f'''
  Depth-1: {total_1}/{m1.size} cells reachable, rank {rank1}, dark: {len(dark1)} domains
  Depth-2: {total_2}/{m2.size} cells reachable, rank {rank2}, dark: {len(dark2)} domains

  The depth-2 combinatorial expansion lifts rank from {rank1} -> {rank2}.
  Fermion generation count = {fermion_generations}.
''')
    print(sep)

    return {
        'depth1_matrix': m1,
        'depth2_matrix': m2,
        'rank1': rank1,
        'rank2': rank2,
        'dark1': dark1,
        'dark2': dark2,
        'fermion_generations': fermion_generations,
    }


if __name__ == '__main__':
    run_analysis()
