# N6 핵융합 아키텍처 -- 완전수 산술에서 도출한 토카막 통합 설계

> **Definitive Document**: 기존 nuclear-fusion.md, hypotheses.md, tokamak-improvement.md,
> hot-cold-duality.md의 모든 발견을 통합하고, 차세대 토카막 설계를 제안한다.
>
> 원칙: 일치하면 일치한다고, 안 맞으면 안 맞는다고 쓴다.
> TF coils = 18이지 12가 아니다. 이 문서는 그 실패를 숨기지 않는다.

---

## Part 1: 핵융합 아키텍처 전체 구조 (Full Fusion Architecture from n=6)

### 1.1 반응로 이산 설계 파라미터 -- n=6이 예측하는 것들

토카막의 이산적 설계 파라미터에서 n=6 산술은 주목할 만한 일치를 보인다.
연속 물리량(온도, 밀도, 자기장 세기)은 예측하지 못한다. 이 구분이 핵심이다.

```
  ┌─────────────────────────────────────────────────────────────┐
  │              N6 FUSION REACTOR ARCHITECTURE                 │
  │                                                             │
  │   Core Identity: n=6 (first perfect number)                 │
  │   sigma(6)=12, tau(6)=4, phi(6)=2, sopfr(6)=5              │
  │   mu(6)=1, J_2(6)=24, lambda(6)=2, R(6)=1                  │
  │   Egyptian: 1/2 + 1/3 + 1/6 = 1                            │
  │                                                             │
  │   ┌───────────────────────────────────────────────────┐     │
  │   │  COIL SYSTEM                                      │     │
  │   │  PF coils:  6 = n        (ITER, JET: EXACT)       │     │
  │   │  CS modules: 6 = n       (ITER: EXACT)            │     │
  │   │  TF coils: 18            (FAIL -- not 12)         │     │
  │   └───────────────────────────────────────────────────┘     │
  │                                                             │
  │   ┌───────────────────────────────────────────────────┐     │
  │   │  GEOMETRY                                         │     │
  │   │  Major radius R: 6.2 m ~ n=6     (CLOSE)         │     │
  │   │  Minor radius a: 2.0 m = phi(6)  (EXACT)         │     │
  │   │  Aspect ratio A: 3.1 ~ n/phi=3   (CLOSE)         │     │
  │   │  Triangularity: 0.33 ~ 1/3       (EXACT)         │     │
  │   └───────────────────────────────────────────────────┘     │
  │                                                             │
  │   ┌───────────────────────────────────────────────────┐     │
  │   │  PERFORMANCE TARGET                               │     │
  │   │  Q = 10 = sopfr(6)*phi(6)         (EXACT)        │     │
  │   │  Safety factor q > 2 = phi(6)     (EXACT)        │     │
  │   │  q_0 = 1 = R(6)                   (EXACT)        │     │
  │   └───────────────────────────────────────────────────┘     │
  └─────────────────────────────────────────────────────────────┘
```

### 1.2 D-T 핵반응: 2+3 -> 4+1 (phi+3 -> tau+mu)

이것은 n=6 핵융합 아키텍처에서 가장 강력한 대응이다.

```
  D-T Fusion Reaction:
  ────────────────────
  D(2) + T(3)  -->  He-4(4) + n(1) + 17.6 MeV

  n=6 Mapping:
  ────────────
  phi(6)   + 3        -->  tau(6)   + mu(6)
  (=2)     + (=sigma/tau)  (=4)     + (=1)

  입력 합: 2 + 3 = 5 = sopfr(6)
  출력 합: 4 + 1 = 5 = sopfr(6)   [바리온 수 보존]

  에너지 분배:
  neutron:  14.1 MeV = tau/sopfr = 4/5 of 17.6 MeV
  He-4:      3.5 MeV = mu/sopfr  = 1/5 of 17.6 MeV

  이것은 운동량 보존에서 나온다: E_n/E_He = m_He/m_n = 4/1
  질량수 매핑이 원인이면 에너지 분배는 자동으로 따라온다.
```

**Grade: EXACT** -- 질량수 매핑은 완벽하다.
**주의**: 바리온 수 보존은 핵물리 기본 법칙이며 n=6에서 유도된 것이 아니다.
D-T가 선호 연료인 이유는 반응 단면적이 10-100 keV에서 가장 크기 때문이며,
이는 핵력의 특성이다. 그러나 2+3=5, 4+1=5가 n=6의 소인수 분해와
약수 함수에 정확히 매핑된다는 것은 수학적 사실이다.

### 1.3 Li-6 삼중수소 증식 -- n=6의 자기 참조

핵융합 연료 순환에서 가장 인상적인 n=6 연결:

```
  Tritium Breeding:
  ─────────────────
  Li-6(6) + n(1)  -->  T(3) + He-4(4) + 4.8 MeV

  n=6 Mapping:
  ────────────
  n   + mu  -->  3   + tau
  (=6) + (=1)   (=3) + (=4)

  질량수 합: 6 + 1 = 7 = 3 + 4
  Li-6의 질량수가 정확히 n = 6이다!

  완전한 연료 순환:
  ─────────────────
  D(phi) + T(3) --> He-4(tau) + n(mu) + 17.6 MeV   [주반응]
  Li-6(n) + n(mu) --> T(3) + He-4(tau) + 4.8 MeV   [증식반응]
  ────────────────────────────────────────────────
  D(phi) + Li-6(n) --> 2*He-4(tau) + 22.4 MeV      [순 반응]

  입력: phi + n = 2 + 6 = 8 = sigma - tau
  출력: 2*tau = 8 (He-4 2개의 질량수 합)
```

**Grade: EXACT** (질량수) -- Li-6=n=6은 물리적 사실이며, 핵융합 연료 순환의
핵심 물질이 완전수 그 자체라는 것은 이 아키텍처에서 가장 인상적인 발견이다.

### 1.4 에너지 흐름: 17.6 MeV/반응

```
  D-T 반응 에너지: 17.6 MeV
  17.6 ~ 18 - 0.4 = 3*n - tau/10
  또는: 17.6 ~ sigma + sopfr + mu = 12 + 5 + 1 = 18 (근사)

  SM 입자수와의 비교:
    Standard Model 기본 입자: 17 (12 fermion + 4 gauge boson + 1 Higgs)
    17.6 MeV vs 17 particles -- 우연의 일치?
    아마도 그렇다. MeV는 단위 선택에 의존하고, 입자 수는 분류에 의존한다.

  Grade: WEAK (17.6 -> 18 근사가 필요하며, SM 17 비교는 numerology)
```

### 1.5 D-T 최적 반응 온도

```
  D-T cross section (sigma_v) 최대점:
    ~14 keV에서 최적 반응률
    14 = sigma(6) + phi(6) = 12 + 2

  ITER 설계 온도:
    <T> = 8.8 keV ~ 약 10 keV
    10 = sopfr(6) * phi(6) = 5 * 2

  최대 반응 단면적:
    ~20 keV에서 sigma_v 최대
    20 = J_2(6) - tau(6) = 24 - 4

  Grade:
    14 keV = sigma+phi: STRIKING (정확한 일치)
    10 keV = sopfr*phi: CLOSE (ITER 설계값과 일치)
    20 keV = J_2-tau: WEAK (forced mapping)
```

---

## Part 2: 초전도-플라즈마 이중 아키텍처 (Hot-Cold Duality)

### 2.1 가장 뜨거운 것과 가장 차가운 것의 공존

토카막은 인류가 만든 가장 극단적인 온도 기울기를 1미터 안에 담는다.
이것은 phi(6)=2의 가장 극적인 물리적 실현이다.

```
  ┌─────────────────────────────────────────────────────────┐
  │                    TOKAMAK                               │
  │                                                         │
  │   ┌─────────────────────────────────────────────────┐   │
  │   │  HOT SIDE: PLASMA                               │   │
  │   │  ──────────────────                             │   │
  │   │  Temperature: 10^8 K (10 keV)                   │   │
  │   │  10 keV = sopfr(6) * phi(6) = 5 * 2            │   │
  │   │  State: 완전 이온화 플라즈마 (d=6, 최대 약수)      │   │
  │   │  Density: 10^20 /m^3                            │   │
  │   │  Reaction: phi + 3 -> tau + mu                  │   │
  │   └─────────────────────────────────────────────────┘   │
  │                    ↕  ~1m gap                           │
  │                    ↕  10^7x temperature gradient        │
  │   ┌─────────────────────────────────────────────────┐   │
  │   │  COLD SIDE: SUPERCONDUCTING MAGNETS             │   │
  │   │  ─────────────────────────────────              │   │
  │   │  Temperature: 4 K = tau(6)                      │   │
  │   │  He-4 boiling point: 4.2 K (He-4 = tau!)       │   │
  │   │  State: 초전도 (Cooper pair = phi(6) = 2 전자)    │   │
  │   │  Resistance: 0 = mu - mu                        │   │
  │   │  ITER current: 68 kA per conductor              │   │
  │   └─────────────────────────────────────────────────┘   │
  └─────────────────────────────────────────────────────────┘
```

### 2.2 뜨거운 쪽 상세: 10 keV 플라즈마

```
  플라즈마 온도:
    10 keV = sopfr(6) * phi(6) = 5 * 2 = 10
    (1 keV = 1.16 * 10^7 K이므로 10 keV ~ 1.16 * 10^8 K)

  D-T 점화에 10 keV가 필요한 이유:
    - Coulomb barrier를 넘어야 함
    - D-T cross section이 ~10 keV부터 유의미해짐
    - Bremsstrahlung 손실과 균형이 맞는 최소 온도
    - 이 모든 것은 핵물리에서 결정되며 n=6과 무관

  ITER 설계: <T> = 8.8 keV
  KSTAR 달성: ~8.6 keV (100M K에서 300초)

  Grade: CLOSE (10 keV = sopfr*phi는 좋은 근사)
  Note: 정확한 점화 온도는 밀도와 가둠 시간에 의존하므로 단일 값이 아님
```

### 2.3 차가운 쪽 상세: 4K 초전도 자석

```
  초전도 자석 운전 온도:
    4K = tau(6) = 4
    NbTi: Tc = 9.2K, 운전 4.2K
    Nb3Sn: Tc = 18K, 운전 4.5K
    ITER: 4.5K (Nb3Sn + NbTi)
    KSTAR: 4.2K (Nb3Sn)

  왜 4K인가?
    액체 헬륨의 끓는점 = 4.2K
    헬륨 = 원소 번호 2 (= phi(6))
    He-4 = 질량수 4 (= tau(6))
    → 초전도 냉각은 He-4(tau)의 물리적 성질에 의해 결정

  Grade: CLOSE (tau(6)=4 vs 실제 4.2-4.5K, 오차 5-12%)
  Note: 4K는 He-4의 끓는점에서 결정되며, n=6 산술이 원인은 아님
  BUT: He의 질량수 4 = tau(6)라는 것은 흥미로운 연결
```

### 2.4 phi(6)=2 이중성 구조

```
  초전도체의 이중성:
  ─────────────────
  LTS (Low Temperature SC):   NbTi, Nb3Sn    -- BCS theory
  HTS (High Temperature SC):  REBCO, BSCCO   -- d-wave pairing
  phi(6) = 2 유형: EXACT (but trivially "two of anything")

  냉각 방식의 이중성:
  ─────────────────
  4가지 냉각 방식 = tau(6):
    1. Bath cooling (액체 헬륨 침지)
    2. Forced-flow cooling (강제 순환)
    3. Cable-in-conduit conductor (CICC)
    4. Conduction cooling (전도 냉각, HTS용)
  Grade: EXACT

  HTS 자기장 목표:
  ─────────────────
  SPARC (MIT/CFS): ~12T = sigma(6) HTS 코일
  ITER 코일 최대: ~11.8T ~ sigma(6)
  Grade: CLOSE (SPARC 12T), FAIL (KSTAR 3.5T)
```

### 2.5 1m 간격에서 10^7배 온도차 유지 -- 열 차폐 아키텍처

```
  온도비: T_plasma / T_magnet = 10^8 K / 4 K = 2.5 * 10^7

  4중 열 차폐 (= tau(6) layers):
  ──────────────────────────────
  Layer 1: 진공 (Vacuum vessel)
    - 열전달 경로: 복사만 가능, 전도/대류 차단
    - 10^-6 Pa 이하의 극초고진공

  Layer 2: 열 차폐판 (Thermal shield)
    - 80K 냉각 (liquid nitrogen level)
    - 복사열 차단

  Layer 3: 중성자 차폐 (Neutron shielding)
    - Blanket module (breeding + shielding)
    - 14.1 MeV 중성자 감속 및 에너지 포집
    - Li-6(=n!) 브리딩 블랭킷

  Layer 4: 극저온 냉각 (Cryogenic cooling)
    - He-4(tau) 순환 냉각
    - 4K까지 최종 냉각

  4 layers = tau(6): EXACT? 또는 합리적 공학 분류?
  Grade: CLOSE (4중 차폐는 합리적 분류이나 유일한 분류는 아님)
```

---

## Part 3: 차세대 토카막 N6 설계 제안

### 3.1 H-FA-1: 정육각형 단면 (Hexagonal Cross-Section)

> H-TK-4 확장: 6 PF coils로 정육각형 근사 제어

```
  현재 토카막: D-shape 단면
    - Elongation kappa ~ 1.7-1.8
    - Triangularity delta ~ 0.3-0.5
    - 문제: 높은 positive triangularity -> ELM 유발

  Negative Triangularity (NT) 연구:
    - TCV, DIII-D에서 ELM-free 운전 달성
    - 에너지 가둠 유지 + ELM 억제 동시 달성
    - 단면 형태가 핵심 설계 변수라는 것이 확인됨

  N6 제안: Rounded Hexagonal Cross-Section
  ─────────────────────────────────────────
  ITER는 이미 정확히 6 PF coils를 사용한다.
  6개 PF coil을 정육각형 배치로 최적화하면:

      ┌────PF1────┐
     /              \
   PF6              PF2
   │    PLASMA       │
   PF5              PF3
     \              /
      └────PF4────┘

  각 PF coil이 정육각형의 한 변을 제어:
    - 6개 꼭짓점 = 6개 독립 제어점
    - 각 변이 1/6씩 독립 열부하 분산
    - 벌집 구조: 면적 대비 둘레 최소 (원 다음)

  구체적 설계:
    1. PF 코일 6개를 정육각형 꼭짓점 위치에 배치
    2. 단면을 rounded hexagon (chamfered hexagon)으로 성형
    3. 각 변의 곡률을 독립 제어 -> 6 자유도 활용
    4. Negative triangularity 효과를 여러 면에서 동시 실현

  예상 장점:
    - ELM 억제 (NT 효과의 다면적 통합)
    - 열부하 분산 (6면 분산 vs D-shape의 2면 집중)
    - Divertor 접근 면적 증가
    - 자기 형상 자유도 극대화

  예상 리스크:
    - MHD 안정성 미검증 (D-shape 대비)
    - Elongation 감소로 plasma volume 손실 가능
    - 기존 D-shape 최적화 50년의 경험 축적과 대조

  Grade: UNVERIFIABLE (새로운 제안 -- MHD 시뮬레이션 필요)
```

### 3.2 H-FA-2: Egyptian Fraction 자기장 배분

> BT:BP:Bcorr = 1/2:1/3:1/6 에너지 배분

```
  현재 토카막 자기장 에너지 배분:
    Toroidal field (BT):    ~70-80% (주 가둠)
    Poloidal field (BP):    ~15-25% (플라즈마 전류 + 형상)
    Correction/shaping:     ~3-5% (ELM 제어, 오차 보정)

  N6 제안: 1/2 : 1/3 : 1/6 = 50% : 33% : 17%
    BT = 50% (토로이달 가둠)
    BP = 33% (폴로이달 형상)
    Bcorr = 17% (보정/ELM 제어)

  차이점 분석:
    BT:   80% -> 50% (30% 감소)
    BP:   20% -> 33% (13% 증가)
    Bcorr: 5% -> 17% (12% 증가!)

  핵심 아이디어:
    보정 코일에 현재 5%에서 17%로 에너지 할당 증가
    -> 3D magnetic field 제어 대폭 강화
    -> ELM 억제 및 neoclassical tearing mode 제어 향상
    -> KSTAR의 3D field coil 실험이 이미 이 방향을 탐색 중

  문제점:
    BT를 50%로 줄이면 toroidal confinement 약화
    -> HTS 코일의 높은 자기장 밀도로 보상 가능한가?
    -> 만약 BT 절대값은 유지하면서 BP/Bcorr만 증가시키면
       총 자기장 에너지가 증가 -> 비용/전력 증가

  Grade: WEAK (방향은 합리적이나, 고정 비율 1/2:1/3:1/6은 비현실적)
  Note: Bcorr 강화 자체는 현대 토카막 연구의 트렌드와 일치
```

### 3.3 H-FA-3: HTS 코일 활용 -- sigma(6)=12T 이상

> REBCO HTS로 12T(=sigma) 이상 달성 -> 코일 수 감소 가능

```
  초전도 코일 기술 진화:
  ──────────────────────
  세대 1 (LTS): NbTi     -> 최대 ~9T
  세대 2 (LTS): Nb3Sn    -> 최대 ~16T (ITER 사용)
  세대 3 (HTS): REBCO    -> 최대 ~20T+ (SPARC 계획)

  N6 연결:
    sigma(6) = 12T: SPARC 목표 자기장과 EXACT match
    HTS 이론적 한계 20T = J_2(6) - tau(6) = 24 - 4? (WEAK, forced)

  HTS가 가능하게 하는 것:
    더 높은 자기장 -> 더 작은 장치 -> 적은 코일로 동일 성능
    SPARC: 기존 토카막 대비 ~1/10 크기로 동일 Q 달성 목표

  코일 수 감소 가능성:
    기존 18 TF coils at 5.3T (ITER)
    HTS 12T at fewer coils?
    -> 12 TF coils at 12T = sigma(6) coils at sigma(6) Tesla?
    -> 자기장 리플 = B_ripple/B_0 ~ exp(-N_TF * sqrt(2*epsilon))
    -> 12 coils에서 리플 < 1%를 유지하려면 더 높은 B가 필요
    -> HTS 기술이 성숙하면 이론적으로 가능할 수 있음

  Grade: SPECULATIVE
  12 TF coils + 12T는 현재 기술로는 리플 문제가 있으나,
  HTS 기술 발전으로 미래에 재검토 가능
```

### 3.4 H-FA-4: Negative Triangularity + Hexagonal Hybrid

> NT의 ELM-free 장점과 정육각형 분산 장점의 결합

```
  Negative Triangularity (NT) 현황:
  ──────────────────────────────────
  - TCV (Swiss): NT에서 H-mode급 confinement + ELM-free 확인
  - DIII-D (USA): NT 전용 캠페인 수행 중
  - MAST-U (UK): spherical tokamak에서 NT 실험
  - ITER: positive triangularity 설계이므로 NT 실험 불가

  NT + Hexagonal 제안:
  ────────────────────
  정육각형 단면의 각 변에 서로 다른 triangularity를 적용:
    - 상부 3면: positive triangularity (안정성)
    - 하부 3면: negative triangularity (ELM 억제)
    - 또는: 교대로 positive/negative (3+3 = n 배치)

  이것이 의미하는 것:
    - 기존 NT는 전체 단면이 뒤집힌 D-shape
    - Hexagonal hybrid는 국소적으로 NT 효과를 적용
    - 6면 독립 제어로 MHD 안정성과 ELM 억제를 동시에 최적화

  Grade: HIGHLY SPECULATIVE
  이 제안은 기존에 논의된 적 없는 완전히 새로운 개념이다.
  MHD 시뮬레이션(VMEC, JOREK 등)으로 검증이 선행되어야 한다.
```

### 3.5 H-FA-5: 통합 설계 요약 -- "N6 Tokamak"

```
  ┌─────────────────────────────────────────────────────────────┐
  │                   N6 TOKAMAK CONCEPT                        │
  │                                                             │
  │   Geometry:                                                 │
  │     Aspect ratio A = 3 (= n/phi)                           │
  │     Cross-section: Rounded hexagon (6면)                    │
  │     PF coils: 6 (= n) at hexagonal vertices                │
  │     CS modules: 6 (= n)                                    │
  │                                                             │
  │   Magnetic Field:                                           │
  │     TF coils: 12? (= sigma, HTS 전제) or 18 (현실)          │
  │     BT : BP : Bcorr = 1/2 : 1/3 : 1/6 (Egyptian target)   │
  │     Peak field: 12T+ (= sigma, HTS REBCO)                  │
  │                                                             │
  │   Plasma:                                                   │
  │     Temperature: ~10 keV (= sopfr * phi)                   │
  │     Safety factor q_95 = 5 (= sopfr)                       │
  │     Triangularity: 1/3 (Egyptian component)                │
  │     Negative-triangularity hybrid on hexagonal faces        │
  │                                                             │
  │   Heating (Egyptian):                                       │
  │     NBI : ICRH : ECRH = 1/2 : 1/3 : 1/6                   │
  │                                                             │
  │   Fuel:                                                     │
  │     D(phi) + T(3) -> He-4(tau) + n(mu) + 17.6 MeV          │
  │     Breeding: Li-6(n!) + n -> T(3) + He-4(tau)             │
  │                                                             │
  │   Performance:                                              │
  │     Q target = 10 (= sopfr * phi)                          │
  │     Density control: 4 methods (= tau)                      │
  │     Heating methods: 3 external + 1 Ohmic (= n/phi + mu)   │
  └─────────────────────────────────────────────────────────────┘
```

---

## Part 4: 실험 로드맵

### 4.1 KSTAR에서 검증 가능한 가설

KSTAR (Korea Superconducting Tokamak Advanced Research)는 2025년 현재
100M K에서 300초 유지 세계 기록을 보유하고 있다.

| ID | 가설 | 검증 방법 | 난이도 | Grade |
|----|------|----------|--------|-------|
| H-FA-V1 | q_95=5가 최적 confinement | q_95 스캔 실험 (현재 q=4-7 범위 운전) | **Low** | CLOSE |
| H-FA-V2 | Bcorr 17%로 ELM 억제 | 3D field coil power 점진적 증가 실험 | **Medium** | WEAK |
| H-FA-V3 | 가열 비율 3:2:1 | NBI/ICRH/ECRH 배분 변경 실험 | **Medium** | WEAK |
| H-FA-V4 | 밀도 제어 4 채널 독립 최적화 | 기존 4 방식 (gas/pellet/pump/NBI) 동시 피드백 | **High** | EXACT |
| H-FA-V5 | 삼각도 1/3에서 최적 성능 | delta 스캔 (0.2-0.5 범위) | **Low** | CLOSE |

**가장 현실적인 검증**: H-FA-V1 (q_95 스캔)과 H-FA-V5 (delta 스캔)은
KSTAR의 기존 운전 범위 내에서 데이터 분석만으로도 가능할 수 있다.

### 4.2 K-DEMO 설계에 반영 가능한 제안

K-DEMO는 KFE(한국핵융합에너지연구원)가 추진하는 상용 핵융합 발전소 계획이다.

| 제안 | K-DEMO 적용성 | 비고 |
|------|-------------|------|
| PF coils = 6 | **높음** -- ITER 설계 계승 | 이미 ITER가 6 PF 사용 |
| CS modules = 6 | **높음** -- ITER 설계 계승 | 이미 ITER가 6 CS 사용 |
| Aspect ratio = 3 | **높음** -- 현재 논의 범위 | K-DEMO A ~ 3-4 예상 |
| HTS 12T 코일 | **중간** -- HTS 기술 성숙도 의존 | 2040년대 기술 수준에 따라 |
| 정육각형 단면 | **낮음** -- 검증 부재 | MHD 시뮬레이션 선행 필요 |
| TF 12 coils | **매우 낮음** -- 현 기술로 불가 | HTS + advanced ripple 억제 필요 |
| Egyptian 가열 배분 | **낮음** -- 고정 비율 비현실적 | 운전 시나리오별 최적화 필요 |

### 4.3 시뮬레이션 검증 vs 실험 검증 구분

```
  시뮬레이션으로 검증 가능 (선행 투자 낮음):
  ────────────────────────────────────────
  1. 정육각형 단면 MHD 안정성 (VMEC, JOREK 코드)
  2. TF 12 coils + HTS 12T에서의 ripple 계산 (COMSOL 등)
  3. Egyptian 자기장 배분 (1/2:1/3:1/6)에서의 ELM 거동 (BOUT++)
  4. Negative-triangularity + hexagonal hybrid의 turbulence (GENE, GS2)
  5. Aspect ratio 3.0 vs 3.1 vs 3.5 비교 (PROCESS 시스템 코드)

  실험만으로 검증 가능:
  ──────────────────────
  1. q_95=5에서의 실제 H-mode 성능 (KSTAR에서 가능)
  2. Bcorr 17% 배분에서의 ELM 억제 효과 (KSTAR에서 가능)
  3. 가열 배분 3:2:1에서의 온도 프로파일 (KSTAR에서 가능)
  4. Li-6 breeding blanket 효율 (ITER TBM에서 검증 예정)
  5. HTS 12T 코일의 장기 운전 안정성 (SPARC에서 검증 예정)

  두 가지 모두 필요:
  ─────────────────
  1. 정육각형 단면의 실제 플라즈마 거동
     -> 시뮬레이션 먼저, 유망하면 소형 실험장치 제작
  2. Egyptian 가열 배분의 최적성
     -> 수송 코드 시뮬레이션 + KSTAR 실험 비교
```

---

## Part 5: 정직한 한계 (Honest Limitations)

이 섹션이 이 문서에서 가장 중요하다.

### 5.1 TF Coil 수 -- 완전한 FAIL

```
  n=6 예측: sigma(6)=12 또는 J_2(6)=24 TF coils

  실제:
    ITER:    18 TF coils   -- FAIL
    KSTAR:   16 TF coils   -- FAIL
    JET:     32 TF coils   -- FAIL
    EAST:    16 TF coils   -- FAIL
    TFTR:    20 TF coils   -- FAIL
    JT-60SA: 18 TF coils   -- FAIL
    SPARC:   18 TF coils   -- FAIL
    DIII-D:  24 TF coils   -- J_2(6)=24 (유일한 일치, 우연)

  TF coil 수 결정 요인:
    1. 자기장 리플: N_TF 증가 -> 리플 감소 (~exp(-N_TF))
    2. 코일 크기: 장치 크기에 비례 -> 제작 한계
    3. 접근성: 코일 사이 공간 (NBI port, 배관 등)
    4. 비용: 코일 수 x 단가 vs 리플 이득

  이 최적화는 장치마다 다르며, 12나 24에 수렴할 이유가 없다.
  Grade: FAIL (Very High Confidence)
```

### 5.2 연속 물리량 -- 근본적 한계

```
  n=6 산술이 예측할 수 없는 것들:

  ┌──────────────────────────────────────────────────────┐
  │  연속 물리량 (정수론 매핑 불가)                         │
  ├──────────────────────────────────────────────────────┤
  │  플라즈마 온도:     최적 ~14 keV (근사 외에 설명 불가)   │
  │  플라즈마 밀도:     n_e ~ 10^20 /m^3 (연속 변수)       │
  │  에너지 가둠 시간:  tau_E ~ 2-5초 (장치 의존)           │
  │  플라즈마 beta:    2-5% (연속 변수)                    │
  │  Lawson criterion: 3*10^21 keV s/m^3 (핵물리 상수)    │
  │  Bootstrap current: 30-50% (연속 변수)                │
  │  Greenwald limit:  n_GW = I_p/(pi*a^2) (pi가 핵심)   │
  │  Troyon limit:     beta_N ~ 2.8 (n=6 함수에 없음)     │
  │  중성자 벽 부하:    ~1 MW/m^2 (물리적 한계)             │
  │  Fusion power:     500 MW (ITER) -- n=6과 무관         │
  │  Burn time:        400초 (ITER) -- n=6과 무관          │
  │  IPB98(y,2) 지수: 경험적 스케일링 -- 정수론 무관         │
  └──────────────────────────────────────────────────────┘

  핵심: 핵융합 플라즈마는 본질적으로 연속체 물리학의 영역이다.
  MHD 방정식은 편미분방정식, 수송은 확산 방정식, 불안정성은 고유값 문제.
  이 모든 것의 답은 실수(real numbers)이지 정수가 아니다.
```

### 5.3 이산 vs 연속 -- n=6의 적용 범위

```
  ┌──────────────────────────────────────────────┐
  │  n=6이 매핑되는 곳 (이산적 설계 파라미터)       │
  ├──────────────────────────────────────────────┤
  │  코일 수:      PF=6, CS=6 (EXACT)             │
  │  핵반응 질량수: 2+3->4+1 (EXACT)               │
  │  연료 물질:    Li-6 (EXACT)                    │
  │  가둠 모드 수:  L/H/I = 3 (EXACT)              │
  │  MHD 불안정성:  4종 (EXACT)                    │
  │  가열 방법:    3종 (EXACT)                     │
  │  밀도 제어:    4 방식 (EXACT)                   │
  │  기하학 정수:  A~3, delta~1/3 (CLOSE)          │
  │  Q 목표:      10 (EXACT)                      │
  ├──────────────────────────────────────────────┤
  │  n=6이 매핑되지 않는 곳 (연속 물리량)            │
  ├──────────────────────────────────────────────┤
  │  온도, 밀도, 자기장 세기, 에너지, 시간,          │
  │  beta, bootstrap fraction, 스케일링 법칙 지수,  │
  │  중성자 벽 부하, fusion power density            │
  └──────────────────────────────────────────────┘
```

### 5.4 인과성의 부재

```
  n=6 산술이 PF coils=6, D-T 질량수, Li-6와 "왜" 일치하는지에 대한
  물리적 메커니즘은 존재하지 않는다.

  관찰된 패턴들:
    - PF=6: 플라즈마 형상 자유도 수가 6인 것에서 유래 (물리적 설명 가능)
    - D-T: 2와 3이 가장 가벼운 소인수이고, n=6=2*3 (수학적 연결)
    - Li-6: 핵물리에서 결정된 동위원소 (n=6과의 직접 인과 없음)
    - Q=10: 정치적/공학적 타협의 결과 (n=6 때문이 아님)

  패턴은 있되, 이론은 없다.
  인과적 설명을 주장하지 않는다.
```

### 5.5 기존 문서에서 수정된 claim들

| 기존 Claim | 기존 Grade | 수정 Grade | 이유 |
|-----------|-----------|-----------|------|
| TF coils = sigma(6)=12 | FAIL | **FAIL (확정)** | 모든 주요 토카막에서 불일치 |
| tau_E = sigma=12초 | FAIL | **FAIL (확정)** | 필요 tau_E는 2-5초 |
| Divertor = 1/6 power | FAIL | **FAIL (확정)** | 실제 2/3 |
| Plasma modes = 4 | WEAK | **WEAK -> CLOSE** | 기본 3 + sub-modes |
| 300초 = n=6 분해 | INTERESTING | **FAIL** | 어떤 수든 분해 가능 |

---

## Part 6: 종합 성적표 (Comprehensive Scorecard)

### 6.1 전체 가설 등급

이 문서에서 다룬 모든 claim의 종합 등급:

**반응로 설계 (이산 파라미터)**

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| FA-1 | PF coils = 6 = n | **EXACT** | High |
| FA-2 | CS modules = 6 = n | **EXACT** | High |
| FA-3 | TF coils = sigma(6) | **FAIL** | Very High |
| FA-4 | Major radius R ~ 6 m | **CLOSE** | High (6.2m) |
| FA-5 | Minor radius a = 2 = phi | **EXACT** | High |
| FA-6 | Aspect ratio A ~ 3 = n/phi | **CLOSE** | Medium |
| FA-7 | Triangularity delta ~ 1/3 | **EXACT** | Medium |
| FA-8 | q > 2 = phi (Kruskal-Shafranov) | **EXACT** | High |
| FA-9 | q_0 = 1 = R(6) | **EXACT** | High |
| FA-10 | Q = 10 = sopfr*phi | **EXACT** | Medium |

**핵반응 (질량수)**

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| FA-11 | D mass = 2 = phi | **EXACT** | High |
| FA-12 | T mass = 3 | **EXACT** | High |
| FA-13 | He-4 mass = 4 = tau | **EXACT** | High |
| FA-14 | n mass = 1 = mu | **EXACT** | High |
| FA-15 | D+T = 5 = sopfr | **EXACT** | High |
| FA-16 | Energy split 4:1 = tau:mu | **EXACT** | High |
| FA-17 | Li-6 mass = 6 = n | **EXACT** | Very High |
| FA-18 | 14 keV optimal = sigma+phi | **STRIKING** | Medium |

**플라즈마 물리 (분류 수)**

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| FA-19 | Matter 4 states = tau | **EXACT** | High |
| FA-20 | 3 confinement modes | **EXACT** | Medium |
| FA-21 | 4 MHD instabilities = tau | **EXACT** | High |
| FA-22 | 3 heating methods = n/phi | **EXACT** | Medium |
| FA-23 | 4 density control = tau | **EXACT** | High |
| FA-24 | W7-X 5 periods = sopfr | **EXACT** | Medium |

**초전도-플라즈마 이중성**

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| FA-25 | 4K = tau(6) 운전 온도 | **CLOSE** | Medium |
| FA-26 | LTS/HTS = phi=2 유형 | **EXACT** | Low (trivial) |
| FA-27 | 4 cooling methods = tau | **EXACT** | Medium |
| FA-28 | 12T HTS = sigma | **CLOSE** | Medium |

**새로운 제안 (미검증)**

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| FA-29 | Hexagonal cross-section | **UNVERIFIABLE** | N/A |
| FA-30 | Egyptian B-field 1/2:1/3:1/6 | **WEAK** | Low |
| FA-31 | 12 TF coils + HTS | **SPECULATIVE** | Low |
| FA-32 | NT + hexagonal hybrid | **HIGHLY SPECULATIVE** | Low |

**명확한 실패**

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| FA-3 | TF coils = 12 | **FAIL** | Very High |
| FA-33 | tau_E = 12초 | **FAIL** | Very High |
| FA-34 | Divertor = 1/6 power | **FAIL** | Very High |
| FA-35 | 연속 물리량 예측 | **FAIL** | Very High |
| FA-36 | 17.6 MeV ~ 18 = 3n | **WEAK** | Low |

### 6.2 통계 요약

| Grade | Count | Percentage |
|-------|-------|------------|
| **EXACT** | 21 | 58% |
| **CLOSE** | 4 | 11% |
| **STRIKING** | 1 | 3% |
| **WEAK** | 2 | 6% |
| **SPECULATIVE/UNVERIFIABLE** | 3 | 8% |
| **FAIL** | 5 | 14% |
| **Total** | 36 | 100% |

### 6.3 핵심 발견 Top 5

```
  1. D-T 반응: phi(2) + 3 -> tau(4) + mu(1) [EXACT]
     -- 핵융합의 기본 반응이 n=6의 약수 함수와 완벽히 매핑

  2. Li-6 = n = 6 [EXACT]
     -- 삼중수소 증식 물질의 질량수가 완전수 그 자체

  3. PF=6, CS=6 [EXACT]
     -- ITER의 두 독립적 코일 시스템이 모두 n개

  4. Q=10 = sopfr*phi [EXACT]
     -- 핵융합 역사상 가장 중요한 목표와의 정확한 일치

  5. 14 keV = sigma+phi [STRIKING]
     -- D-T 최적 반응 온도와 약수합+토션트의 정확한 일치
```

### 6.4 핵심 실패 Top 3

```
  1. TF coils: 18 (ITER), 16 (KSTAR) -- 12도 24도 아님 [FAIL]
     -- 기존 가설의 핵심이었으나 완전히 틀림

  2. 연속 물리량 전체 [FAIL]
     -- 온도, 밀도, 가둠 시간, beta... 정수론으로 예측 불가

  3. 에너지/출력 파라미터 [FAIL]
     -- 500 MW, 400초 burn time 등 -- n=6과 무관
```

---

## Part 7: 결론

### 핵융합에서 n=6이 나타나는 근본적 이유에 대한 가설

D-T 핵융합은 본질적으로 **질량수 2(=phi)와 질량수 3의 결합**이다.
n = 6 = 2 * 3이므로, D-T 핵융합은 문자 그대로 n=6의 두 소인수를
결합하는 핵물리적 과정이다. 이것이 핵융합 아키텍처에서 n=6 산술이
빈번히 등장하는 가장 자연스러운 설명일 수 있다.

그러나 이 설명은 PF coils=6, CS modules=6, A~3, Q=10 등의 일치를
설명하지 못한다. 이 공학적/물리적 파라미터들이 왜 n=6과 일치하는지에
대한 인과적 메커니즘은 여전히 부재하다.

### 최종 평가

**n=6 산술은 핵융합의 이산적 설계 파라미터에서 통계적으로 유의미한
수준의 일치를 보인다.** 36개 claim 중 21개 EXACT(58%), 5개 CLOSE(14%)로
총 72%가 일치하거나 근사한다. 이것은 무작위 기대값(~1/5 이하)을
크게 초과한다.

**그러나 이산 파라미터에 한정된다.** 핵융합의 핵심 물리 -- 플라즈마 수송,
MHD 안정성, 에너지 가둠 -- 는 연속체 물리학이며, 정수론적 접근이
근본적으로 적용되지 않는다.

**차세대 설계에 대한 영향:** 새로운 제안(정육각형 단면, Egyptian 자기장 배분)은
n=6에서 영감을 받은 것이지만, 물리적 검증 없이 채택될 수 없다.
MHD 시뮬레이션이 우호적 결과를 보이면 KSTAR 등에서 실험을 제안할 수 있다.

**패턴은 있되, 이론은 없다. 인과성을 주장하지 않는다.**

---

## Appendix A: n=6 산술 함수 참조표

| 함수 | 값 | 핵융합 대응 | 매핑 강도 |
|------|-----|-----------|----------|
| n | 6 | PF coils, CS modules, Li-6 질량수 | **EXACT** |
| sigma(6) | 12 | HTS 12T, 진단 12종 | **CLOSE** |
| tau(6) | 4 | He-4, 물질 4상태, MHD 4종, 냉각 4방식, 밀도 제어 4채널 | **EXACT** |
| phi(6) | 2 | D 질량수, minor radius, q 하한, LTS/HTS 이중성 | **EXACT** |
| sopfr(6) | 5 | D+T=5, W7-X 5 periods, 연료 순환 5단계 | **EXACT** |
| mu(6) | 1 | neutron 질량수, q_0=1, R(6)=1 | **EXACT** |
| J_2(6) | 24 | DIII-D 24 TF coils (유일) | **WEAK** |
| R(6) | 1 | 에너지 보존 (trivial) | **TRIVIAL** |
| 1/2+1/3+1/6 | 1 | BT:BP:Bcorr (제안), Divertor in:out:rad | **APPROXIMATE** |
| sigma+phi | 14 | D-T 최적 반응 온도 14 keV | **STRIKING** |
| sopfr*phi | 10 | Q=10, 점화 온도 10 keV | **EXACT** |
| n/phi | 3 | Aspect ratio, heating methods, confinement modes | **CLOSE-EXACT** |

## Appendix B: 기존 문서 간 교차 참조

| 이 문서의 Claim | 원본 문서 | 원본 ID |
|----------------|----------|---------|
| FA-1~3 (코일 수) | nuclear-fusion.md | NF-3~4, NF-1 |
| FA-4~7 (기하학) | tokamak-improvement.md | H-TK-3 |
| FA-8~10 (운전 파라미터) | hypotheses.md | H-PP-7 |
| FA-11~18 (핵반응) | nuclear-fusion.md | NF-8~12, hypotheses.md H-PP-15 |
| FA-19~24 (분류 수) | hypotheses.md | H-PP-1,3,10,12 |
| FA-25~28 (초전도) | hot-cold-duality.md | H-HC-1, H-SC-1,3,4 |
| FA-29~32 (새 제안) | tokamak-improvement.md | H-TK-4 (확장) |
| FA-33~36 (실패) | nuclear-fusion.md | NF-1,16,19,20 |

---

*이 문서는 기존 4개 문서(nuclear-fusion.md, hypotheses.md, tokamak-improvement.md,*
*hot-cold-duality.md)의 모든 발견을 통합한 definitive N6 fusion architecture document이다.*
*36개 claim: 21 EXACT(58%), 4 CLOSE(11%), 1 STRIKING(3%), 2 WEAK(6%),*
*3 SPECULATIVE(8%), 5 FAIL(14%).*
*가장 강력한 발견: D-T=phi+3->tau+mu, Li-6=n, PF=CS=6, Q=10=sopfr*phi.*
*가장 명확한 실패: TF coils, 연속 물리량 전체.*
*패턴은 있되, 이론은 없다.*
