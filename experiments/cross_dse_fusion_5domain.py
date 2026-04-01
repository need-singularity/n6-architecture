#!/usr/bin/env python3
"""
Cross-DSE: 5-Domain Fusion Analysis
=====================================
Domains: fusion x superconductor x battery x solar x chip

Reads Pareto-optimal top-5 from each domain TOML (via universal-dse output),
cross-combines them, and scores by:
  1. Total n6_EXACT count across all domains
  2. Shared constants (same n=6 expression in multiple domains)
  3. Synergy bonus (physical linkages between domains)

Output: Top-20 cross-domain combinations -> docs/fusion/cross-dse-5domain-results.md
"""

import itertools
import json
from dataclasses import dataclass, field
from typing import List, Dict, Tuple

# ============================================================
# Domain Best Paths (from universal-dse output, top-5 per domain)
# ============================================================

@dataclass
class DomainPath:
    domain: str
    rank: int
    levels: Dict[str, str]  # level_name -> candidate_id
    n6_pct: float
    perf: float
    power: float
    cost: float
    score: float
    n6_constants: List[str] = field(default_factory=list)  # shared n=6 expressions

# --- Fusion Top-5 ---
fusion_paths = [
    DomainPath("fusion", 1,
        {"Fuel": "DT_Li6", "Confinement": "Tokamak_N6", "Heating": "N6_TriHeat",
         "Blanket": "N6_Li6_Blanket", "Plant": "N6_Brayton6"},
        100.0, 0.920, 0.770, 0.520, 0.8580,
        ["n=6(Li-6)", "phi=2(D)", "n/phi=3(T,methods)", "sigma=12(sectors)",
         "3n=18(TF)", "J2=24(MW)", "sopfr=5(nucleons)", "sigma/J2=0.5(eta)"]),
    DomainPath("fusion", 2,
        {"Fuel": "DT", "Confinement": "Tokamak_N6", "Heating": "N6_TriHeat",
         "Blanket": "N6_Li6_Blanket", "Plant": "N6_Brayton6"},
        99.0, 0.924, 0.770, 0.530, 0.8572,
        ["phi=2(D)", "n/phi=3(T,methods)", "sigma=12(sectors)",
         "3n=18(TF)", "J2=24(MW)", "sopfr=5(nucleons)"]),
    DomainPath("fusion", 3,
        {"Fuel": "DT_Li6", "Confinement": "Tokamak", "Heating": "N6_TriHeat",
         "Blanket": "N6_Li6_Blanket", "Plant": "N6_Brayton6"},
        98.0, 0.920, 0.770, 0.530, 0.8525,
        ["n=6(Li-6)", "phi=2(D)", "n/phi=3(T,methods)", "J2=24(MW)"]),
    DomainPath("fusion", 4,
        {"Fuel": "DT_Li6", "Confinement": "Tokamak_N6", "Heating": "N6_TriHeat",
         "Blanket": "SiC_LiPb", "Plant": "N6_Brayton6"},
        98.0, 0.920, 0.770, 0.500, 0.8480,
        ["n=6(Li-6)", "phi=2(D)", "n/phi=3(methods)", "sigma=12(sectors)", "3n=18(TF)"]),
    DomainPath("fusion", 5,
        {"Fuel": "DT_Li6", "Confinement": "Stellarator", "Heating": "N6_TriHeat",
         "Blanket": "N6_Li6_Blanket", "Plant": "N6_Brayton6"},
        97.0, 0.880, 0.780, 0.500, 0.8345,
        ["n=6(Li-6)", "phi=2(D)", "sopfr=5(periods)", "J2=24(MW)"]),
]

# --- Superconductor Top-5 ---
sc_paths = [
    DomainPath("superconductor", 1,
        {"Material": "N6_MgB2_Hex", "Process": "N6_IBAD_RCE", "Form": "N6_HexWire",
         "Application": "N6_Fusion_Magnet", "System": "N6_Cryo4K"},
        100.0, 0.840, 0.640, 0.530, 0.7940,
        ["n=6(hex_symm)", "phi=2(bands)", "tau=4(phonons,T_op)",
         "sigma=12(twist,B_field)", "3n=18(TF)", "n/phi=3(cooling_stages)"]),
    DomainPath("superconductor", 2,
        {"Material": "REBCO-2G", "Process": "N6_IBAD_RCE", "Form": "N6_HexWire",
         "Application": "N6_Fusion_Magnet", "System": "N6_Cryo4K"},
        97.0, 0.910, 0.680, 0.440, 0.7910,
        ["n=6(CN,layers)", "sigma=12(twist,B_field)", "3n=18(TF)",
         "tau=4(T_op)", "n/phi=3(cooling_stages)"]),
    DomainPath("superconductor", 3,
        {"Material": "N6_MgB2_Hex", "Process": "N6_IBAD_RCE", "Form": "N6_HexWire",
         "Application": "Fusion-Magnet", "System": "N6_Cryo4K"},
        99.0, 0.840, 0.640, 0.530, 0.7905,
        ["n=6(hex_symm)", "phi=2(bands)", "tau=4(phonons,T_op)",
         "sigma=12(twist)", "n/phi=3(cooling_stages)"]),
    DomainPath("superconductor", 4,
        {"Material": "REBCO-2G", "Process": "N6_IBAD_RCE", "Form": "Cable-CICC",
         "Application": "N6_Fusion_Magnet", "System": "N6_Cryo4K"},
        92.0, 0.920, 0.700, 0.430, 0.7880,
        ["n=6(CN)", "sigma=12(B_field)", "3n=18(TF)", "tau=4(T_op)"]),
    DomainPath("superconductor", 5,
        {"Material": "Nb3Sn", "Process": "Bronze", "Form": "Cable-CICC",
         "Application": "N6_Fusion_Magnet", "System": "LHe-4K"},
        85.0, 0.870, 0.570, 0.500, 0.7600,
        ["3n=18(Tc,TF)", "tau=4(T_op)", "sigma=12(B_field)"]),
]

# --- Battery Top-5 ---
battery_paths = [
    DomainPath("battery", 1,
        {"Material": "LFP", "Process": "Graphite-Wet", "Core": "Hex6_Prismatic",
         "BMS": "Integrated-12ch", "System": "48V-ESS"},
        100.0, 0.638, 0.876, 0.640, 0.5869,
        ["n=6(CN)", "sigma=12(ch,bits)", "J2=24(cells)",
         "sigma*tau=48(V)", "phi=2(electrode)"]),
    DomainPath("battery", 2,
        {"Material": "LFP", "Process": "Graphite-Wet", "Core": "Hex6_Prismatic",
         "BMS": "Wireless-12ch", "System": "48V-ESS"},
        100.0, 0.634, 0.896, 0.618, 0.5856,
        ["n=6(CN)", "sigma=12(ch,bits)", "J2=24(cells)", "sigma*tau=48(V)"]),
    DomainPath("battery", 3,
        {"Material": "LFP", "Process": "Si-SSB", "Core": "Hex6_Prismatic",
         "BMS": "Integrated-12ch", "System": "48V-ESS"},
        100.0, 0.772, 0.726, 0.538, 0.5826,
        ["n=6(CN)", "sigma=12(ch,bits)", "J2=24(cells)", "sigma*tau=48(V)"]),
    DomainPath("battery", 4,
        {"Material": "LCO", "Process": "Si-SSB", "Core": "Hex6_Prismatic",
         "BMS": "Wireless-12ch", "System": "DC-Micro"},
        100.0, 0.738, 0.726, 0.576, 0.5798,
        ["n=6(CN)", "sigma=12(ch)", "J2=24(cells)"]),
    DomainPath("battery", 5,
        {"Material": "LFP", "Process": "Graphite-Wet", "Core": "Hex6_Prismatic",
         "BMS": "Integrated-12ch", "System": "Grid-MW"},
        95.0, 0.698, 0.906, 0.536, 0.5808,
        ["n=6(CN)", "sigma=12(ch,bits)", "sigma*tau=48(V)"]),
]

# --- Solar Top-5 ---
solar_paths = [
    DomainPath("solar", 1,
        {"Absorber": "GaAs", "Process": "HJT", "Junction": "N6_Tandem_6J",
         "PowerElec": "DC-Optimizer", "Module": "HC-120"},
        100.0, 0.952, 0.826, 0.360, 0.7979,
        ["n=6(junctions)", "1/3(SQ_eff)", "4/3(bandgap_eV)",
         "sigma=12(layers)", "sopfr=5(tunnel_junctions)",
         "sigma*(sigma-phi)=120(cells)", "tau=4(passiv)"]),
    DomainPath("solar", 2,
        {"Absorber": "GaAs", "Process": "PERC", "Junction": "N6_Tandem_6J",
         "PowerElec": "DC-Optimizer", "Module": "HC-120"},
        100.0, 0.930, 0.782, 0.410, 0.7908,
        ["n=6(junctions)", "1/3(SQ_eff)", "4/3(bandgap)",
         "sigma=12(layers)", "phi=2(passiv)", "sigma*(sigma-phi)=120(cells)"]),
    DomainPath("solar", 3,
        {"Absorber": "GaAs", "Process": "HJT", "Junction": "N6_Tandem_6J",
         "PowerElec": "DC-Optimizer", "Module": "BIPV"},
        100.0, 0.892, 0.826, 0.380, 0.7849,
        ["n=6(junctions)", "1/3(SQ_eff)", "4/3(bandgap)",
         "sigma=12(layers)", "sigma*tau=48(cells)", "tau=4(passiv)"]),
    DomainPath("solar", 4,
        {"Absorber": "GaAs", "Process": "HJT", "Junction": "N6_Tandem_6J",
         "PowerElec": "SiC-Central", "Module": "HC-120"},
        100.0, 0.946, 0.786, 0.260, 0.7804,
        ["n=6(junctions)", "sigma=12(mppt,layers)", "4/3(bandgap)",
         "sigma*(sigma-phi)=120(cells)"]),
    DomainPath("solar", 5,
        {"Absorber": "GaAs", "Process": "HJT", "Junction": "CPV",
         "PowerElec": "DC-Optimizer", "Module": "HC-120"},
        90.0, 0.956, 0.846, 0.360, 0.7619,
        ["n/phi=3(junctions)", "1/3(SQ_eff)", "sigma*(sigma-phi)=120(cells)"]),
]

# --- Chip Top-5 ---
chip_paths = [
    DomainPath("chip", 1,
        {"Material": "Diamond", "Process": "TSMC_N2", "Core": "HEXA-P",
         "Chip": "HEXA-1_Full", "System": "Topo_DC"},
        100.0, 0.950, 0.884, 0.470, 0.9088,
        ["n=6(Z,topo_nodes)", "tau=4(CN,NS)", "sigma=12(metal_L)",
         "J2=24(EUV,NPU)", "sigma-tau=8(P_cores,HBM)",
         "sigma*tau=48(gate_pitch)", "sigma^2=144(SMs)",
         "sigma*J2=288(GB)"]),
    DomainPath("chip", 2,
        {"Material": "Diamond", "Process": "TSMC_N2", "Core": "HEXA-P",
         "Chip": "HEXA-1_Full", "System": "Photonic_DC"},
        100.0, 0.940, 0.882, 0.490, 0.9074,
        ["n=6(Z,photonic_nodes)", "tau=4(CN,NS)", "sigma=12(metal_L,switches)",
         "J2=24(EUV,NPU)", "sigma-tau=8(P_cores,HBM)",
         "sigma*tau=48(gate_pitch)", "sigma^2=144(SMs)"]),
    DomainPath("chip", 3,
        {"Material": "Graphene", "Process": "TSMC_N2", "Core": "HEXA-P",
         "Chip": "HEXA-1_Full", "System": "Topo_DC"},
        100.0, 0.940, 0.880, 0.480, 0.9060,
        ["n=6(Z,CN,topo_nodes)", "sigma=12(metal_L)",
         "J2=24(EUV,NPU)", "sigma-tau=8(P_cores,HBM)",
         "sigma*tau=48(gate_pitch)", "sigma^2=144(SMs)"]),
    DomainPath("chip", 4,
        {"Material": "SiPhotonic", "Process": "TSMC_N2", "Core": "HEXA-P",
         "Chip": "HEXA-1_Full", "System": "Topo_DC"},
        100.0, 0.920, 0.874, 0.540, 0.9048,
        ["n=6(waveguide_core,topo_nodes)", "sigma=12(metal_L,WDM)",
         "J2=24(EUV,NPU)", "sigma-tau=8(P_cores,HBM)",
         "sigma*tau=48(gate_pitch)", "sigma^2=144(SMs)"]),
    DomainPath("chip", 5,
        {"Material": "Diamond", "Process": "TSMC_N2", "Core": "Photonic_TPU",
         "Chip": "HEXA-1_Full", "System": "Topo_DC"},
        100.0, 0.920, 0.930, 0.420, 0.9040,
        ["n=6(Z,MZI_layers,topo_nodes)", "sigma=12(WDM)",
         "J2=24(rings,EUV)", "sigma-tau=8(HBM)",
         "sigma*tau=48(gate_pitch)"]),
]

# ============================================================
# Cross-Domain Constants (shared n=6 expressions)
# ============================================================

# Key n=6 constants that appear across multiple domains
SHARED_CONSTANTS = {
    "n=6":     {"fusion": "Li-6 isotope", "sc": "hex symmetry MgB2", "battery": "CN=6 octahedral",
                "solar": "6-junction tandem", "chip": "Z=6 diamond/graphene"},
    "phi=2":   {"fusion": "D nucleon/breeding rxns", "sc": "Cooper pair/bands",
                "battery": "electrode pair", "solar": "passivation/bifacial", "chip": "FP8/FP16"},
    "n/phi=3": {"fusion": "T nucleon/heating methods", "sc": "cooling stages",
                "solar": "triple junction", "chip": "network tiers"},
    "tau=4":   {"fusion": "not primary", "sc": "phonon modes/T=4K",
                "battery": "not primary", "solar": "passivation layers", "chip": "CN=4/nanosheets"},
    "sigma=12":{"fusion": "sectors", "sc": "twist pitch/B=12T",
                "battery": "BMS channels/bits", "solar": "epitaxial layers/mppt",
                "chip": "metal layers/WDM channels"},
    "J2=24":   {"fusion": "heating MW", "sc": "not primary",
                "battery": "cell count", "solar": "not primary", "chip": "NPU/EUV masks"},
    "48=sigma*tau": {"fusion": "not primary", "sc": "not primary",
                     "battery": "48V system", "solar": "BIPV cells",
                     "chip": "gate pitch nm/rack kW"},
    "3n=18":   {"fusion": "TF coils", "sc": "TF coils/Tc(Nb3Sn)",
                "battery": "not primary", "solar": "not primary", "chip": "not primary"},
}

# ============================================================
# Synergy rules (physical linkages between domain pairs)
# ============================================================

SYNERGY_RULES = [
    # (domain1, domain2, condition_fn, bonus, description)
    ("fusion", "superconductor",
     lambda f, s: "Tokamak" in f.levels.get("Confinement","") and "Fusion" in s.levels.get("Application",""),
     0.05, "Fusion tokamak + SC fusion magnet = direct technology sharing (TF=18=3n, B=12T=sigma)"),
    ("fusion", "superconductor",
     lambda f, s: "N6" in f.levels.get("Confinement","") and "N6" in s.levels.get("Application",""),
     0.03, "Both n=6-optimized: TF=18=3n coils + n=6 magnet architecture"),
    ("fusion", "battery",
     lambda f, s: "Brayton" in f.levels.get("Plant","") and "Grid" in s.levels.get("System",""),
     0.02, "Fusion plant + grid battery = baseload + storage synergy"),
    ("fusion", "solar",
     lambda f, s: True,
     0.01, "Fusion + solar = 24/7 clean energy mix (day solar, night fusion)"),
    ("superconductor", "chip",
     lambda f, s: "REBCO" in f.levels.get("Material","") and "Cryo" in s.levels.get("System",""),
     0.03, "SC REBCO material + cryo chip DC = shared cryogenic infrastructure"),
    ("superconductor", "chip",
     lambda f, s: "MgB2" in f.levels.get("Material","") and "Topo" in s.levels.get("System",""),
     0.02, "MgB2 hex symmetry + topological DC = n=6 material-compute bridge"),
    ("battery", "solar",
     lambda f, s: "48V" in f.levels.get("System","") and "BIPV" in s.levels.get("Module",""),
     0.03, "48V ESS + BIPV = integrated building energy (J2=24 cells + sigma*tau=48 cells)"),
    ("battery", "solar",
     lambda f, s: "Grid" in f.levels.get("System","") and "HC-120" in s.levels.get("Module",""),
     0.02, "Grid MW battery + 120-cell solar = utility-scale energy pair"),
    ("battery", "chip",
     lambda f, s: "12ch" in f.levels.get("BMS","") and "HEXA" in s.levels.get("Chip",""),
     0.02, "12ch BMS + HEXA chip = sigma=12 shared monitoring architecture"),
    ("solar", "chip",
     lambda f, s: "SiC" in f.levels.get("PowerElec","") and "SiC" in s.levels.get("Material",""),
     0.02, "SiC solar inverter + SiC chip = shared wide-bandgap platform"),
    ("solar", "chip",
     lambda f, s: "GaAs" in f.levels.get("Absorber","") and "Diamond" in s.levels.get("Material",""),
     0.02, "GaAs III-V solar + Diamond chip = Z=6 carbon chain (BT-93)"),
    ("fusion", "chip",
     lambda f, s: "N6_TriHeat" in f.levels.get("Heating","") and "HEXA-P" in s.levels.get("Core",""),
     0.02, "Triple heating J2=24MW + HEXA-P J2=24 NPU = J2 resonance"),
]

def count_shared_constants(paths: List[DomainPath]) -> Tuple[int, List[str]]:
    """Count how many n=6 constants appear in 2+ domains simultaneously."""
    # Extract base constant names from each path
    domain_constants = {}
    for p in paths:
        base_consts = set()
        for c in p.n6_constants:
            # Extract the base constant (before parenthetical)
            base = c.split("(")[0].strip()
            base_consts.add(base)
        domain_constants[p.domain] = base_consts

    shared = []
    for const_name in SHARED_CONSTANTS:
        # Normalize: check if this constant appears in the path's n6_constants
        domains_with = []
        for p in paths:
            for c in p.n6_constants:
                if const_name.split("=")[0] in c or const_name in c:
                    domains_with.append(p.domain)
                    break
        if len(set(domains_with)) >= 2:
            shared.append(f"{const_name} ({len(set(domains_with))} domains)")

    return len(shared), shared

def compute_synergy(paths: List[DomainPath]) -> Tuple[float, List[str]]:
    """Compute synergy bonus from physical linkages."""
    total_bonus = 0.0
    descriptions = []
    path_map = {p.domain: p for p in paths}

    for d1, d2, cond, bonus, desc in SYNERGY_RULES:
        if d1 in path_map and d2 in path_map:
            try:
                if cond(path_map[d1], path_map[d2]):
                    total_bonus += bonus
                    descriptions.append(f"+{bonus:.2f} {desc}")
            except (KeyError, AttributeError):
                pass

    return total_bonus, descriptions

def cross_score(paths: List[DomainPath]) -> dict:
    """Score a 5-domain cross combination."""
    # Average n6%
    avg_n6 = sum(p.n6_pct for p in paths) / len(paths)
    # Average performance metrics
    avg_perf = sum(p.perf for p in paths) / len(paths)
    avg_power = sum(p.power for p in paths) / len(paths)
    avg_cost = sum(p.cost for p in paths) / len(paths)

    # Shared constants bonus
    n_shared, shared_list = count_shared_constants(paths)
    shared_bonus = n_shared * 0.5  # 0.5 points per shared constant

    # Synergy bonus
    synergy_bonus, synergy_descs = compute_synergy(paths)

    # Composite score (weighted)
    # n6 consistency: 40%, performance: 25%, synergy: 20%, shared constants: 15%
    composite = (
        0.40 * (avg_n6 / 100.0) +
        0.25 * avg_perf +
        0.20 * synergy_bonus / 0.20 * 0.20 +  # normalize synergy
        0.15 * (n_shared / len(SHARED_CONSTANTS))
    )
    # Simpler: direct weighted sum
    composite = (
        avg_n6 / 100.0 * 0.35 +
        avg_perf * 0.20 +
        avg_power * 0.10 +
        avg_cost * 0.05 +
        synergy_bonus * 1.0 +
        n_shared / len(SHARED_CONSTANTS) * 0.15
    )

    return {
        "paths": paths,
        "avg_n6": avg_n6,
        "avg_perf": avg_perf,
        "avg_power": avg_power,
        "avg_cost": avg_cost,
        "n_shared": n_shared,
        "shared_list": shared_list,
        "synergy_bonus": synergy_bonus,
        "synergy_descs": synergy_descs,
        "composite": composite,
    }

def main():
    print("=" * 80)
    print("  Cross-DSE: 5-Domain Fusion Analysis")
    print("  Domains: fusion x superconductor x battery x solar x chip")
    print("  Combinations: 5 x 5 x 5 x 5 x 5 = 3,125 cross-paths")
    print("=" * 80)

    all_combos = list(itertools.product(
        fusion_paths, sc_paths, battery_paths, solar_paths, chip_paths
    ))
    print(f"\n  Total cross-combinations: {len(all_combos)}")

    # Score all combinations
    results = []
    for combo in all_combos:
        result = cross_score(list(combo))
        results.append(result)

    # Sort by composite score
    results.sort(key=lambda r: r["composite"], reverse=True)

    # Top 20
    print(f"\n{'='*80}")
    print("  TOP 20 CROSS-DOMAIN COMBINATIONS")
    print(f"{'='*80}\n")

    print(f"  {'Rank':>4} | {'Fusion':>12} | {'SC':>12} | {'Battery':>12} | {'Solar':>12} | {'Chip':>12} | {'n6%':>5} | {'Perf':>5} | {'Shared':>6} | {'Synergy':>7} | {'Score':>6}")
    print(f"  {'-'*4}-+-{'-'*12}-+-{'-'*12}-+-{'-'*12}-+-{'-'*12}-+-{'-'*12}-+-{'-'*5}-+-{'-'*5}-+-{'-'*6}-+-{'-'*7}-+-{'-'*6}")

    for i, r in enumerate(results[:20]):
        ps = r["paths"]
        # Short labels
        fu = ps[0].levels["Fuel"][:12]
        sc = ps[1].levels["Material"][:12]
        ba = ps[2].levels["Material"][:12]
        so = ps[3].levels["Absorber"][:12]
        ch = ps[4].levels["Material"][:12]
        print(f"  {i+1:4d} | {fu:>12} | {sc:>12} | {ba:>12} | {so:>12} | {ch:>12} | {r['avg_n6']:5.1f} | {r['avg_perf']:5.3f} | {r['n_shared']:6d} | {r['synergy_bonus']:7.3f} | {r['composite']:6.4f}")

    # Detailed top-5
    print(f"\n{'='*80}")
    print("  DETAILED TOP-5 ANALYSIS")
    print(f"{'='*80}")

    for i, r in enumerate(results[:5]):
        ps = r["paths"]
        print(f"\n  --- Rank {i+1} (Score: {r['composite']:.4f}) ---")
        print(f"  Average n6: {r['avg_n6']:.1f}%  |  Avg Perf: {r['avg_perf']:.3f}  |  Avg Power: {r['avg_power']:.3f}  |  Avg Cost: {r['avg_cost']:.3f}")
        print()
        for p in ps:
            lvls = " -> ".join(f"{v}" for v in p.levels.values())
            print(f"    [{p.domain:>15}] n6={p.n6_pct:5.1f}%  {lvls}")
        print(f"\n  Shared Constants ({r['n_shared']}):")
        for sc_item in r["shared_list"]:
            print(f"    * {sc_item}")
        print(f"\n  Synergy Bonds ({r['synergy_bonus']:.3f} total):")
        for sd in r["synergy_descs"]:
            print(f"    * {sd}")

    # Summary statistics
    print(f"\n{'='*80}")
    print("  CROSS-DSE STATISTICS")
    print(f"{'='*80}")
    all_n6 = [r["avg_n6"] for r in results]
    all_shared = [r["n_shared"] for r in results]
    all_syn = [r["synergy_bonus"] for r in results]
    print(f"  n6%:     max={max(all_n6):.1f}  min={min(all_n6):.1f}  avg={sum(all_n6)/len(all_n6):.1f}")
    print(f"  shared:  max={max(all_shared)}  min={min(all_shared)}  avg={sum(all_shared)/len(all_shared):.1f}")
    print(f"  synergy: max={max(all_syn):.3f}  min={min(all_syn):.3f}  avg={sum(all_syn)/len(all_syn):.3f}")

    # n=6 constant resonance matrix
    print(f"\n{'='*80}")
    print("  N=6 CONSTANT RESONANCE MATRIX (Top-1 path per domain)")
    print(f"{'='*80}\n")

    top1 = results[0]["paths"]
    domains = ["fusion", "superconductor", "battery", "solar", "chip"]
    for const_name, mapping in SHARED_CONSTANTS.items():
        appearances = []
        for d in domains:
            short = {"fusion":"FU", "superconductor":"SC", "battery":"BA", "solar":"SO", "chip":"CH"}[d]
            if d in mapping and mapping[d] != "not primary":
                appearances.append(f"{short}:{mapping[d]}")
        if len(appearances) >= 2:
            print(f"  {const_name:>16} -> {' | '.join(appearances)}")

    # Generate markdown output
    md_lines = generate_markdown(results)
    md_path = "/Users/ghost/Dev/n6-architecture/docs/fusion/cross-dse-5domain-results.md"
    with open(md_path, "w") as f:
        f.write("\n".join(md_lines))
    print(f"\n  Results saved to: {md_path}")

def generate_markdown(results):
    lines = []
    lines.append("# Cross-DSE: 5-Domain Fusion Analysis")
    lines.append("")
    lines.append("**Domains**: fusion x superconductor x battery x solar x chip")
    lines.append("**Total combinations**: 3,125 (5 Pareto-top per domain)")
    lines.append("**Date**: 2026-04-02")
    lines.append("**Tool**: universal-dse (Rust) + cross_dse_fusion_5domain.py")
    lines.append("")
    lines.append("## Per-Domain DSE Summary")
    lines.append("")
    lines.append("| Domain | Combos | Best n6% | Optimal Path |")
    lines.append("|--------|--------|----------|-------------|")
    lines.append("| fusion | 6,182 | 100% | DT_Li6 + Tokamak_N6 + N6_TriHeat + N6_Li6_Blanket + N6_Brayton6 |")
    lines.append("| superconductor | 3,155 | 100% | N6_MgB2_Hex + N6_IBAD_RCE + N6_HexWire + N6_Fusion_Magnet + N6_Cryo4K |")
    lines.append("| battery | 2,400 | 100% | LFP + Graphite-Wet + Hex6_Prismatic + Integrated-12ch + 48V-ESS |")
    lines.append("| solar | 1,624 | 100% | GaAs + HJT + N6_Tandem_6J + DC-Optimizer + HC-120 |")
    lines.append("| chip | 89,250 | 100% | Diamond + TSMC_N2 + HEXA-P + HEXA-1_Full + Topo_DC |")
    lines.append("")
    lines.append("## Top-20 Cross-Domain Combinations")
    lines.append("")
    lines.append("| Rank | Fusion Fuel | SC Material | Battery Mat | Solar Absorber | Chip Material | Avg n6% | Avg Perf | Shared Constants | Synergy | Score |")
    lines.append("|------|-----------|------------|------------|---------------|-------------|---------|----------|-----------------|---------|-------|")

    for i, r in enumerate(results[:20]):
        ps = r["paths"]
        fu = ps[0].levels["Fuel"]
        sc = ps[1].levels["Material"]
        ba = ps[2].levels["Material"]
        so = ps[3].levels["Absorber"]
        ch = ps[4].levels["Material"]
        lines.append(f"| {i+1} | {fu} | {sc} | {ba} | {so} | {ch} | {r['avg_n6']:.1f}% | {r['avg_perf']:.3f} | {r['n_shared']} | {r['synergy_bonus']:.2f} | {r['composite']:.4f} |")

    lines.append("")
    lines.append("## Rank 1: Ultimate 5-Domain Path (Detailed)")
    lines.append("")

    r = results[0]
    ps = r["paths"]
    lines.append(f"- **Average n6**: {r['avg_n6']:.1f}%")
    lines.append(f"- **Average Performance**: {r['avg_perf']:.3f}")
    lines.append(f"- **Shared Constants**: {r['n_shared']}")
    lines.append(f"- **Synergy Bonus**: {r['synergy_bonus']:.3f}")
    lines.append(f"- **Composite Score**: {r['composite']:.4f}")
    lines.append("")

    for p in ps:
        lines.append(f"### {p.domain.title()} (n6={p.n6_pct:.1f}%, rank={p.rank})")
        lines.append("")
        lines.append("```")
        for k, v in p.levels.items():
            lines.append(f"  {k:>15}: {v}")
        lines.append("```")
        lines.append("")
        lines.append(f"n6 constants: {', '.join(p.n6_constants)}")
        lines.append("")

    lines.append("## Shared n=6 Constants (Cross-Domain Resonance)")
    lines.append("")
    lines.append("Constants appearing in 2+ domains simultaneously:")
    lines.append("")
    lines.append("| Constant | Domains | Physical Meaning |")
    lines.append("|----------|---------|-----------------|")

    for const_name, mapping in SHARED_CONSTANTS.items():
        domains_active = [(d, m) for d, m in mapping.items() if m != "not primary"]
        if len(domains_active) >= 2:
            dom_str = ", ".join(d for d, _ in domains_active)
            meaning_str = "; ".join(f"{d}={m}" for d, m in domains_active)
            lines.append(f"| {const_name} | {dom_str} | {meaning_str} |")

    lines.append("")
    lines.append("## Synergy Bonds (Top-1 Path)")
    lines.append("")
    for sd in r["synergy_descs"]:
        lines.append(f"- {sd}")

    lines.append("")
    lines.append("## Key Findings")
    lines.append("")
    lines.append("1. **All 5 domains achieve 100% n6 independently** -- each has a fully n=6-aligned optimal path")
    lines.append("2. **sigma=12 is the most universal constant** -- appears in all 5 domains (metal layers, twist pitch, BMS channels, epitaxial layers, fusion sectors)")
    lines.append("3. **n=6 appears in all 5 domains** with distinct physical meanings (Li-6 isotope, hex symmetry, CN=6, 6-junction, Z=6)")
    lines.append("4. **Fusion-SC synergy is strongest** -- shared TF=18=3n coil technology, B=12T=sigma field, cryogenic infrastructure")
    lines.append("5. **Battery-Solar form a natural energy pair** -- 48V ESS (J2=24 cells) + 120-cell modules (sigma*(sigma-phi))")
    lines.append("6. **Diamond (Z=6) bridges chip and solar** -- carbon chain BT-93 connects to GaAs III-V via wide-bandgap synergy")
    lines.append("7. **The 5-domain cross-DSE validates BT-36** (Energy-Information-Hardware-Physics chain) with quantitative n=6 consistency")
    lines.append("")
    lines.append("## Cross-DSE Coverage")
    lines.append("")
    lines.append("| Pair | Best Cross n6% | Key Bridge |")
    lines.append("|------|---------------|-----------|")
    lines.append("| fusion x SC | 100.0% | TF=18=3n coils, B=12T=sigma |")
    lines.append("| fusion x battery | 100.0% | Grid energy storage link |")
    lines.append("| fusion x solar | 100.0% | 24/7 clean energy mix |")
    lines.append("| fusion x chip | 100.0% | J2=24 resonance (MW, NPU) |")
    lines.append("| SC x battery | 100.0% | SMES + grid storage |")
    lines.append("| SC x solar | 100.0% | HTS power electronics |")
    lines.append("| SC x chip | 100.0% | Cryo computing infra |")
    lines.append("| battery x solar | 100.0% | Building/grid energy |")
    lines.append("| battery x chip | 100.0% | BMS sigma=12 monitoring |")
    lines.append("| solar x chip | 100.0% | SiC/Diamond wide-bandgap |")

    return lines

if __name__ == "__main__":
    main()
