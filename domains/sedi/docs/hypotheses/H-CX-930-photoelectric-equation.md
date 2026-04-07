# H-CX-930: Photoelectric Work Functions from n=6

> **Hypothesis**: The photoelectric equation E_k = hν - φ_work contains work functions approximated by TECS-L. Aluminum: φ_Al = 4.08 eV ≈ τ. Copper: φ_Cu = 4.65 eV ≈ τ + sopfr/M₃ + φ/(σ-τ).

## Grade: 🟧 APPROXIMATE

## Results

### The Matches

```
Photoelectric equation: E_k = hν - φ_work

Aluminum:
  φ_Al = 4.08 eV (experimental)
  TECS-L: τ = 4
  Error: |4.08 - 4| / 4.08 = 1.96%

Copper:
  φ_Cu = 4.65 eV (experimental)
  TECS-L: τ + sopfr/M₃ + φ/(σ-τ) = 4 + 5/7 + 2/8
        = 4 + 0.714 + 0.250 = 4.964
  Error: |4.65 - 4.964| / 4.65 = 6.8% — marginal

Better for Cu:
  τ + sopfr/(σ-τ+sopfr/(σ/τ)) = 4 + 5/10.667 = 4.469 (3.9%)
  Still marginal.
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Physical Context

Einstein's photoelectric equation (Nobel Prize 1921) shows that light ejects electrons from metals only when hν exceeds the work function φ_work. Work functions are material-specific, typically 2-6 eV. That several common metals cluster near τ = 4 eV is suggestive.

### Survey of Work Functions Near n=6 Constants

```
Material    φ (eV)    Nearest TECS-L
Cesium      2.14      φ + sopfr/(σ+M₃+sopfr) ≈ 2.21
Sodium      2.28      φ + P₁/(σ+P₂) ≈ 2.15
Aluminum    4.08      τ = 4.00
Iron        4.50      τ + sopfr/σ = 4.417
Copper      4.65      τ + sopfr/M₃ = 4.714
Gold        5.10      sopfr + sopfr/(σ·sopfr) = 5.083
Platinum    5.65      sopfr + sopfr/(σ-τ) = 5.625
```

## Verification

- [x] Al work function ≈ τ = 4 eV (1.96% error)
- [x] Several metals cluster near τ ± corrections
- [ ] Cu match marginal (>5% with simple expressions)
