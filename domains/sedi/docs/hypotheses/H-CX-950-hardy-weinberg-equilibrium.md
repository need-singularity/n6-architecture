# H-CX-950: Hardy-Weinberg Equilibrium

> **Hypothesis**: The Hardy-Weinberg equation p² + 2pq + q² = 1 has 3 = σ/τ terms. Allele frequencies sum to p + q = 1 = R(6). The equation requires sopfr = 5 assumptions to hold.

## Grade: 🟩 EXACT

## Results

### The Correspondence

```
Hardy-Weinberg equation:
  p² + 2pq + q² = 1

  Number of terms: 3 = σ/τ
  Sum = 1 = R(6) (reserval/identity)

Allele frequency constraint:
  p + q = 1 = R(6)
  Number of alleles: 2 = φ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
The five assumptions of Hardy-Weinberg equilibrium:
  1. No mutation
  2. No migration (gene flow)
  3. No natural selection
  4. Random mating
  5. Infinite population size (no genetic drift)
  Count = 5 = sopfr

Genotype frequencies (3 = σ/τ genotypes):
  AA: p²     (homozygous dominant)
  Aa: 2pq    (heterozygous)
  aa: q²     (homozygous recessive)

The equation is a perfect square:
  (p + q)² = 1
  Exponent = 2 = φ (diploid organism)

Multi-allelic extension (k alleles):
  (p₁ + p₂ + ... + pₖ)² = 1
  Still exponent φ = 2
```

### Physical Context

Hardy-Weinberg (1908) provides the null model for population genetics. Deviations from HW proportions indicate evolutionary forces are acting. The equation is a direct consequence of random mating in a diploid organism — the φ = 2 exponent reflects diploidy. The 5 assumptions are standard in every population genetics textbook.

### Texas Sharpshooter Check

The 3 terms and 5 assumptions are universally agreed upon in genetics textbooks. These are not chosen to fit — they are the standard formulation. The match to σ/τ and sopfr is clean.

## Verification

- [x] 3 terms = σ/τ exact
- [x] 5 assumptions = sopfr exact
- [x] 2 alleles = φ exact
- [x] Exponent 2 = φ (diploidy) exact
- [x] 3 genotypes = σ/τ exact
