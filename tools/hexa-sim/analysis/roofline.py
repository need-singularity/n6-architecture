#!/usr/bin/env python3
"""
Roofline Model Generator for HEXA Chip Family

Plots compute ceiling vs memory bandwidth with AI workload markers.
Compares HEXA-EDGE, HEXA-OMEGA, H100, and Apple M4.

Usage:
    python3 analysis/roofline.py [--stats <gem5_stats.txt>]
                                  [--output roofline.png]
                                  [--chip edge|omega|anima|all]
                                  [--no-plot]  # text-only ASCII output

All chip parameters from n=6 arithmetic.
"""

from __future__ import annotations

import argparse
import math
import sys
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
TWO_N = 64
TWO_SIGMA = 4096
SIGMA_SQ = 144
SIGMA_J2 = 288
SIGMA_TIMES_TAU = 48


# ---------------------------------------------------------------------------
# Chip Specs (peak compute TFLOPS, peak BW GB/s)
# ---------------------------------------------------------------------------
CHIPS = {
    "HEXA-EDGE": {
        "peak_fp16_tflops": TWO_SIGMA / 1000 * SIGMA,  # ~49 TFLOPS (GPU SCs)
        "peak_int8_tops": 72,   # 72 TOPS NPU
        "mem_bw_gbps": SIGMA_TIMES_TAU,  # 48 GB/s LPDDR5X
        "tdp_w": N,  # 6 W
        "color": "blue",
    },
    "HEXA-OMEGA": {
        "peak_fp16_tflops": 295.0,  # ~295 PFLOPS (from spec, FP8/phi)
        "peak_int8_tops": 590.0,
        "mem_bw_gbps": 2880,  # sigma*J2 = 288 TB/s modeled as 2880 GB/s
        "tdp_w": SIGMA_J2,  # 288 W
        "color": "red",
    },
    "ANIMA-HEXA": {
        "peak_fp16_tflops": 200.0,  # shared OMEGA fabric - consciousness overhead
        "peak_int8_tops": 400.0,
        "mem_bw_gbps": 2000,  # HBM4E J2=24 GB
        "tdp_w": SIGMA * SIGMA_PHI,  # 120 W
        "color": "purple",
    },
    # Reference chips for comparison
    "H100": {
        "peak_fp16_tflops": 989.0,  # with sparsity
        "peak_int8_tops": 1979.0,
        "mem_bw_gbps": 3350,  # HBM3
        "tdp_w": 700,
        "color": "green",
    },
    "Apple-M4": {
        "peak_fp16_tflops": 4.6,
        "peak_int8_tops": 38.0,  # Neural Engine
        "mem_bw_gbps": 120,  # unified memory
        "tdp_w": 22,
        "color": "orange",
    },
}


# ---------------------------------------------------------------------------
# AI Workload Operational Intensity
# ---------------------------------------------------------------------------
# ops/byte = compute_ops / bytes_transferred
WORKLOADS = {
    "Transformer-FP16": {
        "oi": 64.0,  # high arithmetic intensity
        "desc": "Standard transformer forward (d=4096, seq=2048)",
    },
    "EFA-Attention": {
        "oi": 96.0,  # higher OI due to 40% fewer memory accesses
        "desc": "Egyptian Fraction Attention (1/2+1/3+1/6 budget)",
    },
    "MoE-Egyptian": {
        "oi": 32.0,  # moderate: sparse routing
        "desc": "Egyptian MoE (8 experts, top-2, 1/2+1/3+1/6)",
    },
    "Mamba-SSM": {
        "oi": 16.0,  # memory-bound due to sequential scan
        "desc": "Mamba selective scan (d_state=16, expand=2)",
    },
    "LoRA-r8": {
        "oi": 8.0,  # very memory-bound (low rank)
        "desc": "LoRA fine-tune (rank=sigma-tau=8)",
    },
    "Embedding-Lookup": {
        "oi": 0.5,  # purely memory-bound
        "desc": "Token embedding (vocab=32K, d=4096)",
    },
}


def ridge_point(peak_tflops: float, bw_gbps: float) -> float:
    """Operational intensity at the ridge point (FLOP/byte)."""
    # peak_compute (TFLOP/s) / bw (GB/s) = TFLOP/GB = FLOP/byte * 1e3
    return (peak_tflops * 1e3) / bw_gbps if bw_gbps > 0 else 0


def attainable_perf(oi: float, peak_tflops: float, bw_gbps: float) -> float:
    """Attainable performance (TFLOPS) at given operational intensity."""
    mem_bound = oi * bw_gbps / 1e3  # TFLOPS
    return min(peak_tflops, mem_bound)


def ascii_roofline(chips: Dict, workloads: Dict) -> str:
    """Generate ASCII roofline chart."""
    W = 72  # chart width
    H = 20  # chart height

    # Determine axis ranges (log scale)
    all_oi = [w["oi"] for w in workloads.values()]
    oi_min = 0.1
    oi_max = 256.0
    perf_min = 0.01
    perf_max = max(c["peak_fp16_tflops"] for c in chips.values()) * 1.5

    def log_x(oi):
        return max(0, min(W - 1, int((math.log10(oi) - math.log10(oi_min)) /
                   (math.log10(oi_max) - math.log10(oi_min)) * (W - 1))))

    def log_y(perf):
        if perf <= 0:
            return H - 1
        return max(0, min(H - 1, H - 1 - int(
            (math.log10(perf) - math.log10(perf_min)) /
            (math.log10(perf_max) - math.log10(perf_min)) * (H - 1))))

    # Build grid
    grid = [[" "] * W for _ in range(H)]

    # Draw roofline for each chip
    chip_labels = []
    for idx, (name, spec) in enumerate(chips.items()):
        marker = name[0] if name[0] not in [c[0] for c in chip_labels] else name[:2]
        chip_labels.append((marker, name, spec.get("color", "")))

        peak = spec["peak_fp16_tflops"]
        bw = spec["mem_bw_gbps"]
        rp = ridge_point(peak, bw)

        # Draw memory-bound slope
        for xi in range(W):
            oi = oi_min * (oi_max / oi_min) ** (xi / (W - 1))
            perf = attainable_perf(oi, peak, bw)
            yi = log_y(perf)
            if 0 <= yi < H and grid[yi][xi] == " ":
                grid[yi][xi] = marker[0].lower()

    # Mark workloads
    for wname, wspec in workloads.items():
        xi = log_x(wspec["oi"])
        # Use best chip performance at this OI
        best_perf = max(attainable_perf(wspec["oi"], c["peak_fp16_tflops"],
                        c["mem_bw_gbps"]) for c in chips.values())
        yi = log_y(best_perf)
        if 0 <= yi < H and 0 <= xi < W:
            grid[yi][xi] = "*"

    # Render
    lines = ["  Roofline Model (log-log)", "  " + "-" * (W + 2)]
    for row_idx, row in enumerate(grid):
        # Y-axis label
        perf = perf_min * (perf_max / perf_min) ** ((H - 1 - row_idx) / (H - 1))
        if row_idx % 5 == 0:
            label = f"{perf:>8.1f}"
        else:
            label = "        "
        lines.append(f"{label} |{''.join(row)}|")
    lines.append("          " + "-" * (W + 2))
    lines.append(f"          OI (FLOP/byte): {oi_min} {'.' * (W - 20)} {oi_max}")
    lines.append("")

    # Legend
    lines.append("  Chips:")
    for marker, name, _ in chip_labels:
        spec = chips[name]
        rp = ridge_point(spec["peak_fp16_tflops"], spec["mem_bw_gbps"])
        lines.append(f"    {marker[0].lower()} = {name:<15s}  "
                     f"Peak={spec['peak_fp16_tflops']:>8.1f} TFLOPS  "
                     f"BW={spec['mem_bw_gbps']:>6.0f} GB/s  "
                     f"Ridge={rp:>6.1f} FLOP/B  "
                     f"TDP={spec['tdp_w']}W")
    lines.append("")
    lines.append("  Workloads (* on chart):")
    for wname, wspec in workloads.items():
        lines.append(f"    OI={wspec['oi']:>6.1f}  {wname:<20s}  {wspec['desc']}")

    return "\n".join(lines)


def parse_gem5_stats(path: str) -> Dict:
    """Parse gem5 stats.txt for roofline-relevant metrics."""
    stats = {
        "sim_seconds": 0.0,
        "sim_ops": 0,
        "system.mem_ctrl.bw_total::total": 0,
    }
    try:
        with open(path) as f:
            for line in f:
                line = line.strip()
                if line.startswith("#") or not line:
                    continue
                for key in stats:
                    if line.startswith(key):
                        parts = line.split()
                        if len(parts) >= 2:
                            try:
                                stats[key] = float(parts[1])
                            except ValueError:
                                pass
    except FileNotFoundError:
        print(f"WARNING: Stats file not found: {path}")
    return stats


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------
def main():
    parser = argparse.ArgumentParser(description="HEXA Roofline Model Generator")
    parser.add_argument("--stats", type=str, default=None,
                        help="gem5 stats.txt path")
    parser.add_argument("--output", type=str, default=None,
                        help="Output PNG path (requires matplotlib)")
    parser.add_argument("--chip", type=str, default="all",
                        choices=["edge", "omega", "anima", "all"],
                        help="Which chip(s) to plot")
    parser.add_argument("--no-plot", action="store_true",
                        help="ASCII-only output, no matplotlib")
    args = parser.parse_args()

    # Select chips
    if args.chip == "all":
        selected = CHIPS
    else:
        name_map = {"edge": "HEXA-EDGE", "omega": "HEXA-OMEGA",
                    "anima": "ANIMA-HEXA"}
        key = name_map[args.chip]
        selected = {key: CHIPS[key], "H100": CHIPS["H100"],
                    "Apple-M4": CHIPS["Apple-M4"]}

    print("=" * 72)
    print("  HEXA Roofline Model")
    print("=" * 72)

    # Parse stats if provided
    if args.stats:
        gem5_stats = parse_gem5_stats(args.stats)
        print(f"\n  gem5 stats from: {args.stats}")
        for k, v in gem5_stats.items():
            print(f"    {k}: {v}")

    # ASCII roofline
    print()
    print(ascii_roofline(selected, WORKLOADS))

    # Performance table
    print("\n  Attainable Performance (TFLOPS) by Chip x Workload")
    print("  " + "-" * 90)
    header = f"  {'Workload':<22s}"
    for cname in selected:
        header += f" {cname:>14s}"
    print(header)
    print("  " + "-" * 90)
    for wname, wspec in WORKLOADS.items():
        row = f"  {wname:<22s}"
        for cname, cspec in selected.items():
            perf = attainable_perf(wspec["oi"], cspec["peak_fp16_tflops"],
                                   cspec["mem_bw_gbps"])
            row += f" {perf:>14.2f}"
        print(row)

    # TFLOPS/W efficiency
    print(f"\n  Efficiency (TFLOPS/W) at OI=64 (Transformer workload)")
    print("  " + "-" * 50)
    for cname, cspec in selected.items():
        perf = attainable_perf(64.0, cspec["peak_fp16_tflops"],
                               cspec["mem_bw_gbps"])
        eff = perf / cspec["tdp_w"]
        print(f"    {cname:<15s}: {perf:>8.2f} TFLOPS / {cspec['tdp_w']:>4d}W "
              f"= {eff:>6.3f} TFLOPS/W")

    # matplotlib plot
    if not args.no_plot and args.output:
        try:
            import matplotlib
            matplotlib.use("Agg")
            import matplotlib.pyplot as plt
            import numpy as np

            fig, ax = plt.subplots(figsize=(12, 8))
            oi_range = np.logspace(-1, 2.5, 500)

            for cname, cspec in selected.items():
                perf = [attainable_perf(oi, cspec["peak_fp16_tflops"],
                        cspec["mem_bw_gbps"]) for oi in oi_range]
                ax.loglog(oi_range, perf, label=f"{cname} ({cspec['tdp_w']}W)",
                          linewidth=2)

            # Mark workloads
            for wname, wspec in WORKLOADS.items():
                best = max(attainable_perf(wspec["oi"], c["peak_fp16_tflops"],
                           c["mem_bw_gbps"]) for c in selected.values())
                ax.plot(wspec["oi"], best, "k*", markersize=10)
                ax.annotate(wname, (wspec["oi"], best),
                            textcoords="offset points", xytext=(5, 5),
                            fontsize=7)

            ax.set_xlabel("Operational Intensity (FLOP/Byte)")
            ax.set_ylabel("Attainable Performance (TFLOPS)")
            ax.set_title("HEXA Chip Family -- Roofline Model")
            ax.legend()
            ax.grid(True, which="both", alpha=0.3)
            fig.tight_layout()
            fig.savefig(args.output, dpi=150)
            print(f"\n  Plot saved to: {args.output}")
        except ImportError:
            print("\n  matplotlib not available. Use --no-plot for ASCII only.")


if __name__ == "__main__":
    main()
