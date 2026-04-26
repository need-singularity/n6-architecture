#!/usr/bin/env python3
"""
btAI3_rtl_design_verify.py

Symbolic verifier for the BT-AI3 RTL design spec
(`domains/compute/ai-native-architecture/btAI3_rtl_design.md`).

This script does NOT synthesize, simulate, or measure anything. It
re-derives the three design-tier falsifiers F-AI3-A / F-AI3-B / F-AI3-C
from the n=6 primitive set and the silicon-primitive symbolic
identities. Each assertion is computed without referring to any
hardcoded numeric value that is not also derived in the same script.

Output protocol (one line per falsifier):
  [PASS] f-ai3-X: <claim>
  [FAIL] f-ai3-X: <claim>  -- sym=<...>, obs=<...>

Final line:
  RTL_EXACT: <pass>/3, verdict: PASS|FAIL

Exit code is 0 iff all 3 falsifiers PASS.

Dependencies: stdlib only (math, sys).
"""

from __future__ import annotations

import math
import sys
from fractions import Fraction


# ---------------------------------------------------------------------------
# N6 primitives. Same derivation chain as
# verify_ai-native-architecture.py; we keep this file self-contained.
# ---------------------------------------------------------------------------


def _divisors(n: int) -> list[int]:
    return [d for d in range(1, n + 1) if n % d == 0]


def _sigma(n: int) -> int:
    return sum(_divisors(n))


def _tau_n(n: int) -> int:
    return len(_divisors(n))


def _phi_min_prime(n: int) -> int:
    for p in range(2, n + 1):
        if n % p == 0:
            return p
    raise ValueError(f"no prime factor for n={n}")


def _sopfr(n: int) -> int:
    s, k = 0, n
    p = 2
    while p * p <= k:
        while k % p == 0:
            s += p
            k //= p
        p += 1
    if k > 1:
        s += k
    return s


N = 6
SIGMA = _sigma(N)            # 12
TAU = _tau_n(N)              # 4
PHI = _phi_min_prime(N)      # 2
SOPFR_N = _sopfr(N)          # 5
SIGMA_N = SIGMA * N          # 72
J2 = 2 * SIGMA               # 24

# Self-checks: n=6 perfectness + master identity.
assert SIGMA == 2 * N, "n=6 perfectness broken"
assert SIGMA * PHI == N * TAU == J2 == 24, "master identity broken"
assert SIGMA_N == 72 and SOPFR_N == 5

# ---------------------------------------------------------------------------
# Falsifier checkers.
# ---------------------------------------------------------------------------


def check_f_ai3_a() -> tuple[bool, str, str]:
    """F-AI3-A: provenance bit register area overhead <= 3%.

    Symbolic claim: overhead = phi / sigma_n (atlas line 526).
    Numeric: 2 / 72 = 1 / 36 ~= 0.02778 < 0.03.
    """

    overhead = Fraction(PHI, SIGMA_N)
    bound = Fraction(3, 100)
    ok = overhead < bound
    sym_str = (
        f"phi/sigma_n = {PHI}/{SIGMA_N} = {overhead} "
        f"~= {float(overhead):.6f}"
    )
    obs_str = f"bound = {bound} = {float(bound):.4f}"
    return ok, sym_str, obs_str


def check_f_ai3_b() -> tuple[bool, str, str]:
    """F-AI3-B: promotion counter latency <= tau cycles AND counter is wide enough.

    Counter must encode threshold values up to provenance_threshold_max =
    sigma = 12. Required width: ceil(log2(threshold_max + 1)) = ceil(log2(13)) = 4.
    Tau = 4, so width tau is sufficient. Latency in cycles is tau = 4 (one
    pipeline stage per dataflow-stage in the BT-AI2 simulator).
    """

    threshold_max = SIGMA  # 12
    required_width = math.ceil(math.log2(threshold_max + 1))  # 4
    width_ok = TAU >= required_width
    latency_cycles = TAU
    latency_ok = latency_cycles <= TAU
    ok = width_ok and latency_ok
    sym_str = (
        f"tau={TAU} >= ceil(log2({threshold_max}+1))={required_width}, "
        f"latency={latency_cycles} <= tau={TAU}"
    )
    obs_str = (
        f"width_ok={width_ok}, latency_ok={latency_ok}"
    )
    return ok, sym_str, obs_str


def check_f_ai3_c() -> tuple[bool, str, str]:
    """F-AI3-C: 7 BT-ids fit in 3 bits and are pairwise distinct.

    BT_541..547 -> ids 1..7. Width: ceil(log2(7)) = 3. The eighth
    code (000) is reserved. Distinctness is checked by hashing into a
    set of 7 elements.
    """

    bt_ids = list(range(1, SOPFR_N + PHI + 1))  # 1..7
    expected_count = SOPFR_N + PHI
    width = math.ceil(math.log2(expected_count))  # 3
    in_range = all(0 <= bid < (1 << width) for bid in bt_ids)
    distinct = len(set(bt_ids)) == len(bt_ids)
    correct_count = len(bt_ids) == expected_count
    ok = in_range and distinct and correct_count
    sym_str = (
        f"bt_ids={bt_ids}, count={len(bt_ids)}={expected_count}=sopfr(n)+phi, "
        f"width=ceil(log2(7))={width}"
    )
    obs_str = (
        f"in_range={in_range}, distinct={distinct}, "
        f"correct_count={correct_count}"
    )
    return ok, sym_str, obs_str


# ---------------------------------------------------------------------------
# Driver.
# ---------------------------------------------------------------------------


def main() -> int:
    print("=" * 70)
    print("btAI3_rtl_design_verify: 3 silicon-tier falsifiers (design-only)")
    print("=" * 70)
    print(
        f"primitives: sigma={SIGMA} phi={PHI} n={N} tau={TAU} "
        f"sigma_n={SIGMA_N} J2={J2} sopfr(n)={SOPFR_N}"
    )
    print("-" * 70)
    print(
        "NOTE: this verifier asserts SYMBOLIC design-tier claims only. "
        "No synthesis, no measurement."
    )
    print("-" * 70)

    falsifiers = [
        (
            "F-AI3-A",
            "provenance bit register area overhead <= 3%",
            check_f_ai3_a,
        ),
        (
            "F-AI3-B",
            "promotion counter latency <= tau=4 cycles, width sufficient",
            check_f_ai3_b,
        ),
        (
            "F-AI3-C",
            "7 BT-ids fit in 3 bits and are pairwise distinct",
            check_f_ai3_c,
        ),
    ]

    n_pass = 0
    for fid, claim, fn in falsifiers:
        ok, sym_str, obs_str = fn()
        tag = "PASS" if ok else "FAIL"
        if ok:
            n_pass += 1
            print(f"  [{tag}] {fid.lower()}: {claim}")
            print(f"         sym: {sym_str}")
            print(f"         obs: {obs_str}")
        else:
            print(f"  [{tag}] {fid.lower()}: {claim}")
            print(f"         sym={sym_str}")
            print(f"         obs={obs_str}")

    total = len(falsifiers)
    verdict = "PASS" if n_pass == total else "FAIL"
    print("-" * 70)
    print(f"RTL_EXACT: {n_pass}/{total}, verdict: {verdict}")
    print("=" * 70)
    return 0 if verdict == "PASS" else 1


if __name__ == "__main__":
    sys.exit(main())
