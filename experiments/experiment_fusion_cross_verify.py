"""
Experiment: Fusion Device Cross-Verification
==============================================
Compare n=6 predictions against KSTAR, ITER, SPARC, and JET.
Statistical analysis: is n=6 matching better than chance?
"""

import math
import random
import numpy as np
from collections import Counter

# n=6 constants
SIGMA, TAU, PHI, SOPFR, J2, MU, N = 12, 4, 2, 5, 24, 1, 6

# Derivable values (single operation)
n6_derivable = set()
base = [SIGMA, TAU, PHI, SOPFR, J2, MU, N]
for a in base:
    n6_derivable.add(a)
    for b in base:
        for r in [a+b, a-b, a*b]:
            if 1 <= r <= 200:
                n6_derivable.add(r)
        if b != 0 and a % b == 0:
            n6_derivable.add(a // b)


def check_match(actual, tolerance=0.05):
    """Check if actual value matches any n=6 derivable value."""
    for v in n6_derivable:
        if v == 0:
            continue
        if actual == v:
            return ("EXACT", v)
        if abs(actual - v) / max(abs(v), 1) <= tolerance:
            return ("CLOSE", v)
    return ("MISS", None)


# ═══════════════════════════════════════════════════════════
# Fusion device parameters (all from published sources)
# ═══════════════════════════════════════════════════════════

devices = {
    "KSTAR": {
        "TF_coils": 16,
        "PF_coils": 14,
        "CS_modules": 8,
        "IVC_coils": 4,
        "major_radius_m": 1.8,
        "minor_radius_m": 0.5,
        "aspect_ratio": 3.6,
        "elongation": 2.0,
        "BT_tesla": 3.5,
        "Ip_MA": 2.0,
        "NBI_MW": 8,
        "ECH_MW": 1,
        "ICH_MW": 6,
        "heating_methods": 3,
        "T_keV": 10,
        "density_ctrl_methods": 4,
        "SC_types": 2,
        "T_operation_K": 4.5,
    },
    "ITER": {
        "TF_coils": 18,
        "PF_coils": 6,
        "CS_modules": 6,
        "major_radius_m": 6.2,
        "minor_radius_m": 2.0,
        "aspect_ratio": 3.1,
        "elongation": 1.7,
        "BT_tesla": 5.3,
        "Ip_MA": 15,
        "Q_target": 10,
        "heating_MW_total": 73,
        "NBI_MW": 33,
        "ICRH_MW": 20,
        "ECRH_MW": 20,
        "heating_methods": 3,
        "TBM_ports": 6,
        "blanket_modules": 440,
        "SC_types": 2,
        "T_operation_K": 4.5,
    },
    "SPARC": {
        "TF_coils": 18,
        "major_radius_m": 1.85,
        "minor_radius_m": 0.57,
        "aspect_ratio": 3.25,
        "BT_tesla": 12.2,
        "Ip_MA": 8.7,
        "Q_target": 11,
        "ICRH_MW": 25,
        "heating_methods": 1,  # ICRH only initially
        "SC_types": 1,  # HTS only
    },
    "JET": {
        "TF_coils": 32,
        "PF_coils": 6,
        "major_radius_m": 2.96,
        "minor_radius_m": 1.25,
        "aspect_ratio": 2.37,
        "BT_tesla": 3.45,
        "Ip_MA": 4.8,
        "heating_methods": 3,
        "NBI_MW": 34,
        "ICRH_MW": 10,
    },
    "W7X": {  # Stellarator
        "field_periods": 5,
        "modular_coils": 50,
        "planar_coils": 20,
        "major_radius_m": 5.5,
        "minor_radius_m": 0.53,
        "BT_tesla": 3.0,
        "heating_methods": 3,
        "ECRH_MW": 10,
        "NBI_MW": 7,
    },
}

# Nuclear reactions
reactions = {
    "D-T": {"reactants": (2, 3), "products": (4, 1), "energy_MeV": 17.6,
             "neutron_E": 14.1, "alpha_E": 3.5},
    "D-D_1": {"reactants": (2, 2), "products": (3, 1), "energy_MeV": 3.27},
    "D-D_2": {"reactants": (2, 2), "products": (3, 1), "energy_MeV": 4.03},
    "D-He3": {"reactants": (2, 3), "products": (4, 1), "energy_MeV": 18.3},
    "p-B11": {"reactants": (1, 11), "products": (4, 4, 4), "energy_MeV": 8.7},
}

# Power conversion chain
conversion = {
    "blanket_types": 2,       # solid/liquid
    "TBM_ports_ITER": 6,
    "breeding_material": 6,   # Li-6
    "TBR_target": 1.0,
    "coolant_types": 3,       # He/H2O/PbLi
    "cooling_loops": 2,       # primary/secondary
    "turbine_types": 3,       # steam/gas/combined
    "turbine_stages": 3,      # HP/IP/LP
    "generator_phases": 3,
    "grid_stages": 3,         # step-up/distribution/step-down
    "direct_conversion_methods": 3,
}


def main():
    print("=" * 70)
    print("  FUSION DEVICE CROSS-VERIFICATION")
    print("=" * 70)

    # ─── Device parameter matching ───
    print("\n--- Device Parameter Matching ---\n")

    device_scores = {}
    all_results = []

    for dev_name, params in devices.items():
        exact = close = miss = 0
        for param_name, value in params.items():
            if isinstance(value, float):
                # Check if close to an integer n=6 value
                rounded = round(value)
                result, matched = check_match(rounded)
                if result == "MISS":
                    result, matched = check_match(value, tolerance=0.1)
            else:
                result, matched = check_match(value)

            if result == "EXACT":
                exact += 1
            elif result == "CLOSE":
                close += 1
            else:
                miss += 1
            all_results.append((dev_name, param_name, value, result, matched))

        total = exact + close + miss
        score = (exact + 0.5 * close) / total if total > 0 else 0
        device_scores[dev_name] = {
            "exact": exact, "close": close, "miss": miss,
            "total": total, "score": score
        }
        print(f"  {dev_name:>6}: EXACT={exact:2d} CLOSE={close:2d} MISS={miss:2d} "
              f"(total={total:2d}, score={score:.2f})")

    # ─── Nuclear reaction matching ───
    print("\n--- Nuclear Reaction Matching ---\n")

    for rxn_name, data in reactions.items():
        reactant_sum = sum(data["reactants"])
        product_sum = sum(data["products"])
        r_match = check_match(reactant_sum)
        p_matches = [check_match(p) for p in data["products"]]

        print(f"  {rxn_name}: reactants {data['reactants']} sum={reactant_sum} "
              f"→ {r_match[0]}")
        for i, (p, (res, m)) in enumerate(zip(data["products"], p_matches)):
            print(f"    product {p} → {res} (matched {m})")

    # ─── Power conversion matching ───
    print("\n--- Power Conversion Chain ---\n")
    conv_exact = conv_miss = 0
    for param, value in conversion.items():
        result, matched = check_match(value)
        status = "✅" if result == "EXACT" else "~" if result == "CLOSE" else "❌"
        print(f"  {status} {param}: {value} → {result} ({matched})")
        if result == "EXACT":
            conv_exact += 1
        else:
            conv_miss += 1

    print(f"\n  Conversion chain: {conv_exact}/{len(conversion)} EXACT "
          f"({conv_exact/len(conversion)*100:.0f}%)")

    # ─── Statistical test ───
    print(f"\n{'='*70}")
    print("  STATISTICAL TEST: Is fusion matching better than chance?")
    print(f"{'='*70}\n")

    # Collect all integer parameters
    all_int_params = []
    for dev_name, params in devices.items():
        for param_name, value in params.items():
            if isinstance(value, int) and 1 <= value <= 50:
                all_int_params.append(value)

    n_params = len(all_int_params)
    n_match = sum(1 for v in all_int_params if v in n6_derivable)
    actual_rate = n_match / n_params

    # Random baseline: what fraction of integers 1-50 are n6-derivable?
    n6_in_range = len([v for v in n6_derivable if 1 <= v <= 50])
    base_rate = n6_in_range / 50

    print(f"  Integer params (1-50): {n_params}")
    print(f"  Matched by n=6: {n_match} ({actual_rate:.1%})")
    print(f"  n=6 derivable in 1-50: {n6_in_range}/50 ({base_rate:.1%})")
    print(f"  Expected by chance: {n_params * base_rate:.1f}")

    # Binomial test
    from math import comb
    p = base_rate
    # P(X >= n_match) under binomial(n_params, p)
    p_value = sum(comb(n_params, k) * p**k * (1-p)**(n_params-k)
                  for k in range(n_match, n_params + 1))

    z = (n_match - n_params * p) / max(1, (n_params * p * (1-p)) ** 0.5)

    print(f"  z-score: {z:.2f}")
    print(f"  p-value (one-sided): {p_value:.4f}")
    print(f"  Significant (p < 0.05)? {'YES' if p_value < 0.05 else 'NO'}")

    # ─── Verdict ───
    print(f"\n{'='*70}")
    print("  VERDICT")
    print(f"{'='*70}")

    best_dev = max(device_scores.items(), key=lambda x: x[1]["score"])
    print(f"\n  Best matching device: {best_dev[0]} (score={best_dev[1]['score']:.2f})")
    print(f"  Statistical significance: z={z:.2f}, p={p_value:.4f}")

    if p_value < 0.05:
        print(f"  ✅ Fusion parameters match n=6 BETTER than chance!")
    else:
        print(f"  ❌ Fusion parameters do NOT significantly exceed chance matching.")

    print(f"\n  Strongest matches across ALL devices:")
    for dev, param, val, res, matched in all_results:
        if res == "EXACT" and matched is not None:
            # Find the n=6 expression
            expr = ""
            if matched == N: expr = "n"
            elif matched == SIGMA: expr = "σ"
            elif matched == TAU: expr = "τ"
            elif matched == PHI: expr = "φ"
            elif matched == SOPFR: expr = "sopfr"
            elif matched == J2: expr = "J₂"
            elif matched == MU: expr = "μ"
            elif matched == SIGMA - TAU: expr = "σ-τ"
            elif matched == SIGMA - SOPFR: expr = "σ-sopfr"
            elif matched == SIGMA - MU: expr = "σ-μ"
            elif matched == SOPFR * PHI: expr = "sopfr×φ"
            elif matched == N // PHI: expr = "n/φ"
            else: expr = str(matched)
            print(f"    {dev:>6}.{param}: {val} = {expr}")


if __name__ == "__main__":
    main()
