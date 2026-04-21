#!/usr/bin/env python3
"""
emergence_tick_jsonl.py — A17 emergence_lens → growth_tick.jsonl 시간별

emergence_score_log.py 와 유사하지만 시간별 bucket 집계 출력 (stub).
실제 구현은 emergence_score_log 가 커버. 이 스크립트는 시간별 aggregation 샘플만.

사용:
  /usr/bin/python3 scripts/emergence_tick_jsonl.py
"""
from __future__ import annotations

import json
import sys
from collections import defaultdict
from pathlib import Path

LOG = Path("/Users/ghost/Dev/nexus/harness/growth_tick.jsonl")


def main() -> int:
    if not LOG.exists():
        print("stub: growth_tick.jsonl 없음 — emergence_score_log 먼저 실행", file=sys.stderr)
        return 0
    hours: dict[str, list[float]] = defaultdict(list)
    for ln in LOG.read_text(encoding="utf-8", errors="replace").splitlines():
        try:
            r = json.loads(ln)
        except json.JSONDecodeError:
            continue
        ts = r.get("ts", "")
        ts_hour = ts[:13]  # YYYY-MM-DDTHH
        v = r.get("total_score")
        if ts_hour and v is not None:
            hours[ts_hour].append(v)
    if not hours:
        print("no data")
        return 0
    for h, vs in sorted(hours.items())[-10:]:
        avg = sum(vs) / len(vs)
        print(f"  {h}  n={len(vs)}  avg={avg:.3f}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
