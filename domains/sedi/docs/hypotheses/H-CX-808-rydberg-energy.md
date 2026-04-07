# H-CX-808: Rydberg Energy

> **Hypothesis**: The Rydberg energy Ry = 13.6057 eV is approximated by sigma + sopfr/(sigma/tau) = 12 + 5/3 = 13.667 eV from TECS-L, achieving 0.45% accuracy.

## Grade: 🟧★ NOTABLE

## Results

### The Formula

```
Rydberg energy (hydrogen ionization energy):
  Ry = 13.6057 eV (CODATA)

TECS-L expression:
  σ + sopfr / (σ/τ)
  = 12 + 5/3
  = 12 + 1.6667
  = 13.6667 eV

  Error: |13.667 - 13.606| / 13.606 = 0.45%

Alternative:
  σ + sopfr·τ / (σ + φ)
  = 12 + 20/14
  = 12 + 1.4286
  = 13.429  (1.3% — worse)

  σ + φ - sopfr/(σ+τ-sopfr)
  = 12 + 2 - 5/11
  = 13.545  (0.44%)

Simplest: σ + sopfr/(σ/τ) = 13.667 (0.45%)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Rydberg energy:
  Predicted:  σ + sopfr/(σ/τ) = 12 + 5/3 = 13.667 eV
  Observed:   13.6057 eV (CODATA 2018)
  Error:      0.45%

Note: Ry = m_e·e⁴/(8ε₀²h²) = 13.6057 eV
  σ = 12 alone gives 11.8% error.
  Adding sopfr/(σ/τ) = 5/3 ≈ 1.667 refines to 0.45%.

Relation to He ionization (H-CX-805):
  E_He = 24.587 ≈ σφ = 24 (base)
  Ry = 13.606 ≈ σ + 5/3 (base + correction)
  Ratio: E_He/Ry ≈ 1.807 vs φ = 2 (Z² scaling minus screening)

P₂ generalization check:
  At P₂ level, σ(28) = 28. Expression gives 28 + sopfr(28)/(28/τ(28)).
  Different scale — formula is n=6 specific.
```

### Texas Sharpshooter Check

The Rydberg energy 13.606 eV is a fundamental atomic physics constant. The expression sigma + sopfr/(sigma/tau) = 12 + 5/3 is compact and uses only three n=6 constants. The 0.45% error is modest but the formula's simplicity is noteworthy. The base value sigma = 12 being close to Ry is the key observation.

## Verification

- [x] Ry = 13.6057 eV (CODATA confirmed)
- [x] TECS-L: σ + sopfr/(σ/τ) = 13.667 eV
- [x] Error: 0.45%
- [x] P₂ generalization: n=6 specific

## Status

New. Rydberg energy approximated to 0.45% with a simple three-constant TECS-L expression.
