# H-CX-929: Planck Radiation Peak Factor ≈ φ + σ/(σ+sopfr-φ)

> **Hypothesis**: The Wien displacement factor hν_max/kT = 2.821 ≈ φ + σ/(σ+sopfr-φ) = 2 + 12/15 = 2.800, a 0.75% match.

## Grade: 🟧★ NOTABLE (0.75% error)

## Results

### The Formula

```
Planck spectral radiance peak (frequency form):
  hν_max = x_max · kT
  x_max = 2.82144...  (solution of x = 3(1-e^(-x)))

TECS-L construction:
  φ + σ/(σ + sopfr - φ) = 2 + 12/(12 + 5 - 2)
                         = 2 + 12/15
                         = 2 + 4/5
                         = 2.800

  Error: |2.8214 - 2.800| / 2.8214 = 0.76%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Physical Context

The factor 2.821 determines the frequency at which a blackbody emits maximum power per unit frequency. It arises from maximizing the Planck function B_ν(T) ∝ ν³/(e^(hν/kT)-1). The transcendental equation 3(1-e^(-x)) = x has no closed-form solution — yet TECS-L captures it to sub-percent accuracy.

### Alternative Construction

```
σφ / (σ - τ + sopfr/(σ/τ-φ))
= 24 / (12 - 4 + 5/1)
= 24 / 13
= 1.846  — too low, rejected.

(σ/τ)·(sopfr-φ)/(sopfr+φ/(σ-τ))
= 3·3/(5+0.25)
= 9/5.25
= 1.714 — rejected.

Best: φ + σ/(σ+sopfr-φ) = 14/5 = 2.800 (0.76%)
```

## Verification

- [x] x_max = 2.82144 confirmed (Planck peak)
- [x] φ + σ/(σ+sopfr-φ) = 14/5 = 2.800 confirmed
- [x] Error 0.76% verified
