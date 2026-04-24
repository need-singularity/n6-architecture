# Verification Code Audit Report (In-Depth)

- Audit date: 2026-04-09
- Target: `docs/paper/n6-*-paper.md` (116 papers total)
- Criteria:
  1. At least one `python` code block
  2. Presence of a "Verification" section heading
  3. At least 3 verification points (assert / append comparisons / dict comparisons)
  4. No runtime errors (isolated exec)
  5. Standard augmentation block present (uniqueness demonstration + prime-bias control)

## Result Summary

| Item | Count |
|------|------|
| Total scanned | 116 |
| PASS (>=3 verify points + runtime OK) | **116** |
| PARTIAL (insufficient verify points) | **0** |
| MISSING (no code) | **0** |
| Runtime errors | **0** |
| Verification section heading present | 116 / 116 |
| Standard augmentation block (uniqueness) present | 64 / 116 |
| Prime-bias control block present | 114 / 116 |

## Full Paper Details

| # | Paper | Blocks | Code Lines | Verify Points | Uniqueness | Control | Status |
|---|------|------|--------|--------|--------|------|------|
| 1 | n6-advanced-packaging-paper | 1 | 57 | 3 | O | O | PASS |
| 2 | n6-aerospace-transport-paper | 1 | 154 | 10 | - | O | PASS |
| 3 | n6-anima-soc-paper | 1 | 60 | 3 | - | O | PASS |
| 4 | n6-antimatter-factory-paper | 1 | 27 | 5 | O | O | PASS |
| 5 | n6-aquaculture-paper | 1 | 34 | 3 | O | O | PASS |
| 6 | n6-ar-vr-xr-paper | 1 | 36 | 3 | O | O | PASS |
| 7 | n6-archaeology-paper | 1 | 39 | 3 | O | O | PASS |
| 8 | n6-autonomous-driving-paper | 1 | 90 | 10 | - | O | PASS |
| 9 | n6-battery-energy-paper | 1 | 75 | 10 | - | O | PASS |
| 10 | n6-biology-medical-paper | 1 | 75 | 10 | - | O | PASS |
| 11 | n6-calendar-time-geography-paper | 2 | 134 | 13 | - | O | PASS |
| 12 | n6-carbon-capture-paper | 1 | 70 | 10 | - | O | PASS |
| 13 | n6-causal-chain-paper | 1 | 163 | 65 | - | O | PASS |
| 14 | n6-classical-mechanics-accelerator-paper | 1 | 70 | 10 | - | O | PASS |
| 15 | n6-cognitive-social-psychology-paper | 1 | 110 | 10 | - | O | PASS |
| 16 | n6-consciousness-chip-paper | 1 | 72 | 3 | - | O | PASS |
| 17 | n6-consciousness-soc-paper | 2 | 118 | 13 | - | O | PASS |
| 18 | n6-construction-structural-paper | 1 | 37 | 3 | O | O | PASS |
| 19 | n6-control-automation-paper | 1 | 67 | 10 | - | O | PASS |
| 20 | n6-crystallography-materials-paper | 1 | 88 | 10 | - | O | PASS |
| 21 | n6-dance-choreography-paper | 1 | 36 | 3 | O | O | PASS |
| 22 | n6-datacenter-reactor-paper | 1 | 28 | 6 | O | O | PASS |
| 23 | n6-desal-paper | 1 | 31 | 7 | O | O | PASS |
| 24 | n6-digital-twin-paper | 1 | 35 | 3 | O | O | PASS |
| 25 | n6-display-8stack-paper | 1 | 58 | 3 | O | O | PASS |
| 26 | n6-dolphin-bioacoustics-paper | 1 | 53 | 14 | O | O | PASS |
| 27 | n6-dram-paper | 1 | 67 | 10 | - | O | PASS |
| 28 | n6-ecology-agriculture-food-paper | 1 | 120 | 10 | - | O | PASS |
| 29 | n6-ecommerce-fintech-paper | 1 | 34 | 3 | O | O | PASS |
| 30 | n6-economics-finance-paper | 1 | 89 | 10 | - | O | PASS |
| 31 | n6-energy-efficiency-paper | 1 | 70 | 10 | - | O | PASS |
| 32 | n6-entomology-paper | 1 | 39 | 14 | O | O | PASS |
| 33 | n6-environment-thermal-paper | 1 | 87 | 10 | - | O | PASS |
| 34 | n6-exynos-paper | 1 | 69 | 10 | - | O | PASS |
| 35 | n6-fashion-textile-paper | 1 | 34 | 3 | O | O | PASS |
| 36 | n6-fermentation-paper | 1 | 36 | 3 | O | O | PASS |
| 37 | n6-fun-car-paper | 2 | 48 | 31 | - | O | PASS |
| 38 | n6-fusion-powerplant-paper | 1 | 34 | 5 | - | O | PASS |
| 39 | n6-games-sports-paper | 1 | 78 | 10 | - | O | PASS |
| 40 | n6-governance-safety-urban-paper | 1 | 69 | 10 | - | O | PASS |
| 41 | n6-hexa-3d-paper | 1 | 70 | 10 | - | O | PASS |
| 42 | n6-hexa-accel-paper | 1 | 59 | 3 | O | O | PASS |
| 43 | n6-hexa-asic-paper | 1 | 54 | 3 | - | O | PASS |
| 44 | n6-hexa-cloak-paper | 1 | 27 | 8 | O | O | PASS |
| 45 | n6-hexa-cosmic-paper | 1 | 26 | 7 | O | O | PASS |
| 46 | n6-hexa-defense-paper | 1 | 28 | 9 | O | O | PASS |
| 47 | n6-hexa-dream-paper | 1 | 38 | 12 | O | O | PASS |
| 48 | n6-hexa-ear-paper | 1 | 24 | 3 | O | O | PASS |
| 49 | n6-hexa-empath-paper | 1 | 27 | 4 | O | O | PASS |
| 50 | n6-hexa-exo-paper | 1 | 38 | 13 | O | O | PASS |
| 51 | n6-hexa-fabric-paper | 1 | 24 | 3 | O | O | PASS |
| 52 | n6-hexa-glass-paper | 1 | 24 | 3 | O | O | PASS |
| 53 | n6-hexa-grav-paper | 1 | 26 | 7 | O | O | PASS |
| 54 | n6-hexa-holo-paper | 1 | 56 | 3 | O | O | PASS |
| 55 | n6-hexa-hover-paper | 1 | 28 | 9 | O | O | PASS |
| 56 | n6-hexa-ios-paper | 1 | 26 | 4 | O | O | PASS |
| 57 | n6-hexa-limb-paper | 1 | 38 | 14 | O | O | PASS |
| 58 | n6-hexa-macos-paper | 1 | 36 | 4 | O | O | PASS |
| 59 | n6-hexa-mind-paper | 1 | 43 | 11 | O | O | PASS |
| 60 | n6-hexa-mram-paper | 1 | 57 | 3 | O | O | PASS |
| 61 | n6-hexa-netproto-paper | 1 | 27 | 4 | O | O | PASS |
| 62 | n6-hexa-neuro-paper | 1 | 61 | 18 | - | O | PASS |
| 63 | n6-hexa-olfact-paper | 1 | 38 | 13 | O | O | PASS |
| 64 | n6-hexa-one-paper | 1 | 47 | 4 | - | O | PASS |
| 65 | n6-hexa-oracle-paper | 1 | 29 | 5 | O | O | PASS |
| 66 | n6-hexa-photon-paper | 1 | 70 | 10 | - | O | PASS |
| 67 | n6-hexa-pim-paper | 1 | 70 | 10 | - | O | PASS |
| 68 | n6-hexa-proglang-paper | 1 | 31 | 4 | O | O | PASS |
| 69 | n6-hexa-recycle-paper | 1 | 26 | 6 | O | O | PASS |
| 70 | n6-hexa-sim-paper | 1 | 39 | 6 | O | O | PASS |
| 71 | n6-hexa-skin-paper | 1 | 38 | 15 | O | O | PASS |
| 72 | n6-hexa-skyway-paper | 1 | 27 | 8 | O | O | PASS |
| 73 | n6-hexa-speak-paper | 1 | 17 | 3 | - | - | PASS |
| 74 | n6-hexa-starship-paper | 1 | 31 | 12 | O | O | PASS |
| 75 | n6-hexa-super-paper | 1 | 70 | 10 | - | O | PASS |
| 76 | n6-hexa-telepathy-paper | 1 | 37 | 12 | O | O | PASS |
| 77 | n6-hexa-teleport-paper | 1 | 29 | 5 | O | O | PASS |
| 78 | n6-hexa-topo-paper | 1 | 60 | 3 | - | O | PASS |
| 79 | n6-hexa-tsunami-paper | 1 | 27 | 8 | O | O | PASS |
| 80 | n6-hexa-ufo-paper | 1 | 28 | 9 | O | O | PASS |
| 81 | n6-hexa-wafer-paper | 1 | 70 | 10 | - | O | PASS |
| 82 | n6-hexa-weather-paper | 1 | 28 | 6 | O | O | PASS |
| 83 | n6-hiv-paper | 1 | 43 | 9 | - | - | PASS |
| 84 | n6-horology-paper | 1 | 41 | 5 | O | O | PASS |
| 85 | n6-insurance-paper | 1 | 35 | 3 | O | O | PASS |
| 86 | n6-isocell-comms-paper | 1 | 70 | 10 | - | O | PASS |
| 87 | n6-jurisprudence-paper | 1 | 39 | 4 | O | O | PASS |
| 88 | n6-manufacturing-quality-paper | 1 | 67 | 10 | - | O | PASS |
| 89 | n6-material-synthesis-paper | 1 | 25 | 5 | O | O | PASS |
| 90 | n6-microplastics-paper | 1 | 27 | 5 | O | O | PASS |
| 91 | n6-monetary-history-paper | 1 | 36 | 3 | O | O | PASS |
| 92 | n6-motorcycle-paper | 1 | 31 | 12 | O | O | PASS |
| 93 | n6-particle-cosmology-paper | 1 | 107 | 10 | - | O | PASS |
| 94 | n6-performance-chip-paper | 1 | 76 | 10 | - | O | PASS |
| 95 | n6-plasma-fusion-deep-paper | 1 | 129 | 10 | - | O | PASS |
| 96 | n6-pure-mathematics-paper | 1 | 70 | 10 | - | O | PASS |
| 97 | n6-quantum-computing-paper | 1 | 83 | 10 | - | O | PASS |
| 98 | n6-reality-map-paper | 1 | 70 | 10 | - | O | PASS |
| 99 | n6-religion-mythology-paper | 1 | 48 | 4 | - | O | PASS |
| 100 | n6-robotics-transport-paper | 1 | 108 | 10 | - | O | PASS |
| 101 | n6-rtsc-12-products-evolution-paper | 1 | 78 | 10 | - | O | PASS |
| 102 | n6-seabed-grid-paper | 1 | 28 | 5 | O | O | PASS |
| 103 | n6-software-crypto-paper | 1 | 98 | 10 | - | O | PASS |
| 104 | n6-space-systems-paper | 1 | 84 | 10 | - | O | PASS |
| 105 | n6-superconductor-paper | 1 | 82 | 10 | - | O | PASS |
| 106 | n6-synthetic-biology-paper | 1 | 41 | 16 | O | O | PASS |
| 107 | n6-telecom-linguistics-paper | 1 | 77 | 10 | - | O | PASS |
| 108 | n6-therapeutic-nanobot-paper | 2 | 54 | 41 | - | O | PASS |
| 109 | n6-thermodynamics-paper | 1 | 69 | 10 | - | O | PASS |
| 110 | n6-ultimate-safety-paper | 1 | 43 | 4 | O | O | PASS |
| 111 | n6-underground-tunnel-paper | 1 | 35 | 3 | O | O | PASS |
| 112 | n6-unified-soc-paper | 1 | 75 | 10 | - | O | PASS |
| 113 | n6-virology-paper | 1 | 125 | 21 | - | O | PASS |
| 114 | n6-vnand-paper | 1 | 70 | 10 | - | O | PASS |
| 115 | n6-wine-enology-paper | 1 | 34 | 3 | O | O | PASS |
| 116 | n6-writing-systems-paper | 1 | 37 | 4 | O | O | PASS |

## Runtime Verification

All 116 papers `exec()`'d in isolated namespaces:
- Runtime errors: **0**
- All asserts pass, all prints output normally

## Verification Code Type Distribution

| Type | Description | Count |
|------|------|------|
| assert-based | `assert expected == computed` | 116 |
| append comparison | `r.append(("label", expected, actual))` + `ok=sum(...)` | 41 |
| dict comparison | `checks={"key": (expected, computed)}` + EXACT tally | 23 |
| Standard augmentation block | uniqueness `_n6==[6]` + prime-bias control | 64 |

## Code Scale Statistics

| Metric | Value |
|------|------|
| Total Python lines | 6,620 |
| Total verify points | 908 |
| Average lines per paper | 57 |
| Average verify points per paper | 7.8 |
| Max lines | 163 (causal-chain) |
| Max verify points | 65 (causal-chain) |
| Min lines | 17 (hexa-speak) |
| Min verify points | 3 (many) |

## Recommended Follow-ups

1. **52 papers missing standard augmentation block** -- adding uniqueness demonstration (`_n6==[6]`) + prime-bias control block will improve consistency
2. **2 papers missing prime-bias control block** (`hexa-speak`, `hiv`) -- adding control block recommended
3. **Code quality** -- manual review needed to check for tautology (value assignment instead of definition derivation)
4. **Execution automation** -- recommend building a CI pipeline that auto-runs every paper's code block

## Notes

- Previous audit (2026-04-09, 115 papers) found "0 missing" -- 1 paper was added afterwards, bringing the total to 116
- Beyond existence, in-depth audit covered verify point counts, runtime execution, and standard augmentation block presence
- Papers without verification code (MISSING) = **0**. All 116 present-state candidates
