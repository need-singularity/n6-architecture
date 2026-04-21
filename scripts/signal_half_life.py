#!/usr/bin/env python3
"""
signal_half_life.py — A6 signal half-life 데몬

atlas.signals.n6 signal 의 last activity (witness 증분 or cross 매칭) 이후 경과일 계산
→ 임계 일수 초과 시 [M7] → [M5] 강등, 더 오래되면 archive.

사용:
  /usr/bin/python3 scripts/signal_half_life.py                 # dry-run 리포트
  /usr/bin/python3 scripts/signal_half_life.py --threshold-days 14
  /usr/bin/python3 scripts/signal_half_life.py --commit        # 실제 반영 (M7→M5 만)

주의: 강등은 [M7] → [M5] 한정 (현 spec 상 M5 중간 등급).
      atlas.signals.n6 의 [M7] 만 대상 + discovered_at 기준 (last_touch 필드 미존재).
"""
from __future__ import annotations

import argparse
import re
import sys
from datetime import date, datetime
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
BACKUP = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.bak.pre-halflife")


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--threshold-days", type=int, default=14)
    ap.add_argument("--commit", action="store_true")
    args = ap.parse_args()

    if not SSOT.exists():
        print("ERR: SSOT 없음", file=sys.stderr)
        return 1

    text = SSOT.read_text(encoding="utf-8", errors="replace")
    lines = text.splitlines()

    today = date.today()
    HEAD_RE = re.compile(
        r"^(@S\s+SIG-\S+\s*=\s*.+?\s*::\s*signal\s+\[[^\]]+\]\s+\[[^\]]+\]\s+)\[M7\](\s+\[[^\]]+\])"
    )
    DISC_RE = re.compile(r"^\s*discovered_at:\s*(\d{4}-\d{2}-\d{2})")

    degrade_ids: list[tuple[int, str, int]] = []  # (line_idx, sig_id, age_days)
    i = 0
    while i < len(lines):
        m = HEAD_RE.match(lines[i])
        if m:
            sig_id_match = re.search(r"SIG-\S+", lines[i])
            sig_id = sig_id_match.group(0) if sig_id_match else "?"
            # 후속 블록에서 discovered_at 찾기
            j = i + 1
            disc = None
            while j < len(lines) and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                dm = DISC_RE.search(lines[j])
                if dm:
                    disc = dm.group(1)
                    break
                j += 1
            if disc:
                try:
                    d = datetime.strptime(disc, "%Y-%m-%d").date()
                    age = (today - d).days
                    if age >= args.threshold_days:
                        degrade_ids.append((i, sig_id, age))
                except ValueError:
                    pass
        i += 1

    print(f"[M7] 강등 후보: {len(degrade_ids)} (threshold={args.threshold_days} days)")
    for _, sid, age in degrade_ids[:10]:
        print(f"  - {sid} age={age}d")
    if len(degrade_ids) > 10:
        print(f"  ... (+ {len(degrade_ids) - 10} more)")

    if not args.commit:
        print("\n[DRY-RUN] --commit 지정 시 [M7] → [M5] 실제 반영")
        return 0

    if not degrade_ids:
        return 0

    if not BACKUP.exists():
        BACKUP.write_bytes(SSOT.read_bytes())
        print(f"백업: {BACKUP}")

    for idx, sid, age in degrade_ids:
        lines[idx] = lines[idx].replace("[M7]", "[M5]", 1)

    SSOT.write_text("\n".join(lines) + ("\n" if text.endswith("\n") else ""), encoding="utf-8")
    print(f"반영: {len(degrade_ids)} signals [M7] → [M5]")
    return 0


if __name__ == "__main__":
    sys.exit(main())
