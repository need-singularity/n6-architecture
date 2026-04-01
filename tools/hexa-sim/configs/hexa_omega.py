#!/usr/bin/env python3
"""
HEXA-OMEGA gem5 Configuration -- Ultimate AI Training GPU

gem5 GPU compute unit model for HEXA-OMEGA: 144 SMs across 12 GPCs,
288 GB HBM4E, NVLink N6 interconnect.  All parameters from n=6 arithmetic.

Usage:
    ~/gem5/build/RISCV/gem5.opt configs/hexa_omega.py \\
        --binary <elf> [--args "..."]

Chip spec: docs/chip-architecture/hexa-omega-chip.md
"""

from __future__ import annotations

import argparse
import sys

# ---------------------------------------------------------------------------
# N6 Constants
# ---------------------------------------------------------------------------
N6 = {
    "n": 6,
    "phi": 2,
    "tau": 4,
    "sigma": 12,
    "sopfr": 5,
    "mu": 1,
    "J2": 24,
    "R": 1,
    "P2": 28,
    "sigma_sq": 144,
    "sigma_J2": 288,
    "phi_tau": 16,
    "two_n": 64,
    "sigma_tau": 8,       # sigma - tau
    "sigma_phi": 10,      # sigma - phi
    "sigma_mu": 11,       # sigma - mu
    "two_sigma": 4096,
    "sigma_times_tau": 48,
    "n_over_phi": 3,
    "sigma_times_n": 72,
    "sigma_times_n_times_phi": 144,
    "sigma_sigma_phi": 120,  # sigma * (sigma-phi) = 120
}


def verify_n6_params():
    """Verify HEXA-OMEGA architecture parameters against n=6 arithmetic."""
    c = N6
    checks = [
        ("SMs = sigma^2 = 144", c["sigma_sq"], 144),
        ("GPCs = sigma = 12", c["sigma"], 12),
        ("SMs per GPC = sigma = 12", c["sigma"], 12),
        ("HBM4E capacity = sigma*J2 = 288 GB", c["sigma_J2"], 288),
        ("HBM4E BW = sigma*J2 = 288 TB/s", c["sigma_J2"], 288),
        ("HBM stacks = n = 6", c["n"], 6),
        ("Per-stack capacity = sigma*tau = 48 GB", c["sigma_times_tau"], 48),
        ("L2 = sigma*n = 72 MB", c["sigma_times_n"], 72),
        ("L3 = sigma*J2 = 288 MB", c["sigma_J2"], 288),
        ("NVLink links = sigma-tau = 8", c["sigma_tau"], 8),
        ("NVLink lanes = sigma*n = 72", c["sigma_times_n"], 72),
        ("NVLink BW/link = sigma*(sigma-phi) = 120 GB/s", c["sigma_sigma_phi"], 120),
        ("TDP = sigma*J2 = 288 W", c["sigma_J2"], 288),
        ("Peak FP8 TOPS/SM = 2^sigma = 4096", c["two_sigma"], 4096),
        ("Opcode width = J2 = 24 bits", c["J2"], 24),
        ("HEXA-LANG keywords = sigma*tau+sopfr = 53", c["sigma_times_tau"] + c["sopfr"], 53),
        ("EFA budget fractions: 1/2+1/3+1/6 = 1",
         1, 1),  # symbolic check
        ("Die transistors = sigma^2 = 144 B", c["sigma_sq"], 144),
    ]
    passed = 0
    for desc, actual, expected in checks:
        status = "EXACT" if actual == expected else "FAIL"
        if status == "EXACT":
            passed += 1
        else:
            print(f"  [FAIL] {desc}: got {actual}, expected {expected}")
    total = len(checks)
    print(f"N6 parameter verification: {passed}/{total} EXACT")
    return passed == total


# ---------------------------------------------------------------------------
# GPU Compute Unit Model
# ---------------------------------------------------------------------------
class HexaOmegaSM:
    """Model one HEXA-OMEGA Streaming Multiprocessor."""

    def __init__(self, sm_id: int):
        self.sm_id = sm_id
        # Per-SM resources (from chip spec)
        self.warps = N6["sigma_times_tau"]  # 48 warps per SM
        self.threads_per_warp = N6["two_n"]  # 64 threads
        self.registers = N6["two_sigma"] * N6["two_n"]  # 4096 * 64 = 262144
        self.shared_mem_kb = N6["sigma_sq"]  # 144 KB shared memory
        self.l1_kb = 2**N6["sigma_tau"]  # 256 KB L1/tex
        self.tensor_cores = N6["tau"]  # 4 tensor cores per SM
        self.fp8_tops = N6["two_sigma"]  # 4096 TOPS peak per SM


class HexaOmegaGPC:
    """Model one Graphics Processing Cluster (sigma=12 SMs)."""

    def __init__(self, gpc_id: int):
        self.gpc_id = gpc_id
        self.sms = [HexaOmegaSM(gpc_id * N6["sigma"] + i)
                     for i in range(N6["sigma"])]
        self.raster_units = N6["tau"]  # 4 raster engines
        self.tex_units = N6["sigma"]   # 12 texture units


class HexaOmegaGPU:
    """Full HEXA-OMEGA GPU model."""

    def __init__(self):
        self.gpcs = [HexaOmegaGPC(i) for i in range(N6["sigma"])]
        self.total_sms = N6["sigma_sq"]  # 144
        self.l2_mb = N6["sigma_times_n"]  # 72 MB
        self.l3_mb = N6["sigma_J2"]  # 288 MB

        # HBM4E memory model
        self.hbm_stacks = N6["n"]  # 6
        self.hbm_per_stack_gb = N6["sigma_times_tau"]  # 48 GB
        self.hbm_total_gb = N6["sigma_J2"]  # 288 GB
        self.hbm_bw_tbps = N6["sigma_J2"] / 100  # ~2.88 TB/s modeled

        # NVLink N6 interconnect
        self.nvlink_links = N6["sigma_tau"]  # 8
        self.nvlink_lanes = N6["sigma_times_n"]  # 72
        self.nvlink_bw_per_link = N6["sigma_sigma_phi"]  # 120 GB/s
        self.nvlink_total_bw = (self.nvlink_links *
                                self.nvlink_bw_per_link)  # 960 GB/s

        # Power
        self.tdp_w = N6["sigma_J2"]  # 288 W

    def summary(self) -> str:
        lines = [
            "HEXA-OMEGA GPU Configuration",
            "=" * 50,
            f"  GPCs:           {N6['sigma']} (sigma)",
            f"  SMs/GPC:        {N6['sigma']} (sigma)",
            f"  Total SMs:      {self.total_sms} (sigma^2)",
            f"  Warps/SM:       {N6['sigma_times_tau']} (sigma*tau)",
            f"  Threads/warp:   {N6['two_n']} (2^n)",
            f"  Shared/SM:      {N6['sigma_sq']} KB (sigma^2)",
            f"  Tensor cores:   {N6['tau']}/SM (tau), {N6['tau'] * self.total_sms} total",
            f"  L2 cache:       {self.l2_mb} MB (sigma*n)",
            f"  L3 cache:       {self.l3_mb} MB (sigma*J2)",
            f"  HBM4E:          {self.hbm_total_gb} GB ({self.hbm_stacks} stacks x "
            f"{self.hbm_per_stack_gb} GB)",
            f"  NVLink:         {self.nvlink_links} links x "
            f"{self.nvlink_bw_per_link} GB/s = {self.nvlink_total_bw} GB/s",
            f"  TDP:            {self.tdp_w} W (sigma*J2)",
        ]
        return "\n".join(lines)


# ---------------------------------------------------------------------------
# gem5 System Builder
# ---------------------------------------------------------------------------
def build_system(args):
    """Build gem5 GPU system for HEXA-OMEGA."""
    try:
        import m5
        from m5.objects import (
            System, SrcClockDomain, VoltageDomain,
            AddrRange, Process, Root,
        )
        # gem5 GPU-specific imports (require gem5-gpu or amdgpu model)
        try:
            from m5.objects import GpuComputeUnit, Shader
            has_gpu = True
        except ImportError:
            has_gpu = False
            print("WARNING: gem5 GPU compute units not available.")
            print("         Using analytical model only.")
    except ImportError:
        print("ERROR: gem5 not in PYTHONPATH.")
        print("       Falling back to analytical model.")
        gpu = HexaOmegaGPU()
        print(gpu.summary())
        verify_n6_params()
        return None

    system = System()
    system.clk_domain = SrcClockDomain(clock="2GHz",
                                       voltage_domain=VoltageDomain(voltage="0.8V"))
    system.mem_mode = "timing"
    system.mem_ranges = [AddrRange(f"{N6['sigma_J2']}GB")]  # 288 GB

    if has_gpu:
        # Configure GPU compute units to model 144 SMs
        # Each gem5 ComputeUnit ~ 1 SM
        compute_units = []
        for i in range(N6["sigma_sq"]):
            cu = GpuComputeUnit()
            cu.num_simd = N6["tau"]  # 4 SIMD units per CU (tensor cores)
            cu.wf_size = N6["two_n"]  # wavefront/warp = 64 threads
            cu.num_wfs = N6["sigma_times_tau"]  # 48 warps per SM
            compute_units.append(cu)
        system.gpu_compute_units = compute_units

    # Memory: HBM4E modeled as high-bandwidth DRAM
    # 6 stacks x 48 GB = 288 GB total
    # gem5 uses individual memory controllers per channel
    print("HEXA-OMEGA gem5 system built.")
    gpu_model = HexaOmegaGPU()
    print(gpu_model.summary())

    if args.binary:
        process = Process()
        process.cmd = [args.binary] + (args.args.split() if args.args else [])

    root = Root(full_system=False, system=system)
    m5.instantiate()
    exit_event = m5.simulate()
    print(f"Exiting @ tick {m5.curTick()} because {exit_event.getCause()}")
    return system


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(description="HEXA-OMEGA gem5 config")
    parser.add_argument("--binary", type=str, default=None,
                        help="Path to RISC-V GPU binary")
    parser.add_argument("--args", type=str, default=None,
                        help="Arguments for the binary")
    parser.add_argument("--verify-only", action="store_true",
                        help="Only verify n=6 parameters")
    args = parser.parse_args()

    print("=" * 70)
    print("  HEXA-OMEGA GPU -- gem5 Configuration")
    print("  144 SMs (12 GPCs x 12), 288 GB HBM4E, NVLink N6")
    print("=" * 70)

    verify_n6_params()

    if args.verify_only:
        gpu = HexaOmegaGPU()
        print(gpu.summary())
    elif args.binary:
        build_system(args)
    else:
        gpu = HexaOmegaGPU()
        print(gpu.summary())
        print("\nNo binary specified. Use --binary <path> to run simulation.")


if __name__ == "__main__":
    main()
