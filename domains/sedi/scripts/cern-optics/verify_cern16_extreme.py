#!/usr/bin/env python3
"""
H-CERN-16 EXTREME: Push psi(3770) = 28 × m_π to its absolute limits.

5 extreme tests:
1. ALL flavor thresholds vs perfect numbers
2. Competing sequences (Fibonacci, triangular, primes, squares)
3. ALL PDG resonances (100+): M/m_π proximity to perfect numbers
4. Inverse: how many resonances have M/m_π ≈ integer? Is 28 special among those?
5. Cross-unit: does P_k × M_unit work for OTHER natural units?
"""

import math
import random
from fractions import Fraction

random.seed(42)

# ============================================================
# PDG data: comprehensive resonance list
# ============================================================

M_PI0 = 134.977  # MeV, PDG 2024
M_PI0_ERR = 0.005
PSI3770_MASS = 3773.7  # MeV, PDG 2024
PSI3770_ERR = 0.4

# ALL known narrow resonances and thresholds (MeV)
RESONANCES = {
    # Light unflavored
    'pi0': 134.977, 'eta': 547.862, 'rho(770)': 775.26, 'omega(782)': 782.66,
    'eta_prime': 957.78, 'f0(980)': 990, 'a0(980)': 980, 'phi(1020)': 1019.461,
    'h1(1170)': 1166, 'b1(1235)': 1229.5, 'a1(1260)': 1230, 'f2(1270)': 1275.5,
    'f1(1285)': 1281.9, 'eta(1295)': 1294, 'pi(1300)': 1300, 'a2(1320)': 1318.2,
    'f0(1370)': 1370, 'omega(1420)': 1410, 'f1(1420)': 1426.3, 'rho(1450)': 1465,
    'f0(1500)': 1506, 'f2_prime(1525)': 1517.4, 'rho3(1690)': 1688.8,
    'phi(1680)': 1680, 'phi3(1850)': 1854,
    # Strange
    'K+': 493.677, 'K0': 497.611, 'K*(892)': 891.67, 'K1(1270)': 1253,
    'K1(1400)': 1403, 'K*(1410)': 1414, 'K0*(1430)': 1425, 'K2*(1430)': 1432.4,
    'K*(1680)': 1718, 'K2(1770)': 1773,
    # Charm
    'D+': 1869.66, 'D0': 1864.84, 'D*+': 2010.26, 'D*0': 2006.85,
    'Ds+': 1968.35, 'Ds*+': 2112.2, 'eta_c(1S)': 2983.9, 'J/psi': 3096.9,
    'chi_c0': 3414.71, 'chi_c1': 3510.67, 'h_c': 3525.38, 'chi_c2': 3556.17,
    'eta_c(2S)': 3637.5, 'psi(2S)': 3686.10, 'psi(3770)': 3773.7,
    'psi(4040)': 4039, 'psi(4160)': 4191, 'psi(4415)': 4421,
    # Bottom
    'B+': 5279.34, 'B0': 5279.65, 'Bs0': 5366.88, 'Bc+': 6274.47,
    'eta_b(1S)': 9399.0, 'Upsilon(1S)': 9460.30, 'chi_b0(1P)': 9859.44,
    'chi_b1(1P)': 9892.78, 'chi_b2(1P)': 9912.21, 'Upsilon(2S)': 10023.26,
    'Upsilon(3S)': 10355.2, 'Upsilon(4S)': 10579.4, 'Upsilon(10860)': 10885.2,
    'Upsilon(11020)': 11000,
    # Baryons
    'proton': 938.272, 'neutron': 939.565, 'Lambda': 1115.683,
    'Sigma+': 1189.37, 'Sigma0': 1192.642, 'Sigma-': 1197.449,
    'Xi0': 1314.86, 'Xi-': 1321.71, 'Omega-': 1672.45,
    'Lambda_c+': 2286.46, 'Sigma_c++': 2453.97, 'Xi_c+': 2467.71,
    'Xi_c0': 2470.44, 'Omega_c0': 2695.2, 'Lambda_b0': 5619.6,
    # Weak bosons
    'W': 80369.2, 'Z': 91187.6, 'H': 125250,
}

PERFECT_NUMBERS = [6, 28, 496, 8128, 33550336]

# Flavor thresholds (MeV)
THRESHOLDS = {
    'KK (ss-bar)': 2 * 493.677,       # 987.4
    'DD (cc-bar)': 2 * 1864.84,       # 3729.7
    'DD* (cc-bar)': 1864.84 + 2006.85, # 3871.7
    'D*D* (cc-bar)': 2 * 2006.85,     # 4013.7
    'DsDs (cs-bar)': 2 * 1968.35,     # 3936.7
    'BB (bb-bar)': 2 * 5279.34,       # 10558.7
    'BB* (bb-bar)': 5279.34 + 5325.2,  # 10604.5
    'BsBs (bs-bar)': 2 * 5366.88,     # 10733.8
    'BcBc': 2 * 6274.47,              # 12549.0
}

# Above-threshold resonances
THRESHOLD_RESONANCES = {
    'psi(3770)': (3773.7, 'DD'),      # Just above DD
    'Upsilon(4S)': (10579.4, 'BB'),   # Just above BB
    'phi(1020)': (1019.461, 'KK'),    # Just above KK
}

print("=" * 80)
print("H-CERN-16 EXTREME VERIFICATION: ψ(3770) = 28 × m_π")
print("=" * 80)

# ============================================================
# TEST 1: ALL FLAVOR THRESHOLDS vs PERFECT NUMBERS × m_π
# ============================================================
print("\n" + "=" * 80)
print("TEST 1: ALL FLAVOR THRESHOLDS vs P_k × m_π")
print("=" * 80)

print(f"\n  {'Threshold':<20} {'M (MeV)':<12} {'M/m_π':<10} {'Near PN':<8} {'Dist':<8} {'Error%':<10} {'Grade'}")
print(f"  {'-'*76}")

threshold_matches = []
for name, mass in sorted(THRESHOLDS.items(), key=lambda x: x[1]):
    n = mass / M_PI0
    best_pn = min(PERFECT_NUMBERS, key=lambda p: abs(n - p))
    dist = abs(n - best_pn)
    err = dist / best_pn * 100
    grade = "⭐⭐⭐" if err < 0.5 else "⭐⭐" if err < 2 else "⭐" if err < 5 else "🟧" if err < 10 else "—"
    threshold_matches.append((name, mass, n, best_pn, dist, err))
    print(f"  {name:<20} {mass:<12.1f} {n:<10.3f} {best_pn:<8} {dist:<8.3f} {err:<10.3f} {grade}")

# The KEY test: do threshold resonances match better than thresholds themselves?
print(f"\n  --- Threshold RESONANCES (just above threshold) ---")
for name, (mass, thr_type) in THRESHOLD_RESONANCES.items():
    n = mass / M_PI0
    best_pn = min(PERFECT_NUMBERS, key=lambda p: abs(n - p))
    dist = abs(n - best_pn)
    err = dist / best_pn * 100
    grade = "⭐⭐⭐" if err < 0.5 else "⭐⭐" if err < 2 else "⭐" if err < 5 else "—"
    thr_mass = THRESHOLDS.get(thr_type, 0) if thr_type in [k.split(' ')[0] for k in THRESHOLDS] else 0
    print(f"  {name:<20} {mass:<12.1f} {n:<10.3f} {best_pn:<8} {dist:<8.3f} {err:<10.3f} {grade}  (above {thr_type})")

# Does ψ(3770) match BECAUSE it's near DD threshold, not because of PN?
print(f"\n  KEY QUESTION: Is ψ(3770) matching PN=28 OR matching DD threshold?")
dd_thr = 2 * 1864.84
dd_n = dd_thr / M_PI0
psi_n = 3773.7 / M_PI0
print(f"    DD threshold:  {dd_thr:.1f} MeV = {dd_n:.3f} m_π")
print(f"    ψ(3770):       3773.7 MeV = {psi_n:.3f} m_π")
print(f"    PN=28:         {28*M_PI0:.1f} MeV = 28.000 m_π")
print(f"    DD/m_π - 28:   {dd_n - 28:.3f}")
print(f"    ψ/m_π - 28:    {psi_n - 28:.3f}")
print(f"    2×m_D/m_π:     {dd_n:.4f}")
print(f"    → m_D/m_π = {1864.84/M_PI0:.4f} ≈ 14 = 2×7")
print(f"    → 2 × m_D/m_π = {dd_n:.4f} ≈ 28 = 4×7 = P₂")
print(f"    ★ THE REAL QUESTION: Is m_D ≈ 14 × m_π? (14 = σ(6)+φ(6))")
md_ratio = 1864.84 / M_PI0
print(f"    m_D/m_π = {md_ratio:.4f}, error from 14: {abs(md_ratio-14)/14*100:.2f}%")

# ============================================================
# TEST 2: COMPETING SEQUENCES
# ============================================================
print("\n" + "=" * 80)
print("TEST 2: COMPETING SEQUENCES × m_π vs RESONANCES")
print("=" * 80)

def fibonacci(max_val):
    a, b = 1, 1
    seq = []
    while a <= max_val:
        seq.append(a)
        a, b = b, a + b
    return seq

def triangular(max_val):
    seq = []
    n = 1
    while n * (n + 1) // 2 <= max_val:
        seq.append(n * (n + 1) // 2)
        n += 1
    return seq

def primes(max_val):
    sieve = [True] * (max_val + 1)
    sieve[0] = sieve[1] = False
    for i in range(2, int(max_val**0.5) + 1):
        if sieve[i]:
            for j in range(i*i, max_val + 1, i):
                sieve[j] = False
    return [i for i in range(2, max_val + 1) if sieve[i]]

def squares(max_val):
    return [i*i for i in range(1, int(max_val**0.5) + 2) if i*i <= max_val]

def highly_composite(max_val):
    # First few: 1,2,4,6,12,24,36,48,60,120,180,240,360,720,...
    hc = [1,2,4,6,12,24,36,48,60,120,180,240,360,720,1260,2520,5040,7560,10080,15120,20160,25200,27720,45360,50400,55440,83160]
    return [x for x in hc if x <= max_val]

sequences = {
    'Perfect numbers':   PERFECT_NUMBERS[:4],
    'Fibonacci':         fibonacci(10000)[:15],
    'Triangular':        triangular(10000)[:15],
    'Primes':            primes(100)[:25],
    'Squares':           squares(10000)[:15],
    'Highly composite':  highly_composite(10000)[:15],
    'Powers of 2':       [2**i for i in range(14)],
}

res_masses = sorted(set(RESONANCES.values()))

def best_match_error(seq, m_unit, res_list):
    """For each sequence element, find best resonance match. Return min error%."""
    best_err = float('inf')
    best_info = None
    for n in seq:
        pred = n * m_unit
        for rm in res_list:
            err = abs(pred - rm) / rm * 100
            if err < best_err:
                best_err = err
                best_info = (n, pred, rm)
    return best_err, best_info

print(f"\n  {'Sequence':<22} {'#Terms':<8} {'Best Err%':<12} {'n':<8} {'Pred(MeV)':<12} {'Res(MeV)':<12}")
print(f"  {'-'*74}")

for seq_name, seq in sorted(sequences.items(), key=lambda x: best_match_error(x[1], M_PI0, res_masses)[0]):
    err, info = best_match_error(seq, M_PI0, res_masses)
    n, pred, rm = info
    pf = " ★" if seq_name == 'Perfect numbers' else ""
    print(f"  {seq_name:<22} {len(seq):<8} {err:<12.4f} {n:<8} {pred:<12.1f} {rm:<12.1f}{pf}")

# Monte Carlo: random 4-element sequences
print(f"\n  --- Monte Carlo: how often does a random 4-number set beat PN? ---")
pn_err = best_match_error(PERFECT_NUMBERS[:4], M_PI0, res_masses)[0]
N_MC = 100000
n_better = 0
for _ in range(N_MC):
    rand_seq = sorted(random.sample(range(1, 10001), 4))
    rand_err, _ = best_match_error(rand_seq, M_PI0, res_masses)
    if rand_err <= pn_err:
        n_better += 1
p_beat = n_better / N_MC
print(f"  PN best error: {pn_err:.4f}%")
print(f"  Random 4-set beats PN: {n_better}/{N_MC} = {p_beat:.4f} ({p_beat*100:.2f}%)")

# Fair comparison: same-size sequences
print(f"\n  --- Fair comparison (same size = 4 terms each) ---")
for seq_name, seq in sequences.items():
    seq4 = seq[:4]
    err4, info4 = best_match_error(seq4, M_PI0, res_masses)
    marker = " ★" if seq_name == 'Perfect numbers' else ""
    print(f"  {seq_name:<22} first 4: {seq4}  best_err={err4:.4f}%{marker}")

# ============================================================
# TEST 3: ALL 100+ RESONANCES — M/m_π proximity to PN
# ============================================================
print("\n" + "=" * 80)
print("TEST 3: ALL RESONANCES — M/m_π PROXIMITY TO PERFECT NUMBERS")
print("=" * 80)

all_proximity = []
for name, mass in RESONANCES.items():
    n = mass / M_PI0
    best_pn = min(PERFECT_NUMBERS[:4], key=lambda p: abs(n - p))
    dist = abs(n - best_pn)
    frac_dist = dist / best_pn
    all_proximity.append((frac_dist, dist, n, best_pn, name, mass))

all_proximity.sort()

print(f"\n  Top 20 closest to a perfect number:")
print(f"  {'Rank':<6} {'Resonance':<20} {'M/m_π':<10} {'PN':<6} {'Dist':<8} {'Err%':<10}")
print(f"  {'-'*60}")
for i, (fd, d, n, pn, name, mass) in enumerate(all_proximity[:20], 1):
    print(f"  {i:<6} {name:<20} {n:<10.3f} {pn:<6} {d:<8.3f} {fd*100:<10.3f}")

# Count within various thresholds
for thr_pct in [0.5, 1, 2, 5]:
    count = sum(1 for fd, *_ in all_proximity if fd * 100 < thr_pct)
    print(f"\n  Within {thr_pct}% of a PN: {count}/{len(all_proximity)}")

# MC null: random resonance masses, same count
print(f"\n  --- MC: random masses, same count ---")
N_MC2 = 10000
actual_within_2pct = sum(1 for fd, *_ in all_proximity if fd * 100 < 2)
n_better2 = 0
for _ in range(N_MC2):
    rand_masses = [10**(random.uniform(math.log10(130), math.log10(130000))) for _ in range(len(RESONANCES))]
    rand_count = 0
    for rm in rand_masses:
        n = rm / M_PI0
        best_pn = min(PERFECT_NUMBERS[:4], key=lambda p: abs(n - p))
        if abs(n - best_pn) / best_pn < 0.02:
            rand_count += 1
    if rand_count >= actual_within_2pct:
        n_better2 += 1
p_val = n_better2 / N_MC2
print(f"  Actual resonances within 2% of PN: {actual_within_2pct}")
print(f"  MC p-value: {p_val:.4f}")

# ============================================================
# TEST 4: INVERSE — HOW SPECIAL IS n≈28 AMONG ALL n≈INTEGER?
# ============================================================
print("\n" + "=" * 80)
print("TEST 4: INVERSE — RESONANCES WITH M/m_π ≈ INTEGER")
print("=" * 80)

# For each resonance, how close is M/m_π to an integer?
int_proximity = []
for name, mass in RESONANCES.items():
    n = mass / M_PI0
    nearest_int = round(n)
    if nearest_int < 1:
        nearest_int = 1
    frac_part = abs(n - nearest_int)
    is_pn = nearest_int in PERFECT_NUMBERS
    int_proximity.append((frac_part, nearest_int, n, name, mass, is_pn))

int_proximity.sort()

print(f"\n  Resonances closest to M/m_π = integer:")
print(f"  {'Rank':<6} {'Resonance':<20} {'M/m_π':<10} {'n_int':<6} {'|frac|':<10} {'PN?'}")
print(f"  {'-'*58}")
for i, (fp, ni, n, name, mass, is_pn) in enumerate(int_proximity[:30], 1):
    pn_mark = "★ PERFECT" if is_pn else ""
    print(f"  {i:<6} {name:<20} {n:<10.4f} {ni:<6} {fp:<10.4f} {pn_mark}")

# Where does ψ(3770) rank?
psi_rank = None
for i, (fp, ni, n, name, mass, is_pn) in enumerate(int_proximity, 1):
    if name == 'psi(3770)':
        psi_rank = i
        psi_frac = fp
        break

print(f"\n  ψ(3770) rank: {psi_rank}/{len(int_proximity)} (|frac|={psi_frac:.4f})")
print(f"  ψ(3770) M/m_π = {3773.7/M_PI0:.4f}, nearest integer = 28")

# How many resonances land near a PN vs near a non-PN integer?
pn_count = sum(1 for fp, ni, _, _, _, is_pn in int_proximity if fp < 0.1 and is_pn)
nonpn_count = sum(1 for fp, ni, _, _, _, is_pn in int_proximity if fp < 0.1 and not is_pn)
total_near_int = pn_count + nonpn_count
print(f"\n  Resonances with |frac| < 0.1:")
print(f"    Near a perfect number: {pn_count}")
print(f"    Near a non-PN integer: {nonpn_count}")
print(f"    Total: {total_near_int}")
if total_near_int > 0:
    pn_frac = pn_count / total_near_int
    # Expected: how many integers 1-1000 are perfect? 3 (6,28,496). So 3/1000 = 0.3%
    expected_pn_frac = 3 / 1000
    enrichment = pn_frac / expected_pn_frac if expected_pn_frac > 0 else 0
    print(f"    PN fraction: {pn_frac:.3f} (expected if random: {expected_pn_frac:.3f})")
    print(f"    Enrichment: {enrichment:.1f}x")

# ============================================================
# TEST 5: CROSS-UNIT — P_k × M_unit for various natural units
# ============================================================
print("\n" + "=" * 80)
print("TEST 5: CROSS-UNIT — P_k × M_unit FOR VARIOUS NATURAL UNITS")
print("=" * 80)

UNITS = {
    'm_pi0': 134.977,
    'm_pi+-': 139.571,
    'm_mu': 105.658,
    'm_e': 0.511,
    'LQCD_217': 217,     # PDG αs extraction
    'LQCD_332': 332,     # Lattice QCD
    'f_pi': 130.2,       # Pion decay constant
    'm_eta': 547.862,
    'm_K': 493.677,
    'm_rho': 775.26,
    '1GeV': 1000,
}

print(f"\n  For each unit, find BEST P_k × M_unit match to any resonance:")
print(f"  {'Unit':<12} {'M(MeV)':<10} {'Best PN':<8} {'Pred':<10} {'Resonance':<20} {'Err%':<10}")
print(f"  {'-'*70}")

unit_results = []
for uname, umass in sorted(UNITS.items(), key=lambda x: x[1]):
    best_err = float('inf')
    best_info = None
    for pn in PERFECT_NUMBERS[:4]:
        pred = pn * umass
        for rname, rmass in RESONANCES.items():
            err = abs(pred - rmass) / rmass * 100
            if err < best_err:
                best_err = err
                best_info = (pn, pred, rname, rmass)
    unit_results.append((best_err, uname, umass, best_info))
    pn, pred, rname, rmass = best_info
    marker = " ★" if uname == 'm_pi0' else ""
    print(f"  {uname:<12} {umass:<10.3f} P={pn:<6} {pred:<10.1f} {rname:<20} {best_err:<10.4f}{marker}")

# Is m_pi0 the best unit?
unit_results.sort()
print(f"\n  Units ranked by best match:")
for i, (err, uname, umass, info) in enumerate(unit_results, 1):
    marker = " ★ ψ(3770)" if uname == 'm_pi0' else ""
    print(f"  {i}. {uname:<12} err={err:.4f}%{marker}")

# ============================================================
# TEST 6: THE KILLER TEST — m_D ≈ 14 × m_π
# ============================================================
print("\n" + "=" * 80)
print("TEST 6: THE KILLER — IS m_D/m_π ≈ 14 THE REAL PATTERN?")
print("=" * 80)

md0 = 1864.84
md_plus = 1869.66
md_avg = (md0 + md_plus) / 2

n_D = md_avg / M_PI0
print(f"\n  m_D (avg) = {md_avg:.2f} MeV")
print(f"  m_D / m_π = {n_D:.4f}")
print(f"  Nearest integer: {round(n_D)}")
print(f"  Error from 14: {abs(n_D - 14)/14*100:.3f}%")

# If m_D/m_π ≈ 14, then DD threshold = 2×14 = 28 × m_π
# This means ψ(3770) ≈ 28 × m_π is DERIVATIVE of m_D ≈ 14 × m_π
print(f"\n  CHAIN OF REASONING:")
print(f"    m_D ≈ 14 × m_π  (err {abs(n_D-14)/14*100:.2f}%)")
print(f"    DD threshold = 2 × m_D ≈ 28 × m_π")
print(f"    ψ(3770) sits just above DD threshold")
print(f"    → ψ(3770) ≈ 28 × m_π is a CONSEQUENCE of m_D ≈ 14 × m_π")
print(f"\n  Is 14 special in n=6 arithmetic?")
print(f"    14 = 2 × 7")
print(f"    14 = σ(6) + φ(6) = 12 + 2  ✓")
print(f"    14 = σ(13)  (σ of a prime)")
print(f"    14 = T(4) + T(3) = 10 + 4 (triangular)")
print(f"    → 14 = σ+φ is weakly n=6-connected")

# Test: m_meson / m_π for ALL pseudoscalar mesons
print(f"\n  --- ALL meson masses / m_π ---")
mesons = {
    'pi': 134.977, 'K': 493.677, 'eta': 547.862, 'eta_prime': 957.78,
    'D': 1867.25, 'Ds': 1968.35, 'eta_c': 2983.9, 'B': 5279.5,
    'Bs': 5366.88, 'Bc': 6274.47, 'eta_b': 9399.0,
}
print(f"  {'Meson':<12} {'M/m_π':<10} {'round':<8} {'|frac|':<10} {'Special?'}")
print(f"  {'-'*48}")
for name, mass in sorted(mesons.items(), key=lambda x: x[1]):
    n = mass / M_PI0
    ni = round(n)
    fp = abs(n - ni)
    special = ""
    if ni in PERFECT_NUMBERS: special = "PERFECT!"
    elif ni in [1, 4, 12, 14, 24]: special = f"n=6 arith"
    print(f"  {name:<12} {n:<10.3f} {ni:<8} {fp:<10.3f} {special}")

# ============================================================
# TEST 7: DEFINITIVE COMBINED p-value
# ============================================================
print("\n" + "=" * 80)
print("TEST 7: DEFINITIVE COMBINED p-VALUE")
print("=" * 80)

# Core claim: ψ(3770) = P₂ × m_π at 0.15% error
# Reframe: m_D/m_π ≈ 14 at 1.3% error → DD = 28 × m_π at 0.7% → ψ(3770) ≈ DD

# Most conservative test: across ALL 100+ resonances,
# does ψ(3770) being 0.15% from P₂ × m_π survive?
print(f"\n  Number of resonances tested: {len(RESONANCES)}")
print(f"  Number of perfect numbers tested: 4")
print(f"  Total comparisons: {len(RESONANCES) * 4}")
print(f"  Bonferroni threshold (p=0.05): {0.05 / (len(RESONANCES) * 4):.6f}")
print(f"  = 1 in {int(len(RESONANCES) * 4 / 0.05)}")

# For each resonance × each PN, compute error
all_matches_full = []
for rname, rmass in RESONANCES.items():
    for pn in PERFECT_NUMBERS[:4]:
        pred = pn * M_PI0
        err = abs(pred - rmass) / rmass * 100
        all_matches_full.append((err, rname, rmass, pn))

all_matches_full.sort()
print(f"\n  Top 10 matches across ALL resonance × PN combinations:")
print(f"  {'Rank':<6} {'Resonance':<20} {'PN':<6} {'Pred(MeV)':<12} {'Actual':<12} {'Err%':<10}")
print(f"  {'-'*66}")
for i, (err, rname, rmass, pn) in enumerate(all_matches_full[:10], 1):
    pred = pn * M_PI0
    marker = " ★" if rname == 'psi(3770)' and pn == 28 else ""
    print(f"  {i:<6} {rname:<20} {pn:<6} {pred:<12.1f} {rmass:<12.1f} {err:<10.4f}{marker}")

# MC: for EACH of 100000 random m_unit values, what's the best match?
print(f"\n  --- ULTIMATE MC: random m_unit, best PN×m_unit match to any resonance ---")
N_ULT = 100000
our_best = all_matches_full[0][0]  # Best error across all
n_better_ult = 0
for _ in range(N_ULT):
    # Random m_unit from 50 to 500 MeV (reasonable QCD scale)
    m_unit = 10**(random.uniform(math.log10(50), math.log10(500)))
    best_e = float('inf')
    for pn in PERFECT_NUMBERS[:4]:
        pred = pn * m_unit
        for rmass in RESONANCES.values():
            e = abs(pred - rmass) / rmass * 100
            if e < best_e:
                best_e = e
    if best_e <= our_best:
        n_better_ult += 1

p_ult = n_better_ult / N_ULT
print(f"  Our best match: {our_best:.4f}%")
print(f"  Random m_unit (50-500 MeV) beats this: {n_better_ult}/{N_ULT} = {p_ult:.4f}")
print(f"  p-value: {p_ult:.4f}")

# ============================================================
# FINAL VERDICT
# ============================================================
print("\n" + "=" * 80)
print("FINAL VERDICT")
print("=" * 80)
print(f"""
  TEST 1 (Thresholds):  ψ(3770) matches because DD ≈ 28×m_π,
                        which reduces to m_D ≈ 14×m_π (1.3% err)
  TEST 2 (Sequences):   How do perfect numbers compare to other sequences?
  TEST 3 (All res):     Enrichment of PN proximity among resonances
  TEST 4 (Inverse):     ψ(3770) rank among M/m_π ≈ integer resonances
  TEST 5 (Cross-unit):  Is m_π the uniquely best unit?
  TEST 6 (Killer):      m_D/m_π ≈ 14 is the ROOT cause
  TEST 7 (Combined):    Ultimate p-value across all tests
""")

print("Done.")
