# 토카막 개선 아이디어 — N6 관점

> KSTAR 도넛형 플라즈마 용기의 n=6 기반 개선 가설

---

## 현재 토카막의 한계

```
  토카막 = 도넛(토러스) 형태의 자기 용기
  문제: 플라즈마가 벽에 닿으면 즉시 냉각 → confinement 유지가 핵심

  KSTAR: 300초 @ 1억도 (2024) — 세계 최고
  ITER: 목표 Q=10 (입력 에너지의 10배 출력)
  한계: ELM(Edge Localized Mode), disruption, 열부하
```

---

## N6 기반 토카막 개선 가설

### H-TK-1: 이집트 분수 자기장 구조 (1/2 + 1/3 + 1/6 = 1)

> 토로이달/폴로이달/보정 자기장의 에너지 배분을 1/2:1/3:1/6으로 최적화

```
  현재 토카막:
    - Toroidal field (BT): 주 가둠 (~70-80% 에너지)
    - Poloidal field (BP): 플라즈마 전류 (~15-25%)
    - Correction/shaping: ELM 제어 (~5%)

  N6 제안:
    BT : BP : B_correction = 1/2 : 1/3 : 1/6 = 3:2:1 에너지비

  장점: 보정 코일에 더 많은 에너지(1/6 ≈ 16.7%) 할당
  → ELM 제어 강화 → disruption 감소
  → 현재 대부분의 tokamak은 correction field에 5% 미만 할당

  검증: KSTAR의 3D field coil power를 16.7%로 올려 ELM 억제 실험
  Grade: WEAK (실제 최적 비율은 플라즈마 조건에 따라 다름)
```

### H-TK-2: 안전계수 q = sopfr(6) = 5

> edge safety factor q_95 = 5가 최적 confinement을 제공

```
  현재:
    - KSTAR: q_95 ≈ 4-7 범위에서 운전
    - ITER: q_95 ≈ 3 (디자인 값)
    - q < 2에서 kink instability 발생 (Kruskal-Shafranov limit)
    - q가 너무 높으면 confinement 감소

  N6 예측: q_95 = sopfr(6) = 5 가 sweet spot

  근거:
    - q = 2 (phi): kink limit (최소)
    - q = 3 (n/phi): ITER 디자인
    - q = 5 (sopfr): 제안 최적값
    - q > 5: confinement 저하

  검증: KSTAR에서 q_95=5 운전 시 H-mode confinement 측정
  Grade: CLOSE (q=4-5 범위가 실제로 좋은 결과를 보임)
```

### H-TK-3: 종횡비 A = n/phi = 3

> Aspect ratio R₀/a = 3이 토카막 최적 설계

```
  실제값:
    - KSTAR: A = 3.6
    - ITER: A = 3.1
    - DEMO: A ≈ 3-4 예상
    - Spherical tokamak (NSTX): A ≈ 1.3-1.5

  N6 예측: A = n/phi = 6/2 = 3

  ITER(3.1)와 ~3% 차이 — CLOSE match
  KSTAR(3.6)와 ~20% 차이 — WEAK

  물리적 이유: A ≈ 3은 beta limit과 bootstrap current의 균형점
  → n=6에서 독립적으로 도출 가능한지는 불확실

  Grade: CLOSE (A=3은 실제 최적 범위의 하한)
```

### H-TK-4: 도넛 → 6각형 단면 (Hexagonal Cross-Section)

> **혁신 제안**: 플라즈마 단면을 기존 D-shape에서 정육각형 근사로 변경

```
  현재: D-shape (elongation κ ≈ 1.7-1.8, triangularity δ ≈ 0.3-0.5)
  문제: 높은 triangularity는 ELM을 유발

  Negative triangularity (NT): 최근 연구에서 ELM-free 운전 달성
  → 단면 모양이 핵심 설계 변수

  N6 아이디어: n=6의 정육각형은 평면 충전 최적 도형
  - 벌집 구조: 면적 대비 둘레가 최소 (원 다음으로)
  - 6개 꼭짓점 = 6 PF coil 제어점

  구체적 제안:
  1. PF 코일 6개를 정육각형 배치 (ITER는 이미 6 PF!)
  2. 단면을 rounded hexagon으로 성형
  3. 각 변이 1/6씩 독립 제어 → Egyptian fraction 열분산

  예상 장점:
  - ELM 억제 (negative triangularity 효과 통합)
  - 열부하 분산 (6면 → 각 면이 1/6씩 담당)
  - Divertor 접근 면적 증가

  Grade: UNVERIFIABLE (새로운 제안 — 시뮬레이션 필요)
  Risk: 기존 D-shape 대비 MHD 안정성 미검증
```

### H-TK-5: 12코일 토로이달 자기장 (σ(6) = 12)

> TF 코일 수 12가 비용-성능 최적

```
  실제:
    - ITER: 18 TF coils
    - KSTAR: 16 TF coils
    - JET: 32 TF coils
    - SPARC (MIT): 18 TF coils

  N6 예측: σ = 12 TF coils

  문제: 실제로 12는 적음. TF coil 수를 줄이면:
    장점: 비용 절감, 유지보수 접근성 향상 (포트 공간)
    단점: ripple 증가 → 빠른 이온 손실 → confinement 저하

  현실: HTS (고온 초전도) 코일이 더 강하면 적은 수로 가능
  SPARC의 HTS 코일은 12T 이상 → 기존 18개에서 줄일 가능성

  Grade: FAIL (현재 기술로는 12개로 충분한 성능 불가)
  미래: HTS 기술이 성숙하면 재검토 가능
```

### H-TK-6: 플라즈마 가열 3방식 최적 배분

> NBI : ICRH : ECRH = 1/2 : 1/3 : 1/6 (Egyptian fraction)

```
  현재 ITER 가열 계획:
    NBI: 33 MW
    ICRH: 20 MW
    ECRH: 20 MW
    비율: 33:20:20 ≈ 45:27:27

  N6 제안: 1/2:1/3:1/6 = 3:2:1 비율
    총 73 MW 기준: NBI=36.5, ICRH=24.3, ECRH=12.2 MW

  차이: ECRH를 현재 20→12 MW로 줄이고 NBI 증가
  물리적 이유: NBI가 가장 효율적인 bulk heating
  → 이집트 분수 배분이 실제로 합리적일 수 있음

  Grade: WEAK (실제 최적은 플라즈마 시나리오에 따라 다름)
```

---

## 핵융합 1억도 유지 — n=6 confinement 가설

### H-TK-7: tau_E 최소 요건 = sigma(6) = 12초

> 상용 핵융합에 필요한 에너지 가둠 시간 τ_E ≈ 12초

```
  Lawson criterion: n·T·τ_E ≥ 5×10²¹ m⁻³·keV·s

  ITER 조건: n = 10²⁰ m⁻³, T = 10 keV
  필요 τ_E = 5×10²¹ / (10²⁰ × 10) = 5초

  DEMO (상용): n = 1.5×10²⁰, T = 15 keV
  필요 τ_E = 5×10²¹ / (1.5×10²⁰ × 15) = 2.2초

  N6 예측: τ_E = σ = 12초 — 너무 큼

  BUT: KSTAR 300초 달성은 "총 유지 시간"이지 τ_E가 아님
  τ_E와 총 운전 시간은 다른 개념

  Grade: FAIL (필요 τ_E는 2-5초, 12초는 과대)
  Note: 12초의 τ_E가 달성되면 매우 높은 Q 가능 (좋은 것!)
```

### H-TK-8: 플라즈마 밀도 제어 — tau(6) = 4 피드백 루프

> 밀도 제어에 4개 독립 피드백 루프 필요

```
  현재 밀도 제어:
    1. Gas puffing (주입)
    2. Pellet injection (고체 연료)
    3. Pumping (배기)
    4. NBI fueling (부산물)

  실제로 4가지 방법 = tau(6) = 4 (EXACT match!)

  Grade: EXACT
```

---

## 정직한 요약

| ID | 가설 | Grade | 비고 |
|----|------|-------|------|
| H-TK-1 | 자기장 1/2:1/3:1/6 배분 | WEAK | 보정 코일 강화 아이디어는 합리적 |
| H-TK-2 | q_95 = 5 | CLOSE | 실제 좋은 범위 |
| H-TK-3 | A = 3 | CLOSE | ITER 3.1, KSTAR 3.6 |
| H-TK-4 | 정육각형 단면 | UNVERIFIABLE | 혁신적이나 검증 필요 |
| H-TK-5 | 12 TF coils | FAIL | 현 기술로 부족 |
| H-TK-6 | 가열 3:2:1 배분 | WEAK | 방향은 맞으나 고정 비율은 비현실적 |
| H-TK-7 | τ_E = 12초 | FAIL | 필요값은 2-5초 |
| H-TK-8 | 밀도 제어 4 방식 | EXACT | 실제 4가지 방법 |

**가장 흥미로운 제안: H-TK-4 (정육각형 단면)**
- ITER가 이미 6 PF coils 사용
- Negative triangularity 연구와 결합 가능
- MHD 시뮬레이션으로 검증 가능한 구체적 제안
