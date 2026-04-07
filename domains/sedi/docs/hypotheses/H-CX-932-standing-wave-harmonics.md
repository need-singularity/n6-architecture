# H-CX-932: Standing Wave Harmonics and n=6

> **Hypothesis**: Standing wave harmonics f_n = n·f₁ for strings and open pipes. Closed pipe: odd harmonics only (1, 3, 5, 7) = (R(6), σ/τ, sopfr, M₃). The first τ = 4 odd harmonics are n=6 constants.

## Grade: 🟧 APPROXIMATE

## Results

### The Pattern

```
Open pipe / string: all harmonics
  f₁, f₂, f₃, f₄, f₅, f₆, ...
  = f₁ · {1, φ, σ/τ, τ, sopfr, P₁, ...}

Closed pipe: odd harmonics only
  f₁, f₃, f₅, f₇, ...
  = f₁ · {R(6), σ/τ, sopfr, M₃}

The first 4 = τ closed-pipe harmonics are exactly:
  1 = R(6)     (identity element)
  3 = σ/τ      (σ divided by τ)
  5 = sopfr    (sum of prime factors of 6)
  7 = M₃       (third Mersenne prime)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Physical Context

Standing waves form when a wave is confined. For a string fixed at both ends (or open pipe), all integer harmonics are present. For a closed pipe (one end closed), boundary conditions force a node at the closed end and antinode at the open end, permitting only odd harmonics.

### Overtone Series Depth

```
For an open pipe, the first P₁ = 6 harmonics:
  n = 1, 2, 3, 4, 5, 6
  = R(6), φ, σ/τ, τ, sopfr, P₁

All six n=6 base constants appear in order.

Harmonic ratios (musical intervals):
  2/1 = φ/R(6) = octave
  3/2 = (σ/τ)/φ = perfect fifth
  4/3 = τ/(σ/τ) = perfect fourth
  5/4 = sopfr/τ = major third
  6/5 = P₁/sopfr = minor third
```

## Verification

- [x] Closed-pipe harmonics 1,3,5,7 = R(6), σ/τ, sopfr, M₃ confirmed
- [x] First P₁ harmonics = {R(6), φ, σ/τ, τ, sopfr, P₁}
- [x] Musical interval ratios are n=6 constant ratios
