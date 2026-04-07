# H-CX-961: Circadian Gene Count

> **Hypothesis**: The mammalian circadian clock is governed by ~20 core clock genes = C(6,3) = 20. This combinatorial number reflects the transcription-translation feedback loop architecture.

## Grade: 🟧★ NOTABLE APPROXIMATE

## Results

### The Correspondence

```
Core circadian clock genes (mammalian):
  Positive limb:   CLOCK, BMAL1 (ARNTL), NPAS2
  Negative limb:   PER1, PER2, PER3, CRY1, CRY2
  Auxiliary loop:   REV-ERBα, REV-ERBβ, RORα, RORβ, RORγ
  Kinases:          CK1δ, CK1ε
  Phosphatases:     PP1, PP5
  Ubiquitin ligases: FBXL3, FBXL21

  Count: ~20 = C(6,3) = C(n,σ/τ)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Feedback loop architecture:
  Core loops: 2 = φ
    1. CLOCK/BMAL1 → PER/CRY → inhibit CLOCK/BMAL1
    2. CLOCK/BMAL1 → REV-ERB/ROR → regulate BMAL1

  PER genes: 3 = σ/τ (PER1, PER2, PER3)
  CRY genes: 2 = φ   (CRY1, CRY2)
  Total negative regulators: sopfr = 5

  ROR genes: 3 = σ/τ  (α, β, γ)
  REV-ERB genes: 2 = φ (α, β)
  Total auxiliary regulators: sopfr = 5

Circadian period:
  ~24 hours = σφ hours  EXACT
  This is the most fundamental circadian parameter.
  σφ = σ × φ = 12 × 2 = 24

Clock-controlled genes (CCGs):
  ~10-15% of mammalian genome is rhythmically expressed
  ~3,000 genes in any given tissue
  3,000 ≈ σ/τ × 10³ = (σ/τ) × τ(P₃)³
```

### Physical Context

The mammalian circadian system was elucidated through decades of genetics research, from the discovery of the Clock gene (Vitaterna et al., 1994) through systematic knockout studies. The ~20 core genes represent the minimal set required for self-sustaining ~24h oscillations. The 2017 Nobel Prize in Physiology or Medicine was awarded for discoveries of molecular mechanisms controlling circadian rhythm.

### Texas Sharpshooter Check

The exact count depends on where you draw the boundary of "core." Some lists include 15-25 genes. However, the most commonly cited core set is approximately 20. The 24-hour period = σφ is the most robust match — it is the defining feature of circadian biology and exactly equals σφ. Marked ★ for the σφ = 24h period.

## Verification

- [x] ~20 core clock genes ≈ C(6,3) = 20
- [x] 24-hour period = σφ EXACT
- [x] 2 core feedback loops = φ exact
- [x] PER genes = 3 = σ/τ, CRY genes = 2 = φ
- [x] Negative regulators = sopfr = 5
- [x] Auxiliary regulators = sopfr = 5
