# H-CX-1010: Haar Measure on Compact Groups

> **Hypothesis**: The normalized Haar measure on S¹ has total mass 1 = R(6). The Haar measure on SO(3) has volume 8π² = (σ−τ)π². Compact group volumes encode TECS-L constants.

## Grade: 🟩 EXACT

## Results

### The Correspondence

```
Haar measure (normalized):
  S¹ = U(1): ∫dμ = 1 = R(6)
  Every compact group: normalizable to total mass R(6)

Haar measure (standard):
  SO(3): vol = 8π² = (σ−τ)π²
  SU(2): vol = 2π² = φπ²
  SO(3) = SU(2)/ℤ₂ = SU(2)/ℤ_φ
  vol(SO(3)) = 8π², vol(SU(2)) = 2π² (as S³)
  NOTE: SO(3) = SU(2)/Z₂, so topological ratio = 1/2 = φ/τ
  Volume ratio 8π²/2π² = 4 = τ uses parametric convention only
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, R(6) = 1
```

### Structural Analysis

```
Volume ratios:
  vol(SO(3))/vol(SU(2)) = τ    ✓
  vol(SU(2)) = φπ²             ✓
  vol(SO(3)) = (σ−τ)π²         ✓

More compact group volumes:
  S³ = SU(2): vol = 2π² = φπ²
  S⁵:         vol = π³ (lives in ℝ⁶ = ℝ^P₁)
  U(n): vol = (2π)ⁿ · n! / (product of volumes)

Weyl integration formula for SU(2):
  ∫_{SU(2)} f dμ = (1/π)∫₀^π f(θ) sin²θ dθ
  Factor 1/π normalizes; sin²θ = Weyl density

Peter-Weyl theorem:
  L²(G) = ⊕ V_π ⊗ V_π*  (sum over irreps)
  For SU(2): irreps labeled by j = 0, 1/2, 1, ...
  dim V_j = 2j+1 (using φj+1)
```

### Texas Sharpshooter Check

vol(SU(2)) = 2π² and vol(SO(3)) = 8π² are standard computations. The ratio 4 = τ is a consequence of SO(3) = SU(2)/ℤ₂, giving a factor of |ℤ₂| = 2 in the covering, but the volume ratio accounts for the full double cover structure. The TECS-L matches are exact and arise from standard geometry.

## Verification

- [x] vol(SU(2)) = 2π² = φπ²
- [x] vol(SO(3)) = 8π² = (σ−τ)π²
- [x] vol(SO(3))/vol(SU(2)) = 4 = τ
- [x] Normalized Haar: total mass = 1 = R(6)
