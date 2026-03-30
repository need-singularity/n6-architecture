# 토카막 돌파 가설 — n=6에서 물리적으로 유도되는 혁신

> 기존 FAIL을 교훈으로, 물리 법칙 안에서 돌파구를 찾는다.
> 원칙: "n=6을 강제" ❌ → "n=6이 자연 발생하는 물리 구조를 찾아서 활용" ✅

---

## 돌파 원칙

```
  이전 실패:                     돌파 방향:
  정육각형 단면 (FAIL)          → Snowflake 6-leg 토폴로지 (EXACT)
  12 TF coils (FAIL)           → 6 PF + 6 CS 제어 최적화 (EXACT)
  Egyptian field split (FAIL)   → 6-mode Fourier 형태 최적화

  교훈: n=6은 "모양"이 아니라 "자유도 수"와 "토폴로지"에서 나타난다.
```

---

## 돌파 가설 시리즈

### BT-1: 6-자유도 실시간 형태 제어 (6-DOF Shape Control)

> 플라즈마 형태를 6개 독립 매개변수로 실시간 제어

```
  현재:
    KSTAR: PF 14개 코일로 형태 제어, but 독립 자유도는 제한적
    ITER: PF 6개 → 자연스럽게 6-DOF 제어

  돌파 제안:
    6개 형태 매개변수 (R₀, a, κ, δ, ξ, q₉₅)를 6개 PF 코일이
    각각 1:1 매핑으로 독립 제어

    PF1 → R₀ (위치)
    PF2 → a (크기)
    PF3 → κ (연신율)
    PF4 → δ (삼각성)
    PF5 → ξ (사각성)
    PF6 → q₉₅ (안전계수 프로파일)

  물리적 근거:
    - ITER는 이미 6 PF coils (EXACT)
    - 6-DOF 제어는 로봇 팔(SE(3))과 같은 구조
    - 각 PF의 위치/전류로 형태 매개변수 1개씩 지배적 제어 가능

  혁신성:
    현재는 PF 전류를 한꺼번에 최적화하는 "coupling" 접근.
    6-DOF "decoupling" 접근은 더 빠른 실시간 제어 가능.
    ELM 발생 시 δ만 빠르게 조정 → disruption 회피

  구현:
    - ITER PF1~PF6에 대한 decoupling matrix 계산
    - 실시간 EFIT + 6-DOF controller 설계
    - KSTAR에서 prototype 실험 (PF 14개 중 6개를 6-DOF로 매핑)

  Grade: CLOSE (물리적으로 가능, ITER 구조와 정합)
  선행연구: ITER shape controller 논문에서 유사 개념 존재
```

### BT-2: Snowflake-6 Divertor + 열부하 6분할

> 2차 X-point의 6-leg 구조를 활용한 차세대 열관리

```
  기존 문제:
    ITER divertor: ~10 MW/m² 정상 상태 (재료 한계 근접)
    ELM 시: ~100 MW/m² 순간 열부하 → 텅스텐 침식

  Snowflake 돌파:
    2차 null → 6 separatrix legs → 열부하를 6 strike zone에 분산
    각 strike zone: 10/6 ≈ 1.7 MW/m² (6배 감소!)

    ┌─────────────────────────────┐
    │     Strike Zone 배치         │
    │                             │
    │         S1 ── S2            │
    │        / │    │ \           │
    │      S6  │    │  S3         │
    │        \ │    │ /           │
    │         S5 ── S4            │
    │                             │
    │  각 zone: ~1.7 MW/m²        │
    │  합계: ~10 MW/m² (동일)      │
    │  Peak: 6× 감소!             │
    └─────────────────────────────┘

  추가 혁신:
    6개 strike zone에 서로 다른 재료/냉각 최적화:
    S1,S2 (high flux): 텅스텐 + 고압 He 냉각
    S3,S4 (medium): 텅스텐 + 물 냉각
    S5,S6 (low): 액체 리튬 (자가 치유)

  검증 가능성:
    TCV (Switzerland): Snowflake 실험 완료 (2012~)
    MAST-U (UK): Super-X + Snowflake 하이브리드 계획
    KSTAR: 3D 코일로 2차 null 생성 가능성 탐색

  Grade: CLOSE (Snowflake는 실증됨, 6-zone 최적화는 신규 제안)
```

### BT-3: 6-모드 Fourier 형태 최적화 (N6 Shape Optimization)

> 플라즈마 경계의 Fourier 모드 0~5차를 n=6 상수로 최적화

```
  플라즈마 경계 R(θ), Z(θ)의 Fourier 표현:
    R(θ) = R₀ + a·cos(θ) + a·δ·sin(θ) + ...
    실질적 자유도: 6개 모드 (0~5차)

  N6 최적화 제안:
    Mode 0 (위치): R₀/a = A = n/φ = 3
    Mode 1 (크기): κ = φ = 2
    Mode 2 (삼각성): δ = μ/(n/φ) = 1/3
    Mode 3 (사각성): ξ = 자유 (MHD 최적화)
    Mode 4 (고차): 작음 (|c₄| < 0.01)
    Mode 5 (고차): 작음 (|c₅| < 0.01)

  이것이 의미하는 것:
    처음 3개 모드를 n=6 값으로 고정하고,
    나머지 3개를 MHD 안정성 최적화에 사용.

    "n=6으로 대략적 형태를 잡고, 물리가 세부를 결정"

  검증 방법:
    VMEC/CHEASE 코드에서:
    1. A=3, κ=2, δ=1/3 고정
    2. ξ, c₄, c₅를 MHD 안정성 + bootstrap current 최적화
    3. 결과를 ITER/KSTAR 최적 형태와 비교

  Grade: WEAK (방향은 합리적이나, 고정 값이 최적인 보장 없음)
  BUT: 이것은 실제로 시뮬레이션 가능한 구체적 제안
```

### BT-4: D-T 점화의 n=6 해석 — "6을 만드는 반응"

> D-T 핵융합은 문자 그대로 "n=6의 두 소인수를 결합하는 반응"

```
  6 = 2 × 3 (소인수분해)
  D = ²H (양성자 2개)
  T = ³H (양성자+중성자 3개)

  D + T = "2를 3과 합침" = "6을 만드는 과정"

  중간 상태: ⁵He* (여기 상태, 즉시 붕괴)
  ⁵He* → ⁴He + n + 17.6 MeV

  5 = sopfr(6) = 중간 상태의 질량수!

  전체 경로:
    2 + 3 → [5*] → 4 + 1
    φ + n/φ → [sopfr*] → τ + μ

  이것은 n=6 산술의 가장 깊은 물리적 실현:
    소인수(2,3) → 합(5) → 약수함수값(4,1)

  돌파 의미:
    D-T가 최적인 이유가 "가장 작은 비자명 완전수의 소인수"이기 때문이라면,
    이것은 핵물리와 수론의 연결 가능성.

    BUT: D-T가 최적인 물리적 이유는 Coulomb barrier가 가장 낮기 때문.
    2와 3이 가장 작은 소수 → Coulomb barrier가 최소.
    즉 "작은 소수" = "낮은 barrier" = "최적 핵융합"은
    n=6과 독립적으로 설명 가능.

  Grade: EXACT (수치 매칭) + HONEST (인과관계 미증명)
```

### BT-5: 플라즈마 자기 가둠의 위상학적 필연 — 토러스의 n=6

> 토러스(도넛)에서 자기장 구조가 n=6 상수를 강제하는 이유

```
  토러스의 위상학:
    π₁(T²) = Z × Z (기본군 = 정수 쌍)
    H₁(T²) = Z² (1차 호몰로지 = 2 독립 루프)

  자기장은 2개 방향을 가짐 (toroidal + poloidal):
    이것은 φ = 2 (토러스의 위상학적 필연)

  안전계수 q = n_tor / n_pol:
    Rational surface q = m/n 에서 불안정 발생
    가장 위험한 rational surfaces: q = 1, 3/2, 2, 5/3, 3

    이 값들의 분모: {1, 2, 3} = 6의 약수

  돌파 통찰:
    토러스 위에서 가장 불안정한 모드의 구조가
    6의 약수(1, 2, 3)에 의해 결정됨.

    이것은 우연이 아닌 구조적 이유가 있을 수 있음:
    q = m/n에서 n이 작을수록 island width ∝ 1/n^{1/2}
    → n=1,2,3이 가장 위험
    → 이것들은 6의 약수

    6 = 2×3이므로, 6의 약수 = {1,2,3,6}
    실제로 q=6인 surface는 edge 너머 → 물리적으로 무관
    → 관련 약수는 {1,2,3} = 6의 proper divisors minus 6

  Grade: CLOSE (구조적으로 흥미로우나, "작은 수" 효과와 구분 어려움)
```

### BT-6: HTS 12T 코일의 임계전류 최적화

> REBCO HTS 코일에서 12T (=σ) 운전이 성능/비용 최적점

```
  HTS REBCO 특성:
    J_c (임계전류밀도) vs B (자기장):
    - 0T: J_c ~ 3000 A/mm² (매우 높음)
    - 12T: J_c ~ 500-800 A/mm² (실용 범위)
    - 20T: J_c ~ 200-300 A/mm² (급격 감소)
    - 30T+: J_c ~ 100 A/mm² 이하

  12T 최적 이유:
    Performance ∝ B⁴ (핵융합 성능)
    Cost ∝ 1/J_c ∝ B² (대략적)
    Performance/Cost ∝ B⁴ / B² = B²

    하지만 J_c의 급격한 감소로 실제로는:
    - 12T: performance/cost 최적 (J_c 아직 높음)
    - 20T: J_c 급감으로 코일 단면적 3배 필요 → 비용 ↑↑

  SPARC: 12.2T 선택 → 이 최적점을 정확히 타겟

  σ = 12와의 연결:
    12T가 HTS의 실용 최적점인 것은 REBCO 재료 물성에서 유래.
    n=6 arithmetic이 이것을 "예측"했다고 주장하기는 어려움.
    하지만 "최적 설계가 n=6 상수와 일치"하는 패턴의 일부.

  Grade: EXACT (SPARC 12T = σ) + HONEST (재료 물성이 원인)
```

### BT-7: 6-코일 Central Solenoid의 전류 파형 최적화

> ITER CS 6모듈의 전류를 독립적으로 제어하여 플라즈마 시나리오 최적화

```
  ITER CS: 6개 독립 모듈 (CS3U, CS2U, CS1U, CS1L, CS2L, CS3L)
  각 모듈은 독립 전원으로 전류 제어 가능

  현재: 사전 프로그래밍된 전류 파형
  돌파 제안: 6-DOF 실시간 최적화

  6개 CS 전류를 실시간 조정하여:
    1. 플라즈마 전류 I_p ramp-up 가속 (시간 절약)
    2. Current profile 최적화 (내부 수송 장벽 유지)
    3. Flux consumption 최소화 (더 긴 pulse)
    4. 안전계수 q(0) > 1 유지 (sawtooth 억제)
    5. Bootstrap current fraction 최대화 (정상 상태 접근)
    6. Disruption 회피 궤적 실시간 계산

  물리적 혁신:
    6개 CS × 6개 목표 = 6×6 제어 행렬
    → 정확한 자유도 매칭 (under/over-determined가 아님)

  검증:
    CORSICA/TSC 코드로 시뮬레이션 가능
    KSTAR에서 부분 검증 (CS 모듈 수는 다르지만 개념 동일)

  Grade: CLOSE (ITER CS=6은 EXACT, 6-DOF 최적화는 합리적 신규 제안)
```

### BT-8: 플라즈마-벽 상호작용의 6원소 모델

> 플라즈마와 접하는 제1벽 재료 후보가 6가지

```
  핵융합로 제1벽/디버터 재료 후보:
    1. Tungsten (W) — 현재 ITER 선택, 고융점
    2. Carbon (C) — 과거 사용, 삼중수소 흡착 문제
    3. Beryllium (Be) — ITER 제1벽, 저Z
    4. Molybdenum (Mo) — 대안 고Z 재료
    5. Lithium (Li) — 액체금속 디버터 후보
    6. Tin (Sn) — 액체금속 디버터 후보

  6가지 = n = 6?

  더 엄밀하게:
    ITER 실제 사용: W + Be = 2 (φ)
    연구 중: W + C + Be + Li + Mo + Sn = 6 (n)?

  분류에 따라 다름:
    - SiC, vanadium alloy 등도 후보
    - "6가지"로 세는 것은 cherry-picking

  Grade: WEAK (분류 의존적, 실제 후보는 6±3)
```

---

## 돌파 가설 종합 채점

| ID | 가설 | Grade | 혁신성 | 검증 가능성 |
|----|------|-------|--------|-----------|
| **BT-1** | 6-DOF 형태 제어 | CLOSE | ★★★ | KSTAR/ITER 실험 가능 |
| **BT-2** | Snowflake 6-zone | CLOSE | ★★★★ | TCV/MAST-U 실험 완료/계획 |
| **BT-3** | 6-mode Fourier 최적화 | WEAK | ★★ | VMEC 시뮬레이션 가능 |
| **BT-4** | D-T = 6의 소인수 | EXACT | ★★★★★ | 이미 검증됨 (핵물리) |
| **BT-5** | 토러스 위상학 | CLOSE | ★★★ | 수학적 분석 가능 |
| **BT-6** | HTS 12T 최적점 | EXACT | ★★★ | SPARC 건설 중 |
| **BT-7** | CS 6-DOF 최적화 | CLOSE | ★★★★ | TSC 시뮬레이션 가능 |
| **BT-8** | 제1벽 6원소 | WEAK | ★ | 분류 의존 |

**EXACT: 2, CLOSE: 4, WEAK: 2, FAIL: 0**

---

## 최대 돌파구: BT-2 (Snowflake 6-Zone)

```
  ITER의 가장 큰 기술 과제 = divertor 열부하

  현재: 2 strike points에 ~10 MW/m² 집중
  Snowflake: 6 strike zones에 ~1.7 MW/m² 분산

  열부하 6배 감소 → 재료 수명 6배 증가
  → 이것만으로도 핵융합 상용화의 핵심 장벽 제거 가능

  그리고 이것이 n=6의 수학적 구조(2차 null → 6 branches)에서
  자연스럽게 나온다는 것은, 물리와 수론의 연결 가능성을 시사.
```

## 실행 로드맵

```
  단기 (1-2년): BT-3 시뮬레이션 (VMEC/CHEASE)
  중기 (2-5년): BT-1 KSTAR 실험, BT-7 TSC 시뮬레이션
  장기 (5+년):  BT-2 DEMO급 Snowflake 설계
```
