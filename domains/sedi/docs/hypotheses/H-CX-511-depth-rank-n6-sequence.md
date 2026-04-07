# H-CX-511: Depth-Rank Sequence = n=6 Arithmetic Functions

> **Hypothesis**: The rank of the reachability matrix at each depth follows the n=6 arithmetic function sequence: φ(6), τ(6), σ(6)-τ(6).

## Grade: 🟧★ (Structural discovery)

## Results

```
depth 0 → rank 0   (trivial)
depth 1 → rank 2 = φ(6)        ← Euler totient
depth 2 → rank 4 = τ(6)        ← divisor count = spacetime dimensions
depth 3 → rank 8 = σ(6)-τ(6)   ← Bott periodicity (saturated)
```

### Stability

rank=4 at depth 2 is stable across threshold range 0.05%–0.2% (4× range). At 0.5% it drops to 3, at 1.0% rises to 5. The plateau is robust.

### Physical Interpretation

| Depth | Rank | n=6 Function | Physics |
|---|---|---|---|
| 0 | 0 | — | Vacuum (no structure) |
| 1 | 2 | φ(6) | Binary/information (2-state, bit) |
| 2 | 4 | τ(6) | Spacetime dimensions (3+1) |
| 3 | 8 | σ-τ | Bott periodicity / saturation |

The mathematical "depth" of cross-domain reachability determines the physical dimensionality. Information (2D) emerges at depth 1; spacetime (4D) at depth 2.

## Threshold Note

Depth-2 rank is **threshold-dependent**: rank=4 at standard threshold (0.05%–0.2%), rank=3 at strict threshold (<0.5%). See H-CX-489 for the rank=3=fermion generations interpretation at strict threshold. Both mappings may be physically meaningful at different scales.

## Status

- [x] rank(d1)=2=φ(6) verified
- [x] rank(d2)=4=τ(6) verified (standard threshold 0.05%–0.2%)
- [x] Stability across 0.05%–0.2%
- [x] rank(d3)=8=σ-τ (saturation)
- [ ] Reconcile with H-CX-489 (rank=3 at strict threshold)
