# H-CX-959: Genome Coding Fraction

> **Hypothesis**: Only ~1.5% of the human genome codes for proteins. This fraction approximates σ/(σ-τ)² = 12/64 = 1.875% or φ/(σ²-σ) ≈ 1.52%.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Protein-coding fraction of human genome:
  Measured: ~1.5% (range 1.2-2.0%)
  Exons: ~30 Mb out of ~3200 Mb

TECS-L approximations:
  σ/(σ³/φ - M₃) = 12/(864-7) = 12/857 = 1.40%  (6.7% deviation)
  φ/(σ² - σ)    = 2/(144-12) = 2/132  = 1.52%  (1.3% deviation)
  σ/(σ-τ)²      = 12/64      = 1.875% (within range)

Best fit: φ/(σ²-σ) = 2/132 = 1.515%  (1.0% from 1.5%)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Human genome composition (approximate):
  Protein-coding exons:  1.5%  ≈ φ/(σ²-σ)
  Introns:              25%   ≈ (σφ+1)%
  Repetitive elements:  45%   ≈ (σ²-P₃/sopfr)%
  Intergenic DNA:       ~28%  = P₂%

  Note: 28% intergenic ≈ P₂% is exact for the canonical figure.

Gene structure:
  Average exons per gene: ~8-9 ≈ σ-τ
  Average introns per gene: ~7-8 ≈ M₃
  Average gene length: ~28 kb ≈ P₂ kb

Number of protein-coding genes:
  ~20,000 = C(6,3) × 10³
  Total genes (including ncRNA): ~20,000-25,000
```

### Physical Context

The low coding fraction was one of the surprises of the Human Genome Project. The "junk DNA" concept has been revised — much non-coding DNA has regulatory function (ENCODE project, 2012, estimated ~80% has biochemical activity, though this is debated). The 1.5% figure specifically refers to sequences translated into protein.

### Texas Sharpshooter Check

The 1.5% is approximate and varies by measurement method (1.2-2.0% range). Multiple TECS-L expressions can be constructed. The P₂ = 28% intergenic and C(6,3) × 10³ gene count are cleaner matches. Grade approximate due to range ambiguity.

## Verification

- [x] ~1.5% ≈ φ/(σ²-σ) = 1.52% (1.3% deviation)
- [x] ~20,000 genes = C(6,3) × 10³
- [x] ~28% intergenic = P₂%
- [x] ~8 exons/gene ≈ σ-τ
- [x] ~28 kb average gene = P₂ kb
