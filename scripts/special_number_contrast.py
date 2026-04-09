#!/usr/bin/env python3
"""
특수 수 대조 실험 — n=6 파생 집합 vs π/e/φ/√2 파생 집합
==========================================================
reality_map.json의 각 노드(measured, n6_value)에 대해
n=6 기반 상수 집합과 대조군(π, e, φ_golden, √2) 기반 집합의
매칭률을 비교한다.

판정: |measured - candidate| / |measured| <= 0.01 → EXACT (±1%)
      measured == 0인 경우: |candidate| < 0.01 → EXACT
"""

import json
import math
import sys
from collections import defaultdict

# ─────────────────────────────────────────────────────────
# 1. 상수 집합 정의
# ─────────────────────────────────────────────────────────

# n=6 산술 상수
n = 6
sigma = 12          # σ(6) = 1+2+3+6
tau = 4             # τ(6) = 약수 개수
phi_euler = 2       # φ(6) = 오일러 토텐트
sopfr = 5           # 소인수 합 2+3
J2 = 24             # 조르단 토텐트
mu = 1              # 뫼비우스

# 황금비
phi_g = (1 + math.sqrt(5)) / 2

def build_n6_set():
    """n=6 파생 비율/상수 집합 (정수 + 비율 + 거듭제곱)"""
    vals = set()
    bases = [n, sigma, tau, phi_euler, sopfr, J2, mu]

    # 기본값과 거듭제곱
    for b in bases:
        for exp in range(-3, 5):
            if exp == 0:
                vals.add(1)
            elif b != 0:
                vals.add(b ** exp)

    # 두 상수 간 비율/곱/합/차
    for i, a in enumerate(bases):
        for j, b in enumerate(bases):
            if a != 0 and b != 0:
                vals.add(a / b)
                vals.add(a * b)
                vals.add(a + b)
                if a != b:
                    vals.add(a - b)
                vals.add(a ** 2 / b)
                vals.add(a / (b ** 2))

    # 자주 등장하는 n=6 파생 표현들
    extras = [
        1/6, 1/3, 1/2, 2/3, 5/6,           # 단위 분수 계열
        sigma/n,                              # = 2
        phi_euler/tau,                        # = 0.5
        tau/sigma,                            # = 1/3
        sigma*tau,                            # = 48
        n*tau,                                # = 24
        n*sigma,                              # = 72
        n**2,                                 # = 36
        sigma**2,                             # = 144
        tau**2,                               # = 16
        math.factorial(n),                    # = 720
        math.factorial(n-1),                  # = 120
        2**n,                                 # = 64
        10**n,                                # = 1000000
        n*(n+1)/2,                            # = 21 (삼각수)
        math.comb(sigma, n),                  # C(12,6) = 924
        math.sqrt(n),                         # √6
        math.sqrt(sigma),                     # √12
        math.pi / sigma,                      # π/12
        math.pi / n,                          # π/6
        math.pi / tau,                        # π/4
        2 * math.pi,                          # 2π (원 = σ·π/n?)
        math.e ** (1/n),                      # e^(1/6)
        math.log(n),                          # ln(6)
        math.log(sigma),                      # ln(12)
        n * math.pi,                          # 6π
        # 미세구조 상수 관련
        1/137.036,                            # α ≈ 1/137
        137.036,
        # 큰 수
        6**6,                                 # 46656
        12**3,                                # 1728
        24**2,                                # 576
    ]
    for v in extras:
        vals.add(v)

    # NaN/inf 제거
    vals = {v for v in vals if math.isfinite(v) and v != 0}
    return sorted(vals)


def build_pi_set():
    """π 파생 집합"""
    pi = math.pi
    vals = set()
    # 거듭제곱
    for exp in range(-4, 7):
        vals.add(pi ** exp)
    # 유명 π 관련 상수
    extras = [
        1/pi, 2/pi, pi/2, pi/3, pi/4, pi/6,
        pi**2/6,            # ζ(2)
        pi**2/12,           # 대체 급수
        pi**4/90,           # ζ(4)
        4/pi,               # 라이프니츠
        2*pi,               # 원주
        pi/180,             # deg→rad
        180/pi,             # rad→deg
        math.sqrt(pi),      # √π
        math.sqrt(2*pi),    # √(2π)
        1/math.sqrt(2*pi),  # 정규분포 계수
        pi**(1/3),          # π^(1/3)
        22/7,               # 근사
        355/113,            # 밀률
        3*pi,               # 3π
        4*pi,               # 4π
        6*pi,               # 6π (n=6 아닌 독립 등장?)
        pi**2,              # π²
        2*pi**2,
        pi**3,
        8*pi,
        16*pi**2,
        4*pi/3,             # 구 부피 계수
        # 정수와 조합
        pi + 1, pi - 1, pi + 2, pi - 3,
        2*pi + 1,
    ]
    for v in extras:
        vals.add(v)

    # 정수 배수 (1~30)
    for k in range(1, 31):
        vals.add(k * pi)
        vals.add(pi / k)
        vals.add(k * pi**2)

    vals = {v for v in vals if math.isfinite(v) and v != 0}
    return sorted(vals)


def build_e_set():
    """e (자연로그 밑) 파생 집합"""
    e = math.e
    vals = set()
    for exp in range(-5, 8):
        vals.add(e ** exp)

    extras = [
        1/e, 2/e, e/2, e/3, e/4, e/6,
        e - 1,              # ≈ 1.718
        e + 1,
        e**2,
        e**(-2),
        math.sqrt(e),
        1/math.sqrt(e),
        e**(1/3),
        e*math.pi,          # eπ
        math.pi**e,         # π^e
        e**(math.pi),       # e^π
        2*e,
        3*e,
        e**e,               # e^e ≈ 15.15
        # 오일러-마스케로니 상수
        0.5772156649,       # γ
        1/0.5772156649,
        math.log(2),        # ln2
        math.log(3),        # ln3
        math.log(10),       # ln10
        1/math.log(2),      # 1/ln2
        1/math.log(10),     # log_10(e)
    ]
    for v in extras:
        vals.add(v)

    for k in range(1, 31):
        vals.add(k * e)
        vals.add(e / k)
        vals.add(e ** (k/10))

    vals = {v for v in vals if math.isfinite(v) and v != 0}
    return sorted(vals)


def build_phi_set():
    """φ (황금비) 파생 집합"""
    phi = phi_g
    vals = set()
    for exp in range(-6, 10):
        vals.add(phi ** exp)

    extras = [
        1/phi,              # φ - 1 ≈ 0.618
        phi**2,             # φ + 1 ≈ 2.618
        phi - 1,            # 1/φ
        2*phi,
        3*phi,
        phi/2,
        phi/3,
        phi/5,
        5*phi,
        math.sqrt(phi),
        1/math.sqrt(phi),
        math.sqrt(5),       # 2φ - 1 과 연관
        (math.sqrt(5)-1)/2, # 1/φ
        (math.sqrt(5)+1)/2, # φ
        phi + 2,
        2*phi + 1,          # = √5 + 2
        2*phi - 1,          # = √5
        phi**2 + phi,       # = φ³
        # 피보나치 수
        1, 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, 144, 233, 377, 610, 987,
        # 뤼카 수
        1, 3, 4, 7, 11, 18, 29, 47, 76, 123, 199, 322, 521,
    ]
    for v in extras:
        vals.add(v)

    for k in range(1, 31):
        vals.add(k * phi)
        vals.add(phi / k)
        vals.add(phi ** (k/5))

    vals = {v for v in vals if math.isfinite(v) and v != 0}
    return sorted(vals)


def build_sqrt2_set():
    """√2 파생 집합"""
    s2 = math.sqrt(2)
    vals = set()
    for exp in range(-6, 10):
        vals.add(s2 ** exp)

    extras = [
        s2, 1/s2, s2/2, 2*s2, 3*s2,
        s2 + 1,            # 은비 ≈ 2.414
        s2 - 1,            # ≈ 0.414
        1/(s2 + 1),        # = s2 - 1
        1/(s2 - 1),        # = s2 + 1
        s2**2,             # = 2
        s2**3,             # = 2√2
        math.sqrt(3),
        math.sqrt(5),
        math.sqrt(6),      # = √2·√3
        math.sqrt(7),
        math.sqrt(10),
        math.sqrt(2) + math.sqrt(3),
        # 2의 거듭제곱 (√2 관련)
        2, 4, 8, 16, 32, 64, 128, 256, 512, 1024,
        0.5, 0.25, 0.125,
    ]
    for v in extras:
        vals.add(v)

    for k in range(1, 31):
        vals.add(k * s2)
        vals.add(s2 / k)

    vals = {v for v in vals if math.isfinite(v) and v != 0}
    return sorted(vals)


# ─────────────────────────────────────────────────────────
# 2. 매칭 엔진
# ─────────────────────────────────────────────────────────

def matches_any(measured, candidate_set, tol=0.01):
    """measured 값이 candidate_set의 어느 원소와 ±tol(1%) 이내인지 판정"""
    if not isinstance(measured, (int, float)) or not math.isfinite(measured):
        return False

    for c in candidate_set:
        if measured == 0:
            if abs(c) < 0.01:
                return True
        else:
            if abs(measured - c) / abs(measured) <= tol:
                return True
    return False


def best_match(measured, candidate_set):
    """가장 가까운 후보와 상대오차 반환"""
    if not isinstance(measured, (int, float)) or not math.isfinite(measured):
        return None, None
    best_c = None
    best_err = float('inf')
    for c in candidate_set:
        if measured == 0:
            err = abs(c)
        else:
            err = abs(measured - c) / abs(measured)
        if err < best_err:
            best_err = err
            best_c = c
    return best_c, best_err


# ─────────────────────────────────────────────────────────
# 3. 메인 실행
# ─────────────────────────────────────────────────────────

def main():
    # reality_map 로드
    map_path = "/Users/ghost/Dev/nexus/shared/reality_map.json"
    with open(map_path) as f:
        data = json.load(f)

    nodes = data.get("nodes", [])

    # 후보 집합 생성
    sets = {
        "n=6":   build_n6_set(),
        "π 파생": build_pi_set(),
        "e 파생": build_e_set(),
        "φ 파생": build_phi_set(),
        "√2 파생": build_sqrt2_set(),
    }

    print("=" * 72)
    print("  특수 수 대조 실험: n=6 vs π/e/φ/√2")
    print("  판정 기준: 상대오차 ±1% 이내 → MATCH")
    print("=" * 72)
    print()

    # 후보 집합 크기 표시
    print("[ 후보 집합 크기 ]")
    for name, s in sets.items():
        print(f"  {name:10s}: {len(s):5d}개")
    print()

    # ── 분석 대상 필터 ──
    # measured 값이 숫자인 노드만 대상 (CONVENTION 제외 선택가능)
    # 모든 grade 포함하되, origin별 분류도 실시

    valid_nodes = []
    for nd in nodes:
        m = nd.get("measured")
        if isinstance(m, (int, float)) and math.isfinite(m) and m != 0:
            valid_nodes.append(nd)

    print(f"분석 대상 노드: {len(valid_nodes)}개 (measured가 유한 비영 숫자)")
    print()

    # ── 전체 매칭률 ──
    results = {}
    for name, cset in sets.items():
        matched = sum(1 for nd in valid_nodes if matches_any(nd["measured"], cset))
        results[name] = matched

    print("=" * 72)
    print("  [ 전체 매칭률 비교 ]")
    print("-" * 72)
    print(f"  {'집합':10s} | {'매칭':>6s} / {'전체':>6s} | {'매칭률':>8s} | 막대")
    print("-" * 72)
    total = len(valid_nodes)
    max_rate = max(v / total for v in results.values())
    for name in ["n=6", "π 파생", "e 파생", "φ 파생", "√2 파생"]:
        cnt = results[name]
        rate = cnt / total * 100
        bar_len = int(rate / max_rate * 30)
        bar = "#" * bar_len
        print(f"  {name:10s} | {cnt:6d} / {total:6d} | {rate:7.2f}% | {bar}")
    print("-" * 72)
    print()

    # ── origin별 분석 (natural / engineering / convention) ──
    print("=" * 72)
    print("  [ origin별 매칭률 ]")
    print("-" * 72)

    for origin in ["natural", "engineering", "convention"]:
        subset = [nd for nd in valid_nodes if nd.get("origin") == origin]
        if not subset:
            continue
        print(f"\n  ▸ origin = {origin} ({len(subset)}개)")
        print(f"    {'집합':10s} | {'매칭':>5s}/{len(subset):<5d} | {'매칭률':>8s}")
        print(f"    {'-'*45}")
        for name in ["n=6", "π 파생", "e 파생", "φ 파생", "√2 파생"]:
            cnt = sum(1 for nd in subset if matches_any(nd["measured"], sets[name]))
            rate = cnt / len(subset) * 100
            print(f"    {name:10s} | {cnt:5d}/{len(subset):<5d} | {rate:7.2f}%")

    print()

    # ── grade별 분석 ──
    print("=" * 72)
    print("  [ grade별 매칭률 (n=6 EXACT/CLOSE/MISS 노드에 대한 대조군 성능) ]")
    print("-" * 72)

    for grade in ["EXACT", "CLOSE", "MISS", "EMPIRICAL"]:
        subset = [nd for nd in valid_nodes if nd.get("grade") == grade]
        if not subset:
            continue
        print(f"\n  ▸ grade = {grade} ({len(subset)}개)")
        print(f"    {'집합':10s} | {'매칭':>5s}/{len(subset):<5d} | {'매칭률':>8s}")
        print(f"    {'-'*45}")
        for name in ["n=6", "π 파생", "e 파생", "φ 파생", "√2 파생"]:
            cnt = sum(1 for nd in subset if matches_any(nd["measured"], sets[name]))
            rate = cnt / len(subset) * 100
            print(f"    {name:10s} | {cnt:5d}/{len(subset):<5d} | {rate:7.2f}%")

    print()

    # ── 독점 분석: n=6만 매칭 vs 대조군에도 매칭 ──
    print("=" * 72)
    print("  [ 독점 매칭 분석 ]")
    print("  n=6에만 매칭되고 어떤 대조군에도 매칭되지 않는 노드 수")
    print("-" * 72)

    n6_set = sets["n=6"]
    alt_sets = [sets[k] for k in ["π 파생", "e 파생", "φ 파생", "√2 파생"]]

    n6_only = 0
    alt_only = 0
    both = 0
    neither = 0

    for nd in valid_nodes:
        m = nd["measured"]
        n6_hit = matches_any(m, n6_set)
        alt_hit = any(matches_any(m, s) for s in alt_sets)

        if n6_hit and not alt_hit:
            n6_only += 1
        elif alt_hit and not n6_hit:
            alt_only += 1
        elif n6_hit and alt_hit:
            both += 1
        else:
            neither += 1

    print(f"  n=6 독점 매칭:           {n6_only:6d}개  ({n6_only/total*100:.1f}%)")
    print(f"  대조군 독점 매칭:        {alt_only:6d}개  ({alt_only/total*100:.1f}%)")
    print(f"  양쪽 모두 매칭:          {both:6d}개  ({both/total*100:.1f}%)")
    print(f"  어디에도 매칭 안됨:      {neither:6d}개  ({neither/total*100:.1f}%)")
    print()

    # ── 통계적 유의성 추정 ──
    print("=" * 72)
    print("  [ 매칭률 차이 통계 ]")
    print("-" * 72)

    n6_rate = results["n=6"] / total
    alt_rates = {k: results[k] / total for k in ["π 파생", "e 파생", "φ 파생", "√2 파생"]}
    best_alt_name = max(alt_rates, key=alt_rates.get)
    best_alt_rate = alt_rates[best_alt_name]

    print(f"  n=6 매칭률:          {n6_rate*100:.2f}%")
    print(f"  최고 대조군 ({best_alt_name}): {best_alt_rate*100:.2f}%")
    print(f"  차이:                 {(n6_rate - best_alt_rate)*100:+.2f}%p")
    print(f"  비율:                 {n6_rate / best_alt_rate:.2f}배" if best_alt_rate > 0 else "  비율: ∞")
    print()

    # ── 크기 보정 분석 ──
    print("=" * 72)
    print("  [ 후보 집합 크기 대비 효율 (매칭수 / 후보수) ]")
    print("-" * 72)
    print(f"  {'집합':10s} | {'후보수':>6s} | {'매칭수':>6s} | {'효율':>10s}")
    print(f"  {'-'*50}")
    for name in ["n=6", "π 파생", "e 파생", "φ 파생", "√2 파생"]:
        csize = len(sets[name])
        cnt = results[name]
        eff = cnt / csize if csize > 0 else 0
        print(f"  {name:10s} | {csize:6d} | {cnt:6d} | {eff:10.4f}")
    print()

    # ── 결론 ──
    print("=" * 72)
    print("  [ 결론 ]")
    print("=" * 72)

    if n6_rate > best_alt_rate * 1.5:
        print(f"  n=6 집합이 최고 대조군({best_alt_name}) 대비 {n6_rate/best_alt_rate:.1f}배 높은 매칭률.")
        print("  → n=6 파생 상수가 자연/공학 상수를 설명하는 데 통계적 우위를 가짐.")
    elif n6_rate > best_alt_rate:
        print(f"  n=6 집합이 대조군보다 다소 높은 매칭률 ({(n6_rate-best_alt_rate)*100:.1f}%p 차이).")
        print("  → 우위가 존재하나 압도적이지 않음. 추가 검증 필요.")
    else:
        print(f"  대조군({best_alt_name})이 n=6과 동등하거나 더 높은 매칭률.")
        print("  → n=6 특수성 주장에 대한 재검토 필요.")

    print()


if __name__ == "__main__":
    main()
