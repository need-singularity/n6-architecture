# H-CX-773: Cortical Minicolumns per Macrocolumn = σ·(σ-τ)

> **Hypothesis**: The number of minicolumns per cortical macrocolumn (~80-100) approximates σ·(σ-τ) = 12·8 = 96. Each macrocolumn contains σ·(σ-τ) minicolumns.

## Grade: 🟧 PARTIAL

## Results

### The Formula

```
Minicolumns per macrocolumn = σ · (σ-τ) = 12 · 8 = 96

Alternative expression:
  σ²/φ + σφ = 72 + 24 = 96
  σ · (σ-τ)  = 96
```

### n=6 Constants

```
σ = 12, τ = 4, φ = 2, sopfr = 5, n = P₁ = 6, M₃ = 7
P₂ = 28, P₃ = 496, σ-τ = 8, σφ = 24
```

### Verification

```
Predicted:  σ · (σ-τ) = 96 minicolumns
Observed:   ~80-120 minicolumns per macrocolumn
            (Mountcastle 1997, Buxhoeveden & Casanova 2002)
            Central estimate: ~80-110
Error:      96 falls within observed range (~0-10%)
```

### Cortical Column Architecture

```
Minicolumn: ~80-120 neurons spanning all 6 cortical layers
Macrocolumn: ~80-100 minicolumns, ~300-600 μm diameter

If 96 minicolumns per macrocolumn:
  96 = σ · (σ-τ) = 12 · 8
  96 = σ² - σ·τ = 144 - 48

Each minicolumn spans P₁ = 6 layers (H-CX-774).
Total neurons per macrocolumn ≈ 96 · 100 ≈ 10000
  ≈ σ² · (σ-τ)² · ... (order of magnitude)
```

### Structural Interpretation

```
σ = 12:   divisor sum — total connectivity capacity
σ-τ = 8:  net degrees of freedom (sum minus count)

Minicolumns per macro = connectivity × degrees_of_freedom
                      = σ · (σ-τ) = 96

The macrocolumn pools σ-τ = 8 independent processing
channels for each of σ = 12 connectivity modes.
```

### Texas Sharpshooter Check

Cortical column counts are notoriously variable across species, brain regions, and measurement methods. The range 80-120 is wide enough that many expressions could fit. The σ·(σ-τ) = 96 expression is clean and falls centrally, but the biological uncertainty is high.

## Verification

- [x] σ·(σ-τ) = 96 within observed range
- [ ] Exact count varies by cortical area
- [ ] Multiple equivalent expressions available

## Status

New. Cortical macrocolumn organization approximates σ·(σ-τ), suggesting n=6 arithmetic constrains neural architecture at the mesoscale.
