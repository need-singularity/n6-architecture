#!/usr/bin/env python3
"""
Chip Phase Diagram -- Computing Paradigms on Power x Performance Axes
with n=6 alignment overlay.

Generates: docs/chip-architecture/chip-phase-diagram.png

Reference: BT-28, BT-37, BT-45, BT-55, BT-59, BT-69
DSE: chip.toml, analog-photonic-memristor.toml, quantum.toml
"""

import os
import numpy as np

try:
    import matplotlib
    matplotlib.use("Agg")
    import matplotlib.pyplot as plt
    from matplotlib.patches import FancyBboxPatch, Polygon
    from matplotlib.collections import PatchCollection
    import matplotlib.patheffects as pe
except ImportError:
    print("ERROR: matplotlib not found. Install with: pip install matplotlib")
    raise SystemExit(1)


# ── Output path ──────────────────────────────────────────────────────
SCRIPT_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(SCRIPT_DIR)
OUT_PATH = os.path.join(PROJECT_ROOT, "docs", "chip-architecture", "chip-phase-diagram.png")


# ── n=6 constants ────────────────────────────────────────────────────
N = 6
PHI = 2
TAU = 4
SIGMA = 12
J2 = 24
SOPFR = 5


# ── Phase regions (polygon vertices in log10 coords) ─────────────────
# Axes: x = log10(TDP in Watts), y = log10(TOPS)
# Each region is a list of (x, y) vertices forming a closed polygon.

PHASES = {
    "Electronic\n(CMOS)": {
        "verts": [
            (0.0, -0.5),   # 1W, 0.3 TOPS
            (0.0, 1.0),    # 1W, 10 TOPS
            (1.0, 2.5),    # 10W, ~300 TOPS
            (2.0, 3.3),    # 100W, ~2000 TOPS
            (3.1, 3.7),    # 1200W, 5000 TOPS
            (3.1, -0.5),   # 1200W, 0.3 TOPS
        ],
        "color": "#2196F3",
        "alpha": 0.25,
        "n6_pct": 100,
    },
    "Photonic": {
        "verts": [
            (0.0, 1.0),    # 1W, 10 TOPS
            (0.0, 3.0),    # 1W, 1000 TOPS
            (1.0, 4.0),    # 10W, 10k TOPS
            (1.7, 4.3),    # 50W, ~20k TOPS
            (2.0, 3.3),    # 100W, ~2000 TOPS (meets electronic)
            (1.0, 2.5),    # 10W, ~300 TOPS
        ],
        "color": "#FF9800",
        "alpha": 0.25,
        "n6_pct": 100,
    },
    "Quantum": {
        "verts": [
            (3.0, 3.5),    # 1kW, ~3000 TOPS-equiv
            (3.0, 6.0),    # 1kW, 10^6 (quantum advantage)
            (4.2, 6.0),    # 15kW, 10^6
            (4.2, 3.5),    # 15kW, ~3000
        ],
        "color": "#9C27B0",
        "alpha": 0.25,
        "n6_pct": 100,
    },
    "Neuromorphic": {
        "verts": [
            (-3.0, -3.0),  # 0.001W, 0.001 TOPS
            (-3.0, 0.5),   # 0.001W, ~3 TOPS
            (-1.0, 1.5),   # 0.1W, ~30 TOPS
            (1.0, 2.0),    # 10W, 100 TOPS
            (1.0, -0.5),   # 10W, 0.3 TOPS
            (0.0, -1.0),   # 1W, 0.1 TOPS
            (-2.0, -3.0),  # 0.01W, 0.001 TOPS
        ],
        "color": "#4CAF50",
        "alpha": 0.25,
        "n6_pct": 100,
    },
    "Memristor": {
        "verts": [
            (-1.0, -0.5),  # 0.1W, 0.3 TOPS
            (-1.0, 2.0),   # 0.1W, 100 TOPS
            (0.5, 2.8),    # ~3W, ~600 TOPS
            (1.7, 3.0),    # 50W, 1000 TOPS
            (1.7, 0.5),    # 50W, ~3 TOPS
            (0.5, -0.5),   # ~3W, 0.3 TOPS
        ],
        "color": "#F44336",
        "alpha": 0.20,
        "n6_pct": 100,
    },
    "Superconducting\nLogic": {
        "verts": [
            (3.5, 2.5),    # ~3kW, ~300 TOPS-equiv
            (3.5, 5.0),    # ~3kW, 10^5
            (4.5, 5.0),    # ~30kW, 10^5
            (4.5, 2.5),    # ~30kW, ~300
        ],
        "color": "#00BCD4",
        "alpha": 0.25,
        "n6_pct": 83,
    },
}


# ── Real chip data points ────────────────────────────────────────────
# (name, TDP_watts, TOPS, paradigm, n6_hits, marker)
CHIPS = [
    # Electronic
    ("H100",          700,   1979,  "Electronic",    "132 SMs",    "o"),
    ("B200",         1000,   4500,  "Electronic",    "12 chiplets", "o"),
    ("B300",         1200,   5000,  "Electronic",    "160 SMs",    "o"),
    ("M4 Ultra",      150,     50,  "Electronic",    "unified",    "o"),
    ("TPU v5e",       200,    393,  "Electronic",    "8 chips",    "o"),
    ("Edge TPU",        2,      4,  "Electronic",    "edge",       "o"),
    ("Jetson Orin",    60,    275,  "Electronic",    "Ampere",     "o"),
    # Photonic
    ("Lightmatter\nEnvise", 10, 1000, "Photonic",   "MZI mesh",   "D"),
    # Quantum (performance in "equivalent" scale for visualization)
    ("IBM Eagle\n127q",    15000, 5000,  "Quantum",  "transmon",   "^"),
    ("IBM Condor\n1121q",  15000, 50000, "Quantum",  "1121 qubits","^"),
    ("Google Willow\n105q", 12000, 4000, "Quantum",  "surface EC", "^"),
    # Neuromorphic
    ("Loihi 2",        1,     15,  "Neuromorphic",  "128 cores",  "s"),
    ("Akida",        0.3,      3,  "Neuromorphic",  "event",      "s"),
    ("SpiNNaker2",     1,     10,  "Neuromorphic",  "spiking",    "s"),
    # Memristor
    ("Mythic M1076",   3,     35,  "Memristor",     "flash",      "p"),
]

PARADIGM_COLORS = {
    "Electronic":    "#1565C0",
    "Photonic":      "#E65100",
    "Quantum":       "#6A1B9A",
    "Neuromorphic":  "#2E7D32",
    "Memristor":     "#C62828",
    "Superconducting": "#00838F",
}

# ── Triple points ────────────────────────────────────────────────────
TRIPLE_POINTS = [
    ("TP-1", 50,   500,  "Elec+Phot+Mem"),
    ("TP-2",  1,    10,  "Elec+Neuro+Mem"),
    ("TP-3", 100, 3000,  "Elec+Phot+Quant"),
]

# ── Phase boundary lines (approximate) ──────────────────────────────
# Each is a list of (TDP_watts, TOPS) points
BOUNDARIES = {
    "Power Wall\n(Elec->Phot)": [
        (10, 100), (50, 500), (100, 2000), (200, 3000),
    ],
    "Edge Wall\n(Elec->Neuro)": [
        (0.01, 0.1), (0.1, 1), (1, 10), (5, 50),
    ],
    "Complexity Wall\n(Elec->Quant)": [
        (500, 2000), (1000, 3000), (2000, 5000),
    ],
}


# ── n=6 contour data ─────────────────────────────────────────────────
def compute_n6_field(x_log, y_log):
    """
    Compute an n6-alignment score across the power-performance plane.
    Higher where n=6 constants appear naturally in the design space.

    This is a synthetic field based on how many n=6 constants align
    at each (power, performance) point.
    """
    # n=6 attractors in log10 space
    attractors = [
        # (log10_watts, log10_tops, strength, description)
        (np.log10(288),  np.log10(4000), 1.0, "J2*sigma=288W, HEXA-1"),
        (np.log10(48),   np.log10(500),  0.8, "sigma*tau=48W, edge AI"),
        (np.log10(12),   np.log10(100),  0.7, "sigma=12W, ultra-efficient"),
        (np.log10(1),    np.log10(10),   0.6, "mu=1W, neuromorphic"),
        (np.log10(700),  np.log10(2000), 0.9, "H100 regime"),
        (np.log10(1200), np.log10(5000), 0.9, "B300 regime"),
        (np.log10(150),  np.log10(50),   0.5, "M4 Ultra regime"),
    ]

    field = np.zeros_like(x_log)
    for ax, ay, strength, _ in attractors:
        dist = np.sqrt((x_log - ax)**2 + (y_log - ay)**2)
        field += strength * np.exp(-dist**2 / 1.5)

    # Normalize to 0-100%
    field = field / field.max() * 100
    return field


def main():
    fig, ax = plt.subplots(1, 1, figsize=(16, 11))

    # ── Draw phase regions ───────────────────────────────────────
    for name, info in PHASES.items():
        verts = info["verts"]
        poly = Polygon(verts, closed=True,
                       facecolor=info["color"], alpha=info["alpha"],
                       edgecolor=info["color"], linewidth=1.5,
                       linestyle="--")
        ax.add_patch(poly)

        # Label at centroid
        cx = np.mean([v[0] for v in verts])
        cy = np.mean([v[1] for v in verts])
        ax.text(cx, cy, f"{name}\nn6={info['n6_pct']}%",
                fontsize=9, fontweight="bold", ha="center", va="center",
                color=info["color"],
                path_effects=[pe.withStroke(linewidth=3, foreground="white")])

    # ── n=6 contour overlay ──────────────────────────────────────
    xi = np.linspace(-3.5, 4.8, 300)
    yi = np.linspace(-3.5, 6.5, 300)
    X, Y = np.meshgrid(xi, yi)
    Z = compute_n6_field(X, Y)

    cs = ax.contour(X, Y, Z, levels=[30, 50, 70, 85, 95],
                    colors="gold", linewidths=0.8, alpha=0.6)
    ax.clabel(cs, fmt="n6=%d%%", fontsize=7, colors="goldenrod")

    cf = ax.contourf(X, Y, Z, levels=[85, 95, 100],
                     colors=["#FFD700"], alpha=0.08)

    # ── Plot real chips ──────────────────────────────────────────
    for name, tdp, tops, paradigm, n6info, marker in CHIPS:
        color = PARADIGM_COLORS.get(paradigm, "gray")
        x = np.log10(max(tdp, 0.001))
        y = np.log10(max(tops, 0.001))
        ax.plot(x, y, marker=marker, markersize=10, color=color,
                markeredgecolor="black", markeredgewidth=0.8, zorder=5)
        ax.annotate(name, (x, y), textcoords="offset points",
                    xytext=(8, 6), fontsize=7, color=color,
                    fontweight="bold",
                    path_effects=[pe.withStroke(linewidth=2, foreground="white")])

    # ── Triple points ────────────────────────────────────────────
    for label, tdp, tops, desc in TRIPLE_POINTS:
        x = np.log10(tdp)
        y = np.log10(tops)
        ax.plot(x, y, marker="*", markersize=18, color="gold",
                markeredgecolor="black", markeredgewidth=1.0, zorder=6)
        ax.annotate(f"{label}\n{desc}", (x, y),
                    textcoords="offset points", xytext=(12, -12),
                    fontsize=7, color="black", fontstyle="italic",
                    bbox=dict(boxstyle="round,pad=0.2", facecolor="lightyellow",
                              edgecolor="gold", alpha=0.85))

    # ── Phase boundary lines ─────────────────────────────────────
    for label, pts in BOUNDARIES.items():
        xs = [np.log10(p[0]) for p in pts]
        ys = [np.log10(p[1]) for p in pts]
        ax.plot(xs, ys, "k--", linewidth=2.0, alpha=0.5, zorder=4)
        mx = xs[len(xs) // 2]
        my = ys[len(ys) // 2]
        ax.annotate(label, (mx, my), textcoords="offset points",
                    xytext=(-30, 10), fontsize=7, color="black",
                    fontweight="bold", fontstyle="italic",
                    bbox=dict(boxstyle="round,pad=0.15", facecolor="white",
                              edgecolor="gray", alpha=0.8))

    # ── n=6 optimal path arrow ───────────────────────────────────
    path_x = [np.log10(w) for w in [5, 50, 150, 288, 500]]
    path_y = [np.log10(t) for t in [20, 500, 2000, 5000, 50000]]
    ax.plot(path_x, path_y, color="red", linewidth=2.5, linestyle="-",
            alpha=0.7, zorder=7)
    ax.annotate("", xy=(path_x[-1], path_y[-1]),
                xytext=(path_x[-2], path_y[-2]),
                arrowprops=dict(arrowstyle="->", color="red", lw=2.5))
    ax.text(np.log10(100), np.log10(8000),
            "n=6 Optimal Path\n(2024 -> OMEGA 2035)",
            fontsize=9, fontweight="bold", color="red",
            bbox=dict(boxstyle="round,pad=0.3", facecolor="mistyrose",
                      edgecolor="red", alpha=0.9))

    # ── Axes formatting ──────────────────────────────────────────
    ax.set_xlim(-3.5, 4.8)
    ax.set_ylim(-3.5, 6.5)

    # Custom tick labels showing actual watts / TOPS
    xtick_vals = [-3, -2, -1, 0, 1, 2, 3, 4]
    xtick_labels = ["1mW", "10mW", "0.1W", "1W", "10W", "100W", "1kW", "10kW"]
    ax.set_xticks(xtick_vals)
    ax.set_xticklabels(xtick_labels, fontsize=9)

    ytick_vals = [-3, -2, -1, 0, 1, 2, 3, 4, 5, 6]
    ytick_labels = ["0.001", "0.01", "0.1", "1", "10", "100",
                    "1K", "10K", "100K", "1M"]
    ax.set_yticks(ytick_vals)
    ax.set_yticklabels(ytick_labels, fontsize=9)

    ax.set_xlabel("TDP Power (Watts) -- log scale", fontsize=12, fontweight="bold")
    ax.set_ylabel("Performance (TOPS) -- log scale", fontsize=12, fontweight="bold")
    ax.set_title(
        "Chip Phase Diagram: Computing Paradigms vs Power/Performance\n"
        r"$\sigma(6)\cdot\varphi(6) = 6\cdot\tau(6) = 24$ "
        "-- n=6 alignment overlay",
        fontsize=14, fontweight="bold"
    )

    ax.grid(True, alpha=0.3, linestyle=":")

    # ── Legend ────────────────────────────────────────────────────
    from matplotlib.lines import Line2D
    legend_elements = [
        Line2D([0], [0], marker="o", color="w", markerfacecolor="#1565C0",
               markersize=10, label="Electronic (CMOS)"),
        Line2D([0], [0], marker="D", color="w", markerfacecolor="#E65100",
               markersize=10, label="Photonic"),
        Line2D([0], [0], marker="^", color="w", markerfacecolor="#6A1B9A",
               markersize=10, label="Quantum"),
        Line2D([0], [0], marker="s", color="w", markerfacecolor="#2E7D32",
               markersize=10, label="Neuromorphic"),
        Line2D([0], [0], marker="p", color="w", markerfacecolor="#C62828",
               markersize=10, label="Memristor"),
        Line2D([0], [0], marker="*", color="gold", markeredgecolor="black",
               markersize=14, label="Triple Point", linestyle="None"),
        Line2D([0], [0], color="red", linewidth=2.5, label="n=6 Optimal Path"),
        Line2D([0], [0], color="gold", linewidth=1, alpha=0.6,
               linestyle="-", label="n6% Contour"),
        Line2D([0], [0], color="black", linewidth=2, linestyle="--",
               alpha=0.5, label="Phase Boundary"),
    ]
    ax.legend(handles=legend_elements, loc="lower right", fontsize=8,
              framealpha=0.9, edgecolor="gray")

    # ── n=6 equation box ─────────────────────────────────────────
    eq_text = (
        "n=6 Phase Constants:\n"
        f"  n={N}, phi={PHI}, tau={TAU}\n"
        f"  sigma={SIGMA}, J2={J2}, sopfr={SOPFR}\n"
        f"  sigma*phi = {SIGMA*PHI} = n*tau = {N*TAU}\n"
        f"  OMEGA TDP = J2*sigma = {J2*SIGMA}W"
    )
    ax.text(0.02, 0.98, eq_text, transform=ax.transAxes,
            fontsize=8, verticalalignment="top", fontfamily="monospace",
            bbox=dict(boxstyle="round,pad=0.4", facecolor="lightyellow",
                      edgecolor="goldenrod", alpha=0.9))

    plt.tight_layout()
    os.makedirs(os.path.dirname(OUT_PATH), exist_ok=True)
    plt.savefig(OUT_PATH, dpi=200, bbox_inches="tight")
    print(f"[OK] Saved: {OUT_PATH}")
    print(f"     Size: {os.path.getsize(OUT_PATH) / 1024:.0f} KB")

    # ── Summary stats ────────────────────────────────────────────
    print("\n=== Chip Phase Diagram Summary ===")
    print(f"  Paradigms:      {len(PHASES)}")
    print(f"  Real chips:     {len(CHIPS)}")
    print(f"  Triple points:  {len(TRIPLE_POINTS)}")
    print(f"  Phase boundaries: {len(BOUNDARIES)}")
    print(f"  n6 constants:   n={N}, phi={PHI}, tau={TAU}, sigma={SIGMA}, J2={J2}")
    print(f"  OMEGA TDP:      {J2*SIGMA}W (J2*sigma)")
    print(f"  sigma*phi = n*tau = {SIGMA*PHI} (core identity)")


if __name__ == "__main__":
    main()
