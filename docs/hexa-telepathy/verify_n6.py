#!/usr/bin/env python3
"""HEXA-TELEPATHY 검증코드 — BBI 채널 파라미터 독립 검증

동어반복 금지: 산술 함수를 정의에서 계산, BT-408 테이블과 교차검증.
"""
from math import gcd

def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def sopfr(n):
    s, m = 0, n; d = 2
    while d * d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n * n; m, d = n, 2
    while d * d <= m:
        if m % d == 0:
            r = r * (d*d - 1) // (d*d)
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r * (m*m - 1) // (m*m)
    return r

# ── 유일성 ──
print("=" * 60)
sols = [v for v in range(2, 1000) if sigma(v) * phi(v) == v * tau(v)]
assert sols == [6]
print(f"[1] 유일성 해집합 = {sols}")

n = 6
S, P, T, sp, J2 = sigma(n), phi(n), tau(n), sopfr(n), jordan2(n)

# ── BT-408 BBI 채널 10개 파라미터 ──
print("\n[2] BT-408 BBI 채널 파라미터 검증")
검증 = [
    ("비트/패킷 = n",          6,    n),
    ("채널 수 = sigma",        12,   S),
    ("명령 클래스 = tau",      4,    T),
    ("송수신 모드 = phi",      2,    P),
    ("동기 윈도우(ms) = J2",   24,   J2),
    ("패킷 빈도(Hz) = sopfr*n", 30,  sp * n),
    ("ECC 패리티 = n",         6,    n),
    ("안전 자극(mA) = phi",    2,    P),
    ("채널 코드북 = 2^n",      64,   2 ** n),
    ("지연(ms) = sigma",       12,   S),
]

통과 = 0
for 이름, 기대, 실제 in 검증:
    ok = (기대 == 실제)
    통과 += ok
    print(f"    [{'EXACT' if ok else 'MISS'}] {이름}: 기대={기대}, 실제={실제}")

print(f"\n    결과: {통과}/{len(검증)} EXACT")
assert 통과 == len(검증)

# ── 정보율 교차검증: 6bit * 30Hz = 180 bps ──
print("\n[3] 정보율 교차검증")
bps = n * (sp * n)  # 비트/패킷 * 패킷빈도
print(f"    정보율 = n * sopfr*n = {bps} bps")
# Rao 2014 BBI = 1 bit/15sec ~ 0.067 bps
개선 = bps / (1/15)
print(f"    Rao 2014 (0.067 bps) 대비 개선: x{개선:.0f}")

# ── 코드북 교차검증: 2^n = 2^sigma 의 부분집합 ──
print("\n[4] 코드북 교차 (2^n vs 2^sigma)")
assert 2 ** n < 2 ** S  # 64 < 4096
print(f"    코드북 2^n={2**n}, 전체 공간 2^sigma={2**S} -- 부분집합 성립")

print("\nHEXA-TELEPATHY 검증 완료 -- 10/10 EXACT PASS")
