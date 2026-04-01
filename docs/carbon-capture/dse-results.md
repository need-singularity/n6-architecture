# HEXA-CCUS DSE Results

> Date: 2026-04-02
> Tool: tools/universal-dse/domains/carbon-capture-8level.toml
> Combos: 1,360,800 valid (from 6^8 = 1,679,616 theoretical)
> Pareto solutions: 54

## Pareto Frontier (Top 10)

| Rank | Sorbent | Process | Reactor | Chip | Plant | Transmute | Universal | Omega | n6% | Score |
|------|---------|---------|---------|------|-------|-----------|-----------|-------|-----|-------|
| 1 | Zeolite-6A | MECS | Honeycomb | Analog ASIC | CCS Hub | Graphene | Crustal | Maxwell | 100 | 0.778 |
| 2 | MOF-74 | MECS | Honeycomb | Analog ASIC | CCS Hub | Graphene | Crustal | Maxwell | 100 | 0.776 |
| 3 | Zeolite-6A | TSA | Honeycomb | RISC-V | CCS Hub | Graphene | Crustal | Maxwell | 100 | 0.774 |
| 4 | MOF-74 | TSA | Rotating | Edge AI | DAC Farm | Graphene | Integrated | Dyson | 100 | 0.772 |
| 5 | Graphene-Ox | MECS | Microreactor | Quantum | CCS Hub | CNT | Crustal | Spacetime | 100 | 0.770 |
| ... | ... | ... | ... | ... | ... | ... | ... | ... | ... | ... |

## Pareto Frontier Visualization

### n6 Score vs Overall Performance
```
n6 (%)
100 ┤●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●●  (54 solutions)
    │
 98 ┤  ○○○○○○○○○                             (dominated)
    │
 96 ┤     ○○○○○○○○○○○○○
    │
 94 ┤        ○○○○○○○○○○○○○○○
    │
 92 ┤           ○○○○○○○○○○○○
    │
 90 ┤              ○○○○○○
    ├────┬────┬────┬────┬────┬────┬────┬──── Score
    0.60 0.65 0.70 0.75 0.78 0.80 0.85

● = Pareto optimal (non-dominated)
○ = Dominated solution
All 54 Pareto solutions achieve n6=100%
```

### Cost vs Energy Trade-off
```
Cost (low=good)
1.0 ┤○
    │ ○
0.8 ┤  ○○
    │    ○○
0.6 ┤      ●●●  ← Pareto front
    │        ●●●
0.4 ┤          ●●●
    │            ●●●
0.2 ┤              ●●
    │                ●
0.0 ┤─────────────────●──
    ├──┬──┬──┬──┬──┬──┬── Energy (low=good)
    0.0 0.2 0.4 0.6 0.8 1.0

Sweet spot: Cost=0.4, Energy=0.5 (Rank 1 solution)
```

## Sensitivity Analysis

### Which level matters most?

Test: Fix 7 levels at optimal, vary 1 level through all 6 candidates.

```
Score sensitivity (Δ from optimal when varying each level):

Level 0 (Sorbent):    ████████░░  Δmax = 0.08  (moderate)
Level 1 (Process):    ██████████  Δmax = 0.12  (HIGH — process choice matters most)
Level 2 (Reactor):    ██████░░░░  Δmax = 0.06  (moderate)
Level 3 (Chip):       ████░░░░░░  Δmax = 0.04  (low)
Level 4 (Plant):      ██████░░░░  Δmax = 0.06  (moderate)
Level 5 (Transmute):  ████████░░  Δmax = 0.09  (moderate-high)
Level 6 (Universal):  ██░░░░░░░░  Δmax = 0.02  (low — all good)
Level 7 (Omega):      █░░░░░░░░░  Δmax = 0.01  (negligible — all n6=100%)

→ Process (L1) is the bottleneck. Get that right and everything else follows.
→ L6-L7 barely matter (all candidates are n6=100% by construction)
```

### n=6 EXACT ratio by level

```
Level 0 (Sorbent):    ██████████████████  6/6 = 100% (all CN=6 or C6)
Level 1 (Process):    █████████████████░  5/6 = 83%  (Photocatalytic is WEAK)
Level 2 (Reactor):    ██████████████████  6/6 = 100% (all hexagonal/6-unit)
Level 3 (Chip):       ██████████████████  6/6 = 100% (all 6-sensor/6-layer)
Level 4 (Plant):      █████████████░░░░  4/6 = 67%  (Ocean/Mobile weaker)
Level 5 (Transmute):  ██████████████████  6/6 = 100% (all C6-based)
Level 6 (Universal):  ██████████████████  6/6 = 100% (all 6-zone)
Level 7 (Omega):      █████████████████░  5/6 = 83%  (BH Penrose forced)

Overall: 44/48 = 91.7% candidates have EXACT n=6 connection
```

## Cross-DSE Summary

| Partner | Score | n6% | Bridge |
|---------|-------|-----|--------|
| MOF | 0.859 | 100 | Zr6 cluster = ideal CO2 sorbent |
| Solar | 0.856 | 100 | 6-junction tandem powers DAC |
| Concrete | 0.856 | 100 | CO2 mineralization |
| Graphene | 0.856 | 96 | CO2→C6 graphene |
| Fusion | 0.854 | 100 | Fusion energy drives CCUS |
| Material | 0.852 | 100 | CO2 as C Z=6 feedstock |
| Wind | 0.850 | 100 | 72MW wind + DAC |
| Climate | 0.844 | 100 | Model validates impact |
| H2-FC | 0.839 | 100 | H2 co-electrolysis |
| Ocean | 0.835 | 100 | AUV monitors CO2 sink |
| Battery | 0.828 | 100 | LFP CN=6 powers DAC |
| Chip | — | — | (previous session) |

→ MOF is the natural partner (shared CN=6 chemistry)
→ 10/11 new pairs achieve n6=100%
