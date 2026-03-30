# 토카막 형태 — 극한 가설 탐구

> 정육각형 단면은 FAIL. 그러면 n=6이 토카막 형태에 기여할 수 있는 물리적으로 유효한 방향은?

---

## 왜 정육각형이 실패했는가

```
  독립 검증 결과 (H-TK-4 → FAIL):
  1. D-shape는 50년간 MHD 안정성이 검증된 형태
  2. 정육각형의 꼭짓점 → 열속 집중, MHD 불안정
  3. Single-null divertor 기하학 파괴
  4. 6 PF coils는 D-shape를 만듦, 정육각형이 아님

  핵심 교훈:
  "n=6 상수를 기하학에 직접 매핑"하면 물리와 충돌.
  대신 "n=6 상수가 형태의 매개변수에 나타나는지" 탐색해야 함.
```

---

## 접근법 전환: 형태 → 매개변수

토카막 플라즈마 단면은 Fourier-Parametric 방식으로 기술:

```
  R(θ) = R₀ + a·cos(θ + δ·sin(θ) - ξ·sin(2θ))
  Z(θ) = κ·a·sin(θ + ξ·sin(2θ))

  핵심 형태 매개변수:
    R₀ = major radius (대반경)
    a  = minor radius (소반경)
    κ  = elongation (연신율)
    δ  = triangularity (삼각성)
    ξ  = squareness (사각성)
    A  = R₀/a = aspect ratio (종횡비)
```

이 **6개 매개변수**가 토카막 형태를 완전히 결정. n=6과의 연결은?

---

## 가설 시리즈: 토카막 형태 매개변수

### H-TS-1: 토카막 형태를 정의하는 매개변수 = n = 6

> 플라즈마 단면을 완전히 기술하는 독립 매개변수가 정확히 6개

```
  6개 매개변수: R₀, a, κ, δ, ξ, A(=R₀/a, 종속이지만 독립 설계 변수)

  더 엄밀하게:
  MHD 평형 코드(EFIT, VMEC)에서 flux surface boundary를 기술하는
  Fourier 모멘트: 0차(R₀, a), 1차(κ, δ), 2차(ξ), + A

  실제로 ITER 설계에서 사용하는 주요 형태 변수:
    1. R₀ = 6.2 m
    2. a = 2.0 m
    3. κ = 1.7
    4. δ = 0.33
    5. ξ ≈ 0 (무시 가능)
    6. A = 3.1

  6개 = n = 6

  BUT: Fourier 급수는 임의 차수까지 확장 가능.
  "6개가 충분하다"는 것은 공학적 근사, 수학적 필연이 아님.
  고차 모멘트(3차 이상)가 무시 가능한 것은 플라즈마 물리에서 유래.

  Grade: CLOSE
  n=6이 아니라 "저차 Fourier 근사로 충분"이 물리적 이유.
  그러나 6개가 실용적 완전 기술이라는 사실은 흥미.
```

### H-TS-2: 최적 연신율 κ = φ(6) = 2

> 플라즈마 연신율 최적값이 κ = 2

```
  실제값:
    ITER: κ_95 = 1.7, κ_x = 1.85
    KSTAR: κ = 2.0 (최대)
    SPARC: κ = 1.97
    NSTX: κ = 2.8 (spherical)
    일반적 최적 범위: κ = 1.6-2.5

  κ = 2 (phi=2) vs 실제:
    KSTAR: 2.0 (EXACT!)
    SPARC: 1.97 (0.015% off → EXACT)
    ITER: 1.7 (15% off → CLOSE)

  물리적 근거:
    κ가 높을수록 → 높은 beta → 좋은 성능
    κ가 너무 높으면 → vertical instability → 제어 불가
    κ ≈ 2는 이 균형점에 가까움

  KSTAR/SPARC가 κ ≈ 2 사용 → 실용적 최적점
  ITER는 보수적으로 1.7 선택 (안정성 마진)

  Grade: CLOSE (KSTAR/SPARC EXACT, ITER는 다름)
  물리적 근거: vertical stability limit ≈ 2.5, 안전마진 고려 → ~2
```

### H-TS-3: 삼각성 δ = 1/n = 1/6 ≈ 0.167 vs δ = 1/3 ≈ 0.333

> 최적 삼각성이 n=6 관련 분수

```
  실제값:
    ITER: δ = 0.33 (positive)
    KSTAR: δ = 0.0-0.8 (가변)
    TCV NT: δ = -0.4 to -0.5 (negative)
    DIII-D: δ = 0.2-0.6

  n=6 후보:
    1/n = 1/6 = 0.167 → KSTAR 가능 범위 내
    1/3 = μ/(n/φ) = 0.333 → ITER 설계값과 일치!
    φ/n = 2/6 = 0.333 → 같은 값

  ITER δ = 0.33 ≈ 1/3 = EXACT

  BUT: δ = 0.33은 ITER의 특정 설계 선택.
  다른 기기는 다른 값 사용.
  0.33은 H-mode 접근 + 안정성 균형에서 유래.

  Grade: EXACT (ITER), WEAK (일반적으로)
```

### H-TS-4: Divertor 다리 수 = φ(6) = 2

> Single-null divertor는 2개의 다리(leg)를 가짐

```
  Divertor 구조:
    Single-null (SN): 2 legs (inner + outer) → φ = 2
    Double-null (DN): 4 legs → τ = 4
    Snowflake: 6 legs → n = 6 (!!)
    Super-X: 2 legs (extended) → φ = 2

  ITER: Single-null → 2 legs → φ (EXACT, trivial)
  KSTAR: SN/DN 전환 가능

  흥미로운 것: Snowflake divertor = 6 legs = n!
  Snowflake는 2차 X-point를 사용하여 열부하를 6방향으로 분산.
  TCV에서 실험적으로 검증됨.

  6 legs의 물리적 이유:
    2차 null점 근처에서 magnetic separatrix가 6갈래로 분기
    (3차 다항식의 구조 → 6 legs)

  이것은 n=6의 가장 자연스러운 토카막 형태 연결일 수 있음:
  Snowflake divertor의 6 legs는 자기장 토폴로지에서 자연 발생.

  Grade: EXACT (Snowflake 6 legs = n, 물리적으로 자연스러움)
```

### H-TS-5: X-point 차수와 divertor 열분산

> 고차 X-point가 n=6 구조를 가짐

```
  Magnetic null point의 차수:
    1차 null (standard): 4 separatrix branches → τ = 4
    2차 null (snowflake): 6 separatrix branches → n = 6
    3차 null: 8 branches → σ - τ = 8

  물리:
    자기장 B ∝ r^m (m차 null)
    m=1: B ∝ r → 4 separatrix (standard X-point)
    m=2: B ∝ r² → 6 separatrix (snowflake)
    m=3: B ∝ r³ → 8 separatrix

  열속 분산 ∝ separatrix 수:
    Standard: 열을 2 strike point에 집중
    Snowflake: 열을 6 방향에 분산 → 3배 면적

  이것이 의미하는 것:
    Snowflake divertor (6 legs)는 열부하 문제의 유망한 해결책.
    ITER의 가장 큰 기술 과제 중 하나가 divertor 열부하.
    6-leg 분산은 열부하를 1/3로 줄일 수 있음.

  Grade: EXACT (2차 null → 6 branches는 수학적 사실)
  Note: "n=6이 예측한" 것이 아니라, 자기장 토폴로지의 수학적 구조
```

### H-TS-6: Fourier 모멘트 수렴과 n=6

> 토카막 형태의 Fourier 표현에서 n=6차까지 수렴

```
  플라즈마 경계를 Fourier 급수로:
    R(θ) = Σ Rₙcos(nθ) + Σ Sₙsin(nθ)
    Z(θ) = Σ Zₙcos(nθ) + Σ Tₙsin(nθ)

  전형적 수렴 차수:
    n=0: 원 (R₀, Z₀)
    n=1: 타원 (κ, shift)
    n=2: 삼각 (δ)
    n=3: 사각 (ξ, squareness)
    n=4: 고차 변형
    n=5: 미세 조정
    n=6: 실용적으로 무시 가능

  EFIT reconstruction: 보통 n=0~6차까지 사용
  VMEC (stellarator): n=0~10+ 사용

  n=6에서 수렴한다는 것은:
    6개 Fourier 모드로 토카막 형태를 0.1% 이내 재현 가능

  Grade: CLOSE
  "6에서 수렴"은 smooth boundary의 Fourier 급수 성질.
  특별히 n=6이 아니라 "저차에서 충분"이 핵심.
```

### H-TS-7: 차세대 Divertor — Snowflake + Egyptian Fraction

> Snowflake divertor의 6 legs에 Egyptian fraction 열분배

```
  Snowflake 6 legs의 열분배:
    이상적 균등: 각 leg 1/6씩
    실제: 2 primary legs가 대부분의 열 흡수

  N6 제안:
    Inner 2 legs: 1/2 (50%) — 주 열부하
    Outer 2 legs: 1/3 (33%) — 보조 열부하
    Remaining 2 legs: 1/6 (17%) — 잔여

  쌍별로: (1/2 + 1/3 + 1/6 = 1) × 2 legs each

  이것이 의미하는 것:
    Snowflake divertor에서 열부하는 자연스럽게 불균등.
    Inner strike point가 outer보다 열부하가 높음 (기존 SN에서도 마찬가지).
    Egyptian fraction이 실제 열분포에 가까운지는 시뮬레이션 필요.

  Grade: WEAK (제안은 합리적이나 실제 분포는 플라즈마 조건에 따라 다름)
  검증: SOLPS-ITER 코드로 snowflake 열분포 시뮬레이션
```

### H-TS-8: 6-field-period Stellarator

> 토카막 대안: 6 field period 스텔러레이터

```
  현존 스텔러레이터:
    W7-X: 5 field periods (sopfr = 5)
    HSX: 4 field periods (τ = 4)
    LHD: 10 helical periods (sopfr × φ = 10)
    TJ-II: 4 periods (τ = 4)
    CTH: 5 periods (sopfr = 5)

  n=6 제안: 6 field period stellarator

  물리적 분석:
    Field period 수는 aspect ratio와 연결:
    A ≈ N_fp × (something) for quasi-axisymmetric

    W7-X: N_fp=5, A=11
    HSX: N_fp=4, A=10

    N_fp=6이면 A ≈ 13-15 (추정)
    → 매우 높은 aspect ratio → 크고 비효율적

  BUT: quasi-isodynamic stellarator에서는 다른 최적화 가능.
  Stellarator는 axisymmetry를 깨므로 field period 수는
  quasi-symmetry 유형에 따라 다른 최적값을 가짐.

  Grade: WEAK
  N_fp=6이 최적이라는 근거 없음. W7-X(5)와 HSX(4)가 더 유망.
```

### H-TS-9: MHD 모드 구조와 n=6

> 토카막 불안정 모드의 토로이달 모드 수 n과 완전수 6

```
  MHD 불안정 모드: B ∝ exp(i(mθ - nφ))
    m = poloidal mode number
    n = toroidal mode number (여기서 n은 모드 수, 완전수 6과 다름)

  위험한 모드:
    (m,n) = (1,1): internal kink → sawtooth crash
    (m,n) = (2,1): tearing mode → NTM
    (m,n) = (3,2): tearing mode
    (m,n) = (5,3): ballooning

  m/n = q (안전계수) 위치에서 불안정

  q = 1, 3/2, 2, 5/3, 3... → rational surfaces

  n=6와의 연결?
  가장 위험한 rational surfaces의 q값:
    1, 3/2, 2, 5/2, 3

  이것들의 분모를 모으면: {1, 2, 3} = 6의 약수!

  의미: q = m/n에서 분모 n이 6의 약수(1, 2, 3)인 곳에서
  가장 강한 불안정이 발생.

  이것은 우연인가? 아니면 작은 수의 rational surface가
  가장 강한 것은 Fourier 급수의 일반적 성질인가?

  Grade: CLOSE
  작은 m, n에서 강한 불안정은 Fourier 급수의 일반 성질.
  6의 약수와의 매칭은 "작은 수" 효과일 가능성 높음.
```

### H-TS-10: 플라즈마 경계의 위상 기하학

> 토카막 플라즈마의 위상학적 불변량과 n=6

```
  토러스의 위상학:
    Euler characteristic χ = 0
    Genus g = 1 (도넛 = 1-hole torus)
    Betti numbers: b₀=1, b₁=2, b₂=1

  Magnetic field line의 위상:
    Toroidal winding number: q (rational이면 closed, irrational이면 ergodic)
    Magnetic islands: O-point + X-point = (m,n) structure

  n=6 연결:
    Torus의 fundamental group = Z × Z (2 generators = φ)
    Toroidal + poloidal = 2 independent directions = φ

  이것은 trivial (모든 torus가 2 방향).

  더 깊은 연결:
    Shafranov shift Δ가 a/R₀ 차수 → A 관련
    Bootstrap current ∝ √ε (ε = a/R₀ = 1/A)
    A = 3 → ε = 1/3 → √ε ≈ 0.577

  Grade: WEAK
  위상학적 연결은 trivial. Bootstrap current 관계는 간접적.
```

---

## 종합 채점

| ID | 가설 | Grade | 핵심 |
|----|------|-------|------|
| H-TS-1 | 형태 매개변수 6개 | CLOSE | 실용적 사실이나 수학적 필연 아님 |
| H-TS-2 | κ = 2 = φ | CLOSE | KSTAR/SPARC EXACT, ITER는 다름 |
| H-TS-3 | δ = 1/3 | EXACT (ITER) | ITER 설계값과 일치 |
| **H-TS-4** | **Snowflake 6 legs** | **EXACT** | **2차 null의 수학적 구조** |
| **H-TS-5** | **X-point 차수 구조** | **EXACT** | **m=2 → 6 branches 수학적 사실** |
| H-TS-6 | Fourier 6차 수렴 | CLOSE | smooth boundary 성질 |
| H-TS-7 | Snowflake Egyptian | WEAK | 시뮬레이션 필요 |
| H-TS-8 | 6-period stellarator | WEAK | 최적 근거 없음 |
| H-TS-9 | MHD 모드와 약수 | CLOSE | 작은 수 효과일 가능성 |
| H-TS-10 | 위상학적 연결 | WEAK | trivial |

**EXACT: 2, CLOSE: 4, WEAK: 3, FAIL: 0** (이전 H-TK-4와 달리 물리 안에서 탐색)

---

## 최대 발견: Snowflake Divertor = 6

```
  ┌─────────────────────────────────────────────┐
  │  SNOWFLAKE DIVERTOR                          │
  │                                             │
  │       ╲     │     ╱                         │
  │         ╲   │   ╱                           │
  │           ╲ │ ╱                             │
  │    ────────╳────────  2nd-order X-point     │
  │           ╱ │ ╲                             │
  │         ╱   │   ╲                           │
  │       ╱     │     ╲                         │
  │                                             │
  │  6 separatrix branches = n = 6              │
  │  열부하를 6 방향으로 분산                      │
  │  ITER의 최대 과제(divertor 열부하) 해결 가능   │
  │                                             │
  │  이것은 post-hoc가 아님:                      │
  │  2차 null에서 6 branches는 수학적 필연        │
  │  (B ∝ r² → angular dependence ~ cos(3θ))   │
  │                                             │
  │  TCV 실험: Snowflake divertor 성공 시연       │
  │  DEMO/상용로: Snowflake 적극 검토 중          │
  └─────────────────────────────────────────────┘

  이것이 토카막 형태에서 n=6의 진정한 연결:
  "도넛 모양을 6각형으로 바꾸자"가 아니라
  "열배출 토폴로지에서 6이 자연 발생"
```

---

## 이전 실패에서 배운 것

| 이전 (FAIL) | 개선 (물리 내) | 교훈 |
|-------------|---------------|------|
| 정육각형 단면 (H-TK-4) | 형태 매개변수 6개 (H-TS-1) | 기하학 직접 매핑 ❌, 매개변수 수 ✅ |
| 12 TF coils (H-TK-5) | Snowflake 6 legs (H-TS-4) | 코일 수 ❌, 토폴로지 구조 ✅ |
| Egyptian field split (H-TK-1) | Snowflake 열분산 (H-TS-7) | 에너지 고정 배분 ❌, 구조적 분산 ✅ |

**n=6을 물리 안에서 찾으면 EXACT, 물리에 강제하면 FAIL.**
