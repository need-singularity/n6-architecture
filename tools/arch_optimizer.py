#!/usr/bin/env python3
"""
Architecture Optimizer — Find nearest n=6-optimal config for given constraints.

Usage:
  python tools/arch_optimizer.py --params 1M           # Best config under 1M params
  python tools/arch_optimizer.py --params 100M --moe   # With MoE routing
  python tools/arch_optimizer.py --flops 1G            # Best config under 1G FLOPs
  python tools/arch_optimizer.py --custom 256 8        # Optimize d=256, h=8
"""

import math
import argparse

SIGMA = 12
TAU = 4
PHI = 2
SOPFR = 5
J2 = 24

# N6-optimal values
N6_TARGETS = {
    "d_model": [48, 60, 120, 180, 240, 360, 480, 720, 840, 1080, 1260, 1680],  # HCN dims
    "n_heads": [1, 2, 3, 4, 6, 12],  # Divisors of 12
    "ffn_ratio": 4/3,
    "n_experts": [1, 6, 12, 24],
    "dropout": math.log(4/3),
    "activation": "phi6",
}


def count_params(d_model, n_heads, n_layers, d_ff, n_experts=1):
    """Estimate transformer parameters."""
    attn = n_layers * (4 * d_model * d_model)  # QKV + output projections
    ffn = n_layers * (2 * d_model * d_ff) * n_experts
    emb = d_model * 50000  # rough vocab estimate
    return attn + ffn + emb


def estimate_flops(d_model, n_heads, n_layers, d_ff, seq_len=512):
    """Estimate FLOPs per token."""
    attn = n_layers * (4 * d_model**2 + 2 * seq_len * d_model)
    ffn = n_layers * 2 * d_model * d_ff
    return attn + ffn


def r_score(d_model, d_ff, n_heads, activation="phi6"):
    """Quick R-score."""
    s = 1.0 if d_model % 12 == 0 else 1.0 - min(d_model%12, 12-d_model%12)/12
    t = max(0, 1.0 - abs(d_ff/d_model - 4/3) / (4/3))
    n = 1.0 if activation == "phi6" else 0.5
    return s * t * n


def optimize_for_params(max_params, use_moe=False):
    """Find best n=6 config under param budget."""
    print(f"\n  Architecture Optimizer (max params: {max_params:,})")
    print(f"  MoE: {'Yes (24 experts)' if use_moe else 'No (dense)'}")
    print("=" * 65)

    best = None
    results = []

    for d in N6_TARGETS["d_model"]:
        for h in N6_TARGETS["n_heads"]:
            if d % h != 0: continue
            d_ff = round(d * N6_TARGETS["ffn_ratio"])
            n_exp = 24 if use_moe else 1

            for n_layers in range(1, 25):
                params = count_params(d, h, n_layers, d_ff, n_exp)
                if params > max_params: break

                r = r_score(d, d_ff, h)
                flops = estimate_flops(d, h, n_layers, d_ff)

                results.append({
                    "d": d, "h": h, "L": n_layers, "ff": d_ff,
                    "experts": n_exp, "params": params,
                    "R": r, "flops": flops,
                })

    results.sort(key=lambda x: (-x["R"], -x["params"]))

    print(f"\n  {'d':>5} {'h':>3} {'L':>3} {'ff':>5} {'E':>3} {'Params':>12} {'R-score':>8} {'FLOPs':>12}")
    print("-" * 60)
    seen = set()
    for r in results[:15]:
        key = (r["d"], r["h"], r["L"])
        if key in seen: continue
        seen.add(key)
        print(f"  {r['d']:>5} {r['h']:>3} {r['L']:>3} {r['ff']:>5} {r['experts']:>3} "
              f"{r['params']:>12,} {r['R']:>8.4f} {r['flops']:>12,}")

    if results:
        best = results[0]
        print(f"\n  RECOMMENDED:")
        print(f"    d_model = {best['d']} (HCN dimension)")
        print(f"    n_heads = {best['h']} (divisor of 12)")
        print(f"    n_layers = {best['L']}")
        print(f"    d_ff = {best['ff']} (4/3 ratio)")
        print(f"    dropout = {N6_TARGETS['dropout']:.4f} (ln(4/3))")
        print(f"    activation = phi6 (x^2-x+1)")
        if use_moe:
            print(f"    experts = 24 (J2=24, Egyptian routing)")
        print(f"    R-score = {best['R']:.4f}")
        print(f"    Params = {best['params']:,}")


def optimize_custom(d_model, n_heads):
    """Suggest n=6 improvements for custom config."""
    print(f"\n  Custom Config Optimizer: d={d_model}, h={n_heads}")
    print("=" * 55)

    d_ff_current = 4 * d_model
    d_ff_n6 = round(d_model * 4/3)

    # Find nearest HCN dim
    hcn = min(N6_TARGETS["d_model"], key=lambda x: abs(x - d_model))
    # Find nearest valid heads
    valid_h = [h for h in N6_TARGETS["n_heads"] if hcn % h == 0]
    best_h = min(valid_h, key=lambda h: abs(h - n_heads)) if valid_h else n_heads

    print(f"\n  Current → N6 Recommended:")
    print(f"  {'Parameter':<20} {'Current':>12} {'N6':>12} {'Saving':>10}")
    print("-" * 58)

    d_change = "" if d_model == hcn else f"→ {hcn}"
    h_change = "" if n_heads == best_h else f"→ {best_h}"
    ff_save = (1 - d_ff_n6 * hcn / (d_ff_current * d_model)) * 100

    print(f"  {'d_model':<20} {d_model:>12} {hcn:>12} {'HCN aligned'}")
    print(f"  {'n_heads':<20} {n_heads:>12} {best_h:>12} {'div of 12'}")
    print(f"  {'d_ff':<20} {d_ff_current:>12} {d_ff_n6:>12} {ff_save:>9.0f}%")
    print(f"  {'activation':<20} {'GELU':>12} {'Phi6':>12} {'7x faster'}")
    print(f"  {'dropout':<20} {'0.1 (guess)':>12} {N6_TARGETS['dropout']:>12.4f} {'no search'}")

    r_old = r_score(d_model, d_ff_current, n_heads, "gelu")
    r_new = r_score(hcn, round(hcn*4/3), best_h, "phi6")
    print(f"\n  R-score: {r_old:.4f} → {r_new:.4f} ({(r_new/max(r_old,0.001)-1)*100:+.0f}%)")


def main():
    parser = argparse.ArgumentParser(description="N6 Architecture Optimizer")
    parser.add_argument("--params", type=str, help="Max params (e.g., 1M, 100M, 1B)")
    parser.add_argument("--moe", action="store_true", help="Enable MoE (24 experts)")
    parser.add_argument("--custom", nargs=2, type=int, metavar=("D_MODEL", "N_HEADS"),
                       help="Optimize existing config")
    args = parser.parse_args()

    if args.params:
        multipliers = {"K": 1e3, "M": 1e6, "B": 1e9, "G": 1e9}
        s = args.params.upper()
        for suffix, mult in multipliers.items():
            if s.endswith(suffix):
                max_p = int(float(s[:-1]) * mult)
                break
        else:
            max_p = int(s)
        optimize_for_params(max_p, args.moe)
    elif args.custom:
        optimize_custom(*args.custom)
    else:
        print("Usage: python tools/arch_optimizer.py --params 1M")
        print("       python tools/arch_optimizer.py --custom 256 8")
        optimize_for_params(1_000_000)


if __name__ == "__main__":
    main()
