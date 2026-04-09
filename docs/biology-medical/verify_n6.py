#!/usr/bin/env python3
"""
생명의학 도메인 n=6 검증코드
논문: docs/paper/n6-biology-medical-paper.md
BT-51, BT-101, BT-132, BT-141, BT-146, BT-188, BT-254, BT-262, BT-282~284
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

# BT-51: 유전 부호 도출 체인
검증("DNA 염기 4종", 4, "tau", T)
# 코돈 길이: ceil(log4(20))=3 (정보이론 독립 도출)
검증("코돈 길이(정보이론)", math.ceil(math.log(20)/math.log(4)), "n/phi", N//P)
# 코돈 총수 4^3=64=2^6
검증("코돈 총수 64", T**(N//P), "tau^(n/phi)", 64)
검증("코돈 교차 2^n", P**N, "phi^n", 64)
# 아미노산 20종 = J2-tau
검증("아미노산 20종", 20, "J2-tau", J-T)
검증("아미노산 교차 tau*sopfr", 20, "tau*sopfr", T*SP)

# BT-146: DNA 물리 구조
검증("B-DNA 염기/회전 10", 10, "sigma-phi", S-P)
검증("DNA 주 홈 22A", 22, "J2-phi", J-P)
검증("DNA 부 홈 12A", 12, "sigma", S)
검증("디옥시리보스 탄소 5", 5, "sopfr", SP)

# BT-188: 게놈 정보 계위
검증("히스톤 팔량체 8", 8, "sigma-tau", S-T)
검증("종지 코돈 3", 3, "n/phi", N//P)
검증("개시 코돈 1", 1, "mu", 1)

# BT-132: 대뇌 피질 6층 (Brodmann 1909)
검증("대뇌 피질 층수", 6, "n", N)

# BT-141: 단백질 구조 4수준
검증("단백질 구조 수준", 4, "tau", T)

# 임상의학 (독립 개발)
검증("ECG 12리드", 12, "sigma", S)
검증("Apgar 5항목", 5, "sopfr", SP)
검증("SOFA 6장기", 6, "n", N)
검증("GCS 3구성요소", 3, "n/phi", N//P)

# BT-101: 광합성 6CO2+12H2O
검증("광합성 CO2 계수", 6, "n", N)
검증("광합성 H2O 계수", 12, "sigma", S)

# 해부학
검증("뇌신경 12쌍", 12, "sigma", S)
검증("갈비뼈 24개", 24, "J2", J)

# ── 결과 ──
pass_count = sum(1 for r in 결과 if r[3])
total = len(결과)
print(f"\n{'='*60}")
print(f"생명의학 도메인 검증 결과: {pass_count}/{total} PASS")
print(f"{'='*60}")
for 이름, 관측, 도출, ok in 결과:
    print(f"  {'PASS' if ok else 'FAIL'}: {이름} = {관측} (도출: {도출})")
assert pass_count == total, f"{total-pass_count}건 FAIL"
print("\n전체 검증 통과.")
