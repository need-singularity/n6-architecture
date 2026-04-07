#!/usr/bin/env python3
"""
verify_cern5_bootstrap.py — H-CERN-5/H-CERN-9: β₀=3 plateau significance test

Tests whether the β₀=3 plateau in the dimuon mass spectrum is statistically
significant against a Monte Carlo null model of random log-uniform masses.
"""

import numpy as np
from collections import defaultdict

# ── 1. Known dimuon resonance peaks (GeV) ──
PEAKS = {
    'rho/omega': 0.7753,
    'phi(1020)': 1.0195,
    'J/psi':     3.0969,
    'psi(2S)':   3.6861,
    'Upsilon':   9.4603,
    'Z':         91.1876,
}

N_PEAKS = len(PEAKS)
LOG_MASSES = np.sort(np.log10(np.array(list(PEAKS.values()))))

# ── 2. β₀(ε) sweep function ──
def compute_beta0_sweep(log_masses, eps_max=3.0, n_steps=1000):
    """Compute β₀ (connected components) for each ε value."""
    eps_values = np.linspace(0, eps_max, n_steps)
    n = len(log_masses)
    sorted_m = np.sort(log_masses)
    gaps = np.diff(sorted_m)

    beta0_values = np.empty(n_steps, dtype=int)
    for i, eps in enumerate(eps_values):
        # Number of connected components = n - (number of gaps <= eps)
        components = n - np.sum(gaps <= eps)
        beta0_values[i] = max(1, components)

    return eps_values, beta0_values

def find_plateaus(eps_values, beta0_values):
    """Find all plateaus: contiguous ranges where β₀ is constant."""
    plateaus = []
    current_val = beta0_values[0]
    start_eps = eps_values[0]

    for i in range(1, len(beta0_values)):
        if beta0_values[i] != current_val:
            width = eps_values[i-1] - start_eps
            if width > 0:
                plateaus.append({
                    'beta0': current_val,
                    'start': start_eps,
                    'end': eps_values[i-1],
                    'width': width,
                })
            current_val = beta0_values[i]
            start_eps = eps_values[i]

    # Last plateau
    width = eps_values[-1] - start_eps
    if width > 0:
        plateaus.append({
            'beta0': current_val,
            'start': start_eps,
            'end': eps_values[-1],
            'width': width,
        })

    return plateaus

def get_plateau_width(plateaus, target_beta0):
    """Get width of plateau for a specific β₀ value."""
    for p in plateaus:
        if p['beta0'] == target_beta0:
            return p['width']
    return 0.0

def get_clusters_at_beta0(log_masses, target_beta0, plateaus):
    """Get cluster centers when β₀ = target_beta0."""
    for p in plateaus:
        if p['beta0'] == target_beta0:
            eps_mid = (p['start'] + p['end']) / 2.0
            return cluster_centers(log_masses, eps_mid)
    return []

def cluster_centers(log_masses, eps):
    """Compute cluster centers at given ε."""
    sorted_m = np.sort(log_masses)
    clusters = [[sorted_m[0]]]
    for m in sorted_m[1:]:
        if m - clusters[-1][-1] <= eps:
            clusters[-1].append(m)
        else:
            clusters.append([m])
    return [np.mean(c) for c in clusters]

# ── 3. Analyze observed data ──
print("=" * 70)
print("H-CERN-5/H-CERN-9: beta_0=3 Plateau Significance Test")
print("=" * 70)

print("\n-- Observed Dimuon Resonance Peaks --")
print(f"{'Peak':<12} {'Mass (GeV)':>12} {'log10(Mass)':>12}")
print("-" * 38)
for name, mass in PEAKS.items():
    print(f"{name:<12} {mass:>12.4f} {np.log10(mass):>12.4f}")

eps_obs, beta0_obs = compute_beta0_sweep(LOG_MASSES)
plateaus_obs = find_plateaus(eps_obs, beta0_obs)

print("\n-- Observed Plateaus --")
print(f"{'beta_0':>6} {'Start eps':>10} {'End eps':>10} {'Width':>10}")
print("-" * 40)
for p in plateaus_obs:
    marker = " <-- TARGET" if p['beta0'] == 3 else ""
    print(f"{p['beta0']:>6} {p['start']:>10.4f} {p['end']:>10.4f} {p['width']:>10.4f}{marker}")

obs_b3_width = get_plateau_width(plateaus_obs, 3)
print(f"\nObserved beta_0=3 plateau width: {obs_b3_width:.4f}")

# Get cluster centers at β₀=3
centers_obs = get_clusters_at_beta0(LOG_MASSES, 3, plateaus_obs)
if len(centers_obs) == 3:
    # Convert back to linear mass for ratios
    linear_centers = [10**c for c in centers_obs]
    print(f"\nCluster centers at beta_0=3 (log10): {[f'{c:.4f}' for c in centers_obs]}")
    print(f"Cluster centers at beta_0=3 (GeV):   {[f'{c:.2f}' for c in linear_centers]}")

    gen2_gen1 = linear_centers[1] / linear_centers[0]
    gen3_gen2 = linear_centers[2] / linear_centers[1]
    print(f"\nGen2/Gen1 ratio: {gen2_gen1:.4f}  (test against tau(6)=4)")
    print(f"Gen3/Gen2 ratio: {gen3_gen2:.4f}  (test against sigma/tau=3)")
else:
    print(f"\nWarning: Expected 3 clusters, got {len(centers_obs)}")
    gen2_gen1 = None
    gen3_gen2 = None

# ── 4. Monte Carlo null model ──
N_TRIALS = 10000
np.random.seed(42)

print(f"\n{'=' * 70}")
print(f"Monte Carlo Null Model: N={N_TRIALS} trials")
print(f"  6 random masses, log-uniform in [0.1, 100] GeV")
print(f"{'=' * 70}")

mc_b3_exists = 0
mc_b3_widths = []
mc_gen2_gen1_ratios = []
mc_gen3_gen2_ratios = []
mc_all_plateau_widths = defaultdict(list)

for trial in range(N_TRIALS):
    # Generate 6 random log-uniform masses in [0.1, 100]
    log_masses_rand = np.random.uniform(np.log10(0.1), np.log10(100), N_PEAKS)
    log_masses_rand = np.sort(log_masses_rand)

    eps_r, beta0_r = compute_beta0_sweep(log_masses_rand)
    plateaus_r = find_plateaus(eps_r, beta0_r)

    b3_width = get_plateau_width(plateaus_r, 3)

    for p in plateaus_r:
        mc_all_plateau_widths[p['beta0']].append(p['width'])

    if b3_width > 0:
        mc_b3_exists += 1
        mc_b3_widths.append(b3_width)

        # Get cluster ratios
        centers_r = get_clusters_at_beta0(log_masses_rand, 3, plateaus_r)
        if len(centers_r) == 3:
            lc = [10**c for c in centers_r]
            mc_gen2_gen1_ratios.append(lc[1] / lc[0])
            mc_gen3_gen2_ratios.append(lc[2] / lc[1])
    else:
        mc_b3_widths.append(0.0)

# ── 5. Compute p-values ──
mc_b3_widths_arr = np.array(mc_b3_widths)
p_exists = mc_b3_exists / N_TRIALS
p_wider = np.sum(mc_b3_widths_arr >= obs_b3_width) / N_TRIALS

print(f"\n-- beta_0=3 Plateau Results --")
print(f"Observed width:             {obs_b3_width:.4f}")
print(f"Random trials with b0=3:   {mc_b3_exists}/{N_TRIALS} ({p_exists*100:.1f}%)")
print(f"Random with width >= obs:  {np.sum(mc_b3_widths_arr >= obs_b3_width)}/{N_TRIALS}")
print(f"p-value (width >= obs):    {p_wider:.6f}")

if p_wider < 0.001:
    sig = "*** p < 0.001 (highly significant)"
elif p_wider < 0.01:
    sig = "** p < 0.01 (very significant)"
elif p_wider < 0.05:
    sig = "* p < 0.05 (significant)"
else:
    sig = "not significant (p >= 0.05)"
print(f"Significance:              {sig}")

# ── 6. Cluster ratio tests ──
print(f"\n-- Cluster Ratio Tests (at beta_0=3) --")

if gen2_gen1 is not None and len(mc_gen2_gen1_ratios) > 0:
    mc_r21 = np.array(mc_gen2_gen1_ratios)
    mc_r32 = np.array(mc_gen3_gen2_ratios)

    # Test Gen2/Gen1 ~ tau(6) = 4
    p_r21 = np.sum(np.abs(mc_r21 - 4.0) <= np.abs(gen2_gen1 - 4.0)) / len(mc_r21)
    print(f"Gen2/Gen1 observed:  {gen2_gen1:.4f}  (target: tau(6)=4)")
    print(f"  MC mean +/- std:   {np.mean(mc_r21):.4f} +/- {np.std(mc_r21):.4f}")
    print(f"  p-value (as close or closer to 4): {p_r21:.6f}")

    # Test Gen3/Gen2 ~ sigma/tau = 3
    p_r32 = np.sum(np.abs(mc_r32 - 3.0) <= np.abs(gen3_gen2 - 3.0)) / len(mc_r32)
    print(f"Gen3/Gen2 observed:  {gen3_gen2:.4f}  (target: sigma/tau=3)")
    print(f"  MC mean +/- std:   {np.mean(mc_r32):.4f} +/- {np.std(mc_r32):.4f}")
    print(f"  p-value (as close or closer to 3): {p_r32:.6f}")
else:
    print("  Could not compute cluster ratios.")

# ── 7. ASCII Histograms ──
def ascii_histogram(data, bins=20, width=50, title=""):
    """Print ASCII histogram."""
    if len(data) == 0:
        print(f"  (no data)")
        return
    counts, edges = np.histogram(data, bins=bins)
    max_count = max(counts) if max(counts) > 0 else 1
    print(f"\n  {title}")
    print(f"  {'Bin':>12}  {'Count':>6}  Bar")
    print(f"  {'-'*12}  {'-'*6}  {'-'*width}")
    for i in range(len(counts)):
        bar_len = int(counts[i] / max_count * width)
        bar = '#' * bar_len
        label = f"{edges[i]:.3f}-{edges[i+1]:.3f}"
        print(f"  {label:>12}  {counts[i]:>6}  {bar}")

print(f"\n{'=' * 70}")
print("ASCII Histograms")
print(f"{'=' * 70}")

# Histogram of β₀=3 widths from MC
mc_b3_nonzero = mc_b3_widths_arr[mc_b3_widths_arr > 0]
ascii_histogram(mc_b3_nonzero, bins=20, width=50,
                title=f"MC beta_0=3 Plateau Widths (N={len(mc_b3_nonzero)} with b0=3)")
print(f"  Observed width = {obs_b3_width:.4f}  |  Mark: >>>")

# Histogram of Gen2/Gen1 ratios
if len(mc_gen2_gen1_ratios) > 0:
    ascii_histogram(mc_gen2_gen1_ratios, bins=20, width=50,
                    title="MC Gen2/Gen1 Ratios (at beta_0=3)")
    print(f"  Observed = {gen2_gen1:.4f}  |  Target tau(6) = 4.0")

# Histogram of Gen3/Gen2 ratios
if len(mc_gen3_gen2_ratios) > 0:
    ascii_histogram(mc_gen3_gen2_ratios, bins=20, width=50,
                    title="MC Gen3/Gen2 Ratios (at beta_0=3)")
    print(f"  Observed = {gen3_gen2:.4f}  |  Target sigma/tau = 3.0")

# ── Summary ──
print(f"\n{'=' * 70}")
print("SUMMARY")
print(f"{'=' * 70}")
print(f"  beta_0=3 plateau width:  {obs_b3_width:.4f}")
print(f"  p-value (width):         {p_wider:.6f}  {sig}")
print(f"  MC trials with b0=3:     {mc_b3_exists}/{N_TRIALS} ({p_exists*100:.1f}%)")
if gen2_gen1 is not None and len(mc_gen2_gen1_ratios) > 0:
    print(f"  Gen2/Gen1 = {gen2_gen1:.4f}  (tau(6)=4, p={p_r21:.6f})")
    print(f"  Gen3/Gen2 = {gen3_gen2:.4f}  (sigma/tau=3, p={p_r32:.6f})")
print(f"{'=' * 70}")
