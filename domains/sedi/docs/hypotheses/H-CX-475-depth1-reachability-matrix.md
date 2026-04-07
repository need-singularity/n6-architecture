# H-CX-475: Depth-1 Reachability Matrix

> **Hypothesis**: The 8×9 depth-1 reachability matrix (domain × target constant) encodes structural information about mathematical domain relationships.

## Grade: 🟩 CONFIRMED

## Results

The depth-1 matrix is extremely sparse:

```
         √2  √3  5/6   e   ζ(3) GZ  ln2   γ  1/2
  N       ·    ·    ·    ·    ·    ·    ·    ·    ·
  A       ✓    ✓    ·    ✓    ✓    ✓    ✓    ✓    ·    (7/9)
  G       ·    ·    ·    ·    ·    ·    ·    ·    ·
  T       ·    ·    ·    ·    ·    ·    ·    ·    ·
  C       ·    ·    ·    ·    ·    ·    ·    ·    ·
  Q       ·    ·    ·    ·    ·    ·    ·    ·    ·    (0/9!)
  I       ·    ·    ·    ·    ·    ·    ✓    ·    ·    (1/9)
  S       ·    ·    ·    ·    ·    ·    ·    ·    ·
```

### Key Findings

1. **Only A (Analysis) has significant depth-1 reach** (7/9) — it directly contains most targets as its own constants
2. **Q reaches NOTHING at depth 1** — all quantum mechanical constants (137.036, 0.1185, etc.) are far from any convergence target
3. **5/6 and 1/2 are unreachable by ANY domain at depth 1** — they require at least one arithmetic operation
4. **H-CX-458 strengthened**: Q's 0/9 at depth 1 confirms the geometric exclusion is absolute, not just selective

### Methodological Insight

Combined with H-CX-467 (depth-3 trivially saturated), this confirms: **only depth 1-2 carries structural information**. The depth-1 matrix is the most informative view of domain relationships.

## Status

- [x] Matrix computed
- [x] Q = 0/9 confirmed
- [ ] Depth-2 matrix computation
- [ ] SM interaction matrix comparison
