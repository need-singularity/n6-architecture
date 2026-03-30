"""
Experiment: R(n) = 1 Complete Solution Search
==============================================
Critical question: Is n=6 the ONLY solution to R(n) = 1?
If yes → n=6 is mathematically unique
If no → the claim collapses

R(n) = sigma(n) * phi(n) / (n * tau(n))

Also compute: probability that SM particle structure matches by chance.
"""

import math
from collections import defaultdict


def sigma(n):
    return sum(d for d in range(1, n + 1) if n % d == 0)


def tau(n):
    return sum(1 for d in range(1, n + 1) if n % d == 0)


def euler_phi(n):
    return sum(1 for k in range(1, n + 1) if math.gcd(k, n) == 1)


def R(n):
    s, t, p = sigma(n), tau(n), euler_phi(n)
    return s * p / (n * t) if n * t != 0 else float('inf')


def is_perfect(n):
    return sigma(n) == 2 * n


def main():
    print("=" * 70)
    print("  R(n) = 1 COMPLETE SOLUTION SEARCH")
    print("=" * 70)

    # ─── Part 1: Search R(n) = 1 for n = 1..100000 ───
    print("\n--- Part 1: All solutions of R(n) = 1 for n ≤ 100,000 ---\n")
    solutions = []
    perfect_numbers = []
    near_solutions = []  # |R(n) - 1| < 0.001

    for n in range(2, 100001):
        r = R(n)
        if r == 1.0:
            solutions.append(n)
            perf = "PERFECT" if is_perfect(n) else "NOT PERFECT"
            print(f"  R({n}) = 1.0  [{perf}]  "
                  f"sigma={sigma(n)}, tau={tau(n)}, phi={euler_phi(n)}")
        if is_perfect(n):
            perfect_numbers.append(n)
        if abs(r - 1.0) < 0.01 and r != 1.0:
            near_solutions.append((n, r))

    print(f"\n  Total solutions: {len(solutions)}")
    print(f"  Solutions: {solutions}")
    print(f"\n  Perfect numbers found: {perfect_numbers}")

    # Check: are all solutions perfect numbers?
    all_perfect = all(is_perfect(n) for n in solutions)
    print(f"  All solutions are perfect numbers: {all_perfect}")

    # Check: are all perfect numbers solutions?
    all_r1 = all(R(n) == 1.0 for n in perfect_numbers)
    print(f"  All perfect numbers have R=1: {all_r1}")

    if not all_r1:
        print("\n  CRITICAL: Some perfect numbers do NOT have R=1!")
        for pn in perfect_numbers:
            r = R(pn)
            print(f"    R({pn}) = {r:.6f}")

    # Near solutions
    print(f"\n  Near-solutions (|R-1| < 0.01): {len(near_solutions)}")
    near_solutions.sort(key=lambda x: abs(x[1] - 1.0))
    for n, r in near_solutions[:20]:
        print(f"    R({n}) = {r:.6f}  (off by {abs(r-1.0):.6f})")

    # ─── Part 2: SM Particle Structure Statistical Test ───
    print(f"\n{'='*70}")
    print(f"  STANDARD MODEL STRUCTURE — Statistical Significance")
    print(f"{'='*70}")

    # The claim: SM = n + n + tau + mu = 6 + 6 + 4 + 1 = 17
    # How likely is this by chance?

    # Method: given 8 arithmetic functions of n=6 producing values
    # {1, 2, 4, 5, 6, 12, 24, ...}, what's the probability that
    # SOME 4-partition of 17 matches 4 of these values?

    n6_vals = {1, 2, 4, 5, 6, 12, 24}  # available n=6 constants

    # Count: how many ways to write 17 = a + b + c + d where a,b,c,d ∈ n6_vals?
    partitions_17 = []
    for a in n6_vals:
        for b in n6_vals:
            for c in n6_vals:
                for d in n6_vals:
                    if a + b + c + d == 17:
                        partition = tuple(sorted([a, b, c, d], reverse=True))
                        if partition not in partitions_17:
                            partitions_17.append(partition)

    print(f"\n  Ways to partition 17 into 4 values from n=6 constants:")
    for p in sorted(set(partitions_17)):
        print(f"    {p[0]} + {p[1]} + {p[2]} + {p[3]} = 17")
    print(f"  Total unique partitions: {len(set(partitions_17))}")

    # But the SPECIFIC partition must also match CATEGORIES:
    # Two identical values (matter) + one value (force) + one value (scalar)
    # Pattern: a, a, b, c where a ≠ b ≠ c
    print(f"\n  Pattern matching (a, a, b, c) for matter+force+scalar:")
    pattern_matches = []
    for p in set(partitions_17):
        # Check if pattern is (x, x, y, z) with x appearing exactly twice
        from collections import Counter
        c = Counter(p)
        for val, count in c.items():
            if count == 2:
                others = [v for v in p if v != val]
                if len(set(others)) == 2:
                    pattern_matches.append(p)

    for p in sorted(set(pattern_matches)):
        c = Counter(p)
        pair = [v for v, cnt in c.items() if cnt == 2][0]
        singles = sorted([v for v, cnt in c.items() if cnt == 1], reverse=True)
        print(f"    {pair}+{pair}+{singles[0]}+{singles[1]} = 17")
    print(f"  Pattern matches: {len(set(pattern_matches))}")

    # Now: what fraction of integers 10-30 can be expressed this way?
    print(f"\n  Control: what fraction of integers can be 4-partitioned from n=6?")
    expressible = 0
    for target in range(10, 31):
        found = False
        for a in n6_vals:
            for b in n6_vals:
                for c in n6_vals:
                    for d in n6_vals:
                        if a + b + c + d == target:
                            found = True
                            break
                    if found: break
                if found: break
            if found: break
        if found:
            expressible += 1
    print(f"  Integers 10-30 expressible as 4-sum from n=6: {expressible}/21")
    print(f"  Base rate: {expressible/21*100:.0f}%")

    # ─── Part 3: Gauge Generator Match ───
    print(f"\n--- Gauge Group Generator Test ---")
    # SU(3)=8, SU(2)=3, U(1)=1, total=12
    # Can n=6 derive 8, 3, 1 as differences/quotients?

    print(f"  Target: (8, 3, 1) with sum=12=sigma")
    # How many 3-partitions of 12 can n=6 derive?
    gauge_partitions = []
    for a in n6_vals:
        for b in n6_vals:
            for c in n6_vals:
                if a + b + c == 12 and a >= b >= c:
                    gauge_partitions.append((a, b, c))

    print(f"  3-partitions of sigma=12 from n=6 constants:")
    for p in sorted(set(gauge_partitions), reverse=True):
        marker = " ◄── SU(3)+SU(2)+U(1) ACTUAL" if p == (8, 3, 1) else ""
        # Check if each value is derivable
        derivable = []
        if p[0] == SIGMA - TAU: derivable.append("sigma-tau")
        if p[1] == N // PHI: derivable.append("n/phi")
        if p[2] == MU: derivable.append("mu")
        d_str = f" [{', '.join(derivable)}]" if derivable else ""
        print(f"    {p[0]}+{p[1]}+{p[2]}=12{d_str}{marker}")

    n_partitions = len(set(gauge_partitions))
    print(f"\n  Total 3-partitions: {n_partitions}")
    print(f"  Probability of hitting (8,3,1) by chance: 1/{n_partitions} = "
          f"{1/n_partitions*100:.1f}%")

    # But (8,3,1) is not just ANY partition — each part has a NAMED derivation
    # 8 = sigma-tau, 3 = n/phi, 1 = mu
    # What fraction of partitions have ALL 3 parts independently derivable?
    SIGMA_V, TAU_V, PHI_V, SOPFR_V, J2_V, MU_V, N_V = 12, 4, 2, 5, 24, 1, 6
    derived_set = set()
    for a in [SIGMA_V, TAU_V, PHI_V, SOPFR_V, J2_V, MU_V, N_V]:
        for b in [SIGMA_V, TAU_V, PHI_V, SOPFR_V, J2_V, MU_V, N_V]:
            for op in [lambda x,y:x+y, lambda x,y:x-y, lambda x,y:x*y,
                       lambda x,y:x//y if y!=0 else None]:
                r = op(a, b)
                if r is not None and 1 <= r <= 24:
                    derived_set.add(r)
    derived_set.update([SIGMA_V, TAU_V, PHI_V, SOPFR_V, J2_V, MU_V, N_V])

    all_derivable = 0
    for p in set(gauge_partitions):
        if all(v in derived_set for v in p):
            all_derivable += 1

    print(f"  Partitions where ALL parts are n=6-derivable: "
          f"{all_derivable}/{n_partitions}")
    print(f"  (This means the gauge match is less impressive than it looks)")

    # ─── Verdict ───
    print(f"\n{'='*70}")
    print(f"  VERDICT")
    print(f"{'='*70}")
    print(f"""
  1. R(n) = 1 solutions: {solutions}
     → {'ONLY n=6 below 100K' if len(solutions) == 1 else f'{len(solutions)} solutions found'}

  2. SM particle count (17 = 6+6+4+1):
     → Pattern (a,a,b,c) partitions: {len(set(pattern_matches))}
     → Base rate: ~{expressible/21*100:.0f}% of integers are 4-partitionable
     → NOT a strong statistical signal alone

  3. Gauge generators (12 = 8+3+1):
     → 1/{n_partitions} chance = {1/n_partitions*100:.1f}%
     → But {all_derivable}/{n_partitions} partitions are fully n=6-derivable
     → MODERATE signal

  4. Combined (SM + gauge):
     → Both matching simultaneously: ~{1/n_partitions * len(set(pattern_matches))/max(expressible,1) * 100:.2f}%
     → This is the strongest structural claim
""")


# Constants for derivation checking
SIGMA, TAU, PHI, SOPFR, J2, MU, N_CONST = 12, 4, 2, 5, 24, 1, 6

if __name__ == "__main__":
    main()
