#!/usr/bin/env python3
"""
blowup_energy_log.py — A4 blowup_emergence energy 기록

blowup_field 실행 시 seed 별 energy 값을 discovery_log.jsonl 에서 추출 →
같은 seed 반복 시 energy 안정화 (std < 0.05) 면 convergence → [M9] 후보.

사용:
  /usr/bin/python3 scripts/blowup_energy_log.py
"""
from __future__ import annotations

import json
import math
import re
import sys
from collections import defaultdict
from datetime import datetime, timezone
from pathlib import Path

LOG = Path("/Users/ghost/Dev/nexus/discovery_log.jsonl")
OUT = Path("/Users/ghost/Dev/nexus/n6/signals/blowup_energy_stability.jsonl")


def now_iso() -> str:
    s = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    return re.sub(r"\.\d+Z$", "Z", s)


def stdev(vals: list[float]) -> float:
    if len(vals) < 2:
        return 0.0
    m = sum(vals) / len(vals)
    return math.sqrt(sum((v - m) ** 2 for v in vals) / (len(vals) - 1))


def main() -> int:
    if not LOG.exists():
        print("ERR: discovery_log 없음", file=sys.stderr)
        return 1

    by_seed: dict[str, list[float]] = defaultdict(list)
    read_n = 0
    for ln in LOG.read_text(encoding="utf-8", errors="replace").splitlines():
        ln = ln.strip()
        if not ln:
            continue
        try:
            r = json.loads(ln)
        except json.JSONDecodeError:
            continue
        read_n += 1
        seed = r.get("pipeline_seed", "")
        val = r.get("value")
        if seed and isinstance(val, (int, float)):
            by_seed[seed].append(float(val))

    print(f"discovery_log rows parsed: {read_n}")
    print(f"seeds: {len(by_seed)}")

    ts = now_iso()
    stable_seeds: list[dict] = []
    for seed, vals in by_seed.items():
        if len(vals) < 3:
            continue
        m = sum(vals) / len(vals)
        s = stdev(vals)
        cv = s / (abs(m) + 1e-9)
        stable = s < 0.05 or cv < 0.02
        entry = {
            "ts": ts,
            "seed": seed,
            "n": len(vals),
            "mean": round(m, 4),
            "std": round(s, 4),
            "cv": round(cv, 4),
            "stable": stable,
        }
        if stable:
            stable_seeds.append(entry)

    print(f"\n안정 수렴 seed 후보 ([M9] 후보): {len(stable_seeds)}")
    for s in stable_seeds[:10]:
        print(f"  {s['seed']}  n={s['n']}  mean={s['mean']}  std={s['std']}")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    with OUT.open("a", encoding="utf-8") as f:
        for s in stable_seeds:
            f.write(json.dumps(s, ensure_ascii=False) + "\n")
    print(f"\nwrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
