#!/usr/bin/env python3
"""
signals_to_dot.py — A15 atlas.signals.n6 → GraphViz dot 시각화

atlas.signals.n6 에서 cross_repo 엣지 + domain 노드 기반 DAG 를 dot 포맷으로 출력.

사용:
  /usr/bin/python3 scripts/signals_to_dot.py > signals.dot
  dot -Tsvg signals.dot > signals.svg

노드: SIG-ID (색상 = grade)
엣지: cross_repo 필드
"""
from __future__ import annotations

import re
import sys
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")

GRADE_COLOR = {
    "M10*": "#ff1744",
    "M10": "#f57c00",
    "M9": "#fbc02d",
    "M7!": "#7cb342",
    "M7": "#388e3c",
    "M?": "#039be5",
    "MN": "#757575",
    "M5": "#bdbdbd",
}

HEAD_RE = re.compile(
    r"^@S\s+(SIG-\S+)\s*=\s*.+?\s*::\s*signal\s+\[([^\]]+)\]\s+\[([^\]]+)\]\s+\[([^\]]+)\]"
)
# groups: 1=sig_id, 2=repo_tags, 3=domain_tags, 4=grade
CROSS_RE = re.compile(r"^\s*cross_repo:\s*\[([^\]]*)\]")


def main() -> int:
    if not SSOT.exists():
        print("ERR: SSOT 없음", file=sys.stderr)
        return 1

    nodes: list[tuple[str, str, str]] = []  # sig_id, grade, domain
    edges: list[tuple[str, str]] = []

    text = SSOT.read_text(encoding="utf-8", errors="replace")
    cur: str | None = None
    cur_grade = ""
    cur_dom = ""
    for ln in text.splitlines():
        m = HEAD_RE.match(ln)
        if m:
            cur = m.group(1)
            cur_grade = m.group(4)  # grade is 4th bracket
            cur_dom = m.group(3).split(",")[0].strip()  # domain is 3rd
            nodes.append((cur, cur_grade, cur_dom))
            continue
        if cur is None:
            continue
        cm = CROSS_RE.search(ln)
        if cm:
            body = cm.group(1).strip()
            if body:
                for tgt in body.split(","):
                    tgt = tgt.strip()
                    if tgt.startswith("SIG-"):
                        edges.append((cur, tgt))

    # 출력
    out: list[str] = []
    out.append("digraph signals {")
    out.append('  rankdir=LR;')
    out.append('  node [shape=box, style="rounded,filled", fontsize=10, fontname="Helvetica"];')
    for sid, grade, dom in nodes:
        color = GRADE_COLOR.get(grade, "#eeeeee")
        label = f"{sid}\\n[{grade}]\\n({dom})"
        out.append(f'  "{sid}" [label="{label}", fillcolor="{color}"];')
    out.append("")
    for a, b in edges:
        out.append(f'  "{a}" -> "{b}";')
    out.append("}")
    print("\n".join(out))
    return 0


if __name__ == "__main__":
    sys.exit(main())
