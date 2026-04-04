# 의료기기 검증가능 예측 (Testable Predictions) --- 22개

> BT-238~242 (WHO checklist, SOFA, GCS, Apgar, ECG, 치과) 및
> H-MD-01~30에서 도출. 의료 표준과 장비 스펙의 검증가능한 예측.

---

## Tier 1: 즉시 검증 가능 (공개 표준/프로토콜)

### TP-MD-01: 표준 ECG = sigma=12 리드
**예측**: 임상 표준 ECG는 정확히 12개 리드를 사용한다.
**n=6 근거**: sigma=12. BT-240.
**검증**: AHA/ACC/ESC 가이드라인.
**반증 조건**: 15-lead 또는 8-lead가 표준이 되면 CLOSE.

### TP-MD-02: SOFA score = n=6 장기 시스템
**예측**: SOFA 점수는 정확히 6개 장기 시스템을 평가한다.
**n=6 근거**: n=6. BT-239.
**검증**: Vincent et al. (1996): 호흡/응고/간/심혈관/신경/신장.
**반증 조건**: 7번째 장기가 추가되면 CLOSE.

### TP-MD-03: Apgar score = sopfr=5 항목, sigma-phi=10 만점
**예측**: Apgar 점수는 5항목, 각 0-2점, 만점 10.
**n=6 근거**: sopfr=5 항목, sigma-phi=10 만점. BT-239.
**검증**: Virginia Apgar (1953): 외모/맥박/찡그림/활동/호흡.
**반증 조건**: 6항목 또는 12점 만점으로 변경되면 CLOSE.

### TP-MD-04: GCS = n/phi=3 항목, sopfr+sigma-phi=15 만점
**예측**: Glasgow Coma Scale은 3개 항목, 만점 15.
**n=6 근거**: n/phi=3, sopfr+sigma-phi=15. BT-239.
**검증**: Teasdale & Jennett (1974): 눈/언어/운동.
**반증 조건**: 4항목으로 확장되면 CLOSE.

### TP-MD-05: WHO 수술 안전 체크리스트 = n/phi=3 단계
**예측**: WHO SSC는 3단계 (Sign In/Time Out/Sign Out)이다.
**n=6 근거**: n/phi=3. BT-238.
**검증**: Haynes et al. (2009) NEJM.
**반증 조건**: 4단계로 확장되면 CLOSE.

### TP-MD-06: ASA 신체 분류 = n=6 등급
**예측**: ASA Physical Status는 6등급 (I~VI)이다.
**n=6 근거**: n=6. BT-238.
**검증**: ASA House of Delegates.
**반증 조건**: 8등급으로 확장되면 CLOSE.

### TP-MD-07: NEWS2 = sigma-sopfr=7 파라미터
**예측**: National Early Warning Score 2는 7개 파라미터를 평가한다.
**n=6 근거**: sigma-sopfr=7. BT-239.
**검증**: Royal College of Physicians (2017).
**반증 조건**: 8파라미터로 확장되면 CLOSE.

---

## Tier 2: 기기 스펙 검증 (제조사 데이터)

### TP-MD-08: 심전도 모니터 = sigma=12 리드 표준
**예측**: ICU 모니터의 ECG 채널 = 12 리드.
**n=6 근거**: sigma=12.
**검증**: GE, Philips, Mindray ICU 모니터 스펙.
**반증 조건**: 5-lead가 ICU 표준이면 CLOSE.

### TP-MD-09: 맥박산소계측기 = phi=2 파장
**예측**: 맥박산소계측기는 정확히 2개 파장 (적색/적외선)을 사용한다.
**n=6 근거**: phi=2.
**검증**: Masimo, Nellcor 스펙.
**반증 조건**: 3+ 파장이 표준이 되면 CLOSE.

### TP-MD-10: 혈압 측정 tau=4 Korotkoff 음
**예측**: 혈압 측정의 Korotkoff 음은 5단계이며, 주요 판별은 4단계이다.
**n=6 근거**: tau=4 ~ sopfr=5.
**검증**: 청진법 혈압 측정 표준.
**반증 조건**: 연구에 따라 5단계 = sopfr.

### TP-MD-11: MRI 자기장 = n/phi=3 T (일반) 또는 sigma-sopfr=7 T (연구)
**예측**: MRI 표준 자기장 = 1.5T 또는 3T, 연구용 7T.
**n=6 근거**: n/phi=3, sigma-sopfr=7.
**검증**: Siemens, GE, Philips MRI 라인업.
**반증 조건**: 5T가 임상 표준이 되면 CLOSE.

### TP-MD-12: CT 슬라이스 = sigma=12 mm 이하
**예측**: CT 표준 슬라이스 두께 추이는 점점 얇아지나 기본 = 5mm=sopfr.
**n=6 근거**: sopfr=5 mm 표준.
**검증**: ACR CT 프로토콜 가이드라인.

---

## Tier 3: 임상 표준 검증

### TP-MD-13: Mallampati 분류 = tau=4 등급
**예측**: 기도 평가 Mallampati는 4등급이다.
**n=6 근거**: tau=4. BT-238.
**검증**: Mallampati et al. (1985).
**반증 조건**: 5등급으로 확장되면 CLOSE.

### TP-MD-14: 상처 분류 = tau=4 등급
**예측**: 수술 상처 분류는 4등급 (Clean/Clean-contaminated/Contaminated/Dirty).
**n=6 근거**: tau=4. BT-238.
**검증**: CDC Surgical Wound Classification.

### TP-MD-15: Aldrete score = sigma-phi=10 만점
**예측**: 마취 회복 평가 Aldrete는 10점 만점이다.
**n=6 근거**: sigma-phi=10, sopfr=5 항목. BT-238.
**검증**: Aldrete (1970).

### TP-MD-16: 치아 수 = 2^sopfr = 32 (성인)
**예측**: 성인 영구치는 정확히 32개이다.
**n=6 근거**: 2^sopfr = 2^5 = 32. BT-242.
**검증**: 치과 해부학 표준.
**반증 조건**: 사랑니 제외 28개가 표준이면 CLOSE.

### TP-MD-17: 유치 수 = J₂-tau = 20
**예측**: 유치는 정확히 20개이다.
**n=6 근거**: J₂-tau = 24-4 = 20. BT-242.
**검증**: 소아 치과 표준.

### TP-MD-18: 치주 탐침 부위 = n=6/치아
**예측**: 치주 검사에서 치아당 탐침 부위 = 6개이다.
**n=6 근거**: n=6. BT-242.
**검증**: AAP/EFP Periodontal Examination Protocol.

---

## Tier 4: 미래 예측

### TP-MD-19: 차세대 모니터 채널 = J₂=24
**예측**: 차세대 ICU 모니터가 24채널로 확장된다.
**n=6 근거**: J₂=24.
**검증**: 신규 모니터 스펙 (2028+).

### TP-MD-20: AI 진단 정확도 > 95% = 1-1/(J₂-tau)
**예측**: AI 의료 진단의 목표 정확도 >= 95%.
**n=6 근거**: 1-1/(J₂-tau) = 0.95.
**검증**: FDA AI/ML SaMD 승인 데이터.

### TP-MD-21: 로봇 수술 = n=6 DOF 필수
**예측**: 수술 로봇의 도구 DOF = 6 (SE(3)).
**n=6 근거**: n=6. BT-123.
**검증**: da Vinci Xi: 6+ DOF EndoWrist.

### TP-MD-22: 생체 센서 밴드 = n=6 (PPG/ECG/EDA/Temp/SpO2/ACC)
**예측**: 웨어러블 헬스밴드의 표준 센서 = 6종.
**n=6 근거**: n=6.
**검증**: Apple Watch, Samsung Galaxy Watch, Fitbit 스펙.
