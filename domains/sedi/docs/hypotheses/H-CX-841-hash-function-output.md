# H-CX-841: Hash Function Output Sizes

> **Hypothesis**: Standard cryptographic hash output sizes — MD5 (128), SHA-256 (256), SHA-512 (512) — are powers of φ with TECS-L exponents, and SHA-3's Keccak state width 1600 = sopfr² · τ³.

## Grade: 🟧★ SUGGESTIVE-PLUS

## Results

### The Formula

```
Cryptographic hash output sizes:

MD5:     128 bits = φ^M₃ = 2⁷
SHA-1:   160 bits = φ^sopfr · sopfr = 32 · 5
SHA-256: 256 bits = φ^(σ-τ) = 2⁸
SHA-384: 384 bits = σ · φ^sopfr = 12 · 32
SHA-512: 512 bits = φ^(σ-τ+1) = 2⁹

SHA-3 (Keccak) internal state:
  Width: 1600 bits = sopfr² · τ³
  = 25 · 64
  = sopfr² · τ³

Keccak-f permutation:
  State: 5 × 5 × w array, w = 64
  5 = sopfr
  64 = τ³ = 4³
  Lanes: sopfr² = 25
  Lane width: τ³ = 64

Keccak capacity + rate = 1600:
  SHA3-256: rate 1088, capacity 512
    512 = φ^(σ-τ+1)
    1088 = 1600 - 512

BLAKE2/BLAKE3:
  Block size: 64 bytes = 512 bits = φ^(σ-τ+1)
  Output: 256 bits = φ^(σ-τ)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Hash output sizes:
  MD5:     128 = φ^M₃ ✓
  SHA-256: 256 = φ^(σ-τ) ✓
  SHA-512: 512 = φ^(σ-τ+1) ✓
  SHA-384: 384 = σ · φ^sopfr ✓

Keccak state:
  1600 = sopfr² · τ³ = 25 · 64 ✓
  State array: sopfr × sopfr × τ³ ✓

SHA3-256 capacity:
  512 = φ^(σ-τ+1) ✓
```

### Texas Sharpshooter Check

Hash output sizes are driven by the byte=8-bit convention, so powers of 2 dominate. The Keccak decomposition 1600 = sopfr² · τ³ is more interesting: the 5×5 state array was a specific design choice by Bertoni et al. that 5 = sopfr and lane width 64 = τ³ are TECS-L values adds structural depth beyond simple powers of 2.

## Verification

- [x] MD5/SHA-256/SHA-512: φ^M₃, φ^(σ-τ), φ^(σ-τ+1)
- [x] SHA-384 = σ · φ^sopfr
- [x] Keccak state = sopfr² · τ³ = 1600
- [x] SHA3-256 capacity = φ^(σ-τ+1) = 512

## Status

New. Hash function outputs span φ^M₃ to φ^(σ-τ+1), and Keccak's state decomposes as sopfr² · τ³.
