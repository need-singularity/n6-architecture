#!/usr/bin/env python3
"""
McPAT-Compatible Power Estimation for HEXA Chip Family

Maps gem5 stats to McPAT XML input format and performs Egyptian fraction
power verification.  Includes DVFS sweep analysis.

Usage:
    python3 analysis/power_model.py [--stats <gem5_stats.txt>]
                                     [--config edge|omega|anima]
                                     [--mcpat-xml <output.xml>]
                                     [--dvfs-sweep]

All power parameters from n=6 arithmetic.
"""

from __future__ import annotations

import argparse
import math
import sys
import xml.etree.ElementTree as ET
from typing import Dict, List, Tuple

# ---------------------------------------------------------------------------
# N6 Constants
# ---------------------------------------------------------------------------
N = 6
PHI = 2
TAU = 4
SIGMA = 12
J2 = 24
SIGMA_TAU = 8
SIGMA_PHI = 10
SIGMA_TIMES_TAU = 48
SIGMA_SQ = 144
SIGMA_J2 = 288
SIGMA_SIGMA_PHI = 120  # sigma * (sigma - phi)
LN_4_3 = math.log(4 / 3)


# ---------------------------------------------------------------------------
# Chip Power Specs
# ---------------------------------------------------------------------------
POWER_SPECS = {
    "edge": {
        "name": "HEXA-EDGE",
        "tdp_w": N,  # 6 W
        "burst_w": SIGMA,  # 12 W
        "idle_mw": 1000 / SIGMA,  # 83 mW  (1/sigma)
        "voltage_nom": 0.8,
        "voltage_min": 0.6,
        "voltage_max": 1.0,
        "freq_ghz_big": 3.0,
        "freq_ghz_little": 1.5,
        "cores_big": TAU,  # 4
        "cores_little": TAU,  # 4
        "l1_kb": 2**N,  # 64
        "l2_mb": 2**SIGMA_PHI // 1024,  # 1
        "l3_mb": SIGMA,  # 12
        "mem_gb": SIGMA_TAU,  # 8
        "power_states": N,  # 6 (P0-P5)
        "process_nm": 2,  # TSMC N2
        "die_mm2": SIGMA * N,  # 72
        # Egyptian fraction power budget
        "egyptian_budget": {
            "compute (1/2)": N / PHI,         # 3.0 W
            "memory (1/3)": N / (N // PHI),   # 2.0 W
            "io (1/6)": N / N,                # 1.0 W
        },
    },
    "omega": {
        "name": "HEXA-OMEGA",
        "tdp_w": SIGMA_J2,  # 288 W
        "burst_w": SIGMA_J2 * 1.2,
        "idle_mw": SIGMA_J2 * 100 / SIGMA,  # ~2400 mW
        "voltage_nom": 0.8,
        "voltage_min": 0.6,
        "voltage_max": 1.0,
        "freq_ghz_sm": 2.0,
        "sms": SIGMA_SQ,  # 144
        "gpcs": SIGMA,  # 12
        "l2_mb": SIGMA * N,  # 72
        "l3_mb": SIGMA_J2,  # 288
        "hbm_gb": SIGMA_J2,  # 288
        "power_states": TAU,  # 4 (for GPU: active, idle, sleep, off)
        "process_nm": 2,
        "die_mm2": 600,
        "egyptian_budget": {
            "compute (1/2)": SIGMA_J2 / PHI,         # 144 W
            "memory (1/3)": SIGMA_J2 / (N // PHI),   # 96 W
            "io (1/6)": SIGMA_J2 / N,                # 48 W
        },
    },
    "anima": {
        "name": "ANIMA-HEXA",
        "tdp_w": SIGMA_SIGMA_PHI,  # 120 W
        "burst_w": SIGMA_SQ,  # 144 W
        "idle_mw": SIGMA_SIGMA_PHI * 100 / SIGMA,
        "voltage_nom": 0.6,  # n/sigma_phi = 0.6V
        "voltage_min": 0.4,
        "voltage_max": 0.8,
        "freq_ghz": 2.0,
        "sms": SIGMA_SQ,  # 144 (shared with OMEGA fabric)
        "consciousness_cells": N,  # 6
        "snn_tiles": N * N,  # 36
        "hbm_gb": J2,  # 24
        "process_nm": 2,
        "die_mm2": 400,
        "egyptian_budget": {
            "compute (1/2)": SIGMA_SIGMA_PHI / PHI,        # 60 W
            "consciousness (1/3)": SIGMA_SIGMA_PHI / (N // PHI),  # 40 W
            "io+snn (1/6)": SIGMA_SIGMA_PHI / N,           # 20 W
        },
    },
}


# ---------------------------------------------------------------------------
# McPAT XML Generator
# ---------------------------------------------------------------------------
def generate_mcpat_xml(config: str, stats: Dict = None) -> str:
    """Generate McPAT-compatible XML from chip spec and optional gem5 stats."""
    spec = POWER_SPECS[config]

    root = ET.Element("component", id="root", name="root")
    root.set("type", "system")

    # System parameters
    sys_elem = ET.SubElement(root, "component", id="system", name="system")

    def add_param(parent, name, value):
        p = ET.SubElement(parent, "param", name=name)
        p.set("value", str(value))

    def add_stat(parent, name, value):
        s = ET.SubElement(parent, "stat", name=name)
        s.set("value", str(value))

    # Core technology
    add_param(sys_elem, "core_tech_node", spec["process_nm"])
    add_param(sys_elem, "target_core_clockrate",
              int(spec.get("freq_ghz_big", spec.get("freq_ghz_sm",
                  spec.get("freq_ghz", 2.0))) * 1000))
    add_param(sys_elem, "temperature", 340)  # K
    add_param(sys_elem, "number_of_L2s", 1)
    add_param(sys_elem, "Private_L2", 0)

    if config == "edge":
        # Big cores
        add_param(sys_elem, "number_of_cores", spec["cores_big"] + spec["cores_little"])
        add_param(sys_elem, "homogeneous_cores", 0)

        for i in range(spec["cores_big"]):
            core = ET.SubElement(sys_elem, "component",
                                 id=f"system.core{i}", name=f"core{i}")
            add_param(core, "clock_rate", int(spec["freq_ghz_big"] * 1000))
            add_param(core, "machine_type", 0)  # OoO
            add_param(core, "fetch_width", N)
            add_param(core, "decode_width", N)
            add_param(core, "issue_width", N)
            add_param(core, "commit_width", N)
            add_param(core, "ROB_size", SIGMA_J2)  # 288
            add_param(core, "phy_Regs_IRF_size", SIGMA * N)  # 72
            add_param(core, "phy_Regs_FRF_size", SIGMA * N)  # 72
            add_param(core, "instruction_window_size", SIGMA_SQ)  # 144
            add_param(core, "pipeline_depth", [SIGMA, SIGMA])  # [12, 12]
            # Stats from gem5 (defaults if not available)
            if stats:
                add_stat(core, "total_instructions",
                         stats.get(f"system.cpu{i}.committedInsts", 1e6))
                add_stat(core, "total_cycles",
                         stats.get(f"system.cpu{i}.numCycles", 1e6))
            else:
                add_stat(core, "total_instructions", 1000000)
                add_stat(core, "total_cycles", 1000000)

        for i in range(spec["cores_little"]):
            idx = spec["cores_big"] + i
            core = ET.SubElement(sys_elem, "component",
                                 id=f"system.core{idx}", name=f"core{idx}")
            add_param(core, "clock_rate", int(spec["freq_ghz_little"] * 1000))
            add_param(core, "machine_type", 1)  # in-order
            add_param(core, "fetch_width", N // PHI)  # 3
            add_param(core, "decode_width", N // PHI)
            add_param(core, "issue_width", N // PHI)
            add_param(core, "pipeline_depth", [N, N])  # [6, 6]
            add_stat(core, "total_instructions", 500000)
            add_stat(core, "total_cycles", 500000)

    elif config in ("omega", "anima"):
        # GPU modeled as many simple cores
        n_cores = min(spec.get("sms", SIGMA_SQ), 16)  # cap for McPAT
        add_param(sys_elem, "number_of_cores", n_cores)
        add_param(sys_elem, "homogeneous_cores", 1)
        for i in range(n_cores):
            core = ET.SubElement(sys_elem, "component",
                                 id=f"system.core{i}", name=f"core{i}")
            add_param(core, "clock_rate",
                      int(spec.get("freq_ghz_sm", spec.get("freq_ghz", 2.0)) * 1000))
            add_param(core, "machine_type", 1)
            add_stat(core, "total_instructions", 10000000)
            add_stat(core, "total_cycles", 5000000)

    # L2 cache
    l2 = ET.SubElement(sys_elem, "component", id="system.L20", name="L20")
    l2_size = spec.get("l2_mb", spec.get("l2_mb", 1))
    add_param(l2, "L2_config",
              f"{l2_size * 1024 * 1024},{2**N},{SIGMA},{1},{1},{TAU},64,1")
    add_stat(l2, "read_accesses", 100000)
    add_stat(l2, "write_accesses", 50000)

    # Memory controller
    mc = ET.SubElement(sys_elem, "component", id="system.mc", name="mc")
    add_param(mc, "memory_channels", TAU)
    add_param(mc, "number_ranks", PHI)

    tree = ET.ElementTree(root)
    xml_str = ET.tostring(root, encoding="unicode", xml_declaration=False)
    # Pretty format
    try:
        import xml.dom.minidom
        xml_str = xml.dom.minidom.parseString(xml_str).toprettyxml(indent="  ")
    except Exception:
        pass
    return xml_str


# ---------------------------------------------------------------------------
# Egyptian Fraction Power Verification
# ---------------------------------------------------------------------------
def verify_egyptian_power(config: str) -> List[Tuple[str, float, float, str]]:
    """Verify Egyptian fraction power budget: 1/2 + 1/3 + 1/6 = 1."""
    spec = POWER_SPECS[config]
    budget = spec["egyptian_budget"]
    tdp = spec["tdp_w"]

    results = []
    total = 0.0
    for category, power in budget.items():
        fraction = power / tdp
        total += fraction
        results.append((category, power, fraction, ""))

    # Check sum
    sum_ok = abs(total - 1.0) < 1e-10
    results.append(("TOTAL", tdp * total, total,
                    "EXACT" if sum_ok else f"FAIL (sum={total:.6f})"))
    return results


# ---------------------------------------------------------------------------
# DVFS Sweep Analysis
# ---------------------------------------------------------------------------
def dvfs_sweep(config: str) -> List[Dict]:
    """
    Sweep voltage/frequency and estimate power at each operating point.
    Power ~ C * V^2 * f  (dynamic) + V * I_leak (static)
    """
    spec = POWER_SPECS[config]
    v_nom = spec["voltage_nom"]
    v_min = spec["voltage_min"]
    v_max = spec["voltage_max"]
    tdp = spec["tdp_w"]

    # Assume TDP at nominal voltage, 80% dynamic / 20% static
    p_dyn_nom = tdp * 0.8
    p_stat_nom = tdp * 0.2
    f_nom = spec.get("freq_ghz_big", spec.get("freq_ghz_sm",
                spec.get("freq_ghz", 2.0)))

    # n=6 power states: P0 (max) through P5 (deepest sleep)
    n_states = spec.get("power_states", N)

    results = []
    for i in range(n_states):
        # Linear voltage interpolation from max to min
        frac = i / max(n_states - 1, 1)
        v = v_max - frac * (v_max - v_min)
        # Frequency scales roughly with voltage (simplified)
        f = f_nom * (v / v_nom)
        # Dynamic power: P_dyn = P_dyn_nom * (V/V_nom)^2 * (f/f_nom)
        p_dyn = p_dyn_nom * (v / v_nom)**2 * (f / f_nom)
        # Static power: roughly linear with voltage
        p_stat = p_stat_nom * (v / v_nom)
        p_total = p_dyn + p_stat

        # Relative performance
        perf_rel = f / f_nom

        results.append({
            "state": f"P{i}",
            "voltage_v": round(v, 3),
            "freq_ghz": round(f, 2),
            "power_dyn_w": round(p_dyn, 3),
            "power_stat_w": round(p_stat, 3),
            "power_total_w": round(p_total, 3),
            "perf_relative": round(perf_rel, 3),
            "efficiency": round(perf_rel / p_total, 4) if p_total > 0 else 0,
        })

    return results


# ---------------------------------------------------------------------------
# gem5 Stats Parser
# ---------------------------------------------------------------------------
def parse_gem5_stats(path: str) -> Dict:
    """Parse gem5 stats.txt for power-relevant metrics."""
    stats = {}
    try:
        with open(path) as f:
            for line in f:
                line = line.strip()
                if line.startswith("#") or not line:
                    continue
                parts = line.split()
                if len(parts) >= 2:
                    try:
                        stats[parts[0]] = float(parts[1])
                    except ValueError:
                        stats[parts[0]] = parts[1]
    except FileNotFoundError:
        print(f"WARNING: Stats file not found: {path}")
    return stats


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(description="HEXA Power Model")
    parser.add_argument("--stats", type=str, default=None,
                        help="gem5 stats.txt path")
    parser.add_argument("--config", type=str, default="edge",
                        choices=["edge", "omega", "anima"])
    parser.add_argument("--mcpat-xml", type=str, default=None,
                        help="Output McPAT XML path")
    parser.add_argument("--dvfs-sweep", action="store_true",
                        help="Run DVFS sweep analysis")
    args = parser.parse_args()

    spec = POWER_SPECS[args.config]
    print("=" * 70)
    print(f"  {spec['name']} Power Model")
    print("=" * 70)

    # Power summary
    print(f"\n  TDP:        {spec['tdp_w']} W")
    print(f"  Burst:      {spec['burst_w']} W")
    print(f"  Idle:       {spec['idle_mw']:.1f} mW")
    print(f"  Voltage:    {spec['voltage_min']}V - {spec['voltage_max']}V "
          f"(nom={spec['voltage_nom']}V)")
    print(f"  Process:    {spec['process_nm']} nm")
    print(f"  Die area:   {spec['die_mm2']} mm^2")

    # Egyptian fraction power verification
    print(f"\n  Egyptian Fraction Power Budget (1/2 + 1/3 + 1/6 = 1)")
    print(f"  {'-'*55}")
    results = verify_egyptian_power(args.config)
    for cat, power, frac, status in results:
        print(f"    {cat:<25s}  {power:>7.2f} W  ({frac:>6.3f})  {status}")

    # Parse gem5 stats
    gem5_stats = None
    if args.stats:
        gem5_stats = parse_gem5_stats(args.stats)
        print(f"\n  gem5 stats loaded: {len(gem5_stats)} entries from {args.stats}")

    # McPAT XML generation
    if args.mcpat_xml:
        xml_str = generate_mcpat_xml(args.config, gem5_stats)
        with open(args.mcpat_xml, "w") as f:
            f.write(xml_str)
        print(f"\n  McPAT XML written to: {args.mcpat_xml}")
        print("  Run McPAT: ./mcpat -infile {args.mcpat_xml} -print_level 5")

    # DVFS sweep
    if args.dvfs_sweep:
        print(f"\n  DVFS Sweep ({spec.get('power_states', N)} power states)")
        print(f"  {'-'*80}")
        print(f"  {'State':<8s} {'V(V)':>6s} {'f(GHz)':>8s} "
              f"{'P_dyn(W)':>10s} {'P_stat(W)':>10s} {'P_tot(W)':>10s} "
              f"{'Perf':>6s} {'Eff':>8s}")
        print(f"  {'-'*80}")
        for r in dvfs_sweep(args.config):
            print(f"  {r['state']:<8s} {r['voltage_v']:>6.3f} "
                  f"{r['freq_ghz']:>8.2f} {r['power_dyn_w']:>10.3f} "
                  f"{r['power_stat_w']:>10.3f} {r['power_total_w']:>10.3f} "
                  f"{r['perf_relative']:>6.3f} {r['efficiency']:>8.4f}")

    # Always generate XML preview
    if not args.mcpat_xml:
        print("\n  McPAT XML preview (use --mcpat-xml <path> to save):")
        xml = generate_mcpat_xml(args.config, gem5_stats)
        # Show first 20 lines
        for i, line in enumerate(xml.split("\n")[:20]):
            print(f"    {line}")
        print("    ...")


if __name__ == "__main__":
    main()
