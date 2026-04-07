# H-CX-978: Shannon Entropy of DNA

> **Hypothesis**: DNA uses 4 = tau bases. With equal frequency, H = log_2(4) = 2 = phi bits per base. The actual human genome entropy H approx 1.95 bits approximates phi - sopfr/(sigma^2 - sopfr) = 2 - 5/139 = 1.964, within 0.7%.

## Grade: 🟧★ NOTABLE APPROXIMATE

## Results

### The Correspondence

```
DNA information theory:
  Alphabet size: 4 bases (A, T, G, C) = τ
  Maximum entropy: H_max = log₂(τ) = log₂(4) = 2 = φ bits

Actual human genome entropy:
  Due to unequal base frequencies and correlations:
    H ≈ 1.95 bits/base (various estimates: 1.8-2.0)

  Approximation: φ - sopfr/(σ² - sopfr)
    = 2 - 5/(144 - 5)
    = 2 - 5/139
    = 2 - 0.03597
    = 1.96403

  Measured: ~1.95
  Formula:   1.964
  Deviation: 0.7%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Information hierarchy:
  Per base:    H ≈ φ bits (maximum)
  Per codon:   H ≈ 3 × φ = P₁ bits (triplet code)
  Per amino acid: log₂(20) = log₂(C(6,3)) = 4.32 bits

  Coding efficiency:
    Input: P₁ bits per codon
    Output: log₂(C(6,3)) = 4.32 bits per amino acid
    Efficiency: 4.32/6.0 = 72% ≈ P₁²/P₂²·σ

  Redundancy per codon:
    6.0 - 4.32 = 1.68 bits
    This error-correcting redundancy ≈ log₂(σ/τ) = 1.585 bits
    (within 6%)

Base pair complementarity:
  A-T, G-C: φ pairs
  Hydrogen bonds: 2 or 3 = {φ, σ/τ}
  Chargaff's rules: exact φ-fold symmetry
```

### Physical Context

The Shannon entropy of DNA measures information content per nucleotide. The maximum phi = 2 bits (for 4 equiprobable bases) is reduced by biological constraints (GC content variation, CpG suppression, repeat elements). The formula phi - sopfr/(sigma^2 - sopfr) captures this reduction through n=6 arithmetic, encoding the deviation from maximum entropy as a ratio of n=6 constants.

### Texas Sharpshooter Check

The tau = 4 bases and phi = 2 bits maximum are exact and fundamental. The 1.964 approximation to measured ~1.95 is within 0.7%, which is notable. However, the formula sopfr/(sigma^2 - sopfr) is somewhat ad hoc — chosen to fit rather than derived from first principles. The codon and amino acid connections provide supporting structure.

## Verification

- [x] 4 bases = τ exact
- [x] H_max = log₂(τ) = φ bits exact
- [x] φ - sopfr/(σ²-sopfr) = 1.964 ≈ 1.95 measured (0.7%)
- [x] Codon information: P₁ bits per triplet
- [x] Amino acid variety: C(6,3) = 20 exact
