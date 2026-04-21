#!/usr/bin/env python3
"""
emergence_score_log.py — A3 emergence_lens score 일일 로그

emergence_lens.hexa 를 실행해서 score 를 capture → growth_tick.jsonl 에 append.
급증 (diff >= 0.1) 감지 시 signal 후보로 flag.

사용:
  /usr/bin/python3 scripts/emergence_score_log.py
"""
from __future__ import annotations

import json
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

LENS = "/Users/ghost/Dev/nexus/lenses/emergence_lens.hexa"
HEXA = "/Users/ghost/Dev/nexus/bin/hexa"
LOG = Path("/Users/ghost/Dev/nexus/harness/growth_tick.jsonl")


def now_iso() -> str:
    s = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    return re.sub(r"\.\d+Z$", "Z", s)


def main() -> int:
    try:
        res = subprocess.run(
            [HEXA, "run", LENS],
            capture_output=True, text=True, timeout=30, check=False,
        )
    except Exception as e:
        print(f"ERR: lens 실행 실패 {e}", file=sys.stderr)
        return 1

    out = res.stdout
    m_core = re.search(r"core_score=(\S+)", out)
    m_boost = re.search(r"signal_boost=(\S+)", out)
    m_total = re.search(r"total_score=(\S+)", out)

    entry = {
        "ts": now_iso(),
        "lens": "emergence_lens",
        "core_score": float(m_core.group(1)) if m_core else None,
        "signal_boost": float(m_boost.group(1)) if m_boost else None,
        "total_score": float(m_total.group(1)) if m_total else None,
    }

    # 직전 기록과 비교 (급증 감지)
    prev_total = None
    if LOG.exists():
        try:
            for ln in LOG.read_text(encoding="utf-8").splitlines()[-20:][::-1]:
                try:
                    r = json.loads(ln)
                    if r.get("lens") == "emergence_lens" and r.get("total_score") is not None:
                        prev_total = r["total_score"]
                        break
                except json.JSONDecodeError:
                    continue
        except Exception:
            pass

    if prev_total is not None and entry.get("total_score") is not None:
        diff = entry["total_score"] - prev_total
        entry["delta"] = round(diff, 4)
        if abs(diff) >= 0.1:
            entry["spike"] = True

    LOG.parent.mkdir(parents=True, exist_ok=True)
    with LOG.open("a", encoding="utf-8") as f:
        f.write(json.dumps(entry, ensure_ascii=False) + "\n")

    print(json.dumps(entry, ensure_ascii=False))
    return 0


if __name__ == "__main__":
    sys.exit(main())
