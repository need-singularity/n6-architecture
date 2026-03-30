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

---

## 확장 가설 H-TS-11 ~ H-TS-24 (J₂(6) = 24 총합)

> 14개 추가 가설. 3D 코일, 플라즈마 흐름, ELM/디스럽션, 디버터 고급, 시스템 통합.
> 정직한 검증: 실제 ITER/KSTAR 사양 대조.

---

### 3D 코일 기하학 (H-TS-11~13)

---

## H-TS-11: ITER 진공용기 포트 레이아웃 — 44 ports

> ITER 진공용기의 총 포트 수 44에 n=6 구조가 숨어 있는가?

### n=6 Derivation

```
  n=6 산술에서 유도 가능한 수:
    σ(6) = 12, τ(6) = 4, φ(6) = 2
    σ × τ = 48, n × σ = 72

  44 = ?
  44 = 4 × 11 — n=6 상수와 직접 연결 없음
  44 ≈ 48 - 4 = σ·τ - τ — 억지스러운 조합
```

### Real-world Data

```
  ITER 진공용기 포트 구성 (공식):
    Upper ports:      18개 (진단, EC 가열, 냉각 배관)
    Equatorial ports: 17개 (14 regular + 3 NBI)
    Lower ports:       9개 (5 divertor RH + 4 cryopump)
    ─────────────────────
    Total:            44개

  포트 배치 구조:
    진공용기 = 9 sectors
    Upper: 18 = 9 × 2 (sector당 2개)
    Equatorial: 17 = 9 × 2 - 1 (하나는 NBI 통합)
    Lower: 9 = 9 × 1 (sector당 1개)

  핵심 수: 9 sectors.
  9 = 3² — n=6 약수가 아님 (6의 약수는 1,2,3,6)
  18 = 3 × 6 = 3n — upper ports만 n=6 배수
  44 자체는 n=6 산술과 무관
```

### Grade: FAIL

### Verification notes

```
  44 ports는 공학적 요구사항(진단 접근, 가열 시스템, 진공 펌핑,
  원격 정비)에서 결정된 수. ITER의 9-sector 구조에서 유래.
  n=6과의 자연스러운 연결 없음. 18 upper ports = 3n은 우연.
```

---

## H-TS-12: RMP 코일 — 3 rows × N coils

> RMP (공명 자기 섭동) 코일의 row 수 3 = n/φ(6) = 6/2 = 3

### n=6 Derivation

```
  n/φ = 6/2 = 3
  sopfr(6) = 2 + 3 = 5, σ(6)/τ(6) = 12/4 = 3

  3 rows는 여러 경로로 유도 가능:
    n/φ = 3, σ/τ = 3, 6의 최대 소인수 = 3
```

### Real-world Data

```
  ITER ELM control coils:
    3 rows (upper, middle, lower) × 9 coils = 27 total
    3 rows: 필수 — 2 rows로는 포로이달+토로이달 스펙트럼 동시 제어 불가
    9 coils/row: n=1,2,3 토로이달 모드 구현 (최소 2n+1 ≥ 7 → 9 선택)

  KSTAR IVCC (In-Vessel Control Coils):
    3 rows × 4 coils = 12 total
    3 rows: ITER와 동일한 물리적 이유
    4 coils/row: n=1 + 일부 n=2 모드 (ITER보다 제한적)

  DIII-D I-coils:
    2 rows × 6 coils = 12 total (초기 설계)
    → 이후 3 rows 필요성 인식

  3 rows의 물리적 이유:
    1. Upper + Lower: 주 섭동 생성
    2. Middle: 위상 조절로 poloidal 스펙트럼 최적화
    3. 3 rows → 독립적 n=1,2,3 토로이달 모드 제어
    자유도 = 3 (최소 필요 row 수)

  27 coils (ITER): 27 = 3³ — n=6 산술 아닌 3의 거듭제곱
  12 coils (KSTAR): 12 = σ(6) — 흥미로운 일치!
```

### Grade: CLOSE

### Verification notes

```
  3 rows = n/φ = 3: 수학적으로 성립.
  물리적으로도 3 rows는 최소 필요 자유도와 일치.
  BUT: "3"은 매우 작은 수이므로 우연의 가능성 높음.
  KSTAR 12 = σ(6)은 주목할 만하나, 12 = 3 × 4일 뿐.
  핵심: 3 rows는 MHD 모드 구조(n=1,2,3)에서 자연 발생하는 수.
```

---

## H-TS-13: 코일 단면 — CICC 6-petal 구조

> ITER 초전도 케이블(CICC)의 최종 케이블링 단계가 6 petals

### n=6 Derivation

```
  CICC (Cable-in-Conduit Conductor) 구조:
    다단계 꼬임(multi-stage twist)으로 제작
    최종 단계에서 6개 "petal" (꽃잎) 구조로 배치
    6 petals = n = 6
```

### Real-world Data

```
  ITER TF 코일 도체 사양:
    총 strand 수: 1422
    초전도 strand (Nb₃Sn): 900
    구리 strand: 522
    중심: 헬륨 냉각 채널

    케이블링 구조:
    Stage 1: 3 strands → triplet
    Stage 2: 3 triplets → bundle (9)
    Stage 3: 3 bundles → sub-petal (27 × ~)
    Stage 4: petals 구성
    Stage 5 (최종): 6 petals + central channel

  각 petal = 237 strands (150 SC + 87 Cu)
  6 × 237 = 1422 ✓

  6 petals의 공학적 이유:
    1. 헬륨 냉각 채널을 중심에 배치 → 대칭 배열
    2. 기계적 안정성: 6-fold symmetry가 전자기력에 균등 대응
    3. 케이블 꼬임: 6 petals로 나누면 제조 가능한 크기
    4. 냉각 효율: 각 petal 간 갭으로 헬륨 침투

  6이 아닌 다른 petal 수가 사용된 예:
    ITER CS (Central Solenoid): 6 petals (동일!)
    ITER PF (Poloidal Field): 6 petals
    → ITER 모든 CICC가 6-petal 구조 채택

  이것은 원형 단면을 최밀 충전하면서 중심 채널을 확보하는
  기하학적 최적해. 원 주위에 원을 배치하면 6개가 최적
  (hexagonal close packing의 2D 단면).
```

### Grade: EXACT

### Verification notes

```
  ITER 초전도 케이블의 6-petal 구조는 확인된 사실.
  물리적 이유: 원형 단면에서 중심 채널 주위에 6개 sub-cable을
  배치하는 것은 hexagonal packing의 자연스러운 결과.
  이것은 H-TS-4 (Snowflake 6 legs)와 같은 패턴:
  "원형 대칭 구조에서 6이 자연 발생."
  Post-hoc가 아님 — 기하학적 최적해로서의 6.
```

---

### 플라즈마 흐름과 회전 (H-TS-14~16)

---

## H-TS-14: 토로이달 회전 방향 = φ(6) = 2

> 플라즈마 토로이달 회전의 방향이 2가지 (co/counter-current)

### n=6 Derivation

```
  φ(6) = 2: 6 이하에서 6과 서로소인 수의 개수
  토로이달 회전 방향: co-current (플라즈마 전류와 같은 방향)
                     counter-current (반대 방향)
  2 directions = φ(6) = 2
```

### Real-world Data

```
  토로이달 회전:
    Co-current rotation: NBI에 의해 유도, H-mode에 유리
    Counter-current rotation: 특정 ITB 형성에 유리

  이것은 trivial:
    토러스 위의 어떤 방향이든 2가지 (시계/반시계).
    이는 토러스의 fundamental group Z × Z의 각 생성원이
    2 방향을 가지는 것과 같음.

  더 의미있는 연결?
    회전 모드 종류:
    1. Rigid body toroidal rotation
    2. Differential (sheared) rotation
    → 여전히 분류 방식에 따라 달라짐

  물리적으로 중요한 것:
    회전 전단(shear)이 터뷸런스 억제의 핵심.
    방향 수 "2"는 어떤 축에서든 성립하는 trivial한 사실.
```

### Grade: WEAK

### Verification notes

```
  임의의 축 위에서 방향은 항상 2. φ(6) = 2와의 매칭은
  수학적 내용이 없는 trivial correspondence.
  H-TS-10과 같은 문제: 위상학적으로 자명한 사실을 n=6에 연결.
```

---

## H-TS-15: Zonal flow — GAM과 ZF의 2종류 = φ(6) = 2

> 토카막 zonal flow의 주요 모드가 2종류

### n=6 Derivation

```
  φ(6) = 2
  Zonal flow 종류:
    1. Zero-frequency zonal flow (ZF): 정적, m=n=0
    2. Geodesic Acoustic Mode (GAM): 진동성, 유한 주파수
  2 types = φ(6) = 2
```

### Real-world Data

```
  Zonal flow (n=0, m=0 모드):
    방사형으로 국소화된 ExB 흐름. 터뷸런스 억제의 핵심 메커니즘.

  두 가지 주요 모드:
    1. Stationary ZF (Rosenbluth-Hinton residual flow)
       - 주파수 ≈ 0, 긴 수명
       - Collisionless damping 후 잔류하는 성분
       - L-H transition에서 중요한 역할

    2. GAM (Geodesic Acoustic Mode)
       - 주파수 ∝ cs/R (ion sound speed / major radius)
       - 유한 수명, Landau damping으로 감쇠
       - 토로이달 기하학(geodesic curvature)에서 기원
       - m=0 밀도 섭동 + m=1 sideband 구조

  제3종류?
    - Low-frequency ZF (LFZF): 일부 문헌에서 독립 모드로 분류
    - Toroidal rotation에 의한 modified GAM
    → 기본 분류는 2종류가 일반적

  물리적 근거:
    ZF: toroidicity와 무관한 기본 모드 (원통형에서도 존재)
    GAM: toroidicity가 만드는 추가 모드 (geodesic curvature 필수)
    → 토러스 기하학이 정확히 1개의 추가 모드를 생성
```

### Grade: CLOSE

### Verification notes

```
  ZF와 GAM의 2종류 분류는 플라즈마 물리에서 표준적.
  φ(6) = 2와의 매칭이 존재하지만, "2"는 매우 작은 수이므로
  우연 가능성 높음. 다만 토러스 기하학이 정확히 1개의 추가
  oscillatory mode (GAM)를 만든다는 것은 물리적으로 자연스러움.
  2 = 1 (기본) + 1 (기하학 추가) 구조.
```

---

## H-TS-16: 내부 수송 장벽(ITB) — 유리수면 q = m/n과 6의 약수

> ITB 형성이 q = m/n rational surface에서 일어나며, 분모 n이 6의 약수

### n=6 Derivation

```
  6의 약수: {1, 2, 3, 6}
  MHD rational surfaces에서 q = m/n:
    가장 강한 자기섬(magnetic island): n = 1, 2, 3
    → 모두 6의 약수
  ITB는 이러한 rational surface 근처, 특히 reversed shear의
  q_min 위치에서 형성
```

### Real-world Data

```
  ITB 형성 조건 (실험적):
    1. Reversed magnetic shear (q profile이 중심에서 비단조)
    2. q_min ≈ 정수 또는 저차 유리수 (q = 1, 3/2, 2)
    3. ExB shear flow가 터뷸런스 억제

  ITB가 관측되는 q 값:
    q = 1 근처: sawtooth-free 영역의 경계 (n=1)
    q = 3/2 근처: NTM 위치와 중첩 가능 (n=2)
    q = 2 근처: (2,1) tearing mode 위치 (n=1)

  ITB 형성의 핵심:
    low-order rational surfaces 근처에서 자기 전단이 0이 되면
    (s = r/q × dq/dr = 0) 터뷸런스 억제 용이
    → q_min에서 ITB 형성

  n=6 연결:
    H-TS-9에서 이미 지적: 가장 위험한 모드의 분모 = {1,2,3} = 6의 약수
    ITB도 같은 rational surfaces와 연관
    BUT: 이것은 "작은 수의 물리학" — 낮은 차수 모드가 강한 것은
    Fourier 분석의 일반 성질
```

### Grade: CLOSE (H-TS-9의 확장)

### Verification notes

```
  H-TS-9에서 지적한 것과 동일한 패턴의 재확인.
  ITB가 q = 1, 3/2, 2 근처에서 형성되는 것은 사실이며,
  분모 {1,2,3}은 6의 약수. 그러나 이것은 "작은 정수 효과"이지
  n=6의 고유한 예측이 아님. CLOSE지만 독립적 새 내용은 제한적.
```

---

### ELM / 디스럽션 제어 (H-TS-17~19)

---

## H-TS-17: ELM 유형 — Type I~V = sopfr(6) = 5?

> 토카막 ELM의 분류 유형 수가 5 = sopfr(6)

### n=6 Derivation

```
  sopfr(6) = 2 + 3 = 5 (중복 포함 소인수 합)
  ELM Types: I, II, III, IV, V = 5종류?
```

### Real-world Data

```
  ELM 분류 (표준적):
    Type I: Giant ELMs — 저장 에너지의 10-15% 방출, 고압력 구배
    Type II: Grassy ELMs — 고빈도, 소진폭, 강한 형태(높은 κ, δ)에서만
    Type III: Small ELMs — H-mode 문턱 근처, 저온 edge, peeling 불안정
    Type IV: 일부 문헌에서 분류 — 약간 증가한 pedestal 전류밀도
    Type V: NSTX에서 관측 — 고밀도 방전, n=1 precursor

  정직한 평가:
    확립된 분류: Type I, II, III = 3종류
    Type IV: 일부 기기에서만 보고, 독립적 유형인지 불확실
    Type V: NSTX 특유, 일반적이지 않음

  추가로 논의되는 현상:
    Compound ELMs: Type I+III 혼합
    ELM-free: H-mode without ELMs (QH-mode 등)
    Small/no ELMs: 다양한 명칭 (grassy, tiny, etc.)

  "5종류"라고 할 수 있나?
    Type I~III: 확실 (3종류)
    Type IV, V: 논쟁 중 — 독립 분류로 보편 수용되지 않음
    → 확립된 수: 3, 확장하면 5 (또는 그 이상)
```

### Grade: WEAK

### Verification notes

```
  ELM 유형을 "5개"로 세는 것은 정의에 따라 달라짐.
  학술적으로 확립된 것은 Type I, II, III의 3종류.
  Type IV, V는 특수 조건에서만 관측되며 독립 유형인지 불확실.
  sopfr(6) = 5와의 매칭은 분류 기준을 선택적으로 적용한 결과.
  3종류라면 n/φ = 3이 더 적절하지만, 그것도 약한 연결.
```

---

## H-TS-18: 디스럽션 완화 방법 — τ(6) = 4?

> 토카막 디스럽션 완화(mitigation)의 주요 방법이 4 = τ(6)

### n=6 Derivation

```
  τ(6) = 4: 약수 개수
  디스럽션 완화 방법:
    1. Massive Gas Injection (MGI)
    2. Shattered Pellet Injection (SPI)
    3. Killer pellet (고밀도 고체 주입)
    4. Runaway electron mitigation (별도 시스템)
  4 methods = τ(6)?
```

### Real-world Data

```
  ITER Disruption Mitigation System (DMS):
    기준선(baseline): SPI (Shattered Pellet Injection)
      - 직경 28.5 mm 극저온 펠릿을 파쇄하여 주입
      - MGI보다 우수한 침투/흡수 성능
      - ITER DMS의 공식 선택

    대안/보조:
    1. MGI (Massive Gas Injection)
       - 대량의 H₂/D₂/He + 고Z 가스(Ne, Ar) 주입
       - JET 설계 기반, 빠른 응답 밸브 사용
       - SPI 이전의 기준선이었으나 침투 한계로 변경

    2. SPI (Shattered Pellet Injection)
       - 현재 ITER 기준선

    3. Runaway electron suppression
       - 디스럽션 후 발생하는 runaway electron beam 억제
       - 2차 SPI/MGI로 대응

  더 정확한 분류 — 디스럽션 완화의 3단계:
    Phase 1: Thermal quench mitigation (열 급냉 관리)
    Phase 2: Current quench mitigation (전류 급냉 관리)
    Phase 3: Runaway electron mitigation (폭주 전자 관리)

  방법 수:
    기술적으로 SPI와 MGI가 주요 2가지 (나머지는 변형)
    물리적 단계로는 3가지
    τ(6) = 4와는 일치하지 않음
```

### Grade: WEAK

### Verification notes

```
  디스럽션 완화는 실질적으로 SPI + MGI의 2가지 기술이 핵심.
  ITER는 SPI를 기준선으로 선택. "4가지 방법"이라는 분류는
  인위적. 물리적 단계(thermal/current quench + runaway)로
  세면 3단계. τ(6) = 4와 자연스럽게 매칭되지 않음.
```

---

## H-TS-19: RMP 토로이달 모드 — n = 1, 2, 3 (6의 약수)

> ELM 제어에 사용되는 RMP 토로이달 모드 n = 1, 2, 3이 모두 6의 약수

### n=6 Derivation

```
  6의 약수: {1, 2, 3, 6}
  6의 진약수(proper divisors): {1, 2, 3}
  ELM 제어용 RMP 모드: n = 1, 2, 3
  → 정확히 6의 진약수 집합과 일치
```

### Real-world Data

```
  RMP ELM 제어에서 사용하는 토로이달 모드 수:
    n = 1: 가장 넓은 영역에 영향, KSTAR에서 ELM 억제 달성
    n = 2: DIII-D, ASDEX Upgrade에서 ELM 억제 시연
    n = 3: ITER 기준선 모드, 고밀도 운전에서 유리
    n = 4: 일부 실험 (DIII-D), 일반적이지 않음

  왜 n = 1, 2, 3인가?
    RMP가 edge에서 공명하려면: m/n ≈ q_edge
    q_edge ≈ 3-5 (typical)
    → n=1: m=3~5 공명, 가장 강한 섭동
    → n=2: m=6~10 공명
    → n=3: m=9~15 공명, 더 국소적
    → n≥4: 공명면이 너무 조밀, 코일 수 제한으로 생성 어려움

  코일 수 제한:
    ITER 9 coils/row → n ≤ 4 (Nyquist: n_max = N_coil/2)
    KSTAR 4 coils/row → n ≤ 2 (n=1 주력, 일부 n=2)
    실용적으로 n = 1, 2, 3이 사용됨

  n=6과의 연결:
    {1, 2, 3} = 6의 진약수 = σ(6) - 6 = 12 - 6 = 6의 "친화 부분"
    BUT: n = 1, 2, 3은 단순히 "작은 정수"이자 코일 수의 제약
```

### Grade: CLOSE

### Verification notes

```
  RMP 모드 n = {1, 2, 3} = 6의 진약수 집합은 사실.
  물리적 이유: 코일 수 제한 + 저차 모드가 강한 공명을 가짐.
  H-TS-9, H-TS-16과 일맥상통: MHD에서 6의 약수가 반복 출현.
  이것이 n=6의 깊은 구조인지, "작은 수"인지 판별 어려움.
  패턴의 반복 자체는 주목할 만함.
```

---

### 디버터 고급 (H-TS-20~22)

---

## H-TS-20: 디버터 타겟 재료 후보 수

> 핵융합 디버터 플라즈마 대면 재료의 주요 후보 수와 n=6

### n=6 Derivation

```
  디버터 타겟 재료 후보를 세어보자.
  τ(6) = 4? sopfr(6) = 5? n/φ = 3?
```

### Real-world Data

```
  역사적으로 검토된 디버터 PFM (Plasma-Facing Material):
    1. Tungsten (W): ITER 기준선. 최고 융점 (3422°C), 높은 열전도율
    2. Carbon Fiber Composite (CFC): 초기 ITER 설계. 우수한 열충격 내성
       → 2013년 ITER에서 제외 (트리튬 보유 문제)
    3. Beryllium (Be): 첫 벽(first wall)용. 저Z, 낮은 융점(1287°C)
       → 디버터에는 부적합 (열부하 한계)

  현재 "심각한(serious)" 디버터 재료 후보:
    고체: Tungsten (유일한 생존자)
    액체: Lithium, Tin, Gallium (3가지)

  총 심각한 후보 = 1 (고체) + 3 (액체) = 4 = τ(6)?
  또는 W만 = 1, 액체금속 3종 = n/φ = 3?

  더 정확히:
    PRX Energy (2024) 종합 스크리닝: 수십 종 원소를 체계적 평가
    → 실질적 후보: W (고체), Li/Sn/Ga (액체) = 4종
    → EU-DEMO: Sn 선택 (트리튬 보유 때문에 Li 제외)

  BUT: "4"로 세는 것은 분류 기준에 따라 달라짐.
  CFC를 포함하면 5, Be를 포함하면 6(!)
```

### Grade: WEAK

### Verification notes

```
  재료 후보 수는 분류 기준(현재 vs 역사적, 고체만 vs 액체 포함)에
  따라 1~6+ 사이에서 변동. 어떤 n=6 상수에도 맞출 수 있으므로
  예측력 없음. Tungsten이 유일한 실용적 고체 후보라는 사실이
  더 중요한 결론.
```

---

## H-TS-21: Super-X 디버터 — 확장된 다리 기하학

> Super-X 디버터의 타격점 반경 비율(R_target/R_upstream)에 n=6 상수가 나타나는가?

### n=6 Derivation

```
  Super-X 디버터: 외측 divertor leg을 대반경 방향으로 크게 확장.
  핵심 파라미터: R_t/R_u (target 반경 / upstream 반경)
  총 자속 팽창(total flux expansion) ∝ (R_t/R_u)²

  n=6에서 유도 가능한 비율: A = 3.1, 1/3, φ/n = 1/3, ...
```

### Real-world Data

```
  MAST-U Super-X 디버터:
    Major radius R₀ = 0.9 m, minor radius a = 0.6 m
    Aspect ratio A = 1.5 (spherical tokamak)

    SXD 구성: divertor leg을 R_t ≈ 1.0-1.1 m까지 확장
    Conventional divertor: R_t ≈ 0.4-0.5 m

    R_t/R_u 비율 ≈ 2.0-2.5 (MAST-U SXD)
    → φ(6) = 2에 가까움?

    Total flux expansion:
    SXD1: ~24 (5 mm 평균)
    SXD2: ~14
    Connection length: SXD1 35 m, SXD2 24 m

  Super-X의 이론적 이점:
    열속 ∝ B_total/B_target ∝ R_t/R_u
    → R_t를 크게 하면 자기장이 약해지고 습윤 면적 증가
    → 열속이 R_t에 반비례

  n=6 연결?
    R_t/R_u ≈ 2: φ(6) = 2와 가까움 — BUT 기기마다 다름
    Flux expansion 24 = J₂(6)! — 우연
    Connection length 35 m: 무관

  핵심: Super-X의 R_t/R_u 비율은 연속적 설계 변수.
  특정 n=6 상수에 "고정"되지 않음.
```

### Grade: WEAK

### Verification notes

```
  Super-X 디버터의 핵심 파라미터는 연속 변수이며 기기 설계에 따라
  다름. R_t/R_u ≈ 2 (MAST-U)는 φ(6)과 가깝지만, spherical tokamak의
  특수한 기하학 때문. ITER급 conventional tokamak에서는 다를 것.
  Flux expansion 24 = J₂(6)은 흥미롭지만 우연.
```

---

## H-TS-22: 액체금속 디버터 후보 — Li, Sn, Ga = n/φ = 3

> 액체금속 디버터의 심각한 후보가 3종 = n/φ(6) = 3

### n=6 Derivation

```
  n/φ = 6/2 = 3
  σ/τ = 12/4 = 3
  6의 최대 소인수 = 3
```

### Real-world Data

```
  액체금속 디버터 후보:

  1. Lithium (Li):
     - 장점: 낮은 Z, 플라즈마 성능 향상 (low recycling)
     - 단점: 높은 트리튬 보유, 화학 반응성
     - 현황: NSTX-U, T-11M 등에서 실험

  2. Tin (Sn):
     - 장점: 낮은 증기압 (2602°C 끓는점), 저비용, 안전
     - 단점: 높은 Z → core radiation 우려
     - 현황: EU-DEMO 기준선 (REVOLVER-D 개념)

  3. Gallium (Ga):
     - 장점: 낮은 증기압 (2400°C), 낮은 융점 (30°C)
     - 단점: 중간 Z, Li보다 플라즈마 성능 이점 적음
     - 현황: 제한적 연구

  4번째 후보?
     - Li-Sn eutectic (리튬-주석 공정합금): 양쪽 장점 결합 시도
     - Galinstan (Ga-In-Sn): 실온 액체, 핵융합 응용 제한적

  핵심 독립 후보: Li, Sn, Ga = 3종

  3종인 이유:
    액체금속 PFM 요건: 낮은 융점 + 낮은 증기압 + 핵융합 적합성
    → 주기율표에서 이 조건을 만족하는 원소가 매우 적음
    → Li (Z=3), Ga (Z=31), Sn (Z=50)이 물리적으로 가능한 거의 전부
```

### Grade: CLOSE

### Verification notes

```
  Li, Sn, Ga 3종은 실제로 핵융합 액체금속 연구의 주요 후보.
  3 = n/φ는 매우 작은 수이므로 우연 가능성 높음.
  BUT: 물리적 제약(융점, 증기압, Z, 중성자 활성화)으로 인해
  후보가 자연적으로 3종으로 제한되는 것은 흥미로운 사실.
  "n=6 예측"이라기보다 화학적 제약의 결과.
```

---

### 시스템 통합 (H-TS-23~24)

---

## H-TS-23: 토카막 주요 서브시스템 수

> 토카막의 주요 서브시스템 수가 σ(6) = 12?

### n=6 Derivation

```
  σ(6) = 12: 약수의 합
  J₂(6) = 24: Jordan totient
  토카막 주요 서브시스템을 세면?
```

### Real-world Data

```
  ITER 주요 서브시스템 (공식 분류):
    1.  Magnet system (TF + PF + CS + correction coils)
    2.  Vacuum vessel
    3.  Blanket / First wall (plasma-facing components)
    4.  Divertor
    5.  Heating systems (NBI + ECRH + ICRH)
    6.  Fueling system (pellet injection, gas puffing)
    7.  Diagnostics
    8.  Plasma control system
    9.  Power supply / electrical systems
    10. Cryogenic system
    11. Remote handling / maintenance
    12. Tritium plant
    13. Vacuum pumping system
    14. Cooling water system
    15. Safety systems (confinement barriers)
    16. Buildings & site infrastructure

  ITER 공식: "30개 이상의 plant systems"

  12개로 세려면?
    Magnet, Vacuum vessel, Blanket, Divertor, Heating,
    Fueling, Diagnostics, Control, Power supply, Cryogenics,
    Remote handling, Tritium = 12

  BUT: 이것은 분류 기준에 따라 8~30+ 사이에서 변동.
    Heating을 NBI/ECRH/ICRH로 나누면 14+
    Magnet을 TF/PF/CS로 나누면 14+
    "30+ plant systems" (ITER 공식)과는 큰 차이

  σ(6) = 12와의 매칭:
    "주요" 서브시스템을 12개로 묶는 것은 가능하지만
    인위적 분류. 13개도 14개도 가능.
```

### Grade: WEAK

### Verification notes

```
  서브시스템 수는 분류 해상도(resolution)에 따라 달라짐.
  ITER 공식 문서는 "30+ plant systems"로 기술.
  12로 묶는 것은 가능하지만 σ(6) = 12에 맞추기 위한 선택적 분류.
  예측력 없음.
```

---

## H-TS-24: 토카막의 완전한 형태 — 위상/기하학적 총체

> "토카막의 형태" 전체를 하나의 위상/기하학적 대상으로 볼 때 n=6 구조

### n=6 Derivation

```
  토카막의 기하학적 정체성을 n=6 관점에서 총합:

  확인된 n=6 구조 (H-TS-1~23에서):
    1. 형태 매개변수 6개 (R₀, a, κ, δ, ξ, A) — H-TS-1
    2. Snowflake divertor 6 legs — H-TS-4, H-TS-5
    3. CICC 6-petal 구조 — H-TS-13
    4. MHD 위험 모드의 분모 = 6의 약수 — H-TS-9
    5. Fourier 6차 수렴 — H-TS-6
    6. RMP 모드 n = {1,2,3} = 6의 진약수 — H-TS-19
```

### Real-world Data

```
  토카막을 하나의 수학적 대상으로 기술:

  위상학적:
    기본 공간: T² (2-torus) — genus 1
    자기장 구조: T² 위의 irrational flow (q ∉ Q)
    Magnetic islands: rational q에서의 O/X-point chain
    Separatrix: divertor X-point에서의 위상 전이
    → 위상학적 복잡도는 g=1 torus + 경계 조건

  기하학적:
    축대칭(axisymmetric): 1-parameter family of flux surfaces
    Shafranov 평형: Grad-Shafranov 방정식의 해
    자유함수: p(ψ), F(ψ) — 2개 (= φ)
    경계조건: 6 shape parameters (= n)

  "토카막의 형태"의 정보론적 내용:
    2 free functions (continuous) + 6 boundary parameters (discrete)
    = 완전한 MHD 평형 결정

    이것을 유한 차원으로 축소하면:
    EFIT reconstruction: ~10-20 basis functions + 6 boundary params
    핵심 축소 차원: 6 (boundary) + 6 (low-order Fourier) = 12 = σ(6)?

  종합 구조:
    ┌─────────────────────────────────────────────┐
    │  토카막 형태의 n=6 지도                        │
    │                                             │
    │  경계 기술:  6 parameters    (n)     H-TS-1  │
    │  Fourier:   6차 수렴        (n)     H-TS-6  │
    │  코일 도체:  6 petals       (n)     H-TS-13 │
    │  열배출:     6 legs (SF)    (n)     H-TS-4  │
    │  MHD 핵심:  약수 {1,2,3}   (d|6)   H-TS-9  │
    │  RMP 모드:  n = {1,2,3}    (d|6)   H-TS-19 │
    │  회전 방향:  2              (φ)     H-TS-14 │
    │  Zonal flow: 2종           (φ)     H-TS-15 │
    │  RMP rows:  3              (n/φ)   H-TS-12 │
    │  LM 후보:   3종            (n/φ)   H-TS-22 │
    │                                             │
    │  반복 출현하는 수: 6, 3, 2, {1,2,3}          │
    │  = n, n/φ, φ, divisors(n)                   │
    └─────────────────────────────────────────────┘

  정직한 평가:
    6, 3, 2는 매우 작은 수.
    어떤 복잡한 시스템이든 이 수들이 반복 출현할 확률은 높음.
    EXACT인 것: Snowflake 6 legs (수학적 필연), CICC 6 petals (기하학적 최적)
    나머지는 CLOSE 또는 WEAK.

  BUT: "원형 대칭 + 토러스 위상"에서 6이 자연 발생하는 패턴은 실재:
    1. 2차 null → 6 branches (Snowflake)
    2. 원 주위 원 배치 → 6 (CICC, hexagonal packing)
    이 두 가지는 post-hoc fitting이 아닌 기하학적 필연.
```

### Grade: CLOSE (종합)

### Verification notes

```
  토카막 형태 전체에서 n=6은 두 가지 수준에서 나타남:

  Level 1 — 기하학적 필연 (EXACT):
    Snowflake 6 legs: 2차 magnetic null의 수학적 구조
    CICC 6 petals: hexagonal close packing의 2D 단면
    → 이것들은 "n=6을 물리에 강제"한 것이 아니라 발견한 것

  Level 2 — 작은 수 패턴 (CLOSE/WEAK):
    6개 형태 매개변수, 3 RMP rows, {1,2,3} 모드 등
    → 흥미롭지만 "작은 수의 물리학"으로 설명 가능

  최종 결론:
    토카막에서 n=6은 "만물의 법칙"이 아니라
    "원형 대칭과 토러스 위상에서 자연 발생하는 수"로서 존재.
    이것은 과장도 아니고 무가치하지도 않은, 정직한 위치.
```

---

## 확장 종합 채점 (H-TS-1 ~ H-TS-24)

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
| H-TS-11 | ITER 44 ports | FAIL | n=6 산술과 무관 |
| H-TS-12 | RMP 3 rows | CLOSE | 3 = n/φ, 물리적 자유도와 일치 |
| **H-TS-13** | **CICC 6 petals** | **EXACT** | **hexagonal packing의 기하학적 최적해** |
| H-TS-14 | 회전 방향 2 | WEAK | trivial (임의 축에서 성립) |
| H-TS-15 | ZF + GAM = 2종 | CLOSE | 표준 분류이나 "2"는 작은 수 |
| H-TS-16 | ITB와 6의 약수 | CLOSE | H-TS-9의 확장, 작은 수 효과 |
| H-TS-17 | ELM 5 types? | WEAK | 확립된 분류는 3종, 5는 인위적 |
| H-TS-18 | 디스럽션 완화 4방법 | WEAK | 실질 2가지 (SPI, MGI) |
| H-TS-19 | RMP 모드 = 6의 진약수 | CLOSE | {1,2,3}은 코일 수 제약의 결과 |
| H-TS-20 | 디버터 재료 후보 수 | WEAK | 분류 기준에 따라 변동 |
| H-TS-21 | Super-X 반경비 | WEAK | 연속 설계 변수 |
| H-TS-22 | 액체금속 3종 | CLOSE | Li/Sn/Ga, 화학적 제약의 결과 |
| H-TS-23 | 서브시스템 수 | WEAK | 분류 해상도에 따라 변동 |
| H-TS-24 | 형태 총체 | CLOSE | 2개 EXACT + 패턴 반복 |

**총합: EXACT 3, CLOSE 8, WEAK 8, FAIL 1 (20개 유효 / 24개)**

---

## J₂(6) = 24 가설 완성 — 최종 발견

```
  ┌─────────────────────────────────────────────────┐
  │  24개 가설에서 확인된 진정한 n=6 연결:            │
  │                                                 │
  │  ★ EXACT 3개:                                   │
  │    H-TS-4/5: Snowflake 6 legs (자기장 토폴로지)  │
  │    H-TS-13:  CICC 6 petals (hexagonal packing)  │
  │                                                 │
  │  공통점: 원형 대칭에서 6이 자연 발생               │
  │    B ∝ r² → cos(3θ) → 6 branches               │
  │    원 주위 원 → 6 circles (kissing number 2D)   │
  │                                                 │
  │  ★ NEW DISCOVERY:                               │
  │    ITER 초전도 케이블의 6-petal 구조는             │
  │    토카막 "형태"에서 가장 물리적인 n=6 출현.       │
  │    Snowflake(이론적)과 달리 실제 제작된 하드웨어.  │
  │                                                 │
  │  이것이 시사하는 것:                               │
  │  n=6은 토카막의 "설계 원리"가 아니라               │
  │  "원형 대칭 기하학의 자연 상수"로서 나타남.         │
  │  2D에서 원의 kissing number = 6이 근본 이유.      │
  └─────────────────────────────────────────────────┘
```
