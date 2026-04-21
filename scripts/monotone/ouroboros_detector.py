#!/usr/bin/env python3
# HONEST-PX-2 OUROBOROS — atlas.n6 자기참조 순환 검출기
# R14 규칙: 자기참조 검증 금지 (외부 데이터/이론만 허용)
# 방법: @R 엔트리의 <- 라인에서 atlas-internal @R ID 참조를 찾아 유향그래프 구축
#       → SCC (Tarjan) 탐지로 cycle 발견
# 출력: reports/ouroboros_report.json + 콘솔 요약 / exit 1 on cycles

import json
import re
import sys
from pathlib import Path
from collections import defaultdict

ATLAS = Path("/Users/ghost/Dev/nexus/n6/atlas.n6")
REPORT = Path("/Users/ghost/Dev/n6-architecture/reports/ouroboros_report.json")


def parse_atlas(text: str) -> dict:
    """
    @R 또는 @X 엔트리 파싱. 각 엔트리는 3 라인:
      @R ID = expr :: axis [grade]
        "description ..."
        <- dep1, dep2, ...
    deps 는 한 줄 또는 다중 줄 가능. 가장 단순: @R 직후 다음 @R 이전까지 블록.
    """
    entries = {}
    current_id = None
    current_block = []
    lines = text.split("\n")
    for line in lines:
        m = re.match(r"^@[RX]\s+([\w\-]+)\s*=", line)
        if m:
            # flush previous
            if current_id:
                entries[current_id] = "\n".join(current_block)
            current_id = m.group(1)
            current_block = [line]
        elif current_id:
            current_block.append(line)
    if current_id:
        entries[current_id] = "\n".join(current_block)
    return entries


def extract_internal_refs(entry_text: str, all_ids: set) -> set:
    """
    entry_text 의 <- 라인들에서 atlas-internal @R ID 참조 추출.
    atlas_ids 중 하나에 매칭되는 token 만 internal ref 로 인정.
    """
    refs = set()
    # <- 라인 (단일 또는 다중)
    for m in re.finditer(r"<-\s*(.+?)(?:\n\s*[#@]|$)", entry_text, re.DOTALL):
        line = m.group(1)
        # comma/space tokenize
        tokens = re.split(r"[,\s]+", line)
        for tok in tokens:
            tok = tok.strip().rstrip(".,;:")
            if tok in all_ids:
                refs.add(tok)
    return refs


def tarjan_scc(graph: dict) -> list:
    """Tarjan SCC algorithm. graph: {node: [neighbors]}. 반환: list of SCC"""
    index_counter = [0]
    stack = []
    on_stack = set()
    indices = {}
    lowlinks = {}
    result = []

    def strongconnect(v, sys_recursion=[1000]):
        # iterative to avoid recursion limit
        call_stack = [(v, iter(graph.get(v, [])))]
        indices[v] = index_counter[0]
        lowlinks[v] = index_counter[0]
        index_counter[0] += 1
        stack.append(v)
        on_stack.add(v)

        while call_stack:
            v, it = call_stack[-1]
            try:
                w = next(it)
                if w not in indices:
                    indices[w] = index_counter[0]
                    lowlinks[w] = index_counter[0]
                    index_counter[0] += 1
                    stack.append(w)
                    on_stack.add(w)
                    call_stack.append((w, iter(graph.get(w, []))))
                elif w in on_stack:
                    lowlinks[v] = min(lowlinks[v], indices[w])
            except StopIteration:
                if lowlinks[v] == indices[v]:
                    scc = []
                    while True:
                        w = stack.pop()
                        on_stack.discard(w)
                        scc.append(w)
                        if w == v:
                            break
                    result.append(scc)
                call_stack.pop()
                if call_stack:
                    w = v
                    v = call_stack[-1][0]
                    lowlinks[v] = min(lowlinks[v], lowlinks[w])

    for v in graph:
        if v not in indices:
            strongconnect(v)
    return result


def main():
    text = ATLAS.read_text(encoding="utf-8")
    entries = parse_atlas(text)
    all_ids = set(entries.keys())
    print(f"[OUROBOROS] atlas.n6 파싱: {len(entries)} entries", file=sys.stderr)

    # 의존 그래프
    graph = defaultdict(list)
    n_refs_total = 0
    n_entries_with_refs = 0
    for eid, body in entries.items():
        refs = extract_internal_refs(body, all_ids)
        refs.discard(eid)  # 자기 자신 ref 제외 (trivial self-loop)
        if refs:
            n_entries_with_refs += 1
            n_refs_total += len(refs)
            graph[eid] = list(refs)
        else:
            graph[eid] = []

    print(f"[OUROBOROS] internal refs: {n_refs_total} (in {n_entries_with_refs} entries)", file=sys.stderr)

    # SCC
    sccs = tarjan_scc(graph)
    cycles = [scc for scc in sccs if len(scc) > 1]  # cycle = SCC size ≥ 2
    self_loops = [eid for eid, neighbors in graph.items() if eid in neighbors]

    # 리포트
    print()
    print("=" * 60)
    if cycles:
        print(f"[OUROBOROS 발견] {len(cycles)} 순환참조 cluster")
        print("=" * 60)
        for i, scc in enumerate(cycles, 1):
            print(f"  Cycle #{i} (size {len(scc)}):")
            for eid in scc:
                neighbors = [n for n in graph[eid] if n in scc]
                print(f"    {eid} → {neighbors}")
    else:
        print(f"[MONOTONE OK] 순환참조 0 건 (N={len(entries)} entries 전수)")
    print("=" * 60)

    if self_loops:
        print(f"[self-loop 주의] {len(self_loops)} 건 — 자기 자신 직접 참조")
        for eid in self_loops[:10]:
            print(f"  {eid} → {eid}")

    # 대용량 SCC는 별도 출력 안 함 (cycles 가 1000 넘으면 경고)
    if len(cycles) > 100:
        print(f"\n[주의] cycles 대량 ({len(cycles)}) — 파싱 오류 가능성, 검토 필요")

    # 저장
    report = {
        "timestamp": __import__("time").strftime("%Y-%m-%dT%H:%M:%SZ", __import__("time").gmtime()),
        "atlas_path": str(ATLAS),
        "n_entries": len(entries),
        "n_internal_refs": n_refs_total,
        "n_entries_with_refs": n_entries_with_refs,
        "cycles": [{"size": len(scc), "nodes": scc} for scc in cycles[:50]],
        "self_loops": self_loops[:50],
        "n_cycles": len(cycles),
        "n_self_loops": len(self_loops),
        "verdict": "OUROBOROS_DETECTED" if (cycles or self_loops) else "R14_CLEAN",
    }
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"\n[저장] {REPORT}")

    sys.exit(1 if (cycles or self_loops) else 0)


if __name__ == "__main__":
    main()
