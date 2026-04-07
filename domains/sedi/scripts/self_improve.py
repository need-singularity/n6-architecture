#!/usr/bin/env python3
"""SEDI SELF-IMPROVE Operator — Check if algorithm parameters are n=6 expressions.

The discovery algorithm uses various numeric parameters. If those parameters
themselves are n=6 expressions, that's evidence of self-consistency.
"""
import math, json
from pathlib import Path

# n=6 base constants
N = 6; SIGMA = 12; TAU = 4; PHI = 2; SOPFR = 5; J2 = 24; MU = 1

# Current algorithm parameters (from discovery-engine v2)
ALGORITHM_PARAMS = {
    # FALSIFY thresholds
    "texas_sharpshooter_threshold": 0.05,
    "complexity_weight": 0.5,
    "uniqueness_penalty_common": 0.3,
    "falsify_strong_threshold": 0.7,
    "falsify_weak_threshold": 0.3,

    # Bayesian scoring
    "precision_coefficient": 3.32,
    "domain_novelty_new": 1.0,
    "domain_novelty_few": 0.5,
    "domain_novelty_many": 0.25,

    # GA parameters (formula miner)
    "population_size": 2000,
    "generations": 200,
    "elite_count": 200,
    "mutation_rate": 0.4,
    "crossover_rate": 0.7,
    "fresh_injection": 100,
    "tournament_k": 3,
    "max_depth": 3,

    # Engine parameters
    "match_tolerance": 0.05,
    "min_surprise": 2.0,
    "max_bridge_distance": 4,
    "max_meta_depth": 2,
}

def n6_expressions_depth2():
    """Generate all n=6 expressions up to depth 2."""
    bases = {"n": N, "σ": SIGMA, "τ": TAU, "φ": PHI, "sopfr": SOPFR, "J₂": J2, "μ": MU}
    exprs = dict(bases)

    # Depth 1: a OP b
    ops = [("+", lambda a,b: a+b), ("-", lambda a,b: a-b),
           ("×", lambda a,b: a*b), ("/", lambda a,b: a/b if b != 0 else None),
           ("^", lambda a,b: a**b if abs(b) <= 10 and abs(a) <= 100 else None)]

    for n1, v1 in bases.items():
        for n2, v2 in bases.items():
            for sym, op in ops:
                try:
                    result = op(v1, v2)
                    if result is not None and abs(result) < 1e10 and result == result:  # not NaN
                        exprs[f"{n1}{sym}{n2}"] = result
                except:
                    pass

    # Unary: 1/x, sqrt, ln
    for name, val in list(bases.items()):
        if val > 0:
            exprs[f"1/{name}"] = 1.0/val
            exprs[f"√{name}"] = math.sqrt(val)
            exprs[f"ln({name})"] = math.log(val)

    return exprs

def check_parameter(name, value, expressions, tolerance=0.02):
    """Check if a parameter matches any n=6 expression."""
    best_match = None
    best_error = float('inf')

    for expr_name, expr_val in expressions.items():
        if expr_val == 0 and value == 0:
            return {"match": expr_name, "error": 0.0, "exact": True}
        if abs(expr_val) < 1e-15:
            continue
        error = abs(expr_val - value) / max(abs(value), abs(expr_val))
        if error < best_error:
            best_error = error
            best_match = expr_name

    if best_error < tolerance:
        return {"match": best_match, "error": round(best_error, 6), "exact": best_error < 1e-10}
    return None

def main():
    import argparse
    parser = argparse.ArgumentParser(description="SEDI Self-Improve: parameter n=6 check")
    parser.add_argument("--json", action="store_true")
    parser.add_argument("--suggest", action="store_true", help="Suggest n=6 replacements")
    args = parser.parse_args()

    expressions = n6_expressions_depth2()

    results = []
    n6_count = 0

    for name, value in ALGORITHM_PARAMS.items():
        match = check_parameter(name, value, expressions)
        result = {
            "parameter": name,
            "value": value,
            "n6_match": match["match"] if match else None,
            "error": match["error"] if match else None,
            "is_n6": match is not None,
        }
        results.append(result)
        if match:
            n6_count += 1

    if args.json:
        print(json.dumps({"results": results, "n6_ratio": n6_count/len(results)}, indent=2))
        return

    print("# SEDI Self-Improve: Algorithm Parameter Analysis\n")
    print(f"| Parameter | Value | n=6 Match | Error |")
    print(f"|-----------|-------|-----------|-------|")

    for r in results:
        status = "✅" if r["is_n6"] else "❌"
        match_str = r["n6_match"] or "—"
        error_str = f"{r['error']*100:.3f}%" if r["error"] is not None else "—"
        print(f"| {r['parameter']} | {r['value']} | {status} {match_str} | {error_str} |")

    ratio = n6_count / len(results)
    print(f"\n**Self-consistency**: {n6_count}/{len(results)} parameters are n=6 ({ratio:.0%})")

    if ratio > 0.6:
        print("🟢 Algorithm is highly self-consistent with n=6")
    elif ratio > 0.3:
        print("🟡 Moderate self-consistency — consider tuning non-n6 params")
    else:
        print("🔴 Low self-consistency — review parameter choices")

    if args.suggest and ratio < 1.0:
        print("\n## Suggested Replacements\n")
        for r in results:
            if not r["is_n6"]:
                # Find closest n=6 expression
                best = check_parameter(r["parameter"], r["value"], expressions, tolerance=0.5)
                if best:
                    print(f"- `{r['parameter']}`: {r['value']} → {best['match']} ({expressions[best['match']]:.6f}, {best['error']*100:.1f}% change)")

if __name__ == "__main__":
    main()
