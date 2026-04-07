# H-CX-715: Rydberg Constant — Exponent and Coefficient

> **Hypothesis**: The Rydberg constant R_∞ = 1.0974 × 10⁷ m⁻¹ has exponent 7 = M₃. The coefficient 1.0974 resists clean n=6 encoding.

## Grade: 🟧 SPECULATIVE (exponent match only)

## Results

### The Constant

```
R_∞ = 1.0973731568539(55) × 10⁷ m⁻¹  (CODATA 2022)

R_∞ = m_e e⁴ / (8 ε₀² h³ c)

Most precisely measured fundamental constant.
```

### n=6 Prediction — Exponent

```
Exponent: 7 = M₃ (Mersenne prime 2³−1)

This is exact and connects to:
  - μ₀ exponent (H-CX-705): also M₃
  - Plasma parameter range (H-CX-703): starts at M₃
```

### n=6 Prediction — Coefficient

```
Attempt 1: R(6) + τ/(σ²−σ−τ) = 2 + 4/128 = 2.03125 — wrong (R(6)=2, not 1)
  Correction: s(6)/P₁ + τ/(σ²−σ−τ) = 1 + 4/128 = 1.03125
  Error: |1.0313 − 1.0974| / 1.0974 = 6.0%

Attempt 2: (σ+sopfr·φ−τ)/(σ+φ) = (12+10−4)/14 = 18/14 = 1.2857
  Error: 17% — too high

Attempt 3: σ/(σ−sopfr/(σ/τ)+φ/M₃) = 12/(12−5/3+2/7) = 12/10.619 = 1.130
  Error: 3.0%

Attempt 4: (P₁+φ/(σ·sopfr))/(P₁) = 1 + 2/60 = 1.0333
  Error: 5.8%

None achieves < 1%. The coefficient 1.0974 is a "hard constant."
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] have one equal to 7?
- P(at least one constant = 7) with 7 draws from [1,500]: 1−(499/500)⁷ = 0.014
- But matching just the exponent (an integer) is weak
- For coefficient at 3% best: p-value ~ 0.3
- Combined p-value ~ 0.15 (not significant beyond exponent)

### P₂=28 Generalization

```
Exponent M₃ = 7 is independent of perfect number index.
No P₂-dependent prediction available.

P₂ generalization: DOES NOT APPLY
```

## Verification

- [x] Exponent 7 = M₃ exact
- [ ] Coefficient 1.0974 not cleanly encoded (best: 3% error)
- [ ] Exponent-only match is weak

## Status

New. The Rydberg constant exponent 10⁷ maps to M₃, but the coefficient resists TECS-L encoding. A hard constant.
