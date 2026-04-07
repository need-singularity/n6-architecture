"""Closed Algebra of Convergence Constants — H-CX-454/502.

The nine TECS-L convergence constants form an unusually dense web of
pairwise algebraic relations (products, quotients, sums, differences).
This module enumerates every such relation within 0.5% tolerance,
counts each constant's participation, and identifies ln(2) as the
center element — the constant appearing in the most relations.

Monte Carlo validation: 10 000 random 9-element sets drawn from [0.1, 3.0]
are scored identically; the Z-score of the real set quantifies how
exceptional the algebraic closure is.
"""
import math
import numpy as np
from collections import OrderedDict, defaultdict
from itertools import combinations, combinations_with_replacement


# =====================================================================
# 1. The Nine Convergence Constants
# =====================================================================

CONSTANTS = OrderedDict([
    ('sqrt2',       1.4142135624),
    ('sqrt3',       1.7320508076),
    ('five_sixths', 5 / 6),
    ('e_val',       2.7182818285),
    ('zeta3',       1.2020569031),
    ('gz',          0.2876820725),
    ('ln2',         0.6931471806),
    ('gamma_em',    0.5772156649),
    ('one_half',    0.5),
])

CONST_NAMES = list(CONSTANTS.keys())
CONST_VALS  = list(CONSTANTS.values())
N_CONST     = len(CONST_NAMES)


# =====================================================================
# 2. Pairwise Relation Finder
# =====================================================================

def _error_pct(observed, expected):
    """Relative error in percent."""
    if expected == 0:
        return float('inf')
    return abs(observed - expected) / abs(expected) * 100


def find_pairwise_relations(tolerance_pct=0.5):
    """For all pairs (i, j) with i <= j, check whether any binary
    operation yields another constant in the set.  Returns a list of
    relation dicts sorted by absolute error."""
    relations = []
    vals = CONST_VALS
    names = CONST_NAMES

    for i in range(N_CONST):
        for j in range(i, N_CONST):
            ci, cj = vals[i], vals[j]
            ni, nj = names[i], names[j]

            candidates = [
                (ci * cj,  f'{ni} * {nj}'),
                (ci / cj,  f'{ni} / {nj}') if cj != 0 else None,
                (ci + cj,  f'{ni} + {nj}'),
            ]
            if i != j:
                candidates.append((cj / ci, f'{nj} / {ni}') if ci != 0 else None)
            # Differences — only positive results
            diff1 = ci - cj
            if diff1 > 0:
                candidates.append((diff1, f'{ni} - {nj}'))
            diff2 = cj - ci
            if diff2 > 0 and i != j:
                candidates.append((diff2, f'{nj} - {ni}'))

            for cand in candidates:
                if cand is None:
                    continue
                result_val, expr = cand
                for k in range(N_CONST):
                    err = _error_pct(result_val, vals[k])
                    if err <= tolerance_pct:
                        relations.append({
                            'expr': expr,
                            'computed': result_val,
                            'matches': names[k],
                            'expected': vals[k],
                            'error_pct': err,
                            'operands': sorted(set([ni, nj])),
                            'result': names[k],
                        })

    # Deduplicate identical (expr, matches) pairs
    seen = set()
    unique = []
    for r in relations:
        key = (r['expr'], r['matches'])
        if key not in seen:
            seen.add(key)
            unique.append(r)

    unique.sort(key=lambda r: r['error_pct'])
    return unique


# =====================================================================
# 3. Depth-2 (Triple) Relations
# =====================================================================

def find_triple_relations(tolerance_pct=0.5):
    """Check c_i * c_j * c_k = c_l for all triples (i <= j <= k)."""
    relations = []
    vals = CONST_VALS
    names = CONST_NAMES

    for i, j, k in combinations_with_replacement(range(N_CONST), 3):
        prod = vals[i] * vals[j] * vals[k]
        expr = f'{names[i]} * {names[j]} * {names[k]}'
        for l in range(N_CONST):
            err = _error_pct(prod, vals[l])
            if err <= tolerance_pct:
                relations.append({
                    'expr': expr,
                    'computed': prod,
                    'matches': names[l],
                    'expected': vals[l],
                    'error_pct': err,
                    'operands': sorted(set([names[i], names[j], names[k]])),
                    'result': names[l],
                })

    seen = set()
    unique = []
    for r in relations:
        key = (r['expr'], r['matches'])
        if key not in seen:
            seen.add(key)
            unique.append(r)

    unique.sort(key=lambda r: r['error_pct'])
    return unique


# =====================================================================
# 4. Participation Ranking
# =====================================================================

def participation_ranking(relations):
    """Count how many relations each constant participates in
    (as operand or result).  Return sorted list of (name, count)."""
    counts = defaultdict(int)
    for r in relations:
        for op in r['operands']:
            counts[op] += 1
        counts[r['result']] += 1

    ranking = sorted(counts.items(), key=lambda x: -x[1])
    return ranking


# =====================================================================
# 5. Key Known Relations — Explicit Verification
# =====================================================================

def verify_key_relations():
    """Verify the five specific relations listed in H-CX-454/502."""
    c = CONSTANTS
    checks = []

    # zeta(3) x ln(2) ~ 5/6
    val = c['zeta3'] * c['ln2']
    checks.append({
        'relation': 'zeta(3) x ln(2) ~ 5/6',
        'computed': val,
        'expected': c['five_sixths'],
        'error_pct': _error_pct(val, c['five_sixths']),
    })

    # gamma / ln(2) ~ 5/6
    val = c['gamma_em'] / c['ln2']
    checks.append({
        'relation': 'gamma / ln(2) ~ 5/6',
        'computed': val,
        'expected': c['five_sixths'],
        'error_pct': _error_pct(val, c['five_sixths']),
    })

    # zeta(3) x ln^2(2) ~ gamma
    val = c['zeta3'] * c['ln2'] ** 2
    checks.append({
        'relation': 'zeta(3) x ln^2(2) ~ gamma',
        'computed': val,
        'expected': c['gamma_em'],
        'error_pct': _error_pct(val, c['gamma_em']),
    })

    # sqrt(3) x gamma ~ 1
    val = c['sqrt3'] * c['gamma_em']
    checks.append({
        'relation': 'sqrt(3) x gamma ~ 1',
        'computed': val,
        'expected': 1.0,
        'error_pct': _error_pct(val, 1.0),
    })

    # GZ / ln(2) ~ sqrt(2) - 1
    val = c['gz'] / c['ln2']
    target = math.sqrt(2) - 1  # 0.41421...
    checks.append({
        'relation': 'GZ / ln(2) ~ sqrt(2) - 1  [0.41421]',
        'computed': val,
        'expected': target,
        'error_pct': _error_pct(val, target),
    })

    return checks


# =====================================================================
# 6. Monte Carlo — Z-score for algebraic density
# =====================================================================

def _count_relations_for_set(vals, tolerance_pct=0.5):
    """Count how many pairwise relations a set of 9 values satisfies."""
    n = len(vals)
    count = 0
    for i in range(n):
        for j in range(i, n):
            ci, cj = vals[i], vals[j]
            candidates = [ci * cj, ci + cj]
            if cj != 0:
                candidates.append(ci / cj)
            if i != j and ci != 0:
                candidates.append(cj / ci)
            d1 = ci - cj
            if d1 > 0:
                candidates.append(d1)
            d2 = cj - ci
            if d2 > 0 and i != j:
                candidates.append(d2)

            for cand in candidates:
                for k in range(n):
                    if vals[k] != 0 and _error_pct(cand, vals[k]) <= tolerance_pct:
                        count += 1
                        break  # one match per candidate suffices
    return count


def monte_carlo_zscore(n_trials=10_000, seed=42):
    """Generate random 9-element sets in [0.1, 3.0], count pairwise
    relations in each, and compute the Z-score of the real set."""
    rng = np.random.default_rng(seed)

    real_count = _count_relations_for_set(CONST_VALS)

    random_counts = np.empty(n_trials, dtype=int)
    for t in range(n_trials):
        fake = rng.uniform(0.1, 3.0, size=N_CONST).tolist()
        random_counts[t] = _count_relations_for_set(fake)

    mu = random_counts.mean()
    sigma = random_counts.std()
    z_score = (real_count - mu) / sigma if sigma > 0 else float('inf')

    return {
        'real_count': real_count,
        'mc_mean': mu,
        'mc_std': sigma,
        'z_score': z_score,
        'n_trials': n_trials,
        'mc_max': int(random_counts.max()),
        'mc_min': int(random_counts.min()),
    }


# =====================================================================
# 7. Full Report
# =====================================================================

def run_analysis():
    """Run all closed-algebra analyses and print comprehensive report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  CLOSED ALGEBRA OF CONVERGENCE CONSTANTS — H-CX-454/502')
    print(sep)

    # --- 1. Constants ---
    print('\n--- 1. The Nine Convergence Constants ---\n')
    print(f'  {"Name":<16s} {"Value":>14s}')
    print(f'  {"-"*16} {"-"*14}')
    for name, val in CONSTANTS.items():
        print(f'  {name:<16s} {val:>14.10f}')

    # --- 2. Pairwise relations ---
    print(f'\n--- 2. Pairwise Algebraic Relations (tolerance 0.5%) ---\n')
    pair_rels = find_pairwise_relations()
    print(f'  Found {len(pair_rels)} pairwise relations.\n')
    print(f'  {"Expression":<32s} {"= Computed":>12s} {"~ Constant":<16s} '
          f'{"Expected":>12s} {"Err%":>8s}')
    print(f'  {"-"*32} {"-"*12} {"-"*16} {"-"*12} {"-"*8}')
    for r in pair_rels:
        print(f'  {r["expr"]:<32s} {r["computed"]:>12.8f} {r["matches"]:<16s} '
              f'{r["expected"]:>12.8f} {r["error_pct"]:>8.4f}')

    # --- 3. Triple relations ---
    print(f'\n--- 3. Depth-2 (Triple Product) Relations ---\n')
    trip_rels = find_triple_relations()
    print(f'  Found {len(trip_rels)} triple-product relations.\n')
    print(f'  {"Expression":<40s} {"= Computed":>12s} {"~ Constant":<16s} '
          f'{"Err%":>8s}')
    print(f'  {"-"*40} {"-"*12} {"-"*16} {"-"*8}')
    for r in trip_rels:
        print(f'  {r["expr"]:<40s} {r["computed"]:>12.8f} {r["matches"]:<16s} '
              f'{r["error_pct"]:>8.4f}')

    # --- 4. Participation ranking ---
    all_rels = pair_rels + trip_rels
    print(f'\n--- 4. Participation Ranking (operand or result) ---\n')
    ranking = participation_ranking(all_rels)
    print(f'  {"Constant":<16s} {"Relations":>10s}')
    print(f'  {"-"*16} {"-"*10}')
    for name, count in ranking:
        marker = '  <-- CENTER' if name == ranking[0][0] else ''
        print(f'  {name:<16s} {count:>10d}{marker}')
    center = ranking[0][0]
    print(f'\n  Center element: {center}  '
          f'(participates in {ranking[0][1]} relations)')

    # --- 5. Key relations ---
    print(f'\n--- 5. Key Known Relations — Explicit Verification ---\n')
    checks = verify_key_relations()
    for ch in checks:
        status = 'PASS' if ch['error_pct'] < 5 else 'MARGINAL'
        print(f'  {ch["relation"]}')
        print(f'    computed = {ch["computed"]:.10f}   '
              f'expected = {ch["expected"]:.10f}   '
              f'error = {ch["error_pct"]:.4f}%   [{status}]')

    # --- 6. Monte Carlo Z-score ---
    print(f'\n--- 6. Monte Carlo Z-score (10 000 random sets) ---\n')
    mc = monte_carlo_zscore()
    print(f'  Real set relation count:   {mc["real_count"]}')
    print(f'  Random sets mean:          {mc["mc_mean"]:.2f}')
    print(f'  Random sets std:           {mc["mc_std"]:.2f}')
    print(f'  Random sets range:         [{mc["mc_min"]}, {mc["mc_max"]}]')
    print(f'  Z-score:                   {mc["z_score"]:.2f}')

    # --- Summary ---
    print(f'\n{sep}')
    print('  SUMMARY')
    print(sep)
    print(f'''
  Nine convergence constants produce {len(pair_rels)} pairwise and
  {len(trip_rels)} triple-product algebraic relations (0.5% tolerance).

  Participation ranking places "{center}" at the center with
  {ranking[0][1]} relations — it is the algebraic hub of the set.

  Key identities verified:
    zeta(3) x ln(2) = 5/6          ({checks[0]["error_pct"]:.4f}%)
    gamma / ln(2)   = 5/6          ({checks[1]["error_pct"]:.4f}%)
    zeta(3) x ln^2(2) = gamma      ({checks[2]["error_pct"]:.4f}%)
    sqrt(3) x gamma = 1            ({checks[3]["error_pct"]:.4f}%)
    GZ / ln(2) = sqrt(2)-1         ({checks[4]["error_pct"]:.4f}%)

  Monte Carlo: real set has {mc["real_count"]} relations vs random
  mean {mc["mc_mean"]:.1f} +/- {mc["mc_std"]:.1f}  =>  Z = {mc["z_score"]:.2f}
''')
    print(sep)

    return {
        'pairwise': pair_rels,
        'triples': trip_rels,
        'ranking': ranking,
        'center': center,
        'key_checks': checks,
        'mc': mc,
    }


if __name__ == '__main__':
    run_analysis()
