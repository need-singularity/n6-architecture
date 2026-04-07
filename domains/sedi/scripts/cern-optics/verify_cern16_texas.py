#!/usr/bin/env python3
"""
H-CERN-16 Texas Sharpshooter Verification: ψ(3770) = 28 × m_π⁰

Rigorous statistical test of whether the match between the perfect number 28
and the ψ(3770) resonance via m_π⁰ is genuinely special or cherry-picked.
"""

import numpy as np
from collections import defaultdict

np.random.seed(42)

# ============================================================
# 1. PDG Resonances (mass in GeV, 30+ known resonances)
# ============================================================
# Source: Particle Data Group (PDG) Review of Particle Physics
RESONANCES = {
    # Light unflavored mesons
    "ρ(770)":       0.77526,
    "ω(782)":       0.78266,
    "φ(1020)":      1.01946,
    "f₂(1270)":     1.2755,
    "a₂(1320)":     1.3169,
    "f₀(1370)":     1.370,
    "ρ₃(1690)":     1.6888,
    "f₀(1710)":     1.720,
    # Strange mesons
    "K*(892)":      0.89167,
    "K₂*(1430)":    1.4273,
    # Charmonium (cc̄)
    "J/ψ(1S)":     3.09690,
    "χ_c0(1P)":    3.41471,
    "χ_c1(1P)":    3.51067,
    "h_c(1P)":     3.52538,
    "χ_c2(1P)":    3.55617,
    "η_c(2S)":     3.6392,
    "ψ(3770)":     3.7737,
    "ψ(4040)":     4.039,
    "ψ(4160)":     4.191,
    "ψ(4415)":     4.421,
    # Bottomonium (bb̄)
    "Υ(1S)":       9.4603,
    "Υ(2S)":       10.0233,
    "Υ(3S)":       10.3552,
    "Υ(4S)":       10.5794,
    "Υ(10860)":    10.8852,
    "Υ(11020)":    11.0190,
    # Light baryons
    "Δ(1232)":     1.232,
    "N(1440)":     1.440,
    "N(1520)":     1.520,
    "N(1535)":     1.535,
    "N(1680)":     1.685,
    "Λ(1520)":     1.5195,
    "Σ(1385)":     1.3828,
    # W/Z bosons
    "Z⁰":          91.1876,
    "W±":           80.379,
    # Higgs
    "H⁰":          125.25,
}

resonance_masses = np.array(list(RESONANCES.values()))
resonance_names = list(RESONANCES.keys())
N_res = len(resonance_masses)

print("=" * 70)
print("H-CERN-16 Texas Sharpshooter Verification: psi(3770) = 28 x m_pi0")
print("=" * 70)
print(f"\nNumber of PDG resonances used: {N_res}")
print(f"Mass range: {resonance_masses.min():.4f} - {resonance_masses.max():.4f} GeV")
print()

# ============================================================
# 2. Perfect number × m_π⁰ matches
# ============================================================
m_pi0 = 0.134977  # GeV (PDG 2024)
m_pi_charged = 0.13957  # GeV
m_mu = 0.105658  # GeV

perfect_numbers = [6, 28, 496, 8128]

print("-" * 70)
print("SECTION 2: Perfect number × m_pi0 matches")
print("-" * 70)
print(f"{'P_k':>6}  {'P_k×m_π⁰ (GeV)':>16}  {'Nearest Resonance':>18}  {'Mass (GeV)':>12}  {'Error%':>8}")
print("-" * 70)

best_pn_error = 1.0  # track best error across perfect numbers
best_pn = None
best_pn_res = None

for pn in perfect_numbers:
    predicted = pn * m_pi0
    diffs = np.abs(resonance_masses - predicted)
    idx = np.argmin(diffs)
    err_pct = 100 * diffs[idx] / resonance_masses[idx]
    print(f"{pn:>6}  {predicted:>16.4f}  {resonance_names[idx]:>18}  {resonance_masses[idx]:>12.4f}  {err_pct:>8.3f}%")
    if err_pct < best_pn_error:
        best_pn_error = err_pct
        best_pn = pn
        best_pn_res = resonance_names[idx]

print(f"\nBest match: P={best_pn}, resonance={best_pn_res}, error={best_pn_error:.4f}%")

# ============================================================
# 3. Monte Carlo null model (N=100,000)
# ============================================================
print("\n" + "-" * 70)
print("SECTION 3: Monte Carlo null model (N=100,000 trials)")
print("-" * 70)

N_MC = 100_000
mc_min_errors = np.zeros(N_MC)

for trial in range(N_MC):
    # Pick 4 random integers from 1-10000
    rand_ints = np.random.randint(1, 10001, size=4)
    trial_min_err = 1e10
    for n in rand_ints:
        predicted = n * m_pi0
        diffs = np.abs(resonance_masses - predicted)
        idx = np.argmin(diffs)
        err_pct = 100 * diffs[idx] / resonance_masses[idx]
        if err_pct < trial_min_err:
            trial_min_err = err_pct
    mc_min_errors[trial] = trial_min_err

# p-value: fraction of random trials with error <= best perfect number error
p_raw = np.mean(mc_min_errors <= best_pn_error)
p_bonferroni = min(1.0, p_raw * len(perfect_numbers))

print(f"Best perfect number error: {best_pn_error:.4f}%")
print(f"MC null distribution (min error across 4 random integers 1-10000):")
print(f"  Mean:   {np.mean(mc_min_errors):.4f}%")
print(f"  Median: {np.median(mc_min_errors):.4f}%")
print(f"  Std:    {np.std(mc_min_errors):.4f}%")
print(f"  Min:    {np.min(mc_min_errors):.6f}%")
print(f"  5th percentile:  {np.percentile(mc_min_errors, 5):.4f}%")
print(f"  1st percentile:  {np.percentile(mc_min_errors, 1):.4f}%")
print(f"\nRaw p-value: {p_raw:.6f} ({100*p_raw:.4f}%)")
print(f"Bonferroni-corrected p-value (×{len(perfect_numbers)}): {p_bonferroni:.6f} ({100*p_bonferroni:.4f}%)")

if p_bonferroni < 0.01:
    grade = "STRUCTURAL (p < 0.01)"
elif p_bonferroni < 0.05:
    grade = "WEAK EVIDENCE (p < 0.05)"
else:
    grade = "NOT SIGNIFICANT (p >= 0.05)"
print(f"Verdict: {grade}")

# Histogram
print("\nMC error distribution (histogram):")
bins = [0, 0.05, 0.1, 0.2, 0.5, 1.0, 2.0, 5.0, 10.0, 50.0]
counts, _ = np.histogram(mc_min_errors, bins=bins)
for i in range(len(bins) - 1):
    bar = "#" * int(50 * counts[i] / N_MC)
    print(f"  [{bins[i]:>5.2f}, {bins[i+1]:>5.2f})  {counts[i]:>6}  {100*counts[i]/N_MC:>6.2f}%  {bar}")

# ============================================================
# 4. Alternative mass units: m_π± and m_μ
# ============================================================
print("\n" + "-" * 70)
print("SECTION 4: Alternative mass units")
print("-" * 70)

for unit_name, unit_mass in [("m_pi0", m_pi0), ("m_pi±", m_pi_charged), ("m_mu", m_mu)]:
    print(f"\n  Unit: {unit_name} = {unit_mass:.6f} GeV")
    print(f"  {'P_k':>6}  {'Predicted':>12}  {'Nearest':>18}  {'Mass':>10}  {'Error%':>8}")
    for pn in perfect_numbers:
        predicted = pn * unit_mass
        diffs = np.abs(resonance_masses - predicted)
        idx = np.argmin(diffs)
        err_pct = 100 * diffs[idx] / resonance_masses[idx]
        print(f"  {pn:>6}  {predicted:>12.4f}  {resonance_names[idx]:>18}  {resonance_masses[idx]:>10.4f}  {err_pct:>8.3f}%")

# ============================================================
# 5. Is 28 special? Rank among ALL n=1..1000
# ============================================================
print("\n" + "-" * 70)
print("SECTION 5: Rank of n=28 among ALL n=1..1000 (m_pi0 unit)")
print("-" * 70)

all_errors = []
for n in range(1, 1001):
    predicted = n * m_pi0
    diffs = np.abs(resonance_masses - predicted)
    idx = np.argmin(diffs)
    err_pct = 100 * diffs[idx] / resonance_masses[idx]
    all_errors.append((n, err_pct, resonance_names[idx], resonance_masses[idx]))

# Sort by error
all_errors_sorted = sorted(all_errors, key=lambda x: x[1])

# Find rank of n=28
rank_28 = None
error_28 = None
for rank, (n, err, rname, rmass) in enumerate(all_errors_sorted, 1):
    if n == 28:
        rank_28 = rank
        error_28 = err
        break

print(f"\nn=28: error={error_28:.4f}%, rank={rank_28}/1000")
print(f"\nTop 30 best matches (n=1..1000):")
print(f"{'Rank':>4}  {'n':>5}  {'n×m_π⁰':>10}  {'Resonance':>18}  {'Mass':>10}  {'Error%':>8}  {'Perfect?':>8}")
print("-" * 75)
for rank, (n, err, rname, rmass) in enumerate(all_errors_sorted[:30], 1):
    is_perfect = "***" if n in perfect_numbers else ""
    print(f"{rank:>4}  {n:>5}  {n*m_pi0:>10.4f}  {rname:>18}  {rmass:>10.4f}  {err:>8.4f}%  {is_perfect:>8}")

# Count how many n give error < 0.15%
threshold = best_pn_error
n_better = sum(1 for _, err, _, _ in all_errors if err <= threshold)
print(f"\nIntegers n=1..1000 with error <= {threshold:.4f}%: {n_better}")
print(f"Fraction: {n_better}/1000 = {100*n_better/1000:.1f}%")

# ============================================================
# 6. Perfect numbers in the ranking
# ============================================================
print("\n" + "-" * 70)
print("SECTION 6: Where do ALL perfect numbers rank?")
print("-" * 70)
for pn in perfect_numbers:
    for rank, (n, err, rname, rmass) in enumerate(all_errors_sorted, 1):
        if n == pn:
            print(f"  P={pn:>5}: rank {rank:>4}/1000, error={err:.4f}%, match={rname}")
            break

# ============================================================
# 7. Summary
# ============================================================
print("\n" + "=" * 70)
print("SUMMARY")
print("=" * 70)
print(f"  Claim: psi(3770) = 28 × m_pi0")
print(f"  Predicted: {28 * m_pi0:.4f} GeV")
print(f"  Observed:  {RESONANCES['ψ(3770)']:.4f} GeV")
print(f"  Error:     {best_pn_error:.4f}%")
print(f"  MC p-value (raw):        {p_raw:.6f}")
print(f"  MC p-value (Bonferroni): {p_bonferroni:.6f}")
print(f"  Rank of n=28 among 1..1000: {rank_28}/1000")
print(f"  Grade: {grade}")
print("=" * 70)
