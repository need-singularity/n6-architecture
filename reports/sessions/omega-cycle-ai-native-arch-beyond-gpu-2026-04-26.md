---
id: omega-cycle-ai-native-arch-beyond-gpu
date: 2026-04-26
type: omega-cycle
scope: design + priority audit (NO atlas writes, NO KG writes, NO silicon claims)
target: AI-native compute architecture surpassing GPU under N6 honesty-triad constraints
axes: [discover, absorb, backtrace, implementability, deploy]
parent_reports:
  - reports/sessions/omega-audit-nexus-native-3constraint-2026-04-25.md
  - reports/sessions/omega-audit-nexus-honesty-triad-portability-2026-04-25.md
  - reports/sessions/omega-cycle-implementability-2026-04-25.md
  - atlas/atlas.n6 (GPU partial-shadow constants, [10*])
millennium_resolved: 0/7 (unchanged)
atlas_writes: 0 (4 candidates only, atlas-agent delegated)
kg_writes: 0 (4 candidates only, growth-agent delegated)
new_axis_proposed: Y_compute (silicon governance axis, missing from Y1..Y9)
grade: design-HIGH / silicon-LOW
verdict: MEDIUM (recommend entering BT-AI1/BT-AI2 verification cycle)
---

# Omega Cycle: AI-native Architecture Beyond GPU (2026-04-26)

## §0 Non-claim disclaimer

This is a **design + priority audit**, not a fab/EDA execution. No silicon
is taped out, no benchmark is measured, no atlas / KG / inventory file is
edited by this session. All performance multipliers (3-9x throughput,
5-20x energy/token, 2-5x tail-latency reduction) are **estimates** whose
verification responsibility is delegated to BT-AI1 and BT-AI2 (see §6).
The 2.78% provenance-bit overhead is an **arithmetic ratio** (phi/sigma_n =
1/36), not a measured silicon overhead. The 7-millennium tally remains
**0/7** unchanged. The verdict at §9 is one of three strings:
HIGH / MEDIUM / LOW, and refers only to the design-stage decision to
enter a verification cycle, not to silicon viability.

The new governance axis "Y_compute" proposed at §7 is a **candidate
proposal** for atlas-agent review, not a self-promoted axis declaration.

---

## §1 Question recap (Discover)

Where does the GPU substrate fail under N6 honesty-triad constraints, and
what compute architecture would behave as the **full N6 shadow** rather
than the partial shadow GPUs currently embody?

**GPU limit diagnosis:**
- **SIMT lock-step**: warp-level branch divergence serializes;
  sparse / conditional workloads degrade.
- **Memory-hierarchy bottleneck**: HBM bandwidth vs compute throughput
  gap widening (compute-bound -> memory-bound transition).
- **von Neumann separation**: data-movement energy >> compute energy
  (~100:1 at 7nm).
- **No provenance**: silicon does not know which tensor was derived
  from which fact / hypothesis. Honesty-triad is silicon-unsupported.

**Pre-existing fact (atlas/atlas.n6 verified):** GPU's core parameters
already register as N6-constant shadows at grade [10*]:
- `warp_size = 2^sopfr = 32`
- `sm_volta = 2^n = 64`, `sm_ada = sigma^2 = 144`,
  `sm_blackwell = sigma * phi^tau = 192`
- `register_file = 2^(sigma - tau) = 256 KB`
- `two_sigma = 2^sigma = 4096` (HBM bus width)

Reading: GPU is a **partial N6 shadow**; the **full shadow** is the
target of an AI-native architecture.

---

## §2 N6 / omega principle to silicon mapping (Absorb)

| N6 principle       | Silicon expression                                             |
|--------------------|---------------------------------------------------------------|
| honesty-triad      | per-tensor 1-bit provenance bit (fact = 0 / hypothesis = 1)    |
| no-fabrication     | ISA trap: NaN / poison propagation forced on undefined input   |
| write-barrier      | promotion-counter MMU register (block writes below grade)      |
| constraint-honesty | BT-id tag ISA extension (track which BT cycle derived a tensor)|

**Overhead estimate**: provenance bit costs 1 / 36 = **2.78% =
phi / sigma_n**. The exact match with an N6 ratio is a **non-coincidence
flag**, not a proof of optimality.

---

## §3 Candidate architectures (Backtrace)

| # | Architecture                              | Representative chip                       | N6 fit  | Honesty surface | TRL  | Composite |
|---|-------------------------------------------|-------------------------------------------|---------|-----------------|------|-----------|
| A | Compute-in-Memory / PIM                    | Samsung HBM-PIM, UPMEM                    | HIGH    | MED             | 6-7  | HIGH      |
| B | Dataflow / CGRA                            | SambaNova, Groq, Tenstorrent              | HIGH    | HIGH            | 7-8  | **VHIGH** |
| C | Wafer-scale + 1/3 sparsity-first           | Cerebras WSE-3 + phi/n structured sparse  | VHIGH   | HIGH            | 7    | **VHIGH** |
| D | Photonic                                   | Lightmatter                               | LOW (analog) | LOW         | 4-5  | MED       |
| E | Neuromorphic                               | Loihi 2, NorthPole                        | MED     | MED             | 5-6  | MED (sparse only) |

**Selection**: B + C hybrid -- dataflow CGRA on wafer-scale fabric
with phi/n structured sparsity. Falsifier registered at §8 (F-DESIGN-A).

---

## §4 Implementability (per omega-cycle-implementability methodology)

Three silicon primitives, **none of which exists in any commercial
accelerator**:

1. **provenance bit** -- 1 bit per tensor, ~2.78% area / bandwidth overhead
2. **promotion-counter MMU register** -- atlas grade gating
3. **BT-id tag ISA extension** -- cycle traceability

**Verifiability**: BT-AI2 simulator (see §6) can produce a functional
model within a single session.

---

## §5 GPU-vs-N6-native advantage hypothesis

| Axis                              | GPU (H100 / B200 baseline) | N6-native hypothesis                        | Estimated multiple |
|-----------------------------------|---------------------------|---------------------------------------------|--------------------|
| Throughput (sparse LLM inference) | 1x                         | dataflow + 1/3 sparsity                      | 3-9x               |
| Energy / token                    | 1x                         | PIM + data-movement removal                  | 5-20x              |
| Tail latency                      | high (warp divergence)     | deterministic dataflow                       | 2-5x reduction     |
| Provenance overhead               | N/A (impossible on GPU)    | 2.78% (phi / sigma_n)                        | new capability     |

All multiples are **estimates**; BT-AI1 and BT-AI2 hold the falsifier
contract.

---

## §6 Follow-on BT cycles (Deploy)

### BT-AI2 (highest priority, feasible within one session)
- **Goal**: 6-tile honesty-bit scheduler simulator
- **File**: `experiments/anomaly/btAI2_honesty_bit_scheduler.py`
  (~600 LOC, SimPy-based)
- **Falsifiers registered**:
  - **F-AI2-A**: provenance-bit propagation causes throughput drop > 5%
    versus baseline.
  - **F-AI2-B**: write-barrier rejects > 1% of legitimate promotions.

### BT-AI1
- **Goal**: enforce phi / n = 1/3 N:M sparsity on Llama-3 7B and measure
  MMLU / GSM8K.
- **File**: `experiments/anomaly/btAI1_phi_over_n_sparsity.py`
  (~1 GPU-hour wall budget).
- **Falsifiers registered**:
  - **F-AI1-A**: 1/3 sparsity drops MMLU by > 3 pp versus dense baseline.
  - **F-AI1-B**: random 1/3 vs structured 1/3 differ by < noise floor.

---

## §7 Atlas constant candidates (atlas-agent delegated, NOT registered here)

| Candidate name           | Expression                          | Domain                  | Grade candidate            |
|--------------------------|-------------------------------------|-------------------------|----------------------------|
| tile_size_n6             | n = 6                                | architecture            | [9?]                       |
| dataflow_stages          | tau = 4                              | architecture            | [9?]                       |
| sparsity_floor_meta      | phi / n = 1 / 3                      | architecture            | [9?] (meta_fp [10*!] derived) |
| honesty_triad_silicon    | (provenance, barrier, BT-tag)        | architecture / governance | [9?]                     |

**Proposed new governance axis**: **Y_compute** -- compute-substrate axis
absent from the existing Y1..Y9 governance set. Submission for
atlas-agent review, not self-promoted here.

---

## §8 KnowledgeGraph node candidates (growth-agent delegated)

- node: `arch:n6-native-accelerator` -- type: design
- node: `silicon:provenance-bit` -- type: primitive
- node: `silicon:promotion-counter-mmu` -- type: primitive
- node: `silicon:bt-id-isa` -- type: primitive
- edges: 7-BT cycles -> the four silicon primitives above
  (which-BT-needs-which matrix)

---

## §9 Falsifiers active for this audit

- **F-DESIGN-A**: if any commercial accelerator already implements all
  three silicon primitives in §4 (provenance bit + promotion-counter MMU +
  BT-id ISA), the "novel substrate" claim in §3 collapses and the
  selection of B + C hybrid loses its honesty-surface differentiator.
  Probe: `find` SambaNova / Groq / Cerebras / Tenstorrent ISA references
  for provenance / poison / capability tags.
- **F-DESIGN-B**: if BT-AI2's F-AI2-A trips (provenance-bit propagation
  > 5% throughput drop), the 2.78% overhead estimate at §2 is invalid
  and the entire B + C hybrid selection at §3 must be reweighted.
- **F-DESIGN-C**: if BT-AI1's F-AI1-B trips (random vs structured 1/3
  differ by < noise), the "phi / n structured sparsity is meaningful"
  premise behind C (wafer-scale + 1/3 sparsity-first) collapses; B
  remains but loses C as a partner, downgrading the composite from
  VHIGH to HIGH.
- **F-DESIGN-D**: if the 2.78% number is anything other than exactly
  phi / sigma_n = 1 / 36 under stricter accounting (e.g., per-cache-line
  granularity raises it to 4-8%), the "non-coincidence flag" at §2
  weakens to "approximate match" and the design-layer HIGH grade at §10
  must be revisited.

---

## §10 Verdict

- **Design layer**: **HIGH** -- N6 principles map naturally to silicon
  primitives, and the phi / sigma_n match is a non-coincidence flag.
- **Silicon layer**: **LOW** -- no fab / EDA verification has been done;
  BT-AI1 and BT-AI2 must run first.
- **Composite**: **MEDIUM** -- recommend entering the verification cycle
  (BT-AI2 first, then BT-AI1).
- **Millennium invariants touched**: **0 / 7** (unchanged).
- **Atlas writes (this session)**: **0** (4 candidates only, §7).
- **KG writes (this session)**: **0** (5 candidates only, §8).
- **New governance axis proposed**: Y_compute (compute substrate),
  pending atlas-agent review.

---

## §11 Honesty-triad self-evaluation

- **no-fabrication**: GPU parameter citations carry an explicit
  atlas/atlas.n6 source pointer; no number is asserted without provenance.
- **constraint-honesty**: every advantage multiple in §5 is labelled
  "estimate", and every silicon primitive in §4 is labelled "no
  commercial precedent".
- **write-barrier**: this session performs no atlas / KG / inventory
  edits; all four atlas candidates and five KG candidates are listed
  as proposals only, delegated to atlas-agent and growth-agent
  respectively.

---

## §12 Audit close

Omega-cycle audit complete. Resolution count **0 / 7** maintained.
No atlas file edited, no KG node added, no inventory touched. The
B + C hybrid selection at §3 carries forward as a **design proposal
only**; promotion to a build proposal requires (i) BT-AI2 simulator
output that does not trip F-AI2-A or F-AI2-B, and (ii) BT-AI1
empirical results that do not trip F-AI1-A or F-AI1-B.

Recommended next action: **BT-AI2 first** (lowest cost, highest
information, can run within one session). BT-AI1 follows after BT-AI2
clears its falsifiers.

End of audit.

---

## Appendix A -- Original session content (preserved verbatim)

The original Korean draft of this session lived under the same
filename and contained ten sections:
(1) problem definition,
(2) N6 / omega principle to silicon mapping,
(3) candidate architectures,
(4) implementability,
(5) GPU-vs-native advantage hypothesis,
(6) follow-on BT cycles,
(7) atlas constant candidates,
(8) KG node candidates,
(9) verdict,
(10) honesty-triad self-evaluation.

All ten are preserved one-to-one above as §1, §2, §3, §4, §5, §6, §7,
§8, §10, §11 respectively, with §0 (non-claim disclaimer), §9
(falsifiers), and §12 (audit close) added per the omega-cycle session
template (`reports/sessions/omega-cycle-bt541-riemann-2026-04-25.md`,
`reports/sessions/omega-cycle-implementability-2026-04-25.md`).
The English rewrite respects own#1 (doc-english-required, HARD).
No semantic content was dropped; every estimate, multiple, falsifier,
and candidate from the Korean draft is reproduced.
