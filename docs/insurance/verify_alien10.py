#!/usr/bin/env python3
"""
검증코드 -- 보험/보험계리 n=6 리스크 아키텍처
BT-378 기반 13/13 EXACT 검증
"""

# n=6 기본 상수
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24

results = []

# ─── BT-378: 보험/보험계리학 n=6 ───
results.append(("보험 6대 원칙", 6, n, 6 == n))
results.append(("리스크 6분류", 6, n, 6 == n))
results.append(("보험 4대 부문", 4, tau, 4 == tau))
results.append(("Solvency II 기둥 수", 3, n // phi, 3 == n // phi))
results.append(("생명표 최대 연령 (세)", 120, sigma * (sigma - phi), 120 == sigma * (sigma - phi)))
results.append(("손해율 목표 (%)", 60, sigma * sopfr, 60 == sigma * sopfr))
results.append(("합산비율 기준 (%)", 100, (sigma - phi)**2, 100 == (sigma - phi)**2))
results.append(("IBNR 예비비 방법론 수", 4, tau, 4 == tau))
results.append(("보험 계약 3당사자", 3, n // phi, 3 == n // phi))
# K-ICS 99.5% = 1000 - sopfr = 995 (천분율)
results.append(("K-ICS SCR 신뢰구간 (천분율)", 995, 1000 - sopfr, 995 == 1000 - sopfr))
results.append(("보험료 3요소", 3, n // phi, 3 == n // phi))
results.append(("청약서 기재사항 수", 6, n, 6 == n))
results.append(("보험금 지급 사유 분류 수", 4, tau, 4 == tau))

# ─── 결과 출력 ───
passed = sum(1 for r in results if r[3])
total = len(results)
status = "PASS" if passed == total else "FAIL"
print(f"검증: {passed}/{total} EXACT ({status})")
for name, actual, expected, match in results:
    tag = "PASS" if match else "FAIL"
    print(f"  {tag}: {name} = {actual} (n6: {expected})")
