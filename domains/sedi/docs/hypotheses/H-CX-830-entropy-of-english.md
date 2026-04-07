# H-CX-830: Entropy of English Text

> **Hypothesis**: Shannon's estimated entropy of English, H ≈ 1.0–1.5 bits/character, spans the interval [R(6), σ/(σ-τ+sopfr/φ)] connecting information density to n=6 arithmetic.

## Grade: 🟧 SUGGESTIVE

## Results

### The Formula

```
Shannon's entropy estimates for English:
  Upper bound: ~1.5 bits/character (letter frequencies)
  Lower bound: ~1.0 bits/character (long-range structure)
  Best estimate: ~1.0–1.3 bits/character

TECS-L approximations:
  Lower bound: R(6) = 1.0 bits/char

  Attempt 1: σ/(σ - τ + sopfr/φ)
    = 12 / (12 - 4 + 2.5)
    = 12 / 10.5
    = 1.143  (within range, ~12% from 1.3)

  Attempt 2: sopfr · φ / (σ - sopfr + φ)
    = 10 / 9
    = 1.111  (within range)

  Attempt 3: (σ/τ)/(σ/τ - φ/(σ-τ))
    = 3 / (3 - 0.25)
    = 3 / 2.75
    = 1.091

  Attempt 4: M₃/P₁
    = 7/6
    = 1.167  (close to ~1.2 estimates)

English alphabet: 26 letters
  Maximum entropy: log₂(26) = 4.70 bits/char
  Redundancy: 1 - H/log₂(26) ≈ 1 - 1.2/4.7 ≈ 0.745
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Shannon (1951) estimate: ~1.0 bits/char (gambling method)
Cover & King (1978): ~1.3 bits/char
Brown et al. (1992): ~1.75 bits/char (word-level)

TECS-L values:
  R(6) = 1.0 ✓ (matches Shannon's lower bound)
  M₃/P₁ = 1.167 (within range)
  σ/10.5 = 1.143 (within range)

No single clean expression matches the ~1.3 consensus.
```

### Texas Sharpshooter Check

The entropy of English is an empirical quantity with a wide uncertainty range (1.0–1.5). Many ratios of small integers fall in this interval. R(6)=1 matching the lower bound is clean but may be coincidental given the imprecision of the target.

## Verification

- [x] H(English) ≈ 1.0–1.5 bits/char (Shannon, Cover)
- [x] R(6) = 1.0 matches lower bound exactly
- [x] M₃/P₁ = 1.167 falls within range
- [x] No single clean TECS-L expression for consensus ~1.3

## Status

New. English entropy lower bound matches R(6)=1 exactly; upper estimates lack a clean TECS-L form.
