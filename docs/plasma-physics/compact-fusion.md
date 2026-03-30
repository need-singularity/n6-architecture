# 소형 핵융합 (Compact Fusion) — N6 설계 원리

> HTS 초전도 + 소형화 = 차세대 핵융합의 핵심 트렌드
> n=6 산술이 소형 핵융합로 설계에 어떻게 적용되는가?

---

## 소형 핵융합의 시대

```
  기존 (ITER 방식):      소형 (SPARC/ARC 방식):
  ─────────────────     ─────────────────────
  R₀ = 6.2 m           R₀ ≈ 1.85 m
  B_T = 5.3 T           B_T ≈ 12 T (HTS)
  P_fusion = 500 MW     P_fusion ≈ 140 MW
  비용 ~$25B            비용 ~$2-5B
  완공 2035+             완공 2028-2030

  핵심: 자기장 세기 ∝ B⁴ → 자기장 2배 = 성능 16배
  HTS(고온 초전도)가 게임 체인저
```

---

## 소형 핵융합 프로젝트와 n=6

### SPARC (MIT / Commonwealth Fusion Systems)

| 파라미터 | 값 | n=6 | Grade |
|----------|-----|-----|-------|
| Major radius R₀ | 1.85 m | ? | N/A |
| Minor radius a | 0.57 m | ~φ/τ? | CLOSE |
| Aspect ratio | 3.25 | n/φ=3 | CLOSE |
| Toroidal field | 12.2 T | **σ=12** | **EXACT** |
| TF coils | 18 | σ=12? | FAIL |
| Plasma current | 8.7 MA | σ-τ=8? | CLOSE |
| Q target | 11 (>10) | σ-μ=11 | **EXACT** |
| Heating: ICRH | 25 MW | J₂+μ=25? | CLOSE |
| Fusion power | 140 MW | ? | N/A |
| HTS material | REBCO | 1종 | N/A |

**SPARC 핵심 매칭**: B_T = 12T = σ (EXACT), Q > 10 (σ-μ=11 CLOSE)

### ARC (MIT concept, SPARC 후속)

| 파라미터 | 값 | n=6 | Grade |
|----------|-----|-----|-------|
| Major radius | 3.3 m | n/φ=3? | CLOSE |
| Aspect ratio | 3.0 | **n/φ=3** | **EXACT** |
| Toroidal field | 9.2 T | ? | N/A |
| Electric output | 525 MW_e | ? | N/A |
| FLiBe blanket | Li-6 based | **n=6** | **EXACT** |

### STEP (UK — Spherical Tokamak)

| 파라미터 | 값 | n=6 | Grade |
|----------|-----|-----|-------|
| Type | Spherical | 다른 기하학 | N/A |
| Aspect ratio | ~1.6-1.8 | n/φ=3? | FAIL |
| Electric output | 100 MW_e | ? | N/A |

### 기타 소형 핵융합 기업

| 기업 | 방식 | n=6 연관 |
|------|------|----------|
| **TAE Technologies** | FRC (Field-Reversed) | p-B11: 1+11→3×⁴He (σ-μ=11) |
| **Helion Energy** | FRC + D-He3 | D(2)+He3(3)→He4(4)+p(1) |
| **Zap Energy** | Z-pinch | 다른 가둠 방식 |
| **Tokamak Energy** | Spherical (HTS) | A~1.8 (FAIL for n/φ=3) |
| **General Fusion** | Magnetized target | 하이브리드 |

**관찰**: 6개 주요 소형 핵융합 기업 = n = 6? (WEAK — 세는 방법에 따라 다름)

---

## N6 소형 핵융합 설계 원리

### 원리 1: B⁴ 스케일링 + σ = 12

```
  핵융합 성능 ∝ β²N × B⁴T × R₀ (가장 단순한 스케일링)

  자기장이 가장 중요. n=6 예측:
    B_T = σ = 12 T (SPARC: 12.2 T → EXACT)

  12T는 HTS (REBCO)로 달성 가능한 실용 상한 근처:
    NbTi: ~9T (한계)
    Nb3Sn: ~16T (ITER coil max)
    REBCO: ~20T+ (이론적)
    12T = REBCO의 실용적 운전점

  n=6이 12T를 "예측"했다기보다,
  12T가 HTS의 sweet spot이고 이것이 σ=12와 일치.
```

### 원리 2: Aspect Ratio = n/φ = 3

```
  A = R₀/a = 3 이 소형 토카막의 최적 설계

  근거:
  - A < 2: spherical tokamak (bootstrap current 높지만 공학적 어려움)
  - A = 3: 전통 토카막 최적점 (MHD 안정성 + 접근성 균형)
  - A > 4: 플라즈마 압력 활용 비효율

  실제:
  - ARC: A = 3.0 (EXACT)
  - SPARC: A = 3.25 (CLOSE)
  - ITER: A = 3.1 (CLOSE)

  이것은 물리적으로 근거가 있는 매칭.
```

### 원리 3: 이중 냉각 (Egyptian Fraction)

```
  소형 핵융합로의 열 관리:

  제안: 냉각 에너지 배분 1/2 : 1/3 : 1/6

  1/2 → 블랭킷 냉각 (중성자 열 흡수, 삼중수소 증식)
  1/3 → 디버터 냉각 (플라즈마 배기 열)
  1/6 → 구조물/자석 냉각 (열 차폐)

  실제 ITER 열 배분:
  - 블랭킷: ~400 MW (열 ~80%)
  - 디버터: ~100 MW (~20%)
  → 80:20 ≈ 4:1, NOT 3:2:1

  Grade: FAIL (실제 배분과 불일치)
```

### 원리 4: Li-6 삼중수소 증식

```
  ⁶Li + n → T + ⁴He + 4.8 MeV

  Li-6의 질량수 = n = 6 (EXACT)
  생성물: T(3) + He4(4) = n/φ + τ
  에너지: 4.8 MeV ≈ sopfr - 0.2? (WEAK)

  삼중수소 증식비 (TBR) 목표: > 1.0 = R(6) = 1
  ITER TBR 설계: 1.05-1.15

  R(6) = 1 과 TBR = 1 의 의미적 유사성:
  - R = 1: 수론적 균형 (자기 유지)
  - TBR = 1: 삼중수소 자급자족 (자기 유지)
  둘 다 "self-sustaining" 조건!

  Grade: EXACT (Li-6 = n), 의미적 CLOSE (R=1 ↔ TBR≥1)
```

---

## 소형 핵융합 N6 설계안

```
  ┌─────────────────────────────────────────────────┐
  │           N6 COMPACT FUSION REACTOR              │
  │                                                 │
  │  Major radius: R₀ = n/φ = 3 m                  │
  │  Minor radius: a = μ = 1 m (A = 3)             │
  │  Toroidal field: B_T = σ = 12 T (HTS REBCO)    │
  │  PF coils: n = 6                                │
  │  CS modules: n = 6                              │
  │                                                 │
  │  Heating: n/φ = 3 methods                       │
  │    NBI:  σ-τ = 8 MW                             │
  │    ICRH: n = 6 MW                               │
  │    ECRH: sopfr-τ = 1 MW                         │
  │    Total: 15 MW                                 │
  │                                                 │
  │  Fuel: D(φ=2) + T(3) → He4(τ=4) + n(μ=1)      │
  │  Breeding: Li-6(=n) blanket, TBR = R(6) = 1+   │
  │                                                 │
  │  Plasma:                                        │
  │    T_i = sopfr×φ = 10 keV                       │
  │    κ = φ = 2 (elongation)                       │
  │    q_95 = sopfr = 5                             │
  │                                                 │
  │  Target: Q = sopfr×φ = 10                       │
  │  Confinement: H-mode (300s+ proven by KSTAR)    │
  └─────────────────────────────────────────────────┘
```

---

## 정직한 평가

**강한 매칭 (물리적 근거 있음)**:
- B_T = 12T = σ: HTS의 실용 sweet spot과 일치
- A = 3 = n/φ: MHD 안정성 최적점
- 가열 3방식 = n/φ: 물리적으로 3개가 최적
- Li-6 = n: 핵물리적 사실
- D-T = 2+3 = 6의 소인수: 핵물리적 사실

**약한/실패 매칭**:
- TF 코일 수: 실제로 12가 아닌 18 (SPARC, ITER 모두)
- 열 배분: Egyptian fraction과 불일치
- 연속 물리량(온도, 밀도 등): n=6로 예측 불가

**결론**: 소형 핵융합에서 n=6의 가장 강한 연결은 **B_T = σ = 12T**와 **A = n/φ = 3**. 둘 다 독립적인 물리적 최적화에서 도출되며, n=6과 우연히 일치.
