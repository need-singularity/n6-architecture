#!/usr/bin/env python3
"""
Grover quantum search re-verification of the n6-architecture uniqueness theorem.

THEOREM (n6-arch core):
    For n >= 2, sigma(n) * phi(n) == n * tau(n) if and only if n == 6.

METHOD:
    Encode [0, N_max) as a register of q = ceil(log2(N_max)) qubits.
    Precompute classical oracle bits f[n] = 1 iff sigma(n)*phi(n) == n*tau(n).
    Build a phase-flip oracle U_f |n> = (-1)^f(n) |n> from f (the "lookup-table
    oracle", as in the classical Grover construction; this is the standard
    data-loading strategy for NISQ-era simulation).
    Apply standard Grover diffusion D = H^q (2|0><0| - I) H^q.
    Iterate k ~ floor(pi/4 * sqrt(N_max / M)) rounds (M = #marked = 1).
    Measure and verify that n = 6 is the dominant outcome with high
    probability (success probability ~ sin^2((2k+1)*theta), where
    sin(theta) = sqrt(M/N_max)).

OUTPUT:
    JSON report (to stdout) summarising qubit count, gate count, oracle
    build, iteration count, empirical success probability, and whether
    the unique-marked outcome is n = 6.

USAGE:
    python3 grover_n6.py --N_max 16 --shots 4096
    python3 grover_n6.py --sweep 16,64,256
"""
from __future__ import annotations

import argparse
import json
import math
import sys
import time
from typing import Dict, List, Tuple

import numpy as np
from qiskit import QuantumCircuit, transpile
from qiskit.circuit.library import ZGate
from qiskit_aer import AerSimulator


# -------------------- classical number theory --------------------

def sigma(n: int) -> int:
    """Divisor sum."""
    s = 0
    for d in range(1, n + 1):
        if n % d == 0:
            s += d
    return s


def phi(n: int) -> int:
    """Euler totient."""
    if n == 1:
        return 1
    r = 0
    for k in range(1, n + 1):
        a, b = k, n
        while b:
            a, b = b, a % b
        if a == 1:
            r += 1
    return r


def tau(n: int) -> int:
    """Divisor count."""
    c = 0
    for d in range(1, n + 1):
        if n % d == 0:
            c += 1
    return c


def marked_set(N_max: int) -> List[int]:
    """Return n in [0, N_max) such that sigma(n)*phi(n) == n*tau(n). n<2 skipped."""
    out = []
    for n in range(2, N_max):
        if sigma(n) * phi(n) == n * tau(n):
            out.append(n)
    return out


# -------------------- Grover building blocks --------------------

def int_to_bits(x: int, q: int) -> List[int]:
    """Little-endian bit list of length q."""
    return [(x >> i) & 1 for i in range(q)]


def phase_oracle_lookup(marked: List[int], q: int) -> QuantumCircuit:
    """Build a phase-flip oracle that flips the sign of every |n> in `marked`.

    For each marked integer m, we build a controlled-Z with control pattern
    matching m's bit pattern:
      1. X on each qubit where m has a 0 bit.
      2. Multi-controlled Z on all q qubits (flip only when all are |1>).
      3. X again to restore.

    This is the canonical lookup-table oracle described in Nielsen & Chuang
    Section 6.1 for classical Boolean functions encoded as phase flips.
    """
    qc = QuantumCircuit(q, name="oracle")
    if not marked:
        return qc

    for m in marked:
        bits = int_to_bits(m, q)
        # X on zero-bits so the target pattern becomes all-ones
        for i, b in enumerate(bits):
            if b == 0:
                qc.x(i)
        if q == 1:
            qc.z(0)
        else:
            # Multi-controlled Z: controls = [0..q-2], target = q-1
            qc.append(ZGate().control(q - 1), list(range(q)))
        for i, b in enumerate(bits):
            if b == 0:
                qc.x(i)
    return qc


def diffusion(q: int) -> QuantumCircuit:
    """Standard Grover diffusion D = H^q (2|0><0| - I) H^q = -H^q X^q (C^{q-1}Z) X^q H^q."""
    qc = QuantumCircuit(q, name="diffusion")
    qc.h(range(q))
    qc.x(range(q))
    if q == 1:
        qc.z(0)
    else:
        qc.append(ZGate().control(q - 1), list(range(q)))
    qc.x(range(q))
    qc.h(range(q))
    return qc


def optimal_iterations(N: int, M: int) -> int:
    """Optimal Grover iteration count k ~ floor(pi/4 * sqrt(N/M))."""
    if M == 0:
        return 0
    return max(1, int(round((math.pi / 4.0) * math.sqrt(N / M))))


def build_grover(N_max: int, marked: List[int], k: int) -> QuantumCircuit:
    """Full Grover circuit with q = ceil(log2(N_max)) qubits and k iterations."""
    q = max(1, math.ceil(math.log2(N_max)))
    qc = QuantumCircuit(q, q)
    qc.h(range(q))
    oracle = phase_oracle_lookup(marked, q).to_gate(label="U_f")
    diff = diffusion(q).to_gate(label="D")
    for _ in range(k):
        qc.append(oracle, range(q))
        qc.append(diff, range(q))
    qc.measure(range(q), range(q))
    return qc


# -------------------- experiment driver --------------------

def run_one(N_max: int, shots: int = 4096, seed: int = 20260416) -> Dict:
    """Run Grover on a single N_max and return a result dict."""
    marked = marked_set(N_max)
    M = len(marked)
    q = max(1, math.ceil(math.log2(N_max)))
    k = optimal_iterations(2 ** q, max(1, M))  # use full register size 2^q as search space

    t_build_0 = time.perf_counter()
    qc = build_grover(N_max, marked, k)
    # Decompose into basis gates so we can count gates meaningfully.
    backend = AerSimulator(seed_simulator=seed)
    tqc = transpile(qc, backend, basis_gates=["cx", "u3", "h", "x", "z"], optimization_level=1)
    t_build = time.perf_counter() - t_build_0

    # Count ops
    op_counts = dict(tqc.count_ops())
    gate_total = sum(v for k_, v in op_counts.items() if k_ != "measure")

    # Execute
    t0 = time.perf_counter()
    result = backend.run(tqc, shots=shots).result()
    t_sim = time.perf_counter() - t0
    counts = result.get_counts()

    # Aggregate by integer value (Qiskit returns little-endian bit strings)
    int_counts: Dict[int, int] = {}
    for bitstr, c in counts.items():
        n_val = int(bitstr, 2)
        int_counts[n_val] = int_counts.get(n_val, 0) + c

    # Top outcome
    top_n, top_c = max(int_counts.items(), key=lambda kv: kv[1])
    top_prob = top_c / shots

    # Probability of 6 specifically
    prob_6 = int_counts.get(6, 0) / shots

    # Theoretical success probability (single marked)
    if M > 0:
        theta = math.asin(math.sqrt(M / (2 ** q)))
        theo = math.sin((2 * k + 1) * theta) ** 2
    else:
        theo = 0.0

    return {
        "N_max": N_max,
        "search_space": 2 ** q,
        "qubits": q,
        "marked_set": marked,
        "num_marked": M,
        "iterations": k,
        "shots": shots,
        "gate_counts": op_counts,
        "gate_total": gate_total,
        "top_n": top_n,
        "top_prob": top_prob,
        "prob_n_eq_6": prob_6,
        "theoretical_success_prob": theo,
        "build_sec": t_build,
        "sim_sec": t_sim,
        "unique_six_confirmed": (marked == [6]) and (top_n == 6),
    }


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--N_max", type=int, default=None, help="upper bound (exclusive) for search space")
    ap.add_argument("--sweep", type=str, default="16,64,256", help="comma-list of N_max values")
    ap.add_argument("--shots", type=int, default=4096)
    ap.add_argument("--out", type=str, default=None, help="optional JSON output path")
    args = ap.parse_args()

    if args.N_max is not None:
        N_list = [args.N_max]
    else:
        N_list = [int(x) for x in args.sweep.split(",") if x.strip()]

    results = []
    for N_max in N_list:
        r = run_one(N_max, shots=args.shots)
        results.append(r)
        print(
            f"[N_max={N_max:5d}] q={r['qubits']} k={r['iterations']:3d} "
            f"marked={r['marked_set']} top_n={r['top_n']} p_top={r['top_prob']:.4f} "
            f"p(n=6)={r['prob_n_eq_6']:.4f} theo={r['theoretical_success_prob']:.4f} "
            f"gates={r['gate_total']} build={r['build_sec']:.3f}s sim={r['sim_sec']:.3f}s "
            f"unique6={r['unique_six_confirmed']}"
        )

    payload = {"results": results}
    if args.out:
        with open(args.out, "w") as f:
            json.dump(payload, f, indent=2)
    else:
        print(json.dumps(payload, indent=2))
    return 0


if __name__ == "__main__":
    sys.exit(main())
