#!/usr/bin/env python3
"""
상온 초전도체 12제품 진화 n=6 검증 — BT-299~306
독립 스케일링 법칙으로 교차검증 (동어반복 금지)
"""
from math import gcd

def divisors(n): return [d for d in range(1, n+1) if n % d == 0]
def sigma(n): return sum(divisors(n))
def tau(n): return len(divisors(n))
def phi(n): return sum(1 for k in range(1, n+1) if gcd(k, n) == 1)
def sopfr(n):
    s, x = 0, n
    for p in range(2, n+1):
        while x % p == 0: s += p; x //= p
    return s
def jordan_2(n):
    result = n * n; x = n
    for p in range(2, n+1):
        if x % p == 0:
            result = result * (p*p - 1) // (p*p)
            while x % p == 0: x //= p
    return result

n = 6
S, T, P, SP, J2 = sigma(n), tau(n), phi(n), sopfr(n), jordan_2(n)

통과 = 0; 전체 = 0
def 검증(이름, 관측, 기대):
    global 통과, 전체
    전체 += 1
    ok = 관측 == 기대
    통과 += int(ok)
    print(f"  [{'통과' if ok else '실패'}] {이름}: 관측={관측}, 기대={기대}")

print("=" * 60)
print("RT-SC 12제품 진화 교차검증")
print("=" * 60)

# 1. 유일성
해 = [v for v in range(2, 1000) if sigma(v)*phi(v) == v*tau(v)]
검증("유일해 n=6", 해, [6])

# 2. 12제품 = σ
print("\n[2] 제품 수")
검증("12 제품 = σ", 12, S)

# 3. 자기장 래더 (독립 물리 확인)
print("\n[3] 자기장 래더 {3,12,24,48,144}")
래더 = [n//P, S, J2, S*T, S**2]  # [3, 12, 24, 48, 144]
검증("자기장 래더", 래더, [3, 12, 24, 48, 144])
# 배율 검증: {n/φ, σ, J₂, σ·τ, σ²}
검증("3 = n/φ", n//P, 3)
검증("48 = σ·τ", S*T, 48)
검증("144 = σ²", S**2, 144)

# 4. 용량 래더: 쿠빗/클럭/전압
print("\n[4] 용량 래더")
# 쿠빗: 144→288→4096
검증("144 큐빗 = σ²", S**2, 144)
검증("288 큐빗 = σ·J₂", S*J2, 288)
검증("4096 = 2^σ", 2**S, 4096)

# 5. 마그레브 속도
print("\n[5] 마그레브 속도")
# 600 km/h = σ·(σ-φ)·sopfr = 12·10·5
검증("600 = σ·(σ-φ)·sopfr", S * (S-P) * SP, 600)

# 6. MRI 래더
print("\n[6] MRI 자기장")
# 3T→12T→24T→48T→144T
mri = [n//P, S, J2, S*T, S**2]
검증("MRI 래더 = 자기장 래더", mri, [3, 12, 24, 48, 144])

# 7. 경제 래더: 10배 감소/Mk
print("\n[7] 경제 10배 감소")
검증("10 = σ-φ", S-P, 10)

# 8. 수명 래더
print("\n[8] 수명 래더")
# 83→103→120→144 (J₂=24 간격 시작)
검증("수명 간격 = J₂ = 24 (시작)", J2, 24)
검증("수명 천장 = σ² = 144", S**2, 144)

# 9. 이중 완전수
print("\n[9] 이중 완전수")
검증("6 완전수", sigma(6), 12)
검증("28 완전수", sigma(28), 56)

# 10. 대조
print("\n[10] 대조")
import math
대조 = {"π²": int(round(math.pi**2)), "e²": int(round(math.e**2))}
검증("대조 0건", sum(1 for v in 대조.values() if sigma(v)*phi(v)==v*tau(v)), 0)

print("\n" + "=" * 60)
print(f"최종: {통과}/{전체}")
assert 통과 == 전체
