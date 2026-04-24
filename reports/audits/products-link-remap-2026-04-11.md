# products.json Link Drift Remap Report

- Created: 2026-04-11
- Source SSOT: `$NEXUS/shared/n6/docs/products.json`
- Reference path: `$N6_ARCH/` (R1 audit exception)
- Prior audit: `reports/audits/products-link-audit-2026-04-11.md` (3/416=0.7%)
- This task: one-shot migration script (`/tmp/remap_work/remap_products.py`)

## Summary

- Total items inspected: 416
- RESOLVED: 242
- UNRESOLVED (MISS): 174
- Completion: 58.2% (prior 0.7%, +57.5 pp)

## Status Distribution

| Status | Count | Meaning |
|---|---:|---|
| MISS | 174 | path resolution failed |
| VERIFY_HEXA | 114 | - |
| DOM | 113 | docs/<dom>/<f> → domains/<axis>/<dom>/<f> |
| SPECS | 4 | - |
| DOCS1 | 4 | docs/<f> single file -> theory or papers |
| EXP_HEXA | 3 | experiments/*.py → *.hexa (R1 conversion) |
| EXIST | 3 | already exists |
| FALLBACK_HEXA | 1 | - |

## Per-section Mapping Statistics (completion ascending)

| Section id | RESOLVED | TOTAL | Completion |
|---|---:|---:|---:|
| hiv-treatment | 0 | 6 | 0.0% |
| hygiene | 0 | 2 | 0.0% |
| chip | 7 | 24 | 29.2% |
| life-culture | 5 | 15 | 33.3% |
| ai | 4 | 12 | 33.3% |
| civilization | 6 | 15 | 40.0% |
| audio | 7 | 16 | 43.8% |
| robotics | 4 | 8 | 50.0% |
| sf | 1 | 2 | 50.0% |
| manufacturing-quality | 1 | 2 | 50.0% |
| environment | 9 | 17 | 52.9% |
| materials | 8 | 15 | 53.3% |
| safety | 4 | 7 | 57.1% |
| aerospace | 4 | 7 | 57.1% |
| tech-industry | 30 | 50 | 60.0% |
| physics | 11 | 18 | 61.1% |
| fusion | 8 | 13 | 61.5% |
| software | 11 | 17 | 64.7% |
| play | 6 | 9 | 66.7% |
| display | 4 | 6 | 66.7% |
| virology | 4 | 6 | 66.7% |
| frontier | 74 | 110 | 67.3% |
| energy | 10 | 14 | 71.4% |
| cognitive-social | 5 | 6 | 83.3% |
| natural-science | 4 | 4 | 100.0% |
| marketing | 4 | 4 | 100.0% |
| digital-medical | 3 | 3 | 100.0% |
| mobility | 2 | 2 | 100.0% |
| tattoo-removal | 1 | 1 | 100.0% |
| keyboard | 1 | 1 | 100.0% |
| mouse | 1 | 1 | 100.0% |
| network | 1 | 1 | 100.0% |
| quantum-computer | 1 | 1 | 100.0% |
| horology | 1 | 1 | 100.0% |

## ASCII Bar Chart (RESOLVED% per section)

```
Section id                  Compl.  [              bar                 ]  R/T
------------------------------------------------------------------------------------------
hiv-treatment                0.0%  [....................................]    0/6  
hygiene                      0.0%  [....................................]    0/2  
chip                        29.2%  [##########..........................]    7/24 
life-culture                33.3%  [############........................]    5/15 
ai                          33.3%  [############........................]    4/12 
civilization                40.0%  [##############......................]    6/15 
audio                       43.8%  [################....................]    7/16 
robotics                    50.0%  [##################..................]    4/8  
sf                          50.0%  [##################..................]    1/2  
manufacturing-quality       50.0%  [##################..................]    1/2  
environment                 52.9%  [###################.................]    9/17 
materials                   53.3%  [###################.................]    8/15 
safety                      57.1%  [#####################...............]    4/7  
aerospace                   57.1%  [#####################...............]    4/7  
tech-industry               60.0%  [######################..............]   30/50 
physics                     61.1%  [######################..............]   11/18 
fusion                      61.5%  [######################..............]    8/13 
software                    64.7%  [#######################.............]   11/17 
play                        66.7%  [########################............]    6/9  
display                     66.7%  [########################............]    4/6  
virology                    66.7%  [########################............]    4/6  
frontier                    67.3%  [########################............]   74/110
energy                      71.4%  [##########################..........]   10/14 
cognitive-social            83.3%  [##############################......]    5/6  
natural-science            100.0%  [####################################]    4/4  
marketing                  100.0%  [####################################]    4/4  
digital-medical            100.0%  [####################################]    3/3  
mobility                   100.0%  [####################################]    2/2  
tattoo-removal             100.0%  [####################################]    1/1  
keyboard                   100.0%  [####################################]    1/1  
mouse                      100.0%  [####################################]    1/1  
network                    100.0%  [####################################]    1/1  
quantum-computer           100.0%  [####################################]    1/1  
horology                   100.0%  [####################################]    1/1  
```

## Mapping Log (per section)

### aerospace (4/7)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/hexa-starship/goal.md` → `domains/space/hexa-starship/goal.md` |
| MISS | `docs/paper/n6-aerospace-transport-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-starship-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-space-systems-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-starship/verify_hexa_starship.py` → `domains/space/hexa-starship/verify.hexa` |
| VERIFY_HEXA | `docs/hexa-starship/verify_hexa_starship.py` → `domains/space/hexa-starship/verify.hexa` |
| VERIFY_HEXA | `docs/hexa-starship/verify_subsystems.py` → `domains/space/hexa-starship/verify.hexa` |

### ai (4/12)

| Status | Old path -> New path |
|---|---|
| MISS | `docs/ai-efficiency/techniques-complete.md` → (unresolved) |
| EXP_HEXA | `experiments/experiment_full_n6_pipeline.py` → `experiments/structural/experiment_full_n6_pipeline.hexa` |
| MISS | `docs/paper/n6-causal-chain-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-reality-map-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-rtsc-12-products-evolution-paper.md` → (unresolved) |
| SPECS | `docs/superpowers/specs/2026-03-28-n6-inevitability-engine-design.md` → `reports/sessions/specs/2026-03-28-n6-inevitability-engine-design.md` |
| DOCS1 | `docs/ai-energy-savings-guide.md` → `reports/discovery/ai-energy-savings-guide.md` |
| DOCS1 | `docs/chip-architecture-guide.md` → `reports/discovery/chip-architecture-guide.md` |
| MISS | `docs/ai-efficiency/full-verification-matrix.md` → (unresolved) |
| MISS | `docs/ai-efficiency/next-model-blowup-2026-04.md` → (unresolved) |
| MISS | `docs/ai-efficiency/bt-391-code-generation.md` → (unresolved) |
| MISS | `docs/ai-efficiency/bt-397-n6-novel-architectures.md` → (unresolved) |

### audio (7/16)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/audio/goal.md` → `domains/culture/audio/goal.md` |
| MISS | `docs/paper/n6-isocell-comms-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-telecom-linguistics-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/audio/verify_alien10.py` → `domains/culture/audio/verify.hexa` |
| MISS | `docs/audio/full-verification-matrix.md` → (unresolved) |
| VERIFY_HEXA | `docs/audio/verify_alien10.py` → `domains/culture/audio/verify.hexa` |
| DOM | `docs/hexa-speak/goal.md` → `domains/cognitive/hexa-speak/goal.md` |
| MISS | `docs/paper/n6-hexa-speak-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-speak/verify_alien10.py` → `domains/cognitive/hexa-speak/verify.hexa` |
| MISS | `docs/audio/hexa-ear-ultimate.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-ear-paper.md` → (unresolved) |
| MISS | `docs/audio/hexa-bone-ultimate.md` → (unresolved) |
| MISS | `docs/audio/hexa-ear-cell.md` → (unresolved) |
| MISS | `docs/audio/hexa-speaker-ultimate.md` → (unresolved) |
| VERIFY_HEXA | `docs/audio/verify_alien10.py` → `domains/culture/audio/verify.hexa` |
| VERIFY_HEXA | `docs/hexa-speak/verify_alien10.py` → `domains/cognitive/hexa-speak/verify.hexa` |

### chip (7/24)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/chip-architecture/goal.md` → `domains/compute/chip-architecture/goal.md` |
| MISS | `docs/paper/n6-dram-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-exynos-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-3d-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-photon-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-pim-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-wafer-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-performance-chip-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-unified-soc-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-vnand-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/chip-architecture/verify_alien10.py` → `domains/compute/chip-architecture/verify.hexa` |
| MISS | `docs/chip-architecture/ultimate-consciousness-soc.md` → (unresolved) |
| MISS | `docs/paper/n6-anima-soc-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-consciousness-soc-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/chip-architecture/verify_alien10.py` → `domains/compute/chip-architecture/verify.hexa` |
| MISS | `docs/chip-architecture/hexa-topological-performance-chip.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-topo-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/chip-architecture/verify_alien10.py` → `domains/compute/chip-architecture/verify.hexa` |
| MISS | `docs/chip-architecture/hexa-asic-skywater.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-asic-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/chip-architecture/verify_alien10.py` → `domains/compute/chip-architecture/verify.hexa` |
| MISS | `docs/chip-architecture/full-verification-matrix.md` → (unresolved) |
| VERIFY_HEXA | `docs/chip-architecture/verify_alien10.py` → `domains/compute/chip-architecture/verify.hexa` |
| VERIFY_HEXA | `docs/chip-architecture/verify_alien10.py` → `domains/compute/chip-architecture/verify.hexa` |

### civilization (6/15)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/religion/goal.md` → `domains/culture/religion/goal.md` |
| MISS | `docs/paper/n6-religion-mythology-paper.md` → (unresolved) |
| DOM | `docs/jurisprudence/goal.md` → `domains/infra/jurisprudence/goal.md` |
| MISS | `docs/paper/n6-jurisprudence-paper.md` → (unresolved) |
| DOM | `docs/writing-systems/goal.md` → `domains/culture/writing-systems/goal.md` |
| MISS | `docs/paper/n6-writing-systems-paper.md` → (unresolved) |
| DOM | `docs/archaeology/goal.md` → `domains/culture/archaeology/goal.md` |
| MISS | `docs/paper/n6-archaeology-paper.md` → (unresolved) |
| DOM | `docs/monetary-history/goal.md` → `domains/infra/monetary-history/goal.md` |
| MISS | `docs/paper/n6-monetary-history-paper.md` → (unresolved) |
| DOM | `docs/dance-choreography/goal.md` → `domains/culture/dance-choreography/goal.md` |
| MISS | `docs/paper/n6-dance-choreography-paper.md` → (unresolved) |
| MISS | `docs/horology/hypotheses.md` → (unresolved) |
| MISS | `docs/paper/n6-calendar-time-geography-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-horology-paper.md` → (unresolved) |

### cognitive-social (5/6)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/cognitive-architecture/goal.md` → `domains/cognitive/cognitive-architecture/goal.md` |
| MISS | `docs/paper/n6-consciousness-chip-paper.md` → (unresolved) |
| DOM | `docs/social-architecture/goal.md` → `domains/culture/social-architecture/goal.md` |
| DOM | `docs/temporal-architecture/goal.md` → `domains/cognitive/temporal-architecture/goal.md` |
| DOM | `docs/linguistics/goal.md` → `domains/culture/linguistics/goal.md` |
| DOM | `docs/economics/goal.md` → `domains/infra/economics/goal.md` |

### digital-medical (3/3)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/browser/goal.md` → `domains/compute/browser/goal.md` |
| DOM | `docs/medical-device/goal.md` → `domains/life/medical-device/goal.md` |
| DOM | `docs/cosmetic-surgery/goal.md` → `domains/life/cosmetic-surgery/goal.md` |

### display (4/6)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/display/goal.md` → `domains/compute/display/goal.md` |
| MISS | `docs/paper/n6-display-8stack-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/display/verify_alien10.py` → `domains/compute/display/verify.hexa` |
| MISS | `docs/display/full-verification-matrix.md` → (unresolved) |
| VERIFY_HEXA | `docs/display/verify_alien10.py` → `domains/compute/display/verify.hexa` |
| VERIFY_HEXA | `docs/display/verify_alien10.py` → `domains/compute/display/verify.hexa` |

### energy (10/14)

| Status | Old path -> New path |
|---|---|
| SPECS | `docs/superpowers/specs/2026-04-01-hexa-battery-design.md` → `reports/sessions/specs/2026-04-01-hexa-battery-design.md` |
| MISS | `docs/paper/n6-battery-energy-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/battery-architecture/verify_alien10.py` → `domains/energy/battery-architecture/verify.hexa` |
| DOM | `docs/solar-architecture/goal.md` → `domains/energy/solar-architecture/goal.md` |
| VERIFY_HEXA | `docs/battery-architecture/verify_alien10.py` → `domains/energy/battery-architecture/verify.hexa` |
| DOM | `docs/energy-architecture/goal.md` → `domains/energy/energy-architecture/goal.md` |
| MISS | `docs/paper/n6-energy-efficiency-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/battery-architecture/verify_alien10.py` → `domains/energy/battery-architecture/verify.hexa` |
| DOM | `docs/smr-datacenter/goal.md` → `domains/energy/smr-datacenter/goal.md` |
| MISS | `docs/paper/n6-datacenter-reactor-paper.md` → (unresolved) |
| MISS | `docs/battery-architecture/hexa-auto-battery.md` → (unresolved) |
| VERIFY_HEXA | `docs/battery-architecture/verify_alien10.py` → `domains/energy/battery-architecture/verify.hexa` |
| VERIFY_HEXA | `docs/solar-architecture/verify_alien10.py` → `domains/energy/solar-architecture/verify.hexa` |
| VERIFY_HEXA | `docs/energy-architecture/verify_alien10.py` → `domains/energy/energy-architecture/verify.hexa` |

### environment (9/17)

| Status | Old path -> New path |
|---|---|
| MISS | `docs/environmental-protection/` → (unresolved) |
| MISS | `docs/paper/n6-environment-thermal-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/environmental-protection/verify_alien10.py` → `domains/infra/environmental-protection/verify.hexa` |
| MISS | `docs/environmental-protection/microplastics-solution.md` → (unresolved) |
| MISS | `docs/paper/n6-microplastics-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/environmental-protection/verify_alien10.py` → `domains/infra/environmental-protection/verify.hexa` |
| DOM | `docs/carbon-capture/goal.md` → `domains/infra/carbon-capture/goal.md` |
| MISS | `docs/paper/n6-carbon-capture-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/environmental-protection/verify_alien10.py` → `domains/infra/environmental-protection/verify.hexa` |
| MISS | `docs/environmental-protection/evolution/` → (unresolved) |
| VERIFY_HEXA | `docs/environmental-protection/verify_alien10.py` → `domains/infra/environmental-protection/verify.hexa` |
| MISS | `docs/environmental-protection/testable-predictions-2030.md` → (unresolved) |
| VERIFY_HEXA | `docs/environmental-protection/verify_alien10.py` → `domains/infra/environmental-protection/verify.hexa` |
| DOM | `docs/recycling/goal.md` → `domains/materials/recycling/goal.md` |
| MISS | `docs/paper/n6-hexa-recycle-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/environmental-protection/verify_alien10.py` → `domains/infra/environmental-protection/verify.hexa` |
| VERIFY_HEXA | `docs/carbon-capture/verify_alien10.py` → `domains/infra/carbon-capture/verify.hexa` |

### frontier (74/110)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/neuro/goal.md` → `domains/life/neuro/goal.md` |
| MISS | `docs/paper/n6-biology-medical-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-hexa-neuro-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/neuro/verify_alien10.py` → `domains/life/neuro/verify.hexa` |
| DOM | `docs/gravity-wave/goal.md` → `domains/physics/gravity-wave/goal.md` |
| MISS | `docs/paper/n6-hexa-grav-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/gravity-wave/verify_alien10.py` → `domains/physics/gravity-wave/verify.hexa` |
| DOM | `docs/cloak/goal.md` → `domains/sf-ufo/cloak/goal.md` |
| MISS | `docs/paper/n6-hexa-cloak-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/cloak/verify_alien10.py` → `domains/sf-ufo/cloak/verify.hexa` |
| DOM | `docs/earth-defense/goal.md` → `domains/infra/earth-defense/goal.md` |
| MISS | `docs/paper/n6-hexa-defense-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/earth-defense/verify_alien10.py` → `domains/infra/earth-defense/verify.hexa` |
| DOM | `docs/quantum-network/goal.md` → `domains/physics/quantum-network/goal.md` |
| MISS | `docs/paper/n6-hexa-teleport-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/quantum-network/verify_alien10.py` → `domains/physics/quantum-network/verify.hexa` |
| DOM | `docs/hover/goal.md` → `domains/sf-ufo/hover/goal.md` |
| MISS | `docs/paper/n6-hexa-hover-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hover/verify_alien10.py` → `domains/sf-ufo/hover/verify.hexa` |
| DOM | `docs/sc-memory/goal.md` → `domains/compute/sc-memory/goal.md` |
| MISS | `docs/paper/n6-hexa-mram-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/sc-memory/verify_alien10.py` → `domains/compute/sc-memory/verify.hexa` |
| DOM | `docs/seabed-grid/goal.md` → `domains/infra/seabed-grid/goal.md` |
| MISS | `docs/paper/n6-seabed-grid-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/seabed-grid/verify_alien10.py` → `domains/infra/seabed-grid/verify.hexa` |
| DOM | `docs/mini-accelerator/goal.md` → `domains/physics/mini-accelerator/goal.md` |
| MISS | `docs/paper/n6-hexa-accel-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/mini-accelerator/verify_alien10.py` → `domains/physics/mini-accelerator/verify.hexa` |
| DOM | `docs/weather-control/goal.md` → `domains/infra/weather-control/goal.md` |
| MISS | `docs/paper/n6-hexa-weather-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/weather-control/verify_alien10.py` → `domains/infra/weather-control/verify.hexa` |
| DOM | `docs/mind-upload/goal.md` → `domains/cognitive/mind-upload/goal.md` |
| MISS | `docs/paper/n6-hexa-mind-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/mind-upload/verify_alien10.py` → `domains/cognitive/mind-upload/verify.hexa` |
| DOM | `docs/telepathy/goal.md` → `domains/cognitive/telepathy/goal.md` |
| MISS | `docs/paper/n6-hexa-telepathy-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/telepathy/verify_alien10.py` → `domains/cognitive/telepathy/verify.hexa` |
| DOM | `docs/holography/goal.md` → `domains/physics/holography/goal.md` |
| MISS | `docs/paper/n6-hexa-holo-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/holography/verify_alien10.py` → `domains/physics/holography/verify.hexa` |
| DOM | `docs/dream-recorder/goal.md` → `domains/cognitive/dream-recorder/goal.md` |
| MISS | `docs/paper/n6-hexa-dream-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/dream-recorder/verify_alien10.py` → `domains/cognitive/dream-recorder/verify.hexa` |
| DOM | `docs/skyway/goal.md` → `domains/infra/skyway/goal.md` |
| MISS | `docs/paper/n6-hexa-skyway-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/skyway/verify_alien10.py` → `domains/infra/skyway/verify.hexa` |
| DOM | `docs/tsunami-shield/goal.md` → `domains/infra/tsunami-shield/goal.md` |
| MISS | `docs/paper/n6-hexa-tsunami-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/tsunami-shield/verify_alien10.py` → `domains/infra/tsunami-shield/verify.hexa` |
| DOM | `docs/antimatter-factory/goal.md` → `domains/physics/antimatter-factory/goal.md` |
| MISS | `docs/paper/n6-antimatter-factory-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/antimatter-factory/verify_alien10.py` → `domains/physics/antimatter-factory/verify.hexa` |
| DOM | `docs/cosmic-observatory/goal.md` → `domains/physics/cosmic-observatory/goal.md` |
| MISS | `docs/paper/n6-hexa-cosmic-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/cosmic-observatory/verify_alien10.py` → `domains/physics/cosmic-observatory/verify.hexa` |
| DOM | `docs/desalination/goal.md` → `domains/infra/desalination/goal.md` |
| MISS | `docs/paper/n6-desal-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/desalination/verify_alien10.py` → `domains/infra/desalination/verify.hexa` |
| DOM | `docs/quantum-oracle/goal.md` → `domains/physics/quantum-oracle/goal.md` |
| MISS | `docs/paper/n6-hexa-oracle-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/quantum-oracle/verify_alien10.py` → `domains/physics/quantum-oracle/verify.hexa` |
| DOM | `docs/hexa-one/goal.md` → `domains/compute/hexa-one/goal.md` |
| MISS | `docs/paper/n6-hexa-one-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-one/verify_alien10.py` → `domains/compute/hexa-one/verify.hexa` |
| DOM | `docs/hexa-glass/goal.md` → `domains/materials/hexa-glass/goal.md` |
| MISS | `docs/paper/n6-hexa-glass-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-glass/verify_alien10.py` → `domains/materials/hexa-glass/verify.hexa` |
| DOM | `docs/hexa-ear/goal.md` → `domains/cognitive/hexa-ear/goal.md` |
| VERIFY_HEXA | `docs/hexa-ear/verify_alien10.py` → `domains/cognitive/hexa-ear/verify.hexa` |
| DOM | `docs/hexa-exo/goal.md` → `domains/infra/hexa-exo/goal.md` |
| MISS | `docs/paper/n6-hexa-exo-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-exo/verify_alien10.py` → `domains/infra/hexa-exo/verify.hexa` |
| DOM | `docs/hexa-limb/goal.md` → `domains/life/hexa-limb/goal.md` |
| MISS | `docs/paper/n6-hexa-limb-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-limb/verify_alien10.py` → `domains/life/hexa-limb/verify.hexa` |
| DOM | `docs/hexa-skin/goal.md` → `domains/life/hexa-skin/goal.md` |
| MISS | `docs/paper/n6-hexa-skin-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-skin/verify_alien10.py` → `domains/life/hexa-skin/verify.hexa` |
| DOM | `docs/hexa-fabric/goal.md` → `domains/materials/hexa-fabric/goal.md` |
| MISS | `docs/paper/n6-hexa-fabric-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-fabric/verify_alien10.py` → `domains/materials/hexa-fabric/verify.hexa` |
| DOM | `docs/hexa-olfact/goal.md` → `domains/cognitive/hexa-olfact/goal.md` |
| MISS | `docs/paper/n6-hexa-olfact-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-olfact/verify_alien10.py` → `domains/cognitive/hexa-olfact/verify.hexa` |
| DOM | `docs/hexa-dream/goal.md` → `domains/cognitive/hexa-dream/goal.md` |
| VERIFY_HEXA | `docs/hexa-dream/verify_alien10.py` → `domains/cognitive/hexa-dream/verify.hexa` |
| DOM | `docs/hexa-empath/goal.md` → `domains/cognitive/hexa-empath/goal.md` |
| MISS | `docs/paper/n6-hexa-empath-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/hexa-empath/verify_alien10.py` → `domains/cognitive/hexa-empath/verify.hexa` |
| DOM | `docs/virology/goal.md` → `domains/life/virology/goal.md` |
| VERIFY_HEXA | `docs/virology/verify_alien10.py` → `domains/life/virology/verify.hexa` |
| DOM | `docs/entomology/goal.md` → `domains/life/entomology/goal.md` |
| MISS | `docs/paper/n6-entomology-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/entomology/verify_alien10.py` → `domains/life/entomology/verify.hexa` |
| MISS | `docs/mycology/hypotheses.md` → (unresolved) |
| VERIFY_HEXA | `docs/mycology/verify_alien10.py` → `domains/life/mycology/verify.hexa` |
| MISS | `docs/mining/hypotheses.md` → (unresolved) |
| VERIFY_HEXA | `docs/mining/verify_alien10.py` → `domains/infra/mining/verify.hexa` |
| MISS | `docs/veterinary/hypotheses.md` → (unresolved) |
| VERIFY_HEXA | `docs/veterinary/verify_alien10.py` → `domains/life/veterinary/verify.hexa` |
| MISS | `docs/horticulture/hypotheses.md` → (unresolved) |
| VERIFY_HEXA | `docs/horticulture/verify_alien10.py` → `domains/life/horticulture/verify.hexa` |
| DOM | `docs/simulation-theory/goal.md` → `domains/physics/simulation-theory/goal.md` |
| MISS | `docs/paper/n6-hexa-sim-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/simulation-theory/verify_alien10.py` → `domains/physics/simulation-theory/verify.hexa` |
| DOCS1 | `docs/new-bt-dimensional-unfolding-2026-04-06.md` → `reports/breakthroughs/new-bt-dimensional-unfolding-2026-04-06.md` |
| VERIFY_HEXA | `docs/cross-domain-mega/verify_alien10.py` → `domains/sf-ufo/cross-domain-mega/verify.hexa` |
| DOM | `docs/therapeutic-nanobot/goal.md` → `domains/life/therapeutic-nanobot/goal.md` |
| MISS | `docs/paper/n6-therapeutic-nanobot-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/therapeutic-nanobot/verify_alien10.py` → `domains/life/therapeutic-nanobot/verify.hexa` |

### fusion (8/13)

| Status | Old path -> New path |
|---|---|
| SPECS | `docs/superpowers/specs/2026-04-02-ultimate-fusion-powerplant-design.md` → `reports/sessions/specs/2026-04-02-ultimate-fusion-powerplant-design.md` |
| MISS | `docs/paper/n6-fusion-powerplant-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-plasma-fusion-deep-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/fusion/verify_alien10.py` → `domains/energy/fusion/verify.hexa` |
| SPECS | `docs/superpowers/specs/2026-04-02-kstar-n6-tokamak-design.md` → `reports/sessions/specs/2026-04-02-kstar-n6-tokamak-design.md` |
| VERIFY_HEXA | `docs/fusion/verify_alien10.py` → `domains/energy/fusion/verify.hexa` |
| MISS | `docs/fusion/evolution/mk-1-first-light.md` → (unresolved) |
| VERIFY_HEXA | `docs/fusion/verify_alien10.py` → `domains/energy/fusion/verify.hexa` |
| MISS | `docs/fusion/alien-level-discoveries.md` → (unresolved) |
| VERIFY_HEXA | `docs/fusion/verify_alien10.py` → `domains/energy/fusion/verify.hexa` |
| MISS | `docs/fusion/physical-limit-proof.md` → (unresolved) |
| VERIFY_HEXA | `docs/fusion/verify_alien10.py` → `domains/energy/fusion/verify.hexa` |
| VERIFY_HEXA | `docs/fusion/verify_alien10.py` → `domains/energy/fusion/verify.hexa` |

### hiv-treatment (0/6)

| Status | Old path -> New path |
|---|---|
| MISS | `docs/paper/n6-hiv-paper.md` → (unresolved) |
| MISS | `docs/hiv-treatment/evolution/mk-1-basic.md` → (unresolved) |
| MISS | `docs/hiv-treatment/evolution/mk-2-short.md` → (unresolved) |
| MISS | `docs/hiv-treatment/evolution/mk-3-mid.md` → (unresolved) |
| MISS | `docs/hiv-treatment/evolution/mk-4-long.md` → (unresolved) |
| MISS | `docs/hiv-treatment/evolution/mk-5-ultimate.md` → (unresolved) |

### horology (1/1)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/horology/goal.md` → `domains/culture/horology/goal.md` |

### hygiene (0/2)

| Status | Old path -> New path |
|---|---|
| MISS | `docs/mens-intimate-cleanser/breakthrough.md` → (unresolved) |
| MISS | `docs/womens-intimate-cleanser/breakthrough.md` → (unresolved) |

### keyboard (1/1)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/keyboard/goal.md` → `domains/compute/keyboard/goal.md` |

### life-culture (5/15)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/fermentation/goal.md` → `domains/life/fermentation/goal.md` |
| MISS | `docs/paper/n6-fermentation-paper.md` → (unresolved) |
| DOM | `docs/wine-enology/goal.md` → `domains/life/wine-enology/goal.md` |
| MISS | `docs/paper/n6-wine-enology-paper.md` → (unresolved) |
| DOM | `docs/fashion-textile/goal.md` → `domains/materials/fashion-textile/goal.md` |
| MISS | `docs/paper/n6-fashion-textile-paper.md` → (unresolved) |
| DOM | `docs/aquaculture/goal.md` → `domains/life/aquaculture/goal.md` |
| MISS | `docs/paper/n6-aquaculture-paper.md` → (unresolved) |
| DOM | `docs/insurance/goal.md` → `domains/infra/insurance/goal.md` |
| MISS | `docs/paper/n6-insurance-paper.md` → (unresolved) |
| MISS | `docs/dolphin/hypotheses.md` → (unresolved) |
| MISS | `docs/paper/n6-dolphin-bioacoustics-paper.md` → (unresolved) |
| MISS | `docs/coffee-science/hypotheses.md` → (unresolved) |
| MISS | `docs/perfumery/hypotheses.md` → (unresolved) |
| MISS | `docs/ceramics/hypotheses.md` → (unresolved) |

### manufacturing-quality (1/2)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/manufacturing-quality/goal.md` → `domains/infra/manufacturing-quality/goal.md` |
| MISS | `docs/paper/n6-manufacturing-quality-paper.md` → (unresolved) |

### marketing (4/4)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/marketing/goal.md` → `domains/infra/marketing/goal.md` |
| DOM | `docs/nexus-service/goal.md` → `domains/compute/nexus-service/goal.md` |
| DOM | `docs/anima-service/goal.md` → `domains/cognitive/anima-service/goal.md` |
| DOM | `docs/unified-service/goal.md` → `domains/compute/unified-service/goal.md` |

### materials (8/15)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/material-synthesis/goal.md` → `domains/materials/material-synthesis/goal.md` |
| MISS | `docs/paper/n6-crystallography-materials-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-material-synthesis-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |
| MISS | `docs/material-synthesis/breakthrough-theorems.md` → (unresolved) |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |
| MISS | `docs/material-synthesis/hypotheses.md` → (unresolved) |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |
| MISS | `docs/material-synthesis/industrial-validation.md` → (unresolved) |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |
| MISS | `docs/material-synthesis/experimental-verification.md` → (unresolved) |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |
| MISS | `docs/material-synthesis/physical-limit-proof.md` → (unresolved) |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |
| VERIFY_HEXA | `docs/material-synthesis/verify_alien10.py` → `domains/materials/material-synthesis/verify.hexa` |

### mobility (2/2)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/autonomous-driving/goal.md` → `domains/infra/autonomous-driving/goal.md` |
| DOM | `docs/aviation/goal.md` → `domains/infra/aviation/goal.md` |

### mouse (1/1)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/mouse/goal.md` → `domains/compute/mouse/goal.md` |

### natural-science (4/4)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/biology/goal.md` → `domains/life/biology/goal.md` |
| DOM | `docs/agriculture/goal.md` → `domains/life/agriculture/goal.md` |
| DOM | `docs/food-science/goal.md` → `domains/life/food-science/goal.md` |
| DOM | `docs/oceanography/goal.md` → `domains/infra/oceanography/goal.md` |

### network (1/1)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/network/goal.md` → `domains/compute/network/goal.md` |

### physics (11/18)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/superconductor/goal.md` → `domains/energy/superconductor/goal.md` |
| MISS | `docs/paper/n6-superconductor-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/pure-mathematics/verify_alien10.py` → `domains/physics/pure-mathematics/verify.hexa` |
| DOM | `docs/pure-mathematics/goal.md` → `domains/physics/pure-mathematics/goal.md` |
| MISS | `docs/paper/n6-pure-mathematics-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/pure-mathematics/verify_alien10.py` → `domains/physics/pure-mathematics/verify.hexa` |
| DOM | `docs/cosmology-particle/goal.md` → `domains/physics/cosmology-particle/goal.md` |
| MISS | `docs/paper/n6-classical-mechanics-accelerator-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-particle-cosmology-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-thermodynamics-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/pure-mathematics/verify_alien10.py` → `domains/physics/pure-mathematics/verify.hexa` |
| DOM | `docs/room-temp-sc/goal.md` → `domains/energy/room-temp-sc/goal.md` |
| MISS | `docs/paper/n6-hexa-super-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-quantum-computing-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/room-temp-sc/verify_alien10.py` → `domains/energy/room-temp-sc/verify.hexa` |
| DOCS1 | `docs/new-bt-dimensional-unfolding-2026-04-06.md` → `reports/breakthroughs/new-bt-dimensional-unfolding-2026-04-06.md` |
| VERIFY_HEXA | `docs/pure-mathematics/verify_alien10.py` → `domains/physics/pure-mathematics/verify.hexa` |
| VERIFY_HEXA | `docs/cosmology-particle/verify_alien10.py` → `domains/physics/cosmology-particle/verify.hexa` |

### play (6/9)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/fun-car/goal.md` → `domains/infra/fun-car/goal.md` |
| MISS | `docs/paper/n6-fun-car-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-games-sports-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/fun-car/verify_alien10.py` → `domains/infra/fun-car/verify.hexa` |
| DOM | `docs/motorcycle/goal.md` → `domains/infra/motorcycle/goal.md` |
| MISS | `docs/paper/n6-motorcycle-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/motorcycle/verify_alien10.py` → `domains/infra/motorcycle/verify.hexa` |
| VERIFY_HEXA | `docs/fun-car/verify_alien10.py` → `domains/infra/fun-car/verify.hexa` |
| VERIFY_HEXA | `docs/motorcycle/verify_alien10.py` → `domains/infra/motorcycle/verify.hexa` |

### quantum-computer (1/1)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/quantum-computer/goal.md` → `domains/physics/quantum-computer/goal.md` |

### robotics (4/8)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/robotics/goal.md` → `domains/infra/robotics/goal.md` |
| MISS | `docs/paper/n6-autonomous-driving-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-control-automation-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-robotics-transport-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/robotics/verify_alien10.py` → `domains/infra/robotics/verify.hexa` |
| MISS | `docs/robotics/full-verification-matrix.md` → (unresolved) |
| VERIFY_HEXA | `docs/robotics/verify_alien10.py` → `domains/infra/robotics/verify.hexa` |
| VERIFY_HEXA | `docs/robotics/verify_alien10.py` → `domains/infra/robotics/verify.hexa` |

### safety (4/7)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/safety/goal.md` → `domains/infra/safety/goal.md` |
| MISS | `docs/paper/n6-governance-safety-urban-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-ultimate-safety-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/safety/verify_alien10.py` → `domains/infra/safety/verify.hexa` |
| MISS | `docs/safety/hypotheses.md` → (unresolved) |
| VERIFY_HEXA | `docs/safety/verify_alien10.py` → `domains/infra/safety/verify.hexa` |
| VERIFY_HEXA | `docs/safety/verify_alien10.py` → `domains/infra/safety/verify.hexa` |

### sf (1/2)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/sf/goal.md` → `domains/sf-ufo/sf/goal.md` |
| MISS | `docs/paper/n6-hexa-ufo-paper.md` → (unresolved) |

### software (11/17)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/programming-language/goal.md` → `domains/compute/programming-language/goal.md` |
| MISS | `docs/paper/n6-hexa-proglang-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-software-crypto-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/programming-language/verify_alien10.py` → `domains/compute/programming-language/verify.hexa` |
| MISS | `docs/software-design/full-verification-matrix.md` → (unresolved) |
| VERIFY_HEXA | `docs/programming-language/verify_alien10.py` → `domains/compute/programming-language/verify.hexa` |
| DOM | `docs/hexa-macos/goal.md` → `domains/compute/hexa-macos/goal.md` |
| MISS | `docs/paper/n6-hexa-macos-paper.md` → (unresolved) |
| EXP_HEXA | `experiments/verify_hexa_macos.py` → `experiments/anomaly/verify_hexa_macos.hexa` |
| DOM | `docs/hexa-ios/goal.md` → `domains/compute/hexa-ios/goal.md` |
| MISS | `docs/paper/n6-hexa-ios-paper.md` → (unresolved) |
| EXP_HEXA | `experiments/verify_hexa_ios.py` → `experiments/anomaly/verify_hexa_ios.hexa` |
| DOM | `docs/network-protocol/goal.md` → `domains/compute/network-protocol/goal.md` |
| MISS | `docs/paper/n6-hexa-netproto-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/programming-language/verify_alien10.py` → `domains/compute/programming-language/verify.hexa` |
| VERIFY_HEXA | `docs/software-design/verify_alien10.py` → `domains/compute/software-design/verify.hexa` |
| FALLBACK_HEXA | `verify_hexa_macos.py` → `experiments/anomaly/verify_hexa_macos.hexa` |

### tattoo-removal (1/1)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/tattoo-removal/goal.md` → `domains/life/tattoo-removal/goal.md` |

### tech-industry (30/50)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/advanced-packaging/goal.md` → `domains/compute/advanced-packaging/goal.md` |
| MISS | `docs/paper/n6-advanced-packaging-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-ecology-agriculture-food-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-manufacturing-quality-paper.md` → (unresolved) |
| EXIST | `domains/life/synbio/synbio.md` → `domains/life/synbio/synbio.md` |
| EXIST | `domains/life/synbio/goal.md` → `domains/life/synbio/goal.md` |
| EXIST | `papers/n6-synthetic-biology-paper.md` → `papers/n6-synthetic-biology-paper.md` |
| DOM | `docs/ar-vr-xr/goal.md` → `domains/culture/ar-vr-xr/goal.md` |
| MISS | `docs/paper/n6-ar-vr-xr-paper.md` → (unresolved) |
| DOM | `docs/digital-twin/goal.md` → `domains/compute/digital-twin/goal.md` |
| MISS | `docs/paper/n6-cognitive-social-psychology-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-digital-twin-paper.md` → (unresolved) |
| DOM | `docs/construction-structural/goal.md` → `domains/infra/construction-structural/goal.md` |
| MISS | `docs/paper/n6-construction-structural-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/construction-structural/verify_alien10_hex.py` → `domains/infra/construction-structural/verify.hexa` |
| DOM | `docs/underground-tunnel/goal.md` → `domains/infra/underground-tunnel/goal.md` |
| MISS | `docs/paper/n6-underground-tunnel-paper.md` → (unresolved) |
| DOM | `docs/ecommerce-fintech/goal.md` → `domains/infra/ecommerce-fintech/goal.md` |
| MISS | `docs/paper/n6-ecommerce-fintech-paper.md` → (unresolved) |
| MISS | `docs/paper/n6-economics-finance-paper.md` → (unresolved) |
| DOM | `docs/nylon/goal.md` → `domains/materials/nylon/goal.md` |
| MISS | `calc/kolon_n6_breakthrough.py` → (unresolved) |
| DOM | `docs/aramid/goal.md` → `domains/materials/aramid/goal.md` |
| MISS | `calc/kolon_n6_breakthrough.py` → (unresolved) |
| DOM | `docs/tire-cord/goal.md` → `domains/materials/tire-cord/goal.md` |
| MISS | `calc/kolon_n6_breakthrough.py` → (unresolved) |
| DOM | `docs/epoxy/goal.md` → `domains/materials/epoxy/goal.md` |
| MISS | `calc/kolon_n6_breakthrough.py` → (unresolved) |
| DOM | `docs/pet-film/goal.md` → `domains/materials/pet-film/goal.md` |
| MISS | `calc/kolon_n6_breakthrough.py` → (unresolved) |
| DOM | `docs/airbag/goal.md` → `domains/infra/airbag/goal.md` |
| MISS | `calc/kolon_n6_breakthrough.py` → (unresolved) |
| DOM | `docs/water-treatment/goal.md` → `domains/infra/water-treatment/goal.md` |
| MISS | `calc/kolon_n6_phase2.py` → (unresolved) |
| DOM | `docs/pemfc/goal.md` → `domains/energy/pemfc/goal.md` |
| MISS | `calc/kolon_n6_phase2.py` → (unresolved) |
| DOM | `docs/concrete/goal.md` → `domains/materials/concrete/goal.md` |
| MISS | `calc/kolon_n6_phase2.py` → (unresolved) |
| DOM | `docs/bio-pharma/goal.md` → `domains/life/bio-pharma/goal.md` |
| MISS | `calc/kolon_n6_phase2.py` → (unresolved) |
| VERIFY_HEXA | `docs/hvac-system/verify_alien10.py` → `domains/energy/hvac-system/verify.hexa` |
| VERIFY_HEXA | `docs/hvac-system/verify_alien10.py` → `domains/energy/hvac-system/verify.hexa` |
| VERIFY_HEXA | `docs/earthquake-engineering/verify_alien10.py` → `domains/infra/earthquake-engineering/verify.hexa` |
| VERIFY_HEXA | `docs/earthquake-engineering/verify_alien10.py` → `domains/infra/earthquake-engineering/verify.hexa` |
| VERIFY_HEXA | `docs/concrete-technology/verify_alien10.py` → `domains/materials/concrete-technology/verify.hexa` |
| VERIFY_HEXA | `docs/concrete-technology/verify_alien10.py` → `domains/materials/concrete-technology/verify.hexa` |
| VERIFY_HEXA | `docs/smart-city/verify_alien10.py` → `domains/infra/smart-city/verify.hexa` |
| VERIFY_HEXA | `docs/smart-city/verify_alien10.py` → `domains/infra/smart-city/verify.hexa` |
| VERIFY_HEXA | `docs/civil-engineering/verify_alien10.py` → `domains/infra/civil-engineering/verify.hexa` |
| VERIFY_HEXA | `docs/civil-engineering/verify_alien10.py` → `domains/infra/civil-engineering/verify.hexa` |

### virology (4/6)

| Status | Old path -> New path |
|---|---|
| DOM | `docs/virology/goal.md` → `domains/life/virology/goal.md` |
| MISS | `docs/paper/n6-virology-paper.md` → (unresolved) |
| VERIFY_HEXA | `docs/virology/verify_alien10.py` → `domains/life/virology/verify.hexa` |
| DOM | `docs/virology/goal.md` → `domains/life/virology/goal.md` |
| DOM | `docs/virology/goal.md` → `domains/life/virology/goal.md` |
| MISS | `docs/virology/evolution/mk-1-current.md` → (unresolved) |

## Remaining MISS List (174)

| Section | kind | Old path |
|---|---|---|
| fusion | link | `docs/paper/n6-fusion-powerplant-paper.md` |
| fusion | link | `docs/paper/n6-plasma-fusion-deep-paper.md` |
| fusion | link | `docs/fusion/evolution/mk-1-first-light.md` |
| fusion | link | `docs/fusion/alien-level-discoveries.md` |
| fusion | link | `docs/fusion/physical-limit-proof.md` |
| chip | link | `docs/paper/n6-dram-paper.md` |
| chip | link | `docs/paper/n6-exynos-paper.md` |
| chip | link | `docs/paper/n6-hexa-3d-paper.md` |
| chip | link | `docs/paper/n6-hexa-photon-paper.md` |
| chip | link | `docs/paper/n6-hexa-pim-paper.md` |
| chip | link | `docs/paper/n6-hexa-wafer-paper.md` |
| chip | link | `docs/paper/n6-performance-chip-paper.md` |
| chip | link | `docs/paper/n6-unified-soc-paper.md` |
| chip | link | `docs/paper/n6-vnand-paper.md` |
| chip | link | `docs/chip-architecture/ultimate-consciousness-soc.md` |
| chip | link | `docs/paper/n6-anima-soc-paper.md` |
| chip | link | `docs/paper/n6-consciousness-soc-paper.md` |
| chip | link | `docs/chip-architecture/hexa-topological-performance-chip.md` |
| chip | link | `docs/paper/n6-hexa-topo-paper.md` |
| chip | link | `docs/chip-architecture/hexa-asic-skywater.md` |
| chip | link | `docs/paper/n6-hexa-asic-paper.md` |
| chip | link | `docs/chip-architecture/full-verification-matrix.md` |
| ai | link | `docs/ai-efficiency/techniques-complete.md` |
| ai | link | `docs/paper/n6-causal-chain-paper.md` |
| ai | link | `docs/paper/n6-reality-map-paper.md` |
| ai | link | `docs/paper/n6-rtsc-12-products-evolution-paper.md` |
| ai | link | `docs/ai-efficiency/full-verification-matrix.md` |
| ai | link | `docs/ai-efficiency/next-model-blowup-2026-04.md` |
| ai | link | `docs/ai-efficiency/bt-391-code-generation.md` |
| ai | link | `docs/ai-efficiency/bt-397-n6-novel-architectures.md` |
| energy | link | `docs/paper/n6-battery-energy-paper.md` |
| energy | link | `docs/paper/n6-energy-efficiency-paper.md` |
| energy | link | `docs/paper/n6-datacenter-reactor-paper.md` |
| energy | link | `docs/battery-architecture/hexa-auto-battery.md` |
| environment | link | `docs/environmental-protection/` |
| environment | link | `docs/paper/n6-environment-thermal-paper.md` |
| environment | link | `docs/environmental-protection/microplastics-solution.md` |
| environment | link | `docs/paper/n6-microplastics-paper.md` |
| environment | link | `docs/paper/n6-carbon-capture-paper.md` |
| environment | link | `docs/environmental-protection/evolution/` |
| environment | link | `docs/environmental-protection/testable-predictions-2030.md` |
| environment | link | `docs/paper/n6-hexa-recycle-paper.md` |
| materials | link | `docs/paper/n6-crystallography-materials-paper.md` |
| materials | link | `docs/paper/n6-material-synthesis-paper.md` |
| materials | link | `docs/material-synthesis/breakthrough-theorems.md` |
| materials | link | `docs/material-synthesis/hypotheses.md` |
| materials | link | `docs/material-synthesis/industrial-validation.md` |
| materials | link | `docs/material-synthesis/experimental-verification.md` |
| materials | link | `docs/material-synthesis/physical-limit-proof.md` |
| robotics | link | `docs/paper/n6-autonomous-driving-paper.md` |
| robotics | link | `docs/paper/n6-control-automation-paper.md` |
| robotics | link | `docs/paper/n6-robotics-transport-paper.md` |
| robotics | link | `docs/robotics/full-verification-matrix.md` |
| physics | link | `docs/paper/n6-superconductor-paper.md` |
| physics | link | `docs/paper/n6-pure-mathematics-paper.md` |
| physics | link | `docs/paper/n6-classical-mechanics-accelerator-paper.md` |
| physics | link | `docs/paper/n6-particle-cosmology-paper.md` |
| physics | link | `docs/paper/n6-thermodynamics-paper.md` |
| physics | link | `docs/paper/n6-hexa-super-paper.md` |
| physics | link | `docs/paper/n6-quantum-computing-paper.md` |
| software | link | `docs/paper/n6-hexa-proglang-paper.md` |
| software | link | `docs/paper/n6-software-crypto-paper.md` |
| software | link | `docs/software-design/full-verification-matrix.md` |
| software | link | `docs/paper/n6-hexa-macos-paper.md` |
| software | link | `docs/paper/n6-hexa-ios-paper.md` |
| software | link | `docs/paper/n6-hexa-netproto-paper.md` |
| display | link | `docs/paper/n6-display-8stack-paper.md` |
| display | link | `docs/display/full-verification-matrix.md` |
| audio | link | `docs/paper/n6-isocell-comms-paper.md` |
| audio | link | `docs/paper/n6-telecom-linguistics-paper.md` |
| audio | link | `docs/audio/full-verification-matrix.md` |
| audio | link | `docs/paper/n6-hexa-speak-paper.md` |
| audio | link | `docs/audio/hexa-ear-ultimate.md` |
| audio | link | `docs/paper/n6-hexa-ear-paper.md` |
| audio | link | `docs/audio/hexa-bone-ultimate.md` |
| audio | link | `docs/audio/hexa-ear-cell.md` |
| audio | link | `docs/audio/hexa-speaker-ultimate.md` |
| safety | link | `docs/paper/n6-governance-safety-urban-paper.md` |
| safety | link | `docs/paper/n6-ultimate-safety-paper.md` |
| safety | link | `docs/safety/hypotheses.md` |
| play | link | `docs/paper/n6-fun-car-paper.md` |
| play | link | `docs/paper/n6-games-sports-paper.md` |
| play | link | `docs/paper/n6-motorcycle-paper.md` |
| aerospace | link | `docs/paper/n6-aerospace-transport-paper.md` |
| aerospace | link | `docs/paper/n6-hexa-starship-paper.md` |
| aerospace | link | `docs/paper/n6-space-systems-paper.md` |
| sf | link | `docs/paper/n6-hexa-ufo-paper.md` |
| frontier | link | `docs/paper/n6-biology-medical-paper.md` |
| frontier | link | `docs/paper/n6-hexa-neuro-paper.md` |
| frontier | link | `docs/paper/n6-hexa-grav-paper.md` |
| frontier | link | `docs/paper/n6-hexa-cloak-paper.md` |
| frontier | link | `docs/paper/n6-hexa-defense-paper.md` |
| frontier | link | `docs/paper/n6-hexa-teleport-paper.md` |
| frontier | link | `docs/paper/n6-hexa-hover-paper.md` |
| frontier | link | `docs/paper/n6-hexa-mram-paper.md` |
| frontier | link | `docs/paper/n6-seabed-grid-paper.md` |
| frontier | link | `docs/paper/n6-hexa-accel-paper.md` |
| frontier | link | `docs/paper/n6-hexa-weather-paper.md` |
| frontier | link | `docs/paper/n6-hexa-mind-paper.md` |
| frontier | link | `docs/paper/n6-hexa-telepathy-paper.md` |
| frontier | link | `docs/paper/n6-hexa-holo-paper.md` |
| frontier | link | `docs/paper/n6-hexa-dream-paper.md` |
| frontier | link | `docs/paper/n6-hexa-skyway-paper.md` |
| frontier | link | `docs/paper/n6-hexa-tsunami-paper.md` |
| frontier | link | `docs/paper/n6-antimatter-factory-paper.md` |
| frontier | link | `docs/paper/n6-hexa-cosmic-paper.md` |
| frontier | link | `docs/paper/n6-desal-paper.md` |
| frontier | link | `docs/paper/n6-hexa-oracle-paper.md` |
| frontier | link | `docs/paper/n6-hexa-one-paper.md` |
| frontier | link | `docs/paper/n6-hexa-glass-paper.md` |
| frontier | link | `docs/paper/n6-hexa-exo-paper.md` |
| frontier | link | `docs/paper/n6-hexa-limb-paper.md` |
| frontier | link | `docs/paper/n6-hexa-skin-paper.md` |
| frontier | link | `docs/paper/n6-hexa-fabric-paper.md` |
| frontier | link | `docs/paper/n6-hexa-olfact-paper.md` |
| frontier | link | `docs/paper/n6-hexa-empath-paper.md` |
| frontier | link | `docs/paper/n6-entomology-paper.md` |
| frontier | link | `docs/mycology/hypotheses.md` |
| frontier | link | `docs/mining/hypotheses.md` |
| frontier | link | `docs/veterinary/hypotheses.md` |
| frontier | link | `docs/horticulture/hypotheses.md` |
| frontier | link | `docs/paper/n6-hexa-sim-paper.md` |
| frontier | link | `docs/paper/n6-therapeutic-nanobot-paper.md` |
| civilization | link | `docs/paper/n6-religion-mythology-paper.md` |
| civilization | link | `docs/paper/n6-jurisprudence-paper.md` |
| civilization | link | `docs/paper/n6-writing-systems-paper.md` |
| civilization | link | `docs/paper/n6-archaeology-paper.md` |
| civilization | link | `docs/paper/n6-monetary-history-paper.md` |
| civilization | link | `docs/paper/n6-dance-choreography-paper.md` |
| civilization | link | `docs/horology/hypotheses.md` |
| civilization | link | `docs/paper/n6-calendar-time-geography-paper.md` |
| civilization | link | `docs/paper/n6-horology-paper.md` |
| life-culture | link | `docs/paper/n6-fermentation-paper.md` |
| life-culture | link | `docs/paper/n6-wine-enology-paper.md` |
| life-culture | link | `docs/paper/n6-fashion-textile-paper.md` |
| life-culture | link | `docs/paper/n6-aquaculture-paper.md` |
| life-culture | link | `docs/paper/n6-insurance-paper.md` |
| life-culture | link | `docs/dolphin/hypotheses.md` |
| life-culture | link | `docs/paper/n6-dolphin-bioacoustics-paper.md` |
| life-culture | link | `docs/coffee-science/hypotheses.md` |
| life-culture | link | `docs/perfumery/hypotheses.md` |
| life-culture | link | `docs/ceramics/hypotheses.md` |
| tech-industry | link | `docs/paper/n6-advanced-packaging-paper.md` |
| tech-industry | link | `docs/paper/n6-ecology-agriculture-food-paper.md` |
| tech-industry | link | `docs/paper/n6-manufacturing-quality-paper.md` |
| tech-industry | link | `docs/paper/n6-ar-vr-xr-paper.md` |
| tech-industry | link | `docs/paper/n6-cognitive-social-psychology-paper.md` |
| tech-industry | link | `docs/paper/n6-digital-twin-paper.md` |
| tech-industry | link | `docs/paper/n6-construction-structural-paper.md` |
| tech-industry | link | `docs/paper/n6-underground-tunnel-paper.md` |
| tech-industry | link | `docs/paper/n6-ecommerce-fintech-paper.md` |
| tech-industry | link | `docs/paper/n6-economics-finance-paper.md` |
| tech-industry | verify_script | `calc/kolon_n6_breakthrough.py` |
| tech-industry | verify_script | `calc/kolon_n6_breakthrough.py` |
| tech-industry | verify_script | `calc/kolon_n6_breakthrough.py` |
| tech-industry | verify_script | `calc/kolon_n6_breakthrough.py` |
| tech-industry | verify_script | `calc/kolon_n6_breakthrough.py` |
| tech-industry | verify_script | `calc/kolon_n6_breakthrough.py` |
| tech-industry | verify_script | `calc/kolon_n6_phase2.py` |
| tech-industry | verify_script | `calc/kolon_n6_phase2.py` |
| tech-industry | verify_script | `calc/kolon_n6_phase2.py` |
| tech-industry | verify_script | `calc/kolon_n6_phase2.py` |
| virology | link | `docs/paper/n6-virology-paper.md` |
| virology | link | `docs/virology/evolution/mk-1-current.md` |
| hiv-treatment | link | `docs/paper/n6-hiv-paper.md` |
| hiv-treatment | link | `docs/hiv-treatment/evolution/mk-1-basic.md` |
| hiv-treatment | link | `docs/hiv-treatment/evolution/mk-2-short.md` |
| hiv-treatment | link | `docs/hiv-treatment/evolution/mk-3-mid.md` |
| hiv-treatment | link | `docs/hiv-treatment/evolution/mk-4-long.md` |
| hiv-treatment | link | `docs/hiv-treatment/evolution/mk-5-ultimate.md` |
| cognitive-social | link | `docs/paper/n6-consciousness-chip-paper.md` |
| hygiene | link | `docs/mens-intimate-cleanser/breakthrough.md` |
| hygiene | link | `docs/womens-intimate-cleanser/breakthrough.md` |
| manufacturing-quality | link | `docs/paper/n6-manufacturing-quality-paper.md` |
