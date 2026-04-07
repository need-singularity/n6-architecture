# H-CX-467: Depth-3 Universal Saturation

> **Hypothesis**: At depth 3, ALL domains achieve near-100% participation in convergence points.

## Grade: ⚪ REJECTED (trivial)

## Results

At depth 3, every domain (including the previously isolated S) reaches 9/9 top convergence points. However, **random constant sets also achieve 100% saturation at depth 3**.

```
Real domains at depth 3:   9/9 = 100%
Random domains at depth 3: 9/9 = 100% (also!)
```

## Why This is Trivial

With 3 arithmetic operations (+, -, ×, ÷) and 5+ constants per domain, the combinatorial explosion at depth 3 is sufficient to reach ANY target value within 0.1% tolerance. This is a property of combinatorial density, not mathematical structure.

## Key Lesson

**Only depth 1-2 reachability is structurally meaningful.** Depth 3+ is combinatorially saturated and cannot distinguish signal from noise.

This VALIDATES the focus of H-CX-453 on depth-2 analysis and strengthens H-CX-457 (S isolation at depth 2) and H-CX-458 (Q selectivity at depth 1).

## Status: Rejected — but important methodological finding
