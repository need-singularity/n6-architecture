# H-CERN-4: Gravitational Lens on Particle Mass Spectrum

> **Thesis**: R-spectrum의 gravitational lens 구조가 CERN 디뮤온 불변 질량 스펙트럼의
> 공명(resonance) 가시성을 예측한다. 렌즈 강도 1/|M(n)|이 큰 정수 n에 대응하는
> 질량 영역에서 공명 피크가 더 선명하게 관측된다.

**Golden Zone 의존성**: 있음 (R-spectrum 해석은 Golden Zone 모델 기반)

## 1. Background

gravitational_optics.py의 렌즈 모드는 각 정수 n에 대해:
- R(n) = σ(n)φ(n)/(nτ(n)) — 산술 균형 비율
- M(n) = |σ(n)/n - 2| — abundancy deviation
- 렌즈 강도 = 1/|M(n)| (M=0일 때 무한 → perfect lens)

완전수 n=6에서 M(6)=0 → **완벽한 achromatic lens**.

CERN CMS 디뮤온 데이터(100k events)에서 관측되는 공명:
ρ/ω(0.77), φ(1020)(1.02), J/ψ(3.10), ψ(2S)(3.69), Υ(9.46), Z(91.19)

## 2. Correspondence Mapping

```
  R-spectrum (정수론)          CERN (입자물리)
  ─────────────────────       ──────────────────────
  n (정수)                     M_particle / M_unit (질량 비율)
  R(n) = 1 (완전수)            가장 선명한 공명
  M(n) → 0 (near-perfect)     높은 S/N 비율 피크
  렌즈 강도 1/|M(n)|           피크 선명도 (significance σ)
  수차 프로파일                 스펙트럼 비대칭성
  Einstein 반경                공명 폭 (Γ)
```

## 3. Quantitative Prediction

각 공명 질량을 기본 단위 M_unit으로 나누어 정수 n에 매핑:

```
  M_unit = m_μ = 0.10566 GeV (뮤온 질량, 디뮤온 실험의 자연 단위)

  Resonance    M(GeV)    n = M/M_unit    R(n)      M(n)      Lens Str
  ─────────    ──────    ────────────    ──────    ──────    ────────
  ρ/ω          0.770     7.29 → 7       15/14     1/14       14
  φ(1020)      1.020     9.65 → 10      9/8       1/4        4
  J/ψ          3.097     29.31 → 29     15/14     1/14       14
  ψ(2S)        3.686     34.89 → 35     36/35     2/35       17.5
  Υ(1S)        9.460     89.54 → 90     ★         ★          ★
  Z            91.188    863.07 → 863   —         —          —
```

### n=90 특별 분석

```
  90 = 2 × 3² × 5
  σ(90) = 234,  φ(90) = 24,  τ(90) = 12
  R(90) = 234×24/(90×12) = 5616/1080 = 26/5 = 5.2
  M(90) = |234/90 - 2| = |2.6 - 2| = 0.6
  Lens Str = 1/0.6 = 1.667
```

### ASCII: Lens Strength vs Resonance Significance

```
  Lens  |
  Str   |
  18 +  .                          ψ(2S)
  16 +
  14 +  * ρ/ω            * J/ψ
  12 +
  10 +
   8 +
   6 +
   4 +        * φ(1020)
   2 +                                     * Υ
   0 +──────────────────────────────────────────→ M(GeV)
       0    1    2    3    4    5    6   7  8  9  10
```

## 4. Verification Strategy

```bash
# Step 1: 각 공명 질량을 뮤온 단위로 변환, 가장 가까운 정수 n 매핑
python3 calc/gravitational_optics.py --lens --range 1-100 --json > lens_1_100.json

# Step 2: CERN 데이터에서 각 공명의 significance(σ) 측정
python3 analyze_cern_data.py  # 이미 피크 추출 포함

# Step 3: Lens Strength vs Peak Significance 상관관계
# Spearman r > 0.7이면 SUPPORTED
```

## 5. Expected Results

| Prediction | Criterion | Status |
|---|---|---|
| ρ/ω(n≈7)과 J/ψ(n≈29)가 비슷한 렌즈 강도 | Lens(7)≈Lens(29) | TO VERIFY |
| φ(1020)(n≈10)이 가장 약한 피크 | Lens(10) < Lens(others) | TO VERIFY |
| Lens Strength ↔ Peak σ 상관 r > 0.5 | Spearman test | TO VERIFY |
| 완전수 근처 n에서 미지의 공명 가능 | n=28 → M≈2.96 GeV | TO VERIFY |

## 6. Limitations

- 질량→정수 매핑에서 M_unit 선택이 자유도 (뮤온 외 다른 단위 시 결과 변경)
- 공명 가시성은 결합상수, 위상공간, 검출기 효율에 주로 의존
- R-spectrum 렌즈 강도가 실제 물리적 메커니즘을 가지는지 미검증
- Small number 효과: n < 100 범위에서 많은 "특별한" 정수 존재

## 7. Next Steps

1. M_unit 변화에 대한 robustness 테스트 (m_π, m_μ, 1 GeV 등)
2. PDG 84개 입자 전체로 확장
3. Lens 강도와 실험적 공명 폭(Γ) 비교
4. n=28(완전수) 근처 2.96 GeV 영역 CMS 데이터 정밀 스캔
