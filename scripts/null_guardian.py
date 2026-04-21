#!/usr/bin/env python3
"""
null_guardian.py — E9 실험 시작 전 [MN] 매칭 경고

atlas.signals.n6 의 [MN] (NULL 확정) signal 과 stdin/argv 로 들어온
가설 statement 를 simhash 매칭 → 유사 NULL 존재 시 경고.

사용:
  # stdin
  echo "ANU 64B seed vs full-urandom" | /usr/bin/python3 scripts/null_guardian.py --stdin
  # argv
  /usr/bin/python3 scripts/null_guardian.py --text "ANU QRNG atlas correlation"
  # retry_forbidden_until 체크
  /usr/bin/python3 scripts/null_guardian.py --text "..." --strict

리턴 코드:
  0 = 안전 (유사 NULL 없음)
  1 = 경고 (유사 NULL 있음, retry_forbidden 기한 내)
  2 = 입력 에러
"""
from __future__ import annotations

import argparse
import hashlib
import re
import sys
from datetime import date, datetime
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")


def simhash(text: str, bits: int = 64) -> int:
    tokens = re.findall(r"[A-Za-z가-힣0-9]+", text.lower())
    if not tokens:
        return 0
    v = [0] * bits
    for t in tokens:
        h = int(hashlib.md5(t.encode("utf-8")).hexdigest(), 16)
        for i in range(bits):
            v[i] += 1 if (h >> i) & 1 else -1
    out = 0
    for i in range(bits):
        if v[i] > 0:
            out |= 1 << i
    return out


def sim_score(a: int, b: int, bits: int = 64) -> float:
    return 1.0 - (bin(a ^ b).count("1") / bits)


def parse_null_signals(text: str) -> list[dict]:
    """@S ... [MN] ... 라인 + retry_forbidden_until 파싱."""
    out: list[dict] = []
    cur: dict | None = None
    for ln in text.splitlines():
        m = re.match(
            r"^@S\s+(SIG-\S+)\s*=\s*(.+?)\s*::\s*signal\s+.*\[MN\]",
            ln,
        )
        if m:
            if cur is not None:
                out.append(cur)
            cur = {
                "sig_id": m.group(1),
                "statement": m.group(2),
                "retry_forbidden_until": None,
                "null_reason": "",
            }
            continue
        if cur is None:
            continue
        rm = re.search(r'^\s*retry_forbidden_until:\s*"?(\d{4}-\d{2}-\d{2})', ln)
        if rm:
            cur["retry_forbidden_until"] = rm.group(1)
            continue
        nm = re.search(r'^\s*null_reason:\s*"([^"]+)"', ln)
        if nm:
            cur["null_reason"] = nm.group(1)
            continue
        # 새 @S 시작 감지는 위에서
        if ln.startswith("@S "):
            out.append(cur)
            cur = None
    if cur is not None:
        out.append(cur)
    return out


def main() -> int:
    ap = argparse.ArgumentParser()
    g = ap.add_mutually_exclusive_group(required=True)
    g.add_argument("--stdin", action="store_true")
    g.add_argument("--text", type=str)
    ap.add_argument("--threshold", type=float, default=0.75)
    ap.add_argument("--strict", action="store_true",
                    help="retry_forbidden_until 이후로 지난 NULL 은 무시")
    ap.add_argument("--top", type=int, default=5)
    args = ap.parse_args()

    if args.stdin:
        hypothesis = sys.stdin.read().strip()
    else:
        hypothesis = args.text.strip()

    if not hypothesis:
        print("null_guardian: 빈 입력", file=sys.stderr)
        return 2

    if not SSOT.exists():
        print(f"null_guardian: SSOT 없음 {SSOT}", file=sys.stderr)
        return 2

    txt = SSOT.read_text(encoding="utf-8", errors="replace")
    nulls = parse_null_signals(txt)

    if not nulls:
        print("null_guardian: [MN] signal 없음 — 안전")
        return 0

    h_hash = simhash(hypothesis)
    today = date.today().isoformat()

    hits: list[tuple[str, str, float, str | None, str]] = []
    for n in nulls:
        if args.strict and n.get("retry_forbidden_until"):
            if n["retry_forbidden_until"] < today:
                continue  # 만료
        sim = sim_score(h_hash, simhash(n["statement"]))
        if sim >= args.threshold:
            hits.append((n["sig_id"], n["statement"], sim,
                         n.get("retry_forbidden_until"), n["null_reason"]))

    hits.sort(key=lambda x: -x[2])

    if not hits:
        print(f"null_guardian: 유사 NULL 없음 (threshold={args.threshold}) — 안전")
        return 0

    print(f"!!! NULL 매칭 경고: {len(hits)}건 (threshold={args.threshold})")
    for sig_id, stmt, sim, rfu, reason in hits[:args.top]:
        expire_note = f"expires {rfu}" if rfu else "no-expire"
        print(f"  [{sig_id}] sim={sim:.3f} ({expire_note})")
        print(f"    '{stmt[:120]}'")
        if reason:
            print(f"    reason: {reason[:120]}")
    return 1


if __name__ == "__main__":
    sys.exit(main())
