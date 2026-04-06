#!/usr/bin/env python3
"""
검증코드 -- 고고학/문명사 n=6 기원
도메인: 고고학 / 문명사 / 고대 수학
EXACT 파라미터 검증 (가설 H-ARC 시리즈 기반)
"""

# n=6 기본 상수
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24

results = []

# 고고학/문명사 EXACT 파라미터 전수 검증
params = [
    # H-ARC-1: 메소포타미아 60진법
    ("메소포타미아 60진법 기저수", 60, "sigma*sopfr", sigma * sopfr),
    ("60의 약수 개수", 12, "sigma", sigma),

    # H-ARC-2: 이집트 12시간 체계
    ("이집트 주간 시간 수", 12, "sigma", sigma),
    ("이집트 야간 시간 수", 12, "sigma", sigma),
    ("이집트 하루 시간 수", 24, "J2", J2),

    # H-ARC-3: C-14 연대측정 핵
    ("탄소-14 원자번호 Z", 6, "n", n),

    # H-ARC-4: 3시대 구분법 (석기/청동기/철기)
    ("3시대 구분법", 3, "n/phi", n // phi),

    # H-ARC-5: 마야 20진법
    ("마야 20진법 기저수", 20, "J2-tau", J2 - tau),

    # H-ARC-9: 바빌로니아 360도
    ("원 360도", 360, "n*sigma*sopfr", n * sigma * sopfr),

    # H-ARC-10: 쐐기문자 기본획
    ("쐐기문자 기본획 종류", 2, "phi", phi),

    # H-ARC-12: 이집트 분수
    ("이집트 분수 1/2+1/3+1/6", 1, "R(6)=mu", mu),

    # 추가: BT-233 연결 파라미터
    ("시간 단위 60초", 60, "sigma*sopfr", sigma * sopfr),
    ("시간 단위 60분", 60, "sigma*sopfr", sigma * sopfr),
    ("각도 단위 360도", 360, "n*sigma*sopfr", n * sigma * sopfr),
]

for name, actual, formula, expected in params:
    match = actual == expected
    results.append((name, actual, expected, formula, match))

# 이집트 분수 특별 검증
from fractions import Fraction
egyptian = Fraction(1, 2) + Fraction(1, 3) + Fraction(1, 6)
results.append(("이집트 분수 합 = 1 검증", int(egyptian), 1, "1/2+1/3+1/6=1", int(egyptian) == 1))

# 결과 출력
passed = sum(1 for r in results if r[4])
total = len(results)
tag = "ALIEN-10 달성!" if passed == total else "미달"
print(f"고고학/문명사 검증: {passed}/{total} PASS {'<>' if passed == total else '!!'} {tag}")
print()
for name, actual, expected, formula, match in results:
    status = "PASS" if match else "FAIL"
    print(f"  {status}: {name} = {actual} (n6: {formula} = {expected})")
print()
print(f"EXACT 비율: {passed}/{total} = {100*passed//total}%")
