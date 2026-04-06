#!/usr/bin/env python3
"""
검증코드 -- BT-373 한글 문자체계 n=6 정보 인코딩
도메인: 문자학 / 한국어학 / 정보 인코딩
14/14 EXACT 검증
"""

# n=6 기본 상수
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24

results = []

# BT-373 파라미터 전수 검증
params = [
    ("현대 한글 자모 수", 24, "J2", J2),
    ("자음 수", 14, "sigma+phi", sigma + phi),
    ("모음 수", 10, "sigma-phi", sigma - phi),
    ("초중종 구조 부분", 3, "n/phi", n // phi),
    ("기본 자음 수 (ㄱㄴㄷㅁㅅ)", 5, "sopfr", sopfr),
    ("기본 모음 수 (천지인)", 3, "n/phi", n // phi),
    ("쌍자음 수", 5, "sopfr", sopfr),
    ("겹모음 수", 11, "sigma-mu", sigma - mu),
    ("초성 수", 19, "J2-sopfr", J2 - sopfr),
    ("중성 수", 21, "J2-n/phi", J2 - n // phi),
    ("종성 수 (없음 포함)", 28, "J2+tau=P2", J2 + tau),
    ("한글 음절 수", 11172, "(J2-sopfr)*(J2-n/phi)*(J2+tau)",
     (J2 - sopfr) * (J2 - n // phi) * (J2 + tau)),
    ("훈민정음 원래 자모", 28, "J2+tau=P2", J2 + tau),
    ("현대 폐지 자모 수", 4, "tau", tau),
]

for name, actual, formula, expected in params:
    match = actual == expected
    results.append((name, actual, expected, formula, match))

# 결과 출력
passed = sum(1 for r in results if r[4])
total = len(results)
tag = "ALIEN-10 달성!" if passed == total else "미달"
print(f"BT-373 한글/문자체계 검증: {passed}/{total} PASS {'<>' if passed == total else '!!'} {tag}")
print()
for name, actual, expected, formula, match in results:
    status = "PASS" if match else "FAIL"
    print(f"  {status}: {name} = {actual} (n6: {formula} = {expected})")
print()
# 추가 검증: 11,172 인수분해
print("--- 추가 검증: 한글 음절 인수분해 ---")
print(f"  19 x 21 x 28 = {19*21*28}")
print(f"  (J2-sopfr) x (J2-n/phi) x (J2+tau) = ({J2-sopfr}) x ({J2-n//phi}) x ({J2+tau}) = {(J2-sopfr)*(J2-n//phi)*(J2+tau)}")
print(f"  자음+모음 대칭: sigma+phi={sigma+phi}, sigma-phi={sigma-phi}, 합={sigma+phi+sigma-phi}=2*sigma={2*sigma}")
print(f"EXACT 비율: {passed}/{total} = {100*passed//total}%")
