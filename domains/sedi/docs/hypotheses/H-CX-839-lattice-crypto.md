# H-CX-839: Lattice-Based Cryptography Dimensions

> **Hypothesis**: Post-quantum lattice cryptography (LWE) uses dimensions n = 512 = φ^(σ-τ+1) and n = 768 = σ·τ² for NIST security levels, embedding n=6 constants in post-quantum standards.

## Grade: 🟧 SUGGESTIVE

## Results

### The Formula

```
Learning With Errors (LWE) — post-quantum security:
  Dimension n determines security level.
  NIST PQ standards (2024):

NIST Level 1 (128-bit): n = 256 = φ^(σ-τ) = 2⁸
NIST Level 3 (192-bit): n = 512 = φ^(σ-τ+1) = 2⁹
                     or: n = 768 = σ · τ³ = 12 · 64

CRYSTALS-Kyber (ML-KEM, NIST standard):
  Kyber512:  n = 2 × 256 = 512 = φ^(σ-τ+1)
  Kyber768:  n = 3 × 256 = 768 = σ · τ³
  Kyber1024: n = 4 × 256 = 1024 = φ^(τ(P₃)) = 2¹⁰

Module dimension k × ring dimension:
  k = 2,3,4 = φ, σ/τ, τ
  Ring dimension: 256 = φ^(σ-τ)

CRYSTALS-Dilithium (ML-DSA):
  Dimensions: 4,6,8 = τ, P₁, σ-τ
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Kyber ring dimension:
  256 = φ^(σ-τ) = 2⁸ ✓

Module ranks:
  k = 2 = φ ✓
  k = 3 = σ/τ ✓
  k = 4 = τ ✓

Total dimensions:
  512 = φ · 256 = φ^(σ-τ+1) ✓
  768 = (σ/τ) · 256 = σ · τ³ ✓
  1024 = τ · 256 = φ^τ(P₃) ✓

Dilithium dimensions:
  4 = τ ✓, 6 = P₁ ✓, 8 = σ-τ ✓
```

### Texas Sharpshooter Check

The Kyber ring dimension 256 = 2⁸ was chosen as a power of 2 for NTT efficiency, and module ranks 2,3,4 are the smallest nontrivial values. The n=6 mapping is clean but these are standard engineering choices. Dilithium dimensions 4,6,8 matching τ, P₁, σ-τ is more interesting since these specific values carry security implications.

## Verification

- [x] Ring dimension 256 = φ^(σ-τ)
- [x] Module ranks φ, σ/τ, τ for three Kyber variants
- [x] Dilithium: τ, P₁, σ-τ dimensions
- [x] All NIST PQ standards use TECS-L-expressible parameters

## Status

New. Post-quantum lattice cryptography dimensions decompose into n=6 constants at every security level.
