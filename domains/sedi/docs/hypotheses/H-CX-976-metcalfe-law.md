# H-CX-976: Metcalfe's Law — Network Value Scaling

> **Hypothesis**: Metcalfe's law states network value is proportional to n^2. For n = P_1 = 6: value proportional to 36 = P_1^2 = (sigma/tau)^(sigma/tau+1). For n = sigma = 12: value proportional to 144 = sigma^2. Network value at n=6 nodes equals 6^2 = 36.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Metcalfe's law (Robert Metcalfe, 1980):
  V(n) ∝ n²  (network value scales as square of users)

  Possible connections: n(n-1)/2

At n=6 canonical nodes:
  V(P₁) ∝ P₁² = 36 = σ² (when σ = 12... no)
  P₁² = 36 ≠ σ² = 144
  Correction: P₁² = 36, σ² = 144 = 4 × 36 = τ × P₁²

  Connections at n = P₁:
    P₁(P₁-1)/2 = 6×5/2 = 15 = σ + σ/τ

At n = σ = 12 nodes:
  V(σ) ∝ σ² = 144
  Connections: σ(σ-1)/2 = 12×11/2 = 66
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Connections at key network sizes:
  n = φ:      φ(φ-1)/2 = 1     (single link)
  n = σ/τ:    3×2/2 = 3 = σ/τ  (triangle)
  n = τ:      4×3/2 = 6 = P₁   (complete K₄)
  n = sopfr:  5×4/2 = 10 = σ-φ
  n = P₁:     6×5/2 = 15
  n = M₃:     7×6/2 = 21 = σ/τ × M₃

Notable: K_τ (complete graph on τ nodes) has exactly
  P₁ edges. The first perfect number counts edges of K_τ.

Metcalfe vs. Reed vs. Odlyzko:
  Metcalfe: V ∝ n²
  Reed:     V ∝ 2ⁿ (group-forming networks)
  Odlyzko:  V ∝ n·log(n)
  At n = P₁: log(P₁) = log(6) ≈ 1.79
  n·log(n) = 6 × 1.79 ≈ 10.7 ≈ σ - 1
```

### Physical Context

Metcalfe's law governs the economics of network effects in telecommunications, social platforms, and marketplaces. The observation that K_tau has exactly P_1 edges provides a structural bridge: the complete graph on tau = 4 nodes generates the first perfect number of connections. This connects network topology to perfect number theory at the most basic level.

### Texas Sharpshooter Check

The K_tau = P_1 edges result (C(4,2) = 6) is exact and non-trivial. The connection counts at various n=6 values produce recognizable constants. Metcalfe's law itself (V proportional to n^2) is the simplest possible network scaling and the n=6 evaluations are direct substitutions rather than deep structural claims.

## Verification

- [x] K_τ has P₁ = 6 edges exact
- [x] K_{P₁} has 15 = σ + σ/τ edges exact
- [x] P₁² = 36 (Metcalfe value at n = P₁)
- [x] σ² = 144 (Metcalfe value at n = σ)
- [ ] Metcalfe's law itself is debated (may overestimate)
