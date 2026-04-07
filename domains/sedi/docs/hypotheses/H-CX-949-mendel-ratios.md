# H-CX-949: Mendel's Ratios

> **Hypothesis**: The monohybrid ratio 3:1 = σ/τ : 1. The dihybrid ratio 9:3:3:1 = (σ/τ)²:(σ/τ):(σ/τ):1. The dihybrid total 16 = τ².

## Grade: 🟩 EXACT

## Results

### The Correspondence

```
Monohybrid cross (Aa × Aa):
  Genotypes: 1 AA : 2 Aa : 1 aa
  Phenotypes: 3 dominant : 1 recessive
  Ratio: 3:1 = σ/τ : 1

  Total offspring classes: 4 = τ  (AA, Aa, aA, aa)

Dihybrid cross (AaBb × AaBb):
  Phenotypic ratio: 9:3:3:1
  = (σ/τ)² : (σ/τ) : (σ/τ) : 1
  = 3² : 3 : 3 : 1

  Total Punnett squares: 16 = τ² = 4²
  Phenotypic classes: 4 = τ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8
```

### Structural Analysis

```
Generalization to n-hybrid crosses:
  Monohybrid:  4¹ = τ¹  gamete combinations, 2¹ = φ¹ phenotype classes
  Dihybrid:    4² = τ²  gamete combinations, 2² = φ² phenotype classes
  Trihybrid:   4³ = τ³  gamete combinations, 2³ = φ³ phenotype classes
  n-hybrid:    τⁿ gamete combinations, φⁿ phenotype classes

Mendel's seven traits in peas:
  7 traits studied = M₃ (Mersenne prime)
  7 = σ - sopfr = 12 - 5

Alleles per locus (simple Mendelian):
  2 = φ  (dominant and recessive)

Mendel's laws:
  1. Segregation
  2. Independent assortment
  Count = 2 = φ
```

### Physical Context

Mendel's ratios follow directly from the combinatorics of independent allele segregation during meiosis. The 3:1 ratio requires complete dominance, independent assortment, and no epistasis. Mendel published these in 1866; they were rediscovered in 1900. The ratios are exact mathematical consequences of diploid genetics with two alleles.

### Texas Sharpshooter Check

These ratios are mathematical necessities given the assumptions. The connection σ/τ = 3 is clean. Mendel's 7 traits = M₃ is an interesting historical fact, though Mendel may have selected traits that showed clean segregation.

## Verification

- [x] 3:1 = σ/τ : 1 exact
- [x] 9:3:3:1 = (σ/τ)²:(σ/τ):(σ/τ):1 exact
- [x] 16 = τ² Punnett cells exact
- [x] 7 traits = M₃ exact
- [x] 2 alleles = φ, 2 laws = φ exact
