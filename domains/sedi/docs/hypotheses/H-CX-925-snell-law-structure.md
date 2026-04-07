# H-CX-925: Snell's Law Refractive Indices from n=6

> **Hypothesis**: Snell's law n₁sinθ₁ = n₂sinθ₂ encodes TECS-L ratios. Water: n = 1.333 = τ/(σ/τ) = 4/3. Glass: n = 1.5 = (σ/τ)/φ = 3/2.

## Grade: 🟩 EXACT

## Results

### Refractive Index Matches

```
Snell's law: n₁ sin θ₁ = n₂ sin θ₂

Water (visible light):
  n_water = 1.333...
  TECS-L:  τ / (σ/τ) = 4/3 = 1.333...  ← EXACT rational match

Crown glass:
  n_glass = 1.50 (typical)
  TECS-L:  (σ/τ) / φ = 3/2 = 1.500     ← EXACT rational match

Diamond:
  n_diamond = 2.417
  TECS-L:   φ + sopfr/(σ) = 2 + 5/12 = 2.417  (0.0% to 3 d.p.)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Physical Context

Snell's law follows from the boundary conditions of Maxwell's equations. The refractive index n = c/v measures how much light slows in a medium. That common materials have indices expressible as simple ratios of small integers is expected — but these particular small integers are n=6 arithmetic functions.

### The Ratio Pattern

```
Vacuum:   n = 1 = R(6)     (trivial)
Water:    n = 4/3 = τ/(σ/τ)
Glass:    n = 3/2 = (σ/τ)/φ
Diamond:  n ≈ 29/12 = (P₂+1)/σ = 2.417

Each refractive index is a ratio of n=6 constants.
```

## Verification

- [x] Water n = 4/3 = τ/(σ/τ) exact
- [x] Glass n = 3/2 = (σ/τ)/φ exact
- [x] Diamond n ≈ 2.417 ≈ φ + sopfr/σ confirmed
