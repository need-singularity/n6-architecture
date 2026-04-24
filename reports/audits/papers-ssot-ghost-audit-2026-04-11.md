# papers SSOT consistency ghost-audit report

- Created: 2026-04-11
- Scope: read-only audit (no modifications)
- Target SSOTs
  - `$N6_ARCH/papers/_registry.json` (declared `_meta.total_papers` = 139)
  - `$N6_ARCH/papers/` directory
  - `$PAPERS/` external repos (tecs-l, n6-architecture, root)
  - `$NEXUS/shared/n6/docs/products.json` (40 sections / 204 products; some products' `links[].path` reference `docs/paper/n6-*-paper.md`)
- Reference: `reports/audits/products-link-remap-2026-04-11.md` (prior Agent 4 paper MISS 116 items report)

---

## 1. Declared vs measured diff overview

### 1-1. Measured disk scan (n6-*-paper.md basenames)

| Location | File count |
|---|---:|
| `$N6_ARCH/papers/` | 13 |
| `$PAPERS/tecs-l/` | 23 |
| `$PAPERS/n6-architecture/` | 1 (`n6-millennium-problems-paper.md`) |
| `$PAPERS/` root | 1 (`n6-hexa-neuro-bci-paper.md`) |
| Union (de-duplicated by basename) | **38** |

### 1-2. Declared comparison

| Item | Value |
|---|---:|
| Declared `_registry.json _meta.total_papers` | 139 |
| Unique basenames actually referenced in `_registry.json` | 127 |
| Unique paper.md basenames in `products.json` `links[].path` | 116 |
| Unique basenames measured on disk | 38 |
| **ghost total** (products declared - disk existing) | **92** |
| **disk orphans** (disk - products referenced) | **14** |

### 1-3. Derivation formulas

```
Declared 139 (meta) vs measured disk 38  ->  declared surplus 101
Declared 127 (_registry path) vs disk 38  ->  surplus 89
products 116 vs disk 24 (products intersect disk)  ->  ghost 92
```

> Of the declared `_meta.total_papers=139`, 38 measured = **actual retention rate 27.3%**.
> Of the 116 products.json references, 24 resolve = **resolution rate 20.7%**.

---

## 2. products.json paper-link ghost distribution

### 2-1. Tally by category

| Category | Count | Meaning |
|---|---:|---|
| **FOUND_ALT** | 24 | File exists at a path different from the declared one (mostly `$PAPERS/tecs-l/`) |
| **GHOST_CEIL** | 92 | Not on disk anywhere; all in ceiling=True sections |
| **GHOST_NOCEIL** | 0 | No ghosts in ceiling=False sections (the quantum-computer section has no paper links at all) |
| **ORPHAN_DECLARED** | 11 | Declared only in _registry.json, 0 references from products.json |
| **ORPHAN_DISK** | 14 | Exists on disk, 0 references from products.json (includes 11 ORPHAN_DECLARED + 3 extra) |

### 2-2. ASCII chart by category

```
Category      Count   Bar (max 92)
GHOST_CEIL     92    ##################################################################
FOUND_ALT      24    #################
ORPHAN_DISK    14    ##########
ORPHAN_DECL    11    ########
GHOST_NOCEIL    0
```

---

## 3. FOUND_ALT 24 items — path update candidates

All declared paths follow the `docs/paper/n6-*.md` pattern, but the actual files are at `$PAPERS/tecs-l/` (23) and `$N6_ARCH/papers/` (1). Update `products.json` `links[].path` or physically migrate the files.

| Declared path (products.json) | Actual location | Section |
|---|---|---|
| `docs/paper/n6-aerospace-transport-paper.md` | `$PAPERS/tecs-l/n6-aerospace-transport-paper.md` | aerospace |
| `docs/paper/n6-autonomous-driving-paper.md` | `$PAPERS/tecs-l/n6-autonomous-driving-paper.md` | robotics |
| `docs/paper/n6-calendar-time-geography-paper.md` | `$PAPERS/tecs-l/n6-calendar-time-geography-paper.md` | civilization |
| `docs/paper/n6-classical-mechanics-accelerator-paper.md` | `$PAPERS/tecs-l/n6-classical-mechanics-accelerator-paper.md` | physics |
| `docs/paper/n6-cognitive-social-psychology-paper.md` | `$PAPERS/tecs-l/n6-cognitive-social-psychology-paper.md` | tech-industry |
| `docs/paper/n6-consciousness-soc-paper.md` | `$PAPERS/tecs-l/n6-consciousness-soc-paper.md` | chip |
| `docs/paper/n6-control-automation-paper.md` | `$PAPERS/tecs-l/n6-control-automation-paper.md` | robotics |
| `docs/paper/n6-ecology-agriculture-food-paper.md` | `$PAPERS/tecs-l/n6-ecology-agriculture-food-paper.md` | tech-industry |
| `docs/paper/n6-economics-finance-paper.md` | `$PAPERS/tecs-l/n6-economics-finance-paper.md` | tech-industry |
| `docs/paper/n6-games-sports-paper.md` | `$PAPERS/tecs-l/n6-games-sports-paper.md` | play |
| `docs/paper/n6-governance-safety-urban-paper.md` | `$PAPERS/tecs-l/n6-governance-safety-urban-paper.md` | safety |
| `docs/paper/n6-hexa-3d-paper.md` | `$PAPERS/tecs-l/n6-hexa-3d-paper.md` | chip |
| `docs/paper/n6-hexa-photon-paper.md` | `$PAPERS/tecs-l/n6-hexa-photon-paper.md` | chip |
| `docs/paper/n6-hexa-pim-paper.md` | `$PAPERS/tecs-l/n6-hexa-pim-paper.md` | chip |
| `docs/paper/n6-hexa-super-paper.md` | `$PAPERS/tecs-l/n6-hexa-super-paper.md` | physics |
| `docs/paper/n6-hexa-wafer-paper.md` | `$PAPERS/tecs-l/n6-hexa-wafer-paper.md` | chip |
| `docs/paper/n6-manufacturing-quality-paper.md` | `$PAPERS/tecs-l/n6-manufacturing-quality-paper.md` | tech-industry |
| `docs/paper/n6-quantum-computing-paper.md` | `$PAPERS/tecs-l/n6-quantum-computing-paper.md` | physics |
| `docs/paper/n6-space-systems-paper.md` | `$PAPERS/tecs-l/n6-space-systems-paper.md` | aerospace |
| `docs/paper/n6-telecom-linguistics-paper.md` | `$PAPERS/tecs-l/n6-telecom-linguistics-paper.md` | audio |
| `docs/paper/n6-therapeutic-nanobot-paper.md` | `$PAPERS/tecs-l/n6-therapeutic-nanobot-paper.md` | frontier |
| `docs/paper/n6-thermodynamics-paper.md` | `$PAPERS/tecs-l/n6-thermodynamics-paper.md` | physics |
| `docs/paper/n6-unified-soc-paper.md` | `$PAPERS/tecs-l/n6-unified-soc-paper.md` | chip |
| `papers/n6-synthetic-biology-paper.md` | `$N6_ARCH/papers/n6-synthetic-biology-paper.md` | tech-industry |

Recommended action: a single migration script that updates `products.json` `links[].path` to the actual paths (or physically moves files to `n6-architecture/papers/` for unified paths).

---

## 4. GHOST_CEIL 92 items — section distribution (ceiling-reached targets)

### 4-1. Ghosts per section (Top 10 — ASCII chart)

```
Section            ghosts  Bar (max 31)
frontier            31     ###############################
chip                 7     #######
civilization         7     #######
life-culture         6     ######
tech-industry        6     ######
software             5     #####
environment          4     ####
ai                   3     ###
energy               3     ###
physics              3     ###
audio                3     ###
fusion               2     ##
materials            2     ##
play                 2     ##
aerospace            1     #
robotics             1     #
sf                   1     #
safety               1     #
display              1     #
virology             1     #
hiv-treatment        1     #
cognitive-social     1     #
```

Ghost papers are distributed across 22 sections. Frontier alone contributes 31 items = 33.7% of all ghosts.

### 4-2. Per-section ghost list (summary)

#### aerospace (1)
- `docs/paper/n6-hexa-starship-paper.md` <- HEXA-STARSHIP

#### ai (3)
- `docs/paper/n6-causal-chain-paper.md`
- `docs/paper/n6-reality-map-paper.md`
- `docs/paper/n6-rtsc-12-products-evolution-paper.md`

#### audio (3)
- `docs/paper/n6-hexa-ear-paper.md` <- HEXA-EAR Ultimate
- `docs/paper/n6-hexa-speak-paper.md` <- HEXA-SPEAK
- `docs/paper/n6-isocell-comms-paper.md`

#### chip (7)
- `n6-anima-soc-paper.md`, `n6-dram-paper.md`, `n6-exynos-paper.md`, `n6-hexa-asic-paper.md`, `n6-hexa-topo-paper.md`, `n6-performance-chip-paper.md`, `n6-vnand-paper.md`

#### civilization (7)
- `n6-archaeology-paper.md`, `n6-dance-choreography-paper.md`, `n6-horology-paper.md`, `n6-jurisprudence-paper.md`, `n6-monetary-history-paper.md`, `n6-religion-mythology-paper.md`, `n6-writing-systems-paper.md`

#### cognitive-social (1)
- `docs/paper/n6-consciousness-chip-paper.md` <- HEXA-CONSCIOUSNESS

#### display (1)
- `docs/paper/n6-display-8stack-paper.md`

#### energy (3)
- `n6-battery-energy-paper.md`, `n6-datacenter-reactor-paper.md`, `n6-energy-efficiency-paper.md`

#### environment (4)
- `n6-carbon-capture-paper.md`, `n6-environment-thermal-paper.md`, `n6-hexa-recycle-paper.md`, `n6-microplastics-paper.md`

#### frontier (31) — 33.7% of all ghosts
- `n6-antimatter-factory-paper.md`, `n6-biology-medical-paper.md`, `n6-desal-paper.md`, `n6-entomology-paper.md`, `n6-hexa-accel-paper.md`, `n6-hexa-cloak-paper.md`, `n6-hexa-cosmic-paper.md`, `n6-hexa-defense-paper.md`, `n6-hexa-dream-paper.md`, `n6-hexa-empath-paper.md`, `n6-hexa-exo-paper.md`, `n6-hexa-fabric-paper.md`, `n6-hexa-glass-paper.md`, `n6-hexa-grav-paper.md`, `n6-hexa-holo-paper.md`, `n6-hexa-hover-paper.md`, `n6-hexa-limb-paper.md`, `n6-hexa-mind-paper.md`, `n6-hexa-mram-paper.md`, `n6-hexa-neuro-paper.md`, `n6-hexa-olfact-paper.md`, `n6-hexa-one-paper.md`, `n6-hexa-oracle-paper.md`, `n6-hexa-sim-paper.md`, `n6-hexa-skin-paper.md`, `n6-hexa-skyway-paper.md`, `n6-hexa-telepathy-paper.md`, `n6-hexa-teleport-paper.md`, `n6-hexa-tsunami-paper.md`, `n6-hexa-weather-paper.md`, `n6-seabed-grid-paper.md`

#### fusion (2)
- `n6-fusion-powerplant-paper.md`, `n6-plasma-fusion-deep-paper.md`

#### hiv-treatment (1)
- `n6-hiv-paper.md`

#### life-culture (6)
- `n6-aquaculture-paper.md`, `n6-dolphin-bioacoustics-paper.md`, `n6-fashion-textile-paper.md`, `n6-fermentation-paper.md`, `n6-insurance-paper.md`, `n6-wine-enology-paper.md`

#### materials (2)
- `n6-crystallography-materials-paper.md`, `n6-material-synthesis-paper.md`

#### physics (3)
- `n6-particle-cosmology-paper.md`, `n6-pure-mathematics-paper.md`, `n6-superconductor-paper.md`

#### play (2)
- `n6-fun-car-paper.md`, `n6-motorcycle-paper.md`

#### robotics (1)
- `n6-robotics-transport-paper.md`

#### safety (1)
- `n6-ultimate-safety-paper.md`

#### sf (1)
- `n6-hexa-ufo-paper.md`

#### software (5)
- `n6-hexa-ios-paper.md`, `n6-hexa-macos-paper.md`, `n6-hexa-netproto-paper.md`, `n6-hexa-proglang-paper.md`, `n6-software-crypto-paper.md`

#### tech-industry (6)
- `n6-advanced-packaging-paper.md`, `n6-ar-vr-xr-paper.md`, `n6-construction-structural-paper.md`, `n6-digital-twin-paper.md`, `n6-ecommerce-fintech-paper.md`, `n6-underground-tunnel-paper.md`

#### virology (1)
- `n6-virology-paper.md`

---

## 5. ORPHAN_DECLARED 11 items — _registry.json only, not referenced in products.json

Declared in `_registry.json`'s `papers_chunk_d_2026-04-11` section, but no product in `products.json` references these paths. The files do exist on disk (`$N6_ARCH/papers/`), so **the papers are valid** but the product mapping is missing.

| path | Disk exists | products.json reference |
|---|---|---|
| `papers/n6-ai-17-techniques-experimental-paper.md` | Y | N |
| `papers/n6-atlas-promotion-7-to-10-paper.md` | Y | N |
| `papers/n6-cross-paradigm-ai-paper.md` | Y | N |
| `papers/n6-curvature-geometry-paper.md` | Y | N |
| `papers/n6-dimensional-unfolding-paper.md` | Y | N |
| `papers/n6-extra-dimensions-paper.md` | Y | N |
| `papers/n6-geology-prem-paper.md` | Y | N |
| `papers/n6-hexa-earphone-paper.md` | Y | N |
| `papers/n6-meteorology-paper.md` | Y | N |
| `papers/n6-oceanography-paper.md` | Y | N |
| `papers/n6-warp-metric-paper.md` | Y | N |

Recommended action: register a corresponding product in products.json for each paper (e.g., as `n6-fusion-powerplant-paper` exists, add a physics-section product for `n6-curvature-geometry-paper`), or add the path to an existing product's `links[]`.

---

## 6. ORPHAN_DISK 14 items — on disk, not referenced in products.json

11 ORPHAN_DECLARED items + the following 3:

| File | Path |
|---|---|
| `n6-hexa-neuro-bci-paper.md` | `$PAPERS/n6-hexa-neuro-bci-paper.md` |
| `n6-millennium-problems-paper.md` | `$PAPERS/n6-architecture/n6-millennium-problems-paper.md` |
| `n6-sota-ssm-paper.md` | `$N6_ARCH/papers/n6-sota-ssm-paper.md` |

These 3 have no official references in `_registry.json` or `products.json`, so SSOT registration is required.

---

## 7. Recommended-action matrix

| Category | Count | Action | Priority |
|---|---:|---|:---:|
| FOUND_ALT | 24 | Update products.json `links[].path` to actual paths (or migrate to n6-architecture/papers/) | **P0** |
| GHOST_CEIL (frontier) | 31 | Author new papers — maximum impact at bt=264 | **P1** |
| GHOST_CEIL (chip) | 7 | Author new papers | P2 |
| GHOST_CEIL (civilization) | 7 | Author new papers | P2 |
| GHOST_CEIL (life-culture) | 6 | Author new papers | P2 |
| GHOST_CEIL (tech-industry) | 6 | Author new papers | P2 |
| GHOST_CEIL other | 35 | Author new papers | P3 |
| ORPHAN_DECLARED | 11 | Add product mapping in products.json | P2 |
| ORPHAN_DISK (extra 3) | 3 | Register in both _registry.json and products.json | P3 |
| Correct `_meta.total_papers` 139 | 1 | Adjust to measured 38 or to a target value for SSOT consistency | **P0** |

---

## 8. Top-3 recommended papers to author first

The frontier section has the largest bt_count (264) and thus the greatest ghost-resolution impact. Among the 31 frontier ghosts, the items whose BT mapping is most advanced:

| Rank | paper basename | Product | Section bt |
|:---:|---|---|---:|
| 1 | `n6-hexa-neuro-paper.md` | HEXA-NEURO (brain-machine interface) | 264 |
| 2 | `n6-antimatter-factory-paper.md` | HEXA-ANTIMATTER (antimatter factory) | 264 |
| 3 | `n6-hexa-mind-paper.md` | HEXA-MIND (consciousness upload) | 264 |

> Reference: `_registry.json` `papers_chunk_c_2026-04-08` already declares `n6-hexa-neuro-paper.md`, `n6-hexa-mind-paper.md` as "planned", but the actual files are absent. This chunk's 11 papers are currently 0-written.

---

## 9. Overall summary

| Measurement | Value |
|---|---:|
| Declared `_meta.total_papers` | 139 |
| Measured disk-basename union | **38** |
| products.json-referenced basenames | 116 |
| GHOST (declared only in products) | **92** |
| FOUND_ALT (alt path exists) | **24** |
| ORPHAN_DECLARED (_registry only) | 11 |
| ORPHAN_DISK (disk only) | 14 |
| Declared vs measured gap (139 - 38) | **101** |

### 9-1. Consistency indicators

```
Declared 139 -> measured 38
Completion: 27.3%  [##########                                      ]

products 116 -> resolved 24
Resolution: 20.7%  [########                                        ]

GHOST to author: 92
ORPHAN to map:  14
Paths to update: 24
```

### 9-2. Limitations of this audit

- This report judges **only file existence**; paper-body quality / completeness / BT verification is out of scope.
- Of the 11 items in `_registry.json`'s `papers_chunk_c_2026-04-08`, only 1 (`n6-synthetic-biology-paper.md`) actually exists. The other 10 are ghosts.
- All 11 items in `papers_chunk_d_2026-04-11` exist on disk (ORPHAN_DECLARED).
- Only proposals for products.json changes are given; actual path updates, migrations, or paper authoring will proceed in **separate user-approved sessions**.

### 9-3. Items requiring user approval

1. `_meta.total_papers: 139` -> whether to correct to the measured value (38) or raise the target (e.g., 140)
2. FOUND_ALT 24 items: products.json path-update style (e.g., relative path `../papers/tecs-l/...` vs absolute)
3. GHOST_CEIL 92 items: approval to author sequentially starting from the 31 frontier items
4. ORPHAN_DECLARED 11 items: whether to create new products in products.json or add links to existing products

---

Auditor: Claude (n6-architecture session)
Date: 2026-04-11
Audit mode: read-only (no SSOT modification)
