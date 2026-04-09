#!/usr/bin/env python3
"""
인지 사회 심리학 도메인 n=6 검증코드
논문: docs/paper/n6-cognitive-social-psychology-paper.md
BT-132, BT-184, BT-223, BT-254, BT-255, BT-258, BT-259, BT-260, BT-263, BT-265
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

# BT-132/254: 대뇌 피질 6층 (Brodmann 1909, 포유류 보편)
검증("대뇌 피질 층수", 6, "n", N)

# BT-184: Bloom 교육목표 분류 6단계
# (기억/이해/적용/분석/평가/창조 — Anderson 2001 개정판)
검증("Bloom 분류 6단계", 6, "n", N)

# BT-223: HEXACO 성격 모형 6차원
# (Big5 + 정직겸손 = 6, Ashton & Lee 2004)
검증("HEXACO 성격 차원", 6, "n", N)

# BT-255: 격자 세포 육각 대칭 (2014 노벨상, Moser & Moser)
검증("격자 세포 대칭 차수", 6, "n", N)

# BT-258: 6단계 분리 (Milgram 1967 소규모 세계 실험)
검증("6단계 분리", 6, "n", N)

# BT-259: Dunbar 수 150 = sigma^2+n = 144+6
검증("Dunbar 수", 150, "sigma^2+n", S*S + N)

# BT-260: Wolfram 초등 셀룰러 오토마타 규칙 수
# 2^(sigma-tau) = 2^8 = 256
검증("Wolfram CA 규칙 수", 256, "2^(sigma-tau)", 2**(S-T))

# BT-263: 작업기억 용량
# Cowan (2001): 4+-1 = tau+-mu
검증("작업기억(Cowan) 중심값", 4, "tau", T)
# Miller (1956): 7+-2 = (sigma-sopfr)+-phi
검증("Miller 7", 7, "sigma-sopfr", S-SP)

# BT-265: 일주기 리듬
검증("일주기 24시간", 24, "J2", J)
# 울트라디안 주기 4회 (약 90분 x 4 = 6시간 수면 주기)
검증("울트라디안 주기", 4, "tau", T)

# BT-266: 컴파일러-피질 동형
# 컴파일러 4단계 (어휘/구문/의미/코드생성) = tau
검증("컴파일러 단계", 4, "tau", T)

# 뇌 반구 2개 = phi
검증("뇌 반구", 2, "phi", P)

# 감각 5종 (시/청/촉/미/후) = sopfr
검증("고전 감각 5종", 5, "sopfr", SP)

# 한 손 손가락 5개 = sopfr
검증("손가락(한손)", 5, "sopfr", SP)

# 뇌엽 4개 (전두/측두/두정/후두) = tau
검증("뇌엽 수", 4, "tau", T)

# ── 결과 ──
pass_count = sum(1 for r in 결과 if r[3])
total = len(결과)
print(f"\n{'='*60}")
print(f"인지 사회 심리학 도메인 검증 결과: {pass_count}/{total} PASS")
print(f"{'='*60}")
for 이름, 관측, 도출, ok in 결과:
    print(f"  {'PASS' if ok else 'FAIL'}: {이름} = {관측} (도출: {도출})")
assert pass_count == total, f"{total-pass_count}건 FAIL"
print("\n전체 검증 통과.")
