#!/usr/bin/env python3
"""
gen_signals_stats.py — E10 atlas.signals.n6.stats sidecar 일일 생성

atlas.signals.n6 를 스캔해서 요약 통계를 JSON + 요약 텍스트 로 출력.
- 도메인별 count
- 등급별 count
- repo_tag 별 count
- evidence 분포
- witness 분포
- 최근 7일 새 signal 수

출력:
  $NEXUS/shared/n6/atlas.signals.n6.stats           (JSON)
  $NEXUS/shared/n6/atlas.signals.n6.stats.md        (요약 md)

사용:
  /usr/bin/python3 scripts/gen_signals_stats.py
"""
from __future__ import annotations

import json
import re
import sys
from collections import Counter
from datetime import date, datetime, timedelta, timezone
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
OUT_JSON = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.stats")
OUT_MD = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6.stats.md")


def now_iso() -> str:
    s = datetime.now(timezone.utc).isoformat().replace("+00:00", "Z")
    return re.sub(r"\.\d+Z$", "Z", s)


HEAD_RE = re.compile(
    r"^@S\s+(SIG-\S+)\s*=\s*(.+?)\s*::\s*signal\s+\[([^\]]+)\]\s+\[([^\]]+)\]\s+\[([^\]]+)\]\s+\[([^\]]+)\]"
)
WIT_RE = re.compile(r"^\s*witness:\s*(\d+)")
DISC_RE = re.compile(r"^\s*discovered_at:\s*(\d{4}-\d{2}-\d{2})")


def parse(text: str) -> list[dict]:
    out: list[dict] = []
    cur: dict | None = None
    for ln in text.splitlines():
        m = HEAD_RE.match(ln)
        if m:
            if cur is not None:
                out.append(cur)
            cur = {
                "sig_id": m.group(1),
                "repo_tags": [t.strip() for t in m.group(3).split(",")],
                "domain_tags": [t.strip() for t in m.group(4).split(",")],
                "grade": m.group(5),
                "evidence": m.group(6),
                "witness": 1,
                "discovered_at": None,
            }
            continue
        if cur is None:
            continue
        wm = WIT_RE.search(ln)
        if wm:
            cur["witness"] = int(wm.group(1))
        dm = DISC_RE.search(ln)
        if dm:
            cur["discovered_at"] = dm.group(1)
    if cur is not None:
        out.append(cur)
    return out


def main() -> int:
    if not SSOT.exists():
        print(f"ERR: {SSOT} 없음", file=sys.stderr)
        return 1

    sigs = parse(SSOT.read_text(encoding="utf-8", errors="replace"))
    total = len(sigs)

    grades = Counter(s["grade"] for s in sigs)
    evidences = Counter(s["evidence"] for s in sigs)
    domains: Counter = Counter()
    repos: Counter = Counter()
    for s in sigs:
        for d in s["domain_tags"]:
            domains[d] += 1
        for r in s["repo_tags"]:
            repos[r] += 1

    # witness bucket
    wit_bucket = Counter()
    for s in sigs:
        w = s["witness"]
        if w >= 5:
            wit_bucket["5+"] += 1
        else:
            wit_bucket[str(w)] += 1

    # 최근 7일 신규
    today = date.today()
    cutoff = today - timedelta(days=7)
    recent_7 = sum(
        1 for s in sigs
        if s["discovered_at"] and s["discovered_at"] >= cutoff.isoformat()
    )

    stats = {
        "generated": now_iso(),
        "total": total,
        "grades": dict(grades.most_common()),
        "evidences": dict(evidences.most_common()),
        "domains": dict(domains.most_common()),
        "repos": dict(repos.most_common()),
        "witness": dict(wit_bucket.most_common()),
        "recent_7_days": recent_7,
    }

    OUT_JSON.write_text(json.dumps(stats, ensure_ascii=False, indent=2))

    # md
    md: list[str] = []
    md.append(f"# atlas.signals.n6.stats (generated {stats['generated']})")
    md.append("")
    md.append(f"- 총 signal: **{total}**")
    md.append(f"- 최근 7일 신규: {recent_7}")
    md.append("")
    md.append("## 등급 분포")
    for g, c in grades.most_common():
        bar = "#" * min(40, c)
        md.append(f"  [{g}] {c:4}  {bar}")
    md.append("")
    md.append("## 도메인 분포 (top 15)")
    for d, c in domains.most_common(15):
        bar = "#" * min(40, c)
        md.append(f"  [{d}] {c:4}  {bar}")
    md.append("")
    md.append("## 리포 태그")
    for r, c in repos.most_common():
        md.append(f"  [{r}] {c:4}")
    md.append("")
    md.append("## witness 분포")
    for w, c in wit_bucket.most_common():
        md.append(f"  witness={w}: {c}")
    md.append("")
    OUT_MD.write_text("\n".join(md), encoding="utf-8")

    print(f"wrote {OUT_JSON}")
    print(f"wrote {OUT_MD}")
    print(f"total={total}  recent_7={recent_7}")
    for g, c in grades.most_common():
        print(f"  [{g}] {c}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
