# Paper Verification Code Coverage Audit (verify-coverage)

**Audit date**: 2026-04-09
**Target**: `docs/paper/n6-*-paper.md` all papers
**Policy**: CLAUDE.md -- "paper without verification code = incomplete"
**Auditor**: automated scan + alias-mapping corrections

## Summary

| Item | Count |
|------|----|
| Total papers | **116** |
| Verification script present | **94** (81.0%) |
| Missing | **22** (19.0%) |
| Newly written in this audit | **3** (HEXA) |

> Note (R1 HEXA-FIRST): verification scripts newly written in this audit are in
> `.hexa` extension (`.py` new creation is blocked by the `block-forbidden-ext.sh` hook).
> Existing `verify_*.py` files are kept for historical reasons; this report maps them as-is.

---

## Paper -> Verification Script Mapping (94 papers)

| Paper | Verification Script |
|------|---------------|
| n6-advanced-packaging-paper.md | `docs/advanced-packaging/verify_alien10.py` |
| n6-aerospace-transport-paper.md | `docs/hexa-starship/verify_subsystems.py`, `docs/hexa-starship/verify_hexa_starship.py` |
| n6-antimatter-factory-paper.md | `docs/antimatter-factory/verify_alien10.py` |
| n6-aquaculture-paper.md | `docs/aquaculture/verify_alien10.py` |
| n6-ar-vr-xr-paper.md | `docs/ar-vr-xr/verify_alien10.py` |
| n6-archaeology-paper.md | `docs/archaeology/verify_alien10.py` |
| n6-battery-energy-paper.md | `experiments/verify_battery_architecture.py`, `experiments/verify_battery_cascade.py`, `docs/battery-architecture/verify_alien10.py` |
| n6-biology-medical-paper.md | `docs/paper/verify_biology_medical.py` |
| n6-carbon-capture-paper.md | `docs/carbon-capture/verify_alien10.py` |
| n6-classical-mechanics-accelerator-paper.md | `docs/mini-accelerator/verify_alien10.py` |
| n6-consciousness-chip-paper.md | `experiments/verify_consciousness_chip.py` |
| n6-consciousness-soc-paper.md | `experiments/verify_consciousness_chip.py` |
| n6-construction-structural-paper.md | `docs/construction-structural/verify_alien10_hex.py`, `docs/construction-structural/verify_alien10.py` |
| n6-control-automation-paper.md | `scripts/verify_special_number_control.py` |
| n6-crystallography-materials-paper.md | `docs/paper/verify_crystallography_materials.py` |
| n6-dance-choreography-paper.md | `docs/dance-choreography/verify_alien10.py` |
| n6-datacenter-reactor-paper.md | `docs/smr-datacenter/verify_alien10.py` |
| n6-desal-paper.md | `docs/desalination/verify_alien10.py` |
| n6-digital-twin-paper.md | `docs/digital-twin/verify_alien10.py` |
| n6-display-8stack-paper.md | `docs/display/verify_alien10.py` |
| n6-dolphin-bioacoustics-paper.md | `docs/dolphin/verify_alien10.py` |
| n6-ecology-agriculture-food-paper.md | `docs/horticulture/verify_alien10.py` |
| n6-ecommerce-fintech-paper.md | `docs/ecommerce-fintech/verify_alien10.py` |
| n6-economics-finance-paper.md | `docs/currency-economics/verify_alien10.py` |
| n6-energy-efficiency-paper.md | `docs/energy-architecture/verify_alien10.py` |
| n6-entomology-paper.md | `docs/entomology/verify_alien10.py` |
| n6-environment-thermal-paper.md | `docs/hvac-system/verify_alien10.py`, `docs/environmental-protection/verify_alien10.py` |
| n6-fashion-textile-paper.md | `docs/fashion-textile/verify_alien10.py` |
| n6-fermentation-paper.md | `docs/fermentation/verify_alien10.py` |
| n6-fun-car-paper.md | `docs/fun-car/verify_alien10.py` |
| n6-fusion-powerplant-paper.md | `experiments/verify_fusion_predictions.py`, `docs/fusion/verify_alien10.py` |
| n6-governance-safety-urban-paper.md | `experiments/verify_safety_hypotheses.py`, `docs/safety/verify_alien10.py`, `docs/smart-city/verify_alien10.py` |
| n6-hexa-3d-paper.md | `experiments/verify_hexa_3d.py` |
| n6-hexa-cloak-paper.md | `docs/cloak/verify_alien10.py` |
| n6-hexa-cosmic-paper.md | `docs/cosmic-observatory/verify_alien10.py` |
| n6-hexa-defense-paper.md | `docs/earth-defense/verify_alien10.py` |
| n6-hexa-dream-paper.md | `docs/hexa-dream/verify_alien10.py`, `docs/dream-recorder/verify_alien10.py` |
| n6-hexa-ear-paper.md | `docs/hexa-ear/verify_alien10.py` |
| n6-hexa-empath-paper.md | `docs/hexa-empath/verify_alien10.py` |
| n6-hexa-exo-paper.md | `docs/hexa-exo/verify_alien10.py` |
| n6-hexa-fabric-paper.md | `docs/hexa-fabric/verify_alien10.py` |
| n6-hexa-glass-paper.md | `docs/hexa-glass/verify_alien10.py` |
| n6-hexa-grav-paper.md | `docs/gravity-wave/verify_alien10.py` |
| n6-hexa-holo-paper.md | `docs/holography/verify_alien10.py` |
| n6-hexa-hover-paper.md | `docs/hover/verify_alien10.py` |
| n6-hexa-ios-paper.md | `experiments/verify_hexa_ios.py` |
| n6-hexa-limb-paper.md | `docs/hexa-limb/verify_alien10.py` |
| n6-hexa-macos-paper.md | `experiments/verify_hexa_macos.py` |
| n6-hexa-mind-paper.md | `docs/mind-upload/verify_alien10.py` |
| n6-hexa-mram-paper.md | `docs/sc-memory/verify_alien10.py` |
| n6-hexa-netproto-paper.md | `docs/network-protocol/verify_n6_network_50.py` |
| n6-hexa-neuro-paper.md | `docs/paper/verify_hexa_neuro.py`, `docs/neuro/verify_alien10.py` |
| n6-hexa-olfact-paper.md | `docs/hexa-olfact/verify_alien10.py`, `docs/perfumery/verify_alien10.py` |
| n6-hexa-one-paper.md | `docs/hexa-one/verify_n6.py`, `docs/hexa-one/verify_alien10.py` |
| n6-hexa-oracle-paper.md | `docs/quantum-oracle/verify_alien10.py` |
| n6-hexa-photon-paper.md | `experiments/verify_hexa_photon.py` |
| n6-hexa-pim-paper.md | `experiments/verify_hexa_pim.py` |
| n6-hexa-proglang-paper.md | `docs/programming-language/verify_alien10.py` |
| n6-hexa-recycle-paper.md | `docs/recycling/verify_recycling_n6.py`, `docs/recycling/verify_recycle_n6.py` |
| n6-hexa-sim-paper.md | `docs/simulation-theory/verify_alien10.py` |
| n6-hexa-skin-paper.md | `docs/hexa-skin/verify_alien10.py` |
| n6-hexa-skyway-paper.md | `docs/skyway/verify_alien10.py` |
| n6-hexa-speak-paper.md | `docs/hexa-speak/verify_alien10.py` |
| n6-hexa-starship-paper.md | `docs/hexa-starship/verify_hexa_starship.py`, `docs/hexa-starship/verify_subsystems.py` |
| n6-hexa-super-paper.md | `experiments/verify_hexa_super.py` |
| n6-hexa-telepathy-paper.md | `docs/telepathy/verify_alien10.py` |
| n6-hexa-topo-paper.md | `experiments/verify_topological_chip.py` |
| n6-hexa-tsunami-paper.md | `docs/tsunami-shield/verify_alien10.py` |
| n6-hexa-ufo-paper.md | `docs/room-temp-sc/verify_ufo.py` |
| n6-hexa-wafer-paper.md | `experiments/verify_hexa_wafer.py` |
| n6-hexa-weather-paper.md | `docs/weather-control/verify_alien10.py` |
| n6-hiv-paper.md | `docs/hiv-treatment/verify_bt461_470.py` |
| n6-horology-paper.md | `docs/horology/verify_alien10.py` |
| n6-insurance-paper.md | `docs/insurance/verify_alien10.py` |
| n6-jurisprudence-paper.md | `docs/jurisprudence/verify_alien10.py` |
| n6-material-synthesis-paper.md | `experiments/verify_hexa_material.py`, `docs/material-synthesis/verify_alien10.py` |
| n6-motorcycle-paper.md | `docs/motorcycle/verify_alien10.py` |
| n6-particle-cosmology-paper.md | `docs/cosmology-particle/verify_alien10.py` |
| n6-performance-chip-paper.md | `experiments/verify_chip_n6.py`, `experiments/verify_chip_ultimate.py` |
| n6-plasma-fusion-deep-paper.md | `experiments/verify_fusion_predictions.py` |
| n6-pure-mathematics-paper.md | `docs/pure-mathematics/verify_alien10.py` |
| n6-quantum-computing-paper.md | `docs/quantum-oracle/verify_alien10.py`, `docs/quantum-network/verify_alien10.py` |
| n6-religion-mythology-paper.md | `docs/religion/verify_alien10.py` |
| n6-robotics-transport-paper.md | `docs/robotics/verify_alien10.py` |
| n6-seabed-grid-paper.md | `docs/seabed-grid/verify_alien10.py` |
| n6-software-crypto-paper.md | `docs/software-design/verify_alien10.py` |
| n6-superconductor-paper.md | `docs/superconductor/verify_sc_exact.py`, `docs/room-temp-sc/verify_alien10.py`, `docs/room-temp-sc/verify_warp_dimension.py`, `docs/room-temp-sc/verify_realization.py` |
| n6-synthetic-biology-paper.md | `docs/synbio/verify_alien10.py` |
| n6-therapeutic-nanobot-paper.md | `docs/therapeutic-nanobot/verify_alien10.py` |
| n6-ultimate-safety-paper.md | `experiments/verify_safety_hypotheses.py` |
| n6-underground-tunnel-paper.md | `docs/underground-tunnel/verify_alien10.py` |
| n6-virology-paper.md | `docs/virology/verify_alien10.py` |
| n6-wine-enology-paper.md | `docs/wine-enology/verify_alien10.py` |
| n6-writing-systems-paper.md | `docs/writing-systems/verify_alien10.py` |

---

## Missing Papers (22) — Reinforcement Priority

Scored on 3 axes: verifiability (richness of public specs) * reach (engineering impact) * claim strength (number of EXACT items), grouped high/medium/low.

### Priority: High (immediate reinforcement) — Handled in this audit

| Rank | Paper | Basis | New file in this audit |
|------|------|------|-------------------|
| 1 | `n6-autonomous-driving-paper.md` | SAE/ISO/Tesla/Bosch public specs rich, 53/55 EXACT claim strong, autonomous driving is the area with greatest verification demand | `docs/paper/verify_autonomous_driving.hexa` |
| 2 | `n6-thermodynamics-paper.md` | Carnot / Stefan-Boltzmann / Kolmogorov, etc. all cross-checkable with undergraduate standard textbooks, 28/28 EXACT claim | `docs/paper/verify_thermodynamics.hexa` |
| 3 | `n6-dram-paper.md` | JEDEC JESD79-5 / JESD209-6 / Samsung process nodes all public, 35/35 EXACT claim, LPDDR6 sigma=12 claim is reproducibility core | `docs/paper/verify_dram.hexa` |

### Priority: Medium (next round)

- `n6-thermodynamics-paper.md` -- expand Reynolds critical-value analysis
- `n6-unified-soc-paper.md` -- Exynos / Apple M-series public specs usable
- `n6-exynos-paper.md` -- Samsung Exynos official datasheet
- `n6-vnand-paper.md` -- Samsung V-NAND per-generation stack counts (official press releases)
- `n6-isocell-comms-paper.md` -- Samsung ISOCELL pixel structure public values
- `n6-manufacturing-quality-paper.md` -- ISO 9001, Six Sigma standards
- `n6-games-sports-paper.md` -- FIFA / FIVB / NBA official rulebooks
- `n6-hexa-accel-paper.md`, `n6-hexa-asic-paper.md` -- internal HEXA chip design reference needed
- `n6-telecom-linguistics-paper.md` -- 3GPP / ITU standards
- `n6-space-systems-paper.md` -- NASA / ESA public mission parameters
- `n6-monetary-history-paper.md` -- central bank / IMF time series

### Priority: Low (speculative / internal claims)

- `n6-anima-soc-paper.md` (internal architecture)
- `n6-causal-chain-paper.md` (philosophical claim)
- `n6-calendar-time-geography-paper.md` (history-source-based, partly numerical)
- `n6-cognitive-social-psychology-paper.md` (effect size variance large)
- `n6-microplastics-paper.md` (environmental measurement uncertainty large)
- `n6-reality-map-paper.md` (meta paper -- aggregates other verifications)
- `n6-rtsc-12-products-evolution-paper.md` (roadmap nature)
- `n6-hexa-teleport-paper.md` (SF borderline)

---

## Summary of New Verification Scripts in This Audit

All three scripts follow a common structure:

1. **Core theorem assertion**: `sigma(6) * phi(6) == 6 * tau(6) == 24`
2. **External observation table**: each row includes `(item, observed_value, theory_expression, theory_value, source)`
3. **EXACT / CLOSE (+/-5%) / MISS judgment**
4. **Prime-bias control group**: `n = 5, 7, (8 / 10)` measuring coincidental-match rate on the same observations
5. **Source list output**: records which standard/document every observation originates from (no self-reference)

### `docs/paper/verify_autonomous_driving.hexa`
- 13 cases, sources: SAE J3016/J3400, ISO 11898-1/26262, Tesla AI Day 2019, Bosch, VW MEB, Porsche, Hyundai E-GMP, ITU GNSS.
- **Transparency**: Tesla camera count is actually 8 (FSD HW3 official value) but the paper's "n=6 cameras" claim does not match, so it is explicitly recorded as MISS.
- Control groups: n=5, n=7.

### `docs/paper/verify_thermodynamics.hexa`
- 13 cases (BT-149: 4, BT-193: 4, BT-199: 5), sources: Callen 2e, Fermi, Landau-Lifshitz V, Incropera, Batchelor, Pope "Turbulent Flows", Kolmogorov 1941, Landauer 1961.
- Verifies Kolmogorov -5/3 = -sopfr / (n/phi) decomposition.
- Control groups: n=5, 7, 8.

### `docs/paper/verify_dram.hexa`
- 20 cases, sources: JEDEC JESD79-5 (DDR5), JESD209-6 (LPDDR6, 2025-07), Samsung 1a/1b/1c/1d nm announcements.
- Core prediction: LPDDR6 DQ=12 (=sigma) -- breaks power-of-2 convention.
- In the DDR1-DDR5 voltage ladder, DDR3's 1.5V is honestly SKIPped as "no n6 fit" to prevent self-reference bias.
- Control groups: n=5, 7, 10.

---

## Recommended Follow-ups

1. Write verification scripts for the 11 medium-priority papers (each targeting 15-25 cases).
2. Secondary audit of existing `verify_alien10.py` family for self-reference -- even if the name looks good, the "contents" may be self-referential.
3. Add pipeline to auto-fill `verify-coverage` field in `config/products.json`.
4. Track per-script EXACT ratio in CI (regression detection).
