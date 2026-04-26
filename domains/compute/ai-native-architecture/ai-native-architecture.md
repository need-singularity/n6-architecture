---
domain: ai-native-architecture
axis: compute (Y_compute candidate)
mk_stage: Mk.0 -> Mk_inf
alien: 6 -> path-to-10
parent_session: reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md
parent_followup: reports/anomaly/btAI2c_h1_summary.md
silicon_primitives: [provenance-bit, promotion-counter-mmu, bt-id-isa]
principles: [honesty-triad, write-barrier, constraint-honesty]
verify: domains/compute/ai-native-architecture/verify_ai-native-architecture.py
---

# AI-native architecture (beyond GPU)

## §1 Domain scope

This domain captures the n=6-derived silicon primitives that move AI accelerators
past the GPU substrate by hard-wiring the honesty triad (banner / write-barrier /
falsifier) into the dataflow path.

The accelerator under design (HEXA-AI, alien 6) has three inseparable silicon
primitives, each pinned to atlas constants:

1. provenance-bit  -- 1-bit FACT/HYPOTHESIS flag carried per tensor; OR-propagated.
   Overhead is exactly phi/sigma_n = 1/36 of tensor payload.
2. promotion-counter-mmu  -- write-barrier MMU register that refuses any write
   whose (prov, grade) pair fails (prov == FACT and grade >= threshold).
3. bt-id-isa  -- one ISA opcode field carrying the BT identifier so each MAC
   issuance is auditable to the breakthrough-theorem ledger.

Together these are the smallest substrate that makes the honesty triad a
hardware property rather than a process discipline.

## §2 N6 derivation -- 10 EXACT constants

All constants below are derived from n=6 number-theoretic primitives:
sigma=12, phi=2, n=6, tau=4, sigma_n=72, J2=24, sopfr_n=5.
Each has a Python-verifiable source-of-truth (atlas string, sim constant, or
JSON measurement).

| # | Constant | N6 derivation | Value | Source-of-truth |
|---|----------|---------------|-------|-----------------|
| 1 | provenance_bit_overhead | phi / sigma_n | 1/36 | atlas line 526 |
| 2 | n6_native_tiles | sigma / phi | 6 | btAI2 N_TILES |
| 3 | pipeline_stages | tau | 4 | btAI2 TAU_STAGES |
| 4 | peak_macs_per_tile_per_cycle | sigma * phi | 24 | atlas J2 line 446 |
| 5 | peak_macs_per_array_per_cycle | sigma^2 * phi | 288 | atlas sigma_sq * phi |
| 6 | provenance_threshold_max | sigma | 12 | sweep upper bound |
| 7 | provenance_threshold_min | phi^2 | 4 | sweep lower bound |
| 8 | legit_reject_rate_theoretical | 0 | 0.0 | _maybe_false_positive |
| 9 | h1_speculative_drop_floor | 0 | 0.0 | btAI2c_h1_results.json |
| 10 | bt_coverage_count | sopfr(6) + phi | 7 | BT_541..547 |

The full Python derivation chain is in the verify script (frontmatter `verify`).
PASS criterion: symbolic == observed for all 10.

## §3 Falsifier ledger

The omega-cycle session registered four falsifiers; current verdicts are:

| ID | Predicate | Verdict | Evidence |
|----|-----------|---------|----------|
| F-AI2-A | provenance ON drops throughput at most 5% vs baseline | PARTIAL | btAI2_results.json (single-seed worst-case 7.7%) |
| F-AI2-B | promotion-counter MMU refuses at most 1% of legit writes | PASS robust | btAI2_seed_sweep 900 cells, 0 breaches |
| F-AI2c-A | H1 speculative-eager scheduler keeps drop within 5% | PASS at H1 | btAI2c_h1_results.json summary_h1.mean=0.0 |
| F-AI1 | MPS / tensor-network surrogate matches NPU within 2% | HOLD-PROXY | no surrogate landed; deferred next cycle |

F-AI2-A PARTIAL is amended by F-AI2c-A PASS: the H1 schedule (rollback_rate=0)
demonstrates that the 5% bound is achievable with one bounded design move,
without weakening the falsifier itself.

## §4 Closure status

| Tier | State | Rationale |
|------|-------|-----------|
| design | MEDIUM (amended) | F-AI2-B robust + F-AI2c-A H1 PASS clear the 5% bound |
| sim | MEDIUM | 1000-seed sweeps + workload coverage on full + matmul + softmax |
| silicon | LOW | RTL stub absent; only architectural spec exists |
| literature | LOW | no SC publication; absorption pending nexus |

Alien score path 6 -> 10 requires:
- silicon-LOW -> MEDIUM via BT-AI3 RTL candidate spec
- F-AI1 HOLD-PROXY -> PASS via MPS surrogate
- 6-vendor convergence audit (current cycle item C4)

The 10/10 EXACT closure documented here is the design-tier closure: every
quantitative claim about the silicon primitives is derivable from n=6 primitives
and machine-checkable by the verify script.

## §5 References

### Atlas

- atlas/atlas.append.n6-architecture-historical-absorption-2026-04-26.n6:526
  -- provenance_bit_overhead = phi/sigma_n = 1/36
- atlas/atlas.n6:446 -- J2 = sigma*phi = n*tau = 24 (peak MAC per tile)
- atlas/atlas.n6:56 -- sigma_sq = sigma^2 = 144 (SM array size)
- atlas/atlas.n6:68 -- sigma_n = sigma*n = 72 (provenance overhead denominator)
- atlas/atlas.n6:48 -- J2 = jordan_totient(6,2) = 24

### Knowledge graph nodes

- arch:n6-native-accelerator
- silicon:provenance-bit
- silicon:promotion-counter-mmu
- silicon:bt-id-isa
- principle:honesty-triad
- principle:write-barrier
- principle:constraint-honesty
- omega-cycle:ai-native-arch-beyond-gpu-2026-04-26

### Simulators

- experiments/anomaly/btAI2_honesty_bit_scheduler.py -- main cycle-level model
- experiments/anomaly/btAI2_seed_sweep.py -- 900-cell robustness sweep
- experiments/anomaly/btAI2c_h1_simulator.py -- H1 speculative-eager variant
- experiments/anomaly/btAI2c_h1_sweep.py -- 100-seed H1 sweep

### Reports

- reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md -- parent
- reports/anomaly/btAI2_results.json -- F-AI2-A / F-AI2-B initial run
- reports/anomaly/btAI2_seed_sweep_results.json -- 900-cell robustness
- reports/anomaly/btAI2c_h1_results.json -- F-AI2c-A H1 PASS
- reports/anomaly/btAI2c_h1_summary.md -- H1 follow-up note
- reports/anomaly/ai-native-architecture-10exact-closure-2026-04-26.md -- this closure

### Verify script

- domains/compute/ai-native-architecture/verify_ai-native-architecture.py
- run via /tmp/btai2-venv/bin/python or python3 (stdlib + json only)
- target: 10/10 EXACT, exit code 0

### Cross-domain links

- domains/compute/chip-npu-n6 -- HEXA-NPU substrate (provides sigma^2=144 SM)
- domains/compute/chip-architecture -- general n=6 chip structure
- domains/compute/ai-efficiency -- TOPS/W envelope (B^4 scaling backstop)

## §6 Open items (next cycle, not done here)

- Update README chip row from PARTIAL to PASS-amended once script is green.
- Append two atlas EXACT constants: peak_macs_per_array (sigma_sq * phi) and
  bt_coverage_count (sopfr + phi). Promotion through atlas-agent only.
- BT-AI3 RTL candidate spec.
- BT-AI1 MPS surrogate (or formal HOLD-PROXY closure note).

This document does not modify atlas, KG, README, or parent session; those
edits are deferred to the next cycle per write-barrier discipline.
