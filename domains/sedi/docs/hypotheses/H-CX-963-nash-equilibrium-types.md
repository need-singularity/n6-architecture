# H-CX-963: Nash Equilibrium Types

> **Hypothesis**: A 2x2 game has at most 3 = sigma/tau Nash equilibria (the odd number theorem). The number of players in standard form = 2 = phi. Pure and mixed strategy NE count is bounded by sigma/tau.

## Grade: 🟧 APPROXIMATE

## Results

### The Correspondence

```
Nash equilibrium structure in finite games:
  Players in standard form:  2 = φ
  Strategies per player:     2 = φ (in simplest 2×2 game)
  Max NE in 2×2 game:        3 = σ/τ (odd number theorem)

Odd number theorem (Robert Wilson, 1971):
  Generic finite games have an odd number of NE.
  For 2×2: the count is 1 or 3 = σ/τ.
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σφ = 24, σ-τ = 8, C(6,3) = 20
```

### Structural Analysis

```
2×2 game anatomy:
  Payoff matrix entries:  4 = τ per player (2×2 grid)
  Total payoffs:          8 = σ-τ (4 per player × 2 players)
  Pure strategy profiles: 4 = τ (φ × φ grid)

NE classification:
  Pure strategy NE:   0, 1, or 2 (from τ profiles)
  Mixed strategy NE:  0 or 1 (interior equilibrium)
  Maximum total:      3 = σ/τ
```

### Physical Context

The odd number theorem guarantees that generic games have an odd number of Nash equilibria. In 2x2 games, this constrains the count to 1 or 3. The matching pair (phi players, sigma/tau max equilibria) suggests a structural link between the simplest game parameters and n=6 arithmetic.

### Texas Sharpshooter Check

The phi = 2 players match is structural but trivially the simplest game. The sigma/tau = 3 max NE is exact and non-trivial via the odd number theorem. The tau = 4 payoff entries per player is definitional for 2x2.

## Verification

- [x] 2 players = φ exact
- [x] Max 3 NE in 2×2 = σ/τ exact
- [x] 4 payoff entries per player = τ exact
- [x] 8 total payoffs = σ-τ exact
