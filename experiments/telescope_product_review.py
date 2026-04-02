#!/usr/bin/env python3
"""telescope_product_review.py — 기존 완성제품 전면 재검토 (망원경 9종 풀스캔)

73개 완성제품의 수치 파라미터를 10개 도메인별로 구성하고,
9종 렌즈를 돌려 숨겨진 패턴, 이상점, 갭을 찾는다.

Features per product (12차원):
  0: alien_index (1-6)
  1: n6_exact_count (해당 제품의 n=6 EXACT 파라미터 수)
  2: n6_exact_ratio (0-1, EXACT 비율)
  3: dse_combinations (log10 스케일)
  4: dse_score (0-1, Pareto 최적 점수)
  5: product_completeness (1-6, 개별 완성도)
  6: bt_count (관련 BT 수)
  7: evolution_stages (0-4)
  8: cross_dse (0 or 1)
  9: domain_id (1-10)
  10: has_design_doc (0 or 1)
  11: has_discovery (0 or 1)
"""

import sys, os, json
import numpy as np

sys.path.insert(0, os.path.join(os.path.dirname(__file__), '..', '.shared'))

# ── Product Data ──────────────────────────────────────────────
# Each row: [alien, exact_cnt, exact_ratio, log10_dse, dse_score, completeness,
#            bt_count, evo_stages, cross_dse, domain_id, has_doc, has_discovery]

DOMAIN_NAMES = {
    1: "핵융합", 2: "칩/반도체", 3: "AI/ML", 4: "에너지",
    5: "환경보호", 6: "물리·수학", 7: "물질합성", 8: "로봇",
    9: "소프트웨어", 10: "디스플레이"
}

PRODUCTS = []
PRODUCT_NAMES = []

def add(name, alien, exact_cnt, exact_ratio, dse, dse_score, completeness,
        bt, evo, cross, domain, doc=1, disc=0):
    log_dse = np.log10(max(dse, 1))
    PRODUCTS.append([alien, exact_cnt, exact_ratio, log_dse, dse_score,
                     completeness, bt, evo, cross, domain, doc, disc])
    PRODUCT_NAMES.append(name)

# ── Domain 1: 핵융합 (12 products) ──
add("KSTAR-N6 토카막",        6, 30, 0.90, 2400, 0.85, 5, 7, 4, 1, 1)
add("300초 정상상태",          6, 25, 0.85, 2400, 0.80, 5, 7, 0, 0, 1)
add("궁극의 핵융합 발전소",   6, 40, 0.95, 2400, 0.98, 6, 7, 0, 1, 1)
add("Mk.I 첫 점화",           6, 15, 0.80, 100,  0.70, 4, 2, 4, 0, 1)
add("Mk.II 도시 전력",        6, 12, 0.75, 100,  0.65, 3, 2, 4, 0, 1)
add("Mk.III 국가 전력",       6, 10, 0.70, 100,  0.60, 2, 2, 4, 0, 1)
add("Mk.IV 대륙 전력",        6, 8,  0.65, 100,  0.55, 1, 2, 4, 0, 1)
add("외계인급 발견",           6, 15, 0.90, 1,    0.90, 6, 5, 0, 0, 1, 1, 1)
add("KSTAR 숨겨진 패턴",      6, 21, 0.51, 1,    0.51, 4, 3, 0, 0, 1, 1, 1)
add("2030 검증 예측",          6, 10, 0.80, 1,    0.80, 4, 2, 0, 0, 1)
add("Cross-DSE 5도메인",       6, 35, 0.98, 1000000, 0.98, 6, 5, 0, 1, 1)
add("논문 #308",               6, 5,  0.74, 1,    0.74, 3, 2, 0, 0, 1)

# ── Domain 2: 칩/반도체 (16 products) ──
add("HEXA-CORE",               6, 103, 1.00, 3000, 0.95, 6, 9, 0, 0, 2)
add("HEXA-1 통합 SoC",        6, 80,  0.95, 3000, 0.92, 6, 9, 0, 0, 2)
add("HEXA-MATERIAL",           6, 20,  0.85, 500,  0.85, 5, 3, 0, 0, 2)
add("HEXA-PROCESS",            6, 18,  0.82, 500,  0.82, 5, 3, 0, 0, 2)
add("HEXA-SYSTEM",             6, 15,  0.80, 500,  0.80, 5, 3, 0, 0, 2)
add("HEXA-PIM",                6, 12,  0.78, 200,  0.78, 5, 2, 0, 0, 2)
add("HEXA-3D",                 6, 10,  0.75, 200,  0.75, 4, 2, 0, 0, 2)
add("HEXA-PHOTON",             6, 8,   0.70, 200,  0.70, 3, 2, 0, 0, 2)
add("HEXA-WAFER",              6, 6,   0.65, 200,  0.65, 2, 1, 0, 0, 2)
add("HEXA-SUPER",              6, 5,   0.60, 200,  0.60, 1, 1, 0, 0, 2)
add("HEXA-OMEGA",              6, 4,   0.55, 200,  0.55, 1, 1, 0, 0, 2)
add("HEXA-EDGE",               6, 10,  0.75, 200,  0.70, 4, 2, 0, 0, 2)
add("ANIMA-SOC 의식칩",        6, 50,  0.92, 500,  0.90, 6, 5, 0, 0, 2)
add("HEXA-TOPO 성능",          6, 20,  0.85, 300,  0.85, 5, 3, 0, 0, 2)
add("HEXA-TOPO 의식",          6, 18,  0.82, 300,  0.82, 5, 3, 0, 0, 2)
add("HEXA-ASIC SkyWater",      6, 12,  0.70, 100,  0.70, 4, 2, 0, 0, 2)

# ── Domain 3: AI/ML (4 products) ──
add("17 Techniques",           5, 17,  0.90, 1,    0.90, 6, 18, 0, 0, 3)
add("N6 Inevitability Engine", 5, 12,  0.85, 1,    0.85, 5, 10, 0, 0, 3)
add("AI Energy Savings Guide", 5, 20,  0.88, 1,    0.88, 6, 15, 0, 0, 3)
add("Chip Architecture Guide", 5, 120, 0.95, 1,    0.95, 6, 9,  0, 0, 3)

# ── Domain 4: 에너지 (12 products) ──
add("HEXA-CELL",               6, 15,  0.82, 1908, 0.85, 5, 5, 0, 1, 4)
add("HEXA-ELECTRODE",          6, 12,  0.80, 1908, 0.82, 5, 5, 0, 0, 4)
add("HEXA-CORE 배터리",        6, 10,  0.78, 1908, 0.80, 5, 5, 0, 0, 4)
add("HEXA-CHIP BMS",           6, 8,   0.72, 500,  0.72, 4, 3, 0, 0, 4)
add("HEXA-PACK+GRID",          6, 12,  0.80, 1000, 0.82, 5, 5, 0, 0, 4)
add("HEXA-SOLID",              6, 6,   0.65, 300,  0.65, 3, 3, 0, 0, 4)
add("HEXA-NUCLEAR",            6, 4,   0.55, 100,  0.55, 2, 2, 0, 0, 4)
add("OMEGA-E",                 6, 3,   0.50, 100,  0.50, 1, 1, 0, 0, 4)
add("배터리 설계서",           6, 30,  0.90, 1908, 0.92, 6, 8, 0, 1, 4)
add("배터리 DSE 결과",         6, 25,  0.88, 1908, 0.90, 6, 8, 0, 1, 4)
add("궁극의 태양전지",         6, 15,  0.80, 1584, 0.80, 4, 4, 0, 0, 4)
add("궁극의 에너지 통합",      6, 35,  0.92, 10225,0.95, 6, 13, 0, 1, 4)

# ── Domain 5: 환경보호 (21 products) ──
for name, comp in [("HEXA-SENSE",5), ("HEXA-MONITOR",5), ("HEXA-CAPTURE",5),
                   ("HEXA-PURIFY",5), ("HEXA-RESTORE",4), ("HEXA-CYCLE",3),
                   ("HEXA-ECOSYSTEM",2), ("OMEGA-ENV",1)]:
    add(f"환경-{name}", 5, 10, 0.70, 3600000, 0.70, comp, 0, 4, 0, 5)
add("환경 외계인급 발견", 5, 15, 0.80, 1, 0.80, 6, 0, 0, 0, 5, 1, 1)
add("환경 진화 Mk.I~IV",  5, 8,  0.65, 1, 0.65, 4, 0, 4, 0, 5)
for name, comp in [("HEXA-SORBENT",5), ("HEXA-PROCESS",5), ("HEXA-REACTOR",5),
                   ("HEXA-CHIP",4), ("HEXA-PLANT",4), ("HEXA-TRANSMUTE",3),
                   ("HEXA-UNIVERSAL",2), ("OMEGA-CC",1)]:
    add(f"CCUS-{name}", 5, 12, 0.75, 3600000, 0.75, comp, 0, 4, 0, 5)
add("CCUS 설계서",     5, 25, 0.85, 3600000, 0.88, 6, 0, 0, 0, 5)
add("CCUS DSE 결과",   5, 20, 0.82, 3600000, 0.85, 6, 0, 0, 0, 5)
add("CCUS 진화 Mk.I~IV", 5, 8, 0.65, 1, 0.65, 4, 0, 4, 0, 5)

# ── Domain 6: 물리·수학 (3 products) ──
add("궁극의 초전도체",    4, 20, 0.80, 28800, 0.85, 5, 5, 0, 0, 6)
add("궁극의 순수수학",    4, 30, 0.94, 38024, 0.94, 5, 4, 0, 0, 6)
add("궁극의 우주론/입자", 4, 5,  0.40, 1,     0.40, 1, 3, 0, 0, 6)

# ── Domain 7: 물질합성 (2 products) ──
add("궁극의 물질합성",    3, 15, 0.70, 3600, 0.70, 3, 4, 0, 0, 7)
add("BT-85~88",           3, 20, 0.85, 1,    0.85, 4, 4, 0, 0, 7)

# ── Domain 8: 로봇 (1 product) ──
add("궁극의 로봇",        2, 8,  0.60, 22500, 0.60, 3, 0, 0, 0, 8)

# ── Domain 9: 소프트웨어 (1 product) ──
add("궁극의 프로그래밍언어", 2, 15, 0.96, 5016, 0.96, 5, 0, 0, 0, 9)

# ── Domain 10: 디스플레이 (1 product) ──
add("궁극의 디스플레이",  1, 0,  0.00, 1, 0.00, 1, 5, 0, 0, 10)

# ── Build matrix ──
data = np.array(PRODUCTS, dtype=np.float64)
labels = ["alien_idx", "n6_exact_cnt", "n6_exact_ratio", "log10_dse",
          "dse_score", "completeness", "bt_count", "evo_stages",
          "cross_dse", "domain_id", "has_doc", "has_discovery"]

print(f"=== 완성제품 전면 재검토 — 망원경 9종 풀스캔 ===")
print(f"총 {len(PRODUCTS)}개 제품, {len(labels)}차원\n")

# ── Normalize for lens input ──
data_norm = data.copy()
for j in range(data_norm.shape[1]):
    col = data_norm[:, j]
    mn, mx = col.min(), col.max()
    if mx > mn:
        data_norm[:, j] = (col - mn) / (mx - mn)

# ── Run 9 lenses ──
results = {}
lens_names = []

try:
    from consciousness_lens import ConsciousnessLens
    print("[1/9] 의식 렌즈 스캔 중...")
    cl = ConsciousnessLens(cells=64, steps=200)
    results['consciousness'] = cl.scan(data_norm)
    lens_names.append('consciousness')
    print(f"  → {results['consciousness']}")
except Exception as e:
    print(f"  ✗ 의식 렌즈 오류: {e}")

try:
    from gravity_lens import GravityLens
    print("[2/9] 중력 렌즈 스캔 중...")
    gl = GravityLens(steps=60)
    results['gravity'] = gl.scan(data_norm)
    lens_names.append('gravity')
    print(f"  → {results['gravity']}")
except Exception as e:
    print(f"  ✗ 중력 렌즈 오류: {e}")

try:
    from topology_lens import TopologyLens
    print("[3/9] 위상 렌즈 스캔 중...")
    tl = TopologyLens()
    results['topology'] = tl.scan(data_norm)
    lens_names.append('topology')
    print(f"  → {results['topology']}")
except Exception as e:
    print(f"  ✗ 위상 렌즈 오류: {e}")

try:
    from thermo_lens import ThermoLens
    print("[4/9] 열역학 렌즈 스캔 중...")
    th = ThermoLens()
    results['thermo'] = th.scan(data_norm)
    lens_names.append('thermo')
    print(f"  → {results['thermo']}")
except Exception as e:
    print(f"  ✗ 열역학 렌즈 오류: {e}")

try:
    from wave_lens import WaveLens
    print("[5/9] 파동 렌즈 스캔 중...")
    wl = WaveLens()
    results['wave'] = wl.scan(data_norm)
    lens_names.append('wave')
    print(f"  → {results['wave']}")
except Exception as e:
    print(f"  ✗ 파동 렌즈 오류: {e}")

try:
    from evolution_lens import EvolutionLens
    print("[6/9] 진화 렌즈 스캔 중...")
    el = EvolutionLens()
    results['evolution'] = el.scan(data_norm)
    lens_names.append('evolution')
    print(f"  → {results['evolution']}")
except Exception as e:
    print(f"  ✗ 진화 렌즈 오류: {e}")

try:
    from info_lens import InfoLens
    print("[7/9] 정보 렌즈 스캔 중...")
    il = InfoLens()
    results['info'] = il.scan(data_norm)
    lens_names.append('info')
    print(f"  → {results['info']}")
except Exception as e:
    print(f"  ✗ 정보 렌즈 오류: {e}")

try:
    from quantum_lens import QuantumLens
    print("[8/9] 양자 렌즈 스캔 중...")
    ql = QuantumLens()
    results['quantum'] = ql.scan(data_norm)
    lens_names.append('quantum')
    print(f"  → {results['quantum']}")
except Exception as e:
    print(f"  ✗ 양자 렌즈 오류: {e}")

try:
    from em_lens import EMLens
    print("[9/9] 전자기 렌즈 스캔 중...")
    eml = EMLens()
    results['em'] = eml.scan(data_norm)
    lens_names.append('em')
    print(f"  → {results['em']}")
except Exception as e:
    print(f"  ✗ 전자기 렌즈 오류: {e}")

# ── Cross-lens anomaly consensus ──
print(f"\n{'='*60}")
print(f"교차 검증 — {len(lens_names)}종 렌즈 합의 분석")
print(f"{'='*60}\n")

anomaly_votes = {}  # product_idx -> list of lens names
for lname, r in results.items():
    anom_list = []
    if hasattr(r, 'anomalies') and r.anomalies:
        for item in r.anomalies:
            if isinstance(item, tuple) and len(item) >= 1:
                idx = int(item[0])
                anom_list.append(idx)
            elif isinstance(item, (int, np.integer)):
                anom_list.append(int(item))
    for idx in set(anom_list):
        if idx < len(PRODUCT_NAMES):
            anomaly_votes.setdefault(idx, []).append(lname)

# Sort by vote count
sorted_anomalies = sorted(anomaly_votes.items(), key=lambda x: -len(x[1]))

print("이상점 (anomaly) — 렌즈 합의 순:")
print(f"{'제품':<30s} {'렌즈수':>5s} {'합의 렌즈'}")
print("-" * 70)
for idx, lenses in sorted_anomalies[:20]:
    name = PRODUCT_NAMES[idx]
    domain = DOMAIN_NAMES[int(data[idx, 9])]
    consensus = "확정" if len(lenses) >= 3 else ("후보" if len(lenses) >= 2 else "가설")
    print(f"[{domain}] {name:<24s} {len(lenses):>3d}종  {', '.join(lenses)}  ({consensus})")

# ── Per-domain summary ──
print(f"\n{'='*60}")
print(f"도메인별 종합 분석")
print(f"{'='*60}\n")

for did in range(1, 11):
    mask = data[:, 9] == did
    if not mask.any():
        continue
    domain_data = data[mask]
    domain_names_list = [PRODUCT_NAMES[i] for i in range(len(PRODUCTS)) if data[i, 9] == did]
    n = domain_data.shape[0]
    avg_alien = domain_data[:, 0].mean()
    avg_exact = domain_data[:, 2].mean()
    avg_complete = domain_data[:, 5].mean()
    max_bt = domain_data[:, 6].max()
    has_cross = domain_data[:, 8].max()
    has_evo = domain_data[:, 7].max()
    has_disc = domain_data[:, 11].max()

    # Count anomalies in this domain
    domain_indices = [i for i in range(len(PRODUCTS)) if data[i, 9] == did]
    domain_anomalies = [(idx, anomaly_votes[idx]) for idx in domain_indices if idx in anomaly_votes]

    print(f"### {DOMAIN_NAMES[did]} (🛸×{int(avg_alien)}) — {n}개 제품")
    print(f"  평균 EXACT 비율: {avg_exact:.0%}")
    print(f"  평균 완성도: {avg_complete:.1f}/6")
    print(f"  BT: {int(max_bt)}, Cross-DSE: {'✅' if has_cross else '❌'}, "
          f"진화: {'✅' if has_evo else '❌'}, 발견: {'✅' if has_disc else '❌'}")
    if domain_anomalies:
        print(f"  ⚠️ 이상점 {len(domain_anomalies)}개:")
        for idx, lenses in domain_anomalies:
            print(f"    - {PRODUCT_NAMES[idx]}: {len(lenses)}종 렌즈 감지")

    # Gap analysis
    gaps = []
    if avg_exact < 0.70:
        gaps.append(f"n6 EXACT 비율 낮음 ({avg_exact:.0%})")
    if not has_cross:
        gaps.append("Cross-DSE 미실행")
    if not has_evo:
        gaps.append("진화 로드맵 없음")
    if not has_disc:
        gaps.append("외계인급 발견 미등록")
    if avg_complete < 3.0:
        gaps.append(f"평균 완성도 낮음 ({avg_complete:.1f})")
    if max_bt == 0:
        gaps.append("BT 없음")

    if gaps:
        print(f"  📋 갭: {' / '.join(gaps)}")
    else:
        print(f"  ✅ 갭 없음 — 완전 도메인")
    print()

# ── Lens summaries ──
print(f"\n{'='*60}")
print(f"렌즈별 상세 요약")
print(f"{'='*60}\n")

for lname in lens_names:
    r = results[lname]
    print(f"--- {lname} ---")
    if hasattr(r, 'summary') and r.summary:
        print(f"  {r.summary[:200]}")
    if hasattr(r, 'discoveries') and r.discoveries:
        print(f"  발견 {len(r.discoveries)}개:")
        for d in r.discoveries[:5]:
            if isinstance(d, dict):
                interp = d.get('interpretation', d.get('description', str(d)))
                print(f"    - {str(interp)[:100]}")
            else:
                print(f"    - {str(d)[:100]}")
    print()

# ── Recommendations ──
print(f"\n{'='*60}")
print(f"🎯 재검토 권고사항")
print(f"{'='*60}\n")

# Find weakest domains
domain_scores = []
for did in range(1, 11):
    mask = data[:, 9] == did
    if mask.any():
        d = data[mask]
        score = (d[:, 0].mean() * 0.3 +   # alien index
                 d[:, 2].mean() * 0.25 +   # exact ratio
                 d[:, 5].mean()/6 * 0.2 +  # completeness
                 d[:, 8].max() * 0.1 +     # cross-dse
                 (1 if d[:, 7].max() > 0 else 0) * 0.1 +  # evolution
                 d[:, 11].max() * 0.05)    # discovery
        domain_scores.append((did, score))

domain_scores.sort(key=lambda x: x[1])

print("우선 개선 대상 (점수 낮은 순):")
for did, score in domain_scores:
    bar = "█" * int(score * 20) + "░" * (20 - int(score * 20))
    print(f"  {DOMAIN_NAMES[did]:<12s} {bar} {score:.2f}")

print("\n구체적 권고:")
for did, score in domain_scores[:5]:
    mask = data[:, 9] == did
    d = data[mask]
    dname = DOMAIN_NAMES[did]
    print(f"\n  [{dname}] (점수: {score:.2f})")
    if d[:, 2].mean() < 0.70:
        print(f"    → n6 EXACT 비율 강화 필요 (현재 {d[:, 2].mean():.0%})")
    if d[:, 8].max() == 0:
        print(f"    → Cross-DSE 실행 권고")
    if d[:, 7].max() == 0:
        print(f"    → 진화 로드맵(Mk.I~IV) 작성 권고")
    if d[:, 11].max() == 0:
        print(f"    → 외계인급 발견 탐색 필요")
    if d[:, 0].mean() < 3:
        print(f"    → 🛸 지수 향상 필요 (현재 {d[:, 0].mean():.0f})")

print(f"\n=== 스캔 완료 ===")
