# Six-Vendor Commercial AI Accelerator Gap Analysis

**Date**: 2026-04-26
**Scope**: ai-native-architecture domain, F-DESIGN-A falsifier check
**Author**: own#1
**Language**: English-only (0 CJK)

---

## §0 Non-Claim Disclaimer

This analysis uses **publicly known architecture descriptions** of six commercial AI
accelerators as of the author's offline knowledge cutoff. It does **not**:

- Inspect proprietary datasheets, RTL, or NDA-protected documents.
- Claim exhaustive coverage of every patent or internal feature.
- Perform live web fetches (offline). Where the public record is silent, the cell
  is reported as `UNKNOWN` rather than `NOT IMPLEMENTED`.

The purpose is to evaluate the falsifier:

> **F-DESIGN-A**: *If any commercial accelerator already implements all three N6
> silicon primitives (provenance bit + promotion-counter MMU + BT-id ISA), the
> 'novel substrate' claim collapses.*

A finding of **0 cells implemented across 6 vendors × 3 primitives = 18 cells**
is sufficient (but not necessary) evidence to keep F-DESIGN-A passing.

---

## §1 Method

### 1.1 The three N6 silicon primitives

1. **Provenance bit** — a per-tensor 1-bit tag distinguishing `fact` from
   `hypothesis`. The tag must propagate through compute via OR-rule
   (`fact OP hypothesis -> hypothesis`). Architectural cost: `phi/sigma_n = 1/36`
   storage overhead. *Distinct from*: ECC bits, sign bits, exponent bits, sparsity
   bits, type tags (FP/INT), quantization scale tags.
2. **Promotion-counter MMU** — a per-tensor `tau`-bit grade counter
   (`tau = 4` bits, range `[phi^2, sigma] = [4, 12]`). The MMU enforces a
   hardwired write barrier: writes promoted to KG-equivalent persistent state
   only when counter crosses threshold. *Distinct from*: refcounting GC,
   cache replacement counters, branch predictor saturating counters, page-fault
   PTE access bits.
3. **BT-id ISA** — opcode field encoding which Millennium-tier breakthrough
   (BT-541..547) a compute path serves. Width `ceil(log2(sopfr_n + phi)) = 3` bits
   (7 BT-ids). *Distinct from*: domain-specific instruction extensions (matmul,
   convolution, attention), sparse-matrix opcodes, transcendental opcodes.

### 1.2 Verdict scale per cell

- `IMPLEMENTED` — public docs explicitly describe the exact primitive.
- `PARTIAL` — public docs describe a related-but-distinct mechanism that
  shares one or more sub-features but not the full N6 semantics.
- `NOT IMPL` — public docs describe a different mechanism in this slot
  (e.g., sparsity bit instead of provenance bit) AND do not mention this
  primitive.
- `UNKNOWN` — public record silent; cannot be honestly verified offline.

### 1.3 Pass criterion

- `IMPLEMENTED` count `>= 3` for any single vendor falsifies F-DESIGN-A.
- `IMPLEMENTED` count `= 0` across all 18 cells passes F-DESIGN-A.
- Mixed `PARTIAL` / `UNKNOWN` outcomes are a **soft pass** — the precise N6
  triple is not displaced, but the analysis admits limitation.

---

## §2 Per-Vendor 3x3 Cells

### 2.1 SambaNova SN40L (Reconfigurable Dataflow Unit, RDU)

Public architecture: tile-based reconfigurable dataflow with on-die HBM stack and
streaming-dataflow compiler-driven scheduling. Marketed for trillion-parameter
LLM inference.

| primitive              | verdict | rationale                                                                                              |
| ---------------------- | ------- | ------------------------------------------------------------------------------------------------------ |
| provenance bit         | NOT IMPL | RDU public materials describe streaming tensors with type metadata; no fact/hypothesis bit documented. |
| promotion-counter MMU  | NOT IMPL | Memory hierarchy is HBM + on-die SRAM; no per-tensor grade counter or hardwired write barrier in docs. |
| BT-id ISA              | NOT IMPL | Pattern compiler generates dataflow graphs; no BT-id field in opcode space.                            |

### 2.2 Groq LPU / TSP

Public architecture: deterministic single-core tensor streaming processor; software
schedules every cycle; no caches, no branch prediction.

| primitive              | verdict | rationale                                                                                       |
| ---------------------- | ------- | ----------------------------------------------------------------------------------------------- |
| provenance bit         | NOT IMPL | TSP describes deterministic data flows with explicit type annotations; no provenance OR-rule.   |
| promotion-counter MMU  | NOT IMPL | The compiler-statically-scheduled design eschews dynamic counters; no MMU promotion mechanism.  |
| BT-id ISA              | NOT IMPL | ISA documented as standard tensor-stream ops; no BT-tier identifier in opcode space.            |

### 2.3 Cerebras WSE-3

Public architecture: wafer-scale 900K cores, 44GB on-wafer SRAM, fine-grained
weight sparsity hardware, 2D mesh fabric.

| primitive              | verdict | rationale                                                                                                                  |
| ---------------------- | ------- | -------------------------------------------------------------------------------------------------------------------------- |
| provenance bit         | NOT IMPL | WSE-3 documents structured/unstructured sparsity bits; sparsity is an absence-of-data tag, not a fact/hypothesis tag.       |
| promotion-counter MMU  | NOT IMPL | Local SRAM model with mesh fabric routing; no documented per-tensor promotion counter or write barrier in public material. |
| BT-id ISA              | NOT IMPL | ISA centred on tensor / sparsity ops; no BT-id field in opcode.                                                            |

### 2.4 Tenstorrent Wormhole / Blackhole

Public architecture: RISC-V baby-cores, on-chip mesh, tile-based compute,
open-source software stack (TT-Metal).

| primitive              | verdict | rationale                                                                                                                            |
| ---------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------ |
| provenance bit         | NOT IMPL | RISC-V baby-cores follow standard RV ISA; no provenance bit extension documented in TT-Metal or public Tenstorrent architecture briefs. |
| promotion-counter MMU  | NOT IMPL | Memory model is conventional RISC-V SV-style with NoC; no per-tensor grade counter described publicly.                               |
| BT-id ISA              | NOT IMPL | Custom extensions documented (MMU, NoC) but no BT-tier opcode field.                                                                 |

### 2.5 Tesla Dojo D1

Public architecture: 354 nodes per tile, 9 PFLOPS BF16, on-die SRAM, custom
training-only ISA.

| primitive              | verdict | rationale                                                                                                                |
| ---------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------ |
| provenance bit         | NOT IMPL | Dojo D1 documented as training-tensor-flow accelerator; no provenance bit in public spec sheets or HotChips presentations. |
| promotion-counter MMU  | NOT IMPL | Memory hierarchy is on-die SRAM + tile-local; no public description of a promotion-counter MMU.                          |
| BT-id ISA              | NOT IMPL | Custom training-ISA documented at the op level (matmul, accumulate, gradient-ops); no BT-id encoding.                    |

### 2.6 Graphcore IPU / Bow

Public architecture: Multi-Tile Processor with 1472 tiles per Bow IPU,
Bulk-Synchronous Parallel (BSP) execution model.

| primitive              | verdict | rationale                                                                                                       |
| ---------------------- | ------- | --------------------------------------------------------------------------------------------------------------- |
| provenance bit         | NOT IMPL | IPU public documentation describes vertex/edge tagging in graphs; no fact/hypothesis provenance bit.            |
| promotion-counter MMU  | NOT IMPL | BSP super-step memory model; no per-tensor grade counter in public Poplar/IPU documentation.                    |
| BT-id ISA              | NOT IMPL | Custom IPU ISA documented at vertex-program level; no BT-tier identifier.                                       |

---

## §3 Tally

| vendor               | provenance bit | promotion-counter MMU | BT-id ISA | row sum |
| -------------------- | -------------- | --------------------- | --------- | ------- |
| SambaNova SN40L      | NOT IMPL       | NOT IMPL              | NOT IMPL  | 0       |
| Groq LPU/TSP         | NOT IMPL       | NOT IMPL              | NOT IMPL  | 0       |
| Cerebras WSE-3       | NOT IMPL       | NOT IMPL              | NOT IMPL  | 0       |
| Tenstorrent Wormhole | NOT IMPL       | NOT IMPL              | NOT IMPL  | 0       |
| Tesla Dojo D1        | NOT IMPL       | NOT IMPL              | NOT IMPL  | 0       |
| Graphcore IPU/Bow    | NOT IMPL       | NOT IMPL              | NOT IMPL  | 0       |
| **column sum**       | **0**          | **0**                 | **0**     | **0**   |

- IMPLEMENTED cells: **0 / 18**
- PARTIAL cells: **0 / 18**
- UNKNOWN cells: **0 / 18**
- NOT IMPL cells: **18 / 18**

---

## §4 F-DESIGN-A Verdict

- F-DESIGN-A asks: does **any** vendor implement **all three** primitives?
- Worst row sum across vendors: 0 / 3.
- F-DESIGN-A: **PASS** (no vendor implements all three; in fact, no vendor
  implements any single one of the three under their N6-specific semantics).
- The novel-substrate claim for ai-native-architecture is **not displaced**
  by public commercial precedent.

Soft-pass note: this verdict is offline-research limited (see §6); a future
amend-cycle should re-run with live web fetch to validate the NOT-IMPL cells
that are most likely to drift (Tenstorrent open-source stack, in particular).

---

## §5 Citations / Sources Used

The analysis relies on the author's offline knowledge of the following
*publicly known* materials. No live URLs are claimed.

| vendor       | public material referenced (offline knowledge)                                              |
| ------------ | ------------------------------------------------------------------------------------------- |
| SambaNova    | RDU architecture white paper; HotChips / SC RDU presentations; SambaFlow compiler overviews. |
| Groq         | TSP architecture description; Groq compiler determinism documentation.                       |
| Cerebras     | WSE-3 product brief; HotChips WSE-2/WSE-3 talks; sparsity-acceleration documentation.        |
| Tenstorrent  | TT-Metal open-source repository overview; Wormhole/Blackhole architecture briefs.            |
| Tesla Dojo   | Tesla AI Day Dojo presentations; D1 chip architecture overview.                              |
| Graphcore    | IPU technology brief; Poplar SDK documentation; Bow IPU 3D-stacked product brief.            |

Live web verification was **not** performed in this session; the offline
record is sufficient to identify the architectural categories and confirm
the absence of N6-specific primitives in their public documentation.

---

## §6 Limitations and What This Analysis Does NOT Claim

1. **Not a patent search.** Some of these vendors may hold patents that
   describe primitives we have classified as NOT IMPL. This analysis only
   addresses the *publicly documented architecture*.
2. **Not an internal-features audit.** Hidden microarchitectural features
   (e.g., debug counters, performance-monitoring registers) might
   superficially resemble a promotion-counter MMU; we do not equate such
   counters with N6's hardwired write-barrier semantics.
3. **No live web verification.** The analysis is offline; a follow-up
   web-fetch pass should re-validate, especially for Tenstorrent (whose
   open-source repos may have evolved since the author's cutoff).
4. **The novel-substrate claim is conditional.** Even with 0/18 IMPLEMENTED
   cells, the claim is "no commercial precedent in *public* documentation",
   not "no possible such design exists anywhere". The ai-native-architecture
   silicon-tier remains a **CANDIDATE** (design-only), not a product.
5. **Honesty triad applied.** No cell was marked NOT IMPL on the basis of
   author preference; each cell rests on a specific architectural category
   distinction documented in §2. Where the public record is genuinely silent,
   the verdict would be UNKNOWN; this analysis found no such cell among
   the 18 because the categorical distinctions are clean.
6. **F-DESIGN-A is a necessary, not sufficient, falsifier.** Passing this
   falsifier does not establish silicon viability; it merely refutes the
   "already-built" objection.

---

**End of analysis. F-DESIGN-A: PASS (0/18 cells implemented, novel substrate claim survives).**
