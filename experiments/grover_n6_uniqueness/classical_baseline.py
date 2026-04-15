#!/usr/bin/env python3
"""
Classical brute-force baseline for sigma(n)*phi(n) == n*tau(n).

Counts arithmetic operations and wall time so we can compare against the
Grover simulator.

USAGE:
    python3 classical_baseline.py --sweep 16,64,256,1024,4096
"""
from __future__ import annotations

import argparse
import json
import math
import time
from typing import Dict, List


def sigma_phi_tau(n: int) -> tuple[int, int, int]:
    """Compute (sigma, phi, tau) in a single divisor scan."""
    s = 0
    c = 0
    for d in range(1, n + 1):
        if n % d == 0:
            s += d
            c += 1
    # Euler totient via coprime count (intentionally O(n) to match naive oracle cost)
    p = 0
    if n == 1:
        p = 1
    else:
        for k in range(1, n + 1):
            a, b = k, n
            while b:
                a, b = b, a % b
            if a == 1:
                p += 1
    return s, p, c


def scan(N_max: int) -> tuple[List[int], int, float]:
    """Return (matches, ops, wall_seconds) for n in [2, N_max)."""
    matches: List[int] = []
    ops = 0
    t0 = time.perf_counter()
    for n in range(2, N_max):
        s, p, c = sigma_phi_tau(n)
        ops += n  # dominant divisor scan cost
        if s * p == n * c:
            matches.append(n)
    return matches, ops, time.perf_counter() - t0


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--sweep", type=str, default="16,64,256,1024,4096")
    ap.add_argument("--out", type=str, default=None)
    args = ap.parse_args()

    N_list = [int(x) for x in args.sweep.split(",") if x.strip()]
    results = []
    for N_max in N_list:
        matches, ops, secs = scan(N_max)
        r = {
            "N_max": N_max,
            "matches": matches,
            "ops_estimate": ops,
            "wall_sec": secs,
            "grover_oracle_calls_estimate": int(round((math.pi / 4.0) * math.sqrt(N_max))),
            "speedup_ratio_theoretical": N_max / max(1, (math.pi / 4.0) * math.sqrt(N_max)),
        }
        results.append(r)
        print(
            f"[N_max={N_max:6d}] classical_matches={matches} "
            f"ops~{ops:>10d} wall={secs*1000:8.2f} ms "
            f"grover_iters~{r['grover_oracle_calls_estimate']:4d} "
            f"speedup~{r['speedup_ratio_theoretical']:.1f}x"
        )

    payload = {"results": results}
    if args.out:
        with open(args.out, "w") as f:
            json.dump(payload, f, indent=2)
    else:
        print(json.dumps(payload, indent=2))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
