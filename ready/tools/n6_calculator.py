#!/usr/bin/env python3
"""
N6 Master Calculator — All n=6 arithmetic functions and design tools.

Usage:
  python tools/n6_calculator.py                    # Show all n=6 constants
  python tools/n6_calculator.py --r-score 120 160 12  # R-score for d=120, ff=160, h=12
  python tools/n6_calculator.py --find 137          # Find n=6 expression for a number
  python tools/n6_calculator.py --leech             # Leech-24 energy for current config
  python tools/n6_calculator.py --chip 12           # Chip metrics for NxN tensor core
  python tools/n6_calculator.py --industry 128      # Check if 128 matches n=6 pattern
  python tools/n6_calculator.py --spectrum 50       # R(n) spectrum for n=1..50
"""

import math
import sys
import argparse

# ─── Core Arithmetic Functions ───

def sigma(n):
    """Sum of divisors."""
    return sum(d for d in range(1, n+1) if n % d == 0)

def tau(n):
    """Count of divisors."""
    return sum(1 for d in range(1, n+1) if n % d == 0)

def euler_phi(n):
    """Euler's totient."""
    return sum(1 for k in range(1, n+1) if math.gcd(k, n) == 1)

def sopfr(n):
    """Sum of prime factors with repetition."""
    s, d, temp = 0, 2, n
    while d * d <= temp:
        while temp % d == 0:
            s += d
            temp //= d
        d += 1
    if temp > 1:
        s += temp
    return s

def mobius(n):
    """Mobius function."""
    if n == 1: return 1
    d, factors = 2, 0
    temp = n
    while d * d <= temp:
        if temp % d == 0:
            factors += 1
            temp //= d
            if temp % d == 0: return 0
        d += 1
    if temp > 1: factors += 1
    return (-1) ** factors

def jordan_j2(n):
    """Jordan totient J_2(n)."""
    result = n * n
    temp = n
    d = 2
    while d * d <= temp:
        if temp % d == 0:
            result = result * (d*d - 1) // (d*d)
            while temp % d == 0:
                temp //= d
        d += 1
    if temp > 1:
        result = result * (temp*temp - 1) // (temp*temp)
    return result

def dedekind_psi(n):
    """Dedekind psi function."""
    result = n
    temp = n
    d = 2
    while d * d <= temp:
        if temp % d == 0:
            result = result * (d + 1) // d
            while temp % d == 0:
                temp //= d
        d += 1
    if temp > 1:
        result = result * (temp + 1) // temp
    return result

def carmichael_lambda(n):
    """Carmichael function (reduced totient)."""
    if n <= 2: return 1
    result = 1
    temp = n
    d = 2
    while d * d <= temp:
        if temp % d == 0:
            pk = 1
            while temp % d == 0:
                pk *= d
                temp //= d
            if d == 2 and pk >= 8:
                lam = pk // 4
            elif d == 2:
                lam = pk // 2
            else:
                lam = pk * (d-1) // d
            result = result * lam // math.gcd(result, lam)
        d += 1
    if temp > 1:
        lam = temp - 1
        result = result * lam // math.gcd(result, lam)
    return result

def sigma_inv(n):
    """Sum of reciprocals of divisors."""
    return sum(1/d for d in range(1, n+1) if n % d == 0)

def R(n):
    """Balance ratio. R(n)=1 iff n in {1,6}."""
    if n < 1: return float('inf')
    return (sigma(n) * euler_phi(n)) / (n * tau(n))


# ─── Derived Constants ───

N6 = {
    "n": 6,
    "sigma": sigma(6),          # 12
    "tau": tau(6),              # 4
    "phi": euler_phi(6),        # 2
    "sopfr": sopfr(6),          # 5
    "J2": jordan_j2(6),        # 24
    "psi": dedekind_psi(6),    # 12
    "mu": mobius(6),            # 1
    "lambda": carmichael_lambda(6),  # 2
    "sigma_inv": sigma_inv(6),  # 2.0
    "R": R(6),                  # 1.0
}

DERIVED = {
    "tau^2/sigma (FFN ratio)": N6["tau"]**2 / N6["sigma"],        # 4/3
    "phi/tau (active ratio)": N6["phi"] / N6["tau"],              # 1/2
    "sigma-tau (Bott/SHA)": N6["sigma"] - N6["tau"],              # 8
    "sigma-sopfr (AES/IPv6)": N6["sigma"] - N6["sopfr"],         # 7
    "sigma-mu (RSA)": N6["sigma"] - N6["mu"],                    # 11
    "J2-tau (ChaCha/AA)": N6["J2"] - N6["tau"],                  # 20
    "sigma*sopfr (60Hz)": N6["sigma"] * N6["sopfr"],              # 60
    "sigma*tau (48kHz)": N6["sigma"] * N6["tau"],                 # 48
    "sigma*phi (Leech)": N6["sigma"] * N6["phi"],                 # 24
    "sigma^2 (junctions)": N6["sigma"] ** 2,                      # 144
    "1/e (Boltzmann)": 1/math.e,                                  # 0.3679
    "ln(4/3) (Mertens)": math.log(4/3),                          # 0.2877
    "sopfr-phi (CAP/RGB)": N6["sopfr"] - N6["phi"],              # 3
}


# ─── Calculators ───

def show_constants():
    """Display all n=6 constants."""
    print("=" * 55)
    print("  N6 Arithmetic Constants (n=6)")
    print("=" * 55)
    print(f"\n  Core Functions:")
    for k, v in N6.items():
        print(f"    {k:<12} = {v}")
    print(f"\n  Derived Ratios:")
    for k, v in DERIVED.items():
        print(f"    {k:<30} = {v:.6g}")
    print(f"\n  Egyptian Fractions: 1/2 + 1/3 + 1/6 = {1/2+1/3+1/6}")
    print(f"  R(6) = sigma*phi/(n*tau) = {N6['sigma']}*{N6['phi']}/({N6['n']}*{N6['tau']}) = {N6['R']}")


def calc_r_score(d_model, d_ff, n_heads, activation="phi6"):
    """Calculate architecture R-score."""
    print(f"\n  R-Score Calculator")
    print(f"  d_model={d_model}, d_ff={d_ff}, n_heads={n_heads}, act={activation}")
    print("-" * 50)

    # Sigma subsystem
    s_score = 1.0 if d_model % 12 == 0 else 1.0 - min(d_model%12, 12-d_model%12)/12
    # Tau subsystem
    ratio = d_ff / d_model if d_model else 0
    t_score = max(0, 1.0 - abs(ratio - 4/3) / (4/3))
    # N subsystem
    n_map = {"phi6": 1.0, "gelu": 0.5, "relu": 0.5, "silu": 0.5}
    n_score = n_map.get(activation, 0.3)
    # Phi subsystem
    p_score = 1.0  # dense model default

    r = s_score * t_score * n_score * p_score
    print(f"  sigma (d_model%12):  {s_score:.3f}")
    print(f"  tau   (FFN ratio):   {t_score:.3f}  (ratio={ratio:.3f}, target=1.333)")
    print(f"  n     (activation):  {n_score:.3f}  ({activation})")
    print(f"  phi   (routing):     {p_score:.3f}")
    print(f"  R-score = {r:.4f}")
    if r > 0.9:
        print(f"  Status: EXCELLENT (near R=1 optimum)")
    elif r > 0.5:
        print(f"  Status: GOOD (room for improvement)")
    else:
        print(f"  Status: POOR (far from n=6 optimum)")


def find_expression(target, tolerance=0.01):
    """Find n=6 arithmetic expressions matching a target number."""
    print(f"\n  Expression Finder: target = {target}")
    print("-" * 60)

    S, T, P, SP, J, MU = 12, 4, 2, 5, 24, 1
    consts = {"sigma":S, "tau":T, "phi":P, "sopfr":SP, "J2":J, "mu":MU, "n":6}
    results = []

    # Level 1: binary
    names = list(consts.keys())
    for n1 in names:
        for n2 in names:
            a, b = consts[n1], consts[n2]
            ops = [
                (f"{n1}+{n2}", a+b), (f"{n1}-{n2}", a-b),
                (f"{n1}*{n2}", a*b), (f"{n1}/{n2}", a/b if b else None),
            ]
            if abs(b) < 15 and a > 0:
                ops.append((f"{n1}^{n2}", a**b))
            for name, val in ops:
                if val is not None and abs(val) < 1e10:
                    err = abs(val - target) / max(abs(target), 1e-10) * 100
                    if err < tolerance * 100:
                        results.append((err, val, name, 2))

    # Level 2: ternary
    for n1 in names:
        for n2 in names:
            for n3 in names:
                a, b, c = consts[n1], consts[n2], consts[n3]
                ops = [
                    (f"{n1}*{n2}+{n3}", a*b+c),
                    (f"{n1}*{n2}-{n3}", a*b-c),
                    (f"{n1}*{n2}*{n3}", a*b*c),
                    (f"({n1}+{n2})*{n3}", (a+b)*c),
                    (f"({n1}-{n2})*{n3}", (a-b)*c),
                ]
                if b != 0:
                    ops.append((f"{n1}*{n2}/{n3}", a*b/c if c else None))
                if abs(b) < 8 and a > 0:
                    ops.append((f"{n1}^{n2}+{n3}", a**b+c))
                    ops.append((f"{n1}^{n2}-{n3}", a**b-c))
                for name, val in ops:
                    if val is not None and abs(val) < 1e10:
                        err = abs(val - target) / max(abs(target), 1e-10) * 100
                        if err < tolerance * 100:
                            results.append((err, val, name, 3))

    # Level 3: with fractions
    for n1 in names:
        for n2 in names:
            a, b = consts[n1], consts[n2]
            base_int = round(target)
            for base_name, base_expr in [(f"{n1}*{n2}", a*b), (f"{n1}+{n2}", a+b),
                                          (f"{n1}-{n2}", a-b)]:
                for n3 in names:
                    for n4 in names:
                        c, d = consts[n3], consts[n4]
                        if d == 0 or c == 0: continue
                        val = base_expr + c/d
                        err = abs(val - target) / max(abs(target), 1e-10) * 100
                        if err < tolerance * 100:
                            results.append((err, val, f"{base_name}+{n3}/{n4}", 4))

    results.sort()
    seen = set()
    count = 0
    print(f"{'Rank':>4} {'Expression':<40} {'Value':>12} {'Error':>10}")
    print("-" * 70)
    for err, val, name, complexity in results:
        key = round(val, 6)
        if key in seen: continue
        seen.add(key)
        count += 1
        if count > 15: break
        print(f"{count:>4} {name:<40} {val:>12.6f} {err:>9.6f}%")


def leech_energy(config=None):
    """Compute Leech-24 energy for a config."""
    from engine.leech24_surface import energy, phi_from_energy, N6_OPTIMA

    if config is None:
        config = dict(N6_OPTIMA)  # default: n=6 optimum

    E, details = energy(config)
    phi = phi_from_energy(E)

    print(f"\n  Leech-24 Energy Calculator")
    print("-" * 55)
    print(f"  E(config) = {E:.6f}")
    print(f"  Phi = 1/(1+E) = {phi:.6f}")
    print(f"\n  Top contributors:")
    sorted_d = sorted(details.items(), key=lambda x: -x[1]["contribution"])
    for name, info in sorted_d[:8]:
        if info["contribution"] > 0.001:
            print(f"    {name:<25} val={info['value']:.3f} opt={info['optimum']:.3f} "
                  f"E+={info['contribution']:.4f}")


def chip_metrics(core_size):
    """Calculate chip metrics for NxN tensor core."""
    print(f"\n  Chip Metrics: {core_size}x{core_size} Tensor Core")
    print("-" * 50)

    macs = core_size ** 2
    divs = sum(1 for d in range(1, core_size+1) if core_size % d == 0)
    divisors = [d for d in range(1, core_size+1) if core_size % d == 0]
    mzi = core_size * (core_size - 1) // 2

    # Compare to 16x16 baseline
    macs_16 = 256
    mzi_16 = 120

    print(f"  MACs:        {macs} ({macs/macs_16*100:.0f}% of 16x16)")
    print(f"  Divisors:    {divs} {divisors}")
    print(f"  MZI (optic): {mzi} ({mzi/mzi_16*100:.0f}% of 16x16)")
    print(f"  Density:     {divs/macs:.4f} divisors/MAC")
    print(f"  Head dims:   {[d for d in divisors if d >= 4]}")

    # Valid head configs for common d_model
    for d in [48, 120, 240, 360, 720]:
        valid = [h for h in divisors if d % h == 0 and h >= 2]
        if valid:
            print(f"  d={d}: valid heads = {valid}")


def industry_check(number):
    """Check if a number matches n=6 arithmetic."""
    print(f"\n  Industry Pattern Check: {number}")
    print("-" * 55)

    S, T, P, SP, J, MU = 12, 4, 2, 5, 24, 1
    matches = []

    expressions = {
        "n": 6, "sigma": S, "tau": T, "phi": P, "sopfr": SP,
        "J2": J, "mu": MU, "sigma-tau": S-T, "sigma-sopfr": S-SP,
        "sigma-mu": S-MU, "sigma*sopfr": S*SP, "sigma*tau": S*T,
        "sigma*phi": S*P, "sigma^2": S**2, "J2-tau": J-T,
        "sopfr-phi": SP-P, "tau-1": T-1, "n+1": 7,
        "tau^2": T**2, "tau^3": T**3,
    }

    # Powers of 2
    for exp_name, exp_val in expressions.items():
        if exp_val > 0 and exp_val < 30:
            power = 2 ** exp_val
            if power == number:
                matches.append(f"2^({exp_name}) = 2^{exp_val} = {power}")

    # Direct
    for exp_name, exp_val in expressions.items():
        if exp_val == number:
            matches.append(f"{exp_name} = {exp_val}")

    # Multiples of 1000
    if number >= 1000:
        base = number / 1000
        for exp_name, exp_val in expressions.items():
            if abs(exp_val - base) < 0.001:
                matches.append(f"{exp_name} * 1000 = {exp_val}k")

    if matches:
        print(f"  MATCH found:")
        for m in matches:
            print(f"    {m}")
    else:
        print(f"  No direct n=6 expression found for {number}")
        # Nearest
        all_vals = [(abs(v - number), k, v) for k, v in expressions.items()]
        all_vals.sort()
        print(f"  Nearest: {all_vals[0][1]} = {all_vals[0][2]} (distance {all_vals[0][0]})")


def r_spectrum(max_n):
    """Display R(n) spectrum."""
    print(f"\n  R(n) Spectrum (n=1..{max_n})")
    print("-" * 45)
    print(f"  {'n':>4} {'sigma':>5} {'tau':>4} {'phi':>4} {'R(n)':>10}")
    for n in range(1, max_n+1):
        r = R(n)
        marker = " <-- R=1!" if abs(r-1) < 1e-10 else ""
        print(f"  {n:>4} {sigma(n):>5} {tau(n):>4} {euler_phi(n):>4} {r:>10.6f}{marker}")


# ─── Main ───

def main():
    parser = argparse.ArgumentParser(description="N6 Architecture Calculator")
    parser.add_argument("--r-score", nargs=3, type=int, metavar=("D_MODEL", "D_FF", "HEADS"),
                       help="Compute R-score for architecture")
    parser.add_argument("--find", type=float, help="Find n=6 expression for a number")
    parser.add_argument("--leech", action="store_true", help="Leech-24 energy at N6 optimum")
    parser.add_argument("--chip", type=int, help="Chip metrics for NxN tensor core")
    parser.add_argument("--industry", type=int, help="Check if number matches n=6")
    parser.add_argument("--spectrum", type=int, help="R(n) spectrum up to n")
    parser.add_argument("--tolerance", type=float, default=0.01, help="Tolerance for --find (fraction)")
    args = parser.parse_args()

    if args.r_score:
        calc_r_score(*args.r_score)
    elif args.find:
        find_expression(args.find, args.tolerance)
    elif args.leech:
        leech_energy()
    elif args.chip:
        chip_metrics(args.chip)
    elif args.industry:
        industry_check(args.industry)
    elif args.spectrum:
        r_spectrum(args.spectrum)
    else:
        show_constants()


if __name__ == "__main__":
    main()
