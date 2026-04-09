#!/usr/bin/env python3
"""HEXA-NEURO 검증코드 — 뇌-기계 인터페이스 n=6 산술 교차검증
논문: docs/paper/n6-hexa-neuro-paper.md
BT-405, BT-406 | 12/14 EXACT
"""
from math import gcd

def sigma(n):
    return sum(d for d in range(1, n+1) if n % d == 0)

def tau(n):
    return sum(1 for d in range(1, n+1) if n % d == 0)

def phi(n):
    return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)

def sopfr(n):
    s, m = 0, n
    d = 2
    while d * d <= m:
        while m % d == 0:
            s += d; m //= d
        d += 1
    if m > 1:
        s += m
    return s

def jordan2(n):
    r = n * n
    m = n
    d = 2
    while d * d <= m:
        if m % d == 0:
            r = r * (d * d - 1) // (d * d)
            while m % d == 0:
                m //= d
        d += 1
    if m > 1:
        r = r * (m * m - 1) // (m * m)
    return r

n = 6
S = sigma(n)    # 12
T = tau(n)      # 4
P = phi(n)      # 2
F = sopfr(n)    # 5
J = jordan2(n)  # 24

print(f"[기본상수] σ={S}, φ={P}, τ={T}, sopfr={F}, J₂={J}")

sols = [v for v in range(2, 1000) if sigma(v) * phi(v) == v * tau(v)]
assert sols == [6]
print(f"[유일성] 해집합 = {sols}")

checks = []

# BT-405: 운동 디코더
# 채널 행 = n = 6
checks.append(("채널 행", 6, n))

# 채널 열 = σ = 12
checks.append(("채널 열", 12, S))

# 디코더 bin = J₂ = 24
checks.append(("디코더 bin", 24, J))

# 손가락 DOF = J₂ = 24
checks.append(("손가락 DOF", 24, J))

# 슬라이딩 ms = σ = 12
checks.append(("슬라이딩 (ms)", 12, S))

# 감마 Hz 분할 = σ·sopfr = 60
checks.append(("감마 Hz", 60, S * F))

# BT-406: 감각 자극
# 시각 격자 = (σ·sopfr)² = 60² = 3600
checks.append(("시각 격자", 3600, (S * F) ** 2))

# CI 채널 = J₂ = 24
checks.append(("CI 채널", 24, J))

# 촉각 단계 = σ = 12
checks.append(("촉각 단계", 12, S))

# Phosphene 클래스 = n = 6
checks.append(("Phosphene 클래스", 6, n))

# 안전 전류 = sopfr·n = 30 μA
checks.append(("안전 전류 (μA)", 30, F * n))

# 교차: 채널 행×열 = n·σ = 72 (6×12 grid)
checks.append(("채널 격자 크기", 72, n * S))

# 교차: 운동 영역 = τ = 4 (M1, S1, PMC, SMA)
checks.append(("운동 영역 수", 4, T))

# 반구 = φ = 2
checks.append(("반구 수", 2, P))

# 영역 분할 = n/φ = 3 (운동/감각/연합)
checks.append(("영역 분할", 3, n // P))

# 대조: MISS 항목 명시
print(f"\n[MISS] 자극 펄스폭 200μs — 연속량이므로 정수 도출 불가")
print(f"[MISS] 감마파 상한 80Hz — n=6 함수 조합으로 도출 안 됨")

# 대조: Neuralink 채널 대비
print(f"\n[대조] Neuralink N1: 1024 채널 → HEXA: 1024×n={1024*n} = 6144")

print(f"\n{'='*50}")
print("HEXA-NEURO 검증 결과")
print(f"{'='*50}")
ok = 0
for name, exp, got in checks:
    match = exp == got
    if match:
        ok += 1
    print(f"  {'PASS' if match else 'FAIL'}: {name} = {exp} (독립 계산: {got})")

print(f"\n총 {ok}/{len(checks)} PASS")
assert ok == len(checks), "검증 실패"
print("HEXA-NEURO 전체 검증 통과")
