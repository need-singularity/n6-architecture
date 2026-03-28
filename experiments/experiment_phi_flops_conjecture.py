"""
Experiment: Phi-FLOPs Conjecture
==================================
Test: Phi * FLOPs_per_token = C (constant ~ sigma(6) = 12?)
Measure across model sizes and configurations.
"""

import sys
sys.path.insert(0, '.')

import torch
import torch.nn as nn
import torch.nn.functional as F
import numpy as np
import math
import time

from engine.phi_efficiency_bridge import approximate_phi, estimate_flops

SEED = 42
torch.manual_seed(SEED)
np.random.seed(SEED)

SIGMA = 12


class Phi6Simple(nn.Module):
    def forward(self, x):
        xc = torch.clamp(x, -2.0, 2.0)
        return xc * xc - xc + 1.0


class FFN(nn.Module):
    def __init__(self, d, d_ff, act):
        super().__init__()
        self.fc1 = nn.Linear(d, d_ff)
        self.act = act
        self.fc2 = nn.Linear(d_ff, d)

    def forward(self, x):
        return self.fc2(self.act(self.fc1(x)))


class Block(nn.Module):
    def __init__(self, d, heads, d_ff, act):
        super().__init__()
        self.attn = nn.MultiheadAttention(d, heads, batch_first=True)
        self.ffn = FFN(d, d_ff, act)
        self.ln1 = nn.LayerNorm(d)
        self.ln2 = nn.LayerNorm(d)

    def forward(self, x):
        L = x.size(1)
        mask = torch.triu(torch.ones(L, L, device=x.device), diagonal=1).bool()
        a, _ = self.attn(x, x, x, attn_mask=mask)
        x = self.ln1(x + a)
        x = self.ln2(x + self.ffn(x))
        return x


class Transformer(nn.Module):
    def __init__(self, d, heads, layers, d_ff, seq, act):
        super().__init__()
        self.pos = nn.Embedding(seq, d)
        self.proj_in = nn.Linear(d, d)
        self.blocks = nn.Sequential(*[Block(d, heads, d_ff, act) for _ in range(layers)])
        self.proj_out = nn.Linear(d, d)

    def forward(self, x):
        B, L, D = x.shape
        x = self.proj_in(x) + self.pos(torch.arange(L, device=x.device))
        x = self.blocks(x)
        return self.proj_out(x)


def main():
    print("=" * 70)
    print("  Experiment: Phi-FLOPs Conjecture")
    print("  Phi * FLOPs_per_token =? sigma(6) = 12")
    print("=" * 70)

    SEQ = 32
    BATCH = 8

    n6_configs = [
        ("n6-tiny",    48,  4,  1, round(48*4/3)),
        ("n6-small",   120, 6,  2, round(120*4/3)),
        ("n6-medium",  120, 12, 4, round(120*4/3)),
        ("n6-large",   240, 12, 4, round(240*4/3)),
        ("n6-xlarge",  360, 12, 6, round(360*4/3)),
        ("n6-xxlarge", 720, 12, 6, round(720*4/3)),
    ]

    std_configs = [
        ("std-small",  128, 8,  2, 512),
        ("std-medium", 256, 8,  4, 1024),
        ("std-large",  512, 8,  4, 2048),
    ]

    all_configs = n6_configs + std_configs
    results = []

    print(f"\n{'Config':<14} {'d':>5} {'h':>3} {'L':>3} {'ff':>5} {'Phi':>8} {'FLOPs/tok':>12} {'Phi*F':>10} {'Params':>10}")
    print("-" * 80)

    for label, d, h, layers, d_ff in all_configs:
        model = Transformer(d, h, layers, d_ff, SEQ, Phi6Simple())
        sample = torch.randn(BATCH, SEQ, d)

        phi = approximate_phi(model, sample)
        flops = estimate_flops(d, d_ff, h, layers, SEQ, BATCH)
        product = phi * flops
        params = sum(p.numel() for p in model.parameters())

        results.append({
            "label": label,
            "is_n6": label.startswith("n6"),
            "phi": phi,
            "flops": flops,
            "product": product,
            "params": params,
        })

        print(f"{label:<14} {d:>5} {h:>3} {layers:>3} {d_ff:>5} {phi:>8.2f} "
              f"{flops:>12.0f} {product:>10.0f} {params:>10,}")

    n6_products = [r["product"] for r in results if r["is_n6"]]
    std_products = [r["product"] for r in results if not r["is_n6"]]

    print(f"\n--- Analysis ---")
    print(f"N=6 configs: Phi*FLOPs mean={np.mean(n6_products):.0f}, std={np.std(n6_products):.0f}, "
          f"CV={np.std(n6_products)/np.mean(n6_products):.3f}")
    if std_products:
        print(f"Std configs:  Phi*FLOPs mean={np.mean(std_products):.0f}, std={np.std(std_products):.0f}, "
              f"CV={np.std(std_products)/np.mean(std_products):.3f}")

    n6_cv = np.std(n6_products) / np.mean(n6_products)
    print(f"\nCoefficient of Variation (lower = more constant):")
    print(f"  N=6 configs: {n6_cv:.3f}")
    if std_products:
        std_cv = np.std(std_products) / np.mean(std_products)
        print(f"  Std configs:  {std_cv:.3f}")

    print(f"\nTarget constant: sigma(6) = {SIGMA}")
    print(f"Note: Phi proxy uses effective rank of activation covariance.")
    print(f"True IIT Phi requires Anima's consciousness_meter for validation.")


if __name__ == "__main__":
    main()
