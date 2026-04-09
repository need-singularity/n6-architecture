#!/usr/bin/env python3
"""
결정학 소재 도메인 n=6 검증코드
논문: docs/paper/n6-crystallography-materials-paper.md
BT-85: 탄소 동소체, 결정 분류, 배위수 래더, 자기조립
"""
import math

def sigma(n): return sum(d for d in range(1, n+1) if n % d == 0)
def tau(n):   return sum(1 for d in range(1, n+1) if n % d == 0)
def phi(n):   return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)
def sopfr(n):
    s, d, m = 0, 2, n
    while d*d <= m:
        while m % d == 0: s += d; m //= d
        d += 1
    if m > 1: s += m
    return s
def jordan2(n):
    r = n*n; m = n; d = 2
    while d*d <= m:
        if m % d == 0:
            r = r*(1-1/(d*d))
            while m % d == 0: m //= d
        d += 1
    if m > 1: r = r*(1-1/(m*m))
    return int(r)

assert [v for v in range(2, 500) if sigma(v)*phi(v) == v*tau(v)] == [6]

N = 6
S, P, T, SP, J = sigma(N), phi(N), tau(N), sopfr(N), jordan2(N)

결과 = []
def 검증(이름, 관측, 식, 도출):
    결과.append((이름, 관측, 도출, 관측 == 도출))

# ── 탄소 Z=6 동소체 ──
검증("탄소 원자번호", 6, "n", N)
검증("탄소 원자가전자", 4, "tau", T)
검증("탄소 전자각", 2, "phi", P)
검증("혼성화 종류(sp/sp2/sp3)", 3, "n/phi", N//P)
검증("동소체 계열", 4, "tau", T)

# 다이아몬드: 단위셀 8원자 = sigma-tau
검증("다이아몬드 단위셀 원자", 8, "sigma-tau", S-T)
# sp3 결합 4개 = tau
검증("다이아몬드 sp3 결합수", 4, "tau", T)
# 모스 경도 10 = sigma-phi
검증("다이아몬드 모스 경도", 10, "sigma-phi", S-P)

# 그래핀: 6-fold 대칭 = n
검증("그래핀 대칭 차수", 6, "n", N)
# 이웃 원자 3개 = n/phi (sp2)
검증("그래핀 이웃수", 3, "n/phi", N//P)
# 단위셀 2원자 = phi
검증("그래핀 단위셀 원자", 2, "phi", P)

# 벤젠 C6H6: 탄소6, 총원자12=sigma
검증("벤젠 탄소", 6, "n", N)
검증("벤젠 총 원자", 12, "sigma", S)

# C60 플러렌: 탄소 60 = sigma*sopfr
검증("C60 탄소 수", 60, "sigma*sopfr", S*SP)
# C60 오각면 12 = sigma (오일러 정리: 위상적 필연)
검증("C60 오각면", 12, "sigma", S)
# C60 육각면 20 = J2-tau
검증("C60 육각면", 20, "J2-tau", J-T)
# C60 총면 32 = 2^sopfr
검증("C60 총면", 32, "2^sopfr", 2**SP)

# ── 결정 분류 계위 ──
# 결정계 7 = sigma-sopfr
검증("결정계 7종", 7, "sigma-sopfr", S-SP)
# 브라베 격자 14 = sigma+phi
검증("브라베 격자 14", 14, "sigma+phi", S+P)
# 점군 32 = 2^sopfr
검증("결정학 점군 32", 32, "2^sopfr", 2**SP)
# 결정학 허용 회전 차수: {1,2,3,4,6} = {mu, phi, n/phi, tau, n}
allowed = {1, 2, 3, 4, 6}
n6_rot = {1, P, N//P, T, N}
검증("허용 회전 차수 집합", allowed, "n6 약수+1", n6_rot)
# sopfr=5 종류의 허용 회전
검증("허용 회전 종류 수", 5, "sopfr", SP)

# ── 배위수 래더: tau->n->(sigma-tau)->sigma = 4->6->8->12 ──
cn_ladder = [T, N, S-T, S]
검증("배위수 래더", cn_ladder, "tau->n->(sigma-tau)->sigma", [4, 6, 8, 12])

# FCC 슬립 방향 12 = sigma
검증("FCC 슬립 방향", 12, "sigma", S)

# ── 결과 ──
pass_count = sum(1 for r in 결과 if r[3])
total = len(결과)
print(f"\n{'='*60}")
print(f"결정학 소재 도메인 검증 결과: {pass_count}/{total} PASS")
print(f"{'='*60}")
for 이름, 관측, 도출, ok in 결과:
    print(f"  {'PASS' if ok else 'FAIL'}: {이름} = {관측} (도출: {도출})")
assert pass_count == total, f"{total-pass_count}건 FAIL"
print("\n전체 검증 통과.")
