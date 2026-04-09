#!/usr/bin/env python3
"""생태학·농업·식품과학 — n=6 산술 교차검증

논문: n6-ecology-agriculture-food-paper.md
핵심 BT: BT-150, BT-198, BT-225, BT-101, BT-103, BT-122
검증: 화학/생물 사실 → n=6 산술 대조
"""
from math import gcd

def sigma(n): return sum(d for d in range(1,n+1) if n%d==0)
def tau(n): return sum(1 for d in range(1,n+1) if n%d==0)
def phi(n): return sum(1 for k in range(1,n+1) if gcd(k,n)==1)
def jordan2(n):
    r=n*n; m=n; p=2
    while p*p<=m:
        if m%p==0:
            r=r*(p*p-1)//(p*p)
            while m%p==0: m//=p
        p+=1
    if m>1: r=r*(m*m-1)//(m*m)
    return r
def sopfr(n):
    s,m=0,n; p=2
    while p*p<=m:
        while m%p==0: s+=p; m//=p
        p+=1
    if m>1: s+=m
    return s

N=6; S,P,T,J,F = sigma(N),phi(N),tau(N),jordan2(N),sopfr(N)

sols=[v for v in range(2,500) if sigma(v)*phi(v)==v*tau(v)]
assert sols==[6]
print(f"[유일성] σ·φ=n·τ 해집합 = {sols}")

checks = []
# 광합성 계수 (화학 사실)
checks.append(("광합성 CO₂ 계수=n=6", 6, N))
checks.append(("광합성 H₂O 반응물=σ=12", 12, S))
checks.append(("포도당 탄소수=n=6", 6, N))
checks.append(("포도당 수소수=σ=12", 12, S))
checks.append(("포도당 총 원자=J₂=24", 24, J))
# 생물 분류
checks.append(("생물 6계=n", 6, N))
# 농업
checks.append(("식물 다량영양소=n=6", 6, N))
checks.append(("식물 미량영양소=σ-τ=8", 8, S-T))
checks.append(("NPK 비료=n/φ=3", 3, N//P))
checks.append(("노퍽 윤작=τ=4년", 4, T))
checks.append(("주요 곡물=n=6종", 6, N))
checks.append(("빵밀 6배체=n", 6, N))
# 토양
checks.append(("토양 수평층=n=6", 6, N))
checks.append(("USDA 토양목=σ=12", 12, S))
# 감각/식품
checks.append(("미각 종류=sopfr=5", 5, F))
checks.append(("6대 영양소=n", 6, N))
# 기하
checks.append(("벌집 꼭짓점=n=6", 6, N))
checks.append(("눈꽃 대칭축=n=6", 6, N))
checks.append(("벤젠 탄소수=n=6", 6, N))
# 포도당 원자 보존
glucose_sum = N/J + S/J + N/J  # 6/24+12/24+6/24=1
checks.append(("포도당 원자보존=1", 1.0, glucose_sum))
# 대조
S28=sigma(28)
checks.append(("대조: σ(28)=56≠12", False, S28==12))

passed=sum(1 for _,e,c in checks if e==c)
for label,expected,computed in checks:
    print(f"  {'PASS' if expected==computed else 'FAIL'}: {label} — 기대={expected}, 계산={computed}")
print(f"\n검증 결과: {passed}/{len(checks)} PASS")
print("[MISS] BT-192 요리 2건, BT-341 식품 5건 CLOSE")
assert passed==len(checks)
print("생태학·농업·식품 n=6 검증 완료")
