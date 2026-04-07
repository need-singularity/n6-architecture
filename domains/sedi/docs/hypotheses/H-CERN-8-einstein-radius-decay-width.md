# H-CERN-8: Einstein Radius Predicts Resonance Decay Width

> **Thesis**: R-spectrum gravitational lens의 Einstein 반경 θ_E(n)이
> 해당 정수 n에 매핑된 입자 공명의 붕괴폭(decay width Γ)과 비례한다.
> 렌즈가 강할수록(θ_E 클수록) 공명이 좁다(Γ 작다) — 역비례 관계.

**Golden Zone 의존성**: 있음

## 1. Background

### Einstein Radius in R-spectrum

gravitational_optics.py에서 Einstein 반경:

```
  θ_E(n) = sqrt(4 × |R(n) - 1| × D_LS / (D_L × D_S))

  Simplified (unit distances):
  θ_E(n) ∝ sqrt(|R(n) - 1|)

  n=6:  R=1  → θ_E = 0 (point focus = perfect imaging)
  n≠6:  R≠1  → θ_E > 0 (ring = imperfect imaging)
```

### Particle Decay Width

```
  Particle    M(GeV)     Γ(GeV)        Γ/M        Lifetime
  ────────    ──────     ──────        ──────      ────────
  ρ(770)      0.775      0.149         0.192       1.3 fm/c
  ω(783)      0.783      0.00849       0.011       23.4 fm/c
  φ(1020)     1.019      0.00426       0.004       44.4 fm/c
  J/ψ         3.097      0.0000929     3.0e-5      ~2000 fm/c
  ψ(2S)       3.686      0.000294      8.0e-5      ~700 fm/c
  Υ(1S)       9.460      0.0000540     5.7e-6      ~40000 fm/c
  Z           91.188     2.4952        0.027       ~0.1 fm/c
```

## 2. Lens-Width Correspondence

```
  n     R(n)     |R-1|    θ_E ∝ √|R-1|   Particle    Γ/M
  ──    ──────   ──────   ─────────────   ────────    ──────
  6     1.000    0.000    0.000           ρ/ω         0.192/0.011
  7     1.071    0.071    0.267           proton      stable!
  8     0.938    0.063    0.250           φ(1020)     0.004
  23    1.091    0.091    0.302           J/ψ         3.0e-5
  70    1.286    0.286    0.535           Υ           5.7e-6
```

### 핵심 패턴: θ_E 증가 → Γ/M 감소!

```
  θ_E   |
  0.6   +                              * Υ(70)
        |
  0.4   +
        |            * J/ψ(23)
  0.3   +  * p(7)
        |    * φ(8)
  0.2   +
        |
  0.1   +
        |
  0.0   +--* ρ/ω(6)  ─────────────────────→ θ_E
        0     0.1    0.2    0.3    0.4    0.5

  Γ/M   |
  0.20  +  * ρ(6)
        |
  0.10  +
        |
  0.01  +     * ω(6)    * φ(8)
        |
  1e-3  +
  1e-4  +                * J/ψ(23)
  1e-5  +                         * Υ(70)
  1e-6  +───────────────────────────────→ θ_E
        0     0.1    0.2    0.3    0.5
```

## 3. Interpretation

```
  θ_E = 0 (n=6, perfect lens):
    → 완벽한 초점 → 에너지를 빠르게 방출 → 넓은 폭 (ρ)
    → "완벽한 렌즈는 빛을 한 점에 집중 → 즉시 붕괴"

  θ_E > 0 (n≠6, imperfect lens):
    → Einstein 고리 → 에너지 분산 → 안정적 → 좁은 폭
    → "불완전한 렌즈는 에너지를 퍼뜨림 → 느린 붕괴"

  역설: 완전수 = perfect lens = LEAST stable (가장 큰 Γ/M)
  → 완벽함은 안정성이 아니라 투과성(transparency)을 의미
```

## 4. Quantitative Test

| Prediction | Test | Expected r | Status |
|---|---|---|---|
| θ_E ↔ 1/(Γ/M) 양의 상관 | Spearman | r > 0.7 | TO VERIFY |
| n=6 (θ_E=0)이 가장 넓은 공명 | ρ Γ/M 최대 | Γ_ρ/M_ρ > others | PLAUSIBLE |
| quarkonium series: θ_E 증가 순서 | J/ψ < Υ | θ_E(23) < θ_E(70) | ✓ 0.302 < 0.535 |
| Z boson 예외 (θ_E 큼 but Γ 큼) | 약력 → 다른 메커니즘 | Z는 outlier | EXPECTED |

## 5. ω vs ρ 분리 문제

```
  ρ(770):  Γ = 149 MeV  (매우 넓음)
  ω(783):  Γ = 8.49 MeV (좁음)

  둘 다 n≈6 매핑인데 폭이 17배 차이!
  → G-parity 보존(ω→3π, ρ→2π)으로 설명
  → 렌즈 모델은 동일 n → 동일 예측이므로 이 구분 불가
  → Limitation: 양자수 효과를 수차 모델이 포착 못함
```

## 6. Verification Commands

```bash
# Einstein 반경 계산
python3 calc/gravitational_optics.py --lens --n 6 --full
python3 calc/gravitational_optics.py --lens --n 23 --full
python3 calc/gravitational_optics.py --lens --n 70 --full

# 범위 스캔
python3 calc/gravitational_optics.py --lens --range 1-100 --json
```

## 7. Limitations

- ω vs ρ 문제: 같은 n에 매핑되나 Γ 17배 차이 → 모델 불완전
- Z boson: 약력 붕괴이므로 QCD lens 모델 적용 부적절
- 단 5-6개 data point → Spearman 검정력 낮음
- M_unit 선택 의존성 (다른 단위 시 n 변경)

## 8. Next Steps

1. PDG 전체 공명(100+ 상태)으로 확장
2. Γ/M vs θ_E 산점도 + power law fit
3. QCD 공명만 vs 전기약력 공명 분리 분석
4. Monte Carlo: random mass → random n → θ_E-Γ 상관 null distribution
