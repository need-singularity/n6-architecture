"""
Experiment: Information-Theoretic Meaning of R(n) = 1
======================================================
THE DEEP QUESTION: What does σ(n)·φ(n) = n·τ(n) mean in information theory?

We proved n=6 is the unique solution. But WHY should this matter physically?

Approach: Interpret each function as an information quantity.
- τ(n) = number of divisors = number of CHANNELS for n
- σ(n) = sum of divisors = total WEIGHT of channels
- φ(n) = totient = number of INDEPENDENT generators mod n
- n = the number itself = the SPACE size

Then:
  σ(n)/n = average divisor "density" = redundancy factor
  φ(n)/n = fraction of generators = independence ratio
  τ(n)   = channel count

  R(n) = (σ/n) · (φ/τ) = redundancy × (independence per channel)

R = 1 means: REDUNDANCY × INDEPENDENCE_PER_CHANNEL = 1
i.e., the system has exactly enough redundancy to be reconstructible,
with each channel carrying exactly one independent piece of information.

This is the CHANNEL CAPACITY theorem condition!
"""

import math
import numpy as np
from fractions import Fraction


def sigma(n):
    s = 0
    for i in range(1, int(math.isqrt(n)) + 1):
        if n % i == 0:
            s += i
            if i != n // i:
                s += n // i
    return s


def tau(n):
    t = 0
    for i in range(1, int(math.isqrt(n)) + 1):
        if n % i == 0:
            t += 1
            if i != n // i:
                t += 1
    return t


def euler_phi(n):
    result = n
    p = 2
    temp = n
    while p * p <= temp:
        if temp % p == 0:
            while temp % p == 0:
                temp //= p
            result -= result // p
        p += 1
    if temp > 1:
        result -= result // temp
    return result


def H_divisors(n):
    """Shannon entropy of the divisor distribution of n."""
    divs = [d for d in range(1, n + 1) if n % d == 0]
    total = sum(divs)
    probs = [d / total for d in divs]
    return -sum(p * math.log2(p) for p in probs if p > 0)


def main():
    print("=" * 70)
    print("  INFORMATION-THEORETIC MEANING OF R(n) = 1")
    print("=" * 70)

    # ═══════════════════════════════════════════════════════════
    # INTERPRETATION 1: Redundancy × Independence = 1
    # ═══════════════════════════════════════════════════════════
    print("\n--- Interpretation 1: Redundancy-Independence Balance ---")
    print()
    print(f"  {'n':>4} {'σ/n':>8} {'φ/n':>8} {'φ/τ':>8} {'(σ/n)·(φ/τ)':>12} {'R(n)':>8}")
    print(f"  {'-'*56}")

    for n in [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 20, 24, 28, 30]:
        s, t, p = sigma(n), tau(n), euler_phi(n)
        redundancy = s / n      # > 1 means divisors sum to more than n
        indep_ratio = p / n     # fraction of independent residues
        indep_per_ch = p / t    # independence per channel
        R = s * p / (n * t)

        marker = " ◄── R=1!" if R == 1.0 else ""
        print(f"  {n:>4} {redundancy:>8.4f} {indep_ratio:>8.4f} {indep_per_ch:>8.4f} "
              f"{redundancy * indep_per_ch:>12.6f} {R:>8.4f}{marker}")

    print("""
  At n=6:
    σ(6)/6 = 12/6 = 2.0  (divisors sum to 2× the number — perfect number!)
    φ(6)/6 = 2/6 = 1/3   (only 1/3 of residues are independent)
    φ(6)/τ(6) = 2/4 = 1/2 (each channel carries 1/2 unit of independence)

    R = 2.0 × 0.5 = 1.0

  MEANING: The "over-representation" in divisor structure (σ/n = 2)
  is EXACTLY compensated by the "under-representation" in independence
  (φ/τ = 1/2). This is a CONSERVATION LAW:

    Redundancy × Efficiency = 1
""")

    # ═══════════════════════════════════════════════════════════
    # INTERPRETATION 2: Divisor Entropy
    # ═══════════════════════════════════════════════════════════
    print("--- Interpretation 2: Divisor Entropy ---")
    print()
    print(f"  {'n':>4} {'τ':>4} {'H(div)':>8} {'H_max':>8} {'H/H_max':>8} {'R(n)':>8}")
    print(f"  {'-'*48}")

    for n in [2, 3, 4, 5, 6, 7, 8, 9, 10, 12, 15, 20, 24, 28, 30]:
        s, t, p = sigma(n), tau(n), euler_phi(n)
        R = s * p / (n * t)
        H = H_divisors(n)
        H_max = math.log2(t)  # maximum entropy = uniform over τ divisors

        ratio = H / H_max if H_max > 0 else 0
        marker = " ◄── R=1!" if R == 1.0 else ""
        print(f"  {n:>4} {t:>4} {H:>8.4f} {H_max:>8.4f} {ratio:>8.4f} {R:>8.4f}{marker}")

    print("""
  n=6 divisors: {1, 2, 3, 6} with weights proportional to values.
  The entropy is NOT maximized at n=6, but that's not the point.

  The point is: R=1 is about BALANCE, not about maximizing any single quantity.
""")

    # ═══════════════════════════════════════════════════════════
    # INTERPRETATION 3: Lattice Theory — Z/nZ structure
    # ═══════════════════════════════════════════════════════════
    print("--- Interpretation 3: Group-Theoretic Meaning ---")
    print("""
  Consider Z/nZ (integers mod n):
    - τ(n) subgroups (one for each divisor)
    - φ(n) generators (elements of order n)
    - σ(n) = sum of subgroup orders

  σ(n)·φ(n) = n·τ(n) means:

    (sum of all subgroup sizes) × (number of generators)
    = (group order) × (number of subgroups)

  LEFT SIDE: "total subgroup capacity" × "generative power"
  RIGHT SIDE: "group size" × "structural complexity"

  At n=6 (and ONLY at n=6): these balance exactly.

  In Z/6Z:
    Subgroups: {0}, {0,3}, {0,2,4}, Z/6Z (sizes 1,2,3,6)
    Generators: {1, 5} (φ=2 elements generate the whole group)
    σ = 1+2+3+6 = 12, τ = 4, φ = 2, n = 6
    12 × 2 = 6 × 4 = 24 ✓

  This means: in Z/6Z, knowing ANY generator and ANY subgroup
  gives you EXACTLY n·τ = 24 units of structural information.
  The group is "self-describing" — its parts encode the whole.
""")

    # ═══════════════════════════════════════════════════════════
    # INTERPRETATION 4: Connection to Shannon Capacity
    # ═══════════════════════════════════════════════════════════
    print("--- Interpretation 4: Channel Capacity Analogy ---")
    print("""
  Shannon's Channel Capacity: C = B · log₂(1 + S/N)

  Consider n as a "number-theoretic channel":
    - Bandwidth B ~ τ(n) (number of divisor-channels)
    - Signal S ~ σ(n) (total divisor power)
    - Noise N ~ n (the number itself as "background")
    - Efficiency ~ φ(n)/n (fraction of usable information)

  Then R(n) = (S/N) × (efficiency × B) / B = (σ/n) × (φ/τ)

  R = 1 means the channel operates at capacity:
    Signal-to-noise × efficiency = 1
    → No wasted capacity, no information loss.

  This is analogous to the water-filling solution in MIMO:
  at the optimal power allocation, total throughput = total capacity.

  n=6 is the "matched filter" of number theory:
  the unique integer where divisor structure and coprime structure
  are perfectly impedance-matched.
""")

    # ═══════════════════════════════════════════════════════════
    # INTERPRETATION 5: Landauer Connection
    # ═══════════════════════════════════════════════════════════
    print("--- Interpretation 5: Thermodynamic / Landauer ---")
    print("""
  Landauer's principle: erasing 1 bit costs ≥ kT·ln(2) energy.

  For a system with n states:
    - τ(n) distinct "resolution levels" (divisor decomposition)
    - φ(n) "free" states (not determined by divisors)
    - σ(n) total "energy" across all resolution levels

  Energy per free state per level = σ(n)·φ(n) / (n·τ(n)) = R(n)

  R = 1 means: exactly 1 unit of energy per free state per level.
  This is the LANDAUER LIMIT for the number-theoretic system:
  the minimum energy cost for maximum information processing.

  At n=6: the system processes information at the thermodynamic limit.
  For n ≠ 6: either wasteful (R > 1) or impossible (R < 1, only 2^1=2
  which has R=3/4, meaning it can't sustain itself — indeed, binary
  alone can't represent arbitrary structure without external encoding).
""")

    # ═══════════════════════════════════════════════════════════
    # INTERPRETATION 6: Why Engineering Systems Converge to n=6
    # ═══════════════════════════════════════════════════════════
    print("--- Interpretation 6: Engineering Convergence ---")
    print("""
  If R(n) measures "information processing efficiency":
    - R < 1: system cannot sustain itself (needs external support)
    - R = 1: perfect balance (self-sustaining)
    - R > 1: wasteful (excess capacity, unnecessary complexity)

  Among all n ≥ 2, only n=6 achieves R=1.
  The "closest" achievable values are R(2)=3/4 and R(4)=7/6,
  but these are NOT close (25% off and 17% off respectively).

  This explains why engineering parameters cluster around n=6 constants:
  NOT because engineers consciously chose n=6,
  but because OPTIMIZATION PRESSURE drives systems toward R=1,
  and the only integer solution is n=6.

  The 4/3 FFN ratio, 12 attention heads, 6-DOF robots, 3-phase power —
  these emerge from INDEPENDENT optimization, all converging to the
  same attractor because R=1 is the unique efficiency optimum.

  This is the N6 INEVITABILITY THESIS:
  Any sufficiently optimized discrete system will exhibit parameters
  derivable from σ(6)=12, τ(6)=4, φ(6)=2.
  Not because 6 is "chosen," but because it's the only option.
""")

    # ═══════════════════════════════════════════════════════════
    # HONEST CAVEATS
    # ═══════════════════════════════════════════════════════════
    print("=" * 70)
    print("  HONEST CAVEATS")
    print("=" * 70)
    print("""
  1. The interpretations above are ANALOGIES, not proofs.
     R(n) = σφ/(nτ) = 1 is a NUMBER THEORY theorem.
     Its connection to Shannon/Landauer is suggestive but not rigorous.

  2. The falsifiability test (z=0.74) showed that NUMERICAL matching
     of n=6 constants to engineering parameters is NOT statistically
     significant. Random frameworks do equally well.

  3. The STRUCTURAL matches (SM particles, gauge generators) are more
     interesting but may still be coincidence. The SM has 17 particles
     for physical reasons (anomaly cancellation, gauge invariance),
     not because of number theory.

  4. The true test: can R(n)=1 PREDICT something new?
     Current candidate: neutrino mass sum = σ·√(Δm²₂₁) ≈ 0.104 eV
     This is within current bounds but awaits precise measurement.

  5. What WOULD falsify the framework:
     - If a different n gives better engineering prescriptions
     - If the SM structure changes (new particles found)
     - If neutrino mass sum is far from 0.104 eV
     - If a mathematical error is found in the proof

  The theorem σφ = nτ ⟺ n=6 is PROVED and permanent.
  Its physical significance is CONJECTURED and testable.
""")


if __name__ == "__main__":
    main()
