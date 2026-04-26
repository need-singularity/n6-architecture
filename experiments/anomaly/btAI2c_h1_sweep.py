"""
BT-AI2c H1 sweep runner.

Runs a 100-seed sweep at (workload=full, threshold=8) comparing:

  - baseline: provenance OFF, barrier OFF (reference for drop ratio)
  - original: provenance ON, barrier ON (parent BT-AI2 simulator,
              keeps the +1-cycle bubble)
  - H1-patched: provenance ON, barrier ON (H1 wrapper with
              the bubble amortized to 0)

Drop_X = (baseline.throughput - X.throughput) / baseline.throughput.

Aggregates: mean, std, p50, p90, p99, max, seeds_breach (count of seeds
where drop > 5%) for both Drop_orig and Drop_H1.

Outputs JSON to --results-out (default
reports/anomaly/btAI2c_h1_results.json).

Verdict on F-AI2c-A:
  - PASS    if H1 mean drop <= 5% AND seeds_breach == 0
  - PARTIAL if H1 mean drop <= 5% but some seeds breach
  - FAIL    otherwise

Usage:
  python btAI2c_h1_sweep.py \
      --results-out reports/anomaly/btAI2c_h1_results.json \
      --seeds 100 --cycles-budget 10000

The script imports the parent simulator and the H1 wrapper in-process
(no subprocess), so all 100 seeds run sequentially in a single python
process.
"""

from __future__ import annotations

import argparse
import json
import math
import os
import sys
import time
from typing import Dict, List

_HERE = os.path.dirname(os.path.abspath(__file__))
if _HERE not in sys.path:
    sys.path.insert(0, _HERE)

import numpy as np  # noqa: E402

from btAI2c_h1_simulator import (  # noqa: E402
    run_h1_simulation,
    run_simulation,
)

WORKLOAD = "full"
THRESHOLD = 8
F_AI2C_A_MAX_DROP = 0.05  # 5% throughput-drop ceiling


def _percentile(values: List[float], q: float) -> float:
    if not values:
        return 0.0
    return float(np.percentile(np.asarray(values, dtype=float), q))


def _aggregate(drops: List[float]) -> Dict[str, float]:
    arr = np.asarray(drops, dtype=float)
    breach = int(np.sum(arr > F_AI2C_A_MAX_DROP))
    return {
        "n": int(arr.size),
        "mean": float(arr.mean()) if arr.size else 0.0,
        "std": float(arr.std(ddof=0)) if arr.size else 0.0,
        "p50": _percentile(drops, 50),
        "p90": _percentile(drops, 90),
        "p99": _percentile(drops, 99),
        "min": float(arr.min()) if arr.size else 0.0,
        "max": float(arr.max()) if arr.size else 0.0,
        "seeds_breaching_5pct": breach,
    }


def _verdict(h1_summary: Dict[str, float]) -> str:
    mean_ok = h1_summary["mean"] <= F_AI2C_A_MAX_DROP
    breach = h1_summary["seeds_breaching_5pct"]
    if mean_ok and breach == 0:
        return "PASS"
    if mean_ok and breach > 0:
        return "PARTIAL"
    return "FAIL"


def run_sweep(seeds: int, cycles_budget: int) -> Dict[str, object]:
    drops_orig: List[float] = []
    drops_h1: List[float] = []
    per_seed: List[Dict[str, object]] = []

    t0 = time.time()
    for seed in range(seeds):
        base = run_simulation(
            seed=seed,
            cycles_budget=cycles_budget,
            threshold=THRESHOLD,
            workload=WORKLOAD,
            provenance_on=False,
            barrier_on=False,
        )
        orig = run_simulation(
            seed=seed,
            cycles_budget=cycles_budget,
            threshold=THRESHOLD,
            workload=WORKLOAD,
            provenance_on=True,
            barrier_on=True,
        )
        h1 = run_h1_simulation(
            seed=seed,
            cycles_budget=cycles_budget,
            threshold=THRESHOLD,
            workload=WORKLOAD,
            provenance_on=True,
            barrier_on=True,
        )

        base_tp = base.throughput_per_cycle
        if base_tp <= 0 or not math.isfinite(base_tp):
            d_orig = float("inf")
            d_h1 = float("inf")
        else:
            d_orig = (base_tp - orig.throughput_per_cycle) / base_tp
            d_h1 = (base_tp - h1.throughput_per_cycle) / base_tp

        drops_orig.append(d_orig)
        drops_h1.append(d_h1)

        per_seed.append(
            {
                "seed": seed,
                "baseline_tp": base_tp,
                "orig_tp": orig.throughput_per_cycle,
                "h1_tp": h1.throughput_per_cycle,
                "drop_orig": d_orig,
                "drop_h1": d_h1,
                "orig_refused": orig.refused_writes,
                "h1_refused": h1.refused_writes,
                "base_cycles_used": base.cycles_used,
                "orig_cycles_used": orig.cycles_used,
                "h1_cycles_used": h1.cycles_used,
            }
        )

    summary_orig = _aggregate([d for d in drops_orig if math.isfinite(d)])
    summary_h1 = _aggregate([d for d in drops_h1 if math.isfinite(d)])
    verdict = _verdict(summary_h1)

    return {
        "schema": "btAI2c_h1_sweep.v1",
        "workload": WORKLOAD,
        "threshold": THRESHOLD,
        "cycles_budget": cycles_budget,
        "seeds": seeds,
        "f_ai2c_a_max_drop": F_AI2C_A_MAX_DROP,
        "summary_h1": summary_h1,
        "summary_orig": summary_orig,
        "verdict_f_ai2c_a": verdict,
        "wall_seconds_total": time.time() - t0,
        "per_seed": per_seed,
    }


def main() -> int:
    p = argparse.ArgumentParser(description="BT-AI2c H1 sweep")
    p.add_argument(
        "--results-out",
        type=str,
        default="reports/anomaly/btAI2c_h1_results.json",
        help="Path to write the results JSON.",
    )
    p.add_argument(
        "--seeds",
        type=int,
        default=100,
        help="Number of seeds (default 100).",
    )
    p.add_argument(
        "--cycles-budget",
        type=int,
        default=10000,
        help="Cycles budget per simulation (default 10000).",
    )
    args = p.parse_args()

    result = run_sweep(seeds=args.seeds, cycles_budget=args.cycles_budget)

    out_path = args.results_out
    if not os.path.isabs(out_path):
        out_path = os.path.abspath(out_path)
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(result, f, indent=2)

    sh1 = result["summary_h1"]
    so = result["summary_orig"]
    print(
        "verdict_f_ai2c_a={v} h1.mean={m:.4f} h1.breach={b} "
        "orig.mean={om:.4f} orig.breach={ob}".format(
            v=result["verdict_f_ai2c_a"],
            m=sh1["mean"],
            b=sh1["seeds_breaching_5pct"],
            om=so["mean"],
            ob=so["seeds_breaching_5pct"],
        )
    )
    print("wrote {p}".format(p=out_path))
    return 0


if __name__ == "__main__":
    sys.exit(main())
