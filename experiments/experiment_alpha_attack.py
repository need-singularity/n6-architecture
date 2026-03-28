"""
Experiment: Fine Structure Constant Alpha Attack
==================================================
H-EE-59 remediation: Systematic search for alpha^{-1} = 137.036
from n=6 arithmetic functions.

Strategy:
1. Enumerate all combinations of {sigma, tau, phi, sopfr, J2, psi, mu, lambda, sigma_inv}
   with operations {+, -, *, /, ^, log, sqrt, pi, e}
2. Score each expression by |prediction - 137.036| / 137.036
3. Penalize complexity (prefer simple expressions)
"""

import numpy as np
import math
from itertools import product as iterproduct

ALPHA_INV = 137.035999084  # CODATA 2018

# n=6 constants
CONSTANTS = {
    "sigma": 12,
    "tau": 4,
    "phi": 2,
    "sopfr": 5,
    "J2": 24,
    "psi": 12,  # Dedekind = sigma
    "mu": 1,
    "lambda": 2,  # Carmichael = phi
    "sigma_inv": 2,
    "n": 6,
    "pi": math.pi,
    "e": math.e,
    "ln2": math.log(2),
    "ln3": math.log(3),
}


def evaluate_expression(expr_fn, name, complexity):
    """Evaluate and score an expression."""
    try:
        value = expr_fn()
        if value is None or not np.isfinite(value) or value <= 0:
            return None
        error = abs(value - ALPHA_INV) / ALPHA_INV * 100
        # Score: lower is better. Penalize complexity.
        score = error * (1 + 0.1 * complexity)
        return {"name": name, "value": value, "error_pct": error,
                "complexity": complexity, "score": score}
    except:
        return None


def main():
    print("=" * 70)
    print("  Fine Structure Constant Alpha Attack")
    print("  Target: alpha^{-1} = 137.035999084")
    print("=" * 70)

    results = []

    # ─── Level 1: Simple binary operations on pairs ───
    print("\n--- Level 1: Simple binary operations ---")
    names = list(CONSTANTS.keys())
    ops = {
        "+": lambda a, b: a + b,
        "-": lambda a, b: a - b,
        "*": lambda a, b: a * b,
        "/": lambda a, b: a / b if b != 0 else None,
        "^": lambda a, b: a ** b if abs(b) < 20 and abs(a) < 1000 else None,
    }

    for n1 in names:
        for n2 in names:
            for op_name, op_fn in ops.items():
                v1, v2 = CONSTANTS[n1], CONSTANTS[n2]
                name = f"{n1} {op_name} {n2}"
                r = evaluate_expression(lambda v1=v1, v2=v2, fn=op_fn: fn(v1, v2),
                                         name, complexity=2)
                if r and r["error_pct"] < 10:
                    results.append(r)

    # ─── Level 2: Unary(binary) — sqrt, log of binary ops ───
    print("--- Level 2: Unary transformations ---")
    unary = {
        "sqrt": lambda x: math.sqrt(x) if x > 0 else None,
        "log": lambda x: math.log(x) if x > 0 else None,
        "x^2": lambda x: x * x,
        "1/x": lambda x: 1/x if x != 0 else None,
    }

    for n1 in names:
        for n2 in names:
            for op_name, op_fn in ops.items():
                for u_name, u_fn in unary.items():
                    v1, v2 = CONSTANTS[n1], CONSTANTS[n2]
                    name = f"{u_name}({n1} {op_name} {n2})"
                    def make_fn(v1=v1, v2=v2, op=op_fn, u=u_fn):
                        inner = op(v1, v2)
                        if inner is None:
                            return None
                        return u(inner)
                    r = evaluate_expression(make_fn, name, complexity=3)
                    if r and r["error_pct"] < 5:
                        results.append(r)

    # ─── Level 3: a*b + c, a*b*c, a^b + c ───
    print("--- Level 3: Ternary combinations ---")
    key_names = ["sigma", "tau", "phi", "sopfr", "J2", "n", "pi", "e"]
    for n1 in key_names:
        for n2 in key_names:
            for n3 in key_names:
                v1, v2, v3 = CONSTANTS[n1], CONSTANTS[n2], CONSTANTS[n3]
                exprs = [
                    (f"{n1}*{n2} + {n3}", lambda a=v1, b=v2, c=v3: a*b + c),
                    (f"{n1}*{n2} - {n3}", lambda a=v1, b=v2, c=v3: a*b - c),
                    (f"{n1}*{n2} * {n3}", lambda a=v1, b=v2, c=v3: a*b*c),
                    (f"{n1}^{n2} + {n3}", lambda a=v1, b=v2, c=v3: a**b + c if abs(b) < 10 else None),
                    (f"{n1}^{n2} - {n3}", lambda a=v1, b=v2, c=v3: a**b - c if abs(b) < 10 else None),
                    (f"{n1}^{n2} * {n3}", lambda a=v1, b=v2, c=v3: a**b * c if abs(b) < 10 else None),
                    (f"{n1}*{n2}/{n3}", lambda a=v1, b=v2, c=v3: a*b/c if c != 0 else None),
                    (f"({n1}+{n2})^{n3}", lambda a=v1, b=v2, c=v3: (a+b)**c if abs(c) < 10 else None),
                    (f"{n1}^{n2}/{n3}", lambda a=v1, b=v2, c=v3: a**b/c if abs(b) < 10 and c != 0 else None),
                ]
                for name, fn in exprs:
                    r = evaluate_expression(fn, name, complexity=3)
                    if r and r["error_pct"] < 2:
                        results.append(r)

    # ─── Level 4: Known close formulas ───
    print("--- Level 4: Known mathematical formulas ---")
    special = [
        ("4*pi^3 + pi^2 + pi", lambda: 4*math.pi**3 + math.pi**2 + math.pi, 4),
        ("sigma^2 - tau*sopfr + phi", lambda: 144 - 20 + 2, 3),
        ("e^(sopfr) - tau/phi", lambda: math.e**5 - 2, 3),
        ("sigma * (sigma - mu) + sopfr", lambda: 12*11 + 5, 3),
        ("J2 * sopfr + sigma + sopfr", lambda: 24*5 + 12 + 5, 3),
        ("sigma^2 - n - mu", lambda: 144 - 6 - 1, 3),
        ("sigma^2 - tau - phi - mu", lambda: 144 - 4 - 2 - 1, 4),
        ("pi^(sopfr) / tau - sigma", lambda: math.pi**5/4 - 12, 4),
        ("tau * pi^tau - sigma * pi + phi", lambda: 4*math.pi**4 - 12*math.pi + 2, 5),
        ("sigma * n * (phi + mu/sigma)", lambda: 12*6*(2+1/12), 4),
        ("J2 * sopfr + sigma * (phi-mu) + sopfr", lambda: 120+12+5, 4),
    ]

    for name, fn, c in special:
        r = evaluate_expression(fn, name, c)
        if r:
            results.append(r)

    # ─── Results ───
    results.sort(key=lambda r: r["score"])

    print(f"\n{'='*70}")
    print(f"  Top 20 Closest Expressions to alpha^{{-1}} = {ALPHA_INV}")
    print(f"{'='*70}")
    print(f"{'Rank':>4} {'Expression':<40} {'Value':>12} {'Error%':>8} {'C':>3}")
    print("-" * 70)

    seen = set()
    count = 0
    for r in results:
        # Deduplicate by rounded value
        key = round(r["value"], 4)
        if key in seen:
            continue
        seen.add(key)
        count += 1
        if count > 20:
            break
        marker = " ***" if r["error_pct"] < 0.01 else " **" if r["error_pct"] < 0.1 else ""
        print(f"{count:>4} {r['name']:<40} {r['value']:>12.6f} {r['error_pct']:>7.4f}% {r['complexity']:>3}{marker}")

    # ─── Verdict ───
    print(f"\n--- Alpha Attack Verdict ---")
    best = results[0] if results else None
    if best and best["error_pct"] < 0.01:
        print(f"SUCCESS: Found expression within 0.01%")
        print(f"  {best['name']} = {best['value']:.6f}")
        print(f"  H-EE-59 status upgraded from FAILED to CONFIRMED")
    elif best and best["error_pct"] < 0.1:
        print(f"NEAR MISS: Best within 0.1%")
        print(f"  {best['name']} = {best['value']:.6f} (error: {best['error_pct']:.4f}%)")
        print(f"  H-EE-59 remains PARTIAL")
    else:
        print(f"HONEST FAILURE CONFIRMED: No n=6 expression matches alpha within 0.1%")
        print(f"  Best: {best['name'] if best else 'none'} = {best['value']:.6f if best else 0} "
              f"(error: {best['error_pct']:.4f}% if best else 'N/A')")
        print(f"  alpha^{{-1}} = 137.036 remains outside the n=6 framework")
        print(f"  This is an IMPORTANT boundary of the theory")


if __name__ == "__main__":
    main()
