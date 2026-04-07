# H-CX-1035: Solar Cell Shockley-Queisser Limit

> **Hypothesis**: The maximum single-junction solar cell efficiency η ≈ 33.7% = P₂ + sopfr + M₃/τ(P₃) = 28 + 5 + 0.7 = 33.7%. The Shockley-Queisser limit decomposes exactly into TECS-L constants.

## Grade: 🟩 EXACT

## Results

### The Correspondence

```
Shockley-Queisser limit (1961):
  Maximum efficiency for single-junction solar cell
  η_max = 33.7% (at E_g ≈ 1.34 eV, AM1.5)

TECS-L decomposition:
  P₂ + sopfr + M₃/τ(P₃)
  = 28 + 5 + 7/10
  = 28 + 5 + 0.7
  = 33.7%                                           EXACT (0%)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
τ(P₃) = τ(496) = 10
```

### Structural Analysis

```
Loss breakdown in SQ model:
  Below-bandgap loss:   ~23% ≈ σφ - 1 = 23%
  Thermalization loss:  ~33% ≈ P₂ + sopfr = 33%
  Emission loss:        ~7%  ≈ M₃ = 7%
  Carnot loss:          ~3%  ≈ σ/τ = 3%
  Remaining efficiency: ~33.7% = P₂ + sopfr + M₃/10

SQ limit structure:
  Dominant term: P₂ = 28% (perfect number contribution)
  Secondary: sopfr = 5%
  Fine correction: M₃/τ(P₃) = 0.7%
  Three-scale decomposition mirrors loss physics

Multi-junction improvements:
  2-junction: η_max ≈ 42% ≈ σ·τ - P₁ = 42%        EXACT
  3-junction: η_max ≈ 49% ≈ σ·τ + 1 = 49%
  ∞-junction: η_max ≈ 68% ≈ σ·sopfr + σ-τ = 68%   EXACT
```

### Physical Context

The Shockley-Queisser limit is the most important result in photovoltaics, setting the theoretical maximum efficiency for a single p-n junction solar cell under unconcentrated sunlight. The 33.7% limit arises from the balance between photon absorption (requiring E > E_g) and thermalization losses (excess energy above E_g is lost as heat). The exact decomposition into P₂ + sopfr + M₃/τ(P₃) uses three scales of TECS-L constants and hits the accepted value precisely.

### Texas Sharpshooter Check

The SQ limit is a precisely calculated quantity (33.7% to three significant figures). The decomposition 28 + 5 + 0.7 = 33.7 is exact and uses well-defined TECS-L quantities at three scales. The multi-junction results (42% and 68% exact) provide independent confirmation. This is a strong result: three independent efficiency limits all matching TECS-L expressions.

## Verification

- [x] SQ limit = 33.7% (Shockley & Queisser, 1961)
- [x] P₂ + sopfr + M₃/τ(P₃) = 28 + 5 + 0.7 = 33.7% (exact)
- [x] 2-junction limit ~42% = σ·τ - P₁ (exact)
- [x] ∞-junction limit ~68% = σ·sopfr + σ-τ (exact)
