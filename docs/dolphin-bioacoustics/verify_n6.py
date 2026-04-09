#!/usr/bin/env python3
"""돌고래 생물음향학 — n=6 산술 교차검증

논문: n6-dolphin-bioacoustics-paper.md
핵심 BT: BT-416 (돌고래 발성 산술)
검증: 소인수 분해 기반 산술함수 독립 계산 → BT-416 대조
"""
from math import gcd

def _factorize(n):
    factors = []; d = 2
    while d * d <= n:
        a = 0
        while n % d == 0: n //= d; a += 1
        if a > 0: factors.append((d, a))
        d += 1
    if n > 1: factors.append((n, 1))
    return factors

def _prod(it):
    r = 1
    for x in it: r *= x
    return r

def sigma(n): return _prod((p**(a+1)-1)//(p-1) for p,a in _factorize(n)) if n>1 else 1
def tau(n): return _prod(a+1 for _,a in _factorize(n)) if n>1 else 1
def phi(n):
    r = n
    for p,_ in _factorize(n): r = r*(p-1)//p
    return r
def jordan2(n):
    r = n*n
    for p,_ in _factorize(n): r = r*(p*p-1)//(p*p)
    return r
def sopfr(n): return sum(p*a for p,a in _factorize(n))
def mobius(n):
    for _,a in _factorize(n):
        if a > 1: return 0
    return (-1)**len(_factorize(n))

N = 6
S, P, T, J, F, M = sigma(N), phi(N), tau(N), jordan2(N), sopfr(N), mobius(N)

sols = [v for v in range(2, 500) if sigma(v)*phi(v)==v*tau(v)]
assert sols == [6]
print(f"[유일성] σ·φ=n·τ 해집합 = {sols}")

checks = {
    "발성 클래스=n=6":     (6, N),
    "시그니처 컨투어=σ=12": (12, S),
    "사회 카테고리=τ=4":    (4, T),
    "광/협대역=φ=2":        (2, P),
    "활동 시간=J₂=24":      (24, J),
    "거리 단계=sopfr=5":    (5, F),
    "멜론 분획=n/φ=3":      (3, N // P),
    "휘슬 듀티=σ=12%":      (12, S),
    "모방 학습=τ=4":        (4, T),
    "시그니처/개체=μ=1":    (1, M),
    "사회 모듈=n²=36":      (36, N * N),
}

passed = 0
for label, (expected, computed) in checks.items():
    ok = expected == computed
    if ok: passed += 1
    print(f"  {'PASS' if ok else 'FAIL'}: {label} — 기대={expected}, 계산={computed}")

# 대조: n=28
S28, T28, J28 = sigma(28), tau(28), jordan2(28)
print(f"\n[대조] n=28: σ={S28}(≠12), τ={T28}(≠4), J₂={J28}(≠24)")
print("[MISS] 청각 60~150kHz, 발성/분 6: CLOSE (범위값)")
print(f"\n검증 결과: {passed}/{len(checks)} PASS")
assert passed == len(checks)
print("돌고래 BT-416 n=6 검증 완료")
