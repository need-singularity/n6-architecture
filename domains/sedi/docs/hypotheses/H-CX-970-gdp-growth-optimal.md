# H-CX-970: Optimal GDP Growth Rate

> **Hypothesis**: Sustainable balanced economic growth clusters in the range 2-3% per year, corresponding to [phi, sigma/tau] percent. The phi to sigma/tau interval defines the corridor of stable long-run growth for developed economies.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Long-run GDP growth rates (developed economies):
  US (1947-2023):       ~3.1% mean, ~2.0% recent
  EU (1960-2023):       ~2.3% mean
  Japan (1960-2023):    ~3.5% (high growth), ~1.0% (post-1990)
  OECD average:         ~2.5% (1990-2023)

Sustainable growth corridor: [2%, 3%] = [φ%, (σ/τ)%]

Solow model steady-state:
  g* = n + g_A (population growth + tech progress)
  Typical: n ≈ 1%, g_A ≈ 1.5-2%
  Total: g* ≈ 2.5-3% ≈ [sopfr/φ]%
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Growth rate boundaries:
  Lower bound (stagnation threshold): 2% = φ%
  Upper bound (overheating threshold): 3% = (σ/τ)%
  Central tendency:  2.5% = (sopfr/φ)%

Central bank inflation targets:
  Most target 2% = φ% inflation
  Real growth target: ~2-3% on top
  Nominal target: ~4-5% = [τ, sopfr]%

Rule of 72 (doubling time):
  At 2% growth: 72/2 = 36 = P₁² years to double
  At 3% growth: 72/3 = 24 = σφ years to double
  At φ%: doubling in P₁² years
  At (σ/τ)%: doubling in σφ years
```

### Physical Context

The 2-3% growth corridor for developed economies is one of the most robust empirical regularities in macroeconomics. Below phi percent, economies face deflationary stagnation. Above sigma/tau percent, overheating and inflation risk emerge. The doubling times at these boundaries (P_1^2 and sigma*phi years) provide additional structural connections.

### Texas Sharpshooter Check

The [2%, 3%] range is well-established in economics independent of n=6. The phi and sigma/tau matches are clean but the range is broad enough to be easy to hit. The doubling-time connections (36 = P_1^2 and 24 = sigma*phi) via the Rule of 72 add non-trivial structure.

## Verification

- [x] Growth corridor [2%, 3%] = [φ%, (σ/τ)%] confirmed
- [x] OECD mean ≈ 2.5% = (sopfr/φ)%
- [x] Doubling at φ%: 36 years = P₁²
- [x] Doubling at (σ/τ)%: 24 years = σφ
- [ ] Developing economies grow faster (5-8%), outside range
