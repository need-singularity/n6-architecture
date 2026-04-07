# H-CX-563: Trefoil Knot Jones Polynomial and n=6

> **Hypothesis**: The trefoil knot — the simplest non-trivial knot — has crossing number σ/τ=3 and its Jones polynomial evaluated at t=-1 gives |V(-1)|=3=σ/τ.

## Grade: 🟩 CONFIRMED (exact)

## Results

### Trefoil Knot Parameters

| Parameter | Value | n=6 |
|---|---|---|
| Crossing number | 3 | σ/τ |
| Determinant | 3 | σ/τ |
| Bridge number | 2 | φ(6) |
| Braid index | 2 | φ(6) |
| Genus | 1 | R(6) |
| Signature | -2 | -φ(6) |

### Jones Polynomial

```
V(trefoil) = -t⁻⁴ + t⁻³ + t⁻¹

At t = -1: V(-1) = -1 - 1 - 1 = -3 → |V(-1)| = 3 = σ/τ
```

### Figure-Eight Knot (4₁)

| Parameter | Value | n=6 |
|---|---|---|
| Crossing number | 4 | τ(6) |
| Determinant | 5 | sopfr(6) |
| Volume | 2.0299... | ≈ φ + 1/σ² (very approximate) |

### Knot Table and n=6

The first 5 knots by crossing number: 0₁, 3₁, 4₁, 5₁, 5₂
Crossing numbers: {0, 3, 4, 5, 5} — containing {σ/τ, τ, sopfr}.

## Status

- [x] Trefoil: crossing=σ/τ, bridge=φ, genus=R(6)
- [x] Figure-eight: crossing=τ, determinant=sopfr
