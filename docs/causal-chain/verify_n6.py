#!/usr/bin/env python3
"""
인과 사슬 도메인 n=6 검증코드
논문: docs/paper/n6-causal-chain-paper.md
쿼크->탄소->생명 인과 경로의 n=6 산술 서명 검증
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

# ── 인과경로 1: 쿼크 -> 탄소 -> 생명 ──
# 쿼크 플레이버 6종 = n (표준모형 독립 사실)
검증("쿼크 플레이버", 6, "n", N)

# 글루온 8종 = sigma-tau (SU(3) 색역학 독립 정리: 3^2-1=8)
su3_generators = 3**2 - 1  # 리 대수 독립 계산
검증("글루온 수", su3_generators, "sigma-tau", S-T)

# 탄소 원자번호 Z=6 (핵합성 결과)
검증("탄소 Z=6", 6, "n", N)

# 탄소 전자각=2 (1s2 2s2 2p2 중 내각)
검증("탄소 전자각 수", 2, "phi", P)

# 탄소 원자가전자=4 (2s2 2p2)
검증("탄소 원자가전자", 4, "tau", T)

# ── 인과경로 2: 배위수 -> 배터리/초전도 ──
# 팔면체 CN=6 (결정장 에너지 최적)
검증("팔면체 배위수", 6, "n", N)

# ── 인과경로 3: 핵합성 -> 철 피크 ──
# 철 원자번호 Z=26 = J2+phi = 24+2
검증("철 Z=26", 26, "J2+phi", J+P)

# ── 인과경로 4: DNA -> 유전부호 -> 단백질 ──
검증("DNA 염기 수", 4, "tau", T)
검증("코돈 길이", 3, "n/phi", N//P)
검증("코돈 총수", 64, "2^n", 2**N)
검증("아미노산", 20, "J2-tau", J-T)

# ── 인과경로 5: 결정학 -> 공간군 ──
# 결정계 7개 = sigma-sopfr
검증("결정계 수", 7, "sigma-sopfr", S-SP)

# 브라베 격자 14 = sigma+phi
검증("브라베 격자", 14, "sigma+phi", S+P)

# 결정학적 점군 32 = 2^sopfr
검증("결정학 점군", 32, "2^sopfr", 2**SP)

# ── 표준모형 게이지 생성자: SU(3)xSU(2)xU(1) ──
# 8+3+1 = 12 = sigma
sm_generators = (3**2 - 1) + (2**2 - 1) + 1  # 독립 리 대수 계산
검증("표준모형 생성자 합", sm_generators, "sigma", S)

# ── 파생 상수 교차검증 ──
검증("sigma-phi=10 (초끈 차원)", 10, "sigma-phi", S-P)
검증("sigma*tau=48 (점군 위수)", 48, "sigma*tau", S*T)
검증("sigma^2=144 (평균율)", 144, "sigma^2", S*S)

# ── n=28 대조: 차순위 완전수는 실패 ──
n28 = 28
s28, p28, t28 = sigma(28), phi(28), tau(28)
balance_28 = s28 * p28 / (28 * t28)
검증("n=28 균형비 != 1", False, "R(28)=1 여부", balance_28 == 1)

# ── 결과 ──
pass_count = sum(1 for r in 결과 if r[3])
total = len(결과)
print(f"\n{'='*60}")
print(f"인과 사슬 도메인 검증 결과: {pass_count}/{total} PASS")
print(f"{'='*60}")
for 이름, 관측, 도출, ok in 결과:
    print(f"  {'PASS' if ok else 'FAIL'}: {이름} = {관측} (도출: {도출})")
assert pass_count == total, f"{total-pass_count}건 FAIL"
print("\n전체 검증 통과.")
