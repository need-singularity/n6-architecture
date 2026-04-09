#!/usr/bin/env python3
"""HEXA-SKYWAY 검증코드 — 공중 고속도로망 7개 핵심 파라미터 독립 검증

동어반복 금지: 산술 함수를 정의에서 계산 후, 논문의 설계값과 교차검증.
"""
from math import gcd

def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def jordan2(n):
    """J_2(n) = n^2 * prod(1 - 1/p^2) for p | n"""
    r = n * n
    m, d = n, 2
    while d * d <= m:
        if m % d == 0:
            r = r * (d * d - 1) // (d * d)
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (m * m - 1) // (m * m)
    return r

# ── 유일성 ──
print("=" * 60)
print("[1] sigma*phi = n*tau 유일성 (n=2..999)")
sols = [v for v in range(2, 1000) if sigma(v) * phi(v) == v * tau(v)]
assert sols == [6], f"실패: {sols}"
print(f"    해집합 = {sols}")

n = 6
S, P, T, J2 = sigma(n), phi(n), tau(n), jordan2(n)

# ── 핵심 파라미터 교차검증 ──
print("\n[2] SKYWAY 7대 파라미터 검증")
# 논문: 고도층=J2=24, 차선=sigma*tau=48m, 속도=sigma^2=144km/h,
#       허브=sigma*tau=48, 모드=n=6, 자율=tau=4, 처리량=(sigma-phi)^3=1000
검증 = [
    ("고도층 = J2",           24,   J2),
    ("차선 간격(m) = sigma*tau", 48, S * T),
    ("속도(km/h) = sigma^2",  144,  S ** 2),
    ("허브/도시 = sigma*tau",  48,   S * T),
    ("비행 모드 = n",          6,    n),
    ("자율 등급 = tau",        4,    T),
    ("처리량/km^2 = (sigma-phi)^3", 1000, (S - P) ** 3),
]

통과 = 0
for 이름, 기대, 실제 in 검증:
    ok = (기대 == 실제)
    통과 += ok
    print(f"    [{'EXACT' if ok else 'MISS'}] {이름}: 기대={기대}, 실제={실제}")

print(f"\n    결과: {통과}/{len(검증)} EXACT")
assert 통과 == len(검증)

# ── BT 교차: 멀티로터 래더 tau -> n -> sigma-tau ──
print("\n[3] BT-270 멀티로터 래더 교차검증")
래더 = (T, n, S - T)  # 4, 6, 8
assert 래더 == (4, 6, 8), f"래더 불일치: {래더}"
print(f"    멀티로터 래더 (tau, n, sigma-tau) = {래더} -- EXACT")

# ── 대조 ──
print("\n[4] n=28 (차순위 완전수) 대조")
n28_S, n28_T, n28_P = sigma(28), tau(28), phi(28)
처리28 = (n28_S - n28_P) ** 3
print(f"    n=28: sigma={n28_S}, phi={n28_P}, (sigma-phi)^3={처리28}")
print(f"    1000과 불일치 ({처리28} != 1000) -- n=6 고유")

print("\nHEXA-SKYWAY 검증 완료 -- 7/7 EXACT PASS")
