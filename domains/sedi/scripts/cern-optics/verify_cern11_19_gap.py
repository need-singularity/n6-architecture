#!/usr/bin/env python3
"""H-CERN-11 (R-gap) and H-CERN-19 (sigma-chain) deep verification.

Analyses:
  1. R-GAP STRUCTURE: R(n) for n=2..10000, forbidden zone, near-R=1 values, hadron matching
  2. SIGMA-CHAIN: sigma-chains from n=1..30, perfect number hits, PDG particle matching
  3. PERFECT NUMBER PROXIMITY: hadron masses vs perfect numbers
"""

import math
from fractions import Fraction
from collections import defaultdict

# ─────────────────────────────────────────────────────────────
# Arithmetic functions
# ─────────────────────────────────────────────────────────────

def factorize(n):
    factors = {}
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors[d] = factors.get(d, 0) + 1
            n //= d
        d += 1
    if n > 1:
        factors[n] = factors.get(n, 0) + 1
    return factors

def sigma(n):
    if n <= 0:
        return 0
    factors = factorize(n)
    result = 1
    for p, a in factors.items():
        result *= (p ** (a + 1) - 1) // (p - 1)
    return result

def phi(n):
    if n <= 0:
        return 0
    factors = factorize(n)
    result = n
    for p in factors:
        result = result * (p - 1) // p
    return result

def tau(n):
    if n <= 0:
        return 0
    factors = factorize(n)
    result = 1
    for a in factors.values():
        result *= (a + 1)
    return result

def R(n):
    """R(n) = sigma(n)*phi(n)/(n*tau(n))"""
    if n <= 1:
        return 1.0
    s, p, t = sigma(n), phi(n), tau(n)
    return s * p / (n * t)

def aliquot_step(n):
    """s(n) = sigma(n) - n"""
    return sigma(n) - n

# ─────────────────────────────────────────────────────────────
# PDG Hadron Data (masses in MeV)
# ─────────────────────────────────────────────────────────────

M_PI = 139.57  # charged pion mass (MeV)

# Extended PDG list: name, mass (MeV)
PDG_HADRONS = [
    ("pi0",        134.98),
    ("pi+",        139.57),
    ("K+",         493.68),
    ("K0",         497.61),
    ("eta",        547.86),
    ("rho(770)",   775.26),
    ("omega(782)", 782.66),
    ("K*(892)",    891.67),
    ("eta'(958)",  957.78),
    ("phi(1020)", 1019.46),
    ("p",          938.27),
    ("n",          939.57),
    ("Lambda",    1115.68),
    ("Sigma+",    1189.37),
    ("Sigma0",    1192.64),
    ("Sigma-",    1197.45),
    ("Xi0",       1314.86),
    ("Xi-",       1321.71),
    ("Omega-",    1672.45),
    ("Delta(1232)", 1232.0),
    ("N(1440)",   1440.0),
    ("N(1520)",   1520.0),
    ("N(1535)",   1535.0),
    ("f0(500)",    475.0),
    ("f0(980)",    990.0),
    ("a0(980)",    980.0),
    ("a1(1260)",  1230.0),
    ("f2(1270)",  1275.5),
    ("D+",        1869.66),
    ("D0",        1864.84),
    ("D_s+",      1968.35),
    ("J/psi",     3096.90),
    ("psi(2S)",   3686.10),
    ("B+",        5279.34),
    ("B0",        5279.65),
    ("B_s0",      5366.88),
    ("Upsilon(1S)", 9460.30),
    ("eta_c(1S)", 2983.90),
    ("chi_c1",    3510.67),
    ("Lambda_c+", 2286.46),
]

PERFECT_NUMBERS = [6, 28, 496, 8128, 33550336]

# ─────────────────────────────────────────────────────────────
# PART 1: R-GAP STRUCTURE (H-CERN-11)
# ─────────────────────────────────────────────────────────────

def part1_rgap():
    print("=" * 80)
    print("PART 1: R-GAP STRUCTURE (H-CERN-11)")
    print("=" * 80)

    N_MAX = 10000

    # Compute R(n) for n=2..N_MAX
    print(f"\nComputing R(n) for n=2..{N_MAX}...")
    r_values = {}
    for n in range(2, N_MAX + 1):
        r_values[n] = R(n)

    # Verify perfect numbers
    print("\n--- R(n) for known perfect numbers ---")
    print(f"{'n':>10} {'R(n)':>15} {'|R-1|':>15}")
    print("-" * 42)
    for pn in PERFECT_NUMBERS:
        if pn <= N_MAX:
            rv = r_values[pn]
            print(f"{pn:>10} {rv:>15.10f} {abs(rv - 1):>15.2e}")

    # 50 closest to R=1
    print(f"\n--- 50 closest R(n) to 1 (n=2..{N_MAX}) ---")
    sorted_by_gap = sorted(r_values.items(), key=lambda x: abs(x[1] - 1))
    print(f"{'Rank':>4} {'n':>8} {'R(n)':>15} {'|R-1|':>15} {'factorization'}")
    print("-" * 70)
    for rank, (n, rv) in enumerate(sorted_by_gap[:50], 1):
        fac = factorize(n)
        fac_str = " * ".join(f"{p}^{a}" if a > 1 else str(p) for p, a in sorted(fac.items()))
        print(f"{rank:>4} {n:>8} {rv:>15.10f} {abs(rv - 1):>15.2e} {fac_str}")

    # Minimum |R-1| for n != 1, 6, 28, 496, 8128
    perfect_set = set(PERFECT_NUMBERS)
    non_perfect = [(n, rv) for n, rv in r_values.items() if n not in perfect_set]
    non_perfect_sorted = sorted(non_perfect, key=lambda x: abs(x[1] - 1))
    min_gap_non_perfect = abs(non_perfect_sorted[0][1] - 1)
    print(f"\n--- Minimum |R-1| for non-perfect n (n=2..{N_MAX}, excluding 6,28,496,8128) ---")
    print(f"  n = {non_perfect_sorted[0][0]}, R(n) = {non_perfect_sorted[0][1]:.10f}, |R-1| = {min_gap_non_perfect:.10f}")
    print(f"\n  Top 10 non-perfect closest to R=1:")
    print(f"  {'n':>8} {'R(n)':>15} {'|R-1|':>15}")
    print("  " + "-" * 40)
    for n, rv in non_perfect_sorted[:10]:
        print(f"  {n:>8} {rv:>15.10f} {abs(rv - 1):>15.10f}")

    # Forbidden zone analysis
    print(f"\n--- Forbidden zone analysis ---")
    thresholds = [0.001, 0.01, 0.05, 0.1, 0.2]
    print(f"{'Threshold':>12} {'Count (all)':>12} {'Count (non-perf)':>16} {'Fraction':>10}")
    print("-" * 52)
    for th in thresholds:
        count_all = sum(1 for n, rv in r_values.items() if abs(rv - 1) < th)
        count_np = sum(1 for n, rv in non_perfect if abs(rv - 1) < th)
        frac = count_np / len(non_perfect)
        print(f"{th:>12.3f} {count_all:>12} {count_np:>16} {frac:>10.6f}")

    # Gap density histogram (|R-1| distribution)
    print(f"\n--- |R-1| Histogram (n=2..{N_MAX}) ---")
    bins = [0, 0.01, 0.05, 0.1, 0.2, 0.3, 0.5, 0.7, 1.0, 2.0, 5.0, 100.0]
    hist = [0] * (len(bins) - 1)
    for n, rv in r_values.items():
        gap = abs(rv - 1)
        for i in range(len(bins) - 1):
            if bins[i] <= gap < bins[i + 1]:
                hist[i] += 1
                break

    max_count = max(hist) if hist else 1
    print(f"{'Bin':>20} {'Count':>8} {'Bar'}")
    print("-" * 60)
    for i in range(len(hist)):
        label = f"[{bins[i]:.2f}, {bins[i+1]:.2f})"
        bar_len = int(40 * hist[i] / max_count) if max_count > 0 else 0
        print(f"{label:>20} {hist[i]:>8} {'#' * bar_len}")

    # Near-R=1 hadron matching
    print(f"\n--- Near-R=1 values: n*m_pi hadron check (top 20 non-perfect) ---")
    print(f"{'n':>8} {'R(n)':>12} {'n*m_pi (MeV)':>14} {'Nearest hadron':>20} {'Delta (MeV)':>14}")
    print("-" * 72)
    for n, rv in non_perfect_sorted[:20]:
        mass_estimate = n * M_PI
        # Find nearest hadron
        best_hadron = None
        best_delta = float('inf')
        for name, mass in PDG_HADRONS:
            delta = abs(mass - mass_estimate)
            if delta < best_delta:
                best_delta = delta
                best_hadron = name
        print(f"{n:>8} {rv:>12.8f} {mass_estimate:>14.1f} {best_hadron:>20} {best_delta:>14.1f}")

    # R(n) for perfect numbers: hadron matching
    print(f"\n--- Perfect number n*m_pi hadron matching ---")
    print(f"{'n':>8} {'n*m_pi (MeV)':>14} {'Nearest hadron':>20} {'Delta (MeV)':>14}")
    print("-" * 60)
    for pn in PERFECT_NUMBERS:
        mass_estimate = pn * M_PI
        best_hadron = None
        best_delta = float('inf')
        for name, mass in PDG_HADRONS:
            delta = abs(mass - mass_estimate)
            if delta < best_delta:
                best_delta = delta
                best_hadron = name
        print(f"{pn:>8} {mass_estimate:>14.1f} {best_hadron:>20} {best_delta:>14.1f}")

    return r_values


# ─────────────────────────────────────────────────────────────
# PART 2: SIGMA-CHAIN (H-CERN-19)
# ─────────────────────────────────────────────────────────────

def part2_sigma_chain():
    print("\n" + "=" * 80)
    print("PART 2: SIGMA-CHAIN (H-CERN-19)")
    print("=" * 80)

    perfect_set = set(PERFECT_NUMBERS)

    # Sigma chains: sigma(n), sigma(sigma(n)), ...
    def sigma_chain(start, max_steps=20, max_val=10**8):
        chain = [start]
        n = start
        for _ in range(max_steps):
            n = sigma(n)
            if n > max_val:
                chain.append(n)
                break
            chain.append(n)
        return chain

    print("\n--- Sigma-chains from n=1..30 (up to 20 steps) ---")
    chain_hits_perfect = {}
    for start in range(1, 31):
        chain = sigma_chain(start)
        hits = [x for x in chain if x in perfect_set]
        chain_hits_perfect[start] = hits
        chain_str = " -> ".join(str(x) for x in chain[:12])
        if len(chain) > 12:
            chain_str += " -> ..."
        hit_str = f"  [hits perfect: {hits}]" if hits else ""
        print(f"  n={start:>2}: {chain_str}{hit_str}")

    # Count chains hitting perfect numbers
    print(f"\n--- Summary: chains hitting perfect numbers ---")
    print(f"{'Start n':>8} {'Hits perfect?':>14} {'Which':>20}")
    print("-" * 44)
    for start in range(1, 31):
        hits = chain_hits_perfect[start]
        print(f"{start:>8} {'YES' if hits else 'no':>14} {str(hits) if hits else '':>20}")

    total_hits = sum(1 for h in chain_hits_perfect.values() if h)
    print(f"\n  Total chains hitting a perfect number: {total_hits}/30")

    # Chain from n=6 in detail: PDG particle matching
    print(f"\n--- Chain from n=6: PDG particle matching ---")
    chain_6 = sigma_chain(6, max_steps=15, max_val=10**9)
    print(f"  Chain: {' -> '.join(str(x) for x in chain_6)}")
    print(f"\n  {'Step':>4} {'n':>10} {'n*m_pi (MeV)':>14} {'Nearest hadron':>20} {'Delta (MeV)':>14} {'Delta/m_pi':>12}")
    print("  " + "-" * 78)
    for i, n in enumerate(chain_6):
        if n > 10**7:
            break
        mass_est = n * M_PI
        best_hadron = None
        best_delta = float('inf')
        for name, mass in PDG_HADRONS:
            delta = abs(mass - mass_est)
            if delta < best_delta:
                best_delta = delta
                best_hadron = name
        print(f"  {i:>4} {n:>10} {mass_est:>14.1f} {best_hadron:>20} {best_delta:>14.1f} {best_delta/M_PI:>12.3f}")

    # Compare chains from n=2,3,4,5,6: average match quality
    print(f"\n--- Chain particle matching comparison (n=2,3,4,5,6) ---")
    for start in [2, 3, 4, 5, 6]:
        chain = sigma_chain(start, max_steps=10)
        total_delta = 0
        count = 0
        matches = []
        for n in chain:
            if n > 10**6:
                break
            mass_est = n * M_PI
            best_hadron = None
            best_delta = float('inf')
            for name, mass in PDG_HADRONS:
                delta = abs(mass - mass_est)
                if delta < best_delta:
                    best_delta = delta
                    best_hadron = name
            total_delta += best_delta
            count += 1
            matches.append((n, best_hadron, best_delta))
        avg_delta = total_delta / count if count else 0
        print(f"\n  Chain from n={start} (avg |Delta|={avg_delta:.1f} MeV):")
        print(f"  {'n':>8} {'Nearest':>20} {'Delta (MeV)':>14}")
        print("  " + "-" * 44)
        for n, h, d in matches:
            print(f"  {n:>8} {h:>20} {d:>14.1f}")

    # Aliquot sequences: s(n) = sigma(n) - n
    print(f"\n--- Aliquot sequences s(n) = sigma(n) - n ---")
    def aliquot_chain(start, max_steps=30):
        chain = [start]
        n = start
        seen = {start}
        for _ in range(max_steps):
            n = aliquot_step(n)
            chain.append(n)
            if n == 0:
                break
            if n in seen:
                break
            seen.add(n)
        return chain

    for start in [6, 12, 28, 220, 496]:
        chain = aliquot_chain(start)
        chain_str = " -> ".join(str(x) for x in chain[:20])
        if len(chain) > 20:
            chain_str += " -> ..."
        # Detect cycle
        if len(chain) >= 2 and chain[-1] in chain[:-1]:
            idx = chain.index(chain[-1])
            cycle_len = len(chain) - 1 - idx
            chain_str += f"  [CYCLE length {cycle_len}]"
        elif chain[-1] == 0:
            chain_str += "  [TERMINATES at 0]"
        print(f"  s({start}): {chain_str}")


# ─────────────────────────────────────────────────────────────
# PART 3: PERFECT NUMBER PROXIMITY
# ─────────────────────────────────────────────────────────────

def part3_perfect_proximity():
    print("\n" + "=" * 80)
    print("PART 3: PERFECT NUMBER PROXIMITY")
    print("=" * 80)

    print(f"\n--- Hadron M/m_pi and distance to nearest perfect number ---")
    print(f"{'Hadron':>20} {'M (MeV)':>12} {'M/m_pi':>10} {'round':>8} {'Near perf':>10} {'Dist':>8} {'Dist/perf':>10}")
    print("-" * 82)

    distances = []
    for name, mass in PDG_HADRONS:
        ratio = mass / M_PI
        rounded = round(ratio)
        # Nearest perfect number
        best_pn = None
        best_dist = float('inf')
        for pn in PERFECT_NUMBERS:
            d = abs(ratio - pn)
            if d < best_dist:
                best_dist = d
                best_pn = pn
        rel_dist = best_dist / best_pn if best_pn else 0
        distances.append((name, mass, ratio, rounded, best_pn, best_dist, rel_dist))
        print(f"{name:>20} {mass:>12.2f} {ratio:>10.3f} {rounded:>8} {best_pn:>10} {best_dist:>8.3f} {rel_dist:>10.4f}")

    # Rank by distance to nearest perfect number
    distances_sorted = sorted(distances, key=lambda x: x[5])
    print(f"\n--- Ranked by distance to nearest perfect number ---")
    print(f"{'Rank':>4} {'Hadron':>20} {'M/m_pi':>10} {'Near perf':>10} {'Dist':>8}")
    print("-" * 56)
    for rank, (name, mass, ratio, rounded, pn, dist, rdist) in enumerate(distances_sorted, 1):
        marker = " <-- CLOSE" if dist < 1.0 else ""
        print(f"{rank:>4} {name:>20} {ratio:>10.3f} {pn:>10} {dist:>8.3f}{marker}")

    # Statistical test: are hadrons preferentially near perfect numbers?
    print(f"\n--- Statistical test: hadron proximity to perfect numbers vs random ---")
    import random
    random.seed(42)

    # Actual average distance
    actual_dists = [x[5] for x in distances]
    actual_mean = sum(actual_dists) / len(actual_dists)

    # Monte Carlo: random masses in same range, compute distance to nearest perfect
    n_trials = 10000
    max_ratio = max(x[2] for x in distances)
    min_ratio = min(x[2] for x in distances)
    random_means = []
    for _ in range(n_trials):
        random_dists = []
        for _ in range(len(distances)):
            r = random.uniform(min_ratio, max_ratio)
            best_d = min(abs(r - pn) for pn in PERFECT_NUMBERS)
            random_dists.append(best_d)
        random_means.append(sum(random_dists) / len(random_dists))

    random_avg = sum(random_means) / len(random_means)
    random_std = (sum((x - random_avg) ** 2 for x in random_means) / len(random_means)) ** 0.5
    z_score = (actual_mean - random_avg) / random_std if random_std > 0 else 0

    print(f"  Actual mean distance to nearest perfect:  {actual_mean:.3f}")
    print(f"  Random mean distance (uniform):           {random_avg:.3f} +/- {random_std:.3f}")
    print(f"  Z-score:                                  {z_score:.3f}")
    print(f"  p-value (one-sided, closer):              {sum(1 for x in random_means if x <= actual_mean) / n_trials:.4f}")

    # Additional: count hadrons within 1 pion mass of a perfect number
    close_count = sum(1 for x in distances if x[5] < 1.0)
    # Expected from random
    random_close_counts = []
    for _ in range(n_trials):
        rc = 0
        for _ in range(len(distances)):
            r = random.uniform(min_ratio, max_ratio)
            best_d = min(abs(r - pn) for pn in PERFECT_NUMBERS)
            if best_d < 1.0:
                rc += 1
        random_close_counts.append(rc)
    expected_close = sum(random_close_counts) / len(random_close_counts)

    print(f"\n  Hadrons within 1.0 of a perfect number:   {close_count}/{len(distances)}")
    print(f"  Expected from random:                     {expected_close:.1f}/{len(distances)}")

    # Distribution of M/m_pi modulo small perfect numbers
    print(f"\n--- M/m_pi modulo 6 and modulo 28 ---")
    print(f"{'Hadron':>20} {'M/m_pi':>10} {'mod 6':>8} {'mod 28':>8} {'|mod6-3|':>10} {'|mod28-14|':>10}")
    print("-" * 70)
    for name, mass, ratio, rounded, pn, dist, rdist in distances:
        mod6 = ratio % 6
        mod28 = ratio % 28
        d6 = min(mod6, 6 - mod6)
        d28 = min(mod28, 28 - mod28)
        print(f"{name:>20} {ratio:>10.3f} {mod6:>8.3f} {mod28:>8.3f} {d6:>10.3f} {d28:>10.3f}")


# ─────────────────────────────────────────────────────────────
# MAIN
# ─────────────────────────────────────────────────────────────

if __name__ == "__main__":
    print("H-CERN-11 (R-gap) and H-CERN-19 (sigma-chain) Deep Verification")
    print("=" * 80)

    r_values = part1_rgap()
    part2_sigma_chain()
    part3_perfect_proximity()

    print("\n" + "=" * 80)
    print("VERIFICATION COMPLETE")
    print("=" * 80)
