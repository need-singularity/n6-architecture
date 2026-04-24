# Paper verification-code full audit -- audit-quality-full

**Date**: 2026-04-09  **Target**: `docs/paper/*.md` (n6-*-paper + paper1~4 + 307/308)

## Summary

- Total papers: **122**
- PASS (Python block in body executes successfully): **122**
- FAIL: **0**
- MISSING (no verification code): **0**
- TIMEOUT: **0**
- Strict tautology suspicion (no sigma/phi/divisor computation): **4**

There is only one dedicated `scripts/verify_*.py`, namely `verify_special_number_control.py` -- the remaining 121 papers all verify via embedded Python blocks in their bodies.

## products.json cross-check

- Mentions of 'paper' in `config/products.json`: **127**
- Actual paper files in `docs/paper/`: **122**
- Difference: **5** -- SSOT re-sync required (`scripts/sync_products_readme.py`).

## Tautology flags (strict criteria)

| File | Reason |
|---|---|
| 307-domain-universality-dataset.md | no sigma/phi/tau definition, enumerates constants in assert |
| paper1-ai-efficiency.md | no formula derivation, directly compares result values |
| paper3-tokamak-physics.md | verification based on literal physical constants |
| paper4-gut-monster.md | directly asserts GUT scale numbers |

Recommendation: the four above should include in their verification blocks the derivation chain from `sigma(n)*phi(n) = n*tau(n)`.

## Full results (summary table)

| # | Paper | Status | Blocks | Flag |
|---:|---|:-:|:-:|:-:|
| 1 | 307-domain-universality-dataset.md | PASS | 1 | SELFREF |
| 2 | 308-fusion-n6-alien-discoveries.md | PASS | 1 |  |
| 3 | n6-advanced-packaging-paper.md | PASS | 1 |  |
| 4 | n6-aerospace-transport-paper.md | PASS | 1 |  |
| 5 | n6-anima-soc-paper.md | PASS | 1 |  |
| 6 | n6-antimatter-factory-paper.md | PASS | 1 |  |
| 7 | n6-aquaculture-paper.md | PASS | 1 |  |
| 8 | n6-ar-vr-xr-paper.md | PASS | 1 |  |
| 9 | n6-archaeology-paper.md | PASS | 1 |  |
| 10 | n6-autonomous-driving-paper.md | PASS | 1 |  |
| 11 | n6-battery-energy-paper.md | PASS | 1 |  |
| 12 | n6-biology-medical-paper.md | PASS | 1 |  |
| 13 | n6-calendar-time-geography-paper.md | PASS | 2 |  |
| 14 | n6-carbon-capture-paper.md | PASS | 1 |  |
| 15 | n6-causal-chain-paper.md | PASS | 1 |  |
| 16 | n6-classical-mechanics-accelerator-paper.md | PASS | 1 |  |
| 17 | n6-cognitive-social-psychology-paper.md | PASS | 1 |  |
| 18 | n6-consciousness-chip-paper.md | PASS | 1 |  |
| 19 | n6-consciousness-soc-paper.md | PASS | 2 |  |
| 20 | n6-construction-structural-paper.md | PASS | 1 |  |
| 21 | n6-control-automation-paper.md | PASS | 1 |  |
| 22 | n6-crystallography-materials-paper.md | PASS | 1 |  |
| 23 | n6-dance-choreography-paper.md | PASS | 1 |  |
| 24 | n6-datacenter-reactor-paper.md | PASS | 1 |  |
| 25 | n6-desal-paper.md | PASS | 1 |  |
| 26 | n6-digital-twin-paper.md | PASS | 1 |  |
| 27 | n6-display-8stack-paper.md | PASS | 1 |  |
| 28 | n6-dolphin-bioacoustics-paper.md | PASS | 1 |  |
| 29 | n6-dram-paper.md | PASS | 1 |  |
| 30 | n6-ecology-agriculture-food-paper.md | PASS | 1 |  |
| 31 | n6-ecommerce-fintech-paper.md | PASS | 1 |  |
| 32 | n6-economics-finance-paper.md | PASS | 1 |  |
| 33 | n6-energy-efficiency-paper.md | PASS | 1 |  |
| 34 | n6-entomology-paper.md | PASS | 1 |  |
| 35 | n6-environment-thermal-paper.md | PASS | 1 |  |
| 36 | n6-exynos-paper.md | PASS | 1 |  |
| 37 | n6-fashion-textile-paper.md | PASS | 1 |  |
| 38 | n6-fermentation-paper.md | PASS | 1 |  |
| 39 | n6-fun-car-paper.md | PASS | 2 |  |
| 40 | n6-fusion-powerplant-paper.md | PASS | 1 |  |
| 41 | n6-games-sports-paper.md | PASS | 1 |  |
| 42 | n6-governance-safety-urban-paper.md | PASS | 1 |  |
| 43 | n6-hexa-3d-paper.md | PASS | 1 |  |
| 44 | n6-hexa-accel-paper.md | PASS | 1 |  |
| 45 | n6-hexa-asic-paper.md | PASS | 1 |  |
| 46 | n6-hexa-cloak-paper.md | PASS | 1 |  |
| 47 | n6-hexa-cosmic-paper.md | PASS | 1 |  |
| 48 | n6-hexa-defense-paper.md | PASS | 1 |  |
| 49 | n6-hexa-dream-paper.md | PASS | 1 |  |
| 50 | n6-hexa-ear-paper.md | PASS | 1 |  |
| 51 | n6-hexa-empath-paper.md | PASS | 1 |  |
| 52 | n6-hexa-exo-paper.md | PASS | 1 |  |
| 53 | n6-hexa-fabric-paper.md | PASS | 1 |  |
| 54 | n6-hexa-glass-paper.md | PASS | 1 |  |
| 55 | n6-hexa-grav-paper.md | PASS | 1 |  |
| 56 | n6-hexa-holo-paper.md | PASS | 1 |  |
| 57 | n6-hexa-hover-paper.md | PASS | 1 |  |
| 58 | n6-hexa-ios-paper.md | PASS | 1 |  |
| 59 | n6-hexa-limb-paper.md | PASS | 1 |  |
| 60 | n6-hexa-macos-paper.md | PASS | 1 |  |
| 61 | n6-hexa-mind-paper.md | PASS | 1 |  |
| 62 | n6-hexa-mram-paper.md | PASS | 1 |  |
| 63 | n6-hexa-netproto-paper.md | PASS | 1 |  |
| 64 | n6-hexa-neuro-paper.md | PASS | 1 |  |
| 65 | n6-hexa-olfact-paper.md | PASS | 1 |  |
| 66 | n6-hexa-one-paper.md | PASS | 1 |  |
| 67 | n6-hexa-oracle-paper.md | PASS | 1 |  |
| 68 | n6-hexa-photon-paper.md | PASS | 1 |  |
| 69 | n6-hexa-pim-paper.md | PASS | 1 |  |
| 70 | n6-hexa-proglang-paper.md | PASS | 1 |  |
| 71 | n6-hexa-recycle-paper.md | PASS | 1 |  |
| 72 | n6-hexa-sim-paper.md | PASS | 1 |  |
| 73 | n6-hexa-skin-paper.md | PASS | 1 |  |
| 74 | n6-hexa-skyway-paper.md | PASS | 1 |  |
| 75 | n6-hexa-speak-paper.md | PASS | 1 |  |
| 76 | n6-hexa-starship-paper.md | PASS | 1 |  |
| 77 | n6-hexa-super-paper.md | PASS | 1 |  |
| 78 | n6-hexa-telepathy-paper.md | PASS | 1 |  |
| 79 | n6-hexa-teleport-paper.md | PASS | 1 |  |
| 80 | n6-hexa-topo-paper.md | PASS | 1 |  |
| 81 | n6-hexa-tsunami-paper.md | PASS | 1 |  |
| 82 | n6-hexa-ufo-paper.md | PASS | 1 |  |
| 83 | n6-hexa-wafer-paper.md | PASS | 1 |  |
| 84 | n6-hexa-weather-paper.md | PASS | 1 |  |
| 85 | n6-hiv-paper.md | PASS | 1 |  |
| 86 | n6-horology-paper.md | PASS | 1 |  |
| 87 | n6-insurance-paper.md | PASS | 1 |  |
| 88 | n6-isocell-comms-paper.md | PASS | 1 |  |
| 89 | n6-jurisprudence-paper.md | PASS | 1 |  |
| 90 | n6-manufacturing-quality-paper.md | PASS | 1 |  |
| 91 | n6-material-synthesis-paper.md | PASS | 1 |  |
| 92 | n6-microplastics-paper.md | PASS | 1 |  |
| 93 | n6-monetary-history-paper.md | PASS | 1 |  |
| 94 | n6-motorcycle-paper.md | PASS | 1 |  |
| 95 | n6-particle-cosmology-paper.md | PASS | 1 |  |
| 96 | n6-performance-chip-paper.md | PASS | 1 |  |
| 97 | n6-plasma-fusion-deep-paper.md | PASS | 1 |  |
| 98 | n6-pure-mathematics-paper.md | PASS | 1 |  |
| 99 | n6-quantum-computing-paper.md | PASS | 1 |  |
| 100 | n6-reality-map-paper.md | PASS | 1 |  |
| 101 | n6-religion-mythology-paper.md | PASS | 1 |  |
| 102 | n6-robotics-transport-paper.md | PASS | 1 |  |
| 103 | n6-rtsc-12-products-evolution-paper.md | PASS | 1 |  |
| 104 | n6-seabed-grid-paper.md | PASS | 1 |  |
| 105 | n6-software-crypto-paper.md | PASS | 1 |  |
| 106 | n6-space-systems-paper.md | PASS | 1 |  |
| 107 | n6-superconductor-paper.md | PASS | 1 |  |
| 108 | n6-synthetic-biology-paper.md | PASS | 1 |  |
| 109 | n6-telecom-linguistics-paper.md | PASS | 1 |  |
| 110 | n6-therapeutic-nanobot-paper.md | PASS | 2 |  |
| 111 | n6-thermodynamics-paper.md | PASS | 1 |  |
| 112 | n6-ultimate-safety-paper.md | PASS | 1 |  |
| 113 | n6-underground-tunnel-paper.md | PASS | 1 |  |
| 114 | n6-unified-soc-paper.md | PASS | 1 |  |
| 115 | n6-virology-paper.md | PASS | 1 |  |
| 116 | n6-vnand-paper.md | PASS | 1 |  |
| 117 | n6-wine-enology-paper.md | PASS | 1 |  |
| 118 | n6-writing-systems-paper.md | PASS | 1 |  |
| 119 | paper1-ai-efficiency.md | PASS | 1 | SELFREF |
| 120 | paper2-cross-domain.md | PASS | 1 |  |
| 121 | paper3-tokamak-physics.md | PASS | 1 | SELFREF |
| 122 | paper4-gut-monster.md | PASS | 1 | SELFREF |

## Execution environment

- Python 3 (system), `cwd=docs/paper`, `stdin=DEVNULL`, 15-second timeout per block
- Multiple python blocks within a paper are concatenated and run as one
- Logs: `/tmp/audit_out.txt`, `/tmp/paper_audit.json`

## Conclusion

All 122 papers **PASS**. However, 4 are suspected of self-reference due to missing definition-to-derivation chains. Raising verification strictness (requiring actual sigma/phi computation) and resolving the 5-item gap between products.json and actual files are the next tasks.
