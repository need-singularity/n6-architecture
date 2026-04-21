#!/usr/bin/env python3
"""
cluster_signals.py — A9 signal cluster 군집화

simhash + 간이 k-means 로 atlas.signals.n6 를 클러스터링.
출력: cluster summary → signal_clusters.json + stdout dendrogram lite.

사용:
  /usr/bin/python3 scripts/cluster_signals.py --k 10
  /usr/bin/python3 scripts/cluster_signals.py --k 15 --print-top 3

주의:
- 목적은 빠른 similarity 군집 파악 (진짜 dendrogram 아님)
- simhash hamming distance 기반 nearest-centroid k-means
"""
from __future__ import annotations

import argparse
import hashlib
import json
import random
import re
import sys
from collections import defaultdict
from pathlib import Path

SSOT = Path("/Users/ghost/Dev/nexus/n6/atlas.signals.n6")
OUT = Path("/Users/ghost/Dev/nexus/n6/signals/signal_clusters.json")

HEAD_RE = re.compile(
    r"^@S\s+(SIG-\S+)\s*=\s*(.+?)\s*::\s*signal\s+\[([^\]]+)\]\s+\[([^\]]+)\]\s+\[([^\]]+)\]"
)


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


def hamming(a: int, b: int) -> int:
    return bin(a ^ b).count("1")


def parse(text: str) -> list[dict]:
    out = []
    for ln in text.splitlines():
        m = HEAD_RE.match(ln)
        if m:
            out.append({
                "sig_id": m.group(1),
                "statement": m.group(2),
                "repo_tags": m.group(3),
                "domain_tags": m.group(4),
            })
    return out


def kmeans(sigs: list[dict], k: int, max_iter: int = 20, seed: int = 42) -> list[list[int]]:
    random.seed(seed)
    n = len(sigs)
    if n == 0:
        return []
    hashes = [simhash(s["statement"]) for s in sigs]
    centroids = random.sample(hashes, min(k, n))
    if len(centroids) < k:
        centroids = centroids * ((k // len(centroids)) + 1)
        centroids = centroids[:k]

    assign = [0] * n
    for _ in range(max_iter):
        changed = False
        for i, h in enumerate(hashes):
            best = min(range(k), key=lambda j: hamming(h, centroids[j]))
            if assign[i] != best:
                assign[i] = best
                changed = True

        # recompute centroids = bit majority within cluster
        for j in range(k):
            members = [hashes[i] for i in range(n) if assign[i] == j]
            if not members:
                continue
            bits = [0] * 64
            for h in members:
                for b in range(64):
                    if (h >> b) & 1:
                        bits[b] += 1
                    else:
                        bits[b] -= 1
            cn = 0
            for b in range(64):
                if bits[b] > 0:
                    cn |= 1 << b
            centroids[j] = cn

        if not changed:
            break

    clusters: list[list[int]] = [[] for _ in range(k)]
    for i, a in enumerate(assign):
        clusters[a].append(i)
    return clusters


def main() -> int:
    ap = argparse.ArgumentParser()
    ap.add_argument("--k", type=int, default=10)
    ap.add_argument("--print-top", type=int, default=3)
    args = ap.parse_args()

    if not SSOT.exists():
        print("ERR: SSOT 없음", file=sys.stderr)
        return 1

    sigs = parse(SSOT.read_text(encoding="utf-8", errors="replace"))
    print(f"입력 signal: {len(sigs)}")
    clusters = kmeans(sigs, args.k)

    out_data = []
    for j, member_idx in enumerate(clusters):
        # 대표 domain_tag 집계
        dom_count: dict[str, int] = defaultdict(int)
        for i in member_idx:
            for d in sigs[i]["domain_tags"].split(","):
                dom_count[d.strip()] += 1
        top_doms = sorted(dom_count.items(), key=lambda x: -x[1])[:3]
        out_data.append({
            "cluster": j,
            "size": len(member_idx),
            "top_domains": top_doms,
            "samples": [sigs[i]["sig_id"] for i in member_idx[:args.print_top]],
        })

    out_data.sort(key=lambda x: -x["size"])

    OUT.parent.mkdir(parents=True, exist_ok=True)
    OUT.write_text(json.dumps(out_data, ensure_ascii=False, indent=2))

    print(f"\n클러스터 요약 (k={args.k}):")
    for c in out_data:
        bar = "#" * min(40, c["size"])
        dom_str = ",".join(f"{d}:{n}" for d, n in c["top_domains"][:2])
        print(f"  [C{c['cluster']:2}] n={c['size']:3} {bar}  doms=[{dom_str}]")
        for s in c["samples"]:
            print(f"       - {s}")

    print(f"\nwrote {OUT}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
