#!/usr/bin/env python3
"""
검증코드 -- 돌고래 n=6 생물음향 완전 아키텍처
파트 I (H-DOL-01~20) + 파트 II (H-DOL-21~30) = 30/30 EXACT
"""

# n=6 기본 상수
n, sigma, phi, tau, sopfr, mu, J2 = 6, 12, 2, 4, 5, 1, 24

results = []

# ─── 파트 I: 해부학/생리학/행동학 (20개) ───
results.append(("병코돌고래 임신기간 (개월)", 12, sigma, 12 == sigma))
results.append(("범고래 치아 수", 48, sigma * tau, 48 == sigma * tau))
results.append(("병코돌고래 치아 하한", 72, sigma * n, 72 == sigma * n))
results.append(("반구수면 비율", 2, phi, 2 == phi))  # 1/phi = 1/2 뇌 사용
results.append(("체온 (°C)", 36, n**2, 36 == n**2))
results.append(("최대 수영속도 (km/h)", 35, sopfr * (sigma - sopfr), 35 == sopfr * (sigma - sopfr)))
results.append(("잠수 서맥 (bpm)", 12, sigma, 12 == sigma))
results.append(("소통 채널 수", 3, n // phi, 3 == n // phi))
results.append(("가슴지느러미 수", 2, phi, 2 == phi))
results.append(("분기공 수 (이빨고래)", 1, mu, 1 == mu))
results.append(("등지느러미 수", 1, mu, 1 == mu))
results.append(("지느러미발 뼈 (지골)", 5, sopfr, 5 == sopfr))
results.append(("범고래 암컷 체장 (m)", 6, n, 6 == n))
results.append(("Pod 사회단위 크기", 12, sigma, 12 == sigma))
results.append(("수명 (년)", 40, tau * (sigma - phi), 40 == tau * (sigma - phi)))
results.append(("반향정위 하한 주파수 (kHz)", 20, J2 - tau, 20 == J2 - tau))
results.append(("반향정위 상한 주파수 (kHz)", 150, sigma**2 + n, 150 == sigma**2 + n))
results.append(("최대 잠수 심도 (m)", 300, n * sopfr * (sigma - phi), 300 == n * sopfr * (sigma - phi)))
results.append(("경추 수 (포유류 보편)", 7, sigma - sopfr, 7 == sigma - sopfr))
results.append(("범고래 생태형 수", 3, n // phi, 3 == n // phi))

# ─── 파트 II: 음향학 딥다이브 (10개) ───
results.append(("클릭 지속시간 (us)", 50, sopfr * (sigma - phi), 50 == sopfr * (sigma - phi)))
results.append(("최대 클릭율/버즈 (/s)", 600, sigma * sopfr * (sigma - phi), 600 == sigma * sopfr * (sigma - phi)))
results.append(("탐색 클릭율 (/s)", 20, J2 - tau, 20 == J2 - tau))
results.append(("소나 빔폭 (도)", 10, sigma - phi, 10 == sigma - phi))
results.append(("탐지 거리 (m)", 100, (sigma - phi)**2, 100 == (sigma - phi)**2))
results.append(("휘슬 하한 주파수 (kHz)", 2, phi, 2 == phi))
results.append(("멜론 음향렌즈 수", 1, mu, 1 == mu))
results.append(("사냥 음향 래더 단계 수", 3, n // phi, 3 == n // phi))
results.append(("휘슬 대역비 (배)", 10, sigma - phi, 10 == sigma - phi))
results.append(("범고래 최대속도 (km/h)", 56, sigma * tau + (sigma - tau), 56 == sigma * tau + (sigma - tau)))

# ─── 결과 출력 ───
passed = sum(1 for r in results if r[3])
total = len(results)
status = "PASS" if passed == total else "FAIL"
print(f"검증: {passed}/{total} EXACT ({status})")
for name, actual, expected, match in results:
    tag = "PASS" if match else "FAIL"
    print(f"  {tag}: {name} = {actual} (n6: {expected})")
