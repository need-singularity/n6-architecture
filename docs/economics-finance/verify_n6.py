#!/usr/bin/env python3
"""경제학·금융공학 — n=6 산술 교차검증

논문: n6-economics-finance-paper.md
핵심 BT: BT-147, BT-183, BT-338, BT-339
검증: 국제 금융 표준 vs n=6 산술 독립 대조
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
def mobius(n):
    m=n; primes=[]; p=2
    while p*p<=m:
        if m%p==0:
            c=0
            while m%p==0: m//=p; c+=1
            if c>1: return 0
            primes.append(p)
        p+=1
    if m>1: primes.append(m)
    return (-1)**len(primes)

N=6; S,P,T,J,F,M = sigma(N),phi(N),tau(N),jordan2(N),sopfr(N),mobius(N)

sols=[v for v in range(2,500) if sigma(v)*phi(v)==v*tau(v)]
assert sols==[6]
print(f"[유일성] σ·φ=n·τ 해집합 = {sols}")

checks = []
checks.append(("월/년=σ=12", 12, S))
checks.append(("분기=τ=4", 4, T))
checks.append(("FX사이클=J₂=24h", 24, J))
checks.append(("복식부기=φ=2면", 2, P))
checks.append(("거래일/주=sopfr=5", 5, F))
checks.append(("미국주요지수=n/φ=3", 3, N//P))
checks.append(("GICS섹터=σ-μ=11", 11, S-M))
checks.append(("FOMC회의=σ-τ=8/년", 8, S-T))
checks.append(("포터5Forces=sopfr=5", 5, F))
checks.append(("Basel III=n/φ=3기둥", 3, N//P))
checks.append(("회계방정식=n/φ=3요소", 3, N//P))
checks.append(("재무제표=τ=4종", 4, T))
checks.append(("S&P등급=σ=12", 12, S))
checks.append(("신용노치=J₂=24", 24, J))
checks.append(("GAAP원칙=σ-φ=10", 10, S-P))
checks.append(("G20=J₂-τ=20", 20, J-T))
checks.append(("장기파동=σ·sopfr=60년", 60, S*F))
checks.append(("연간주수≈τ·(σ+μ)=52", 52, T*(S+M)))
# 12의 약수가 회계달력 완전분할
divs12=sorted(d for d in range(1,13) if 12%d==0)
checks.append(("12약수={1,2,3,4,6,12}", [1,2,3,4,6,12], divs12))
# 대조
checks.append(("대조: σ(28)≠12", False, sigma(28)==12))

passed=sum(1 for _,e,c in checks if e==c)
for label,expected,computed in checks:
    print(f"  {'PASS' if expected==computed else 'FAIL'}: {label} — 기대={expected}, 계산={computed}")
print(f"\n검증 결과: {passed}/{len(checks)} PASS")
assert passed==len(checks)
print("경제학·금융공학 n=6 검증 완료")
