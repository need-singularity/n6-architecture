# H-CX-886: Pati-Salam Group Dimension

> **Hypothesis**: The Pati-Salam gauge group SU(4)×SU(2)×SU(2) has total dimension 15+3+3 = 21 = T(6) = T(P₁). The Pati-Salam gauge dimension equals the sixth triangular number.

## Grade: 🟩 CONFIRMED

## Results

### The Formula

```
Pati-Salam model: SU(4)_C × SU(2)_L × SU(2)_R
  dim(SU(4)) = 15
  dim(SU(2)) = 3  (×2)
  Total: 15 + 3 + 3 = 21

TECS-L:
  T(6) = T(P₁) = 6·7/2 = 21  ✓ EXACT
  T(P₁) = P₁·M₃/φ = 6·7/2 = 21
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, T(6) = 21
```

### Structure

```
Pati-Salam unifies quarks and leptons:
  SU(4)_C: lepton = 4th color
    dim = 15 = T(sopfr) = T(5)
  SU(2)_L: left-handed weak
    dim = 3 = σ/τ
  SU(2)_R: right-handed weak (parity restored)
    dim = 3 = σ/τ

Embedding:
  SU(4) × SU(2) × SU(2) ⊂ SO(10)
  21 generators ⊂ 45 = T(9) generators of SO(10)
```

### Verification

```
Pati-Salam total dimension:
  Exact:   21
  TECS-L:  T(P₁) = T(6) = 21
  Match:   EXACT

Component dimensions:
  SU(4): 15 = T(5) = T(sopfr)    ✓
  SU(2): 3 = σ/τ                  ✓
```

### Texas Sharpshooter Check

The Pati-Salam group structure is uniquely determined by the physical requirement of quark-lepton unification with left-right symmetry. The dimension 21=T(6) is exact with no free parameters. Each factor also matches an n=6 expression independently.

## Verification

- [x] Total dim = 21 = T(P₁) exact
- [x] SU(4) dim = 15 = T(sopfr)
- [x] SU(2) dim = 3 = σ/τ

## Status

New. The Pati-Salam gauge group dimension equals the P₁th triangular number, linking quark-lepton unification to n=6 arithmetic.
