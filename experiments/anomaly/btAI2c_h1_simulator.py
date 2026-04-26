"""
BT-AI2c H1 simulator wrapper.

Hypothesis H1 (from reports/anomaly/btAI2c_design.md sec.2): the +1-cycle
"pipeline bubble" emitted by the write-barrier when refusing a hypothesis
write is the dominant cost source. With speculative-read + rollback
amortized to ~0 (assuming low rollback rate), the bubble can be eliminated.

This wrapper subclasses HonestyBitScheduler with a SINGLE behavioural
modification: when the barrier blocks a hypothesis write, replace
`yield env.timeout(1)` with `yield env.timeout(0)`. All other logic
(refused_writes counter, poison propagation, completion marking) is
preserved. The original simulator file is NOT modified.

Caveat: rollback_rate is implicitly 0 here. The H1 result is therefore
an OPTIMISTIC bound. Real silicon with non-zero rollback rate would
have a higher drop than reported by this wrapper.
"""

from __future__ import annotations

import os
import sys
import time

# Allow importing the parent simulator from the same directory regardless of
# the cwd from which this wrapper is launched.
_HERE = os.path.dirname(os.path.abspath(__file__))
if _HERE not in sys.path:
    sys.path.insert(0, _HERE)

import numpy as np  # noqa: E402
import simpy  # noqa: E402

from btAI2_honesty_bit_scheduler import (  # noqa: E402
    HonestyBitScheduler,
    N_TILES,
    PROV_FACT,
    PROV_HYPOTHESIS,
    PROVENANCE_BIT_OVERHEAD,
    SimResult,
    TAU_STAGES,
    TensorNode,
    TileStats,
    build_workload,
    evaluate_falsifiers,
    run_simulation,
)


class H1HonestyBitScheduler(HonestyBitScheduler):
    """HonestyBitScheduler variant with the H1 zero-bubble barrier."""

    def tile_process(self, tile: int):  # SimPy generator
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

            barrier_blocked = False
            if self.barrier_on and self.provenance_on:
                if new_prov == PROV_HYPOTHESIS and t.grade < self.threshold:
                    stats.refused_writes += 1
                    stats.hyp_writes_blocked += 1
                    barrier_blocked = True
                    self.effective_prov[tid] = PROV_HYPOTHESIS
                    self.produced[tid] = int(env.now)
                    t.produced_at = self.produced[tid]
                    self.completed += 1
                    self._enqueue_consumers(tid)
                    self.last_activity = int(env.now)
                    # H1 modification: amortize the rollback bubble to 0.
                    yield env.timeout(0)
                    continue

                if new_prov == PROV_FACT and t.grade >= self.threshold:
                    stats.legit_promotions_attempted += 1
                    refused = self._maybe_false_positive(t)
                    if refused:
                        stats.legit_promotions_refused += 1

            if not barrier_blocked:
                if self.provenance_on and new_prov == PROV_FACT:
                    stats.fact_writes += 1
                self.produced[tid] = int(env.now)
                self.effective_prov[tid] = int(new_prov)
                t.produced_at = self.produced[tid]
                self.completed += 1
                self.prop_latency_sum += max(0, self.produced[tid] - ready_at)
                self.prop_latency_count += 1
                self._enqueue_consumers(tid)
                self.last_activity = int(env.now)


def run_h1_simulation(
    seed: int,
    cycles_budget: int,
    threshold: int,
    workload: str,
    provenance_on: bool,
    barrier_on: bool,
) -> SimResult:
    """Run one H1-patched simulation pass and return its SimResult.

    Identical to run_simulation in the parent module except the scheduler
    is H1HonestyBitScheduler.
    """

    t0 = time.time()
    rng = np.random.default_rng(seed)
    tensors = build_workload(rng, workload)

    env = simpy.Environment()
    sched = H1HonestyBitScheduler(
        env=env,
        tensors=tensors,
        threshold=threshold,
        provenance_on=provenance_on,
        barrier_on=barrier_on,
        rng=rng,
    )
    for tile in range(N_TILES):
        env.process(sched.tile_process(tile))

    while env.now < cycles_budget and sched.completed < len(tensors):
        next_check = min(env.now + TAU_STAGES, cycles_budget)
        env.run(until=next_check)
        if sched.completed >= len(tensors):
            break
        all_empty = all(len(q) == 0 for q in sched.queues)
        idle = (int(env.now) - sched.last_activity) >= (4 * TAU_STAGES)
        if all_empty and idle:
            break

    cycles_used = int(env.now)
    completed = sched.completed
    total = len(tensors)
    throughput = completed / cycles_used if cycles_used > 0 else 0.0

    refused = sum(s.refused_writes for s in sched.stats)
    legit_attempted = sum(s.legit_promotions_attempted for s in sched.stats)
    legit_refused = sum(s.legit_promotions_refused for s in sched.stats)
    legit_rate = (legit_refused / legit_attempted) if legit_attempted else 0.0
    avg_lat = (
        sched.prop_latency_sum / sched.prop_latency_count
        if sched.prop_latency_count
        else 0.0
    )

    area_overhead = PROVENANCE_BIT_OVERHEAD if provenance_on else 0.0
    bw_overhead = PROVENANCE_BIT_OVERHEAD if provenance_on else 0.0

    per_tile = [
        {
            "tile": i,
            "productive_cycles": s.productive_cycles,
            "stall_cycles": s.stall_cycles,
            "refused_writes": s.refused_writes,
            "legit_promotions_attempted": s.legit_promotions_attempted,
            "legit_promotions_refused": s.legit_promotions_refused,
            "hyp_writes_blocked": s.hyp_writes_blocked,
            "fact_writes": s.fact_writes,
            "propagation_events": s.propagation_events,
        }
        for i, s in enumerate(sched.stats)
    ]

    mode = "h1_provenance_on"
    if not provenance_on:
        mode = "h1_baseline"
    elif not barrier_on:
        mode = "h1_provenance_only_no_barrier"

    return SimResult(
        mode=mode,
        cycles_budget=cycles_budget,
        cycles_used=cycles_used,
        completed_tensors=completed,
        total_tensors=total,
        throughput_per_cycle=throughput,
        refused_writes=refused,
        legit_promotions_attempted=legit_attempted,
        legit_promotions_refused=legit_refused,
        legit_reject_rate=legit_rate,
        avg_propagation_latency=avg_lat,
        area_overhead_fraction=area_overhead,
        bandwidth_overhead_fraction=bw_overhead,
        seed=seed,
        workload=workload,
        threshold=threshold,
        per_tile=per_tile,
        wall_seconds=time.time() - t0,
    )


__all__ = [
    "H1HonestyBitScheduler",
    "run_h1_simulation",
    "run_simulation",
    "evaluate_falsifiers",
]
