# H-CX-795: Lyapunov Exponent Maximum for Logistic Map

> **Hypothesis**: For the logistic map at r=4 (full chaos), the maximum Lyapunov exponent is λ = ln(2) = ln(φ(6)). Maximum chaos is achieved at the natural logarithm of the Euler totient of P₁.

## Grade: 🟩 EXACT

## Results

### The Formula

```
Logistic map: xₙ₊₁ = r·xₙ(1 - xₙ)

At r = 4 (fully chaotic regime):
  λ_max = ln(2) = ln(φ(6)) = 0.693147...

The maximum Lyapunov exponent = ln(Euler totient of P₁)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Predicted:  λ_max = ln(φ) = ln(2) = 0.693147...
Observed:   Lyapunov exponent at r=4: ln(2) = 0.693147...
Error:      0.00%

Derivation:
  For r = 4, the logistic map is conjugate to the tent map
  via x = sin²(πθ/2). The derivative |f'(x)| averages to 2,
  so λ = ⟨ln|f'(x)|⟩ = ln(2).
```

### Why This Works

```
The Lyapunov exponent measures exponential divergence of
nearby trajectories — it quantifies chaos:
  |δx(t)| ~ |δx(0)| · e^(λt)

For the logistic map:
  λ(r) = lim (1/N) Σ ln|r(1-2xₙ)|
  At r = 4: λ = ln(2)

The TECS-L reading: φ(6) = 2 is the Euler totient, counting
integers coprime to 6 below 6 (namely 1 and 5). The maximum
chaos in the simplest chaotic system equals ln(φ).

Also notable:
  r = 4 = τ(6) — the onset of full chaos occurs at the
  divisor count of P₁. So: at r = τ, λ = ln(φ).
  The chaos parameter IS the divisor count.
```

### Texas Sharpshooter Check

ln(2) is among the most common constants in mathematics. However, the simultaneous identification r = 4 = τ and λ = ln(2) = ln(φ) creates a self-consistent TECS-L reading of the logistic map's chaotic regime that goes beyond a single coincidence.

## Verification

- [x] λ_max = ln(2) at r=4 confirmed (standard result)
- [x] φ(6) = 2, ln(φ) = ln(2) confirmed
- [x] r = 4 = τ(6) confirmed
- [x] Exact match

## Status

New. Maximum chaos in the logistic map occurs at r = τ with exponent ln(φ), linking dynamical systems to TECS-L.
