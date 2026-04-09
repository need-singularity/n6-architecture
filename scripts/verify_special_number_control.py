#!/usr/bin/env python3
"""
docs/special-number-control.md 수치 재현 검증 스크립트
================================================================
문서 규칙 그대로:
- n=6 풀 (21): {1, 2, 4(tau), 5(sopfr), 6, 12(sigma), 24(J2)} 각각 x2, x3, /2, /3
- π/e/φ 풀 (36): {pi, e, phi, 2pi, pi/2, pi^2, e^2, phi^2} 각각 x2, x3, /2, /3
- 스케일 불변: v * 10^k (정수 k) 허용
- 판정: EXACT ≤1%, CLOSE ≤5%, MISS >5%
- 각 노드는 풀 내 최선 매칭 1건만
대상: ~/Dev/nexus/shared/reality_map.json
"""
import json, math, os
from collections import defaultdict

MAP = os.path.expanduser("~/Dev/nexus/shared/reality_map.json")

def expand(bases):
    pool = set()
    for b in bases:
        for m in (b, 2*b, 3*b, b/2, b/3):
            if math.isfinite(m) and m != 0:
                pool.add(m)
    return sorted(pool)

N6_BASES = [1, 2, 4, 5, 6, 12, 24]
PEP_BASES = [math.pi, math.e, (1+math.sqrt(5))/2,
             2*math.pi, math.pi/2, math.pi**2, math.e**2, ((1+math.sqrt(5))/2)**2]

def best_rel_err(x, pool):
    """스케일 불변 최선 상대오차."""
    if x == 0 or not math.isfinite(x):
        return float('inf')
    ax = abs(x)
    # 스케일 정규화: x를 [1,10)로
    logx = math.log10(ax)
    kx = math.floor(logx)
    xn = ax / (10**kx)
    best = float('inf')
    for v in pool:
        av = abs(v)
        if av == 0:
            continue
        kv = math.floor(math.log10(av))
        vn = av / (10**kv)
        # xn, vn 모두 [1,10). 동일 스케일에서 비교
        err = abs(xn - vn) / xn
        if err < best:
            best = err
        # xn 을 vn*10 또는 vn/10 과도 비교(경계 케이스)
        for alt in (vn*10, vn/10):
            err2 = abs(xn - alt) / xn
            if err2 < best:
                best = err2
    return best

def grade(err):
    if err <= 0.01: return "EXACT"
    if err <= 0.05: return "CLOSE"
    return "MISS"

def analyze(nodes, pool, label):
    counts = defaultdict(int)
    level_counts = defaultdict(lambda: defaultdict(int))
    for nd in nodes:
        m = nd.get("measured")
        if not isinstance(m, (int, float)) or not math.isfinite(m) or m == 0:
            continue
        err = best_rel_err(m, pool)
        g = grade(err)
        counts[g] += 1
        lvl = nd.get("level", "?")
        level_counts[lvl][g] += 1
        level_counts[lvl]["total"] += 1
    total = counts["EXACT"] + counts["CLOSE"] + counts["MISS"]
    return counts, level_counts, total

def main():
    with open(MAP) as f:
        data = json.load(f)
    nodes = data.get("nodes", [])
    version = data.get("version") or data.get("_meta", {}).get("version", "?")
    print(f"reality_map version: {version}")
    print(f"total nodes loaded:  {len(nodes)}")

    valid = [nd for nd in nodes
             if isinstance(nd.get("measured"), (int, float))
             and math.isfinite(nd["measured"]) and nd["measured"] != 0]
    print(f"valid measured nodes: {len(valid)}")
    print()

    n6_pool = expand(N6_BASES)
    pep_pool = expand(PEP_BASES)
    print(f"n=6 pool size:     {len(n6_pool)}  (문서: 21)")
    print(f"π/e/φ pool size:   {len(pep_pool)}  (문서: 36)")
    print()

    n6_c, n6_lvl, tot = analyze(valid, n6_pool, "n=6")
    pe_c, pe_lvl, _   = analyze(valid, pep_pool, "πeφ")

    def pct(c, t): return f"{c} ({c/t*100:.2f}%)" if t else "-"

    print("=" * 72)
    print("  전체 결과 (재현)")
    print("=" * 72)
    print(f"  {'집합':12s} | {'EXACT':>18s} | {'CLOSE':>18s} | {'MISS':>18s}")
    print("-" * 72)
    print(f"  {'n=6 (21)':12s} | {pct(n6_c['EXACT'],tot):>18s} | {pct(n6_c['CLOSE'],tot):>18s} | {pct(n6_c['MISS'],tot):>18s}")
    print(f"  {'πeφ (36)':12s} | {pct(pe_c['EXACT'],tot):>18s} | {pct(pe_c['CLOSE'],tot):>18s} | {pct(pe_c['MISS'],tot):>18s}")
    print()

    # 문서 주장치
    doc = {"EXACT": (1175, 52.67), "CLOSE": (222, 9.95), "MISS": (834, 37.38),
           "PEP_EXACT": (355, 15.91), "PEP_CLOSE": (917, 41.10), "PEP_MISS": (959, 42.99),
           "total": 2231}

    print("=" * 72)
    print("  문서 주장 수치 대비")
    print("=" * 72)
    print(f"  total       doc=2231  actual={tot}  diff={tot-2231:+d}")
    for k in ("EXACT","CLOSE","MISS"):
        dc, dp = doc[k]
        ac = n6_c[k]
        ap = ac/tot*100 if tot else 0
        match = "OK" if abs(ac-dc) <= 5 else "MISMATCH"
        print(f"  n6  {k:5s}  doc={dc:5d} ({dp:5.2f}%)  actual={ac:5d} ({ap:5.2f}%)  [{match}]")
    for k in ("EXACT","CLOSE","MISS"):
        dc, dp = doc["PEP_"+k]
        ac = pe_c[k]
        ap = ac/tot*100 if tot else 0
        match = "OK" if abs(ac-dc) <= 5 else "MISMATCH"
        print(f"  πeφ {k:5s}  doc={dc:5d} ({dp:5.2f}%)  actual={ac:5d} ({ap:5.2f}%)  [{match}]")
    print()

    # 레벨별
    print("=" * 72)
    print("  레벨별 EXACT% (n=6 vs πeφ)")
    print("=" * 72)
    all_levels = sorted(set(list(n6_lvl.keys()) + list(pe_lvl.keys())), key=lambda x: str(x))
    print(f"  {'level':10s} | {'total':>6s} | {'n6 EXACT%':>10s} | {'πeφ EXACT%':>11s} | delta")
    print("-" * 60)
    for lv in all_levels:
        t = n6_lvl[lv].get("total", 0)
        if t == 0: continue
        n6e = n6_lvl[lv]["EXACT"]/t*100
        pee = pe_lvl[lv]["EXACT"]/t*100
        print(f"  {str(lv):10s} | {t:6d} | {n6e:9.1f}% | {pee:10.1f}% | {n6e-pee:+6.1f}%p")

if __name__ == "__main__":
    main()
