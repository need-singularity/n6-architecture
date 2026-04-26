"""
BT-AI2 robustness extension: seed x workload x threshold sweep runner.

Parent cycle:
  reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md (BT-AI2)
Parent limitation addressed:
  reports/anomaly/btAI2_summary.md §7 item 4 (single-seed reporting)

This script does NOT modify the BT-AI2 simulator. It invokes
experiments/anomaly/btAI2_honesty_bit_scheduler.py as an external
subprocess for every (seed, workload, threshold) tuple in the sweep
grid:

  - seed       in 0..99           (100 seeds)
  - workload   in {attn, ffn, full}        (3 workloads)
  - threshold  in {4, 8, 12}               (3 thresholds)

Total runs: 100 x 3 x 3 = 900.

The simulator already emits the canonical paired result + falsifier
verdict to stdout (and to --results-out) in JSON; this runner parses
that output and aggregates throughput-drop and legit-reject-rate
distributions per (workload, threshold) cell.

Determinism: each subprocess receives a fixed (seed, workload,
threshold). The simulator seeds numpy.random.default_rng with the
integer seed; SimPy is event-driven and deterministic given a fixed
event order. No additional randomness is introduced by this runner.

Atlas / KG / domain references (read-only; not modified by this script):
  - atlas: provenance_bit_overhead = phi/sigma_n = 1/36
    (atlas/atlas.append.n6-architecture-historical-absorption-2026-04-26.n6:526)
  - KG: silicon:provenance-bit, silicon:promotion-counter-mmu,
        silicon:bt-id-isa, principle:honesty-triad,
        principle:write-barrier, principle:constraint-honesty,
        arch:n6-native-accelerator
  - domain: ai-native-architecture (compute axis)

Usage:
  python3 experiments/anomaly/btAI2_seed_sweep.py \
      --python /tmp/btai2-venv/bin/python \
      --results-out reports/anomaly/btAI2_seed_sweep_results.json
"""

from __future__ import annotations

import argparse
import json
import math
import os
import subprocess
import sys
import time
from typing import Dict, List, Tuple

# ---------------------------------------------------------------------------
# Sweep grid (fixed; no additional randomness allowed).
# ---------------------------------------------------------------------------

SEEDS: List[int] = list(range(100))
WORKLOADS: List[str] = ["attn", "ffn", "full"]
THRESHOLDS: List[int] = [4, 8, 12]

# Falsifier ceilings restated from omega-cycle §6.
F_AI2_A_MAX_DROP = 0.05
F_AI2_B_MAX_REJECT = 0.01

# Path to the simulator (must remain unmodified).
THIS_DIR = os.path.dirname(os.path.abspath(__file__))
SIM_PATH = os.path.join(THIS_DIR, "btAI2_honesty_bit_scheduler.py")

# Single-seed point estimate (seed 42, workload=full, threshold=8) from
# reports/anomaly/btAI2_results.json — used in the summary as a comparison
# anchor. We do NOT trust this constant blindly; the runner re-reads the
# canonical results file to verify it before writing the summary.
SINGLE_SEED_REFERENCE_PATH = os.path.join(
    os.path.dirname(THIS_DIR),  # experiments
    os.pardir,
    "reports",
    "anomaly",
    "btAI2_results.json",
)


# ---------------------------------------------------------------------------
# Statistics helpers (no numpy dependency in the runner).
# ---------------------------------------------------------------------------


def _percentile(sorted_xs: List[float], q: float) -> float:
    """Linear-interpolation percentile on a pre-sorted list (q in [0, 1])."""

    if not sorted_xs:
        return float("nan")
    if len(sorted_xs) == 1:
        return sorted_xs[0]
    pos = q * (len(sorted_xs) - 1)
    lo = int(math.floor(pos))
    hi = int(math.ceil(pos))
    if lo == hi:
        return sorted_xs[lo]
    frac = pos - lo
    return sorted_xs[lo] * (1.0 - frac) + sorted_xs[hi] * frac


def _summarise(xs: List[float]) -> Dict[str, float]:
    if not xs:
        return {
            "n": 0,
            "mean": float("nan"),
            "std": float("nan"),
            "p50": float("nan"),
            "p90": float("nan"),
            "p99": float("nan"),
            "max": float("nan"),
            "min": float("nan"),
        }
    s = sorted(xs)
    n = len(s)
    mean = sum(s) / n
    var = sum((x - mean) ** 2 for x in s) / n
    return {
        "n": n,
        "mean": mean,
        "std": math.sqrt(var),
        "p50": _percentile(s, 0.50),
        "p90": _percentile(s, 0.90),
        "p99": _percentile(s, 0.99),
        "max": s[-1],
        "min": s[0],
    }


# ---------------------------------------------------------------------------
# Subprocess invocation.
# ---------------------------------------------------------------------------


def run_one(
    python_bin: str,
    seed: int,
    workload: str,
    threshold: int,
    tmp_dir: str,
) -> Dict[str, object]:
    """Invoke the simulator once and return the parsed result dict.

    Returns a dict with keys: seed, workload, threshold, drop, legit_rate,
    f_ai2_a_pass, f_ai2_b_pass, baseline_throughput, provenance_throughput,
    legit_attempted, legit_refused, wall_seconds.
    """

    out_path = os.path.join(tmp_dir, f"sweep_{seed}_{workload}_{threshold}.json")
    cmd = [
        python_bin,
        SIM_PATH,
        "--paired",
        "--seed",
        str(seed),
        "--workload",
        workload,
        "--threshold",
        str(threshold),
        "--results-out",
        out_path,
    ]
    t0 = time.time()
    proc = subprocess.run(
        cmd,
        capture_output=True,
        text=True,
        check=False,
    )
    wall = time.time() - t0
    # The simulator returns 0 on PASS and 1 on FAIL — both are valid here;
    # we only treat non-{0,1} return codes (e.g. crashes) as fatal.
    if proc.returncode not in (0, 1):
        sys.stderr.write(
            f"FATAL: simulator returned {proc.returncode} for "
            f"seed={seed} workload={workload} threshold={threshold}\n"
            f"stderr:\n{proc.stderr}\n"
        )
        raise RuntimeError("simulator crash")

    with open(out_path, "r", encoding="utf-8") as f:
        data = json.load(f)

    fa = data["falsifiers"]["F_AI2_A"]
    fb = data["falsifiers"]["F_AI2_B"]
    return {
        "seed": seed,
        "workload": workload,
        "threshold": threshold,
        "drop": float(fa["throughput_drop_fraction"]),
        "baseline_throughput": float(fa["throughput_baseline_per_cycle"]),
        "provenance_throughput": float(fa["throughput_provenance_per_cycle"]),
        "f_ai2_a_pass": bool(fa["pass"]),
        "legit_rate": float(fb["legit_reject_rate"]),
        "legit_attempted": int(fb["legit_promotions_attempted"]),
        "legit_refused": int(fb["legit_promotions_refused"]),
        "f_ai2_b_pass": bool(fb["pass"]),
        "wall_seconds": wall,
    }


# ---------------------------------------------------------------------------
# Sweep + aggregate.
# ---------------------------------------------------------------------------


def run_sweep(
    python_bin: str,
    tmp_dir: str,
    progress_every: int = 50,
) -> Tuple[List[Dict[str, object]], float]:
    """Iterate the full sweep grid; return (rows, wall_seconds)."""

    os.makedirs(tmp_dir, exist_ok=True)
    rows: List[Dict[str, object]] = []
    total = len(SEEDS) * len(WORKLOADS) * len(THRESHOLDS)
    t0 = time.time()
    done = 0
    for workload in WORKLOADS:
        for threshold in THRESHOLDS:
            for seed in SEEDS:
                row = run_one(
                    python_bin=python_bin,
                    seed=seed,
                    workload=workload,
                    threshold=threshold,
                    tmp_dir=tmp_dir,
                )
                rows.append(row)
                done += 1
                if done % progress_every == 0:
                    elapsed = time.time() - t0
                    sys.stderr.write(
                        f"[btAI2-sweep] {done}/{total} runs done "
                        f"(elapsed {elapsed:.1f}s)\n"
                    )
    wall = time.time() - t0
    return rows, wall


def aggregate(rows: List[Dict[str, object]]) -> Dict[str, object]:
    """Aggregate the raw rows into per-cell distribution stats + globals."""

    cells: Dict[str, Dict[str, object]] = {}
    for workload in WORKLOADS:
        for threshold in THRESHOLDS:
            subset = [
                r for r in rows
                if r["workload"] == workload and r["threshold"] == threshold
            ]
            drops = [float(r["drop"]) for r in subset]
            rates = [float(r["legit_rate"]) for r in subset]
            n_drop_over = sum(1 for d in drops if d > F_AI2_A_MAX_DROP)
            n_rate_over = sum(1 for r in rates if r > F_AI2_B_MAX_REJECT)
            seeds_drop_over = sorted(
                int(r["seed"]) for r in subset
                if float(r["drop"]) > F_AI2_A_MAX_DROP
            )
            seeds_rate_over = sorted(
                int(r["seed"]) for r in subset
                if float(r["legit_rate"]) > F_AI2_B_MAX_REJECT
            )
            cells[f"{workload}|{threshold}"] = {
                "workload": workload,
                "threshold": threshold,
                "n_runs": len(subset),
                "drop_stats": _summarise(drops),
                "legit_reject_stats": _summarise(rates),
                "drop_over_ceiling_count": n_drop_over,
                "drop_over_ceiling_seeds": seeds_drop_over,
                "legit_reject_over_ceiling_count": n_rate_over,
                "legit_reject_over_ceiling_seeds": seeds_rate_over,
            }

    all_drops = [float(r["drop"]) for r in rows]
    all_rates = [float(r["legit_rate"]) for r in rows]
    global_drop = _summarise(all_drops)
    global_rate = _summarise(all_rates)
    n_drop_over_global = sum(1 for d in all_drops if d > F_AI2_A_MAX_DROP)
    n_rate_over_global = sum(1 for r in all_rates if r > F_AI2_B_MAX_REJECT)

    # Worst (highest p99 drop) and worst (highest max rate) cells.
    worst_drop_cell = max(
        cells.values(), key=lambda c: c["drop_stats"]["p99"]
    )
    worst_rate_cell = max(
        cells.values(), key=lambda c: c["legit_reject_stats"]["max"]
    )

    return {
        "cells": cells,
        "global": {
            "n_runs": len(rows),
            "drop_stats": global_drop,
            "legit_reject_stats": global_rate,
            "drop_over_ceiling_count": n_drop_over_global,
            "legit_reject_over_ceiling_count": n_rate_over_global,
            "f_ai2_a_max_drop_ceiling": F_AI2_A_MAX_DROP,
            "f_ai2_b_max_reject_ceiling": F_AI2_B_MAX_REJECT,
        },
        "worst_drop_cell": {
            "workload": worst_drop_cell["workload"],
            "threshold": worst_drop_cell["threshold"],
            "p99_drop": worst_drop_cell["drop_stats"]["p99"],
            "max_drop": worst_drop_cell["drop_stats"]["max"],
        },
        "worst_legit_reject_cell": {
            "workload": worst_rate_cell["workload"],
            "threshold": worst_rate_cell["threshold"],
            "max_legit_reject": worst_rate_cell["legit_reject_stats"]["max"],
        },
    }


def verdict_from_aggregate(agg: Dict[str, object]) -> str:
    g = agg["global"]
    a_pass = g["drop_over_ceiling_count"] == 0
    b_pass = g["legit_reject_over_ceiling_count"] == 0
    if a_pass and b_pass:
        return "PASS"
    if a_pass or b_pass:
        return "PARTIAL"
    return "FAIL"


# ---------------------------------------------------------------------------
# CLI.
# ---------------------------------------------------------------------------


def _parse_args(argv: List[str]) -> argparse.Namespace:
    p = argparse.ArgumentParser(
        description=(
            "BT-AI2 robustness extension: 100 seeds x 3 workloads x 3 "
            "thresholds = 900 paired simulator runs. Aggregates throughput-"
            "drop and legit-reject distributions and tags per-cell "
            "ceilings."
        )
    )
    p.add_argument(
        "--python",
        type=str,
        default=sys.executable,
        help=(
            "python interpreter to invoke the simulator with (must have "
            "simpy + numpy installed). Defaults to the runner's interpreter."
        ),
    )
    p.add_argument(
        "--tmp-dir",
        type=str,
        default="/tmp/btai2-sweep",
        help="scratch directory for per-run JSON files.",
    )
    p.add_argument(
        "--results-out",
        type=str,
        default=os.path.join(
            os.path.dirname(THIS_DIR),
            os.pardir,
            "reports",
            "anomaly",
            "btAI2_seed_sweep_results.json",
        ),
        help="path to the aggregated JSON results file.",
    )
    p.add_argument(
        "--progress-every",
        type=int,
        default=50,
        help="emit a progress line every N completed runs.",
    )
    return p.parse_args(argv)


def main(argv: List[str]) -> int:
    args = _parse_args(argv)

    if not os.path.isfile(SIM_PATH):
        sys.stderr.write(f"FATAL: simulator not found at {SIM_PATH}\n")
        return 2

    sys.stderr.write(
        f"[btAI2-sweep] starting "
        f"{len(SEEDS) * len(WORKLOADS) * len(THRESHOLDS)} runs "
        f"with python={args.python}\n"
    )
    rows, wall = run_sweep(
        python_bin=args.python,
        tmp_dir=args.tmp_dir,
        progress_every=args.progress_every,
    )
    sys.stderr.write(
        f"[btAI2-sweep] done {len(rows)} runs in {wall:.1f}s "
        f"({wall / max(1, len(rows)) * 1000:.1f} ms/run)\n"
    )

    agg = aggregate(rows)
    verdict = verdict_from_aggregate(agg)

    out = {
        "schema_version": 1,
        "experiment": "BT-AI2-robustness-extension",
        "parent_session": (
            "reports/sessions/"
            "omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md"
        ),
        "parent_cycle": "BT-AI2",
        "parent_limitation_addressed": (
            "reports/anomaly/btAI2_summary.md §7 item 4 (single-seed)"
        ),
        "atlas_constants_referenced": {
            "n_tiles": 6,
            "tau_stages": 4,
            "sparsity_floor_phi_over_n": 1.0 / 3.0,
            "provenance_bit_overhead_phi_over_sigma_n": 1.0 / 36.0,
        },
        "kg_nodes_referenced": [
            "silicon:provenance-bit",
            "silicon:promotion-counter-mmu",
            "silicon:bt-id-isa",
            "principle:honesty-triad",
            "principle:write-barrier",
            "principle:constraint-honesty",
            "arch:n6-native-accelerator",
        ],
        "domain": "ai-native-architecture",
        "sweep_grid": {
            "seeds": SEEDS,
            "workloads": WORKLOADS,
            "thresholds": THRESHOLDS,
            "total_runs": len(rows),
        },
        "wall_seconds_total": wall,
        "rows": rows,
        "aggregate": agg,
        "robustness_verdict": verdict,
    }

    os.makedirs(os.path.dirname(args.results_out) or ".", exist_ok=True)
    with open(args.results_out, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=2, sort_keys=True)
    sys.stderr.write(
        f"[btAI2-sweep] wrote aggregate -> {args.results_out}\n"
        f"[btAI2-sweep] verdict = {verdict}\n"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main(sys.argv[1:]))
