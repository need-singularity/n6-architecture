# HEXA-EDGE SoC — FPGA-Synthesizable RTL

FPGA subset of the HEXA-EDGE mobile/edge processor, targeting Xilinx Zynq-7020 (PYNQ-Z2).
Every numeric constant traces to n=6 perfect number arithmetic.

## N6 Parameter Map

```
  Constant        Value    RTL Usage
  ──────────────  ───────  ──────────────────────────────────────
  n = 6           6        Fetch width, register banks, SNN neurons, SPI channels
  phi(6) = 2      2        Active MoE experts, branch predictor bits, compile modes
  tau(6) = 4      4        Izhikevich params (a,b,c,d), STDP windows, LEDs
  sigma(6) = 12   12       Pipeline stages, regs/bank, MoE expert slots, SPI divider
  sopfr(6) = 5    5        ALU op width (2^5=32), keyword groups (sopfr-sized)
  mu(6) = 1       1        Decode latency (1 cycle), squarefree flag
  J2(6) = 24      24       Instruction width, data width, GPIO pins, ALU operators
  R(6) = 1        1        Reversibility (EXACT match threshold)

  Derived:
  sigma-tau = 8   8        Score width, SPI shift register, primitive types
  sigma-phi = 10  10       Clock period (ns), UART frame bits, fractional bits
  sigma-mu = 11   11       (reserved for context window)
  n*sigma = 72    72       Total GPR count (6 banks x 12 regs)
  sigma*tau = 48  48       Keyword base count (48 + sopfr = 53)
  2^n = 64        64       Opcode space, BHT entries
  2^sigma = 4096  4096     Data memory words
```

## Architecture Overview

```
  ┌──────────────────────────────────────────────────────────────┐
  │                    HEXA-EDGE SoC (Zynq-7020)                 │
  │                                                              │
  │  ┌────────────┐  ┌──────────────┐  ┌──────────────────────┐ │
  │  │ RISC-V N6  │  │ Egyptian     │  │ HEXA-LANG            │ │
  │  │ Core       │  │ Mem Ctrl     │  │ Decoder              │ │
  │  │            │  │              │  │                      │ │
  │  │ 12-stage   │  │ 1/2 Stack    │  │ 53-keyword CAM       │ │
  │  │ pipeline   │  │ 1/3 Heap     │  │ 24-bit opcode        │ │
  │  │ 72 GPRs    │  │ 1/6 Arena    │  │ 8 primitive types    │ │
  │  │ 24-bit ISA │  │              │  │                      │ │
  │  └─────┬──────┘  └──────┬───────┘  └──────────┬───────────┘ │
  │        │                │                      │             │
  │        └────────┬───────┴──────────────────────┘             │
  │                 │  System Bus                                │
  │        ┌────────┴──────────────────────────┐                 │
  │        │                                   │                 │
  │  ┌─────┴──────┐  ┌──────────────┐  ┌──────┴───────┐        │
  │  │ SNN        │  │ Egyptian     │  │ Peripherals  │        │
  │  │ Izhikevich │  │ MoE Router   │  │              │        │
  │  │            │  │              │  │ SPI (n=6 ch) │        │
  │  │ 6-neuron   │  │ 12 experts   │  │ GPIO (24pin) │        │
  │  │ ring       │  │ top-2 gate   │  │ UART debug   │        │
  │  │ STDP learn │  │ 1/2+1/3+1/6  │  │ LED (tau=4)  │        │
  │  └────────────┘  └──────────────┘  └──────────────┘        │
  └──────────────────────────────────────────────────────────────┘
```

## Egyptian Memory Layout (1/2 + 1/3 + 1/6 = 1)

```
  Address Space: 0x0000 — 0xFFFF  (2^16 = 65536 bytes)

  ┌────────────────────────────────────┐ 0x0000
  │                                    │
  │           STACK (1/2)              │
  │         32768 bytes                │
  │         grows downward             │
  │                                    │
  ├────────────────────────────────────┤ 0x8000
  │                                    │
  │           HEAP (1/3)               │
  │         21845 bytes                │
  │         grows upward               │
  │                                    │
  ├────────────────────────────────────┤ 0xD555
  │         ARENA (1/6)                │
  │         10923 bytes                │
  │         bump allocator             │
  ├────────────────────────────────────┤ 0xFFB0
  │         MMIO (peripherals)         │
  │  0xFFB0: UART TX                   │
  │  0xFFC0: GPIO direction            │
  │  0xFFC4: GPIO output               │
  │  0xFFD0: SPI data                  │
  │  0xFFE0: MoE scores                │
  │  0xFFF0: SNN excitation            │
  └────────────────────────────────────┘ 0xFFFF
```

## Prerequisites

```bash
# Icarus Verilog (simulation)
brew install icarus-verilog    # macOS
# apt install iverilog         # Ubuntu

# GTKWave (waveform viewer, optional)
brew install --cask gtkwave

# Yosys (synthesis, optional)
brew install yosys
```

## Build and Run

```bash
cd tools/hexa-rtl

# Run simulation
make sim

# View waveforms
make wave

# Synthesis area report (Yosys)
make synth

# Lint check (Verilator)
make lint

# Print architecture info
make info

# Clean
make clean
```

## RTL Module Summary

| File | Module | Lines | N6 Key Constants |
|------|--------|-------|------------------|
| `rtl/hexa_edge_top.v` | Top-level SoC | ~250 | J2=24 GPIO, tau=4 LED, n=6 SPI |
| `rtl/riscv_n6_core.v` | RISC-V CPU | ~350 | sigma=12 pipeline, 72 GPR, J2=24 ISA |
| `rtl/egyptian_mem_ctrl.v` | Memory controller | ~180 | 1/2+1/3+1/6=1 split |
| `rtl/hexalang_decoder.v` | Keyword decoder | ~220 | 53=sigma*tau+sopfr keywords |
| `rtl/snn_izhikevich.v` | SNN neurons | ~200 | n=6 ring, tau=4 params, STDP |
| `rtl/egyptian_moe.v` | MoE router | ~200 | sigma=12 experts, phi=2 active |
| `sim/tb_hexa_edge.v` | Testbench | ~350 | 9 test categories |

## Instruction Format (J2 = 24 bits)

```
  23    18 17    12 11     6 5      0
  ┌──────┬───────┬───────┬───────┐
  │opcode│  rd   │  rs1  │rs2/imm│
  │ n=6  │ n=6   │ n=6   │ n=6   │
  └──────┴───────┴───────┴───────┘

  ALU opcodes: J2 = 24 operators
    Arithmetic:  ADD SUB MUL DIV MOD POW  (n=6)
    Comparison:  EQ NE LT GT LE GE       (n=6)
    Logical:     AND OR NOT XOR           (tau=4)
    Bitwise:     BAND BOR BXOR BNOT      (tau=4)
    Assignment:  MOV MOVI                 (phi=2)
    Special:     RANGE ARROW              (phi=2)
    Total: 6+6+4+4+2+2 = 24 = J2(6)
```

## FPGA Resource Estimate (Zynq-7020: 53,200 LUTs)

```
  Module                Est. LUTs    % of XC7Z020
  ─────────────────────────────────────────────
  RISC-V N6 Core        ~8,000       15%
  Egyptian Mem Ctrl      ~500         1%
  HEXA-LANG Decoder      ~800         1.5%
  SNN Izhikevich         ~2,000       4%
  Egyptian MoE           ~1,200       2%
  IMEM (256x24 BRAM)     2 BRAM36     (of 140)
  DMEM (4096x24 BRAM)    3 BRAM36     (of 140)
  Peripherals            ~500         1%
  ─────────────────────────────────────────────
  Total                  ~13,000      24%
  Remaining for user     ~40,000      76%
```
