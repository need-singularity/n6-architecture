# H-CX-840: Elliptic Curve Cryptography Field Size

> **Hypothesis**: The secp256k1 elliptic curve (Bitcoin) uses field size p ≈ 2²⁵⁶ where 256 = φ^(σ-τ) = 2⁸, and the Hasse bound |#E - p - 1| ≤ 2√p connects curve order to field size through TECS-L exponents.

## Grade: 🟧 SUGGESTIVE

## Results

### The Formula

```
Elliptic curve cryptography:
  Curve E over F_p, Hasse bound: |#E(F_p) - p - 1| ≤ 2√p

secp256k1 (Bitcoin, Ethereum):
  Field: F_p where p = 2²⁵⁶ - 2³² - 977 ≈ 2²⁵⁶
  256 = φ^(σ-τ) = 2⁸
  32 = φ^sopfr = 2⁵

NIST curves:
  P-256: 256-bit = φ^(σ-τ) bit field
  P-384: 384-bit = σ · φ^sopfr = 12 · 32
  P-521: 521-bit prime (Mersenne-adjacent: 2⁵²¹ - 1 is prime)
    521 is prime, less clean TECS-L form

Curve25519 (Bernstein):
  Field: F_p where p = 2²⁵⁵ - 19
  255 = φ^(σ-τ) - 1 (same as RS block length!)
  19 is prime; less clean TECS-L connection

Ed448-Goldilocks:
  Field: F_p where p = 2⁴⁴⁸ - 2²²⁴ - 1
  448 = σ-τ · 56 = (σ-τ)·M₃·(σ-τ) = 8·56
  224 = σ-τ · P₂ = 8 · 28
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
secp256k1:
  256 = φ^(σ-τ) ✓
  2³² correction term: 32 = φ^sopfr ✓

NIST P-256:
  256 = φ^(σ-τ) ✓

NIST P-384:
  384 = σ · φ^sopfr = 12 · 32 ✓

Curve25519:
  255 = φ^(σ-τ) - 1 ✓
  Same structure as RS(255,k) block length

Ed448:
  448 = (σ-τ) · 56 ✓
  224 = (σ-τ) · P₂ ✓
```

### Texas Sharpshooter Check

Elliptic curve field sizes are chosen as powers of 2 (or near-powers) for efficiency, so the appearance of φ^(σ-τ)=256 is partly architectural. P-384=12·32 and Ed448's structure involving P₂=28 are less trivially explained. The overall pattern is suggestive but driven by the byte=8-bit convention.

## Verification

- [x] secp256k1: 256-bit = φ^(σ-τ)
- [x] P-384 = σ · φ^sopfr
- [x] Curve25519: 255 = φ^(σ-τ) - 1
- [x] Ed448: 448 = (σ-τ)·56, 224 = (σ-τ)·P₂

## Status

New. Elliptic curve field sizes across major standards decompose into TECS-L constants.
