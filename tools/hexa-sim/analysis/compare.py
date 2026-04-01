#!/usr/bin/env python3
"""
Multi-Configuration Comparison Tool for HEXA gem5 Simulations

Parses gem5 stats from multiple simulation runs and generates comparison
tables with IPC, CPI, cache miss rates, bandwidth, and ASCII bar charts.

Usage:
    python3 analysis/compare.py --dirs edge_out/ omega_out/ anima_out/
    python3 analysis/compare.py --files stats_edge.txt stats_omega.txt
    python3 analysis/compare.py --demo  # run with sample data

All reference values from n=6 arithmetic.
"""

from __future__ import annotations

import argparse
import os
import math
import sys
from typing import Dict, List, Tuple, Optional

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
SIGMA_SQ = 144
SIGMA_J2 = 288
SIGMA_TIMES_TAU = 48
TWO_N = 64


# ---------------------------------------------------------------------------
# Stats Parser
# ---------------------------------------------------------------------------
def parse_stats(path: str) -> Dict[str, float]:
    """Parse a gem5 stats.txt file into a dict of metric -> value."""
    stats = {}
    try:
        with open(path) as f:
            for line in f:
                line = line.strip()
                if not line or line.startswith("#") or line.startswith("---"):
                    continue
                parts = line.split()
                if len(parts) >= 2:
                    key = parts[0]
                    try:
                        stats[key] = float(parts[1])
                    except ValueError:
                        pass
    except FileNotFoundError:
        print(f"WARNING: {path} not found")
    return stats


def find_stats_file(directory: str) -> Optional[str]:
    """Find stats.txt in a gem5 output directory."""
    candidates = ["stats.txt", "m5out/stats.txt"]
    for c in candidates:
        p = os.path.join(directory, c)
        if os.path.exists(p):
            return p
    # Search recursively
    for root, dirs, files in os.walk(directory):
        for f in files:
            if f == "stats.txt":
                return os.path.join(root, f)
    return None


# ---------------------------------------------------------------------------
# Metric Extraction
# ---------------------------------------------------------------------------
def extract_metrics(stats: Dict[str, float], label: str = "") -> Dict[str, float]:
    """Extract key comparison metrics from raw gem5 stats."""
    def get(key: str, default: float = 0.0) -> float:
        # Try exact match, then prefix match
        if key in stats:
            return stats[key]
        for k, v in stats.items():
            if key in k:
                return v
        return default

    cycles = get("sim_ticks", 1) / get("sim_freq", 1e12)  # approximate
    insts = get("sim_insts", get("committedInsts", 0))
    ops = get("sim_ops", insts)

    ipc = insts / max(get("numCycles", 1), 1)
    cpi = 1.0 / ipc if ipc > 0 else float("inf")

    # Cache metrics
    l1i_hits = get("icache.overall_hits", 0)
    l1i_misses = get("icache.overall_misses", 0)
    l1d_hits = get("dcache.overall_hits", 0)
    l1d_misses = get("dcache.overall_misses", 0)
    l2_hits = get("l2cache.overall_hits", get("l2.overall_hits", 0))
    l2_misses = get("l2cache.overall_misses", get("l2.overall_misses", 0))
    l3_hits = get("l3cache.overall_hits", get("l3.overall_hits", 0))
    l3_misses = get("l3cache.overall_misses", get("l3.overall_misses", 0))

    def miss_rate(hits, misses):
        total = hits + misses
        return (misses / total * 100) if total > 0 else 0.0

    # Bandwidth
    bytes_read = get("bytesRead", get("bw_total::total", 0))
    sim_seconds = get("sim_seconds", 1.0)
    bw_gbps = bytes_read / sim_seconds / 1e9 if sim_seconds > 0 else 0

    return {
        "label": label,
        "sim_seconds": get("sim_seconds", 0),
        "sim_ticks": get("sim_ticks", 0),
        "instructions": insts,
        "ops": ops,
        "cycles": get("numCycles", 0),
        "IPC": round(ipc, 4),
        "CPI": round(cpi, 4),
        "L1I_miss%": round(miss_rate(l1i_hits, l1i_misses), 2),
        "L1D_miss%": round(miss_rate(l1d_hits, l1d_misses), 2),
        "L2_miss%": round(miss_rate(l2_hits, l2_misses), 2),
        "L3_miss%": round(miss_rate(l3_hits, l3_misses), 2),
        "BW_GB/s": round(bw_gbps, 2),
    }


# ---------------------------------------------------------------------------
# Demo Data (when no gem5 stats available)
# ---------------------------------------------------------------------------
def demo_data() -> List[Dict[str, float]]:
    """Generate realistic demo metrics for the three HEXA chips."""
    return [
        {
            "label": "HEXA-EDGE (big)",
            "sim_seconds": 0.001,
            "instructions": 3000000,
            "cycles": 1000000,
            "IPC": 3.0,
            "CPI": 0.333,
            "L1I_miss%": 1.2,
            "L1D_miss%": 4.8,
            "L2_miss%": 12.0,
            "L3_miss%": 2.4,  # J2/sigma_phi percent
            "BW_GB/s": SIGMA_TIMES_TAU * 0.7,  # 70% of 48 GB/s
        },
        {
            "label": "HEXA-EDGE (little)",
            "sim_seconds": 0.002,
            "instructions": 1500000,
            "cycles": 1000000,
            "IPC": 1.5,
            "CPI": 0.667,
            "L1I_miss%": 0.8,
            "L1D_miss%": 3.6,
            "L2_miss%": 8.0,
            "L3_miss%": 1.8,
            "BW_GB/s": SIGMA_TIMES_TAU * 0.4,
        },
        {
            "label": "HEXA-OMEGA (SM avg)",
            "sim_seconds": 0.0005,
            "instructions": 50000000,
            "cycles": 1000000,
            "IPC": 50.0,  # many-thread aggregate
            "CPI": 0.02,
            "L1I_miss%": 0.5,
            "L1D_miss%": 6.0,
            "L2_miss%": 18.0,
            "L3_miss%": 3.6,
            "BW_GB/s": 2880 * 0.75,  # 75% of 2880 GB/s
        },
        {
            "label": "ANIMA-HEXA (consc.)",
            "sim_seconds": 0.001,
            "instructions": 10000000,
            "cycles": 1000000,
            "IPC": 10.0,
            "CPI": 0.1,
            "L1I_miss%": 2.0,
            "L1D_miss%": 8.0,
            "L2_miss%": 15.0,
            "L3_miss%": 4.0,
            "BW_GB/s": 2000 * 0.6,
        },
    ]


# ---------------------------------------------------------------------------
# ASCII Bar Chart
# ---------------------------------------------------------------------------
def ascii_bar(value: float, max_val: float, width: int = 40,
              char: str = "#") -> str:
    """Render a single ASCII bar."""
    if max_val <= 0:
        return ""
    fill = int(value / max_val * width)
    fill = max(0, min(width, fill))
    return char * fill + "." * (width - fill)


def ascii_bar_chart(metrics_list: List[Dict], key: str,
                    title: str = "", width: int = 40) -> str:
    """Render grouped ASCII bar chart for a single metric."""
    values = [(m["label"], m.get(key, 0)) for m in metrics_list]
    max_val = max(v for _, v in values) if values else 1
    if max_val == 0:
        max_val = 1

    lines = []
    if title:
        lines.append(f"  {title}")
    lines.append(f"  {'':20s} {'0':>{1}} {'':{width-6}} {max_val:>.2f}")
    lines.append(f"  {'':20s} |{'='*width}|")
    for label, val in values:
        short = label[:20]
        bar = ascii_bar(val, max_val, width)
        lines.append(f"  {short:<20s} |{bar}| {val:.2f}")
    lines.append(f"  {'':20s} |{'='*width}|")
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Comparison Table
# ---------------------------------------------------------------------------
def comparison_table(metrics_list: List[Dict]) -> str:
    """Generate a formatted comparison table."""
    if not metrics_list:
        return "  No data to compare."

    # Columns to display
    cols = ["IPC", "CPI", "L1I_miss%", "L1D_miss%", "L2_miss%",
            "L3_miss%", "BW_GB/s"]

    # Header
    max_label = max(len(m["label"]) for m in metrics_list)
    col_w = 12
    header = f"  {'Config':<{max_label}s}"
    for c in cols:
        header += f" {c:>{col_w}s}"
    sep = "  " + "-" * (max_label + len(cols) * (col_w + 1))

    lines = [sep, header, sep]
    for m in metrics_list:
        row = f"  {m['label']:<{max_label}s}"
        for c in cols:
            val = m.get(c, 0)
            row += f" {val:>{col_w}.4f}"
        lines.append(row)
    lines.append(sep)
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# N6 Architecture Reference Verification
# ---------------------------------------------------------------------------
def n6_reference_table() -> str:
    """Print expected n=6 architectural parameters for reference."""
    lines = [
        "  N6 Architectural Reference",
        "  " + "-" * 55,
        f"  {'Parameter':<30s} {'Value':>10s} {'Source':>15s}",
        "  " + "-" * 55,
        f"  {'Decode width (big)':30s} {N:>10d} {'n':>15s}",
        f"  {'Decode width (little)':30s} {N // PHI:>10d} {'n/phi':>15s}",
        f"  {'Pipeline (big)':30s} {SIGMA:>10d} {'sigma':>15s}",
        f"  {'Pipeline (little)':30s} {N:>10d} {'n':>15s}",
        f"  {'ROB entries':30s} {SIGMA_J2:>10d} {'sigma*J2':>15s}",
        f"  {'Phys regs':30s} {SIGMA * N:>10d} {'sigma*n':>15s}",
        f"  {'IQ entries':30s} {SIGMA_SQ:>10d} {'sigma^2':>15s}",
        f"  {'LQ entries':30s} {SIGMA * N:>10d} {'sigma*n':>15s}",
        f"  {'SQ entries':30s} {SIGMA_TIMES_TAU:>10d} {'sigma*tau':>15s}",
        f"  {'L1 (KB)':30s} {2**N:>10d} {'2^n':>15s}",
        f"  {'L2 (MB)':30s} {2**SIGMA_PHI // 1024:>10d} {'2^(sigma-phi)':>15s}",
        f"  {'L3 (MB)':30s} {SIGMA:>10d} {'sigma':>15s}",
        f"  {'LPDDR5X (GB)':30s} {SIGMA_TAU:>10d} {'sigma-tau':>15s}",
        f"  {'Cache line (B)':30s} {TWO_N:>10d} {'2^n':>15s}",
        f"  {'BW target (GB/s)':30s} {SIGMA_TIMES_TAU:>10d} {'sigma*tau':>15s}",
        f"  {'GPU SMs (OMEGA)':30s} {SIGMA_SQ:>10d} {'sigma^2':>15s}",
        f"  {'HBM4E (GB, OMEGA)':30s} {SIGMA_J2:>10d} {'sigma*J2':>15s}",
        f"  {'HBM4E (GB, ANIMA)':30s} {J2:>10d} {'J2':>15s}",
        "  " + "-" * 55,
    ]
    return "\n".join(lines)


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(description="HEXA gem5 Comparison Tool")
    parser.add_argument("--dirs", nargs="+", type=str, default=None,
                        help="gem5 output directories to compare")
    parser.add_argument("--files", nargs="+", type=str, default=None,
                        help="gem5 stats.txt files directly")
    parser.add_argument("--demo", action="store_true",
                        help="Run with demo/sample data")
    parser.add_argument("--reference", action="store_true",
                        help="Show n=6 architectural reference table")
    args = parser.parse_args()

    print("=" * 70)
    print("  HEXA gem5 Simulation Comparison")
    print("=" * 70)

    metrics_list = []

    if args.demo or (args.dirs is None and args.files is None):
        print("\n  Using demo data (run with --dirs or --files for real results)")
        metrics_list = demo_data()
    elif args.files:
        for f in args.files:
            stats = parse_stats(f)
            label = os.path.basename(f).replace("stats_", "").replace(".txt", "")
            metrics_list.append(extract_metrics(stats, label))
    elif args.dirs:
        for d in args.dirs:
            sf = find_stats_file(d)
            if sf:
                stats = parse_stats(sf)
                label = os.path.basename(d.rstrip("/"))
                metrics_list.append(extract_metrics(stats, label))
            else:
                print(f"  WARNING: No stats.txt found in {d}")

    if not metrics_list:
        print("  No metrics to compare. Use --demo for sample output.")
        return

    # Comparison table
    print(f"\n{comparison_table(metrics_list)}")

    # ASCII bar charts for key metrics
    chart_metrics = [
        ("IPC", "Instructions Per Cycle"),
        ("L1D_miss%", "L1D Cache Miss Rate (%)"),
        ("L2_miss%", "L2 Cache Miss Rate (%)"),
        ("BW_GB/s", "Memory Bandwidth (GB/s)"),
    ]
    for key, title in chart_metrics:
        print(f"\n{ascii_bar_chart(metrics_list, key, title)}")

    # N6 reference
    if args.reference or args.demo:
        print(f"\n{n6_reference_table()}")

    # Relative comparison (normalize to first entry)
    if len(metrics_list) > 1:
        base = metrics_list[0]
        print(f"\n  Relative to {base['label']}:")
        print(f"  {'-'*60}")
        for m in metrics_list[1:]:
            ipc_ratio = m["IPC"] / base["IPC"] if base["IPC"] > 0 else 0
            bw_ratio = m["BW_GB/s"] / base["BW_GB/s"] if base["BW_GB/s"] > 0 else 0
            print(f"    {m['label']:<25s}  IPC: {ipc_ratio:>5.2f}x  "
                  f"BW: {bw_ratio:>5.2f}x")


if __name__ == "__main__":
    main()
