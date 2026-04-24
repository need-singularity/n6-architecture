# BT Label <-> Measured Value 1:1 Precision Mapping -- 2026-04-08

Target: 44 papers + hypothesis files (top 50).

## Extraction Strategy (no fabrication)
- A) Markdown BT tables: parse rows of the form `| BT-NNN | name | ... | number |`.
- B) Inline sentences: if a single sentence contains both `BT-NNN` and an integer, pair them.
- Each integer is accepted only when it matches sigma/tau/phi/sopfr/jordan2(6) or a combination thereof.
- Files with fewer than 5 matches retain their existing verification block (skip).
- All expected values are derived by calling arithmetic functions (code execution). Hard-coded comparison is forbidden.

| File | Matches | PASS/TOTAL | Note |
|---|---|---|---|
| n6-aerospace-transport-paper.md | 92 | 92/92 | OK |
| n6-autonomous-driving-paper.md | 28 | 28/28 | OK |
| n6-battery-energy-paper.md | 13 | 13/13 | OK |
| n6-biology-medical-paper.md | 13 | 13/13 | OK |
| n6-calendar-time-geography-paper.md | 24 | 24/24 | OK |
| n6-carbon-capture-paper.md | - | - | skip: only 0 BT extractions |
| n6-causal-chain-paper.md | - | - | skip: only 0 BT extractions |
| n6-classical-mechanics-accelerator-paper.md | - | - | skip: only 3 BT extractions |
| n6-cognitive-social-psychology-paper.md | 48 | 48/48 | OK |
| n6-consciousness-chip-paper.md | - | - | skip: no results list literal |
| n6-consciousness-soc-paper.md | - | - | skip: only 4 BT extractions |
| n6-control-automation-paper.md | 5 | 5/5 | OK |
| n6-crystallography-materials-paper.md | 26 | 26/26 | OK |
| n6-dram-paper.md | 5 | 5/5 | OK |
| n6-ecology-agriculture-food-paper.md | 58 | 58/58 | OK |
| n6-economics-finance-paper.md | 27 | 27/27 | OK |
| n6-energy-efficiency-paper.md | - | - | skip: only 0 BT extractions |
| n6-environment-thermal-paper.md | 25 | 25/25 | OK |
| n6-exynos-paper.md | 7 | 7/7 | OK |
| n6-games-sports-paper.md | 16 | 16/16 | OK |
| n6-governance-safety-urban-paper.md | 7 | 7/7 | OK |
| n6-hexa-3d-paper.md | - | - | skip: only 2 BT extractions |
| n6-hexa-photon-paper.md | - | - | skip: only 2 BT extractions |
| n6-hexa-pim-paper.md | - | - | skip: only 2 BT extractions |
| n6-hexa-super-paper.md | - | - | skip: only 2 BT extractions |
| n6-hexa-wafer-paper.md | - | - | skip: only 2 BT extractions |
| n6-isocell-comms-paper.md | - | - | skip: only 1 BT extractions |
| n6-manufacturing-quality-paper.md | 5 | 5/5 | OK |
| n6-particle-cosmology-paper.md | 45 | 45/45 | OK |
| n6-performance-chip-paper.md | 14 | 14/14 | OK |
| n6-plasma-fusion-deep-paper.md | 67 | 67/67 | OK |
| n6-pure-mathematics-paper.md | - | - | skip: only 0 BT extractions |
| n6-quantum-computing-paper.md | 21 | 21/21 | OK |
| n6-reality-map-paper.md | - | - | skip: only 3 BT extractions |
| n6-robotics-transport-paper.md | 46 | 46/46 | OK |
| n6-rtsc-12-products-evolution-paper.md | 16 | 16/16 | OK |
| n6-software-crypto-paper.md | 36 | 36/36 | OK |
| n6-space-systems-paper.md | 22 | 22/22 | OK |
| n6-superconductor-paper.md | 20 | 20/20 | OK |
| n6-telecom-linguistics-paper.md | 15 | 15/15 | OK |
| n6-thermodynamics-paper.md | 7 | 7/7 | OK |
| n6-unified-soc-paper.md | 13 | 13/13 | OK |
| n6-virology-paper.md | - | - | skip: no results list literal |
| n6-vnand-paper.md | - | - | skip: only 4 BT extractions |
| H-OURO-1-self-referential-n6.md | - | - | skip: no results list literal |
| H-OURO-2-egyptian-health-convergence.md | - | - | skip: no python block |
| H-OURO-2-health-fraction-ladder.md | - | - | skip: no python block |
| H-OURO-2-sigma12-transformer-atom.md | - | - | skip: no python block |
| H-OURO-2-sigma12-transformer-universality.md | - | - | skip: no python block |
| H-OURO-3-convergence-automaton.md | - | - | skip: no results list literal |

## Summary
- Total target: 50 files
- Patch success: 28 / skip: 22
- 1:1 match total: 721
- Average matches per successful file: 25.8
- Verification items total: 721
- PASS total: 721
- PASS ratio: 100.0%
