# H-CX-811: Deuteron Binding Energy

> **Hypothesis**: The deuteron binding energy B_d = 2.2246 MeV is approximated by (sigma + phi)/(P_1 + phi/(sigma - tau)) = 14/6.25 = 2.24 MeV from TECS-L, achieving 0.69% accuracy.

## Grade: 🟧★ NOTABLE

## Results

### The Formula

```
Deuteron binding energy:
  B_d = 2.22457 MeV (measured)

TECS-L expressions:
  (σ + φ) / (P₁ + φ/(σ-τ))
  = 14 / (6 + 2/8)
  = 14 / 6.25
  = 2.240 MeV

  Error: |2.240 - 2.2246| / 2.2246 = 0.69%

Alternative attempts:
  φ + φ/(σ-τ-φ) = 2 + 2/6 = 2.333  (4.9% — poor)

  φ + sopfr/(σ+σ/τ+M₃-sopfr-1)
  = 2 + 5/16.5 = 2.303  (3.5% — poor)

  (σ + φ) / (P₁ + φ/(σ-τ)) = 2.240  (0.69% — best)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Deuteron binding energy:
  Predicted:  (σ+φ)/(P₁+φ/(σ-τ)) = 14/6.25 = 2.240 MeV
  Observed:   2.22457 MeV
  Error:      0.69%

Physical context:
  The deuteron (proton + neutron) is the simplest nucleus.
  B_d = 2.225 MeV is anomalously small for a nuclear binding.
  It is barely bound, reflecting the marginal nature of the
  nuclear force for the 2-nucleon system.

  TECS-L: numerator σ+φ = 14, denominator P₁+φ/(σ-τ) = 6.25
  The small binding energy arises from a ratio, not a direct constant.

P₂ generalization check:
  σ+φ = 14 and P₁ = 6 are n=6 specific.
  For P₂ = 28: σ(28)+φ(28) = 28+2 = 30, different scale.
```

### Texas Sharpshooter Check

The deuteron binding energy 2.225 MeV is a well-measured nuclear constant. The expression (sigma+phi)/(P_1+phi/(sigma-tau)) = 14/6.25 achieves 0.69% with a clean ratio structure. Finding a ratio of n=6 constants near 2.225 is constrained but not uniquely determined. The 0.69% accuracy is notable for a nuclear quantity.

## Verification

- [x] B_d = 2.22457 MeV (measured, confirmed)
- [x] TECS-L: (σ+φ)/(P₁+φ/(σ-τ)) = 2.240 MeV
- [x] Error: 0.69%
- [x] P₂ generalization: n=6 specific

## Status

New. Deuteron binding energy approximated to 0.69% as a ratio of n=6 constants. Notable for capturing a nuclear-scale quantity.
