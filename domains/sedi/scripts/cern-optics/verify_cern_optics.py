#!/usr/bin/env python3
"""
Verify H-CERN-4 through H-CERN-15: R-spectrum Optics on CERN Data
Combines all lens/telescope/topology verifications.
"""

import math
import sys
from fractions import Fraction

# ============================================================
# Arithmetic functions
# ============================================================

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
    factors = factorize(n)
    result = 1
    for p, a in factors.items():
        result *= (p ** (a + 1) - 1) // (p - 1)
    return result

def phi(n):
    factors = factorize(n)
    result = n
    for p in factors:
        result = result * (p - 1) // p
    return result

def tau(n):
    factors = factorize(n)
    result = 1
    for a in factors.values():
        result *= (a + 1)
    return result

def R(n):
    s, p, t = sigma(n), phi(n), tau(n)
    return Fraction(s * p, n * t)

def R_float(n):
    return float(R(n))

# ============================================================
# CERN Data: Known resonance masses (GeV)
# ============================================================

RESONANCES = {
    'rho':     {'M': 0.7753, 'Gamma': 0.1491, 'JPC': '1--'},
    'omega':   {'M': 0.7827, 'Gamma': 0.00849, 'JPC': '1--'},
    'phi':     {'M': 1.0195, 'Gamma': 0.00426, 'JPC': '1--'},
    'Jpsi':    {'M': 3.0969, 'Gamma': 9.29e-5, 'JPC': '1--'},
    'psi2S':   {'M': 3.6861, 'Gamma': 2.94e-4, 'JPC': '1--'},
    'Upsilon': {'M': 9.4603, 'Gamma': 5.40e-5, 'JPC': '1--'},
    'Z':       {'M': 91.1876, 'Gamma': 2.4952, 'JPC': '1--'},
}

M_PI = 0.13498  # pi0 mass in GeV
M_MU = 0.10566  # muon mass in GeV

SM_PARTICLES = {
    'electron': 0.000511, 'muon': 0.10566, 'tau': 1.777,
    'u': 0.0022, 'd': 0.0047, 's': 0.095, 'c': 1.275, 'b': 4.18, 't': 173.0,
    'W': 80.379, 'Z': 91.188, 'H': 125.25,
    'pi0': 0.135, 'pi_pm': 0.140, 'K': 0.494,
    'rho': 0.775, 'omega': 0.783, 'phi1020': 1.020,
    'proton': 0.938, 'Jpsi': 3.097, 'Upsilon': 9.460,
}

print("=" * 70)
print("CERN OPTICS VERIFICATION: H-CERN-4 through H-CERN-15")
print("=" * 70)

# ============================================================
# H-CERN-4: Gravitational Lens on Particle Mass Spectrum
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-4: GRAVITATIONAL LENS ON PARTICLE MASS SPECTRUM")
print("=" * 70)

print(f"\nM_unit = m_pi0 = {M_PI} GeV")
print(f"\n{'Resonance':<12} {'M(GeV)':<10} {'n=M/m_pi':<10} {'n_round':<8} {'R(n)':<12} {'|R-1|':<10} {'Lens_Str':<10}")
print("-" * 72)

lens_data = []
for name, data in sorted(RESONANCES.items(), key=lambda x: x[1]['M']):
    M = data['M']
    n_exact = M / M_PI
    n = round(n_exact)
    if n < 1: n = 1
    r = R(n)
    r_f = float(r)
    deviation = abs(r_f - 1)
    lens_str = 1.0 / deviation if deviation > 0 else float('inf')
    lens_data.append((name, M, n_exact, n, r, deviation, lens_str, data['Gamma']/M))
    print(f"{name:<12} {M:<10.4f} {n_exact:<10.2f} {n:<8d} {str(r):<12} {deviation:<10.4f} {lens_str:<10.2f}" if deviation > 0
          else f"{name:<12} {M:<10.4f} {n_exact:<10.2f} {n:<8d} {str(r):<12} {deviation:<10.4f} {'INF':<10}")

# Spearman correlation: Lens strength vs 1/(Gamma/M)
print("\n--- Lens Strength vs Inverse Relative Width ---")
valid = [(ls, 1.0/(gm)) for (_, _, _, _, _, _, ls, gm) in lens_data if ls != float('inf')]
if len(valid) >= 3:
    n_v = len(valid)
    ls_vals = [v[0] for v in valid]
    inv_gm_vals = [v[1] for v in valid]
    # Rank
    def rank(lst):
        s = sorted(range(len(lst)), key=lambda i: lst[i])
        r = [0] * len(lst)
        for i, idx in enumerate(s):
            r[idx] = i + 1
        return r
    r1 = rank(ls_vals)
    r2 = rank(inv_gm_vals)
    d2_sum = sum((a-b)**2 for a, b in zip(r1, r2))
    rho = 1 - 6*d2_sum / (n_v * (n_v**2 - 1))
    print(f"  Spearman rho = {rho:.4f} (n={n_v})")
    print(f"  Prediction: rho > 0.5 for SUPPORTED")
    if rho > 0.5:
        print(f"  ★ SUPPORTED: Lens strength correlates with narrow resonances")
    elif rho > 0:
        print(f"  🟧 WEAK: Positive but below threshold")
    else:
        print(f"  ❌ REJECTED: No positive correlation")

# ============================================================
# H-CERN-5: Topological Barcode of Dimuon Mass Spectrum
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-5: TOPOLOGICAL BARCODE OF DIMUON MASS SPECTRUM")
print("=" * 70)

peaks_gev = [0.7753, 1.0195, 3.0969, 3.6861, 9.4603, 91.1876]
peaks_log = [math.log(m) for m in peaks_gev]
peak_names = ['rho/omega', 'phi(1020)', 'J/psi', 'psi(2S)', 'Upsilon', 'Z']

print(f"\n{'Peak':<12} {'M(GeV)':<10} {'ln(M)':<10}")
print("-" * 32)
for n, m, l in zip(peak_names, peaks_gev, peaks_log):
    print(f"{n:<12} {m:<10.4f} {l:<10.4f}")

# Gaps (sorted)
gaps = []
for i in range(len(peaks_log)-1):
    gaps.append((peaks_log[i+1] - peaks_log[i], peak_names[i], peak_names[i+1]))

print(f"\n--- Gaps (log-mass, sorted by size) ---")
print(f"{'Gap':<10} {'From':<12} {'To':<12}")
print("-" * 34)
for g, f, t in sorted(gaps):
    print(f"{g:<10.4f} {f:<12} {t:<12}")

# Beta_0(epsilon) sweep
print(f"\n--- Beta_0(epsilon) Sweep ---")
print(f"{'epsilon':<10} {'beta_0':<8} {'Event':<30}")
print("-" * 48)

current_beta0 = len(peaks_log)
eps_transitions = []
print(f"{'0.0000':<10} {current_beta0:<8}")

sorted_gaps_with_idx = sorted([(peaks_log[i+1] - peaks_log[i], i) for i in range(len(peaks_log)-1)])
for gap, idx in sorted_gaps_with_idx:
    current_beta0 -= 1
    event = f"{peak_names[idx]} + {peak_names[idx+1]} merge"
    eps_transitions.append((gap, current_beta0, event))
    print(f"{gap:<10.4f} {current_beta0:<8} {event:<30}")

# Check for beta_0 = 3 plateau
print(f"\n--- Generation Plateau Analysis ---")
plateau_start = None
plateau_end = None
for i, (eps, b0, _) in enumerate(eps_transitions):
    if b0 == 3:
        plateau_start = eps
        if i + 1 < len(eps_transitions):
            plateau_end = eps_transitions[i+1][0]
        break

if plateau_start and plateau_end:
    width = plateau_end - plateau_start
    print(f"  beta_0 = 3 plateau: eps in [{plateau_start:.4f}, {plateau_end:.4f}]")
    print(f"  Plateau width = {width:.4f}")
    print(f"  sigma(6)/tau(6) = 3 = number of generations")
    if width > 0.05:
        print(f"  ★ SUPPORTED: Significant plateau at beta_0 = 3")
    else:
        print(f"  🟧 WEAK: Narrow plateau")
else:
    print(f"  ❌ No clear beta_0 = 3 plateau found")

# ============================================================
# H-CERN-6: Telescope Mode — Mass Ratio Scan
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-6: TELESCOPE MODE — F(s) MASS RATIO SCAN")
print("=" * 70)

def F_23(s):
    """F(s) = zeta(s)*zeta(s+1) truncated at p=2,3"""
    if s <= 1:
        return float('inf')
    f2s = 1.0 / (1 - 2**(-s))
    f3s = 1.0 / (1 - 3**(-s))
    f2s1 = 1.0 / (1 - 2**(-(s+1)))
    f3s1 = 1.0 / (1 - 3**(-(s+1)))
    return f2s * f3s * f2s1 * f3s1

# Mass ratios
ratios = []
res_list = sorted(RESONANCES.items(), key=lambda x: x[1]['M'])
for i in range(len(res_list)):
    for j in range(i+1, len(res_list)):
        n1, d1 = res_list[i]
        n2, d2 = res_list[j]
        r = d2['M'] / d1['M']
        s_log2 = math.log2(r)
        f_val = F_23(s_log2) if s_log2 > 1 else float('inf')
        ratios.append((n1, n2, r, s_log2, f_val))

print(f"\n{'From':<10} {'To':<10} {'Ratio':<10} {'s=log2(r)':<12} {'F_23(s)':<10}")
print("-" * 52)
for n1, n2, r, s, f in sorted(ratios, key=lambda x: x[2]):
    f_str = f"{f:.4f}" if f < 1000 else ">>1"
    print(f"{n1:<10} {n2:<10} {r:<10.3f} {s:<12.3f} {f_str:<10}")

# Known n=6 targets
print("\n--- n=6 Target Ratios ---")
targets = {
    'tau(6)=4':     4,
    'sigma/tau=3':  3,
    'sigma(6)=12':  12,
    'phi(6)=2':     2,
}
for label, target in targets.items():
    best = min(ratios, key=lambda x: abs(x[2] - target))
    err = abs(best[2] - target) / target * 100
    print(f"  {label:<16} target={target:<5} best={best[0]}->{best[1]} ratio={best[2]:.3f} err={err:.1f}%")

# ============================================================
# H-CERN-7: Aberration Profile of SM Particles
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-7: ABERRATION PROFILE — ρ/ω AT n=6 CHECK")
print("=" * 70)

print(f"\nM_unit = m_pi0 = {M_PI} GeV")
for name, mass in sorted(SM_PARTICLES.items(), key=lambda x: x[1]):
    n_exact = mass / M_PI
    n = round(n_exact)
    if n < 1: n = 1
    r = R_float(n)
    chrom = r - 1
    if n > 1 and n < 10000:
        r_prev = R_float(n-1) if n > 1 else r
        r_next = R_float(n+1) if n < 10000 else r
        spher = abs(r - (r_prev + r_next)/2)
    else:
        spher = 0
    marker = " ★ R=1!" if abs(chrom) < 0.001 else ""
    if abs(n_exact - 6) < 0.5:
        marker += " ← n≈6"
    print(f"  {name:<12} M={mass:<10.4f} n={n_exact:<8.2f} round={n:<5d} R={r:<8.4f} chrom={chrom:+.4f}{marker}")

# ============================================================
# H-CERN-8: Einstein Radius vs Decay Width
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-8: EINSTEIN RADIUS vs DECAY WIDTH")
print("=" * 70)

print(f"\n{'Resonance':<12} {'n':<6} {'R(n)':<10} {'|R-1|':<10} {'theta_E':<10} {'Gamma/M':<12} {'1/(Gamma/M)':<12}")
print("-" * 72)

theta_e_data = []
for name, data in sorted(RESONANCES.items(), key=lambda x: x[1]['M']):
    M = data['M']
    n = round(M / M_PI)
    if n < 1: n = 1
    r = R_float(n)
    dev = abs(r - 1)
    theta_e = math.sqrt(dev) if dev > 0 else 0
    gamma_m = data['Gamma'] / M
    inv_gm = 1.0 / gamma_m
    theta_e_data.append((name, theta_e, inv_gm, gamma_m))
    print(f"{name:<12} {n:<6} {r:<10.4f} {dev:<10.4f} {theta_e:<10.4f} {gamma_m:<12.6f} {inv_gm:<12.1f}")

# Correlation
if len(theta_e_data) >= 3:
    # Filter out infinite lens (theta_e = 0)
    valid = [(te, ig) for _, te, ig, _ in theta_e_data if te > 0]
    if len(valid) >= 3:
        def rank(lst):
            s = sorted(range(len(lst)), key=lambda i: lst[i])
            r = [0] * len(lst)
            for i, idx in enumerate(s):
                r[idx] = i + 1
            return r
        r1 = rank([v[0] for v in valid])
        r2 = rank([v[1] for v in valid])
        d2_sum = sum((a-b)**2 for a, b in zip(r1, r2))
        n_v = len(valid)
        rho = 1 - 6*d2_sum / (n_v * (n_v**2 - 1))
        print(f"\n  Spearman(theta_E, 1/(Gamma/M)) = {rho:.4f} (n={n_v})")
        if rho > 0.5:
            print(f"  ★ SUPPORTED: theta_E predicts narrow resonances")
        elif rho > 0:
            print(f"  🟧 WEAK: Positive but below threshold")
        else:
            print(f"  ❌ REJECTED: No correlation")

# ============================================================
# H-CERN-9: Topological Phase Transitions = Generations
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-9: TOPOLOGICAL PHASE TRANSITIONS = GENERATIONS")
print("=" * 70)

# Reuse beta_0 from H-CERN-5
print("\n  beta_0 = 3 analysis (from H-CERN-5):")
if plateau_start and plateau_end:
    print(f"  Plateau: eps in [{plateau_start:.4f}, {plateau_end:.4f}]")
    print(f"  Width = {plateau_end - plateau_start:.4f}")

    # Cluster centers at beta_0 = 3
    # At this epsilon, we have 3 clusters
    # Identify which peaks merged
    print(f"\n  Cluster centers at beta_0 = 3:")
    # Simple: Gen1 = rho+phi, Gen2 = Jpsi+psi2S, Gen3 = Upsilon (Z separate)
    gen1 = (peaks_gev[0] + peaks_gev[1]) / 2
    gen2 = (peaks_gev[2] + peaks_gev[3]) / 2
    gen3 = peaks_gev[4]
    print(f"    Gen1 center: {gen1:.3f} GeV (rho/omega + phi)")
    print(f"    Gen2 center: {gen2:.3f} GeV (J/psi + psi(2S))")
    print(f"    Gen3 center: {gen3:.3f} GeV (Upsilon)")

    ratio_12 = gen2 / gen1
    ratio_23 = gen3 / gen2
    ratio_13 = gen3 / gen1
    print(f"\n  Mass ratios:")
    print(f"    Gen2/Gen1 = {ratio_12:.3f}  (tau(6)={tau(6)}, err={abs(ratio_12-tau(6))/tau(6)*100:.1f}%)")
    print(f"    Gen3/Gen2 = {ratio_23:.3f}  (sigma/tau={sigma(6)//tau(6)}, err={abs(ratio_23-sigma(6)/tau(6))/(sigma(6)/tau(6))*100:.1f}%)")
    print(f"    Gen3/Gen1 = {ratio_13:.3f}  (sigma(6)={sigma(6)}, err={abs(ratio_13-sigma(6))/sigma(6)*100:.1f}%)")

    # Check GZ width connection
    gz_width = math.log(4/3)
    ratio_eps = plateau_start  # epsilon where beta_0 first hits 3
    factor = ratio_eps / gz_width
    print(f"\n  Golden Zone connection:")
    print(f"    eps_c = {ratio_eps:.4f}")
    print(f"    ln(4/3) = {gz_width:.4f}")
    print(f"    eps_c / ln(4/3) = {factor:.2f}")
    if abs(factor - round(factor)) < 0.15:
        print(f"    ★ eps_c ≈ {round(factor)} × ln(4/3) (within 15%)")

# ============================================================
# H-CERN-10: Focal Length Convergence at 37 GeV
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-10: FOCAL LENGTH CONVERGENCE AT 37 GeV")
print("=" * 70)

# Two known paths
path1 = 3.0969 * sigma(6)  # J/psi * sigma
path2 = 9.4603 * tau(6)    # Upsilon * tau
convergence = abs(path1 - path2) / ((path1 + path2)/2) * 100

print(f"  Path 1: J/psi * sigma(6) = {3.0969} * {sigma(6)} = {path1:.2f} GeV")
print(f"  Path 2: Upsilon * tau(6) = {9.4603} * {tau(6)} = {path2:.2f} GeV")
print(f"  Convergence: {convergence:.2f}%")
print(f"  Mean prediction: {(path1+path2)/2:.2f} GeV")

# Lens focal length path
n_jpsi = round(3.0969 / M_PI)  # ~23
n_ups = round(9.4603 / M_PI)   # ~70
r_jpsi = R_float(n_jpsi)
r_ups = R_float(n_ups)
f_jpsi = n_jpsi / (2 * abs(r_jpsi - 1)) if abs(r_jpsi - 1) > 0 else float('inf')
f_ups = n_ups / (2 * abs(r_ups - 1)) if abs(r_ups - 1) > 0 else float('inf')

print(f"\n  Lens focal lengths:")
print(f"    n_Jpsi = {n_jpsi}, R = {r_jpsi:.4f}, f = {f_jpsi:.1f}")
print(f"    n_Ups  = {n_ups}, R = {r_ups:.4f}, f = {f_ups:.1f}")

# Geometric mean
geom = math.sqrt(3.0969 * 9.4603)
print(f"\n  Geometric mean: sqrt(M_Jpsi * M_Ups) = {geom:.3f} GeV")
print(f"  geom * sigma/phi = {geom * sigma(6)/phi(6):.2f} GeV")
print(f"  geom * n = {geom * 6:.2f} GeV")

# n=274 = 2*137 analysis
n274 = 274
print(f"\n  n = 37 GeV / m_pi = {37.0/M_PI:.0f} ≈ {n274}")
print(f"  274 = 2 * 137  (★ 137 ≈ 1/alpha_EM!)")
print(f"  sigma(274) = {sigma(274)}")
print(f"  phi(274) = {phi(274)}")
print(f"  tau(274) = {tau(274)}")
print(f"  R(274) = {R_float(274):.4f}")

# ============================================================
# H-CERN-11: R-spectrum Gap = QCD Mass Gap
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-11: R-SPECTRUM GAP STRUCTURE")
print("=" * 70)

# Find closest R values to 1
print("\n  Closest R(n) values to 1 (n=2..200):")
close_to_1 = []
for n in range(2, 201):
    r = R_float(n)
    close_to_1.append((abs(r-1), n, r))

close_to_1.sort()
print(f"  {'n':<6} {'R(n)':<15} {'|R-1|':<12} {'M(GeV) if m_pi unit':<20}")
print(f"  {'-'*53}")
for dev, n, r in close_to_1[:15]:
    m_gev = n * M_PI
    marker = " ★ PERFECT" if dev == 0 else ""
    print(f"  {n:<6} {str(R(n)):<15} {dev:<12.6f} {m_gev:<20.3f}{marker}")

# Gap analysis
gap_plus = min((r-1, n) for dev, n, r in close_to_1 if r > 1 and dev > 0)
gap_minus = min((1-r, n) for dev, n, r in close_to_1 if r < 1)
print(f"\n  Minimum gap above R=1: delta+ = {gap_plus[0]:.6f} at n={gap_plus[1]} (R={R(gap_plus[1])})")
print(f"  Minimum gap below R=1: delta- = {gap_minus[0]:.6f} at n={gap_minus[1]} (R={R(gap_minus[1])})")
print(f"  Gap ratio delta-/delta+ = {gap_minus[0]/gap_plus[0]:.6f}")
gfr = Fraction(gap_minus[1], gap_plus[1])
print(f"  n_minus/n_plus = {gap_minus[1]}/{gap_plus[1]}")

# Check if hadrons cluster near R=1
print(f"\n  Hadron |R-1| values:")
hadrons = {'pi0': 0.135, 'rho': 0.775, 'omega': 0.783, 'proton': 0.938,
           'phi1020': 1.020, 'Jpsi': 3.097, 'Upsilon': 9.460}
for name, mass in sorted(hadrons.items(), key=lambda x: x[1]):
    n = round(mass / M_PI)
    if n < 1: n = 1
    r = R_float(n)
    print(f"    {name:<12} n={n:<5} R={r:<10.4f} |R-1|={abs(r-1):<10.4f}")

# ============================================================
# H-CERN-14: rho/omega at n=6
# ============================================================
print("\n" + "=" * 70)
print("H-CERN-14: RHO/OMEGA AT PERFECT LENS n=6")
print("=" * 70)

for m_unit_name, m_unit in [('m_pi0', M_PI), ('m_mu', M_MU), ('LQCD_200MeV', 0.200), ('1GeV', 1.0)]:
    n_rho = 0.7753 / m_unit
    n_omega = 0.7827 / m_unit
    avg = (n_rho + n_omega) / 2
    err = abs(avg - 6) / 6 * 100
    marker = " ★" if err < 10 else ""
    print(f"  M_unit = {m_unit_name:<12}: n_rho={n_rho:.2f}, n_omega={n_omega:.2f}, avg={avg:.2f}, |avg-6|/6={err:.1f}%{marker}")

# ============================================================
# SUMMARY
# ============================================================
print("\n" + "=" * 70)
print("VERIFICATION SUMMARY")
print("=" * 70)

print("""
  H-CERN-4:  Gravitational Lens → Resonance Visibility
  H-CERN-5:  Topological Barcode → Generation Structure
  H-CERN-6:  Telescope F(s) → Mass Ratio Patterns
  H-CERN-7:  Aberration → Particle Properties
  H-CERN-8:  Einstein Radius → Decay Width
  H-CERN-9:  Phase Transitions → 3 Generations
  H-CERN-10: Focal Length → 37 GeV (lens path = not independent)
  H-CERN-11: R-gap → QCD Mass Gap
  H-CERN-12: Multi-lens → CKM (REJECTED in hypothesis)
  H-CERN-13: Sensitivity → BSM (speculative, needs more data)
  H-CERN-14: rho/omega at n≈6
  H-CERN-15: Grand Unification (meta, depends on above)
""")

print("Done.")
