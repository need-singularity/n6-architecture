"""
Experiment: N6 Falsifiability Test — Is n=6 better than random?
================================================================
Nobel-grade question: Can n=6 arithmetic PREDICT constants better than
a random number generator with the same degrees of freedom?

Methodology:
1. Collect ALL known engineering/physics constants used in our 286 hypotheses
2. For each, record the n=6 prediction and actual value
3. Generate 10,000 RANDOM frameworks with same # of base constants
4. Compare: does n=6 produce significantly more EXACT matches than chance?

If n=6 is just post-hoc numerology, random frameworks should do equally well.
If n=6 has genuine structure, it should significantly outperform random.

This is the CRITICAL experiment that separates science from numerology.
"""

import math
import random
import numpy as np
from collections import Counter

# ─── n=6 Base Constants ───
N6_CONSTANTS = {
    "sigma": 12, "tau": 4, "phi": 2, "sopfr": 5,
    "J2": 24, "mu": 1, "lambda": 2, "n": 6,
}

# ─── Target Constants: Real-world values from our hypotheses ───
# Only including values where we claimed EXACT or CLOSE match
TARGETS = {
    # Network Protocol (strongest domain)
    "IPv6_bits": 128,           # 2^7
    "TCP_flags": 6,             # original RFC 793
    "OSI_layers": 7,
    "DNS_root_servers": 13,
    "HTTP_status_classes": 5,
    "MAC_bytes": 6,
    "TCP_states": 11,
    "port_space_exp": 16,       # 2^16 = 65536

    # Cryptography
    "AES_block_exp": 7,         # 2^7 = 128
    "SHA256_exp": 8,            # 2^8 = 256
    "RSA2048_exp": 11,          # 2^11 = 2048
    "ChaCha_rounds": 20,
    "AES_rounds_128": 10,

    # Robotics
    "robot_arm_DOF": 6,
    "human_fingers": 5,
    "hexapod_legs": 6,

    # Compiler/OS
    "process_states": 6,        # Linux: new/ready/run/wait/term/zombie
    "boot_stages": 4,
    "ext4_direct_blocks": 12,
    "page_table_levels": 4,
    "signals_base": 64,         # tau^3

    # Energy
    "turbine_blades": 3,
    "power_phases": 3,
    "HVDC_pulse": 12,

    # Physics
    "string_dims": 10,
    "M_theory_dims": 11,
    "quark_flavors": 6,
    "lepton_generations": 3,
    "SM_gauge_groups": 3,       # SU(3) x SU(2) x U(1)
    "codons": 64,
    "amino_acids": 20,

    # Fusion
    "ITER_PF_coils": 6,
    "DT_mass_sum": 5,           # D=2, T=3
    "He4_mass": 4,
    "plasma_heating_methods": 3,
}


def n6_derive(constants: dict) -> set:
    """Generate all derivable integers from a set of constants using simple operations."""
    vals = set(constants.values())
    derived = set(vals)

    # Pairwise operations
    for a in list(vals):
        for b in list(vals):
            for op in [lambda x, y: x + y, lambda x, y: x - y,
                       lambda x, y: x * y,
                       lambda x, y: x // y if y != 0 else None]:
                try:
                    r = op(a, b)
                    if r is not None and isinstance(r, int) and 1 <= r <= 200:
                        derived.add(r)
                except:
                    pass

    # Powers (small)
    for a in list(vals):
        for b in [2, 3, 4, 5]:
            if a ** b <= 200:
                derived.add(a ** b)
            if 2 ** a <= 200:
                derived.add(2 ** a)

    # Differences and sums with 2^ results
    power2 = set()
    for a in list(vals):
        for b in list(vals):
            diff = a - b
            if 1 <= diff <= 20:
                power2.add(2 ** diff)
    derived.update({v for v in power2 if 1 <= v <= 200})

    return derived


def random_framework(n_constants=8, max_val=24) -> dict:
    """Generate a random arithmetic framework with same structure as n=6."""
    return {f"c{i}": random.randint(1, max_val) for i in range(n_constants)}


def score_framework(framework: dict, targets: dict) -> dict:
    """Score how many targets a framework can derive."""
    derivable = n6_derive(framework)
    target_vals = set(targets.values())

    exact = target_vals & derivable
    return {
        "exact_count": len(exact),
        "exact_vals": exact,
        "total_derivable": len(derivable),
        "coverage": len(exact) / len(target_vals) * 100,
    }


def main():
    print("=" * 70)
    print("  N6 FALSIFIABILITY TEST")
    print("  Does n=6 predict better than random?")
    print("=" * 70)

    # ─── Step 1: Score n=6 ───
    print("\n--- Step 1: N=6 Framework Score ---")
    n6_score = score_framework(N6_CONSTANTS, TARGETS)
    print(f"  Base constants: {N6_CONSTANTS}")
    print(f"  Derivable integers (1-200): {n6_score['total_derivable']}")
    print(f"  Target values matched: {n6_score['exact_count']}/{len(TARGETS)}")
    print(f"  Coverage: {n6_score['coverage']:.1f}%")
    print(f"  Matched values: {sorted(n6_score['exact_vals'])}")

    unmatched_targets = {k: v for k, v in TARGETS.items()
                         if v not in n6_score['exact_vals']}
    if unmatched_targets:
        print(f"\n  UNMATCHED targets:")
        for k, v in sorted(unmatched_targets.items(), key=lambda x: x[1]):
            print(f"    {k} = {v}")

    # ─── Step 2: Monte Carlo — Random Frameworks ───
    print(f"\n--- Step 2: Monte Carlo — 10,000 Random Frameworks ---")
    N_TRIALS = 10000
    random_scores = []
    random_derivable_counts = []

    random.seed(42)
    for _ in range(N_TRIALS):
        fw = random_framework(n_constants=8, max_val=24)
        s = score_framework(fw, TARGETS)
        random_scores.append(s["exact_count"])
        random_derivable_counts.append(s["total_derivable"])

    random_scores = np.array(random_scores)
    random_derivable = np.array(random_derivable_counts)

    print(f"  Random derivable (mean): {random_derivable.mean():.1f}")
    print(f"  N6 derivable:            {n6_score['total_derivable']}")
    print(f"")
    print(f"  Random matches (mean):   {random_scores.mean():.2f}")
    print(f"  Random matches (std):    {random_scores.std():.2f}")
    print(f"  Random matches (max):    {random_scores.max()}")
    print(f"  N6 matches:              {n6_score['exact_count']}")

    # Z-score
    z = (n6_score['exact_count'] - random_scores.mean()) / random_scores.std()
    percentile = (random_scores < n6_score['exact_count']).sum() / N_TRIALS * 100

    print(f"\n  Z-score: {z:.2f}")
    print(f"  Percentile: {percentile:.1f}% (n=6 beats {percentile:.1f}% of random)")

    # ─── Step 3: Histogram ───
    print(f"\n--- Step 3: Distribution ---")
    counts = Counter(random_scores)
    max_count = max(counts.values())
    for score_val in range(int(random_scores.min()), int(random_scores.max()) + 1):
        c = counts.get(score_val, 0)
        bar = "#" * int(c / max_count * 50)
        marker = " <<<< N6" if score_val == n6_score['exact_count'] else ""
        print(f"  {score_val:3d} matches: {bar} ({c}){marker}")

    # ─── Step 4: Control — Score other perfect numbers ───
    print(f"\n--- Step 4: Control — Other Perfect Numbers ---")
    perfect_numbers = {
        6: N6_CONSTANTS,
        28: {"sigma": 56, "tau": 6, "phi": 12, "sopfr": 9,
             "J2": 168, "mu": -1, "lambda": 12, "n": 28},
        496: {"sigma": 992, "tau": 10, "phi": 240, "sopfr": 36,
              "J2": 992*240//496, "mu": 1, "lambda": 240, "n": 496},
    }

    for pn, consts in perfect_numbers.items():
        s = score_framework(consts, TARGETS)
        print(f"  n={pn}: {s['exact_count']}/{len(TARGETS)} matches "
              f"({s['coverage']:.1f}%), derivable={s['total_derivable']}")

    # ─── Step 5: Verdict ───
    print(f"\n{'='*70}")
    print("  VERDICT")
    print(f"{'='*70}")

    if z > 3.0:
        print(f"  ✅ SIGNIFICANT (z={z:.2f}, p<0.003)")
        print(f"  n=6 is NOT random numerology — it derives {n6_score['exact_count']}")
        print(f"  targets while random frameworks average {random_scores.mean():.1f}")
        print(f"  This is a {z:.1f}-sigma result.")
    elif z > 2.0:
        print(f"  ⚠️  SUGGESTIVE (z={z:.2f}, p<0.05)")
        print(f"  n=6 outperforms random but not overwhelmingly.")
    else:
        print(f"  ❌ NOT SIGNIFICANT (z={z:.2f})")
        print(f"  n=6 does NOT outperform random frameworks.")
        print(f"  The matches are likely post-hoc numerology.")

    # Additional analysis: coverage of small integers
    print(f"\n--- Base Rate Analysis ---")
    all_target_vals = sorted(set(TARGETS.values()))
    print(f"  Target values: {all_target_vals}")
    small = sum(1 for v in all_target_vals if v <= 24)
    print(f"  Values ≤ 24: {small}/{len(all_target_vals)} "
          f"({small/len(all_target_vals)*100:.0f}%)")
    print(f"  (Small integers are easy to derive from ANY framework)")


if __name__ == "__main__":
    main()
