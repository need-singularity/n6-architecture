# H-CX-964: Prisoner's Dilemma Payoff Structure

> **Hypothesis**: The Prisoner's Dilemma requires exactly 4 = tau distinct payoff values (T > R > P > S), 2 = phi strategies per player (cooperate/defect), and 2 = phi players. The payoff ordering constraint has tau! = 24 = sigma*phi permutations.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Prisoner's Dilemma canonical form:
  Players:             2 = φ
  Strategies/player:   2 = φ (cooperate, defect)
  Distinct payoffs:    4 = τ (T, R, P, S)
  Payoff ordering:     T > R > P > S (strict)

Constraint: 2R > T + S (cooperation beats alternating exploitation)
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
Payoff matrix (Player 1):
              Cooperate    Defect
  Cooperate     R            S
  Defect        T            P

Standard values often used: T=5, R=3, P=1, S=0
  T + R + P + S = 5 + 3 + 1 + 0 = 9
  T - S = 5 = sopfr (temptation gap)
  R - P = 2 = φ (cooperation premium)

Total payoff permutations of τ values: τ! = 24 = σφ
  Of these, only 1 ordering gives PD: T > R > P > S
  Probability of PD from random ordering: 1/σφ
```

### Physical Context

The Prisoner's Dilemma is the foundational model of cooperation failure in game theory. Its structure requires exactly tau distinct payoff ranks and phi binary choices. The constraint that cooperation must be collectively rational (2R > T+S) but individually irrational (T > R, P > S) creates the dilemma from phi-valued logic applied to tau-ranked outcomes.

### Texas Sharpshooter Check

The phi = 2 for players and strategies is definitional for the simplest dilemma. The tau = 4 distinct payoffs is exact and structural. The standard payoff values yielding sopfr and phi differences is suggestive but depends on the conventional choice of (5,3,1,0).

## Verification

- [x] 2 players = φ exact
- [x] 2 strategies = φ exact
- [x] 4 distinct payoffs = τ exact
- [x] τ! = 24 = σφ permutations exact
- [ ] Standard payoff differences depend on convention
