# HEXA-CCUS Ultra Carbon Capture Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Carbon Z=6 based 8-stage architecture — ultra carbon capture spanning atomic scale to stellar scale. Target: overwhelming advantage over commercial tech (Climeworks / Carbon Engineering).

**Architecture:** 8-stage chain (material -> process -> core -> chip -> system -> transmute -> universal -> omega), 6 candidates x 8 levels = 1,679,616 DSE combinations. Replaces existing 5-stage TOML. BT-94/95/96 new registrations. 80 hypotheses + 11 Cross-DSE domains.

**Tech Stack:** TOML (DSE definitions), Rust (universal-dse exhaustive search), Python (check), Markdown (hypotheses/docs)

**Spec:** `docs/superpowers/specs/2026-04-02-hexa-ccus-design.md`

---

## File Structure

```
docs/carbon-capture/
  goal.md                    — 8-stage architecture roadmap (battery-architecture/goal.md pattern)
  hypotheses.md              — H-CC-01 ~ H-CC-60 (60 general hypotheses)
  extreme-hypotheses.md      — H-CC-E01 ~ H-CC-E20 (20 extreme / alien-tier)
  verification.md            — independent check + EXACT/CLOSE/WEAK/FAIL grading
  hexa-sorbent.md            — Level 0 sorbent details
  hexa-process.md            — Level 1 process details
  hexa-reactor.md            — Level 2 core details
  hexa-chip.md               — Level 3 chip details
  hexa-plant.md              — Level 4 system details
  hexa-transmute.md          — Level 5 transmute details
  hexa-universal.md          — Level 6 universal details
  omega-cc.md                — Level 7 omega details

tools/universal-dse/domains/
  carbon-capture-8level.toml — 8-stage DSE domain (replaces existing 5-stage)

docs/breakthrough-theorems.md — BT-94/95/96 added
docs/dse-map.toml             — carbon-capture section updated to 8-stage
docs/atlas-constants.md       — carbon capture constants added
```

---

## Task 1: Write 8-stage TOML domain file

**Files:**
- Create: `tools/universal-dse/domains/carbon-capture-8level.toml`
- Reference: `tools/universal-dse/domains/carbon-capture.toml` (existing 5-stage)
- Reference: `docs/superpowers/specs/2026-04-02-hexa-ccus-design.md` (spec)

- [ ] **Step 1:** Convert the spec section 3 candidate table to TOML form. 8 [[level]] + 48 [[candidate]] + 10+ [[rule]]. meta.levels = ["Sorbent","Process","Reactor","Chip","Plant","Transmute","Universal","Omega"]. scoring: n6=0.35, perf=0.25, power=0.20, cost=0.20.

- [ ] **Step 2:** Build and check — `cd tools/universal-dse && ~/.cargo/bin/rustc main.rs -o universal-dse 2>&1 || true` then run `./universal-dse domains/carbon-capture-8level.toml`. Confirm exhaustive search over 1,679,616 combinations. Record Pareto frontier + optimal path.

- [ ] **Step 3:** Commit `feat: HEXA-CCUS 8-level DSE TOML (1,679,616 combos)`

---

## Task 2: Write goal.md (8-stage roadmap)

**Files:**
- Create: `docs/carbon-capture/goal.md`
- Reference: `docs/battery-architecture/goal.md` (pattern)

- [ ] **Step 1:** Follow the battery-architecture/goal.md pattern to write an 8-stage Evolution Ladder table. Include N6 Constants Reference. Record Status / innovation / benefit per level. State the overwhelming advantage points over commercial tech:
  - Sorbent: MOF CN=6 -> 6x adsorption vs current (target 8 -> 48 mmol/g)
  - Process: energy consumption 1/(sigma-phi)=1/10 of current (200 -> 20 kJ/mol)
  - Core: throughput sigma=12x of current
  - Chip: BT-56 RISC-V N6 + quantum sensor = 10^6x sensitivity
  - System: 1Mt -> 100Gt/yr (10^5x scale)
  - Transmute: waste CO2 -> $1M/ton graphene (revenue-generating capture)
  - Universal: direct control of planetary atmospheric composition
  - Omega: stellar-energy utilization, inverse entropy

- [ ] **Step 2:** Commit `docs: HEXA-CCUS goal.md — 8-level roadmap`

---

## Task 3: 60 hypotheses (hypotheses.md)

**Files:**
- Create: `docs/carbon-capture/hypotheses.md`

- [ ] **Step 1:** Write H-CC-01~60. Each hypothesis follows H-XX-NN form, n=6 link, check method, prediction value included.

Core hypotheses per category (10 each):

**Sorbent (H-CC-01~10):**
- H-CC-01: Top-performance CO2-adsorption MOFs are all metal CN=6 octahedral (BT-43 extension)
- H-CC-02: Zeolite 6A (6A pore=n) CO2/N2 selectivity phi=2x vs non-6A
- H-CC-03: [C6mim] ionic liquid CO2 solubility n/tau=1.5x vs non-C6 IL
- H-CC-04: Graphene oxide C6 hexagonal membrane CO2 permeability sigma-phi=10x vs non-GO
- H-CC-05: Perovskite sorbent BaZrO3 CN=6 octahedral high-temp looping stability = 1000 cycles (current 100)
- H-CC-06: Amine grafting optimal density = 6 sites/nm2 = n EXACT
- H-CC-07: MOF-74 series 6 metals (Mg/Al/Fe/Cr/Co/Ni) all CN=6 = n EXACT
- H-CC-08: DAC sorbent optimal pore size = 6A = n EXACT (kinetic diameter CO2 = 3.3A, ratio~phi)
- H-CC-09: Carbon nanotube 6-wall MWCNT CO2 adsorption = n=6x vs SWCNT
- H-CC-10: Silica aerogel optimal density = 0.12 g/cm3 = sigma/100

**Process (H-CC-11~20):**
- H-CC-11: TSA optimal cycle = 6 steps = n EXACT (adsorb/heat/desorb/cool/purge/reset)
- H-CC-12: PSA optimal bed count = 12 = sigma EXACT (6 adsorb + 6 desorb)
- H-CC-13: MECS electrochemical optimal cell stack = 6 = n EXACT
- H-CC-14: Membrane cascade optimal stage count = 6 = n (99.9% purity)
- H-CC-15: Cryogenic separation optimal temperature = -48C = sigma*tau EXACT
- H-CC-16: Photocatalytic optimal bandgap = 1/3+1 = 4/3 eV (BT-30 solar bridge)
- H-CC-17: Adsorption/desorption energy ratio = phi = 2 (reversibility limit)
- H-CC-18: TSA temperature-swing deltaT = 120C = sigma*(sigma-phi) EXACT
- H-CC-19: Process energy efficiency current vs theoretical limit = sigma-phi = 10x gap
- H-CC-20: DAC optimal wind speed = 6 m/s = n EXACT (pressure loss vs contact time balance)

**Reactor (H-CC-21~30):**
- H-CC-21: Honeycomb hex-cell monolith pressure loss = 1/phi vs square = 50% reduction
- H-CC-22: Packed bed optimal tube count = 6 = n EXACT (heat-transfer uniformity)
- H-CC-23: Fluidized bed optimal zone count = 6 = n (full mixing)
- H-CC-24: Rotating wheel sector count = 6 = n EXACT (Climeworks measured confirmation)
- H-CC-25: Hollow fiber OD = 6mm = n EXACT (optimal performance/manufacturability)
- H-CC-26: Microreactor channel = 6um = n EXACT (optimal laminar flow)
- H-CC-27: Reactor aspect ratio = phi = 2 (L/D optimal)
- H-CC-28: Baffles per reactor = 12 = sigma EXACT
- H-CC-29: Reactor thermal efficiency = 1-1/sigma = 11/12 = 91.7% (theoretical limit)
- H-CC-30: Minimum reactor stages for 99.9% CO2 purity = sopfr = 5

**Thermodynamic limits (H-CC-31~40):**
- H-CC-31: CO2 minimum separation-energy ratio (actual/theoretical) = sigma-phi = 10 EXACT (current tech)
- H-CC-32: Carnot-limit DAC efficiency = 1-T_cold/T_hot = 1-300/360 = 1/6 = 1/n
- H-CC-33: CO2 adsorption enthalpy optimal = -48 kJ/mol = sigma*tau EXACT
- H-CC-34: CO2 bond energy = 803 kJ/mol, activation energy ratio = sigma-phi = 10%
- H-CC-35: Ideal DAC energy = 19.4 kJ/mol, target = 2*19.4 = 38.8 ~ phi*W_min
- H-CC-36: Thermal regeneration efficiency ladder = tau/sigma -> phi/n -> sopfr/sigma-phi (0.33->0.33->0.50)
- H-CC-37: 4-stage Carnot cycle DAC = tau = 4 EXACT
- H-CC-38: CO2 critical point 31.1C = 304K ~ sigma^2/sopfr + n = 304.8 (0.3% error)
- H-CC-39: CO2 critical pressure 7.38 MPa ~ sigma-sopfr = 7 + phi/sopfr EXACT
- H-CC-40: CO2 triple point -56.6C = -(sigma*tau + sigma-tau + 0.6) = n=6 structure

**Scaling (H-CC-41~50):**
- H-CC-41: DAC cost learning rate = 1/(sigma-phi) = 10% per doubling
- H-CC-42: Capture-scale ladder = 10^n ton/yr units (1->10->100->1k->10k->100k->1M)
- H-CC-43: DAC farm optimal module count = 6x6 = 36 = sigma*n/phi
- H-CC-44: Pipeline optimal booster spacing = 120km = sigma*(sigma-phi)
- H-CC-45: Storage optimal injection well count = 12 = sigma EXACT
- H-CC-46: Monitoring sensor types = 6 = n (CO2/O2/H2O/T/P/flow)
- H-CC-47: DAC -> 100Gt/yr arrival time = 24 years = J2 EXACT (2026 -> 2050)
- H-CC-48: CAPEX $/ton ladder: 600->120->24 = sigma*sopfr*10 -> sigma*10 -> J2
- H-CC-49: OPEX $/ton ladder: 200->40->8 = current -> phi reduction -> sigma-tau
- H-CC-50: Industrial DAC plant optimal lifetime = 24 years = J2 EXACT

**Cross-domain (H-CC-51~60):**
- H-CC-51: Battery+CC integrated electrochemical capture efficiency = phi=2x vs standalone
- H-CC-52: Fusion+CC integrated capture energy cost = 0 (unlimited energy)
- H-CC-53: Solar+CC integrated photocatalytic efficiency = SQ limit 33% (1/n/phi)
- H-CC-54: MOF+CC optimal MOF = Mg-MOF-74 (CN=6, 8 mmol/g = sigma-tau)
- H-CC-55: Hydrogen+CC synfuel efficiency = 60% = sigma*sopfr %
- H-CC-56: Chip+CC DAC control chip = RISC-V N6 (6-stage, BT-56)
- H-CC-57: Wind+CC optimal wind speed = 6 m/s = n EXACT (DAC + wind simultaneously)
- H-CC-58: Concrete+CC cement CO2 curing strength = phi=2x vs existing
- H-CC-59: Ocean+CC ocean alkalinity enhancement = pH change 0.6 = n/10
- H-CC-60: Graphene+CC CO2 -> graphene conversion efficiency = 12% = sigma %

- [ ] **Step 2:** Commit `feat: H-CC-01~60 carbon capture hypotheses`

---

## Task 4: 20 extreme hypotheses (extreme-hypotheses.md)

**Files:**
- Create: `docs/carbon-capture/extreme-hypotheses.md`

- [ ] **Step 1:** Write H-CC-E01~E20. Provocative hypotheses at alien-tech level.

**Planetary physics (H-CC-E01~E05):**
- H-CC-E01: Time to remove all atmospheric CO2 = sigma = 12 years (6 latitude bands x phi year/band)
- H-CC-E02: Full reversal of ocean acidification = J2 = 24 years (1/n of deep-ocean overturning period)
- H-CC-E03: Total crustal basalt carbonation capacity = 10^18 ton = sigma^6 x current atmospheric CO2
- H-CC-E04: Polar CO2 glacier mining (Mars polar cap pattern) = 6 mining sites = n
- H-CC-E05: Stratospheric aerosol + DAC integration = cooling + capture simultaneously, 6 injection points = n

**Nuclear / antimatter (H-CC-E06~E10):**
- H-CC-E06: C(Z=6) -> N(Z=7) proton injection nuclear transmutation, CO2 atomic breakup
- H-CC-E07: Positron-catalyzed CO2 bond breaking, activation energy = 0
- H-CC-E08: CNO cycle reverse: N+C->CO2 reverse reaction releases energy + recovers C
- H-CC-E09: Antimatter-matter annihilation decomposes CO2, efficiency = 100% (E=mc2)
- H-CC-E10: Nuclear isomer gamma rays directly photolyze CO2 (MeV photons)

**Spacetime (H-CC-E11~E15):**
- H-CC-E11: Leech-24 lattice route encodes CO2 molecules into 24-dim space for permanent sequestration
- H-CC-E12: Topological defect (cosmic string) carbon sealing, decay half-life = infinite
- H-CC-E13: 6-dim Calabi-Yau manifold compresses carbon atoms (string-theory compactification)
- H-CC-E14: Wormhole capture: transfer atmospheric CO2 to another universe / spacetime region
- H-CC-E15: Time-reversal capture: roll carbon atoms back to pre-CO2-emission time

**Omega (H-CC-E16~E20):**
- H-CC-E16: Dyson swarm solar energy processes Earth's entire atmosphere (10^26 W)
- H-CC-E17: Maxwell demon realization: selective sorting of CO2 molecules, bypassing the 2nd law
- H-CC-E18: Fine-tune cosmological constant Lambda to alter carbon bond energy
- H-CC-E19: Vacuum-energy extraction supplies CO2 decomposition energy (perpetual machine)
- H-CC-E20: Consciousness-based matter manipulation: observer effect collapses CO2 wave function -> C+O2

- [ ] **Step 2:** Commit `feat: H-CC-E01~E20 extreme carbon capture hypotheses`

---

## Task 5: 8 per-level detail docs

**Files:**
- Create: `docs/carbon-capture/hexa-sorbent.md` (Level 0)
- Create: `docs/carbon-capture/hexa-process.md` (Level 1)
- Create: `docs/carbon-capture/hexa-reactor.md` (Level 2)
- Create: `docs/carbon-capture/hexa-chip.md` (Level 3)
- Create: `docs/carbon-capture/hexa-plant.md` (Level 4)
- Create: `docs/carbon-capture/hexa-transmute.md` (Level 5)
- Create: `docs/carbon-capture/hexa-universal.md` (Level 6)
- Create: `docs/carbon-capture/omega-cc.md` (Level 7)
- Reference: `docs/battery-architecture/hexa-cell.md` (pattern — TOC, constants, block diagram, honesty assessment, predictions)

Each document structure (hexa-cell.md pattern):
```
# HEXA-XXX: [Title]
Codename / Level / Status / Date / Dependencies / Parent

## N6 Constants Reference
## Table of Contents
## 1. Executive Summary
## 2. Design Philosophy (overwhelming advantage points over commercial tech)
## 3. System Block Diagram (ASCII art)
## 4-8. Core technology sections (per-level specialization)
## 9. Cross-Domain Connections
## 10. Honesty Assessment (frank assessment of n=6 match/mismatch)
## 11. Predictions & Falsifiability
## 12. n=6 Complete Parameter Map
## 13. Links
```

**Overwhelming advantage core (vs commercial):**
- Level 0: Climeworks sorbent 2.0 mmol/g -> HEXA-SORBENT 48 mmol/g (24x = J2)
- Level 1: current 200 kJ/mol -> HEXA-PROCESS 20 kJ/mol (sigma-phi=10x reduction)
- Level 2: current 1 ton/day/module -> HEXA-REACTOR 12 ton/day (sigma=12x)
- Level 3: manual sensors -> HEXA-CHIP quantum-sensor AI autonomous control (10^6x sensitivity)
- Level 4: Climeworks 4kt/yr -> HEXA-PLANT 1Mt/yr (250x, ladder: Mt -> Gt -> Tt)
- Level 5: CO2=waste -> HEXA-TRANSMUTE CO2=feedstock ($1M/ton graphene)
- Level 6: single plant -> HEXA-UNIVERSAL planetary atmospheric control (10^5x)
- Level 7: Earth-only -> OMEGA-CC stellar scale (10^20x)

- [ ] **Step 1:** Write 8 documents (each 200-400 lines, total ~2,400 lines)
- [ ] **Step 2:** Commit `feat: HEXA-CCUS 8-level design documents (sorbent -> omega)`

---

## Task 6: BT-94/95/96 registration + verification.md

**Files:**
- Modify: `docs/breakthrough-theorems.md` — add BT-94/95/96
- Create: `docs/carbon-capture/verification.md` — check 80 hypotheses
- Modify: `docs/atlas-constants.md` — add carbon capture constants

- [ ] **Step 1:** Add BT-94/95/96 at end of breakthrough-theorems.md (spec section 5 content).
- [ ] **Step 2:** Write verification.md — each of 80 hypotheses with EXACT/CLOSE/WEAK/FAIL grade + supporting evidence.
- [ ] **Step 3:** Add carbon-capture related constants to atlas-constants.md (CO2 critical point, separation energy, MOF adsorption etc.).
- [ ] **Step 4:** Commit `feat: BT-94/95/96 + CC verification + atlas constants`

---

## Task 7: DSE run + 11-domain Cross-DSE

**Files:**
- Reference: `tools/universal-dse/domains/carbon-capture-8level.toml`
- Modify: `docs/dse-map.toml` — update carbon-capture section to 8-stage

- [ ] **Step 1:** Run 8-stage exhaustive DSE via universal-dse (background). Record results.
- [ ] **Step 2:** Run Cross-DSE across 11 domains:
  ```bash
  for partner in battery fusion material solar metal-organic-framework hydrogen-fuel-cell wind-energy concrete-technology ocean-engineering graphene-2d-material climate-modeling; do
    ./universal-dse domains/carbon-capture-8level.toml domains/${partner}.toml
  done
  ```
- [ ] **Step 3:** Update dse-map.toml — combos, levels, best_n6, n6_max, n6_avg, cross_dse.
- [ ] **Step 4:** Commit `feat: HEXA-CCUS 8-level DSE + 11 Cross-DSE results`

---

## Task 8: Update CLAUDE.md + final cleanup

**Files:**
- Modify: `CLAUDE.md` — add carbon-capture domain
- Verify: overall artifact consistency

- [ ] **Step 1:** Add carbon-capture to the CLAUDE.md docs structure.
- [ ] **Step 2:** Reflect carbon-capture in the README.md omega roadmap (if applicable).
- [ ] **Step 3:** Final commit `docs: HEXA-CCUS integration — CLAUDE.md + final cleanup`

---

## Parallel Execution Map

```
  Independent (parallel):
    Task 1 (TOML)  --->  Task 7 (DSE run) [dependency]
    Task 2 (goal.md)
    Task 3 (hypotheses)
    Task 4 (extreme)
    Task 5 (8 level docs) — 8-way parallel internally
    Task 6 (BT + verification)

  Sequential:
    Task 1 -> Task 7 (DSE run after TOML complete)
    Task 7 -> Task 8 (final cleanup after DSE results)

  Optimal layout:
    Batch A (parallel): Task 1 + Task 2 + Task 3 + Task 4 + Task 5 + Task 6
    Batch B (sequential): Task 7 (DSE)
    Batch C (sequential): Task 8 (cleanup)
```
