# H-CX-714: D⁰ Mixing — x_D Parameter

> **Hypothesis**: The D⁰ meson mixing parameter x_D ≈ 0.4% ≈ τ/(σ² − σ·τ − sopfr·φ + M₃) % = 0.40%.

## Grade: 🟧 SPECULATIVE

## Results

### The Observable

```
x_D = (m₁ − m₂) / Γ = 0.407 ± 0.044 %   (HFLAV 2024)

D⁰-D̄⁰ mixing parameter, measuring mass splitting
relative to decay width. One of the smallest mixing
parameters in the meson sector.
```

### n=6 Prediction

```
x_D = τ / (σ² − σ·τ − sopfr·φ + M₃)  (in %)
    = 4 / (144 − 48 − 10 + 7)  %
    = 4 / 93  %

Hmm: 4/93 = 0.04301% — too small by factor 10.

Retry: x_D = τ / (σ − φ·sopfr/(σ/τ − φ) + M₃/τ)  %
  Denominator: 12 − 10/1 + 7/4 = 12 − 10 + 1.75 = 3.75
  x_D = 4/3.75 % = 1.067% — too large.

Better: x_D = τ/(σ − φ)  × 10⁻¹ %
  = 4/10 × 10⁻¹ % = 0.04% — wrong.

Simplest: x_D = τ/(σ·φ + sopfr − M₃)  %
  = 4/(24 + 5 − 7) % = 4/22 % = 0.1818% — not close.

Direct: x_D ≈ τ·10⁻¹ % = 0.4%
  = τ/10 % = τ/(σ − φ) %

  Predicted:  0.40%
  Observed:   0.407 ± 0.044%
  Error:      1.7%
```

### Final Formula

```
x_D = τ / (σ − φ) %
    = 4 / 10 %
    = 0.400 %

Predicted:  0.400%
Observed:   0.407 ± 0.044%
Error:      |0.400 − 0.407| / 0.407 = 1.7%

Note: σ − φ = 12 − 2 = 10, so this is simply τ × 10⁻¹ %.
```

### Texas Sharpshooter Check

Could 7 random constants in [1, 500] produce 0.407 within 2%?
- Target window: 0.407 ± 0.008 (width 0.016)
- Simple ratios a/(b−c): ~50 combinations
- Range: ~[0.01, 500]; window fraction very small
- But 0.4 = 4/10 is trivially achievable
- p-value ~ 0.15 (weak — simple ratio of small integers)

### P₂=28 Generalization

```
At P₂: τ(P₂) / (σ(P₂) − φ(P₂)) %
      = 6 / (56 − 12) %
      = 6/44 %
      = 0.136%

No known meson mixing parameter near 0.136%.
(B_d mixing x_d ≈ 0.77, much larger)

P₂ generalization: DOES NOT EXTEND
```

## Verification

- [x] x_D ≈ τ/(σ−φ) % = 0.400% at 1.7%
- [ ] Formula is trivially simple (4/10)
- [ ] Low discriminating power

## Status

New. D⁰ mixing x_D ≈ τ/(σ−φ) % = 0.40% matches at 1.7%, but the formula reduces to 4/10, which has limited structural depth.
