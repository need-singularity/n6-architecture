#!/usr/bin/env python3
"""
R-spectrum verification: 15 physical constants vs n=6 arithmetic.
"""
import math

S, T, P, SP = 12, 4, 2, 5  # sigma, tau, phi, sopfr of 6
pi, e, ln2, ln3 = math.pi, math.e, math.log(2), math.log(3)

constants = {
    "alpha (fine structure)": 1/137.036,
    "alpha_s(M_Z)": 0.1179,
    "sin²θ_W": 0.2312,
    "m_e (MeV)": 0.511,
    "m_p (MeV)": 938.272,
    "m_p/m_e": 1836.15,
    "Δm(n-p) (MeV)": 1.293,
    "m_H (GeV)": 125.25,
    "m_W (GeV)": 80.377,
    "m_Z (GeV)": 91.1876,
    "m_t (GeV)": 172.76,
    "H₀ (km/s/Mpc)": 67.4,
    "T_CMB (K)": 2.7255,
    "n_s (spectral index)": 0.9649,
    "G_F×10⁵ (GeV⁻²)": 1.1664,
}

# Expression library: ~50 expressions from n=6 arithmetic
exprs = {}
for a in [S,T,P,SP,6]:
    for b in [S,T,P,SP,6,1]:
        if b == 0: continue
        exprs[f"{a}/{b}"] = a/b
        exprs[f"{a}*{b}"] = a*b
        exprs[f"{a}+{b}"] = a+b
        exprs[f"{a}-{b}"] = a-b
        if a > 0: exprs[f"{a}^{b}"] = a**b if b <= 4 else None
for a in [S,T,P,SP,6]:
    exprs[f"1/{a}"] = 1/a
    exprs[f"pi/{a}"] = pi/a
    exprs[f"e/{a}"] = e/a
    exprs[f"ln(2)*{a}"] = ln2*a
    exprs[f"pi^{a}"] = pi**a if a <= 6 else None
    exprs[f"sqrt({a})"] = math.sqrt(a)
    exprs[f"{a}*pi"] = a*pi
    exprs[f"{a}*e"] = a*e
exprs = {k:v for k,v in exprs.items() if v is not None and 0 < abs(v) < 1e20}

# Special combos known from literature
exprs["sigma*tau*phi"] = S*T*P
exprs["6*pi^5"] = 6*pi**5
exprs["sigma*tau"] = S*T
exprs["e^(pi*sqrt(6))"] = math.exp(pi*math.sqrt(6))
exprs["pi^2/6"] = pi**2/6
exprs["ln(4/3)"] = math.log(4/3)
exprs["1/e"] = 1/e
exprs["1/2-ln(4/3)"] = 0.5-math.log(4/3)

print("╔" + "═"*68 + "╗")
print("║  Physical Constants vs n=6 Arithmetic                                ║")
print("╚" + "═"*68 + "╝\n")

results = []
for cname, cval in constants.items():
    best_expr, best_err = None, float('inf')
    for ename, eval_ in exprs.items():
        if eval_ == 0: continue
        # Try direct and reciprocal
        for candidate, label in [(eval_, ename), (1/eval_, f"1/({ename})")]:
            if candidate == 0: continue
            err = abs(candidate - cval) / abs(cval)
            if err < best_err:
                best_err = err
                best_expr = label
                best_val = candidate

    grade = "GREEN" if best_err < 0.001 else "ORANGE" if best_err < 0.01 else "WHITE" if best_err < 0.1 else "BLACK"
    results.append((cname, cval, best_expr, best_val, best_err, grade))

# Print results
print(f"{'Constant':<22} {'Value':>12} {'Best n=6 expr':<25} {'Approx':>12} {'Error':>8} {'Grade':>7}")
print("-"*90)
for cname, cval, expr, approx, err, grade in results:
    icon = {"GREEN":"🟩","ORANGE":"🟧","WHITE":"⚪","BLACK":"⬛"}[grade]
    print(f"{cname:<22} {cval:>12.4f} {expr:<25} {approx:>12.4f} {err:>7.2%} {icon} {grade}")

# Texas Sharpshooter
n_expr = len(exprs)
n_const = len(constants)
greens = sum(1 for r in results if r[5] == "GREEN")
oranges = sum(1 for r in results if r[5] == "ORANGE")
expected_green = n_const * (0.002)  # 0.1% window out of ~range
expected_orange = n_const * (0.02)

print(f"\n{'='*70}")
print(f"Texas Sharpshooter Analysis")
print(f"{'='*70}")
print(f"  Expression library: {n_expr}")
print(f"  Constants tested: {n_const}")
print(f"  GREEN (<0.1%): {greens}  (expected by chance: ~{expected_green:.1f})")
print(f"  ORANGE (<1%):  {oranges}  (expected: ~{expected_orange:.1f})")
print(f"  With {n_expr} expressions × {n_const} constants = {n_expr*n_const} comparisons,")
print(f"  some matches are EXPECTED. Significance requires GREEN >> {expected_green:.0f}.")
grade_counts = {"GREEN":greens, "ORANGE":oranges,
                "WHITE":sum(1 for r in results if r[5]=="WHITE"),
                "BLACK":sum(1 for r in results if r[5]=="BLACK")}
print(f"\n  Grade distribution: {grade_counts}")

verdict = "SIGNIFICANT" if greens > 3*expected_green else "NOT SIGNIFICANT" if greens <= expected_green else "MARGINAL"
print(f"\n  VERDICT: {verdict}")
print(f"  n=6 arithmetic {'does' if verdict=='SIGNIFICANT' else 'does NOT'} predict physical constants beyond chance.")
