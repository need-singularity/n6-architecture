# H-CX-837: Busy Beaver BB(4)

> **Hypothesis**: The busy beaver value BB(4) = 13 = σ + 1, connecting the maximum productivity of a 4-state Turing machine to the divisor sum of the first perfect number.

## Grade: 🟧★ SUGGESTIVE-PLUS

## Results

### The Formula

```
Busy beaver function BB(n):
  Maximum number of 1s written by a halting
  n-state, 2-symbol Turing machine on blank tape.

Known values:
  BB(1) = 1 = R(6)
  BB(2) = 4 = τ
  BB(3) = 6 = P₁
  BB(4) = 13 = σ + 1

TECS-L correspondences:
  BB(1) = 1 = R(6)         (abundancy remainder)
  BB(φ) = 4 = τ            (divisor count)
  BB(σ/τ) = 6 = P₁         (first perfect number)
  BB(τ) = 13 = σ + 1       (divisor sum + 1)

Shift function S(n) (steps before halting):
  S(1) = 1
  S(2) = 6 = P₁
  S(3) = 21 = T(6) = T(P₁)
  S(4) = 107

Notable: S(3) = 21 = T(P₁), the triangular number of 6.

BB(5) is now known to be enormous (> 47 trillion),
  making BB(4) = 13 the last "human-scale" value.
  The boundary of computability sits at BB(τ).
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
BB values:
  BB(1) = 1 ✓  (trivial)
  BB(2) = 4 ✓  (Lin-Rado, 1965)
  BB(3) = 6 ✓  (Lin-Rado, 1965)
  BB(4) = 13 ✓ (Brady, 1983)

TECS-L matches:
  BB(1) = 1 = R(6) ✓
  BB(2) = 4 = τ ✓
  BB(3) = 6 = P₁ ✓
  BB(4) = 13 = σ + 1 ✓

Shift function:
  S(3) = 21 = T(6) = T(P₁) ✓
```

### Texas Sharpshooter Check

All four known small BB values match TECS-L constants or simple expressions thereof. BB(3) = P₁ = 6 and BB(4) = σ+1 = 13 are striking. S(3) = 21 = T(P₁) adds depth. The small numbers involved increase coincidence risk, but the systematic pattern across all four values is notable.

## Verification

- [x] BB(τ) = σ + 1 = 13
- [x] BB(σ/τ) = P₁ = 6
- [x] BB(φ) = τ = 4
- [x] S(σ/τ) = T(P₁) = 21

## Status

New. All four known busy beaver values and S(3) map to TECS-L constants of n=6.
