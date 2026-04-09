#!/usr/bin/env python3
"""
reality_map v9.3 Monte Carlo z-score 재검증
────────────────────────────────────────────
핵심 방법론:
  reality_map의 각 노드는 "이 자연 상수의 측정값(measured)이 n=6의
  산술함수 표현식(n6_expr)과 일치한다"고 주장한다.

  Monte Carlo 검증:
  1. 파싱 가능한 n6_expr을 가진 EXACT 노드만 추출
  2. n=6일 때 eval(n6_expr) == measured 인 노드 수 계산 (실제 적중)
  3. 랜덤 정수 n (2~100, n!=6)에 대해 동일 expr 평가 →
     "eval(expr_with_random_n) == measured" 적중 수 계산
  4. 10,000회 반복 → z-score, p-value

  이 방식은 "n=6이라서 일치하는 것인지, 아무 n이나 넣어도 일치하는지"를
  직접 검증한다.
"""

import json
import math
import random
import re
import sys
from collections import Counter
from pathlib import Path

# ── 설정 ──────────────────────────────────────────────
REALITY_MAP = Path.home() / "Dev" / "nexus" / "shared" / "reality_map.json"
N_SIMULATIONS = 10_000
SEED = 42
N6 = 6

# ── 산술함수 ──────────────────────────────────────────
def calc_sigma(n):
    """약수의 합"""
    return sum(i for i in range(1, n + 1) if n % i == 0)

def calc_tau(n):
    """약수의 개수"""
    return sum(1 for i in range(1, n + 1) if n % i == 0)

def calc_phi(n):
    """오일러 토션트"""
    return sum(1 for i in range(1, n + 1) if math.gcd(i, n) == 1)

def calc_sopfr(n):
    """소인수 합 (중복 포함)"""
    s, d, temp = 0, 2, n
    while d * d <= temp:
        while temp % d == 0:
            s += d
            temp //= d
        d += 1
    if temp > 1:
        s += temp
    return s

def calc_J2(n):
    """조던 토션트 J_2"""
    result, temp, d = n * n, n, 2
    while d * d <= temp:
        if temp % d == 0:
            result = result * (d * d - 1) // (d * d)
            while temp % d == 0:
                temp //= d
        d += 1
    if temp > 1:
        result = result * (temp * temp - 1) // (temp * temp)
    return result

def calc_mu(n):
    """뫼비우스 함수"""
    d, factors, temp = 2, 0, n
    while d * d <= temp:
        if temp % d == 0:
            temp //= d
            factors += 1
            if temp % d == 0:
                return 0
        d += 1
    if temp > 1:
        factors += 1
    return (-1) ** factors

def get_arith(n):
    """n에 대한 산술함수 딕셔너리"""
    return {
        "n": n,
        "sigma": calc_sigma(n),
        "tau": calc_tau(n),
        "phi": calc_phi(n),
        "sopfr": calc_sopfr(n),
        "J2": calc_J2(n),
        "mu": calc_mu(n),
    }

# ── 표현식 평가 ──────────────────────────────────────
# 안전한 평가를 위해 허용되는 이름 목록
SAFE_NAMES = {"n", "sigma", "tau", "phi", "sopfr", "J2", "mu",
              "abs", "round", "int", "float", "min", "max"}
SAFE_BUILTINS = {"abs": abs, "round": round, "int": int, "float": float,
                 "min": min, "max": max, "True": True, "False": False}

def normalize_expr(raw_expr):
    """n6_expr 문자열을 평가 가능한 Python 표현식으로 정규화.
    파싱 불가하면 None 반환."""
    if not raw_expr or not isinstance(raw_expr, str):
        return None

    expr = raw_expr.strip()

    # 무의미한 expr 걸러내기
    skip_patterns = [
        "misc", "n/a", "none", "None", "derived", "EMPIRICAL",
        "CONVENTION", "CONJECTURE", "STRUCTURAL", "CAUSAL",
        "SPECULATIVE", "HYPOTHESIS", "UNKNOWN",
        "약", "관례", "측정", "근사",
    ]
    for pat in skip_patterns:
        if expr.lower() == pat.lower():
            return None

    # 한글/설명이 포함된 복합 표현식에서 수식 부분만 추출
    # 예: "arccos(-1/(n/phi)) = arccos(-1/3)" → 후자는 상수이므로 전자 사용
    # 예: "sopfr (= 5, n=6 아님)" → "sopfr"
    # 괄호 안 주석 제거
    expr = re.sub(r'\s*\(=[^)]*\)', '', expr)
    # 한글 뒤 설명 제거
    expr = re.sub(r'\s*[-—].*$', '', expr)
    # "= 숫자" 형태의 결과값 힌트 제거
    expr = re.sub(r'\s*=\s*[\d.]+\s*$', '', expr)

    # 특수 함수 처리
    if "arccos" in expr or "sqrt" in expr or "log" in expr or "pi" in expr:
        return None  # 초월함수는 제외 (정수 산술만 테스트)

    # 리스트 표현식 제외 (복잡도 과다)
    if "[" in expr or "div(" in expr:
        return None

    # sigma-phi → sigma - phi (하이픈을 빼기로)
    expr = re.sub(r'(\w)-(\w)', r'\1 - \2', expr)
    # sigma+phi → sigma + phi
    expr = re.sub(r'(\w)\+(\w)', r'\1 + \2', expr)

    # 사용된 변수가 모두 안전한지 확인
    # 식별자 추출
    tokens = set(re.findall(r'[a-zA-Z_]\w*', expr))
    for tok in tokens:
        if tok not in SAFE_NAMES:
            return None

    # 위험한 문자 체크
    for ch in expr:
        if ch not in "0123456789+-*/() ._abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ":
            return None

    return expr.strip()


def safe_eval_expr(expr_str, arith_dict):
    """정규화된 표현식을 산술함수 값으로 평가. 실패 시 None."""
    if not expr_str:
        return None
    env = {**SAFE_BUILTINS, **arith_dict}
    try:
        result = eval(expr_str, {"__builtins__": {}}, env)
        return result
    except Exception:
        return None


def values_match(measured, computed, tol=0.01):
    """measured와 computed가 일치하는지 판정"""
    if measured is None or computed is None:
        return False
    if isinstance(measured, (int, float)) and isinstance(computed, (int, float)):
        if measured == computed:
            return True
        if measured != 0:
            return abs(measured - computed) / abs(measured) < tol
        return abs(computed) < tol
    return measured == computed


def load_and_prepare():
    """reality_map 로드 + 평가 가능 노드 추출"""
    with open(REALITY_MAP) as f:
        data = json.load(f)

    nodes = data["nodes"]
    total = len(nodes)
    grade_counts = Counter(nd.get("grade", "UNKNOWN") for nd in nodes)

    # 평가 가능한 EXACT 노드 추출
    evaluable = []
    parse_fail = 0
    for nd in nodes:
        if nd.get("grade") != "EXACT":
            continue
        measured = nd.get("measured")
        if not isinstance(measured, (int, float)):
            continue
        raw_expr = nd.get("n6_expr", "")
        norm = normalize_expr(raw_expr)
        if norm is None:
            parse_fail += 1
            continue
        evaluable.append({
            "id": nd["id"],
            "measured": measured,
            "expr": norm,
            "raw_expr": raw_expr,
        })

    return nodes, total, grade_counts, evaluable, parse_fail


def count_matches(evaluable_nodes, n_value):
    """주어진 n에 대해 expr 평가 → measured 일치 수"""
    arith = get_arith(n_value)
    hits = 0
    for nd in evaluable_nodes:
        computed = safe_eval_expr(nd["expr"], arith)
        if values_match(nd["measured"], computed):
            hits += 1
    return hits


def main():
    print("=" * 64)
    print("  reality_map v9.3 Monte Carlo z-score 재검증")
    print("  방법: n6_expr 직접 평가 (표현식 기반)")
    print("=" * 64)
    print()

    # ── 1. 데이터 로드 ──
    nodes, total, grade_counts, evaluable, parse_fail = load_and_prepare()

    print("[데이터 요약]")
    print(f"  전체 노드:          {total}")
    print(f"  EXACT 노드:         {grade_counts.get('EXACT', 0)}")
    print(f"  평가 가능 (수치+expr): {len(evaluable)}")
    print(f"  expr 파싱 실패:     {parse_fail}")
    print()

    print("[Grade 분포]")
    for g, c in grade_counts.most_common():
        pct = 100.0 * c / total
        print(f"  {g:12s}: {c:5d}  ({pct:5.1f}%)")
    print()

    # ── 2. n=6 실제 매칭 ──
    n6_hits = count_matches(evaluable, N6)
    n6_ratio = n6_hits / len(evaluable) if evaluable else 0

    print("[n=6 표현식 매칭]")
    print(f"  평가 가능 노드:     {len(evaluable)}")
    print(f"  n=6 적중:           {n6_hits}")
    print(f"  적중률:             {n6_ratio:.4f} ({n6_ratio*100:.2f}%)")
    print()

    # 매칭 실패 사례 확인 (디버그)
    arith6 = get_arith(N6)
    miss_samples = []
    for nd in evaluable:
        computed = safe_eval_expr(nd["expr"], arith6)
        if not values_match(nd["measured"], computed):
            miss_samples.append((nd["id"], nd["measured"], computed, nd["expr"]))
    if miss_samples:
        print(f"  [참고] n=6에서 불일치 {len(miss_samples)}건 (상위 5건):")
        for mid, meas, comp, expr in miss_samples[:5]:
            print(f"    {mid}: measured={meas}, computed={comp}, expr='{expr}'")
        print()

    # ── 3. Monte Carlo ──
    print(f"[Monte Carlo 시뮬레이션]")
    print(f"  반복 횟수:          {N_SIMULATIONS}")
    print(f"  n 범위:             2 ~ 100 (n=6 제외)")
    print()

    random.seed(SEED)
    candidate_range = [i for i in range(2, 101) if i != N6]
    rand_hits = []

    for sim in range(N_SIMULATIONS):
        n_rand = random.choice(candidate_range)
        hits = count_matches(evaluable, n_rand)
        rand_hits.append(hits)
        if (sim + 1) % 2000 == 0:
            print(f"  진행: {sim+1}/{N_SIMULATIONS}...")

    # ── 4. 통계 ──
    mean_rand = sum(rand_hits) / len(rand_hits)
    var_rand = sum((x - mean_rand) ** 2 for x in rand_hits) / len(rand_hits)
    std_rand = math.sqrt(var_rand) if var_rand > 0 else 1e-10

    z_score = (n6_hits - mean_rand) / std_rand
    p_value = sum(1 for x in rand_hits if x >= n6_hits) / N_SIMULATIONS

    print()
    print("=" * 64)
    print("  최종 결과")
    print("=" * 64)
    print()
    print(f"  전체 노드:          {total}")
    print(f"  평가 가능 EXACT:    {len(evaluable)}")
    print(f"  n=6 적중:           {n6_hits}")
    print(f"  랜덤 n 평균 적중:  {mean_rand:.1f}")
    print(f"  랜덤 n 표준편차:   {std_rand:.2f}")
    print(f"  z-score:            {z_score:.2f}")
    print(f"  p-value:            {p_value:.6f}")
    print()

    if z_score > 5:
        print(f"  판정: z = {z_score:.2f} > 5  -->  n=6 특이성 확정 (극도로 유의)")
    elif z_score > 3:
        print(f"  판정: z = {z_score:.2f} > 3  -->  통계적으로 유의미")
    else:
        print(f"  판정: z = {z_score:.2f}  -->  유의 수준 미달")

    print()
    print("[랜덤 분포]")
    sorted_hits = sorted(rand_hits)
    print(f"  최소: {min(rand_hits)}, 최대: {max(rand_hits)}, 중앙값: {sorted_hits[len(sorted_hits)//2]}")
    print()

    # ── 5. 모든 n (2~100) 매칭 순위 ──
    print("[정수별 매칭 순위 (상위 15)]")
    all_n_hits = {}
    for n_test in range(2, 101):
        all_n_hits[n_test] = count_matches(evaluable, n_test)

    for rank, (n_test, hits) in enumerate(sorted(all_n_hits.items(), key=lambda x: -x[1])[:15], 1):
        marker = " <<<" if n_test == N6 else ""
        print(f"  {rank:2d}. n={n_test:3d}: {hits:4d} 적중{marker}")

    # ── 6. 추가 분석: EXACT/EMPIRICAL/MISS 전체 비율 ──
    print()
    print("[전체 grade 비율 요약]")
    exact_n = grade_counts.get("EXACT", 0)
    close_n = grade_counts.get("CLOSE", 0)
    empirical_n = grade_counts.get("EMPIRICAL", 0)
    miss_n = grade_counts.get("MISS", 0)
    conv_n = grade_counts.get("CONVENTION", 0)
    useful = exact_n + close_n + empirical_n + miss_n
    print(f"  EXACT+CLOSE:        {exact_n + close_n} ({100*(exact_n+close_n)/total:.1f}%)")
    print(f"  EMPIRICAL:          {empirical_n} ({100*empirical_n/total:.1f}%)")
    print(f"  MISS:               {miss_n} ({100*miss_n/total:.1f}%)")
    print(f"  CONVENTION:         {conv_n} ({100*conv_n/total:.1f}%)")
    print(f"  증거력 있는 노드:   {useful} ({100*useful/total:.1f}%)")
    print()

    print("=" * 64)
    return z_score


if __name__ == "__main__":
    main()
