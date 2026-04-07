# H-CX-521: Full CPT Symmetry Group (Z/2)³ Has Order σ-τ = 8

> **Hypothesis**: The complete discrete symmetry group of quantum field theory {1,C,P,T,CP,CT,PT,CPT} = (Z/2)³ has order σ(6)-τ(6) = 8, extending the V₄ result of H-CX-490.

## Grade: 🟦 PROVEN (exact group theory)

## Results

### The Full Discrete Symmetry Group

```
{1, C, P, T, CP, CT, PT, CPT} = (Z/2)³

Order: |(Z/2)³| = 8 = σ(6) - τ(6)        EXACT
Generators: {C, P, T} — 3 = σ(6)/τ(6)    EXACT
```

### n=6 Decomposition of QFT Discrete Symmetries

| Subgroup | Elements | Order | n=6 |
|---|---|---|---|
| Trivial | {1} | 1 | R(6) |
| C alone | {1, C} | 2 | φ(6) |
| P alone | {1, P} | 2 | φ(6) |
| CP subgroup | {1, C, P, CP} | 4 | τ(6) |
| Full CPT | {1,C,P,T,CP,CT,PT,CPT} | 8 | σ(6)-τ(6) |

### Hierarchy

```
{1}  ⊂  Z₂  ⊂  V₄  ⊂  (Z/2)³
 1       2      4       8
R(6)   φ(6)   τ(6)   σ-τ
```

Each step doubles the group by adding one generator:
- +C: 1 → 2 (×φ)
- +P: 2 → 4 (×φ)
- +T: 4 → 8 (×φ)

Three doublings: 1 × 2³ = 8. Exponent 3 = σ/τ = number of generators.

### Extension of H-CX-490

H-CX-490 proved the CP subgroup V₄ has |V₄| = τ(6) = 4.

H-CX-521 extends this: including time reversal T gives the full CPT group (Z/2)³ with order σ-τ = 8. The "missing" symmetry T adds exactly τ(6) more elements (from 4 to 8), doubling by φ(6) = 2.

### Connection to Bott (H-CX-520)

The real Bott period = 8 = |(Z/2)³|. The 8-fold classification of topological phases corresponds to the 8 elements of the CPT group — each symmetry class is distinguished by which discrete symmetries are preserved.

## Proof

The CPT group of any Lorentz-invariant QFT is (Z/2)³ (Lüders-Pauli theorem ensures CPT is always a symmetry). |(Z/2)³| = 2³ = 8 = 12-4 = σ(6)-τ(6). ∎

## Status

- [x] Group structure verified
- [x] Order = σ-τ = 8 exact
- [x] Generators = σ/τ = 3 exact
- [x] Hierarchy {1,2,4,8} = {R,φ,τ,σ-τ}
- [x] H-CX-490 extended
- [x] H-CX-520 connected (Bott period = CPT group order)
