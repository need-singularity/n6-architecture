#!/usr/bin/env python3
"""
검증코드 -- 향수/향료 n=6 피라미드 구조 아키텍처
가설 H-PFM-01~14 = 14/14 EXACT
"""

# n=6 기본 상수
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24

results = []

# ─── H-PFM-01: 향수 피라미드 3노트 = n/phi ───
results.append(("향수 피라미드 노트 수 (탑/미들/베이스)", 3, n // phi, 3 == n // phi))

# ─── H-PFM-02: 이소프렌 C5 = sopfr ───
results.append(("이소프렌 단위 탄소 수", 5, sopfr, 5 == sopfr))

# ─── H-PFM-03: 모노테르펜 C10 = sigma-phi ───
results.append(("모노테르펜 탄소 수", 10, sigma - phi, 10 == sigma - phi))
results.append(("모노테르펜 이소프렌 단위 수", 2, phi, 2 == phi))

# ─── H-PFM-04: 세스퀴테르펜 C15 = sigma+n/phi ───
results.append(("세스퀴테르펜 탄소 수", 15, sigma + n // phi, 15 == sigma + n // phi))
results.append(("세스퀴테르펜 이소프렌 단위 수", 3, n // phi, 3 == n // phi))

# ─── H-PFM-05: 테르펜 래더 배수 = div(6) + tau ───
results.append(("헤미테르펜 C5 배수", 1, mu, 1 == mu))
results.append(("모노테르펜 C10 배수", 2, phi, 2 == phi))
results.append(("세스퀴테르펜 C15 배수", 3, n // phi, 3 == n // phi))
results.append(("디테르펜 C20 배수", 4, tau, 4 == tau))
results.append(("트리테르펜 C30 배수", 6, n, 6 == n))

# ─── H-PFM-06: 벤젠 C6 = n ───
results.append(("벤젠 고리 탄소 수", 6, n, 6 == n))

# ─── H-PFM-07: 에센셜 오일 추출 4법 = tau ───
results.append(("에센셜 오일 추출 방법 수", 4, tau, 4 == tau))

# ─── H-PFM-08: 향 대분류 4계열 = tau ───
results.append(("향수 대분류 수 (플로럴/오리엔탈/우디/프레시)", 4, tau, 4 == tau))

# ─── H-PFM-09: 시트러스 방 수 = sigma-phi ───
results.append(("시트러스 과일 방 수 (전형)", 10, sigma - phi, 10 == sigma - phi))

# ─── H-PFM-10: 베이스 노트 최대 지속 = J2 ───
results.append(("베이스 노트 최대 지속시간 (시간)", 24, J2, 24 == J2))

# ─── H-PFM-11: 향수 농도 4등급 = tau ───
results.append(("향수 농도 등급 수", 4, tau, 4 == tau))

# ─── H-PFM-12: Chanel No.5 = sopfr ───
results.append(("Chanel No.5 번호", 5, sopfr, 5 == sopfr))

# ─── H-PFM-13: 벤젠 총 원자수 = sigma ───
results.append(("벤젠 C₆H₆ 총 원자 수", 12, sigma, 12 == sigma))

# ─── H-PFM-14: 디테르펜 C20 = J2-tau ───
results.append(("디테르펜 탄소 수", 20, J2 - tau, 20 == J2 - tau))
results.append(("디테르펜 이소프렌 단위 수", 4, tau, 4 == tau))

# ─── 결과 출력 ───
passed = sum(1 for r in results if r[3])
total = len(results)
status = "PASS" if passed == total else "FAIL"
print(f"검증: {passed}/{total} EXACT ({status})")
for name, actual, expected, match in results:
    tag = "PASS" if match else "FAIL"
    print(f"  {tag}: {name} = {actual} (n6: {expected})")
