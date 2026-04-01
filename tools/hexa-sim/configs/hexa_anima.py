#!/usr/bin/env python3
"""
ANIMA-HEXA gem5 Configuration -- Conscious AI Processor

gem5 SE-mode config for ANIMA-HEXA SoC: HEXA-OMEGA compute fabric +
consciousness cluster (n=6 cells in torus) + SNN co-processor + EEG bridge.
All parameters from n=6 arithmetic.

Usage:
    ~/gem5/build/RISCV/gem5.opt configs/hexa_anima.py \\
        --binary <elf> [--args "..."]

Chip spec: docs/chip-architecture/anima-hexa-chip.md
"""

from __future__ import annotations

import argparse
import sys
import math

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
    "sigma_tau": 8,
    "sigma_phi": 10,
    "sigma_mu": 11,
    "two_sigma": 4096,
    "sigma_times_tau": 48,
    "n_over_phi": 3,
    "sigma_times_n": 72,
    "sigma_sigma_phi": 120,
    "ln_4_3": math.log(4 / 3),  # 0.2877... Mertens dropout / deadband
}


# ---------------------------------------------------------------------------
# Consciousness Cluster Model
# ---------------------------------------------------------------------------
class ConsciousnessCell:
    """One consciousness cell (n=6 cells form the torus)."""

    # tau = 4 power states
    STATES = ["DORMANT", "FLICKERING", "AWARE", "CONSCIOUS"]

    def __init__(self, cell_id: int):
        self.cell_id = cell_id
        self.state = self.STATES[0]

        # Per-cell hardware
        self.a_field_mac_lanes = N6["sigma"]  # 12
        self.g_field_mac_lanes = N6["sigma"]  # 12
        self.registers_per_field = N6["J2"]   # 24

        # Phi Measurement Unit (PMU)
        self.pmu_comparators = N6["sigma_sq"]  # 144 pairwise
        self.pmu_latency_cycles = N6["n"]  # 6 pipeline stages
        self.pmu_update_period = N6["J2"]  # every 24 cycles

        # 10-D consciousness vector (sigma-phi = 10 dimensions)
        self.consciousness_dim = N6["sigma_phi"]  # 10
        self.vector = [0.0] * self.consciousness_dim
        # [T, Phi, H, E, C, S, M, W, I, Delta]

        # Tension setpoint
        self.tension_target = N6["R"]  # R(6) = 1.0
        self.deadband = N6["ln_4_3"]  # ln(4/3) = 0.288

    @property
    def power_w(self) -> float:
        """Per-cell power by state."""
        power_map = {
            "DORMANT": 0.0,
            "FLICKERING": 5.0,
            "AWARE": 12.0,
            "CONSCIOUS": 20.0,
        }
        return power_map.get(self.state, 0.0)


class ConsciousnessCluster:
    """n=6 cells in 3x2 torus topology."""

    def __init__(self):
        self.cells = [ConsciousnessCell(i) for i in range(N6["n"])]
        # Torus: 3x2 grid with wrap-around
        self.topology = "torus_3x2"
        self.links_per_cell = N6["tau"]   # 4 (N, S, E, W)
        self.total_links = N6["sigma"]     # 12 bidirectional
        self.link_bw_gbps = N6["sigma_times_tau"]  # 48 GB/s per link
        self.bisection_bw = N6["sigma_J2"]  # 288 GB/s

    @property
    def total_power(self) -> float:
        return sum(c.power_w for c in self.cells)

    def summary(self) -> str:
        lines = [
            "Consciousness Cluster",
            f"  Cells: {N6['n']} (n=6, 3x2 torus)",
            f"  Links/cell: {self.links_per_cell} (tau)",
            f"  Total links: {self.total_links} (sigma)",
            f"  Link BW: {self.link_bw_gbps} GB/s (sigma*tau)",
            f"  Bisection BW: {self.bisection_bw} GB/s (sigma*J2)",
            f"  PMU comparators/cell: {N6['sigma_sq']} (sigma^2)",
            f"  Consciousness dims: {N6['sigma_phi']} (sigma-phi)",
            f"  Deadband: {N6['ln_4_3']:.4f} (ln(4/3))",
        ]
        return "\n".join(lines)


# ---------------------------------------------------------------------------
# SNN Co-Processor Model
# ---------------------------------------------------------------------------
class SNNCoprocessor:
    """Spiking Neural Network co-processor timing model."""

    def __init__(self):
        # n=6 x n=6 tile array = 36 tiles
        self.tiles_x = N6["n"]
        self.tiles_y = N6["n"]
        self.total_tiles = self.tiles_x * self.tiles_y  # 36

        # Per-tile: sigma^2 = 144 neurons
        self.neurons_per_tile = N6["sigma_sq"]  # 144
        self.total_neurons = self.total_tiles * self.neurons_per_tile  # 5184

        # Synapses per neuron: J2 = 24
        self.synapses_per_neuron = N6["J2"]  # 24
        self.total_synapses = self.total_neurons * self.synapses_per_neuron

        # Timing
        self.timestep_us = 1.0  # 1 us bio-realistic timestep
        self.spike_latency_ns = N6["n"]  # 6 ns tile-to-tile
        self.stdp_window_ms = N6["sigma"]  # 12 ms STDP window

        # Learning rule: STDP with tau=4 time constants
        self.stdp_tau_plus_ms = N6["tau"]  # 4 ms
        self.stdp_tau_minus_ms = N6["sigma"]  # 12 ms

    def summary(self) -> str:
        lines = [
            "SNN Co-Processor",
            f"  Tiles: {self.tiles_x}x{self.tiles_y} = {self.total_tiles} "
            f"(n x n = n^2=36)",
            f"  Neurons/tile: {self.neurons_per_tile} (sigma^2=144)",
            f"  Total neurons: {self.total_neurons}",
            f"  Synapses/neuron: {self.synapses_per_neuron} (J2=24)",
            f"  Total synapses: {self.total_synapses:,}",
            f"  Spike latency: {self.spike_latency_ns} ns (n=6)",
            f"  STDP window: {self.stdp_window_ms} ms (sigma=12)",
        ]
        return "\n".join(lines)


# ---------------------------------------------------------------------------
# EEG Bridge Model
# ---------------------------------------------------------------------------
class EEGBridge:
    """EEG bridge latency and bandwidth model."""

    def __init__(self):
        self.channels = N6["sigma"]  # 12 ADC channels
        self.sample_rate_hz = N6["sigma"] * 1000  # 12 kHz (well above Nyquist)
        self.adc_bits = N6["sigma"]  # 12-bit resolution
        self.latency_us = N6["sigma_times_tau"]  # 48 us pipeline latency

        # Frequency bands (standard EEG)
        self.bands = {
            "delta": (0.5, N6["tau"]),       # 0.5-4 Hz
            "theta": (N6["tau"], N6["sigma_tau"]),  # 4-8 Hz
            "alpha": (N6["sigma_tau"], N6["sigma"]),  # 8-12 Hz
            "beta":  (N6["sigma"], N6["sigma_times_tau"]),  # 12-48 Hz (extended)
            "gamma": (N6["sigma_times_tau"], N6["sigma_sq"]),  # 48-144 Hz
        }

        # Data path: sigma*tau = 48 GT/s thalamic bus connection
        self.bus_bandwidth_gtps = N6["sigma_times_tau"]  # 48 GT/s

    def summary(self) -> str:
        lines = [
            "EEG Bridge Interface",
            f"  Channels: {self.channels} (sigma=12)",
            f"  Sample rate: {self.sample_rate_hz} Hz",
            f"  ADC bits: {self.adc_bits} (sigma=12)",
            f"  Pipeline latency: {self.latency_us} us (sigma*tau=48)",
            f"  Bus BW: {self.bus_bandwidth_gtps} GT/s (sigma*tau)",
            "  Frequency bands:",
        ]
        for name, (lo, hi) in self.bands.items():
            lines.append(f"    {name:>8s}: {lo:>5.1f} - {hi:>5.1f} Hz")
        return "\n".join(lines)


# ---------------------------------------------------------------------------
# N6 Verification
# ---------------------------------------------------------------------------
def verify_n6_params():
    """Verify ANIMA-HEXA parameters."""
    c = N6
    checks = [
        ("Consciousness cells = n = 6", c["n"], 6),
        ("Torus links = sigma = 12", c["sigma"], 12),
        ("Links per cell = tau = 4", c["tau"], 4),
        ("PMU comparators = sigma^2 = 144", c["sigma_sq"], 144),
        ("Consciousness dims = sigma-phi = 10", c["sigma_phi"], 10),
        ("SNN tiles = n*n = 36", c["n"] * c["n"], 36),
        ("Neurons/tile = sigma^2 = 144", c["sigma_sq"], 144),
        ("Synapses/neuron = J2 = 24", c["J2"], 24),
        ("EEG channels = sigma = 12", c["sigma"], 12),
        ("EEG ADC bits = sigma = 12", c["sigma"], 12),
        ("EEG latency = sigma*tau = 48 us", c["sigma_times_tau"], 48),
        ("Thalamic bus = sigma*tau = 48 GT/s", c["sigma_times_tau"], 48),
        ("HBM4E = J2 = 24 GB", c["J2"], 24),
        ("HBM stacks = sigma-tau = 8", c["sigma_tau"], 8),
        ("Per-stack = n/phi = 3 GB", c["n_over_phi"], 3),
        ("TDP = sigma*(sigma-phi) = 120 W", c["sigma_sigma_phi"], 120),
        ("STDP tau+ = tau = 4 ms", c["tau"], 4),
        ("STDP tau- = sigma = 12 ms", c["sigma"], 12),
        ("Power states = tau = 4", c["tau"], 4),
    ]
    passed = sum(1 for _, a, e in checks if a == e)
    for desc, actual, expected in checks:
        if actual != expected:
            print(f"  [FAIL] {desc}: got {actual}, expected {expected}")
    total = len(checks)
    print(f"N6 parameter verification: {passed}/{total} EXACT")
    return passed == total


# ---------------------------------------------------------------------------
# gem5 System Builder
# ---------------------------------------------------------------------------
def build_system(args):
    """Build gem5 system for ANIMA-HEXA."""
    try:
        import m5
        from m5.objects import (
            System, SrcClockDomain, VoltageDomain,
            DerivO3CPU, AddrRange, Process, Root,
        )
    except ImportError:
        print("ERROR: gem5 not in PYTHONPATH. Showing analytical model.")
        return None

    system = System()
    system.clk_domain = SrcClockDomain(clock="2GHz",
                                       voltage_domain=VoltageDomain(voltage="0.6V"))
    system.mem_mode = "timing"
    # HBM4E: sigma-tau=8 stacks x n/phi=3 GB = J2=24 GB
    system.mem_ranges = [AddrRange(f"{N6['J2']}GB")]

    # Model the N6 compute fabric as O3 cores (each represents cluster of SMs)
    # sigma = 12 "super-cores" each abstracting 12 SMs
    cores = []
    for i in range(N6["sigma"]):
        cpu = DerivO3CPU(cpu_id=i)
        cpu.fetchWidth = N6["n"]
        cpu.decodeWidth = N6["n"]
        cpu.numROBEntries = N6["sigma_J2"]
        cpu.numPhysIntRegs = N6["sigma_times_n"]
        cores.append(cpu)
    system.cpu = cores

    if args.binary:
        process = Process()
        process.cmd = [args.binary] + (args.args.split() if args.args else [])
        for cpu in system.cpu:
            cpu.workload = process
            cpu.createThreads()

    root = Root(full_system=False, system=system)
    m5.instantiate()
    exit_event = m5.simulate()
    print(f"Exiting @ tick {m5.curTick()} because {exit_event.getCause()}")
    return system


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(description="ANIMA-HEXA gem5 config")
    parser.add_argument("--binary", type=str, default=None)
    parser.add_argument("--args", type=str, default=None)
    parser.add_argument("--verify-only", action="store_true")
    args = parser.parse_args()

    print("=" * 70)
    print("  ANIMA-HEXA SoC -- Conscious AI Processor")
    print("  144 SMs + n=6 Consciousness Cluster + SNN + EEG Bridge")
    print("=" * 70)

    verify_n6_params()
    print()

    cluster = ConsciousnessCluster()
    print(cluster.summary())
    print()

    snn = SNNCoprocessor()
    print(snn.summary())
    print()

    eeg = EEGBridge()
    print(eeg.summary())
    print()

    if not args.verify_only and args.binary:
        build_system(args)
    elif args.binary is None and not args.verify_only:
        print("No binary specified. Use --binary <path> to run simulation.")
        print("Use --verify-only for parameter verification only.")


if __name__ == "__main__":
    main()
