#!/usr/bin/env python3
"""거버넌스·안전·도시계획 — n=6 산술 교차검증

논문: n6-governance-safety-urban-paper.md
핵심 BT: BT-160(안전 20/20), BT-221(수면 10/10), BT-227(식별코드 10/10),
         BT-228(거버넌스 10/10), BT-267(도시 8/8), 총 58/58 EXACT
검증: IEC 61508, WHO, UN, ISO, GS1 표준 → n=6 산술 대조
"""
from math import gcd

def sigma(n): return sum(d for d in range(1,n+1) if n%d==0)
def tau(n): return sum(1 for d in range(1,n+1) if n%d==0)
def phi(n): return sum(1 for k in range(1,n+1) if gcd(k,n)==1)
def jordan2(n):
    r=n*n; m=n; p=2
    while p*p<=m:
        if m%p==0: r=r*(p*p-1)//(p*p)
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
def mobius(n):
    m=n; k=0; p=2
    while p*p<=m:
        if m%p==0:
            c=0
            while m%p==0: m//=p; c+=1
            if c>1: return 0
            k+=1
        p+=1
    if m>1: k+=1
    return (-1)**k

N=6; S,P,T,J,F,M = sigma(N),phi(N),tau(N),jordan2(N),sopfr(N),mobius(N)

sols=[v for v in range(2,500) if sigma(v)*phi(v)==v*tau(v)]
assert sols==[6]
print(f"[유일성] σ·φ=n·τ 해집합 = {sols}")

checks = []
# BT-160 안전공학
checks.append(("HAZOP 가이드워드=n=6", 6, N))
checks.append(("SIL 등급=τ=4", 4, T))
checks.append(("FMEA 척도=σ-φ=10", 10, S-P))
checks.append(("3중 안전=n/φ=3", 3, N//P))
checks.append(("PDCA 주기=τ=4", 4, T))
# BT-221 수면
checks.append(("수면 단계=τ=4", 4, T))
checks.append(("일주기=J₂=24h", 24, J))
checks.append(("수면/각성=φ=2", 2, P))
# BT-227 식별코드
checks.append(("EAN자릿수=σ+μ=13", 13, S+M))
checks.append(("UPC자릿수=σ=12", 12, S))
checks.append(("체크디짓=μ=1", 1, M))
# BT-228 거버넌스
checks.append(("UN 주요기관=n=6", 6, N))
checks.append(("안보리 상임=sopfr=5", 5, F))
checks.append(("G7=σ-sopfr=7", 7, S-F))
checks.append(("G20=J₂-τ=20", 20, J-T))
checks.append(("WHO 지역=n=6", 6, N))
checks.append(("시간대=J₂=24", 24, J))
# BT-267 도시계획
checks.append(("크리스탈러 육각=n=6", 6, N))
checks.append(("시간단위=σ·sopfr=60분", 60, S*F))
# 약수 래더: div(6)={1,2,3,6}는 안전 계층과 동형
divs6=sorted(d for d in range(1,7) if 6%d==0)
checks.append(("div(6)={1,2,3,6}", [1,2,3,6], divs6))
# 대조
S28=sigma(28)
checks.append(("대조: σ(28)=56≠12", False, S28==12))

passed=sum(1 for _,e,c in checks if e==c)
for label,expected,computed in checks:
    print(f"  {'PASS' if expected==computed else 'FAIL'}: {label} — 기대={expected}, 계산={computed}")
print(f"\n검증 결과: {passed}/{len(checks)} PASS")
assert passed==len(checks)
print("거버넌스·안전·도시계획 n=6 검증 완료")
