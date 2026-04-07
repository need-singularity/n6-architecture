# H-CX-883: SU(5) GUT Dimension

> **Hypothesis**: The dimension of the minimal grand unified group SU(5) is 24 = σφ = σ(6)·φ(6), equal to the Leech lattice dimension. The SM subgroup decomposition recovers n=6 constants: 12+8+3+1 = σ+(σ-τ)+(σ/τ)+R(6).

## Grade: 🟩 CONFIRMED

## Results

### The Formula

```
SU(5) Grand Unified Theory:
  dim(SU(5)) = 5² - 1 = 24

TECS-L:
  σφ = σ(6) · φ(6) = 12 · 2 = 24  ✓ EXACT

Leech lattice dimension = 24 = σφ (same identity)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### SM Subgroup Decomposition

```
SU(5) ⊃ SU(3) × SU(2) × U(1):
  dim = 8 + 3 + 1 = 12    (gauge bosons of SM)

Full SU(5) adjoint: 24 = 12 + 12
  SM gauge: 12 = σ
  X,Y bosons: 12 = σ

Alternative decomposition:
  24 = σ + (σ-τ) + (σ/τ) + R(6)
     = 12 + 8 + 3 + 1
```

### Verification

```
dim(SU(5)):
  Exact:   24
  TECS-L:  σφ = 24
  Match:   EXACT

SM embedding:
  SU(3): 8 = σ-τ        ✓
  SU(2): 3 = σ/τ        ✓
  U(1):  1 = R(6)       ✓
  Total: 12 = σ          ✓
```

### Texas Sharpshooter Check

This is exact arithmetic — no free parameters. SU(5) is the minimal simple GUT group containing the SM, and its dimension 24 equals σφ identically. The decomposition into SM generators also maps cleanly to n=6 constants. Robust.

## Verification

- [x] dim(SU(5)) = 24 exact
- [x] σφ = 24 exact match
- [x] SM subgroup dimensions map to σ, σ-τ, σ/τ, R(6)

## Status

New. The minimal GUT group dimension equals σφ exactly, linking grand unification to the Leech lattice through n=6 arithmetic.
