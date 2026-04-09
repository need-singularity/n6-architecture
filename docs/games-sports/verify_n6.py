#!/usr/bin/env python3
"""게임·스포츠 — n=6 산술 교차검증

논문: n6-games-sports-paper.md
핵심 BT: BT-144(체스), BT-212(보드게임 래더)
검증: 독립 문명이 만든 게임 상수 → n=6 산술 대조
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
# 체스
checks.append(("체스 말 종류=n=6", 6, N))
checks.append(("체스판=(σ-τ)²=64=2^n", 64, (S-T)**2))
checks.append(("체스판=2^n 교차", 64, 2**N))
checks.append(("말/선수=2^τ=16", 16, 2**T))
# 주사위
checks.append(("주사위 면=n=6", 6, N))
checks.append(("반대면합=σ-sopfr=7", 7, S-F))
# 도미노 (더블식스) = 28 = 두번째 완전수
P2 = T * (S - F)  # τ·(σ-sopfr) = 4·7 = 28
checks.append(("도미노 타일=τ·(σ-sopfr)=28", 28, P2))
# 카드
checks.append(("카드 무늬=τ=4", 4, T))
checks.append(("카드 숫자/무늬=σ+μ=13", 13, S+M))
checks.append(("페이스카드=σ=12", 12, S))
checks.append(("카드 총=τ·(σ+μ)=52", 52, T*(S+M)))
# 백개먼
checks.append(("백개먼 포인트=J₂=24", 24, J))
# 마작
checks.append(("마작 타일=σ²=144", 144, S*S))
# 올림픽
checks.append(("올림픽 고리=sopfr=5", 5, F))
checks.append(("올림픽 주기=τ=4년", 4, T))
# 팀 스포츠
checks.append(("축구 팀=σ-μ=11명", 11, S-M))
checks.append(("농구 팀=sopfr=5명", 5, F))
# Nash 균형 유형 = φ = 2
checks.append(("Nash 균형유형=φ=2", 2, P))
# Arrow 조건 = sopfr = 5
checks.append(("Arrow 조건=sopfr=5", 5, F))
# 감각 = sopfr = 5
checks.append(("고전감각=sopfr=5", 5, F))
# 원추세포 = n/φ = 3
checks.append(("원추세포=n/φ=3종", 3, N//P))
# 대조
checks.append(("대조: σ(28)²=3136≠144", False, sigma(28)**2==144))

passed=sum(1 for _,e,c in checks if e==c)
for label,expected,computed in checks:
    print(f"  {'PASS' if expected==computed else 'FAIL'}: {label} — 기대={expected}, 계산={computed}")
print(f"\n검증 결과: {passed}/{len(checks)} PASS")
assert passed==len(checks)
print("게임·스포츠 n=6 검증 완료")
