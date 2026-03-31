#!/usr/bin/env python3
"""
Deep Optimizer Hyperparameter vs n=6 Arithmetic Analysis
=========================================================
Exhaustive search: do optimizer defaults encode perfect number 6 structure?

n=6 constants: sigma=12, tau=4, phi=2, sopfr=5, n!=720
Golden Zone: center=1/e, upper=1/2, lower=1/2-ln(4/3), width=ln(4/3)

Analyzes: Adam, AdamW, SGD, LAMB, Lion, Sophia, Adafactor, Prodigy,
          Schedule-Free, Muon, CAME, LARS, RAdam, Adadelta, RMSprop

Usage:
    python3 tools/optimizer_constants_n6_deep.py
    python3 tools/optimizer_constants_n6_deep.py --section 1   # Adam only
    python3 tools/optimizer_constants_n6_deep.py --section 4   # 0.9 meta-pattern
"""

import math
import argparse
import itertools
from typing import List, Tuple, Optional

# ============================================================
# n=6 Constants
# ============================================================
N = 6
SIGMA = 12          # sum of divisors sigma(6)
TAU = 4             # number of divisors tau(6)
PHI = 2             # Euler totient phi(6)
SOPFR = 5           # sum of prime factors 2+3
GPF = 3             # greatest prime factor
LPF = 2             # least prime factor
FACTORIAL = 720     # 6!
SIGMA_MINUS_TAU = 8 # sigma - tau
E = math.e
INV_E = 1.0 / E
LN2 = math.log(2)
LN_4_3 = math.log(4.0 / 3.0)
GZ_UPPER = 0.5
GZ_LOWER = 0.5 - LN_4_3
GZ_CENTER = INV_E

# Jordan's totient J_2(6) = 24
J2 = 24

# Dedekind psi(6) = 12
DEDEKIND_PSI = 12

# ============================================================
# Expression Engine: systematic n=6 formula search
# ============================================================

# Atom pool: name, value pairs
ATOMS = [
    ("n", N), ("sigma", SIGMA), ("tau", TAU), ("phi", PHI),
    ("sopfr", SOPFR), ("gpf", GPF), ("n!", FACTORIAL),
    ("1/e", INV_E), ("ln2", LN2), ("ln(4/3)", LN_4_3),
    ("J2", J2), ("1", 1), ("2", 2), ("3", 3), ("10", 10),
    ("100", 100), ("1000", 1000),
    ("sigma-tau", SIGMA_MINUS_TAU),
]


def generate_expressions(max_complexity=3) -> List[Tuple[str, float]]:
    """Generate n=6 arithmetic expressions up to given complexity.

    Complexity 1: single atoms and simple unary ops (1/x)
    Complexity 2: binary ops on atoms (a+b, a-b, a*b, a/b, a^b)
    Complexity 3: 1-1/expr, expr/10^k, and selected ternary combos
    """
    exprs = []
    seen_vals = set()

    def add(name, val):
        if val is None or not math.isfinite(val) or abs(val) > 1e12:
            return
        key = round(val, 12)
        if key not in seen_vals:
            seen_vals.add(key)
            exprs.append((name, val))

    # Level 1: atoms + reciprocals
    for aname, aval in ATOMS:
        add(aname, aval)
        if aval != 0:
            add(f"1/{aname}", 1.0 / aval)

    # Level 2: binary ops on n=6-specific atoms only
    n6_atoms = [a for a in ATOMS if a[0] in
                ("n", "sigma", "tau", "phi", "sopfr", "gpf", "n!",
                 "1/e", "ln2", "ln(4/3)", "J2", "sigma-tau")]

    for (an, av), (bn, bv) in itertools.product(n6_atoms, repeat=2):
        add(f"{an}+{bn}", av + bv)
        add(f"{an}-{bn}", av - bv)
        add(f"{an}*{bn}", av * bv)
        if bv != 0:
            add(f"{an}/{bn}", av / bv)
        if av > 0 and abs(bv) <= 10 and bv == int(bv):
            try:
                add(f"{an}^{bn}", av ** bv)
            except (OverflowError, ValueError):
                pass

    # Level 3: common optimizer patterns
    # 1 - 1/expr (momentum-like)
    for (en, ev) in list(exprs):
        if ev > 1:
            add(f"1-1/{en}", 1.0 - 1.0 / ev)
        if ev > 0 and ev < 1:
            add(f"1-{en}", 1.0 - ev)

    # expr * 10^(-k) for k=1..8 (learning rate patterns)
    for (en, ev) in list(exprs)[:50]:  # top atoms only
        for k in range(1, 9):
            add(f"{en}*10^(-{k})", ev * 10**(-k))

    # (a*b)/(c) three-atom combos (selected)
    for (an, av), (bn, bv), (cn, cv) in itertools.combinations(n6_atoms[:8], 3):
        if cv != 0:
            add(f"{an}*{bn}/{cn}", av * bv / cv)
            add(f"({an}+{bn})/{cn}", (av + bv) / cv)
        if bv != 0:
            add(f"{an}*{cn}/{bn}", av * cv / bv)

    if max_complexity >= 3:
        # 1 - 1/(a*b) patterns
        for (an, av), (bn, bv) in itertools.product(n6_atoms[:8], repeat=2):
            prod = av * bv
            if prod > 1:
                add(f"1-1/({an}*{bn})", 1.0 - 1.0 / prod)
            diff = av - bv
            if diff > 1:
                add(f"1-1/({an}-{bn})", 1.0 - 1.0 / diff)
            s = av + bv
            if s > 1:
                add(f"1-1/({an}+{bn})", 1.0 - 1.0 / s)

    return exprs


def find_best_match(target: float, exprs: List[Tuple[str, float]],
                    top_k: int = 5) -> List[Tuple[str, float, float]]:
    """Find the top-k expressions closest to target value."""
    if target == 0:
        results = [(n, v, abs(v)) for n, v in exprs]
    else:
        results = [(n, v, abs(v - target) / abs(target) * 100.0) for n, v in exprs]
    results.sort(key=lambda x: x[2])
    return results[:top_k]


def bonferroni_p(error_pct: float, search_space: int) -> float:
    """Bonferroni-corrected p-value for matching within error_pct%."""
    # Under uniform [0, 2*target], prob of landing within error_pct% of target
    p_single = min(1.0, 2.0 * error_pct / 100.0)
    p_corrected = 1.0 - (1.0 - p_single) ** search_space
    return min(1.0, p_corrected)


# ============================================================
# Section 1: Adam Hyperparameters Exhaustive Analysis
# ============================================================

def section_adam_exhaustive(exprs):
    print("=" * 78)
    print("SECTION 1: ADAM HYPERPARAMETERS EXHAUSTIVE ANALYSIS")
    print("=" * 78)
    print()
    print("Source: Kingma & Ba, 'Adam: A Method for Stochastic Optimization', 2015")
    print("        (arXiv:1412.6980)")
    print()

    adam_params = [
        ("beta1", 0.9, "First moment decay"),
        ("beta2", 0.999, "Second moment decay"),
        ("epsilon", 1e-8, "Numerical stability"),
        ("lr (Adam paper)", 0.001, "Default learning rate"),
        ("lr (Karpathy)", 3e-4, "Universal LLM learning rate"),
        ("wd (AdamW)", 0.01, "Default weight decay (Loshchilov & Hutter 2019)"),
        ("wd (transformer)", 0.1, "Common transformer weight decay"),
    ]

    search_space = len(exprs)
    print(f"Search space: {search_space} n=6 expressions")
    print()

    for pname, pval, desc in adam_params:
        print(f"--- {pname} = {pval} ({desc}) ---")
        matches = find_best_match(pval, exprs, top_k=5)
        print(f"  {'Rank':<5} {'Expression':<35} {'Value':<14} {'Error%':<10} {'p(Bonf)':<10}")
        print(f"  {'-'*5} {'-'*35} {'-'*14} {'-'*10} {'-'*10}")
        for i, (ename, eval_, err) in enumerate(matches, 1):
            p = bonferroni_p(err, search_space)
            sig = "***" if p < 0.001 else "**" if p < 0.01 else "*" if p < 0.05 else ""
            print(f"  {i:<5} {ename:<35} {eval_:<14.8f} {err:<10.4f} {p:<10.6f} {sig}")
        print()

    # Special analysis: beta2 = 1-1/720 vs 1-1/1000
    print("--- DEEP DIVE: beta2 = 0.999 ---")
    print()
    cands = [
        ("1 - 1/6!", 1 - 1/720, "n=6 factorial"),
        ("1 - 1/1000", 1 - 1/1000, "round number (human preference)"),
        ("1 - 1/(sigma*tau*sopfr)", 1 - 1/(SIGMA*TAU*SOPFR), "240 = sigma*tau*sopfr"),
        ("1 - 1/(J2*tau*sopfr)", 1 - 1/(J2*TAU*SOPFR), "480 = J2*tau*sopfr"),
        ("1 - 1/sigma^phi", 1 - 1/SIGMA**PHI, "144 = sigma^phi"),
        ("1 - 1/(n*sigma*tau)", 1 - 1/(N*SIGMA*TAU), "288"),
        ("1 - 1/(n!)", 1 - 1/FACTORIAL, "720 = 6!"),
        ("1 - phi/n!", 1 - PHI/FACTORIAL, "718/720"),
    ]
    print(f"  {'Expression':<35} {'Value':<14} {'Error vs 0.999':<16} {'Natural?'}")
    print(f"  {'-'*35} {'-'*14} {'-'*16} {'-'*20}")
    for name, val, note in cands:
        err = abs(val - 0.999) / 0.999 * 100
        print(f"  {name:<35} {val:<14.8f} {err:<16.4f}% {note}")

    print()
    print("  Verdict: 1-1/1000 has 0% error (trivially, it IS 0.999).")
    print("  But 0.999 = 1-0.001 is suspiciously a power of 10.")
    print("  1-1/6! = 0.99861 has 0.039% error -- within the range where")
    print("  practitioners find beta2 insensitive (0.99-0.9999).")
    print("  The RANGE [0.99, 0.9999] corresponds to [1-1/100, 1-1/10000]")
    print("  = [1-1/10^phi, 1-1/10^tau]. This is more interesting.")
    print()

    # Special: epsilon = 1e-8
    print("--- DEEP DIVE: epsilon = 1e-8 ---")
    print(f"  10^(-8) where 8 = sigma - tau = {SIGMA} - {TAU} = {SIGMA_MINUS_TAU}")
    print(f"  Also: 8 = 2^gpf = 2^{GPF} = {2**GPF}")
    print(f"  Also: 8 = sigma - tau = n + phi = {N} + {PHI}")
    print(f"  Note: epsilon is purely for numerical stability, not optimization.")
    print(f"  Modern practice often uses 1e-6 or 1e-12 depending on dtype.")
    print(f"  Significance: LOW (epsilon chosen for float32 precision, not theory)")
    print()

    # Special: lr = 3e-4
    print("--- DEEP DIVE: Karpathy's lr = 3e-4 ---")
    print(f"  3e-4 = gpf(6) * 10^(-tau) = {GPF} * 10^(-{TAU})")
    print(f"       = (n/phi) * 10^(-tau) = {N}/{PHI} * 10^(-{TAU})")
    print(f"  Error: 0.000% (exact if n=6 expression)")
    print(f"  But: 3 is just a small integer. 10^-4 is a human scale.")
    print(f"  Competing explanation: 3e-4 works because it's in the")
    print(f"  sweet spot of LR landscape. The '3' might be anthropic.")
    print(f"  Test: Does lr=2e-4 or 5e-4 work comparably? If yes, '3' is noise.")
    print(f"  Literature: Yes, [1e-4, 6e-4] all work. The range matters, not '3'.")
    print()


# ============================================================
# Section 2: ALL Major Optimizers — Real Defaults
# ============================================================

def section_all_optimizers(exprs):
    print("=" * 78)
    print("SECTION 2: ALL MAJOR OPTIMIZERS — REAL DEFAULT VALUES")
    print("=" * 78)
    print()

    # Each entry: (optimizer, [(param_name, value, source)])
    optimizers = [
        ("SGD", [
            ("lr", 0.01, "PyTorch default"),
            ("momentum", 0.0, "PyTorch default (0.9 when enabled)"),
            ("weight_decay", 0.0, "PyTorch default"),
            ("dampening", 0.0, "PyTorch default"),
        ]),
        ("Adam (Kingma & Ba 2015)", [
            ("lr", 0.001, "Paper Table 1"),
            ("beta1", 0.9, "Paper Table 1"),
            ("beta2", 0.999, "Paper Table 1"),
            ("epsilon", 1e-8, "Paper Table 1"),
        ]),
        ("AdamW (Loshchilov & Hutter 2019)", [
            ("lr", 0.001, "PyTorch default"),
            ("beta1", 0.9, "Same as Adam"),
            ("beta2", 0.999, "Same as Adam"),
            ("epsilon", 1e-8, "Same as Adam"),
            ("weight_decay", 0.01, "Paper default"),
        ]),
        ("LAMB (You+ 2020)", [
            ("lr", 0.001, "Paper"),
            ("beta1", 0.9, "Same as Adam"),
            ("beta2", 0.999, "Same as Adam"),
            ("epsilon", 1e-6, "Paper"),
            ("weight_decay", 0.01, "Paper"),
            ("trust_ratio_clip", 10.0, "Paper"),
        ]),
        ("LARS (You+ 2017)", [
            ("lr", 0.001, "Paper"),
            ("momentum", 0.9, "Paper"),
            ("weight_decay", 1e-4, "Paper"),
            ("trust_coefficient", 0.001, "Paper"),
        ]),
        ("Lion (Chen+ Google 2023)", [
            ("lr", 1e-4, "Paper (3-10x smaller than Adam)"),
            ("beta1", 0.9, "Paper Table 1"),
            ("beta2", 0.99, "Paper Table 1"),
            ("weight_decay", 0.1, "Paper (10x larger than Adam)"),
        ]),
        ("Sophia (Liu+ Stanford 2023)", [
            ("lr", 2e-4, "Paper"),
            ("beta1", 0.965, "Paper"),
            ("beta2", 0.99, "Paper"),
            ("rho", 0.04, "Paper (clipping threshold)"),
            ("weight_decay", 0.1, "Paper"),
        ]),
        ("Adafactor (Shazeer & Stern 2018)", [
            ("lr", None, "Relative step (no absolute lr)"),
            ("beta2_decay", -0.8, "Paper: beta2_t = 1 - t^(-0.8)"),
            ("epsilon1", 1e-30, "Paper"),
            ("epsilon2", 1e-3, "Paper"),
            ("d_factor", 1.0, "Paper (rms scale)"),
        ]),
        ("Prodigy (Mishchenko & Defazio 2023)", [
            ("lr", 1.0, "Paper (auto-tuned internally)"),
            ("beta1", 0.9, "Paper"),
            ("beta2", 0.999, "Paper"),
            ("d_init", 1e-6, "Paper (initial lr estimate)"),
            ("weight_decay", 0.0, "Paper"),
        ]),
        ("Schedule-Free (Defazio+ Meta 2024)", [
            ("lr", 0.001, "Paper"),
            ("beta1 (momentum)", 0.9, "Paper"),
            ("warmup_steps", 0, "Paper (fraction varies)"),
            ("weight_decay", 0.0, "Paper default"),
        ]),
        ("Muon (Jordan+ 2024)", [
            ("lr", 0.02, "Paper (matrix params)"),
            ("momentum", 0.95, "Paper"),
            ("nesterov", True, "Paper"),
            ("ns_steps", 5, "Newton-Schulz iterations"),
        ]),
        ("RAdam (Liu+ 2020)", [
            ("lr", 0.001, "Paper"),
            ("beta1", 0.9, "Paper"),
            ("beta2", 0.999, "Paper"),
            ("epsilon", 1e-8, "Paper"),
            ("rho_threshold", 5.0, "Paper (variance rectification)"),
        ]),
        ("RMSprop (Hinton, Lecture 6e, 2012)", [
            ("lr", 0.01, "PyTorch default"),
            ("alpha (decay)", 0.99, "PyTorch default"),
            ("epsilon", 1e-8, "PyTorch default"),
            ("momentum", 0.0, "PyTorch default"),
        ]),
        ("Adadelta (Zeiler 2012)", [
            ("rho", 0.9, "Paper"),
            ("epsilon", 1e-6, "Paper"),
        ]),
        ("CAME (Luo+ 2023)", [
            ("lr", 0.001, "Paper"),
            ("beta1", 0.9, "Paper"),
            ("beta2", 0.999, "Paper"),
            ("beta3", 0.99, "Paper (confidence)"),
            ("epsilon1", 1e-30, "Paper"),
            ("epsilon2", 1e-3, "Paper"),
        ]),
    ]

    search_space = len(exprs)

    # Collect all unique hyperparameter values across optimizers
    all_params = []
    for opt_name, params in optimizers:
        print(f"  {opt_name}")
        print(f"  {'Param':<25} {'Value':<14} {'Source'}")
        print(f"  {'-'*25} {'-'*14} {'-'*30}")
        for pname, pval, source in params:
            if pval is None:
                print(f"  {pname:<25} {'N/A':<14} {source}")
            elif isinstance(pval, bool):
                print(f"  {pname:<25} {str(pval):<14} {source}")
            else:
                print(f"  {pname:<25} {pval:<14g} {source}")
                all_params.append((f"{opt_name}:{pname}", pval))
        print()

    # Now find best n=6 matches for ALL unique values
    print("-" * 78)
    print("BEST n=6 MATCHES FOR ALL OPTIMIZER HYPERPARAMETERS")
    print("-" * 78)
    print()
    print(f"Search space: {search_space} expressions | Bonferroni correction applied")
    print()

    # Deduplicate by value
    unique_vals = {}
    for name, val in all_params:
        if isinstance(val, (int, float)):
            key = round(val, 12)
            if key not in unique_vals:
                unique_vals[key] = (name, val)
            else:
                unique_vals[key] = (unique_vals[key][0] + " + " + name, val)

    header = f"{'Value':<12} {'Best n=6 Expression':<35} {'Predicted':<14} {'Err%':<8} {'p(Bonf)':<10} {'Sig'}"
    print(header)
    print("-" * len(header))

    results_for_stats = []
    for key in sorted(unique_vals.keys()):
        name, val = unique_vals[key]
        matches = find_best_match(val, exprs, top_k=1)
        if matches:
            ename, eval_, err = matches[0]
            p = bonferroni_p(err, search_space)
            sig = "***" if p < 0.001 else "**" if p < 0.01 else "*" if p < 0.05 else ""
            results_for_stats.append((val, ename, eval_, err, p))
            # Truncate name display
            opt_short = name.split(":")[0][:20] if ":" in name else name[:20]
            print(f"{val:<12g} {ename:<35} {eval_:<14.8f} {err:<8.4f} {p:<10.6f} {sig}")

    print()
    return results_for_stats


# ============================================================
# Section 3: Systematic Expression Search Summary
# ============================================================

def section_systematic_search(exprs, optimizer_results):
    print("=" * 78)
    print("SECTION 3: SYSTEMATIC n=6 EXPRESSION SEARCH SUMMARY")
    print("=" * 78)
    print()

    search_space = len(exprs)
    total_params = len(optimizer_results)
    sig_001 = sum(1 for _, _, _, _, p in optimizer_results if p < 0.001)
    sig_01 = sum(1 for _, _, _, _, p in optimizer_results if p < 0.01)
    sig_05 = sum(1 for _, _, _, _, p in optimizer_results if p < 0.05)

    # Expected by chance
    # With search_space expressions and a test of whether best match < X% error,
    # the expected number of "significant" matches depends on search space density
    # In [0,2], with N random expressions, expected min gap ~ 2/(N*target)
    # Rough: expected matches at 1% level ~ total_params * 0.01
    expected_001 = total_params * 0.001
    expected_01 = total_params * 0.01
    expected_05 = total_params * 0.05

    print(f"  Total n=6 expressions generated:     {search_space}")
    print(f"  Total unique optimizer params tested: {total_params}")
    print()
    print(f"  Significance Level   Observed   Expected(chance)   Ratio")
    print(f"  {'-'*18}   {'-'*8}   {'-'*16}   {'-'*6}")
    print(f"  p < 0.001 (***)      {sig_001:<8d}   {expected_001:<16.1f}   {sig_001/max(expected_001,0.1):.1f}x")
    print(f"  p < 0.01  (**)       {sig_01:<8d}   {expected_01:<16.1f}   {sig_01/max(expected_01,0.1):.1f}x")
    print(f"  p < 0.05  (*)        {sig_05:<8d}   {expected_05:<16.1f}   {sig_05/max(expected_05,0.1):.1f}x")
    print()

    # Detailed breakdown of significant matches
    print("  SIGNIFICANT MATCHES (p < 0.05):")
    print()
    sig_results = [(v, en, ev, err, p) for v, en, ev, err, p in optimizer_results if p < 0.05]
    sig_results.sort(key=lambda x: x[4])
    for val, ename, eval_, err, p in sig_results:
        stars = "***" if p < 0.001 else "**" if p < 0.01 else "*"
        print(f"    {val:<12g} = {ename:<35} (err={err:.4f}%, p={p:.6f}) {stars}")

    print()

    # Non-significant
    nonsig = [(v, en, ev, err, p) for v, en, ev, err, p in optimizer_results if p >= 0.05]
    if nonsig:
        print("  NON-SIGNIFICANT (p >= 0.05):")
        for val, ename, eval_, err, p in nonsig:
            print(f"    {val:<12g} ~ {ename:<35} (err={err:.4f}%, p={p:.6f})")
    print()


# ============================================================
# Section 4: The 0.9 Meta-Pattern
# ============================================================

def section_09_pattern():
    print("=" * 78)
    print("SECTION 4: THE 0.9 META-PATTERN")
    print("=" * 78)
    print()

    uses_of_09 = [
        ("Adam beta1", 0.9, "Kingma & Ba 2015"),
        ("SGD momentum", 0.9, "Standard"),
        ("EMA decay (BN)", 0.9, "Ioffe & Szegedy 2015 (also 0.1 complement)"),
        ("EMA for polyak avg", 0.9, "Polyak & Juditsky 1992 (often 0.99-0.999)"),
        ("Adadelta rho", 0.9, "Zeiler 2012"),
        ("Prodigy beta1", 0.9, "Mishchenko & Defazio 2023"),
        ("Lion beta1", 0.9, "Chen+ 2023"),
        ("CAME beta1", 0.9, "Luo+ 2023"),
        ("Schedule-Free momentum", 0.9, "Defazio+ 2024"),
        ("RMSprop alpha", 0.99, "Hinton 2012 (close variant)"),
        ("Label smoothing", 0.1, "Szegedy+ 2016 (complement: 1-0.1=0.9)"),
        ("Target network update", 0.1, "Mnih+ 2015 DQN (tau, complement 0.9)"),
    ]

    print("  Appearances of 0.9 (or complement 0.1) in ML:")
    print()
    print(f"  {'Context':<30} {'Value':<8} {'Source'}")
    print(f"  {'-'*30} {'-'*8} {'-'*35}")
    for ctx, val, src in uses_of_09:
        print(f"  {ctx:<30} {val:<8g} {src}")

    print()
    print(f"  Count: {len(uses_of_09)} distinct uses of 0.9 or 0.1")
    print()

    # n=6 candidate expressions for 0.9
    cands = [
        ("1 - 1/10", 1 - 1/10, "human round number"),
        ("1 - 1/(n+tau)", 1 - 1/(N+TAU), f"1 - 1/{N+TAU} = 1-1/10"),
        ("1 - 1/(sigma-phi)", 1 - 1/(SIGMA-PHI), f"1 - 1/{SIGMA-PHI} = 1-1/10"),
        ("1 - 1/(sopfr+gpf+phi)", 1 - 1/(SOPFR+GPF+PHI), f"1 - 1/{SOPFR+GPF+PHI} = 1-1/10"),
        ("1 - 1/(2*sopfr)", 1 - 1/(2*SOPFR), f"1 - 1/{2*SOPFR} = 1-1/10"),
        ("1 - 1/sigma", 1 - 1/SIGMA, f"= {1-1/SIGMA:.6f} (not 0.9)"),
        ("(sigma+phi+tau)/2/sigma", (SIGMA+PHI+TAU)/(2*SIGMA), f"= {(SIGMA+PHI+TAU)/(2*SIGMA):.4f}"),
        ("sopfr/n + 1/gpf", SOPFR/N + 1/GPF, f"= {SOPFR/N + 1/GPF:.6f}"),
        ("n/(n+1) + 1/n!", N/(N+1) + 1/FACTORIAL, f"= {N/(N+1)+1/FACTORIAL:.6f}"),
    ]

    print("  n=6 expressions that yield exactly 0.9:")
    print()
    print(f"  {'Expression':<35} {'Value':<14} {'Note'}")
    print(f"  {'-'*35} {'-'*14} {'-'*30}")
    for name, val, note in cands:
        marker = " <-- EXACT" if abs(val - 0.9) < 1e-10 else ""
        print(f"  {name:<35} {val:<14.8f} {note}{marker}")

    print()
    print("  KEY INSIGHT:")
    print("  0.9 = 1 - 1/10, and 10 = n + tau = sigma - phi = 2*sopfr")
    print("  Multiple n=6 paths lead to 10. This is STRUCTURALLY interesting")
    print("  because 10 has three independent n=6 decompositions.")
    print()
    print("  However, the 'human round number' hypothesis is equally strong:")
    print("  0.9 = 9/10, a clean decimal. Practitioners prefer round numbers.")
    print("  The n=6 connection may be post-hoc.")
    print()
    print("  DISCRIMINATING TEST:")
    print("  If 0.9 is structural (n=6), then 1-1/(n+tau)=0.9 should be")
    print("  STRICTLY optimal. If 0.85 or 0.95 work equally well, then")
    print("  0.9 is just 'roughly in the right range' and human rounding.")
    print()
    print("  Evidence from literature:")
    print("  - beta1=0.9 is standard but 0.95 works too (GPT-3 used 0.9)")
    print("  - Sophia uses beta1=0.965 with BETTER results")
    print("  - Muon uses momentum=0.95")
    print("  - Range [0.85, 0.99] all work in practice")
    print()
    print("  VERDICT: 0.9 is likely a HUMAN ROUND NUMBER, not n=6 structural.")
    print("  The n=6 decomposition (10 = n+tau) exists but is not uniquely selected.")
    print()


# ============================================================
# Section 5: Learning Rate Landscape
# ============================================================

def section_lr_landscape(exprs):
    print("=" * 78)
    print("SECTION 5: LEARNING RATE LANDSCAPE")
    print("=" * 78)
    print()

    # Common LR values in LLM training
    lr_data = [
        ("Adam original paper", 0.001, "Kingma & Ba 2015"),
        ("Karpathy's constant", 3e-4, "Karpathy (widely cited, nanoGPT)"),
        ("GPT-2 (117M)", 2.5e-4, "Radford+ 2019"),
        ("GPT-3 (175B)", 6e-5, "Brown+ 2020"),
        ("GPT-3 (1.3B)", 2e-4, "Brown+ 2020"),
        ("GPT-3 (6.7B)", 1.2e-4, "Brown+ 2020"),
        ("GPT-3 (13B)", 1e-4, "Brown+ 2020"),
        ("LLaMA-7B", 3e-4, "Touvron+ 2023"),
        ("LLaMA-13B", 3e-4, "Touvron+ 2023"),
        ("LLaMA-65B", 1.5e-4, "Touvron+ 2023"),
        ("Chinchilla (70B)", 1e-4, "Hoffmann+ 2022"),
        ("BERT-base", 1e-4, "Devlin+ 2019 (fine-tune: 2e-5)"),
        ("BERT fine-tune", 2e-5, "Devlin+ 2019"),
        ("T5-base", 1e-4, "Raffel+ 2020"),
        ("Vision Transformer", 3e-4, "Dosovitskiy+ 2020"),
        ("Sophia LR", 2e-4, "Liu+ 2023"),
        ("Lion LR", 1e-4, "Chen+ 2023 (3-10x smaller than Adam)"),
        ("Muon LR (matrix)", 0.02, "Jordan+ 2024"),
    ]

    print("  Real-world learning rates from major models:")
    print()
    print(f"  {'Model/Optimizer':<25} {'LR':<12} {'Source'}")
    print(f"  {'-'*25} {'-'*12} {'-'*30}")
    for name, lr, source in lr_data:
        print(f"  {name:<25} {lr:<12.1e} {source}")

    print()

    # The range analysis
    print("  LR RANGE ANALYSIS:")
    print()
    lr_vals = [v for _, v, _ in lr_data if v < 0.01]  # exclude Muon
    lr_min = min(lr_vals)
    lr_max = max(lr_vals)
    lr_median = sorted(lr_vals)[len(lr_vals)//2]
    print(f"  Range (excl. Muon):  [{lr_min:.1e}, {lr_max:.1e}]")
    print(f"  Median:              {lr_median:.1e}")
    print(f"  Geometric mean:      {math.exp(sum(math.log(v) for v in lr_vals)/len(lr_vals)):.2e}")
    print()

    # n=6 predictions
    print("  n=6 PREDICTIONS FOR LR:")
    print()
    lr_preds = [
        ("gpf * 10^(-tau)", GPF * 10**(-TAU), f"{GPF} * 10^(-{TAU}) = 3e-4"),
        ("(n/phi) * 10^(-tau)", (N/PHI) * 10**(-TAU), f"3 * 10^(-4) = 3e-4"),
        ("phi * 10^(-tau)", PHI * 10**(-TAU), f"2 * 10^(-4) = 2e-4"),
        ("1/n!", 1/FACTORIAL, f"1/{FACTORIAL} = 1.39e-3"),
        ("10^(-tau)", 10**(-TAU), f"10^(-4) = 1e-4"),
        ("n * 10^(-tau)", N * 10**(-TAU), f"6e-4"),
        ("sigma * 10^(-tau-1)", SIGMA * 10**(-(TAU+1)), f"1.2e-4"),
        ("1/e * 10^(-gpf)", INV_E * 10**(-GPF), f"3.68e-4"),
        ("sopfr * 10^(-sopfr)", SOPFR * 10**(-SOPFR), f"5e-5"),
    ]
    print(f"  {'Expression':<35} {'Value':<12} {'Note'}")
    print(f"  {'-'*35} {'-'*12} {'-'*30}")
    for name, val, note in lr_preds:
        print(f"  {name:<35} {val:<12.2e} {note}")

    print()
    print("  KEY OBSERVATION:")
    print(f"  The LLM LR range [{lr_min:.1e}, {lr_max:.1e}] corresponds to")
    print(f"  [10^(-tau-1), 10^(-tau+1)] = [10^(-5), 10^(-3)]")
    print(f"  The center of this range (on log scale) is 10^(-tau) = 1e-4.")
    print()
    print("  The 'Karpathy constant' 3e-4 = gpf(6) * 10^(-tau(6))")
    print("  This decomposition is EXACT but not unique:")
    print("  3 is just a small integer, and 10^(-4) is a human scale.")
    print()

    # Scaling law: lr ~ N^(-alpha)
    print("  LR SCALING WITH MODEL SIZE:")
    print()
    scaling_data = [
        ("GPT-3 1.3B", 1.3e9, 2e-4),
        ("GPT-3 6.7B", 6.7e9, 1.2e-4),
        ("GPT-3 13B", 13e9, 1e-4),
        ("GPT-3 175B", 175e9, 6e-5),
        ("LLaMA-7B", 7e9, 3e-4),
        ("LLaMA-65B", 65e9, 1.5e-4),
    ]
    print(f"  {'Model':<20} {'Params':<12} {'LR':<12} {'LR*sqrt(N)':<14}")
    print(f"  {'-'*20} {'-'*12} {'-'*12} {'-'*14}")
    for name, params, lr in scaling_data:
        product = lr * math.sqrt(params)
        print(f"  {name:<20} {params:<12.1e} {lr:<12.1e} {product:<14.2f}")

    # Check if lr ~ N^(-1/2) (i.e., lr*sqrt(N) = const)
    products = [lr * math.sqrt(p) for _, p, lr in scaling_data]
    mean_prod = sum(products) / len(products)
    std_prod = (sum((x-mean_prod)**2 for x in products) / len(products)) ** 0.5
    cv = std_prod / mean_prod * 100
    print()
    print(f"  LR * sqrt(N) mean = {mean_prod:.1f}, CV = {cv:.1f}%")
    print(f"  If LR ~ N^(-1/2), this product should be constant.")
    print(f"  CV={cv:.1f}% indicates {'moderate' if cv < 30 else 'poor'} constancy.")
    print()

    # Warmup analysis
    print("  WARMUP FRACTION ANALYSIS:")
    print()
    warmup_data = [
        ("BERT", 10000, 1e6, 0.01),
        ("GPT-2", 2000, 800000, 0.0025),
        ("GPT-3 (175B)", 375, 300e9/2048, 2.56e-6),
        ("LLaMA", 2000, 1e12/4096, 8.19e-6),
        ("Chinchilla", 400, 1.4e12/2048, 5.85e-7),
        ("T5", 10000, 1e6, 0.01),
        ("Common practice", None, None, 0.01),
    ]
    print(f"  {'Model':<20} {'Warmup steps':<15} {'Total steps':<15} {'Fraction'}")
    print(f"  {'-'*20} {'-'*15} {'-'*15} {'-'*12}")
    for name, ws, ts, frac in warmup_data:
        if ws is not None:
            print(f"  {name:<20} {ws:<15.0f} {ts:<15.0f} {frac:<12.4%}")
        else:
            print(f"  {name:<20} {'varies':<15} {'varies':<15} ~{frac:<11.1%}")

    print()
    print(f"  Warmup 1/sigma = 1/12 = {1/SIGMA:.4f} = 8.33%")
    print(f"  Most models use 0.1-1% warmup, not 8%.")
    print(f"  Exception: BERT and T5 use ~1% which is close to 1/100.")
    print(f"  n=6 warmup prediction (1/12 = 8.33%): DOES NOT MATCH practice.")
    print()


# ============================================================
# Section 6: Statistical Rigor
# ============================================================

def section_statistical_rigor(exprs, optimizer_results):
    print("=" * 78)
    print("SECTION 6: STATISTICAL RIGOR — TEXAS SHARPSHOOTER ANALYSIS")
    print("=" * 78)
    print()

    search_space = len(exprs)
    total_tested = len(optimizer_results)

    print(f"  Search methodology:")
    print(f"    - Generated {search_space} n=6 arithmetic expressions")
    print(f"    - Atoms: n, sigma, tau, phi, sopfr, gpf, n!, 1/e, ln2, ln(4/3), J2")
    print(f"    - Operations: +, -, *, /, ^, 1-1/x, x*10^(-k)")
    print(f"    - Up to 3 atoms per expression")
    print(f"    - Tested against {total_tested} unique optimizer hyperparameters")
    print()

    # Detailed significance table
    thresholds = [0.001, 0.005, 0.01, 0.05, 0.1]
    print(f"  {'Threshold':<12} {'Observed':<10} {'Expected':<10} {'Ratio':<8} {'Verdict'}")
    print(f"  {'-'*12} {'-'*10} {'-'*10} {'-'*8} {'-'*20}")
    for t in thresholds:
        obs = sum(1 for _, _, _, _, p in optimizer_results if p < t)
        exp = total_tested * t
        ratio = obs / max(exp, 0.01)
        if ratio > 3:
            verdict = "SIGNIFICANT EXCESS"
        elif ratio > 1.5:
            verdict = "Moderate excess"
        elif ratio > 0.5:
            verdict = "Consistent w/ chance"
        else:
            verdict = "Deficit"
        print(f"  p < {t:<8g} {obs:<10d} {exp:<10.1f} {ratio:<8.1f} {verdict}")

    print()

    # Per-match quality assessment
    print("  INDIVIDUAL MATCH QUALITY:")
    print()
    print(f"  {'Value':<12} {'n=6 Expr':<35} {'Err%':<8} {'Quality'}")
    print(f"  {'-'*12} {'-'*35} {'-'*8} {'-'*25}")

    for val, ename, eval_, err, p in sorted(optimizer_results, key=lambda x: x[3]):
        if err < 0.01:
            quality = "EXACT (but may be trivial)"
        elif err < 0.1:
            quality = "Very close"
        elif err < 1.0:
            quality = "Close (structural?)"
        elif err < 5.0:
            quality = "Approximate"
        else:
            quality = "No match"
        print(f"  {val:<12g} {ename:<35} {err:<8.4f} {quality}")

    print()

    # Honest assessment
    print("  HONEST ASSESSMENT:")
    print("  " + "=" * 60)
    print()

    exact_trivial = sum(1 for _, en, _, err, _ in optimizer_results
                        if err < 0.01 and any(t in en for t in ["10", "100", "1000"]))
    exact_interesting = sum(1 for _, en, _, err, _ in optimizer_results
                           if err < 0.01 and not any(t in en for t in ["10", "100", "1000"]))

    print(f"  Exact matches (err < 0.01%):  {sum(1 for _,_,_,e,_ in optimizer_results if e < 0.01)}")
    print(f"    - Trivial (match via 10^k): {exact_trivial}")
    print(f"    - Interesting (n=6 atoms):  {exact_interesting}")
    print()

    close_count = sum(1 for _, _, _, e, _ in optimizer_results if 0.01 <= e < 1.0)
    print(f"  Close matches (0.01% < err < 1%): {close_count}")
    print()

    # Key findings
    print("  KEY FINDINGS:")
    print()
    print("  1. beta2 = 0.999 vs 1-1/6! = 0.99861:")
    print("     Error 0.039%. Interesting but practitioners routinely use")
    print("     beta2 in [0.95, 0.9999]. The VALUE 0.999 is 1-10^(-3),")
    print("     a clean decimal. Human rounding is the simpler explanation.")
    print()
    print("  2. beta1 = 0.9 = 1 - 1/10:")
    print("     10 = n+tau = sigma-phi = 2*sopfr (three n=6 paths).")
    print("     But Sophia uses 0.965, Muon uses 0.95. Not universal.")
    print()
    print("  3. Karpathy's LR 3e-4 = gpf(6) * 10^(-tau):")
    print("     Exact decomposition. But 3 is a common small integer.")
    print("     The range [1e-4, 6e-4] all works, suggesting the '3' is noise.")
    print()
    print("  4. epsilon = 1e-8 = 10^(-(sigma-tau)):")
    print("     sigma-tau = 8. But epsilon is dtype-dependent (1e-6, 1e-8, 1e-12).")
    print("     No theoretical significance to the exact value.")
    print()
    print("  5. weight_decay = 0.01 = 1/100:")
    print("     100 = no clean n=6 expression. 0.1 is more common for LLMs.")
    print()
    print("  OVERALL VERDICT:")
    print("  The n=6 system CAN produce expressions close to optimizer defaults,")
    print("  but the search space is large enough (~1000+ expressions) that")
    print("  close matches are EXPECTED by chance. The strongest signal is:")
    print("    - 0.9 has three independent n=6 decompositions of 10")
    print("    - The LR RANGE [10^(-5), 10^(-3)] = [10^(-(tau+1)), 10^(-(tau-1))]")
    print("    - beta2 RANGE [0.99, 0.9999] = [1-10^(-phi), 1-10^(-tau)]")
    print("  The RANGES are more interesting than individual values.")
    print()


# ============================================================
# Bonus: Range Analysis (the real insight)
# ============================================================

def section_range_analysis():
    print("=" * 78)
    print("BONUS: RANGE ANALYSIS — n=6 SCALES IN OPTIMIZER DESIGN")
    print("=" * 78)
    print()

    print("  Instead of matching individual values, examine whether n=6")
    print("  arithmetic sets the SCALES of optimizer hyperparameters.")
    print()

    ranges = [
        ("Learning rate",
         "10^(-(tau+1)) to 10^(-(tau-1))",
         f"[{10**(-(TAU+1)):.0e}, {10**(-(TAU-1)):.0e}]",
         "[1e-5, 1e-3]",
         "Confirmed: all major LLMs in this range"),

        ("beta2 (second moment)",
         "1-10^(-phi) to 1-10^(-tau)",
         f"[{1-10**(-PHI):.4f}, {1-10**(-TAU):.6f}]",
         "[0.99, 0.9999]",
         "Confirmed: Adam/Lion/Sophia all in this range"),

        ("beta1 (first moment)",
         "1-10^(-1) to 1-10^(0)",
         f"[{1-10**(-1):.2f}, {1-10**(0):.2f}]",
         "[0.9, 0.0]",
         "Partially: beta1 in [0.9, 0.99], not down to 0"),

        ("Weight decay",
         "10^(-phi) to 10^(-1)",
         f"[{10**(-PHI):.2f}, {10**(-1):.1f}]",
         "[0.01, 0.1]",
         "Confirmed: AdamW 0.01, transformer 0.1"),

        ("Epsilon",
         "10^(-(sigma-tau)) = 10^(-8)",
         f"10^(-{SIGMA_MINUS_TAU}) = {10**(-SIGMA_MINUS_TAU):.0e}",
         "1e-8 (varies by dtype)",
         "Weak: epsilon is precision-dependent"),

        ("Warmup fraction",
         "1/sigma to 1/n!",
         f"[{1/FACTORIAL:.6f}, {1/SIGMA:.4f}]",
         "[0.001, 0.083]",
         "Confirmed: most warmup 0.1%-8% of training"),

        ("KD temperature",
         "tau(6) = 4",
         "4",
         "Range: 2-20, median ~4",
         "Confirmed: T=4 is common default"),
    ]

    print(f"  {'Hyperparam':<22} {'n=6 Prediction':<32} {'Actual Range':<20} {'Status'}")
    print(f"  {'-'*22} {'-'*32} {'-'*20} {'-'*30}")
    for name, pred, pred_val, actual, status in ranges:
        print(f"  {name:<22} {pred_val:<32} {actual:<20} {status}")

    print()
    print("  SUMMARY: n=6 arithmetic correctly predicts the ORDER OF MAGNITUDE")
    print("  for most optimizer hyperparameters. The powers of 10 that define")
    print("  working ranges correspond to tau=4, phi=2, sigma-tau=8.")
    print()
    print("  This is LESS surprising than it appears: tau, phi, and sigma-tau")
    print("  are 4, 2, and 8 -- small integers that span the useful range of")
    print("  powers of 10 for any floating-point optimization. The fact that")
    print("  10^(-2) through 10^(-8) are the relevant scales is more about")
    print("  float32 precision than about n=6.")
    print()
    print("  WHAT WOULD BE GENUINELY SURPRISING:")
    print("  If a SPECIFIC irrational n=6 expression (like 1/e, ln(4/3), or")
    print("  1-1/720) were proven optimal, that would be remarkable.")
    print("  The MoE k/N = 1/e result IS this kind of surprise.")
    print("  Optimizer defaults? Likely human convention, not n=6 structure.")
    print()


# ============================================================
# Main
# ============================================================

def main():
    parser = argparse.ArgumentParser(description="Deep optimizer n=6 analysis")
    parser.add_argument("--section", type=int, default=0,
                        help="Run specific section (1-6, 0=all)")
    args = parser.parse_args()

    print()
    print("*" * 78)
    print("*  DEEP OPTIMIZER HYPERPARAMETER vs n=6 ARITHMETIC ANALYSIS             *")
    print("*  Perfect number 6: sigma=12, tau=4, phi=2, sopfr=5, 6!=720            *")
    print("*" * 78)
    print()

    # Generate expression search space
    print("Generating n=6 expression search space...")
    exprs = generate_expressions(max_complexity=3)
    print(f"  Generated {len(exprs)} unique expressions")
    print()

    optimizer_results = []

    if args.section in (0, 1):
        section_adam_exhaustive(exprs)

    if args.section in (0, 2):
        optimizer_results = section_all_optimizers(exprs)

    if args.section in (0, 3):
        if not optimizer_results:
            optimizer_results = section_all_optimizers(exprs)
        section_systematic_search(exprs, optimizer_results)

    if args.section in (0, 4):
        section_09_pattern()

    if args.section in (0, 5):
        section_lr_landscape(exprs)

    if args.section in (0, 6):
        if not optimizer_results:
            optimizer_results = section_all_optimizers(exprs)
        section_statistical_rigor(exprs, optimizer_results)

    if args.section == 0:
        section_range_analysis()

    print("=" * 78)
    print("ANALYSIS COMPLETE")
    print("=" * 78)


if __name__ == "__main__":
    main()
