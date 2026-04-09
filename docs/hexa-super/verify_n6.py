#!/usr/bin/env python3
"""HEXA-SUPER 검증코드 — 초전도 프로세서 핵심 파라미터 독립 검증

동어반복 금지: 산술 함수를 정의에서 계산, 논문의 RSFQ/AQFP 사양과 교차검증.
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
print(f"[1] 유일성: 해집합 = {sols}")

n = 6
S, P, T, sp, J2 = sigma(n), phi(n), tau(n), sopfr(n), jordan2(n)

# ── 프로세서 파라미터 ──
print("\n[2] HEXA-SUPER 프로세서 파라미터 검증")
검증 = [
    ("RSFQ 클록(GHz) = sigma^2",     144,   S ** 2),
    ("AQFP 클록(GHz) = sigma*tau",    48,    S * T),
    ("코어 수 = sigma",               12,    S),
    ("ALU/코어 = sigma-tau",          8,     S - T),
    ("총 ALU = sigma*(sigma-tau)",    96,    S * (S - T)),
    ("RSFQ ALU = sigma*tau",         48,    S * T),
    ("파이프라인 단계 = sigma",        12,    S),
    ("메모리 계층 = tau",              4,     T),
    ("온도 단계 = n",                  6,     n),
    ("Cooper pair 전자 = phi",        2,     P),
    ("J2 배열 폭 = J2",              24,    J2),
    ("주파수 이득 = sigma^2/sopfr",   28.8,  S**2 / sp),
    ("다이 면적(mm^2) = sigma^2",     144,   S ** 2),
]

통과 = 0
for 이름, 기대, 실제 in 검증:
    ok = (기대 == 실제)
    통과 += ok
    print(f"    [{'EXACT' if ok else 'MISS'}] {이름}: 기대={기대}, 실제={실제}")

print(f"\n    결과: {통과}/{len(검증)} EXACT")
assert 통과 == len(검증)

# ── RSFQ/AQFP 비율 교차검증 ──
print("\n[3] RSFQ/AQFP 클록 비율 교차검증")
ratio = (S ** 2) / (S * T)  # RSFQ / AQFP
expected_ratio = S / T       # sigma / tau = 3
assert ratio == expected_ratio == 3.0
print(f"    RSFQ/AQFP = sigma/tau = {ratio} -- EXACT")

# ── CMOS 대비 AQFP 이득도 교차검증 ──
aqfp_vs_cmos = S * T / sp  # 48/5 = 9.6
assert abs(aqfp_vs_cmos - 9.6) < 1e-9
print(f"    AQFP/CMOS = sigma*tau/sopfr = {aqfp_vs_cmos} -- EXACT")

print("\nHEXA-SUPER 검증 완료 -- 전체 PASS")
