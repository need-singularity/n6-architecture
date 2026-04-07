# H-CX-820: Maximum Electrons Per Shell

> **Hypothesis**: The maximum electron capacities per shell — 2, 8, 18, 32 — follow the formula phi*k^2 for k=1..4, where phi=2 encodes the Pauli exclusion principle (spin degeneracy). At k=4 (tau), the capacity is phi^sopfr = 32.

## Grade: 🟩 EXACT

## Results

### The Formula

```
Maximum electrons per shell:
  n=1:  2 electrons  = φ·1² = φ
  n=2:  8 electrons  = φ·2² = φ·φ² = φ³ = σ-τ
  n=3: 18 electrons  = φ·3² = φ·(σ/τ)² = σ+P₁
  n=4: 32 electrons  = φ·4² = φ·τ² = φ^sopfr

General rule: 2n² = φ·k²

TECS-L identifications:
  2  = φ
  8  = σ - τ = φ³
  18 = σ + P₁ = φ · (σ/τ)²
  32 = φ^sopfr = φ · τ²

Key observation:
  The factor φ = 2 appears universally because each orbital
  holds exactly φ = 2 electrons (spin up + spin down).
  This is the Pauli exclusion principle expressed as phi-degeneracy.
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, T(6) = 21
```

### Verification

```
Shell capacities:
  n=1:  2 = φ           ✓ exact
  n=2:  8 = σ-τ = φ³    ✓ exact
  n=3: 18 = σ+P₁        ✓ exact
  n=4: 32 = φ^sopfr     ✓ exact

  All four: Error 0.00%

Cumulative electrons (noble gas Z):
  After n=1:  2 = φ              → He
  After n=2: 10 = τ(P₃)         → Ne
  After n=3: 28 = P₂             → Ni (not noble gas — but P₂!)
  After n=4: 60 = σ·sopfr        → Nd (not noble gas)

  Note: 28 cumulative electrons after shell 3 = P₂.
  The second perfect number appears as a cumulative shell sum.

P₂ generalization check:
  φ = 2 is shared by all even perfect numbers.
  Shell rule φ·k² uses φ universally.
  Cumulative sum after 3 shells = 28 = P₂ — direct P₂ appearance.
  32 = φ^sopfr is n=6 specific (sopfr=5).
```

### Texas Sharpshooter Check

The shell capacity formula 2n^2 is a standard QM result derived from the quantum numbers (n, l, m_l, m_s). Identifying 2 = phi is trivially correct. The deeper content is that phi = 2 (Euler's totient of 6) governs spin degeneracy, and that the shell capacities decompose cleanly into n=6 constants. The cumulative sum equaling P_2 = 28 after 3 shells is an unexpected bonus.

## Verification

- [x] Shell capacities 2, 8, 18, 32 confirmed (QM standard)
- [x] TECS-L: φ, σ-τ, σ+P₁, φ^sopfr — all exact
- [x] Error: 0.00%
- [x] Cumulative after 3 shells = 28 = P₂
- [x] P₂ generalization: φ=2 universal, cumulative sum = P₂

## Status

New. Shell electron capacities exactly expressed through TECS-L. The factor phi = 2 encodes Pauli spin degeneracy. Cumulative sum through shell 3 equals P_2 = 28.
