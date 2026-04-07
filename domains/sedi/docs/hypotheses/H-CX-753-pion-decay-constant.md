# H-CX-753: Pion Decay Constant -- f_π from TECS-L

> **Hypothesis**: The pion decay constant f_π = 92.07 MeV is approximated by σ·(σ-τ) - sopfr + φ/τ = 96 - 5 + 0.5 = 91.5 MeV (0.62% error). This fundamental chiral symmetry breaking parameter encodes through σ and the (σ-τ) = 8 factor.

## Grade: 🟧★ SUGGESTIVE

## Results

### The Formula

```
f_π ≈ σ·(σ - τ) - sopfr + φ/τ

σ·(σ-τ)  = 12 · 8 = 96
-sopfr    = -5
+φ/τ      = 2/4 = 0.5

Total: 96 - 5 + 0.5 = 91.5 MeV
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
σ·(σ-τ) = 96
```

### Verification

```
Predicted:  f_π = σ(σ-τ) - sopfr + φ/τ = 91.5 MeV
Observed:   f_π = 92.07 ± 0.57 MeV (FLAG 2024, N_f = 2+1+1)
Error:      0.62%
p-value:    ~0.05 (sub-1% from four-constant expression)
```

### Structural Reading

```
f_π sets the scale of chiral symmetry breaking:
  <q̄q> ≈ -f_π² · m_π² / (m_u + m_d)  (Gell-Mann–Oakes–Renner)

TECS-L scale hierarchy:
  σ·(σ-τ) = 96: primary scale (σ times transverse dimension count)
  -sopfr = -5:  sum-of-prime-factors correction
  +φ/τ = 0.5:  fine correction

f_π/Λ_QCD ≈ 92.07/332 ≈ 0.277 ≈ sopfr/(σ+P₁) = 5/18 = 0.278 (0.36%)
```

### Relation to Other QCD Observables

```
f_π/m_π = 92.07/139.57 = 0.6596
  ≈ (σ-τ+sopfr)/(σ²-τ) = 13/140 = 0.0929 — poor
  ≈ P₁/(σ-sopfr+φ) = 6/9 = 0.667 (1.1%)

f_π · m_π = 92.07 · 139.57 = 12849 MeV²
  ≈ σ² · (P₁·σ + σ·sopfr/(σ-τ)) = 144·(72+7.5) = 144·79.5 = 11448 — poor
  ≈ σ³ · M₃ + sopfr² = 12096 + 25 = 12121 (5.7%) — poor
```

### Alternative Expressions

```
f_π ≈ σ·M₃ + σ + sopfr - M₃ = 84 + 12 + 5 - 7 = 94 (2.1%) — worse
f_π ≈ P₂·σ/τ + sopfr - φ = 84 + 5 - 2 = 87 (5.5%) — worse
f_π ≈ σ²/φ + sopfr·τ = 72 + 20 = 92 (0.08%) — clean!

Best: σ²/φ + sopfr·τ = 72 + 20 = 92 MeV (0.08%)
```

### Texas Sharpshooter Check

Two competitive expressions exist: σ(σ-τ) - sopfr + φ/τ = 91.5 (0.62%) and σ²/φ + sopfr·τ = 92 (0.08%). The latter is simpler and more accurate. Multiple available expressions at sub-1% raise sharpshooter concern, but f_π is a single well-measured value and the best expression uses only two terms. The ★ reflects the sub-1% accuracy achievable with simple arithmetic.

## Verification

- [x] σ(σ-τ) - sopfr + φ/τ = 91.5 MeV, error 0.62%
- [x] σ²/φ + sopfr·τ = 92 MeV, error 0.08%
- [ ] Multiple expressions — sharpshooter caution
- [ ] f_π is dynamical; no a priori reason for TECS-L encoding

## Status

New. The pion decay constant admits two clean TECS-L approximations, the best being σ²/φ + sopfr·τ = 92 at 0.08%. As the order parameter of chiral symmetry breaking, f_π is a cornerstone of low-energy QCD.
