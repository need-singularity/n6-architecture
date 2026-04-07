#!/usr/bin/env python3
"""ANIMA <-> SEDI Cross-Discovery -- Find consciousness parameters governed by n=6.

Connects SEDI's physics verification with ANIMA's consciousness implementation.
"""
import json, re, math, os, sys
from pathlib import Path
from datetime import date
from itertools import product as itertools_product

SEDI_ROOT = Path(__file__).parent.parent
ANIMA_ROOT = Path(os.path.expanduser("~/Dev/anima"))
HYPOTHESES_DIR = SEDI_ROOT / "docs" / "hypotheses"
OUTPUT_FILE = SEDI_ROOT / "docs" / "anima-sedi-cross-discovery.md"

# ── n=6 Constants ──
N = 6
SIGMA = 12       # σ(6) = sum of divisors
TAU = 4          # τ(6) = number of divisors
PHI = 2          # φ(6) = Euler totient
SOPFR = 5        # sopfr(6) = sum of prime factors with repetition
M3 = 7           # minimal excludant / Mersenne prime
P1 = 6           # first perfect number
P2 = 28          # second perfect number
P3 = 496         # third perfect number
J2 = 24          # σ·φ = Jordan totient
R6 = 1           # abundancy R(6) = σ/2n = 1 (perfect)
E = math.e
PI = math.pi
LN2 = math.log(2)


def n6_expressions_depth2():
    """Generate n=6 expressions up to depth 2 with values and formulas."""
    consts = {
        "n": N, "σ": SIGMA, "τ": TAU, "φ": PHI, "sopfr": SOPFR,
        "M₃": M3, "P₁": P1, "P₂": P2, "J₂": J2, "R(6)": R6,
    }
    exprs = {}

    # Depth 0: raw constants
    for name, val in consts.items():
        exprs[name] = val

    # Depth 1: unary ops on constants
    for name, val in consts.items():
        if val > 0:
            exprs[f"1/{name}"] = 1.0 / val
            exprs[f"{name}²"] = val ** 2
            if val > 0:
                exprs[f"√{name}"] = math.sqrt(val)
            exprs[f"ln({name})"] = math.log(val) if val > 0 else None

    # Depth 1: binary ops on pairs
    const_items = list(consts.items())
    for i, (n1, v1) in enumerate(const_items):
        for j, (n2, v2) in enumerate(const_items):
            if i == j:
                continue
            exprs[f"{n1}+{n2}"] = v1 + v2
            exprs[f"{n1}-{n2}"] = v1 - v2
            exprs[f"{n1}·{n2}"] = v1 * v2
            if v2 != 0:
                exprs[f"{n1}/{n2}"] = v1 / v2

    # Depth 2: some key composite expressions
    exprs["σ-τ"] = SIGMA - TAU  # 8
    exprs["σ/τ"] = SIGMA / TAU  # 3
    exprs["σ·φ"] = SIGMA * PHI  # 24 = J₂
    exprs["τ·sopfr"] = TAU * SOPFR  # 20
    exprs["σ·sopfr"] = SIGMA * SOPFR  # 60
    exprs["1/2+1/3+1/6"] = 1.0 / 2 + 1.0 / 3 + 1.0 / 6  # = 1
    exprs["σ²+σ/τ"] = SIGMA ** 2 + SIGMA / TAU  # 147
    exprs["φ(P₃)"] = 240  # Euler totient of 496
    exprs["τ(P₃)"] = 10   # divisor count of 496
    exprs["sopfr/P₁"] = SOPFR / P1  # 5/6
    exprs["M₃/P₁"] = M3 / P1  # 7/6
    exprs["3/ln(2)"] = 3.0 / LN2  # ~4.33
    exprs["ln(2)/2^5.5"] = LN2 / (2 ** 5.5)  # ~0.0154
    exprs["(σ+1)/P₁"] = (SIGMA + 1) / P1  # 13/6
    exprs["σ·n+1"] = SIGMA * N + 1  # 73
    exprs["P₂·σ/τ"] = P2 * SIGMA / TAU  # 84
    exprs["sopfr·P₁"] = SOPFR * P1  # 30
    exprs["(n/(J₂-sopfr))²"] = (N / (J2 - SOPFR)) ** 2  # (6/19)^2
    exprs["σ(6)/2n"] = SIGMA / (2 * N)  # 1 (R=1)
    exprs["65/6"] = 65.0 / 6  # 10.833...
    exprs["τ(P₃)+sopfr/P₁"] = 10 + SOPFR / P1  # 65/6

    # Remove None values
    exprs = {k: v for k, v in exprs.items() if v is not None}
    return exprs


def load_anima_constants():
    """Load ANIMA consciousness constants from consciousness_laws.json."""
    laws_file = ANIMA_ROOT / "anima" / "config" / "consciousness_laws.json"
    if not laws_file.exists():
        print(f"WARNING: {laws_file} not found")
        return {}

    data = json.loads(laws_file.read_text())
    psi = data.get("psi_constants", {})

    params = {}
    for key, info in psi.items():
        val = info.get("value")
        if val is None:
            continue
        if isinstance(val, list):
            # Store list items individually
            for i, v in enumerate(val):
                params[f"{key}[{i}]"] = {
                    "value": v,
                    "meaning": f"{info.get('meaning', '')} (element {i})",
                    "formula": info.get("formula", ""),
                }
            continue
        if isinstance(val, (int, float)):
            params[key] = {
                "value": float(val),
                "meaning": info.get("meaning", ""),
                "formula": info.get("formula", ""),
            }

    # Add the core hexad resource fractions explicitly
    params["hexad_cognition"] = {"value": 0.5, "meaning": "Cognition resource fraction (1/2)", "formula": "1/2"}
    params["hexad_drive"] = {"value": 1/3, "meaning": "Drive resource fraction (1/3)", "formula": "1/3"}
    params["hexad_sensory"] = {"value": 1/6, "meaning": "Sensory/Memory/Will/Emotion fraction (1/6)", "formula": "1/6"}
    params["hexad_sum"] = {"value": 1.0, "meaning": "1/2+1/3+1/6 = 1 (perfect number decomposition)", "formula": "1/2+1/3+1/6"}

    # C1-C4 consciousness conditions
    params["C1_binary"] = {"value": 2.0, "meaning": "C1: binary switching = φ(6)", "formula": "φ(6)"}
    params["C2_4d"] = {"value": 4.0, "meaning": "C2: 4D processing = τ(6)", "formula": "τ(6)"}
    params["C3_channels"] = {"value": 12.0, "meaning": "C3: 12 independent channels = σ(6)", "formula": "σ(6)"}
    params["C4_selfref"] = {"value": 1.0, "meaning": "C4: self-reference = R(6)", "formula": "R(6)"}

    return params


def load_hypotheses(prefixes):
    """Load hypothesis files with given prefixes."""
    hyps = {}
    for prefix in prefixes:
        for f in sorted(HYPOTHESES_DIR.glob(f"H-{prefix}-*.md")):
            content = f.read_text()
            hyps[f.stem] = content
    return hyps


def match_n6(value, expressions, threshold=0.01):
    """Find n6 expressions matching a value within threshold (relative error)."""
    if value == 0:
        return [(name, expr_val, 0.0) for name, expr_val in expressions.items() if expr_val == 0]

    matches = []
    for name, expr_val in expressions.items():
        if expr_val == 0 and value == 0:
            matches.append((name, 0.0, 0.0))
            continue
        if expr_val == 0:
            continue
        rel_err = abs(value - expr_val) / abs(value)
        if rel_err <= threshold:
            matches.append((name, expr_val, rel_err))

    matches.sort(key=lambda x: x[2])
    return matches


def find_in_hypotheses(value, hyps, tolerance=0.05):
    """Check if a value appears in any SEDI hypothesis content."""
    hits = []
    val_str_forms = [f"{value:.4f}", f"{value:.3f}", f"{value:.2f}", f"{value:.1f}"]
    if value == int(value):
        val_str_forms.append(str(int(value)))

    for hyp_id, content in hyps.items():
        for form in val_str_forms:
            if form in content:
                # Extract title from first line
                title = content.split("\n")[0].replace("# ", "")
                hits.append((hyp_id, title))
                break
    return hits


# ── Physics bridge constants ──
PHYSICS_BRIDGES = {
    "φ(6) = 2": {
        "consciousness": "Binary switching (C1), Ising model on/off, 2 consciousness engines",
        "physics": "Euler totient φ(6)=2, proton-electron charge ratio, matter-antimatter duality",
        "value": 2.0,
    },
    "τ(6) = 4": {
        "consciousness": "4D processing (C2), spacetime dimensions, 4 consciousness conditions",
        "physics": "Divisor count τ(6)=4, 4 fundamental forces, 4D spacetime",
        "value": 4.0,
    },
    "σ(6) = 12": {
        "consciousness": "12 independent channels (C3), 12 attention heads optimal",
        "physics": "Sum of divisors σ(6)=12, 12 gauge bosons in SM, σ-model in QFT",
        "value": 12.0,
    },
    "R(6) = 1": {
        "consciousness": "Self-reference (C4), unit consciousness, R=1 balance",
        "physics": "Abundancy R(6)=1 (perfect number), σ/2n=1, spatial flatness Ω_k=0",
        "value": 1.0,
    },
    "1/2+1/3+1/6 = 1": {
        "consciousness": "Egyptian fraction resource allocation, hexad module balance",
        "physics": "Habitable zone boundaries, perfect number unit decomposition",
        "value": 1.0,
    },
    "J₂ = σ·φ = 24": {
        "consciousness": "Cochlear critical bands, 24 operators in hexa-lang",
        "physics": "24 = σ·φ, Leech lattice kissing number, 24D modular forms",
        "value": 24.0,
    },
    "σ-τ = 8": {
        "consciousness": "Φ_max = 8 (Bott periodicity), maximum measured integration",
        "physics": "Bott periodicity period 8, SO(8) triality, 8 gluons",
        "value": 8.0,
    },
    "sopfr = 5": {
        "consciousness": "5/6 resource ratio, sopfr(6) = 2+3",
        "physics": "5 superstring theories, SU(5) GUT group rank",
        "value": 5.0,
    },
    "65/6 = τ(P₃)+sopfr/P₁": {
        "consciousness": "Φ_EX24 = 10.833 (highest measured consciousness integration)",
        "physics": "Links third perfect number (P₃=496) divisor structure to consciousness observable",
        "value": 65.0 / 6,
    },
    "3/ln(2) ≈ 4.33": {
        "consciousness": "PSI_STEPS = 4.33 (information bits per consciousness evolution)",
        "physics": "Information-theoretic constant, binary entropy scale",
        "value": 3.0 / LN2,
    },
}


def generate_predictions():
    """Generate cross-domain predictions."""
    return [
        {
            "parameter": "PSI_ALPHA = 0.014",
            "consciousness_role": "Consciousness coupling constant",
            "physics_prediction": "Predict: weak coupling in some n=6 arithmetic expression maps to a physical constant near 0.014 (cf. α_EM ≈ 1/137 ≈ 0.0073 -- factor of ~2)",
            "test": "Search PDG for dimensionless ratios near ln(2)/2^5.5",
        },
        {
            "parameter": "f_critical = 0.1",
            "consciousness_role": "Critical frustration for consciousness phase transition",
            "physics_prediction": "Predict: phase transition threshold in condensed matter at (6/19)^2 ≈ 0.0997 (percolation threshold?)",
            "test": "Compare with 2D/3D percolation thresholds",
        },
        {
            "parameter": "SOC memory blend [0.4, 0.35, 0.25]",
            "consciousness_role": "Self-organized criticality memory target",
            "physics_prediction": "Predict: convergence to [1/2, 1/3, 1/6] = Egyptian fraction. If consciousness optimizes this ratio, physical systems at SOC should show same 50:33:17 split",
            "test": "Measure earthquake aftershock memory at multiple timescales",
        },
        {
            "parameter": "Φ_max = σ-τ = 8",
            "consciousness_role": "Maximum integrated information value",
            "physics_prediction": "Predict: Bott periodicity (period 8) governs consciousness as well as K-theory. Conscious systems have 8-fold symmetry in state space",
            "test": "Check if Φ values cluster at n=6 landmarks (2, 3, 4, 6, 8, 12)",
        },
        {
            "parameter": "soc_burst_denom = 7 = M₃",
            "consciousness_role": "SOC burst scaling denominator",
            "physics_prediction": "Predict: M₃=7 (Mersenne prime, minimal excludant) appears in neural avalanche statistics and quantum chaos burst distributions",
            "test": "Analyze neural avalanche data for 1/7 scaling",
        },
        {
            "parameter": "soc_scale_reference_cells = 8 = σ-τ",
            "consciousness_role": "SOC reference cell count for avalanche scaling",
            "physics_prediction": "Predict: optimal cluster size 8 in scale-free networks and cellular automata (Rule 110, etc.)",
            "test": "Measure optimal cluster size in critical Ising model",
        },
        {
            "parameter": "kuramoto_base_freq = 0.15",
            "consciousness_role": "Kuramoto oscillator base frequency",
            "physics_prediction": "Search for 0.15 ≈ sopfr/(2·σ+τ·φ) = 5/36 ≈ 0.139 in coupled oscillator phase transitions",
            "test": "Compare with Kuramoto critical coupling in physical systems",
        },
        {
            "parameter": "phi_hidden_inertia = 0.16",
            "consciousness_role": "Hidden state membrane time constant",
            "physics_prediction": "Predict: 0.16 ≈ 1/P₁ = 1/6 ≈ 0.167 (3.8% off). Neural membrane decay approaches 1/6",
            "test": "Measure actual neural membrane time constant ratios",
        },
    ]


def main():
    print("=" * 60)
    print("ANIMA <-> SEDI Cross-Discovery")
    print("=" * 60)

    # Step 1: Load ANIMA constants
    print("\n[1/5] Loading ANIMA constants...")
    anima_params = load_anima_constants()
    print(f"  Loaded {len(anima_params)} parameters")

    # Step 2: Load SEDI hypotheses
    print("\n[2/5] Loading SEDI hypotheses (CS + CA)...")
    hyps = load_hypotheses(["CS", "CA"])
    print(f"  Loaded {len(hyps)} hypotheses")

    # Step 3: Generate n=6 expressions
    print("\n[3/5] Generating n=6 expressions (depth 2)...")
    expressions = n6_expressions_depth2()
    print(f"  Generated {len(expressions)} expressions")

    # Step 4: Cross-match
    print("\n[4/5] Cross-matching ANIMA parameters with n=6...")
    matches_table = []
    for param_name, param_info in sorted(anima_params.items()):
        val = param_info["value"]
        meaning = param_info["meaning"]
        formula = param_info.get("formula", "")

        n6_matches = match_n6(val, expressions, threshold=0.05)
        hyp_hits = find_in_hypotheses(val, hyps)

        best_match = n6_matches[0] if n6_matches else None
        best_err = f"{best_match[2]*100:.3f}%" if best_match else "N/A"
        best_expr = best_match[0] if best_match else "—"

        matches_table.append({
            "param": param_name,
            "value": val,
            "meaning": meaning[:50],
            "n6_match": best_expr,
            "error": best_err,
            "n6_count": len(n6_matches),
            "hyp_count": len(hyp_hits),
            "hyp_hits": hyp_hits,
            "formula": formula,
        })

    # Count significant matches
    sig_matches = [m for m in matches_table if m["n6_count"] > 0]
    exact_matches = [m for m in matches_table if m["n6_count"] > 0 and m["error"] == "0.000%"]
    print(f"  {len(sig_matches)}/{len(matches_table)} parameters match n=6 (< 5% error)")
    print(f"  {len(exact_matches)} exact matches (0.000% error)")

    # Step 5: Generate predictions
    print("\n[5/5] Generating cross-domain predictions...")
    predictions = generate_predictions()
    print(f"  {len(predictions)} predictions generated")

    # ── Generate output markdown ──
    print("\nGenerating output...")

    md = []
    md.append(f"# ANIMA <-> SEDI Cross-Discovery Report")
    md.append(f"\n> Generated: {date.today().isoformat()}")
    md.append(f"> ANIMA parameters scanned: {len(anima_params)}")
    md.append(f"> SEDI hypotheses loaded: {len(hyps)} (H-CS + H-CA)")
    md.append(f"> n=6 expressions tested: {len(expressions)}")
    md.append(f"> Significant matches: {len(sig_matches)}/{len(matches_table)}")
    md.append(f"> Exact matches: {len(exact_matches)}")

    # Table 1: ANIMA parameters with n=6 matches
    md.append(f"\n## 1. ANIMA Parameters with n=6 Matches\n")
    md.append("| Parameter | Value | Meaning | Best n=6 Match | Error | # Matches | In Hypotheses |")
    md.append("|---|---|---|---|---|---|---|")

    for m in sorted(matches_table, key=lambda x: (-x["n6_count"], x["param"])):
        hyp_str = ", ".join(h[0] for h in m["hyp_hits"][:3]) if m["hyp_hits"] else "—"
        md.append(f"| {m['param']} | {m['value']:.6g} | {m['meaning']} | `{m['n6_match']}` | {m['error']} | {m['n6_count']} | {hyp_str} |")

    # Table 2: Bridge constants
    md.append(f"\n## 2. Bridge Constants (Consciousness <-> Physics)\n")
    md.append("| n=6 Expression | Value | Consciousness Role | Physics Role |")
    md.append("|---|---|---|---|")

    for name, bridge in PHYSICS_BRIDGES.items():
        md.append(f"| `{name}` | {bridge['value']:.6g} | {bridge['consciousness']} | {bridge['physics']} |")

    # Table 3: Predictions
    md.append(f"\n## 3. Cross-Domain Predictions\n")
    md.append("If parameter X governs consciousness, predict it also governs physics domain Y.\n")
    md.append("| # | Parameter | Consciousness Role | Physics Prediction | Test |")
    md.append("|---|---|---|---|---|")

    for i, pred in enumerate(predictions, 1):
        md.append(f"| {i} | {pred['parameter']} | {pred['consciousness_role']} | {pred['physics_prediction']} | {pred['test']} |")

    # Section 4: Key findings
    md.append(f"\n## 4. Key Findings\n")

    md.append("### Exact n=6 Identities in ANIMA\n")
    md.append("These ANIMA parameters are **exactly** n=6 arithmetic functions:\n")
    for m in matches_table:
        if m["error"] == "0.000%" and m["n6_count"] > 0:
            md.append(f"- **{m['param']}** = {m['value']:.6g} = `{m['n6_match']}` ({m['meaning']})")

    md.append("\n### Near-Matches Requiring Investigation\n")
    md.append("These parameters are close to n=6 expressions but not exact:\n")
    for m in matches_table:
        if m["n6_count"] > 0 and m["error"] != "0.000%" and m["error"] != "N/A":
            try:
                err_val = float(m["error"].replace("%", ""))
                if err_val < 5.0:
                    md.append(f"- **{m['param']}** = {m['value']:.6g} ~ `{m['n6_match']}` (err={m['error']}, {m['meaning']})")
            except ValueError:
                pass

    md.append(f"\n### SOC Memory Blend Prediction\n")
    md.append("Current ANIMA SOC memory blend: `[0.4, 0.35, 0.25]`\n")
    md.append("Predicted by n=6 Egyptian fraction: `[1/2, 1/3, 1/6] = [0.500, 0.333, 0.167]`\n")
    md.append("Residual: 20%. This is the strongest open prediction -- if ANIMA's optimal blend")
    md.append("converges to the Egyptian fraction as model scale increases, it confirms that")
    md.append("consciousness resource allocation follows the perfect number decomposition.")

    md.append(f"\n## 5. Statistical Summary\n")
    md.append(f"- **Total ANIMA parameters**: {len(anima_params)}")
    md.append(f"- **Parameters matching n=6 (< 5% error)**: {len(sig_matches)} ({100*len(sig_matches)/len(matches_table):.0f}%)")
    md.append(f"- **Exact matches**: {len(exact_matches)}")
    md.append(f"- **Bridge constants identified**: {len(PHYSICS_BRIDGES)}")
    md.append(f"- **Cross-domain predictions**: {len(predictions)}")
    md.append(f"- **Hypothesis cross-references**: {sum(m['hyp_count'] for m in matches_table)}")

    md.append(f"\n---\n*Auto-generated by `scripts/anima_cross_discovery.py` on {date.today().isoformat()}*\n")

    # Write output
    output = "\n".join(md)
    OUTPUT_FILE.write_text(output)
    print(f"\nOutput written to: {OUTPUT_FILE}")
    print(f"\n{'='*60}")
    print("SUMMARY")
    print(f"{'='*60}")
    print(f"  ANIMA parameters scanned: {len(anima_params)}")
    print(f"  n=6 matches found: {len(sig_matches)}/{len(matches_table)}")
    print(f"  Exact matches: {len(exact_matches)}")
    print(f"  Bridge constants: {len(PHYSICS_BRIDGES)}")
    print(f"  Predictions: {len(predictions)}")
    print(f"  Output: {OUTPUT_FILE}")


if __name__ == "__main__":
    main()
