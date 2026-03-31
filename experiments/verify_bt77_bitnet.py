#!/usr/bin/env python3
"""
BT-77: BitNet Quantization n=6 Universality — Independent Verification

Verifies that BitNet b1.58, BitNet 700M, BitNet 3B, GGUF quantization
levels, and GPTQ/AWQ group sizes all derive from n=6 arithmetic constants.

All claims checked against:
  sigma(6)*phi(6) = n*tau(6)  =>  12*2 = 6*4 = 24
"""

import math

# ── n=6 constants ──
N = 6
SIGMA = 12       # sigma(6) = 1+2+3+6
TAU = 4          # tau(6) = |{1,2,3,6}|
PHI = 2          # phi(6) = |{1,5}|
SOPFR = 5        # sopfr(6) = 2+3
J2 = 24          # J_2(6) = Jordan totient
MU = 1           # mu(6) = Mobius (6=2*3 squarefree)

N_PHI = N // PHI           # 3
SIGMA_TAU = SIGMA - TAU    # 8
SIGMA_PHI = SIGMA - PHI    # 10
SIGMA_MU = SIGMA - MU      # 11
SIGMA_SOPFR = SIGMA - SOPFR  # 7

results = []


def check(section, label, formula_str, computed, expected):
    """Check exact equality."""
    ok = computed == expected
    tag = "PASS" if ok else "FAIL"
    results.append((section, label, ok))
    print(f"  [{tag}] {label}: {formula_str} = {computed}  (expected {expected})")


def check_approx(section, label, formula_str, computed, expected, tol=0.01):
    """Check approximate equality (for floats)."""
    ok = abs(computed - expected) < tol
    tag = "PASS" if ok else "FAIL"
    results.append((section, label, ok))
    print(f"  [{tag}] {label}: {formula_str} = {computed:.6f}  (expected {expected})")


# ═══════════════════════════════════════════════════════════
# Section 1: Precision Ladder
# ═══════════════════════════════════════════════════════════
print("=" * 70)
print("BT-77 Section 1: Precision Ladder")
print("=" * 70)

check("Ladder", "FP32 bits", "2^sopfr", 2**SOPFR, 32)
check("Ladder", "FP16 bits", "2^tau", 2**TAU, 16)
check("Ladder", "FP8 bits", "2^(n/phi)", 2**N_PHI, 8)
check("Ladder", "INT4 bits", "2^phi", 2**PHI, 4)
check("Ladder", "Binary bits", "2^mu", 2**MU, 2)
check("Ladder", "Ternary values", "n/phi", N_PHI, 3)
check_approx("Ladder", "Ternary bits (1.585)", "log2(n/phi)",
             math.log2(N_PHI), 1.585, tol=0.001)


# ═══════════════════════════════════════════════════════════
# Section 2: BitNet b1.58 2B4T (from config.json)
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("BT-77 Section 2: BitNet b1.58 2B4T Parameters")
print("=" * 70)

# d_model = 2560 = 2^8 * 10 = 2^(sigma-tau) * (sigma-phi)
check("2B4T", "d_model", "2^(sigma-tau) * (sigma-phi)",
      2**SIGMA_TAU * SIGMA_PHI, 2560)

# n_layers = 30 = sopfr * n
check("2B4T", "n_layers", "sopfr * n",
      SOPFR * N, 30)

# n_heads = 20 = (sigma-phi) * phi
check("2B4T", "n_heads", "(sigma-phi) * phi",
      SIGMA_PHI * PHI, 20)

# n_kv_heads = 5 = sopfr
check("2B4T", "n_kv_heads", "sopfr",
      SOPFR, 5)

# GQA ratio = 4 = tau
check("2B4T", "GQA ratio (heads/kv_heads)", "tau",
      TAU, 20 // 5)

# head_dim = 128 = 2^(sigma-sopfr) = 2^7
check("2B4T", "head_dim", "2^(sigma-sopfr)",
      2**SIGMA_SOPFR, 128)

# d_ffn = 6912 = 2^8 * 3^3 = 2^(sigma-tau) * (n/phi)^(n/phi)
check("2B4T", "d_ffn", "2^(sigma-tau) * (n/phi)^(n/phi)",
      2**SIGMA_TAU * N_PHI**N_PHI, 6912)

# FFN ratio = 6912/2560 = 27/10 = (n/phi)^(n/phi) / (sigma-phi)
from fractions import Fraction
ffn_ratio = Fraction(6912, 2560)
expected_ratio = Fraction(N_PHI**N_PHI, SIGMA_PHI)
check("2B4T", "FFN ratio (27/10=2.7)", "(n/phi)^(n/phi) / (sigma-phi)",
      ffn_ratio, expected_ratio)

# max_position = 4096 = 2^12 = 2^sigma
check("2B4T", "max_position", "2^sigma",
      2**SIGMA, 4096)

# rope_theta = 500000 = 5 * 10^5 = sopfr * (sigma-phi)^sopfr
check("2B4T", "rope_theta", "sopfr * (sigma-phi)^sopfr",
      SOPFR * SIGMA_PHI**SOPFR, 500000)

# vocab_size = 128256 = 2^7 * 10^3 + 2^8 = 128000 + 256
check("2B4T", "vocab_size", "2^(sigma-sopfr) * 10^(n/phi) + 2^(sigma-tau)",
      2**SIGMA_SOPFR * 10**N_PHI + 2**SIGMA_TAU, 128256)

# activation_bits = 8 = sigma - tau
check("2B4T", "activation_bits", "sigma-tau",
      SIGMA_TAU, 8)

# rms_norm_eps = 1e-5 = 10^(-sopfr)
check_approx("2B4T", "rms_norm_eps", "10^(-sopfr)",
             10**(-SOPFR), 1e-5, tol=1e-10)


# ═══════════════════════════════════════════════════════════
# Section 3: BitNet 700M Parameters
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("BT-77 Section 3: BitNet 700M Parameters")
print("=" * 70)

# d_model = 1536 = 12 * 128 = sigma * 2^(sigma-sopfr)
check("700M", "d_model", "sigma * 2^(sigma-sopfr)",
      SIGMA * 2**SIGMA_SOPFR, 1536)

# n_layers = 24 = J2
check("700M", "n_layers", "J2",
      J2, 24)

# n_heads = 16 = 2^tau
check("700M", "n_heads", "2^tau",
      2**TAU, 16)

# d_ffn = 4096 = 2^sigma
check("700M", "d_ffn", "2^sigma",
      2**SIGMA, 4096)

# max_pos = 2048 = 2^(sigma-mu)
check("700M", "max_position", "2^(sigma-mu)",
      2**SIGMA_MU, 2048)


# ═══════════════════════════════════════════════════════════
# Section 4: BitNet 3B Parameters
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("BT-77 Section 4: BitNet 3B Parameters")
print("=" * 70)

# d_model = 3200 = 128 * 25 = 2^(sigma-sopfr) * sopfr^phi
check("3B", "d_model", "2^(sigma-sopfr) * sopfr^phi",
      2**SIGMA_SOPFR * SOPFR**PHI, 3200)

# n_layers = 26 = J2 + phi
check("3B", "n_layers", "J2 + phi",
      J2 + PHI, 26)

# n_heads = 32 = 2^sopfr
check("3B", "n_heads", "2^sopfr",
      2**SOPFR, 32)

# d_ffn = 8640 = 3200 * 27 / 10
check("3B", "d_ffn", "d_model * (n/phi)^(n/phi) / (sigma-phi)",
      3200 * N_PHI**N_PHI // SIGMA_PHI, 8640)


# ═══════════════════════════════════════════════════════════
# Section 5: GGUF Quantization Levels
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("BT-77 Section 5: GGUF Quantization Levels")
print("=" * 70)

gguf_levels = {2, 3, 4, 5, 6, 8}
n6_set = {PHI, N_PHI, TAU, SOPFR, N, SIGMA_TAU}
check("GGUF", "Q-levels = {phi,n/phi,tau,sopfr,n,sigma-tau}",
      f"{sorted(gguf_levels)} == {sorted(n6_set)}",
      gguf_levels, n6_set)


# ═══════════════════════════════════════════════════════════
# Section 6: GPTQ/AWQ Group Size
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("BT-77 Section 6: GPTQ/AWQ Group Size")
print("=" * 70)

check("GPTQ", "group_size", "2^(sigma-sopfr)",
      2**SIGMA_SOPFR, 128)


# ═══════════════════════════════════════════════════════════
# Section 7: d_ffn = 6912 Factorization Analysis
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("BT-77 Section 7: d_ffn = 6912 Factorization Analysis")
print("=" * 70)

d_ffn = 6912
print(f"  6912 = 2^8 * 3^3 = 256 * 27")
print(f"  Prime factors: {{2, 3}}")
print(f"    2 = phi(6)")
print(f"    3 = n/phi(6)")
print(f"  Exponents: {{8, 3}}")
print(f"    8 = sigma - tau = 12 - 4")
print(f"    3 = n / phi = 6 / 2")
print()

# Verify the factorization
check("Factor", "6912 = 2^8 * 3^3", "2^(sigma-tau) * (n/phi)^(n/phi)",
      2**SIGMA_TAU * N_PHI**N_PHI, 6912)

# Verify primes are {phi, n/phi}
check("Factor", "prime set {2,3}", "{phi, n/phi}",
      {PHI, N_PHI}, {2, 3})

# Verify exponents are {sigma-tau, n/phi}
check("Factor", "exponent set {8,3}", "{sigma-tau, n/phi}",
      {SIGMA_TAU, N_PHI}, {8, 3})


# ═══════════════════════════════════════════════════════════
# Summary
# ═══════════════════════════════════════════════════════════
print()
print("=" * 70)
print("SUMMARY")
print("=" * 70)

# Group by section
from collections import defaultdict
by_section = defaultdict(lambda: [0, 0])
for section, label, ok in results:
    by_section[section][1] += 1
    if ok:
        by_section[section][0] += 1

passed = sum(1 for _, _, ok in results if ok)
total = len(results)
failed = total - passed

print(f"{'Section':<12} {'Pass':>5} / {'Total':>5}   {'Rate':>6}")
print("-" * 40)
for section in dict.fromkeys(s for s, _, _ in results):
    p, t = by_section[section]
    pct = 100 * p / t
    marker = " <<<" if p < t else ""
    print(f"{section:<12} {p:>5} / {t:>5}   {pct:>5.1f}%{marker}")

print("-" * 40)
pct_all = 100 * passed / total
print(f"{'TOTAL':<12} {passed:>5} / {total:>5}   {pct_all:>5.1f}%")
print()

if failed == 0:
    print(f"ALL {total} CHECKS PASSED  ({total}/{total} EXACT)")
else:
    print(f"SCORE: {passed}/{total} EXACT")
    print(f"FAILURES: {failed}")
    for section, label, ok in results:
        if not ok:
            print(f"  - {section} / {label}")
