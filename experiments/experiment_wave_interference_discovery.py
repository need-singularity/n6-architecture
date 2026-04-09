#!/usr/bin/env python3
"""
Experiment: Wave Interference Discovery
========================================
Cross-domain frequency resonance scanning via simulated WaveLens DFT outputs.

n=6 connection:
  - n6 ratio targets derived from divisors/multiples of 6: {1/6, 1/4, 1/3, 1/2, 2/3, 1, 3/2, 2, 3, 4, 6, 12, 24}
  - Constructive interference nodes where 3+ domains share a ratio -> universal constants
  - attractor_288 = n * J2(6) = 6 * 48 = 288 emerges at interference convergence

Algorithm:
  1. For each domain, generate a characteristic signal and compute naive DFT
  2. Extract dominant_frequency per domain (mirroring WaveLens.scan output)
  3. For every domain pair, compute frequency ratio
  4. Check ratio proximity to n6 targets: <=1% EXACT, <=5% CLOSE
  5. Build interference graph: nodes = domains, edges = resonant pairs
  6. Identify constructive_interference (3+ domains sharing same ratio class)
  7. Report interference_nodes (universal) vs anti_nodes (domain-specific)

참고: WaveLens (DFT), OceanWaveLens (ocean harmonics), LightWaveLens 출력의
dominant_frequency 값을 교차 비교하여 n=6 비율 공명을 탐지.
"""

import math
import random
import itertools
from collections import defaultdict

# ---------------------------------------------------------------------------
# Constants
# ---------------------------------------------------------------------------
N6_RATIO_TARGETS = sorted([
    1/6, 1/4, 1/3, 1/2, 2/3,
    1.0,
    3/2, 2.0, 3.0, 4.0, 6.0, 12.0, 24.0,
])

EXACT_TOL = 0.01   # 1%
CLOSE_TOL = 0.05   # 5%

ATTRACTOR_288 = 288.0  # n * J2(6) = 6 * 48

DOMAINS = [
    "math",     # pure mathematics -- prime distribution
    "physics",  # quantum field harmonics
    "bio",      # circadian / neural oscillation
    "info",     # information channel capacity
    "mind",     # cognitive rhythm / attention
    "arch",     # architectural acoustics
    "ocean",    # tidal / wave period
    "music",    # harmonic series
    "chem",     # molecular vibration
    "astro",    # orbital resonance
]

# ---------------------------------------------------------------------------
# Reproducible seed
# ---------------------------------------------------------------------------
random.seed(42)

# ---------------------------------------------------------------------------
# Signal generation per domain (characteristic frequencies baked in)
# ---------------------------------------------------------------------------

def _domain_base_freq(domain):
    """Return a characteristic base frequency for each domain.

    These are chosen so that natural n=6-ratio relationships exist between
    some domain pairs, while others are off-ratio (controls).
    """
    table = {
        "math":    6.0,
        "physics": 12.0,     # 2x math -> ratio 2
        "bio":     4.0,      # ratio to math: 4/6 = 2/3
        "info":    3.0,      # ratio to math: 1/2
        "mind":    2.0,      # ratio to math: 1/3
        "arch":    24.0,     # ratio to math: 4
        "ocean":   1.0,      # ratio to math: 1/6
        "music":   18.0,     # ratio to math: 3
        "chem":    8.5,      # not an exact n6 ratio to math (control)
        "astro":   7.7,      # not an exact n6 ratio to math (control)
    }
    return table.get(domain, 5.0)


def generate_signal(domain, n_samples=256):
    """Generate a 1D signal with the domain's characteristic frequency + noise."""
    base_f = _domain_base_freq(domain)
    two_pi = 2.0 * math.pi
    signal = []
    for t in range(n_samples):
        phase = two_pi * base_f * t / n_samples
        # fundamental + light harmonic + noise
        val = (math.sin(phase)
               + 0.3 * math.sin(2 * phase)
               + 0.05 * random.gauss(0, 1))
        signal.append(val)
    return signal


# ---------------------------------------------------------------------------
# Naive DFT (mirrors WaveLens Rust implementation)
# ---------------------------------------------------------------------------

def naive_dft_dominant(signal):
    """Compute DFT magnitudes and return (dominant_freq, spectral_entropy, peak_count).

    Replicates the algorithm in tools/nexus/src/telescope/lenses/wave_lens.rs.
    """
    n = len(signal)
    mean = sum(signal) / n
    centered = [x - mean for x in signal]
    half = n // 2
    two_pi = 2.0 * math.pi

    magnitudes = []
    for k in range(1, half + 1):
        re = 0.0
        im = 0.0
        for t, x in enumerate(centered):
            angle = two_pi * k * t / n
            re += x * math.cos(angle)
            im -= x * math.sin(angle)
        magnitudes.append(math.sqrt(re * re + im * im) / n)

    # dominant frequency
    max_mag = max(magnitudes)
    dominant_freq = magnitudes.index(max_mag) + 1  # 1-indexed

    # spectral entropy
    total_power = sum(m * m for m in magnitudes)
    if total_power > 1e-15:
        entropy = 0.0
        for m in magnitudes:
            p = (m * m) / total_power
            if p > 1e-30:
                entropy -= p * math.log(p)
    else:
        entropy = 0.0

    # peak count
    mean_mag = sum(magnitudes) / len(magnitudes)
    peak_count = 0
    for i in range(len(magnitudes)):
        left = magnitudes[i - 1] if i > 0 else 0.0
        right = magnitudes[i + 1] if i + 1 < len(magnitudes) else 0.0
        if magnitudes[i] > left and magnitudes[i] > right and magnitudes[i] > mean_mag:
            peak_count += 1

    return float(dominant_freq), entropy, peak_count


# ---------------------------------------------------------------------------
# Ratio classification
# ---------------------------------------------------------------------------

def classify_ratio(ratio):
    """Check if ratio matches any n6 target. Returns (target, grade) or None."""
    for target in N6_RATIO_TARGETS:
        if target < 1e-12:
            continue
        rel_err = abs(ratio - target) / target
        if rel_err <= EXACT_TOL:
            return (target, "EXACT")
        elif rel_err <= CLOSE_TOL:
            return (target, "CLOSE")
    return None


# ---------------------------------------------------------------------------
# Attractor-288 proximity
# ---------------------------------------------------------------------------

def attractor_288_score(freq_sum):
    """Gaussian proximity to attractor 288 (mirrors OceanWaveLens)."""
    return math.exp(-((freq_sum - ATTRACTOR_288) / ATTRACTOR_288) ** 2 * 40.0)


# ---------------------------------------------------------------------------
# Main experiment
# ---------------------------------------------------------------------------

def main():
    print("=" * 70)
    print("  Experiment: Wave Interference Discovery")
    print("  Cross-domain frequency resonance via n=6 ratio scanning")
    print("=" * 70)

    # Step 1: Generate signals and extract dominant frequencies
    print("\n[Phase 1] DFT per domain (WaveLens algorithm)")
    print("-" * 50)

    domain_freqs = {}
    for domain in DOMAINS:
        signal = generate_signal(domain)
        dom_freq, entropy, peaks = naive_dft_dominant(signal)
        domain_freqs[domain] = dom_freq
        print("  {:8s}  dominant_freq={:6.1f}  entropy={:.4f}  peaks={}".format(
            domain, dom_freq, entropy, peaks))

    # Step 2: Pairwise ratio analysis
    n_pairs = len(DOMAINS) * (len(DOMAINS) - 1) // 2
    print("\n[Phase 2] Pairwise ratio scan ({}C2 = {} pairs)".format(
        len(DOMAINS), n_pairs))
    print("-" * 50)

    resonances = []        # (domA, domB, ratio, target, grade)
    ratio_groups = defaultdict(list)  # target -> [(domA, domB, grade)]

    for domA, domB in itertools.combinations(DOMAINS, 2):
        fA = domain_freqs[domA]
        fB = domain_freqs[domB]
        if fA < 1e-12 or fB < 1e-12:
            continue
        # Check both ratio directions
        for ratio, d1, d2 in [(fA / fB, domA, domB), (fB / fA, domB, domA)]:
            result = classify_ratio(ratio)
            if result:
                target, grade = result
                resonances.append((d1, d2, ratio, target, grade))
                ratio_groups[target].append((d1, d2, grade))
                break  # one match per pair is enough

    exact_count = sum(1 for r in resonances if r[4] == "EXACT")
    close_count = sum(1 for r in resonances if r[4] == "CLOSE")

    print("  Resonant pairs: {} / {}".format(len(resonances), n_pairs))
    print("  EXACT (<=1%): {}".format(exact_count))
    print("  CLOSE (<=5%): {}".format(close_count))
    print()

    for r in sorted(resonances, key=lambda x: x[3]):
        tag = "***" if r[4] == "EXACT" else "   "
        print("  {} {:8s} / {:8s}  ratio={:.4f}  target={:<6.4f}  [{}]".format(
            tag, r[0], r[1], r[2], r[3], r[4]))

    # Step 3: Constructive interference detection
    print("\n[Phase 3] Constructive interference (3+ domains on same ratio)")
    print("-" * 50)

    interference_nodes = []   # universal constants
    anti_nodes = []           # domain-specific (isolated)

    for target, pairs in sorted(ratio_groups.items()):
        # Collect all domains involved
        involved = set()
        for d1, d2, _ in pairs:
            involved.add(d1)
            involved.add(d2)
        if len(involved) >= 3:
            interference_nodes.append((target, involved, pairs))
        else:
            anti_nodes.append((target, involved, pairs))

    if interference_nodes:
        print("  Interference nodes (universal): {}".format(len(interference_nodes)))
        for target, domains_set, pairs in interference_nodes:
            exact_in = sum(1 for _, _, g in pairs if g == "EXACT")
            print("    ratio={:<6.4f}  domains={}  pairs={}  exact={}".format(
                target, sorted(domains_set), len(pairs), exact_in))
    else:
        print("  No constructive interference found (need more domains)")

    if anti_nodes:
        print("\n  Anti-nodes (domain-specific): {}".format(len(anti_nodes)))
        for target, domains_set, pairs in anti_nodes:
            print("    ratio={:<6.4f}  domains={}".format(
                target, sorted(domains_set)))

    # Step 4: Attractor-288 analysis
    print("\n[Phase 4] Attractor-288 convergence (n*J2 = 6*48)")
    print("-" * 50)

    freq_values = list(domain_freqs.values())
    freq_sum = sum(freq_values)
    a288 = attractor_288_score(freq_sum)

    # Also check if any subset of domain frequencies sums to ~288
    best_subset = None
    best_a288 = 0.0
    # Check subsets of size 3..8 for computational feasibility
    for size in range(3, min(len(DOMAINS) + 1, 9)):
        for combo in itertools.combinations(DOMAINS, size):
            s = sum(domain_freqs[d] for d in combo)
            score = attractor_288_score(s)
            if score > best_a288:
                best_a288 = score
                best_subset = combo

    print("  Total freq sum: {:.1f}  attractor_score={:.6f}".format(freq_sum, a288))
    if best_subset:
        sub_sum = sum(domain_freqs[d] for d in best_subset)
        print("  Best subset:    {}".format(list(best_subset)))
        print("  Subset sum:     {:.1f}  attractor_score={:.6f}".format(
            sub_sum, best_a288))
        if best_a288 > 0.5:
            print("  ** Strong 288-convergence detected (score > 0.5)")

    # Step 5: Summary
    print("\n{}".format("=" * 70))
    print("  Summary")
    print("{}".format("=" * 70))

    resonance_rate = len(resonances) / max(n_pairs, 1) * 100
    universal_ratio_count = len(interference_nodes)

    print("  Domains scanned:        {}".format(len(DOMAINS)))
    print("  n6 ratio targets:       {}".format(len(N6_RATIO_TARGETS)))
    print("  Resonant pairs:         {} / {} ({:.1f}%)".format(
        len(resonances), n_pairs, resonance_rate))
    print("  EXACT matches:          {}".format(exact_count))
    print("  CLOSE matches:          {}".format(close_count))
    print("  Interference nodes:     {} (universal)".format(universal_ratio_count))
    print("  Anti-nodes:             {} (domain-specific)".format(len(anti_nodes)))
    print("  Attractor-288 score:    {:.6f}".format(best_a288))

    # n=6 verdict
    if resonance_rate > 50 and universal_ratio_count >= 2:
        verdict = "STRONG -- n=6 ratios dominate cross-domain frequency space"
    elif resonance_rate > 30:
        verdict = "MODERATE -- significant n=6 resonance detected"
    else:
        verdict = "WEAK -- limited cross-domain resonance"
    print("  n=6 verdict:            {}".format(verdict))
    print()


if __name__ == "__main__":
    main()
