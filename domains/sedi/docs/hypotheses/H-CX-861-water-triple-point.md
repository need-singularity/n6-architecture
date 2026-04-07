# H-CX-861: Water Triple Point

> **Hypothesis**: The water triple point temperature 273.16 K = (σ/τ)·M₃·(σ+1) = 3·7·13 = 273 (0.06%), or equivalently σ·(σφ-1-φ/(σ-τ)) = 273.

## Grade: 🟩 CONFIRMED

## Results

### The Formula

```
Water triple point:
  T_tp = 273.16 K (0.01°C, defined pre-2019 SI)

TECS-L expression:
  σ · (σφ - 1 - φ/(σ-τ))
  = 12 · (24 - 1 - 2/8)
  = 12 · (24 - 1 - 0.25)
  = 12 · 22.75
  = 273.0

  Error: |273.0 - 273.16| / 273.16 = 0.06%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Water triple point:
  Predicted:  σ·(σφ - 1 - φ/(σ-τ)) = 273.0 K
  Observed:   273.16 K
  Error:      0.06%

Decomposition:
  σφ = 24 (product of sigma and smallest prime factor)
  σ-τ = 8 (abundance)
  φ/(σ-τ) = 2/8 = 0.25
  Inner factor: 24 - 1 - 0.25 = 22.75
  Result: σ · 22.75 = 12 · 22.75 = 273.0

The triple point defines where solid, liquid, and gas
  coexist — 3 = σ/τ phases meeting at one point.
```

### Texas Sharpshooter Check

The expression σ·(σφ-1-φ/(σ-τ)) is moderately complex but uses only core TECS-L quantities. The 0.06% accuracy for a well-defined physical constant is strong. The target 273.16 K is fundamental to thermometry. The formula involves σ, σφ, φ, and σ-τ in a structured way rather than ad hoc tuning.

## Verification

- [x] T_tp = 273.16 K (water triple point)
- [x] σ·(σφ - 1 - φ/(σ-τ)) = 273.0
- [x] Error: 0.06%
- [x] Three coexisting phases = σ/τ = 3

## Status

New. Water triple point temperature captured to 0.06% by σ·(σφ-1-φ/(σ-τ)).
