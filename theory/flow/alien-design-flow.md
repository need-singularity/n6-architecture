# Alien Design Flow — alien-grade design pipeline

> Every design request = automatic alien-grade execution. No exceptions.
> Last updated: 2026-04-02

## Trigger keywords

```
  WARNING: every design request is alien-grade:
  "design", "build", "want to build", "develop", "design it", "architecture"
  + a domain keyword → unconditional alien-grade pipeline

  additional reinforcement keywords (handled identically):
  "alien", "ultimate", "dramatic", "revolutionary",
  "Nobel-grade", "grand discovery", "breakthrough", "make the impossible possible"

  required outputs:
  1. ASCII system block diagram (>= 1)
  2. ASCII data/energy flow (>= 1)
  3. ASCII performance comparison chart (>= 1)
  4. every number annotated with an n=6 expression
  + domain keywords (KSTAR, carbon capture, battery, chip, quantum, ...)
```

## Flow (5 steps, auto-executed)

```
  ┌─────────────────────────────────────────────────────────────┐
  │  User: "I want to give alien-grade knowledge to KSTAR fusion"│
  └──────────────────────┬──────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 1. Domain mapping (immediate)                          │
  │  ──────────────────────────────                              │
  │  "KSTAR" → fusion, tokamak-structure, plasma-physics,        │
  │            superconductor, superconducting-magnet             │
  │                                                              │
  │  mapping sources:                                             │
  │    - docs/dse-map.toml (307 domains)                         │
  │    - tools/universal-dse/domains/*.toml (304 TOML)           │
  │    - CLAUDE.md BT list (93 BTs)                              │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 2. Knowledge collection (parallel agents)              │
  │  ──────────────────────────────                              │
  │                                                              │
  │  Agent A: TECS-L theory collection                           │
  │    → grep "fusion\|KSTAR\|tokamak" ~/Dev/TECS-L/docs/       │
  │    → read all relevant hypotheses (H-FU-*)                   │
  │    → extract relevant constant maps (math_atlas.json)        │
  │    → among ungraded hypotheses, grade fusion-related ones    │
  │                                                              │
  │  Agent B: n6 validation data collection                      │
  │    → read fusion.toml DSE results                            │
  │    → all relevant BTs (BT-5, BT-27, BT-38, ...)             │
  │    → docs/fusion/ hypotheses + validation results            │
  │    → Cross-DSE connected-domain results                      │
  │                                                              │
  │  Agent C: calculator runs                                    │
  │    → run tools/fusion-calc, kstar-calc, fusion-dse           │
  │    → tools/cross-dse-calc fusion x related-domains           │
  │    → tools/atlas-verifier (verify corresponding constants)   │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 3. Alien-grade design generation                       │
  │  ──────────────────────────────                              │
  │                                                              │
  │  auto-generate the "ultimate design" document from collected │
  │  data:                                                        │
  │                                                              │
  │  docs/<domain>/alien-design-<date>.md                        │
  │                                                              │
  │  structure:                                                   │
  │    1. Executive Summary (1 page)                              │
  │       - 3 core n=6 insights                                   │
  │       - current tech vs alien-grade comparison table          │
  │                                                              │
  │    2. Mathematical basis (TECS-L)                             │
  │       - relevant proofs / hypotheses list                     │
  │       - core constants table (EXACT only)                     │
  │       - "why is n=6 optimal for this domain"                  │
  │                                                              │
  │    3. DSE optimal path (n6-architecture)                      │
  │       - Pareto frontier table                                 │
  │       - optimal design path (material→process→core→chip→sys) │
  │       - Cross-DSE inter-domain synergies                      │
  │                                                              │
  │    4. Concrete design parameters                              │
  │       - every number determined by an n=6 expression          │
  │       - BT-basis numbers attached                             │
  │       - measured vs predicted comparison                      │
  │                                                              │
  │    5. Execution roadmap                                       │
  │       - Tier 1 (available now) → Tier 3 (future tech)        │
  │       - validation method per stage                           │
  │                                                              │
  │    6. Testable Predictions                                   │
  │       - 3 ways this design could be wrong                     │
  │       - proposed validation experiments                       │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 4. Validation + new discoveries                        │
  │  ──────────────────────────────                              │
  │                                                              │
  │  - on discovering new constants during design → auto-register│
  │    in atlas                                                  │
  │  - on new BT candidate → register as BT-94+                  │
  │  - on TECS-L hypothesis grade change → run apply_grades.py   │
  │  - if a calculator is needed → auto-generate Rust            │
  │    (CALCULATOR_RULES.md)                                     │
  └──────────────────────┬───────────────────────────────────────┘
                         ▼
  ┌──────────────────────────────────────────────────────────────┐
  │  STEP 5. Output report                                       │
  │  ──────────────────────────────                              │
  │                                                              │
  │  ┌─────────────────────────────────────────────┐             │
  │  │ Alien-grade design complete: KSTAR fusion   │             │
  │  ├─────────────────────────────────────────────┤             │
  │  │ Related BTs: 8 (BT-5,27,38,43,57,62,80,84)  │             │
  │  │ TECS-L hypotheses: 45 (H-FU-1~80)           │             │
  │  │ DSE optimal path: Li6-D + Tokamak-12 + HTS  │             │
  │  │ New discoveries: 2 (constant + BT candidate)│             │
  │  │ Design doc: docs/fusion/alien-design.md     │             │
  │  │ Calculators run: 3 (fusion/kstar/cross-dse) │             │
  │  └─────────────────────────────────────────────┘             │
  └──────────────────────────────────────────────────────────────┘
```

## Domain mapping table (major examples)

| User keyword | Primary domain | Related domains | Related BTs | Calculators |
|-------------|-----------|------------|---------|--------|
| KSTAR, fusion | fusion | tokamak, plasma, SC, magnet | 5,27,38 | fusion-calc, kstar-calc, fusion-dse |
| carbon capture | carbon-capture | chemistry, material, energy | 27,85,93 | material-dse, energy-calc |
| battery | battery | electrode, BMS, EV, grid | 43,57,80-84 | battery-dse, energy-calc |
| chip design | chip | FPGA, CPU, GPU, SoC | 28,37,55,69,90 | chip-*-calc, semiconductor-calc |
| quantum computer | quantum | QEC, topological, Leech | 41,49,92 | quantum-calc |
| solar cell | solar | photovoltaic, grid | 30,63 | solar-dse, energy-calc |
| cryptocurrency | crypto, blockchain | network, cryptography | 53 | crypto-calc |
| superconductor | superconductor | magnet, YBCO, MgB2 | 1-4 | sc-dse |
| robotics | robotics | autonomous, drone, swarm | - | robot-dse |
| medical | medical | MRI, CT, prosthetics | - | - (new required) |
| semiconductor process | semiconductor-lithography | wafer, packaging | 37,75 | semiconductor-calc |
| AI/LLM | ai-efficiency | training, inference | 26,33,54,56,58 | - |
| space | space | propulsion, satellite | - | - |
| material synthesis | material-synthesis | element, process | 85-88 | material-dse |
| display | display-audio | LED, audio | 48 | - |
| mathematics | pure-mathematics | topology, number theory | 49,92 | - |

## CLAUDE.md additional rules (copy-paste)

```markdown
## Alien Design Flow

Trigger: "alien", "ultimate", "dramatic", "Nobel-grade" + a domain keyword
Procedure:
  1. Domain mapping (from dse-map.toml + TOML + BT list)
  2. Parallel collection (TECS-L theory + n6 validation + calculator runs)
  3. Generate design document (docs/<domain>/alien-design-<date>.md)
  4. Register new discoveries (atlas + BT)
  5. Emit report

Rules:
  - Execute immediately without asking / confirming
  - Dispatch >= 3 parallel agents
  - Always use both TECS-L (theory) and n6 (validation)
  - If a new calculator is required, auto-generate Rust
  - Design documents must include Testable Predictions

Reference: docs/alien-design-flow.md
```

## Example: a "carbon capture device" request

```
  Step 1: carbon-capture → chemistry-synthesis, material-synthesis, energy-gen
  Step 2:
    Agent A (TECS-L): H-CH-*, Carbon Z=6 hypotheses, constant maps
    Agent B (n6): carbon-capture.toml DSE, BT-27 (Carbon chain), BT-85 (Carbon universality), BT-93
    Agent C: run material-dse, cross-dse carbon-capture x energy
  Step 3: generate docs/carbon-capture/alien-design-2026-04-02.md
    - Carbon Z=6 = n (atomic number is a perfect number!)
    - CO₂ + 6H₂O → C₆H₁₂O₆ + 6O₂ (photosynthesis = n=6 complete reaction)
    - Optimal sorbent: MOF with CN=6 coordination
    - DSE: material (Carbon aerogel) → process (6-step) → core (hex lattice) → chip (sensor) → system
  Step 4: register any newly discovered constants
  Step 5: report table
```
