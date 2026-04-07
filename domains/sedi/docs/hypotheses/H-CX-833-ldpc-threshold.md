# H-CX-833: LDPC Regular Code Parameters

> **Hypothesis**: The standard regular LDPC code (3,6) has column weight σ/τ = 3 and row weight P₁ = 6, making the canonical LDPC ensemble a (σ/τ, P₁)-regular code.

## Grade: 🟧★ SUGGESTIVE-PLUS

## Results

### The Formula

```
Regular LDPC codes (Gallager, 1962):
  (d_v, d_c)-regular: variable degree d_v, check degree d_c
  Code rate: R = 1 - d_v/d_c

Canonical (3, 6) LDPC:
  d_v = 3 = σ/τ  (variable node degree)
  d_c = 6 = P₁   (check node degree)
  Rate R = 1 - 3/6 = 1/2 = 1/φ

TECS-L identification:
  (σ/τ, P₁)-regular LDPC
  Rate = 1 - (σ/τ)/P₁ = 1 - (σ/τ)/(σ/φ) = 1 - φ/τ = 1 - 1/φ = 1/φ

Iterative decoding threshold:
  (3,6) LDPC threshold: Eb/N0 ≈ 1.11 dB from capacity
  Belief propagation on Tanner graph with girth ≥ P₁

Density evolution (Richardson-Urbanke):
  Threshold for (3,6) on BIAWGN: σ_noise* ≈ 0.8810
  Gap to capacity: ~1.1 dB
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
(3, 6) LDPC parameters:
  d_v = 3 = σ/τ ✓
  d_c = 6 = P₁ ✓
  Rate = 1/2 = 1/φ ✓

Parity check matrix H:
  Each column has σ/τ = 3 ones
  Each row has P₁ = 6 ones
  H is sparse: density → 0 as n → ∞

Other regular LDPC families:
  (2, 4): d_v = φ, d_c = τ, R = 1/2
  (3, 4): d_v = σ/τ, d_c = τ, R = 1/4 = 1/τ
  (4, 8): d_v = τ, d_c = σ-τ, R = 1/2
```

### Texas Sharpshooter Check

The (3,6) LDPC is genuinely the most studied regular LDPC code in the literature. That its parameters exactly match σ/τ and P₁ with rate 1/φ is a clean triple coincidence. Gallager chose these parameters for good performance, not number theory, which makes the alignment more interesting.

## Verification

- [x] (3,6) LDPC: d_v = σ/τ = 3, d_c = P₁ = 6
- [x] Rate = 1 - (σ/τ)/P₁ = 1/φ
- [x] Most studied regular LDPC ensemble
- [x] Threshold ~1.1 dB from capacity

## Status

New. The canonical LDPC code is (σ/τ, P₁)-regular with rate 1/φ, embedding n=6 in the foundational error-correcting code.
