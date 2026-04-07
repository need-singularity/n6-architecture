# H-CERN-7: Aberration Profile of Standard Model Particles

> **Thesis**: 각 SM 입자 질량을 정수 n에 매핑했을 때, gravitational_optics.py의
> 5가지 수차(aberration) 프로파일이 입자의 물리적 특성(스핀, 전하, 색전하)과
> 체계적으로 대응한다. 특히 chromatic aberration이 색전하와 연결된다.

**Golden Zone 의존성**: 있음

## 1. Background

### 5가지 수차 유형

```
  수차 유형           정의                        물리 대응 (가설)
  ───────────        ────────────────            ──────────────────
  Chromatic          R(n) - 1                    색전하 (color charge)
  Spherical          |R(n) - R(n±1)|             질량 안정성
  Astigmatic         R(n) 방향 의존              스핀-질량 결합
  Coma               d(R)/dn 비대칭              CP 위반
  Distortion         R(n²) vs R(n)²             세대 간 비선형성
```

### 입자→정수 매핑

M_unit = m_π⁰ = 0.135 GeV (가장 가벼운 하드론, 자연 스케일)

```
  Particle     M(GeV)    n=M/M_π    Spin   Q    Color
  ──────────   ──────    ──────     ────   ──   ─────
  π⁰           0.135     1          0      0    no
  π±           0.140     1          0      ±1   no
  K±           0.494     4          0      ±1   no
  ρ            0.770     6          1      0    no     ★ n=6!
  ω            0.783     6          1      0    no     ★ n=6!
  p            0.938     7          1/2    +1   no
  φ(1020)      1.020     8          1      0    no
  J/ψ          3.097     23         1      0    no
  Υ            9.460     70         1      0    no
  Z            91.188    675        1      0    no
  W            80.379    595        1      ±1   no
  H            125.25    928        0      0    no
```

## 2. Critical Observation: ρ/ω = n=6

```
  ρ(770)/m_π = 5.70 ≈ 6
  ω(783)/m_π = 5.80 ≈ 6

  ★ 가장 가벼운 벡터 메존의 질량이 π 단위로 ~6 = 완전수!

  R(6) = 1 (perfect lens, zero aberration)
  → ρ/ω 메존은 "수차 없는 완벽한 렌즈" 위치에 존재
  → 이들이 벡터 메존 중 가장 기본적/안정적인 이유?
```

## 3. Aberration Table

```
  n     R(n)      Chrom    Spher    Particle     Physical Property
  ──    ──────    ──────   ──────   ──────────   ──────────────────
  1     1/1       0.000    0.500    π⁰           Goldstone boson
  4     7/8       -0.125   0.268    K±           Strange (s quark)
  6     1/1       0.000    0.071    ρ/ω          ★ Perfect lens
  7     15/14     0.071    0.196    proton       Stable baryon
  8     15/16     -0.063   0.134    φ(1020)      ss-bar (hidden s)
  23    12/11     0.091    0.091    J/ψ          cc-bar (hidden c)
  70    72/56     0.286    0.036    Υ            bb-bar (hidden b)
  675   —         —        —        Z            Weak neutral
  928   —         —        —        H            Higgs scalar
```

### ASCII: Chromatic Aberration vs Particle Mass

```
  Chrom |
   0.3  +                               * Υ(70)
        |
   0.2  +
        |
   0.1  +  * p(7)        * J/ψ(23)
        |
   0.0  +--*π(1)---*ρ/ω(6)---------------------------→ n
        |         ↑
  -0.1  +  * K(4)  ZERO = perfect lens
        |    * φ(8)
  -0.2  +
```

## 4. Key Predictions

| # | Prediction | Mechanism | Status |
|---|---|---|---|
| 1 | ρ/ω가 n=6 (R=1)에 위치 | perfect lens = 가장 기본적 벡터 메존 | TO VERIFY |
| 2 | quarkonium (cc,bb)은 양의 chromatic | hidden flavor → R>1 | TO VERIFY |
| 3 | strange 입자는 음의 chromatic | s quark → R<1 | TO VERIFY |
| 4 | 스핀 0 입자의 spherical aberration > 스핀 1 | scalar vs vector | TO VERIFY |
| 5 | CP 위반 입자에서 coma 수차 비대칭 | K⁰ system | TO VERIFY |

## 5. Verification Commands

```bash
# 각 n에 대한 full aberration profile
python3 calc/gravitational_optics.py --lens --n 6 --full
python3 calc/gravitational_optics.py --lens --n 7 --full
python3 calc/gravitational_optics.py --lens --n 23 --full
python3 calc/gravitational_optics.py --lens --n 70 --full

# Range scan으로 전체 조감
python3 calc/gravitational_optics.py --lens --range 1-100

# Perfect number 비교
python3 calc/gravitational_optics.py --lens --perfect --compare
```

## 6. Limitations

- M_unit 선택에 따라 n 매핑 완전히 변경 (m_π vs m_μ vs 1 GeV)
- 반올림 오차: M/M_π가 정확히 정수가 아님 (ρ = 5.70, not 6)
- 수차-물리특성 대응은 현재 정성적(qualitative)
- 84개 입자 중 정수에 가까운 것만 선별하면 선택 편향

## 7. Next Steps

1. M_unit sensitivity 분석: m_π, m_μ, ΛQCD, 1 GeV 각각에서 매핑
2. 84개 PDG 입자 전체 aberration 프로파일 생성
3. 수차 유형 vs 양자수(J, Q, I, S, C, B) 상관 행렬
4. Texas Sharpshooter: random mass set 대비 n=6 근처 집중 유의성
