#!/usr/bin/env python3
# v3 M5: OUROBOROS 2.0 — namespace-aware severity
# 기존 v1 (scripts/monotone/ouroboros_detector.py) 확장:
# - MILL-* / BT-*: CRITICAL (cycle 발견 시 exit 1)
# - L0-L7: ADVISORY (cycle 발견 시 경고, exit 0)
# - L8+: DATA_OK (cycle 정당, exit 0)
# - 기타: UNKNOWN (정책 질의 필요, exit 0)

import json
import re
import sys
from pathlib import Path
from collections import defaultdict

ATLAS = Path("/Users/ghost/Dev/nexus/n6/atlas.n6")
REPORT = Path("/Users/ghost/Dev/n6-architecture/reports/ouroboros_v2_report.json")

SEVERITY = {
    "CRITICAL": ["MILL-", "BT-"],                      # 밀레니엄 본문 — 0 tolerance
    "ADVISORY": ["L0-", "L1-", "L2-", "L3-", "L4-",   # n=6 유도층 — 경고
                 "L5-", "L6-", "L7-"],
    "DATA_OK":  ["L8-", "L9-", "L10-"],               # 관측 데이터 catalog
}


def classify(entry_id: str) -> tuple:
    """(severity, prefix) 반환"""
    for sev, prefixes in SEVERITY.items():
        for pfx in prefixes:
            if entry_id.startswith(pfx):
                return sev, pfx
    return "UNKNOWN", "?"


def parse_atlas(text: str) -> dict:
    entries = {}
    current_id = None
    current_block = []
    for line in text.split("\n"):
        m = re.match(r"^@[RX]\s+([\w\-]+)\s*=", line)
        if m:
            if current_id:
                entries[current_id] = "\n".join(current_block)
            current_id = m.group(1)
            current_block = [line]
        elif current_id:
            current_block.append(line)
    if current_id:
        entries[current_id] = "\n".join(current_block)
    return entries


def extract_internal_refs(body: str, all_ids: set) -> set:
    refs = set()
    for m in re.finditer(r"<-\s*(.+?)(?:\n\s*[#@]|$)", body, re.DOTALL):
        for tok in re.split(r"[,\s]+", m.group(1)):
            tok = tok.strip().rstrip(".,;:")
            if tok in all_ids:
                refs.add(tok)
    return refs


def tarjan_scc(graph: dict) -> list:
    index_counter = [0]
    stack, on_stack = [], set()
    indices, lowlinks = {}, {}
    result = []

    def strongconnect(start):
        call_stack = [(start, iter(graph.get(start, [])))]
        indices[start] = index_counter[0]
        lowlinks[start] = index_counter[0]
        index_counter[0] += 1
        stack.append(start)
        on_stack.add(start)
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
    print(f"[OUROBOROS v2] {len(entries)} entries", file=sys.stderr)

    graph = defaultdict(list)
    for eid, body in entries.items():
        refs = extract_internal_refs(body, all_ids)
        refs.discard(eid)
        graph[eid] = list(refs)

    sccs = tarjan_scc(graph)
    cycles = [scc for scc in sccs if len(scc) > 1]
    self_loops = [eid for eid, neighbors in graph.items() if eid in neighbors]

    # namespace 분류
    severity_counts = {"CRITICAL": 0, "ADVISORY": 0, "DATA_OK": 0, "UNKNOWN": 0}
    critical_cycles = []
    advisory_cycles = []
    data_ok_cycles = []
    unknown_cycles = []

    for scc in cycles:
        severities = set()
        for eid in scc:
            sev, _ = classify(eid)
            severities.add(sev)
            severity_counts[sev] += 1
        # cycle 의 최고 severity 로 분류
        if "CRITICAL" in severities:
            critical_cycles.append(scc)
        elif "ADVISORY" in severities:
            advisory_cycles.append(scc)
        elif "DATA_OK" in severities:
            data_ok_cycles.append(scc)
        else:
            unknown_cycles.append(scc)

    print()
    print("=" * 70)
    print("[OUROBOROS v2 namespace-aware 리포트]")
    print("=" * 70)
    print(f"  총 cycle: {len(cycles)}")
    print(f"  총 self-loop: {len(self_loops)}")
    print()
    print(f"  🔴 CRITICAL cycles (MILL-*, BT-*): {len(critical_cycles)}")
    print(f"  🟡 ADVISORY cycles (L0-L7):        {len(advisory_cycles)}")
    print(f"  🟢 DATA_OK cycles (L8+):           {len(data_ok_cycles)}")
    print(f"  ⚪ UNKNOWN cycles:                 {len(unknown_cycles)}")
    print()

    if critical_cycles:
        print("🔴 CRITICAL 발견 — R14 위반 의심:")
        for i, scc in enumerate(critical_cycles[:5], 1):
            print(f"  Cycle {i}: {scc}")
        print("  → exit 1 (CI 실패)")
        exit_code = 1
    elif advisory_cycles:
        print(f"🟡 ADVISORY {len(advisory_cycles)} cycles — L0-L7 도메인 검토 권장")
        for i, scc in enumerate(advisory_cycles[:3], 1):
            print(f"  Cycle {i}: {scc[:3]}...")
        print("  → exit 0 (경고만)")
        exit_code = 0
    else:
        print("✓ CRITICAL / ADVISORY 0건 — R14 CLEAN")
        exit_code = 0

    if data_ok_cycles:
        print(f"\n🟢 DATA_OK {len(data_ok_cycles)} cycles (L8+ 천문 observables — legitimate)")

    # JSON 저장
    report = {
        "timestamp": __import__("time").strftime("%Y-%m-%dT%H:%M:%SZ", __import__("time").gmtime()),
        "version": "v2_namespace_aware",
        "atlas_path": str(ATLAS),
        "n_entries": len(entries),
        "total_cycles": len(cycles),
        "total_self_loops": len(self_loops),
        "severity_distribution": {
            "CRITICAL": len(critical_cycles),
            "ADVISORY": len(advisory_cycles),
            "DATA_OK": len(data_ok_cycles),
            "UNKNOWN": len(unknown_cycles),
        },
        "namespace_entry_counts": severity_counts,
        "critical_cycles": [{"size": len(s), "nodes": s} for s in critical_cycles[:10]],
        "advisory_cycles": [{"size": len(s), "nodes": s} for s in advisory_cycles[:10]],
        "data_ok_cycles": [{"size": len(s), "nodes": s[:5]} for s in data_ok_cycles[:10]],
        "verdict": "R14_CLEAN" if not critical_cycles else "R14_VIOLATION",
        "severity_policy": {
            "CRITICAL_prefixes": SEVERITY["CRITICAL"],
            "ADVISORY_prefixes": SEVERITY["ADVISORY"],
            "DATA_OK_prefixes": SEVERITY["DATA_OK"],
        },
    }
    REPORT.parent.mkdir(parents=True, exist_ok=True)
    REPORT.write_text(json.dumps(report, indent=2, ensure_ascii=False), encoding="utf-8")
    print(f"\n[저장] {REPORT}")
    sys.exit(exit_code)


if __name__ == "__main__":
    main()
