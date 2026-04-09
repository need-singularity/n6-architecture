#!/usr/bin/env python3
"""디스플레이 8단 스택 — n=6 산술 교차검증

논문: n6-display-8stack-paper.md
핵심 BT: 8단=σ-τ, ppi=σ·J₂=288, Hz=σ²=144, FoV=σ·J₂/φ=144
검증 방식: 산술함수를 독립 계산 후 논문 값과 대조
"""
from math import gcd

# ── 산술함수 독립 구현 (소인수 분해 기반) ──
def sigma(n):
    result = 1
    m = n; p = 2
    while p * p <= m:
        if m % p == 0:
            pk = 1
            while m % p == 0: m //= p; pk *= p
            result *= (pk * p - 1) // (p - 1)
        p += 1
    if m > 1: result *= (m + 1)
    return result

def tau(n):
    result = 1
    m = n; p = 2
    while p * p <= m:
        a = 0
        while m % p == 0: m //= p; a += 1
        if a > 0: result *= (a + 1)
        p += 1
    if m > 1: result *= 2
    return result

def phi(n):
    result = n; m = n; p = 2
    while p * p <= m:
        if m % p == 0:
            while m % p == 0: m //= p
            result -= result // p
        p += 1
    if m > 1: result -= result // m
    return result

def jordan2(n):
    r = n * n; m = n; p = 2
    while p * p <= m:
        if m % p == 0:
            while m % p == 0: m //= p
            r = r * (p * p - 1) // (p * p)
        p += 1
    if m > 1: r = r * (m * m - 1) // (m * m)
    return r

def sopfr(n):
    s = 0; m = n; p = 2
    while p * p <= m:
        while m % p == 0: s += p; m //= p
        p += 1
    if m > 1: s += m
    return s

# ── n=6 상수 독립 계산 ──
N = 6
S, P, T, J, F = sigma(N), phi(N), tau(N), jordan2(N), sopfr(N)

# ── 유일성: σ·φ = n·τ 는 n=6에서만 성립 ──
sols = [v for v in range(2, 500) if sigma(v) * phi(v) == v * tau(v)]
assert sols == [6], f"유일성 위반: {sols}"
print(f"[유일성] σ(n)·φ(n)=n·τ(n) 해집합 (2≤n<500) = {sols}")

# ── 논문 수식 독립 교차검증 ──
checks = []
checks.append(("8단 스택 수 = σ-τ", 8, S - T))
checks.append(("패널 ppi = σ·J₂ = 288", 288, S * J))
checks.append(("HDR 비트깊이 = 2^τ = 16", 16, 2 ** T))
checks.append(("리프레시 = σ² = 144 Hz", 144, S * S))
checks.append(("FoV = σ·J₂/φ = 144", 144, S * J // P))
checks.append(("트래킹 DoF = J₂ = 24", 24, J))
checks.append(("톤매핑 LUT = 2^(σ+μ) = 8192", 8192, 2 ** (S + 1)))
checks.append(("휘도 = σ²·J₂ = 3456 nit", 3456, S * S * J))
checks.append(("드라이버 채널 = σ² = 144", 144, S * S))
checks.append(("홀로 갱신 = J₂ = 24 Hz", 24, J))
checks.append(("뉴럴 IF = σ-τ = 8 kHz", 8, S - T))
checks.append(("이집트분수 1/2+1/3+1/6", True, abs(1/2 + 1/3 + 1/6 - 1.0) < 1e-14))

# ── 대조: n=28 로는 디스플레이 상수 불일치 ──
S28, T28 = sigma(28), tau(28)
checks.append(("대조: σ(28)-τ(28)≠8", False, (S28 - T28) == 8))
checks.append(("대조: σ(28)²≠144", False, S28 * S28 == 144))

passed = sum(1 for _, e, c in checks if e == c)
for label, expected, computed in checks:
    print(f"  {'PASS' if expected == computed else 'FAIL'}: {label} — 기대={expected}, 계산={computed}")

print(f"\n검증 결과: {passed}/{len(checks)} PASS")
assert passed == len(checks)
print("디스플레이 8단 n=6 검증 완료")
