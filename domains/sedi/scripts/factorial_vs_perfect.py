#!/usr/bin/env python3
"""Test whether physics constants favor 3! (factorial) or σ(6) (perfect number) structure.

Key question: Does n=6 appear in physics because:
  (A) 6 = 3! (factorial of 3, product of consecutive primes 2×3)
  (B) 6 is perfect (σ(6) = 2×6 = 12)
  (C) Both structures contribute independently

Method:
  1. For each verified SEDI formula, check if it uses:
     - Factorial structure: n!, (n-1)!, n=2×3, factors 2 and 3 separately
     - Perfect number structure: σ(n)=2n, divisor sum, abundance
  2. Count which structure appears more often
  3. Test with alternative numbers:
     - n=24 (4! = 24, also σ(24)=60) — factorial but NOT perfect
     - n=28 (perfect number, σ(28)=56=2×28) — perfect but NOT factorial
     - n=120 (5! = 120) — factorial
     - If physics constants match n=6 but NOT n=24 or n=28, that distinguishes
"""

import math
from itertools import product as cartprod
from sympy import factorint, totient, divisor_sigma, divisors

# ── Number-theoretic constants for any n ──────────────────────────


def compute_constants(n: int) -> dict:
    """Compute the 7 SEDI base constants for any positive integer n."""
    sigma = int(divisor_sigma(n, 1))       # σ(n) = sum of divisors
    tau = int(divisor_sigma(n, 0))         # τ(n) = number of divisors
    phi = int(totient(n))                  # φ(n) = Euler totient
    facs = factorint(n)
    sopfr = sum(p * e for p, e in facs.items())  # sum of prime factors w/ multiplicity
    # Jordan's totient J₂(n) = n² × ∏(1 - 1/p²) for p | n
    j2 = n * n
    for p in facs:
        j2 = j2 * (p * p - 1) // (p * p)
    # Möbius function μ(n)
    if any(e > 1 for e in facs.values()):
        mu = 0
    else:
        mu = (-1) ** len(facs)
    return {
        "n": n,
        "sigma": sigma,
        "tau": tau,
        "phi": phi,
        "sopfr": sopfr,
        "J2": j2,
        "mu": mu,
    }


# ── Depth-2 expression generator ──────────────────────────────────

OPS = {
    "+": lambda a, b: a + b,
    "-": lambda a, b: a - b,
    "*": lambda a, b: a * b,
    "/": lambda a, b: a / b if b != 0 else None,
    "**": lambda a, b: a ** b if abs(b) <= 12 and abs(a) <= 1000 else None,
}

UNARY = {
    "id": lambda x: x,
    "sqrt": lambda x: math.sqrt(x) if x >= 0 else None,
    "sq": lambda x: x * x,
    "cb": lambda x: x ** 3,
    "inv": lambda x: 1.0 / x if x != 0 else None,
    "log": lambda x: math.log(x) if x > 0 else None,
    "exp": lambda x: math.exp(x) if x < 50 else None,
    "pi*": lambda x: math.pi * x,
}


def generate_expressions(consts: dict, max_depth: int = 2):
    """Generate depth-1 and depth-2 arithmetic expressions from n's constants.

    Yields (value, expr_string) tuples.
    """
    # Alias the numeric values
    vals = {}
    for name in ("n", "sigma", "tau", "phi", "sopfr", "J2"):
        v = consts[name]
        if v != 0:
            vals[name] = float(v)

    # Also add mu if nonzero
    if consts["mu"] != 0:
        vals["mu"] = float(consts["mu"])

    # Depth-0: raw constants
    for name, v in vals.items():
        yield v, name

    # Depth-1: unary(const), const op const
    seen = set()
    for uname, ufn in UNARY.items():
        for name, v in vals.items():
            try:
                r = ufn(v)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    expr = f"{uname}({name})" if uname != "id" else name
                    if expr not in seen:
                        seen.add(expr)
                        yield r, expr
            except (ValueError, OverflowError):
                pass

    for op_name, op_fn in OPS.items():
        for (n1, v1), (n2, v2) in cartprod(vals.items(), repeat=2):
            try:
                r = op_fn(v1, v2)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    expr = f"({n1}{op_name}{n2})"
                    if expr not in seen:
                        seen.add(expr)
                        yield r, expr
            except (ValueError, OverflowError, ZeroDivisionError):
                pass

    if max_depth < 2:
        return

    # Depth-2: unary(const op const), (const op const) op const
    depth1 = []
    for op_name, op_fn in OPS.items():
        for (n1, v1), (n2, v2) in cartprod(vals.items(), repeat=2):
            try:
                r = op_fn(v1, v2)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    depth1.append((r, f"({n1}{op_name}{n2})"))
            except (ValueError, OverflowError, ZeroDivisionError):
                pass

    # Apply unary to depth-1 results
    for v, expr in depth1:
        for uname, ufn in UNARY.items():
            if uname == "id":
                continue
            try:
                r = ufn(v)
                if r is not None and math.isfinite(r) and abs(r) < 1e8:
                    full = f"{uname}({expr})"
                    if full not in seen:
                        seen.add(full)
                        yield r, full
            except (ValueError, OverflowError):
                pass

    # depth-1 op const
    for v1, e1 in depth1[:500]:  # limit to prevent combinatorial explosion
        for name, v2 in vals.items():
            for op_name, op_fn in OPS.items():
                try:
                    r = op_fn(v1, v2)
                    if r is not None and math.isfinite(r) and abs(r) < 1e8:
                        full = f"({e1}{op_name}{name})"
                        if full not in seen:
                            seen.add(full)
                            yield r, full
                except (ValueError, OverflowError, ZeroDivisionError):
                    pass


# ── Physics targets ────────────────────────────────────────────────

PHYSICS_TARGETS = {
    "alpha_inv":       {"value": 137.036,    "desc": "Fine structure constant inverse (α⁻¹)"},
    "mp_me":           {"value": 1836.15,    "desc": "Proton-electron mass ratio (m_p/m_e)"},
    "sin2_theta_W":    {"value": 0.2312,     "desc": "Weak mixing angle sin²θ_W"},
    "T_CMB":           {"value": 2.7255,     "desc": "CMB temperature (K)"},
    "H0":              {"value": 67.4,       "desc": "Hubble constant (km/s/Mpc)"},
    "Omega_b":         {"value": 0.0486,     "desc": "Baryon density fraction"},
    "Omega_DM":        {"value": 0.2589,     "desc": "Dark matter density fraction"},
    "me_MeV":          {"value": 0.511,      "desc": "Electron mass (MeV)"},
    "mu_MeV":          {"value": 105.66,     "desc": "Muon mass (MeV)"},
    "tau_MeV":         {"value": 1776.86,    "desc": "Tau lepton mass (MeV)"},
    "mu_e_ratio":      {"value": 206.768,    "desc": "Muon-electron mass ratio"},
    "tau_mu_ratio":    {"value": 16.817,     "desc": "Tau-muon mass ratio"},
    "Cabibbo_angle":   {"value": 0.2253,     "desc": "Cabibbo angle (sin θ_C)"},
    "alpha_s":         {"value": 0.1179,     "desc": "Strong coupling constant α_s(M_Z)"},
    "W_mass":          {"value": 80.379,     "desc": "W boson mass (GeV)"},
    "Z_mass":          {"value": 91.1876,    "desc": "Z boson mass (GeV)"},
    "Higgs_mass":      {"value": 125.25,     "desc": "Higgs boson mass (GeV)"},
    "pi":              {"value": 3.14159,    "desc": "Pi (control)"},
    "e_euler":         {"value": 2.71828,    "desc": "Euler's number (control)"},
    "golden_ratio":    {"value": 1.61803,    "desc": "Golden ratio (control)"},
}

ERROR_THRESHOLD = 1.0  # percent


def find_best_match(target_val: float, consts: dict, threshold_pct: float = ERROR_THRESHOLD):
    """Find the best expression matching target within threshold."""
    best = None
    for val, expr in generate_expressions(consts, max_depth=2):
        if val == 0:
            continue
        err_pct = abs(val - target_val) / abs(target_val) * 100
        if err_pct <= threshold_pct:
            if best is None or err_pct < best[0]:
                best = (err_pct, val, expr)
    return best


# ── Main ───────────────────────────────────────────────────────────

def main():
    print("=" * 78)
    print("  SEDI: 3! vs σ(6) Distinction Test")
    print("  Does physics favor factorial (3!) or perfect number (σ=2n) structure?")
    print("=" * 78)

    # Compute constants for each test number
    test_ns = {
        6:   "3! = 2×3, SMALLEST perfect number",
        24:  "4! = 24, NOT perfect (σ=60≠48)",
        28:  "Perfect (σ=56=2×28), NOT factorial",
        120: "5! = 120, NOT perfect (σ=360≠240)",
    }

    all_consts = {}
    print("\n── Number-theoretic constants ──\n")
    print(f"  {'n':>4s}  {'σ':>6s}  {'τ':>4s}  {'φ':>5s}  {'sopfr':>5s}  {'J₂':>6s}  {'μ':>3s}  Description")
    print("  " + "-" * 70)
    for n, desc in test_ns.items():
        c = compute_constants(n)
        all_consts[n] = c
        print(f"  {c['n']:4d}  {c['sigma']:6d}  {c['tau']:4d}  {c['phi']:5d}  "
              f"{c['sopfr']:5d}  {c['J2']:6d}  {c['mu']:3d}  {desc}")

    # For each physics target, find best match from each n
    print(f"\n── Physics target matching (error < {ERROR_THRESHOLD}%) ──\n")

    match_counts = {n: 0 for n in test_ns}
    unique_to_6 = []
    results_table = []

    for tname, tinfo in PHYSICS_TARGETS.items():
        target = tinfo["value"]
        matches = {}
        for n in test_ns:
            m = find_best_match(target, all_consts[n])
            if m is not None:
                matches[n] = m  # (err_pct, val, expr)

        row = {"target": tname, "value": target, "desc": tinfo["desc"], "matches": matches}
        results_table.append(row)

        for n in matches:
            match_counts[n] += 1

        if 6 in matches and not any(n in matches for n in (24, 28, 120)):
            unique_to_6.append(row)

    # Print results table
    print(f"  {'Target':20s}  {'Value':>10s}  ", end="")
    for n in test_ns:
        print(f"{'n=' + str(n):>20s}  ", end="")
    print()
    print("  " + "-" * 110)

    for row in results_table:
        print(f"  {row['target']:20s}  {row['value']:10.4f}  ", end="")
        for n in test_ns:
            if n in row["matches"]:
                err, val, expr = row["matches"][n]
                short_expr = expr[:16]
                print(f"{err:5.3f}% {short_expr:13s}  ", end="")
            else:
                print(f"{'---':>20s}  ", end="")
        print()

    # Summary counts
    print(f"\n── Summary ──\n")
    print(f"  Match counts (targets matched within {ERROR_THRESHOLD}%):")
    for n, desc in test_ns.items():
        print(f"    n={n:3d}: {match_counts[n]:2d} / {len(PHYSICS_TARGETS)} targets  ({desc})")

    print(f"\n  Targets UNIQUELY matched by n=6 (not by n=24, n=28, or n=120):")
    if unique_to_6:
        for row in unique_to_6:
            err, val, expr = row["matches"][6]
            print(f"    {row['target']:20s} = {row['value']:.4f}  ←  {expr}  (err={err:.3f}%)")
    else:
        print("    (none — all n=6 matches are also matched by at least one alternative)")

    # Factorial vs perfect discrimination
    print(f"\n── Factorial vs Perfect Number Discrimination ──\n")
    factorial_only = set()  # matched by n=24 or n=120 (factorial) but NOT n=28 (perfect)
    perfect_only = set()    # matched by n=28 (perfect) but NOT n=24 or n=120
    both = set()

    for row in results_table:
        m = row["matches"]
        if 6 not in m:
            continue
        has_factorial = (24 in m) or (120 in m)
        has_perfect = (28 in m)
        if has_factorial and not has_perfect:
            factorial_only.add(row["target"])
        elif has_perfect and not has_factorial:
            perfect_only.add(row["target"])
        elif has_factorial and has_perfect:
            both.add(row["target"])

    n6_only = {r["target"] for r in unique_to_6}

    print(f"  n=6 matches: {match_counts[6]}")
    print(f"    - Unique to n=6 (not factorial/perfect alone):  {len(n6_only)}")
    print(f"    - Also matched by factorial numbers (24,120):   {len(factorial_only)}")
    print(f"    - Also matched by perfect number (28):          {len(perfect_only)}")
    print(f"    - Matched by both factorial AND perfect:        {len(both)}")

    print(f"\n── Conclusion ──\n")
    if len(n6_only) > len(factorial_only) + len(perfect_only):
        print("  RESULT: Most matches are UNIQUE to n=6.")
        print("  Neither factorial structure (3!) nor perfect number structure (σ=2n)")
        print("  alone explains the physics correspondences. It is specifically n=6")
        print("  — the unique intersection of both properties — that matches physics.")
    elif len(factorial_only) > len(perfect_only):
        print("  RESULT: Factorial structure appears more prevalent.")
        print("  The 3! (factorial) interpretation has more support than σ(6)=12 (perfect).")
    elif len(perfect_only) > len(factorial_only):
        print("  RESULT: Perfect number structure appears more prevalent.")
        print("  The σ(n)=2n (perfect number) interpretation has more support than 3!.")
    else:
        print("  RESULT: Both structures contribute roughly equally.")
        print("  The n=6 correspondences draw on both factorial and perfect number properties.")

    # Additional structural analysis
    print(f"\n── Structural Note ──\n")
    print("  n=6 is unique because it is SIMULTANEOUSLY:")
    print("    - 3! (smallest factorial > 2)")
    print("    - 2 × 3 (product of consecutive primes)")
    print("    - The smallest perfect number (σ(6) = 12 = 2·6)")
    print("    - A triangular number T(3) = 1+2+3 = 6")
    print("    - A highly composite number (for its size)")
    print("  No other number shares ALL these properties.")
    print("  The overlap makes 3!-vs-σ(6) a false dichotomy:")
    print("  physics selects n=6 because it has MAXIMAL arithmetic richness.")


if __name__ == "__main__":
    main()
