#!/usr/bin/env python3
"""
H-CERN-6: Mass ratio ladder matching n=6 constants
Comprehensive verification with Texas Sharpshooter test.

Checks whether pairwise mass ratios of real particles preferentially
land on n=6 number-theoretic targets compared to random mass sets.
"""

import numpy as np
from itertools import combinations
from collections import defaultdict
import time

# ─── PDG Particle Masses (GeV/c²) ───────────────────────────────────────────
PARTICLES = {
    "pi0":      0.135,
    "pi+":      0.140,
    "K+":       0.494,
    "K0":       0.498,
    "eta":      0.548,
    "rho":      0.775,
    "omega":    0.783,
    "eta'":     0.958,
    "phi":      1.020,
    "proton":   0.938,
    "neutron":  0.940,
    "Lambda":   1.116,
    "Sigma+":   1.189,
    "Xi0":      1.315,
    "Omega-":   1.672,
    "D0":       1.865,
    "D+":       1.870,
    "Ds":       1.968,
    "D*":       2.010,
    "eta_c":    2.984,
    "J/psi":    3.097,
    "chi_c1":   3.511,
    "psi2S":    3.686,
    "psi3770":  3.774,
    "B+":       5.279,
    "Bs":       5.367,
    "Upsilon1S": 9.460,
    "eta_b":    9.399,
    "chi_b0":   9.859,
    "Upsilon2S": 10.023,
    "Upsilon3S": 10.355,
    "W":        80.379,
    "Z":        91.188,
    "H":        125.25,
    "top":      173.0,
}

# ─── n=6 Number-Theoretic Targets ───────────────────────────────────────────
# n=6: σ(6)=12, τ(6)=4, φ(6)=2, sopfr(6)=5
# Derived: σ/τ=3, σ-τ=8, n=6, σ+φ=14, σ×φ=24, σ²=144
TARGETS = {
    "phi=2":      2,
    "sigma/tau=3": 3,
    "tau=4":      4,
    "sopfr=5":    5,
    "n=6":        6,
    "sigma-tau=8": 8,
    "sigma=12":   12,
    "sigma+phi=14": 14,
    "sigma*phi=24": 24,
    "sigma^2=144": 144,
}

TOLERANCES = [0.01, 0.02, 0.05]  # 1%, 2%, 5%

def compute_ratios(masses):
    """Compute all pairwise ratios (heavier/lighter)."""
    ratios = []
    for m1, m2 in combinations(masses, 2):
        if m1 > 0 and m2 > 0:
            r = max(m1, m2) / min(m1, m2)
            ratios.append(r)
    return np.array(ratios)

def compute_ratios_with_names(particle_dict):
    """Compute all pairwise ratios with particle pair names."""
    names = list(particle_dict.keys())
    masses = list(particle_dict.values())
    results = []
    for i, j in combinations(range(len(names)), 2):
        m1, m2 = masses[i], masses[j]
        if m1 > 0 and m2 > 0:
            if m1 >= m2:
                r = m1 / m2
                pair = f"{names[i]}/{names[j]}"
            else:
                r = m2 / m1
                pair = f"{names[j]}/{names[i]}"
            results.append((pair, r))
    return results

def count_matches(ratios, target, tol):
    """Count how many ratios are within tol fraction of target."""
    return np.sum(np.abs(ratios - target) / target <= tol)

def main():
    print("=" * 78)
    print("H-CERN-6: Mass Ratio Ladder Matching n=6 Constants")
    print("=" * 78)

    n_particles = len(PARTICLES)
    masses = np.array(list(PARTICLES.values()))
    print(f"\nParticles: {n_particles}")
    print(f"Mass range: {masses.min():.3f} - {masses.max():.1f} GeV/c^2")

    # ─── Compute real ratios ─────────────────────────────────────────────
    named_ratios = compute_ratios_with_names(PARTICLES)
    real_ratios = np.array([r for _, r in named_ratios])
    n_pairs = len(real_ratios)
    print(f"Pairwise ratios: {n_pairs}")
    print(f"Ratio range: {real_ratios.min():.3f} - {real_ratios.max():.1f}")

    # ─── Best matches for each target ────────────────────────────────────
    print("\n" + "=" * 78)
    print("BEST MATCHES PER TARGET (top 3 closest)")
    print("=" * 78)

    for tname, tval in sorted(TARGETS.items(), key=lambda x: x[1]):
        deviations = [(pair, r, abs(r - tval) / tval) for pair, r in named_ratios]
        deviations.sort(key=lambda x: x[2])
        print(f"\n  Target {tname} = {tval}:")
        for pair, r, dev in deviations[:3]:
            marker = " <-- 1%" if dev <= 0.01 else (" <-- 2%" if dev <= 0.02 else (" <-- 5%" if dev <= 0.05 else ""))
            print(f"    {pair:30s}  ratio={r:8.4f}  dev={dev*100:6.2f}%{marker}")

    # ─── Match counts at each tolerance ──────────────────────────────────
    print("\n" + "=" * 78)
    print("MATCH COUNTS (real data)")
    print("=" * 78)

    header = f"{'Target':>20s}"
    for tol in TOLERANCES:
        header += f"  {tol*100:.0f}%"
    print(header)
    print("-" * 40)

    real_counts = {}
    for tname, tval in sorted(TARGETS.items(), key=lambda x: x[1]):
        row = f"{tname:>20s}"
        real_counts[tname] = {}
        for tol in TOLERANCES:
            c = count_matches(real_ratios, tval, tol)
            real_counts[tname][tol] = c
            row += f"  {c:3d}"
        print(row)

    total_real = {}
    row = f"{'TOTAL':>20s}"
    for tol in TOLERANCES:
        s = sum(real_counts[t][tol] for t in TARGETS)
        total_real[tol] = s
        row += f"  {s:3d}"
    print("-" * 40)
    print(row)

    # ─── Texas Sharpshooter: Random Baseline ─────────────────────────────
    print("\n" + "=" * 78)
    print("TEXAS SHARPSHOOTER TEST (10,000 random particle sets)")
    print("  Log-uniform masses in [0.1, 200] GeV, same particle count")
    print("=" * 78)

    N_RANDOM = 10000
    rng = np.random.default_rng(42)
    t0 = time.time()

    # Store counts: random_counts[tname][tol] = list of counts per trial
    random_counts = {t: {tol: [] for tol in TOLERANCES} for t in TARGETS}
    random_totals = {tol: [] for tol in TOLERANCES}

    for trial in range(N_RANDOM):
        # Log-uniform: 10^(uniform(log10(0.1), log10(200)))
        log_masses = rng.uniform(np.log10(0.1), np.log10(200), size=n_particles)
        rand_masses = 10 ** log_masses
        rand_ratios = compute_ratios(rand_masses)

        trial_total = {tol: 0 for tol in TOLERANCES}
        for tname, tval in TARGETS.items():
            for tol in TOLERANCES:
                c = count_matches(rand_ratios, tval, tol)
                random_counts[tname][tol].append(c)
                trial_total[tol] += c
        for tol in TOLERANCES:
            random_totals[tol].append(trial_total[tol])

    elapsed = time.time() - t0
    print(f"  Completed in {elapsed:.1f}s\n")

    # ─── Per-target results ──────────────────────────────────────────────
    print(f"{'Target':>20s} {'Tol':>4s} {'Real':>5s} {'Mean':>7s} {'Std':>6s} {'Z':>7s} {'p-value':>10s}  Sig")
    print("-" * 78)

    significant_results = []

    for tname, tval in sorted(TARGETS.items(), key=lambda x: x[1]):
        for tol in TOLERANCES:
            real_c = real_counts[tname][tol]
            rand_arr = np.array(random_counts[tname][tol])
            mean_r = rand_arr.mean()
            std_r = rand_arr.std()
            z = (real_c - mean_r) / std_r if std_r > 0 else 0.0
            p = np.sum(rand_arr >= real_c) / N_RANDOM

            sig = ""
            if p < 0.001:
                sig = "***"
            elif p < 0.01:
                sig = "**"
            elif p < 0.05:
                sig = "*"

            print(f"{tname:>20s} {tol*100:3.0f}% {real_c:5d} {mean_r:7.2f} {std_r:6.2f} {z:+7.2f} {p:10.4f}  {sig}")

            if p < 0.05:
                significant_results.append((tname, tol, real_c, mean_r, z, p))

    # ─── Total across all targets ────────────────────────────────────────
    print("\n" + "-" * 78)
    print("TOTAL (all targets combined)")
    print(f"{'':>20s} {'Tol':>4s} {'Real':>5s} {'Mean':>7s} {'Std':>6s} {'Z':>7s} {'p-value':>10s}  Sig")
    print("-" * 78)

    for tol in TOLERANCES:
        real_t = total_real[tol]
        rand_arr = np.array(random_totals[tol])
        mean_r = rand_arr.mean()
        std_r = rand_arr.std()
        z = (real_t - mean_r) / std_r if std_r > 0 else 0.0
        p = np.sum(rand_arr >= real_t) / N_RANDOM

        sig = ""
        if p < 0.001:
            sig = "***"
        elif p < 0.01:
            sig = "**"
        elif p < 0.05:
            sig = "*"

        print(f"{'ALL TARGETS':>20s} {tol*100:3.0f}% {real_t:5d} {mean_r:7.2f} {std_r:6.2f} {z:+7.2f} {p:10.4f}  {sig}")

    # ─── Summary ─────────────────────────────────────────────────────────
    print("\n" + "=" * 78)
    print("SUMMARY")
    print("=" * 78)

    if significant_results:
        print(f"\nSignificant results (p < 0.05): {len(significant_results)}")
        print(f"{'Target':>20s} {'Tol':>4s} {'Real':>5s} {'Expected':>8s} {'Z':>7s} {'p':>8s}")
        for tname, tol, real_c, mean_r, z, p in sorted(significant_results, key=lambda x: x[5]):
            print(f"{tname:>20s} {tol*100:3.0f}% {real_c:5d} {mean_r:8.2f} {z:+7.2f} {p:8.4f}")
    else:
        print("\nNo significant results (all p >= 0.05)")

    # Bonferroni correction
    n_tests = len(TARGETS) * len(TOLERANCES)
    bonf_threshold = 0.05 / n_tests
    bonf_sig = [r for r in significant_results if r[5] < bonf_threshold]

    print(f"\nBonferroni correction: {n_tests} tests, threshold = {bonf_threshold:.5f}")
    if bonf_sig:
        print(f"Survives Bonferroni: {len(bonf_sig)}")
        for tname, tol, real_c, mean_r, z, p in bonf_sig:
            print(f"  {tname} at {tol*100:.0f}%: real={real_c}, expected={mean_r:.2f}, Z={z:+.2f}, p={p:.6f}")
    else:
        print("No results survive Bonferroni correction.")

    # ─── Ratio distribution overview ─────────────────────────────────────
    print("\n" + "=" * 78)
    print("RATIO DISTRIBUTION (ASCII histogram, log scale)")
    print("=" * 78)

    bins = [1, 2, 3, 4, 5, 6, 8, 10, 12, 14, 24, 50, 100, 200, 1500]
    for i in range(len(bins) - 1):
        lo, hi = bins[i], bins[i + 1]
        count = np.sum((real_ratios >= lo) & (real_ratios < hi))
        bar = "#" * min(count, 60)
        label = f"[{lo:>4d}-{hi:>4d})"
        print(f"  {label} {count:4d} {bar}")

    # Mark target positions
    print("\n  Target positions in ratio space:")
    for tname, tval in sorted(TARGETS.items(), key=lambda x: x[1]):
        nearby = np.sum(np.abs(real_ratios - tval) / tval <= 0.05)
        print(f"    {tname:>20s} = {tval:>4d}  ({nearby} ratios within 5%)")

    print("\n" + "=" * 78)
    print("H-CERN-6 verification complete.")
    print("=" * 78)

if __name__ == "__main__":
    main()
