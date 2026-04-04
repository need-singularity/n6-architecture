# 열관리 검증가능 예측 (Testable Predictions) --- 20개

> BT-60, BT-74, BT-89, BT-93 및 H-TM-01~30에서 도출.
> PUE, 냉각 효율, 열전달 상수 등의 검증가능한 예측.

---

## Tier 1: 즉시 검증 가능 (공개 데이터)

### TP-TM-01: ASHRAE PUE 목표 = sigma/(sigma-phi) = 1.2
**예측**: 업계 표준 PUE 목표 = 1.2.
**n=6 근거**: sigma/(sigma-phi) = 12/10 = 1.2. BT-60.
**검증**: ASHRAE TC 9.9, Google/Microsoft/Meta PUE 목표.
**반증 조건**: PUE 1.0이 업계 표준 목표가 되면 CLOSE.

### TP-TM-02: Google PUE = sigma/(sigma-mu) = 12/11 = 1.091
**예측**: Google fleet PUE ~ 1.09.
**n=6 근거**: sigma/(sigma-mu) = 12/11 = 1.0909. BT-60.
**검증**: Google 2024 환경 보고서: PUE = 1.09.
**반증 조건**: PUE가 1.15+로 악화되면 CLOSE.

### TP-TM-03: Immersion cooling PUE -> R(6) = 1.0 한계
**예측**: 2상 침지냉각의 PUE -> 1.0 = R(6).
**n=6 근거**: R(6) = 1 (이론적 하한). BT-60.
**검증**: LiquidStack, BitFury PUE = 1.02-1.04.
**반증 조건**: PUE < 1.0이 달성되면 FAIL (열역학 위반).

### TP-TM-04: CPU TDP 이진 스케일링 = phi=2 배
**예측**: CPU 세대별 TDP는 대략 2배씩 증가한다.
**n=6 근거**: phi=2. BT-45.
**검증**: Intel/AMD 세대별 TDP 추이.
**반증 조건**: 3배 이상 증가가 표준이 되면 CLOSE.

### TP-TM-05: DC 전력 체인 48V = sigma*tau
**예측**: 데이터센터 내부 배전 표준 = 48V.
**n=6 근거**: sigma*tau = 12*4 = 48. BT-60.
**검증**: OCP (Open Compute Project) 48V 표준.
**반증 조건**: 72V 또는 400V가 랙 내부 표준이 되면 CLOSE.

### TP-TM-06: 서버 팬 PWM = sigma=12V 기준
**예측**: 서버 냉각 팬 전압 = 12V = sigma.
**n=6 근거**: sigma=12.
**검증**: Intel/AMD 서버 팬 규격.
**반증 조건**: 5V 팬이 서버 표준이 되면 FAIL.

---

## Tier 2: 실험 검증 (장비)

### TP-TM-07: Heat pipe 구리 관 직경 = n=6 mm 범위
**예측**: 노트북/서버 heat pipe 직경 = 6mm 근방.
**n=6 근거**: n=6.
**검증**: Cooler Master, Noctua 히트파이프 스펙.
**반증 조건**: 10mm+가 표준이 되면 CLOSE.

### TP-TM-08: TIM 두께 최적 = 0.1mm = 1/(sigma-phi)
**예측**: 열전도 재료(TIM) 최적 두께 ~ 0.1mm.
**n=6 근거**: 1/(sigma-phi) = 0.1.
**검증**: Intel/AMD IHS TIM 스펙.
**반증 조건**: 0.05mm 미만이 표준이 되면 CLOSE.

### TP-TM-09: 열전 소재 ZT > 1 = R(6) 목표
**예측**: 열전 소재의 figure of merit 목표 ZT > 1 = R(6).
**n=6 근거**: R(6) = 1.
**검증**: Bi₂Te₃, PbTe, SnSe 최신 ZT 값.
**반증 조건**: ZT < 0.5가 상용 한계로 확정되면 CLOSE.

### TP-TM-10: 냉각수 유량 = J₂=24 LPM 범위
**예측**: 서버 랙 수냉 유량 ~ 24 LPM 범위.
**n=6 근거**: J₂ = 24.
**검증**: CDU (Coolant Distribution Unit) 스펙.
**반증 조건**: 유량이 10 LPM 미만이면 CLOSE.

---

## Tier 3: 전문 연구

### TP-TM-11: Stefan-Boltzmann 복사 T^4 = tau 멱법칙
**예측**: 열복사 파워가 T^4에 비례하며, 지수 = tau(6) = 4.
**n=6 근거**: tau = 4.
**검증**: Stefan-Boltzmann 법칙 (물리학 정리).
**반증 조건**: 이것은 물리 법칙이므로 반증 불가.

### TP-TM-12: Fourier 열전도 = phi=2차원 편미분방정식
**예측**: 열전도 방정식은 2차 편미분방정식이다.
**n=6 근거**: phi = 2 (2차 도함수).
**검증**: 물리학 교과서.
**반증 조건**: 반증 불가 (물리 법칙).

### TP-TM-13: 3상 냉각 (공기/물/냉매) = n/phi
**예측**: 데이터센터 냉각 매체 = 3종류 (공기, 물, 냉매).
**n=6 근거**: n/phi = 3.
**검증**: ASHRAE DC 냉각 가이드라인.
**반증 조건**: 4번째 매체가 표준이 되면 CLOSE.

### TP-TM-14: 물 비열 4.18 kJ/(kg*K) ~ tau
**예측**: 물의 비열이 tau=4 근방이다.
**n=6 근거**: tau = 4.
**검증**: 물의 비열 = 4.184 kJ/(kg*K).
**반증 조건**: 반증 불가 (물리 상수).

### TP-TM-15: Carnot 효율 한계 = 1-T_cold/T_hot
**예측**: 열 -> 일 변환의 이론 상한은 Carnot 효율이다.
**n=6 근거**: 열역학 제2법칙 (n=6 독립).
**검증**: 물리학 정리.
**반증 조건**: 반증 불가.

---

## Tier 4: 미래 예측

### TP-TM-16: 2030 DC PUE < 1.1 = sigma/(sigma-mu) 근방
**예측**: 2030년 대형 DC 평균 PUE < 1.1.
**n=6 근거**: sigma/(sigma-mu) = 12/11 = 1.091.
**검증**: Uptime Institute 연간 보고서.

### TP-TM-17: 광자 칩 PUE -> 1.02 = R(6) + 0.02
**예측**: 광자 컴퓨팅 기반 DC PUE ~ 1.02. BT-89.
**n=6 근거**: R(6) = 1 + 보조 냉각.
**검증**: 광자 칩 상용화 시 (2030+).

### TP-TM-18: 48V DC 배전 보급률 > 50%
**예측**: 신규 DC의 48V 배전 채택률 > 50% (2028).
**n=6 근거**: sigma*tau = 48.
**검증**: OCP Summit 보고서.

### TP-TM-19: AI DC TDP/rack > 100kW
**예측**: AI 전용 DC 랙 당 전력이 100kW+에 도달한다.
**n=6 근거**: 100 = (sigma-phi)^phi = 10^2.
**검증**: NVIDIA DGX, AMD MI300 랙 스펙.

### TP-TM-20: 냉각 에너지 비율 < 5% = sopfr%
**예측**: 최첨단 DC의 냉각 에너지 비율 < 5%.
**n=6 근거**: sopfr = 5.
**검증**: PUE 1.05에서 냉각 = (1.05-1)/1.05 = 4.76% ~ sopfr%.
