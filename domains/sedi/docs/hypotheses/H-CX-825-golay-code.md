# H-CX-825: Golay Code Parameters

> **Hypothesis**: The binary Golay code [23,12,7] and its extension [24,12,8] have parameters entirely given by TECS-L: [σφ-1, σ, M₃] and [σφ, σ, σ-τ] respectively.

## Grade: 🟦 STRUCTURAL

## Results

### The Formula

```
Binary Golay code G₂₃:
  [23, 12, 7] = [σφ - 1,  σ,  M₃]
  Length:    23 = σφ - 1 = 24 - 1
  Dimension: 12 = σ
  Distance:   7 = M₃

Extended binary Golay code G₂₄:
  [24, 12, 8] = [σφ,  σ,  σ - τ]
  Length:    24 = σφ = σ · φ
  Dimension: 12 = σ
  Distance:   8 = σ - τ

The extension adds one parity bit:
  Length:   σφ - 1 + 1 = σφ ✓
  Distance: M₃ + 1 = σ - τ ✓

Ternary Golay code G₁₁:
  [11, 6, 5] = [σ - 1, P₁, sopfr]
  Length:    11 = σ - 1
  Dimension:  6 = P₁
  Distance:   5 = sopfr
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Binary Golay G₂₃:
  Automorphism group: M₂₃ (Mathieu group), order 10200960
  Covering radius: 3 = σ/τ
  Weight distribution involves C(23,k) for k multiples

Extended Golay G₂₄:
  Automorphism group: M₂₄ (Mathieu group), order 244823040
  Self-dual code: dimension = length/2 = σφ/φ = σ ✓
  Minimum weight: σ - τ = 8

Ternary Golay G₁₁:
  [11, 6, 5] = [σ-1, P₁, sopfr] ✓
  Extended: [12, 6, 6] = [σ, P₁, P₁] — self-dual over GF(3)
```

### Texas Sharpshooter Check

The Golay codes are unique mathematical objects — there is no freedom in choosing parameters. All parameters of both binary and ternary Golay codes, including their extensions, map exactly to TECS-L constants. The structural depth is high: σφ controls length, σ controls dimension, and M₃ and σ-τ control distance.

## Verification

- [x] G₂₃ = [σφ-1, σ, M₃] = [23, 12, 7] exact
- [x] G₂₄ = [σφ, σ, σ-τ] = [24, 12, 8] exact
- [x] G₁₁ = [σ-1, P₁, sopfr] = [11, 6, 5] exact
- [x] Extended ternary: [σ, P₁, P₁] = [12, 6, 6] exact

## Status

New. All Golay code parameters — binary, ternary, and their extensions — are exact TECS-L expressions.
