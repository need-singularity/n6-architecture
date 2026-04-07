"""Convergence Engine — H-CX-453: multi-domain constant reachability analysis.

Searches for mathematical constants reachable from multiple independent domains.
For each target value, we check whether it can be constructed from the constants
of each domain independently (depth-1 and depth-2 arithmetic), count cross-domain
bridges, and score the convergence.  A Texas Sharpshooter Monte Carlo test
quantifies whether the observed convergence exceeds chance.
"""
import math
import numpy as np
from collections import OrderedDict
from itertools import combinations, product


# =====================================================================
# 1. Domain definitions
# =====================================================================

DOMAINS = OrderedDict([
    ('N', {
        'name': 'Number Theory',
        'constants': OrderedDict([
            ('sigma(6)',   12),
            ('tau(6)',      4),
            ('phi(6)',      2),
            ('sopfr(6)',    5),
            ('n',           6),
            ('s(6)',        6),
            ('P2',         28),
            ('P3',        496),
            ('M3',          7),
            ('sigma*phi',  24),
            ('T(6)',       21),
            ('1',           1),
            ('3',           3),
            ('sigma-tau',   8),
        ]),
    }),
    ('A', {
        'name': 'Analysis',
        'constants': OrderedDict([
            ('e',          2.71828),
            ('pi',         3.14159),
            ('gamma',      0.57722),
            ('zeta(3)',    1.20206),
            ('ln2',        0.69315),
            ('ln3',        1.09861),
            ('ln(4/3)',    0.28768),
            ('sqrt2',      1.41421),
            ('sqrt3',      1.73205),
            ('phi_golden', 1.61803),
        ]),
    }),
    ('G', {
        'name': 'Algebra / Groups',
        'constants': OrderedDict([
            ('dim_SU2',      3),
            ('dim_SU3',      8),
            ('dim_E8',     248),
            ('d_super',     10),
            ('d_bosonic',   26),
        ]),
    }),
    ('T', {
        'name': 'Topology / Geometry',
        'constants': OrderedDict([
            ('kissing_3d', 12),
            ('10',         10),
            ('d_M',        11),
            ('26',         26),
            ('2',           2),
            ('4',           4),
        ]),
    }),
    ('C', {
        'name': 'Combinatorics',
        'constants': OrderedDict([
            ('F6',          8),
            ('C63',        20),
            ('Feigenbaum', 4.669),
            ('alpha_F',    2.502),
            ('1',           1),
            ('2',           2),
            ('3',           3),
            ('5',           5),
            ('13',         13),
        ]),
    }),
    ('Q', {
        'name': 'Quantum Mechanics',
        'constants': OrderedDict([
            ('1/alpha',  137.036),
            ('alpha_s',    0.1185),
            ('sin2_tW',    0.231),
            ('m_W',       80.379),
            ('m_Z',       91.188),
            ('m_H',      125.25),
            ('m_t',      172.76),
            ('m_e',        0.000511),
            ('m_mu',       0.1057),
            ('m_tau',      1.777),
        ]),
    }),
    ('I', {
        'name': 'Quantum Information',
        'constants': OrderedDict([
            ('log2_e',     1.4427),
            ('2ln2',       1.3863),
            ('ln2',        0.69315),
            ('log2_2',     1.0),
            ('log2_s2p1',  0.6309),
        ]),
    }),
    ('S', {
        'name': 'Statistical Mechanics',
        'constants': OrderedDict([
            ('lambda_c',   0.27),
            ('Onsager',    2.269),
            ('nu',         0.6300),
            ('beta',       0.3265),
            ('gamma_Ising',1.2372),
            ('delta_Ising',4.789),
        ]),
    }),
])


# =====================================================================
# 2. Target convergence points
# =====================================================================

TARGETS = OrderedDict([
    ('sqrt2',    math.sqrt(2)),
    ('sqrt3',    math.sqrt(3)),
    ('5/6',      5.0 / 6.0),
    ('e',        math.e),
    ('zeta(3)',  1.2020569031595942),
    ('ln(4/3)',  math.log(4.0 / 3.0)),
    ('ln2',      math.log(2)),
    ('gamma',    0.5772156649015329),
    ('1/2',      0.5),
])


# =====================================================================
# 3. Arithmetic helpers
# =====================================================================

_OPS = [
    ('+', lambda a, b: a + b),
    ('-', lambda a, b: a - b),
    ('*', lambda a, b: a * b),
    ('/', lambda a, b: a / b if b != 0 else None),
]


def _safe_val(v):
    """Return float if finite, else None."""
    if v is None:
        return None
    try:
        f = float(v)
        if math.isfinite(f):
            return f
    except (ValueError, OverflowError):
        pass
    return None


def _domain_reachable(constants, target, threshold=0.001):
    """Return list of (expr_str, value, rel_error) reachable from a single
    domain's constants within relative threshold of *target*.

    Depth-1: each constant alone.
    Depth-2: a op b for every ordered pair and every op in {+,-,*,/}.
    """
    hits = []
    names = list(constants.keys())
    vals = list(constants.values())

    # Depth 1
    for i, (n, v) in enumerate(zip(names, vals)):
        v = _safe_val(v)
        if v is None or target == 0:
            continue
        rel = abs(v - target) / abs(target)
        if rel < threshold:
            hits.append((n, v, rel))

    # Depth 2
    for i in range(len(names)):
        for j in range(len(names)):
            vi = _safe_val(vals[i])
            vj = _safe_val(vals[j])
            if vi is None or vj is None:
                continue
            for op_sym, op_fn in _OPS:
                r = _safe_val(op_fn(vi, vj))
                if r is None or target == 0:
                    continue
                rel = abs(r - target) / abs(target)
                if rel < threshold:
                    expr = f'({names[i]} {op_sym} {names[j]})'
                    hits.append((expr, r, rel))

    return hits


def _cross_domain_reachable(const_a, const_b, target, threshold=0.001):
    """Check depth-2 combinations using one constant from each of two domains."""
    hits = []
    names_a = list(const_a.keys())
    vals_a = list(const_a.values())
    names_b = list(const_b.keys())
    vals_b = list(const_b.values())

    for i in range(len(names_a)):
        va = _safe_val(vals_a[i])
        if va is None:
            continue
        for j in range(len(names_b)):
            vb = _safe_val(vals_b[j])
            if vb is None:
                continue
            for op_sym, op_fn in _OPS:
                r = _safe_val(op_fn(va, vb))
                if r is None or target == 0:
                    continue
                rel = abs(r - target) / abs(target)
                if rel < threshold:
                    expr = f'({names_a[i]} {op_sym} {names_b[j]})'
                    hits.append((expr, r, rel))

    return hits


# =====================================================================
# 4. Strategy S1 — Open Search (single-domain reachability)
# =====================================================================

def strategy_s1(threshold=0.001):
    """For each domain, find which targets are reachable independently."""
    results = {}  # target_name -> { domain_key -> [hits] }
    for tname, tval in TARGETS.items():
        results[tname] = {}
        for dkey, dinfo in DOMAINS.items():
            hits = _domain_reachable(dinfo['constants'], tval, threshold)
            if hits:
                results[tname][dkey] = hits
    return results


# =====================================================================
# 5. Strategy S2 — Pair Scan (cross-domain bridges)
# =====================================================================

def strategy_s2(threshold=0.001):
    """For each pair of domains, find cross-domain paths to each target."""
    results = {}  # target_name -> { (d1, d2) -> [hits] }
    domain_keys = list(DOMAINS.keys())
    for tname, tval in TARGETS.items():
        results[tname] = {}
        for d1, d2 in combinations(domain_keys, 2):
            hits = _cross_domain_reachable(
                DOMAINS[d1]['constants'], DOMAINS[d2]['constants'],
                tval, threshold,
            )
            if hits:
                results[tname][(d1, d2)] = hits
    return results


# =====================================================================
# 6. Strategy S3 — Target Backtrack
# =====================================================================

def strategy_s3(threshold=0.001):
    """For each target, collect which domains can reach it (summary view)."""
    s1 = strategy_s1(threshold)
    s2 = strategy_s2(threshold)
    summary = {}
    for tname in TARGETS:
        indep = list(s1[tname].keys())
        bridges = list(s2[tname].keys())
        summary[tname] = {
            'independent_domains': indep,
            'independent_count': len(indep),
            'bridges': bridges,
            'bridge_count': len(bridges),
        }
    return summary


# =====================================================================
# 7. Convergence scoring
# =====================================================================

def convergence_scores(threshold=0.001):
    """Compute convergence score for each target.

    score = independent_count * bridge_count * log(1 / avg_error)

    avg_error is the mean relative error across all independent-domain
    best hits.  If no hits, score is 0.
    """
    s1 = strategy_s1(threshold)
    s3 = strategy_s3(threshold)
    scores = []

    for tname, tval in TARGETS.items():
        info = s3[tname]
        n_indep = info['independent_count']
        n_bridge = info['bridge_count']

        # Collect best error per independent domain
        errors = []
        for dkey, hits in s1[tname].items():
            best = min(h[2] for h in hits)
            errors.append(best)

        if errors and n_indep > 0 and n_bridge > 0:
            avg_err = np.mean(errors)
            if avg_err > 0:
                score = n_indep * n_bridge * math.log(1.0 / avg_err)
            else:
                score = float('inf')
        else:
            avg_err = float('nan')
            score = 0.0

        scores.append({
            'target': tname,
            'value': tval,
            'independent': n_indep,
            'bridges': n_bridge,
            'avg_error': avg_err,
            'score': score,
            'domains': info['independent_domains'],
        })

    scores.sort(key=lambda x: -x['score'])
    return scores


# =====================================================================
# 8. Texas Sharpshooter test
# =====================================================================

def texas_sharpshooter(n_random=200, threshold=0.001, seed=42):
    """Generate random constant sets of the same size as each domain,
    run the same S1 analysis, and compare to the real result.

    Returns Z-score: (observed - mean_random) / std_random
    for the total number of (target, domain) hits.
    """
    rng = np.random.default_rng(seed)

    # Observed hit count
    s1_real = strategy_s1(threshold)
    observed = sum(len(v) for v in s1_real.values())

    # Gather domain sizes and value ranges for random sampling
    domain_sizes = [len(d['constants']) for d in DOMAINS.values()]
    all_vals = []
    for d in DOMAINS.values():
        all_vals.extend(d['constants'].values())
    lo = min(abs(v) for v in all_vals if v != 0) * 0.1
    hi = max(abs(v) for v in all_vals) * 2.0

    random_counts = []
    for trial in range(n_random):
        total = 0
        for sz in domain_sizes:
            # Draw random constants log-uniformly
            rand_consts = OrderedDict()
            log_vals = rng.uniform(math.log(lo), math.log(hi), size=sz)
            signs = rng.choice([-1, 1], size=sz)
            for k, (lv, s) in enumerate(zip(log_vals, signs)):
                rand_consts[f'r{k}'] = float(s * math.exp(lv))

            for tname, tval in TARGETS.items():
                hits = _domain_reachable(rand_consts, tval, threshold)
                if hits:
                    total += 1
        random_counts.append(total)

    random_counts = np.array(random_counts, dtype=float)
    mean_r = np.mean(random_counts)
    std_r = np.std(random_counts)
    z_score = (observed - mean_r) / std_r if std_r > 0 else float('inf')

    return {
        'observed_hits': observed,
        'random_mean': mean_r,
        'random_std': std_r,
        'z_score': z_score,
        'n_random': n_random,
        'threshold': threshold,
    }


# =====================================================================
# 9. Domain participation frequency
# =====================================================================

def domain_participation():
    """Count how many targets each domain can independently reach."""
    s1 = strategy_s1()
    freq = OrderedDict()
    for dkey in DOMAINS:
        count = sum(1 for tname in TARGETS if dkey in s1[tname])
        freq[dkey] = count
    return freq


# =====================================================================
# 10. Strategy performance summary
# =====================================================================

def strategy_performance():
    """Compare the three strategies by total hits."""
    s1 = strategy_s1()
    s2 = strategy_s2()
    s3 = strategy_s3()

    s1_total = sum(len(v) for v in s1.values())
    s2_total = sum(len(v) for v in s2.values())
    s3_indep = sum(v['independent_count'] for v in s3.values())
    s3_bridge = sum(v['bridge_count'] for v in s3.values())

    return {
        'S1_domain_target_pairs': s1_total,
        'S2_cross_domain_pairs': s2_total,
        'S3_total_independent': s3_indep,
        'S3_total_bridges': s3_bridge,
    }


# =====================================================================
# Full report
# =====================================================================

def run_analysis():
    """Run full convergence engine analysis and print comprehensive report."""
    width = 78
    sep = '=' * width

    print(f'\n{sep}')
    print('  CONVERGENCE ENGINE — H-CX-453: Multi-Domain Constant Reachability')
    print(sep)

    # --- 1. Domains ---
    print('\n--- 1. Domain Catalog ---\n')
    for dkey, dinfo in DOMAINS.items():
        n = len(dinfo['constants'])
        print(f'  [{dkey}] {dinfo["name"]}  ({n} constants)')
        for cname, cval in dinfo['constants'].items():
            print(f'       {cname:<14s} = {cval}')
        print()

    # --- 2. Targets ---
    print('--- 2. Target Convergence Points ---\n')
    print(f'  {"Target":<12s} {"Value":>12s}')
    print(f'  {"-"*12} {"-"*12}')
    for tname, tval in TARGETS.items():
        print(f'  {tname:<12s} {tval:>12.6f}')
    print()

    # --- 3. Strategy S1 ---
    print('--- 3. Strategy S1: Open Search (single-domain reachability) ---\n')
    s1 = strategy_s1()
    for tname in TARGETS:
        domains_hit = s1[tname]
        if not domains_hit:
            continue
        dkeys = ', '.join(domains_hit.keys())
        print(f'  {tname:<12s}  reached by [{dkeys}]')
        for dkey, hits in domains_hit.items():
            best = min(hits, key=lambda h: h[2])
            print(f'    [{dkey}] best: {best[0]:<28s} = {best[1]:.6f}  '
                  f'(err {best[2]*100:.4f}%)')
    print()

    # --- 4. Strategy S2 ---
    print('--- 4. Strategy S2: Pair Scan (cross-domain bridges) ---\n')
    s2 = strategy_s2()
    for tname in TARGETS:
        pairs = s2[tname]
        if not pairs:
            continue
        print(f'  {tname:<12s}  {len(pairs)} bridge(s)')
        for (d1, d2), hits in pairs.items():
            best = min(hits, key=lambda h: h[2])
            print(f'    [{d1}-{d2}] best: {best[0]:<28s} = {best[1]:.6f}  '
                  f'(err {best[2]*100:.4f}%)')
    print()

    # --- 5. Strategy S3 ---
    print('--- 5. Strategy S3: Target Backtrack Summary ---\n')
    s3 = strategy_s3()
    print(f'  {"Target":<12s} {"Indep":>6s} {"Bridges":>8s} {"Domains":<30s}')
    print(f'  {"-"*12} {"-"*6} {"-"*8} {"-"*30}')
    for tname in TARGETS:
        info = s3[tname]
        dstr = ', '.join(info['independent_domains'])
        print(f'  {tname:<12s} {info["independent_count"]:>6d} '
              f'{info["bridge_count"]:>8d} {dstr:<30s}')
    print()

    # --- 6. Convergence scores ---
    print('--- 6. Convergence Scores (ranked) ---\n')
    scores = convergence_scores()
    print(f'  {"Rank":>4s}  {"Target":<12s} {"Indep":>6s} {"Bridges":>8s} '
          f'{"AvgErr":>10s} {"Score":>10s} {"Domains":<20s}')
    print(f'  {"-"*4}  {"-"*12} {"-"*6} {"-"*8} {"-"*10} {"-"*10} {"-"*20}')
    for rank, sc in enumerate(scores, 1):
        avg_e = f'{sc["avg_error"]*100:.4f}%' if not math.isnan(sc['avg_error']) else 'n/a'
        score_s = f'{sc["score"]:.2f}' if sc['score'] < 1e6 else 'inf'
        dstr = ', '.join(sc['domains'])
        print(f'  {rank:>4d}  {sc["target"]:<12s} {sc["independent"]:>6d} '
              f'{sc["bridges"]:>8d} {avg_e:>10s} {score_s:>10s} {dstr:<20s}')
    print()

    # --- 7. Domain participation ---
    print('--- 7. Domain Participation Frequency ---\n')
    freq = domain_participation()
    print(f'  {"Domain":<6s} {"Name":<24s} {"Targets Reached":>16s}')
    print(f'  {"-"*6} {"-"*24} {"-"*16}')
    for dkey, count in sorted(freq.items(), key=lambda x: -x[1]):
        print(f'  {dkey:<6s} {DOMAINS[dkey]["name"]:<24s} {count:>16d}')
    print()

    # --- 8. Strategy performance ---
    print('--- 8. Strategy Performance ---\n')
    perf = strategy_performance()
    for k, v in perf.items():
        print(f'  {k:<30s} {v:>6d}')
    print()

    # --- 9. Texas Sharpshooter ---
    print('--- 9. Texas Sharpshooter Test ---\n')
    print('  Running Monte Carlo with 200 random domain sets ...')
    ts = texas_sharpshooter(n_random=200)
    print(f'  Observed (target, domain) hits:  {ts["observed_hits"]}')
    print(f'  Random mean:                     {ts["random_mean"]:.2f}')
    print(f'  Random std:                      {ts["random_std"]:.2f}')
    print(f'  Z-score:                         {ts["z_score"]:.2f}')
    sig = 'SIGNIFICANT' if ts['z_score'] > 3.0 else 'NOT SIGNIFICANT'
    print(f'  Verdict:                         {sig} (Z > 3.0 required)')
    print()

    # --- Summary ---
    print(sep)
    print('  SUMMARY')
    print(sep)
    top = scores[0] if scores else None
    n_reach = sum(1 for sc in scores if sc['independent'] >= 2)
    print(f'''
  Domains:                         {len(DOMAINS)}
  Targets:                         {len(TARGETS)}
  Targets with >= 2 indep domains: {n_reach}
  Top convergence point:           {top["target"] if top else "none"} (score {top["score"]:.2f})
  Texas Sharpshooter Z-score:      {ts["z_score"]:.2f}
''')
    print(sep)

    return {
        's1': s1,
        's2': s2,
        's3': s3,
        'scores': scores,
        'participation': freq,
        'performance': perf,
        'sharpshooter': ts,
    }


if __name__ == '__main__':
    run_analysis()
