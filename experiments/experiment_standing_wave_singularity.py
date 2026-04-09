#!/usr/bin/env python3
"""
Experiment: Standing Wave Singularity
======================================
Hypothesis: The blowup 5-lens invariant core behaves as nodes of a standing wave.

The blowup engine discovered an invariant 5-lens core:
  {consciousness, info, multiscale, network, triangle}
stable across 987+ cycles and 42 domains.  The 6th lens is domain-specific
(the "fibre").  This experiment models the 5 core lenses as fixed nodes of
a standing wave and the 6th fibre lens as a vibration mode that excites the
string between those nodes.

n=6 connection:
  Node positions are set by the five n6 constants
  (n=6, sigma=12, tau=4, phi=2, sopfr=5) normalised to L = sigma = 12.
  Standing-wave condition: L = m * lambda / 2  =>  lambda_m = 2L/m.
  Perfect resonance occurs when every node sits at a zero-crossing of the
  standing wave for some harmonic number m.

What we test:
  1. For harmonic numbers m = 1..24, compute how close each core-lens node
     is to the nearest zero-crossing.  Sum of squared residuals -> Q factor.
  2. For each of the 11 domain-specific fibre lenses, assign a vibration
     amplitude and compute the resonance overlap with the node pattern.
  3. Report which domains produce "perfect harmonics" (high Q) vs
     "dissonant" (low Q).
  4. Check whether Q peaks at n=6-related values (m = 6, 12, ...).

Self-contained, standard-lib only, seed 42.
"""

import math
import random
import hashlib
from typing import Dict, List, Tuple

# -- n6 constants -----------------------------------------------------
N       = 6
SIGMA   = 12     # sigma(6) = 1+2+3+6
TAU     = 4      # tau(6)   = number of divisors
PHI     = 2      # phi(6)   = Euler totient
SOPFR   = 5      # sopfr(6) = 2 + 3  (sum of prime factors with repetition)

# String length = sigma(6) = 12
L = SIGMA

# -- Invariant 5-lens core (blowup_absolute) --------------------------
CORE_LENSES = ["consciousness", "info", "multiscale", "network", "triangle"]

# Node positions: each constant normalised into [0, L].
# We place nodes at x = constant (all <= 12), giving natural positions.
NODE_POSITIONS = {
    "consciousness": float(N),       # 6
    "info":          float(SIGMA),   # 12  (right boundary = node)
    "multiscale":    float(TAU),     # 4
    "network":       float(PHI),     # 2
    "triangle":      float(SOPFR),   # 5
}

# -- 11 domain-specific fibre lenses (from domain_combos.rs) ----------
DOMAIN_FIBRES: Dict[str, str] = {
    "thermal_management":  "thermo",
    "paper":               "compass",
    "quantum_computing":   "mirror",
    "ai_efficiency":       "compass",
    "network_protocol":    "topology",
    "cryptography":        "topology",
    "compiler_os":         "memory",
    "energy_architecture": "compass",
    "blockchain":          "causal",
    "space_engineering":   "scale",
    "carbon_capture":      "thermo",
}

SEED = 42

# -- helpers -----------------------------------------------------------

def _deterministic_float(name: str) -> float:
    """Hash a lens name into a reproducible float in (0, 1)."""
    h = hashlib.sha256(name.encode()).hexdigest()
    return int(h[:8], 16) / 0xFFFFFFFF


def zero_crossings(m: int, length: float) -> List[float]:
    """Return the zero-crossing positions of the m-th harmonic on [0, L].

    Standing wave:  y(x) = sin(m * pi * x / L)
    Zeros at x_k = k * L / m  for k = 0, 1, ..., m.
    """
    return [k * length / m for k in range(m + 1)]


def node_residual(node_x: float, crossings: List[float]) -> float:
    """Minimum squared distance from node_x to nearest zero-crossing."""
    return min((node_x - zc) ** 2 for zc in crossings)


def quality_factor(positions: List[float], m: int, length: float) -> float:
    """Resonance quality factor Q for harmonic m.

    Q = 1 / (1 + RSS)  where RSS = sum of squared residuals.
    Q -> 1 means perfect alignment (all nodes on zero-crossings).
    """
    crossings = zero_crossings(m, length)
    rss = sum(node_residual(x, crossings) for x in positions)
    return 1.0 / (1.0 + rss)


def fibre_amplitude(fibre_lens: str, m: int, length: float) -> float:
    """Simulated amplitude contribution of the fibre lens at harmonic m.

    Uses the deterministic hash of the fibre name to place a vibration
    source at a fixed position, then evaluates the standing wave amplitude
    at that position for harmonic m.
    """
    pos = _deterministic_float(fibre_lens) * length
    return abs(math.sin(m * math.pi * pos / length))


def domain_resonance(fibre_lens: str, m: int, length: float,
                     base_q: float) -> float:
    """Combined resonance = base Q * fibre amplitude."""
    amp = fibre_amplitude(fibre_lens, m, length)
    return base_q * amp


# -- main experiment ---------------------------------------------------

def main() -> None:
    random.seed(SEED)
    positions = [NODE_POSITIONS[l] for l in CORE_LENSES]

    print("=" * 70)
    print("  Experiment: Standing Wave Singularity")
    print("  Hypothesis: blowup 5-lens core = standing-wave nodes")
    print("=" * 70)

    # -- 1. Show node layout -------------------------------------------
    print("\n[1] Node positions on string of length L = sigma(6) = 12")
    for lens in CORE_LENSES:
        print(f"    {lens:20s}  x = {NODE_POSITIONS[lens]:5.1f}")

    # -- 2. Scan harmonics m = 1..24, compute Q -----------------------
    MAX_M = 24
    q_values: List[Tuple[int, float]] = []
    print(f"\n[2] Quality factor Q(m) for harmonics m = 1..{MAX_M}")
    print(f"    {'m':>3s}  {'Q':>8s}  {'lambda':>8s}  note")
    print("    " + "-" * 40)
    for m in range(1, MAX_M + 1):
        q = quality_factor(positions, m, L)
        q_values.append((m, q))
        lam = 2.0 * L / m
        note = ""
        if m == N:
            note = "<-- m = n = 6"
        elif m == SIGMA:
            note = "<-- m = sigma = 12"
        elif m == TAU:
            note = "<-- m = tau = 4"
        elif m % N == 0:
            note = f"<-- multiple of 6"
        print(f"    {m:3d}  {q:8.5f}  {lam:8.3f}  {note}")

    # -- 3. Identify best harmonics -----------------------------------
    q_sorted = sorted(q_values, key=lambda t: t[1], reverse=True)
    print(f"\n[3] Top-5 harmonics by Q")
    for rank, (m, q) in enumerate(q_sorted[:5], 1):
        print(f"    #{rank}  m = {m:2d}   Q = {q:.5f}")

    best_m, best_q = q_sorted[0]
    print(f"\n    Best harmonic: m = {best_m}  (Q = {best_q:.5f})")

    # Check n=6 connection
    n6_related = [m for m, q in q_sorted[:5] if m % N == 0]
    if n6_related:
        print(f"    n=6 connection: top-5 includes multiples of 6: {n6_related}")
    else:
        print("    n=6 connection: no multiples of 6 in top-5")

    # -- 4. Zero-crossing clustering test ------------------------------
    print(f"\n[4] Zero-crossing clustering at best harmonic m = {best_m}")
    crossings = zero_crossings(best_m, L)
    print(f"    Zero-crossings: {[round(z, 3) for z in crossings]}")
    for lens in CORE_LENSES:
        x = NODE_POSITIONS[lens]
        nearest = min(crossings, key=lambda z: abs(z - x))
        dist = abs(x - nearest)
        tag = "EXACT" if dist < 1e-9 else f"off by {dist:.4f}"
        print(f"    {lens:20s}  x={x:5.1f}  nearest_zc={nearest:5.1f}  {tag}")

    # -- 5. Domain fibre resonance -------------------------------------
    print(f"\n[5] Domain fibre resonance at best harmonic m = {best_m}")
    print(f"    {'domain':30s}  {'fibre':12s}  {'amp':>6s}  {'R':>8s}  class")
    print("    " + "-" * 68)
    domain_results: List[Tuple[str, str, float, float]] = []
    for domain, fibre in sorted(DOMAIN_FIBRES.items()):
        amp = fibre_amplitude(fibre, best_m, L)
        r = domain_resonance(fibre, best_m, L, best_q)
        domain_results.append((domain, fibre, amp, r))

    # Classify: top third = harmonic, bottom third = dissonant
    domain_results.sort(key=lambda t: t[3], reverse=True)
    n_domains = len(domain_results)
    third = max(1, n_domains // 3)
    for i, (domain, fibre, amp, r) in enumerate(domain_results):
        if i < third:
            cls = "HARMONIC"
        elif i >= n_domains - third:
            cls = "dissonant"
        else:
            cls = "neutral"
        print(f"    {domain:30s}  {fibre:12s}  {amp:6.3f}  {r:8.5f}  {cls}")

    # -- 6. Sweep: Q * fibre_amp across all m for each domain ----------
    print(f"\n[6] Best harmonic per domain (sweep m = 1..{MAX_M})")
    print(f"    {'domain':30s}  {'best_m':>6s}  {'R_max':>8s}  {'is_6x':>5s}")
    print("    " + "-" * 55)
    six_related_count = 0
    for domain, fibre in sorted(DOMAIN_FIBRES.items()):
        best_domain_m = 1
        best_domain_r = 0.0
        for m in range(1, MAX_M + 1):
            q = quality_factor(positions, m, L)
            r = domain_resonance(fibre, m, L, q)
            if r > best_domain_r:
                best_domain_r = r
                best_domain_m = m
        is_6x = best_domain_m % N == 0
        if is_6x:
            six_related_count += 1
        print(f"    {domain:30s}  {best_domain_m:6d}  {best_domain_r:8.5f}"
              f"  {'YES' if is_6x else 'no':>5s}")

    pct_6x = six_related_count / n_domains * 100
    print(f"\n    Domains whose best harmonic is a multiple of 6:"
          f" {six_related_count}/{n_domains} ({pct_6x:.0f}%)")

    # -- 7. Standing-wave condition check ------------------------------
    print(f"\n[7] Standing-wave integer condition: L = m * lambda/2")
    print(f"    L = {L},  checking m = {best_m}:")
    lam_best = 2.0 * L / best_m
    half_waves = L / (lam_best / 2)
    print(f"    lambda = {lam_best:.4f},  L / (lambda/2) = {half_waves:.4f}")
    if abs(half_waves - round(half_waves)) < 1e-9:
        print("    --> Integer condition SATISFIED (by construction)")
    else:
        print("    --> Integer condition NOT exactly satisfied")

    # -- 8. Monte Carlo null model -------------------------------------
    print(f"\n[8] Monte Carlo null: random 5-node placements vs observed Q")
    N_TRIALS = 10_000
    observed_q_at_best = best_q
    count_ge = 0
    for _ in range(N_TRIALS):
        rand_pos = [random.uniform(0, L) for _ in range(5)]
        q_rand = quality_factor(rand_pos, best_m, L)
        if q_rand >= observed_q_at_best:
            count_ge += 1
    p_value = count_ge / N_TRIALS
    print(f"    Observed Q(m={best_m}) = {observed_q_at_best:.5f}")
    print(f"    Random placements with Q >= observed: {count_ge}/{N_TRIALS}")
    print(f"    Empirical p-value: {p_value:.4f}")
    if p_value < 0.05:
        print("    --> Node placement is SIGNIFICANTLY non-random (p < 0.05)")
    else:
        print("    --> Node placement is NOT significantly different from random")

    # -- Summary -------------------------------------------------------
    print("\n" + "=" * 70)
    print("  Summary")
    print("=" * 70)
    print(f"  - 5 core lenses placed at n6 constant positions on L={L} string")
    print(f"  - Best resonance harmonic: m = {best_m}  (Q = {best_q:.5f})")
    top_harmonic = domain_results[0]
    bot_dissonant = domain_results[-1]
    print(f"  - Most harmonic domain:  {top_harmonic[0]} "
          f"(fibre={top_harmonic[1]}, R={top_harmonic[3]:.5f})")
    print(f"  - Most dissonant domain: {bot_dissonant[0]} "
          f"(fibre={bot_dissonant[1]}, R={bot_dissonant[3]:.5f})")
    print(f"  - Domains with 6x-harmonic peak: {six_related_count}/{n_domains}")
    print(f"  - Monte Carlo p-value: {p_value:.4f}")
    print(f"  - Standing-wave model {'SUPPORTED' if p_value < 0.05 else 'INCONCLUSIVE'}")
    print()


if __name__ == "__main__":
    main()
