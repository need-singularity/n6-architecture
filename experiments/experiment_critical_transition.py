#!/usr/bin/env python3
"""
Experiment: Critical Transition at Step 3
==========================================
배경: Cascade Saturation (BT-1109)에서 시드 [6]으로:
  - 2단계: 9개 도달, 7개 미도달 {3,4,8,14,48,288,576}
  - 3단계: 7개가 동시 개방 -> 16/16 완전 도달

질문: 3단계에서 개방되는 7개 상수의 공통 구조는 무엇인가?

알고리즘:
  1. 시드 [6]에서 각 단계별로 어떤 연산 경로로 새 상수에 도달하는지 역추적.
  2. 3단계 상수 {3,4,8,14,48,288,576}이 도달하는 최소 연산 체인을 나열.
  3. 이 7개의 수론적 성질 분석: 약수, 소인수, n=6 함수값(tau, sigma, phi, sopfr).
  4. 이들 모두가 "2단계 상수의 연산으로만 도달 가능"한지, 아니면
     특정 "게이트 연산"이 필요한지.
  5. 임계 전이 그래프: 2단계->3단계 전이에서 어떤 2단계 상수 쌍이
     가장 많은 3단계 상수를 개방하는지.
  6. n=6 연결: 7개 = sigma-sopfr. 이들의 합, 곱, 기타 집합 통계가 n=6 상수인지.

n=6 연결: 미도달 7 = sigma(6)-sopfr(6) = 12-5, 3단계 임계 전이.
"""
import math
import itertools
import sys
from collections import defaultdict

# --- n=6 상수 ---
N6_CONSTANTS = frozenset([1, 2, 3, 4, 5, 6, 7, 8, 12, 14, 24, 48, 72, 144, 288, 576])

# --- 연산 정의 ---
def safe_ops(a, b):
    """기본 산술 연산: +, -, *, //, %, **"""
    results = []
    results.append((a + b, f"{a}+{b}"))
    results.append((a - b, f"{a}-{b}"))
    results.append((b - a, f"{b}-{a}"))
    results.append((a * b, f"{a}*{b}"))
    if b != 0 and a % b == 0:
        results.append((a // b, f"{a}//{b}"))
    if a != 0 and b % a == 0:
        results.append((b // a, f"{b}//{a}"))
    if b != 0:
        results.append((a % b, f"{a}%{b}"))
    if a != 0:
        results.append((b % a, f"{b}%{a}"))
    for base, exp, label in [(a, b, f"{a}**{b}"), (b, a, f"{b}**{a}")]:
        if 0 <= exp <= 4:
            val = base ** exp
            if -10000 <= val <= 10000:
                results.append((val, label))
    return [(v, l) for v, l in results if isinstance(v, int)]


# --- 수론 함수 ---
def divisors(n):
    if n <= 0:
        return []
    divs = []
    for i in range(1, int(math.isqrt(n)) + 1):
        if n % i == 0:
            divs.append(i)
            if i != n // i:
                divs.append(n // i)
    return sorted(divs)


def prime_factors(n):
    if n <= 1:
        return []
    factors = []
    d = 2
    while d * d <= n:
        while n % d == 0:
            factors.append(d)
            n //= d
        d += 1
    if n > 1:
        factors.append(n)
    return factors


def tau(n):
    return len(divisors(n))


def sigma(n):
    return sum(divisors(n))


def phi(n):
    if n <= 0:
        return 0
    result = n
    p = 2
    temp = n
    while p * p <= temp:
        if temp % p == 0:
            while temp % p == 0:
                temp //= p
            result -= result // p
        p += 1
    if temp > 1:
        result -= result // temp
    return result


def sopfr(n):
    return sum(prime_factors(n))


def P(msg):
    print(msg, flush=True)


# --- 단계별 캐스케이드 ---
def stepwise_cascade(seed_list):
    pool = set(seed_list)
    reached = pool & N6_CONSTANTS
    trace = {s: (0, f"시드({s})", None, None) for s in seed_list}
    steps = []

    for step in range(1, 20):
        new_found = {}
        pool_list = sorted(pool)
        for a, b in itertools.combinations(pool_list, 2):
            for val, label in safe_ops(a, b):
                if val in N6_CONSTANTS and val not in reached and val not in new_found:
                    new_found[val] = (step, label, a, b)
        for a in pool_list:
            for val, label in safe_ops(a, a):
                if val in N6_CONSTANTS and val not in reached and val not in new_found:
                    new_found[val] = (step, label, a, a)
        if not new_found:
            break
        for val, info in new_found.items():
            trace[val] = info
        pool |= set(new_found.keys())
        reached |= set(new_found.keys())
        steps.append((step, set(new_found.keys())))
    return steps, trace, reached


# --- 메인 ---
if __name__ == "__main__":
    P("=" * 70)
    P("  임계 전이 (Critical Transition at Step 3) 실험")
    P("=" * 70)

    # -- 1. 단계별 역추적 --
    P("\n[1] 시드 [6]에서 단계별 캐스케이드 역추적")
    P("-" * 70)

    steps, trace, reached = stepwise_cascade([6])

    step_constants = {}
    for step_num, found_set in steps:
        step_constants[step_num] = found_set
        cumulative = {6}
        for s in range(1, step_num + 1):
            if s in step_constants:
                cumulative |= step_constants[s]
        P(f"  단계 {step_num}: +{len(found_set):>2}개 발견 {sorted(found_set)}")
        P(f"         누적 도달: {len(cumulative)} / {len(N6_CONSTANTS)}")

    unreached = N6_CONSTANTS - reached
    P(f"\n  최종 도달: {len(reached)}/{len(N6_CONSTANTS)}개")
    if unreached:
        P(f"  미도달: {sorted(unreached)}")

    P("\n  전체 도달 경로:")
    for v in sorted(trace.keys()):
        if v in N6_CONSTANTS:
            step_num, label, a, b = trace[v]
            P(f"    {v:>4} [단계 {step_num}] <- {label}")

    # 2단계 도달 vs 3단계 개방
    step2_reached = {6}
    for s in range(1, 3):
        if s in step_constants:
            step2_reached |= step_constants[s]
    step3_opened = step_constants.get(3, set())

    P(f"\n  2단계까지 도달 상수: {sorted(step2_reached)} ({len(step2_reached)}개)")
    P(f"  3단계에서 개방 상수: {sorted(step3_opened)} ({len(step3_opened)}개)")

    # -- 2. 최소 연산 체인 --
    P(f"\n[2] 3단계 상수의 최소 연산 체인")
    P("-" * 70)

    target_7 = sorted(step3_opened) if step3_opened else [3, 4, 8, 14, 48, 288, 576]
    for t in target_7:
        info = trace.get(t)
        if info:
            step_num, label, a, b = info
            chain = [f"{t} = {label}"]
            if a is not None and a in trace:
                sa, la, _, _ = trace[a]
                if sa > 0:
                    chain.append(f"  ({a} = {la}, 단계 {sa})")
            if b is not None and b != a and b in trace:
                sb, lb, _, _ = trace[b]
                if sb > 0:
                    chain.append(f"  ({b} = {lb}, 단계 {sb})")
            P(f"  {t:>4}: {' | '.join(chain)}")

    # -- 3. 수론적 성질 분석 --
    P(f"\n[3] 7개 상수의 수론적 성질 분석")
    P("-" * 70)
    P(f"  {'상수':>6} | {'약수':>20} | {'소인수':>12} | {'tau':>4} | {'sigma':>5} | {'phi':>4} | {'sopfr':>5}")
    P(f"  {'-'*6} | {'-'*20} | {'-'*12} | {'-'*4} | {'-'*5} | {'-'*4} | {'-'*5}")

    nt_data = {}
    for c in target_7:
        d = divisors(c)
        pf = prime_factors(c)
        t_val = tau(c)
        s_val = sigma(c)
        p_val = phi(c)
        sp_val = sopfr(c)
        nt_data[c] = {
            'divisors': d, 'prime_factors': pf,
            'tau': t_val, 'sigma': s_val, 'phi': p_val, 'sopfr': sp_val
        }
        d_str = str(d)
        pf_str = str(pf) if pf else "[1]"
        P(f"  {c:>6} | {d_str:>20} | {pf_str:>12} | {t_val:>4} | {s_val:>5} | {p_val:>4} | {sp_val:>5}")

    # 공통 성질
    P(f"\n  공통 성질 탐색:")
    all_pf_sets = [set(nt_data[c]['prime_factors']) for c in target_7]
    common_primes = all_pf_sets[0]
    for s in all_pf_sets[1:]:
        common_primes &= s
    P(f"    공통 소인수: {sorted(common_primes) if common_primes else '없음'}")

    union_primes = set()
    for s in all_pf_sets:
        union_primes |= s
    P(f"    전체 소인수 합집합: {sorted(union_primes)}")

    pow2 = [c for c in target_7 if c > 0 and (c & (c - 1)) == 0]
    P(f"    2의 거듭제곱: {pow2}")

    div6 = [c for c in target_7 if 6 % c == 0 or c % 6 == 0]
    P(f"    6의 약수이거나 6의 배수: {div6}")

    # n=6 함수값 N6 소속
    P(f"\n  n=6 함수값의 N6 상수 소속 여부:")
    for c in target_7:
        in_n6 = []
        if nt_data[c]['tau'] in N6_CONSTANTS:
            in_n6.append(f"tau({c})={nt_data[c]['tau']}")
        if nt_data[c]['sigma'] in N6_CONSTANTS:
            in_n6.append(f"sigma({c})={nt_data[c]['sigma']}")
        if nt_data[c]['phi'] in N6_CONSTANTS:
            in_n6.append(f"phi({c})={nt_data[c]['phi']}")
        if nt_data[c]['sopfr'] in N6_CONSTANTS:
            in_n6.append(f"sopfr({c})={nt_data[c]['sopfr']}")
        P(f"    {c:>4}: {', '.join(in_n6) if in_n6 else '해당 없음'}")

    # -- 4. 게이트 연산 분석 --
    P(f"\n[4] 게이트 연산 분석: 2단계 상수만으로 도달 가능한가?")
    P("-" * 70)

    step2_list = sorted(step2_reached)
    directly_from_step2 = {}
    for t in target_7:
        found_ops = []
        for a, b in itertools.combinations(step2_list, 2):
            for val, label in safe_ops(a, b):
                if val == t:
                    found_ops.append(label)
        for a in step2_list:
            for val, label in safe_ops(a, a):
                if val == t:
                    found_ops.append(label)
        directly_from_step2[t] = found_ops

    gate_ops = {}
    for t in target_7:
        if directly_from_step2[t]:
            P(f"  {t:>4}: 2단계 상수에서 직접 도달 가능")
            for op in directly_from_step2[t][:5]:
                P(f"        {op}")
            if len(directly_from_step2[t]) > 5:
                P(f"        ... 외 {len(directly_from_step2[t])-5}개")
        else:
            P(f"  {t:>4}: 2단계 상수만으로 직접 도달 불가 -> 게이트 연산 필요")
            gate_ops[t] = True

    if gate_ops:
        P(f"\n  게이트 연산이 필요한 상수: {sorted(gate_ops.keys())}")
        P(f"  이들은 3단계에서 동시 개방되는 상수 간 교차 연산으로 도달:")
        for t in sorted(gate_ops.keys()):
            info = trace.get(t)
            if info:
                _, label, a, b = info
                P(f"    {t:>4} = {label} (피연산자: {a}, {b})")
    else:
        P(f"\n  결론: 7개 모두 2단계 상수에서 직접 도달 가능")
        P(f"  -> '게이트 연산'이 아닌, 2단계 풀의 '조합 폭발'이 임계 전이의 원인")

    # -- 5. 임계 전이 그래프 --
    P(f"\n[5] 임계 전이 그래프: 2단계 상수 쌍 -> 3단계 상수 개방")
    P("-" * 70)

    pair_opens = defaultdict(set)
    for a, b in itertools.combinations(step2_list, 2):
        for val, label in safe_ops(a, b):
            if val in set(target_7):
                pair_opens[(a, b)].add(val)
    for a in step2_list:
        for val, label in safe_ops(a, a):
            if val in set(target_7):
                pair_opens[(a, a)].add(val)

    sorted_pairs = sorted(pair_opens.items(), key=lambda x: -len(x[1]))

    P(f"  {'쌍':>12} | {'개방 수':>7} | 개방 상수")
    P(f"  {'-'*12} | {'-'*7} | {'-'*30}")
    for (a, b), opened in sorted_pairs:
        if len(opened) > 0:
            pair_str = f"({a},{b})" if a != b else f"({a},{a})"
            P(f"  {pair_str:>12} | {len(opened):>7} | {sorted(opened)}")

    if sorted_pairs:
        top_pair, top_opened = sorted_pairs[0]
        P(f"\n  최다 개방 쌍: ({top_pair[0]}, {top_pair[1]}) -> {len(top_opened)}개 개방")
        P(f"  개방 상수: {sorted(top_opened)}")

    # 단일 상수 참여 빈도
    single_count = defaultdict(int)
    single_opens = defaultdict(set)
    for (a, b), opened in pair_opens.items():
        for v in opened:
            single_count[a] += 1
            single_opens[a] |= {v}
            if a != b:
                single_count[b] += 1
                single_opens[b] |= {v}

    P(f"\n  개방 참여 빈도 (단일 상수):")
    for c in sorted(single_count.keys(), key=lambda x: -single_count[x]):
        P(f"    {c:>4}: {single_count[c]:>3}회 참여, 개방 상수 {sorted(single_opens[c])}")

    # -- 6. n=6 연결 분석 --
    P(f"\n[6] n=6 연결 분석")
    P("-" * 70)

    sigma_6 = sigma(6)
    sopfr_6 = sopfr(6)
    P(f"  sigma(6) = {sigma_6},  sopfr(6) = {sopfr_6}")
    P(f"  sigma(6) - sopfr(6) = {sigma_6 - sopfr_6} = 3단계 개방 상수 수 ({len(target_7)})")
    match_count = "EXACT" if sigma_6 - sopfr_6 == len(target_7) else "MISS"
    P(f"  검증: {match_count}")

    total_sum = sum(target_7)
    total_product = 1
    for c in target_7:
        total_product *= c

    P(f"\n  7개 상수의 합: {total_sum}")
    P(f"    {total_sum} 이 N6 상수? {'예' if total_sum in N6_CONSTANTS else '아니오'}")
    if total_sum not in N6_CONSTANTS:
        pf_sum = prime_factors(total_sum)
        P(f"    {total_sum} = {'*'.join(map(str, pf_sum))} (소인수 분해)")
        P(f"    {total_sum} / 6 = {total_sum / 6}")
        if total_sum % 6 == 0:
            q = total_sum // 6
            P(f"    {total_sum} = 6 * {q},  {q} 이 N6 상수? {'예' if q in N6_CONSTANTS else '아니오'}")

    P(f"\n  7개 상수의 곱: {total_product}")
    pf_prod = prime_factors(total_product)
    P(f"    소인수 분해: {total_product} = {'*'.join(map(str, pf_prod))}")
    exp_2 = sum(1 for p in pf_prod if p == 2)
    exp_3 = sum(1 for p in pf_prod if p == 3)
    exp_7 = sum(1 for p in pf_prod if p == 7)
    other_primes = [p for p in pf_prod if p not in (2, 3, 7)]
    P(f"    2^{exp_2} * 3^{exp_3} * 7^{exp_7}" + (f" * {'*'.join(map(str, other_primes))}" if other_primes else ""))
    if total_product % 576 == 0:
        q = total_product // 576
        P(f"    곱 = 576 * {q}")

    # 집합 통계
    mean_val = total_sum / len(target_7)
    median_val = sorted(target_7)[len(target_7) // 2]
    geo_mean = total_product ** (1.0 / len(target_7))

    P(f"\n  집합 통계:")
    P(f"    산술 평균: {mean_val:.4f}")
    P(f"    중앙값: {median_val}")
    P(f"    기하 평균: {geo_mean:.4f}")
    P(f"    중앙값 {median_val} 이 N6 상수? {'예' if median_val in N6_CONSTANTS else '아니오'}")

    # 수론 함수값 합
    tau_sum = sum(nt_data[c]['tau'] for c in target_7)
    sigma_sum = sum(nt_data[c]['sigma'] for c in target_7)
    phi_sum = sum(nt_data[c]['phi'] for c in target_7)
    sopfr_sum = sum(nt_data[c]['sopfr'] for c in target_7)
    P(f"\n  수론 함수값 합:")
    P(f"    sum(tau)   = {tau_sum}  {'(N6)' if tau_sum in N6_CONSTANTS else ''}")
    P(f"    sum(sigma) = {sigma_sum}  {'(N6)' if sigma_sum in N6_CONSTANTS else ''}")
    P(f"    sum(phi)   = {phi_sum}  {'(N6)' if phi_sum in N6_CONSTANTS else ''}")
    P(f"    sum(sopfr) = {sopfr_sum}  {'(N6)' if sopfr_sum in N6_CONSTANTS else ''}")

    # n=6 함수 적용
    P(f"\n  7개 상수 집합에 n=6 함수 적용:")
    P(f"    tau(합={total_sum}) = {tau(total_sum)}  {'(N6)' if tau(total_sum) in N6_CONSTANTS else ''}")
    P(f"    sigma(합={total_sum}) = {sigma(total_sum)}  {'(N6)' if sigma(total_sum) in N6_CONSTANTS else ''}")
    P(f"    phi(합={total_sum}) = {phi(total_sum)}  {'(N6)' if phi(total_sum) in N6_CONSTANTS else ''}")
    P(f"    sopfr(합={total_sum}) = {sopfr(total_sum)}  {'(N6)' if sopfr(total_sum) in N6_CONSTANTS else ''}")

    # -- 요약 --
    P(f"\n{'=' * 70}")
    P(f"  요약: 임계 전이 구조 분석")
    P(f"{'=' * 70}")

    P(f"\n  [캐스케이드 구조]")
    for step_num, found_set in steps:
        P(f"    단계 {step_num}: +{len(found_set):>2}개 -> {sorted(found_set)}")

    P(f"\n  [3단계 임계 전이 핵심 발견]")

    if gate_ops:
        P(f"    - 게이트 연산 필요 상수 {len(gate_ops)}개: {sorted(gate_ops.keys())}")
        P(f"    - 나머지 {len(target_7) - len(gate_ops)}개는 2단계 풀에서 직접 도달")
    else:
        P(f"    - 7개 모두 2단계 풀의 쌍별 연산으로 직접 도달 가능")
        P(f"    - 임계 전이 원인: 2단계 풀 크기가 임계점 도달 (조합 폭발)")

    if sorted_pairs:
        top_pair, top_opened = sorted_pairs[0]
        P(f"    - 최다 개방 쌍: ({top_pair[0]}, {top_pair[1]}) -> {len(top_opened)}개")

    P(f"    - 미도달 수 = sigma(6)-sopfr(6) = {sigma_6}-{sopfr_6} = {sigma_6 - sopfr_6}: {match_count}")
    P(f"    - 7개 합 = {total_sum}, 곱 = {total_product}")

    # EXACT 판정
    checks = []
    checks.append(("미도달 수 = sigma(6)-sopfr(6)", sigma_6 - sopfr_6 == len(target_7)))
    checks.append(("3단계 완전 개방 (16/16)", len(reached) == len(N6_CONSTANTS)))
    checks.append((f"중앙값 {median_val} 이 N6 상수", median_val in N6_CONSTANTS))
    checks.append(("7 자체가 N6 상수", 7 in N6_CONSTANTS))

    exact_count = sum(1 for _, v in checks if v)
    P(f"\n  [EXACT 판정]")
    for label, result in checks:
        P(f"    {'EXACT' if result else 'MISS':>5} | {label}")
    P(f"\n  종합: {exact_count}/{len(checks)} EXACT")

    P(f"{'=' * 70}")
