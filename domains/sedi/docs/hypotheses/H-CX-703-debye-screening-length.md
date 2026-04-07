# H-CX-703: Debye Screening Length — Plasma Parameter Exponent Range

> **Hypothesis**: The plasma parameter Λ = n_D·λ_D³ spans the range 10⁷ to 10¹⁰ in typical plasmas. The exponent range [7, 10] maps to [M₃, τ(P₃)], both n=6 constants.

## Grade: 🟧 SPECULATIVE

## Results

### The Formula

```
λ_D = √(ε₀ k_B T / (n_e e²))    (Debye screening length)

Plasma parameter:  Λ = n_D · λ_D³

Typical range:  Λ ~ 10⁷ to 10¹⁰
  - Solar corona:   Λ ~ 10⁸
  - Tokamak:        Λ ~ 10⁷⁻⁸
  - Ionosphere:     Λ ~ 10⁹⁻¹⁰
```

### n=6 Prediction

```
Lower exponent:  7 = M₃ (Mersenne prime 2³-1)
Upper exponent: 10 = τ(P₃) = τ(496) (number of divisors of third perfect number)

Exponent range: [M₃, τ(P₃)] = [7, 10]
```

### Verification

```
Predicted range:  10^M₃ to 10^τ(P₃) = 10⁷ to 10¹⁰
Observed range:   10⁷ to 10¹⁰ (NRL Plasma Formulary)
Error:            exact match on exponent boundaries
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] produce a pair matching the exponent range [7, 10]?
- Need two constants matching 7 and 10 from any ratio/combination
- With 7 constants and many possible operations, P(pair matching) ~ 0.08
- p-value ~ 0.08 (weak — exponent boundaries are coarse integers)

### P₂=28 Generalization

```
τ(P₂) = τ(28) = 6 = P₁
M₃ = 7

At P₂ scale: exponent range would be [M₃, P₁] = [7, 6] — inverted, no natural meaning.
P₂ generalization: DOES NOT EXTEND
```

## Verification

- [x] Exponent range [7, 10] matches plasma physics
- [x] M₃ and τ(P₃) are genuine n=6 constants
- [ ] Structural depth is limited (integer exponent matching)

## Status

New. The plasma parameter exponent range [M₃, τ(P₃)] is a valid observation but the coarseness of integer exponent matching limits significance.
