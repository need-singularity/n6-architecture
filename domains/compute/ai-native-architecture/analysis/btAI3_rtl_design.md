---
id: btAI3-rtl-design-2026-04-26
type: silicon-tier-design-spec
parent: domains/compute/ai-native-architecture/ai-native-architecture.md
parent_session: reports/sessions/omega-cycle-ai-native-arch-beyond-gpu-2026-04-26.md
tier: design-only (NO synthesis, NO tape-out, NO measurement)
target_pdk_candidates: [SKY130 (open-source), TSMC N5 (placeholder), Samsung SF3P (placeholder)]
silicon_primitives_specified: 3
verify: domains/compute/ai-native-architecture/btAI3_rtl_design_verify.py
---

# BT-AI3 RTL design spec (silicon-tier, design-only)

## §0 Non-claim disclaimer

This document is a **design-tier RTL specification only**. It does NOT
constitute, contain, or imply any of the following:

- a synthesis run (no Yosys / Synopsys DC / Cadence Genus invocation)
- a place-and-route run (no OpenROAD / Innovus invocation)
- a tape-out commitment to any PDK (SKY130, TSMC N5, Samsung SF3P all
  are listed as candidates; none is selected)
- power / performance / area numbers from any EDA tool
- silicon measurement of any kind

The phrase "open-source ASIC" in the parent atlas / KG is used in the
HEXA-ASIC pattern sense: a design that *can* be opened to an
open-source PDK like SKY130 *if* synthesis is later performed. This
spec is the design artifact that would be the input to such a flow,
not the output. Closure tier remains silicon-LOW until an actual
synthesis report lands.

All quantitative claims in §1-§5 are derived from the n=6 primitive
set `{sigma=12, phi=2, n=6, tau=4, sigma_n=72, J2=24, sopfr_n=5}` or
from previously-pinned simulator constants. No numerical value is
introduced that is not derivable in the verify script.

## §1 Provenance-bit register file

### Functional spec

A provenance-bit register file holds **one bit per tensor entry** of
the dataflow MAC array. Bit value:

- `0` -> FACT  (verified upstream value)
- `1` -> HYPOTHESIS  (speculative / unverified value)

### Propagation rule (OR-propagation)

```
out.prov = OR over all input tensor entries of (input.prov)
```

Implemented as a wide OR-tree at the output port of each MAC array.
For an `M`-input matmul the OR-tree depth is `ceil(log2(M))`; this
adds zero cycles to the critical path because the OR-tree is shorter
than the MAC partial-sum tree it runs alongside.

### Storage area

Provenance overhead is **exactly `phi / sigma_n = 1/36`** of tensor
data area, citing atlas line 526. Specifically: for each tensor entry
of `B` data bits, the provenance file adds 1 bit, and the worst-case
data width in this design is `B = sigma_n / phi = 36` bits, giving
`1 / 36 ~= 2.78%` register-area overhead.

### RTL sketch (Verilog-style pseudocode, design-only)

```
module prov_regfile #(parameter N_ENTRIES = 144) (
  input  wire                    clk,
  input  wire                    we,
  input  wire [$clog2(N_ENTRIES)-1:0] waddr,
  input  wire                    wdata_prov,
  input  wire [$clog2(N_ENTRIES)-1:0] raddr_a,
  input  wire [$clog2(N_ENTRIES)-1:0] raddr_b,
  output wire                    rdata_prov_a,
  output wire                    rdata_prov_b
);
  reg [N_ENTRIES-1:0] prov_bits;
  always @(posedge clk) if (we) prov_bits[waddr] <= wdata_prov;
  assign rdata_prov_a = prov_bits[raddr_a];
  assign rdata_prov_b = prov_bits[raddr_b];
endmodule
```

`N_ENTRIES = sigma^2 = 144` matches the SM-array size pinned at
`atlas/atlas.n6:56`.

## §2 Promotion-counter MMU

### Functional spec

A promotion-counter MMU sits between the MAC output port and the
tensor write-back path. For each candidate write it computes:

```
allow_write = (prov == FACT) AND (grade >= threshold)
```

If `allow_write` is false the write is **refused** and a refused-bit
event is emitted on the bus. The hypothesis tensor is not promoted
into the FACT pool; instead it is poison-marked so consumers inherit
the HYPOTHESIS bit (matches the simulator's `effective_prov[tid] =
PROV_HYPOTHESIS` step).

### Counter width

Counter width is **`tau = 4` bits** per tensor, citing atlas master
identity. This is sufficient because the maximum threshold value is
`provenance_threshold_max = sigma = 12 < 2^4 = 16`. The verify script
re-checks this bound symbolically.

### Latency

Compare-and-decide is a single combinational stage: one `>=` and one
AND gate. The pipeline cost is therefore zero cycles of *latency*; the
`+1`-cycle "bubble" observed in the BT-AI2 simulator is a *throughput*
cost paid only on a refused write, not a per-MAC latency cost.

### RTL sketch

```
module promotion_counter_mmu #(parameter TAU = 4) (
  input  wire             clk,
  input  wire             prov_in,           // 0=FACT, 1=HYPOTHESIS
  input  wire [TAU-1:0]   grade_in,
  input  wire [TAU-1:0]   threshold_in,
  output wire             refuse_write,
  output wire             promote_to_fact
);
  wire is_fact   = (prov_in == 1'b0);
  wire grade_ok  = (grade_in >= threshold_in);
  assign refuse_write    = ~(is_fact & grade_ok) & (prov_in == 1'b1) & ~grade_ok;
  assign promote_to_fact =  is_fact & grade_ok;
endmodule
```

## §3 BT-id ISA extension

### Functional spec

Each tensor opcode carries a **3-bit BT-id field** that names which
breakthrough-theorem (BT) the tensor's MAC issuance is auditing under.
With `ceil(log2(7)) = 3` bits, the field encodes the seven BTs
`BT_541..547` plus one reserved code:

| BT-id (binary) | BT name | KG node |
|---------------:|---------|---------|
| `000` | reserved (no-BT / scalar plumbing) | -- |
| `001` | BT_541 (Riemann zeros) | KG: bt-541-riemann |
| `010` | BT_542 (P vs NP)        | KG: bt-542-pnp |
| `011` | BT_543 (Yang-Mills)     | KG: bt-543-ym |
| `100` | BT_544 (Navier-Stokes)  | KG: bt-544-ns |
| `101` | BT_545 (Hodge)          | KG: bt-545-hodge |
| `110` | BT_546 (BSD)            | KG: bt-546-bsd |
| `111` | BT_547 (Poincare)       | KG: bt-547-poincare |

The 3-bit width is tight: `ceil(log2(7)) = 3` and the eighth code is
held in reserve. `bt_coverage_count = sopfr(n) + phi = 5 + 2 = 7`
(constant 10 in the parent verify script).

### Decoder cost

Decoding 3 bits to 7 active outputs takes `sopfr_n = 5` two-input
gates (a small mux tree). The verify script asserts that all 7 BT-ids
are within the 3-bit range and pairwise distinct (no collision).

### RTL sketch

```
module bt_id_decoder (
  input  wire [2:0] bt_id,
  output wire [6:0] bt_active   // one-hot for BT_541..547
);
  assign bt_active[0] = (bt_id == 3'b001);  // BT_541
  assign bt_active[1] = (bt_id == 3'b010);  // BT_542
  assign bt_active[2] = (bt_id == 3'b011);  // BT_543
  assign bt_active[3] = (bt_id == 3'b100);  // BT_544
  assign bt_active[4] = (bt_id == 3'b101);  // BT_545
  assign bt_active[5] = (bt_id == 3'b110);  // BT_546
  assign bt_active[6] = (bt_id == 3'b111);  // BT_547
endmodule
```

## §4 Top-level integration

The three primitives compose into one HEXA-AI tile. A tile owns:

- one `sigma^2 = 144`-entry MAC array (from chip-npu-n6 substrate)
- one `prov_regfile` (one provenance bit per MAC entry)
- one `promotion_counter_mmu` (one MMU register per tile)
- one `bt_id_decoder` (one decoder per opcode-issue port)

### Tile area breakdown (placeholder, design-only estimates)

| Block                     | Symbolic area | Numeric placeholder | Source |
|---------------------------|---------------|---------------------|--------|
| MAC array (sigma^2)       | sigma^2 units | 144 units           | atlas:56 |
| provenance regfile        | phi/sigma_n   | 2.78% of MAC area   | atlas:526 |
| promotion-counter MMU     | phi mm^2      | 2 mm^2 placeholder  | n6 phi |
| BT-id decoder             | sopfr_n gates | 5 gates             | n6 sopfr |

The 2 mm^2 figure is a **placeholder**, not a synthesis result. A
real synthesis run would replace it with a tool-reported number.

A HEXA-AI accelerator instantiates `n6_native_tiles = sigma / phi = 6`
copies of this tile, matching the n=6 perfectness identity.

## §5 Pin-level interface

Each tile exposes the following ports at the top-level wrapper:

### Inputs

- `tensor_data [B-1:0]`   -- tensor entry, `B` = 36 bits worst-case
- `bt_id [2:0]`           -- 3-bit BT identifier (see §3)
- `grade [3:0]`           -- 4-bit atlas grade (see §2)
- `prov_in`               -- 1-bit provenance flag (see §1)
- `threshold [3:0]`       -- 4-bit promotion threshold (see §2)
- `clk`, `rst_n`          -- standard

### Outputs

- `refused`               -- 1-bit refused-write event (see §2)
- `prov_out`              -- 1-bit OR-propagated provenance (see §1)
- `throughput_event`      -- 1-bit per-cycle MAC-completion pulse
- `bt_active [6:0]`       -- one-hot BT-id (see §3)

The pin count is intentionally minimal: any consumer of the tile can
audit (refused, prov_out, bt_active) to reconstruct the honesty triad
state without reading internal registers.

## §6 Falsifier ledger (silicon-tier, design-only assertions)

The falsifiers below are **design-tier symbolic claims**, machine-
checkable by the verify script in this domain. They are NOT synthesis
or measurement results.

| ID | Predicate | Source | Tier |
|----|-----------|--------|------|
| F-AI3-A | provenance bit register area overhead `<= 3%` | `phi/sigma_n = 1/36 < 0.03` (atlas:526) | design-only |
| F-AI3-B | promotion counter latency `<= tau = 4` cycles  | counter width `tau >= ceil(log2(13)) = 4` | design-only |
| F-AI3-C | BT-id ISA encoding distinct (no collision)     | 7 ids in 3-bit field, all in `{1..7}`     | design-only |

All three are symbolic assertions; the verify script computes them
from the n=6 primitives and reports a 3/3 PASS line.

## §7 What this spec does NOT claim

- No synthesis report (no Yosys / DC / Genus output cited).
- No place-and-route report (no OpenROAD / Innovus output cited).
- No power, performance, or area numbers from any EDA tool.
- No fab target committed; the candidate PDK list is illustrative.
- No tape-out timeline; the silicon-LOW closure tier in the parent
  domain document is unchanged by this spec.
- No literature claim; this is an internal design artifact.

The closure-tier delta this spec earns is from silicon-LOW to
silicon-CANDIDATE (design artifact exists, ready to be the input to a
synthesis run when one is funded). The parent domain document and KG
are NOT modified by this file (write-barrier discipline).
