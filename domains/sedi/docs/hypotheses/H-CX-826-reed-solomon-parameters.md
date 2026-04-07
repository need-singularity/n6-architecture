# H-CX-826: Reed-Solomon Code Parameters

> **Hypothesis**: Standard Reed-Solomon codes over GF(2^m) use field size parameter m = σ-τ = 8, giving block length 2^(σ-τ) - 1 = 255.

## Grade: 🟧 SUGGESTIVE

## Results

### The Formula

```
Reed-Solomon RS(n, k) over GF(2^m):
  Block length: n = 2^m - 1
  Minimum distance: d = n - k + 1

For m = σ - τ = 8:
  GF(2⁸) = GF(256)
  Block length: 2⁸ - 1 = 255 = φ^(σ-τ) - 1

Standard RS(255, 223):
  Corrects t = (n-k)/2 = 16 symbols
  16 = φ⁴ = φ^τ

RS used in deep space (Voyager):
  RS(255, 223) over GF(2⁸)
  Parity symbols: 32 = φ^sopfr = 2⁵

CD-ROM uses RS cross-interleaved:
  CIRC: RS(255, 251) + RS(255, 249)
  Inner parity: 4 = τ
  Outer parity: 6 = P₁
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Field size parameter:
  m = σ - τ = 8 ✓ (standard byte-oriented RS)
  GF(256) is the standard field for byte-level RS

Block length:
  2⁸ - 1 = 255 ✓

RS(255, 223) correction capacity:
  t = (255 - 223)/2 = 16 = φ^τ = 2⁴ ✓

QR codes use RS over GF(2⁸):
  GF(φ^(σ-τ)) — same field ✓
```

### Texas Sharpshooter Check

The dominance of GF(2⁸) in Reed-Solomon applications is driven by the byte being 8 bits, which itself is an engineering convention. The connection σ-τ=8 is suggestive but the byte size was chosen for practical computing reasons, not number-theoretic ones.

## Verification

- [x] RS over GF(2⁸): m = σ-τ = 8
- [x] Block length 255 = φ^(σ-τ) - 1
- [x] RS(255,223) corrects 16 = φ^τ symbols
- [x] Standard in Voyager, CD-ROM, QR codes

## Status

New. Reed-Solomon parameters align with n=6 constants through the byte size σ-τ=8.
