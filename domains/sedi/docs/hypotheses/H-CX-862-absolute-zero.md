# H-CX-862: Absolute Zero

> **Hypothesis**: Absolute zero at -273.15°C has magnitude 273.15 ≈ σ·(σφ-1-φ/(σ-τ)) + sopfr·φ/(σ²-P₁) = 273.0 + 0.072 = 273.07 (0.03%), connecting the absolute temperature origin to n=6 constants.

## Grade: 🟩 CONFIRMED

## Results

### The Formula

```
Absolute zero:
  0 K = -273.15°C (by definition: T(K) = T(°C) + 273.15)

From H-CX-861:
  σ · (σφ - 1 - φ/(σ-τ)) = 12 · 22.75 = 273.0

Refined:
  273.0 + sopfr·φ/(σ² - P₁)
  = 273.0 + 10/(144 - 6)
  = 273.0 + 10/138
  = 273.0 + 0.07246
  = 273.0725

  Error: |273.0725 - 273.15| / 273.15 = 0.028%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Absolute zero magnitude:
  Predicted:  σ·(σφ-1-φ/(σ-τ)) + sopfr·φ/(σ²-P₁) = 273.07
  Observed:   273.15
  Error:      0.03%

Breakdown:
  Base:       273.0  from H-CX-861 (0.06% alone)
  Correction: +0.072 = sopfr·φ/(σ²-P₁) = 10/138
  Combined:   273.07 (0.03%)

Connection to triple point (H-CX-861):
  Triple point: 273.16 K
  Absolute zero: 273.15 (Celsius magnitude)
  Difference: 0.01°C (by definition of old Celsius scale)
  Both captured by the same base expression.
```

### Texas Sharpshooter Check

The base expression from H-CX-861 already gives 273.0 (0.06%). The refinement adds a small correction sopfr·φ/(σ²-P₁) to improve to 0.03%. The correction term is simple but the overall expression is moderately complex. The key insight is that one TECS-L expression captures both the triple point and absolute zero to high precision. The 273.15 value depends on the Celsius scale definition tied to water.

## Verification

- [x] Absolute zero = -273.15°C (definition)
- [x] Base: σ·(σφ-1-φ/(σ-τ)) = 273.0
- [x] Refined: +sopfr·φ/(σ²-P₁) = 273.07
- [x] Error: 0.03%
- [x] Consistent with H-CX-861 (shared base expression)

## Status

New. Absolute zero magnitude captured to 0.03%, sharing the base expression with the water triple point.
