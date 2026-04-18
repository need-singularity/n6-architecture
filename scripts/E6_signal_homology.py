#!/usr/bin/env python3
# E6_signal_homology.py — signal graph homology / Betti 위상 분석
#
# 목적: atlas.signals.n6 의 cross_repo / refs / cross 그래프 구조의
#       Betti number (b0 = connected components, b1 = independent loops) 측정.
#
# 측정:
#   1. signal node + cross_repo edge → undirected graph G
#   2. G 의 connected components 수 = b0
#   3. b1 = E - V + b0 (Euler 공식 for undirected planar/general graph)
#   4. degree distribution
#
# 사용:
#   python3 scripts/E6_signal_homology.py
#
# 산출:
#   reports/E6_signal_homology_20260415.md
#   reports/E6_signal_homology_data.json
#
# 정직: networkx 미사용 (외부 의존 회피). union-find 로 b0 계산.
#       b1 = E - V + b0 는 1-skeleton 의 1차 Betti — 고차 호몰로지 미산출.

import json
import re
from pathlib import Path

NEXUS = Path.home() / "Dev" / "nexus"
N6_ROOT = Path.home() / "Dev" / "n6-architecture"
SIGNALS = NEXUS / "shared/n6/atlas.signals.n6"
OUT_MD = N6_ROOT / "reports/E6_signal_homology_20260415.md"
OUT_JSON = N6_ROOT / "reports/E6_signal_homology_data.json"


def parse_signal_graph(path: Path):
    """signal 본문에서 (id, [neighbors]) 추출.
    neighbors: cross_repo / cross / refs (SIG-* 만)."""
    if not path.exists():
        return {}, []
    text = path.read_text(encoding="utf-8")
    lines = text.split("\n")
    nodes = {}
    edges = []
    cur_id = None
    sig_re = re.compile(r"^@S\s+(SIG-[A-Z0-9-]+)")
    nbr_re = re.compile(r"SIG-[A-Z0-9-]+")
    for line in lines:
        m = sig_re.match(line)
        if m:
            cur_id = m.group(1)
            nodes[cur_id] = True
            continue
        if cur_id is None:
            continue
        s = line.strip()
        # 본 줄에 SIG-* 가 있으면 edge 후보
        if s.startswith("cross_repo:") or s.startswith("cross:") or s.startswith("refs:") or s.startswith("predicts:"):
            for nbr in nbr_re.findall(s):
                if nbr != cur_id:
                    edges.append((cur_id, nbr))
                    nodes[nbr] = True
    return nodes, edges


class UF:
    def __init__(self, items):
        self.p = {x: x for x in items}
        self.r = {x: 0 for x in items}

    def find(self, x):
        while self.p[x] != x:
            self.p[x] = self.p[self.p[x]]
            x = self.p[x]
        return x

    def union(self, a, b):
        ra, rb = self.find(a), self.find(b)
        if ra == rb:
            return False
        if self.r[ra] < self.r[rb]:
            ra, rb = rb, ra
        self.p[rb] = ra
        if self.r[ra] == self.r[rb]:
            self.r[ra] += 1
        return True


def betti(nodes, edges):
    """Compute b0, b1 for undirected graph G = (V, E)."""
    V = list(nodes.keys())
    E_unique = set()
    for a, b in edges:
        if a in nodes and b in nodes:
            key = tuple(sorted([a, b]))
            E_unique.add(key)
    E = list(E_unique)
    uf = UF(V)
    spanning_edges = 0
    for a, b in E:
        if uf.union(a, b):
            spanning_edges += 1
    roots = set(uf.find(v) for v in V)
    b0 = len(roots)
    # b1 = E - V + b0 (1차 Betti for a graph 1-skeleton)
    b1 = len(E) - len(V) + b0
    return {
        "V": len(V),
        "E_directed_count": len(edges),
        "E_unique": len(E),
        "b0": b0,
        "b1": b1,
        "components": [list(roots)[:5]],  # 샘플
    }


def degree_distribution(nodes, edges):
    deg = {n: 0 for n in nodes}
    for a, b in edges:
        if a in nodes and b in nodes:
            deg[a] += 1
            deg[b] += 1
    counts = sorted(deg.values(), reverse=True)
    return {
        "max": counts[0] if counts else 0,
        "mean": sum(counts) / max(len(counts), 1),
        "median": counts[len(counts) // 2] if counts else 0,
        "isolated_nodes": sum(1 for d in counts if d == 0),
        "top_5_nodes": sorted(deg.items(), key=lambda x: -x[1])[:5],
    }


def main():
    nodes, edges = parse_signal_graph(SIGNALS)
    b = betti(nodes, edges)
    deg = degree_distribution(nodes, edges)

    # SOC-flavoured 해석
    isolated_share = deg["isolated_nodes"] / max(b["V"], 1)
    if b["b1"] > b["V"] / 4:
        homology_class = "RICH-LOOP (b1/V > 0.25)"
    elif b["b1"] > 0:
        homology_class = "MILD-LOOP"
    else:
        homology_class = "TREE-LIKE"

    out_data = {
        "ts": "2026-04-15",
        "graph": b,
        "degree": {**deg, "isolated_share": isolated_share},
        "homology_class": homology_class,
    }
    OUT_JSON.parent.mkdir(parents=True, exist_ok=True)
    OUT_JSON.write_text(json.dumps(out_data, ensure_ascii=False, indent=2))

    md = []
    md.append("# E6 Signal Graph Homology / Betti — 2026-04-15")
    md.append("")
    md.append(f"> 입력: `{SIGNALS}`")
    md.append("> Edge 정의: signal 본문의 cross_repo/cross/refs/predicts 에 등장하는 SIG-* 참조")
    md.append("> Undirected graph 1-skeleton 에서 b0/b1 만 계산. 고차 Betti 미산출.")
    md.append("> 7대 난제 해결 0/7 유지.")
    md.append("")
    md.append("## 1. Graph 통계")
    md.append("")
    md.append(f"- 노드 수 V = {b['V']}")
    md.append(f"- 디렉티드 edge 등장 카운트 = {b['E_directed_count']}")
    md.append(f"- 유일 undirected edge 수 E = {b['E_unique']}")
    md.append("")
    md.append("## 2. Betti 수")
    md.append("")
    md.append(f"- **b0** (connected components) = {b['b0']}")
    md.append(f"- **b1** (independent loops) = E - V + b0 = {b['E_unique']} - {b['V']} + {b['b0']} = {b['b1']}")
    md.append("")
    md.append(f"- Homology class: **{homology_class}**")
    md.append("")
    md.append("## 3. Degree 분포")
    md.append("")
    md.append(f"- max degree: {deg['max']}")
    md.append(f"- mean degree: {deg['mean']:.2f}")
    md.append(f"- median degree: {deg['median']}")
    md.append(f"- isolated nodes: {deg['isolated_nodes']} ({isolated_share:.2%})")
    md.append("")
    md.append("### Top-5 hub")
    md.append("")
    md.append("| node | degree |")
    md.append("|------|-------:|")
    for nid, d in deg["top_5_nodes"]:
        md.append(f"| {nid} | {d} |")
    md.append("")
    md.append("## 4. 해석")
    md.append("")
    md.append("- **b0 > 1**: signal 그래프가 단일 cluster 가 아닌 다중 component.")
    md.append("- **b1 > 0**: 독립 루프 존재 — cross_repo 가 단순 트리 아닌 cycle 포함.")
    md.append("- **isolated_share 높음 (>40%)**: 다수 신호가 외부 참조 없는 sole entry.")
    md.append("")
    md.append("## 5. 정직 한계")
    md.append("")
    md.append("- 1-skeleton 만 — 2차 Betti b2 (3D void) 미산출.")
    md.append("- Edge 정의가 SIG-* 본문 매칭 — 의미적 cross-link 누락 가능.")
    md.append("- isolated 노드는 본 측정의 'self-contained' 신호; 약점 아님.")
    md.append("- 7대 난제 0/7 유지.")

    OUT_MD.write_text("\n".join(md))
    print(f"E6 homology: V={b['V']} E={b['E_unique']} b0={b['b0']} b1={b['b1']} class={homology_class}")
    print(f"  isolated nodes: {deg['isolated_nodes']} ({isolated_share:.2%})")
    print(f"  top hub: {deg['top_5_nodes'][0] if deg['top_5_nodes'] else 'none'}")
    print(f"  -> {OUT_MD}")
    print(f"  -> {OUT_JSON}")


if __name__ == "__main__":
    main()
