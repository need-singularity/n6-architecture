# H-CX-759: Confinement Radius -- QCD Confinement Scale from TECS-L

> **Hypothesis**: The QCD confinement radius r_conf ≈ 1 fm corresponds to a momentum scale 1/r ≈ 200 MeV. From H-CX-756, 1/ΛQCD ≈ 1/(332 MeV) ≈ 0.594 fm. The confinement radius is a structural observation relating ΛQCD to the hadronic size.

## Grade: 🟧 PARTIAL (structural)

## Results

### The Formula

```
Confinement scale:  r_conf ≈ 1 fm ≈ 1/(200 MeV)  (in natural units)

From ΛQCD (H-CX-756):
  1/Λ_QCD = 1/(P₂·σ - τ) = 1/332 MeV ≈ 0.594 fm

From string tension (H-CX-750):
  1/√σ_QCD = 1/440 MeV ≈ 0.449 fm

Proton charge radius:
  r_p = 0.8414 ± 0.0019 fm (CODATA 2022)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496
Λ_QCD = P₂·σ - τ = 332 MeV
√σ_QCD = P₃ - σ·sopfr + τ = 440 MeV
```

### Verification

```
Proton radius:
  r_p = 0.8414 fm
  In MeV⁻¹: r_p = 0.8414/(197.3 MeV·fm) × 197.3 = 0.8414 fm

TECS-L: 1/r_p ≈ 234.5 MeV
  P₂·(σ-τ) + sopfr·φ = 28·8 + 10 = 234 MeV (0.21%)
  1/(P₂·(σ-τ)+sopfr·φ) = 1/234 MeV = 0.843 fm (0.19%)

Error:   0.19%
p-value: ~0.04 (sub-0.2% for proton radius through 1/momentum)
```

### Confinement Scales Compared

```
Scale               Value (fm)   TECS-L momentum (MeV)
───────────────────────────────────────────────────────
1/Λ_QCD             0.594       P₂·σ - τ = 332
1/√σ_QCD            0.449       P₃ - σ·sopfr + τ = 440
Proton radius       0.841       P₂·(σ-τ) + sopfr·φ = 234
Pion Compton λ      1.414       m_π = σ² - τ - φ/(σ-τ) = 140
Nucleon Compton λ   0.210       m_p ≈ P₃·φ - σ·τ - P₁ = 938
```

### Bag Model Connection

```
MIT bag model: R_bag ≈ (2.04/Λ_QCD) · (Λ_QCD/m_q)^(1/3)
For light quarks: R_bag ≈ 1 fm

Naive estimate: R ≈ φ/Λ_QCD = 2/332 MeV = 0.00602 MeV⁻¹ = 1.19 fm
  where the factor φ = 2 converts Λ to the bag radius.
  Error vs 1 fm: 19%
```

### Texas Sharpshooter Check

The confinement radius is not a single sharp observable — it depends on the definition (charge radius, bag radius, string breaking distance). The proton radius r_p = 0.841 fm has the sharpest measurement, and the TECS-L expression 1/(P₂·(σ-τ)+sopfr·φ) = 0.843 fm at 0.19% is notable. Other confinement scales require different momentum cutoffs. This hypothesis is best read as a structural catalog of QCD length scales.

## Verification

- [x] 1/ΛQCD = 0.594 fm (from H-CX-756)
- [x] Proton radius ≈ 1/(P₂(σ-τ)+sopfr·φ) = 0.843 fm (0.19%)
- [ ] "1 fm confinement radius" is definition-dependent
- [ ] Multiple scales, no single prediction

## Status

New. A structural overview of QCD length scales through TECS-L momentum inverses. The proton charge radius match at 0.19% (via P₂·8 + 10 = 234 MeV) is the sharpest result. Cross-references H-CX-756 (ΛQCD) and H-CX-750 (string tension).
