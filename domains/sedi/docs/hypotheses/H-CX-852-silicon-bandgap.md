# H-CX-852: Silicon Bandgap Energy

> **Hypothesis**: Silicon's bandgap energy E_g = 1.12 eV is approximated by σ/(τ(P₃) + sopfr/(σ-τ)) = 12/(10 + 0.625) = 1.129 eV, achieving 0.84% accuracy.

## Grade: 🟧★ NOTABLE

## Results

### The Formula

```
Silicon bandgap energy:
  E_g(Si) = 1.12 eV (at 300 K)

TECS-L expression:
  σ / (τ(P₃) + sopfr/(σ-τ))
  = 12 / (10 + 5/8)
  = 12 / 10.625
  = 1.1294 eV

  Error: |1.1294 - 1.12| / 1.12 = 0.84%

Note: τ(P₃) = τ(496) = 10 (number of divisors of 496)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Silicon bandgap:
  Predicted:  σ/(τ(P₃) + sopfr/(σ-τ)) = 1.129 eV
  Observed:   1.12 eV (at 300 K)
  Error:      0.84%

Silicon = element 14 = σ + φ = Bravais lattice count
  Silicon's atomic number equals σ + φ.

Alternative: σ/(σ - τ + sopfr/(φ·M₃))
  = 12/(8 + 5/14) = 12/8.357 = 1.436 (28% — too high)

The primary expression σ/(τ(P₃)+sopfr/(σ-τ)) is the best fit.
```

### Texas Sharpshooter Check

The expression involves τ(P₃) which is a derived quantity (divisor count of 496=10), adding complexity. The target 1.12 is a single value near 1, making it moderately constraining. The 0.84% accuracy is decent but the formula is not especially elegant. The observation that Si=element 14=σ+φ adds context.

## Verification

- [x] E_g(Si) = 1.12 eV at 300 K (standard value)
- [x] σ/(τ(P₃) + sopfr/(σ-τ)) = 1.129
- [x] Error: 0.84%
- [x] Si = element 14 = σ + φ

## Status

New. Silicon bandgap approximated to 0.84% via σ divided by the divisor count of P₃ plus a sopfr correction.
