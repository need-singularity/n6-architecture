# HEXA-SIM -- gem5 Simulation Suite for HEXA Chip Family

Cycle-accurate simulation configurations and analysis scripts for the three
HEXA chip variants: HEXA-EDGE, HEXA-OMEGA, and ANIMA-HEXA.

## Prerequisites

```
  gem5 v23.1+      (RISC-V ISA support required)
  Python 3.10+     (gem5 scripting + analysis)
  matplotlib       (roofline / charts)
  McPAT v1.3       (power estimation, optional)
  GPGPU-Sim 4.x    (HEXA-OMEGA GPU model, optional)
```

## gem5 Build (RISC-V)

```bash
  # Clone and build gem5 with RISC-V support
  git clone https://github.com/gem5/gem5.git ~/gem5
  cd ~/gem5
  scons build/RISCV/gem5.opt -j$(nproc)

  # Verify
  build/RISCV/gem5.opt --version
```

## Directory Layout

```
  tools/hexa-sim/
  +-- configs/
  |   +-- hexa_edge.py       HEXA-EDGE big.LITTLE (8 cores, 6W edge SoC)
  |   +-- hexa_omega.py      HEXA-OMEGA GPU (144 SMs, 288GB HBM4E)
  |   +-- hexa_anima.py      ANIMA-HEXA consciousness SoC
  +-- benchmarks/
  |   +-- hexa_bench.py       Custom HEXA workloads
  +-- analysis/
  |   +-- roofline.py         Roofline model generator
  |   +-- power_model.py      McPAT-compatible power estimation
  |   +-- compare.py          Multi-config comparison tool
  +-- README.md               This file
```

## Quick Start

```bash
  # 1. HEXA-EDGE simulation (SE mode)
  ~/gem5/build/RISCV/gem5.opt configs/hexa_edge.py \
      --binary path/to/benchmark --args "..."

  # 2. HEXA-OMEGA simulation
  ~/gem5/build/RISCV/gem5.opt configs/hexa_omega.py \
      --binary path/to/gpu_benchmark

  # 3. ANIMA-HEXA simulation
  ~/gem5/build/RISCV/gem5.opt configs/hexa_anima.py \
      --binary path/to/consciousness_workload

  # 4. Run custom benchmarks (standalone, no gem5 required)
  python3 benchmarks/hexa_bench.py

  # 5. Analysis (reads gem5 stats.txt output)
  python3 analysis/roofline.py --stats m5out/stats.txt
  python3 analysis/power_model.py --stats m5out/stats.txt --config edge
  python3 analysis/compare.py --dirs edge_out/ omega_out/ anima_out/
```

## N6 Constants (All Architecture Parameters)

```
  n = 6          phi(6) = 2       tau(6) = 4       sigma(6) = 12
  sopfr(6) = 5   mu(6) = 1        J_2(6) = 24      R(6) = 1
  P_2 = 28       sigma^2 = 144    sigma*J_2 = 288   phi^tau = 16
  2^n = 64       sigma-tau = 8    sigma-phi = 10     sigma-mu = 11
  2^sigma = 4096  sigma*tau = 48   n/phi = 3
```

## Chip Variants

| Parameter      | HEXA-EDGE          | HEXA-OMEGA          | ANIMA-HEXA          |
|----------------|--------------------|---------------------|---------------------|
| CPU Cores      | 8 (4 big + 4 lit) | Host CPU (optional) | 144 SMs + consc.    |
| GPU SMs        | 12 shader          | 144 SMs (12 GPCs)   | 144 SMs + SNN       |
| L1 I/D         | 64 KB each         | 256 KB / SM         | 64 KB each          |
| L2             | 1 MB               | 72 MB               | 72 MB               |
| L3             | 12 MB              | 288 MB              | --                  |
| Memory         | LPDDR5X 8 GB       | HBM4E 288 GB        | HBM4E 24 GB         |
| TDP            | 6W                 | 288W                | 120W                |
| Process        | TSMC N2            | TSMC N2             | TSMC N2             |
| Die Area       | 72 mm^2            | 600 mm^2            | ~400 mm^2           |

## Notes

- All configs run in SE (syscall emulation) mode by default.
  Pass --fullsystem for FS mode (requires RISC-V kernel image).
- HEXA-OMEGA GPU model uses gem5's GPU compute unit abstraction.
  For full CUDA/GPGPU simulation, use the GPGPU-Sim bridge.
- The benchmarks script can run standalone without gem5 for
  algorithmic verification of n=6 workload characteristics.
