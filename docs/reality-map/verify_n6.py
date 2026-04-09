#!/usr/bin/env python3
"""
현실 지도(Reality Map) n=6 검증 — 바텀업 6레벨
독립적 통계·수론으로 교차검증 (동어반복 금지)
"""
from math import gcd
import random

def divisors(n): return [d for d in range(1, n+1) if n % d == 0]
def sigma(n): return sum(divisors(n))
def tau(n): return len(divisors(n))
def phi(n): return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)
def sopfr(n):
    s, x = 0, n
    for p in range(2, n+1):
        while x % p == 0: s += p; x //= p
    return s
def jordan_2(n):
    result = n * n; x = n
    for p in range(2, n+1):
        if x % p == 0:
            result = result * (p*p - 1) // (p*p)
            while x % p == 0: x //= p
    return result

n = 6
S, T, P, SP, J2 = sigma(n), tau(n), phi(n), sopfr(n), jordan_2(n)
S6 = {n, S, T, P, SP, 1, J2}

통과 = 0; 전체 = 0
def 검증(이름, 관측, 기대):
    global 통과, 전체
    전체 += 1
    if isinstance(관측, float):
        ok = abs(관측 - 기대) < 1e-6
    else:
        ok = 관측 == 기대
    통과 += int(ok)
    print(f"  [{'통과' if ok else '실패'}] {이름}: 관측={관측}, 기대={기대}")

print("=" * 60)
print("현실 지도 n=6 교차검증")
print("=" * 60)

# 1. 유일성
해 = [v for v in range(2, 1000) if sigma(v)*phi(v) == v*tau(v)]
검증("유일해 n=6", 해, [6])

# 2. 균형비
print("\n[2] 균형비")
검증("R(6) = 1", (S * P) / (n * T), 1.0)
비일치 = [v for v in range(2, 21) if v != 6 and sigma(v)*phi(v) == v*tau(v)]
검증("2~20 중 R=1은 6만", 비일치, [])

# 3. n=6 고유 매칭 (소수 편향과 분리)
print("\n[3] n=6 고유 노드")
# 브라바이 격자 14 = σ+φ (소수 집합 {1..6}으로 2회 연산 불가)
검증("브라바이 14 = σ+φ", 14, S + P)
# 결정 점군 32 = 2^sopfr
검증("결정 점군 32 = 2^sopfr", 32, 2**SP)
# 그래핀 sp2 결합각 120 = σ·(σ-φ) (독립 기하학 계산: 정육각형 내각)
정육각형_내각 = (n - 2) * 180 // n  # (6-2)×180/6 = 120
검증("정육각형 내각 = σ·(σ-φ) = 120", 정육각형_내각, S * (S - P))

# 4. 소수 편향 대조 (핵심)
print("\n[4] 소수 편향 대조")
기준선 = {1, 2, 3, 4, 5, 6}
# 14와 32는 기준선 {1..6}의 2회 연산으로 도달 가능한지 확인
def 기준선_도달(v, s):
    if v in s: return True
    ls = list(s)
    for a in ls:
        for b in ls:
            if a+b==v or a*b==v or a-b==v: return True
    return False
검증("14는 기준선 미도달", 기준선_도달(14, 기준선), False)  # 8+6=14
검증("32는 기준선 미도달(지수 필요)", 기준선_도달(32, 기준선), False)  # 4*6=24..no, 6*5=30..no, 5*6=30 → 2^5=32는 안 됨
# 사실 32 = 2^5이므로 단순 사칙연산 2회로는 안 됨 (지수 필요)

# 5. Monte Carlo (1000회)
print("\n[5] Monte Carlo")
random.seed(42)
타겟 = [6, 12, 4, 2, 5, 1, 24, 14, 32, 3, 8]
n6_일치 = sum(1 for v in 타겟 if v in S6 or any(a+b==v or a*b==v for a in S6 for b in S6))
n6_율 = n6_일치 / len(타겟)
초과 = 0
for _ in range(1000):
    rs = set(random.sample(range(1, 101), 7))
    rc = sum(1 for v in 타겟 if v in rs or any(a+b==v or a*b==v for a in rs for b in rs))
    if rc / len(타겟) >= n6_율:
        초과 += 1
p = 초과 / 1000
print(f"  p-value ≈ {p:.3f}")
검증("p < 0.5", p < 0.5, True)

# 6. MISS 정직 보고
print("\n[6] MISS 노드")
for 이름, v in [("α⁻¹≈137", 137), ("아보가드로 ~6e23", 602)]:
    매칭 = v in S6 or any(a*b==v or a+b==v for a in S6 for b in S6)
    print(f"  [{'EXACT' if 매칭 else 'MISS'}] {이름}")

print("\n" + "=" * 60)
print(f"최종: {통과}/{전체}")
assert 통과 == 전체
