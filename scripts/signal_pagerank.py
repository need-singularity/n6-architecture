#!/usr/bin/env python3
"""
signal_pagerank.py — A16 Signal PageRank

atlas.signals.n6 의 cross_repo 엣지 기반 PageRank 계산.
centrality 높은 signal 은 cross-resonance 허브로 간주.

사용:
  /usr/bin/python3 scripts/signal_pagerank.py --top 20
  /usr/bin/python3 scripts/signal_pagerank.py --iter 100

출력: signal_pagerank.json + stdout top N.
"""
from __future__ import annotations

import argparse
import json
import re
import sys
from collections import defaultdict
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
OUT = Path("/Users/ghost/Dev/nexus/n6/signals/signal_pagerank.json")

HEAD_RE = re.compile(r"^@S\s+(SIG-\S+)\s*=")
CROSS_RE = re.compile(r"^\s*cross_repo:\s*\[([^\]]*)\]")


def parse_edges(text: str) -> tuple[list[str], dict[str, list[str]]]:
    nodes: list[str] = []
    edges: dict[str, list[str]] = defaultdict(list)
    cur: str | None = None
    for ln in text.splitlines():
        m = HEAD_RE.match(ln)
        if m:
            cur = m.group(1)
            nodes.append(cur)
            continue
        if cur is None:
            continue
        cm = CROSS_RE.search(ln)
        if cm:
            body = cm.group(1).strip()
            if body:
                targets = [t.strip() for t in body.split(",") if t.strip().startswith("SIG-")]
                if targets:
                    edges[cur].extend(targets)
    return nodes, dict(edges)


def pagerank(nodes: list[str], edges: dict[str, list[str]], damping: float = 0.85, n_iter: int = 50):
    n = len(nodes)
    if n == 0:
        return {}
    idx = {v: i for i, v in enumerate(nodes)}
    rank = [1.0 / n] * n

    # inbound 링크
    inbound: list[list[int]] = [[] for _ in range(n)]
    out_deg: list[int] = [0] * n
    for src, tgts in edges.items():
        if src not in idx:
            continue
        si = idx[src]
        real = [t for t in tgts if t in idx]
        out_deg[si] = max(len(real), 1)
        for t in real:
            ti = idx[t]
            inbound[ti].append(si)

    base = (1.0 - damping) / n
    for _ in range(n_iter):
        new_rank = [base] * n
        for i in range(n):
            s = 0.0
            for si in inbound[i]:
                s += rank[si] / out_deg[si]
            new_rank[i] += damping * s
        rank = new_rank

    return {nodes[i]: rank[i] for i in range(n)}


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--top", type=int, default=20)
    ap.add_argument("--iter", type=int, default=50)
    args = ap.parse_args()

    if not SSOT.exists():
        print("ERR: SSOT 없음", file=sys.stderr)
        return 1

    nodes, edges = parse_edges(SSOT.read_text(encoding="utf-8", errors="replace"))
    n_edges = sum(len(v) for v in edges.values())
    print(f"nodes={len(nodes)}  edges={n_edges}")

    pr = pagerank(nodes, edges, n_iter=args.iter)

    top = sorted(pr.items(), key=lambda x: -x[1])[:args.top]
    print(f"\nTop {args.top} PageRank:")
    for sid, score in top:
        bar = "#" * min(40, int(score * 1000))
        print(f"  {sid:20} {score:.6f}  {bar}")

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(
        [{"sig_id": s, "score": v} for s, v in sorted(pr.items(), key=lambda x: -x[1])],
        ensure_ascii=False, indent=2,
    ))
    print(f"\nwrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
