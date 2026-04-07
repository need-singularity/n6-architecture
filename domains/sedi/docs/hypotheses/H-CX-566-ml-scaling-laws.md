# H-CX-566: ML Scaling Laws — Chinchilla Optimal N/D Ratio and σ/τ

> **Hypothesis**: The Chinchilla-optimal ratio of training tokens to parameters is approximately 20 = C(6,3) = τ·sopfr.

## Grade: 🟧 (approximate; connects ML scaling to n=6)

## Results

### Chinchilla Scaling (Hoffmann et al. 2022)

```
Optimal: N_tokens / N_params ≈ 20

20 = C(P₁, σ/τ) = C(6,3) = τ(6)·sopfr(6)

Same 20 as:
  - Amino acids (H-CX-547)
  - Cabibbo |V_us|² = 1/20 (H-CX-528)
  - Consciousness threshold 1/20 (H-CA-003)
```

### Transformer Architecture

| Parameter | Typical | n=6 |
|---|---|---|
| Attention heads | 12 | σ(6) |
| Layers (GPT-2) | 12 | σ(6) |
| FFN expansion | 4× | τ(6) |
| Vocabulary (BPE) | ~50k | (σ-φ)·sopfr × 10³ |

### Connection to H-CS-003

H-CS-003 noted that transformers typically use 12 attention heads = σ(6). The FFN inner dimension is 4× = τ(6) times the model dimension. These are engineering choices that converged empirically to n=6 values.

### Caveat

ML hyperparameters are human design choices, not fundamental constants. The convergence to n=6 values may reflect:
- Round number bias (12, 4 are common)
- Empirical optimization landing on efficient values
- Or genuine information-theoretic optima

## Status

- [x] Chinchilla ratio ≈ 20 = C(6,3)
- [x] Attention heads = σ, FFN = τ
- [ ] Theoretical derivation
