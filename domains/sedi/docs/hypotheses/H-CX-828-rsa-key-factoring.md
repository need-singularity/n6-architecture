# H-CX-828: RSA Key Size and Factoring

> **Hypothesis**: The standard RSA security parameter 2048 bits = φ^(σ-1) = 2¹¹, where σ-1=11 connects the RSA key length to n=6 divisor-sum structure.

## Grade: 🟧★ SUGGESTIVE-PLUS

## Results

### The Formula

```
RSA security:
  Standard key: 2048 bits = 2¹¹ = φ^(σ-1)
  Exponent: 11 = σ - 1

Key size hierarchy:
  RSA-1024 = φ^(τ(P₃)) = 2¹⁰     (deprecated)
  RSA-2048 = φ^(σ-1) = 2¹¹        (standard)
  RSA-4096 = φ^σ = 2¹²            (high security)

Shor's algorithm:
  Quantum factoring: O(n³) gates for n-bit number
  For 2048-bit RSA: O(2048³) ≈ O(2³³) logical gates
  33 = σ·(σ/τ) - σ/τ = 12·3 - 3 = 33

RSA public exponent:
  Common e = 65537 = 2¹⁶ + 1 = Fermat prime F₄
  16 = φ^τ = 2⁴
  So e = φ^(φ^τ) + 1
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
RSA-2048:
  2048 = 2¹¹ ✓
  11 = σ - 1 = 12 - 1 ✓

Key progression:
  1024 = 2¹⁰ = φ^τ(P₃) ✓
  2048 = 2¹¹ = φ^(σ-1) ✓
  4096 = 2¹² = φ^σ ✓

Public exponent:
  65537 = 2¹⁶ + 1 ✓
  16 = φ^τ ✓
```

### Texas Sharpshooter Check

The RSA key size 2048 being 2¹¹ is exact, and 11=σ-1 is a simple TECS-L expression. The key progression 2¹⁰, 2¹¹, 2¹² mapping to τ(P₃), σ-1, σ is clean. However, key sizes are engineering choices driven by security margins against known attacks.

## Verification

- [x] RSA-2048 = φ^(σ-1) exact
- [x] Key progression through τ(P₃), σ-1, σ
- [x] Public exponent e = φ^(φ^τ) + 1
- [x] Shor gate count exponent relates to σ and σ/τ

## Status

New. RSA key sizes form a progression through n=6 constants in the exponent.
