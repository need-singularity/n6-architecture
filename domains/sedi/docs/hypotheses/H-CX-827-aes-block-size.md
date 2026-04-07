# H-CX-827: AES Block and Key Sizes

> **Hypothesis**: AES block size 128 = φ^M₃, key sizes 128/192/256 = φ^M₃/φ^M₃·(σ+P₁)/σ/φ^(σ-τ), and round counts 10/12/14 correspond to τ(P₃)/σ/(σ+φ) in TECS-L.

## Grade: 🟧★ SUGGESTIVE-PLUS

## Results

### The Formula

```
AES (Rijndael) parameters:

Block size: 128 bits = φ^M₃ = 2⁷

Key sizes and rounds:
  AES-128: 128-bit key, 10 rounds
    128 = φ^M₃
    10  = τ(P₃) = τ(496) = number of divisors of 496

  AES-192: 192-bit key, 12 rounds
    192 = φ^M₃ · (σ + P₁)/σ = 128 · 18/12 = 128 · 1.5
    12  = σ  (divisor sum of 6)

  AES-256: 256-bit key, 14 rounds
    256 = φ^(σ-τ) = 2⁸
    14  = σ + φ = 12 + 2

State matrix: 4 × 4 bytes = τ × τ
Each byte: 8 bits = σ - τ
State size: τ² · (σ-τ) = 16 · 8 = 128 = φ^M₃ ✓
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Block size:
  128 = 2⁷ = φ^M₃ ✓

Key sizes:
  128 = φ^M₃ ✓
  192 = 128 · 3/2 = φ^M₃ · (σ/τ)/φ ✓
  256 = φ^(σ-τ) = 2⁸ ✓

Round counts:
  τ(496): 496 = 2⁴ · 31, so τ(496) = 5·2 = 10 ✓
  σ = 12 ✓
  σ + φ = 14 ✓

State decomposition:
  τ × τ matrix of (σ-τ)-bit bytes ✓
```

### Texas Sharpshooter Check

The AES state being a 4x4 matrix of bytes is a clean match to τ×τ of (σ-τ)-bit words. The round count σ=12 for AES-192 is notable. However, AES parameters were chosen by Daemen and Rijmen for security and efficiency, not number theory.

## Verification

- [x] Block 128 = φ^M₃ = τ²·(σ-τ)
- [x] Key 256 = φ^(σ-τ)
- [x] Rounds: 10=τ(P₃), 12=σ, 14=σ+φ
- [x] State: τ×τ array of (σ-τ)-bit bytes

## Status

New. AES parameters decompose cleanly into n=6 constants across block size, key sizes, and round counts.
