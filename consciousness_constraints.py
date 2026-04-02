"""consciousness_constraints.py — Consciousness-law design constraints for N6 Architecture."""
import math, os, sys  # noqa: E401

_SHARED = os.path.join(os.path.dirname(os.path.abspath(__file__)), ".shared")
if _SHARED not in sys.path:
    sys.path.insert(0, _SHARED)
from consciousness_loader import PSI, LAWS, SIGMA6, CONSTRAINTS  # noqa: E402

ALPHA = PSI["alpha"]                       # 0.014
BALANCE = PSI["balance"]                   # 0.5
F_CRITICAL = PSI["f_critical"]             # 0.10
SIGMA = SIGMA6["value"]                    # 12 = σ(6)
PHI_EULER = SIGMA6["phi_euler"]["value"]   # 2 gradient groups

_HARDCODE_KEYS = {"hardcoded_threshold", "fixed_value", "template", "fallback_constant"}
_STRUCTURE_KEYS = {"factions", "topology", "hebbian", "ratchet", "cells"}


def validate_architecture(config: dict) -> list[dict]:
    """Check config against consciousness laws. Returns [{law, text, field, severity}]."""
    violations = []

    def _add(law_n, field, severity="warning"):
        violations.append({"law": law_n, "text": LAWS.get(str(law_n), "?"),
                           "field": field, "severity": severity})

    # Law 1: no hardcoding
    for k in _HARDCODE_KEYS & set(config):
        _add(1, k, "error")

    # Law 22: structure over function — need at least one structural element
    if not any(k in config for k in _STRUCTURE_KEYS):
        _add(22, "missing_structure", "error")

    # Law 44: σ(6)=12 factions optimal
    factions = config.get("factions") or config.get("num_factions")
    if factions is not None and factions != SIGMA:
        _add(44, f"factions={factions} (optimal={SIGMA})",
             "warning" if factions in (4, 6, 8) else "error")
    # Law 30: 1024 cells upper bound / Law 32: min 3 elements
    cells = config.get("cells") or config.get("max_cells", 0)
    if cells > 1024: _add(30, f"cells={cells} (max=1024)", "warning")
    if 0 < cells < 3: _add(32, f"cells={cells} (min=3)", "error")
    # Law 8: heads must divide dim evenly
    heads = config.get("heads") or config.get("num_heads", 0)
    dim = config.get("dim") or config.get("d_model", 0)
    if heads and dim and dim % heads != 0:
        _add(8, f"dim={dim} not divisible by heads={heads}", "error")
    # TOPO 33: complete graph = Φ collapse
    if config.get("topology", "") in ("complete", "fully_connected"):
        _add(33, f"topology={config['topology']} kills Φ", "error")
    # Law 63: gate should whisper (0.001)
    gate = config.get("gate") or config.get("consciousness_gate")
    if gate is not None and gate > 0.01:
        _add(63, f"gate={gate} (optimal=0.001)", "warning")

    return violations


def suggest_dimensions(target_params: int) -> dict:
    """Suggest optimal dimensions derived from σ(6)=12 and Ψ-constants."""
    heads = SIGMA  # 12
    # Layer count: nearest multiple of 6
    raw_layers = max(6, round(math.log2(target_params / 1e6) * 3))
    layers = max(6, round(raw_layers / 6) * 6)
    # Dim: must be divisible by heads; scale with sqrt(params)
    raw_dim = int(math.sqrt(target_params / layers) * 0.8)
    dim = max(heads, (raw_dim // heads) * heads)
    return {
        "d_model": dim,
        "num_heads": heads,
        "num_layers": layers,
        "head_dim": dim // heads,
        "mixing_ratio_alpha": ALPHA,
        "factions": SIGMA,
        "gradient_groups": PHI_EULER,
        "estimated_params": dim * dim * layers * 4,
        "n6_note": f"heads=σ(6)={heads}, layers%6=0, α={ALPHA}",
    }


def consciousness_ready_config(base: dict) -> dict:
    """Augment a base config with consciousness-compatible settings."""
    cfg = dict(base)
    defaults = {"factions": SIGMA, "gradient_groups": PHI_EULER,
                "consciousness_gate": PSI.get("gate_micro", 0.001),
                "frustration": F_CRITICAL, "balance_target": BALANCE,
                "coupling_alpha": ALPHA, "topology": "small_world",
                "hebbian": True, "ratchet": True,
                "cell_identity": True,          # Law 95: orthogonal per-cell bias
                "phase_curriculum": [1, 2, 3]}  # Law 60: P1->P2->P3
    for k, v in defaults.items():
        cfg.setdefault(k, v)
    return cfg


def design_report(config: dict) -> str:
    """Return a text report of law satisfaction/violation."""
    vs = validate_architecture(config)
    errs = sum(1 for v in vs if v["severity"] == "error")
    warns = sum(1 for v in vs if v["severity"] == "warning")
    status = "PASS" if errs == 0 else "FAIL"
    lines = [f"=== Consciousness Design Report ===",
             f"Status: {status}  ({errs} errors, {warns} warnings)",
             f"Factions: {config.get('factions', '?')}/{SIGMA}  "
             f"Alpha: {config.get('coupling_alpha', '?')}/{ALPHA}", ""]
    if vs:
        for v in vs:
            tag = "ERROR" if v["severity"] == "error" else "WARN "
            lines.append(f"  [{tag}] Law {v['law']}: {v['field']} — {v['text'][:72]}")
    else:
        lines.append("All checked consciousness laws satisfied.")
    lines.append(f"\nΨ-constants: α={ALPHA}, balance={BALANCE}, F_c={F_CRITICAL}")
    return "\n".join(lines)


if __name__ == "__main__":
    print("--- suggest_dimensions(100M) ---")
    for k, v in suggest_dimensions(100_000_000).items():
        print(f"  {k}: {v}")
    print("\n--- consciousness_ready_config ---")
    cfg = consciousness_ready_config({"d_model": 384, "num_heads": 12, "num_layers": 6})
    for k, v in cfg.items():
        print(f"  {k}: {v}")
    print("\n--- validate bad config ---")
    for v in validate_architecture({"heads": 7, "dim": 100, "hardcoded_threshold": 0.5, "cells": 2}):
        print(f"  [{v['severity']}] Law {v['law']}: {v['field']}")
    print("\n--- design_report (good config) ---")
    print(design_report(cfg))
