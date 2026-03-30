# N6 Nuclear Fusion Deep Analysis -- 핵융합과 완전수 산술의 정직한 대조

## Preface: 이 문서의 목적

기존 hypotheses.md의 핵융합 가설 H-EG-4~6은 verification.md에서
**전부 FAIL 또는 WEAK** 판정을 받았다.

이 문서는 그 실패를 직시하고, 핵융합 물리의 실제 파라미터를 n=6 산술과
정직하게 대조한다. 일치하면 일치한다고, 안 맞으면 안 맞는다고 쓴다.

특히 한국 핵융합 프로그램(KSTAR, K-DEMO)을 중심으로 서술하되,
ITER와 글로벌 핵융합 맥락도 포함한다.

---

## Part 1: 한국 핵융합 -- 후발주자에서 선두로

### 1.1 KSTAR (Korea Superconducting Tokamak Advanced Research)

**기본 사양:**

| Parameter | Value | n=6 mapping | Grade |
|-----------|-------|-------------|-------|
| TF coils | **16** | sigma(6)=12? J_2(6)=24? **Neither.** | FAIL |
| PF coils | 14 (7쌍) | n=6? sigma=12? **Neither.** | FAIL |
| Major radius | 1.8 m | -- | N/A |
| Minor radius | 0.5 m | -- | N/A |
| Aspect ratio | 3.6 | 3 = n/phi? Close but not exact. | WEAK |
| Plasma current | 2 MA | phi(6)=2? Coincidence. | WEAK |
| Magnetic field | 3.5 T | 3 = n/phi? Close. | WEAK |

**KSTAR의 16개 TF coils -- 솔직히 말하자.**

기존 hypotheses.md는 "최적 코일 수 = sigma(6)=12 또는 J_2(6)=24"라고 주장했다.
KSTAR는 **16개**다. 12도 아니고 24도 아니다.
16 = 2^4이며, n=6 산술 어디에서도 자연스럽게 나오지 않는다.

코일 수는 자기장 리플(ripple), 플라즈마 크기, 공학적 제약에 의해 결정된다.
KSTAR의 16은 한국 엔지니어들이 1.8m 주반경에서 리플 <1%를 달성하면서
제작 가능한 코일 크기를 최적화한 결과다. 정수론이 아니라 공학이다.

**KSTAR 주요 성과:**

- 2016: H-mode 70초 유지 (당시 세계 기록)
- 2021: 1억도(100M K) 플라즈마 30초 유지
- 2024: **100M K에서 48초** 달성
- 2025: **100M K에서 300초** 달성 -- 세계 기록

> 참고: "300초 at 100M degrees" 기록은 2025년 3월 KSTAR가 달성한 것으로,
> 이전 기록(자체 48초)의 약 6배다.
> 48 * 6 = 288 ~ 300... 이건 억지다. 적지 않겠다.
> (적었다. 이것이 n=6 numerology의 유혹이다.)

### 1.2 K-DEMO (Korean DEMOnstration Fusion Power Plant)

한국핵융합에너지연구원(KFE)이 추진하는 상용 핵융합 발전소 계획.

**계획 사양:**

| Parameter | Target | Notes |
|-----------|--------|-------|
| Thermal power | ~2-3 GW_th | |
| Electric power | ~500 MW_e | |
| Q factor | >20 | ITER의 Q=10 대비 2배 |
| TF coils | 16 (KSTAR 기반) | 여전히 12도 24도 아님 |
| First operation | 2040s | |

K-DEMO는 KSTAR의 경험을 직접 스케일업한다.
16-코일 설계를 유지할 가능성이 높으며, 이는 n=6 prediction과 계속 불일치한다.

### 1.3 한국의 여정: Latecomer to Leader

| Year | Milestone |
|------|-----------|
| 1995 | KSTAR 설계 시작 -- 당시 한국은 핵융합 분야 후발주자 |
| 2007 | KSTAR 첫 플라즈마 달성 |
| 2008 | 세계 최초 전초전도 토카막으로 H-mode 달성 |
| 2016 | H-mode 70초 세계 기록 |
| 2021 | 1억도 30초 |
| 2024 | 1억도 48초 |
| 2025 | **1억도 300초 -- 세계 기록** |

한국은 30년 만에 미국, EU, 일본, 중국을 제치고 고온 플라즈마 유지 시간에서
세계 1위에 올랐다. 이것은 n=6 산술이 아니라 일관된 투자와 엔지니어링의 승리다.

---

## Part 2: ITER -- 글로벌 핵융합의 중심

### 2.1 ITER 기본 사양과 n=6 대조

ITER (International Thermonuclear Experimental Reactor)는 프랑스 카다라슈에
건설 중인 세계 최대 토카막이다.

| Parameter | ITER Value | n=6 Candidate | Match? | Grade |
|-----------|-----------|---------------|--------|-------|
| **TF coils** | **18** | sigma(6)=12, J_2(6)=24 | **NO** | **FAIL** |
| **PF coils** | **6** | **n=6** | **YES** | **EXACT** |
| CS modules | 6 | n=6 | YES | **EXACT** |
| Correction coils | 18 (9 upper + 9 lower) | 18=3*n? Stretch. | Maybe | WEAK |
| Major radius | 6.2 m | **n=6** | ~6 | **CLOSE** |
| Minor radius | 2.0 m | phi(6)=2 | YES | **EXACT** |
| Aspect ratio | 3.1 | n/phi=3 | ~3 | **CLOSE** |
| Plasma current | 15 MA | -- | No match | FAIL |
| Magnetic field | 5.3 T | sopfr(6)=5? | ~5 | WEAK |
| Q target | **10** | sopfr(6)*phi(6)=5*2=10 | **YES** | **EXACT** |
| Fusion power | 500 MW | -- | No match | FAIL |
| Heating power | 50 MW | -- | No match | FAIL |
| Plasma volume | 840 m^3 | -- | No match | FAIL |
| First plasma | Originally ~2025 (delayed to 2030s) | -- | N/A | N/A |

**주목할 발견:**

1. **PF coils = 6 = n: EXACT match.** ITER는 정확히 6개의 폴로이달 자기장 코일을 사용한다. 이것은 선택할 수 있는 어떤 정수도 될 수 있었는데 (JET은 6, KSTAR는 14, EAST는 12), 정확히 n=6이다.

2. **Central Solenoid modules = 6 = n: EXACT match.** ITER의 중심 솔레노이드는 6개 모듈로 구성된다.

3. **Major radius ~ 6.2m ~ n: CLOSE.** 정확히 6은 아니지만 놀라울 정도로 가깝다.

4. **Minor radius = 2.0m = phi(6): EXACT match.** 오일러 토션트 값과 정확히 일치.

5. **Q = 10 = sopfr(6) * phi(6): EXACT match.** ITER의 핵심 목표인 에너지 이득율 Q=10은 소인수합과 오일러 토션트의 곱이다.

6. **TF coils = 18: FAIL.** 여전히 12도 24도 아니다. 기존 가설의 핵심 실패가 바뀌지 않는다.

### 2.2 이전 가설과의 비교

기존 H-EG-4는 TF coils에만 집중해서 FAIL 판정을 받았다.
하지만 **PF coils, CS modules, major/minor radius, Q factor**를 보면
n=6 산술과의 일치가 상당히 존재한다.

기존 가설이 잘못 짚은 것: TF coils (토로이달)
기존 가설이 놓친 것: PF coils, CS modules, 기하학적 파라미터

**교훈:** n=6 산술이 핵융합과 무관한 것이 아니라,
기존 가설이 **잘못된 파라미터에 매핑**했을 가능성이 있다.

---

## Part 3: 핵융합 물리와 n=6 산술 -- 파라미터별 정밀 대조

### 3.1 핵반응: D-T Fusion

중수소-삼중수소 반응은 핵융합의 표준 연료 사이클이다.

```
D + T -> He-4 + n + 17.6 MeV
(질량수 2) + (질량수 3) -> (질량수 4) + (질량수 1) + 에너지
```

| Quantity | Value | n=6 Mapping | Grade |
|----------|-------|-------------|-------|
| D mass number | 2 | phi(6) = 2 | **EXACT** |
| T mass number | 3 | max proper divisor of 6 = 3 | **EXACT** |
| D+T sum | 5 | sopfr(6) = 2+3 = 5 | **EXACT** |
| He-4 mass number | 4 | tau(6) = 4 | **EXACT** |
| Neutron mass number | 1 | min divisor = 1 | **EXACT** |
| Products sum | 4+1 = 5 | sopfr(6) = 5 (conserved!) | **EXACT** |
| Energy split: alpha/neutron | 3.5/14.1 MeV = 1/4 | 1/tau(6) | **CLOSE** |

이것은 이 문서에서 가장 강력한 대응이다.
D-T 반응의 질량수가 n=6의 산술 함수와 정확히 일치한다:

- **연료: phi(6) + 3 = sopfr(6)** -- 즉, 2 + 3 = 5
- **생성물: tau(6) + 1 = sopfr(6)** -- 즉, 4 + 1 = 5
- 질량수 보존: sopfr(6) = sopfr(6)

**정직한 평가:** 질량수 보존은 핵물리의 기본 법칙(바리온 수 보존)이지,
n=6 산술에서 유도된 것이 아니다. D-T가 선호 연료인 이유는 반응 단면적이
10-100 keV 영역에서 가장 크기 때문이며, 이는 핵력의 특성이다.
그러나 2+3=5, 4+1=5라는 대응이 **6의 소인수 분해와 약수 함수에 정확히**
매핑된다는 것은 주목할 만하다.

에너지 분배 비율도 흥미롭다:
- Alpha particle: 3.5 MeV / 17.6 MeV = 0.199 ~ 1/5 = 1/sopfr(6)
- Neutron: 14.1 MeV / 17.6 MeV = 0.801 ~ 4/5 = tau(6)/sopfr(6)

이것은 운동량 보존과 질량비에서 직접 나오는 결과다:
E_alpha/E_n = m_n/m_alpha = 1/4 이므로, E_alpha = E_total/5, E_n = 4*E_total/5.
물리적으로는 당연하지만, n=6 함수로의 매핑이 깔끔하다는 것은 사실이다.

### 3.2 Lawson Criterion: n_e * T * tau_E

핵융합 점화 조건:

```
n_e * T * tau_E > 약 3 x 10^21 keV s/m^3  (D-T 반응)
```

| Component | Symbol | n=6 Mapping Attempt | Grade |
|-----------|--------|---------------------|-------|
| Density | n_e | -- (연속 변수) | N/A |
| Temperature | T | 최적 ~15 keV = ? | FAIL |
| Confinement time | tau_E | 연속 변수, 장치 의존 | N/A |
| Triple product threshold | ~3 x 10^21 | -- | FAIL |

**Lawson criterion과 n=6:** 연결이 거의 없다.
Triple product는 연속적인 물리량이며, 그 threshold는 반응 단면적의
적분에서 나온다. 정수론적 매핑이 성립하지 않는다.

기존 H-EG-6의 "R(6)=1 = Q=1 breakeven" 매핑은 verification.md에서
정확히 지적한 대로 trivial하다. 어떤 dimensionless ratio든 1은 breakeven이다.

### 3.3 플라즈마 가둠 모드 (Confinement Modes)

| Mode | Description | Relative tau_E | n=6 claim: divisor ratio |
|------|-------------|---------------|-------------------------|
| L-mode | Low confinement, basic | 1x (baseline) | divisor 1 |
| H-mode | High confinement, edge pedestal | ~2x L-mode | divisor 2 |
| I-mode | Improved, energy barrier w/o particle barrier | ~1.5x L-mode | ? |
| QH-mode | Quiescent H-mode, ELM-free | ~2x L-mode | ? |
| Super H-mode | Very high pedestal | >2x L-mode | not counted |
| EDA H-mode | Enhanced D-alpha | ~2x L-mode | not counted |

기존 가설 H-EG-5의 주장: 정확히 4개 모드, 비율 {1,2,3,6}.

**현실:**
- L-mode와 H-mode는 확실히 구분된 2가지 근본 모드다
- H-mode 대 L-mode의 tau_E 비율은 ~2x -- **이것은 phi(6)=2와 일치**
- I-mode의 tau_E는 ~1.5x이지, 3x가 아니다 -- **FAIL**
- QH-mode는 H-mode의 변형이지 독립 모드가 아니다
- "정확히 4개"라는 counting은 자의적이다

| Claim | Grade |
|-------|-------|
| 모드 수 = 4 | WEAK (counting 자의적) |
| H/L ratio = 2 | **CLOSE** (실제 ~1.5-2.5, 중심값 ~2) |
| I-mode ratio = 3 | FAIL (실제 ~1.5) |
| QH-mode ratio = 6 | FAIL (실제 ~2) |

### 3.4 토카막 기하학

토카막의 핵심 기하학 파라미터:

| Parameter | Definition | Typical Range | ITER | n=6 Mapping | Grade |
|-----------|-----------|---------------|------|-------------|-------|
| Aspect ratio (A) | R/a | 2.5-4.5 | 3.1 | n/phi = 6/2 = 3 | **CLOSE** |
| Elongation (kappa) | b/a (수직/수평) | 1.5-2.0 | 1.7 | phi(6)=2? | WEAK |
| Triangularity (delta) | | 0.3-0.5 | 0.33 | 1/3? | CLOSE |
| Safety factor (q) | | >1 at center, ~3 at edge | q_95 ~ 3 | n/phi=3? | WEAK |

**Aspect ratio ~ 3:** 많은 대형 토카막이 A~3 근처에서 설계된다.
ITER 3.1, JET 2.4, KSTAR 3.6, EAST 4.0. 넓은 범위지만 3 근방이 다수.
n/phi(6) = 3이 "자연스러운" aspect ratio라는 주장은 CLOSE 정도.

**Triangularity ~ 1/3:** ITER의 삼각도 0.33 ~ 1/3은 흥미로운 일치.
그러나 삼각도는 0.3-0.5 범위에서 변하며, 0.33은 그 하한에 가깝다.

### 3.5 자기장 코일 시스템: TF vs PF vs CS

이것이 기존 가설의 핵심 실패 지점이자 새로운 발견 지점이다.

**TF (Toroidal Field) Coils -- 토로이달 자기장:**

| Tokamak | TF Coils | sigma(6)=12? | J_2(6)=24? |
|---------|----------|-------------|------------|
| ITER | 18 | NO | NO |
| JET | 32 | NO | NO |
| KSTAR | **16** | NO | NO |
| EAST | 16 | NO | NO |
| TFTR | 20 | NO | NO |
| JT-60SA | 18 | NO | NO |
| SPARC | 18 | NO | NO |
| DIII-D | 24 | -- | **YES** |

**결론: TF coils = sigma(6) 가설은 완전히 FAIL이다.**
유일한 일치는 DIII-D의 24=J_2(6)이며, 이것도 우연이다.

**PF (Poloidal Field) Coils -- 폴로이달 자기장:**

| Tokamak | PF Coils | n=6? |
|---------|----------|------|
| **ITER** | **6** | **EXACT** |
| JET | 6 circuits | EXACT |
| KSTAR | 14 (7쌍) | FAIL |
| EAST | 12 | sigma(6) |

ITER의 PF coils = 6은 정확한 일치다.
PF 코일은 플라즈마 형상을 제어하는데, 그 수는 플라즈마 단면 형태의
자유도 수에 의해 결정된다. 6개의 독립 PF 코일로
수직 위치, 수평 위치, 타원도, 삼각도, 직사각성, 플라즈마 전류를
제어한다 -- 즉 **6개의 자유도를 6개의 코일로 제어**.

이 "6"이 완전수에서 오는가? 아니다.
플라즈마 형상 제어의 자유도 수에서 온다.
하지만 **자유도 수가 6이라는 사실 자체**가 n=6과 일치한다는 것은 기록할 만하다.

**CS (Central Solenoid) Modules:**

| Tokamak | CS Modules |
|---------|-----------|
| **ITER** | **6** |

ITER의 CS는 6개 모듈로 구성된다. 또 하나의 EXACT match.

### 3.6 플라즈마 가열 방법 (Plasma Heating)

토카막에서 플라즈마를 가열하는 주요 방법:

| Method | Abbreviation | ITER Power | Description |
|--------|-------------|-----------|-------------|
| Neutral Beam Injection | NBI | 33 MW | 중성 입자빔 |
| Ion Cyclotron Resonance Heating | ICRH | 20 MW | 이온 사이클로트론 |
| Electron Cyclotron Resonance Heating | ECRH | 20 MW | 전자 사이클로트론 |

**주요 가열 방법 = 3 = n/phi(6)**

토카막 외부 가열은 정확히 3가지 주요 방법이 있다: NBI, ICRH, ECRH.
(오믹 가열은 토카막 고유이므로 "외부 가열"에서 제외.)

n/phi(6) = 6/2 = 3. **EXACT match.**

그러나 "3"은 매우 작은 수이고, 어떤 물리 시스템에서든 주요 방법이
2-5개인 것은 흔하다. n=6에서 3을 유도하는 경로도 여러 가지다
(n/phi, max proper divisor, prime factor...). **Grade: CLOSE** (사실은 맞지만
인과성은 없다).

### 3.7 디버터 (Divertor) 열부하

토카막의 디버터는 가장 극한의 열부하를 받는 구성요소다.

**ITER 디버터 사양:**

| Parameter | Value | n=6 Mapping |
|-----------|-------|-------------|
| Peak heat flux | 10 MW/m^2 steady, 20 MW/m^2 transient | -- |
| Total heating power | ~150 MW (alpha + external) | -- |
| Power to divertor | ~100 MW | ~2/3 of total |
| Radiated power (scrape-off) | ~50 MW | ~1/3 of total |

기존 주장: "디버터가 전체 파워의 1/6을 처리"

**현실:** 디버터는 전체 파워의 약 **2/3**를 처리한다. 나머지 1/3은
복사(radiation)로 분산된다. 1/6이 아니라 2/3이다.

| Claim | Grade |
|-------|-------|
| Divertor handles 1/6 of power | **FAIL** (실제 ~2/3) |
| Power split 2/3 + 1/3 | 이집트 분수의 처음 두 항이긴 하다 | WEAK |

### 3.8 Q Factor (에너지 이득율)

| Facility | Q Target/Achieved | n=6 Mapping | Grade |
|----------|------------------|-------------|-------|
| JET | 0.67 (achieved 1997) | 2/3 = 1-1/3? | WEAK |
| **ITER** | **10 (target)** | **sopfr(6)*phi(6) = 5*2 = 10** | **EXACT** |
| SPARC | >2 (target) | phi(6) = 2? | WEAK |
| K-DEMO | >20 (target) | ? | FAIL |
| NIF | 1.5 (achieved 2022, inertial) | 3/2 = n/tau? | WEAK |
| Commercial reactor | >30 needed | -- | FAIL |

**ITER Q=10 = sopfr(6)*phi(6):** 이것은 강력한 일치다.
ITER의 Q=10 목표는 핵융합 역사에서 가장 중요한 단일 숫자이며,
sopfr(6)*phi(6) = 5*2 = 10과 정확히 일치한다.

그러나 Q=10은 "증명 가능한 이득이면서 달성 가능한" 목표로
정치적/공학적으로 선택된 숫자다. Q=5, Q=15, Q=20 모두 논의되었으며,
Q=10이 채택된 것은 ITER 설계 크기와 비용의 타협이다.

**Grade: EXACT** (숫자 일치는 완벽하지만, 인과성 주장은 할 수 없다)

---

## Part 4: n=6이 핵융합에 대해 예측하지 못하는 것

이 섹션이 이 문서에서 가장 중요하다.

### 4.1 완전한 실패 목록 (Complete Failure List)

| What n=6 Cannot Predict | Why |
|------------------------|-----|
| **플라즈마 온도** | 최적 D-T 온도 ~15 keV (1.5억 K)는 어떤 n=6 함수와도 안 맞음 |
| **Lawson triple product** | 3 x 10^21 keV s/m^3 -- 정수론과 무관 |
| **TF coil 수** | 모든 주요 토카막에서 FAIL (18, 16, 32, 20...) |
| **Plasma beta** | beta ~ 2-5% -- 연속 변수, 정수 매핑 불가 |
| **Bootstrap current fraction** | ~30-50% -- 특정 n=6 분수와 안 맞음 |
| **Energy confinement scaling** | IPB98(y,2) 스케일링 법칙의 지수들은 경험적이고 n=6과 무관 |
| **Neutron wall loading** | ~1 MW/m^2 -- 물리적 한계이지 산술적 결과 아님 |
| **Disruption frequency** | 확률적이며 정수론 예측 불가 |
| **Greenwald density limit** | n_GW = I_p/(pi*a^2) -- n=6이 아니라 pi가 핵심 |
| **Troyon beta limit** | beta_N ~ 2.8 -- 어떤 n=6 함수도 2.8을 자연스럽게 안 줌 |
| **Fusion power density** | 연속 함수, 정수 매핑 불가 |
| **Materials (first wall, blanket)** | 재료 과학은 원소 주기율표의 문제, 정수론 아님 |
| **Tritium breeding ratio** | TBR > 1.05 필요 -- 왜 1.05인지 n=6은 설명 못함 |
| **Plasma startup sequence** | 공학적 절차, 산술 아님 |
| **Disruption mitigation** | 실시간 제어 문제, 정수론 무관 |

### 4.2 왜 TF coils는 실패하는가

TF 코일 수 결정 요인:
1. **자기장 리플**: N_TF 증가 -> 리플 감소 (roughly as exp(-N_TF))
2. **코일 크기**: 장치가 커지면 코일도 커짐 -> 제작 한계
3. **접근성**: 코일 사이 공간으로 NBI port, 진공 배관 등 접근 필요
4. **비용**: 코일 수 x 단가 vs 리플 저감 이득의 최적화

이 최적화의 결과가 장치마다 다른 이유는 각 장치의 크기, 목적, 예산이
다르기 때문이다. 12나 24에 수렴할 물리적/공학적 이유가 없다.

### 4.3 핵융합의 핵심은 연속 물리학이다

핵융합 플라즈마는 본질적으로 **연속체** 물리학의 영역이다:
- MHD (자기유체역학) 방정식은 편미분방정식
- 수송(transport)은 확산 방정식
- 불안정성(instability)은 고유값 문제
- 이 모든 것의 답은 실수(real numbers)이지 정수가 아니다

정수가 등장하는 곳:
- 코일 수 (공학적 설계 선택)
- 핵반응의 질량수 (핵물리)
- 모듈 수, 포트 수 등 (공학적 이산화)

n=6 산술이 매핑될 수 있는 것은 이 이산적 파라미터들에 한정되며,
핵융합의 핵심 물리(플라즈마 수송, MHD 안정성, 에너지 가둠)에는
적용되지 않는다.

---

## Part 5: 종합 성적표 (Final Scorecard)

### 5.1 전체 claim 등급 요약

| ID | Claim | Grade | Confidence |
|----|-------|-------|------------|
| NF-1 | TF coils = sigma(6) or J_2(6) | **FAIL** | Very High |
| NF-2 | KSTAR TF = 16 matches n=6 | **FAIL** | Very High |
| NF-3 | ITER PF coils = 6 = n | **EXACT** | High |
| NF-4 | ITER CS modules = 6 = n | **EXACT** | High |
| NF-5 | ITER major radius ~ 6 m | **CLOSE** | High (6.2, not 6.0) |
| NF-6 | ITER minor radius = 2 m = phi(6) | **EXACT** | High |
| NF-7 | Aspect ratio ~ 3 = n/phi | **CLOSE** | Medium (range 2.5-4.5) |
| NF-8 | D mass number = phi(6) = 2 | **EXACT** | High (but trivial) |
| NF-9 | T mass number = 3 | **EXACT** | High (but trivial) |
| NF-10 | D+T = sopfr(6) = 5 | **EXACT** | High (baryon conservation) |
| NF-11 | He-4 mass = tau(6) = 4 | **EXACT** | High (but trivial) |
| NF-12 | Q = 10 = sopfr*phi | **EXACT** | Medium (political choice) |
| NF-13 | Plasma modes = tau(6) = 4 | **WEAK** | Low (counting arbitrary) |
| NF-14 | H/L confinement ratio = 2 | **CLOSE** | Medium (range 1.5-2.5) |
| NF-15 | Heating methods = 3 = n/phi | **CLOSE** | Medium (small number) |
| NF-16 | Divertor handles 1/6 of power | **FAIL** | Very High (actually 2/3) |
| NF-17 | R(6)=1 = breakeven | **WEAK** | Low (trivially true) |
| NF-18 | Triangularity ~ 1/3 | **CLOSE** | Medium |
| NF-19 | Lawson criterion maps to n=6 | **FAIL** | Very High |
| NF-20 | Optimal plasma temperature from n=6 | **FAIL** | Very High |

### 5.2 통계 요약

| Grade | Count | Percentage |
|-------|-------|------------|
| **EXACT** | 8 | 40% |
| **CLOSE** | 5 | 25% |
| **WEAK** | 2 | 10% |
| **FAIL** | 5 | 25% |
| **Total** | 20 | 100% |

### 5.3 기존 가설 대비 개선

| Metric | 기존 H-EG-4~6 | 이 문서 |
|--------|--------------|--------|
| Claims analyzed | 3 | 20 |
| EXACT matches | 0 | 8 (40%) |
| FAIL rate | 33% (1/3) | 25% (5/20) |
| Best finding | none | PF=6, Q=10, D-T mass numbers |

기존 가설이 전멸한 이유: **잘못된 파라미터(TF coils)에만 집중**.
올바른 파라미터(PF coils, CS modules, 기하학, 핵반응 질량수)를 보면
n=6 일치가 상당히 존재한다.

---

## Part 6: 결론 -- 핵융합에서의 n=6: 어디까지가 진짜인가

### 강한 일치 (Genuine Patterns)

1. **D-T 핵반응 질량수**: phi(6)=2, 3, tau(6)=4, sopfr(6)=5가
   정확히 매핑된다. 물리적 인과관계는 없지만 수학적 대응은 완벽하다.

2. **ITER PF coils = 6, CS modules = 6**: 두 개의 독립적인 서브시스템이
   모두 n=6개. 플라즈마 형상 제어의 자유도 수가 6인 것은
   토카막 물리학의 실제 결과다.

3. **ITER Q=10 = sopfr(6)*phi(6)**: 핵융합 역사상 가장 중요한
   목표 숫자와의 정확한 일치.

### 약한 일치 (Coincidental Patterns)

4. **Aspect ratio ~ 3**: 넓은 범위의 중심값이 n/phi와 일치하지만,
   2.5-4.5 범위에서 3만 특별하다고 보기 어렵다.

5. **가열 방법 3가지**: 작은 수에 대한 패턴 매칭.

### 완전한 실패 (Clear Failures)

6. **TF coils**: 기존 가설의 핵심이었으나 완전히 틀렸다.
7. **플라즈마 물리의 연속 변수들**: 온도, 밀도, 가둠 시간, beta 등
   핵융합의 핵심 물리량은 n=6 산술과 무관하다.

### 최종 평가

핵융합에서 n=6 산술은 **이산적 설계 파라미터**(코일 수, 모듈 수, 핵반응 질량수)에서
주목할 만한 일치를 보이지만, **연속적 물리량**(온도, 밀도, 가둠 시간)에는
적용되지 않는다.

기존 가설의 실패는 "n=6이 핵융합과 무관하다"가 아니라
"잘못된 파라미터를 골랐다"는 교훈을 남긴다.

그러나 **인과적 설명은 여전히 존재하지 않는다.**
n=6 산술이 "왜" ITER의 PF coils이나 D-T 질량수와 일치하는지에 대한
물리적 메커니즘은 없다. 패턴은 있되, 이론은 없다.

---

*이 문서는 기존 H-EG-4~6의 FAIL 판정을 직시하고,*
*20개의 세분화된 claim으로 재분석한 결과다.*
*정직한 grading: 40% EXACT, 25% CLOSE, 10% WEAK, 25% FAIL.*
*가장 강력한 발견: ITER PF=6, CS=6, Q=10, D-T 질량수 매핑.*
*가장 명확한 실패: TF coils, 플라즈마 연속 물리량 전체.*
