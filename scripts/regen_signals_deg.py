#!/usr/bin/env python3
"""
regen_signals_deg.py — A8

atlas.signals.n6 sidecar (atlas.signals.n6.deg) 자동 생성.

포맷: TSV  sig_id\tdegree  (atlas.n6.deg 와 호환)
degree 계산:
  base:   1
  + witness 값
  + cross_repo 수 × 2
  + CROSS tag 존재 시 +3

출력: /Users/ghost/Dev/nexus/n6/atlas.signals.n6.deg

사용법:
  /usr/bin/python3 scripts/regen_signals_deg.py
  /usr/bin/python3 scripts/regen_signals_deg.py --dry-run
"""
from __future__ import annotations

import argparse
import re
import sys
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
OUT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.deg")
BACKUP = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.deg.bak.pre-regen")


def parse_signals(text: str) -> list[dict]:
    signals: list[dict] = []
    lines = text.split("\n")
    i = 0
    n_lines = len(lines)
    while i < n_lines:
        m = re.match(
            r"^@S\s+(\S+)\s*=\s*(.+?)\s*::\s*signal\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]\s*\[([^\]]+)\]",
            lines[i],
        )
        if m:
            repo_tags = [t.strip() for t in m.group(3).split(",")]
            sig = {
                "sig_id": m.group(1),
                "repo_tags": repo_tags,
                "domain_tags": [t.strip() for t in m.group(4).split(",")],
                "grade": m.group(5).strip(),
                "evidence": m.group(6).strip(),
                "witness": 1,
                "cross_repo": [],
            }
            j = i + 1
            while j < n_lines and not lines[j].startswith("@S ") and not lines[j].startswith("# ─"):
                wm = re.search(r"^\s*witness:\s*(\d+)", lines[j])
                if wm:
                    sig["witness"] = int(wm.group(1))
                cm = re.match(r"^\s*cross_repo:\s*\[(.*)\]\s*$", lines[j])
                if cm:
                    inner = cm.group(1).strip()
                    if inner:
                        sig["cross_repo"] = [t.strip() for t in inner.split(",") if t.strip()]
                j += 1
            signals.append(sig)
            i = j
        else:
            i += 1
    return signals


def compute_degree(sig: dict) -> int:
    base = 1
    witness = sig.get("witness", 1)
    cross_n = len(sig.get("cross_repo", []))
    has_cross_tag = "CROSS" in sig.get("repo_tags", [])
    return base + witness + cross_n * 2 + (3 if has_cross_tag else 0)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--dry-run", action="store_true")
    args = ap.parse_args()

    if not SSOT.exists():
        print(f"ERR: SSOT 없음: {SSOT}", file=sys.stderr)
        sys.exit(1)

    text = SSOT.read_text(encoding="utf-8", errors="replace")
    signals = parse_signals(text)
    print(f"parsed signals: {len(signals)}")

    rows: list[tuple[str, int]] = []
    for s in signals:
        rows.append((s["sig_id"], compute_degree(s)))

    rows.sort(key=lambda x: (-x[1], x[0]))

    if args.dry_run:
        print("\n상위 20 미리보기:")
        for sid, d in rows[:20]:
            print(f"  {sid}\t{d}")
        total = sum(d for _, d in rows)
        print(f"\n총 엔트리: {len(rows)}  총 degree: {total}  평균: {total/max(len(rows),1):.2f}")
        print("[DRY RUN] --dry-run 제거 시 실제 쓰기")
        return

    if OUT.exists() and not BACKUP.exists():
        BACKUP.write_bytes(OUT.read_bytes())
        print(f"백업: {BACKUP}")

    with OUT.open("w", encoding="utf-8") as f:
        for sid, d in rows:
            f.write(f"{sid}\t{d}\n")

    print(f"생성: {OUT}")
    print(f"  엔트리: {len(rows)}")
    print(f"  파일 크기: {OUT.stat().st_size:,} bytes")
    top = rows[:5]
    print("  상위 5:")
    for sid, d in top:
        print(f"    {sid}\t{d}")


if __name__ == "__main__":
    main()
