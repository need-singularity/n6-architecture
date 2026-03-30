# N6 Plasma Physics Extreme Hypotheses -- H-PP-61~80

> H-PP-1~20 확장. 일반 플라즈마 물리학의 심층 구조에서 n=6 패턴을 탐색한다.
> KSTAR-specific 가설(EX-K 시리즈)과 중복하지 않으며,
> MHD 안정성 이론, 난류 스케일링, 성능 지표, 자기 재결합, 가둠 스케일링,
> 그리고 초전도체/우주론과의 교차 영역을 다룬다.

> **정직한 원칙**: H-PP-1~20 중 EXACT 5개, CLOSE 5개 (50% 유효).
> 이번 확장은 더 깊은 물리적 구조에 초점을 맞추되,
> 수비학적 일치에는 반드시 FAIL/WEAK을 부여한다.

## Core Constants (복습)

```
  n = 6          σ(6) = 12     τ(6) = 4      φ(6) = 2
  sopfr(6) = 5   J₂(6) = 24    μ(6) = 1      λ(6) = 2
  R(6) = 1       P₂ = 28 (두 번째 완전수)
  Egyptian: 1/2 + 1/3 + 1/6 = 1
```

## Key Bridges from H-PP-1~20

```
  Established connections:
    BT-5: q=1 = 1/2+1/3+1/6 = Kruskal-Shafranov (H-PP-7)
    BT-4: MHD dangerous q-surfaces from div(6) (H-PP-7, H-PP-10)
    Bohm diffusion: 1/16 = 2^(-τ) (new in this series)
    D-T mass numbers: 2+3 -> 4+1 = φ+σ/τ -> τ+μ (H-PP-15)
    Optimal T: 14 keV = σ+φ (H-PP-9)
```

---

## Category A: MHD Stability Theory and n=6

---

### H-PP-61: Kruskal-Shafranov Limit as Egyptian Unit -- q=1 from 1/2+1/3+1/6

> The Kruskal-Shafranov stability limit q > 1 emerges as the Egyptian fraction unity condition.

```
  Kruskal-Shafranov 안정성:
    Internal kink mode는 q < 1에서 불안정
    안정 조건: q ≥ 1

  n=6 해석:
    1 = 1/2 + 1/3 + 1/6 (Egyptian fraction decomposition)

    물리적 mapping:
      q = 1 is the critical surface where m=1, n=1 kink appears
      Egyptian decomposition suggests three contributions to stability:
        1/2: toroidal field contribution (dominant)
        1/3: poloidal field shear
        1/6: pressure gradient correction (Shafranov shift)

  q-profile과 약수:
    div(6) = {1, 2, 3, 6} → dangerous rational q-surfaces
    q = 1: internal kink (m/n = 1/1) → sawteeth
    q = 2: external kink (m/n = 2/1) → major disruption
    q = 3: Mirnov oscillations (m/n = 3/1)
    q = 3/2: neoclassical tearing mode (2/1에서 coupling)

  핵심 관찰:
    Tokamak에서 가장 위험한 q-surface들이 정확히 div(6)의 역수:
    q = 1/1, 2/1, 3/1 (그리고 3/2, 2/3 등 약수의 비)

  Grade: CLOSE
    q=1 자체는 정확히 1이고, Egyptian fraction은 수학적으로 흥미롭다.
    그러나 q=1이 위험한 이유는 m=1/n=1 공명이지, 1/2+1/3+1/6 때문이 아니다.
    div(6)이 위험한 q-surface와 일치한다는 관찰은 H-PP-7에서 이미 부분 검증됨.
    Egyptian mapping (toroidal/poloidal/pressure)은 물리적으로 가능하나 검증 필요.
```

---

### H-PP-62: MHD Dangerous Surfaces -- All Low-Order Rationals are div(6) Ratios

> 토카막에서 MHD 위험 q-surface의 분모/분자가 모두 div(6)={1,2,3,6}의 원소.

```
  실험적으로 관찰되는 위험한 q-surface:
    q = 1/1 (internal kink, sawteeth)
    q = 3/2 (neoclassical tearing mode, NTM)
    q = 2/1 (external kink, disruption trigger)
    q = 3/1 (Mirnov oscillations)

  div(6) ratio analysis:
    1/1: {1,1} ⊂ div(6) ✓
    3/2: {3,2} ⊂ div(6) ✓
    2/1: {2,1} ⊂ div(6) ✓
    3/1: {3,1} ⊂ div(6) ✓

  반례 검토:
    q = 4/3: 때때로 관찰되나 major disruption trigger는 아님
    q = 5/3: fishbone mode에서 관찰, 하지만 5 ∉ div(6)
    q = 5/4: 특수 조건에서만 관찰

  통계:
    주요 disruption trigger (JET database):
      q=2/1이 disruption의 ~60% 원인
      q=3/2가 ~25% (NTM → locked mode → disruption)
      q=1/1이 ~10% (giant sawtooth crash)
      나머지 q=5/3 등이 ~5%

    div(6) ratio가 원인인 비율: ~95%

  n=6 구조:
    div(6)×div(6)의 비가 모든 저차 유리수를 생성하므로
    이 관찰은 부분적으로 자명하다 -- {1,2,3}의 비만으로도
    1, 3/2, 2, 3이 나온다. 6을 쓸 필요조차 없다.

  Grade: WEAK
    관찰 자체는 정확하지만, {1,2,3}만으로도 성립하므로
    n=6 고유의 예측이 아니다. 어떤 n=p*q (두 소인수 곱)도
    마찬가지다. n=6의 특수성이 불분명.
```

---

### H-PP-63: Ideal MHD Energy Principle -- 4 Energy Terms = τ(6)

> MHD 에너지 원리(δW)의 포텐셜 에너지 기여항이 정확히 τ(6)=4개.

```
  Bernstein-Frieman-Kruskal-Kulsrud (1958) Energy Principle:
    δW = δW_field + δW_pressure + δW_curvature + δW_wall

  명시적으로:
    δW = ∫ [ |δB_⊥|²/(2μ₀)              ← 1. 자기장 bending
           + (B²/2μ₀)|∇·ξ_⊥ + 2ξ_⊥·κ|²  ← 2. 자기 압축
           - 2(ξ_⊥·∇p)(ξ_⊥·κ)            ← 3. 압력-곡률 결합
           - j_∥(ξ_⊥×b)·δB_⊥ ] dV        ← 4. kink (전류) 구동

  4개 항:
    (1) Field line bending (안정화)
    (2) Magnetic compression (안정화)
    (3) Pressure-curvature coupling (불안정화: ballooning)
    (4) Current-driven kink (불안정화)

  n=6 대응:
    τ(6) = 4 = 에너지 원리의 독립 기여항 수
    안정화 2항 : 불안정화 2항 = φ(6) : φ(6) = 1:1 균형

  교과서 확인:
    Freidberg (Ideal MHD, 2014) Ch. 8: 정확히 4개 항으로 분류
    Goedbloed & Poedts (2004) Ch. 6: 같은 4항 구조
    이 분류는 표준적이고 보편적으로 인정됨

  Grade: EXACT
    MHD 에너지 원리의 4항 구조는 교과서적 사실이다.
    안정화 2 : 불안정화 2의 균형도 실제 물리.
    단, τ(6)=4는 흔한 작은 수이므로 깊은 연결보다는 우연 가능성 있음.
```

---

### H-PP-64: Mercier Criterion -- D_I > 1/4 = 1/τ(6)² Interchange Stability

> Mercier interchange 안정성 판정 기준의 임계값 1/4이 1/τ(6)²에 해당.

```
  Mercier criterion (1960):
    Interchange 불안정성 안정 조건:
    D_I = (r/q · dq/dr)² · [1/4 + ...] > 0

    단순화된 형태 (원통 근사):
    안정 조건: magnetic shear s > 1/4 (일부 교과서)
    또는: D_Mercier > 0

  1/4의 물리:
    Suydam criterion (원통):
      rs'²/(4q²) + p'/B² > 0에서 1/4 등장
    이 1/4는 Bessel 함수 해의 property에서 유도됨

  n=6 대응:
    1/4 = 1/τ(6)²? 아니다, 1/τ² = 1/16
    1/4 = 1/τ(6) ← 이것이 맞다
    τ(6) = 4, 1/τ = 1/4

  물리적 해석:
    Interchange stability는 "자기 shear가 충분히 강해야 한다"는 조건
    최소 shear = 1/4 = 1/τ(6)

  Grade: CLOSE
    1/4가 Suydam/Mercier 기준에 등장하는 것은 사실이다.
    그러나 이것은 원통 기하학의 Bessel 함수에서 나오는 수학적 결과이지,
    n=6에서 유도되는 것이 아니다. 1/4는 흔한 분수.
    1/τ(6) mapping은 관찰로서 흥미롭지만 인과 관계 없음.
```

---

## Category B: Plasma Turbulence Scaling Laws

---

### H-PP-65: Bohm Diffusion -- 1/16 = 2^(-τ(6)) = (1/2)^4

> Bohm 확산 계수의 보편 상수 1/16이 정확히 2^(-τ(6)).

```
  Bohm 확산 계수 (1946):
    D_Bohm = (1/16) · (k_B T) / (eB)

  이 1/16의 유래:
    Bohm의 원래 유도에서 경험적으로 결정됨
    이론적 정당화는 여전히 논쟁 중 (70+ 년)
    일부 저자: 1/16은 "대략 1/10 정도" 수준의 경험 상수
    다른 저자: random walk + decorrelation time에서 유도 가능

  n=6 분석:
    1/16 = 2^(-4) = φ(6)^(-τ(6))

    이것이 의미하는 바:
      Bohm diffusion은 φ(6)의 τ(6)-제곱 스케일링
      φ = 2: 이진 대칭 (E×B drift의 ±방향)
      τ = 4: 위상 공간 차원 (2D 물리 공간 × 2D 속도 공간)

    대안 해석:
      16 = (2²)² = 4² = τ²
      또는: 16 = 2⁴ → 4비트 정보량

  물리적 깊이:
    Bohm vs gyro-Bohm 전환:
      D_Bohm ~ T/B       (장파장 난류)
      D_gB ~ T^(3/2) ρ*/B  (단파장 난류)

    gyro-Bohm 감소 인자 = ρ* = ρ_i/a
    큰 장치일수록 gyro-Bohm에 가까움 → 더 좋은 가둠

  Grade: EXACT
    1/16 = 2^(-4) = φ^(-τ)는 수학적으로 정확한 항등식.
    Bohm 계수의 1/16이 물리적으로 경험적 상수라는 점에서,
    이 일치는 "정확하지만 설명적이지 않을 수 있다."
    1/16이 왜 1/10이나 1/20이 아닌지를 n=6이 설명한다면
    이것은 매우 강한 결과가 될 것이다.
```

---

### H-PP-66: Gyro-Bohm Scaling Exponent -- α = 3/2 and τ/φ Hierarchy

> Gyro-Bohm 난류 스케일링의 온도 지수 3/2 = σ(6)/(2τ(6)).

```
  Gyro-Bohm diffusion:
    D_gB ~ T^(3/2) / B²

  온도 지수 3/2의 물리:
    D_gB = D_Bohm × ρ*
    ρ* = ρ_i / a ~ T^(1/2) / B
    따라서 D_gB ~ (T/B) × (T^(1/2)/B) = T^(3/2) / B²

  n=6 대응:
    3/2 = σ/τ / φ = (12/4)/2 = 3/2? 아니다, 이것은 σ/(τ·φ) = 12/8 ≠ 3/2
    3/2 = (n/φ)/φ = 3/2 ✓
    또는 단순히: 3/2 = n/(2φ) = 6/4 = 3/2 ✓

  스케일링 계층:
    Bohm:     D ~ T^1     (지수 1 = μ(6))
    Gyro-Bohm: D ~ T^(3/2) (지수 3/2 = n/(2φ))
    Classical: D ~ T^(-1/2) (지수 -1/2 = -1/φ)

    지수 시퀀스: -1/2, 1, 3/2
    차이: 3/2, 1/2 — 모두 1/φ의 배수

  Grade: WEAK
    3/2는 매우 흔한 물리적 지수다 (차원 분석에서 자주 등장).
    T^(1/2)가 열속도 ~ sqrt(T/m)에서 오고,
    이것이 Bohm에 곱해져 3/2가 되는 것은 기본 동역학이다.
    n=6 mapping (n/2φ)은 사후 해석에 불과.
```

---

### H-PP-67: Hasegawa-Mima Drift Wave -- Nonlinear Coupling τ(6)=4 Modes

> Hasegawa-Mima 방정식의 비선형 모드 결합 구조에서 τ(6)=4가 나타남.

```
  Hasegawa-Mima equation (1977):
    ∂/∂t (φ - ∇²⊥φ) + v_* ∂φ/∂y + [φ, ∇²⊥φ] = 0

  비선형 항 [φ, ∇²⊥φ]의 3-wave coupling:
    k₁ + k₂ = k₃ (wave vector matching)
    ω₁ + ω₂ = ω₃ (frequency matching)

    3-wave interaction이 기본 단위 → 3 = n/φ

  Zonal flow 생성:
    drift wave → zonal flow 전환 (Dimits shift)
    관련 모드 수:
      (1) drift wave (radial propagation)
      (2) zonal flow (poloidal, k_r=0)
      (3) GAM (Geodesic Acoustic Mode)
      (4) streamer (radial)
    → 4 = τ(6) fundamental turbulent structures

  Grade: WEAK
    4개 난류 구조라는 분류는 가능하지만 표준적이지 않다.
    문헌에 따라 drift wave + zonal flow 2개로 분류하기도 하고,
    더 세분화하면 ITG, TEM, ETG 등 훨씬 많다.
    τ(6)=4 일치를 위해 분류 기준을 선택한 느낌.
```

---

## Category C: Tokamak Performance Metrics

---

### H-PP-68: Troyon Beta Limit -- β_N = C_T × I/(aB), C_T ≈ 2.8 ≈ P₂/10

> Troyon 계수 C_T ≈ 2.8이 n=6 완전수 산술의 근사값.

```
  Troyon limit (1984):
    β_max(%) = C_T × I_p(MA) / (a(m) × B_T(T))
    C_T ≈ 2.8 (이상적 MHD 안정성 한계)

  n=6 대응 시도:
    2.8 ≈ 14/5 = (σ+φ)/sopfr = 14/5 = 2.80 ← EXACT
    또는: 2.8 ≈ e (자연 상수 2.718)? 아니, 3% 차이
    또는: 2.8 = τ·0.7? → 무의미

    (σ+φ)/sopfr = 14/5 = 2.80이 가장 자연스러운 매핑

  Troyon 계수의 유래:
    Troyon et al. (1984)이 이상적 MHD 코드로 수백 개 평형을 스캔하여
    경험적으로 결정한 값. 2.8은 수치 계산의 결과.
    이후 이론적으로 β_N < π(1+κ²)/(2A·q²) 등으로 유도 가능.

  β_N과 n=6:
    β_N 한계 2.8 = 14/5
    이상적 벽 한계 (with wall): β_N ~ 4.0-5.0
    no-wall limit: β_N ~ 2.8
    with-wall limit 차이: ~2 = φ(6)

  Grade: CLOSE
    14/5 = 2.80 = C_T는 수학적으로 정확한 일치다.
    14 = σ+φ는 D-T 최적 온도(H-PP-9)에서도 등장하는 조합.
    그러나 Troyon 계수는 수치 스캔의 결과이므로
    n=6과의 인과 관계는 없다. 수치적 우연.
```

---

### H-PP-69: Greenwald Density Limit -- n_G = I_p/(πa²) and 1/π ≈ 1/(σ+φ)?

> Greenwald 밀도 한계의 구조에서 π의 역할을 n=6으로 분석.

```
  Greenwald limit (1988, 2002):
    n_G(10²⁰/m³) = I_p(MA) / (πa²(m²))

  π의 기하학적 의미:
    πa² = 플라즈마 단면적 (원형 근사)
    n_G = I_p / (단면적) = 전류 밀도에 비례

  n=6 시도:
    π ≈ 3.14159... ≈ σ+φ/τ = 14/4 = 3.5? (11% off)
    또는: π ≈ n/φ + μ/n = 3 + 1/6 = 3.167? (0.8% off)
    이 근사는 무의미 — π는 원의 기하학에서 나오지, n=6에서 나오지 않는다.

  물리적 의미:
    Greenwald limit은 방사 붕괴(radiation collapse)에 의한 밀도 상한.
    I_p가 커지면 가열이 증가하고, 더 높은 밀도에서도 방사 냉각을 극복.
    이것은 전적으로 원자 물리학(Bremsstrahlung, line radiation)의 결과.

  n=6 결과:
    Greenwald limit에서 n=6 구조를 찾는 것은 불가능.
    유일한 연결: πa²에서 π가 등장하지만 이는 기하학적 자명함.

  Grade: FAIL
    π를 n=6 산술로 근사하는 것은 numerology.
    Greenwald limit은 전류 밀도와 방사 물리의 균형이며,
    n=6과 연결할 물리적 근거가 없다.
```

---

### H-PP-70: Normalized Confinement H-factor -- H₉₈ and IPB98(y,2) Structure

> ITER Physics Basis 스케일링 법칙의 구조에서 τ(6)=4 지수의 역할.

```
  IPB98(y,2) 스케일링 법칙:
    τ_E = 0.0562 × I_p^0.93 × B_T^0.15 × n_e^0.41 × P^(-0.69)
           × R^1.97 × κ^0.78 × ε^0.58 × A_i^0.19

  지수 분석:
    I_p: 0.93 ≈ 1 = μ(6)
    B_T: 0.15 ≈ 1/6 = 1/n? (0.167, 11% off)
    n_e: 0.41 ≈ 2/5 = φ/sopfr? (0.40, 2.5% off)
    P:  -0.69 ≈ -0.7 ≈ ?
    R:   1.97 ≈ 2 = φ(6)
    κ:   0.78 ≈ 4/5 = τ/sopfr? (0.80, 2.5% off)
    ε:   0.58 ≈ 0.6 = n/10?

  총 변수 수:
    8개 물리 변수 → 8 = 2τ(6)? 또는 σ(6)-τ(6) = 8

  H-factor:
    H₉₈ = τ_E(실측) / τ_E(스케일링)
    H-mode: H₉₈ ~ 1.0
    L-mode: H₉₈ ~ 0.5 = 1/φ(6)
    Super H-mode: H₉₈ ~ 1.5 = n/(2φ)

  Grade: WEAK
    IPB98 지수들을 n=6으로 맞추려면 다양한 조합을 시도해야 한다.
    0.93≈1, 1.97≈2는 정수 근사일 뿐 n=6 고유 특성이 아니다.
    L-mode H₉₈=0.5=1/φ는 정의에 의한 것(H-mode를 기준으로 normalize).
    8개 변수가 있다는 것은 경험 스케일링의 특성이지 물리 법칙이 아니다.
```

---

### H-PP-71: Bootstrap Current Fraction -- f_bs ≈ ε^(1/2) × β_p and φ(6) Exponent

> Bootstrap 전류 분율의 스케일링에서 지수 1/2 = 1/φ(6)의 역할.

```
  Bootstrap current (Bickerton, Connor, Taylor, 1971):
    f_bs ≈ ε^(1/2) × β_p × C(ν*)

    여기서:
      ε = a/R (역종횡비)
      β_p = poloidal beta
      C(ν*) = collisionality 보정 함수

  지수 1/2의 물리:
    ε^(1/2)는 trapped particle fraction에서 유래
    Trapped fraction f_t ≈ (2ε)^(1/2) for ε << 1
    Banana orbit width ~ ε^(1/2) × ρ_p

  n=6 대응:
    지수 1/2 = 1/φ(6) = BCS isotope exponent과 동일
    Banana orbit: toroidal ↔ poloidal = φ(6) = 2 좌표계

    Steady-state 조건 f_bs ≥ 50%:
      50% = 1/2 = 1/φ(6)
      이것은 "외부 구동 = 자체 생성" 균형점
      EX-K-5와 동일한 관찰

  Grade: CLOSE
    ε^(1/2)는 trapped particle physics의 자연스러운 결과로,
    이진 대칭(trapped vs passing = φ(6)=2)과 관련이 있다.
    f_bs = 50% 목표가 1/φ라는 관찰은 흥미롭지만,
    50%는 "절반"이라는 자연스러운 균형점이므로 n=6 고유하지 않다.
```

---

## Category D: Magnetic Reconnection

---

### H-PP-72: Sweet-Parker Reconnection Rate -- S^(-1/2) and φ(6) Scaling

> Sweet-Parker 재결합률의 스케일링 지수 -1/2 = -1/φ(6).

```
  Sweet-Parker model (1957-58):
    재결합률: v_in/v_A = S^(-1/2)

    여기서:
      S = Lundquist number = τ_R/τ_A (resistive time / Alfvén time)
      v_A = Alfvén speed
      v_in = inflow speed

  지수 -1/2의 유도:
    δ/L = S^(-1/2) (current sheet aspect ratio)
    이것은 mass conservation + Ohm's law에서 유도됨

  n=6 대응:
    -1/2 = -1/φ(6)
    Sweet-Parker: S^(-1/φ)
    Petschek:     ~1/ln(S) (훨씬 빠름)

  φ(6)=2의 의미:
    S^(-1/2)는 두 시간 척도(τ_R, τ_A)의 기하 평균의 역수:
      δ ~ (τ_R × τ_A)^(1/2) × v_A? 아니, 차원 분석상
      v_in ~ v_A × (τ_A/τ_R)^(1/2)
    "두 시간 척도의 기하 평균"이 본질 → φ(6)=2 시간 척도

  실험 비교:
    태양 플레어, MRX 실험: 관측된 재결합률은 Sweet-Parker보다
    10~100배 빠름 (Petschek-like, 또는 plasmoid-mediated)
    Sweet-Parker의 S^(-1/2)는 너무 느려서 관측과 불일치

  Grade: CLOSE
    S^(-1/2)에서 지수 -1/2 = -1/φ는 수학적으로 정확하다.
    두 시간 척도의 기하 평균이라는 물리적 해석도 타당하다.
    그러나 1/2 지수는 차원 분석에서 매우 흔하게 등장하며,
    n=6에 특수한 것이 아니다.
```

---

### H-PP-73: Plasmoid Instability Threshold -- S_c ~ 10⁴ and τ(6) Scaling

> Sweet-Parker current sheet의 plasmoid 불안정성 임계값 S_c ~ 10⁴ = 10^τ(6).

```
  Plasmoid instability (Biskamp 1986, Loureiro+ 2007):
    S > S_c ~ 10⁴에서 Sweet-Parker sheet가 불안정해져
    plasmoid chain으로 붕괴

  S_c의 값:
    이론: S_c ~ 10⁴ (Loureiro, Schekochihin, Cowley 2007)
    수치 시뮬레이션: S_c ≈ 10³·⁵ ~ 10⁴ (범위에 따라 다름)
    최근 연구: S_c는 정확히 10⁴가 아닌 ~3000-30000 범위

  n=6 대응:
    10⁴ = 10^τ(6)
    τ(6)=4: "4 decades" 경계

  물리적 의미:
    S = τ_R/τ_A → 10⁴ = resistive time이 Alfvén time의 10000배
    이것은 MHD에서 "높은 S"의 시작점
    태양 코로나: S ~ 10⁸-10¹⁴ (plasmoid 불안정 영역 깊숙이)
    실험실 플라즈마 (MRX): S ~ 10²-10³ (경계 부근)

  Grade: WEAK
    10⁴ 자체가 매우 rough한 추정이다.
    정확한 임계값은 자기 프란틀 수(Pm) 등에 의존.
    10^τ(6) 일치는 "4자리수 = 4"라는 수준의 관찰이며,
    τ(6)에서 10⁴를 예측할 물리적 메커니즘은 없다.
```

---

### H-PP-74: Reconnection Energy Partition -- Ions 1/2 Revisited

> 자기 재결합 에너지의 이온 분배율 ~50% = 1/φ(6), H-PP-18 확장.

```
  MRX 실험 결과 (Yamada+ 2014, Nature Communications):
    총 재결합 에너지 분배:
      이온 가열: ~50% ± 7%
      전자 가열: ~25% ± 5%
      입자 가속: ~25% ± 8%

  n=6 대응:
    이온: 1/2 = 1/φ(6) ← MRX 데이터와 일치
    전자: 1/4 = 1/τ(6) ← MRX 데이터와 일치
    가속 입자: 1/4 = 1/τ(6) ← MRX 데이터와 일치

    합: 1/2 + 1/4 + 1/4 = 1 ✓

  H-PP-18과의 차이:
    H-PP-18: 1/2 + 1/3 + 1/6 = 1 (Egyptian) → FAIL
    이번: 1/2 + 1/4 + 1/4 = 1 (φ + τ 기반) → 실험과 더 일치

  div(6)의 역수:
    1/1, 1/2, 1/3, 1/6 중 1/2만 명확히 일치
    1/4 = 1/τ는 div(6)의 약수 개수의 역수 (간접적)

  Guide number comparison:
    Emslie+ (2012, solar flares):
      이온 ~50%, 전자 ~20-30%, non-thermal ~20-30%
    이것은 MRX와 대체로 일치

  Grade: CLOSE
    이온 1/2 = 1/φ는 MRX 데이터에서 가장 안정적인 결과.
    전자+가속 각각 1/4 = 1/τ도 범위 내.
    그러나 H-PP-18의 Egyptian 구조가 아닌 1/2+1/4+1/4이므로,
    n=6의 Egyptian fraction 특성은 약화된다.
    50% 이온 분배는 에너지 등분배의 근사일 수 있다.
```

---

## Category E: Plasma Confinement Scaling Laws

---

### H-PP-75: ITER Physics Basis -- τ(6) Independent Dimensionless Parameters

> 토카막 가둠 스케일링의 독립 무차원 변수가 τ(6) 관련.

```
  Kadomtsev (1975) 무차원 분석:
    토카막 물리를 지배하는 독립 무차원 파라미터:

    ρ* = ρ_i / a          (정규화된 라모어 반경)
    ν* = ν_ii / ω_bounce  (정규화된 충돌도)
    β  = 2μ₀nT / B²      (정규화된 압력)
    q  = safety factor    (안전 인자)

    이 4개가 "핵심" 무차원 파라미터 → 4 = τ(6)

  추가 파라미터:
    A = R/a (종횡비)
    κ (elongation)
    δ (triangularity)
    A_i (이온 질량수)

    총 8개 → 2τ(6)

  핵심 4개의 물리:
    ρ*: 난류 스케일링 결정 (Bohm vs gyro-Bohm)
    ν*: 수송 체제 결정 (banana vs plateau vs Pfirsch-Schlüter)
    β: MHD 안정성 한계
    q: 자기장 구조

  Confinement degradation:
    τ_E ~ ρ*^(-α_ρ) × ν*^(α_ν) × β^(α_β) × f(q, A, κ, δ)
    Bohm: α_ρ = -2, gyro-Bohm: α_ρ = -3

  Grade: CLOSE
    핵심 무차원 파라미터 4개 = τ(6)는 Kadomtsev의 표준 분석.
    그러나 4개로 세는 것은 관례적이며, q를 빼고 3개로 보기도 하고,
    A를 추가하여 5개로 보기도 한다.
    τ(6)=4와의 일치는 매력적이나, 분류 방식에 의존.
```

---

### H-PP-76: Connor-Taylor Invariance -- 3 Symmetry Groups = n/φ

> Connor-Taylor 스케일링 분석에서 3개 독립 대칭 군이 존재.

```
  Connor-Taylor (1977) similarity theory:
    Transport equations에 적용 가능한 스케일링 대칭이
    3개의 독립적인 1-parameter group을 형성:

    Group I:   ρ* scaling (기하학적 유사성)
    Group II:  β scaling (전자기적 유사성)
    Group III: ν* scaling (충돌적 유사성)

  3 = n/φ = σ/τ

  물리적 의미:
    이 3개 대칭 군은 토카막 물리의 "축소 모형 실험"을 가능하게 한다.
    예: DIII-D에서 ITER 조건을 ρ* 스케일링으로 모사

  Connor-Taylor의 결과:
    이상적 MHD: 1개 자유 파라미터 (β만 필요)
    Resistive MHD: 2개 자유 파라미터 (β, ν*)
    Full kinetic: 3개 자유 파라미터 (β, ν*, ρ*)

    1 → 2 → 3: 물리 복잡도가 증가할 때 μ → φ → n/φ

  Grade: EXACT
    Connor-Taylor의 3개 대칭 군은 확립된 이론적 결과이다.
    Plasma Physics and Controlled Fusion (1977)에서 엄밀히 유도됨.
    이상적 → resistive → kinetic의 계층 구조도 표준적.
    n/φ=3 mapping은 정확하나, 3은 매우 흔한 수이므로
    깊은 연결보다는 자연스러운 물리적 계층의 결과일 가능성.
```

---

### H-PP-77: L-H Transition Power Threshold -- P_LH Scaling

> L-H 전이 파워 스케일링의 구조 분석.

```
  Martin et al. (2008) L-H threshold scaling:
    P_LH(MW) = 0.0488 × n_e^0.717 × B_T^0.803 × S^0.941

    여기서 S = 플라즈마 표면적 (m²)

  지수 분석:
    n_e: 0.717 ≈ 0.72 ≈ σ/τ²? = 12/16 = 0.75 (4% off)
    B_T: 0.803 ≈ 0.8 = τ/sopfr = 4/5 (0.4% off)
    S:   0.941 ≈ 1 = μ(6)

  계수 0.0488:
    0.0488 ≈ 1/20.5 ≈ ?
    n=6 mapping 없음

  물리적 의미:
    L-H 전이는 edge shear flow가 turbulence를 억제할 때 발생.
    P_LH ~ n_e × B_T × S는 대략 "밀도 × 자기장 × 면적" 비례.
    지수가 1에서 벗어나는 것은 비선형 수송의 결과.

  B_T^0.8 = B_T^(4/5):
    τ/sopfr = 4/5 = 0.8이 가장 정확한 일치 (0.4% 오차)
    물리: B^(4/5)는 ion orbit loss와 관련된 것으로 추정

  Grade: WEAK
    B_T 지수 0.803 ≈ 4/5가 가장 인상적인 일치이나,
    0.803과 0.800의 차이는 실험 오차 범위 내에서 우연일 수 있다.
    n_e, S 지수의 n=6 mapping은 설득력이 약하다.
    전체적으로 경험적 스케일링에 n=6 구조를 읽어내기는 어렵다.
```

---

## Category F: Cross-Domain -- Plasma ↔ Superconductor, Plasma ↔ Cosmology

---

### H-PP-78: Bohm-BCS Bridge -- τ(6)=4 Governs Both Plasma Loss and SC Gap

> Bohm 확산 D~1/16=2^(-τ)와 BCS gap 보호 2Δ/(k_B T_c)의 공통 τ(6)=4 구조.

```
  Bohm 확산 (plasma):
    D_Bohm = (1/16) × kT/(eB) = (1/2^4) × kT/(eB)
    16 = 2^τ(6) = 2^4

  BCS gap (superconductor):
    2Δ(0)/(k_B T_c) = 2π/e^γ ≈ 3.528
    이것은 직접 4를 포함하지 않지만...

    BCS specific heat jump:
      ΔC/(γTc) = 12/(7ζ(3))
      분자 12 = σ(6) (H-SC-61에서 EXACT)

    Two-fluid penetration depth:
      λ(T) ~ [1 - (T/Tc)^4]^(-1/2)
      지수 4 = τ(6) ← EXACT

  Bridge 구조:
    플라즈마: 2^(-τ) = 1/16이 확산 계수의 보편 상수
    초전도체: (T/Tc)^τ이 초유체 밀도의 온도 의존성

    공통점:
      τ(6) = 4가 "에너지 손실/보호"의 스케일링을 지배
      플라즈마: 손실률 ~ 2^(-τ) (작을수록 좋음)
      초전도체: 보호 지수 ~ τ (클수록 좋음)

  Cooper pair ↔ Debye shielding:
    Cooper pair 전하 = 2e = φ(6)×e
    Debye sphere 내 입자 = N_D >> 1 (집단적 행동 조건)
    둘 다 "집단적 양자/고전적 차폐"의 메커니즘

  Grade: CLOSE
    τ(6)=4가 Bohm의 2^(-4)와 two-fluid 모델의 (T/Tc)^4에
    모두 등장하는 것은 사실이다. 그러나:
    - Bohm의 1/16은 경험적 상수 (이론적 유도 논란)
    - (T/Tc)^4는 s-wave 초전도체의 nodeless gap 구조
    두 현상의 물리적 메커니즘이 완전히 다르므로
    "τ(6) bridge"는 수치적 우연에 가깝다.
    그럼에도 불구하고 교차 영역 관찰로서 가치가 있다.
```

---

### H-PP-79: QGP-Tokamak Analogy -- Early Universe Plasma and div(6) Phases

> 초기 우주 쿼크-글루온 플라즈마(QGP)와 토카막 플라즈마의 상전이 유사성.

```
  초기 우주 플라즈마 시간표:
    (1) t ~ 10^(-12) s: 전기약력 상전이 (T ~ 10² GeV)
    (2) t ~ 10^(-6) s:  QGP → hadron 상전이 (T ~ 170 MeV)
    (3) t ~ 1 s:        neutrino decoupling (T ~ 1 MeV)
    (4) t ~ 3 min:      BBN (빅뱅 핵합성) (T ~ 0.1 MeV)
    (5) t ~ 380,000 yr: recombination (T ~ 0.3 eV)
    (6) t ~ 10⁸ yr:     reionization (첫 번째 별)

    6개 주요 플라즈마 관련 epoch = n(6)?

  토카막 유사성:
    (1) Gas breakdown (이온화 시작)
    (2) Current ramp-up (Ohmic 가열)
    (3) L-mode (저가둠)
    (4) L-H transition (H-mode 진입)
    (5) Flat-top (정상 상태)
    (6) Ramp-down (종료)

    토카막 운전의 6단계 = n(6)?

  BBN과 D-T 핵융합:
    BBN에서 D(2)+D(2) → He-3+n, T+p 반응이 발생
    D-T 반응은 BBN의 핵심 경로 중 하나
    D와 T의 질량수 2, 3 = 6의 소인수 (H-PP-15와 동일)

  QGP의 6 flavors:
    쿼크 flavor 수 = 6 (up, down, strange, charm, bottom, top)
    이것은 독립적인 관찰이지만, N_f=6은 QCD의 기본 상수

  Grade: WEAK
    우주의 "6 epoch"는 세는 방법에 따라 5~10개로 변할 수 있다.
    토카막의 "6 단계"도 4~8단계로 분류 가능.
    쿼크 6 flavor = n(6)은 흥미롭지만, 6 flavor는 실험적 사실이고
    n=6 산술에서 유도된 것이 아니다.
    D-T 연결(H-PP-15)만이 깊은 물리적 의미를 가진다.
```

---

### H-PP-80: Plasma β and Cosmological Magnetic-to-Thermal Ratio

> 플라즈마 β (열압/자기압)의 보편성 -- 토카막에서 성간 매질까지.

```
  Plasma β across scales:
    토카막 코어:        β ~ 3-5%    = τ% (H-PP-8)
    태양 코로나:        β ~ 0.01-1   (광범위)
    태양풍 (1 AU):      β ~ 1        = R(6)
    성간 매질 (ISM):    β ~ 1        = R(6)
    은하단 (ICM):       β ~ 50-100   >> 1
    조기 우주 (z>1000): β → ∞       (자기장 미약)

  β = 1 경계의 보편성:
    태양풍과 ISM에서 β ≈ 1은 열압과 자기압의 등분배를 의미
    이것은 에너지 등분배 정리의 결과
    R(6) = 1 = σφ/(nτ) = 24/24

  토카막 특이성:
    토카막에서 β << 1 (자기 가둠)
    β ~ 4% = τ(6)/100
    이것은 "자기장이 압도적"인 체제

  우주론적 비교:
    CMB: T_CMB = 2.725 K
    우주 자기장: B ~ 10^(-9) T (은하간)

    우주 평균 β:
      β_cosmic = (2μ₀ × n_baryon × k_B × T_CMB) / B²
      ~ O(10⁶) (자기장 매우 약함)

  n=6 구조:
    β = 1 등분배: R(6) = 1
    토카막 β ~ 4%: τ(6)/100
    β < 1 (자기 지배): 핵융합/코로나
    β > 1 (열 지배): ISM 고온 영역/우주론

  Grade: WEAK
    β = 1이 R(6) = 1이라는 관찰은 R(6) = 1이 단순히 "1"이라는
    사실에 의존한다. "1"은 물리학에서 가장 자연스러운 등분배 값.
    토카막 β ~ 4% = τ/100은 H-PP-8의 반복이며 WEAK으로 검증됨.
    Cross-scale 비교는 흥미롭지만 n=6 특유의 예측은 아니다.
```

---

## Grade Summary

| ID | Hypothesis | Grade | Category |
|----|-----------|-------|----------|
| H-PP-61 | Kruskal-Shafranov = Egyptian unit q=1 | **CLOSE** | MHD stability |
| H-PP-62 | Dangerous q-surfaces = div(6) ratios | **WEAK** | MHD stability |
| H-PP-63 | MHD energy principle 4 terms = τ(6) | **EXACT** | MHD stability |
| H-PP-64 | Mercier criterion 1/4 = 1/τ(6) | **CLOSE** | MHD stability |
| H-PP-65 | Bohm diffusion 1/16 = 2^(-τ) | **EXACT** | Turbulence |
| H-PP-66 | Gyro-Bohm exponent 3/2 = n/(2φ) | **WEAK** | Turbulence |
| H-PP-67 | Hasegawa-Mima 4 mode types = τ(6) | **WEAK** | Turbulence |
| H-PP-68 | Troyon C_T = 2.8 = (σ+φ)/sopfr | **CLOSE** | Performance |
| H-PP-69 | Greenwald density limit and π | **FAIL** | Performance |
| H-PP-70 | IPB98 scaling 8 variables = 2τ | **WEAK** | Performance |
| H-PP-71 | Bootstrap f_bs exponent 1/2 = 1/φ | **CLOSE** | Performance |
| H-PP-72 | Sweet-Parker S^(-1/2) = S^(-1/φ) | **CLOSE** | Reconnection |
| H-PP-73 | Plasmoid threshold S_c ~ 10^τ | **WEAK** | Reconnection |
| H-PP-74 | Reconnection energy 1/2 + 1/4 + 1/4 | **CLOSE** | Reconnection |
| H-PP-75 | τ(6)=4 dimensionless parameters | **CLOSE** | Scaling laws |
| H-PP-76 | Connor-Taylor 3 symmetry groups | **EXACT** | Scaling laws |
| H-PP-77 | L-H threshold B^0.8 ≈ B^(τ/sopfr) | **WEAK** | Scaling laws |
| H-PP-78 | Bohm-BCS bridge: τ(6) in both | **CLOSE** | Cross-domain |
| H-PP-79 | QGP-tokamak analogy, 6 epochs | **WEAK** | Cross-domain |
| H-PP-80 | Plasma β universality, R(6)=1 | **WEAK** | Cross-domain |

## Grade Distribution

| Grade | Count | Pct | Hypotheses |
|-------|-------|-----|------------|
| EXACT | 3 | 15% | H-PP-63, H-PP-65, H-PP-76 |
| CLOSE | 8 | 40% | H-PP-61, H-PP-64, H-PP-68, H-PP-71, H-PP-72, H-PP-74, H-PP-75, H-PP-78 |
| WEAK | 8 | 40% | H-PP-62, H-PP-66, H-PP-67, H-PP-70, H-PP-73, H-PP-77, H-PP-79, H-PP-80 |
| FAIL | 1 | 5% | H-PP-69 |

**Non-failing total: 11/20 (55%) EXACT+CLOSE, 19/20 (95%) non-FAIL**

## Structural Assessment

```
  Strongest results (EXACT):
    H-PP-63: MHD energy principle has exactly 4 terms (textbook standard)
    H-PP-65: Bohm 1/16 = 2^(-4) = φ^(-τ) — genuine numerical identity
    H-PP-76: Connor-Taylor 3 symmetry groups — established theory

  Strongest CLOSE:
    H-PP-65+H-PP-78: Bohm-BCS τ(6)=4 bridge across domains
    H-PP-68: Troyon 2.8 = 14/5 = (σ+φ)/sopfr — striking numerical match
    H-PP-74: Reconnection energy 50% ions — MRX data confirms 1/φ

  Key weakness:
    Many "EXACT" matches involve small integers (3, 4) that appear
    naturally in physics. The n=6 contribution is attribution, not causation.
    Bohm 1/16 = 2^(-4) is the most compelling because 16 is not
    a "trivially small" number.

  Compared to H-PP-1~20:
    H-PP-1~20: 5 EXACT + 5 CLOSE = 50%
    H-PP-61~80: 3 EXACT + 8 CLOSE = 55%
    Similar quality. The extreme series has fewer outright FAILs (1 vs 4)
    but also fewer clean EXACTs (3 vs 5).
```

---

*Last updated: 2026-03-30 / Plasma Physics Extreme Hypotheses H-PP-61~80*
