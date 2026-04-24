# CHIP-P1-4 — 6-Tool Bridge Integration Summary

P1 roadmap item CHIP-P1-4. Integrates 6 calculator/DSE tools into `bridge/_registry.json` so that they can be routed directly from the `nexus` CLI.

- Completed: 2026-04-14
- Principle: HEXA-ONLY (Rust reimplementation deferred; every tool delegates directly to the hexa binary)
- Integration SSOT: `bridge/_registry.json`
- Routing model: `nexus <category> <name>` → `$HEXA bridge/origins/<origin>/main.hexa`
- Parent rule: `bridge/CLAUDE.md` (HEXA-ONLY workspace)

## Integration Result Table

| Tool | Slot | Category | origin (hexa source) | Invocation | Status |
|------|------|----------|--------------------|------|------|
| crypto-calc | calc.crypto | Calculator | bridge/origins/crypto-calc/main.hexa | `nexus calc crypto` | integrated |
| interconnect-calc | calc.interconnect | Calculator | bridge/origins/interconnect-calc/main.hexa | `nexus calc interconnect` | integrated |
| lang-dse | dse.lang | DSE | bridge/origins/universal-dse/domains/programming-language.toml | `nexus dse lang` | integrated (new) |
| gpu-arch-calc | calc.gpu-arch | Calculator | bridge/origins/gpu-arch-calc/main.hexa | `nexus calc gpu-arch` | integrated |
| chip-n6-calc | calc.chip-n6 | Calculator | bridge/origins/chip-n6-calc/main.hexa | `nexus calc chip-n6` | integrated |
| semiconductor-calc | calc.semiconductor | Calculator | bridge/origins/semiconductor-calc/main.hexa | `nexus calc semiconductor` | integrated |

## Per-Tool Summary

### 1. crypto-calc — Cryptography / Hash Calculator
- Role: Parameter computation aligned with n=6 for cryptographic primitives such as hashes, signatures, KDFs, and symmetric/asymmetric round costs.
- Slot: `calc.crypto`
- Invocation: `nexus calc crypto`
- Status: Existing origin; added invocation/status metadata to the registry.

### 2. interconnect-calc — Interconnect Calculator
- Role: Trade-off analysis of bandwidth, latency, area, and power for on-chip / in-package / die-to-die interconnects.
- Slot: `calc.interconnect`
- Invocation: `nexus calc interconnect`
- Status: Existing origin; metadata enriched.

### 3. lang-dse — HEXA-LANG DSE
- Role: DSE for the ultimate programming language. Searches BT-50 IEEE-754 ladder, BT-52 6-phase compiler, and BT-56 complete LLM integration. Scores across 5 levels (Foundation/Process/Core/Engine/System) and 4 axes (n6/perf/power/cost).
- Slot: `dse.lang` (new)
- Invocation: `nexus dse lang`
- Source: `bridge/origins/universal-dse/domains/programming-language.toml` (no separate origin crate; the universal-dse engine loads this domain TOML)
- Status: Integrated via a new slot.

### 4. gpu-arch-calc — GPU Architecture Calculator
- Role: Calculates SM/CU configuration, cache hierarchy, memory bandwidth, peak TFLOPS, and n=6 alignment ratios on the roofline. (Packaged alongside chip_design_n6_analysis.hexa.)
- Slot: `calc.gpu-arch`
- Invocation: `nexus calc gpu-arch`
- Status: Existing origin; metadata enriched.

### 5. chip-n6-calc — n=6 Chip Calculator
- Role: Validates chip parameters based on the n=6 principle (tile count, register file, pipeline stages, and the 6-axis balance of power, performance, and area).
- Slot: `calc.chip-n6`
- Invocation: `nexus calc chip-n6`
- Status: Existing origin; metadata enriched.

### 6. semiconductor-calc — Semiconductor Calculator
- Role: Process-node analysis, ITRS/IRDS scaling, and transistor density/delay/leakage calculations.
- Slot: `calc.semiconductor`
- Invocation: `nexus calc semiconductor`
- Status: Existing origin; metadata enriched.

## Routing Model

`bridge/_registry.json` is the SSOT of the HEXA-ONLY dispatcher. Only the config is updated; no separate binary changes are required.

- Tool registration format: `{ desc, src, origin, invocation, status }`
- Execution path: `nexus <category> <name>` → delegates to `$HEXA bridge/origins/<origin>`
- The `_runtime` field explicitly declares the hexa invocation contract.
- 5 tools were already registered in the calc slot previously; this task adds invocation/status metadata and the `p1_chip_p1_4: true` flag.
- lang-dse adds a new `dse.lang` slot (reusing the universal-dse domain TOML).

## Verification Items

- 6/6 tools registered in the SSOT (`bridge/_registry.json` `p1_chip_p1_4.tools`)
- 6/6 origin paths exist on disk (5 main.hexa + 1 programming-language.toml)
- No emoji. Document is in English. No binary or compiled artifacts were modified.
- Roadmap linkage: `$NEXUS/shared/roadmaps/n6-architecture.json` CHIP-P1-4 status done + result_2026_04_14.
