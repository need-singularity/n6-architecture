#!/usr/bin/env python3
"""n=6 Uniqueness Tester -- Check if an identity holds only for n=6

Tests arithmetic function identities for uniqueness via exhaustive search.
Based on H-CX-502 and 400-hypothesis offensive campaign.
Uses tecsrs Rust library for sieve computation (83x faster).

Arithmetic functions available in expressions:
  sigma(n)  -- sum of divisors
  tau(n)    -- number of divisors
  phi(n)    -- Euler totient
  sopfr(n)  -- sum of prime factors (with repetition)
  omega(n)  -- number of distinct prime factors
  lcm1n(n)  -- lcm(1, 2, ..., n)
  tri(n)    -- triangular number n*(n+1)//2
  fact(n)   -- n! factorial
  n         -- the number itself

Usage:
  python3 calc/n6_uniqueness_tester.py --equation "phi*sigma==n*tau" --limit 5000
  python3 calc/n6_uniqueness_tester.py --known                    # Test all known unique identities
  python3 calc/n6_uniqueness_tester.py --scan --limit 1000        # Find new unique equations
"""

import argparse
import math
import sys

import tecsrs


def build_tables(limit):
    """Precompute arithmetic function tables up to limit using Rust sieves."""
    rt = tecsrs.SieveTables(limit)
    sigma_t = rt.sigma_list()
    tau_t = rt.tau_list()
    phi_t = rt.phi_list()
    sopfr_t = rt.sopfr_list()
    omega_t = rt.omega_list()
    # lcm(1..n) still in Python (not in tecsrs)
    lcm1n_t = [0] * (limit + 1)
    if limit >= 1:
        lcm1n_t[1] = 1
    for n in range(2, limit + 1):
        lcm1n_t[n] = lcm1n_t[n-1] * n // math.gcd(lcm1n_t[n-1], n)

    return {
        'sigma':  sigma_t,
        'tau':    tau_t,
        'phi':    phi_t,
        'sopfr':  sopfr_t,
        'omega':  omega_t,
        'lcm1n':  lcm1n_t,
    }


# ═══════════════════════════════════════════════════════════════
# Known unique identities from GZ offensive campaign
# ═══════════════════════════════════════════════════════════════

KNOWN_IDENTITIES = [
    {
        "id":   1,
        "name": "phi*sigma = n*tau",
        "desc": "phi(n)*sigma(n) = n*tau(n)",
        "fn":   lambda n, T: T['phi'][n] * T['sigma'][n] == n * T['tau'][n],
    },
    {
        "id":   2,
        "name": "(n-3)! = n  (n>=4)",
        "desc": "For n>=4: (n-3)! = n",
        "fn":   lambda n, T: n >= 4 and math.factorial(n - 3) == n,
    },
    {
        "id":   3,
        "name": "tri(n) = fact(n)  (sum=product)",
        "desc": "n*(n+1)/2 = n! (triangular = factorial)",
        "fn":   lambda n, T: n > 0 and n*(n+1)//2 == math.factorial(n),
    },
    {
        "id":   4,
        "name": "sigma(tau(sigma(n))) = sigma(n)  (self-loop)",
        "desc": "sigma(tau(sigma(n))) = sigma(n)",
        "fn":   lambda n, T: (
            T['sigma'][n] <= len(T['sigma'])-1 and
            T['tau'][T['sigma'][n]] <= len(T['sigma'])-1 and
            T['sigma'][T['tau'][T['sigma'][n]]] == T['sigma'][n]
        ),
    },
    {
        "id":   5,
        "name": "3n-6 = sigma(n)  (n-body DOF)",
        "desc": "3*n - 6 = sigma(n)",
        "fn":   lambda n, T: 3*n - 6 == T['sigma'][n],
    },
    {
        "id":   6,
        "name": "n-2 = tau(n)  (Cayley exponent)",
        "desc": "n - 2 = tau(n)",
        "fn":   lambda n, T: n - 2 == T['tau'][n],
    },
    {
        "id":   7,
        "name": "(n-1)!/2 = sopfr(n)*sigma(n)  (Hamiltonian)",
        "desc": "(n-1)!/2 = sopfr(n) * sigma(n)",
        "fn":   lambda n, T: n > 1 and math.factorial(n-1) // 2 == T['sopfr'][n] * T['sigma'][n],
    },
    {
        "id":   8,
        "name": "lcm(1..n) = sopfr(n)*sigma(n)",
        "desc": "lcm(1..n) = sopfr(n) * sigma(n)",
        "fn":   lambda n, T: T['lcm1n'][n] == T['sopfr'][n] * T['sigma'][n],
    },
    {
        "id":   9,
        "name": "tau(n)*phi(n) = sigma(n)-tau(n)",
        "desc": "tau(n)*phi(n) = sigma(n) - tau(n)",
        "fn":   lambda n, T: T['tau'][n] * T['phi'][n] == T['sigma'][n] - T['tau'][n],
    },
    {
        "id":   10,
        "name": "sigma(n) = 2*n  (perfect number definition)",
        "desc": "sigma(n) = 2*n  (definition of perfect number)",
        "fn":   lambda n, T: T['sigma'][n] == 2 * n,
    },
    {
        "id":   11,
        "name": "sopfr(phi(n)) = omega(n)  (composition identity)",
        "desc": "sopfr(phi(n)) = omega(n)  -- unique composition, verified to 50,000",
        "fn":   lambda n, T: (
            T['phi'][n] >= 2 and T['phi'][n] < len(T['sopfr']) and
            T['sopfr'][T['phi'][n]] == T['omega'][n]
        ),
    },
]


# ═══════════════════════════════════════════════════════════════
# Custom equation evaluator
# ═══════════════════════════════════════════════════════════════

def make_eval_fn(equation):
    """Build a test function from an equation string like 'phi*sigma==n*tau'.

    Variables: phi, sigma, tau, sopfr, omega, n, lcm1n, tri, fact
    """
    # Replace single '=' that isn't already '==' with '=='
    eq = equation.strip()
    # Normalize: replace '==' markers -- already correct
    code = f"""
def _test(n, T):
    phi   = T['phi'][n]
    sigma = T['sigma'][n]
    tau   = T['tau'][n]
    sopfr = T['sopfr'][n]
    omega = T['omega'][n]
    lcm1n = T['lcm1n'][n]
    tri   = n * (n + 1) // 2
    try:
        fact = __import__('math').factorial(n)
    except Exception:
        fact = 0
    try:
        return bool({eq})
    except Exception:
        return False
"""
    namespace = {}
    exec(code, namespace)
    return namespace['_test']


# ═══════════════════════════════════════════════════════════════
# Test a single identity
# ═══════════════════════════════════════════════════════════════

def test_identity(fn, tables, limit, name=""):
    """Test fn(n, T) for n in 1..limit. Return list of n that satisfy it."""
    hits = [n for n in range(1, limit + 1) if fn(n, tables)]
    return hits


def print_identity_result(identity, hits, limit):
    is_unique = (hits == [6]) or (6 in hits and len(hits) == 1)
    only_hit = len(hits) == 1 and hits[0] == 6
    # Summarize hits
    if len(hits) == 0:
        hit_str = "none"
        status = "MISS (6 not satisfied)"
    elif len(hits) <= 10:
        hit_str = str(hits)
        status = "UNIQUE to n=6" if only_hit else f"n=6 + others ({len(hits)} total)"
    else:
        hit_str = str(hits[:10]) + f"... ({len(hits)} total)"
        status = "COMMON (many hits)"

    icon = "+" if only_hit else ("-" if 6 not in hits else "~")
    print(f"  [{icon}] {identity['name']}")
    print(f"      {identity['desc']}")
    print(f"      Hits (n<=>{limit}): {hit_str}")
    print(f"      Status: {status}")
    print()


# ═══════════════════════════════════════════════════════════════
# Scan for unique equations
# ═══════════════════════════════════════════════════════════════

SCAN_TEMPLATES = [
    "sigma == 2*n",
    "phi + tau == n",
    "phi * tau == n",
    "sigma - phi == tau * n",
    "sigma == phi + tau + n",
    "omega * phi == tau",
    "sopfr + phi == n + tau",
    "sigma * omega == n * tau",
    "phi * sigma == n * tau",
    "sopfr * tau == phi",
    "sigma - tau == phi * tau",
    "lcm1n == sopfr * sigma",
    "omega + tau == phi",
    "3*n - 6 == sigma",
    "n - 2 == tau",
    "n - 4 == phi",
]


def print_scan(tables, limit):
    print()
    print(f"  Scanning {len(SCAN_TEMPLATES)} equation templates for n=6 uniqueness (n<=>{limit})")
    print("  " + "=" * 60)
    unique_found = []
    for tmpl in SCAN_TEMPLATES:
        try:
            fn = make_eval_fn(tmpl)
            hits = test_identity(fn, tables, limit)
            only_6 = (hits == [6])
            if only_6:
                unique_found.append(tmpl)
            icon = "+" if only_6 else " "
            hit_summary = str(hits[:6]) + ("..." if len(hits) > 6 else "")
            print(f"  [{icon}] {tmpl:<40}  hits: {hit_summary}")
        except Exception as e:
            print(f"  [E] {tmpl:<40}  error: {e}")
    print()
    if unique_found:
        print(f"  Unique-to-6 equations found: {len(unique_found)}")
        for eq in unique_found:
            print(f"    * {eq}")
    else:
        print("  No new unique-to-6 equations found in template set.")


# ═══════════════════════════════════════════════════════════════
# Main
# ═══════════════════════════════════════════════════════════════

def main():
    parser = argparse.ArgumentParser(
        description="n=6 Uniqueness Tester -- exhaustive search for identities unique to n=6"
    )
    parser.add_argument("--equation", type=str, metavar="EQ",
                        help="Test custom equation, e.g. 'phi*sigma==n*tau'")
    parser.add_argument("--limit", type=int, default=1000,
                        help="Search limit (default: 1000)")
    parser.add_argument("--known", action="store_true",
                        help="Test all known unique identities from campaign")
    parser.add_argument("--scan", action="store_true",
                        help="Scan template equations for new unique identities")
    args = parser.parse_args()

    limit = args.limit
    print()
    print("  ╔══════════════════════════════════════════════════════╗")
    print("  ║        n=6 Uniqueness Tester (H-CX-502)              ║")
    print(f"  ║        Search limit: n <= {limit:<6}                      ║")
    print("  ╚══════════════════════════════════════════════════════╝")
    print()
    print(f"  Building arithmetic function tables up to n={limit}...", end=" ", flush=True)
    tables = build_tables(limit)
    print("done.")
    print()

    if args.equation:
        fn = make_eval_fn(args.equation)
        hits = test_identity(fn, tables, limit)
        ident = {"name": args.equation, "desc": args.equation}
        print(f"  Testing: {args.equation}")
        print("  " + "=" * 50)
        print_identity_result(ident, hits, limit)

    elif args.known:
        print(f"  Testing {len(KNOWN_IDENTITIES)} known unique identities")
        print("  " + "=" * 60)
        print()
        unique_count = 0
        for ident in KNOWN_IDENTITIES:
            hits = test_identity(ident['fn'], tables, limit)
            print_identity_result(ident, hits, limit)
            if hits == [6]:
                unique_count += 1
        print(f"  Summary: {unique_count}/{len(KNOWN_IDENTITIES)} identities unique to n=6 (n<={limit})")

    elif args.scan:
        print_scan(tables, limit)

    else:
        # Default: show known identities
        print(f"  Testing {len(KNOWN_IDENTITIES)} known unique identities (default mode)")
        print("  " + "=" * 60)
        print()
        unique_count = 0
        for ident in KNOWN_IDENTITIES:
            hits = test_identity(ident['fn'], tables, limit)
            print_identity_result(ident, hits, limit)
            if hits == [6]:
                unique_count += 1
        print(f"  Summary: {unique_count}/{len(KNOWN_IDENTITIES)} identities unique to n=6 (n<={limit})")
        print()
        print("  Use --known, --equation EQ, or --scan for more options.")


if __name__ == "__main__":
    main()
