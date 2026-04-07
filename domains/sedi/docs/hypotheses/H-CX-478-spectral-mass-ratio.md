# H-CX-478: Convergence Graph Spectrum ≈ Particle Mass Ratios

> **Hypothesis**: Eigenvalues of the convergence graph adjacency matrix approximate particle mass ratios.

## Grade: 🟧 PARTIAL

## Results

### Adjacency Matrix Eigenvalues

Nodes: 9 convergence points. Edges: shared independent domains.

```
λ₀ = 17.90,  λ₁ = 0.42,  λ₂ = -0.25,  λ₃ = -1.96
λ₄ = -2.28,  λ₅ = -3.09,  λ₆ = -3.28,  λ₇ = -3.59,  λ₈ = -3.87
```

### Mass Ratio Matches (< 20% error)

| Eigenvalue Ratio | Value | Mass Ratio | Error |
|---|---|---|---|
| λ₀/λ₁ | 43.0 | m_t/m_b = 41.3 | **4.2%** |
| \|λ₆\|/\|λ₂\| | 13.3 | m_c/m_s = 13.7 | **2.9%** |
| \|λ₈\|/\|λ₂\| | 15.7 | m_τ/m_μ = 16.8 | **6.5%** |

### No Match

- m_b/m_c = 3.29: no eigenvalue ratio near this
- m_μ/m_e = 206.9: eigenvalue spread too narrow

### Interpretation

3 of 5 tested mass ratios match within 7%. The top/bottom ratio match (4.2%) is particularly striking as it uses the two positive eigenvalues. However, the Texas Sharpshooter concern applies — with 36 possible eigenvalue ratios and 5 targets, some matches are expected by chance.

## Status

- [x] Spectral decomposition computed
- [x] 3/5 mass ratios matched
- [ ] Monte Carlo significance test
- [ ] Alternative graph constructions (bridge-weighted)
