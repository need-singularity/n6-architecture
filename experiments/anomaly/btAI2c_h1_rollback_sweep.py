"""BT-AI2c H1 rollback-rate sensitivity sweep.

Extends the H1 wrapper with a non-zero `rollback_rate` that charges
`int(rollback_rate * cost * downstream_count)` cycles per refused
write. Sweeps over rates at (workload=full, threshold=8, seeds=50)
and finds the break-even rate where mean drop crosses the F-AI2c-A
5% bound. Output: reports/anomaly/btAI2c_h1_rollback_results.json.
The break-even rate is computed numerically; never pre-filled.
"""

from __future__ import annotations

import json
import math
import os
import sys
import time
from typing import Dict, List, Optional, Tuple

_HERE = os.path.dirname(os.path.abspath(__file__))
if _HERE not in sys.path:
    sys.path.insert(0, _HERE)

import numpy as np  # noqa: E402
import simpy  # noqa: E402

from btAI2_honesty_bit_scheduler import (  # noqa: E402
    HonestyBitScheduler, N_TILES, PROV_HYPOTHESIS, TAU_STAGES,
    build_workload, run_simulation,
)

WORKLOAD = "full"
THRESHOLD = 8
SEEDS = 50
CYCLES_BUDGET = 10000
ROLLBACK_RATES = [0.0, 0.01, 0.025, 0.05, 0.1]
F_AI2C_A_MAX_DROP = 0.05


class H1RollbackScheduler(HonestyBitScheduler):
    """H1 wrapper that charges rollback_rate * cost * downstream_count on refuse."""

    def __init__(self, *args, rollback_rate: float = 0.0, **kwargs) -> None:
        super().__init__(*args, **kwargs)
        self.rollback_rate = float(rollback_rate)
        self._dcount: Dict[int, int] = {t.tid: 0 for t in self.tensors}
        for t in self.tensors:
            for i in t.inputs:
                if i in self._dcount:
                    self._dcount[i] += 1

    def tile_process(self, tile: int):
        env = self.env
        stats = self.stats[tile]
        while True:
            tid = self._scan_ready(tile)
            if tid is None:
                stats.stall_cycles += 1
                yield env.timeout(1)
                continue
            t = self.tensors[tid]
            ready_at = self._ready_input_time(t)
            yield env.timeout(t.cost)
            stats.productive_cycles += t.cost
            new_prov = self._propagate_prov(t)
            if self.provenance_on:
                stats.propagation_events += 1
            blocked = False
            if self.barrier_on and self.provenance_on:
                if new_prov == PROV_HYPOTHESIS and t.grade < self.threshold:
                    stats.refused_writes += 1
                    stats.hyp_writes_blocked += 1
                    blocked = True
                    self.effective_prov[tid] = PROV_HYPOTHESIS
                    self.produced[tid] = int(env.now)
                    t.produced_at = self.produced[tid]
                    self.completed += 1
                    self._enqueue_consumers(tid)
                    self.last_activity = int(env.now)
                    bubble = int(self.rollback_rate * t.cost * self._dcount.get(tid, 0))
                    yield env.timeout(bubble if bubble > 0 else 0)
                    continue
                if new_prov == 0 and t.grade >= self.threshold:
                    stats.legit_promotions_attempted += 1
                    if self._maybe_false_positive(t):
                        stats.legit_promotions_refused += 1
            if not blocked:
                if self.provenance_on and new_prov == 0:
                    stats.fact_writes += 1
                self.produced[tid] = int(env.now)
                self.effective_prov[tid] = int(new_prov)
                t.produced_at = self.produced[tid]
                self.completed += 1
                self.prop_latency_sum += max(0, self.produced[tid] - ready_at)
                self.prop_latency_count += 1
                self._enqueue_consumers(tid)
                self.last_activity = int(env.now)


def run_one(seed: int, rollback_rate: float) -> Tuple[float, float]:
    base = run_simulation(seed=seed, cycles_budget=CYCLES_BUDGET, threshold=THRESHOLD,
                         workload=WORKLOAD, provenance_on=False, barrier_on=False)
    rng = np.random.default_rng(seed)
    tensors = build_workload(rng, WORKLOAD)
    env = simpy.Environment()
    sched = H1RollbackScheduler(env=env, tensors=tensors, threshold=THRESHOLD,
                                provenance_on=True, barrier_on=True, rng=rng,
                                rollback_rate=rollback_rate)
    for tile in range(N_TILES):
        env.process(sched.tile_process(tile))
    while env.now < CYCLES_BUDGET and sched.completed < len(tensors):
        env.run(until=min(env.now + TAU_STAGES, CYCLES_BUDGET))
        if sched.completed >= len(tensors):
            break
        all_empty = all(len(q) == 0 for q in sched.queues)
        idle = (int(env.now) - sched.last_activity) >= (4 * TAU_STAGES)
        if all_empty and idle:
            break
    used = int(env.now)
    tp = sched.completed / used if used > 0 else 0.0
    return base.throughput_per_cycle, tp


def aggregate(drops: List[float]) -> Dict[str, float]:
    arr = np.asarray(drops, dtype=float)
    if not arr.size:
        return {"n": 0, "mean": 0.0, "std": 0.0, "p50": 0.0, "p90": 0.0,
                "p99": 0.0, "min": 0.0, "max": 0.0, "seeds_breaching_5pct": 0}
    return {"n": int(arr.size), "mean": float(arr.mean()),
            "std": float(arr.std(ddof=0)),
            "p50": float(np.percentile(arr, 50)),
            "p90": float(np.percentile(arr, 90)),
            "p99": float(np.percentile(arr, 99)),
            "min": float(arr.min()), "max": float(arr.max()),
            "seeds_breaching_5pct": int(np.sum(arr > F_AI2C_A_MAX_DROP))}


def find_break_even(per_rate: List[Dict[str, object]]) -> Optional[float]:
    """Linear interpolation of the rate at which mean drop crosses 5%."""
    s = sorted(per_rate, key=lambda r: r["rollback_rate"])
    prev = None
    for entry in s:
        rate = float(entry["rollback_rate"])
        mean = float(entry["summary"]["mean"])
        if mean >= F_AI2C_A_MAX_DROP:
            if prev is None:
                return rate
            pr = float(prev["rollback_rate"])
            pm = float(prev["summary"]["mean"])
            if mean == pm:
                return rate
            return pr + (F_AI2C_A_MAX_DROP - pm) / (mean - pm) * (rate - pr)
        prev = entry
    return None


def main() -> int:
    t0 = time.time()
    per_rate: List[Dict[str, object]] = []
    per_seed: List[Dict[str, object]] = []
    for rate in ROLLBACK_RATES:
        drops: List[float] = []
        for seed in range(SEEDS):
            base_tp, h1_tp = run_one(seed, rate)
            drop = (base_tp - h1_tp) / base_tp if base_tp > 0 and math.isfinite(base_tp) else float("inf")
            drops.append(drop)
            per_seed.append({"rollback_rate": rate, "seed": seed,
                             "baseline_tp": base_tp, "h1_tp": h1_tp, "drop": drop})
        per_rate.append({"rollback_rate": rate,
                         "summary": aggregate([d for d in drops if math.isfinite(d)])})
    break_even = find_break_even(per_rate)
    out = {
        "schema": "btAI2c_h1_rollback_sweep.v1",
        "workload": WORKLOAD, "threshold": THRESHOLD, "cycles_budget": CYCLES_BUDGET,
        "seeds": SEEDS, "rollback_rates": ROLLBACK_RATES,
        "f_ai2c_a_max_drop": F_AI2C_A_MAX_DROP,
        "per_rate": per_rate, "break_even_rollback_rate": break_even,
        "wall_seconds_total": time.time() - t0, "per_seed": per_seed,
    }
    out_path = os.path.abspath(os.path.join(_HERE, os.pardir, os.pardir,
        "reports", "anomaly", "btAI2c_h1_rollback_results.json"))
    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    with open(out_path, "w", encoding="utf-8") as f:
        json.dump(out, f, indent=2)
    for entry in per_rate:
        s = entry["summary"]
        print(f"rate={entry['rollback_rate']:.4f} mean={s['mean']:.4f} "
              f"p90={s['p90']:.4f} max={s['max']:.4f} "
              f"breach={s['seeds_breaching_5pct']}/{s['n']}")
    if break_even is None:
        print("break_even_rollback_rate=None (no crossing within sweep range)")
    else:
        print(f"break_even_rollback_rate={break_even:.6f}")
    print(f"wrote {out_path}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
