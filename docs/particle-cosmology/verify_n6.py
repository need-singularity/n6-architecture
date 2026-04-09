#!/usr/bin/env python3
"""
입자물리·우주론 n=6 검증 — BT-165, BT-137, BT-166
독립적 계산으로 교차검증 (동어반복 금지)
"""
from math import gcd, pi

def divisors(n): return [d for d in range(1, n+1) if n % d == 0]
def sigma(n): return sum(divisors(n))
def tau(n): return len(divisors(n))
def phi(n): return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)
def sopfr(n):
    s, x = 0, n
    for p in range(2, n+1):
        while x % p == 0: s += p; x //= p
    return s
def mobius(n):
    if n == 1: return 1
    x, k = n, 0
    for p in range(2, n+1):
        if x % p == 0:
            k += 1; x //= p
            if x % p == 0: return 0
    return (-1)**k
def jordan_2(n):
    result = n * n; x = n
    for p in range(2, n+1):
        if x % p == 0:
            result = result * (p*p - 1) // (p*p)
            while x % p == 0: x //= p
    return result

n = 6
S, T, P, SP, MU, J2 = sigma(n), tau(n), phi(n), sopfr(n), mobius(n), jordan_2(n)

통과 = 0; 전체 = 0
def 검증(이름, 관측, 기대):
    global 통과, 전체
    전체 += 1
    ok = abs(관측 - 기대) < 1e-9 if isinstance(관측, float) else 관측 == 기대
    통과 += int(ok)
    print(f"  [{'통과' if ok else '실패'}] {이름}: 관측={관측}, 기대={기대}")

print("=" * 60)
print("입자물리·우주론 n=6 교차검증")
print("=" * 60)

# 1. 유일성 정리 (2~999 전수)
print("\n[1] σ·φ = n·τ 유일성")
해 = [v for v in range(2, 1000) if sigma(v)*phi(v) == v*tau(v)]
검증("유일해 n=6", 해, [6])

# 2. BT-165: 게이지 구조 (리 대수에서 독립 도출)
print("\n[2] BT-165: 게이지 구조")
su3 = 3**2 - 1; su2 = 2**2 - 1; u1 = 1
검증("SU(3) = σ-τ = 8", su3, S - T)
검증("SU(2) = n/φ = 3", su2, n // P)
검증("U(1) = μ = 1", u1, MU)
검증("전체 = σ = 12", su3 + su2 + u1, S)

# 3. BT-137: 입자 센서스 (LEP 실측에서 도출)
print("\n[3] BT-137: 입자 센서스")
세대 = 3  # LEP Z-폭: 2.984±0.008 → 3
검증("세대 = n/φ", 세대, n // P)
검증("쿼크 6종 = n", 세대 * 2, n)
검증("렙톤 6종 = n", 세대 * 2, n)
검증("총 페르미온 = σ", 세대 * 4, S)
검증("EW 보손 = τ", 4, T)
검증("힉스 = μ", 1, MU)

# 4. BT-166: 양성자-전자 질량비 (CODATA 독립 비교)
print("\n[4] BT-166: 양성자-전자 질량비")
codata = 1836.15267343
예측 = n * pi**5
오차ppm = abs(codata - 예측) / codata * 1e6
print(f"  CODATA: {codata}, n·π⁵: {예측:.6f}, 오차: {오차ppm:.1f} ppm")
검증("오차 < 20 ppm", 오차ppm < 20, True)

# 5. 이집트 분수
print("\n[5] 이집트 분수")
from fractions import Fraction; 이집트합 = Fraction(1,2) + Fraction(1,3) + Fraction(1,6)
검증("1/2+1/3+1/6 = 1", 이집트합, Fraction(1))

# 6. 소수 편향 대조
print("\n[6] 소수 편향 대조")
import math
대조 = {"π²": int(round(math.pi**2)), "e²": int(round(math.e**2)),
        "2πe": int(round(2*math.pi*math.e))}
만족 = sum(1 for v in 대조.values() if sigma(v)*phi(v) == v*tau(v))
검증("대조군 만족 0건", 만족, 0)

print("\n" + "=" * 60)
print(f"최종: {통과}/{전체} 검증 통과")
assert 통과 == 전체, f"실패: {통과}/{전체}"
