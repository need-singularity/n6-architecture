#!/usr/bin/env python3
"""
고전역학 가속기 도메인 n=6 검증코드
논문: docs/paper/n6-classical-mechanics-accelerator-paper.md
BT-201, BT-238, BT-165, BT-168
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

# ── BT-201: 고전역학 위상공간 ──
# 1입자 위상공간 dim = 2*3 = 6 (심플렉틱 기하)
검증("1입자 위상공간 차원", 2*3, "n", N)

# 단순 기계 6종 (아르키메데스 ~250 BC)
검증("단순 기계 수", 6, "n", N)

# 뉴턴 법칙 3개
검증("뉴턴 법칙", 3, "n/phi", N//P)

# 케플러 법칙 3개
검증("케플러 법칙", 3, "n/phi", N//P)

# 오일러 각 3개
검증("오일러 각", 3, "n/phi", N//P)

# 뇌터 대칭-보존 쌍 3개 (시간/공간/회전 -> 에너지/운동량/각운동량)
검증("뇌터 보존 쌍", 3, "n/phi", N//P)

# 해밀턴 정준 방정식 2종 (dq/dt, dp/dt)
검증("해밀턴 정준 방정식", 2, "phi", P)

# 라그랑주 변수 2종 (q, dq/dt)
검증("라그랑주 변수", 2, "phi", P)

# 2체 위상공간 = 2*6 = 12 = sigma
검증("2체 위상공간 차원", 2*6, "sigma", S)

# ── BT-238: LHC 가속기 ──
# LHC 터널 둘레 27km = (n/phi)^3
검증("LHC 둘레(km)", 27, "(n/phi)^3", (N//P)**3)

# LHC 설계 에너지 14 TeV = sigma+phi
검증("LHC 설계 에너지(TeV)", 14, "sigma+phi", S+P)

# LHC 섹터 8개 = sigma-tau (독립: CERN 공학 설계)
검증("LHC 옥탄트", 8, "sigma-tau", S-T)

# LHC 주요 실험 4개 (ATLAS/CMS/ALICE/LHCb)
검증("LHC 주요 실험", 4, "tau", T)

# LHC 인젝터 체인 5단계 (Linac4/PSB/PS/SPS/LHC)
검증("LHC 인젝터 체인", 5, "sopfr", SP)

# 번치 간격 25 ns = sopfr^2
검증("LHC 번치 간격(ns)", 25, "sopfr^2", SP**2)

# ── BT-165: 표준모형 게이지 구조 ──
# SU(3): 3^2-1 = 8 = sigma-tau
검증("SU(3) 생성자", 3**2-1, "sigma-tau", S-T)
# SU(2): 2^2-1 = 3 = n/phi
검증("SU(2) 생성자", 2**2-1, "n/phi", N//P)
# U(1): 1
검증("U(1) 생성자", 1, "mu", 1)
# 합: 8+3+1=12=sigma
검증("SM 생성자 합", 8+3+1, "sigma", S)

# ── BT-168: SU(5) GUT ──
# SU(5) 생성자 = 5^2-1 = 24 = J2
검증("SU(5) 생성자", 5**2-1, "J2", J)

# 물질 4상태 = tau (고체/액체/기체/플라즈마)
검증("물질 4상태", 4, "tau", T)

# ── 결과 ──
pass_count = sum(1 for r in 결과 if r[3])
total = len(결과)
print(f"\n{'='*60}")
print(f"고전역학 가속기 도메인 검증 결과: {pass_count}/{total} PASS")
print(f"{'='*60}")
for 이름, 관측, 도출, ok in 결과:
    print(f"  {'PASS' if ok else 'FAIL'}: {이름} = {관측} (도출: {도출})")
assert pass_count == total, f"{total-pass_count}건 FAIL"
print("\n전체 검증 통과.")
